// Format module - handles all formatting-related functionality
// This module contains:
// - File formatting logic (format_files)
// - Formatting rules application (apply_formatting_rules)
// - Diff generation (generate_file_diff)
// - Diff rendering (render_diff, render_diff_json)

use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;
use log::debug;
use std::fs;

/// Result of formatting operation
#[derive(Debug)]
pub struct FormatResult {
    pub files_changed: usize,
    pub diffs: Vec<FileDiff>,
    pub dry_run: bool,
}

/// Represents a diff for a single file
#[derive(Debug, Clone)]
pub struct FileDiff {
    pub file_path: String,
    pub lines: Vec<DiffLine>,
}

/// Represents a single line in a diff
#[derive(Debug, Clone)]
pub struct DiffLine {
    pub prefix: String,
    pub content: String,
    pub color: String, // "green" for additions, "red" for removals, "context" for context
}

/// Format all files in the registry, optionally in dry-run mode
pub fn format_files(registry: &GraphRegistry, dry_run: bool) -> Result<FormatResult, ReqvireError> {
    let base_dir = std::env::current_dir()
        .map_err(|e| ReqvireError::PathError(format!("Failed to get current directory: {}", e)))?;

    let grouped_elements = registry.group_elements_by_location();
    let mut files_changed = 0;
    let mut files_with_diffs = Vec::new();

    // Sort file paths alphabetically for deterministic order
    let mut sorted_files: Vec<_> = grouped_elements.into_iter().collect();
    sorted_files.sort_by(|a, b| a.0.cmp(&b.0));

    for (file_path, sections) in sorted_files {
        // Generate the new markdown content for this file
        let mut new_content = registry.generate_file_markdown(&file_path, &sections);

        // Apply linting rules to ensure consistent formatting
        new_content = apply_formatting_rules(&new_content);

        // Construct the full file path relative to current directory
        let full_file_path = base_dir.join(&file_path);

        // Read current content if file exists
        let current_content = if full_file_path.exists() {
            fs::read_to_string(&full_file_path)
                .map_err(|e| ReqvireError::IoError(e))?
        } else {
            String::new() // File doesn't exist, treat as empty
        };

        // Check if content has changed
        if current_content != new_content {
            files_changed += 1;

            // Generate and store diff for both dry-run and actual formatting
            let diff = generate_file_diff(&file_path, &current_content, &new_content);
            // Only add non-empty diffs
            if !diff.lines.is_empty() {
                files_with_diffs.push(diff);
            }

            if !dry_run {
                // Create parent directories if needed
                if let Some(parent_dir) = full_file_path.parent() {
                    fs::create_dir_all(parent_dir)
                        .map_err(|e| ReqvireError::IoError(e))?;
                }

                // Write the new content
                fs::write(&full_file_path, new_content)
                    .map_err(|e| ReqvireError::IoError(e))?;

                debug!("Formatted {} with {} elements",
                    file_path, sections.values().map(|v| v.len()).sum::<usize>());
            }
        }
    }

    Ok(FormatResult {
        files_changed,
        diffs: files_with_diffs, // Always include diffs for both dry-run and actual formatting
        dry_run,
    })
}

/// Apply basic formatting rules to generated markdown content
fn apply_formatting_rules(content: &str) -> String {
    // Since we're generating from the model, we only need basic cleanup
    // of content that comes from original files (element content, page content, section content)

    // Trim extra whitespace at both beginning and end of the content and ensure proper file ending
    let mut formatted = content.trim().to_string();

    // Ensure file ends with exactly one newline
    if !formatted.is_empty() {
        formatted.push('\n');
    }

    formatted
}

