use globset::{Glob, GlobSet, GlobSetBuilder};
use log::{debug, warn};
use std::env;
use std::fs;
use std::path::PathBuf;

/// Finds the root of the git repository.
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

/// Reads gitignore patterns from the repository root .gitignore file.
fn read_gitignore_patterns() -> Vec<String> {
    read_root_ignore_patterns(".gitignore")
}

/// Reads reqvireignore patterns from the repository root .reqvireignore file.
fn read_reqvireignore_patterns() -> Vec<String> {
    read_root_ignore_patterns(".reqvireignore")
}

fn read_root_ignore_patterns(filename: &str) -> Vec<String> {
    let git_root = match find_git_root() {
        Some(root) => root,
        None => {
            debug!("No git repository found, skipping {}", filename);
            return vec![];
        }
    };

    let ignore_path = git_root.join(filename);

    if !ignore_path.exists() {
        debug!("No {} file found at repository root", filename);
        return vec![];
    }

    match fs::read_to_string(&ignore_path) {
        Ok(content) => content
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
            .map(gitignore_pattern_to_glob)
            .collect(),
        Err(e) => {
            warn!("Failed to read {} content: {}", filename, e);
            vec![]
        }
    }
}

fn gitignore_pattern_to_glob(line: &str) -> String {
    let pattern = line.trim();

    if pattern.starts_with("**/") || pattern.starts_with("**") {
        pattern.to_string()
    } else if pattern.ends_with('/') {
        let dir_name = pattern.trim_end_matches('/');
        format!("**/{}/**", dir_name)
    } else if pattern.contains('/') {
        if pattern.starts_with('/') {
            format!("**{}", pattern)
        } else {
            format!("**/{}", pattern)
        }
    } else {
        format!("**/{}", pattern)
    }
}

/// Builds the standard Reqvire exclusion set from root .gitignore and
/// .reqvireignore relative to the current workspace.
pub fn get_excluded_filename_patterns_glob_set() -> GlobSet {
    let mut builder = GlobSetBuilder::new();

    for pattern in read_gitignore_patterns() {
        if let Ok(glob) = Glob::new(&pattern) {
            builder.add(glob);
            debug!("Added gitignore pattern: {}", pattern);
        } else {
            warn!("Invalid gitignore pattern: {}", pattern);
        }
    }

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
mod tests {
    use super::get_excluded_filename_patterns_glob_set;

    #[test]
    fn builds_standard_excluded_patterns() {
        let _globset = get_excluded_filename_patterns_glob_set();
    }
}
