use anyhow::Result;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;
use std::fs;
use std::path::{Path, PathBuf};
use log::debug;
use globset::GlobSet;
use crate::error::ReqvireError;
use crate::utils;


// Import submodules
pub mod absolute_links;
pub mod whitespace;
pub mod newlines;
pub mod separators;
pub mod indentation;
pub mod reserved_subsections;
pub mod nonlink_identifiers;

pub fn run_linting(
    excluded_filename_patterns: &GlobSet,
    dry_run: bool,
) -> Result<(), ReqvireError> {
    debug!("Starting linting process...");

    let files = utils::scan_markdown_files(
        None,
        excluded_filename_patterns
    );
    debug!("Found {} markdown files to lint", files.len());

    let mut total_suggestions = 0;

    for file_path in files {
        let original_content = fs::read_to_string(&file_path)?;
        let mut modified_content = original_content.clone();

        // ── PHASE 1: STRUCTURAL ──
        let structural_rules: Vec<fn(&str, &Path) -> Vec<LintSuggestion>> = vec![
            // pick whatever order you like now—either nonlink→reserved or reserved→nonlink
            nonlink_identifiers::find_nonlink_identifiers,
            reserved_subsections::fix_reserved_subsections,
            absolute_links::find_absolute_links,
            whitespace::find_excess_whitespace,
            separators::find_missing_separators,
        ];

        let mut all_suggestions = Vec::new();
        for rule in structural_rules {
            let mut suggestions = rule(&remove_details_blocks(&modified_content), &file_path);

            if !dry_run {
                suggestions.sort_by(|a, b| {
                    b.line_number.unwrap_or(0).cmp(&a.line_number.unwrap_or(0))
                });
                for suggestion in &suggestions {
                    modified_content = apply_fix(&modified_content, suggestion);
                    println!(
                        "✅ Applied fix: {} to {}",
                        suggestion.description,
                        file_path.display()
                    );
                }
            }

            all_suggestions.append(&mut suggestions);
        }

        // ── PHASE 2: SPACING ──
        let mut spacing_suggestions =
            newlines::find_inconsistent_newlines(&modified_content, &file_path);
        if !dry_run {
            spacing_suggestions.sort_by(|a, b| {
                b.line_number.unwrap_or(0).cmp(&a.line_number.unwrap_or(0))
            });
            for suggestion in &spacing_suggestions {
                modified_content = apply_fix(&modified_content, suggestion);
                println!(
                    "✅ Applied spacing fix: {} to {}",
                    suggestion.description,
                    file_path.display()
                );
            }
        } else {
            all_suggestions.append(&mut spacing_suggestions);
        }

        // write out if changed
        if !dry_run && modified_content != original_content {
            fs::write(&file_path, &modified_content)?;
        }

        // dry‐run report        
        if dry_run {
            total_suggestions += all_suggestions.len();        
            
            if !all_suggestions.is_empty() {
                for suggestion in &all_suggestions {
                    let _ = suggestion.print_colorized_diff();
                }
                println!("⚠️ Found {} linting issues in {}:", 
                         all_suggestions.len(), file_path.display());
                println!("Run without --dry-run to apply fixes.\n");
            }
        }
    }

    // overall dry‑run summary
    if dry_run && total_suggestions == 0 {
        println!("✅ No linting issues found in any files.");
    }
 
 
    Ok(())
}


/// Applies a single lint fix to the given content.
fn apply_fix(content: &str, suggestion: &LintSuggestion) -> String {
    match &suggestion.fix {
        LintFix::ReplacePattern { pattern, replacement } => content.replace(pattern, replacement),
        LintFix::ReplaceLine { line, new_content } => replace_line(content, *line, new_content),
        LintFix::RemoveLines { lines } => remove_lines(content, lines), 
        LintFix::InsertAt { line, new_content } => insert_at_line(content, *line, new_content),         
    }
}

