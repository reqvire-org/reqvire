use crate::error::ReqvireError;
#[cfg(test)]
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{LazyLock, Mutex};

// Workspace owns the shared plumbing; this adapter retains core errors.
pub use reqvire_workspace::git::{GitReader, TreeFile};
pub fn run_local(
    directory: &Path,
    args: &[&str],
    input: Option<&[u8]>,
) -> Result<Vec<u8>, ReqvireError> {
    reqvire_workspace::git::run_local(directory, args, input).map_err(Into::into)
}
pub fn local_bytes(directory: &Path, args: &[&str]) -> Result<Vec<u8>, ReqvireError> {
    run_local(directory, args, None)
}
pub fn local_text(directory: &Path, args: &[&str]) -> Result<String, ReqvireError> {
    reqvire_workspace::git::local_text(directory, args).map_err(Into::into)
}

static REPO_URL: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));
static COMMIT_HASH: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));
static GIT_ROOT_DIR: LazyLock<Mutex<Option<PathBuf>>> = LazyLock::new(|| Mutex::new(None));
static BRANCH_NAME: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));

// Disable caching in tests to prevent interference between parallel tests
static DISABLE_CACHE_FOR_TESTS: AtomicBool = AtomicBool::new(false);

pub fn clear_git_cache() {
    REPO_URL.lock().expect("mutex not poisoned").take();
    COMMIT_HASH.lock().expect("mutex not poisoned").take();
    GIT_ROOT_DIR.lock().expect("mutex not poisoned").take();
    BRANCH_NAME.lock().expect("mutex not poisoned").take();
    reqvire_workspace::git::clear_root_cache();
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
    if let Some((head, _)) = crate::mutation_io::git_identity() {
        return Ok(head);
    }
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
    if let Some((_, branch)) = crate::mutation_io::git_identity() {
        return Ok(branch);
    }
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
pub fn find_git_repo_root(absolute_folder_path: &Path) -> Result<String, ReqvireError> {
    reqvire_workspace::git::find_root(absolute_folder_path).map_err(Into::into)
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

#[cfg(test)]
mod local_reader_tests {
    use super::*;

    #[test]
    fn shared_reader_round_trips_binary_objects_and_modes() {
        let directory = tempfile::tempdir().expect("Git reader fixture");
        let root = directory.path();
        local_bytes(root, &["init", "--quiet"]).expect("Git reader fixture");
        local_bytes(root, &["config", "user.name", "Git reader test"]).expect("Git reader fixture");
        local_bytes(root, &["config", "user.email", "reader@example.invalid"])
            .expect("Git reader fixture");
        let payload: Vec<u8> = (0..262_144).map(|n| (n % 256) as u8).collect();
        let oid = String::from_utf8(
            run_local(root, &["hash-object", "-w", "--stdin"], Some(&payload))
                .expect("Git reader fixture"),
        )
        .expect("Git reader fixture");
        let oid = oid.trim();
        for (name, mode) in [("payload.bin", "100644"), ("executable.bin", "100755")] {
            local_bytes(
                root,
                &[
                    "update-index",
                    "--add",
                    "--cacheinfo",
                    &format!("{mode},{oid},{name}"),
                ],
            )
            .expect("Git reader fixture");
        }
        let tree = local_text(root, &["write-tree"]).expect("Git reader fixture");
        let commit = local_text(root, &["commit-tree", &tree, "-m", "binary fixture"])
            .expect("Git reader fixture");
        let paths = BTreeSet::from([root.join("payload.bin"), root.join("executable.bin")]);
        let mut calls = Vec::new();
        let files = GitReader::new(root, |args: &[&str], input| {
            calls.push(args.first().copied().unwrap_or_default().to_owned());
            run_local(root, args, input)
        })
        .regular_files_at_commit(&commit, &paths)
        .expect("Git reader fixture");
        assert_eq!(calls, ["ls-tree", "cat-file"]);
        assert_eq!(*files[&root.join("payload.bin")].bytes, payload);
        assert_eq!(
            files[&root.join("payload.bin")].bytes,
            files[&root.join("executable.bin")].bytes
        );
        assert!(!files[&root.join("payload.bin")].executable);
        assert!(files[&root.join("executable.bin")].executable);
    }

    #[test]
    fn local_runner_disallows_transports_even_for_file_remotes() {
        let directory = tempfile::tempdir().expect("Git reader fixture");
        local_bytes(directory.path(), &["init", "--quiet"]).expect("Git reader fixture");
        let remote = format!("file://{}", directory.path().display());
        let error = local_bytes(directory.path(), &["ls-remote", &remote])
            .expect_err("transports disabled");
        assert!(
            error.to_string().contains("transport 'file' not allowed"),
            "{error}"
        );
    }
}
