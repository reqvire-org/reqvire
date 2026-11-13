//! Complete, read-only historical Repo compositions for CLI and accepted MCP reads.
use crate::{
    error::ReqvireError,
    exclusions::ExclusionSet,
    mutation_io::{SnapshotFiles, SnapshotGuard},
    repo_workspace::{self, RepoWorkspace},
    workspace::WorkspaceScope,
    ModelManager,
};
use serde::Serialize;
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug, Clone, Serialize)]
pub struct ManifestRevision {
    pub path: PathBuf,
    pub commit: String,
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
                .manifest_input_paths()
                .into_iter()
                .map(|path| ManifestRevision {
                    path,
                    commit: repo.central_commit.clone(),
                })
                .collect(),
            projects,
        }
    }

    /// Web links retain the owning repository's path and revision. Unsupported
    /// transports stay workspace-relative instead of borrowing the root URL.
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

pub fn historical_model(
    scope: &WorkspaceScope,
    current: &RepoWorkspace,
    git_commit: &str,
    exclusions: &ExclusionSet,
) -> Result<(ModelManager, Comparison), ReqvireError> {
    let root = &scope.root;
    let reconstruction = {
        let _suspended = SnapshotGuard::suspend();
        reqvire_workspace::baseline::read(root, current, git_commit)?
    };
    let base = reconstruction.composition;
    let files = reconstruction.files;
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
        files,
        executable: Arc::new(BTreeSet::new()),
    };
    let view = snapshot.enter_historical(Some(Arc::new(reconstruction.sources)));
    let exclusions = exclusions.rebuilt();
    let mut model = ModelManager::new();
    match model.parse_and_validate_with_mode(None, &exclusions, false) {
        Ok(_) => {}
        Err(ReqvireError::ValidationError(_) | ReqvireError::ValidationDiagnostics { .. }) => {
            model.parse_and_validate_with_mode(None, &exclusions, true)?;
        }
        Err(error) => return Err(error),
    }
    view.check_historical_sources()?;
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
}