/// Inserts content at a specific line in the file.
fn insert_at_line(content: &str, line: usize, new_content: &str) -> String {
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    // Ensure the line number is within range (allowing append at EOF)
    if line > lines.len() {
        return content.to_string(); // No change if line number is out of bounds
    }

    // Insert the new content as a single block at the correct position
    lines.insert(line, new_content.to_string());

    // Ensure proper formatting with a trailing newline
    lines.join("\n") + "\n"
}

/// Replaces a specific line in the file content.
fn replace_line(content: &str, line: usize, new_content: &str) -> String {
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    if line >= lines.len() {
        return content.to_string(); // No change if line number is out of bounds
    }

    let new_lines: Vec<String> = new_content.lines().map(String::from).collect();

    // Replace single line with multiple lines if needed
    lines.splice(line..=line, new_lines);

    lines.join("\n") + "\n" // Preserve trailing newline
}

/// Removes multiple lines at once (supports `RemoveLines`)
fn remove_lines(content: &str, lines_to_remove: &[usize]) -> String {
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    // Sort indices in descending order to avoid shifting issues
    let mut sorted_lines: Vec<usize> = lines_to_remove.to_vec();
    sorted_lines.sort_by(|a, b| b.cmp(a));

    for &line in &sorted_lines {
        if line < lines.len() {
            lines.remove(line);
        }
    }

    lines.join("\n") + "\n" // Preserve trailing newline
}

