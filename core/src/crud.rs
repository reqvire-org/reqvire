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
/// * `excluded_patterns` - Patterns to exclude from path validation
/// * `current_dir` - Current working directory (where command was invoked)
/// * `git_root` - Git root directory
/// * `dry_run` - If true, don't write changes to disk
pub fn add_element(
    model_manager: &mut ModelManager,
    element_markdown: &str,
    target_file: &str,
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
/// * `excluded_patterns` - Patterns to exclude from path validation
/// * `current_dir` - Current working directory (where command was invoked)
/// * `git_root` - Git root directory
/// * `dry_run` - If true, don't write changes to disk
pub fn move_element(
    model_manager: &mut ModelManager,
    element_id: &str,
    target_file: &str,
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
    use std::path::PathBuf;

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

    // Calculate file-relative path for the attachment link in markdown
    let file_dir = PathBuf::from(&file_path).parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_default();
    let attachment_path_buf = PathBuf::from(attachment_path);
    let relative_attachment_path = pathdiff::diff_paths(&attachment_path_buf, &file_dir)
        .unwrap_or_else(|| attachment_path_buf.clone());
    let relative_attachment_str = relative_attachment_path.to_string_lossy();

    // Find the element in the file and add/update Attachments subsection
    let new_content = add_attachment_to_element(&content, element_name, &relative_attachment_str)?;

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
    use std::path::PathBuf;

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

    // Calculate file-relative path for finding the attachment link in markdown
    let file_dir = PathBuf::from(&file_path).parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_default();
    let attachment_path_buf = PathBuf::from(attachment_path);
    let relative_attachment_path = pathdiff::diff_paths(&attachment_path_buf, &file_dir)
        .unwrap_or_else(|| attachment_path_buf.clone());
    let relative_attachment_str = relative_attachment_path.to_string_lossy();

    // Remove attachment from element
    let new_content = remove_attachment_from_element(&content, element_name, &relative_attachment_str)?;

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

/// Attach a Refinement element to another element by adding its identifier to the Attachments subsection
///
/// # Arguments
/// * `model_manager` - The model manager
/// * `element_name` - Name of the element to attach to
/// * `attachment_element_name` - Name of the Refinement element to attach
/// * `git_root` - Git root directory
/// * `dry_run` - If true, don't write changes to disk
pub fn attach_element(
    model_manager: &mut ModelManager,
    element_name: &str,
    attachment_element_name: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    use std::fs;

    // Find the target element by name (the element to attach TO)
    let target_element = model_manager.graph_registry.get_element_by_name(element_name)
        .ok_or_else(|| ReqvireError::ElementNotFound(
            format!("Element '{}' not found", element_name)
        ))?;

    let element_id = target_element.identifier.clone();
    let file_path = target_element.file_path.clone();

    // Find the attachment element by name (the Refinement element to attach)
    let attachment_element = model_manager.graph_registry.get_element_by_name(attachment_element_name)
        .ok_or_else(|| ReqvireError::ElementNotFound(
            format!("Attachment '{}' not found as file or element. Neither file exists nor element with this name was found in the model.", attachment_element_name)
        ))?;

    // Verify the attachment element is a Refinement type (constraint, behavior, specification)
    if !attachment_element.element_type.is_refinement() {
        return Err(ReqvireError::InvalidAttachmentTarget(
            format!("Element '{}' is not a Refinement type (constraint, behavior, specification). Only Refinement elements can be attached.", attachment_element_name)
        ));
    }

    let attachment_identifier = attachment_element.identifier.clone();
    let attachment_display_name = attachment_element.name.clone();

    // Check if already attached
    if target_element.attachments.iter().any(|a| a.target.as_str() == attachment_identifier) {
        return Ok(CrudResult {
            operation: CrudOperation::Update,
            element_id: element_id.clone(),
            element_name: format!("Attachment already exists: {}", attachment_element_name),
            diffs: vec![],
            dry_run,
        });
    }

    // Read current file content
    let absolute_file_path = git_root.join(&file_path);
    let content = fs::read_to_string(&absolute_file_path)
        .map_err(|e| ReqvireError::IoError(e))?;

    // Calculate relative identifier from target element's file to attachment element
    // If both elements are in the same file, use just #fragment format
    let attachment_file_path = attachment_element.file_path.clone();
    let relative_identifier = if file_path == attachment_file_path {
        // Same file - use just the fragment (like relations do)
        let (_path, fragment_opt) = crate::utils::extract_path_and_fragment(&attachment_identifier);
        let fragment = fragment_opt.unwrap_or(&attachment_identifier);
        format!("#{}", fragment)
    } else {
        // Different files - calculate relative path
        let target_file_path_buf = std::path::PathBuf::from(&file_path);
        let target_folder = target_file_path_buf.parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .to_path_buf();

        crate::utils::to_relative_identifier(
            &attachment_identifier,
            &target_folder,
            true
        ).unwrap_or_else(|_| attachment_identifier.clone())
    };

    // Add element attachment to file
    let new_content = add_element_attachment_to_element(&content, element_name, &attachment_display_name, &relative_identifier)?;

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
        element_name: format!("Attached element {} to {}", attachment_element_name, element_name),
        diffs: vec![diff],
        dry_run,
    })
}

