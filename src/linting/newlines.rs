use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};

/// Enforces that every header (levels 1–4) in the markdown content is preceded
/// by exactly one blank line. If not, a suggestion is generated using LintType::InconsistentNewlines.
/// Note: This check only looks *before* headers.
pub fn find_inconsistent_newlines(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    lazy_static! {
        // Match headers (levels 1–4) with optional leading whitespace.
        static ref HEADER_REGEX: Regex = Regex::new(r"^\s*(#{1,4})\s+(.+)$").unwrap();
    }
    
    let mut suggestions = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    // Iterate over all lines.
    for i in 0..lines.len() {
        if let Some(caps) = HEADER_REGEX.captures(lines[i]) {
            let header_text = caps.get(2).unwrap().as_str().trim();
            // Skip headers at the very start of the document.
            if i == 0 {
                continue;
            }
            
            // Walk backward from the header to count consecutive blank lines.
            let mut blank_count = 0;
            let mut k = i;
            while k > 0 {
                k -= 1;
                if lines[k].trim().is_empty() {
                    blank_count += 1;
                } else {
                    break;
                }
            }
            
            // If there is not exactly one blank line, we need a suggestion.
            if blank_count == 0 {
                // No blank line: insert one between previous line and the header.
                let prev_line = lines[i - 1];
                suggestions.push(LintSuggestion::new(
                    LintType::InconsistentNewlines,
                    file_path.to_path_buf(),
                    Some(i + 1), // header line (1-indexed)
                    format!("Missing blank line before header '{}'", header_text),
                    LintFix::ReplacePattern {
                        pattern: format!("{}\n{}", prev_line, lines[i]),
                        replacement: format!("{}\n\n{}", prev_line, lines[i]),
                    },
                ));
            } else if blank_count > 1 {
                // Too many blank lines: create a string that represents the actual blank block.
                // For example, if blank_count is 2, then the actual block is "\n\n".
                let pattern = "\n".repeat(blank_count);
                let replacement = "\n".to_string(); // exactly one blank line.
                suggestions.push(LintSuggestion::new(
                    LintType::InconsistentNewlines,
                    file_path.to_path_buf(),
                    Some(k + 2), // starting at the first blank line (1-indexed)
                    format!("Excess blank lines before header '{}'", header_text),
                    LintFix::ReplacePattern {
                        pattern,
                        replacement,
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
    fn test_missing_blank_line_before_header() {
        // In this test the header "## Second Header" is immediately preceded by content.
        let content = r#"# Test Document
Content preceding header.
## Second Header
More content.
"#;
        let file_path = PathBuf::from("test.md");
        let suggestions = find_inconsistent_newlines(content, &file_path);
        // Expect one suggestion for the missing blank line before "## Second Header".
        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].description.contains("Missing blank line before header"));
    }

    #[test]
    fn test_excess_blank_lines_before_header() {
        // In this test, there are two consecutive blank lines before "## Second Header".
        // Lines (ignoring leading/trailing spaces):
        // 0: "# Header"
        // 1: "Some content."
        // 2: ""         (blank)
        // 3: "   "      (blank with spaces)
        // 4: "## Second Header"
        // 5: "More content."
        let content = r#"# Header
Some content.

   
## Second Header
More content.
"#;
        let file_path = PathBuf::from("test.md");
        let suggestions = find_inconsistent_newlines(content, &file_path);
        // Expect one suggestion for excess blank lines before "## Second Header".
        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].description.contains("Excess blank lines before header"));
    }

    #[test]
    fn test_no_issue_with_consistent_blank_line_before_header() {
        // Here, every header is preceded by exactly one blank line.
        let content = r#"# Test Document

Content here.

## Second Header

More content.
"#;
        let file_path = PathBuf::from("test.md");
        let suggestions = find_inconsistent_newlines(content, &file_path);
        // There should be no suggestions when formatting is correct.
        assert_eq!(suggestions.len(), 0);
    }
}

