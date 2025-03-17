use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};

/// Checks reserved subsection headers (level‑4) with names "Relations", "Metadata", or "Properties".
/// For each such header, it requires that the first non‑empty line following the header
/// is a bullet line that begins exactly with "  * ". If not, a suggestion is generated
/// to remove any extra blank lines and to fix the bullet marker.
pub fn fix_reserved_subsections(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    lazy_static! {
        // Match a reserved header line, case-insensitively.
        // This regex runs in multiline mode: it expects the header on its own line.
        static ref RESERVED_HEADER_REGEX: Regex = Regex::new(
            r"(?m)^(####\s*(?i:(Relations|Metadata|Properties))\s*)\n+"
        ).unwrap();
    }
    
    let mut suggestions = Vec::new();
    
    // Iterate over each reserved header match.
    for caps in RESERVED_HEADER_REGEX.captures_iter(content) {
        let header_line = caps.get(1).unwrap().as_str(); // e.g., "#### Metadata"
        // Get the position of the match in the overall content.
        let mat = caps.get(0).unwrap();
        let end_index = mat.end();
        // Get the remainder of the content after the reserved header and its blank lines.
        if let Some(remaining) = content.get(end_index..) {
            // Find the first non-empty line, which should be the bullet line.
            if let Some(bullet_line) = remaining.lines().find(|l| !l.trim().is_empty()) {
                // Check if the bullet line starts with exactly "  * ".
                if !bullet_line.starts_with("  * ") {
                    // Remove any existing bullet marker: remove any leading whitespace and asterisk.
                    let bullet_marker_re = Regex::new(r"^\s*\*\s*").unwrap();
                    let fixed_bullet_text = bullet_marker_re.replace(bullet_line, "");
                    // Prepend the correct bullet marker.
                    let fixed_bullet_line = format!("  * {}", fixed_bullet_text);
                    
                    // Build the block to replace:
                    // From the reserved header line through the first non-empty line (bullet_line)
                    // We find the range from the start of the header to the end of the bullet line.
                    // For simplicity, we extract the block by splitting the content into lines.
                    // First, get the header line index by counting lines before end_index.
                    let header_line_index = content[..end_index].lines().count() - 1;
                    let all_lines: Vec<&str> = content.lines().collect();
                    // Now, find the index of the bullet line in all_lines (first non-empty line after header).
                    let mut bullet_line_index = header_line_index + 1;
                    while bullet_line_index < all_lines.len() && all_lines[bullet_line_index].trim().is_empty() {
                        bullet_line_index += 1;
                    }
                    if bullet_line_index < all_lines.len() {
                        // Construct the block that spans from the header through the bullet line.
                        let block_to_replace = all_lines[header_line_index..=bullet_line_index].join("\n");
                        let replacement = format!("{}\n{}", header_line.trim_end(), fixed_bullet_line);
                        suggestions.push(LintSuggestion::new(
                            LintType::InconsistentNewlines,
                            file_path.to_path_buf(),
                            None, // optionally compute a line number here
                            format!("Reserved subsection header '{}' should be immediately followed by a bullet starting with \"  * \"", header_line.trim()),
                            LintFix::ReplacePattern {
                                pattern: block_to_replace,
                                replacement,
                            },
                        ));
                    }
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
    fn test_reserved_subsection_failing() {
        let content = r#"#### Metadata

* type: verification
"#;
        let file_path = PathBuf::from("test.md");
        let suggestions = fix_reserved_subsections(content, &file_path);
        
        // We expect one suggestion
        assert_eq!(suggestions.len(), 1, "Expected one suggestion to be generated");
        
        // Expected replacement: header immediately followed by a newline and then a bullet line
        // starting with exactly "  * ".
        let expected_replacement = "#### Metadata\n  * type: verification";
        
        match &suggestions[0].fix {
            LintFix::ReplacePattern { pattern: _, replacement } => {
                assert_eq!(replacement, expected_replacement, "Replacement did not match expected format");
            }
            _ => panic!("Expected ReplacePattern fix"),
        }
    }
    
    #[test]
    fn test_fix_reserved_subsections_with_blank_line_and_bad_bullet() {
        let content = r#"#### Relations

* refine: [UserStories.md/Validating Structures](UserStories.md#validating-structures)
"#;
        let file_path = PathBuf::from("test.md");
        let suggestions = fix_reserved_subsections(content, &file_path);
        // We expect a suggestion because there is a blank line and the bullet does not start with "  * ".
        assert_eq!(suggestions.len(), 1);
        let fix = &suggestions[0].fix;
        if let LintFix::ReplacePattern { pattern, replacement } = fix {
            // The replacement should have the header, a newline, then a bullet line starting with "  * ".
            assert!(replacement.starts_with("#### Relations\n  * "));
        } else {
            panic!("Expected a ReplacePattern fix");
        }
    }

    #[test]
    fn test_fix_reserved_subsections_correct_format() {
        let content = r#"#### Metadata
  * refine: [UserStories.md/Validating Structures](UserStories.md#validating-structures)
"#;
        let file_path = PathBuf::from("test.md");
        let suggestions = fix_reserved_subsections(content, &file_path);
        // There should be no suggestion if the header is immediately followed by a bullet with "  * ".
        assert_eq!(suggestions.len(), 0);
    }
}


