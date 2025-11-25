// CRUD module - high-level operations for element manipulation
// This module contains all business logic for Create, Read, Update, Delete operations
// CLI should only parse arguments and call these functions

use crate::error::ReqvireError;
use crate::model::ModelManager;
use crate::diff::{CrudResult, CrudOperation, generate_crud_diffs, generate_file_diff};
use globset::GlobSet;
use std::path::Path;

/// Add a new element to the model
///
/// # Arguments
/// * `model_manager` - The model manager
/// * `element_markdown` - The markdown content for the element
/// * `target_file` - Target file path (relative to current working directory)
/// * `index` - Optional index for insertion
/// * `excluded_patterns` - Patterns to exclude from path validation
/// * `current_dir` - Current working directory (where command was invoked)
/// * `git_root` - Git root directory
/// * `dry_run` - If true, don't write changes to disk
pub fn add_element(
    model_manager: &mut ModelManager,
    element_markdown: &str,
    target_file: &str,
    index: Option<usize>,
    excluded_patterns: &GlobSet,
    current_dir: &Path,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    // Normalize target_file: convert from CWD-relative to git-root-relative
    use crate::utils;
    let absolute_target = current_dir.join(target_file);
    let target_file_normalized = utils::get_relative_path(&absolute_target)?
        .to_string_lossy()
        .to_string();
    // Track which files were modified before the operation
    let modified_before: Vec<String> = model_manager.graph_registry.modified_files
        .iter()
        .cloned()
        .collect();

    // Create element using core business logic
    let element = model_manager.graph_registry.create_element_from_string(
        element_markdown,
        &target_file_normalized,
        index,
        excluded_patterns,
    )?;

    // Get list of newly modified files (sorted for deterministic output)
    let mut modified_files: Vec<String> = model_manager.graph_registry.modified_files
        .iter()
        .filter(|f| !modified_before.contains(f))
        .cloned()
        .collect();
    modified_files.sort();

    // Generate diffs for output
    let diffs = generate_crud_diffs(
        &model_manager.graph_registry,
        &modified_files,
        git_root,
    )?;

    // Flush changes if not dry-run
    if !dry_run {
        model_manager.graph_registry.flush_modified_files(git_root)?;
    }

    // Create result structure
    Ok(CrudResult {
        operation: CrudOperation::Add,
        element_id: element.identifier.clone(),
        element_name: element.name.clone(),
        diffs,
        dry_run,
    })
}

/// Remove an element from the model
///
/// # Arguments
/// * `model_manager` - The model manager
/// * `element_id` - ID of the element to remove
/// * `git_root` - Git root directory
/// * `dry_run` - If true, don't write changes to disk
pub fn remove_element(
    model_manager: &mut ModelManager,
    element_id: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    // Get element info before removal
    let element = model_manager.graph_registry.nodes.get(element_id)
        .ok_or_else(|| ReqvireError::MissingElement(
            format!("Element not found: {}", element_id)
        ))?;
    let element_name = element.element.name.clone();

    // Track which files were modified before the operation
    let modified_before: Vec<String> = model_manager.graph_registry.modified_files
        .iter()
        .cloned()
        .collect();

    // Remove element using core business logic
    let _affected_files = model_manager.graph_registry.remove_element_with_cleanup(element_id)?;

    // Get list of newly modified files (sorted for deterministic output)
    let mut modified_files: Vec<String> = model_manager.graph_registry.modified_files
        .iter()
        .filter(|f| !modified_before.contains(f))
        .cloned()
        .collect();
    modified_files.sort();

    // Generate diffs for output
    let diffs = generate_crud_diffs(
        &model_manager.graph_registry,
        &modified_files,
        git_root,
    )?;

    // Flush changes if not dry-run
    if !dry_run {
        model_manager.graph_registry.flush_modified_files(git_root)?;
    }

    // Create result structure
    Ok(CrudResult {
        operation: CrudOperation::Remove,
        element_id: element_id.to_string(),
        element_name,
        diffs,
        dry_run,
    })
}

