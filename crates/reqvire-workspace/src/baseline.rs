//! Offline source reconstruction. Model parsing belongs to consumers.
use crate::{
    git::{local_text as text, run_local},
    Error, WorkspaceComposition, WorkspaceProject,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Component, Path, PathBuf},
    sync::Arc,
};
fn failure(s: impl Into<String>) -> Error {
    Error::ProcessError(s.into())
}
pub fn resolve(directory: &Path, reference: &str) -> Result<String, Error> {
    if reference.trim().is_empty() {
        return Err(failure(format!(
            "Historical Git revision is empty: {}",
            directory.display()
        )));
    }
    text(
        directory,
        &[
            "rev-parse",
            "--verify",
            "--end-of-options",
            &format!("{reference}^{{commit}}"),
        ],
    )
}
fn relative(path: &Path) -> Result<(), Error> {
    if path.as_os_str().is_empty()
        || path
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
    {
        return Err(failure(format!(
            "Invalid historical source path {}",
            path.display()
        )));
    }
    Ok(())
}

/// Read Git objects directly: no checkout, filters, index, ref updates or fetch.
pub fn tree(directory: &Path, commit: &str) -> Result<BTreeMap<PathBuf, Arc<Vec<u8>>>, Error> {
    let mut reader =
        crate::git::GitReader::new(directory, |args, input| run_local(directory, args, input));
    let entries = reader.files_at_commit(commit)?;
    let mut files = BTreeMap::new();
    let mut links = BTreeSet::new();
    for (path, entry) in entries {
        let path = path
            .strip_prefix(directory)
            .map_err(|e| failure(e.to_string()))?
            .to_owned();
        relative(&path)?;
        if entry.symlink {
            links.insert(path.clone());
        }
        files.insert(path, entry.bytes);
    }
    let resolved = links
        .iter()
        .map(|path| {
            link_contents(path, &files, &links, &mut BTreeSet::new())
                .map(|bytes| (path.clone(), bytes))
        })
        .collect::<Result<Vec<_>, _>>()?;
    for (path, bytes) in resolved {
        if let Some(bytes) = bytes {
            files.insert(path, bytes);
        } else {
            files.remove(&path);
        } // Directory symlinks are not file sources.
    }
    Ok(files)
}

fn link_contents(
    path: &Path,
    files: &BTreeMap<PathBuf, Arc<Vec<u8>>>,
    links: &BTreeSet<PathBuf>,
    visiting: &mut BTreeSet<PathBuf>,
) -> Result<Option<Arc<Vec<u8>>>, Error> {
    let Some(bytes) = files.get(path) else {
        if let Some(parent) = path
            .ancestors()
            .skip(1)
            .find(|parent| links.contains(*parent))
        {
            if !visiting.insert(parent.to_owned()) {
                return Err(failure(format!(
                    "Historical symlink cycle: {}",
                    parent.display()
                )));
            }
            let target = link_target(parent, &files[parent])?;
            let suffix = path.strip_prefix(parent).expect("source ancestor");
            return link_contents(&target.join(suffix), files, links, visiting);
        }
        if files
            .keys()
            .any(|file| file != path && file.starts_with(path))
        {
            return Ok(None);
        }
        return Err(failure(format!(
            "Unavailable historical symlink target: {}",
            path.display()
        )));
    };
    if !links.contains(path) {
        return Ok(Some(Arc::clone(bytes)));
    }
    if !visiting.insert(path.to_owned()) {
        return Err(failure(format!(
            "Historical symlink cycle: {}",
            path.display()
        )));
    }
    link_contents(&link_target(path, bytes)?, files, links, visiting)
}

fn link_target(path: &Path, bytes: &[u8]) -> Result<PathBuf, Error> {
    let target = std::str::from_utf8(bytes)
        .map_err(|e| failure(format!("Historical symlink {}: {e}", path.display())))?;
    let mut resolved = PathBuf::new();
    for component in path
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .join(target)
        .components()
    {
        match component {
            Component::Normal(name) => resolved.push(name),
            Component::CurDir => {}
            Component::ParentDir if resolved.pop() => {}
            _ => {
                return Err(failure(format!(
                    "Historical symlink escapes its repository: {}",
                    path.display()
                )))
            }
        }
    }
    Ok(resolved)
}

pub fn materialize(root: &Path, files: &BTreeMap<PathBuf, Arc<Vec<u8>>>) -> Result<(), Error> {
    for (path, bytes) in files {
        relative(path)?;
        let destination = root.join(path);
        std::fs::create_dir_all(
            destination
                .parent()
                .ok_or_else(|| failure("Source path has no parent"))?,
        )?;
        std::fs::write(destination, bytes.as_slice())?;
    }
    Ok(())
}