/// Generate a diff showing changes between current and new content
fn generate_file_diff(file_path: &str, current: &str, new: &str) -> FileDiff {
    use difference::{Difference, Changeset};

    let changeset = Changeset::new(current, new, "\n");

    // Check if there are any actual changes (additions or removals)
    let has_changes = changeset.diffs.iter().any(|diff| !matches!(diff, Difference::Same(_)));
    if !has_changes {
        // No actual changes, return empty diff
        return FileDiff {
            file_path: file_path.to_string(),
            lines: Vec::new(),
        };
    }

    let mut diff_lines = Vec::new();

    // Calculate max line numbers to determine padding width
    let max_current_lines = current.lines().count();
    let max_new_lines = new.lines().count();
    let max_line_num = std::cmp::max(max_current_lines, max_new_lines);
    let width = max_line_num.to_string().len();

    let mut new_line_num = 1;
    let mut previous_was_change = false;

    let context_lines = 3; // Number of context lines to show before and after changes

    for (i, diff) in changeset.diffs.iter().enumerate() {
        match diff {
            Difference::Same(text) => {
                let lines: Vec<&str> = text.split('\n').collect();
                let line_count = if lines.last() == Some(&"") { lines.len() - 1 } else { lines.len() };

                // Determine if we should show context lines
                let next_has_change = changeset.diffs.get(i + 1).map_or(false, |d| !matches!(d, Difference::Same(_)));
                let show_context = previous_was_change || next_has_change;

                // Special case: handle empty Same sections (blank lines)
                if text.is_empty() && show_context {
                    diff_lines.push(DiffLine {
                        prefix: format!("{:0width$}", new_line_num, width = width),
                        content: "".to_string(),
                        color: "context".to_string(),
                    });
                    new_line_num += 1;
                } else if show_context && line_count > 0 {
                    // Show context lines
                    // When before first change, show leading context as if it were trailing context
                    let start_lines = if !previous_was_change && next_has_change {
                        // Before first change: show enough leading lines to fill context
                        // This ensures file headers are visible in diff
                        std::cmp::min(context_lines, line_count)
                    } else if previous_was_change {
                        std::cmp::min(context_lines, line_count)
                    } else {
                        0
                    };
                    let end_lines = if next_has_change {
                        std::cmp::min(context_lines, line_count.saturating_sub(start_lines))
                    } else {
                        0
                    };

                    // Show leading context (after a change)
                    for line_idx in 0..start_lines {
                        if line_idx < lines.len() {
                            let content = if lines[line_idx].is_empty() {
                                "".to_string()
                            } else {
                                format!("    {}", lines[line_idx])
                            };
                            diff_lines.push(DiffLine {
                                prefix: format!("{:0width$}", new_line_num + line_idx, width = width),
                                content,
                                color: "context".to_string(),
                            });
                        }
                    }

                    // Show separator if there's a gap in the middle
                    if line_count > start_lines + end_lines && (start_lines > 0 || end_lines > 0) {
                        diff_lines.push(DiffLine {
                            prefix: "".to_string(),
                            content: "".to_string(),
                            color: "separator".to_string(),
                        });
                    }

                    // Show trailing context (before a change)
                    let next_is_removal = changeset.diffs.get(i + 1)
                        .map_or(false, |d| matches!(d, Difference::Rem(_)));
                    let start_end_lines = line_count.saturating_sub(end_lines);
                    for line_idx in start_end_lines..line_count {
                        if line_idx < lines.len() {
                            let line_number = new_line_num + line_idx;
                            let is_blank = lines[line_idx].is_empty();
                            let is_last_blank = line_idx == lines.len() - 1 && is_blank;

                            if !is_last_blank || next_is_removal {
                                // Show all non-blanks, and trailing blanks if they precede a removal
                                let content = if is_blank {
                                    "".to_string()
                                } else {
                                    format!("    {}", lines[line_idx])
                                };
                                diff_lines.push(DiffLine {
                                    prefix: format!("{:0width$}", line_number, width = width),
                                    content,
                                    color: "context".to_string(),
                                });
                            }
                            // Note: if we skip rendering a blank line, line numbering is still preserved
                            // via new_line_num += line_count at the end of Same processing
                        }
                    }

                    new_line_num += line_count;
                    previous_was_change = false;
                } else {
                    // No context needed, just skip these lines
                    new_line_num += line_count;
                    previous_was_change = false;
                }
            },
            Difference::Add(text) => {
                previous_was_change = true;
                for line in text.split('\n') {
                    // For blank lines, use the special character to indicate they're being added
                    let content = if line.is_empty() {
                        "+   ␤".to_string()
                    } else {
                        format!("+   {}", line)
                    };
                    diff_lines.push(DiffLine {
                        prefix: format!("{:0width$}", new_line_num, width = width),
                        content,
                        color: "green".to_string(),
                    });
                    new_line_num += 1;
                }
            },
            Difference::Rem(text) => {
                previous_was_change = true;
                for line in text.split('\n') {
                    // For blank lines, use the special character to indicate they're being removed
                    let content = if line.is_empty() {
                        "-   ␤".to_string()
                    } else {
                        format!("-   {}", line)
                    };
                    diff_lines.push(DiffLine {
                        prefix: format!("{:0width$}", new_line_num, width = width),
                        content,
                        color: "red".to_string(),
                    });
                    // Don't increment new_line_num - removed lines don't exist in new file
                }
            },
        }
    }

    FileDiff {
        file_path: file_path.to_string(),
        lines: diff_lines,
    }
}