/// Move an element to a new location
///
/// # Arguments
/// * `model_manager` - The model manager
/// * `element_id` - ID of the element to move
/// * `target_file` - Target file path (relative to current working directory)
/// * `index` - Optional index for insertion
/// * `excluded_patterns` - Patterns to exclude from path validation
/// * `current_dir` - Current working directory (where command was invoked)
/// * `git_root` - Git root directory
/// * `dry_run` - If true, don't write changes to disk
pub fn move_element(
    model_manager: &mut ModelManager,
    element_id: &str,
    target_file: &str,
    index: Option<usize>,
    excluded_patterns: &GlobSet,
    current_dir: &Path,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    // Normalize target_file: convert from CWD-relative to git-root-relative
    use crate::utils;
    let absolute_target = current_dir.join(target_file);
    let target_file_normalized = utils::get_relative_path(&absolute_target)?
        .to_string_lossy()
        .to_string();

    // Get element info before move
    let element = model_manager.graph_registry.nodes.get(element_id)
        .ok_or_else(|| ReqvireError::MissingElement(
            format!("Element not found: {}", element_id)
        ))?;
    let element_name = element.element.name.clone();

    // Track which files were modified before the operation
    let modified_before: Vec<String> = model_manager.graph_registry.modified_files
        .iter()
        .cloned()
        .collect();

    // Move element using core business logic
    let (new_id, _affected_files) = model_manager.graph_registry.move_element_comprehensive(
        element_id,
        &target_file_normalized,
        index,
        excluded_patterns,
    )?;

    // Get list of newly modified files (sorted for deterministic output)
    let mut modified_files: Vec<String> = model_manager.graph_registry.modified_files
        .iter()
        .filter(|f| !modified_before.contains(f))
        .cloned()
        .collect();
    modified_files.sort();

    // Generate diffs for output
    let diffs = generate_crud_diffs(
        &model_manager.graph_registry,
        &modified_files,
        git_root,
    )?;

    // Flush changes if not dry-run
    if !dry_run {
        model_manager.graph_registry.flush_modified_files(git_root)?;
    }

    // Create result structure
    Ok(CrudResult {
        operation: CrudOperation::Move,
        element_id: new_id,
        element_name,
        diffs,
        dry_run,
    })
}

/// Rename an element
///
/// # Arguments
/// * `model_manager` - The model manager
/// * `element_id` - ID of the element to rename
/// * `new_name` - New name for the element
/// * `git_root` - Git root directory
/// * `dry_run` - If true, don't write changes to disk
pub fn rename_element(
    model_manager: &mut ModelManager,
    element_id: &str,
    new_name: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    // Get element info before rename
    let element = model_manager.graph_registry.nodes.get(element_id)
        .ok_or_else(|| ReqvireError::MissingElement(
            format!("Element not found: {}", element_id)
        ))?;
    let old_name = element.element.name.clone();

    // Track which files were modified before the operation
    let modified_before: Vec<String> = model_manager.graph_registry.modified_files
        .iter()
        .cloned()
        .collect();

    // Rename element using core business logic
    let new_id = model_manager.graph_registry.rename_element(
        element_id,
        new_name,
    )?;

    // Get list of newly modified files (sorted for deterministic output)
    let mut modified_files: Vec<String> = model_manager.graph_registry.modified_files
        .iter()
        .filter(|f| !modified_before.contains(f))
        .cloned()
        .collect();
    modified_files.sort();

    // Generate diffs for output
    let diffs = generate_crud_diffs(
        &model_manager.graph_registry,
        &modified_files,
        git_root,
    )?;

    // Flush changes if not dry-run
    if !dry_run {
        model_manager.graph_registry.flush_modified_files(git_root)?;
    }

    // Create result structure showing the rename
    Ok(CrudResult {
        operation: CrudOperation::Rename,
        element_id: new_id,
        element_name: format!("{} → {}", old_name, new_name),
        diffs,
        dry_run,
    })
}

