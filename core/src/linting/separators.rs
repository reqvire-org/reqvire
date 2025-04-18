use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};

/// Find missing separator lines ("---") before each ## or ### heading.
/// Completely ignore anything inside <details>…</details>.
pub fn find_missing_separators(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    lazy_static! {
        static ref HEADER_REGEX: Regex      = Regex::new(r"^(##{1,2})\s+.+$").unwrap();
        static ref DETAILS_OPEN: Regex      = Regex::new(r"(?i)<details[^>]*>").unwrap();
        static ref DETAILS_CLOSE: Regex     = Regex::new(r"(?i)</details>").unwrap();
    }

    let lines: Vec<&str> = content.lines().collect();
    let mut suggestions = Vec::new();

    // ——— Phase 0: collect all <details>…</details> intervals ———
    let mut details_intervals = Vec::new();
    let mut in_details = false;
    let mut open_idx = 0;
    for (i, &raw) in lines.iter().enumerate() {
        let t = raw.trim();
        if !in_details && DETAILS_OPEN.is_match(t) {
            in_details = true;
            open_idx = i;
        }

        if in_details && DETAILS_CLOSE.is_match(t) {
            // record _only_ the lines between open and close,
            // i.e. skip [open_idx+1 .. i)
            if open_idx + 1 < i {
                details_intervals.push((open_idx+1)..i);
            }
            in_details = false;
        }        
    }
    // helper to ask “is this line inside any details block?”
    let in_details_block = |idx: usize| {
        details_intervals.iter().any(|r| r.contains(&idx))
    };

    // ——— Phase 1: main heading‐scan, skipping details entirely ———
    let mut heading_count = 0;
    for i in 0..lines.len() {
        if in_details_block(i) {
            continue;
        }
        let trimmed = lines[i].trim();
        if HEADER_REGEX.is_match(trimmed) {
            heading_count += 1;
            if heading_count == 1 {
                // skip the very first heading
                continue;
            }

            // scan backwards for the first *real* line (not blank, not comment,
            // not inside a details block); if it's `---` we’re done.
            let mut need_separator = true;
            let mut k = i;
            while k > 0 {
                k -= 1;
                if in_details_block(k) {
                    // skip any lines inside details
                    continue;
                }
                let prev = lines[k].trim();
                if prev.is_empty() || prev.starts_with("<!--") {
                    continue;
                }
                if prev == "---" {
                    need_separator = false;
                }
                break;
            }

            if need_separator {
                // choose to insert on the blank line before, if it exists
                let insert_at = if i > 0 && lines[i - 1].trim().is_empty() {
                    i - 1
                } else {
                    i
                };
                suggestions.push(LintSuggestion::new(
                    LintType::MissingSeparator,
                    file_path.to_path_buf(),
                    Some(i + 1),
                    format!("Missing separator before heading '{}'", trimmed),
                    LintFix::InsertAt {
                        line: insert_at,
                        new_content: "---".to_string(),
                    },
                ));
            }
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
    #[test]
    fn test_ignore_inside_details() {
        let content = r#"# Doc

<details>
### Hidden Header
Should never warn here.
</details>

### Visible Header
Content.
"#;
        let file_path = PathBuf::from("test.md");
        let suggestions = find_missing_separators(content, &file_path);
        // Only one suggestion (if Visible Header needs a separator)
        // Or zero if Visible Header is first after doc
        assert!(suggestions.iter().all(|s| !s.description.contains("Hidden Header")));
    }
    
}
