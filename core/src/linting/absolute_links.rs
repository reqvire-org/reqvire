use std::path::{Path, PathBuf};
use regex::Regex;
use lazy_static::lazy_static;
use crate::linting::{LintSuggestion, LintType, LintFix};
use crate::ReqvireError;
use crate::utils;

/// Find absolute links in markdown content that could be converted to relative links
pub fn find_absolute_links(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    lazy_static! {
        static ref ABSOLUTE_LINK_REGEX: Regex = Regex::new(r"\[([^\]]+)\]\(/([^)]+)\)").unwrap();
    }

    let mut suggestions = Vec::new();

    let parent_dir = match file_path.parent() {
        Some(dir) => dir.to_path_buf(),
        None => PathBuf::from("."),
    };

    if !file_path.exists() {
        return suggestions;
    }

    for (line_num, line) in content.lines().enumerate() {       
        for capture in ABSOLUTE_LINK_REGEX.captures_iter(line) {
            let full_match = capture[0].to_string();
            let link_text = &capture[1];
            let link_path = &capture[2];
            
            match utils::to_relative_identifier(link_path, &parent_dir,true) {
                Ok(relative_path_str) => {
                    let replacement = format!("[{}]({})", link_text, relative_path_str);
                    suggestions.push(LintSuggestion::new(
                        LintType::AbsoluteLink,
                        file_path.to_path_buf(),
                        Some(line_num + 1),
                        format!("Absolute link '[{}](/{})', consider using relative path", link_text, link_path),
                        LintFix::ReplacePattern {
                            pattern: full_match,
                            replacement,
                        },
                    ));
                  
                }
                Err(ReqvireError::PathError(_)) => {
                    // Skip this link — could not normalize path (outside repo or invalid)
                    continue;
                }
                Err(e) => {
                    eprintln!("Unexpected path error: {}", e);
                    continue;
                }
            }
        }
    }

    suggestions
}
// Note: Apply functionality is implemented in the main lint_directory function

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;
    use std::fs;
    use std::process::Command;

    fn init_git_repo(path: &Path) {
        Command::new("git")
            .arg("init")
            .current_dir(path)
            .output()
            .expect("Failed to initialize git repo");

        // Optionally add a dummy commit to prevent any git weirdness
        fs::write(path.join("README.md"), "Initial").unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(path)
            .output()
            .expect("Git add failed");

        Command::new("git")
            .args(["commit", "-m", "Initial commit"])
            .current_dir(path)
            .env("GIT_AUTHOR_NAME", "Test")
            .env("GIT_AUTHOR_EMAIL", "test@example.com")
            .env("GIT_COMMITTER_NAME", "Test")
            .env("GIT_COMMITTER_EMAIL", "test@example.com")
            .output()
            .expect("Git commit failed");
    }

    #[test]
    fn test_find_absolute_links() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let temp_path = temp_dir.path();

        // Step 1: Git init
        init_git_repo(temp_path);
        std::env::set_current_dir(temp_path).expect("Failed to set cwd to temp git repo");

        // Step 2: Create documents
        let path_dir = temp_path.join("path/to");
        fs::create_dir_all(&path_dir).expect("Failed to create path/to");
        fs::write(path_dir.join("document.md"), "# Test Document").expect("Failed to write document");

        let another_dir = temp_path.join("another/path");
        fs::create_dir_all(&another_dir).expect("Failed to create another");
        fs::write(another_dir.join("document.md"), "Test content").expect("Failed to write another/path");

        // Step 3: Write test file that contains markdown links
        let content = r###"
# Test Document

### Requirement

Some text

#### Relations
  * derivedFrom: [link to absolute path](/path/to/document.md)
  * derivedFrom: [another one](/another/path/document.md)
  * refine: [relative link](relative/path.md)
  
"###;

        let test_file_path = temp_path.join("path").join("test.md");
        fs::write(&test_file_path, content).expect("Failed to write test.md");

        // Step 4: Run the actual logic
        let suggestions = find_absolute_links(content, &test_file_path);
        dbg!("{}",&suggestions);

        // Step 5: Validate suggestions
        assert_eq!(suggestions.len(), 2);
        assert_eq!(suggestions[0].suggestion_type, LintType::AbsoluteLink);
        assert_eq!(suggestions[0].line_number, Some(9));
        assert!(suggestions[0].description.contains("link to absolute path"));

        assert_eq!(suggestions[1].suggestion_type, LintType::AbsoluteLink);
        assert_eq!(suggestions[1].line_number, Some(10));
        assert!(suggestions[1].description.contains("another one"));
    }
}

