use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};


/// Fixes level‑4 reserved subsections named Relations, Metadata, or Properties.
/// It merges the header + any intervening blank lines + the bullet line
/// into one block, replacing it with:
/// ```text
/// #### HeaderName
///   * key: rest
/// ```
/// preserving whatever “rest” was (link, text, etc.).
pub fn fix_reserved_subsections(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    lazy_static! {
        // Match "#### Relations" (or Metadata|Properties) plus any number of \n after it
        static ref RESERVED_HEADER_REGEX: Regex =
            Regex::new(r"(?m)^####\s*(?:Relations|Metadata|Properties)\s*\n*")
                .unwrap();
        // For tightening up "key:[foo]" → "key: [foo]"
        static ref AFTER_COLON_RE: Regex = Regex::new(r":([^\s])").unwrap();
        // To strip off whatever bullet marker was there
        static ref BULLET_PREFIX_RE: Regex = Regex::new(r"^\s*\*?\s*").unwrap();
    }

    let mut suggestions = Vec::new();
    let all_lines: Vec<&str> = content.lines().collect();

    for mat in RESERVED_HEADER_REGEX.find_iter(content) {
        // Figure out which line index this match starts on
        let header_start = mat.start();
        let header_idx = content[..header_start].lines().count();

        // Guard against a weird match at the very end
        if header_idx >= all_lines.len() {
            continue;
        }

        let header_line = all_lines[header_idx].trim_end();

        // Now find the first non‑blank line after the header
        let mut bullet_idx = header_idx + 1;
        while bullet_idx < all_lines.len() && all_lines[bullet_idx].trim().is_empty() {
            bullet_idx += 1;
        }
        if bullet_idx >= all_lines.len() {
            // no bullet to fix
            continue;
        }
        let bullet_line = all_lines[bullet_idx];

        // Build the corrected bullet
        let stripped = BULLET_PREFIX_RE.replace(bullet_line, "");
        let spaced  = AFTER_COLON_RE.replace_all(&stripped, ": $1");
        let fixed_bullet = format!("  * {}", spaced);

        // If it was already exactly "  * key: rest" on the very next line, skip
        let already_ok = bullet_line.starts_with("  * ")
            && bullet_line.contains(": ")
            && bullet_idx == header_idx + 1;
        if already_ok {
            continue;
        }

        // Prepare the single ReplacePattern:
        //  - pattern = all_lines[header_idx..=bullet_idx].join("\n")
        //  - replacement = header_line + "\n" + fixed_bullet
        let block = all_lines[header_idx..=bullet_idx].join("\n");
        let replacement = format!("{}\n{}", header_line, fixed_bullet);

        suggestions.push(LintSuggestion::new(
            LintType::InconsistentReservedSubsections,
            file_path.to_path_buf(),
            Some(bullet_idx + 1),
            format!(
                "Reserved subsection header '{}' should be followed by a bullet \
                 starting with \"  * \" and one space after colons.",
                header_line
            ),
            LintFix::ReplacePattern {
                pattern: block,
                replacement,
            },
        ));
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

*type: verification
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
        if let LintFix::ReplacePattern { pattern: _, replacement } = fix {
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


