//! Git ownership and publication boundary for mutation-enabled MCP.
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
    process::{Command, Stdio},
    sync::Arc,
};

pub struct MutationSession {
    root: PathBuf,
    branch: String,
    head: String,
    _ownership: File,
    _worktree_ownership: File,
    files: SnapshotFiles,
    model: Arc<ModelManager>,
    exclusions: ExclusionSet,
    options: ModelBuildOptions,
    poisoned: bool,
    enable_commits: bool,
}
fn error(message: impl Into<String>) -> ReqvireError {
    ReqvireError::ProcessError(message.into())
}
fn git(
    root: &Path,
    args: &[&str],
    input: Option<&[u8]>,
    index: Option<&Path>,
) -> Result<Vec<u8>, ReqvireError> {
    let mut command = Command::new("git");
    command
        .current_dir(root)
        .args(args)
        .env("GIT_LITERAL_PATHSPECS", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(index) = index {
        command.env("GIT_INDEX_FILE", index);
    }
    let mut child = command.spawn()?;
    if let Some(input) = input {
        child
            .stdin
            .take()
            .ok_or_else(|| error("Git stdin unavailable"))?
            .write_all(input)?;
    }
    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err(error(format!(
            "MCP Git operation {} failed: {}",
            args[0],
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    Ok(output.stdout)
}
fn git_text(root: &Path, args: &[&str]) -> Result<String, ReqvireError> {
    Ok(String::from_utf8(git(root, args, None, None)?)
        .map_err(|e| error(e.to_string()))?
        .trim()
        .to_string())
}
#[cfg(unix)]
fn claim(file: &File) -> Result<(), ReqvireError> {
    rustix::fs::flock(file, rustix::fs::FlockOperation::NonBlockingLockExclusive)
        .map_err(|_| error("MCP branch/worktree already owned by another mutation-enabled server"))
}
#[cfg(windows)]
fn claim(file: &File) -> Result<(), ReqvireError> {
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
        Err(error(
            "MCP branch/worktree already owned by another mutation-enabled server",
        ))
    } else {
        Ok(())
    }
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
        let ownership = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(lock_path)?;
        claim(&ownership)?;
        let worktree_lock = root.join(git_text(
            &root,
            &["rev-parse", "--git-path", "reqvire-mcp-worktree.lock"],
        )?);
        let worktree_ownership = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(worktree_lock)?;
        claim(&worktree_ownership)?;
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
            files: BTreeMap::new(),
            executable: BTreeSet::new(),
        };
        for path in git(&root, &["ls-files", "-z"], None, None)?
            .split(|b| *b == 0)
            .filter(|p| !p.is_empty())
        {
            let path = root.join(std::str::from_utf8(path).map_err(|e| error(e.to_string()))?);
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if fs::metadata(&path).is_ok_and(|m| m.permissions().mode() & 0o111 != 0) {
                    files.executable.insert(path.clone());
                }
            }
            if path.is_file() {
                files.files.insert(path.clone(), Arc::new(fs::read(path)?));
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
        files = capture.prepared().0;
        drop(capture);
        require_clean(&root)?;
        let session = Self {
            root,
            branch,
            head,
            _ownership: ownership,
            _worktree_ownership: worktree_ownership,
            files,
            model: Arc::new(model),
            exclusions: exclusions.clone(),
            options,
            poisoned: false,
            enable_commits,
        };
        session.check_head()?;
        Ok(session)
    }
    pub fn model(&self) -> ModelManager {
        (*self.model).clone()
    }
    fn check_head(&self) -> Result<(), ReqvireError> {
        if self.poisoned {
            return Err(error(
                "MCP session requires recovery; further mutations are disabled",
            ));
        }
        if git_text(&self.root, &["symbolic-ref", "--quiet", "HEAD"])? != self.branch
            || git_text(&self.root, &["rev-parse", "HEAD"])? != self.head
        {
            return Err(error("MCP owned branch or HEAD changed externally; restart the server before further mutations"));
        }
        Ok(())
    }
    pub fn execute(
        &mut self,
        persists: bool,
        tool: &str,
        call: impl FnOnce() -> Result<Value, rmcp::ErrorData>,
    ) -> (Result<Value, rmcp::ErrorData>, bool) {
        match self.execute_inner(persists, tool, call) {
            Ok(result) => result,
            Err(error) => (Ok(crate::mcp::tool_error(tool, error)), false),
        }
    }
    fn execute_inner(
        &mut self,
        persists: bool,
        tool: &str,
        call: impl FnOnce() -> Result<Value, rmcp::ErrorData>,
    ) -> Result<(Result<Value, rmcp::ErrorData>, bool), ReqvireError> {
        self.check_head()?;
        let scope = self.files.enter(Some(Arc::clone(&self.model)));
        let result = call();
        let (candidate_files, changed) = scope.prepared();
        drop(scope);
        // Keep protocol errors and tool rejection envelopes intact.
        let mut result = match result {
            Ok(value) => value,
            Err(error) => return Ok((Err(error), false)),
        };
        if result.get("isError").and_then(Value::as_bool) == Some(true) {
            return Ok((Ok(result), false));
        }
        let changed: BTreeSet<_> = changed
            .into_iter()
            .filter(|p| self.files.files.get(p) != candidate_files.files.get(p))
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
        self.check_head()?;
        let commit = self.persist_candidate(&candidate_files, &changed, tool)?;
        if let Some(commit) = &commit {
            self.head = commit.clone();
        }
        self.files = candidate_files;
        self.model = Arc::new(candidate);
        reqvire::model_cache::invalidate();
        if let (Some(commit), Some(content)) = (
            commit,
            result
                .get_mut("structuredContent")
                .and_then(Value::as_object_mut),
        ) {
            content.insert("commit".into(), Value::String(commit));
            result["content"] = serde_json::json!([{ "type": "text", "text": serde_json::to_string_pretty(content)? }]);
        }
        Ok((Ok(result), true))
    }
    fn prepare_commit(
        &mut self,
        files: &SnapshotFiles,
        changed: &BTreeSet<PathBuf>,
        tool: &str,
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
            &[
                "commit-tree",
                tree.trim(),
                "-p",
                &self.head,
                "-m",
                &format!("reqvire: {tool}"),
            ],
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
        })
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
            Some(self.prepare_commit(files, changed, tool)?)
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
        let result = (|| {
            for path in changed {
                persist(path, files.files.get(path).map(|b| b.as_slice()))?;
                #[cfg(unix)]
                if files.files.contains_key(path) {
                    use std::os::unix::fs::PermissionsExt;
                    fs::set_permissions(
                        path,
                        fs::Permissions::from_mode(if files.executable.contains(path) {
                            0o755
                        } else {
                            0o644
                        }),
                    )?;
                }
            }
            self.check_head()?;
            if let Some(publication) = &mut publication {
                git(
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
                )?;
                advanced = true;
                fs::rename(&publication.index_lock.path, &publication.index_path)?;
                publication.index_lock.published = true;
            }
            Ok::<_, ReqvireError>(())
        })();
        if let Err(failure) = result {
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
                for (path, bytes) in &backups {
                    persist(path, bytes.as_ref().map(|(bytes, _)| bytes.as_slice()))?;
                    if let Some((_, permissions)) = bytes {
                        fs::set_permissions(path, permissions.clone())?;
                    }
                }
                Ok::<_, ReqvireError>(())
            })();
            if let Err(recovery) = recovery {
                self.poisoned = true;
                return Err(error(format!("MCP mutation failed: {failure}; recovery failed: {recovery}; further writes disabled")));
            }
            return Err(failure);
        }
        Ok(publication.map(|p| p.commit))
    }
}
struct PreparedCommit {
    commit: String,
    index_path: PathBuf,
    index_lock: IndexLock,
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
fn require_clean(root: &Path) -> Result<(), ReqvireError> {
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