/// Move entire file with all its elements to a new location
pub fn move_file(
    model_manager: &mut ModelManager,
    source_file: &str,
    target_file: &str,
    current_dir: &Path,
    git_root: &Path,
    dry_run: bool,
    squash: bool,
) -> Result<CrudResult, ReqvireError> {
    // Normalize file paths: convert from CWD-relative to git-root-relative
    use crate::utils;
    let absolute_source = current_dir.join(source_file);
    let source_file_normalized = utils::get_relative_path(&absolute_source)?
        .to_string_lossy()
        .to_string();

    let absolute_target = current_dir.join(target_file);
    let target_file_normalized = utils::get_relative_path(&absolute_target)?
        .to_string_lossy()
        .to_string();

    // Track which files were modified before the operation
    let modified_before: Vec<String> = model_manager.graph_registry.modified_files
        .iter()
        .cloned()
        .collect();

    // Move file using core business logic
    let identifier_mappings = model_manager.graph_registry.move_file(
        &source_file_normalized,
        &target_file_normalized,
        squash,
    )?;

    // Get list of newly modified files (sorted for deterministic output)
    let mut modified_files: Vec<String> = model_manager.graph_registry.modified_files
        .iter()
        .filter(|f| !modified_before.contains(f))
        .cloned()
        .collect();
    modified_files.sort();

    // Generate diffs for output
    let diffs = generate_crud_diffs(
        &model_manager.graph_registry,
        &modified_files,
        git_root,
    )?;

    // Flush changes if not dry-run
    if !dry_run {
        model_manager.graph_registry.flush_modified_files(git_root)?;

        // Delete the source file from disk
        let source_path = git_root.join(&source_file_normalized);
        if source_path.exists() {
            std::fs::remove_file(&source_path)
                .map_err(|e| ReqvireError::IoError(e))?;
        }
    }

    // Create summary of moved elements
    let element_count = identifier_mappings.len();
    let element_name = format!("{} element{} from {} → {}",
        element_count,
        if element_count == 1 { "" } else { "s" },
        source_file,
        target_file
    );

    Ok(CrudResult {
        operation: CrudOperation::Move,
        element_id: source_file_normalized.clone(),
        element_name,
        diffs,
        dry_run,
    })
}

/// Attach a file to an element by adding it to the Attachments subsection
///
/// # Arguments
/// * `model_manager` - The model manager
/// * `element_name` - Name of the element to attach to
/// * `attachment_path` - Path to the file to attach (git-root-relative)
/// * `git_root` - Git root directory
/// * `dry_run` - If true, don't write changes to disk
pub fn attach(
    model_manager: &mut ModelManager,
    element_name: &str,
    attachment_path: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    use std::fs;

    // Find the element by name
    let element = model_manager.graph_registry.get_element_by_name(element_name)
        .ok_or_else(|| ReqvireError::ElementNotFound(
            format!("Element '{}' not found", element_name)
        ))?;

    let element_id = element.identifier.clone();
    let file_path = element.file_path.clone();

    // Read current file content
    let absolute_file_path = git_root.join(&file_path);
    let content = fs::read_to_string(&absolute_file_path)
        .map_err(|e| ReqvireError::IoError(e))?;

    // Check if attachment already exists (idempotent)
    if element.attachments.iter().any(|a| a.target.as_str() == attachment_path) {
        // Already attached, return success without changes
        return Ok(CrudResult {
            operation: CrudOperation::Update,
            element_id: element_id.clone(),
            element_name: format!("Attachment already exists: {}", attachment_path),
            diffs: vec![],
            dry_run,
        });
    }

    // Find the element in the file and add/update Attachments subsection
    let new_content = add_attachment_to_element(&content, element_name, attachment_path)?;

    // Generate diff
    let diff = generate_file_diff(&file_path, &content, &new_content);

    // Write to file if not dry run
    if !dry_run {
        fs::write(&absolute_file_path, &new_content)
            .map_err(|e| ReqvireError::IoError(e))?;

        // Mark file as modified for re-parsing
        model_manager.graph_registry.modified_files.insert(file_path.clone());
    }

    Ok(CrudResult {
        operation: CrudOperation::Update,
        element_id,
        element_name: format!("Attached {} to {}", attachment_path, element_name),
        diffs: vec![diff],
        dry_run,
    })
}