/// Detach a Refinement element from another element by removing its identifier from the Attachments subsection
///
/// # Arguments
/// * `model_manager` - The model manager
/// * `element_name` - Name of the element to detach from
/// * `attachment_element_name` - Name of the Refinement element to detach
/// * `git_root` - Git root directory
/// * `dry_run` - If true, don't write changes to disk
pub fn detach_element(
    model_manager: &mut ModelManager,
    element_name: &str,
    attachment_element_name: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    use std::fs;

    // Find the target element by name
    let target_element = model_manager.graph_registry.get_element_by_name(element_name)
        .ok_or_else(|| ReqvireError::ElementNotFound(
            format!("Element '{}' not found", element_name)
        ))?;

    let element_id = target_element.identifier.clone();
    let file_path = target_element.file_path.clone();

    // Find the attachment element by name to get its identifier
    let attachment_element = model_manager.graph_registry.get_element_by_name(attachment_element_name)
        .ok_or_else(|| ReqvireError::ElementNotFound(
            format!("Attachment '{}' not found as file or element. Neither file exists nor element with this name was found in the model.", attachment_element_name)
        ))?;

    let attachment_identifier = attachment_element.identifier.clone();
    let attachment_display_name = attachment_element.name.clone();

    // Read current file content
    let absolute_file_path = git_root.join(&file_path);
    let content = fs::read_to_string(&absolute_file_path)
        .map_err(|e| ReqvireError::IoError(e))?;

    // Calculate relative identifier from target element's file to attachment element
    let target_file_path = std::path::PathBuf::from(&file_path);
    let target_folder = target_file_path.parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .to_path_buf();

    let relative_identifier = crate::utils::to_relative_identifier(
        &attachment_identifier,
        &target_folder,
        true
    ).unwrap_or_else(|_| attachment_identifier.clone());

    // Remove element attachment from file
    let new_content = remove_element_attachment_from_element(&content, element_name, &attachment_display_name, &relative_identifier)?;

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
        element_name: format!("Detached element {} from {}", attachment_element_name, element_name),
        diffs: vec![diff],
        dry_run,
    })
}

