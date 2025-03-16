use anyhow::Result;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

/// Runs linting on all **Requirement** and **Verification** elements from the `ElementRegistry`,
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use log::debug;
use globset::GlobSet;
use crate::error::ReqFlowError;
use crate::utils;


/// Runs linting checks on all Markdown files in the specification folder.
/// If `dry_run == false`, it automatically applies the fixes.
pub fn run_linting(
    specification_folder: &PathBuf, 
    external_folders: &[PathBuf], 
    excluded_filename_patterns: &GlobSet, 
    dry_run: bool
) -> Result<(), ReqFlowError> {
    debug!("Starting linting process in {:?}", specification_folder);
    
    let mut lint_suggestions = Vec::new();

    // Define all folders to check (specification folder + external folders)
    let all_folders: Vec<&PathBuf> = std::iter::once(specification_folder)
        .chain(external_folders.iter())
        .collect();

    // Iterate over all folders (main + external)
    for folder in &all_folders {
    
             for entry in WalkDir::new(folder)
             .into_iter()
             .filter_map(Result::ok)
             .filter(|e| e.path().is_file())
             .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
             .filter(|e| utils::is_requirements_file_by_path(e.path(), excluded_filename_patterns))
         {
             let file_path = entry.path();

             // No need for `.map_err()` - automatically converts `std::io::Error` to `ReqFlowError::IoError`
             let mut file_content = fs::read_to_string(file_path)?;

             let suggestions = lint_file_content(&file_content, file_path)?;

             if !suggestions.is_empty() {
                 if dry_run {
                     lint_suggestions.extend(suggestions);
                 } else {
                     file_content = apply_fixes(&file_content, &suggestions);
                     fs::write(file_path, file_content)?;
                     println!("✅ Applied {} fixes to {}", suggestions.len(), file_path.display());
                 }
             }
         }
     }

    if dry_run {
        if lint_suggestions.is_empty() {
            println!("✅ No linting issues found.");
        } else {
            println!("⚠️ Found {} linting issues:", lint_suggestions.len());

            for suggestion in &lint_suggestions {
                let _ = suggestion.print_colorized_diff();
            }

            println!("Run without --dry-run to apply fixes.");
        }
    }

    Ok(())
}


/// Applies all lint fixes to the given file content.
fn apply_fixes(content: &str, suggestions: &[LintSuggestion]) -> String {
    let mut updated_content = content.to_string();

    for suggestion in suggestions {
        match &suggestion.fix {
            LintFix::ReplacePattern { pattern, replacement } => {
                updated_content = updated_content.replace(pattern, replacement);
            }
            LintFix::ReplaceLine { line, new_content } => {
                updated_content = replace_line(&updated_content, *line, new_content);
            }
            LintFix::InsertAt { line, content } => {
                updated_content = insert_at_line(&updated_content, *line, content);
            }
        }
    }

    updated_content
}

/// Inserts content at a specific line in the file.
fn insert_at_line(content: &str, line: usize, new_content: &str) -> String {
    let mut lines: Vec<&str> = content.lines().collect();
    if line > lines.len() {
        return content.to_string(); // No change if line number is invalid
    }
    lines.insert(line, new_content);
    lines.join("\n")
}

/// Replaces a specific line in the file content.
fn replace_line(content: &str, line: usize, new_content: &str) -> String {
    let mut lines: Vec<&str> = content.lines().collect();
    if line >= lines.len() {
        return content.to_string(); // No change if line number is invalid
    }
    lines[line] = new_content;
    lines.join("\n")
}


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



    /// Format a git-like diff for the suggestion (used as fallback for non-colorized output)
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
                let pattern_lines: Vec<_> = pattern.lines().collect();
                let replacement_lines: Vec<_> = replacement.lines().collect();

                // Removed lines in RED
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                if pattern_lines.is_empty() {
                    writeln!(&mut stdout, "- <empty line>")?;
                } else {
                    for line in &pattern_lines {
                        writeln!(&mut stdout, "- {}", line)?;
                    }
                }

                // Added lines in GREEN
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



/// Runs linting checks on a file's content and returns a list of linting suggestions.
pub fn lint_file_content(content: &str, file_path: &Path) -> Result<Vec<LintSuggestion>, ReqFlowError> {
    let mut suggestions = Vec::new();


    let result = || -> Result<(), ReqFlowError> {
        // Rule 1: Detect absolute links
        suggestions.extend(find_absolute_links(content, file_path));

        // Rule 2: Detect excess whitespace
        suggestions.extend(find_excess_whitespace(content, file_path));

        // Rule 3: Detect inconsistent newlines
        suggestions.extend(find_inconsistent_newlines(content, file_path));

        // Rule 4: Detect missing separator lines
        suggestions.extend(find_missing_separators(content, file_path));

        // Rule 5: Detect inconsistent indentation
        suggestions.extend(find_inconsistent_indentation(content, file_path));

        Ok(())
    }();

    // If there was an error, return it as `ReqFlowError::LintError`
    result.map(|_| suggestions).map_err(|e| ReqFlowError::LintError(e.to_string()))
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
    whitespace::find_excess_whitespace(content, file_path,)
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