fn remove_details_blocks(content: &str) -> String {
    let mut in_details = false;
    let mut output = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim_start();

        if trimmed.starts_with("<details") {
            in_details = true;
        }

        if !in_details {
            output.push(line.to_string());
        } else {
            // Optionally preserve line count for accurate diffs:
            output.push("<!-- skipped details -->".to_string());
        }

        if trimmed.starts_with("</details>") {
            in_details = false;
        }
    }

    output.join("\n")
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
        new_content: String,
    }, 
    /// Remove n lines in a row from the file.  
    RemoveLines {
        lines: Vec<usize>,
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
            LintFix::InsertAt { line, new_content } => format!("+ <insert at line {}>: {}", line, new_content),

            LintFix::RemoveLines { lines } => {
                lines.iter().map(|line| format!("- <line {} removed>", line)).collect::<Vec<_>>().join("\n")
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
            LintFix::InsertAt { line, new_content } => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                writeln!(stdout, "+ <insert at line {}>: {}", line, new_content)?;
            }

   
            // 4) Remove a specific lines            
            LintFix::RemoveLines { lines } => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                for line in lines {
                    writeln!(stdout, "- <line {} removed>", line)?;
                }
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use globset::GlobSetBuilder;

    /// Helper function to create a test file path
    fn test_file_path() -> PathBuf {
        PathBuf::from("test.md")
    }

    /// Test: Insert content at a specific line
    #[test]
    fn test_insert_at_line() {
        let content = "Line 1\nLine 2\nLine 3\n";
        let result = insert_at_line(content, 1, "Inserted Line");

        assert_eq!(result, "Line 1\nInserted Line\nLine 2\nLine 3\n");
    }

    /// Test: Replace a specific line
    #[test]
    fn test_replace_line() {
        let content = "Line 1\nLine 2\nLine 3\n";
        let result = replace_line(content, 1, "New Line 2");

        assert_eq!(result, "Line 1\nNew Line 2\nLine 3\n");
    }

    /// Test: Remove a specific line
    #[test]
    fn test_remove_line() {
        let content = "Line 1\nLine 2\nLine 3\n";
        let result = remove_lines(content, &[1]);

        assert_eq!(result, "Line 1\nLine 3\n");
    }

    /// Test: Apply Fix - Replace Pattern
    #[test]
    fn test_apply_fix_replace_pattern() {
        let content = "Hello, world!";
        let fix = LintFix::ReplacePattern {
            pattern: "world".to_string(),
            replacement: "Rust".to_string(),
        };
        let suggestion = LintSuggestion::new(
            LintType::ExcessWhitespace,
            test_file_path(),
            Some(1),
            "Replace 'world' with 'Rust'".to_string(),
            fix,
        );

        let result = apply_fix(content, &suggestion);
        assert_eq!(result, "Hello, Rust!");
    }

    /// Test: Apply Fix - Replace Line
    #[test]
    fn test_apply_fix_replace_line() {
        let content = "Hello\nWorld\nRust";
        let fix = LintFix::ReplaceLine {
            line: 1,
            new_content: "New Line".to_string(),
        };
        let suggestion = LintSuggestion::new(
            LintType::InconsistentNewlines,
            test_file_path(),
            Some(1),
            "Replace line 1".to_string(),
            fix,
        );

        let result = apply_fix(content, &suggestion);
        assert_eq!(result, "Hello\nNew Line\nRust\n");
    }

    /// Test: Apply Fix - Insert At Line
    #[test]
    fn test_apply_fix_insert_at() {
        let content = "Hello\nWorld";
        let fix = LintFix::InsertAt {
            line: 1,
            new_content: "Inserted Line".to_string(),
        };
        let suggestion = LintSuggestion::new(
            LintType::InconsistentNewlines,
            test_file_path(),
            Some(1),
            "Insert line at position 1".to_string(),
            fix,
        );

        let result = apply_fix(content, &suggestion);
        assert_eq!(result, "Hello\nInserted Line\nWorld\n");
    }

    /// Test: Apply Fix - Remove Line
    #[test]
    fn test_apply_fix_remove_line() {
        let content = "Hello\nWorld\nRust";
        let fix = LintFix::RemoveLines { lines: [1].to_vec() };
        let suggestion = LintSuggestion::new(
            LintType::NonLinkIdentifier,
            test_file_path(),
            Some(1),
            "Remove line 1".to_string(),
            fix,
        );

        let result = apply_fix(content, &suggestion);
        assert_eq!(result, "Hello\nRust\n");
    }

    /// Test: Running Linting with Dry Run Mode on created test files
    #[test]
    #[serial_test::serial]
    fn test_run_linting_dry_run() {
        use std::process::Command;
        
        // Save current working directory
        let original_dir = std::env::current_dir().unwrap();
        
        // Create a temporary directory for the test
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path();
        
        // Change to temp directory
        std::env::set_current_dir(temp_path).unwrap();
        
        // Initialize git repo in temp directory
        Command::new("git")
            .args(&["init"])
            .current_dir(temp_path)
            .output()
            .expect("Failed to initialize git repo");
            
        Command::new("git")
            .args(&["config", "user.email", "test@example.com"])
            .current_dir(temp_path)
            .output()
            .expect("Failed to set git config");
            
        Command::new("git")
            .args(&["config", "user.name", "Test User"])
            .current_dir(temp_path)
            .output()
            .expect("Failed to set git config");

        // Create a markdown file with intentional linting issues
        let test_file_path = temp_path.join("test.md");
        let test_content = "## Test Header\n\n \nThis is a test file.\n";
        fs::write(&test_file_path, test_content).unwrap();
        
        // Add file to git so it gets scanned
        Command::new("git")
            .args(&["add", "test.md"])
            .current_dir(temp_path)
            .output()
            .expect("Failed to add file to git");

        let excluded_patterns = GlobSetBuilder::new().build().unwrap();
        
        // Run linting - now it will only scan the temp directory
        let result = run_linting(&excluded_patterns, true);

        // Restore original working directory
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok(), "Linting should run without errors on test files");
    }
    /// Test: Remove content within <details> blocks
    #[test]
    fn test_remove_details_blocks() {
        let input = r#"
### Some Element

Intro content.

### Details
<details>
<summary>More</summary>

### Should not be parsed as requirement

This is not requirement.

#### Relations
Broken relation which is not relation.

</details>

After details.
"#;

        let expected = r#"
### Some Element

Intro content.

### Details
<!-- skipped details -->
<!-- skipped details -->
<!-- skipped details -->
<!-- skipped details -->
<!-- skipped details -->
<!-- skipped details -->
<!-- skipped details -->
<!-- skipped details -->
<!-- skipped details -->
<!-- skipped details -->
<!-- skipped details -->

After details.
"#;

        let result = remove_details_blocks(input);

        assert_eq!(result.trim(), expected.trim());
    }    
}