/// Detach a file from an element by removing it from the Attachments subsection
pub fn detach(
    model_manager: &mut ModelManager,
    element_name: &str,
    attachment_path: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    use std::fs;

    // Find the element by name
    let element = model_manager.graph_registry.get_element_by_name(element_name)
        .ok_or_else(|| ReqvireError::ElementNotFound(
            format!("Element '{}' not found", element_name)
        ))?;

    let element_id = element.identifier.clone();
    let file_path = element.file_path.clone();

    // Read current file content
    let absolute_file_path = git_root.join(&file_path);
    let content = fs::read_to_string(&absolute_file_path)
        .map_err(|e| ReqvireError::IoError(e))?;

    // Remove attachment from element
    let new_content = remove_attachment_from_element(&content, element_name, attachment_path)?;

    // Generate diff
    let diff = generate_file_diff(&file_path, &content, &new_content);

    // Write to file if not dry run
    if !dry_run {
        fs::write(&absolute_file_path, &new_content)
            .map_err(|e| ReqvireError::IoError(e))?;

        // Mark file as modified for re-parsing
        model_manager.graph_registry.modified_files.insert(file_path.clone());
    }

    Ok(CrudResult {
        operation: CrudOperation::Update,
        element_id,
        element_name: format!("Detached {} from {}", attachment_path, element_name),
        diffs: vec![diff],
        dry_run,
    })
}

/// Move an attachment file and update all references across elements
pub fn mv_attachment(
    model_manager: &mut ModelManager,
    old_path: &str,
    new_path: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    use std::fs;

    // Find all elements with this attachment
    let affected_elements: Vec<_> = model_manager.graph_registry.nodes.values()
        .map(|node| &node.element)
        .filter(|elem| elem.attachments.iter().any(|a| a.target.as_str() == old_path))
        .map(|elem| (elem.identifier.clone(), elem.file_path.clone()))
        .collect();

    if affected_elements.is_empty() {
        return Err(ReqvireError::MissingAttachmentTarget(
            format!("No elements have attachment '{}'", old_path)
        ));
    }

    let mut all_diffs = vec![];
    let mut affected_files: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Update references in each affected file
    for (_, file_path) in &affected_elements {
        if affected_files.contains(file_path) {
            continue; // Already processed this file
        }
        affected_files.insert(file_path.clone());

        let absolute_file_path = git_root.join(file_path);
        let content = fs::read_to_string(&absolute_file_path)
            .map_err(|e| ReqvireError::IoError(e))?;

        // Replace old attachment path with new path
        let old_link = format!("[{}]({})", old_path, old_path);
        let new_link = format!("[{}]({})", new_path, new_path);
        let new_content = content.replace(&old_link, &new_link);

        if content != new_content {
            let diff = generate_file_diff(file_path, &content, &new_content);
            all_diffs.push(diff);

            if !dry_run {
                fs::write(&absolute_file_path, &new_content)
                    .map_err(|e| ReqvireError::IoError(e))?;

                model_manager.graph_registry.modified_files.insert(file_path.clone());
            }
        }
    }

    // Move the actual file
    if !dry_run {
        let old_abs = git_root.join(old_path);
        let new_abs = git_root.join(new_path);

        // Create parent directory if needed
        if let Some(parent) = new_abs.parent() {
            fs::create_dir_all(parent).map_err(|e| ReqvireError::IoError(e))?;
        }

        fs::rename(&old_abs, &new_abs).map_err(|e| ReqvireError::IoError(e))?;
    }

    Ok(CrudResult {
        operation: CrudOperation::Move,
        element_id: old_path.to_string(),
        element_name: format!("Moved attachment {} → {} (updated {} file(s))",
            old_path, new_path, affected_files.len()),
        diffs: all_diffs,
        dry_run,
    })
}

