//! Git ownership and publication boundary for mutation-enabled MCP.
#[cfg(test)]
#[path = "mcp_session_tests.rs"]
mod tests;
use reqvire::{
    error::ReqvireError, exclusions::ExclusionSet, mutation_io::SnapshotFiles, ModelBuildOptions,
    ModelManager,
};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
};

pub struct MutationSession {
    root: PathBuf,
    branch: String,
    head: String,
    _ownership: FileLock,
    _worktree_ownership: FileLock,
    files: SnapshotFiles,
    committed_files: SnapshotFiles,
    pending_paths: BTreeSet<PathBuf>,
    model: Arc<ModelManager>,
    model_revision: String,
    exclusions: ExclusionSet,
    options: ModelBuildOptions,
    recovery_error: Option<String>,
    recovery_attempt: Option<RecoveryAttempt>,
    enable_commits: bool,
    #[cfg(unix)]
    track_file_mode: bool,
}
struct RecoveryAttempt {
    commit: String,
    files: SnapshotFiles,
    changed: BTreeSet<PathBuf>,
    original_index: Vec<u8>,
}
/// Immutable inputs and identity pinned when a worker read is admitted.
pub struct ReadSnapshot {
    files: SnapshotFiles,
    model: Arc<ModelManager>,
    metadata: Value,
}
impl ReadSnapshot {
    pub(crate) fn execute(
        &self,
        call: impl FnOnce() -> Result<Value, rmcp::ErrorData>,
    ) -> Result<Value, rmcp::ErrorData> {
        let scope = self.files.enter(Some(Arc::clone(&self.model)));
        let result = call();
        if scope.prepared().is_some() {
            return Err(rmcp::ErrorData::internal_error(
                "Snapshot read attempted to prepare changes",
                None,
            ));
        }
        result
    }
    pub(crate) fn status(&self, mut current: Value) -> Value {
        // Availability/recovery and physical Git observations are current, but
        // the result's accepted model identity must stay pinned to this read.
        current
            .as_object_mut()
            .expect("context status object")
            .extend(
                self.metadata
                    .as_object()
                    .expect("snapshot metadata object")
                    .clone(),
            );
        current
    }
}
fn error(message: impl Into<String>) -> ReqvireError {
    ReqvireError::ProcessError(message.into())
}

#[cfg(test)]
thread_local! {
    static GIT_TIMEOUT: std::cell::Cell<std::time::Duration> = const {
        std::cell::Cell::new(std::time::Duration::from_secs(30))
    };
}

#[cfg(test)]
pub fn with_git_timeout<T>(timeout: std::time::Duration, call: impl FnOnce() -> T) -> T {
    struct Reset(std::time::Duration);
    impl Drop for Reset {
        fn drop(&mut self) {
            GIT_TIMEOUT.with(|value| value.set(self.0));
        }
    }
    let _reset = Reset(GIT_TIMEOUT.with(|value| value.replace(timeout)));
    call()
}
pub fn git(
    root: &Path,
    args: &[&str],
    input: Option<&[u8]>,
    index: Option<&Path>,
) -> Result<Vec<u8>, ReqvireError> {
    git_result(root, args, input, index).map_err(|failure| failure.error)
}

fn git_result(
    root: &Path,
    args: &[&str],
    input: Option<&[u8]>,
    index: Option<&Path>,
) -> Result<Vec<u8>, crate::mcp_process::Failure> {
    let mut command = Command::new("git");
    command
        .current_dir(root)
        .args(args)
        .env("GIT_LITERAL_PATHSPECS", "1");
    if let Some(index) = index {
        command.env("GIT_INDEX_FILE", index);
    }
    #[cfg(test)]
    let timeout = GIT_TIMEOUT.with(std::cell::Cell::get);
    #[cfg(not(test))]
    let timeout = std::time::Duration::from_secs(30);
    let output = crate::mcp_process::run_binary(command, input, timeout, None)?;
    if !output.success {
        return Err(crate::mcp_process::Failure {
            error: error(format!(
                "MCP Git operation {} failed: {}",
                args.first().unwrap_or(&"operation"),
                String::from_utf8_lossy(&output.stderr).trim()
            )),
            outcome_unknown: false,
        });
    }
    Ok(output.stdout)
}
pub fn git_text(root: &Path, args: &[&str]) -> Result<String, ReqvireError> {
    Ok(String::from_utf8(git(root, args, None, None)?)
        .map_err(|e| error(e.to_string()))?
        .trim()
        .to_string())
}

/// One checkout observation, never retained across requests or state transitions.
pub struct GitCheckout {
    branch: Option<String>,
    head: Option<String>,
}
impl GitCheckout {
    pub(crate) fn read(root: &Path) -> Result<Self, ReqvireError> {
        // One process, without scanning the index/worktree just to check HEAD.
        match git_text(root, &["rev-parse", "HEAD", "--symbolic-full-name", "HEAD"]) {
            Ok(output) => {
                let (head, branch) = output
                    .split_once('\n')
                    .ok_or_else(|| error("Invalid Git checkout observation"))?;
                Ok(Self {
                    branch: (branch != "HEAD").then(|| branch.to_owned()),
                    head: Some(head.to_owned()),
                })
            }
            // An unborn named branch has no resolvable HEAD. Preserve its label
            // for read-only metadata; it cannot match a mutation session's HEAD.
            Err(failure) => git_text(root, &["symbolic-ref", "--quiet", "HEAD"]).map_or(
                Err(failure),
                |branch| {
                    Ok(Self {
                        branch: Some(branch),
                        head: None,
                    })
                },
            ),
        }
    }
    pub(crate) fn branch_name(&self) -> &str {
        self.branch
            .as_deref()
            .map(|b| b.strip_prefix("refs/heads/").unwrap_or(b))
            .unwrap_or("Detached HEAD")
    }
    pub(crate) fn read_status(&self, root: &Path) -> Value {
        serde_json::json!({"workspace_root":root, "branch":self.branch_name(),
            "head":self.head, "available":true, "owned":false})
    }
}

/// A physical status scan supplies checkout and dirty metadata together. NUL
/// records prevent unusual file names from being interpreted as branch headers.
struct GitObservation {
    checkout: GitCheckout,
    dirty: bool,
}
impl GitObservation {
    fn read(root: &Path) -> Result<Self, ReqvireError> {
        let bytes = git(
            root,
            &[
                "--no-optional-locks",
                "status",
                "--porcelain=v2",
                "--branch",
                "--no-ahead-behind",
                "--untracked-files=all",
                "-z",
            ],
            None,
            None,
        )?;
        let mut branch = None;
        let mut head = None;
        let mut dirty = false;
        let mut records = bytes.split(|b| *b == 0).filter(|r| !r.is_empty());
        while let Some(record) = records.next() {
            if let Some(value) = record.strip_prefix(b"# branch.head ") {
                branch = Some(
                    std::str::from_utf8(value)
                        .map_err(|e| error(e.to_string()))?
                        .to_owned(),
                );
            } else if let Some(value) = record.strip_prefix(b"# branch.oid ") {
                head = Some(
                    std::str::from_utf8(value)
                        .map_err(|e| error(e.to_string()))?
                        .to_owned(),
                );
            } else if !record.starts_with(b"# ") {
                dirty = true;
                if record.starts_with(b"2 ") {
                    // Renames/copies append a raw original pathname record.
                    // Even a pathname resembling a header is still file data.
                    records.next();
                }
            }
        }
        let branch = branch.ok_or_else(|| error("Missing Git status branch"))?;
        let head = head.ok_or_else(|| error("Missing Git status HEAD"))?;
        Ok(Self {
            checkout: GitCheckout {
                branch: (branch != "(detached)").then(|| format!("refs/heads/{branch}")),
                head: (head != "(initial)").then_some(head),
            },
            dirty,
        })
    }
}
#[cfg(unix)]
fn claim(file: &File) -> std::io::Result<()> {
    rustix::fs::flock(file, rustix::fs::FlockOperation::NonBlockingLockExclusive)
        .map_err(Into::into)
}
#[cfg(windows)]
fn claim(file: &File) -> std::io::Result<()> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::{
        Storage::FileSystem::{LockFileEx, LOCKFILE_EXCLUSIVE_LOCK, LOCKFILE_FAIL_IMMEDIATELY},
        System::IO::OVERLAPPED,
    };
    // The synchronous handle remains open for the session; Windows releases the
    // byte-range lock on close/process exit. No asynchronous operation is used.
    let mut overlapped: OVERLAPPED = unsafe { std::mem::zeroed() };
    let result = unsafe {
        LockFileEx(
            file.as_raw_handle(),
            LOCKFILE_EXCLUSIVE_LOCK | LOCKFILE_FAIL_IMMEDIATELY,
            0,
            1,
            0,
            &mut overlapped,
        )
    };
    if result == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(unix)]
