//! Explicit HEAD refresh: protect model inputs, then use normal validated loading.
use super::*;
use reqvire::model_inputs;
use serde_json::json;

pub(super) fn observe_policy(root: &Path) {
    for name in [".gitignore", ".reqvireignore"] {
        let path = root.join(name);
        if model_inputs::exists(&path) {
            let _ = model_inputs::read_to_string(&path);
        }
    }
}

impl MutationSession {
    fn input_blockers(&self, exclusions: &ExclusionSet) -> Vec<Value> {
        let mut blockers = Vec::new();
        let path_label = |p: &Path| {
            p.strip_prefix(&self.root)
                .unwrap_or(p)
                .to_string_lossy()
                .into_owned()
        };
        let mut block = |reason: &str, path: &Path| {
            blockers.push(json!({
            "reason":reason, "path":path_label(path),
            "next_action":"Restore the accepted input or use full validated worktree admission."}))
        };
        let active = |owners: &BTreeSet<PathBuf>| {
            owners
                .iter()
                .any(|owner| self.markdown_candidate(owner, exclusions))
        };
        for (path, owners) in self.inputs.semantic_sources() {
            if active(owners) && !self.inputs.content_is_current(path) {
                block("semantic_source_changed", path);
            }
        }
        for (path, owners) in self.inputs.semantic_candidates() {
            if active(owners) && !self.inputs.existence_is_current(path) {
                block("semantic_resolution_changed", path);
            }
        }
        for (path, owners) in self.inputs.reference_targets() {
            if active(owners) && !self.inputs.existence_is_current(path) {
                block("missing_relation_target", path);
            }
        }
        blockers
    }

    fn head_result(&self, observed: Option<&str>, dry_run: bool) -> Value {
        json!({"outcome":"blocked", "can_refresh":false, "dry_run":dry_run,
            "branch":self.branch_name(), "previous_head":self.head,
            "observed_head":observed, "accepted_head":self.head,
            "model_revision":self.model_revision, "changed_files":0, "blockers":[]})
    }

    fn rejected(mut result: Value, blockers: Vec<Value>) -> Value {
        let count = blockers.len();
        result["blockers"] = json!(blockers.into_iter().take(128).collect::<Vec<_>>());
        result["blockers_total"] = json!(count);
        result["blockers_truncated"] = json!(count > 128);
        result
    }