/// Remove an attachment file and detach from all elements
pub fn rm_attachment(
    model_manager: &mut ModelManager,
    attachment_path: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    use std::fs;

    // Find all elements with this attachment
    let affected_elements: Vec<_> = model_manager.graph_registry.nodes.values()
        .map(|node| &node.element)
        .filter(|elem| elem.attachments.iter().any(|a| a.target.as_str() == attachment_path))
        .map(|elem| elem.name.clone())
        .collect();

    let mut all_diffs = vec![];
    let mut affected_files: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Detach from each affected element
    for element_name in &affected_elements {
        let element = model_manager.graph_registry.get_element_by_name(element_name)
            .ok_or_else(|| ReqvireError::ElementNotFound(
                format!("Element '{}' not found", element_name)
            ))?;

        let file_path = element.file_path.clone();

        if affected_files.contains(&file_path) {
            continue; // Already processed this file
        }

        let absolute_file_path = git_root.join(&file_path);
        let content = fs::read_to_string(&absolute_file_path)
            .map_err(|e| ReqvireError::IoError(e))?;

        // Remove all occurrences of this attachment from the file
        let new_content = remove_attachment_from_file(&content, attachment_path)?;

        if content != new_content {
            let diff = generate_file_diff(&file_path, &content, &new_content);
            all_diffs.push(diff);
            affected_files.insert(file_path.clone());

            if !dry_run {
                fs::write(&absolute_file_path, &new_content)
                    .map_err(|e| ReqvireError::IoError(e))?;

                model_manager.graph_registry.modified_files.insert(file_path);
            }
        }
    }

    // Delete the actual file
    if !dry_run {
        let abs_path = git_root.join(attachment_path);
        if abs_path.exists() {
            fs::remove_file(&abs_path).map_err(|e| ReqvireError::IoError(e))?;
        }
    }

    Ok(CrudResult {
        operation: CrudOperation::Remove,
        element_id: attachment_path.to_string(),
        element_name: format!("Removed attachment {} (detached from {} element(s))",
            attachment_path, affected_elements.len()),
        diffs: all_diffs,
        dry_run,
    })
}

// Helper function to add attachment to element in markdown content
fn add_attachment_to_element(content: &str, element_name: &str, attachment_path: &str) -> Result<String, ReqvireError> {
    let mut result = String::new();
    let mut in_target_element = false;
    let mut inserted = false;
    let mut lines_iter = content.lines().peekable();

    let attachment_line = format!("* [{}]({})", attachment_path, attachment_path);

    while let Some(line) = lines_iter.next() {
        let trimmed = line.trim();

        // Check if we're entering the target element
        if trimmed.starts_with("### ") {
            let name = trimmed.trim_start_matches("### ").trim();
            in_target_element = name == element_name;
        }

        // Check for Attachments subsection
        if in_target_element && trimmed == "#### Attachments" {
            result.push_str(line);
            result.push('\n');

            // Add the new attachment after existing ones
            while let Some(next_line) = lines_iter.peek() {
                let next_trimmed = next_line.trim();
                if next_trimmed.starts_with("* ") || next_trimmed.starts_with("- ") {
                    result.push_str(lines_iter.next().unwrap());
                    result.push('\n');
                } else if next_trimmed.is_empty() {
                    result.push_str(lines_iter.next().unwrap());
                    result.push('\n');
                } else {
                    break;
                }
            }

            // Add our new attachment
            result.push_str(&attachment_line);
            result.push('\n');
            inserted = true;
            continue;
        }

        // Check for separator (end of element) - insert Attachments section if not found
        if in_target_element && !inserted && trimmed == "---" {
            // Need to add Attachments section before the separator
            result.push_str("\n#### Attachments\n");
            result.push_str(&attachment_line);
            result.push('\n');
            inserted = true;
        }

        result.push_str(line);
        result.push('\n');
    }

    if !inserted {
        return Err(ReqvireError::ElementNotFound(
            format!("Could not find element '{}' to add attachment", element_name)
        ));
    }

    Ok(result)
}

