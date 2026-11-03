//! Transport-neutral workspace preparation. Only explicit operations use network.
use crate::{
    git::local_text as local,
    locks::{lock_file, FileLock},
    manifest::{
        self, document, full_commit, matching_remote, validate_checkout, WorkspaceComposition,
        WorkspaceProject, MANIFEST,
    },
    paths::canonicalize_existing,
    process, Error,
};
use serde::Serialize;
use serde_json::{json, Value};
use std::hash::Hasher;
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    time::Duration,
};
fn failure(s: impl Into<String>) -> Error {
    Error::ProcessError(s.into())
}

pub fn hash(bytes: &[u8]) -> String {
    let mut h = rustc_hash::FxHasher::default();
    h.write(bytes);
    format!("{:x}", h.finish())
}
pub fn git(root: &Path, args: &[&str]) -> Result<String, Error> {
    let mut command = std::process::Command::new("git");
    command.current_dir(root).args(args);
    let output = process::run_binary(
        command,
        None,
        Duration::from_secs(60),
        Some(16 * 1024 * 1024),
    )
    .map_err(|f| Error::Command {
        message: f.error.to_string(),
        outcome_unknown: f.outcome_unknown,
    })?;
    if !output.success {
        return Err(failure(format!(
            "Git {} in {}: {}",
            args.first().unwrap_or(&"operation"),
            root.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    String::from_utf8(output.stdout)
        .map(|s| s.trim_end_matches(['\n', '\r']).to_owned())
        .map_err(|e| failure(e.to_string()))
}
pub fn require_clean(root: &Path) -> Result<(), Error> {
    if !local(
        root,
        &["status", "--porcelain=v1", "--untracked-files=normal"],
    )?
    .is_empty()
    {
        return Err(failure(format!(
            "Dirty or uncommitted component {}",
            root.display()
        )));
    }
    let mut reader =
        crate::git::GitReader::new(root, |args, input| crate::git::run_local(root, args, input));
    if reader.operation_in_progress()? {
        return Err(failure(format!(
            "Finish the in-progress Git operation in {}",
            root.display()
        )));
    }
    Ok(())
}

/// Find a native workspace within the containing Git boundary, without crossing it.
pub fn effective_root(start: &Path) -> Result<PathBuf, Error> {
    let start = canonicalize_existing(start)?;
    let owner = canonicalize_existing(&PathBuf::from(local(
        &start,
        &["rev-parse", "--show-toplevel"],
    )?))?;
    let mut path = start.clone();
    loop {
        if std::fs::symlink_metadata(path.join(MANIFEST)).is_ok() {
            return Ok(path);
        }
        if path == owner {
            break;
        }
        if !path.pop() {
            break;
        }
    }
    Ok(start)
}

#[derive(Debug, Clone)]
pub struct Workspace {
    pub root: PathBuf,
    pub owner: PathBuf,
    pub composition: Option<WorkspaceComposition>,
}
#[derive(Debug, Clone)]
pub enum Operation {
    Add {
        url: String,
        path: PathBuf,
        reference: String,
    },
    Remove {
        path: PathBuf,
    },
    Sync {
        latest: bool,
    },
    Freeze,
    Migrate {
        source: Option<PathBuf>,
    },
}
#[derive(Debug, Clone, Serialize)]
pub struct Outcome {
    pub path: PathBuf,
    pub operation: String,
    pub commit: Option<String>,
    pub outcome: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub operation: String,
    pub dry_run: bool,
    pub changed: bool,
    pub manifest: PathBuf,
    pub before: String,
    pub after: String,
    pub outcomes: Vec<Outcome>,
    pub ignore_suggestions: Vec<String>,
}

/// A receipt owns the same locks through candidate validation and admission.
type CheckoutChange = (PathBuf, String, Option<String>, String, Option<String>);
pub struct Prepared {
    pub report: Report,
    root: PathBuf,
    original: Option<Vec<u8>>,
    changed_heads: Vec<CheckoutChange>,
    _locks: Vec<FileLock>,
    accepted: bool,
}
impl Prepared {
    pub fn retain_for_recovery(mut self) {
        self.accepted = true;
    }
    pub fn held_locks(&self) -> Vec<&FileLock> {
        self._locks.iter().collect()
    }
    pub fn take_locks(&mut self) -> Vec<FileLock> {
        std::mem::take(&mut self._locks)
    }
    pub fn accept(mut self) -> Report {
        self.accepted = true;
        self.report.clone()
    }
    pub fn rollback(&mut self) -> Result<(), Error> {
        for (path, old, branch, new, new_branch) in self.changed_heads.iter().rev() {
            require_clean(path)?;
            if local(path, &["rev-parse", "HEAD"])? != *new {
                return Err(failure(format!(
                    "Cannot restore externally changed project {}",
                    path.display()
                )));
            }
            if let Some(branch) = branch {
                if Some(branch) == new_branch.as_ref() {
                    git(path, &["checkout", "--quiet", "--detach", new])?;
                    git(
                        path,
                        &["update-ref", &format!("refs/heads/{branch}"), old, new],
                    )?;
                }
                git(path, &["checkout", "--quiet", branch])?;
            } else {
                git(path, &["checkout", "--quiet", "--detach", old])?;
            }
        }
        if self.report.changed {
            let target = self.root.join(&self.report.manifest);
            if std::fs::read(&target).ok().as_deref() != Some(self.report.after.as_bytes()) {
                return Err(failure(
                    "Workspace manifest changed during candidate validation",
                ));
            }
            match &self.original {
                Some(bytes) => std::fs::write(target, bytes)?,
                None => std::fs::remove_file(target)?,
            }
        }
        self.accepted = true;
        Ok(())
    }
}
impl Drop for Prepared {
    fn drop(&mut self) {
        if !self.accepted && !self.report.dry_run {
            if let Err(e) = self.rollback() {
                log::error!("Workspace candidate recovery required: {e}");
            }
        }
    }
}

impl Workspace {
    pub fn open(root: &Path) -> Result<Self, Error> {
        let root = canonicalize_existing(root)?;
        let owner = canonicalize_existing(&PathBuf::from(local(
            &root,
            &["rev-parse", "--show-toplevel"],
        )?))?;
        let composition = manifest::selected_manifest(&root)?
            .map(|p| WorkspaceComposition::resolve(&root, p))
            .transpose()?;
        Ok(Self {
            root,
            owner,
            composition,
        })
    }
    pub fn status(&self) -> Result<Value, Error> {
        let projects=self.composition.as_ref().map(|c|c.projects.iter().map(|p|{
            let checkout=self.root.join(&p.path);
            let head=local(&checkout,&["rev-parse","HEAD"]).ok();
            let branch=local(&checkout,&["symbolic-ref","--quiet","--short","HEAD"]).ok();
            let diagnostic=validate_checkout(&checkout,p).err().map(|e|e.to_string());
            let dirty=local(&checkout,&["status","--porcelain=v1"]).ok().map(|s|!s.is_empty());
            json!({"name":p.name,"path":p.path,"ref":p.tracking_ref,"revision":p.revision,"commit":p.commit,"head":head,"branch":branch,"dirty":dirty,"diagnostic":diagnostic,"remote":p.remote,"fetch_url":p.fetch_url})
        }).collect::<Vec<_>>()).unwrap_or_default();
        Ok(
            json!({"workspace_root":self.root,"owning_repository":self.owner,"branch":local(&self.root,&["symbolic-ref","--quiet","--short","HEAD"]).ok(),"manifest":self.composition.as_ref().map(|c|&c.manifest),"repositories_dir":self.composition.as_ref().map(|c|&c.repositories_dir),"projects":projects}),
        )
    }
    fn acquire(&self, held: &[&FileLock]) -> Result<Vec<FileLock>, Error> {
        let mut paths = BTreeSet::new();
        let requested_branch = local(&self.root, &["symbolic-ref", "--quiet", "HEAD"]).ok();
        let mut owners = vec![self.root.clone()];
        owners.extend(
            self.composition
                .iter()
                .flat_map(|c| c.projects.iter().map(|p| self.root.join(&p.path)))
                .filter(|p| p.join(".git").exists()),
        );
        for owner in owners {
            let worktree = owner.join(local(
                &owner,
                &["rev-parse", "--git-path", "reqvire-mcp-worktree.lock"],
            )?);
            paths.insert(worktree);
            if let Ok(branch) = local(&owner, &["symbolic-ref", "--quiet", "HEAD"]) {
                let common = canonicalize_existing(&PathBuf::from(local(
                    &owner,
                    &["rev-parse", "--path-format=absolute", "--git-common-dir"],
                )?))?;
                paths.insert(common.join(format!("reqvire-mcp-{}.lock", hash(branch.as_bytes()))));
                if let Some(requested) = &requested_branch {
                    paths.insert(
                        common.join(format!("reqvire-mcp-{}.lock", hash(requested.as_bytes()))),
                    );
                }
            }
        }
        paths
            .into_iter()
            .filter(|p| !held.iter().any(|guard| guard.path() == p))
            .map(|p| lock_file(&p))
            .collect()
    }
    /// Apply under ownership; the caller validates before accepting the receipt.
    pub fn prepare(
        &self,
        operation: Operation,
        dry_run: bool,
        held: &[&FileLock],
    ) -> Result<Prepared, Error> {
        let root_head = local(&self.root, &["rev-parse", "--verify", "HEAD"]).ok();
        let root_branch = local(&self.root, &["symbolic-ref", "--quiet", "--short", "HEAD"]).ok();
        let mut locks = self.acquire(held)?;
        if local(&self.root, &["rev-parse", "--verify", "HEAD"]).ok() != root_head
            || local(&self.root, &["symbolic-ref", "--quiet", "--short", "HEAD"]).ok()
                != root_branch
        {
            return Err(failure(
                "Owning branch or HEAD changed during workspace preparation",
            ));
        }
        let manifest = self
            .composition
            .as_ref()
            .map(|c| c.manifest.clone())
            .unwrap_or_else(|| MANIFEST.into());
        let original = match std::fs::read(self.root.join(&manifest)) {
            Ok(bytes) => Some(bytes),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
            Err(e) => return Err(e.into()),
        };
        if self
            .composition
            .as_ref()
            .is_some_and(|c| original.as_deref() != Some(c.source.as_bytes()))
            || (self.composition.is_none() && original.is_some())
        {
            return Err(failure("Manifest changed before workspace preparation"));
        }
        let mut source_guards = self
            .composition
            .as_ref()
            .map(|c| c.inputs.clone())
            .unwrap_or_default();
        let mut view=match &operation {
            Operation::Migrate{source}=>{
                let source=source.clone().unwrap_or_else(||PathBuf::from(".repo/manifest.xml"));
                let old=crate::legacy::resolve_migration(&self.root,&source)?;
                source_guards.extend(old.inputs.clone());
                let mut converted=toml_edit::DocumentMut::new();
                converted["version"]=toml_edit::value(1);
                converted["repositories_dir"]=toml_edit::value(".");
                converted["repositories"]=Item::ArrayOfTables(toml_edit::ArrayOfTables::new());
                for project in &old.projects {
                    let mut table=toml_edit::Table::new();table["path"]=toml_edit::value(project.path.to_string_lossy().as_ref());table["url"]=toml_edit::value(&project.fetch_url);
                    table["name"]=toml_edit::value(&project.name);
                    if let Some(reference)=&project.tracking_ref{table["ref"]=toml_edit::value(reference);}
                    if !project.commit.is_empty(){table["commit"]=toml_edit::value(&project.commit);}
                    converted["repositories"].as_array_of_tables_mut().expect("repository array").push(table);
                }
                if let Some(existing)=&self.composition {
                    let equivalent=old.projects.len()==existing.projects.len() && old.projects.iter().all(|old|existing.projects.iter().any(|new|old.path==new.path && manifest::normalize_url(&old.fetch_url)==manifest::normalize_url(&new.fetch_url) && old.tracking_ref==new.tracking_ref && (old.commit.is_empty() || old.commit==new.commit)));
                    if !equivalent{return Err(failure("Native workspace manifest already exists; do not overwrite a conflicting target"));}
                    existing.clone()
                }else{WorkspaceComposition{selection:manifest.clone(),manifest:manifest.clone(),repositories_dir:".".into(),source:converted.to_string(),projects:old.projects,..WorkspaceComposition::default()}}
            },
            _=>self.composition.clone().ok_or_else(||failure("Create reqvire-workspace.toml with version = 1 and an explicit repositories_dir before this operation"))?,
        };
        let mut doc = document(&view.source)?;
        let mut outcomes = Vec::new();
        let mut changed_heads = Vec::new();
        let name = match &operation {
            Operation::Add { .. } => "add",
            Operation::Remove { .. } => "remove",
            Operation::Sync { .. } => "sync",
            Operation::Freeze => "freeze",
            Operation::Migrate { .. } => "migrate",
        }
        .to_owned();
        match &operation {
            Operation::Add {
                url,
                path,
                reference,
            } => {
                let relative = manifest::relative_path(&self.root, path)?;
                let destination = if view.repositories_dir == Path::new(".") {
                    relative.clone()
                } else {
                    view.repositories_dir.join(&relative)
                };
                manifest::contained(&self.root, &destination)?;
                if view
                    .projects
                    .iter()
                    .any(|p| p.path.starts_with(&destination) || destination.starts_with(&p.path))
                {
                    return Err(failure("Duplicate or overlapping workspace dependency"));
                }
                if reference.is_empty() || reference.starts_with('-') {
                    return Err(failure("Invalid tracking ref"));
                }
                if doc.get("repositories").is_none()
                    || doc
                        .get("repositories")
                        .and_then(Item::as_array)
                        .is_some_and(|a| a.is_empty())
                {
                    doc["repositories"] = Item::ArrayOfTables(toml_edit::ArrayOfTables::new());
                }
                let mut table = toml_edit::Table::new();
                table["path"] = toml_edit::value(relative.to_string_lossy().as_ref());
                table["url"] = toml_edit::value(url);
                table["ref"] = toml_edit::value(reference);
                doc["repositories"]
                    .as_array_of_tables_mut()
                    .ok_or_else(|| failure("Invalid repository array"))?
                    .push(table);
                view.projects.push(WorkspaceProject {
                    name: relative.to_string_lossy().into_owned(),
                    path: destination,
                    revision: reference.clone(),
                    commit: String::new(),
                    tracking_ref: Some(reference.clone()),
                    remote: "origin".into(),
                    fetch_url: url.clone(),
                });
            }
            Operation::Remove { path } => {
                let tables = doc
                    .get_mut("repositories")
                    .and_then(Item::as_array_of_tables_mut)
                    .ok_or_else(|| failure(format!("Missing dependency {}", path.display())))?;
                let index = tables
                    .iter()
                    .position(|table| table.get("path").and_then(Item::as_str) == path.to_str())
                    .ok_or_else(|| failure(format!("Missing dependency {}", path.display())))?;
                tables.remove(index);
                if tables.is_empty() {
                    doc["repositories"] = toml_edit::value(toml_edit::Array::new());
                }
                view.projects.retain(|p| {
                    p.path
                        != if view.repositories_dir == Path::new(".") {
                            path.clone()
                        } else {
                            view.repositories_dir.join(path)
                        }
                });
            }
            _ => {}
        }
        view.source = doc.to_string();
        // Complete all non-network preconditions before preparing any checkout.
        let latest = matches!(operation, Operation::Sync { latest: true });
        if latest && view.projects.iter().any(|p| p.tracking_ref.is_none()) {
            return Err(failure(
                "Latest synchronization requires a configured tracking ref for every dependency",
            ));
        }
        for project in &view.projects {
            let checkout = self.root.join(&project.path);
            if checkout.exists() {
                validate_checkout(&checkout, project)?;
                require_clean(&checkout)?;
            }
        }
        let mut active = None;
        let preparation = (|| -> Result<(), Error> {
            if !matches!(operation, Operation::Remove { .. }) {
                for project in &mut view.projects {
                    active = Some(project.path.clone());
                    if matches!(operation, Operation::Add { .. }) && !project.commit.is_empty() {
                        continue;
                    }
                    let checkout = self.root.join(&project.path);
                    if matches!(operation, Operation::Freeze) {
                        validate_checkout(&checkout, project)?;
                        require_clean(&checkout)?;
                        let head = local(&checkout, &["rev-parse", "HEAD"])?;
                        outcomes.push(Outcome {
                            path: project.path.clone(),
                            operation: name.clone(),
                            commit: Some(head.clone()),
                            outcome: if head == project.commit {
                                "unchanged"
                            } else {
                                "pin_update"
                            }
                            .into(),
                        });
                        project.commit = head;
                        continue;
                    }
                    if dry_run {
                        outcomes.push(Outcome {
                            path: project.path.clone(),
                            operation: name.clone(),
                            commit: (!project.commit.is_empty()).then(|| project.commit.clone()),
                            outcome: "planned".into(),
                        });
                        continue;
                    }
                    let fresh = !checkout.exists();
                    if fresh {
                        std::fs::create_dir_all(
                            checkout
                                .parent()
                                .ok_or_else(|| failure("Checkout has no parent"))?,
                        )?;
                        git(
                            &self.root,
                            &[
                                "clone",
                                "--quiet",
                                "--no-checkout",
                                "--",
                                &project.fetch_url,
                                checkout
                                    .to_str()
                                    .ok_or_else(|| failure("Non-UTF8 checkout path"))?,
                            ],
                        )?;
                        // A failed fetch must leave a reusable clean checkout,
                        // rather than the deletion state of --no-checkout.
                        let initial = if !project.commit.is_empty()
                            && local(
                                &checkout,
                                &["cat-file", "-e", &format!("{}^{{commit}}", project.commit)],
                            )
                            .is_ok()
                        {
                            project.commit.as_str()
                        } else {
                            "HEAD"
                        };
                        git(&checkout, &["checkout", "--quiet", "--detach", initial])?;
                        let worktree = checkout.join(local(
                            &checkout,
                            &["rev-parse", "--git-path", "reqvire-mcp-worktree.lock"],
                        )?);
                        let mut new_locks = vec![worktree];
                        if let Some(branch) = &root_branch {
                            let common = canonicalize_existing(&PathBuf::from(local(
                                &checkout,
                                &["rev-parse", "--path-format=absolute", "--git-common-dir"],
                            )?))?;
                            new_locks.push(common.join(format!(
                                "reqvire-mcp-{}.lock",
                                hash(format!("refs/heads/{branch}").as_bytes())
                            )));
                        }
                        for path in new_locks {
                            if !held.iter().any(|guard| guard.path() == path)
                                && !locks.iter().any(|guard| guard.path() == path)
                            {
                                locks.push(lock_file(&path)?);
                            }
                        }
                    }
                    project.remote = matching_remote(&checkout, project)?;
                    let need_ref = latest || project.commit.is_empty();
                    let selected = if need_ref {
                        let reference = project
                            .tracking_ref
                            .as_deref()
                            .ok_or_else(|| failure("Missing configured ref"))?;
                        git(
                            &checkout,
                            &[
                                "fetch",
                                "--quiet",
                                "--no-tags",
                                "--",
                                &project.fetch_url,
                                reference,
                            ],
                        )?;
                        local(&checkout, &["rev-parse", "--verify", "FETCH_HEAD^{commit}"])?
                    } else {
                        if local(
                            &checkout,
                            &["cat-file", "-e", &format!("{}^{{commit}}", project.commit)],
                        )
                        .is_err()
                        {
                            git(
                                &checkout,
                                &[
                                    "fetch",
                                    "--quiet",
                                    "--no-tags",
                                    "--",
                                    &project.fetch_url,
                                    &project.commit,
                                ],
                            )?;
                        }
                        project.commit.clone()
                    };
                    if !full_commit(&selected) {
                        return Err(failure("Synchronization did not resolve a full commit"));
                    }
                    let old = local(&checkout, &["rev-parse", "HEAD"]).ok();
                    let old_branch =
                        local(&checkout, &["symbolic-ref", "--quiet", "--short", "HEAD"]).ok();
                    let branch = root_branch.clone();
                    if let Some(branch) = &branch {
                        let existing = local(
                            &checkout,
                            &["rev-parse", "--verify", &format!("refs/heads/{branch}")],
                        )
                        .ok();
                        if fresh {
                            git(&checkout, &["checkout", "--quiet", "-B", branch, &selected])?;
                        } else {
                            if let Some(existing) =
                                existing.as_ref().filter(|tip| *tip != &selected)
                            {
                                if local(
                                    &checkout,
                                    &["merge-base", "--is-ancestor", existing, &selected],
                                )
                                .is_err()
                                {
                                    return Err(failure(format!("Project {} branch {branch} diverged or has unrecorded commits; commit/freeze or resolve it first",project.path.display())));
                                }
                            }
                            if old_branch.as_deref() == Some(branch)
                                && old.as_deref() != Some(&selected)
                            {
                                git(&checkout, &["merge", "--ff-only", "--", &selected])?;
                            } else if existing.is_some() {
                                git(&checkout, &["checkout", "--quiet", branch])?;
                                if local(&checkout, &["rev-parse", "HEAD"])? != selected {
                                    git(&checkout, &["merge", "--ff-only", "--", &selected])?;
                                }
                            } else {
                                git(&checkout, &["checkout", "--quiet", "-b", branch, &selected])?;
                            }
                        }
                    } else {
                        git(&checkout, &["checkout", "--quiet", "--detach", &selected])?;
                    }
                    if let Some(old) = old.filter(|old| old != &selected || old_branch != branch) {
                        changed_heads.push((
                            checkout.clone(),
                            old,
                            old_branch,
                            selected.clone(),
                            branch,
                        ));
                    }
                    project.commit = selected.clone();
                    project.revision = selected.clone();
                    outcomes.push(Outcome {
                        path: project.path.clone(),
                        operation: name.clone(),
                        commit: Some(selected),
                        outcome: "prepared".into(),
                    });
                }
            }
            Ok(())
        })();
        if let Err(source) = preparation {
            if let Some(path) = active {
                outcomes.push(Outcome {
                    path,
                    operation: name,
                    commit: None,
                    outcome: if source.outcome_unknown() {
                        "unknown"
                    } else {
                        "failed"
                    }
                    .into(),
                });
            }
            return Err(Error::Preparation {
                source: Box::new(source),
                outcomes,
            });
        }
        let after =
            String::from_utf8(view.pinned_manifest()?).map_err(|e| failure(e.to_string()))?;
        if local(&self.root, &["rev-parse", "--verify", "HEAD"]).ok() != root_head
            || local(&self.root, &["symbolic-ref", "--quiet", "--short", "HEAD"]).ok()
                != root_branch
        {
            return Err(failure(
                "Owning branch or HEAD changed during workspace preparation",
            ));
        }
        if std::fs::read(self.root.join(&manifest)).ok() != original {
            return Err(failure("Manifest changed during workspace preparation"));
        }
        for (path, bytes) in &source_guards {
            if std::fs::read(self.root.join(path)).ok().as_deref() != Some(bytes.as_slice()) {
                return Err(failure(format!(
                    "Manifest input {} changed during workspace preparation",
                    path.display()
                )));
            }
        }
        let changed = original.as_deref() != Some(after.as_bytes());
        if changed && !dry_run {
            manifest::contained(&self.root, &manifest)?;
            let target = self.root.join(&manifest);
            if let Err(write) = std::fs::write(&target, &after) {
                let recovery = original
                    .as_ref()
                    .map_or_else(
                        || match std::fs::remove_file(&target) {
                            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
                            other => other,
                        },
                        |bytes| std::fs::write(&target, bytes),
                    )
                    .is_err();
                outcomes.push(Outcome {
                    path: manifest,
                    operation: "write_source".into(),
                    commit: None,
                    outcome: if recovery { "unknown" } else { "failed" }.into(),
                });
                return Err(Error::Preparation {
                    source: Box::new(Error::Command {
                        message: format!(
                            "Workspace manifest write failed: {write}; {}",
                            if recovery {
                                "source recovery required"
                            } else {
                                "original source restored"
                            }
                        ),
                        outcome_unknown: recovery,
                    }),
                    outcomes,
                });
            }
        }
        let ignore_suggestions = match &operation {
            Operation::Add { path, .. } => {
                let destination = if view.repositories_dir == Path::new(".") {
                    path.clone()
                } else {
                    view.repositories_dir.join(path)
                };
                vec![format!(
                    "/{}/",
                    self.root
                        .strip_prefix(&self.owner)
                        .map_err(|e| failure(e.to_string()))?
                        .join(destination)
                        .to_string_lossy()
                )]
            }
            _ => Vec::new(),
        };
        let before = original
            .as_ref()
            .map(|b| String::from_utf8_lossy(b).into_owned())
            .unwrap_or_default();
        Ok(Prepared {
            report: Report {
                operation: name,
                dry_run,
                changed,
                manifest,
                before,
                after,
                outcomes,
                ignore_suggestions,
            },
            root: self.root.clone(),
            original,
            changed_heads,
            _locks: locks,
            accepted: dry_run,
        })
    }
}

use toml_edit::Item;
