//! Local-only branch-set preparation and retained objects for disposable contexts.
use crate::{
    manifest::{normalize_url, resolve_revision, WorkspaceComposition, WorkspaceProject},
    service::{git, require_clean},
    Error,
};
use std::{
    fs,
    path::{Path, PathBuf},
};
fn error(message: impl Into<String>) -> Error {
    Error::ProcessError(message.into())
}
fn path_text(path: &Path) -> Result<&str, Error> {
    path.to_str().ok_or_else(|| error("Non-UTF8 Repo path"))
}
fn cache_path(common: &Path, project: &WorkspaceProject) -> PathBuf {
    common
        .join("reqvire-repo-objects")
        .join(crate::service::hash(
            format!("{}:{}", project.name, normalize_url(&project.fetch_url)).as_bytes(),
        ))
}

pub fn prepare(
    source: &Path,
    target: &Path,
    common: &Path,
    manifest: &Path,
    creating: bool,
) -> Result<(), Error> {
    let source_view = WorkspaceComposition::load(source, manifest.to_owned())?;
    let selection = WorkspaceComposition::resolve(target, manifest.to_owned())?;
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
        if destination.exists() {
            return Err(error(format!(
                "Existing checkout {} must be opened as its prepared composition",
                destination.display()
            )));
        }
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
    WorkspaceComposition::load(target, manifest.to_owned())?;
    Ok(())
}

/// Copy required objects into owning-repository storage before removing the
/// disposable checkouts. A failed transfer leaves the entire checkout intact.
pub fn retain(root: &Path, common: &Path, manifest: &Path) -> Result<WorkspaceComposition, Error> {
    let view = WorkspaceComposition::load(root, manifest.to_owned())?;
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
    Ok(view)
}

pub fn remove_checkouts(root: &Path, view: &WorkspaceComposition) -> Result<(), Error> {
    for project in &view.projects {
        fs::remove_dir_all(root.join(&project.path))?;
    }
    Ok(())
}
