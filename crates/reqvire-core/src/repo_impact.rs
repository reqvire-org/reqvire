//! Complete, read-only historical Repo compositions for CLI and accepted MCP reads.
use crate::{
    error::ReqvireError,
    exclusions::ExclusionSet,
    git_commands::{local_bytes as git, local_text as text, run_local},
    mutation_io::{SnapshotFiles, SnapshotGuard},
    repo_composition::CompositionRecord,
    repo_workspace::{self, RepoProject, RepoWorkspace},
    workspace::WorkspaceScope,
    ModelManager,
};
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Component, Path, PathBuf},
    sync::Arc,
};

fn failure(message: impl Into<String>) -> ReqvireError {
    ReqvireError::ProcessError(message.into())
}

fn is_root_owner(path: &Path) -> bool {
    path.as_os_str().is_empty()
}

#[derive(Debug, Clone, Serialize)]
pub struct ManifestRevision {
    pub path: PathBuf,
    pub commit: String,
    #[serde(skip_serializing_if = "is_root_owner")]
    pub owner: PathBuf,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub repository: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct ProjectRevision {
    pub name: String,
    pub path: PathBuf,
    pub repository: String,
    pub commit: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct WorkspaceRevision {
    pub central_commit: String,
    pub manifest_inputs: Vec<ManifestRevision>,
    pub projects: Vec<ProjectRevision>,
    #[serde(skip)]
    pub central_repository: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct Comparison {
    pub workspace_mode: &'static str,
    pub base: WorkspaceRevision,
    pub current: WorkspaceRevision,
}
impl WorkspaceRevision {
    pub fn from_repo(repo: &RepoWorkspace) -> Self {
        let mut projects: Vec<_> = repo
            .projects
            .iter()
            .map(|p| ProjectRevision {
                name: p.name.clone(),
                path: p.path.clone(),
                repository: p.fetch_url.clone(),
                commit: p.commit.clone(),
            })
            .collect();
        projects.sort_by(|a, b| {
            (&a.path, &a.name, &a.repository).cmp(&(&b.path, &b.name, &b.repository))
        });
        Self {
            central_commit: repo.central_commit.clone(),
            central_repository: repo.central_repository.clone(),
            manifest_inputs: repo
                .manifest_sources
                .iter()
                .map(|(path, source)| ManifestRevision {
                    path: path.clone(),
                    commit: source.commit.clone(),
                    owner: source.root.clone(),
                    repository: if source.root.as_os_str().is_empty() {
                        String::new()
                    } else {
                        source.repository.clone()
                    },
                })
                .collect(),
            projects,
        }
    }

    /// Web links retain the owning repository's path and revision. Unsupported
    /// transports stay workspace-relative instead of borrowing Central's URL.
    pub fn source_url(&self, identifier: &str) -> String {
        let relative;
        let identifier = if Path::new(identifier).is_absolute() {
            relative = crate::workspace::workspace_root().ok().and_then(|root| {
                Path::new(identifier)
                    .strip_prefix(root)
                    .ok()
                    .map(|p| p.to_string_lossy().into_owned())
            });
            relative.as_deref().unwrap_or(identifier)
        } else {
            identifier
        };
        let (file, fragment) = identifier.split_once('#').unwrap_or((identifier, ""));
        let owner = self
            .projects
            .iter()
            .filter_map(|p| {
                Path::new(file)
                    .strip_prefix(&p.path)
                    .ok()
                    .map(|path| (p, path))
            })
            .max_by_key(|(p, _)| p.path.components().count());
        let (repository, commit, path) = match owner {
            Some((p, path)) => (
                &p.repository,
                &p.commit,
                path.to_string_lossy().into_owned(),
            ),
            None => (
                &self.central_repository,
                &self.central_commit,
                file.to_owned(),
            ),
        };
        let identity = repo_workspace::normalize_url(repository);
        if identity.starts_with("github.com/") {
            format!(
                "https://{identity}/blob/{commit}/{path}{}",
                if fragment.is_empty() {
                    String::new()
                } else {
                    format!("#{fragment}")
                }
            )
        } else {
            identifier.to_owned()
        }
    }
}

fn resolve(directory: &Path, reference: &str) -> Result<String, ReqvireError> {
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
fn relative(path: &Path) -> Result<(), ReqvireError> {
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
fn tree(directory: &Path, commit: &str) -> Result<BTreeMap<PathBuf, Arc<Vec<u8>>>, ReqvireError> {
    let mut reader = crate::git_commands::GitReader::new(directory, |args, input| {
        run_local(directory, args, input)
    });
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
) -> Result<Option<Arc<Vec<u8>>>, ReqvireError> {
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

fn link_target(path: &Path, bytes: &[u8]) -> Result<PathBuf, ReqvireError> {
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

fn materialize(root: &Path, files: &BTreeMap<PathBuf, Arc<Vec<u8>>>) -> Result<(), ReqvireError> {
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

fn project_objects(
    root: &Path,
    current: &RepoWorkspace,
    project: &RepoProject,
) -> Result<PathBuf, ReqvireError> {
    let identity = repo_workspace::normalize_url(&project.fetch_url);
    let mut candidates: Vec<PathBuf> = current
        .projects
        .iter()
        .filter(|p| {
            p.name == project.name && repo_workspace::normalize_url(&p.fetch_url) == identity
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
    let common = crate::git_commands::local_text(
        root,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?;
    candidates.push(PathBuf::from(common).join("reqvire-repo-objects").join(
        crate::utils::hash_content(&format!("{}:{}", project.name, identity)),
    ));
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
                        .is_ok_and(|url| repo_workspace::normalize_url(&url) == identity)
                })
            });
        if matches && resolve(&candidate, &project.revision).is_ok() {
            return Ok(candidate);
        }
    }
    Err(failure(format!("repo_impact_base_unavailable: project {} ({}) revision {} is unavailable locally; no fetch was attempted",project.name,project.path.display(),project.revision)))
}

fn manifest_history_unavailable(path: &Path) -> ReqvireError {
    failure(format!(
        "repo_impact_manifest_history_unavailable: {} has no identified version in the selected composition; commit its manifest-owner revision association in .reqvire-composition.json or restore the recorded local objects",
        path.display()
    ))
}

pub fn historical_model(
    scope: &WorkspaceScope,
    current: &RepoWorkspace,
    git_commit: &str,
    exclusions: &ExclusionSet,
) -> Result<(ModelManager, Comparison), ReqvireError> {
    let root = &scope.root;
    let central = resolve(
        root,
        if git_commit == "HEAD" {
            &current.central_commit
        } else {
            git_commit
        },
    )?;
    let central_files = tree(root, &central)?;
    let mut record = CompositionRecord::at_commit(root, &central, &current.selection)?;
    if record.is_none() && current.uses_repo_managed_inputs() {
        let mut captured = CompositionRecord::capture(current)
            .map_err(|_| manifest_history_unavailable(&current.manifest))?;
        if git_commit != "HEAD" {
            // Legacy same-history native selections can be inferred only when
            // the selected root tree actually versions each source path.
            for source in &mut captured.sources {
                if !source.owner.as_os_str().is_empty() {
                    if source.repository.is_empty()
                        || repo_workspace::normalize_url(&source.repository)
                            != repo_workspace::normalize_url(&current.central_repository)
                        || !central_files.contains_key(&source.source)
                    {
                        return Err(manifest_history_unavailable(&source.path));
                    }
                    source.commit = Some(central.clone());
                }
            }
        }
        record = Some(captured);
    }
    if record.is_none() {
        for (path, owner) in &current.manifest_sources {
            if !owner.tracked || !owner.root.as_os_str().is_empty() {
                return Err(manifest_history_unavailable(path));
            }
        }
    }
    let temporary = tempfile::tempdir()?;
    let staged = temporary.path();
    let mut base;
    {
        let _suspended = SnapshotGuard::suspend();
        materialize(staged, &central_files)?;
        if let Some(record) = &record {
            for (path, bytes) in record.historical_inputs(root, &central, current)? {
                let destination = staged.join(path);
                std::fs::create_dir_all(destination.parent().expect("validated source path"))?;
                std::fs::write(destination, bytes)?;
            }
        }
        std::fs::create_dir_all(staged.join(".repo/manifests"))?;
        git(
            staged,
            &[
                "init",
                "--quiet",
                "--bare",
                &staged.join(".repo/manifests.git").to_string_lossy(),
            ],
        )?;
        let config = current
            .inputs
            .get(Path::new(".repo/manifests.git/config"))
            .ok_or_else(|| failure("Captured manifest repository configuration is unavailable"))?;
        std::fs::write(staged.join(".repo/manifests.git/config"), config)?;
        std::fs::write(
            staged.join(".repo/manifests/.git"),
            "gitdir: ../manifests.git\n",
        )?;
        // Repo's wrapper is reconstructed administration; XML sources remain
        // the immutable files read from their recorded owning Git trees.
        let entry = record.as_ref().map_or_else(
            || "<manifest/>\n".to_owned(),
            |record| {
                let selected = record
                    .manifest
                    .strip_prefix(".repo/manifests")
                    .map(Path::to_path_buf)
                    .unwrap_or_else(|_| staged.join(&record.manifest));
                format!(
                    "<manifest><include name=\"{}\"/></manifest>\n",
                    quick_xml::escape::escape(selected.to_string_lossy())
                )
            },
        );
        std::fs::write(staged.join(".repo/manifest.xml"), entry)?;
        if let Some(source) = record
            .as_ref()
            .and_then(|record| record.sources.iter().find(|s| s.path == record.manifest))
            .filter(|source| !source.repository.is_empty())
        {
            git(
                staged,
                &[
                    "--git-dir",
                    ".repo/manifests.git",
                    "config",
                    "remote.origin.url",
                    &source.repository,
                ],
            )?;
        }
        base = RepoWorkspace::resolve_with_repo(
            staged,
            current.selection.clone(),
            &root.join(".repo/repo"),
        )?;
    }
    base.central_commit = central.clone();
    base.central_repository = current.central_repository.clone();
    for path in base.manifest_input_paths() {
        let source = record
            .as_ref()
            .and_then(|record| record.sources.iter().find(|s| s.path == path));
        if source.is_none() && (record.is_some() || !central_files.contains_key(&path)) {
            return Err(manifest_history_unavailable(&path));
        }
        base.manifest_sources.insert(
            path,
            repo_workspace::ManifestSource {
                root: source.map(|s| s.owner.clone()).unwrap_or_default(),
                commit: source
                    .and_then(|s| s.commit.clone())
                    .unwrap_or_else(|| central.clone()),
                tracked: true,
                repository: source
                    .map(|s| s.repository.clone())
                    .unwrap_or_else(|| current.central_repository.clone()),
            },
        );
    }
    if record
        .as_ref()
        .is_some_and(|record| record.sources.len() != base.manifest_sources.len())
    {
        return Err(failure(
            "Historical composition record contains unconsumed manifest sources",
        ));
    }
    let mut files = BTreeMap::new();
    for (path, bytes) in central_files {
        if !path.starts_with(".repo") && !base.projects.iter().any(|p| path.starts_with(&p.path)) {
            files.insert(root.join(path), bytes);
        }
    }
    for project in &mut base.projects {
        if !matches!(project.revision.len(), 40 | 64)
            || !project.revision.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err(failure(format!("repo_impact_base_unlocked: project {} ({}) revision {} must be a full commit object ID",project.name,project.path.display(),project.revision)));
        }
        let objects = project_objects(root, current, project)?;
        project.commit = resolve(&objects, &project.revision)?;
        if !project.commit.eq_ignore_ascii_case(&project.revision) {
            return Err(failure(format!(
                "repo_impact_base_unlocked: project {} ({}) revision {} is not a commit object ID",
                project.name,
                project.path.display(),
                project.revision
            )));
        }
        for (path, bytes) in tree(&objects, &project.commit)? {
            files.insert(root.join(&project.path).join(path), bytes);
        }
    }
    let comparison = Comparison {
        workspace_mode: "repo",
        base: WorkspaceRevision::from_repo(&base),
        current: WorkspaceRevision::from_repo(current),
    };
    let historical = WorkspaceScope {
        root: root.clone(),
        git_worktrees: std::iter::once(root.clone())
            .chain(base.projects.iter().map(|p| root.join(&p.path)))
            .collect(),
        repo: Some(Arc::new(base)),
    };
    let snapshot = SnapshotFiles {
        root: root.clone(),
        workspace: Some(Arc::new(historical)),
        files: Arc::new(files),
        executable: Arc::new(BTreeSet::new()),
    };
    let _view = snapshot.enter_historical();
    let exclusions = exclusions.rebuilt();
    let mut model = ModelManager::new();
    match model.parse_and_validate_with_mode(None, &exclusions, false) {
        Ok(_) => {}
        Err(ReqvireError::ValidationError(_) | ReqvireError::ValidationDiagnostics { .. }) => {
            model.parse_and_validate_with_mode(None, &exclusions, true)?;
        }
        Err(error) => return Err(error),
    }
    Ok((model, comparison))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_use_the_side_and_repository_that_owns_each_source() {
        let mut base = WorkspaceRevision {
            central_commit: "central-base".into(),
            central_repository: "git@github.com:product/central.git".into(),
            manifest_inputs: Vec::new(),
            projects: vec![
                ProjectRevision {
                    name: "server".into(),
                    path: "repositories/server".into(),
                    repository: "ssh://git@github.com/product/server.git".into(),
                    commit: "server-base".into(),
                },
                ProjectRevision {
                    name: "docs".into(),
                    path: "repositories/docs".into(),
                    repository: "file:///local/docs".into(),
                    commit: "docs-base".into(),
                },
            ],
        };
        assert_eq!(
            base.source_url("Model.md#root"),
            "https://github.com/product/central/blob/central-base/Model.md#root"
        );
        assert_eq!(
            base.source_url("repositories/server/specifications/API.md#contract"),
            "https://github.com/product/server/blob/server-base/specifications/API.md#contract"
        );
        assert_eq!(
            base.source_url("repositories/docs/Guide.txt"),
            "repositories/docs/Guide.txt"
        );
        // A common string prefix is not a repository ownership boundary.
        assert_eq!(base.source_url("repositories/server-other/Model.md#root"),"https://github.com/product/central/blob/central-base/repositories/server-other/Model.md#root");
        let previous = base.clone();
        base.projects[0].commit = "server-current".into();
        assert!(base
            .source_url("repositories/server/API.md")
            .contains("/server-current/"));
        assert!(previous
            .source_url("repositories/server/API.md")
            .contains("/server-base/"));
    }

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
