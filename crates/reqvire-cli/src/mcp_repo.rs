//! Repository-owned state for the existing accepted-model transaction.
//! Component ref updates precede the root-owned source manifest checkpoint.
use super::*;
use reqvire::repo_workspace::RepoWorkspace;
use serde_json::json;

pub(super) struct RepoSession {
    pub workspace: RepoWorkspace,
    branches: BTreeMap<PathBuf, Option<String>>,
    #[cfg(unix)]
    modes: BTreeMap<PathBuf, bool>,
    _locks: Vec<FileLock>,
    lock_paths: BTreeSet<PathBuf>,
    pub outcomes: Vec<Value>,
}

impl RepoSession {
    pub fn start(root: &Path, workspace: &RepoWorkspace) -> Result<Self, ReqvireError> {
        Self::replacement(root, workspace, None)
    }

    pub fn replacement(
        root: &Path,
        workspace: &RepoWorkspace,
        previous: Option<&Self>,
    ) -> Result<Self, ReqvireError> {
        let mut locks = Vec::new();
        let mut lock_paths = BTreeSet::new();
        let mut acquire = |path: PathBuf| -> Result<(), ReqvireError> {
            if lock_paths.insert(path.clone())
                && !previous.is_some_and(|old| old.lock_paths.contains(&path))
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
            let observed = GitCheckout::read(&checkout)?;
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
                    reqvire::utils::hash_content(branch)
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
            if self.lock_paths.contains(&lock.path) {
                self._locks.push(lock);
            }
        }
    }

    pub fn branch(&self, path: &Path) -> Option<&str> {
        self.branches
            .get(path)
            .and_then(Option::as_deref)
            .map(|b| b.trim_start_matches("refs/heads/"))
    }