fn release(file: &File) -> std::io::Result<()> {
    rustix::fs::flock(file, rustix::fs::FlockOperation::Unlock).map_err(Into::into)
}

#[cfg(windows)]
fn release(file: &File) -> std::io::Result<()> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::{Storage::FileSystem::UnlockFileEx, System::IO::OVERLAPPED};
    let mut overlapped: OVERLAPPED = unsafe { std::mem::zeroed() };
    let result = unsafe { UnlockFileEx(file.as_raw_handle(), 0, 1, 0, &mut overlapped) };
    if result == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn lock_contention(failure: &std::io::Error) -> bool {
    #[cfg(unix)]
    {
        failure.kind() == std::io::ErrorKind::WouldBlock
    }
    #[cfg(windows)]
    {
        failure.raw_os_error() == Some(windows_sys::Win32::Foundation::ERROR_LOCK_VIOLATION as i32)
    }
}

fn unsupported_lock(failure: &std::io::Error) -> bool {
    if failure.kind() == std::io::ErrorKind::Unsupported {
        return true;
    }
    #[cfg(unix)]
    {
        failure.raw_os_error() == Some(rustix::io::Errno::NOSYS.raw_os_error())
    }
    #[cfg(windows)]
    {
        failure.raw_os_error()
            == Some(windows_sys::Win32::Foundation::ERROR_INVALID_FUNCTION as i32)
    }
}

fn lock_error(path: &Path, operation: &str, failure: std::io::Error) -> ReqvireError {
    let guidance = if operation == "acquire" && lock_contention(&failure) {
        "already owned or busy; another operation holds this lock"
    } else if failure.kind() == std::io::ErrorKind::PermissionDenied {
        "permission denied; check this file and its directory permissions for the server/container user"
    } else if operation == "acquire" && unsupported_lock(&failure) {
        "filesystem locking is unsupported; use a filesystem with exclusive lock support"
    } else {
        "lock operation failed; ownership was not acquired"
    };
    error(format!(
        "Cannot {operation} MCP lock '{}': {failure}; {guidance}. Leave lock files in place.",
        path.display()
    ))
}

fn open_lock(path: &Path) -> std::io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
}

/// Owns one OS lock, releasing it explicitly before closing the handle.
/// An incidental fork/exec descriptor copy must not extend the owner's lifetime.
/// The handle is private and cannot be cloned or transferred out of this guard.
#[derive(Debug)]
pub struct FileLock {
    file: File,
    path: PathBuf,
}

impl Drop for FileLock {
    fn drop(&mut self) {
        if let Err(failure) = release(&self.file) {
            log::error!(
                "Cannot release MCP lock '{}': {failure}",
                self.path.display()
            );
        }
    }
}

// Shared by context ownership and repository administration. Never remove the
// filename to release a lock: other contenders must continue using its inode.
pub fn lock_file(path: &Path) -> Result<FileLock, ReqvireError> {
    lock_file_with(path, open_lock, claim)
}

fn lock_file_with(
    path: &Path,
    open: impl FnOnce(&Path) -> std::io::Result<File>,
    acquire: impl FnOnce(&File) -> std::io::Result<()>,
) -> Result<FileLock, ReqvireError> {
    let file = open(path).map_err(|failure| lock_error(path, "open", failure))?;
    acquire(&file).map_err(|failure| lock_error(path, "acquire", failure))?;
    Ok(FileLock {
        file,
        path: path.to_path_buf(),
    })
}

