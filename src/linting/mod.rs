use std::path::{Path, PathBuf};
use anyhow::Result;
use crate::error::ReqFlowError;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LintType {
    /// Absolute links that could be relative
    AbsoluteLink,
    /// Excess whitespace after element names or relation identifiers
    ExcessWhitespace,
    /// Inconsistent newlines before subsections
    InconsistentNewlines,
    /// Missing separator lines between elements
    MissingSeparator,
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

    /// Format a user-friendly description of the suggestion (used for non-visual output)
    #[allow(dead_code)]
    fn format(&self) -> String {
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

    /// Format a git-like diff for the suggestion (used as fallback for non-colorized output)
    #[allow(dead_code)]
    pub fn format_diff(&self) -> String {
        let file_header = format!("diff --lint a/{} b/{}", 
                                self.file_path.display(), 
                                self.file_path.display());
        
        let line_info = match &self.line_number {
            Some(line) => format!("@@ line {} @@", line),
            None => "@@ unknown line @@".to_string(),
        };
        
        // Format the change based on the type of fix
        let change_diff = match &self.fix {
            LintFix::ReplacePattern { pattern, replacement } => {
                // Handle multiline patterns by adding prefix to each line
                let pattern_lines: Vec<_> = pattern.lines().collect();
                let replacement_lines: Vec<_> = replacement.lines().collect();
                
                let mut diff_lines = Vec::new();
                
                // Add the pattern lines with '-' prefix
                for line in &pattern_lines {
                    diff_lines.push(format!("- {}", line));
                }
                
                // Add the replacement lines with '+' prefix
                for line in &replacement_lines {
                    diff_lines.push(format!("+ {}", line));
                }
                
                // If pattern or replacement is empty, show it explicitly
                if pattern_lines.is_empty() {
                    diff_lines.insert(0, "- <empty line>".to_string());
                }
                if replacement_lines.is_empty() {
                    diff_lines.push("+ <empty line>".to_string());
                }
                
                diff_lines.join("\n")
            },
            LintFix::ReplaceLine { line: _, new_content } => {
                format!("- <current line>\n+ {}", new_content)
            },
            LintFix::InsertAt { line: _, content } => {
                format!("+ {}", content)
            },
        };
        
        let description = format!("# {}: {}", self.suggestion_type_name(), self.description);
        
        format!("{}\n{}\n{}\n{}\n", file_header, description, line_info, change_diff)
    }
    
    /// Print a colorized git-style diff to the terminal
    pub fn print_colorized_diff(&self) -> Result<(), std::io::Error> {
        let mut stdout = StandardStream::stdout(ColorChoice::Auto);
        
        // Print file header in bold
        stdout.set_color(ColorSpec::new().set_bold(true))?;
        writeln!(&mut stdout, "diff --lint a/{} b/{}", 
                 self.file_path.display(), 
                 self.file_path.display())?;
        
        // Print description in cyan
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        writeln!(&mut stdout, "# {}: {}", self.suggestion_type_name(), self.description)?;
        
        // Print line info in magenta
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))?;
        let line_info = match &self.line_number {
            Some(line) => format!("@@ line {} @@", line),
            None => "@@ unknown line @@".to_string(),
        };
        writeln!(&mut stdout, "{}", line_info)?;
        
        // Format and print the change diff with colors
        match &self.fix {
            LintFix::ReplacePattern { pattern, replacement } => {
                // Handle multiline patterns by adding prefix to each line
                let pattern_lines: Vec<_> = pattern.lines().collect();
                let replacement_lines: Vec<_> = replacement.lines().collect();
                
                // Add the pattern lines with '-' prefix in red
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                if pattern_lines.is_empty() {
                    writeln!(&mut stdout, "- <empty line>")?;
                } else {
                    for line in &pattern_lines {
                        writeln!(&mut stdout, "- {}", line)?;
                    }
                }
                
                // Add the replacement lines with '+' prefix in green
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                if replacement_lines.is_empty() {
                    writeln!(&mut stdout, "+ <empty line>")?;
                } else {
                    for line in &replacement_lines {
                        writeln!(&mut stdout, "+ {}", line)?;
                    }
                }
            },
            LintFix::ReplaceLine { line: _, new_content } => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                writeln!(&mut stdout, "- <current line>")?;
                
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                writeln!(&mut stdout, "+ {}", new_content)?;
            },
            LintFix::InsertAt { line: _, content } => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                writeln!(&mut stdout, "+ {}", content)?;
            },
        }
        
        // Reset color
        stdout.reset()?;
        writeln!(&mut stdout, "")?;
        
        Ok(())
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

use crate::config::Config;

/// Run linting on all files in a directory
/// 
/// This is kept for backward compatibility. New code should use lint_directory_with_config instead.
#[allow(dead_code)]
pub fn lint_directory(directory: &Path, dry_run: bool) -> Result<Vec<LintSuggestion>, ReqFlowError> {
    // Use default config initially
    lint_directory_with_config(directory, dry_run, &Config::default())
}

/// Run linting on all files in a directory with the given configuration
pub fn lint_directory_with_config(directory: &Path, dry_run: bool, config: &Config) -> Result<Vec<LintSuggestion>, ReqFlowError> {
    let mut all_suggestions = Vec::new();
    
    // Generate the index.md file
    if !dry_run {
        if let Err(e) = index_generator::generate_index(directory, config) {
            log::warn!("Failed to generate index.md: {}", e);
        }
    }
    
    // Find all markdown files in the directory that are requirements documents
    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            crate::utils::is_requirements_file_only(
                e.path(), 
                config, 
                directory, 
                config.general.verbose
            )
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
                if !dry_run && !file_suggestions.is_empty() {
                    // Start with the original content
                    let mut updated_content = content.clone();
                    let mut content_changed = false;
                    
                    // Apply all fixes sequentially to the updated content
                    for suggestion in &file_suggestions {
                        match &suggestion.fix {
                            LintFix::ReplacePattern { pattern, replacement } => {
                                // Apply this fix to the current updated content
                                let new_content = updated_content.replace(pattern, replacement);
                                
                                // Update the content if something was changed
                                if new_content != updated_content {
                                    updated_content = new_content;
                                    content_changed = true;
                                }
                            },
                            LintFix::ReplaceLine { line, new_content: new_line } => {
                                // Split current content into lines, replace the specified line, and join back
                                let mut lines: Vec<&str> = updated_content.lines().collect();
                                if line < &lines.len() {
                                    lines[*line] = new_line;
                                    updated_content = lines.join("\n");
                                    content_changed = true;
                                }
                            },
                            LintFix::InsertAt { line, content: insert_content } => {
                                // Split current content into lines, insert at the specified line, and join back
                                let mut lines: Vec<&str> = updated_content.lines().collect();
                                if line <= &lines.len() {
                                    lines.insert(*line, insert_content);
                                    updated_content = lines.join("\n");
                                    content_changed = true;
                                }
                            },
                        }
                    }
                    
                    // Write the file back only once, after all fixes have been applied
                    if content_changed {
                        fs::write(file_path, updated_content)?;
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
pub mod index_generator;

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