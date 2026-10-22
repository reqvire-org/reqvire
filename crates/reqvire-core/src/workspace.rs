use crate::error::ReqvireError;
use crate::git_commands;
use rustc_hash::FxHashSet;
use std::path::{Component, Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceScope {
    pub root: PathBuf,
    pub git_worktrees: Vec<PathBuf>,
    pub repo: Option<std::sync::Arc<crate::repo_workspace::RepoWorkspace>>,
}

impl WorkspaceScope {
    pub fn discover() -> Result<Self, ReqvireError> {
        Self::discover_from(workspace_root()?)
    }

    pub fn discover_from(root: PathBuf) -> Result<Self, ReqvireError> {
        let root = normalize_existing_path(root)?;
        if let Some(scope) = crate::mutation_io::workspace_scope() {
            if scope.root == root {
                return Ok(scope);
            }
        }
        if let Some(manifest) = crate::repo_workspace::selected_manifest(&root)? {
            let owner = crate::repo_workspace::git_text(&root, &["rev-parse", "--show-toplevel"])?;
            if Path::new(&owner).canonicalize()? != root {
                return Err(ReqvireError::PathError(
                    "Repo mode requires Central's Git worktree at the workspace root".into(),
                ));
            }
            let repo = crate::repo_workspace::RepoWorkspace::load(&root, manifest)?;
            let mut git_worktrees = vec![root.clone()];
            git_worktrees.extend(repo.projects.iter().map(|p| root.join(&p.path)));
            return Ok(Self {
                root,
                git_worktrees,
                repo: Some(std::sync::Arc::new(repo)),
            });
        }
        let mut roots = Vec::new();
        let mut seen = FxHashSet::default();

        if let Ok(containing_root) = git_commands::find_git_repo_root(&root) {
            let containing_root = normalize_existing_path(PathBuf::from(containing_root))?;
            if seen.insert(containing_root.clone()) {
                roots.push(containing_root);
            }
            roots.sort();
            return Ok(Self {
                root,
                git_worktrees: roots,
                repo: None,
            });
        }

        for entry in WalkDir::new(&root)
            .into_iter()
            .filter_entry(|entry| entry.file_name() != ".git")
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_dir())
        {
            let path = entry.path();
            if !path.join(".git").exists() {
                continue;
            }
            let Ok(git_root) = git_commands::find_git_repo_root(&path.to_path_buf()) else {
                continue;
            };
            let git_root = normalize_existing_path(PathBuf::from(git_root))?;
            if git_root.starts_with(&root) && seen.insert(git_root.clone()) {
                roots.push(git_root);
            }
        }

        roots.sort();

        if roots.is_empty() {
            return Err(ReqvireError::GitCommandError(format!(
                "Workspace '{}' does not contain an eligible Git worktree",
                root.display()
            )));
        }

        Ok(Self {
            root,
            git_worktrees: roots,
            repo: None,
        })
    }

    pub fn scan_roots(&self) -> Vec<PathBuf> {
        if self
            .git_worktrees
            .iter()
            .any(|git_root| self.root.starts_with(git_root))
        {
            vec![self.root.clone()]
        } else {
            self.git_worktrees.clone()
        }
    }

    pub fn is_eligible_path(&self, path: &Path) -> bool {
        let absolute = absolute_logical_path(path);
        if self.repo.is_some() {
            let Some(owner) = self.git_worktree_for_path(&absolute) else {
                return false;
            };
            if absolute.strip_prefix(&self.root).is_ok_and(|p| {
                p.components()
                    .any(|c| matches!(c, Component::Normal(n) if n == ".repo" || n == ".git"))
            }) {
                return false;
            }
            let mut parent = absolute.parent();
            while let Some(path) = parent {
                if path == owner {
                    return true;
                }
                if path.join(".git").exists() {
                    return false;
                }
                parent = path.parent();
            }
            return false;
        }
        absolute.starts_with(&self.root)
            && self
                .git_worktrees
                .iter()
                .any(|git_root| absolute.starts_with(git_root))
    }

    pub fn git_worktree_for_path(&self, path: &Path) -> Option<&PathBuf> {
        let absolute = absolute_logical_path(path);
        self.git_worktrees
            .iter()
            .filter(|git_root| absolute.starts_with(*git_root))
            .max_by_key(|git_root| git_root.components().count())
    }

    pub fn owned_files(&self) -> Result<Vec<PathBuf>, ReqvireError> {
        let mut files = std::collections::BTreeSet::new();
        for owner in &self.git_worktrees {
            let paths = crate::repo_workspace::git_text(
                owner,
                &[
                    "ls-files",
                    "-z",
                    "--cached",
                    "--others",
                    "--exclude-standard",
                ],
            )?;
            for relative in paths.split('\0').filter(|p| !p.is_empty()) {
                let path = owner.join(relative);
                if self.git_worktree_for_path(&path) == Some(owner)
                    && self.is_eligible_path(&path)
                    && path.is_file()
                {
                    if !path.canonicalize()?.starts_with(owner) {
                        return Err(ReqvireError::PathError(format!(
                            "Source symlink escapes repository: {}",
                            path.display()
                        )));
                    }
                    files.insert(path);
                }
            }
        }
        Ok(files.into_iter().collect())
    }
}

pub fn workspace_root() -> Result<PathBuf, ReqvireError> {
    let current = std::env::current_dir().map_err(ReqvireError::IoError)?;
    normalize_existing_path(current)
}

pub fn workspace_relative_path(path: &Path) -> Result<PathBuf, ReqvireError> {
    let root = workspace_root()?;
    let absolute = absolute_logical_path(path);
    absolute
        .strip_prefix(&root)
        .map(PathBuf::from)
        .map_err(|_| {
            ReqvireError::PathError(format!(
                "Failed to determine workspace-relative path: {} is outside {}",
                absolute.display(),
                root.display()
            ))
        })
}

pub fn absolute_logical_path(path: &Path) -> PathBuf {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    };
    normalize_components(&absolute)
}

fn normalize_existing_path(path: PathBuf) -> Result<PathBuf, ReqvireError> {
    path.canonicalize()
        .map_err(|error| ReqvireError::PathError(error.to_string()))
}

fn normalize_components(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Normal(part) => normalized.push(part),
        }
    }
    normalized
}
