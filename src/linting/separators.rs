use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};

/// Always ensures there's a `---` line before every `##` or `###` heading,
/// even if it's the first one in the file.
pub fn find_missing_separators(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    lazy_static! {
        // Match level-2 or level-3 headings
        static ref HEADER_REGEX: Regex = Regex::new(r"^#{2,3}\s+(.+)$").unwrap();
    }

    let mut suggestions = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    // For every heading, check if there's a `---` line somewhere between 
    // the *previous heading (if any)* and this heading index, ignoring blank lines.
    // If we find no separator, we add one.
    let mut previous_heading_index: Option<usize> = None;

    for (i, line) in lines.iter().enumerate() {
        if HEADER_REGEX.is_match(line) {
            let start_check = previous_heading_index.map(|idx| idx + 1).unwrap_or(0);

            let mut has_separator = false;
            for j in (start_check..i).rev() {
                let check_line = lines[j].trim();
                if check_line.is_empty() {
                    continue; // skip blank lines
                }
                if check_line == "---" {
                    has_separator = true;
                }
                break; // once we see a non-empty, non-"---" line, stop
            }

            // If we didn't find a separator, suggest inserting one
            if !has_separator {
                // If the line immediately above is blank, we insert the separator there;
                // otherwise, we insert it right at the heading's line index.
                let insert_at = if i > 0 && lines[i - 1].trim().is_empty() {
                    i - 1
                } else {
                    i
                };

                suggestions.push(LintSuggestion::new(
                    LintType::MissingSeparator,
                    file_path.to_path_buf(),
                    Some(i + 1),
                    format!("Missing separator before heading '{}'", line.trim()),
                    LintFix::InsertAt {
                        line: insert_at,
                        new_content: "\n---".to_owned(),
                    },
                ));
            }

            // Update reference for next iteration
            previous_heading_index = Some(i);
        }
    }

    suggestions
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    /// In this test, neither heading has a `---` above it. We expect two fixes:
    ///  1) One for "### First Element"
    ///  2) One for "### Second Element Without Separator"
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
        
        // Now we expect TWO suggestions, because "### First Element" is missing
        // a separator, and "### Second Element Without Separator" is also missing one.
        assert_eq!(
            suggestions.len(),
            2,
            "Expected two missing-separator suggestions"
        );

        // Ensure each suggestion is for LintType::MissingSeparator
        for s in &suggestions {
            assert_eq!(s.suggestion_type, LintType::MissingSeparator);
        }

        // Check that one suggestion is for "First Element"
        assert!(
            suggestions.iter().any(|s| s.description.contains("First Element")),
            "Missing suggestion for 'First Element'"
        );
        // And another for "Second Element Without Separator"
        assert!(
            suggestions
                .iter()
                .any(|s| s.description.contains("Second Element Without Separator")),
            "Missing suggestion for 'Second Element Without Separator'"
        );
    }
    
    /// Demonstrates a file where EVERY level-2 or level-3 heading is properly preceded by `---`.
    /// Since the "First Element" also has a separator, there should be zero suggestions.
    #[test]
    fn test_no_issue_with_proper_separators() {
        let content = r#"# Test Document

---
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
        
        assert_eq!(
            suggestions.len(),
            0,
            "Expected zero suggestions because every heading has a preceding '---'"
        );
    }
    
    /// Demonstrates that if a heading ALREADY has a `---` above it, we don't suggest another.
    /// We DO want a separator for any heading that lacks one. The first heading here is preceded
    /// by `---`, so no suggestion is added for it. The third heading is missing a separator,
    /// so we get exactly one suggestion for "Third Element Without Separator".
    #[test]
    fn test_no_duplicate_separators_added() {
        let content = r#"# Test Document

---
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
        assert_eq!(suggestions[0].suggestion_type, LintType::MissingSeparator);
        assert!(suggestions[0].description.contains("Third Element Without Separator"));
        
        // Double-check we didn't add an extra fix where there's already a separator
        assert!(!suggestions
            .iter()
            .any(|s| s.description.contains("Second Element With Existing Separator")));
        assert!(!suggestions
            .iter()
            .any(|s| s.description.contains("First Element")));
    }
    
    /// Tests that multiple consecutive headings with no separators each get a fix,
    /// i.e. if we have "### A", "### B", "### C" all lacking `---`, we get suggestions
    /// for B and C if the code still decides to skip the *very first* heading. 
    /// If you want to enforce a separator for the first heading as well, expect 3 suggestions.
    #[test]
    fn test_multiple_missing_separators() {
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

        // If your logic truly enforces a separator for EVERY heading,
        // you'll get 3 suggestions (one for A, one for B, one for C).
        // If your code only enforces from the *second* heading onward,
        // you'll see 2 suggestions (for B and C).
        //
        // Adjust accordingly. Here we assume we want a separator for *all* headings:
        assert_eq!(
            suggestions.len(),
            3,
            "Expected exactly three missing-separator suggestions (for A, B, and C)."
        );

        // Check that the suggestions mention the correct headings
        assert!(
            suggestions.iter().any(|s| s.description.contains("A")),
            "Expected a missing-separator suggestion mentioning heading A"
        );
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