// Helper function to remove attachment from element in markdown content
fn remove_attachment_from_element(content: &str, element_name: &str, attachment_path: &str) -> Result<String, ReqvireError> {
    let mut result = String::new();
    let mut in_target_element = false;
    let mut in_attachments_section = false;
    let mut removed = false;
    let mut remaining_attachments_count = 0;

    let attachment_link = format!("[{}]({})", attachment_path, attachment_path);

    for line in content.lines() {
        let trimmed = line.trim();

        // Check if we're entering a new element
        if trimmed.starts_with("### ") {
            let name = trimmed.trim_start_matches("### ").trim();
            // Only track target element, don't reset counts when leaving
            if name == element_name {
                in_target_element = true;
            } else if in_target_element {
                // We're leaving the target element
                in_target_element = false;
            }
            in_attachments_section = false;
        }

        // Check for Attachments subsection
        if in_target_element && trimmed == "#### Attachments" {
            in_attachments_section = true;
        }

        // Check for end of Attachments section (another h4 header or element separator)
        if in_attachments_section && ((trimmed.starts_with("####") && trimmed != "#### Attachments") || trimmed == "---") {
            in_attachments_section = false;
        }

        // Skip the attachment line we want to remove
        if in_target_element && in_attachments_section {
            if (trimmed.starts_with("* ") || trimmed.starts_with("- ")) && trimmed.contains(&attachment_link) {
                removed = true;
                continue; // Skip this line
            }
            if trimmed.starts_with("* ") || trimmed.starts_with("- ") {
                remaining_attachments_count += 1;
            }
        }

        result.push_str(line);
        result.push('\n');
    }

    // If we removed the last attachment, clean up the empty Attachments section
    if removed && remaining_attachments_count == 0 {
        result = remove_empty_attachments_section(&result, element_name);
    }

    Ok(result)
}

// Helper function to remove attachment from all elements in a file
fn remove_attachment_from_file(content: &str, attachment_path: &str) -> Result<String, ReqvireError> {
    let mut result = String::new();
    let attachment_link = format!("[{}]({})", attachment_path, attachment_path);

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip attachment lines matching our path
        if (trimmed.starts_with("* ") || trimmed.starts_with("- ")) && trimmed.contains(&attachment_link) {
            continue; // Skip this line
        }

        result.push_str(line);
        result.push('\n');
    }

    // Clean up any empty Attachments sections
    result = remove_all_empty_attachments_sections(&result);

    Ok(result)
}

// Helper function to remove empty Attachments section for a specific element
fn remove_empty_attachments_section(content: &str, element_name: &str) -> String {
    let mut result = String::new();
    let mut in_target_element = false;
    let mut lines_iter = content.lines().peekable();

    while let Some(line) = lines_iter.next() {
        let trimmed = line.trim();

        // Check if we're entering the target element
        if trimmed.starts_with("### ") {
            let name = trimmed.trim_start_matches("### ").trim();
            in_target_element = name == element_name;
        }

        // Check for empty Attachments subsection to remove
        if in_target_element && trimmed == "#### Attachments" {
            // Look ahead to see if there are any attachment lines
            let mut has_attachments = false;
            let mut temp_lines = vec![];

            while let Some(next_line) = lines_iter.peek() {
                let next_trimmed = next_line.trim();
                if next_trimmed.is_empty() {
                    temp_lines.push(lines_iter.next().unwrap());
                } else if next_trimmed.starts_with("* ") || next_trimmed.starts_with("- ") {
                    has_attachments = true;
                    break;
                } else {
                    break;
                }
            }

            if has_attachments {
                // Keep the header and empty lines
                result.push_str(line);
                result.push('\n');
                for temp in temp_lines {
                    result.push_str(temp);
                    result.push('\n');
                }
            }
            // If no attachments, skip the header (and empty lines are already consumed)
            continue;
        }

        result.push_str(line);
        result.push('\n');
    }

    result
}

// Helper function to remove all empty Attachments sections
fn remove_all_empty_attachments_sections(content: &str) -> String {
    let mut result = String::new();
    let mut lines_iter = content.lines().peekable();

    while let Some(line) = lines_iter.next() {
        let trimmed = line.trim();

        // Check for Attachments subsection
        if trimmed == "#### Attachments" {
            // Look ahead to see if there are any attachment lines
            let mut has_attachments = false;
            let mut temp_lines = vec![];

            while let Some(next_line) = lines_iter.peek() {
                let next_trimmed = next_line.trim();
                if next_trimmed.is_empty() {
                    temp_lines.push(lines_iter.next().unwrap());
                } else if next_trimmed.starts_with("* ") || next_trimmed.starts_with("- ") {
                    has_attachments = true;
                    break;
                } else {
                    break;
                }
            }

            if has_attachments {
                // Keep the header and empty lines
                result.push_str(line);
                result.push('\n');
                for temp in temp_lines {
                    result.push_str(temp);
                    result.push('\n');
                }
            }
            // If no attachments, skip the header
            continue;
        }

        result.push_str(line);
        result.push('\n');
    }

    result
}