/// Move an asset file and update all references across elements (Attachments and Relations)
pub fn mv_asset(
    model_manager: &mut ModelManager,
    old_path: &str,
    new_path: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    use std::fs;
    use crate::relation::LinkType;
    use std::path::PathBuf;

    let old_path_buf = PathBuf::from(old_path);

    // Find all elements with this file as attachment OR as InternalPath relation target
    let mut affected_files: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut attachment_count = 0;
    let mut relation_count = 0;

    for node in model_manager.graph_registry.nodes.values() {
        let elem = &node.element;

        // Check attachments
        for attachment in &elem.attachments {
            if attachment.target.as_str() == old_path {
                affected_files.insert(elem.file_path.clone());
                attachment_count += 1;
            }
        }

        // Check relations with InternalPath
        for relation in &elem.relations {
            if let LinkType::InternalPath(ref path) = relation.target.link {
                if path.to_string_lossy() == old_path {
                    affected_files.insert(elem.file_path.clone());
                    relation_count += 1;
                }
            }
        }
    }

    if affected_files.is_empty() {
        return Err(ReqvireError::MissingAttachmentTarget(
            format!("No elements reference file '{}'", old_path)
        ));
    }

    let mut all_diffs = vec![];

    // Update references in each affected file
    for file_path in &affected_files {
        let absolute_file_path = git_root.join(file_path);
        let content = fs::read_to_string(&absolute_file_path)
            .map_err(|e| ReqvireError::IoError(e))?;

        let mut new_content = content.clone();

        // Paths in markdown are file-relative, but stored in registry as reqvire-root-relative
        // Calculate the file-relative paths that appear in the markdown
        let file_dir = PathBuf::from(file_path).parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_default();

        // Calculate old and new file-relative paths
        let old_relative = pathdiff::diff_paths(&old_path_buf, &file_dir)
            .unwrap_or_else(|| old_path_buf.clone());
        let new_relative = pathdiff::diff_paths(new_path, &file_dir)
            .unwrap_or_else(|| PathBuf::from(new_path));

        let old_relative_str = old_relative.to_string_lossy();
        let new_relative_str = new_relative.to_string_lossy();

        // Replace attachment links: [path](path)
        let old_link = format!("[{}]({})", old_relative_str, old_relative_str);
        let new_link = format!("[{}]({})", new_relative_str, new_relative_str);
        new_content = new_content.replace(&old_link, &new_link);

        // Replace relation links: [display](path) where display may differ from path
        // We need to match any [text](old_path) pattern
        let old_link_pattern = format!("]({})", old_relative_str);
        let new_link_pattern = format!("]({})", new_relative_str);
        new_content = new_content.replace(&old_link_pattern, &new_link_pattern);

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
        element_name: format!("Moved {} → {} ({} attachment(s), {} relation(s) in {} file(s))",
            old_path, new_path, attachment_count, relation_count, affected_files.len()),
        diffs: all_diffs,
        dry_run,
    })
}

/// Remove an asset file and remove all references from elements (Attachments and Relations)
pub fn rm_asset(
    model_manager: &mut ModelManager,
    file_path_arg: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    use std::fs;
    use crate::relation::LinkType;
    use std::path::PathBuf;

    // Find all elements with this file as attachment OR as InternalPath relation target
    let mut affected_files: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut attachment_count = 0;
    let mut relation_count = 0;

    for node in model_manager.graph_registry.nodes.values() {
        let elem = &node.element;

        // Check attachments
        for attachment in &elem.attachments {
            if attachment.target.as_str() == file_path_arg {
                affected_files.insert(elem.file_path.clone());
                attachment_count += 1;
            }
        }

        // Check relations with InternalPath
        for relation in &elem.relations {
            if let LinkType::InternalPath(ref path) = relation.target.link {
                if path.to_string_lossy() == file_path_arg {
                    affected_files.insert(elem.file_path.clone());
                    relation_count += 1;
                }
            }
        }
    }

    let mut all_diffs = vec![];
    let file_path_buf = PathBuf::from(file_path_arg);

    // Remove references from each affected file
    for spec_file_path in &affected_files {
        let absolute_file_path = git_root.join(spec_file_path);
        let content = fs::read_to_string(&absolute_file_path)
            .map_err(|e| ReqvireError::IoError(e))?;

        // Paths in markdown are file-relative, calculate the relative path from this file
        let file_dir = PathBuf::from(spec_file_path).parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_default();
        let relative_path = pathdiff::diff_paths(&file_path_buf, &file_dir)
            .unwrap_or_else(|| file_path_buf.clone());
        let relative_path_str = relative_path.to_string_lossy();

        // Remove attachments
        let mut new_content = remove_attachment_from_file(&content, &relative_path_str)?;

        // Remove InternalPath relations
        new_content = remove_relation_with_path(&new_content, &relative_path_str)?;

        if content != new_content {
            let diff = generate_file_diff(spec_file_path, &content, &new_content);
            all_diffs.push(diff);

            if !dry_run {
                fs::write(&absolute_file_path, &new_content)
                    .map_err(|e| ReqvireError::IoError(e))?;

                model_manager.graph_registry.modified_files.insert(spec_file_path.clone());
            }
        }
    }

    // Delete the actual file
    if !dry_run {
        let abs_path = git_root.join(file_path_arg);
        if abs_path.exists() {
            fs::remove_file(&abs_path).map_err(|e| ReqvireError::IoError(e))?;
        }
    }

    Ok(CrudResult {
        operation: CrudOperation::Remove,
        element_id: file_path_arg.to_string(),
        element_name: format!("Removed {} ({} attachment(s), {} relation(s) from {} file(s))",
            file_path_arg, attachment_count, relation_count, affected_files.len()),
        diffs: all_diffs,
        dry_run,
    })
}

