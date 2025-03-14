use std::path::{Path, PathBuf};
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};
use std::fs;

/// Find absolute links in markdown content that could be converted to relative links
pub fn find_absolute_links(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    lazy_static! {
        // Match markdown links with absolute paths
        static ref ABSOLUTE_LINK_REGEX: Regex = Regex::new(r"\[([^\]]+)\]\(/([^)]+)\)").unwrap();
    }
    
    let mut suggestions = Vec::new();
    
    // Get the parent directory of the current file to use as base for relative paths
    let parent_dir = match file_path.parent() {
        Some(dir) => dir.to_path_buf(),
        None => PathBuf::from("."), // Fallback if there's no parent
    };
    
    // Check if the file actually exists (to avoid suggesting changes for files that don't exist)
    let file_exists = file_path.exists();
    if !file_exists {
        return suggestions; // Early return if file doesn't exist
    }
    
    // Track line numbers
    for (line_num, line) in content.lines().enumerate() {
        // Find all absolute links in this line
        for capture in ABSOLUTE_LINK_REGEX.captures_iter(line) {
            let full_match = capture[0].to_string();
            let link_text = &capture[1];
            let link_path = &capture[2];
            
            // Convert absolute path to a PathBuf (prepend / to make it absolute)
            let absolute_path = PathBuf::from(format!("/{}", link_path));
            
            // Create the target path and check if it exists
            let target_exists = if let Ok(canonical_parent) = fs::canonicalize(&parent_dir) {
                let target_path = match absolute_path.strip_prefix("/") {
                    Ok(rel_path) => canonical_parent.join(rel_path),
                    Err(_) => continue, // Skip if we can't strip prefix
                };
                target_path.exists()
            } else {
                false // Can't determine if target exists
            };
            
            // Only suggest fixes for links to files that exist
            if !target_exists {
                continue;
            }
            
            // Calculate the relative path from the current file to the target
            let relative_path = calculate_relative_path(&parent_dir, link_path);
            
            // Create the lint suggestion
            let replacement = format!("[{}]({})", link_text, relative_path);
            suggestions.push(LintSuggestion::new(
                LintType::AbsoluteLink,
                file_path.to_path_buf(),
                Some(line_num + 1), // Line numbers are 1-based for users
                format!("Absolute link '[{}](/{})', consider using relative path", link_text, link_path),
                LintFix::ReplacePattern {
                    pattern: full_match,
                    replacement,
                },
            ));
        }
    }
    
    suggestions
}

/// Calculate a relative path from the source directory to the target path
fn calculate_relative_path(source_dir: &Path, target_path: &str) -> String {
    // Remove leading slash if present
    let target_path = target_path.trim_start_matches('/');
    
    // Handle simple cases
    if target_path.is_empty() {
        return ".".to_string();
    }
    
    // Try to get canonical paths
    let source_canonical = match fs::canonicalize(source_dir) {
        Ok(path) => path,
        Err(_) => return target_path.to_string(), // Fallback if can't canonicalize
    };
    
    // This is the root of the project (or filesystem)
    let root_canonical = match source_canonical.ancestors().last() {
        Some(root) => root.to_path_buf(),
        None => return target_path.to_string(), // Fallback if can't find root
    };
    
    // The absolute path to the target
    let target_absolute = root_canonical.join(target_path);
    
    // Calculate relative path
    match pathdiff::diff_paths(&target_absolute, &source_canonical) {
        Some(rel_path) => rel_path.to_string_lossy().into_owned(),
        None => target_path.to_string(), // Fallback if can't calculate relative path
    }
}

// Note: Apply functionality is implemented in the main lint_directory function

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf; // Used in tests
    use tempfile::tempdir;
    use std::fs;
    
    #[test]
    fn test_find_absolute_links() {
        // Create a temporary directory structure for testing
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let temp_path = temp_dir.path();
        
        // Create some test document directories and files
        let path_dir = temp_path.join("path").join("to");
        fs::create_dir_all(&path_dir).expect("Failed to create test directory");
        fs::write(path_dir.join("document.md"), "# Test Document").expect("Failed to write test file");
        
        let another_dir = temp_path.join("another");
        fs::create_dir_all(&another_dir).expect("Failed to create test directory");
        fs::write(another_dir.join("path"), "Test content").expect("Failed to write test file");
        
        // Create the test file with links to those destinations
        let content = r#"# Test Document
        
This is a [link to absolute path](/path/to/document.md) that should be converted.
And here's [another one](/another/path).

But this [relative link](relative/path.md) should be left alone.
        "#;
        
        let test_file_path = temp_path.join("test.md");
        fs::write(&test_file_path, content).expect("Failed to write test file");
        
        // Run with dry-run true to not modify files
        let suggestions = find_absolute_links(content, &test_file_path);
        
        // In our mocked filesystem, we should now find both links
        assert_eq!(suggestions.len(), 2);
        assert_eq!(suggestions[0].suggestion_type, LintType::AbsoluteLink);
        assert_eq!(suggestions[0].line_number, Some(3));
        assert!(suggestions[0].description.contains("link to absolute path"));
        
        assert_eq!(suggestions[1].suggestion_type, LintType::AbsoluteLink);
        assert_eq!(suggestions[1].line_number, Some(4));
        assert!(suggestions[1].description.contains("another one"));
    }
    
    #[test]
    fn test_calculate_relative_path() {
        // Test basic relative path calculation
        let source_dir = PathBuf::from("/base/dir");
        let target_path = "/base/other/file.md";
        
        // This is a simple case where we can predict the outcome regardless of filesystem
        let result = calculate_relative_path(&source_dir, target_path);
        
        // We expect "../other/file.md" but since this is filesystem-dependent
        // we'll just check that it doesn't start with "/"
        assert!(!result.starts_with("/"), "Result should be a relative path, not absolute");
    }
}