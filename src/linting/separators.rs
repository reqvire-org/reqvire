use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};

/// Find missing separator lines ("---") before each ## or ### heading.
/// By default, this code will skip adding a separator for the very first matched heading,
/// and only enforce "between consecutive headings."
pub fn find_missing_separators(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    lazy_static! {
        // Match level-2 or level-3 headings, e.g. "## Some Title" or "### Some Title"
        static ref HEADER_REGEX: Regex = Regex::new(r"^#{2,3}\s+(.+)$").unwrap();
    }
    
    let mut suggestions = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    // Track if we've encountered our first matching heading yet
    let mut found_first_element = false;
    // Keep track of the line index of the *previous* heading
    let mut previous_element_index = 0;
    
    for (i, line) in lines.iter().enumerate() {
        // Check if this line is a ## or ### heading
        if HEADER_REGEX.is_match(line) {
            // Skip the first heading encountered
            if found_first_element {
                // Look backward between the previous heading and this heading
                // to see if there's already a separator line ("---").
                let mut has_separator = false;
                
                // We only check lines *after* the previous heading, up to just before i.
                for j in (previous_element_index + 1 .. i).rev() {
                    let check_line = lines[j].trim();
                    
                    // Ignore blank lines
                    if check_line.is_empty() {
                        continue;
                    }
                    // If we find a separator, great—stop
                    if check_line == "---" {
                        has_separator = true;
                        break;
                    }
                    // Otherwise it's some other non-blank line, stop looking
                    break;
                }
                
                // If we didn't find a separator, we need to insert one
                if !has_separator {
                    // Figure out the best place to insert the separator
                    // If the line just above this heading is blank, we'll overwrite that blank;
                    // otherwise, we insert right on this heading's line index.
                    let mut insert_at = i;
                    if i > 0 && lines[i - 1].trim().is_empty() {
                        insert_at = i - 1;
                    }
                    
                    suggestions.push(LintSuggestion::new(
                        LintType::MissingSeparator,
                        file_path.to_path_buf(),
                        Some(i + 1),  // 1-based for human-readability
                        format!("Missing separator before heading '{}'", line.trim()),
                        LintFix::InsertAt {
                            line: insert_at,
                            new_content: "\n\n---\n".to_string(),
                        },
                    ));
                }
            }
            
            // Mark that we've seen at least one heading and store its index
            found_first_element = true;
            previous_element_index = i;
        }
    }
    
    suggestions
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
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
        
        // We expect exactly one suggestion: before "### Second Element Without Separator".
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
        
        // Everything already has separators; no suggestions
        assert_eq!(suggestions.len(), 0);
    }
    
    #[test]
    fn test_no_duplicate_separators_added() {
        let content = r#"# Test Document
        
### First Element
Content here.

---
        
### Second Element With Existing Separator
This already has a separator before it.

### Third Element Without Separator
This should have a separator.
"#;
        
        let file_path = PathBuf::from("test.md");
        let suggestions = find_missing_separators(content, &file_path);
        
        // Only 1 new separator needed: for "Third Element Without Separator".
        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].description.contains("Third Element Without Separator"));
        assert!(!suggestions.iter().any(|s| s.description.contains("Second Element With Existing Separator")));
    }
    
    #[test]
    fn test_multiple_missing_separators() {
    {
        let content = r#"# Document Title

### A
Some content

### B
Content 2

### C
Content 3
"#;

        let file_path = PathBuf::from("test.md");
        let suggestions = find_missing_separators(content, &file_path);

        // We expect 2 suggestions: one for "B" and one for "C"
        assert_eq!(
            suggestions.len(),
            2,
            "Expected exactly two missing-separator suggestions."
        );

        // Check that the suggestions mention the correct headings
        assert!(
            suggestions.iter().any(|s| s.description.contains("B")),
            "Expected a missing-separator suggestion mentioning heading B"
        );
        assert!(
            suggestions.iter().any(|s| s.description.contains("C")),
            "Expected a missing-separator suggestion mentioning heading C"
        );
    }
}    
    
}