/// Render diff output in human-readable format with colors
pub fn render_diff(format_result: &FormatResult) {
    if format_result.dry_run {
        if format_result.diffs.is_empty() {
            println!("No formatting changes needed.");
        } else {
            println!("Found {} file(s) with formatting changes:\n", format_result.diffs.len());
            render_file_diffs(&format_result.diffs);
            println!("Run without --dry-run to apply these changes.");
        }
    } else {
        // Actual formatting - show diffs when changes are applied
        if format_result.files_changed == 0 {
            println!("No files needed formatting.");
        } else {
            println!("Formatted {} file(s):\n", format_result.files_changed);
            render_file_diffs(&format_result.diffs);
        }
    }
}

/// Render file diffs with ANSI colors
fn render_file_diffs(diffs: &[FileDiff]) {
    for file_diff in diffs {
        println!("📄 {}", file_diff.file_path);
        for line in &file_diff.lines {
            match line.color.as_str() {
                "green" => {
                    if line.content.is_empty() {
                        println!("  \x1b[32m{}\x1b[0m", line.prefix)
                    } else {
                        println!("  \x1b[32m{} {}\x1b[0m", line.prefix, line.content)
                    }
                },
                "red" => {
                    if line.content.is_empty() {
                        println!("  \x1b[31m{}\x1b[0m", line.prefix)
                    } else {
                        println!("  \x1b[31m{} {}\x1b[0m", line.prefix, line.content)
                    }
                },
                "context" => {
                    if line.content.is_empty() {
                        println!("  \x1b[37m{}\x1b[0m", line.prefix)
                    } else {
                        println!("  \x1b[37m{} {}\x1b[0m", line.prefix, line.content)
                    }
                },
                "separator" => println!(""),
                _ => {
                    if line.content.is_empty() {
                        println!("  {}", line.prefix)
                    } else {
                        println!("  {} {}", line.prefix, line.content)
                    }
                },
            }
        }
        println!();
        println!();
        println!();
    }
}

/// Render diff output in JSON format
pub fn render_diff_json(format_result: &FormatResult) -> String {
    let json_result = serde_json::json!({
        "dry_run": format_result.dry_run,
        "files_changed": format_result.files_changed,
        "diffs": format_result.diffs.iter().map(|file_diff| {
            serde_json::json!({
                "file_path": file_diff.file_path,
                "lines": file_diff.lines.iter().map(|line| {
                    serde_json::json!({
                        "prefix": line.prefix,
                        "content": line.content,
                        "color": line.color
                    })
                }).collect::<Vec<_>>()
            })
        }).collect::<Vec<_>>()
    });
    serde_json::to_string_pretty(&json_result).unwrap()
}