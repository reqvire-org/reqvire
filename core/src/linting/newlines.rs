use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};

/// Ensures exactly one blank line before each heading (##, ###, ####) or horizontal rule (`---`),
/// but ignores any such lines inside <details>…</details> blocks.
pub fn find_inconsistent_newlines(
    content: &str,
    file_path: &Path,
) -> Vec<LintSuggestion> {
    lazy_static! {
        // Trigger on either '---' or any ##/###/#### heading
        static ref TRIGGER_REGEX: Regex = Regex::new(r"^(---|#{2,4}\s+.+)$").unwrap();
        static ref DETAILS_OPEN: Regex  = Regex::new(r"(?i)<details[^>]*>").unwrap();
        static ref DETAILS_CLOSE: Regex = Regex::new(r"(?i)</details>").unwrap();
    }

    let lines: Vec<&str> = content.lines().collect();
    let mut suggestions = Vec::new();

    // Phase 0: record all <details>…</details> intervals
    let mut details_ranges = Vec::new();
    let mut in_details = false;
    let mut start_idx = 0;
    for (i, raw) in lines.iter().enumerate() {
        let t = raw.trim();
        if !in_details && DETAILS_OPEN.is_match(t) {
            in_details = true;
            start_idx = i;
        }
        if in_details && DETAILS_CLOSE.is_match(t) {
            // record only the interior lines [start_idx+1 .. i)
            if start_idx + 1 < i {
                details_ranges.push((start_idx+1)..i);
            }
            in_details = false;
        }        
    }
    let in_details_block = |idx: usize| {
        details_ranges.iter().any(|r| r.contains(&idx))
    };

    let required = 1;

    for i in 0..lines.len() {
        // skip any lines inside details blocks
        if in_details_block(i) {
            continue;
        }
        if i == 0 {
            continue;
        }
        let trimmed = lines[i].trim();
        if !TRIGGER_REGEX.is_match(trimmed) {
            continue;
        }

        // collect all *outside‑details* blank lines immediately above i
        let mut blank_idxs = Vec::new();
        let mut k = i;
        while k > 0 {
            let above = k - 1;
            if in_details_block(above) {
                // skip over the detail region
                k = above;
                continue;
            }
            if lines[above].trim().is_empty() {
                blank_idxs.push(above);
                k = above;
                continue;
            }
            break;
        }

        match blank_idxs.len().cmp(&required) {
            std::cmp::Ordering::Less => {
                let need = required - blank_idxs.len();
                suggestions.push(LintSuggestion::new(
                    LintType::InconsistentNewlines,
                    file_path.to_path_buf(),
                    Some(i + 1),
                    format!("Missing blank line(s) before '{}'. Need {}.", trimmed, required),
                    LintFix::InsertAt {
                        line: i,
                        new_content: "\n".repeat(need),
                    },
                ));
            }
            std::cmp::Ordering::Equal => {
                // exactly one: check for trailing spaces
                let idx = blank_idxs[0];
                if !lines[idx].is_empty() {
                    suggestions.push(LintSuggestion::new(
                        LintType::InconsistentNewlines,
                        file_path.to_path_buf(),
                        Some(idx + 1),
                        format!(
                            "Blank line above '{}' contains trailing spaces. Replacing with a truly empty line.",
                            trimmed
                        ),
                        LintFix::ReplaceLine {
                            line: idx,
                            new_content: "".to_string(),
                        },
                    ));
                }
            }
            std::cmp::Ordering::Greater => {
                // keep only the topmost blank line, remove the rest
                let mut to_remove = blank_idxs.clone();
                to_remove.sort_unstable();     // ascending
                // leave to_remove[0], drop the rest:
                let extra = to_remove.into_iter().skip(1).collect();
                suggestions.push(LintSuggestion::new(
                    LintType::InconsistentNewlines,
                    file_path.to_path_buf(),
                    Some(i + 1),
                    format!("Excess blank lines before '{}'. Expected {}.", trimmed, required),
                    LintFix::RemoveLines { lines: extra },
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
        let content = r#"# Test Document
Content preceding header.
## Second Header
More content.
"#;
        let path = PathBuf::from("test.md");
        let s = find_inconsistent_newlines(content, &path);
        assert_eq!(s.len(), 1);
        assert!(s[0]
            .description
            .contains("Missing blank line(s) before '## Second Header'. Need 1."));
    }

    #[test]
    fn test_excess_blank_lines_before_header() {
        let content = r#"# Header
Some content.

   
## Second Header
More content.
"#;
        let path = PathBuf::from("test.md");
        let s = find_inconsistent_newlines(content, &path);
        assert_eq!(s.len(), 1);
        assert!(s[0]
            .description
            .contains("Excess blank lines before '## Second Header'. Expected 1."));
    }

    #[test]
    fn test_no_issue_with_consistent_blank_line_before_header() {
        let content = r#"# Test Document

Content here.

## Second Header

More content.
"#;
        let path = PathBuf::from("test.md");
        let s = find_inconsistent_newlines(content, &path);
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_ignore_headings_inside_details() {
        let content = r#"# Doc

<details>
<summary>More</summary>

## Hidden
No blank-line warning here.
</details>


## Visible
Needs a blank line before.
"#;
        let path = PathBuf::from("test.md");
        let s = find_inconsistent_newlines(content, &path);
        // Should only report for "## Visible", not "## Hidden"
        assert!(s.iter().all(|x| !x.description.contains("Hidden")));
        assert!(s.iter().any(|x| x.description.contains("Visible")));
    }
}

