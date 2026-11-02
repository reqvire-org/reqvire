//! Composition ownership, branch provenance and recovery outcomes.
use crate::{
    git::local_text as git_text,
    locks::{lock_file, FileLock},
    service::require_clean,
    Error, WorkspaceComposition,
};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
fn error(s: impl Into<String>) -> Error {
    Error::ProcessError(s.into())
}
struct Checkout {
    head: Option<String>,
    branch: Option<String>,
}
impl Checkout {
    fn read(root: &Path) -> Result<Self, Error> {
        Ok(Self {
            head: Some(git_text(root, &["rev-parse", "--verify", "HEAD"])?),
            branch: git_text(root, &["symbolic-ref", "--quiet", "HEAD"]).ok(),
        })
    }
}
fn git_operation_in_progress(root: &Path) -> Result<bool, Error> {
    let mut reader = crate::git::GitReader::new(root, |a, b| crate::git::run_local(root, a, b));
    reader.operation_in_progress()
}
pub struct CompositionLease {
    pub workspace: WorkspaceComposition,
    branches: BTreeMap<PathBuf, Option<String>>,
    #[cfg(unix)]
    modes: BTreeMap<PathBuf, bool>,
    _locks: Vec<FileLock>,
    lock_paths: BTreeSet<PathBuf>,
    pub outcomes: Vec<Value>,
}

impl CompositionLease {
    /// Coordinate one checkpoint. The consumer supplies its accepted-byte
    /// single-repository transaction; composition ordering and ref ownership
    /// are defined here, independently of model or transport.
    pub fn checkpoint_components<E, F>(
        &mut self,
        root: &Path,
        root_branch: &str,
        pending: &BTreeSet<PathBuf>,
        mut save: F,
    ) -> Result<(), E>
    where
        E: From<Error>,
        F: FnMut(
            &mut crate::WorkspaceProject,
            &str,
            &BTreeSet<PathBuf>,
            &mut Vec<Value>,
        ) -> Result<(), E>,
    {
        self.check(root)?;
        let mut destinations = BTreeMap::new();
        for (path, repository, branch) in std::iter::once((
            Path::new(""),
            self.workspace.central_repository.as_str(),
            Some(root_branch),
        ))
        .chain(
            self.workspace
                .projects
                .iter()
                .map(|p| (p.path.as_path(), p.fetch_url.as_str(), self.branch(&p.path))),
        ) {
            if let Some(branch) = branch.filter(|_| !repository.is_empty()) {
                let branch = branch.trim_start_matches("refs/heads/");
                let key = (
                    crate::manifest::normalize_url(repository),
                    branch.to_owned(),
                );
                if let Some(previous) = destinations.insert(key, path.to_owned()) {
                    if previous != path {
                        return Err(error(format!("Repository owners {} and {} share authoring branch {branch}; prepare distinct branches with branch.create before checkpointing",if previous.as_os_str().is_empty(){"workspace root".into()}else{previous.display().to_string()},path.display())).into());
                    }
                }
            }
        }
        self.outcomes.clear();
        for project in &mut self.workspace.projects {
            let changed = pending
                .iter()
                .filter(|p| p.starts_with(root.join(&project.path)))
                .cloned()
                .collect::<BTreeSet<_>>();
            if changed.is_empty() {
                continue;
            }
            let reference=self.branches.get(&project.path).and_then(Option::as_deref)
                .ok_or_else(||error(format!("Project {} needs a prepared named authoring branch; use branch.create or prepare it before admission",project.path.display())))?;
            save(project, reference, &changed, &mut self.outcomes)?;
        }
        self.check(root)?;
        Ok(())
    }
    pub fn start(root: &Path, workspace: &WorkspaceComposition) -> Result<Self, Error> {
        Self::replacement(root, workspace, None)
    }

