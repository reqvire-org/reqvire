//! Disposable Repo clients for Central branch contexts. All preparation and
//! retention uses local objects; this boundary never performs a network sync.
use crate::{mcp_process::git, mcp_session::require_clean};
use reqvire::{
    error::ReqvireError,
    repo_composition::{owner_cache_key, CompositionRecord},
    repo_workspace::{normalize_url, resolve_revision, RepoProject, RepoWorkspace},
};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn error(message: impl Into<String>) -> ReqvireError {
    ReqvireError::ProcessError(message.into())
}
fn path_text(path: &Path) -> Result<&str, ReqvireError> {
    path.to_str().ok_or_else(|| error("Non-UTF8 Repo path"))
}
fn cache_path(common: &Path, project: &RepoProject) -> PathBuf {
    common
        .join("reqvire-repo-objects")
        .join(reqvire::utils::hash_content(&format!(
            "{}:{}",
            project.name,
            normalize_url(&project.fetch_url)
        )))
}

fn prepare_native_manifest(
    source: &Path,
    target: &Path,
    _common: &Path,
    view: &RepoWorkspace,
    creating: bool,
) -> Result<(), ReqvireError> {
    let metadata = target.join(".repo");
    let head = git(target, &["rev-parse", "HEAD"])?;
    let record = match CompositionRecord::at_commit(target, &head, &view.selection)? {
        Some(record) => record,
        None if head == view.central_commit => CompositionRecord::capture(view)?,
        None => return Err(error(format!(
            "Native manifest composition at {head} is unavailable; record its owning revisions in .reqvire-composition.json before preparing this branch"
        ))),
    };
    let branch = git(target, &["symbolic-ref", "--short", "HEAD"])?;
    let mut owners = std::collections::BTreeSet::new();
    for selected in &record.sources {
        if selected.owner.as_os_str().is_empty() || !owners.insert(selected.owner.clone()) {
            continue;
        }
        let objects = record.object_source(source, view, selected)?;
        let owner_branch = if !selected.repository.is_empty()
            && normalize_url(&selected.repository) == normalize_url(&view.central_repository)
        {
            format!("reqvire/manifests/{branch}")
        } else {
            branch.clone()
        };
        if creating
            && git(
                &objects,
                &[
                    "show-ref",
                    "--verify",
                    &format!("refs/heads/{owner_branch}"),
                ],
            )
            .is_ok()
        {
            return Err(error(format!(
                "Manifest owner {} branch {owner_branch} already exists",
                selected.owner.display()
            )));
        }
        let destination = target.join(&selected.owner);
        fs::create_dir_all(
            destination
                .parent()
                .ok_or_else(|| error("Manifest owner has no parent"))?,
        )?;
        git(
            target,
            &[
                "clone",
                "--quiet",
                "--shared",
                "--no-checkout",
                "--",
                path_text(&objects)?,
                path_text(&destination)?,
            ],
        )?;
        if selected.owner == Path::new(".repo/manifests") {
            fs::rename(destination.join(".git"), metadata.join("manifests.git"))?;
            fs::write(destination.join(".git"), "gitdir: ../manifests.git\n")?;
        }
        if !selected.repository.is_empty() {
            git(
                &destination,
                &["remote", "set-url", "origin", &selected.repository],
            )?;
        }
        git(
            &destination,
            &[
                "checkout",
                "--quiet",
                "-B",
                &owner_branch,
                selected
                    .commit
                    .as_deref()
                    .expect("validated owner revision"),
            ],
        )?;
    }
    for (path, bytes) in record.historical_inputs(source, &head, view)? {
        let destination = target.join(path);
        if fs::read(&destination).ok().as_deref() != Some(bytes.as_slice()) {
            return Err(error(format!(
                "Prepared manifest {} differs from its recorded owner tree",
                destination.display()
            )));
        }
    }
    let selected = record
        .manifest
        .strip_prefix(".repo/manifests")
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| target.join(&record.manifest));
    let selected = quick_xml::escape::escape(path_text(&selected)?);
    fs::write(
        metadata.join("manifest.xml"),
        format!("<manifest><include name=\"{selected}\"/></manifest>\n"),
    )?;
    Ok(())
}