/// Helper function to remove a relation line containing a specific path from file content
fn remove_relation_with_path(content: &str, path: &str) -> Result<String, ReqvireError> {
    let mut result = String::new();
    let mut in_relations_section = false;
    let mut relations_section_empty = true;
    let mut pending_relations_header: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim();

        // Detect Relations section start
        if trimmed == "#### Relations" {
            in_relations_section = true;
            relations_section_empty = true;
            pending_relations_header = Some(line.to_string());
            continue;
        }

        // Detect section end
        if in_relations_section && (trimmed.starts_with("####") || trimmed == "---") {
            // Output the Relations header only if section is not empty
            if !relations_section_empty {
                if let Some(header) = pending_relations_header.take() {
                    result.push_str(&header);
                    result.push('\n');
                }
            }
            in_relations_section = false;
            pending_relations_header = None;
        }

        if in_relations_section {
            // Check if this line contains a relation with the target path
            let link_pattern = format!("]({})", path);
            if trimmed.starts_with("*") && trimmed.contains(&link_pattern) {
                // Skip this line (remove the relation)
                continue;
            } else if trimmed.starts_with("*") {
                // This is a valid relation line, section is not empty
                relations_section_empty = false;
                // Output header if we haven't yet
                if let Some(header) = pending_relations_header.take() {
                    result.push_str(&header);
                    result.push('\n');
                }
            }
        }

        result.push_str(line);
        result.push('\n');
    }

    Ok(result)
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

