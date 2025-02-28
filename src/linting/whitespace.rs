use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};

/// Find excess whitespace in markdown content
pub fn find_excess_whitespace(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    lazy_static! {
        // Match element headers with excess whitespace
        static ref ELEMENT_WHITESPACE_REGEX: Regex = Regex::new(r"###\s+(.+?)\s{2,}$").unwrap();
        // Match relation identifiers with excess whitespace
        static ref RELATION_WHITESPACE_REGEX: Regex = Regex::new(r"\s*\*\s+(\w+):\s{2,}").unwrap();
    }
    
    let mut suggestions = Vec::new();
    
    // Track line numbers
    for (line_num, line) in content.lines().enumerate() {
        // Check for excess whitespace in element headers
        if let Some(capture) = ELEMENT_WHITESPACE_REGEX.captures(line) {
            let full_match = capture[0].to_string();
            let element_name = capture[1].trim();
            
            // Create the fix
            let replacement = format!("### {}", element_name);
            suggestions.push(LintSuggestion::new(
                LintType::ExcessWhitespace,
                file_path.to_path_buf(),
                Some(line_num + 1), // Line numbers are 1-based for users
                format!("Excess whitespace after element name '{}'", element_name),
                LintFix::ReplacePattern {
                    pattern: full_match,
                    replacement,
                },
            ));
        }
        
        // Check for excess whitespace in relation identifiers
        if let Some(capture) = RELATION_WHITESPACE_REGEX.captures(line) {
            let full_match = capture[0].to_string();
            let relation_type = capture[1].trim();
            
            // Create the fix
            let replacement = format!("* {}: ", relation_type);
            suggestions.push(LintSuggestion::new(
                LintType::ExcessWhitespace,
                file_path.to_path_buf(),
                Some(line_num + 1), // Line numbers are 1-based for users
                format!("Excess whitespace after relation identifier '{}'", relation_type),
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
    fn test_find_excess_whitespace_in_element_headers() {
        let content = r#"# Test Document
        
### Element Name With Excess Whitespace    

This is normal content.

### Normal Element
        "#;
        
        let file_path = PathBuf::from("test.md");
        let suggestions = find_excess_whitespace(content, &file_path);
        
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].suggestion_type, LintType::ExcessWhitespace);
        assert_eq!(suggestions[0].line_number, Some(3));
        assert!(suggestions[0].description.contains("Element Name With Excess Whitespace"));
    }
    
    #[test]
    fn test_find_excess_whitespace_in_relation_identifiers() {
        let content = r#"# Test Document
        
### Element

#### Relations
* derivedFrom:     User Requirement
* refine: Another Requirement
        "#;
        
        let file_path = PathBuf::from("test.md");
        let suggestions = find_excess_whitespace(content, &file_path);
        
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].suggestion_type, LintType::ExcessWhitespace);
        assert_eq!(suggestions[0].line_number, Some(6));
        assert!(suggestions[0].description.contains("derivedFrom"));
    }
}