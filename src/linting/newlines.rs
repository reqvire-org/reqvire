use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};
/// Ensures exactly one blank line before each heading (##, ###, ####) or horizontal rule (`---`).
pub fn find_inconsistent_newlines(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    lazy_static! {
        // Combine your old heading check with an `---` check in a single pattern.
        // This pattern means: start of line, then either exactly `---` or `##+ some text`.
        // Adjust if you also want # or other heading forms.
        static ref TRIGGER_REGEX: Regex = Regex::new(r"^(---|#{2,4}\s+.+)$").unwrap();
    }

    // Keep your existing required blank lines
    let required_blank_lines = 1;
    let lines: Vec<&str> = content.lines().collect();
    let mut suggestions = Vec::new();

    for i in 0..lines.len() {
        // If we're at line 0, there's no line above. Typically skip or handle differently
        if i == 0 {
            continue;
        }
        let current_line = lines[i];

        // Check if line is "### heading" OR "---"
        if TRIGGER_REGEX.is_match(current_line.trim()) {
            // We now enforce exactly one blank line above `i`.
            // Count how many consecutive blank lines are immediately above `i`.
            let mut blank_count = 0;
            let mut j = i;
            while j > 0 {
                if lines[j - 1].trim().is_empty() {
                    blank_count += 1;
                    j -= 1;
                } else {
                    break;
                }
            }

            match blank_count.cmp(&required_blank_lines) {
                // ============================================
                // 0 blank lines => Insert the missing one
                // ============================================
                std::cmp::Ordering::Less => {
                    let needed = required_blank_lines - blank_count;
                    suggestions.push(LintSuggestion::new(
                        LintType::InconsistentNewlines,
                        file_path.to_path_buf(),
                        Some(i + 1),
                        format!(
                            "Missing blank line(s) before '{}'. Need {}.",
                            current_line.trim(),
                            required_blank_lines
                        ),
                        LintFix::InsertAt {
                            line: i,
                            new_content: "\n".repeat(needed),
                        },
                    ));
                }

                // ============================================
                // Exactly 1 blank line => Maybe fix trailing spaces
                // ============================================
                std::cmp::Ordering::Equal => {
                    // If there's exactly 1 blank line, see if it has trailing spaces
                    let blank_line_index = i - 1;
                    let actual_line = lines[blank_line_index];
                    if !actual_line.is_empty() {
                        // Means there's some whitespace
                        suggestions.push(LintSuggestion::new(
                            LintType::InconsistentNewlines,
                            file_path.to_path_buf(),
                            Some(blank_line_index + 1),
                            format!(
                                "Blank line above '{}' contains trailing spaces. Replacing with a truly empty line.",
                                current_line.trim()
                            ),
                            LintFix::ReplaceLine {
                                line: blank_line_index,
                                new_content: "".to_string(),
                            }
                        ));
                    }
                }

                // ============================================
                // More than 1 => remove extras
                // ============================================
                std::cmp::Ordering::Greater => {


                    // We keep the *topmost* blank line, remove the rest
                    let remove_start = j + 1; // j is the index of the first blank line, keep that
                    let remove_end = j + blank_count; 
                    let remove_lines: Vec<usize> = (remove_start..remove_end).collect();

                    suggestions.push(LintSuggestion::new(
                        LintType::InconsistentNewlines,
                        file_path.to_path_buf(),
                        Some(i + 1),
                        format!(
                            "Excess blank lines before '{}'. Expected {}.",
                            current_line.trim(),
                            required_blank_lines
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
        assert!(suggestions[0].description.contains("Missing blank line(s) before '## Second Header'. Need 1."));
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
        assert!(suggestions[0].description.contains("Excess blank lines before '## Second Header'. Expected 1."));
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