pub fn project_objects(
    root: &Path,
    current: &WorkspaceComposition,
    project: &WorkspaceProject,
) -> Result<PathBuf, Error> {
    let identity = crate::manifest::normalize_url(&project.fetch_url);
    let mut candidates: Vec<PathBuf> = current
        .projects
        .iter()
        .filter(|p| {
            p.name == project.name && crate::manifest::normalize_url(&p.fetch_url) == identity
        })
        .map(|p| root.join(&p.path))
        .collect();
    // Repo retains this per-checkout Git directory after a project is removed.
    // Its remote config identifies the shared object database unambiguously.
    let mut stored = root
        .join(".repo/projects")
        .join(&project.path)
        .into_os_string();
    stored.push(".git");
    let stored = PathBuf::from(stored);
    candidates.push(stored);
    candidates.push(root.join(&project.path));
    let common = crate::git::local_text(
        root,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?;
    candidates.push(
        PathBuf::from(common)
            .join("reqvire-repo-objects")
            .join(crate::service::hash(
                format!("{}:{}", project.name, identity).as_bytes(),
            )),
    );
    for candidate in candidates {
        if !candidate.is_dir() {
            continue;
        }
        // A retained Reqvire cache is keyed by both name and repository identity.
        let cached = candidate
            .parent()
            .is_some_and(|p| p.ends_with("reqvire-repo-objects"));
        let matches = cached
            || text(&candidate, &["remote"]).is_ok_and(|remotes| {
                remotes.lines().any(|remote| {
                    text(&candidate, &["remote", "get-url", remote])
                        .is_ok_and(|url| crate::manifest::normalize_url(&url) == identity)
                })
            });
        if matches && resolve(&candidate, &project.revision).is_ok() {
            return Ok(candidate);
        }
    }
    Err(failure(format!("repo_impact_base_unavailable: project {} ({}) revision {} is unavailable locally; no fetch was attempted",project.name,project.path.display(),project.revision)))
}

pub struct Baseline {
    pub composition: WorkspaceComposition,
    pub files: BTreeMap<PathBuf, Arc<Vec<u8>>>,
}
pub fn read(
    root: &Path,
    current: &WorkspaceComposition,
    reference: &str,
) -> Result<Baseline, Error> {
    let central = resolve(
        root,
        if reference == "HEAD" {
            &current.central_commit
        } else {
            reference
        },
    )?;
    let owner = PathBuf::from(text(root, &["rev-parse", "--show-toplevel"])?).canonicalize()?;
    let prefix = root
        .strip_prefix(&owner)
        .map_err(|e| failure(e.to_string()))?;
    let central_files: BTreeMap<_, _> = tree(&owner, &central)?
        .into_iter()
        .filter_map(|(p, b)| p.strip_prefix(prefix).ok().map(|p| (p.to_owned(), b)))
        .collect();
    let temporary = tempfile::tempdir()?;
    let staged = temporary.path();
    materialize(staged, &central_files)?;
    let mut base = if central_files.contains_key(&current.manifest) {
        resolve_native(staged, current.manifest.clone())?
    } else if central_files.contains_key(Path::new(crate::manifest::MANIFEST)) {
        resolve_native(staged, crate::manifest::MANIFEST.into())?
    } else {
        let mut candidates = BTreeSet::new();
        let mut included = BTreeSet::new();
        for (path, bytes) in &central_files {
            if path.extension().is_some_and(|e| e == "xml") {
                if let Ok(text) = std::str::from_utf8(bytes) {
                    if let Ok(tags) = crate::legacy::tags(text) {
                        if tags.iter().any(|(tag, _)| tag == "manifest") {
                            candidates.insert(path.clone());
                        }
                        for (tag, attrs) in tags {
                            if tag == "include" {
                                if let Some(name) = attrs.get("name") {
                                    included.insert(PathBuf::from(name));
                                }
                            }
                        }
                    }
                }
            }
        }
        let roots: Vec<_> = candidates.difference(&included).collect();
        match roots.as_slice(){
    []=>WorkspaceComposition::default(),
    [selection]=>crate::legacy::resolve(staged,selection)?,
    _=>return Err(failure("repo_impact_manifest_history_unavailable: ambiguous versioned XML selection in migration baseline")),
   }
    };
    base.central_commit = central;
    base.central_repository = current.central_repository.clone();
    let mut files = BTreeMap::new();
    for (path, bytes) in central_files {
        if !path.starts_with(".repo") && !base.projects.iter().any(|p| path.starts_with(&p.path)) {
            files.insert(root.join(path), bytes);
        }
    }
    for project in &mut base.projects {
        if !crate::manifest::full_commit(&project.revision) {
            return Err(failure(format!("repo_impact_base_unlocked: project {} ({}) revision {} must be a full commit object ID",project.name,project.path.display(),project.revision)));
        }
        let objects = project_objects(root, current, project)?;
        project.commit = resolve(&objects, &project.revision)?;
        if !project.commit.eq_ignore_ascii_case(&project.revision) {
            return Err(failure(format!(
                "repo_impact_base_unlocked: project {} revision is not a commit ID",
                project.path.display()
            )));
        }
        for (path, bytes) in tree(&objects, &project.commit)? {
            files.insert(root.join(&project.path).join(path), bytes);
        }
    }
    Ok(Baseline {
        composition: base,
        files,
    })
}
fn resolve_native(root: &Path, manifest: PathBuf) -> Result<WorkspaceComposition, Error> {
    WorkspaceComposition::resolve(root, manifest).map_err(|error| {
        if error.to_string().contains("must be a full commit ID") {
            failure(format!("repo_impact_base_unlocked: {error}"))
        } else {
            error
        }
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::git::local_bytes as git;
    #[test]
    fn historical_objects_preserve_binary_bytes_and_unusual_file_names() {
        let directory = tempfile::tempdir().expect("temporary repository");
        let root = directory.path();
        git(root, &["init", "-q", "-b", "main"]).expect("initialize repository");
        let path = Path::new("binary with tab\tand newline\n.bin");
        let original = b"historical\0bytes\r\n\xff";
        std::fs::write(root.join(path), original).expect("binary fixture");
        std::fs::write(root.join("empty.txt"), b"").expect("empty fixture");
        git(root, &["add", "."]).expect("stage fixture");
        git(
            root,
            &[
                "-c",
                "user.name=Impact Test",
                "-c",
                "user.email=impact@example.invalid",
                "-c",
                "commit.gpgSign=false",
                "commit",
                "-qm",
                "baseline",
            ],
        )
        .expect("commit fixture");
        let head = resolve(root, "HEAD").expect("baseline commit");
        assert!(
            resolve(root, "").is_err(),
            "an empty selector must not become HEAD"
        );
        std::fs::write(root.join(path), b"live bytes").expect("uncommitted edit");
        let files = tree(root, &head).expect("read historical objects");
        assert_eq!(files[path].as_slice(), original);
        assert!(files[Path::new("empty.txt")].is_empty());
        assert_eq!(
            std::fs::read(root.join(path)).expect("live source"),
            b"live bytes"
        );
        assert_eq!(resolve(root, "HEAD").expect("unchanged HEAD"), head);
    }

    #[cfg(unix)]
    #[test]
    fn historical_symlinks_resolve_inside_their_own_tree_and_reject_escape() {
        use std::os::unix::fs::symlink;
        let files = BTreeMap::from([
            (
                PathBuf::from("data.bin"),
                Arc::new(b"historical\0data".to_vec()),
            ),
            (
                PathBuf::from("nested/alias"),
                Arc::new(b"../data.bin".to_vec()),
            ),
            (PathBuf::from("escape"), Arc::new(b"../outside".to_vec())),
            (PathBuf::from("cycle-a"), Arc::new(b"cycle-b".to_vec())),
            (PathBuf::from("cycle-b"), Arc::new(b"cycle-a".to_vec())),
        ]);
        let links = BTreeSet::from([
            PathBuf::from("nested/alias"),
            PathBuf::from("escape"),
            PathBuf::from("cycle-a"),
            PathBuf::from("cycle-b"),
        ]);
        assert_eq!(
            link_contents(
                Path::new("nested/alias"),
                &files,
                &links,
                &mut BTreeSet::new()
            )
            .expect("owned target")
            .expect("file source")
            .as_slice(),
            b"historical\0data"
        );
        assert!(
            link_contents(Path::new("escape"), &files, &links, &mut BTreeSet::new())
                .expect_err("reject escape")
                .to_string()
                .contains("escapes")
        );
        assert!(
            link_contents(Path::new("cycle-a"), &files, &links, &mut BTreeSet::new())
                .expect_err("reject cycle")
                .to_string()
                .contains("cycle")
        );
        // Exercise the Git mode and object path as well as target resolution.
        let directory = tempfile::tempdir().expect("temporary repository");
        let root = directory.path();
        git(root, &["init", "-q"]).expect("initialize repository");
        std::fs::write(root.join("data.bin"), b"owned data").expect("owned fixture");
        symlink("data.bin", root.join("alias.bin")).expect("symlink fixture");
        symlink(".", root.join("directory-link")).expect("directory symlink fixture");
        symlink("directory-link/data.bin", root.join("chained-alias.bin"))
            .expect("symlink through directory fixture");
        git(root, &["add", "."]).expect("stage fixture");
        git(
            root,
            &[
                "-c",
                "user.name=Impact Test",
                "-c",
                "user.email=impact@example.invalid",
                "-c",
                "commit.gpgSign=false",
                "commit",
                "-qm",
                "baseline",
            ],
        )
        .expect("commit fixture");
        let historical = tree(root, "HEAD").expect("historical symlink");
        assert!(!historical.contains_key(Path::new("directory-link")));
        assert_eq!(
            historical[Path::new("chained-alias.bin")].as_slice(),
            b"owned data"
        );
        assert_eq!(historical[Path::new("alias.bin")].as_slice(), b"owned data");
    }
}
