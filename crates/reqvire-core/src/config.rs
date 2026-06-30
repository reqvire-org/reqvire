use globset::{Glob, GlobSet, GlobSetBuilder};
use log::{debug, warn};
use std::fs;
use std::path::{Path, PathBuf};

/// Reads gitignore patterns from the repository root .gitignore file.
fn read_gitignore_patterns() -> Vec<String> {
    let scope = match crate::workspace::WorkspaceScope::discover() {
        Ok(scope) => scope,
        Err(error) => {
            debug!(
                "No eligible Git worktree found, skipping .gitignore: {}",
                error
            );
            return vec![];
        }
    };

    scope
        .git_worktrees
        .iter()
        .flat_map(|git_root| {
            let prefix = git_root
                .strip_prefix(&scope.root)
                .ok()
                .filter(|path| !path.as_os_str().is_empty())
                .map(PathBuf::from);
            read_root_ignore_patterns(git_root, ".gitignore", prefix.as_deref())
        })
        .collect()
}

/// Reads reqvireignore patterns from the effective workspace root .reqvireignore file.
fn read_reqvireignore_patterns() -> Vec<String> {
    let workspace_root = match crate::workspace::workspace_root() {
        Ok(root) => root,
        Err(error) => {
            debug!(
                "No workspace root found, skipping .reqvireignore: {}",
                error
            );
            return vec![];
        }
    };
    read_root_ignore_patterns(&workspace_root, ".reqvireignore", None)
}

fn read_root_ignore_patterns(root: &Path, filename: &str, prefix: Option<&Path>) -> Vec<String> {
    let ignore_path = root.join(filename);

    if !ignore_path.exists() {
        debug!("No {} file found at {}", filename, root.display());
        return vec![];
    }

    match fs::read_to_string(&ignore_path) {
        Ok(content) => content
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
            .map(|line| gitignore_pattern_to_glob(line, prefix))
            .collect(),
        Err(e) => {
            warn!("Failed to read {} content: {}", filename, e);
            vec![]
        }
    }
}

fn gitignore_pattern_to_glob(line: &str, prefix: Option<&Path>) -> String {
    let pattern = line.trim();

    let glob = if pattern.starts_with("**/") || pattern.starts_with("**") {
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
    };

    if let Some(prefix) = prefix {
        let prefix = prefix.to_string_lossy();
        if glob.starts_with("**/") {
            format!("{}/{}", prefix, glob.trim_start_matches("**/"))
        } else {
            format!("{}/{}", prefix, glob)
        }
    } else {
        glob
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
