use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};

/// Find missing separator lines between elements in markdown content
pub fn find_missing_separators(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    lazy_static! {
        // Match element headers (level 3)
        static ref ELEMENT_REGEX: Regex = Regex::new(r"^###\s+(.+)$").unwrap();
    }
    
    let mut suggestions = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    // Track if we've found an element already
    let mut found_first_element = false;
    let mut current_element_index = 0;
    
    for (i, line) in lines.iter().enumerate() {
        // Check if this is an element header
        if ELEMENT_REGEX.is_match(line) {
            // Skip the first element - we only care about transitions between elements
            if found_first_element {
                // Check if there's a separator before this element
                let mut has_separator = false;
                
                // Look backwards from the current line to the previous element
                for j in (current_element_index + 1..i).rev() {
                    let check_line = lines[j].trim();
                    // Empty lines are allowed
                    if check_line.is_empty() {
                        continue;
                    }
                    // If we find a separator, we're good
                    if check_line == "---" {
                        has_separator = true;
                        break;
                    }
                    // If we find non-empty content without a separator first, we need one
                    break;
                }
                
                if !has_separator {
                    // Create a fix by adding a separator
                    suggestions.push(LintSuggestion::new(
                        LintType::MissingSeparator,
                        file_path.to_path_buf(),
                        Some(i + 1), // Line numbers are 1-based for users
                        format!("Missing separator before element '{}'", line.trim()),
                        LintFix::ReplacePattern {
                            pattern: format!("{}\n{}", lines[i - 1], line),
                            replacement: format!("{}\n\n---\n\n{}", lines[i - 1], line),
                        },
                    ));
                }
            }
            
            found_first_element = true;
            current_element_index = i;
        }
    }
    
    suggestions
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf; // Used in tests
    
    #[test]
    fn test_find_missing_separators() {
        let content = r#"# Test Document
        
### First Element
Content here.
Some more content.

### Second Element Without Separator
This should have a separator before it.

---

### Third Element With Separator
This is fine.
"#;
        
        let file_path = PathBuf::from("test.md");
        let suggestions = find_missing_separators(content, &file_path);
        
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].suggestion_type, LintType::MissingSeparator);
        assert!(suggestions[0].description.contains("Second Element Without Separator"));
    }
    
    #[test]
    fn test_no_issue_with_proper_separators() {
        let content = r#"# Test Document
        
### First Element
Content here.

---

### Second Element With Separator
This is fine.

---

### Third Element With Separator
This is also fine.
"#;
        
        let file_path = PathBuf::from("test.md");
        let suggestions = find_missing_separators(content, &file_path);
        
        assert_eq!(suggestions.len(), 0);
    }
}