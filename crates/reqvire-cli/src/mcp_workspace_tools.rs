//! Accepted-model admission adapter for shared workspace preparations.
use super::*;
use reqvire_workspace::{Operation, Workspace};
use serde_json::json;

impl MutationSession {
    pub fn workspace_operation(&mut self, tool: &str, args: &Value) -> Result<Value, ReqvireError> {
        self.check_recovery()?;
        if self.recovery_attempt.is_some() {
            return Err(error(
                "Resolve commit recovery before workspace preparation",
            ));
        }
        self.check_checkout(&GitCheckout::read(&self.root)?)?;
        let manifest = self
            .repo
            .as_ref()
            .map(|r| self.root.join(&r.workspace.manifest))
            .unwrap_or_else(|| self.root.join(reqvire_workspace::manifest::MANIFEST));
        let configuration = BTreeSet::from([manifest.clone()]);
        if self.repo.is_none() && manifest.exists() && !self.files.files.contains_key(&manifest) {
            return Err(error("Workspace configuration changed externally; prepare and admit the composition explicitly"));
        }
        if self
            .pending_paths
            .iter()
            .any(|p| !configuration.contains(p))
        {
            return Err(error(
                "Commit or resolve accepted model edits before changing workspace composition",
            ));
        }
        if let Some(repo) = &self.repo {
            for (path, bytes) in &repo.workspace.inputs {
                if fs::read(self.root.join(path)).ok().as_deref() != Some(bytes.as_slice()) {
                    return Err(error(format!(
                        "Accepted manifest input {} changed externally",
                        path.display()
                    )));
                }
            }
        }
        let string = |name: &str| {
            args[name]
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| error(format!("Missing workspace {name}")))
        };
        let operation = match tool {
            "reqvire.workspace.add" => Operation::Add {
                url: string("url")?,
                path: string("path")?.into(),
                reference: string("ref")?,
            },
            "reqvire.workspace.remove" => Operation::Remove {
                path: string("path")?.into(),
            },
            "reqvire.workspace.sync" => Operation::Sync {
                latest: args["latest"].as_bool().unwrap_or(false),
            },
            "reqvire.workspace.freeze" => Operation::Freeze,
            "reqvire.workspace.migrate" => {
                if string("from")? != "repo" {
                    return Err(error("Unsupported workspace migration source"));
                }
                Operation::Migrate {
                    source: args["source"].as_str().map(PathBuf::from),
                }
            }
            _ => return Err(error("Unknown workspace operation")),
        };
        let dry_run = args["dry_run"].as_bool().unwrap_or(false);
        let workspace = Workspace::open(&self.root)?;
        let mut held = vec![&self._ownership, &self._worktree_ownership];
        if let Some(repo) = &self.repo {
            held.extend(repo.held_locks());
        }
        let mut prepared = match workspace.prepare(operation, dry_run, &held) {
            Ok(prepared) => prepared,
            Err(failure) => {
                self.workspace_outcomes = failure
                    .outcomes()
                    .iter()
                    .map(|o| serde_json::to_value(o).expect("workspace outcome"))
                    .collect();
                if let Some(repo) = &mut self.repo {
                    repo.outcomes = self.workspace_outcomes.clone();
                }
                if failure.outcome_unknown() {
                    self.recovery_error=Some(format!("Workspace operation has an unknown outcome; retain reported checkouts and verify their state before admission: {failure}"));
                }
                return Err(failure.into());
            }
        };
        if dry_run {
            return Ok(serde_json::to_value(prepared.accept())?);
        }
        let allowed = configuration.clone();
        let admission = (|| -> Result<_, ReqvireError> {
            let loaded = load_workspace_candidate(
                &self.root,
                &self.branch,
                &self.head,
                &self.exclusions,
                self.options,
                Some(&allowed),
            )?;
            let scope = loaded
                .files
                .workspace
                .as_ref()
                .expect("validated workspace");
            let next = scope
                .repo
                .as_ref()
                .map(|view| {
                    repo::RepoSession::replacement_with(
                        &self.root,
                        view,
                        self.repo.as_ref(),
                        &prepared.held_locks(),
                    )
                })
                .transpose()?;
            if !loaded.inputs.is_current() {
                return Err(error(
                    "Workspace inputs changed during validated preparation",
                ));
            }
            self.check_checkout(&GitCheckout::read(&self.root)?)?;
            if let Some(next) = &next {
                next.check(&self.root)?;
            }
            Ok((loaded, next))
        })();
        let (loaded, mut next) = match admission {
            Ok(admitted) => admitted,
            Err(validation) => {
                if let Err(rollback) = prepared.rollback() {
                    self.recovery_error=Some(format!("Workspace validation failed ({validation}); recovery required after restoring the candidate failed: {rollback}"));
                    prepared.retain_for_recovery();
                    return Err(error(self.recovery_error.as_deref().unwrap_or_default()));
                }
                return Err(validation);
            }
        };
        if let (Some(next), Some(old)) = (&mut next, &mut self.repo) {
            next.retain_previous_locks(old);
        }
        if let Some(next) = &mut next {
            next.adopt_locks(prepared.take_locks());
        }
        let report = prepared.accept();
        self.workspace_outcomes = report
            .outcomes
            .iter()
            .map(|o| serde_json::to_value(o).expect("workspace outcome"))
            .collect();
        if let Some(next) = &mut next {
            next.outcomes = self.workspace_outcomes.clone();
        }
        // Component commits were explicitly selected by this operation, not
        // authored pending bytes. Only configuration remains pending at root.
        let committed_sources: Vec<_> = configuration
            .iter()
            .map(|p| (p.clone(), self.committed_files.files.get(p).cloned()))
            .collect();
        self.committed_files = loaded.files.clone();
        for (path, bytes) in committed_sources {
            if let Some(bytes) = bytes {
                Arc::make_mut(&mut self.committed_files.files).insert(path, bytes);
            } else {
                Arc::make_mut(&mut self.committed_files.files).remove(&path);
            }
        }
        self.files = loaded.files;
        self.model = loaded.model;
        self.model_revision = loaded.model_revision;
        self.inputs = loaded.inputs;
        self.repo = next;
        for path in configuration {
            if self.files.files.get(&path) != self.committed_files.files.get(&path) {
                self.pending_paths.insert(path);
            } else {
                self.pending_paths.remove(&path);
            }
        }
        reqvire::model_cache::invalidate();
        let mut result = serde_json::to_value(report)?;
        if self.enable_commits && !self.pending_paths.is_empty() {
            result["commit"] = self.explicit_commit(&format!("Update workspace via {tool}"))?;
        }
        result["model_revision"] = json!(self.model_revision);
        Ok(result)
    }
}