impl MutationSession {
    pub fn start(
        exclusions: &ExclusionSet,
        with_size_estimates: bool,
        enable_commits: bool,
    ) -> Result<Self, ReqvireError> {
        let scope = reqvire::workspace::WorkspaceScope::discover()?;
        if scope.git_worktrees.len() != 1 || scope.git_worktrees[0] != scope.root {
            return Err(error(
                "Mutation-enabled MCP requires one Git worktree at the workspace root",
            ));
        }
        let root = scope.root;
        let branch = git_text(&root, &["symbolic-ref", "--quiet", "HEAD"]).map_err(|_| {
            error("Mutation-enabled MCP requires a named branch (detached HEAD is not supported)")
        })?;
        let common = PathBuf::from(git_text(
            &root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?);
        let lock_path = common.join(format!(
            "reqvire-mcp-{}.lock",
            reqvire::utils::hash_content(&branch)
        ));
        let ownership = lock_file(&lock_path)?;
        let worktree_lock = root.join(git_text(
            &root,
            &["rev-parse", "--git-path", "reqvire-mcp-worktree.lock"],
        )?);
        let worktree_ownership = lock_file(&worktree_lock)?;
        require_clean(&root)?;
        let head = git_text(&root, &["rev-parse", "--verify", "HEAD"])
            .map_err(|_| error("Mutation-enabled MCP requires an existing commit"))?;
        git_text(&root, &["var", "GIT_AUTHOR_IDENT"])?;
        git_text(&root, &["var", "GIT_COMMITTER_IDENT"])?;
        for marker in [
            "MERGE_HEAD",
            "CHERRY_PICK_HEAD",
            "REVERT_HEAD",
            "rebase-merge",
            "rebase-apply",
        ] {
            let path = git_text(&root, &["rev-parse", "--git-path", marker])?;
            if root.join(path).exists() {
                return Err(error(
                    "Finish the in-progress Git operation before starting mutation-enabled MCP",
                ));
            }
        }
        let mut files = SnapshotFiles {
            root: root.clone(),
            files: Arc::new(BTreeMap::new()),
            executable: Arc::new(BTreeSet::new()),
        };
        // Git's executable bit is authoritative even when the filesystem cannot
        // represent it (core.filemode=false, e.g. a Windows-backed bind mount).
        for entry in git(&root, &["ls-files", "--stage", "-z"], None, None)?
            .split(|b| *b == 0)
            .filter(|p| !p.is_empty())
        {
            let entry = std::str::from_utf8(entry).map_err(|e| error(e.to_string()))?;
            let (metadata, path) = entry
                .split_once('\t')
                .ok_or_else(|| error("Invalid Git index entry"))?;
            let path = root.join(path);
            if metadata.starts_with("100755 ") {
                Arc::make_mut(&mut files.executable).insert(path.clone());
            }
            if path.is_file() {
                Arc::make_mut(&mut files.files).insert(path.clone(), Arc::new(fs::read(path)?));
            }
        }
        let options = ModelBuildOptions {
            lenient: false,
            with_size_estimates,
        };
        let capture = files.enter(None);
        capture.capture_dependencies();
        let mut model = ModelManager::new();
        model.parse_and_validate_with_options(None, exclusions, options)?;
        if let Some((captured, _)) = capture.prepared() {
            files = captured;
        }
        require_clean(&root)?;
        let model_revision = reqvire::model_fingerprint(&model.graph_registry.get_all_elements())?;
        #[cfg(unix)]
        let track_file_mode = git_text(
            &root,
            &[
                "config",
                "--bool",
                "--default",
                "true",
                "--get",
                "core.filemode",
            ],
        )? == "true";
        let session = Self {
            root,
            branch,
            head,
            _ownership: ownership,
            _worktree_ownership: worktree_ownership,
            committed_files: files.clone(),
            pending_paths: BTreeSet::new(),
            files,
            model: Arc::new(model),
            model_revision,
            exclusions: exclusions.clone(),
            options,
            recovery_error: None,
            recovery_attempt: None,
            enable_commits,
            #[cfg(unix)]
            track_file_mode,
        };
        session.check_head()?;
        Ok(session)
    }
    pub fn model(&self) -> Arc<ModelManager> {
        Arc::clone(&self.model)
    }
    pub fn branch_name(&self) -> &str {
        self.branch
            .strip_prefix("refs/heads/")
            .unwrap_or(&self.branch)
    }
    pub fn check_head(&self) -> Result<(), ReqvireError> {
        self.check_recovery()?;
        self.check_checkout(&GitCheckout::read(&self.root)?)
    }
    fn check_recovery(&self) -> Result<(), ReqvireError> {
        if let Some(reason) = &self.recovery_error {
            return Err(error(format!(
                "MCP session requires recovery; further mutations are disabled: {reason}"
            )));
        }
        Ok(())
    }
    fn check_checkout(&self, observed: &GitCheckout) -> Result<(), ReqvireError> {
        self.check_recovery()?;
        if observed.branch.as_deref() != Some(self.branch.as_str())
            || observed.head.as_deref() != Some(self.head.as_str())
        {
            return Err(error("MCP owned branch or HEAD changed externally; restart the server before further mutations"));
        }
        Ok(())
    }
    pub(crate) fn capture_read(&self, tool: &str) -> Result<ReadSnapshot, ReqvireError> {
        if !snapshot_read(tool) {
            return Err(error("Operation is not an audited snapshot read"));
        }
        if self.recovery_error.is_none() {
            self.check_head()?;
        }
        Ok(ReadSnapshot {
            files: self.files.clone(),
            model: self.model(),
            metadata: self.accepted_metadata(),
        })
    }
    fn accepted_metadata(&self) -> Value {
        serde_json::json!({
            "workspace_root": self.root, "branch": self.branch_name(),
            "head": self.head, "enable_commits": self.enable_commits,
            "model_revision": self.model_revision,
            "pending_changes": self.pending_paths().iter().filter_map(|p| p.strip_prefix(&self.root).ok()).collect::<Vec<_>>(),
        })
    }
    pub fn execute(
        &mut self,
        persists: bool,
        tool: &str,
        call: impl FnOnce() -> Result<Value, rmcp::ErrorData>,
    ) -> (Result<Value, rmcp::ErrorData>, bool) {
        let (mut result, changed) = match self.execute_inner(persists, tool, call) {
            Ok(result) => result,
            Err(error) => (Ok(crate::mcp::tool_error(tool, error)), false),
        };
        if self.recovery_error.is_some() {
            if let Ok(value) = &mut result {
                value["_meta"]["reqvire/context"] = self.status();
            }
        }
        (result, changed)
    }
    fn execute_inner(
        &mut self,
        persists: bool,
        tool: &str,
        call: impl FnOnce() -> Result<Value, rmcp::ErrorData>,
    ) -> Result<(Result<Value, rmcp::ErrorData>, bool), ReqvireError> {
        // A live session retains its immutable accepted graph/files even when
        // rollback fails. Only audited snapshot reads may bypass write recovery.
        if self.recovery_error.is_some() {
            if persists || !snapshot_read(tool) {
                return Err(error(format!(
                    "MCP operation {tool} requires recovery before execution: {}",
                    self.recovery_error.as_deref().unwrap_or_default()
                )));
            }
        } else {
            self.check_head()?;
        }
        let scope = self.files.enter(Some(Arc::clone(&self.model)));
        let result = call();
        // Keep protocol errors and tool rejection envelopes intact.
        let mut result = match result {
            Ok(value) => value,
            Err(error) => return Ok((Err(error), false)),
        };
        if result.get("isError").and_then(Value::as_bool) == Some(true) {
            return Ok((Ok(result), false));
        }
        let Some((candidate_files, changed)) = scope.prepared() else {
            return Ok((Ok(result), false));
        };
        let changed: BTreeSet<_> = changed
            .into_iter()
            .filter(|p| {
                self.files.files.get(p) != candidate_files.files.get(p)
                    || self.files.executable.contains(p) != candidate_files.executable.contains(p)
            })
            .collect();
        if changed.is_empty() {
            return Ok((Ok(result), false));
        }
        let scope = candidate_files.enter(None);
        let mut candidate = ModelManager::new();
        candidate.parse_and_validate_with_options(None, &self.exclusions, self.options)?;
        drop(scope);
        if !persists {
            return Ok((Ok(result), false));
        }
        let revision = reqvire::model_fingerprint(&candidate.graph_registry.get_all_elements())?;
        self.check_head()?;
        let (commit, reconciled) = match self.persist_candidate(&candidate_files, &changed, tool) {
            Ok(commit) => (commit, false),
            Err(failure) => {
                let Some(attempt) = &self.recovery_attempt else {
                    return Err(failure);
                };
                let commit = attempt.commit.clone();
                // Persistence has released its index lock. Observe and verify
                // the recorded publication instead of repeating update-ref.
                self.resolve_recorded_commit(&commit)?;
                (Some(commit), true)
            }
        };
        if let Some(commit) = &commit {
            self.head = commit.clone();
        }
        self.files = candidate_files;
        if self.enable_commits {
            self.committed_files = self.files.clone();
            self.pending_paths.clear();
        } else {
            for path in changed {
                if self.files.files.get(&path) != self.committed_files.files.get(&path)
                    || self.files.executable.contains(&path)
                        != self.committed_files.executable.contains(&path)
                {
                    self.pending_paths.insert(path);
                } else {
                    self.pending_paths.remove(&path);
                }
            }
        }
        self.model = Arc::new(candidate);
        self.model_revision = revision;
        reqvire::model_cache::invalidate();
        if let (Some(commit), Some(content)) = (
            commit,
            result
                .get_mut("structuredContent")
                .and_then(Value::as_object_mut),
        ) {
            content.insert("commit".into(), Value::String(commit));
            if reconciled {
                content.insert("reconciled".into(), Value::Bool(true));
            }
            result["content"] = serde_json::json!([{ "type": "text", "text": serde_json::to_string_pretty(content)? }]);
        }
        Ok((Ok(result), true))
    }
    pub fn status(&self) -> Value {
        let observed = GitObservation::read(&self.root);
        let (writable, dirty) = match observed {
            Ok(observed) => (
                self.check_checkout(&observed.checkout),
                Some(observed.dirty),
            ),
            // Dirty-state failure alone does not invalidate a valid checkout.
            Err(_) => (self.check_head(), None),
        };
        let mut status = self.accepted_metadata();
        let live = serde_json::json!({
            "dirty": dirty, "available": writable.is_ok() || self.recovery_error.is_some(),
            "writes_available": writable.is_ok(), "recovery_required": self.recovery_error.is_some(),
            "diagnostic": self.recovery_error.clone().or_else(|| writable.err().map(|e| e.to_string())),
            "recovery_attempt": self.recovery_attempt.as_ref().map(|attempt| serde_json::json!({
                "accepted_head":self.head, "attempted_commit":attempt.commit,
                "affected_paths":attempt.changed.iter().filter_map(|p|p.strip_prefix(&self.root).ok()).collect::<Vec<_>>(),
                "reconcile_tool":"reqvire.git.reconcile",
            })),
            "model_source":"accepted_snapshot", "git_source":"live_observation",
        });
        status
            .as_object_mut()
            .expect("context status object")
            .extend(
                live.as_object()
                    .expect("live context status is an object")
                    .clone(),
            );
        status
    }
    const fn pending_paths(&self) -> &BTreeSet<PathBuf> {
        &self.pending_paths
    }
    pub fn explicit_commit(&mut self, message: &str) -> Result<Value, ReqvireError> {
        if message.trim().is_empty() {
            return Err(error("Commit message must not be empty"));
        }
        self.check_head()?;
        let changed = self.pending_paths().clone();
        if changed.is_empty() {
            return Ok(serde_json::json!({"outcome":"no_op", "head": self.head}));
        }
        let files = self.files.clone();
        let scope = files.enter(None);
        let mut candidate = ModelManager::new();
        candidate.parse_and_validate_with_options(None, &self.exclusions, self.options)?;
        drop(scope);
        let mut publication = self.prepare_commit(&files, &changed, message)?;
        self.check_head()?;
        if let Err(failure) = git_result(
            &self.root,
            &[
                "update-ref",
                "-m",
                "reqvire: explicit commit",
                &self.branch,
                &publication.commit,
                &self.head,
            ],
            None,
            None,
        ) {
            if failure.outcome_unknown {
                // The owned process has stopped. An unchanged branch confirms
                // failure; pending accepted edits remain available to commit.
                if self.ref_at_accepted_head() {
                    return Err(error(format!(
                        "Explicit commit was not published; pending changes retained: {}",
                        failure.error
                    )));
                }
                let reason = format!(
                    "Explicit commit ref publication failed for {}: accepted HEAD {}, attempted commit {}; {}; further writes disabled",
                    self.branch, self.head, publication.commit, failure.error
                );
                self.recovery_error = Some(reason);
                self.record_attempt(&publication, &files, &changed);
                let commit = publication.commit.clone();
                drop(publication);
                return self.resolve_recorded_commit(&commit);
            }
            return Err(failure.error);
        }
        if let Err(failure) = fs::rename(&publication.index_lock.path, &publication.index_path) {
            // Compare-and-swap rollback never overwrites a subsequent external ref update.
            if git(
                &self.root,
                &["update-ref", &self.branch, &self.head, &publication.commit],
                None,
                None,
            )
            .is_err()
            {
                let reason = "Explicit commit index publication failed; ref recovery failed; writes disabled";
                self.recovery_error = Some(reason.into());
                return Err(error(reason));
            }
            return Err(failure.into());
        }
        publication.index_lock.published = true;
        let old_head = std::mem::replace(&mut self.head, publication.commit);
        self.committed_files = files;
        self.pending_paths.clear();
        Ok(
            serde_json::json!({"outcome":"completed", "old_head":old_head, "head":self.head,
            "commit":self.head, "changed_files":changed.iter().filter_map(|p| p.strip_prefix(&self.root).ok()).collect::<Vec<_>>() }),
        )
    }
    pub fn with_snapshot<T>(&self, call: impl FnOnce(&ModelManager) -> T) -> T {
        let _scope = self.files.enter(Some(Arc::clone(&self.model)));
        call(&self.model)
    }
    /// Complete only the recorded publication. Never issue another ref update.
    pub fn reconcile_commit(&mut self, commit: &str, dry_run: bool) -> Result<Value, ReqvireError> {
        let attempt = self.recovery_attempt.as_ref().filter(|_| self.recovery_error.is_some())
            .ok_or_else(|| error("No recorded unknown-outcome commit to reconcile; use normal admission for other recovery failures"))?;
        if commit != attempt.commit {
            return Err(error(
                "Reconciliation requires the exact recorded attempted_commit",
            ));
        }
        self.verify_attempt_checkout(attempt)?;
        let parents = git_text(&self.root, &["rev-list", "--parents", "-n", "1", commit])?;
        if parents != format!("{commit} {}", self.head) {
            return Err(error(
                "Attempted commit does not have the accepted HEAD as its sole parent",
            ));
        }
        let changed = git(
            &self.root,
            &[
                "diff-tree",
                "--no-commit-id",
                "--name-only",
                "-r",
                "-z",
                &self.head,
                commit,
            ],
            None,
            None,
        )?;
        let changed: BTreeSet<_> = nul_paths(&self.root, &changed)?;
        if changed != attempt.changed {
            return Err(error(
                "Attempted commit's affected paths differ from the recorded candidate",
            ));
        }
        let scope = attempt.files.enter(None);
        let mut model = ModelManager::new();
        model.parse_and_validate_with_options(None, &self.exclusions, self.options)?;
        drop(scope);
        let revision = reqvire::model_fingerprint(&model.graph_registry.get_all_elements())?;
        self.verify_attempt_files(attempt)?;

        let index_path = self
            .root
            .join(git_text(&self.root, &["rev-parse", "--git-path", "index"])?);
        let mut lock = IndexLock {
            file: OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(index_path.with_file_name("index.lock"))?,
            path: index_path.with_file_name("index.lock"),
            published: false,
        };
        let old_index = tempfile::NamedTempFile::new()?;
        fs::write(old_index.path(), &attempt.original_index)?;
        let current_index = fs::read(&index_path)?;
        let merged = tempfile::NamedTempFile::new()?;
        fs::write(merged.path(), &current_index)?;
        let expected = tempfile::NamedTempFile::new()?.into_temp_path();
        fs::remove_file(&expected)?;
        git(&self.root, &["read-tree", commit], None, Some(&expected))?;
        let before = index_entries(&self.root, old_index.path())?;
        let current = index_entries(&self.root, merged.path())?;
        let target = index_entries(&self.root, &expected)?;
        for path in before
            .keys()
            .chain(current.keys())
            .chain(attempt.changed.iter())
        {
            if attempt.changed.contains(path) {
                if current.get(path) != before.get(path) && current.get(path) != target.get(path) {
                    return Err(error(format!(
                        "Affected index entry changed outside the attempt: {}",
                        path.display()
                    )));
                }
            } else if current.get(path) != before.get(path) {
                return Err(error("Unrelated index entries changed after the attempt; preserve and resolve them before reconciliation"));
            }
        }
        let mut entries = Vec::new();
        for path in &attempt.changed {
            let relative = path
                .strip_prefix(&self.root)
                .map_err(|e| error(e.to_string()))?
                .to_str()
                .ok_or_else(|| error("Non-UTF8 reconciliation path"))?;
            let expected_entry = if let Some(bytes) = attempt.files.files.get(path) {
                let oid = String::from_utf8(git(
                    &self.root,
                    &["hash-object", "--stdin"],
                    Some(bytes),
                    None,
                )?)
                .map_err(|e| error(e.to_string()))?;
                let mode = if attempt.files.executable.contains(path) {
                    "100755"
                } else {
                    "100644"
                };
                format!("{mode} {} 0\t{relative}", oid.trim())
            } else {
                format!("0 {}\t{relative}", "0".repeat(self.head.len()))
            };
            if attempt.files.files.contains_key(path)
                && target.get(path) != Some(&vec![expected_entry.as_bytes().to_vec()])
            {
                return Err(error(
                    "Attempted commit content or mode differs from the recorded candidate",
                ));
            }
            if !attempt.files.files.contains_key(path) && target.contains_key(path) {
                return Err(error("Attempted commit retained a recorded deletion"));
            }
            entries.extend_from_slice(expected_entry.as_bytes());
            entries.push(0);
        }
        git(
            &self.root,
            &["update-index", "-z", "--index-info"],
            Some(&entries),
            Some(merged.path()),
        )?;
        self.verify_attempt_checkout(attempt)?;
        self.verify_attempt_files(attempt)?;
        let result = serde_json::json!({"outcome":if dry_run {"ready"} else {"completed"},
            "dry_run":dry_run, "old_head":self.head, "head":commit, "commit":commit,
            "changed_files":attempt.changed.iter().filter_map(|p|p.strip_prefix(&self.root).ok()).collect::<Vec<_>>()});
        if dry_run {
            return Ok(result);
        }
        lock.file.write_all(&fs::read(merged.path())?)?;
        lock.file.sync_all()?;
        fs::rename(&lock.path, &index_path)?;
        lock.published = true;
        // Final observation cannot import an external checkout into accepted state.
        self.verify_attempt_checkout(attempt)?;
        let model_changed = self.files.files != attempt.files.files
            || self.files.executable != attempt.files.executable;
        self.files = attempt.files.clone();
        self.committed_files = self.files.clone();
        self.pending_paths.clear();
        self.head = commit.to_owned();
        // Explicit commits accept the same files/model that were already
        // published by mutations; preserve their model identity.
        if model_changed {
            self.model = Arc::new(model);
        }
        self.model_revision = revision;
        self.recovery_attempt = None;
        self.recovery_error = None;
        reqvire::model_cache::invalidate();
        Ok(result)
    }
    fn ref_at_accepted_head(&self) -> bool {
        GitCheckout::read(&self.root).is_ok_and(|observed| {
            observed.branch.as_deref() == Some(&self.branch)
                && observed.head.as_deref() == Some(&self.head)
        })
    }
    fn resolve_recorded_commit(&mut self, commit: &str) -> Result<Value, ReqvireError> {
        match self.reconcile_commit(commit, false) {
            Ok(mut result) => {
                result["reconciled"] = Value::Bool(true);
                Ok(result)
            }
            Err(failure) => {
                let reason = format!(
                    "{}; automatic reconciliation failed: {failure}",
                    self.recovery_error.as_deref().unwrap_or_default()
                );
                self.recovery_error = Some(reason.clone());
                Err(error(reason))
            }
        }
    }
    fn verify_attempt_checkout(&self, attempt: &RecoveryAttempt) -> Result<(), ReqvireError> {
        let observed = GitCheckout::read(&self.root)?;
        if observed.branch.as_deref() != Some(&self.branch)
            || observed.head.as_deref() != Some(&attempt.commit)
        {
            return Err(error(
                "Reconciliation requires the owned branch at the exact recorded attempted commit",
            ));
        }
        for marker in [
            "MERGE_HEAD",
            "CHERRY_PICK_HEAD",
            "REVERT_HEAD",
            "rebase-merge",
            "rebase-apply",
        ] {
            let path = git_text(&self.root, &["rev-parse", "--git-path", marker])?;
            if self.root.join(path).exists() {
                return Err(error(
                    "Finish the in-progress Git operation before reconciliation",
                ));
            }
        }
        Ok(())
    }
    fn verify_attempt_files(&self, attempt: &RecoveryAttempt) -> Result<(), ReqvireError> {
        for path in attempt.files.files.keys().chain(attempt.changed.iter()) {
            ensure_regular_destination(&self.root, path)?;
            match attempt.files.files.get(path) {
                Some(bytes) => {
                    let metadata = fs::symlink_metadata(path)?;
                    if !metadata.is_file() || fs::read(path)? != **bytes {
                        return Err(error(format!(
                            "Worktree content differs from the recorded candidate: {}",
                            path.display()
                        )));
                    }
                    #[cfg(unix)]
                    if self.track_file_mode {
                        use std::os::unix::fs::PermissionsExt;
                        if (metadata.permissions().mode() & 0o100 != 0)
                            != attempt.files.executable.contains(path)
                        {
                            return Err(error(
                                "Worktree executable mode differs from the recorded candidate",
                            ));
                        }
                    }
                }
                None => match fs::symlink_metadata(path) {
                    Ok(_) => return Err(error("Worktree retained a recorded deletion")),
                    Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => {}
                    Err(failure) => return Err(failure.into()),
                },
            }
        }
        let untracked = git(
            &self.root,
            &["ls-files", "--others", "--exclude-standard", "-z"],
            None,
            None,
        )?;
        if nul_paths(&self.root, &untracked)?
            .iter()
            .any(|p| !attempt.files.files.contains_key(p))
        {
            return Err(error("Untracked files outside the recorded candidate must be preserved and resolved before reconciliation"));
        }
        Ok(())
    }
    fn prepare_commit(
        &self,
        files: &SnapshotFiles,
        changed: &BTreeSet<PathBuf>,
        message: &str,
    ) -> Result<PreparedCommit, ReqvireError> {
        let candidate_index = tempfile::NamedTempFile::new()?.into_temp_path();
        fs::remove_file(&candidate_index)?;
        git(
            &self.root,
            &["read-tree", &self.head],
            None,
            Some(&candidate_index),
        )?;
        let mut entries = Vec::new();
        for path in changed {
            let relative = path
                .strip_prefix(&self.root)
                .map_err(|e| error(e.to_string()))?
                .to_str()
                .ok_or_else(|| error("Non-UTF8 mutation path"))?;
            let (mode, oid) = if let Some(bytes) = files.files.get(path) {
                let oid = String::from_utf8(git(
                    &self.root,
                    &["hash-object", "-w", "--stdin"],
                    Some(bytes),
                    None,
                )?)
                .map_err(|e| error(e.to_string()))?;
                let mode = if files.executable.contains(path) {
                    "100755"
                } else {
                    "100644"
                };
                (mode, oid.trim().to_string())
            } else {
                ("0", "0".repeat(self.head.len()))
            };
            entries.extend_from_slice(format!("{mode} {oid}\t{relative}\0").as_bytes());
        }
        git(
            &self.root,
            &["update-index", "-z", "--index-info"],
            Some(&entries),
            Some(&candidate_index),
        )?;
        let tree = String::from_utf8(git(
            &self.root,
            &["write-tree"],
            None,
            Some(&candidate_index),
        )?)
        .map_err(|e| error(e.to_string()))?;
        let commit = git_text(
            &self.root,
            &["commit-tree", tree.trim(), "-p", &self.head, "-m", message],
        )?;
        // Hold Git's real index lock across persistence and reference publication.
        // A separate index preserves unrelated staged changes without including
        // any of them in the commit tree above.
        let index_path = self
            .root
            .join(git_text(&self.root, &["rev-parse", "--git-path", "index"])?);
        let index_lock_path = index_path.with_file_name("index.lock");
        let mut index_lock = IndexLock {
            file: OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&index_lock_path)?,
            path: index_lock_path,
            published: false,
        };
        let old_index = fs::read(&index_path)?;
        let merged_index = tempfile::NamedTempFile::new()?;
        fs::write(merged_index.path(), &old_index)?;
        git(
            &self.root,
            &["update-index", "-z", "--index-info"],
            Some(&entries),
            Some(merged_index.path()),
        )?;
        index_lock.file.write_all(&fs::read(merged_index.path())?)?;
        index_lock.file.sync_all()?;
        Ok(PreparedCommit {
            commit,
            index_path,
            index_lock,
            original_index: old_index,
        })
    }
    fn record_attempt(
        &mut self,
        publication: &PreparedCommit,
        files: &SnapshotFiles,
        changed: &BTreeSet<PathBuf>,
    ) {
        self.recovery_attempt = Some(RecoveryAttempt {
            commit: publication.commit.clone(),
            files: files.clone(),
            changed: changed.clone(),
            original_index: publication.original_index.clone(),
        });
    }
    fn persist_candidate(
        &mut self,
        files: &SnapshotFiles,
        changed: &BTreeSet<PathBuf>,
        tool: &str,
    ) -> Result<Option<String>, ReqvireError> {
        for path in changed {
            ensure_regular_destination(&self.root, path)?;
            if !self.files.files.contains_key(path) && fs::symlink_metadata(path).is_ok() {
                return Err(error(format!(
                    "MCP destination appeared outside the accepted snapshot: {}",
                    path.display()
                )));
            }
        }
        // No index or Git object writes in the default mode. Both modes use the
        // same validated candidate, persistence, and rollback boundary.
        let mut publication = if self.enable_commits {
            Some(self.prepare_commit(files, changed, &format!("reqvire: {tool}"))?)
        } else {
            None
        };
        self.check_head()?;
        let mut backups = BTreeMap::new();
        for path in changed {
            backups.insert(
                path.clone(),
                match fs::read(path) {
                    Ok(bytes) => Some((bytes, fs::metadata(path)?.permissions())),
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
                    Err(e) => return Err(e.into()),
                },
            );
        }
        let mut advanced = false;
        let mut attempted = Vec::new();
        let result = (|| {
            for path in changed {
                // A failed write may still have changed bytes. Recovery includes
                // this path but must not rewrite paths we have not attempted.
                attempted.push(path);
                persist(path, files.files.get(path).map(|b| b.as_slice()))?;
                #[cfg(unix)]
                if self.track_file_mode && files.files.contains_key(path) {
                    reconcile_executable(path, files.executable.contains(path))?;
                }
            }
            self.check_head()?;
            if let Some(publication) = &mut publication {
                git_result(
                    &self.root,
                    &[
                        "update-ref",
                        "-m",
                        &format!("reqvire: {tool}"),
                        &self.branch,
                        &publication.commit,
                        &self.head,
                    ],
                    None,
                    None,
                )
                .map_err(|failure| {
                    if failure.outcome_unknown && !self.ref_at_accepted_head() {
                        self.record_attempt(publication, files, changed);
                    }
                    error(format!(
                    "Git ref publication failed for {}: accepted HEAD {}, attempted commit {}; {}",
                    self.branch, self.head, publication.commit, failure.error
                ))
                })?;
                advanced = true;
                fs::rename(&publication.index_lock.path, &publication.index_path)?;
                publication.index_lock.published = true;
            }
            Ok::<_, ReqvireError>(())
        })();
        if let Err(failure) = result {
            if self.recovery_attempt.is_some() {
                let reason = format!(
                    "MCP mutation failed: {failure}; outcome unknown; further writes disabled"
                );
                self.recovery_error = Some(reason.clone());
                return Err(error(reason));
            }
            let recovery = (|| {
                let advanced_commit = publication.as_ref().filter(|_| advanced).map(|p| &p.commit);
                let expected = advanced_commit.unwrap_or(&self.head);
                if git_text(&self.root, &["symbolic-ref", "--quiet", "HEAD"])? != self.branch
                    || git_text(&self.root, &["rev-parse", "HEAD"])? != *expected
                {
                    return Err(error(
                        "Git checkout changed during persistence; automatic restoration is unsafe",
                    ));
                }
                if let Some(commit) = advanced_commit {
                    git(
                        &self.root,
                        &["update-ref", &self.branch, &self.head, commit],
                        None,
                        None,
                    )?;
                }
                let mut recovery_failure = None;
                for path in attempted.iter().rev() {
                    if let Err(failure) = restore_file(path, backups[*path].as_ref()) {
                        // Restore the remaining attempted paths even if one is
                        // unrecoverable; retain the first actionable diagnostic.
                        recovery_failure.get_or_insert(failure);
                    }
                }
                recovery_failure.map_or(Ok(()), Err)
            })();
            if let Err(recovery) = recovery {
                let reason = format!("MCP mutation failed: {failure}; recovery failed: {recovery}; further writes disabled");
                self.recovery_error = Some(reason.clone());
                return Err(error(reason));
            }
            return Err(failure);
        }
        Ok(publication.map(|p| p.commit))
    }
}
// Deliberately explicit: a new read-only tool may still depend on live Git or
// filesystem state and needs an audit before it can run during recovery.
pub fn snapshot_read(tool: &str) -> bool {
    matches!(
        tool,
        "reqvire.workspace_status"
            | "reqvire.model_revision"
            | "reqvire.tool_contract"
            | "reqvire.read_element"
            | "reqvire.search"
            | "reqvire.model"
            | "reqvire.containment"
            | "reqvire.collect"
            | "reqvire.submodels"
            | "reqvire.coverage"
            | "reqvire.traces"
            | "reqvire.resources"
            | "reqvire.lint"
            | "reqvire.concepts.list"
            | "reqvire.concepts.get"
            | "reqvire.concept_schemes.list"
            | "reqvire.concept_mappings.list"
            | "reqvire.semantic.export"
            | "reqvire.semantic.ontologies"
            | "reqvire.semantic.shapes"
            | "reqvire.semantic.concepts"
            | "reqvire.semantic.model"
            | "reqvire.semantic.graph"
            | "reqvire.semantic.prefixes"
            | "reqvire.semantic.vocabulary"
            | "reqvire.semantic.queries"
            | "reqvire.semantic.queries.validate"
            | "reqvire.semantic.sparql"
            | "resources/read"
            | "prompts/get"
            | "tools/list"
            | "resources/list"
            | "prompts/list"
    )
}
struct PreparedCommit {
    commit: String,
    index_path: PathBuf,
    index_lock: IndexLock,
    original_index: Vec<u8>,
}
fn nul_paths(root: &Path, bytes: &[u8]) -> Result<BTreeSet<PathBuf>, ReqvireError> {
    bytes
        .split(|b| *b == 0)
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            std::str::from_utf8(entry)
                .map(|path| root.join(path))
                .map_err(|e| error(e.to_string()))
        })
        .collect()
}
fn index_entries(
    root: &Path,
    index: &Path,
) -> Result<BTreeMap<PathBuf, Vec<Vec<u8>>>, ReqvireError> {
    let mut result: BTreeMap<PathBuf, Vec<Vec<u8>>> = BTreeMap::new();
    for entry in git(root, &["ls-files", "--stage", "-z"], None, Some(index))?
        .split(|b| *b == 0)
        .filter(|entry| !entry.is_empty())
    {
        let (_, path) = entry.split_at(
            entry
                .iter()
                .position(|b| *b == b'\t')
                .ok_or_else(|| error("Invalid index entry"))?
                + 1,
        );
        let path = root.join(std::str::from_utf8(path).map_err(|e| error(e.to_string()))?);
        result.entry(path).or_default().push(entry.to_vec());
    }
    Ok(result)
}
struct IndexLock {
    file: File,
    path: PathBuf,
    published: bool,
}
impl Drop for IndexLock {
    fn drop(&mut self) {
        if !self.published {
            let _ = fs::remove_file(&self.path);
        }
    }
}
pub fn require_clean(root: &Path) -> Result<(), ReqvireError> {
    if !git(
        root,
        &["status", "--porcelain=v1", "--untracked-files=all"],
        None,
        None,
    )?
    .is_empty()
    {
        return Err(error("Mutation-enabled MCP requires a clean index and worktree, including non-ignored untracked files"));
    }
    Ok(())
}
fn ensure_regular_destination(root: &Path, path: &Path) -> Result<(), ReqvireError> {
    let relative = path.strip_prefix(root).map_err(|e| error(e.to_string()))?;
    let mut cursor = root.to_path_buf();
    for component in relative.components() {
        cursor.push(component);
        if fs::symlink_metadata(&cursor).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err(error(format!(
                "MCP mutation refuses a symbolic-link destination: {}",
                cursor.display()
            )));
        }
    }
    Ok(())
}
fn persist(path: &Path, bytes: Option<&[u8]>) -> Result<(), ReqvireError> {
    #[cfg(all(test, unix))]
    permission_test_io::record_write(path);
    match bytes {
        Some(bytes) => {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(path, bytes)?;
        }
        None => match fs::remove_file(path) {
            Ok(()) => (),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
            Err(e) => return Err(e.into()),
        },
    }
    Ok(())
}