    pub fn refresh_head(&mut self, dry_run: bool) -> Result<Value, ReqvireError> {
        let observed = GitObservation::read(&self.root)?;
        let mut result = self.head_result(observed.checkout.head.as_deref(), dry_run);
        let state_block =
            |reason: &str, action: &str| vec![json!({"reason":reason,"next_action":action})];
        if self.recovery_error.is_some() || self.recovery_attempt.is_some() {
            return Ok(Self::rejected(
                result,
                state_block(
                    "recovery_required",
                    "Resolve the recorded recovery state; HEAD refresh cannot reconcile a commit.",
                ),
            ));
        }
        if let Some(repo) = &self.repo {
            if repo.check(&self.root).is_err()
                || repo.workspace.inputs.iter().any(|(path, bytes)| {
                    std::fs::read(self.root.join(path)).ok().as_deref() != Some(bytes.as_slice())
                })
            {
                return Ok(Self::rejected(result, state_block("manifest_or_project_changed", "Restore the accepted Repo selection or use full validated worktree admission.")));
            }
        }
        if !self.pending_paths.is_empty() {
            return Ok(Self::rejected(
                result,
                state_block(
                    "pending_changes",
                    "Commit or resolve accepted pending model changes first.",
                ),
            ));
        }
        if observed.checkout.branch.as_deref() != Some(&self.branch)
            || observed.checkout.head.is_none()
        {
            return Ok(Self::rejected(
                result,
                state_block(
                    "branch_changed",
                    "Restore the owned named branch before refreshing.",
                ),
            ));
        }
        if git_operation_in_progress(&self.root)? {
            return Ok(Self::rejected(
                result,
                state_block(
                    "git_operation_in_progress",
                    "Finish the active Git operation first.",
                ),
            ));
        }
        if observed.dirty {
            return Ok(Self::rejected(
                result,
                state_block(
                    "dirty_worktree",
                    "Resolve staged, unstaged and nonignored untracked changes first.",
                ),
            ));
        }
        let head = observed.checkout.head.expect("checked committed HEAD");
        let changed = reqvire::git_commands::GitReader::new(&self.root, |args, input| {
            git(&self.root, args, input, None)
        })
        .changed_paths(&self.head, &head)?;
        result["changed_files"] = json!(changed.len());
        let exclusions = self.exclusions.refreshed();
        let mut blockers = self.input_blockers(&exclusions);
        for path in &changed {
            let reason = if self.inputs.semantic_sources().any(|(source, owners)| {
                source == path
                    && owners
                        .iter()
                        .any(|owner| self.markdown_candidate(owner, &exclusions))
            }) {
                Some("semantic_source_changed")
            } else if self.model_document(
                path,
                self.files.files.get(path).map(|b| b.as_slice()),
                &exclusions,
            ) {
                Some("protected_model_source")
            } else {
                None
            };
            if let Some(reason) = reason {
                blockers.push(
                    json!({"reason":reason,"path":path.strip_prefix(&self.root).unwrap_or(path),
                    "next_action":"Restore this protected input or use full validated admission."}),
                );
            }
        }
        if !blockers.is_empty() {
            return Ok(Self::rejected(result, blockers));
        }
        let markdown: BTreeSet<_> = changed
            .iter()
            .filter(|path| self.markdown_candidate(path, &exclusions))
            .cloned()
            .collect();
        let documents = reqvire::git_commands::GitReader::new(&self.root, |args, input| {
            git(&self.root, args, input, None)
        })
        .regular_files_at_commit(&head, &markdown)?;
        for (path, file) in &documents {
            if self.model_document(path, Some(file.bytes.as_slice()), &exclusions) {
                blockers.push(json!({"reason":"protected_model_source", "path":path.strip_prefix(&self.root).unwrap_or(path),
                    "next_action":"New or converted model documents require explicit full admission."}));
            }
        }
        if !blockers.is_empty() {
            return Ok(Self::rejected(result, blockers));
        }
        result["can_refresh"] = json!(true);
        result["outcome"] = json!(if head == self.head {
            "unchanged"
        } else if dry_run {
            "checked"
        } else {
            "refreshed"
        });
        if result["outcome"] == "refreshed" {
            let loaded = load_snapshot(&self.root, &self.branch, &head, &exclusions, self.options)?;
            self.files = loaded.files;
            self.committed_files = self.files.clone();
            self.model = loaded.model;
            self.model_revision = loaded.model_revision;
            self.inputs = loaded.inputs;
            self.exclusions = exclusions;
            self.head = head;
            reqvire::model_cache::invalidate();
            result["accepted_head"] = json!(self.head);
            result["model_revision"] = json!(self.model_revision);
        }
        Ok(result)
    }

    fn markdown_candidate(&self, path: &Path, exclusions: &ExclusionSet) -> bool {
        path.extension().is_some_and(|ext| ext == "md")
            && path.starts_with(&self.root)
            && !reqvire::utils::is_to_be_ignored(path, exclusions)
    }

    fn model_document(&self, path: &Path, bytes: Option<&[u8]>, exclusions: &ExclusionSet) -> bool {
        self.markdown_candidate(path, exclusions)
            && bytes.is_some_and(|bytes| {
                std::str::from_utf8(bytes).map_or(true, |content| {
                    reqvire::parser::detect_model_file_type(content)
                        != reqvire::parser::ModelFileType::Unsupported
                })
            })
    }
}
