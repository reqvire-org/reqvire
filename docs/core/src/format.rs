// Format module - handles all formatting-related functionality
// This module contains:
// - File formatting logic (format_files)
// - Formatting rules application (apply_formatting_rules)
// - Diff generation (delegates to diff module)
// - Diff rendering (delegates to diff module)

use crate::diff::{generate_file_diff, render_file_diffs, FileDiff};
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

/// Format all files in the registry, optionally in dry-run mode
/// When with_full_relations is true, includes all relations (user-created and auto-generated inverse relations)
pub fn format_files(
    registry: &GraphRegistry,
    dry_run: bool,
    with_full_relations: bool,
) -> Result<FormatResult, ReqvireError> {
    let base_dir = std::env::current_dir()
        .map_err(|e| ReqvireError::PathError(format!("Failed to get current directory: {}", e)))?;

    let grouped_elements = registry.group_elements_by_location();
    let mut files_changed = 0;
    let mut files_with_diffs = Vec::new();

    // Sort file paths alphabetically for deterministic order
    let mut sorted_files: Vec<_> = grouped_elements.into_iter().collect();
    sorted_files.sort_by(|a, b| a.0.cmp(&b.0));

    for (file_path, elements) in sorted_files {
        // Generate the new markdown content for this file
        let mut new_content =
            registry.generate_file_markdown(&file_path, &elements, with_full_relations);

        // Apply linting rules to ensure consistent formatting
        new_content = apply_formatting_rules(&new_content);

        // Construct the full file path relative to current directory
        let full_file_path = base_dir.join(&file_path);

        // Read current content if file exists
        let current_content = if full_file_path.exists() {
            fs::read_to_string(&full_file_path).map_err(ReqvireError::IoError)?
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
                    fs::create_dir_all(parent_dir).map_err(ReqvireError::IoError)?;
                }

                // Write the new content
                fs::write(&full_file_path, new_content).map_err(ReqvireError::IoError)?;

                debug!("Formatted {} with {} elements", file_path, elements.len());
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

/// Render diff output in human-readable format with colors
pub fn render_diff(format_result: &FormatResult) {
    if format_result.dry_run {
        if format_result.diffs.is_empty() {
            println!("✅ No formatting changes needed.");
        } else {
            println!(
                "Found {} file(s) with formatting changes:\n",
                format_result.diffs.len()
            );
            render_file_diffs(&format_result.diffs);
            println!("Run with --fix to apply these changes.");
        }
    } else {
        // Actual formatting - show diffs when changes are applied
        if format_result.files_changed == 0 {
            println!("✅ No files needed formatting.");
        } else {
            println!("Formatted {} file(s):\n", format_result.files_changed);
            render_file_diffs(&format_result.diffs);
        }
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
