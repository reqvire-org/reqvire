use std::fs;
use log::{debug, warn};
use std::path::PathBuf;
use std::env;
use globset::{Glob, GlobSet, GlobSetBuilder};

/// Finds the root of the git repository
fn find_git_root() -> Option<PathBuf> {
        let current_dir = env::current_dir().ok()?;
        let mut dir = current_dir.as_path();

        loop {
            if dir.join(".git").exists() {
                return Some(dir.to_path_buf());
            }

            dir = dir.parent()?;
        }
    }

/// Reads gitignore patterns from the repository root .gitignore file
fn read_gitignore_patterns() -> Vec<String> {
        let git_root = match find_git_root() {
            Some(root) => root,
            None => {
                debug!("No git repository found, skipping .gitignore");
                return vec![];
            }
        };

        let gitignore_path = git_root.join(".gitignore");

        if !gitignore_path.exists() {
            debug!("No .gitignore file found at repository root");
            return vec![];
        }

        // Read the gitignore file and extract patterns
        // Note: We're just parsing the file content directly rather than using
        // the gitignore builder since we need to convert patterns to globs
        match fs::read_to_string(&gitignore_path) {
            Ok(content) => {
                content
                    .lines()
                    .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
                    .map(|line| {
                        let pattern = line.trim();
                        // Convert gitignore patterns to glob patterns

                        // If pattern already starts with globstar, use as-is
                        if pattern.starts_with("**/") || pattern.starts_with("**") {
                            pattern.to_string()
                        } else if pattern.ends_with('/') {
                            // Directory pattern - match everything under directory anywhere in tree
                            let dir_name = pattern.trim_end_matches('/');
                            format!("**/{}/**", dir_name)
                        } else if pattern.contains('/') {
                            // Path pattern - use as-is with globstar prefix if needed
                            if pattern.starts_with('/') {
                                format!("**{}", pattern)
                            } else {
                                format!("**/{}", pattern)
                            }
                        } else {
                            // Filename pattern - match anywhere in tree
                            format!("**/{}", pattern)
                        }
                    })
                    .collect()
            }
            Err(e) => {
                warn!("Failed to read .gitignore content: {}", e);
                vec![]
            }
        }
    }

/// Returns reserved filenames that are always excluded from structured markdown processing
fn reserved_filenames() -> Vec<String> {
        vec![
            "**/README.md".to_string(),
            "**/CHANGELOG.md".to_string(),
            "**/CHANGES.md".to_string(),
            "**/CONTRIBUTING.md".to_string(),
            "**/LICENSE.md".to_string(),
            "**/CODE_OF_CONDUCT.md".to_string(),
            "**/SECURITY.md".to_string(),
            "**/AUTHORS.md".to_string(),
            "**/ROADMAP.md".to_string(),
            "**/CLAUDE.md".to_string(),
            "**/AGENT.md".to_string(),
            "**/AI.md".to_string(),
            "**/PROMPT.md".to_string(),
            "**/INSTRUCTIONS.md".to_string(),
            "**/CONTEXT.md".to_string(),
            "**/CURSOR.md".to_string(),
            "**/COPILOT.md".to_string(),
        ]
    }

/// Reads reqvireignore patterns from the repository root .reqvireignore file
fn read_reqvireignore_patterns() -> Vec<String> {
        let git_root = match find_git_root() {
            Some(root) => root,
            None => {
                debug!("No git repository found, skipping .reqvireignore");
                return vec![];
            }
        };

        let reqvireignore_path = git_root.join(".reqvireignore");

        if !reqvireignore_path.exists() {
            debug!("No .reqvireignore file found at repository root");
            return vec![];
        }

        // Read the reqvireignore file and extract patterns
        // Same format as gitignore patterns
        match fs::read_to_string(&reqvireignore_path) {
            Ok(content) => {
                content
                    .lines()
                    .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
                    .map(|line| {
                        let pattern = line.trim();
                        // Convert reqvireignore patterns to glob patterns

                        // If pattern already starts with globstar, use as-is
                        if pattern.starts_with("**/") || pattern.starts_with("**") {
                            pattern.to_string()
                        } else if pattern.ends_with('/') {
                            // Directory pattern - match everything under directory anywhere in tree
                            let dir_name = pattern.trim_end_matches('/');
                            format!("**/{}/**", dir_name)
                        } else if pattern.contains('/') {
                            // Path pattern - use as-is with globstar prefix if needed
                            if pattern.starts_with('/') {
                                format!("**{}", pattern)
                            } else {
                                format!("**/{}", pattern)
                            }
                        } else {
                            // Filename pattern - match anywhere in tree
                            format!("**/{}", pattern)
                        }
                    })
                    .collect()
            }
            Err(e) => {
                warn!("Failed to read .reqvireignore content: {}", e);
                vec![]
            }
        }
    }

/// Builds a GlobSet from reserved filenames, gitignore, and reqvireignore
pub fn get_excluded_filename_patterns_glob_set() -> GlobSet {
        let mut builder = GlobSetBuilder::new();

        // Add reserved filenames (always excluded)
        for pattern in reserved_filenames() {
            if let Ok(glob) = Glob::new(&pattern) {
                builder.add(glob);
                debug!("Added reserved filename pattern: {}", pattern);
            } else {
                warn!("Invalid reserved filename pattern: {}", pattern);
            }
        }

        // Add patterns from root .gitignore
        for pattern in read_gitignore_patterns() {
            if let Ok(glob) = Glob::new(&pattern) {
                builder.add(glob);
                debug!("Added gitignore pattern: {}", pattern);
            } else {
                warn!("Invalid gitignore pattern: {}", pattern);
            }
        }

        // Add patterns from root .reqvireignore
        for pattern in read_reqvireignore_patterns() {
            if let Ok(glob) = Glob::new(&pattern) {
                builder.add(glob);
                debug!("Added reqvireignore pattern: {}", pattern);
            } else {
                warn!("Invalid reqvireignore pattern: {}", pattern);
            }
        }

        builder.build().expect("Failed to build glob set")
}

#[cfg(test)]
mod config_tests {
    use crate::config::get_excluded_filename_patterns_glob_set;

    #[test]
    fn test_build_excluded_patterns() {
        // Build globset to verify it works correctly
        // (The actual patterns will be tested in e2e tests)
        let _globset = get_excluded_filename_patterns_glob_set();
        // Note: Whether files are excluded depends on reserved filenames, .gitignore, and .reqvireignore
        // This test just verifies the globset builds successfully
    }
}