    pub fn needs_checkpoint(&self, _files: &SnapshotFiles) -> Result<bool, ReqvireError> {
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
    pub fn check(&self, root: &Path) -> Result<(), ReqvireError> {
        self.workspace.check_selection(root)?;
        for project in &self.workspace.projects {
            let observed = GitCheckout::read(&root.join(&project.path))?;
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

impl MutationSession {
    pub(super) fn refresh_repo_provenance(&mut self) {
        if let Some(repo) = &mut self.repo {
            Self::set_repo_head(repo, &self.head, &mut self.files, &mut self.committed_files);
        }
    }

    fn set_repo_head(
        repo: &mut RepoSession,
        head: &str,
        files: &mut SnapshotFiles,
        committed: &mut SnapshotFiles,
    ) {
        repo.workspace.central_commit = head.to_owned();
        let mut scope = files
            .workspace
            .as_deref()
            .expect("workspace provenance")
            .clone();
        scope.repo = Some(Arc::new(repo.workspace.clone()));
        files.workspace = Some(Arc::new(scope));
        committed.workspace.clone_from(&files.workspace);
    }

    pub(super) fn commit_repo(&mut self, message: &str) -> Result<Value, ReqvireError> {
        let mut repo = self.repo.take().expect("Repo transaction");
        let result = self.commit_repo_inner(&mut repo, message);
        self.repo = Some(repo);
        result
    }

    fn commit_repo_inner(
        &mut self,
        repo: &mut RepoSession,
        message: &str,
    ) -> Result<Value, ReqvireError> {
        repo.check(&self.root)?;
        let mut destinations = BTreeMap::new();
        let mut select = |path: &Path, repository: &str, branch: Option<&str>| {
            if let Some(branch) = branch.filter(|_| !repository.is_empty()) {
                let branch = branch.trim_start_matches("refs/heads/");
                let key = (
                    reqvire::repo_workspace::normalize_url(repository),
                    branch.to_owned(),
                );
                if let Some(previous) = destinations.insert(key, path.to_owned()) {
                    if previous != path {
                        return Err(error(format!(
                            "Repository owners {} and {} share authoring branch {branch}; prepare distinct branches with branch.create before checkpointing",
                            if previous.as_os_str().is_empty() { "workspace root".into() } else { previous.display().to_string() },
                            path.display()
                        )));
                    }
                }
            }
            Ok(())
        };
        select(
            Path::new(""),
            &repo.workspace.central_repository,
            Some(&self.branch),
        )?;
        for project in &repo.workspace.projects {
            select(
                &project.path,
                &project.fetch_url,
                repo.branch(&project.path),
            )?;
        }
        repo.outcomes.clear();
        for project in &mut repo.workspace.projects {
            let owner = self.root.join(&project.path);
            let changed: BTreeSet<_> = self
                .pending_paths
                .iter()
                .filter(|p| p.starts_with(&owner))
                .cloned()
                .collect();
            if changed.is_empty() {
                continue;
            }
            let reference = repo.branches.get(&project.path).and_then(Option::as_deref)
                .ok_or_else(|| error(format!("Project {} needs a prepared named authoring branch; use branch.create or prepare it before admission", project.path.display())))?;
            let mut publication =
                prepare_commit_for(&owner, &project.commit, &self.files, &changed, message)?;
            if git_text(
                &owner,
                &["rev-parse", &format!("{}^{{tree}}", publication.commit)],
            )? == git_text(
                &owner,
                &["rev-parse", &format!("{}^{{tree}}", project.commit)],
            )? {
                // A previous partial save already published these exact bytes.
                repo.outcomes.push(json!({"name":project.name,"path":project.path,"commit":project.commit,"outcome":"no_op"}));
                continue;
            }
            let outcome = git_result(
                &owner,
                &[
                    "update-ref",
                    "-m",
                    "reqvire: accepted component commit",
                    reference,
                    &publication.commit,
                    &project.commit,
                ],
                None,
                None,
            );
            let observed = match git_text(&owner, &["rev-parse", "HEAD"]) {
                Ok(observed) => observed,
                Err(failure) => {
                    repo.outcomes.push(json!({"name":project.name,"path":project.path,"commit":publication.commit,"outcome":"unknown"}));
                    self.recovery_error=Some(format!("Component {} ref observation failed after attempting {}; preserve its files/index and verify the recorded commit before normal admission: {failure}",project.path.display(),publication.commit));
                    return Err(error(self.recovery_error.as_deref().unwrap_or_default()));
                }
            };
            if observed != publication.commit {
                let unknown = observed != project.commit;
                repo.outcomes.push(json!({"name":project.name,"path":project.path,"commit":publication.commit,"outcome":if unknown {"unknown"} else {"failed"}}));
                if unknown {
                    self.recovery_error = Some(format!("Component {} has an unrecognized ref outcome; retained accepted model is read-only", project.path.display()));
                }
                return Err(error(format!(
                    "Component {} commit not confirmed: {}",
                    project.path.display(),
                    outcome
                        .err()
                        .map(|e| e.error.to_string())
                        .unwrap_or_else(|| "HEAD did not match the attempted commit".into())
                )));
            }
            // Record confirmed ref publication before doing any later side effect.
            project.commit = publication.commit.clone();
            repo.outcomes.push(json!({"name":project.name,"path":project.path,"commit":project.commit,"outcome":"completed"}));
            if let Err(failure) = fs::rename(&publication.index_lock.path, &publication.index_path)
            {
                self.recovery_error = Some(format!("Component {} commit is retained but its index requires reconciliation: {failure}", project.path.display()));
                return Err(failure.into());
            }
            publication.index_lock.published = true;
            git(
                &owner,
                &[
                    "update-ref",
                    &format!("refs/reqvire/commits/{}", project.commit),
                    &project.commit,
                ],
                None,
                None,
            )?;
        }
        repo.check(&self.root)?;
        let pins = repo.workspace.pinned_inputs()?;
        for (relative, bytes) in pins {
            let manifest = self.root.join(&relative);
            let bytes = Arc::new(bytes);
            if self.files.files.get(&manifest) != Some(&bytes) {
                ensure_regular_destination(&self.root, &manifest)?;
                persist(&manifest, Some(bytes.as_slice()))?;
                Arc::make_mut(&mut self.files.files).insert(manifest.clone(), Arc::clone(&bytes));
                self.pending_paths.insert(manifest);
            }
            repo.workspace
                .inputs
                .insert(relative, bytes.as_ref().clone());
        }
        for project in &mut repo.workspace.projects {
            project.revision.clone_from(&project.commit);
        }
        let bytes = repo.workspace.pinned_xml()?;
        repo.workspace.resolved_xml = String::from_utf8(bytes).map_err(|e| error(e.to_string()))?;
        let mut scope = self
            .files
            .workspace
            .as_deref()
            .expect("workspace provenance")
            .clone();
        scope.repo = Some(Arc::new(repo.workspace.clone()));
        self.files.workspace = Some(Arc::new(scope));
        let files = self.files.clone();
        let all_changed = self.pending_paths.clone();
        let central: BTreeSet<_> = all_changed
            .iter()
            .filter(|path| {
                !repo
                    .workspace
                    .projects
                    .iter()
                    .any(|p| path.starts_with(self.root.join(&p.path)))
            })
            .cloned()
            .collect();
        let result = self.commit_central(&files, &central, message);
        match result {
            Ok(mut value) => {
                Self::set_repo_head(repo, &self.head, &mut self.files, &mut self.committed_files);
                repo.outcomes
                    .push(json!({"name":"central","commit":self.head,
                    "outcome":if central.is_empty() {"no_op"} else {"completed"}}));
                value["projects"] = json!(repo.outcomes);
                value["manifest"] = json!({"path":repo.workspace.manifest, "outcome":"committed", "versioned_in_central":true});
                value["changed_files"] = json!(all_changed
                    .iter()
                    .filter_map(|p| p.strip_prefix(&self.root).ok())
                    .collect::<Vec<_>>());
                Ok(value)
            }
            Err(failure) => {
                repo.outcomes.push(json!({"name":"central","outcome":if self.recovery_error.is_some() {"unknown"} else {"failed"},"diagnostic":failure.to_string()}));
                Err(failure)
            }
        }
    }
}
