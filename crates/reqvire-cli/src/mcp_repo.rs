//! Repository-owned state for the existing accepted-model transaction.
//! Component ref updates precede the root-owned source manifest checkpoint.
use super::*;
use serde_json::json;

pub(super) use reqvire_workspace::lease::CompositionLease as RepoSession;

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
        let root = self.root.clone();
        let branch = self.branch.clone();
        let pending = self.pending_paths.clone();
        repo.checkpoint_components(&root, &branch, &pending, |project, reference, changed, outcomes| -> Result<(), ReqvireError> {
            let owner = root.join(&project.path);
            let mut publication =
                prepare_commit_for(&owner, &project.commit, &self.files, changed, message)?;
            if git_text(
                &owner,
                &["rev-parse", &format!("{}^{{tree}}", publication.commit)],
            )? == git_text(
                &owner,
                &["rev-parse", &format!("{}^{{tree}}", project.commit)],
            )? {
                // A previous partial save already published these exact bytes.
                outcomes.push(json!({"name":project.name,"path":project.path,"commit":project.commit,"outcome":"no_op"}));
                return Ok(());
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
                    outcomes.push(json!({"name":project.name,"path":project.path,"commit":publication.commit,"outcome":"unknown"}));
                    self.recovery_error=Some(format!("Component {} ref observation failed after attempting {}; preserve its files/index and verify the recorded commit before normal admission: {failure}",project.path.display(),publication.commit));
                    return Err(error(self.recovery_error.as_deref().unwrap_or_default()));
                }
            };
            if observed != publication.commit {
                let unknown = observed != project.commit;
                outcomes.push(json!({"name":project.name,"path":project.path,"commit":publication.commit,"outcome":if unknown {"unknown"} else {"failed"}}));
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
            outcomes.push(json!({"name":project.name,"path":project.path,"commit":project.commit,"outcome":"completed"}));
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
            Ok(())
        })?;
        repo.check(&self.root)?;
        let relative = repo.workspace.manifest.clone();
        let manifest = self.root.join(&relative);
        let bytes = Arc::new(repo.workspace.pinned_manifest()?);
        if self.files.files.get(&manifest) != Some(&bytes) {
            ensure_regular_destination(&self.root, &manifest)?;
            persist(&manifest, Some(bytes.as_slice()))?;
            Arc::make_mut(&mut self.files.files).insert(manifest.clone(), Arc::clone(&bytes));
            self.pending_paths.insert(manifest);
        }
        repo.workspace
            .inputs
            .insert(relative, bytes.as_ref().clone());
        for project in &mut repo.workspace.projects {
            project.revision.clone_from(&project.commit);
        }
        let bytes = repo.workspace.pinned_manifest()?;
        repo.workspace.source = String::from_utf8(bytes).map_err(|e| error(e.to_string()))?;
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
