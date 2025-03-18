use std::path::{Path};
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};
use crate::utils;
use crate::relation;
/// Finds relation identifiers in a reserved subsection line. The expected format is:
///
///     * relationName: IDENTIFIER
///
/// For example:
/// 
///     * tracedFrom: file.md#Element Name with spaces
///
/// It suggests converting the raw IDENTIFIER into a markdown link where:
/// - The link text is the original identifier (with the leading '#' removed if it's a fragment-only reference)
/// - The link target is the normalized identifier (using utils::normalize_fragment), ensuring that a fragment-only reference gets a '#' prefix.
pub fn find_nonlink_identifiers(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    // Build the alternation group from supported relation types.
    let supported = relation::get_supported_relation_types().join("|");

    // Build a regex pattern that matches:
    //  - begin: leading whitespace, a bullet, at least one space,
    //           then a relation type that is one of the supported types, optional spaces, colon, and spaces.
    //  - id: captures the identifier (the rest of the line, non-greedy).
    //  - end: trailing whitespace.
    let pattern = format!(
        r"(?m)^(?P<begin>\s*\*\s+(?P<relation>(?:{}))\s*:\s*)(?P<id>.+?)(?P<end>\s*)$",
        supported
    );
    let relation_regex = Regex::new(&pattern).unwrap();

    // This regex matches a markdown link, e.g. "[text](target)"
    lazy_static! {
        static ref MD_LINK_REGEX: Regex = Regex::new(r"^\[[^\]]+\]\([^\)]+\)$").unwrap();
    }
        
    let mut suggestions = Vec::new();

    for caps in relation_regex.captures_iter(content) {
        let begin = caps.name("begin").unwrap().as_str();
        let id_str = caps.name("id").unwrap().as_str();
        let end = caps.name("end").unwrap().as_str();

        // Skip if the identifier already looks like a markdown link.
        if MD_LINK_REGEX.is_match(id_str) {
            continue;
        }
        
        // Split the identifier into file part and optional fragment.
        let (file_part, fragment_opt) = utils::extract_path_and_fragment(id_str);

        // Normalize the fragment if present using your utility function.
        let normalized = if let Some(frag) = fragment_opt {
            let norm_frag = utils::normalize_fragment(&frag);
            if file_part.is_empty() {
                // For fragment-only references, always include a leading '#' in the target.
                format!("#{}", norm_frag)
            } else {
                format!("{}#{}", file_part, norm_frag)
            }
        } else {
            file_part.to_string()
        };

        // For link text: if it's a fragment-only reference, display it without the leading '#' (if present).
        let display_text = if file_part.is_empty() {
            if id_str.starts_with('#') {
                id_str.trim_start_matches('#').to_string()
            } else {
                id_str.to_string()
            }
        } else {
            id_str.to_string()
        };

        // Build the markdown link: link text is the processed display_text,
        // and link target is the normalized identifier.
        let link_text = format!("[{}]({})", display_text, normalized);

        // Build the replacement by preserving the captured begin and end.
        let replacement = format!("{}{}{}", begin, link_text, end);

        // The entire matched line (including leading/trailing whitespace).
        let entire_match = caps.get(0).unwrap().as_str();

        suggestions.push(LintSuggestion::new(
            LintType::NonLinkIdentifier,
            file_path.to_path_buf(),
            None, // Optionally compute a line number if desired.
            format!("Convert non-link identifier '{}' to markdown link", id_str),
            LintFix::ReplacePattern {
                pattern: entire_match.to_string(),
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
    use crate::linting::{LintFix, LintSuggestion, LintType};
    // For demonstration, we can define a dummy "normalize_fragment" in test scope
    // or rely on the real crate::utils::normalize_fragment if available

    #[test]
    fn test_non_relation_line() {
        let content = "Check out file.md please.";
        let file_path = PathBuf::from("test.md");

        let suggestions = find_nonlink_identifiers(content, &file_path);

        // Expect zero suggestions
        assert_eq!(suggestions.len(), 0)
    }
    
    #[test]
    fn test_nonrelation_no_fix() {
        let content = " * lovedBy: file.md#Element Name with spaces";
        let file_path = PathBuf::from("test.md");

        let suggestions = find_nonlink_identifiers(content, &file_path);
        assert_eq!(suggestions.len(), 0);

    }    

    #[test]
    fn test_file_md_with_fragment() {
        let content = " * trace: file.md#Element Name with spaces";
        let file_path = PathBuf::from("test.md");

        let suggestions = find_nonlink_identifiers(content, &file_path);
        assert_eq!(suggestions.len(), 1);

        let suggestion = &suggestions[0];

        if let LintFix::ReplacePattern { pattern, replacement } = &suggestion.fix {
            assert!(pattern.contains(" * trace: file.md#Element Name with spaces"));
            assert!(replacement.contains(" * trace: [file.md#Element Name with spaces](file.md#element-name-with-spaces)"));
        } else {
            panic!("Expected ReplacePattern");
        }
    }

    #[test]
    fn test_has_only_fragment() {
        let content = " * refine: #Some Fragment only in doc";
        let file_path = PathBuf::from("test.md");

        let suggestions = find_nonlink_identifiers(content, &file_path);
        assert_eq!(suggestions.len(), 1);

        let suggestion = &suggestions[0];        
        if let LintFix::ReplacePattern { pattern, replacement } = &suggestion.fix {
            assert_eq!(pattern, " * refine: #Some Fragment only in doc");
            assert_eq!(replacement, " * refine: [Some Fragment only in doc](#some-fragment-only-in-doc)");
        } else {
            panic!("Expected ReplacePattern");
        }
    }
    
    #[test]
    fn test_has_only_fragment_no_hashtag() {
        let content = " * refine: Some Fragment only in doc";
        let file_path = PathBuf::from("test.md");

        let suggestions = find_nonlink_identifiers(content, &file_path);
        assert_eq!(suggestions.len(), 1);

        let suggestion = &suggestions[0];

        if let LintFix::ReplacePattern { pattern, replacement } = &suggestion.fix {
            assert_eq!(pattern, " * refine: Some Fragment only in doc");
            assert_eq!(replacement, " * refine: [Some Fragment only in doc](#some-fragment-only-in-doc)");
        } else {
            panic!("Expected ReplacePattern");
        }
    }    

    #[test]
    fn test_file_with_path_and_fragment() {
        let content = " * refine: ../path/to/file.md#Fragment";
        let file_path = PathBuf::from("test.md");

        let suggestions = find_nonlink_identifiers(content, &file_path);
        assert_eq!(suggestions.len(), 1);

        let suggestion = &suggestions[0];

        if let LintFix::ReplacePattern { pattern, replacement } = &suggestion.fix {
            assert_eq!(pattern, " * refine: ../path/to/file.md#Fragment");
            assert_eq!(replacement, " * refine: [../path/to/file.md#Fragment](../path/to/file.md#fragment)");
        } else {
            panic!("Expected ReplacePattern");
        }
    }


    #[test]
    fn test_already_bracketed_link_ignored() {
        // If the line is "[file.md](file.md)", we do NOT want to match it as a raw identifier
        // It's already a link
        let content = "  * containedBy: [file.md](file.md) ";
        let file_path = PathBuf::from("test.md");
        let suggestions = find_nonlink_identifiers(content, &file_path);


        // Expect zero suggestions
        assert_eq!(suggestions.len(), 0);
    }
}





