use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};

/// Find absolute links in markdown content that could be converted to relative links
pub fn find_absolute_links(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    lazy_static! {
        // Match markdown links with absolute paths
        static ref ABSOLUTE_LINK_REGEX: Regex = Regex::new(r"\[([^\]]+)\]\(/([^)]+)\)").unwrap();
    }
    
    let mut suggestions = Vec::new();
    
    // Track line numbers
    for (line_num, line) in content.lines().enumerate() {
        // Find all absolute links in this line
        for capture in ABSOLUTE_LINK_REGEX.captures_iter(line) {
            let full_match = capture[0].to_string();
            let link_text = &capture[1];
            let link_path = &capture[2];
            
            // Calculate the relative path
            let relative_path = link_path.to_string(); // This is simplified - real implementation would calculate actual relative path
            
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

// Note: Apply functionality is implemented in the main lint_directory function

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf; // Used in tests
    
    #[test]
    fn test_find_absolute_links() {
        let content = r#"# Test Document
        
This is a [link to absolute path](/path/to/document.md) that should be converted.
And here's [another one](/another/path).

But this [relative link](relative/path.md) should be left alone.
        "#;
        
        let file_path = PathBuf::from("test.md");
        let suggestions = find_absolute_links(content, &file_path);
        
        assert_eq!(suggestions.len(), 2);
        assert_eq!(suggestions[0].suggestion_type, LintType::AbsoluteLink);
        assert_eq!(suggestions[0].line_number, Some(3));
        assert!(suggestions[0].description.contains("link to absolute path"));
        
        assert_eq!(suggestions[1].suggestion_type, LintType::AbsoluteLink);
        assert_eq!(suggestions[1].line_number, Some(4));
        assert!(suggestions[1].description.contains("another one"));
    }
}