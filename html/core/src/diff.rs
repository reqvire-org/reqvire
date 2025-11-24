// Diff module - handles diff generation and rendering for file operations
// This module contains:
// - Diff structures (FileDiff, DiffLine)
// - Diff generation (generate_file_diff)
// - Diff rendering (render_file_diffs, render_crud_result, render_crud_json)
// - CRUD operation result structures (CrudResult, CrudOperation)

use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;
use std::fs;
use std::path::Path;

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

/// CRUD operation type
#[derive(Debug, Clone)]
pub enum CrudOperation {
    Add,
    Remove,
    Move,
    Rename,
    Update,
}

/// Result of a CRUD operation
#[derive(Debug)]
pub struct CrudResult {
    pub operation: CrudOperation,
    pub element_id: String,
    pub element_name: String,
    pub diffs: Vec<FileDiff>,
    pub dry_run: bool,
}

/// Generate a diff showing changes between current and new content
pub fn generate_file_diff(file_path: &str, current: &str, new: &str) -> FileDiff {
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

/// Render file diffs with ANSI colors
pub fn render_file_diffs(diffs: &[FileDiff]) {
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

/// Generate diffs for modified files after a CRUD operation
pub fn generate_crud_diffs(
    registry: &GraphRegistry,
    modified_files: &[String],
    git_root: &Path,
) -> Result<Vec<FileDiff>, ReqvireError> {
    let mut diffs = Vec::new();

    // Get all elements grouped by location
    let grouped_elements = registry.group_elements_by_location();

    for file_path in modified_files {
        let full_path = git_root.join(file_path);

        // Get original content if file exists
        let original_content = if full_path.exists() {
            fs::read_to_string(&full_path)?
        } else {
            String::new()
        };

        // Generate new content from registry
        let new_content = if let Some(sections) = grouped_elements.get(file_path) {
            registry.generate_file_markdown(file_path, sections)
        } else {
            // File was deleted or emptied
            String::new()
        };

        // Generate diff
        let diff = generate_file_diff(file_path, &original_content, &new_content);

        // Only add non-empty diffs
        if !diff.lines.is_empty() {
            diffs.push(diff);
        }
    }

    Ok(diffs)
}

/// Render CRUD operation result in human-readable format
pub fn render_crud_result(result: &CrudResult) {
    let operation_name = match result.operation {
        CrudOperation::Add => "Added",
        CrudOperation::Remove => "Removed",
        CrudOperation::Move => "Moved",
        CrudOperation::Rename => "Renamed",
        CrudOperation::Update => "Updated",
    };

    if result.dry_run {
        println!("{} element: {} ({})", operation_name, result.element_name, result.element_id);
        println!("Dry run - no files modified\n");
    } else {
        println!("{} element: {} ({})\n", operation_name, result.element_name, result.element_id);
    }

    if !result.diffs.is_empty() {
        render_file_diffs(&result.diffs);
    } else {
        println!("No file changes.");
    }
}

/// Render CRUD operation result in JSON format
pub fn render_crud_json(result: &CrudResult) -> String {
    let operation_str = match result.operation {
        CrudOperation::Add => "add",
        CrudOperation::Remove => "remove",
        CrudOperation::Move => "move",
        CrudOperation::Rename => "rename",
        CrudOperation::Update => "update",
    };

    let json_result = serde_json::json!({
        "operation": operation_str,
        "element_id": result.element_id,
        "element_name": result.element_name,
        "dry_run": result.dry_run,
        "diffs": result.diffs.iter().map(|file_diff| {
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