    pub fn replacement(
        root: &Path,
        workspace: &WorkspaceComposition,
        previous: Option<&Self>,
    ) -> Result<Self, Error> {
        Self::replacement_with(root, workspace, previous, &[])
    }
    pub fn replacement_with(
        root: &Path,
        workspace: &WorkspaceComposition,
        previous: Option<&Self>,
        held: &[&FileLock],
    ) -> Result<Self, Error> {
        let mut locks = Vec::new();
        let mut lock_paths = BTreeSet::new();
        let mut acquire = |path: PathBuf| -> Result<(), Error> {
            if lock_paths.insert(path.clone())
                && !previous.is_some_and(|old| old.lock_paths.contains(&path))
                && !held.iter().any(|guard| guard.path() == path)
            {
                locks.push(lock_file(&path)?);
            }
            Ok(())
        };
        let mut branches = BTreeMap::new();
        #[cfg(unix)]
        let mut modes = BTreeMap::new();
        let participants: BTreeMap<_, _> = workspace
            .projects
            .iter()
            .map(|p| (p.path.clone(), p.commit.clone()))
            .collect();
        for (path, expected) in &participants {
            let checkout = root.join(path);
            let lock = checkout.join(git_text(
                &checkout,
                &["rev-parse", "--git-path", "reqvire-mcp-worktree.lock"],
            )?);
            acquire(lock)?;
            require_clean(&checkout)
                .map_err(|e| error(format!("Repository {}: {e}", path.display())))?;
            if git_operation_in_progress(&checkout)? {
                return Err(error(format!(
                    "Finish the in-progress Git operation in {}",
                    path.display()
                )));
            }
            git_text(&checkout, &["var", "GIT_AUTHOR_IDENT"])?;
            git_text(&checkout, &["var", "GIT_COMMITTER_IDENT"])?;
            let observed = Checkout::read(&checkout)?;
            if observed.head.as_deref() != Some(expected) {
                return Err(error(format!(
                    "Project {} changed during admission",
                    path.display()
                )));
            }
            if let Some(branch) = &observed.branch {
                let common = PathBuf::from(git_text(
                    &checkout,
                    &["rev-parse", "--path-format=absolute", "--git-common-dir"],
                )?);
                acquire(common.join(format!(
                    "reqvire-mcp-{}.lock",
                    crate::service::hash(branch.as_bytes())
                )))?;
            }
            #[cfg(unix)]
            modes.insert(
                path.clone(),
                git_text(
                    &checkout,
                    &[
                        "config",
                        "--bool",
                        "--default",
                        "true",
                        "--get",
                        "core.filemode",
                    ],
                )? == "true",
            );
            branches.insert(path.clone(), observed.branch);
        }
        Ok(Self {
            workspace: workspace.clone(),
            branches,
            #[cfg(unix)]
            modes,
            _locks: locks,
            lock_paths,
            outcomes: Vec::new(),
        })
    }

    pub fn retain_previous_locks(&mut self, previous: &mut Self) {
        for lock in previous._locks.drain(..) {
            if self.lock_paths.contains(lock.path()) {
                self._locks.push(lock);
            }
        }
    }

    pub fn held_locks(&self) -> Vec<&FileLock> {
        self._locks.iter().collect()
    }
    pub fn adopt_locks(&mut self, locks: Vec<FileLock>) {
        for lock in locks {
            if self.lock_paths.contains(lock.path()) {
                self._locks.push(lock);
            }
        }
    }
    pub const fn references(&self) -> &BTreeMap<PathBuf, Option<String>> {
        &self.branches
    }

    pub fn branch(&self, path: &Path) -> Option<&str> {
        self.branches
            .get(path)
            .and_then(Option::as_deref)
            .map(|b| b.trim_start_matches("refs/heads/"))
    }

    pub fn needs_checkpoint(&self) -> Result<bool, Error> {
        Ok(self
            .workspace
            .projects
            .iter()
            .any(|project| project.revision != project.commit))
    }
    #[cfg(unix)]
    pub fn track_file_mode(&self, root: &Path, path: &Path) -> Option<bool> {
        self.modes
            .iter()
            .find(|(owner, _)| path.starts_with(root.join(owner)))
            .map(|(_, value)| *value)
    }
    pub fn check(&self, root: &Path) -> Result<(), Error> {
        self.workspace.check_selection(root)?;
        for project in &self.workspace.projects {
            let observed = Checkout::read(&root.join(&project.path))?;
            if observed.head.as_deref() != Some(&project.commit)
                || self.branches.get(&project.path) != Some(&observed.branch)
            {
                return Err(error(format!(
                    "Owned project {} branch or HEAD changed externally",
                    project.path.display()
                )));
            }
        }
        for (path, accepted) in &self.workspace.inputs {
            if fs::read(root.join(path)).ok().as_deref() != Some(accepted.as_slice()) {
                return Err(error(format!(
                    "Accepted manifest input {} changed externally",
                    path.display()
                )));
            }
        }
        Ok(())
    }
}
