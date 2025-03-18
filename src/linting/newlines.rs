use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};
/// Checks that each header (levels 1–4) has exactly 1 blank line before it.
/// Produces InsertAt suggestions if there are too few blank lines,
/// and RemoveLine suggestions if there are too many.
pub fn find_inconsistent_newlines(
    content: &str,
    file_path: &Path,
) -> Vec<LintSuggestion> {
    // We want exactly 1 blank line
    let required_blank_lines = 1;
    
    lazy_static! {
        // Match headers (levels 1..=4) with optional leading whitespace.
        static ref HEADER_REGEX: Regex = Regex::new(r"^\s*(#{1,4})\s+(.+)$").unwrap();
    }

    // Convert the file content into a list of lines
    let lines: Vec<&str> = content.lines().collect();
    let mut suggestions = Vec::new();

    // Traverse lines from top to bottom (0..lines.len()) for detection
    for i in 0..lines.len() {
        // Skip if it's the very first line (can't have preceding lines)
        if i == 0 {
            continue;
        }

        // Check if the line is a header
        if let Some(caps) = HEADER_REGEX.captures(lines[i]) {
            let header_text = caps.get(2).unwrap().as_str().trim();

            // Count consecutive blank lines above this header
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

            // If blank_count == required_blank_lines, do nothing
            if blank_count == required_blank_lines {
                continue;
            }

            let desc = if blank_count < required_blank_lines {
                "Missing blank lines"
            } else {
                "Excess blank lines"
            };

            // The topmost blank line index is (i - blank_count).
            let top_blank_idx = i - blank_count;

            if blank_count < required_blank_lines {
                // We need to insert (required_blank_lines - blank_count) extra blank lines
                let needed = required_blank_lines - blank_count;
                for _ in 0..needed {
                    // We produce an InsertAt suggestion for each missing blank line
                    suggestions.push(LintSuggestion::new(
                        LintType::InconsistentNewlines,
                        file_path.to_path_buf(),
                        Some(i + 1), // 1-based line number for the header
                        format!(
                            "{} before header '{}', enforcing exactly {} blank line(s).",
                            desc, header_text, required_blank_lines
                        ),
                        LintFix::InsertAt {
                            // Insert the blank line above the header
                            line: top_blank_idx,
                            content: "".to_string(), // a truly empty line
                        },
                    ));
                }
            } else {
                // blank_count > required_blank_lines
                // We have too many blank lines; we need to remove (blank_count - required_blank_lines).
                let remove_count = blank_count - required_blank_lines;
                for r in 0..remove_count {
                    let line_to_remove = top_blank_idx + r;
                    // We'll produce a RemoveLine fix for each line that should be removed.
                    suggestions.push(LintSuggestion::new(
                        LintType::InconsistentNewlines,
                        file_path.to_path_buf(),
                        Some(i + 1), // 1-based line number for the header
                        format!(
                            "{} before header '{}', enforcing exactly {} blank line(s).",
                            desc, header_text, required_blank_lines
                        ),
                        LintFix::RemoveLine {
                            line: line_to_remove,
                        },
                    ));
                }
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
        assert!(suggestions[0].description.contains("Missing blank lines before header"));
    }

    #[test]
    fn test_excess_blank_lines_before_header() {
        let content = r#"# Header
Some content.

   
## Second Header
More content.
"#;
        let file_path = PathBuf::from("test.md");
        let suggestions = find_inconsistent_newlines(content, &file_path);
        // Expect one suggestion for excess blank lines before "## Second Header".
        assert_eq!(suggestions.len(), 1);
        println!("{}",&suggestions[0].description);
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

