use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};

/// Ensures exactly one blank line before headers (##, ###, ####).
pub fn find_inconsistent_newlines(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    lazy_static! {
        static ref HEADER_REGEX: Regex = Regex::new(r"^\s*(#{1,4})\s+(.+)$").unwrap();
    }

    let required_blank_lines = 1;
    let lines: Vec<&str> = content.lines().collect();
    let mut suggestions = Vec::new();

    // Go top-to-bottom so line numbering remains stable in the final output,
    // and gather all suggestions without attempting to "apply" them yet.
    for i in 0..lines.len() {
    
        // If we're at line 0, there's no "previous line" to consider, so skip
        if i == 0 {
           continue;
        }
        let current_line = lines[i];

        // Check if this line is a header
        if let Some(caps) = HEADER_REGEX.captures(current_line) {
            let header_text = caps.get(2).map_or("", |m| m.as_str().trim());

            // Count the number of consecutive blank lines *immediately* above `i`.
            // We'll walk backwards from `i-1`, as long as lines are blank.
            let mut blank_count = 0;
            let mut j = i;
            while j > 0 {
                let line_above = lines[j - 1];
                if line_above.trim().is_empty() {
                    blank_count += 1;
                    j -= 1;
                } else {
                    break;
                }
            }

            match blank_count.cmp(&required_blank_lines) {
                // Not enough blank lines
                std::cmp::Ordering::Less => {
                    let needed = required_blank_lines - blank_count;
                    // We want to insert these newlines *immediately* above `i - blank_count`.
                    // Because `j` now points to the first non-blank line above the blank block.
                    // The "insert line" for the user is `j + blank_count` or simply `i`.
                    //
                    // For the "line to insert at," we typically use 1-based indexing in lint messages,
                    // so the displayed line is `i + 1` for humans. But the fix struct often wants
                    // a 0-based insertion index. Make sure you’re consistent with your own crate’s convention.
                    let insert_at = i; 
                    suggestions.push(LintSuggestion::new(
                        LintType::InconsistentNewlines,
                        file_path.to_path_buf(),
                        Some(i + 1),
                        format!(
                            "Missing blank line(s) before header '{}'. Need {} blank line(s).",
                            header_text, required_blank_lines
                        ),
                        LintFix::InsertAt {
                            line: insert_at,
                            new_content: "\n".repeat(needed),
                        },
                    ));
                }
                // Exactly right
                std::cmp::Ordering::Equal => {
                    // No suggestion needed
                }
                // Too many blank lines
                std::cmp::Ordering::Greater => {
                    // The "blank lines" above are those in the range (j..i).
                    // We want to remove `excess` lines from that block of blank lines.
                    // E.g., if blank_count=3 and required_blank_lines=1, we have 2 too many lines.
                    // That means removing lines [j, j+1] if those are both blank.
                    let remove_start = j + required_blank_lines; 
                    let remove_end   = j + blank_count; // exclusive end
                    let remove_lines: Vec<usize> = (remove_start..remove_end).collect();

                    suggestions.push(LintSuggestion::new(
                        LintType::InconsistentNewlines,
                        file_path.to_path_buf(),
                        Some(i + 1),
                        format!(
                            "Excess blank lines before header '{}'. Expected {} blank line(s).",
                            header_text, required_blank_lines
                        ),
                        LintFix::RemoveLines { lines: remove_lines },
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
        assert!(suggestions[0].description.contains("Missing blank line(s) before header"));
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

