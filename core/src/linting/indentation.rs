use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};

/// Find inconsistent indentation in relation lists
pub fn find_inconsistent_indentation(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    lazy_static! {
        // Match relation sections
        static ref RELATION_SECTION_REGEX: Regex = Regex::new(r"^####\s+Relations$").unwrap();
        // Match relation entries with varying indentation
        static ref RELATION_ENTRY_REGEX: Regex = Regex::new(r"^(\s*)(\*|-)\s+(\w+):\s*(.+)$").unwrap();
    }
    
    let mut suggestions = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    // Track if we're in a relations section
    let mut in_relations_section = false;
    let mut first_relation_indentation: Option<String> = None;
    
    for (i, line) in lines.iter().enumerate() {
        // Check if this line starts a relations section
        if RELATION_SECTION_REGEX.is_match(line) {
            in_relations_section = true;
            first_relation_indentation = None;
            continue;
        }
        
        // If we're in a relations section and find a new heading, we're done with this section
        if in_relations_section && line.trim().starts_with("#") && !RELATION_SECTION_REGEX.is_match(line) {
            in_relations_section = false;
            first_relation_indentation = None;
            continue;
        }
        
        // If we're in a relations section, check relation entries
        if in_relations_section {
            if let Some(captures) = RELATION_ENTRY_REGEX.captures(line) {
                let indent = captures[1].to_string();
                let bullet = &captures[2];
                let relation_type = &captures[3];
                let target = &captures[4];
                
                // If this is the first relation entry in this section, remember its indentation
                if first_relation_indentation.is_none() {
                    first_relation_indentation = Some(indent.clone());
                } else if let Some(ref expected_indent) = first_relation_indentation {
                    // Check if indentation matches the first relation entry
                    if indent != *expected_indent {
                        // Create a fix with consistent indentation
                        let fixed_line = format!("{}* {}: {}", expected_indent, relation_type, target);
                        
                        suggestions.push(LintSuggestion::new(
                            LintType::InconsistentIndentation,
                            file_path.to_path_buf(),
                            Some(i + 1), // Line numbers are 1-based for users
                            format!("Inconsistent indentation for relation '{}'", relation_type),
                            LintFix::ReplacePattern {
                                pattern: line.to_string(),
                                replacement: fixed_line,
                            },
                        ));
                    }
                    
                    // Check if bullet type is consistent (always use *)
                    if bullet != "*" {
                        let fixed_line = format!("{}* {}: {}", indent, relation_type, target);
                        
                        suggestions.push(LintSuggestion::new(
                            LintType::InconsistentIndentation,
                            file_path.to_path_buf(),
                            Some(i + 1), // Line numbers are 1-based for users
                            format!("Inconsistent bullet type for relation '{}'", relation_type),
                            LintFix::ReplacePattern {
                                pattern: line.to_string(),
                                replacement: fixed_line,
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
    use std::path::PathBuf; // Used in tests
    
    #[test]
    fn test_find_inconsistent_indentation() {
        let content = r#"# Test Document
        
### Element
Content here.

#### Relations
* derivedFrom: First Requirement
  * refine: Second Requirement
* verifies: Third Requirement
- implements: Fourth Requirement
"#;
        
        let file_path = PathBuf::from("test.md");
        let suggestions = find_inconsistent_indentation(content, &file_path);
        
        // Should find two issues:
        // 1. Inconsistent indentation in "refine" (has 2 spaces)
        // 2. Inconsistent bullet type in "implements" (uses - instead of *)
        assert_eq!(suggestions.len(), 2);
        
        // Check that one suggestion is about indentation
        let indentation_suggestion = suggestions.iter()
            .find(|s| s.description.contains("indentation"))
            .expect("Should have a suggestion about indentation");
        assert_eq!(indentation_suggestion.suggestion_type, LintType::InconsistentIndentation);
        assert!(indentation_suggestion.description.contains("refine"));
        
        // Check that one suggestion is about bullet type
        let bullet_suggestion = suggestions.iter()
            .find(|s| s.description.contains("bullet"))
            .expect("Should have a suggestion about bullet type");
        assert_eq!(bullet_suggestion.suggestion_type, LintType::InconsistentIndentation);
        assert!(bullet_suggestion.description.contains("implements"));
    }
    
    #[test]
    fn test_no_issue_with_consistent_indentation() {
        let content = r#"# Test Document
        
### Element
Content here.

#### Relations
* derivedFrom: First Requirement
* refine: Second Requirement
* verifies: Third Requirement
* implements: Fourth Requirement
"#;
        
        let file_path = PathBuf::from("test.md");
        let suggestions = find_inconsistent_indentation(content, &file_path);
        
        assert_eq!(suggestions.len(), 0);
    }
}