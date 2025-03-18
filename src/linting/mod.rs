use anyhow::Result;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;
use std::fs;
use std::path::{Path, PathBuf};
use log::debug;
use globset::GlobSet;
use crate::error::ReqFlowError;
use crate::utils;
use std::collections::HashMap;


// Import submodules
pub mod absolute_links;
pub mod whitespace;
pub mod newlines;
pub mod separators;
pub mod indentation;
pub mod index_generator;
pub mod reserved_subsections;
pub mod nonlink_identifiers;

pub fn run_linting(
    specification_folder: &PathBuf, 
    external_folders: &[PathBuf], 
    excluded_filename_patterns: &GlobSet, 
    dry_run: bool
) -> Result<(), ReqFlowError> {
    debug!("Starting linting process in {:?}", specification_folder);
    
    let mut lint_suggestions = Vec::new();
    let files = utils::scan_markdown_files(specification_folder, external_folders, excluded_filename_patterns);
    debug!("Found {} markdown files to lint", files.len());

    for (file_path, _) in files {        
        // Read file content.
        let mut file_content = fs::read_to_string(&file_path)?;

        // Apply each linting rule separately, saving the file after each.
        let linting_rules: Vec<fn(&str, &Path) -> Vec<LintSuggestion>> = vec![
            absolute_links::find_absolute_links,
            whitespace::find_excess_whitespace,
            newlines::find_inconsistent_newlines,
            separators::find_missing_separators,
            indentation::find_inconsistent_indentation,
            reserved_subsections::fix_reserved_subsections,
            nonlink_identifiers::find_nonlink_identifiers,            
        ];

        for lint_rule in linting_rules {
            let mut suggestions = lint_rule(&file_content, &file_path);

            if suggestions.is_empty() {
                continue; // No issues found in this rule, move to the next rule
            }

            if dry_run {
                lint_suggestions.extend(suggestions);
            } else {
                // Sort suggestions by line_number descending so that fixes are applied from bottom to top
                suggestions.sort_by(|a, b| b.line_number.unwrap_or(0).cmp(&a.line_number.unwrap_or(0)));

                for suggestion in &suggestions {
                    let new_file_content = apply_fix(&file_content, suggestion);
                    file_content=new_file_content;
                    println!("✅ Applied fix: {} to {}", suggestion.description, file_path.display());                    
                }
                fs::write(&file_path, &file_content)?;
             
            }
        }
    }
    
    if dry_run {
        if lint_suggestions.is_empty() {
            println!("✅ No linting issues found.");
        } else {
            for suggestion in &lint_suggestions {
                let _ = suggestion.print_colorized_diff();
            }
            println!("⚠️ Found {} linting issues:", lint_suggestions.len());             
            println!("Run without --dry-run to apply fixes.");
        }
    }

    Ok(())
}

/// Applies a single lint fix to the given content.
fn apply_fix(content: &str, suggestion: &LintSuggestion) -> String {
    match &suggestion.fix {
        LintFix::ReplacePattern { pattern, replacement } => content.replace(pattern, replacement),
        LintFix::ReplaceLine { line, new_content } => replace_line(content, *line, new_content),
        LintFix::InsertAt { line, content: insert_content } => insert_at_line(content, *line, insert_content),
        LintFix::RemoveLine { line } => {
            remove_line(content, *line)
        }        
    }
}
/// Inserts content at a specific line in the file.
fn insert_at_line(content: &str, line: usize, new_content: &str) -> String {
    let mut lines: Vec<&str> = content.lines().collect();
    
    // Ensure the line number is within range
    if line > lines.len() {
        return content.to_string(); // No change if line number is out of bounds
    }

    lines.insert(line, new_content);
    lines.join("\n") + "\n" // Preserve trailing newline
}

