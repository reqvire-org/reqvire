//! Repository-owned state for the existing accepted-model transaction.
//! Component ref updates precede the owning Central manifest checkpoint.
use super::*;
use reqvire::repo_workspace::RepoWorkspace;
use serde_json::json;

pub(super) struct RepoSession {
    pub workspace: RepoWorkspace,
    branches: BTreeMap<PathBuf, Option<String>>,
    #[cfg(unix)]
    modes: BTreeMap<PathBuf, bool>,
    _locks: Vec<FileLock>,
    pub outcomes: Vec<Value>,
}

impl RepoSession {
    pub fn start(root: &Path, workspace: &RepoWorkspace) -> Result<Self, ReqvireError> {
        let mut locks = Vec::new();
        let mut branches = BTreeMap::new();
        #[cfg(unix)]
        let mut modes = BTreeMap::new();
        for project in &workspace.projects {
            let checkout = root.join(&project.path);
            let lock = checkout.join(git_text(
                &checkout,
                &["rev-parse", "--git-path", "reqvire-mcp-worktree.lock"],
            )?);
            locks.push(lock_file(&lock)?);
            require_clean(&checkout)
                .map_err(|e| error(format!("Project {}: {e}", project.path.display())))?;
            if git_operation_in_progress(&checkout)? {
                return Err(error(format!(
                    "Finish the in-progress Git operation in {}",
                    project.path.display()
                )));
            }
            git_text(&checkout, &["var", "GIT_AUTHOR_IDENT"])?;
            git_text(&checkout, &["var", "GIT_COMMITTER_IDENT"])?;
            let observed = GitCheckout::read(&checkout)?;
            if observed.head.as_deref() != Some(&project.commit) {
                return Err(error(format!(
                    "Project {} changed during admission",
                    project.path.display()
                )));
            }
            if let Some(branch) = &observed.branch {
                let common = PathBuf::from(git_text(
                    &checkout,
                    &["rev-parse", "--path-format=absolute", "--git-common-dir"],
                )?);
                locks.push(lock_file(&common.join(format!(
                    "reqvire-mcp-{}.lock",
                    reqvire::utils::hash_content(branch)
                )))?);
            }
            #[cfg(unix)]
            modes.insert(
                project.path.clone(),
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
            branches.insert(project.path.clone(), observed.branch);
        }
        Ok(Self {
            workspace: workspace.clone(),
            branches,
            #[cfg(unix)]
            modes,
            _locks: locks,
            outcomes: Vec::new(),
        })
    }
    #[cfg(unix)]
    pub fn track_file_mode(&self, root: &Path, path: &Path) -> Option<bool> {
        self.modes
            .iter()
            .find(|(owner, _)| path.starts_with(root.join(owner)))
            .map(|(_, value)| *value)
    }
    pub fn check(&self, root: &Path) -> Result<(), ReqvireError> {
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
        Ok(())
    }
}

impl MutationSession {
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
        let original_commits: Vec<_> = repo
            .workspace
            .projects
            .iter()
            .map(|p| p.commit.clone())
            .collect();
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
            let reference = repo
                .branches
                .get(&project.path)
                .and_then(Option::as_deref)
                .unwrap_or("HEAD");
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
            let observed = git_text(&owner, &["rev-parse", "HEAD"])?;
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
        let native = repo.workspace.uses_repo_managed_inputs();
        let components_changed = repo
            .workspace
            .projects
            .iter()
            .zip(original_commits)
            .any(|(p, old)| p.commit != old);
        let pins = if !native || components_changed {
            repo.workspace.pinned_inputs()?
        } else {
            BTreeMap::new()
        };
        for (relative, bytes) in pins {
            let manifest = self.root.join(&relative);
            let bytes = Arc::new(bytes);
            if self.files.files.get(&manifest) != Some(&bytes) {
                ensure_regular_destination(&self.root, &manifest)?;
                persist(&manifest, Some(bytes.as_slice()))?;
                Arc::make_mut(&mut self.files.files).insert(manifest.clone(), Arc::clone(&bytes));
                if !native {
                    self.pending_paths.insert(manifest);
                }
            }
            repo.workspace
                .inputs
                .insert(relative, bytes.as_ref().clone());
        }
        if !native || components_changed {
            for project in &mut repo.workspace.projects {
                project.revision.clone_from(&project.commit);
            }
        }
        let bytes = repo.workspace.pinned_xml()?;
        repo.workspace.resolved_xml = String::from_utf8(bytes).map_err(|e| error(e.to_string()))?;
        if !native {
            // Central checkpoints flatten the resolved manifest. Prior include
            // files remain source assets, but no longer select this composition.
            let manifest = &repo.workspace.manifest;
            repo.workspace.inputs.retain(|path, _| {
                path == manifest || path == Path::new(".repo/manifests.git/config")
            });
            repo.workspace
                .manifest_sources
                .retain(|path, _| path == manifest);
        }
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
        let result = if native && central.is_empty() {
            self.committed_files = files.clone();
            self.pending_paths.clear();
            Ok(json!({"outcome":"completed", "commit":self.head, "head":self.head}))
        } else {
            self.commit_central(&files, &central, message)
        };
        match result {
            Ok(mut value) => {
                repo.workspace.central_commit.clone_from(&self.head);
                for source in repo.workspace.manifest_sources.values_mut() {
                    if source.root.as_os_str().is_empty() {
                        source.commit.clone_from(&self.head);
                    }
                }
                let mut scope = self
                    .files
                    .workspace
                    .as_deref()
                    .expect("workspace provenance")
                    .clone();
                scope.repo = Some(Arc::new(repo.workspace.clone()));
                self.files.workspace = Some(Arc::new(scope));
                self.committed_files
                    .workspace
                    .clone_from(&self.files.workspace);
                repo.outcomes
                    .push(json!({"name":"central","commit":self.head,
                    "outcome":if native && central.is_empty() {"no_op"} else {"completed"}}));
                value["projects"] = json!(repo.outcomes);
                if native {
                    value["manifest"] = json!({"path":repo.workspace.manifest, "outcome":"saved", "versioned_in_central":false});
                }
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
