use std::path::{Path, PathBuf};
use anyhow::Result;
use crate::error::ReqFlowError;

/// Represents a lint suggestion (like a warning) that can be fixed automatically
#[derive(Debug, Clone)]
pub struct LintSuggestion {
    /// Type of the lint suggestion
    pub suggestion_type: LintType,
    /// File path where the issue was found
    pub file_path: PathBuf,
    /// Line number if available
    pub line_number: Option<usize>,
    /// Description of the issue
    pub description: String,
    /// Suggested fix
    pub fix: LintFix,
}

/// Type of lint suggestion
#[derive(Debug, Clone, PartialEq)]
pub enum LintType {
    /// Absolute links that could be relative
    AbsoluteLink,
    /// Excess whitespace after element names or relation identifiers
    ExcessWhitespace,
    // The following are currently placeholders for future implementation
    #[allow(dead_code)]
    /// Inconsistent newlines before subsections
    InconsistentNewlines,
    #[allow(dead_code)]
    /// Missing separator lines between elements
    MissingSeparator,
    #[allow(dead_code)]
    /// Inconsistent indentation in relation lists
    InconsistentIndentation,
}

/// Represents a fix for a lint issue
#[derive(Debug, Clone)]
pub enum LintFix {
    /// Replace a specific pattern in a file
    ReplacePattern {
        /// The pattern to find
        pattern: String,
        /// What to replace it with
        replacement: String,
    },
    // The following are placeholders for future implementation
    #[allow(dead_code)]
    /// Replace an entire line in a file
    ReplaceLine {
        /// Line number to replace
        line: usize,
        /// New line content
        new_content: String,
    },
    #[allow(dead_code)]
    /// Insert content at a specific line
    InsertAt {
        /// Line number to insert at
        line: usize,
        /// Content to insert
        content: String,
    },
}

impl LintSuggestion {
    /// Create a new lint suggestion
    pub fn new(
        suggestion_type: LintType,
        file_path: PathBuf,
        line_number: Option<usize>,
        description: String,
        fix: LintFix,
    ) -> Self {
        Self {
            suggestion_type,
            file_path,
            line_number,
            description,
            fix,
        }
    }

    /// Format a user-friendly description of the suggestion
    pub fn format(&self) -> String {
        match &self.line_number {
            Some(line) => format!("{} (in {}:{}): {}", 
                                self.suggestion_type_name(), 
                                self.file_path.display(), 
                                line, 
                                self.description),
            None => format!("{} (in {}): {}", 
                         self.suggestion_type_name(), 
                         self.file_path.display(), 
                         self.description),
        }
    }

    /// Get a user-friendly name for the suggestion type
    fn suggestion_type_name(&self) -> &'static str {
        match self.suggestion_type {
            LintType::AbsoluteLink => "Absolute link",
            LintType::ExcessWhitespace => "Excess whitespace",
            LintType::InconsistentNewlines => "Inconsistent newlines",
            LintType::MissingSeparator => "Missing separator",
            LintType::InconsistentIndentation => "Inconsistent indentation",
        }
    }

    // Apply method is implemented directly in the lint_directory function
    // to avoid redundant code
}

use walkdir::WalkDir;
use std::fs;

/// Run linting on all files in a directory
pub fn lint_directory(directory: &Path, dry_run: bool) -> Result<Vec<LintSuggestion>, ReqFlowError> {
    let mut all_suggestions = Vec::new();
    
    // Find all markdown files in the directory
    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            path.is_file() && path.extension().map_or(false, |ext| ext == "md")
        }) 
    {
        let file_path = entry.path();
        
        // Read the file content
        match fs::read_to_string(file_path) {
            Ok(content) => {
                // Run all linters on this file
                let mut file_suggestions = Vec::new();
                
                // Check for absolute links
                file_suggestions.extend(find_absolute_links(&content, file_path));
                
                // Check for excess whitespace
                file_suggestions.extend(find_excess_whitespace(&content, file_path));
                
                // Check for inconsistent newlines
                file_suggestions.extend(find_inconsistent_newlines(&content, file_path));
                
                // Check for missing separators
                file_suggestions.extend(find_missing_separators(&content, file_path));
                
                // Check for inconsistent indentation
                file_suggestions.extend(find_inconsistent_indentation(&content, file_path));
                
                // If not in dry-run mode, apply the fixes
                if !dry_run {
                    for suggestion in &file_suggestions {
                        match &suggestion.fix {
                            LintFix::ReplacePattern { pattern, replacement } => {
                                // Create a new file content with the fix applied
                                let new_content = content.replace(pattern, replacement);
                                
                                // Write the file back if content has changed
                                if new_content != content {
                                    fs::write(file_path, new_content)?;
                                }
                            },
                            LintFix::ReplaceLine { line, new_content: new_line } => {
                                // Split content into lines, replace the specified line, and join back
                                let mut lines: Vec<&str> = content.lines().collect();
                                if line < &lines.len() {
                                    lines[*line] = new_line;
                                    let new_content = lines.join("\n");
                                    fs::write(file_path, new_content)?;
                                }
                            },
                            LintFix::InsertAt { line, content: insert_content } => {
                                // Split content into lines, insert at the specified line, and join back
                                let mut lines: Vec<&str> = content.lines().collect();
                                if line <= &lines.len() {
                                    lines.insert(*line, insert_content);
                                    let new_content = lines.join("\n");
                                    fs::write(file_path, new_content)?;
                                }
                            },
                        }
                    }
                }
                
                // Add all suggestions to the results
                all_suggestions.extend(file_suggestions);
            },
            Err(err) => {
                // Handle file reading errors
                return Err(ReqFlowError::IoError(err));
            }
        }
    }
    
    Ok(all_suggestions)
}

// Import submodules
pub mod absolute_links;
pub mod whitespace;
pub mod newlines;
pub mod separators;
pub mod indentation;

/// Find absolute links that could be converted to relative links
pub fn find_absolute_links(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    absolute_links::find_absolute_links(content, file_path)
}

/// Find and fix excess whitespace
pub fn find_excess_whitespace(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    whitespace::find_excess_whitespace(content, file_path)
}

/// Find and fix inconsistent newlines
pub fn find_inconsistent_newlines(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    newlines::find_inconsistent_newlines(content, file_path)
}

/// Find missing separator lines
pub fn find_missing_separators(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    separators::find_missing_separators(content, file_path)
}

/// Find inconsistent indentation in relation lists
pub fn find_inconsistent_indentation(content: &str, file_path: &Path) -> Vec<LintSuggestion> {
    indentation::find_inconsistent_indentation(content, file_path)
}

// Add test module
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // Helper function to create a test file path
    fn test_file_path() -> PathBuf {
        PathBuf::from("test.md")
    }

    #[test]
    fn test_lint_suggestion_format() {
        let suggestion = LintSuggestion::new(
            LintType::AbsoluteLink,
            test_file_path(),
            Some(10),
            "Use relative link instead of absolute".to_string(),
            LintFix::ReplacePattern {
                pattern: "[Link](/absolute/path)".to_string(),
                replacement: "[Link](relative/path)".to_string(),
            },
        );

        let formatted = suggestion.format();
        assert!(formatted.contains("Absolute link"));
        assert!(formatted.contains("test.md"));
        assert!(formatted.contains("10"));
        assert!(formatted.contains("Use relative link instead of absolute"));
    }
}