// Helper function to add element attachment (with display name) to element in markdown content
fn add_element_attachment_to_element(content: &str, element_name: &str, display_name: &str, identifier: &str) -> Result<String, ReqvireError> {
    let mut result = String::new();
    let mut in_target_element = false;
    let mut inserted = false;
    let mut lines_iter = content.lines().peekable();

    // Format: * [Display Name](#identifier) or * [Display Name](file.md#identifier)
    let attachment_line = format!("* [{}]({})", display_name, identifier);

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

// Helper function to remove element attachment from element in markdown content
fn remove_element_attachment_from_element(content: &str, element_name: &str, display_name: &str, identifier: &str) -> Result<String, ReqvireError> {
    let mut result = String::new();
    let mut in_target_element = false;
    let mut in_attachments_section = false;
    let mut removed = false;
    let mut remaining_attachments_count = 0;

    // Match by either identifier or display name in the link
    let attachment_link_by_id = format!("]({})", identifier);
    let attachment_link_full = format!("[{}]({})", display_name, identifier);

    for line in content.lines() {
        let trimmed = line.trim();

        // Check if we're entering a new element
        if trimmed.starts_with("### ") {
            let name = trimmed.trim_start_matches("### ").trim();
            if name == element_name {
                in_target_element = true;
            } else if in_target_element {
                in_target_element = false;
            }
            in_attachments_section = false;
        }

        // Check for Attachments subsection
        if in_target_element && trimmed == "#### Attachments" {
            in_attachments_section = true;
        }

        // Check for end of Attachments section
        if in_attachments_section && ((trimmed.starts_with("####") && trimmed != "#### Attachments") || trimmed == "---") {
            in_attachments_section = false;
        }

        // Skip the attachment line we want to remove
        if in_target_element && in_attachments_section {
            if (trimmed.starts_with("* ") || trimmed.starts_with("- ")) &&
               (trimmed.contains(&attachment_link_by_id) || trimmed.contains(&attachment_link_full)) {
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

/// Link a relation between two elements
///
/// # Arguments
/// * `model_manager` - The model manager
/// * `source` - Source element name or file path (auto-detected)
/// * `relation_type` - The relation type (derivedFrom, derive, verifiedBy, verify, satisfiedBy, satisfy, trace)
/// * `target_name` - Target element name
/// * `git_root` - Git root directory
/// * `dry_run` - If true, don't write changes to disk
pub fn link(
    model_manager: &mut ModelManager,
    source: &str,
    relation_type: &str,
    target_name: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    use std::fs;
    use crate::relation::RELATION_TYPES;

    // Validate relation type
    if !RELATION_TYPES.contains_key(relation_type) {
        return Err(ReqvireError::UnsupportedRelationType(
            format!("Invalid relation type '{}'. Supported types: derivedFrom, derive, verifiedBy, verify, satisfiedBy, satisfy, trace", relation_type)
        ));
    }

    // Resolve source element by name
    let source_element = model_manager.graph_registry.get_element_by_name(source)
        .ok_or_else(|| ReqvireError::ElementNotFound(
            format!("Source element '{}' not found", source)
        ))?;

    let source_id = source_element.identifier.clone();
    let source_name = source_element.name.clone();
    let source_file_path = source_element.file_path.clone();

    // Resolve target element by name
    let target_element = model_manager.graph_registry.get_element_by_name(target_name)
        .ok_or_else(|| ReqvireError::ElementNotFound(
            format!("Target element '{}' not found", target_name)
        ))?;

    let target_id = target_element.identifier.clone();
    let target_display_name = target_element.name.clone();
    let target_file_path = target_element.file_path.clone();

    // Check if relation already exists (idempotent) - only check user_created relations
    let relation_exists = source_element.relations.iter().any(|r| {
        r.user_created && r.relation_type.name == relation_type && r.target.link.as_str() == target_id
    });

    if relation_exists {
        return Ok(CrudResult {
            operation: CrudOperation::Update,
            element_id: source_id.clone(),
            element_name: format!("Relation already exists: {} {} {}", source_name, relation_type, target_name),
            diffs: vec![],
            dry_run,
        });
    }

    // Read current file content
    let absolute_file_path = git_root.join(&source_file_path);
    let content = fs::read_to_string(&absolute_file_path)
        .map_err(|e| ReqvireError::IoError(e))?;

    // Calculate relative identifier from source element's file to target element
    let source_file_path_buf = std::path::PathBuf::from(&source_file_path);
    let source_folder = source_file_path_buf.parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .to_path_buf();

    let relation_target = if source_file_path == target_file_path {
        // Same file - use just the fragment
        let (_path, fragment_opt) = crate::utils::extract_path_and_fragment(&target_id);
        let fragment = fragment_opt.unwrap_or(&target_id);
        format!("#{}", fragment)
    } else {
        // Different files - calculate relative path
        crate::utils::to_relative_identifier(
            &target_id,
            &source_folder,
            true
        ).unwrap_or_else(|_| target_id.clone())
    };

    // Add relation to element
    let new_content = add_relation_to_element(&content, &source_name, relation_type, &target_display_name, &relation_target)?;

    // Generate diff
    let diff = generate_file_diff(&source_file_path, &content, &new_content);

    // Write to file if not dry run
    if !dry_run {
        fs::write(&absolute_file_path, &new_content)
            .map_err(|e| ReqvireError::IoError(e))?;

        // Mark file as modified for re-parsing
        model_manager.graph_registry.modified_files.insert(source_file_path.clone());
    }

    Ok(CrudResult {
        operation: CrudOperation::Update,
        element_id: source_id,
        element_name: format!("Linked {} {} {}", source_name, relation_type, target_name),
        diffs: vec![diff],
        dry_run,
    })
}

/// Unlink a relation between two elements
///
/// # Arguments
/// * `model_manager` - The model manager
/// * `source` - Source element name or file path (auto-detected)
/// * `relation_type` - The relation type (derivedFrom, derive, verifiedBy, verify, satisfiedBy, satisfy, trace)
/// * `target_name` - Target element name
/// * `git_root` - Git root directory
/// * `dry_run` - If true, don't write changes to disk
pub fn unlink(
    model_manager: &mut ModelManager,
    source: &str,
    relation_type: &str,
    target_name: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    use std::fs;
    use crate::relation::RELATION_TYPES;

    // Validate relation type
    if !RELATION_TYPES.contains_key(relation_type) {
        return Err(ReqvireError::UnsupportedRelationType(
            format!("Invalid relation type '{}'. Supported types: derivedFrom, derive, verifiedBy, verify, satisfiedBy, satisfy, trace", relation_type)
        ));
    }

    // Resolve source element by name
    let source_element = model_manager.graph_registry.get_element_by_name(source)
        .ok_or_else(|| ReqvireError::ElementNotFound(
            format!("Source element '{}' not found", source)
        ))?;

    let source_id = source_element.identifier.clone();
    let source_name = source_element.name.clone();
    let source_file_path = source_element.file_path.clone();

    // Resolve target element by name
    let target_element = model_manager.graph_registry.get_element_by_name(target_name)
        .ok_or_else(|| ReqvireError::ElementNotFound(
            format!("Target element '{}' not found", target_name)
        ))?;

    let target_id = target_element.identifier.clone();
    let target_display_name = target_element.name.clone();

    // Check if relation exists - only check user_created relations
    let relation_exists = source_element.relations.iter().any(|r| {
        r.user_created && r.relation_type.name == relation_type && r.target.link.as_str() == target_id
    });

    if !relation_exists {
        return Err(ReqvireError::RelationError(
            format!("Relation '{}' from '{}' to '{}' does not exist", relation_type, source_name, target_name)
        ));
    }

    // Read current file content
    let absolute_file_path = git_root.join(&source_file_path);
    let content = fs::read_to_string(&absolute_file_path)
        .map_err(|e| ReqvireError::IoError(e))?;

    // Remove relation from element
    let new_content = remove_relation_from_element(&content, &source_name, relation_type, &target_display_name)?;

    // Generate diff
    let diff = generate_file_diff(&source_file_path, &content, &new_content);

    // Write to file if not dry run
    if !dry_run {
        fs::write(&absolute_file_path, &new_content)
            .map_err(|e| ReqvireError::IoError(e))?;

        // Mark file as modified for re-parsing
        model_manager.graph_registry.modified_files.insert(source_file_path.clone());
    }

    Ok(CrudResult {
        operation: CrudOperation::Update,
        element_id: source_id,
        element_name: format!("Unlinked {} {} {}", source_name, relation_type, target_name),
        diffs: vec![diff],
        dry_run,
    })
}

/// Helper function to add a relation to an element in markdown content
fn add_relation_to_element(content: &str, element_name: &str, relation_type: &str, target_name: &str, target_path: &str) -> Result<String, ReqvireError> {
    let mut result = String::new();
    let mut in_target_element = false;
    let mut inserted = false;
    let mut lines_iter = content.lines().peekable();

    let relation_line = format!("  * {}: [{}]({})", relation_type, target_name, target_path);

    while let Some(line) = lines_iter.next() {
        let trimmed = line.trim();

        // Check if we're entering the target element
        if trimmed.starts_with("### ") {
            let name = trimmed.trim_start_matches("### ").trim();
            in_target_element = name == element_name;
        }

        // Check for Relations subsection
        if in_target_element && trimmed == "#### Relations" {
            result.push_str(line);
            result.push('\n');

            // Add the new relation after existing ones
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

            // Add our new relation
            result.push_str(&relation_line);
            result.push('\n');
            inserted = true;
            continue;
        }

        // Check for separator (end of element) - insert Relations section if not found
        if in_target_element && !inserted && trimmed == "---" {
            // Need to add Relations section before the separator
            result.push_str("\n#### Relations\n");
            result.push_str(&relation_line);
            result.push('\n');
            inserted = true;
        }

        result.push_str(line);
        result.push('\n');
    }

    if !inserted {
        return Err(ReqvireError::ElementNotFound(
            format!("Could not find element '{}' to add relation", element_name)
        ));
    }

    Ok(result)
}

/// Helper function to remove a relation from an element in markdown content
fn remove_relation_from_element(content: &str, element_name: &str, relation_type: &str, target_name: &str) -> Result<String, ReqvireError> {
    let mut result = String::new();
    let mut in_target_element = false;
    let mut in_relations_section = false;
    let mut removed = false;
    let mut remaining_relations_count = 0;

    // Match pattern like: * derivedFrom: [Target Name](path)
    let relation_pattern = format!("{}: [{}]", relation_type, target_name);

    for line in content.lines() {
        let trimmed = line.trim();

        // Check if we're entering a new element
        if trimmed.starts_with("### ") {
            let name = trimmed.trim_start_matches("### ").trim();
            if name == element_name {
                in_target_element = true;
            } else if in_target_element {
                in_target_element = false;
            }
            in_relations_section = false;
        }

        // Check for Relations subsection
        if in_target_element && trimmed == "#### Relations" {
            in_relations_section = true;
        }

        // Check for end of Relations section (another h4 header or element separator)
        if in_relations_section && ((trimmed.starts_with("####") && trimmed != "#### Relations") || trimmed == "---") {
            in_relations_section = false;
        }

        // Skip the relation line we want to remove
        if in_target_element && in_relations_section {
            if (trimmed.starts_with("* ") || trimmed.starts_with("- ")) && trimmed.contains(&relation_pattern) {
                removed = true;
                continue; // Skip this line
            }
            if trimmed.starts_with("* ") || trimmed.starts_with("- ") {
                remaining_relations_count += 1;
            }
        }

        result.push_str(line);
        result.push('\n');
    }

    if !removed {
        return Err(ReqvireError::RelationError(
            format!("Could not find relation '{}' to '{}' in element '{}'", relation_type, target_name, element_name)
        ));
    }

    // If we removed the last relation, clean up the empty Relations section
    if remaining_relations_count == 0 {
        result = remove_empty_relations_section(&result, element_name);
    }

    Ok(result)
}

/// Helper function to remove empty Relations section from markdown content
fn remove_empty_relations_section(content: &str, element_name: &str) -> String {
    let mut result = String::new();
    let mut in_target_element = false;
    let mut lines_iter = content.lines().peekable();

    while let Some(line) = lines_iter.next() {
        let trimmed = line.trim();

        // Check if we're entering a new element
        if trimmed.starts_with("### ") {
            let name = trimmed.trim_start_matches("### ").trim();
            in_target_element = name == element_name;
        }

        // Check for Relations subsection header
        if in_target_element && trimmed == "#### Relations" {
            // Check if there are any relations following
            let mut has_relations = false;
            let mut temp_lines: Vec<&str> = Vec::new();

            while let Some(next_line) = lines_iter.peek() {
                let next_trimmed = next_line.trim();
                if next_trimmed.is_empty() {
                    temp_lines.push(lines_iter.next().unwrap());
                } else if next_trimmed.starts_with("* ") || next_trimmed.starts_with("- ") {
                    has_relations = true;
                    break;
                } else {
                    break;
                }
            }

            if has_relations {
                // Keep the header and empty lines
                result.push_str(line);
                result.push('\n');
                for temp in temp_lines {
                    result.push_str(temp);
                    result.push('\n');
                }
            }
            // If no relations, skip the header
            continue;
        }

        result.push_str(line);
        result.push('\n');
    }

    result
}
