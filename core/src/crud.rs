// CRUD module - high-level operations for element manipulation
// This module contains all business logic for Create, Read, Update, Delete operations
// CLI should only parse arguments and call these functions

use crate::error::ReqvireError;
use crate::model::ModelManager;
use crate::diff::{CrudResult, CrudOperation, generate_crud_diffs};
use globset::GlobSet;
use std::path::Path;

/// Add a new element to the model
///
/// # Arguments
/// * `model_manager` - The model manager
/// * `element_markdown` - The markdown content for the element
/// * `target_file` - Target file path (relative to git root)
/// * `target_section` - Target section name
/// * `index` - Optional index for insertion
/// * `excluded_patterns` - Patterns to exclude from path validation
/// * `git_root` - Git root directory
/// * `dry_run` - If true, don't write changes to disk
pub fn add_element(
    model_manager: &mut ModelManager,
    element_markdown: &str,
    target_file: &str,
    target_section: &str,
    index: Option<usize>,
    excluded_patterns: &GlobSet,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    // Track which files were modified before the operation
    let modified_before: Vec<String> = model_manager.graph_registry.modified_files
        .iter()
        .cloned()
        .collect();

    // Create element using core business logic
    let element = model_manager.graph_registry.create_element_from_string(
        element_markdown,
        target_file,
        target_section,
        index,
        excluded_patterns,
    )?;

    // Get list of newly modified files
    let modified_files: Vec<String> = model_manager.graph_registry.modified_files
        .iter()
        .filter(|f| !modified_before.contains(f))
        .cloned()
        .collect();

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

    // Get list of newly modified files
    let modified_files: Vec<String> = model_manager.graph_registry.modified_files
        .iter()
        .filter(|f| !modified_before.contains(f))
        .cloned()
        .collect();

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
/// * `target_file` - Target file path (relative to git root)
/// * `target_section` - Target section name
/// * `index` - Optional index for insertion
/// * `excluded_patterns` - Patterns to exclude from path validation
/// * `git_root` - Git root directory
/// * `dry_run` - If true, don't write changes to disk
pub fn move_element(
    model_manager: &mut ModelManager,
    element_id: &str,
    target_file: &str,
    target_section: &str,
    index: Option<usize>,
    excluded_patterns: &GlobSet,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
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
        target_file,
        target_section,
        index,
        excluded_patterns,
    )?;

    // Get list of newly modified files
    let modified_files: Vec<String> = model_manager.graph_registry.modified_files
        .iter()
        .filter(|f| !modified_before.contains(f))
        .cloned()
        .collect();

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
