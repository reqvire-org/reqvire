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
    
    // Find all markdown files in the directory
    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            
            // Check if file is a markdown file
            if !path.is_file() || path.extension().map_or(true, |ext| ext != "md") {
                return false;
            }
            
            // If requirements_only is true, only include files that appear to be requirements documents
            if config.linting.requirements_only {
                // Check if filename contains "Requirements" or the file is in a requirements directory
                let path_str = path.to_string_lossy().to_lowercase();
                let filename = path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
                let req_pattern = config.paths.requirements_filename_match.to_lowercase();
                
                // File with "notreq" or "not_req" in the name should be excluded
                if filename.contains("notreq") || filename.contains("not_req") || filename.contains("not-req") {
                    return false;
                }
                
                // Check different criteria for a requirements file
                let is_req_filename = filename.contains(&req_pattern);
                let is_in_req_directory = path_str.contains(&format!("/{}/", config.paths.system_requirements_folder.to_lowercase())) ||
                                         path_str.contains(&format!("/{}/", req_pattern.to_lowercase()));
                
                // If the directories or filenames match, it's definitely a requirements file
                if is_req_filename || is_in_req_directory {
                    if config.general.verbose {
                        println!("DEBUG: File {} identified as requirements document by name/path", path.display());
                    }
                    return true;
                }
                
                // If still not identified, check file content as a last resort
                if let Ok(content) = std::fs::read_to_string(path) {
                    // Requirements files must have both elements and Relations sections
                    let has_elements = content.contains("### ");
                    let has_relations = content.contains("#### Relations");
                    
                    // And ideally contain some key phrases that indicate it's a requirements document
                    let has_req_keywords = content.to_lowercase().contains("requirement") || 
                                          content.to_lowercase().contains("specification");
                    
                    // The most strict check is to require all three conditions
                    let is_requirements_file = has_elements && has_relations && has_req_keywords;
                    
                    if is_requirements_file && config.general.verbose {
                        println!("DEBUG: File {} identified as requirements document by content", path.display());
                    }
                    
                    is_requirements_file
                } else {
                    false
                }
            } else {
                // If not requirements_only, include all markdown files
                true
            }
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