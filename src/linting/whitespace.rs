use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};

/// Find excess whitespace in markdown content
pub fn find_excess_whitespace(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    lazy_static! {
        // Match element headers (level 3) with excess whitespace - ensure it's exactly 3 #
        static ref ELEMENT_WHITESPACE_REGEX: Regex = Regex::new(r"^###\s+([^#].+?)\s{2,}$").unwrap();
        // Match subsection headers (level 4) with excess whitespace
        static ref SUBSECTION_WHITESPACE_REGEX: Regex = Regex::new(r"^####\s+(.+?)\s{2,}$").unwrap();
        // Match relation identifiers with excess whitespace
        static ref RELATION_WHITESPACE_REGEX: Regex = Regex::new(r"\s*\*\s+(\w+):\s{2,}").unwrap();
    }
    
    let mut suggestions = Vec::new();
    
    // Track line numbers
    for (line_num, line) in content.lines().enumerate() {
        // Check for excess whitespace in element headers (level 3)
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
        
        // Check for excess whitespace in subsection headers (level 4)
        if let Some(capture) = SUBSECTION_WHITESPACE_REGEX.captures(line) {
            let full_match = capture[0].to_string();
            let subsection_name = capture[1].trim();
            
            // Create the fix
            let replacement = format!("#### {}", subsection_name);
            suggestions.push(LintSuggestion::new(
                LintType::ExcessWhitespace,
                file_path.to_path_buf(),
                Some(line_num + 1), // Line numbers are 1-based for users
                format!("Excess whitespace after subsection name '{}'", subsection_name),
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

#### Subsection With Excess Whitespace    

Normal subsection content.
        "#;
        
        let file_path = PathBuf::from("test.md");
        let suggestions = find_excess_whitespace(content, &file_path);
        
        // Debug print commented out to keep tests quiet
        // for (i, suggestion) in suggestions.iter().enumerate() {
        //     println!("Suggestion {}: {}", i, suggestion.description);
        // }
        
        assert_eq!(suggestions.len(), 2);
        
        // Check element header fix
        let element_fix = suggestions.iter().find(|s| s.description.contains("Element Name With Excess Whitespace"))
            .expect("Should have found excess whitespace in element");
        assert_eq!(element_fix.suggestion_type, LintType::ExcessWhitespace);
        assert_eq!(element_fix.line_number, Some(3));
        
        // Check subsection header fix
        let subsection_fix = suggestions.iter().find(|s| s.description.contains("Subsection With Excess Whitespace"))
            .expect("Should have found excess whitespace in subsection");
        assert_eq!(subsection_fix.suggestion_type, LintType::ExcessWhitespace);
        assert_eq!(subsection_fix.line_number, Some(9));
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