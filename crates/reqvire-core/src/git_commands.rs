use crate::error::ReqvireError;
use rustc_hash::FxHashMap;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{LazyLock, Mutex};

/// Read-only Git plumbing with the caller's execution policy. MCP supplies its
/// bounded, binary-safe runner; repository parsing stays in this shared layer.
pub struct GitReader<'a, F> {
    root: &'a Path,
    run: F,
}

pub struct TreeFile {
    pub bytes: std::sync::Arc<Vec<u8>>,
    pub executable: bool,
    pub symlink: bool,
}

impl<'a, F> GitReader<'a, F>
where
    F: FnMut(&[&str], Option<&[u8]>) -> Result<Vec<u8>, ReqvireError>,
{
    pub const fn new(root: &'a Path, run: F) -> Self {
        Self { root, run }
    }

    pub fn operation_in_progress(&mut self) -> Result<bool, ReqvireError> {
        let bytes = (self.run)(&["rev-parse", "--absolute-git-dir"], None)?;
        let directory = PathBuf::from(git_utf8(&bytes)?.trim_end_matches(['\n', '\r']));
        Ok([
            "MERGE_HEAD",
            "CHERRY_PICK_HEAD",
            "REVERT_HEAD",
            "rebase-merge",
            "rebase-apply",
            "sequencer",
        ]
        .iter()
        .any(|marker| directory.join(marker).exists()))
    }

    pub fn changed_paths(
        &mut self,
        old: &str,
        new: &str,
    ) -> Result<BTreeSet<PathBuf>, ReqvireError> {
        let bytes = (self.run)(
            &[
                "diff",
                "--no-ext-diff",
                "--no-renames",
                "--name-only",
                "-z",
                old,
                new,
                "--",
            ],
            None,
        )?;
        bytes
            .split(|b| *b == 0)
            .filter(|p| !p.is_empty())
            .map(|p| git_utf8(p).map(|p| self.root.join(p)))
            .collect()
    }

    /// One tree read and one deduplicated blob batch, regardless of file count.
    /// Missing paths represent deletions. Nonregular entries require a caller
    /// with explicit symlink/submodule support rather than a byte-file snapshot.
    pub fn regular_files_at_commit(
        &mut self,
        commit: &str,
        paths: &BTreeSet<PathBuf>,
    ) -> Result<BTreeMap<PathBuf, TreeFile>, ReqvireError> {
        if paths.is_empty() {
            return Ok(BTreeMap::new());
        }
        self.selected_files_at_commit(commit, Some(paths), false)
    }

    /// Read a complete tree for isolated historical reconstruction. Symlink
    /// bytes are declarations; the caller must resolve them inside this tree.
    pub(crate) fn files_at_commit(
        &mut self,
        commit: &str,
    ) -> Result<BTreeMap<PathBuf, TreeFile>, ReqvireError> {
        self.selected_files_at_commit(commit, None, true)
    }

    fn selected_files_at_commit(
        &mut self,
        commit: &str,
        paths: Option<&BTreeSet<PathBuf>>,
        with_symlinks: bool,
    ) -> Result<BTreeMap<PathBuf, TreeFile>, ReqvireError> {
        let tree = (self.run)(&["ls-tree", "-r", "-z", "--full-tree", commit], None)?;
        let mut entries = BTreeMap::new();
        for entry in tree.split(|b| *b == 0).filter(|e| !e.is_empty()) {
            let (metadata, path) = git_utf8(entry)?
                .split_once('\t')
                .ok_or_else(|| git_error("Invalid Git tree entry"))?;
            let mut metadata = metadata.split_whitespace();
            let mode = metadata
                .next()
                .ok_or_else(|| git_error("Missing tree mode"))?;
            let kind = metadata
                .next()
                .ok_or_else(|| git_error("Missing tree object kind"))?;
            let oid = metadata
                .next()
                .ok_or_else(|| git_error("Missing tree object ID"))?;
            let path = self.root.join(path);
            if paths.is_none_or(|paths| paths.contains(&path)) {
                if paths.is_none() && kind == "commit" && mode == "160000" {
                    continue; // Undeclared submodules are outside the composition.
                }
                if kind != "blob"
                    || !(matches!(mode, "100644" | "100755") || (with_symlinks && mode == "120000"))
                {
                    return Err(git_error(format!(
                        "Changed nonregular artifact {} requires full admission",
                        path.display()
                    )));
                }
                entries.insert(path, (mode == "100755", mode == "120000", oid.to_owned()));
            }
        }
        let oids: BTreeSet<_> = entries.values().map(|(_, _, oid)| oid.clone()).collect();
        let mut blobs = BTreeMap::new();
        if !oids.is_empty() {
            let input = oids.iter().map(|s| format!("{s}\n")).collect::<String>();
            let bytes = (self.run)(&["cat-file", "--batch"], Some(input.as_bytes()))?;
            let mut cursor = bytes.as_slice();
            for oid in oids {
                let end = cursor
                    .iter()
                    .position(|b| *b == b'\n')
                    .ok_or_else(|| git_error("Missing blob header"))?;
                let mut fields = git_utf8(&cursor[..end])?.split_whitespace();
                if fields.next() != Some(oid.as_str()) || fields.next() != Some("blob") {
                    return Err(git_error("Invalid blob batch reply"));
                }
                let length: usize = fields
                    .next()
                    .ok_or_else(|| git_error("Missing blob length"))?
                    .parse()
                    .map_err(|_| git_error("Invalid blob length"))?;
                cursor = &cursor[end + 1..];
                if cursor.len() <= length || cursor[length] != b'\n' {
                    return Err(git_error("Incomplete blob reply"));
                }
                blobs.insert(oid, std::sync::Arc::new(cursor[..length].to_vec()));
                cursor = &cursor[length + 1..];
            }
        }
        Ok(entries
            .into_iter()
            .map(|(path, (executable, symlink, oid))| {
                (
                    path,
                    TreeFile {
                        bytes: std::sync::Arc::clone(&blobs[&oid]),
                        executable,
                        symlink,
                    },
                )
            })
            .collect())
    }
}

