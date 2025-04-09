use std::process::Command;
use anyhow::Result;
use crate::error::ReqFlowError;
use std::path::PathBuf;

/// Retrieves the repository base URL (HTTPS format) from Git remote configuration.
pub fn get_repository_base_url() -> Result<String, ReqFlowError> {
    // Fetch the repository URL from git configuration
    let output = Command::new("git")
        .args(&["config", "--get", "remote.origin.url"])
        .output()?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(ReqFlowError::GitCommandError(format!("Failed to get repository URL: {}", err)));
    }

    let url = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if url.is_empty() {
        return Err(ReqFlowError::GitCommandError("Repository URL is empty or not set".to_string()));
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
        return Err(ReqFlowError::GitCommandError(format!(
            "Unsupported remote URL format: {}", url
        )));
    };

    Ok(base_url)
}


/// Retrieves the current commit hash from the repository.
pub fn get_commit_hash() -> Result<String,ReqFlowError> {
    // Run the git command to get the current commit hash
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(ReqFlowError::GitCommandError(format!("Failed to get current commit hash: {}", err)));
    }

    // Convert the output to a string and trim any newline or whitespace
    let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if hash.is_empty() {
        return Err(ReqFlowError::GitCommandError("Commit hash is empty".to_string()));
    }

    Ok(hash)
}

/// Retrieves the content of a file at a given commit (e.g. "HEAD~1").
pub fn get_file_at_commit(file_path: &str,folder:&PathBuf, commit: &str) -> Result<String, ReqFlowError> {

    let folder_path = PathBuf::from(folder);
    // Find the Git root for the given folder
    let git_root = find_git_repo_root(&folder_path)?;

    match file_path.strip_prefix(&git_root) {
        Some(relative_path) => {           
              let output = Command::new("git")
                  .args(&["show", &format!("{}:{}", commit, relative_path.trim_start_matches('/'))])
                  .current_dir(&git_root)
                  .output()?;
              if !output.status.success() {
                  let err = String::from_utf8_lossy(&output.stderr);
                  return Err(ReqFlowError::GitCommandError(format!("git show failed for {}: {}", file_path, err)));
              }
              Ok(String::from_utf8_lossy(&output.stdout).into())
            
        },
        None => Err(ReqFlowError::PathError(format!("Problem extracting git relative path: {} for root {}",&file_path, &git_root)))
    }
}

/// Finds the Git repository root for a given absolute folder path.
pub fn find_git_repo_root(absolute_folder_path: &PathBuf) -> Result<String, ReqFlowError> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .current_dir(absolute_folder_path)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(root)
        }
        Ok(output) => {
            let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(ReqFlowError::GitCommandError(format!(
                "git rev-parse failed to get repository root for {}: {}",
                absolute_folder_path.to_string_lossy(),
                err
            )))
        }
        Err(err) => Err(ReqFlowError::GitCommandError(format!(
            "Failed to execute git rev-parse for {}: {}",
            absolute_folder_path.to_string_lossy(),
            err
        ))),
    }
}

/// Retrieves the repository root folder
/*
pub fn get_repository_root(file_path: &str,folder:&str, commit: &str) -> Result<String, ReqFlowError> {
    println!("{}", &format!("{}:{}", commit, file_path));
    let output = Command::new("git")
        .args(&["show", &format!("{}:{}", commit, file_path)])
        .current_dir(folder)        
        .output()?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(ReqFlowError::GitCommandError(format!("git show failed for {}: {}", file_path, err)));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into())
}

*/
pub fn repository_root() -> Result<PathBuf, ReqFlowError> {
    let output = Command::new("git")
        .args(&["rev-parse", "--show-toplevel"])
        .output()?;

    if !output.status.success() {
        return Err(ReqFlowError::GitCommandError(
            "git failed to find repository root".to_string(),
        ));
    }

    let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(PathBuf::from(path_str))
}

/// Returns a list of files that have changed (according to `git diff --name-only`).
#[allow(dead_code)]
fn get_changed_files_from_git() -> Result<Vec<String>, ReqFlowError> {
    let output = Command::new("git")
        .args(&["diff", "--name-only"])
        .output()?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(ReqFlowError::GitCommandError(format!("git diff failed: {}", err)));        
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
pub fn ls_tree_commit_in_folder(commit: &str, folder: &PathBuf) -> Result<Vec<String>,ReqFlowError> {
    let output = Command::new("git")
        .args(&["ls-tree", "--name-only", "-r", commit])
        .current_dir(folder)
        .output()?;

    if !output.status.success() {
        // Convert stderr to string for error context
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        return Err(ReqFlowError::GitCommandError(format!("git ls-tree failed (commit = {}, folder = {:?}): {}", commit, folder, stderr_str)));        

    }

    // Convert stdout lines into a Vec<String>
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let files = stdout_str
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    Ok(files)
}


