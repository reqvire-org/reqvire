//! Repository inventory shared by model processing and workspace operations.
use crate::{
    manifest::{selected_manifest, WorkspaceComposition},
    paths::canonicalize_existing,
    Error,
};
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    sync::Arc,
};
#[derive(Debug, Clone)]
pub struct Description {
    pub root: PathBuf,
    pub git_worktrees: Vec<PathBuf>,
    pub composition: Option<Arc<WorkspaceComposition>>,
}
pub fn discover(root: &Path) -> Result<Description, Error> {
    let root = canonicalize_existing(root)?;
    if let Some(manifest) = selected_manifest(&root)? {
        let owner = PathBuf::from(crate::git::find_root(&root)?);
        let composition = WorkspaceComposition::load(&root, manifest)?;
        let git_worktrees = std::iter::once(owner)
            .chain(composition.projects.iter().map(|p| root.join(&p.path)))
            .collect();
        return Ok(Description {
            root,
            git_worktrees,
            composition: Some(Arc::new(composition)),
        });
    }
    let mut roots = BTreeSet::new();
    if let Ok(owner) = crate::git::find_root(&root) {
        roots.insert(PathBuf::from(owner));
    } else {
        for entry in walkdir::WalkDir::new(&root)
            .into_iter()
            .filter_entry(|e| e.file_name() != ".git" && e.file_name() != ".repo")
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_dir())
        {
            if entry.path().join(".git").exists() {
                if let Ok(owner) = crate::git::find_root(entry.path()) {
                    let owner = PathBuf::from(owner);
                    if owner.starts_with(&root) {
                        roots.insert(owner);
                    }
                }
            }
        }
    }
    if roots.is_empty() {
        return Err(Error::GitCommandError(format!(
            "Workspace '{}' does not contain an eligible Git worktree",
            root.display()
        )));
    }
    Ok(Description {
        root,
        git_worktrees: roots.into_iter().collect(),
        composition: None,
    })
}