fn git_error(message: impl Into<String>) -> ReqvireError {
    ReqvireError::GitCommandError(message.into())
}
fn git_utf8(bytes: &[u8]) -> Result<&str, ReqvireError> {
    std::str::from_utf8(bytes).map_err(|e| git_error(e.to_string()))
}

static REPO_URL: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));
static COMMIT_HASH: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));
static GIT_ROOT_DIR: LazyLock<Mutex<Option<PathBuf>>> = LazyLock::new(|| Mutex::new(None));
static BRANCH_NAME: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));
static GIT_ROOT_CACHE: LazyLock<Mutex<FxHashMap<PathBuf, String>>> =
    LazyLock::new(|| Mutex::new(FxHashMap::default()));

// Disable caching in tests to prevent interference between parallel tests
static DISABLE_CACHE_FOR_TESTS: AtomicBool = AtomicBool::new(false);

pub fn clear_git_cache() {
    REPO_URL.lock().expect("mutex not poisoned").take();
    COMMIT_HASH.lock().expect("mutex not poisoned").take();
    GIT_ROOT_DIR.lock().expect("mutex not poisoned").take();
    BRANCH_NAME.lock().expect("mutex not poisoned").take();
    GIT_ROOT_CACHE.lock().expect("mutex not poisoned").clear();
}

#[cfg(test)]
pub fn disable_git_cache_for_tests() {
    DISABLE_CACHE_FOR_TESTS.store(true, Ordering::SeqCst);
    clear_git_cache();
}

fn is_cache_disabled() -> bool {
    DISABLE_CACHE_FOR_TESTS.load(Ordering::SeqCst)
}