pub fn prepare(
    source: &Path,
    target: &Path,
    common: &Path,
    manifest: &Path,
    creating: bool,
) -> Result<(), ReqvireError> {
    if target.join(".repo").exists() {
        return Ok(());
    }
    let source_view = RepoWorkspace::load(source, manifest.to_owned())?;
    let metadata = target.join(".repo");
    fs::create_dir(&metadata)?;
    // Private metadata, including private writable Git configuration and refs.
    // --shared borrows only immutable object availability from local storage.
    git(
        target,
        &[
            "clone",
            "--quiet",
            "--shared",
            "--",
            path_text(&source.join(".repo/repo"))?,
            path_text(&metadata.join("repo"))?,
        ],
    )?;
    let manifests = metadata.join("manifests");
    if source_view.uses_repo_managed_inputs() {
        prepare_native_manifest(source, target, common, &source_view, creating)?;
    } else {
        git(
            target,
            &[
                "clone",
                "--quiet",
                "--shared",
                "--no-checkout",
                "--",
                path_text(source)?,
                path_text(&manifests)?,
            ],
        )?;
        fs::rename(manifests.join(".git"), metadata.join("manifests.git"))?;
        fs::write(manifests.join(".git"), "gitdir: ../manifests.git\n")?;
        let manifest_remote = git(
            &source.join(".repo/manifests"),
            &["remote", "get-url", "origin"],
        )?;
        git(
            &manifests,
            &["remote", "set-url", "origin", &manifest_remote],
        )?;
        let selected = quick_xml::escape::escape(path_text(manifest)?);
        fs::write(
            metadata.join("manifest.xml"),
            format!("<manifest><include name=\"{selected}\"/></manifest>\n"),
        )?;
    }
    let selection = RepoWorkspace::resolve(target, manifest.to_owned())?;
    for project in &selection.projects {
        let available = source_view
            .projects
            .iter()
            .find(|p| {
                p.name == project.name
                    && normalize_url(&p.fetch_url) == normalize_url(&project.fetch_url)
            })
            .map(|p| source.join(&p.path));
        let cached = cache_path(common, project);
        let (objects, commit) = if let Some(available) = available {
            let mut local_project = project.clone();
            if let Some(owner) = source_view
                .projects
                .iter()
                .find(|p| p.path == available.strip_prefix(source).unwrap_or(&available))
            {
                local_project.remote.clone_from(&owner.remote);
            }
            match resolve_revision(&available, &local_project) {
                Ok(commit) => (available, commit),
                Err(failure) => {
                    if !cached.exists() {
                        return Err(failure);
                    }
                    let commit = resolve_revision(&cached, project)?;
                    (cached, commit)
                }
            }
        } else if cached.exists() {
            let commit = resolve_revision(&cached, project)?;
            (cached, commit)
        } else {
            return Err(error(format!("Manifest {}, project {}: selected objects are unavailable locally; no fetch was attempted", manifest.display(), project.path.display())));
        };
        let destination = target.join(&project.path);
        let branch = git(target, &["symbolic-ref", "--short", "HEAD"])?;
        if creating
            && git(
                &objects,
                &["show-ref", "--verify", &format!("refs/heads/{branch}")],
            )
            .is_ok()
        {
            return Err(error(format!(
                "Project {} branch {branch} already exists",
                project.path.display()
            )));
        }
        fs::create_dir_all(
            destination
                .parent()
                .ok_or_else(|| error("Project has no parent"))?,
        )?;
        git(
            target,
            &[
                "clone",
                "--quiet",
                "--shared",
                "--no-checkout",
                "--origin",
                &project.remote,
                "--",
                path_text(&objects)?,
                path_text(&destination)?,
            ],
        )?;
        git(
            &destination,
            &["remote", "set-url", &project.remote, &project.fetch_url],
        )?;
        git(
            &destination,
            &["checkout", "--quiet", "-B", &branch, &commit],
        )?;
    }
    RepoWorkspace::load(target, manifest.to_owned())?;
    Ok(())
}

/// Copy required objects into owning-repository storage before removing the
/// disposable checkouts. A failed transfer leaves the entire checkout intact.
pub fn retain(root: &Path, common: &Path, manifest: &Path) -> Result<RepoWorkspace, ReqvireError> {
    let view = RepoWorkspace::load(root, manifest.to_owned())?;
    for project in &view.projects {
        let checkout = root.join(&project.path);
        require_clean(&checkout)?;
        let cache = cache_path(common, project);
        if !cache.exists() {
            fs::create_dir_all(cache.parent().ok_or_else(|| error("Cache has no parent"))?)?;
            git(common, &["init", "--quiet", "--bare", path_text(&cache)?])?;
        }
        let reference = format!("refs/reqvire/retained/{}", project.commit);
        git(
            &cache,
            &[
                "fetch",
                "--quiet",
                "--no-tags",
                "--",
                path_text(&checkout)?,
                &format!("{}:{reference}", project.commit),
                "refs/reqvire/commits/*:refs/reqvire/commits/*",
            ],
        )?;
        if git(&cache, &["rev-parse", "--verify", &reference])? != project.commit {
            return Err(error(format!(
                "Cannot retain component {} before removal",
                project.path.display()
            )));
        }
    }
    let mut owners = std::collections::BTreeSet::new();
    for selected in view.manifest_sources.values() {
        if selected.root.as_os_str().is_empty() || !owners.insert(selected.root.clone()) {
            continue;
        }
        let checkout = root.join(&selected.root);
        require_clean(&checkout)?;
        let cache = common
            .join("reqvire-manifest-objects")
            .join(owner_cache_key(&selected.repository, &selected.root));
        if !cache.exists() {
            fs::create_dir_all(
                cache
                    .parent()
                    .ok_or_else(|| error("Manifest cache has no parent"))?,
            )?;
            git(common, &["init", "--quiet", "--bare", path_text(&cache)?])?;
        }
        if !selected.repository.is_empty() {
            git(
                &cache,
                &["config", "remote.origin.url", &selected.repository],
            )?;
        }
        let reference = format!("refs/reqvire/retained/{}", selected.commit);
        git(
            &cache,
            &[
                "fetch",
                "--quiet",
                "--no-tags",
                "--",
                path_text(&checkout)?,
                &format!("{}:{reference}", selected.commit),
            ],
        )?;
        if git(&cache, &["rev-parse", "--verify", &reference])? != selected.commit {
            return Err(error(format!(
                "Cannot retain manifest owner {} before removal",
                selected.root.display()
            )));
        }
    }
    Ok(view)
}

pub fn remove_checkouts(root: &Path, view: &RepoWorkspace) -> Result<(), ReqvireError> {
    for project in &view.projects {
        fs::remove_dir_all(root.join(&project.path))?;
    }
    fs::remove_dir_all(root.join(".repo"))?;
    Ok(())
}
