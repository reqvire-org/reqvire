//! Disposable Repo clients for Central branch contexts. All preparation and
//! retention uses local objects; this boundary never performs a network sync.
use crate::{mcp_process::git, mcp_session::require_clean};
use reqvire::{
    error::ReqvireError,
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

pub fn prepare(
    source: &Path,
    target: &Path,
    common: &Path,
    manifest: &Path,
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
        git(&destination, &["checkout", "--quiet", "--detach", &commit])?;
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
        let reference = format!("refs/reqvire/commits/{}", project.commit);
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
    Ok(view)
}

pub fn remove_checkouts(root: &Path, view: &RepoWorkspace) -> Result<(), ReqvireError> {
    for project in &view.projects {
        fs::remove_dir_all(root.join(&project.path))?;
    }
    fs::remove_dir_all(root.join(".repo"))?;
    Ok(())
}