/// Retrieves the repository base URL (HTTPS format) from Git remote configuration.
pub fn get_repository_base_url() -> Result<String, ReqvireError> {
    let mut cached = REPO_URL.lock().expect("mutex not poisoned");
    if let Some(ref url) = *cached {
        return Ok(url.clone());
    }

    // Fetch the repository URL from git configuration
    let output = Command::new("git")
        .args(["config", "--get", "remote.origin.url"])
        .output()?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(ReqvireError::GitCommandError(format!(
            "Failed to get repository URL: {}",
            err
        )));
    }

    let url = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if url.is_empty() {
        return Err(ReqvireError::GitCommandError(
            "Repository URL is empty or not set".to_string(),
        ));
    }

    let base_url = if url.starts_with("git@") {
        // Convert SSH URL to HTTPS (git@github.com:owner/repo.git -> https://github.com/owner/repo)
        let ssh_to_https = url
            .trim_start_matches("git@")
            .replace(':', "/")
            .trim_end_matches(".git")
            .to_string();
        format!("https://{}", ssh_to_https)
    } else if url.starts_with("https://") {
        // HTTPS URLs (https://github.com/owner/repo.git -> https://github.com/owner/repo)
        url.trim_end_matches(".git").to_string()
    } else {
        return Err(ReqvireError::GitCommandError(format!(
            "Unsupported remote URL format: {}",
            url
        )));
    };

    *cached = Some(base_url.clone());
    drop(cached);

    Ok(base_url)
}

/// Retrieves the current commit hash from the repository.
pub fn get_commit_hash() -> Result<String, ReqvireError> {
    let mut cached = COMMIT_HASH.lock().expect("mutex not poisoned");
    if let Some(ref hash) = *cached {
        return Ok(hash.clone());
    }

    // Run the git command to get the current commit hash
    let output = Command::new("git").args(["rev-parse", "HEAD"]).output()?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(ReqvireError::GitCommandError(format!(
            "Failed to get current commit hash: {}",
            err
        )));
    }

    // Convert the output to a string and trim any newline or whitespace
    let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if hash.is_empty() {
        return Err(ReqvireError::GitCommandError(
            "Commit hash is empty".to_string(),
        ));
    }

    *cached = Some(hash.clone());
    drop(cached);

    Ok(hash)
}

/// Retrieves the current Git branch name.
pub fn get_branch_name() -> Result<String, ReqvireError> {
    let mut cached = BRANCH_NAME.lock().expect("mutex not poisoned");
    if let Some(ref branch) = *cached {
        return Ok(branch.clone());
    }

    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(ReqvireError::GitCommandError(format!(
            "Failed to get current branch: {}",
            err
        )));
    }

    let mut branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if branch.is_empty() {
        let commit = get_commit_hash()?;
        branch = format!(
            "removed contract binding@{}",
            commit.chars().take(7).collect::<String>()
        );
    }

    *cached = Some(branch.clone());
    drop(cached);

    Ok(branch)
}

/// Retrieves the content of a file at a given commit (e.g. "HEAD~1").
pub fn get_file_at_commit(
    file_path: &str,
    folder: &PathBuf,
    commit: &str,
) -> Result<String, ReqvireError> {
    let folder_path = PathBuf::from(folder);
    // Find the Git root for the given folder
    let git_root = find_git_repo_root(&folder_path)?;

    match file_path.strip_prefix(&git_root) {
        Some(relative_path) => {
            let output = Command::new("git")
                .args([
                    "show",
                    &format!("{}:{}", commit, relative_path.trim_start_matches('/')),
                ])
                .current_dir(&git_root)
                .output()?;
            if !output.status.success() {
                let err = String::from_utf8_lossy(&output.stderr);
                return Err(ReqvireError::GitCommandError(format!(
                    "git show failed for {}: {}",
                    file_path, err
                )));
            }
            Ok(String::from_utf8_lossy(&output.stdout).into())
        }
        None => Err(ReqvireError::PathError(format!(
            "Problem extracting git relative path: {} for root {}",
            &file_path, &git_root
        ))),
    }
}

