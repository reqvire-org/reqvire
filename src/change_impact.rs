use std::error::Error;
use std::fs;
use std::process::Command;
use std::env;
use serde::Serialize;
use serde_json;
use globset::GlobSet;
use crate::utils;
use std::path::PathBuf;

/// Represents a change for a file: the file path, its content before the change,
/// and its content after the change.
#[derive(Debug, Serialize)]
pub struct FileChange {
    pub file_path: String,
    pub content: String,
}

            
pub fn generate_change_report(commit: &str,excluded_filename_patterns: &GlobSet) -> Result<ChangeReport, Box<dyn Error>> {
    let changed_files = get_changed_files_from_git()?;
    let mut file_changes = Vec::new();
    for file in changed_files {
        let before_content = get_file_at_commit(&file, commit)?;
        file_changes.push(FileChange {
            file_path: file,
            content: before_content
        });
    }
    Ok(ChangeReport { file_changes: file_changes.into_iter().filter(|e| utils::is_requirements_file_by_path(&PathBuf::from(e.file_path.clone()), excluded_filename_patterns)).collect() })
}



/// Retrieves the content of a file at a given commit (e.g. "HEAD~1").
pub fn get_file_at_commit(file_path: &str, commit: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .args(&["show", &format!("{}:{}", commit, file_path)])
        .output()?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("git show failed for {}: {}", file_path, err),
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into())
}

/// Returns a list of files that have changed (according to `git diff --name-only`).
fn get_changed_files_from_git() -> Result<Vec<String>, Box<dyn Error>> {
    let output = Command::new("git")
        .args(&["diff", "--name-only"])
        .output()?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("git diff failed: {}", err),
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