#[cfg(unix)]
fn reconcile_executable(path: &Path, executable: bool) -> Result<(), ReqvireError> {
    use std::os::unix::fs::PermissionsExt;
    let mode = fs::metadata(path)
        .map_err(|e| permission_error("read", path, e))?
        .permissions()
        .mode();
    // Git tracks the owner's execute bit. Group/other-only execute bits on an
    // existing file do not make its tracked mode executable.
    if (mode & 0o100 != 0) != executable {
        let mode = if executable {
            // Respect restrictive creation permissions when making a new asset
            // executable; do not grant group/other execution without read access.
            mode | ((mode & 0o444) >> 2) | 0o100
        } else {
            mode & !0o111
        };
        ensure_permissions(path, fs::Permissions::from_mode(mode))?;
    }
    Ok(())
}

fn permission_error(operation: &str, path: &Path, failure: std::io::Error) -> ReqvireError {
    error(format!(
        "MCP failed to {operation} permissions for {}: {failure}",
        path.display()
    ))
}

fn permissions_match(actual: &fs::Permissions, expected: &fs::Permissions) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        actual.mode() & 0o7777 == expected.mode() & 0o7777
    }
    #[cfg(not(unix))]
    {
        actual.readonly() == expected.readonly()
    }
}

fn ensure_permissions(path: &Path, expected: fs::Permissions) -> Result<(), ReqvireError> {
    let actual = fs::metadata(path)
        .map_err(|e| permission_error("read", path, e))?
        .permissions();
    if permissions_match(&actual, &expected) {
        return Ok(());
    }
    set_native_permissions(path, expected.clone()).map_err(|e| permission_error("set", path, e))?;
    let actual = fs::metadata(path)
        .map_err(|e| permission_error("verify", path, e))?
        .permissions();
    if !permissions_match(&actual, &expected) {
        return Err(error(format!(
            "MCP permissions did not change as required for {}",
            path.display()
        )));
    }
    Ok(())
}