/// Finds the Git repository root for a given absolute folder path.
pub fn find_git_repo_root(absolute_folder_path: &PathBuf) -> Result<String, ReqvireError> {
    let mut cache = GIT_ROOT_CACHE.lock().expect("mutex not poisoned");
    if let Some(cached_root) = cache.get(absolute_folder_path) {
        return Ok(cached_root.clone());
    }

    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .current_dir(absolute_folder_path)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
            cache.insert(absolute_folder_path.clone(), root.clone());
            drop(cache);
            Ok(root)
        }
        Ok(output) => {
            let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(ReqvireError::GitCommandError(format!(
                "git rev-parse failed to get repository root for {}: {}",
                absolute_folder_path.to_string_lossy(),
                err
            )))
        }
        Err(err) => Err(ReqvireError::GitCommandError(format!(
            "Failed to execute git rev-parse for {}: {}",
            absolute_folder_path.to_string_lossy(),
            err
        ))),
    }
}

/// Returns the Git repository root directory
pub fn get_git_root_dir() -> Result<PathBuf, ReqvireError> {
    // Skip caching in tests to prevent parallel test interference
    if !is_cache_disabled() {
        let cached = GIT_ROOT_DIR.lock().expect("mutex not poisoned");
        if let Some(ref path) = *cached {
            return Ok(path.clone());
        }
    }

    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()?;

    if !output.status.success() {
        return Err(ReqvireError::GitCommandError(
            "git failed to find repository root".to_string(),
        ));
    }
    let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let path = PathBuf::from(path_str);

    // Only cache if caching is enabled
    if !is_cache_disabled() {
        let mut cached = GIT_ROOT_DIR.lock().expect("mutex not poisoned");
        *cached = Some(path.clone());
    }

    Ok(path)
}

/// Lists all files in a commit by running `git ls-tree --name-only -r <commit>`
/// from the git repository root. Returns a list of file paths.
pub fn ls_tree_commit(commit: &str) -> Result<Vec<String>, ReqvireError> {
    let git_root = get_git_root_dir()?;
    ls_tree_commit_from_dir(commit, &git_root, || {
        format!("git ls-tree failed (commit = {})", commit)
    })
}

fn ls_tree_commit_from_dir<F>(
    commit: &str,
    directory: &PathBuf,
    error_context: F,
) -> Result<Vec<String>, ReqvireError>
where
    F: FnOnce() -> String,
{
    let output = Command::new("git")
        .args(["ls-tree", "--name-only", "-r", commit])
        .current_dir(directory)
        .output()?;

    if !output.status.success() {
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        return Err(ReqvireError::GitCommandError(format!(
            "{}: {}",
            error_context(),
            stderr_str
        )));
    }

    // Convert stdout lines into a Vec<String>
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let files = stdout_str
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    Ok(files)
}

/// Returns a list of files that have changed (according to `git diff --name-only`).
#[allow(dead_code)]
fn get_changed_files_from_git() -> Result<Vec<String>, ReqvireError> {
    let output = Command::new("git").args(["diff", "--name-only"]).output()?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(ReqvireError::GitCommandError(format!(
            "git diff failed: {}",
            err
        )));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let changed_files: Vec<String> = stdout
        .lines()
        .map(|s| s.trim().to_string())
        // Only process Markdown files
        .filter(|s| s.ends_with(".md"))
        .collect();
    Ok(changed_files)
}

/// Lists files in `commit` by running `git ls-tree --name-only -r <commit>`
/// with `folder` as the current directory. Returns a list of file paths.
pub fn ls_tree_commit_in_folder(
    commit: &str,
    folder: &PathBuf,
) -> Result<Vec<String>, ReqvireError> {
    ls_tree_commit_from_dir(commit, folder, || {
        format!(
            "git ls-tree failed (commit = {}, folder = {:?})",
            commit, folder
        )
    })
}
