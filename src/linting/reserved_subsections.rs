use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};


/// Fixes reserved subsection headers (level‑4) with names "Relations", "Metadata", or "Properties".
/// Immediately after "#### (Relations|Metadata|Properties)", removes any blank lines
/// and enforces a bullet line that starts with `"  * "`. Also ensures there's exactly
/// one space after a colon (e.g. `key:[stuff]` → `key: [stuff]`).
pub fn fix_reserved_subsections(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    lazy_static! {
        // This pattern matches "#### (Relations|Metadata|Properties)" line (case-insensitive),
        // plus any subsequent blank lines (`\n*`) in multiline mode.
        static ref RESERVED_HEADER_REGEX: Regex = Regex::new(
            r"(?im)^(####\s*(?i:(Relations|Metadata|Properties))\s*)\n*"
        ).unwrap();

        // For ensuring exactly one space after a colon, we match a colon
        // followed by a non-whitespace char ([^\s]) and capture it.
        // We'll do a replacement with `": $1"`.
        static ref AFTER_COLON_RE: Regex = Regex::new(r":([^\s])").unwrap();
    }

    let mut suggestions = Vec::new();

    // We'll split into lines so we can identify line indices more easily
    let all_lines: Vec<&str> = content.lines().collect();

    // For each match in top-to-bottom
    for mat_caps in RESERVED_HEADER_REGEX.captures_iter(content) {
        let header_line = mat_caps.get(1).unwrap().as_str(); 
        // e.g. "#### Relations"

        // Entire matched block (header + blank lines).
        let mat = mat_caps.get(0).unwrap();
        let end_index = mat.end(); 

        // 1) Find line index of the header by counting lines up to `mat.start()`.
        let start_index = mat.start();
        let header_line_index = content[..start_index].lines().count(); 

        // 2) Find the next non-empty line index after `header_line_index`
        let mut bullet_line_index = header_line_index + 1;
        while bullet_line_index < all_lines.len() && all_lines[bullet_line_index].trim().is_empty() {
            bullet_line_index += 1;
        }
        if bullet_line_index >= all_lines.len() {
            // No bullet line at all. We skip or optionally add a default bullet fix
            continue;
        }

        let bullet_line_text = all_lines[bullet_line_index];

        // 3) Force the bullet line to start with "  * " and ensure colons have a space after them.
        // a) Remove leading whitespace and optional "*"
        let bullet_marker_re = Regex::new(r"^\s*\*?\s*").unwrap();
        let bullet_stripped = bullet_marker_re.replace(bullet_line_text, "");

        // b) Ensure exactly one space after colons
        //    e.g. "derivedFrom:[Billing]" => "derivedFrom: [Billing]"
        let bullet_colon_spaced = AFTER_COLON_RE.replace_all(&bullet_stripped, ": $1");

        // c) Prepend "  * "
        let fixed_bullet_line = format!("  * {}", bullet_colon_spaced);

        // If bullet is already correct & no blank lines, skip
        let bullet_ok = bullet_line_text.starts_with("  * ")
            && bullet_line_text.contains(": ")
            && bullet_line_index == header_line_index + 1;
        if bullet_ok {
            continue;
        }

        // 4) We build a single fix that merges [header..bullet] block
        //    into "header_line + \n + fixed_bullet_line"
        // We gather from header_line_index..=bullet_line_index inclusive
        let block_to_replace = all_lines[header_line_index..=bullet_line_index].join("\n");
        let replacement = format!("{}\n{}", header_line.trim_end(), fixed_bullet_line);

        suggestions.push(LintSuggestion::new(
            LintType::InconsistentNewlines,
            file_path.to_path_buf(),
            None,
            format!(
                "Reserved subsection header '{}' should be followed by a bullet starting with \"  * \" and one space after colons.",
                header_line.trim()
            ),
            LintFix::ReplacePattern {
                pattern: block_to_replace,
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