fn restore_file(
    path: &Path,
    backup: Option<&(Vec<u8>, fs::Permissions)>,
) -> Result<(), ReqvireError> {
    let expected = backup.map(|(bytes, _)| bytes.as_slice());
    let read = || -> Result<Option<Vec<u8>>, ReqvireError> {
        match fs::read(path) {
            Ok(bytes) => Ok(Some(bytes)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(error(format!(
                "MCP failed to read recovery file {}: {e}",
                path.display()
            ))),
        }
    };
    if read()?.as_deref() != expected {
        persist(path, expected)
            .map_err(|e| error(format!("MCP failed to restore {}: {e}", path.display())))?;
    }
    if let Some((_, permissions)) = backup {
        ensure_permissions(path, permissions.clone())?;
    }
    if read()?.as_deref() != expected {
        return Err(error(format!(
            "MCP file recovery verification failed for {}",
            path.display()
        )));
    }
    Ok(())
}

fn set_native_permissions(path: &Path, permissions: fs::Permissions) -> std::io::Result<()> {
    #[cfg(all(test, unix))]
    if !permission_test_io::attempt(path)? {
        return Ok(());
    }
    fs::set_permissions(path, permissions)
}

#[cfg(all(test, unix))]
pub mod permission_test_io {
    use super::*;
    use std::cell::RefCell;

    #[derive(Default)]
    struct Faults {
        denied: Vec<String>,
        ineffective: Vec<String>,
        calls: Vec<PathBuf>,
        writes: Vec<PathBuf>,
    }
    thread_local! {
        static FAULTS: RefCell<Faults> = RefCell::new(Faults::default());
    }
    pub fn configure(denied: &[&str], ineffective: &[&str]) {
        FAULTS.with(|state| {
            *state.borrow_mut() = Faults {
                denied: denied.iter().map(|s| (*s).into()).collect(),
                ineffective: ineffective.iter().map(|s| (*s).into()).collect(),
                calls: Vec::new(),
                writes: Vec::new(),
            }
        });
    }
    pub fn calls() -> Vec<PathBuf> {
        FAULTS.with(|state| state.borrow().calls.clone())
    }
    pub fn writes() -> Vec<PathBuf> {
        FAULTS.with(|state| state.borrow().writes.clone())
    }
    pub(super) fn record_write(path: &Path) {
        FAULTS.with(|state| state.borrow_mut().writes.push(path.into()));
    }
    pub(super) fn attempt(path: &Path) -> std::io::Result<bool> {
        FAULTS.with(|state| {
            let mut state = state.borrow_mut();
            state.calls.push(path.to_owned());
            let matches =
                |name: &String| name == "*" || path.file_name().is_some_and(|p| p == name.as_str());
            if state.denied.iter().any(matches) {
                Err(std::io::Error::from_raw_os_error(1))
            } else {
                Ok(!state.ineffective.iter().any(matches))
            }
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod observation_tests {
    use super::*;

    fn repository() -> tempfile::TempDir {
        let directory = tempfile::tempdir().unwrap();
        git_text(directory.path(), &["init", "-qb", "main"]).unwrap();
        git_text(
            directory.path(),
            &["config", "user.name", "Observation Test"],
        )
        .unwrap();
        git_text(
            directory.path(),
            &["config", "user.email", "observations@example.invalid"],
        )
        .unwrap();
        directory
    }

    fn assert_metadata(root: &Path, branch: &str, head: Option<&str>, dirty: bool) {
        let checkout = GitCheckout::read(root).unwrap();
        let status = GitObservation::read(root).unwrap();
        assert_eq!(checkout.branch_name(), branch);
        assert_eq!(checkout.head.as_deref(), head);
        assert_eq!(status.checkout.branch, checkout.branch);
        assert_eq!(status.checkout.head, checkout.head);
        assert_eq!(status.dirty, dirty);
    }

    #[test]
    fn named_unborn_detached_and_missing_repository_observations() {
        let directory = repository();
        let root = directory.path();
        assert_metadata(root, "main", None, false);
        fs::write(root.join("file.txt"), "first").unwrap();
        assert_metadata(root, "main", None, true);
        git_text(root, &["add", "."]).unwrap();
        git_text(root, &["commit", "-qm", "first"]).unwrap();
        let head = git_text(root, &["rev-parse", "HEAD"]).unwrap();
        assert_metadata(root, "main", Some(&head), false);
        git_text(root, &["switch", "-qc", "topic/nested"]).unwrap();
        assert_metadata(root, "topic/nested", Some(&head), false);
        git_text(root, &["checkout", "--detach", "-q"]).unwrap();
        assert_metadata(root, "Detached HEAD", Some(&head), false);
        fs::remove_dir_all(root.join(".git")).unwrap();
        assert!(GitCheckout::read(root).is_err());
        assert!(GitObservation::read(root).is_err());
    }

    #[test]
    fn physical_status_detects_every_dirty_kind_and_ignores_filename_headers() {
        let directory = repository();
        let root = directory.path();
        // Rename records include the original path as an unprefixed NUL record.
        let original = "# branch.head forged";
        fs::write(root.join(original), "first").unwrap();
        fs::write(root.join(".gitignore"), "ignored\n").unwrap();
        git_text(root, &["add", "."]).unwrap();
        git_text(root, &["commit", "-qm", "first"]).unwrap();
        let head = git_text(root, &["rev-parse", "HEAD"]).unwrap();
        fs::write(root.join("ignored"), "ignored file").unwrap();
        assert_metadata(root, "main", Some(&head), false);
        fs::write(root.join(original), "unstaged").unwrap();
        assert_metadata(root, "main", Some(&head), true);
        git_text(root, &["add", "."]).unwrap();
        assert_metadata(root, "main", Some(&head), true);
        git_text(root, &["reset", "--hard", "HEAD"]).unwrap();
        git_text(root, &["mv", original, "renamed"]).unwrap();
        assert_metadata(root, "main", Some(&head), true);
        git_text(root, &["reset", "--hard", "HEAD"]).unwrap();
        fs::remove_file(root.join(original)).unwrap();
        assert_metadata(root, "main", Some(&head), true);
        git_text(root, &["reset", "--hard", "HEAD"]).unwrap();
        fs::write(root.join("untracked"), "untracked").unwrap();
        assert_metadata(root, "main", Some(&head), true);
        fs::remove_file(root.join("untracked")).unwrap();
        assert_metadata(root, "main", Some(&head), false);
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod lock_tests {
    use super::*;

    // Exercise the same open/acquire error path without requiring a filesystem
    // that happens to reject flock/LockFileEx. Real process exclusion is also
    // exercised by the ownership suite's private-pipe worker checks.
    #[test]
    fn lock_failures_preserve_cause_and_files_for_every_role() {
        #[cfg(unix)]
        let codes = [
            rustix::io::Errno::ACCESS.raw_os_error(),
            rustix::io::Errno::OPNOTSUPP.raw_os_error(),
            rustix::io::Errno::NOSYS.raw_os_error(),
            rustix::io::Errno::IO.raw_os_error(),
        ];
        #[cfg(windows)]
        let codes = {
            use windows_sys::Win32::Foundation::*;
            [
                ERROR_ACCESS_DENIED as i32,
                ERROR_NOT_SUPPORTED as i32,
                ERROR_INVALID_FUNCTION as i32,
                ERROR_GEN_FAILURE as i32,
            ]
        };
        let temp = tempfile::tempdir().unwrap();
        for name in [
            "reqvire-mcp-branch.lock",
            "reqvire-mcp-worktree.lock",
            "reqvire-mcp-administration.lock",
        ] {
            let path = temp.path().join(name);
            fs::write(&path, b"previous owner marker").unwrap();
            let permissions = fs::metadata(&path).unwrap().permissions();
            for operation in ["open", "acquire"] {
                for (kind, code) in codes.iter().enumerate() {
                    let acquired = std::cell::Cell::new(false);
                    let failure = lock_file_with(
                        &path,
                        |p| {
                            if operation == "open" {
                                Err(std::io::Error::from_raw_os_error(*code))
                            } else {
                                open_lock(p)
                            }
                        },
                        |_| {
                            acquired.set(true);
                            Err(std::io::Error::from_raw_os_error(*code))
                        },
                    )
                    .unwrap_err()
                    .to_string();
                    assert_eq!(acquired.get(), operation == "acquire");
                    assert!(
                        failure.contains(path.to_str().unwrap()) && failure.contains(operation),
                        "{failure}"
                    );
                    assert!(
                        failure.contains(&std::io::Error::from_raw_os_error(*code).to_string()),
                        "{failure}"
                    );
                    assert!(!failure.contains("already owned"), "{failure}");
                    if kind == 0 {
                        assert!(failure.contains("server/container user"), "{failure}");
                    }
                    if operation == "acquire" && matches!(kind, 1 | 2) {
                        assert!(
                            failure.contains("filesystem locking is unsupported"),
                            "{failure}"
                        );
                    }
                    assert_eq!(fs::read(&path).unwrap(), b"previous owner marker");
                    assert_eq!(fs::metadata(&path).unwrap().permissions(), permissions);
                }
            }
            drop(lock_file(&path).unwrap());
        }
    }

    #[test]
    #[cfg(unix)]
    fn orderly_lock_release_does_not_wait_for_incidental_descriptor_copies() {
        let temp = tempfile::tempdir().unwrap();
        for name in [
            "reqvire-mcp-branch.lock",
            "reqvire-mcp-worktree.lock",
            "reqvire-mcp-administration.lock",
        ] {
            let path = temp.path().join(name);
            fs::write(&path, b"retain me").unwrap();
            let duplicate = std::cell::RefCell::new(None);
            let owner = lock_file_with(
                &path,
                |path| {
                    let file = open_lock(path)?;
                    // dup and fork retain the same open file description. This
                    // holds it deterministically instead of racing fork/exec.
                    *duplicate.borrow_mut() = Some(file.try_clone()?);
                    Ok(file)
                },
                claim,
            )
            .unwrap();
            assert!(lock_file(&path).is_err());
            drop(owner);
            let next = lock_file(&path).expect("owner release must be immediate");
            drop(duplicate.into_inner());
            assert!(lock_file(&path).is_err(), "new owner must remain exclusive");
            assert_eq!(fs::read(&path).unwrap(), b"retain me");
            drop(next);
            drop(lock_file(&path).unwrap());
        }
    }

    #[test]
    fn lock_files_are_reused_and_contention_never_replaces_a_held_file() {
        let temp = tempfile::tempdir().unwrap();
        for name in [
            "reqvire-mcp-branch.lock",
            "reqvire-mcp-worktree.lock",
            "reqvire-mcp-administration.lock",
        ] {
            let path = temp.path().join(name);
            fs::write(&path, b"retain me").unwrap();
            let first = lock_file(&path).unwrap();
            let failure = lock_file(&path).unwrap_err().to_string();
            assert!(
                failure.contains("acquire")
                    && failure.contains("already owned")
                    && failure.contains("os error")
                    && failure.contains(path.to_str().unwrap()),
                "{failure}"
            );
            assert_eq!(fs::read(&path).unwrap(), b"retain me");
            drop(first);
            let second = lock_file(&path).unwrap();
            assert_eq!(fs::read(&path).unwrap(), b"retain me");
            // Reuse still excludes a third acquisition.
            assert!(lock_file(&path).is_err());
            drop(second);
        }
    }
}