/// Replaces a specific line in the file content.
fn replace_line(content: &str, line: usize, new_content: &str) -> String {
    let mut lines: Vec<&str> = content.lines().collect();
    
    // Ensure the line number is within range
    if line >= lines.len() {
        return content.to_string(); // No change if line number is out of bounds
    }

    lines[line] = new_content;
    lines.join("\n") + "\n" // Preserve trailing newline
}
/// Removes the line at index `line` (0-based).
fn remove_line(content: &str, line: usize) -> String {
    let mut lines: Vec<&str> = content.lines().collect();
    
    // Ensure the line number is within range
    if line < lines.len() {
        lines.remove(line);
    }
    lines.join("\n") + "\n" // Preserve trailing newline
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
    /// Incosistent Reserved Sections
    InconsistentReservedSubsections,
    /// Identifier not a markdown link
    NonLinkIdentifier    
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
    /// Replace an entire line in a file
    ReplaceLine {
        /// Line number to replace
        line: usize,
        /// New line content
        new_content: String,
    },
    /// Insert content at a specific line
    InsertAt {
        /// Line number to insert at
        line: usize,
        /// Content to insert
        content: String,
    },
    /// Remove a specific line from the file.
    RemoveLine {
        line: usize,
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
        let file_header = format!(
            "diff --lint a/{} b/{}",
            self.file_path.display(),
            self.file_path.display()
        );
        
        let line_info = match &self.line_number {
            Some(line) => format!("@@ line {} @@", line),
            None => "@@ unknown line @@".to_string(),
        };
        
        // Format the change based on the type of fix
        let change_diff = match &self.fix {
            LintFix::ReplacePattern { pattern, replacement } => {
                // Handle multi-line patterns by adding prefix to each line
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
            }
            LintFix::ReplaceLine { line: _, new_content } => {
                format!("- <current line>\n+ {}", new_content)
            }
            LintFix::InsertAt { line: _, content } => {
                // InsertAt means we add a new line
                if content.is_empty() {
                    "+ <empty line>".to_string()
                } else {
                    format!("+ {}", content)
                }
            }
            LintFix::RemoveLine { line } => {
                // Show removal of line (we don't store old content in LintFix::RemoveLine)
                format!("- <line {} removed>", line)
            }
        };
        
        // Give a descriptive header for the suggestion
        let description = format!(
            "# {}: {}",
            self.suggestion_type_name(),
            self.description
        );
        
        format!("{}\n{}\n{}\n{}\n", file_header, description, line_info, change_diff)
    }
    

    /// Print a colorized git-style diff to the terminal for the current suggestion.
    pub fn print_colorized_diff(&self) -> std::io::Result<()> {

        
        let mut stdout = StandardStream::stdout(ColorChoice::Auto);

        // Print file header in bold
        stdout.set_color(ColorSpec::new().set_bold(true))?;
        writeln!(
            &mut stdout,
            "diff --lint a/{} b/{}",
            self.file_path.display(),
            self.file_path.display()
        )?;

        // Print description in cyan
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        writeln!(
            &mut stdout,
            "# {}: {}",
            self.suggestion_type_name(),
            self.description
        )?;

        // Print line info in magenta
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))?;
        let line_info = match &self.line_number {
            Some(line) => format!("@@ line {} @@", line),
            None => "@@ unknown line @@".to_string(),
        };
        writeln!(&mut stdout, "{}", line_info)?;

        // Format and print the change diff with colors
        match &self.fix {
            // 1) Pattern-based fixes
            LintFix::ReplacePattern { pattern, replacement } => {
                // Removed lines in RED
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                let pattern_lines: Vec<_> = pattern.lines().collect();
                if pattern_lines.is_empty() {
                    writeln!(&mut stdout, "- <empty line>")?;
                } else {
                    for line in pattern_lines {
                        writeln!(&mut stdout, "- {}", line)?;
                    }
                }

                // Added lines in GREEN
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                let replacement_lines: Vec<_> = replacement.lines().collect();
                if replacement_lines.is_empty() {
                    writeln!(&mut stdout, "+ <empty line>")?;
                } else {
                    for line in replacement_lines {
                        writeln!(&mut stdout, "+ {}", line)?;
                    }
                }
            }

            // 2) Replace an entire line with new content
            LintFix::ReplaceLine { line: _, new_content } => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                writeln!(&mut stdout, "- <current line>")?;

                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                if new_content.is_empty() {
                    writeln!(&mut stdout, "+ <empty line>")?;
                } else {
                    writeln!(&mut stdout, "+ {}", new_content)?;
                }
            }

            // 3) Insert a new line
            LintFix::InsertAt { line: _, content } => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                if content.is_empty() {
                    writeln!(&mut stdout, "+ <empty line>")?;
                } else {
                    writeln!(&mut stdout, "+ {}", content)?;
                }
            }

            // 4) Remove a specific line
            LintFix::RemoveLine { line } => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                writeln!(&mut stdout, "- <line {} removed>", line)?;
            }
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
            LintType::NonLinkIdentifier => "Nonlink identifier",            
            LintType::ExcessWhitespace => "Excess whitespace",
            LintType::InconsistentNewlines => "Inconsistent newlines",
            LintType::MissingSeparator => "Missing separator",
            LintType::InconsistentIndentation => "Inconsistent indentation",
            LintType::InconsistentReservedSubsections => "Inconsistent reserved subsections",            
        }
    }

    // Apply method is implemented directly in the lint_directory function
    // to avoid redundant code
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

 
}
