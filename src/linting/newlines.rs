use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};

/// Find inconsistent newlines before subsections in markdown content
pub fn find_inconsistent_newlines(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    lazy_static! {
        // Match subsection headers (level 4)
        static ref SUBSECTION_REGEX: Regex = Regex::new(r"^####\s+(.+)$").unwrap();
    }
    
    let mut suggestions = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    for (i, line) in lines.iter().enumerate() {
        // Skip the first line since we need to check what's before it
        if i == 0 {
            continue;
        }
        
        // Check if this is a subsection header
        if let Some(capture) = SUBSECTION_REGEX.captures(line) {
            let subsection_name = capture[1].trim();
            let prev_line = lines[i - 1];
            
            // Check if there should be a blank line before subsection
            if !prev_line.trim().is_empty() && i > 1 && !prev_line.trim().starts_with("#") {
                // Create a fix by adding a blank line
                // (We don't need the actual joined content here, just the pattern and replacement for this specific line)
                
                suggestions.push(LintSuggestion::new(
                    LintType::InconsistentNewlines,
                    file_path.to_path_buf(),
                    Some(i + 1), // Line numbers are 1-based for users
                    format!("Missing blank line before subsection '{}'", subsection_name),
                    LintFix::ReplacePattern {
                        pattern: format!("{}\n{}", prev_line, line),
                        replacement: format!("{}\n\n{}", prev_line, line),
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
    use std::path::PathBuf; // Used in tests
    
    #[test]
    fn test_find_inconsistent_newlines() {
        let content = r#"# Test Document
        
### Element Name
Content here.
#### Subsection Without Blank Line
This should have a blank line before it.

#### Subsection With Blank Line
This is fine.
"#;
        
        let file_path = PathBuf::from("test.md");
        let suggestions = find_inconsistent_newlines(content, &file_path);
        
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].suggestion_type, LintType::InconsistentNewlines);
        assert!(suggestions[0].description.contains("Subsection Without Blank Line"));
    }
    
    #[test]
    fn test_no_issue_with_consistent_newlines() {
        let content = r#"# Test Document
        
### Element Name
Content here.

#### Subsection With Blank Line
This is fine.

#### Another Subsection With Blank Line
This is also fine.
"#;
        
        let file_path = PathBuf::from("test.md");
        let suggestions = find_inconsistent_newlines(content, &file_path);
        
        assert_eq!(suggestions.len(), 0);
    }
}