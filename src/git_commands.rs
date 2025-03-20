use std::process::Command;
use anyhow::Result;
use crate::error::ReqFlowError;
use std::path::PathBuf;

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
