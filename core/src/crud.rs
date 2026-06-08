// CRUD module - high-level operations for element manipulation
// This module contains all business logic for Create, Read, Update, Delete operations
// CLI should only parse arguments and call these functions

use crate::diff::{generate_crud_diffs, generate_file_diff, CrudOperation, CrudResult, FileDiff};
use crate::element::{Attachment, AttachmentTarget};
use crate::error::ReqvireError;
use crate::model::ModelManager;
use globset::GlobSet;
use std::collections::HashSet;
use std::path::Path;

fn snapshot_modified_files(model_manager: &ModelManager) -> HashSet<String> {
    model_manager
        .graph_registry
        .modified_files
        .iter()
        .cloned()
        .collect()
}

fn collect_new_modified_files(
    model_manager: &ModelManager,
    before: &HashSet<String>,
) -> Vec<String> {
    let mut modified_files: Vec<String> = model_manager
        .graph_registry
        .modified_files
        .iter()
        .filter(|f| !before.contains(*f))
        .cloned()
        .collect();
    modified_files.sort();
    modified_files
}

fn finalize_crud_operation(
    model_manager: &mut ModelManager,
    modified_before: &HashSet<String>,
    git_root: &Path,
    dry_run: bool,
    removed_declaration_source: Option<&str>,
) -> Result<Vec<FileDiff>, ReqvireError> {
    validate_semantic_contracts_after_mutation(model_manager, removed_declaration_source)?;

    let modified_files = collect_new_modified_files(model_manager, modified_before);
    let diffs = generate_crud_diffs(&model_manager.graph_registry, &modified_files, git_root)?;

    if !dry_run {
        model_manager
            .graph_registry
            .flush_modified_files(git_root)?;
    }

    Ok(diffs)
}

fn validate_semantic_contracts_after_mutation(
    model_manager: &ModelManager,
    removed_declaration_source: Option<&str>,
) -> Result<(), ReqvireError> {
    let semantic_errors = if let Some(source) = removed_declaration_source {
        model_manager
            .graph_registry
            .validate_semantic_contracts_after_removal(source)?
    } else {
        model_manager
            .graph_registry
            .validate_semantic_contracts_in_memory()?
    };

    if semantic_errors.is_empty() {
        Ok(())
    } else {
        Err(ReqvireError::ValidationError(semantic_errors))
    }
}

fn validate_semantic_contracts_after_attachment_candidate(
    model_manager: &ModelManager,
    element_id: &str,
    attachment_identifier: &str,
    attach: bool,
) -> Result<(), ReqvireError> {
    let mut candidate = model_manager.graph_registry.clone();
    let Some(node) = candidate.nodes.get_mut(element_id) else {
        return Err(ReqvireError::ElementNotFound(format!(
            "Element '{}' not found",
            element_id
        )));
    };

    if attach {
        if !node
            .element
            .attachments
            .iter()
            .any(|attachment| attachment.target.as_str() == attachment_identifier)
        {
            node.element.attachments.push(Attachment {
                target: AttachmentTarget::ElementIdentifier(attachment_identifier.to_string()),
                content_hash: None,
            });
        }
    } else {
        node.element
            .attachments
            .retain(|attachment| attachment.target.as_str() != attachment_identifier);
    }

    let semantic_errors = candidate.validate_semantic_contracts_in_memory()?;
    if semantic_errors.is_empty() {
        Ok(())
    } else {
        Err(ReqvireError::ValidationError(semantic_errors))
    }
}

fn enforce_single_root_after_mutation(model_manager: &ModelManager) -> Result<(), ReqvireError> {
    let ownership_errors = model_manager
        .graph_registry
        .validate_single_root_hierarchy_ownership_in_memory()?;
    if ownership_errors.is_empty() {
        Ok(())
    } else {
        let mut details: Vec<String> = ownership_errors.iter().map(|e| e.to_string()).collect();
        details.sort();
        let message = details
            .first()
            .cloned()
            .unwrap_or_else(|| "Single-root hierarchy ownership violation detected".to_string());
        Err(ReqvireError::InvalidOperation(format!(
            "Single-root hierarchy ownership violation: {}",
            message
        )))
    }
}

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
/// * `override_existing` - If true, replace existing element with same name
#[allow(clippy::too_many_arguments)]
pub fn add_element(
    model_manager: &mut ModelManager,
    element_markdown: &str,
    target_file: &str,
    excluded_patterns: &GlobSet,
    current_dir: &Path,
    git_root: &Path,
    dry_run: bool,
    override_existing: bool,
) -> Result<CrudResult, ReqvireError> {
    // Normalize target_file: convert from CWD-relative to git-root-relative
    use crate::utils;
    let absolute_target = current_dir.join(target_file);
    let target_file_normalized = utils::get_relative_path(&absolute_target)?
        .to_string_lossy()
        .to_string();

    // Track which files were modified before the operation
    // IMPORTANT: Must track BEFORE any modifications (including override removal)
    let modified_before = snapshot_modified_files(model_manager);
    let registry_snapshot = model_manager.graph_registry.clone();
    let mut removed_declaration_source = None;

    // If override is requested, extract element name and remove existing element first
    if override_existing {
        // Extract element name from markdown (looks for ### Element Name pattern)
        let element_name = extract_element_name_from_markdown(element_markdown)?;

        // Check if element exists and remove it
        // Validation is handled by remove_element_with_cleanup
        if let Some(existing_element) = model_manager
            .graph_registry
            .get_element_by_name(&element_name)
        {
            let existing_id = existing_element.identifier.clone();
            model_manager
                .graph_registry
                .remove_element_with_cleanup(&existing_id)?;
            removed_declaration_source = Some(existing_id);
        }
    }

    // Create element using core business logic
    let element = model_manager.graph_registry.create_element_from_string(
        element_markdown,
        &target_file_normalized,
        excluded_patterns,
    )?;

    let diffs = match finalize_crud_operation(
        model_manager,
        &modified_before,
        git_root,
        dry_run,
        removed_declaration_source.as_deref(),
    ) {
        Ok(diffs) => diffs,
        Err(err) => {
            model_manager.graph_registry = registry_snapshot;
            return Err(err);
        }
    };

    // Create result structure
    let operation = if override_existing {
        CrudOperation::Update
    } else {
        CrudOperation::Add
    };

    Ok(CrudResult {
        operation,
        element_id: element.identifier.clone(),
        element_name: element.name.clone(),
        diffs,
        dry_run,
    })
}

/// Extract element name from markdown content
/// Looks for pattern: ### Element Name
fn extract_element_name_from_markdown(markdown: &str) -> Result<String, ReqvireError> {
    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("### ") && !trimmed.starts_with("#### ") {
            let name = trimmed.trim_start_matches("### ").trim();
            if !name.is_empty() {
                return Ok(name.to_string());
            }
        }
    }
    Err(ReqvireError::InvalidMarkdownStructure(
        "Could not find element name in markdown. Expected '### Element Name' pattern.".to_string(),
    ))
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
    // Get element name before removal (needed for result structure)
    let element_name = model_manager
        .graph_registry
        .nodes
        .get(element_id)
        .ok_or_else(|| ReqvireError::MissingElement(format!("Element not found: {}", element_id)))?
        .element
        .name
        .clone();

    // Track which files were modified before the operation
    let modified_before = snapshot_modified_files(model_manager);
    let registry_snapshot = model_manager.graph_registry.clone();

    // Remove element using core business logic (includes orphan validation)
    let _affected_files = model_manager
        .graph_registry
        .remove_element_with_cleanup(element_id)?;

    let diffs = match finalize_crud_operation(
        model_manager,
        &modified_before,
        git_root,
        dry_run,
        Some(element_id),
    ) {
        Ok(diffs) => diffs,
        Err(err) => {
            model_manager.graph_registry = registry_snapshot;
            return Err(err);
        }
    };

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
    let element = model_manager
        .graph_registry
        .nodes
        .get(element_id)
        .ok_or_else(|| {
            ReqvireError::MissingElement(format!("Element not found: {}", element_id))
        })?;
    let element_name = element.element.name.clone();

    // Track which files were modified before the operation
    let modified_before = snapshot_modified_files(model_manager);

    let registry_snapshot = model_manager.graph_registry.clone();

    // Move element using core business logic
    let (new_id, _affected_files) = model_manager.graph_registry.move_element_comprehensive(
        element_id,
        &target_file_normalized,
        excluded_patterns,
    )?;

    if let Err(err) = enforce_single_root_after_mutation(model_manager) {
        model_manager.graph_registry = registry_snapshot;
        return Err(err);
    }

    let diffs =
        match finalize_crud_operation(model_manager, &modified_before, git_root, dry_run, None) {
            Ok(diffs) => diffs,
            Err(err) => {
                model_manager.graph_registry = registry_snapshot;
                return Err(err);
            }
        };

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
    let element = model_manager
        .graph_registry
        .nodes
        .get(element_id)
        .ok_or_else(|| {
            ReqvireError::MissingElement(format!("Element not found: {}", element_id))
        })?;
    let old_name = element.element.name.clone();

    // Track which files were modified before the operation
    let modified_before = snapshot_modified_files(model_manager);
    let registry_snapshot = model_manager.graph_registry.clone();

    // Rename element using core business logic
    let new_id = model_manager
        .graph_registry
        .rename_element(element_id, new_name)?;

    let diffs =
        match finalize_crud_operation(model_manager, &modified_before, git_root, dry_run, None) {
            Ok(diffs) => diffs,
            Err(err) => {
                model_manager.graph_registry = registry_snapshot;
                return Err(err);
            }
        };

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
    let modified_before = snapshot_modified_files(model_manager);
    let registry_snapshot = model_manager.graph_registry.clone();

    // Move file using core business logic
    let identifier_mappings = model_manager.graph_registry.move_file(
        &source_file_normalized,
        &target_file_normalized,
        squash,
    )?;

    let diffs =
        match finalize_crud_operation(model_manager, &modified_before, git_root, dry_run, None) {
            Ok(diffs) => diffs,
            Err(err) => {
                model_manager.graph_registry = registry_snapshot;
                return Err(err);
            }
        };

    if !dry_run {
        // Delete the source file from disk
        let source_path = git_root.join(&source_file_normalized);
        if source_path.exists() {
            std::fs::remove_file(&source_path).map_err(ReqvireError::IoError)?;
        }
    }

    // Create summary of moved elements
    let element_count = identifier_mappings.len();
    let element_name = format!(
        "{} element{} from {} → {}",
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
    let element = model_manager
        .graph_registry
        .get_element_by_name(element_name)
        .ok_or_else(|| {
            ReqvireError::ElementNotFound(format!("Element '{}' not found", element_name))
        })?;

    let element_id = element.identifier.clone();
    let file_path = element.file_path.clone();

    // Read current file content
    let absolute_file_path = git_root.join(&file_path);
    let content = fs::read_to_string(&absolute_file_path).map_err(ReqvireError::IoError)?;

    // Check if attachment already exists - return error
    if element
        .attachments
        .iter()
        .any(|a| a.target.as_str() == attachment_path)
    {
        return Err(ReqvireError::ElementError(format!(
            "Attachment '{}' already exists on '{}'",
            attachment_path, element_name
        )));
    }

    // Check for cross-section duplicate: target already exists in Relations
    let in_relations = element
        .relations
        .iter()
        .any(|r| r.target.link.as_str() == attachment_path);

    if in_relations {
        return Err(ReqvireError::CrossSectionDuplicate(format!(
            "Target '{}' already exists in Relations of '{}'. Cannot add to Attachments.",
            attachment_path, element_name
        )));
    }

    // Calculate file-relative path for the attachment link in markdown
    let file_dir = crate::utils::get_parent_dir(&file_path);
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
        fs::write(&absolute_file_path, &new_content).map_err(ReqvireError::IoError)?;

        // Mark file as modified for re-parsing
        model_manager
            .graph_registry
            .modified_files
            .insert(file_path.clone());
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
    let element = model_manager
        .graph_registry
        .get_element_by_name(element_name)
        .ok_or_else(|| {
            ReqvireError::ElementNotFound(format!("Element '{}' not found", element_name))
        })?;

    let element_id = element.identifier.clone();
    let file_path = element.file_path.clone();

    // Read current file content
    let absolute_file_path = git_root.join(&file_path);
    let content = fs::read_to_string(&absolute_file_path).map_err(ReqvireError::IoError)?;

    // Calculate file-relative path for finding the attachment link in markdown
    let file_dir = crate::utils::get_parent_dir(&file_path);
    let attachment_path_buf = PathBuf::from(attachment_path);
    let relative_attachment_path = pathdiff::diff_paths(&attachment_path_buf, &file_dir)
        .unwrap_or_else(|| attachment_path_buf.clone());
    let relative_attachment_str = relative_attachment_path.to_string_lossy();

    // Remove attachment from element
    let new_content =
        remove_attachment_from_element(&content, element_name, &relative_attachment_str)?;

    // Generate diff
    let diff = generate_file_diff(&file_path, &content, &new_content);

    // Write to file if not dry run
    if !dry_run {
        fs::write(&absolute_file_path, &new_content).map_err(ReqvireError::IoError)?;

        // Mark file as modified for re-parsing
        model_manager
            .graph_registry
            .modified_files
            .insert(file_path.clone());
    }

    Ok(CrudResult {
        operation: CrudOperation::Update,
        element_id,
        element_name: format!("Detached {} from {}", attachment_path, element_name),
        diffs: vec![diff],
        dry_run,
    })
}

fn resolve_attachment_identifier_for_element(
    model_manager: &ModelManager,
    target_element_file_path: &str,
    attachment_target: &str,
) -> Result<String, ReqvireError> {
    if !attachment_target.contains('#') {
        return Err(ReqvireError::InvalidAttachmentTarget(format!(
            "Invalid attachment target '{}'. Attachments must use attachable element identifiers in the form 'file.md#element-id' or '#element-id'.",
            attachment_target
        )));
    }

    if attachment_target.starts_with('#') {
        return Ok(format!("{}{}", target_element_file_path, attachment_target));
    }

    if model_manager
        .graph_registry
        .get_element(attachment_target)
        .is_some()
    {
        return Ok(attachment_target.to_string());
    }

    let base_path = std::path::Path::new(target_element_file_path)
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));
    crate::utils::normalize_identifier(attachment_target, base_path).map_err(|e| {
        ReqvireError::InvalidAttachmentTarget(format!(
            "Invalid attachment target '{}': {}. Attachments must use attachable element identifiers in the form 'file.md#element-id' or '#element-id'.",
            attachment_target, e
        ))
    })
}

/// Attach an ontology or compatible refinement element identifier to an element.
pub fn attach_element_identifier(
    model_manager: &mut ModelManager,
    element_name: &str,
    attachment_target: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    use std::fs;

    let target_element = model_manager
        .graph_registry
        .get_element_by_name(element_name)
        .ok_or_else(|| {
            ReqvireError::ElementNotFound(format!("Element '{}' not found", element_name))
        })?;

    let element_id = target_element.identifier.clone();
    let file_path = target_element.file_path.clone();
    let attachment_identifier =
        resolve_attachment_identifier_for_element(model_manager, &file_path, attachment_target)?;

    let attachment_element = model_manager
        .graph_registry
        .get_element(&attachment_identifier)
        .ok_or_else(|| {
            ReqvireError::MissingAttachmentTarget(format!(
                "Attachment target '{}' could not be resolved to an existing element identifier.",
                attachment_target
            ))
        })?;

    if !attachment_element.element_type.is_refinement()
        && !attachment_element.element_type.is_ontology()
    {
        return Err(ReqvireError::InvalidAttachmentTarget(format!(
            "Element '{}' is not an attachable type. Only ontology elements and compatible Refinement elements can be attached.",
            attachment_element.name
        )));
    }

    let target_is_ontology = attachment_element.element_type.is_ontology();
    if !target_element.element_type.is_capability() && !target_element.element_type.is_requirement()
    {
        return Err(ReqvireError::InvalidAttachmentTarget(format!(
            "Element '{}' (type: {}) cannot author attachments. Only capability and requirement elements may author attachments; verification evidence must use satisfiedBy and verified targets must use verify.",
            element_name,
            target_element.element_type.as_str(),
        )));
    }

    let attachment_type_valid = if target_element.element_type.is_capability() {
        target_is_ontology
    } else {
        attachment_element.element_type.is_requirement_refinement()
    };

    if !attachment_type_valid {
        return Err(ReqvireError::InvalidAttachmentTarget(format!(
            "Element '{}' (type: {}) cannot attach '{}' (type: {}). Capability attachments may target ontology only; requirement attachments may target requirement-owned source, semantic-contract, semantic-query-contract, constraint, behavior, specification, state, or input-output.",
            element_name,
            target_element.element_type.as_str(),
            attachment_element.name,
            attachment_element.element_type.as_str()
        )));
    }

    if attachment_element.element_type.is_refinement()
        && !model_manager
            .graph_registry
            .refinement_has_refine_relation(&attachment_identifier)
    {
        return Err(ReqvireError::InvalidAttachmentTarget(
            format!(
                "'{}' has no refine relation. Refinements must refine a requirement before they can be attached; refinements are requirement-owned only. Capabilities attach ontology and are specified/verified, not refined by implementation-detail refinements.",
                attachment_element.name
            )
        ));
    }

    let defining_reqs = model_manager
        .graph_registry
        .get_defining_requirements(&attachment_identifier);
    for defining_req_id in defining_reqs {
        if model_manager
            .graph_registry
            .is_in_hierarchy(&element_id, &defining_req_id)
        {
            return Err(ReqvireError::InvalidAttachmentScope(
                format!(
                    "'{}' cannot be attached to '{}' because it is within the refinement's defining hierarchy. Attachments are only allowed from elements outside the refinedBy chain.",
                    attachment_element.name, element_name
                )
            ));
        }
    }

    if attachment_element.element_type.is_refinement() {
        if let Some(msg) = model_manager
            .graph_registry
            .build_attachment_direction_scope_error(
                &attachment_identifier,
                &element_id,
                element_name,
                None,
            )
        {
            return Err(ReqvireError::InvalidAttachmentScope(msg));
        }
    }

    if target_element
        .attachments
        .iter()
        .any(|a| a.target.as_str() == attachment_identifier)
    {
        return Err(ReqvireError::ElementError(format!(
            "Attachment '{}' already exists on '{}'",
            attachment_target, element_name
        )));
    }

    let in_relations = target_element
        .relations
        .iter()
        .any(|r| r.target.link.as_str() == attachment_identifier);
    if in_relations {
        return Err(ReqvireError::CrossSectionDuplicate(format!(
            "Target '{}' already exists in Relations of '{}'. Cannot add to Attachments.",
            attachment_target, element_name
        )));
    }

    validate_semantic_contracts_after_attachment_candidate(
        model_manager,
        &element_id,
        &attachment_identifier,
        true,
    )?;

    let absolute_file_path = git_root.join(&file_path);
    let content = fs::read_to_string(&absolute_file_path).map_err(ReqvireError::IoError)?;
    let attachment_display_name = attachment_element.name.clone();
    let attachment_file_path = attachment_element.file_path.clone();

    let relative_identifier = if file_path == attachment_file_path {
        let (_path, fragment_opt) = crate::utils::extract_path_and_fragment(&attachment_identifier);
        let fragment = fragment_opt.unwrap_or(&attachment_identifier);
        format!("#{}", fragment)
    } else {
        let target_file_path_buf = std::path::PathBuf::from(&file_path);
        let target_folder = target_file_path_buf
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .to_path_buf();
        crate::utils::to_relative_identifier(&attachment_identifier, &target_folder, true)
            .unwrap_or_else(|_| attachment_identifier.clone())
    };

    let new_content = add_element_attachment_to_element(
        &content,
        element_name,
        &attachment_display_name,
        &relative_identifier,
    )?;
    let diff = generate_file_diff(&file_path, &content, &new_content);

    if !dry_run {
        fs::write(&absolute_file_path, &new_content).map_err(ReqvireError::IoError)?;
        model_manager
            .graph_registry
            .modified_files
            .insert(file_path.clone());
    }

    Ok(CrudResult {
        operation: CrudOperation::Update,
        element_id,
        element_name: format!("Attached element {} to {}", attachment_target, element_name),
        diffs: vec![diff],
        dry_run,
    })
}

/// Attach a Refinement element by name to another element.
pub fn attach_element(
    model_manager: &mut ModelManager,
    element_name: &str,
    attachment_element_name: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    let attachment_identifier = model_manager
        .graph_registry
        .get_element_by_name(attachment_element_name)
        .ok_or_else(|| {
            ReqvireError::ElementNotFound(format!(
                "Attachment '{}' not found",
                attachment_element_name
            ))
        })?
        .identifier
        .clone();
    attach_element_identifier(
        model_manager,
        element_name,
        &attachment_identifier,
        git_root,
        dry_run,
    )
}

/// Detach a Refinement element identifier from an element.
pub fn detach_element_identifier(
    model_manager: &mut ModelManager,
    element_name: &str,
    attachment_target: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    use std::fs;

    let target_element = model_manager
        .graph_registry
        .get_element_by_name(element_name)
        .ok_or_else(|| {
            ReqvireError::ElementNotFound(format!("Element '{}' not found", element_name))
        })?;

    let element_id = target_element.identifier.clone();
    let file_path = target_element.file_path.clone();
    let attachment_identifier =
        resolve_attachment_identifier_for_element(model_manager, &file_path, attachment_target)?;

    let attachment_display_name = model_manager
        .graph_registry
        .get_element(&attachment_identifier)
        .map(|e| e.name.clone())
        .unwrap_or_else(|| attachment_identifier.clone());

    let absolute_file_path = git_root.join(&file_path);
    let content = fs::read_to_string(&absolute_file_path).map_err(ReqvireError::IoError)?;

    let target_file_path = std::path::PathBuf::from(&file_path);
    let target_folder = target_file_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .to_path_buf();
    let relative_identifier = {
        let (identifier_path, fragment_opt) =
            crate::utils::extract_path_and_fragment(&attachment_identifier);
        if identifier_path == file_path {
            format!("#{}", fragment_opt.unwrap_or(&attachment_identifier))
        } else {
            crate::utils::to_relative_identifier(&attachment_identifier, &target_folder, true)
                .unwrap_or_else(|_| attachment_identifier.clone())
        }
    };

    let new_content = remove_element_attachment_from_element(
        &content,
        element_name,
        &attachment_display_name,
        &relative_identifier,
    )?;

    validate_semantic_contracts_after_attachment_candidate(
        model_manager,
        &element_id,
        &attachment_identifier,
        false,
    )?;

    let diff = generate_file_diff(&file_path, &content, &new_content);

    if !dry_run {
        fs::write(&absolute_file_path, &new_content).map_err(ReqvireError::IoError)?;
        model_manager
            .graph_registry
            .modified_files
            .insert(file_path.clone());
    }

    Ok(CrudResult {
        operation: CrudOperation::Update,
        element_id,
        element_name: format!(
            "Detached element {} from {}",
            attachment_target, element_name
        ),
        diffs: vec![diff],
        dry_run,
    })
}

/// Detach a Refinement element by name from another element.
pub fn detach_element(
    model_manager: &mut ModelManager,
    element_name: &str,
    attachment_element_name: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    let attachment_identifier = model_manager
        .graph_registry
        .get_element_by_name(attachment_element_name)
        .ok_or_else(|| {
            ReqvireError::ElementNotFound(format!(
                "Attachment '{}' not found",
                attachment_element_name
            ))
        })?
        .identifier
        .clone();
    detach_element_identifier(
        model_manager,
        element_name,
        &attachment_identifier,
        git_root,
        dry_run,
    )
}

/// Move an asset file and update all references across elements (Attachments and Relations)
pub fn mv_asset(
    model_manager: &mut ModelManager,
    old_path: &str,
    new_path: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    use crate::relation::LinkType;
    use std::fs;
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
        return Err(ReqvireError::MissingAttachmentTarget(format!(
            "No elements reference file '{}'",
            old_path
        )));
    }

    let mut all_diffs = vec![];

    // Update references in each affected file
    for file_path in &affected_files {
        let absolute_file_path = git_root.join(file_path);
        let content = fs::read_to_string(&absolute_file_path).map_err(ReqvireError::IoError)?;

        let mut new_content = content.clone();

        // Paths in markdown are file-relative, but stored in registry as reqvire-root-relative
        // Calculate the file-relative paths that appear in the markdown
        let file_dir = crate::utils::get_parent_dir(file_path);

        // Calculate old and new file-relative paths
        let old_relative =
            pathdiff::diff_paths(&old_path_buf, &file_dir).unwrap_or_else(|| old_path_buf.clone());
        let new_relative =
            pathdiff::diff_paths(new_path, &file_dir).unwrap_or_else(|| PathBuf::from(new_path));

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
                fs::write(&absolute_file_path, &new_content).map_err(ReqvireError::IoError)?;

                model_manager
                    .graph_registry
                    .modified_files
                    .insert(file_path.clone());
            }
        }
    }

    // Move the actual file
    if !dry_run {
        let old_abs = git_root.join(old_path);
        let new_abs = git_root.join(new_path);

        // Create parent directory if needed
        if let Some(parent) = new_abs.parent() {
            fs::create_dir_all(parent).map_err(ReqvireError::IoError)?;
        }

        fs::rename(&old_abs, &new_abs).map_err(ReqvireError::IoError)?;
    }

    Ok(CrudResult {
        operation: CrudOperation::Move,
        element_id: old_path.to_string(),
        element_name: format!(
            "Moved {} → {} ({} attachment(s), {} relation(s) in {} file(s))",
            old_path,
            new_path,
            attachment_count,
            relation_count,
            affected_files.len()
        ),
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
    use crate::relation::LinkType;
    use std::fs;
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
        let content = fs::read_to_string(&absolute_file_path).map_err(ReqvireError::IoError)?;

        // Paths in markdown are file-relative, calculate the relative path from this file
        let file_dir = crate::utils::get_parent_dir(spec_file_path);
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
                fs::write(&absolute_file_path, &new_content).map_err(ReqvireError::IoError)?;

                model_manager
                    .graph_registry
                    .modified_files
                    .insert(spec_file_path.clone());
            }
        }
    }

    // Delete the actual file
    if !dry_run {
        let abs_path = git_root.join(file_path_arg);
        if abs_path.exists() {
            fs::remove_file(&abs_path).map_err(ReqvireError::IoError)?;
        }
    }

    Ok(CrudResult {
        operation: CrudOperation::Remove,
        element_id: file_path_arg.to_string(),
        element_name: format!(
            "Removed {} ({} attachment(s), {} relation(s) from {} file(s))",
            file_path_arg,
            attachment_count,
            relation_count,
            affected_files.len()
        ),
        diffs: all_diffs,
        dry_run,
    })
}

/// Merge multiple source elements into a target element
///
/// # Arguments
/// * `model_manager` - The model manager
/// * `target_name` - Name of the target element (receives merged content)
/// * `source_names` - Names of source elements to merge into target
/// * `git_root` - Git root directory
/// * `dry_run` - If true, don't write changes to disk
pub fn merge_elements(
    model_manager: &mut ModelManager,
    target_name: &str,
    source_names: &[String],
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    // Resolve target element by name
    let target_element = model_manager
        .graph_registry
        .get_element_by_name(target_name)
        .ok_or_else(|| {
            ReqvireError::ElementNotFound(format!("Target element '{}' not found", target_name))
        })?;
    let target_id = target_element.identifier.clone();

    // Resolve all source elements (validation is done in graph_registry.merge_elements)
    let mut source_ids = Vec::new();
    for source_name in source_names {
        let source_element = model_manager
            .graph_registry
            .get_element_by_name(source_name)
            .ok_or_else(|| {
                ReqvireError::ElementNotFound(format!("Source element '{}' not found", source_name))
            })?;

        source_ids.push(source_element.identifier.clone());
    }

    // Track which files were modified before the operation
    let modified_before = snapshot_modified_files(model_manager);

    // Capture BEFORE state: Get current in-memory content for files that will be modified
    // We need to do this BEFORE the merge to show proper diffs
    let target_file = target_element.file_path.clone();
    let source_files: Vec<String> = source_ids
        .iter()
        .filter_map(|id| model_manager.graph_registry.get_element(id))
        .map(|el| el.file_path.clone())
        .collect();

    let mut before_content: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    let grouped_before = model_manager.graph_registry.group_elements_by_location();

    // Capture content for target file
    if let Some(sections) = grouped_before.get(&target_file) {
        before_content.insert(
            target_file.clone(),
            model_manager
                .graph_registry
                .generate_file_markdown(&target_file, sections, false),
        );
    }

    // Capture content for source files
    for source_file in &source_files {
        if !before_content.contains_key(source_file.as_str()) {
            if let Some(sections) = grouped_before.get(source_file.as_str()) {
                before_content.insert(
                    source_file.clone(),
                    model_manager.graph_registry.generate_file_markdown(
                        source_file,
                        sections,
                        false,
                    ),
                );
            }
        }
    }

    let registry_snapshot = model_manager.graph_registry.clone();

    // Perform the merge in graph_registry
    model_manager
        .graph_registry
        .merge_elements(&target_id, &source_ids)?;

    if let Err(err) = enforce_single_root_after_mutation(model_manager) {
        model_manager.graph_registry = registry_snapshot;
        return Err(err);
    }

    if let Err(err) = validate_semantic_contracts_after_mutation(model_manager, None) {
        model_manager.graph_registry = registry_snapshot;
        return Err(err);
    }

    let modified_files = collect_new_modified_files(model_manager, &modified_before);

    // Generate diffs for output using BEFORE vs AFTER in-memory state
    let mut diffs = Vec::new();
    let grouped_after = model_manager.graph_registry.group_elements_by_location();

    for file_path in &modified_files {
        let before = before_content
            .get(file_path)
            .map(|s| s.as_str())
            .unwrap_or("");

        let after = if let Some(sections) = grouped_after.get(file_path) {
            model_manager
                .graph_registry
                .generate_file_markdown(file_path, sections, false)
        } else {
            String::new()
        };

        let diff = crate::diff::generate_file_diff(file_path, before, &after);
        if !diff.lines.is_empty() {
            diffs.push(diff);
        }
    }

    // Flush changes if not dry-run
    if !dry_run {
        model_manager
            .graph_registry
            .flush_modified_files(git_root)?;
    }

    // Create result structure
    Ok(CrudResult {
        operation: CrudOperation::Merge,
        element_id: target_id,
        element_name: format!(
            "Merged {} element(s) into '{}'",
            source_names.len(),
            target_name
        ),
        diffs,
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
fn add_attachment_to_element(
    content: &str,
    element_name: &str,
    attachment_path: &str,
) -> Result<String, ReqvireError> {
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
                if next_trimmed.starts_with("* ")
                    || next_trimmed.starts_with("- ")
                    || next_trimmed.is_empty()
                {
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
        return Err(ReqvireError::ElementNotFound(format!(
            "Could not find element '{}' to add attachment",
            element_name
        )));
    }

    Ok(result)
}

// Helper function to remove attachment from element in markdown content
fn remove_attachment_from_element(
    content: &str,
    element_name: &str,
    attachment_path: &str,
) -> Result<String, ReqvireError> {
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
        if in_attachments_section
            && ((trimmed.starts_with("####") && trimmed != "#### Attachments") || trimmed == "---")
        {
            in_attachments_section = false;
        }

        // Skip the attachment line we want to remove
        if in_target_element && in_attachments_section {
            if (trimmed.starts_with("* ") || trimmed.starts_with("- "))
                && trimmed.contains(&attachment_link)
            {
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
fn add_element_attachment_to_element(
    content: &str,
    element_name: &str,
    display_name: &str,
    identifier: &str,
) -> Result<String, ReqvireError> {
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
                if next_trimmed.starts_with("* ")
                    || next_trimmed.starts_with("- ")
                    || next_trimmed.is_empty()
                {
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
        return Err(ReqvireError::ElementNotFound(format!(
            "Could not find element '{}' to add attachment",
            element_name
        )));
    }

    Ok(result)
}

// Helper function to remove element attachment from element in markdown content
fn remove_element_attachment_from_element(
    content: &str,
    element_name: &str,
    display_name: &str,
    identifier: &str,
) -> Result<String, ReqvireError> {
    let mut result = String::new();
    let mut in_target_element = false;
    let mut in_attachments_section = false;
    let mut removed = false;
    let mut remaining_attachments_count = 0;

    // Match by either identifier or display name in the link
    let attachment_link_by_id = format!("]({})", identifier);
    let attachment_link_full = format!("[{}]({})", display_name, identifier);
    let attachment_link_by_name = format!("[{}](", display_name);

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
        if in_attachments_section
            && ((trimmed.starts_with("####") && trimmed != "#### Attachments") || trimmed == "---")
        {
            in_attachments_section = false;
        }

        // Skip the attachment line we want to remove
        if in_target_element && in_attachments_section {
            if (trimmed.starts_with("* ") || trimmed.starts_with("- "))
                && (trimmed.contains(&attachment_link_by_id)
                    || trimmed.contains(&attachment_link_full)
                    || trimmed.contains(&attachment_link_by_name))
            {
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
fn remove_attachment_from_file(
    content: &str,
    attachment_path: &str,
) -> Result<String, ReqvireError> {
    let mut result = String::new();
    let attachment_link = format!("[{}]({})", attachment_path, attachment_path);

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip attachment lines matching our path
        if (trimmed.starts_with("* ") || trimmed.starts_with("- "))
            && trimmed.contains(&attachment_link)
        {
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

/// Link a relation between two elements or to external URLs/internal paths
///
/// # Arguments
/// * `model_manager` - The model manager
/// * `source` - Source element name
/// * `relation_type` - The relation type (derivedFrom, derive, verifiedBy, verify, satisfiedBy, satisfy, trace)
/// * `target` - Target element name, internal file path, or external URL
/// * `git_root` - Git root directory
/// * `dry_run` - If true, don't write changes to disk
pub fn link(
    model_manager: &mut ModelManager,
    source: &str,
    relation_type: &str,
    target: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    // Resolve source element by name
    let source_element = model_manager
        .graph_registry
        .get_element_by_name(source)
        .ok_or_else(|| {
            ReqvireError::ElementNotFound(format!("Source element '{}' not found", source))
        })?;

    let source_id = source_element.identifier.clone();
    let source_name = source_element.name.clone();

    // Track which files were modified before the operation
    let modified_before = snapshot_modified_files(model_manager);

    let registry_snapshot = model_manager.graph_registry.clone();

    // Delegate to graph_registry (includes all validation and target resolution)
    // NOTE: This will mutate the in-memory graph even in dry-run mode
    let link_result = model_manager.graph_registry.add_element_relation_full(
        &source_id,
        target,
        relation_type,
        git_root,
    );
    if let Err(err) = link_result {
        model_manager.graph_registry = registry_snapshot;
        return Err(err);
    }

    if let Err(err) = enforce_single_root_after_mutation(model_manager) {
        model_manager.graph_registry = registry_snapshot;
        return Err(err);
    }

    let diffs =
        match finalize_crud_operation(model_manager, &modified_before, git_root, dry_run, None) {
            Ok(diffs) => diffs,
            Err(err) => {
                model_manager.graph_registry = registry_snapshot;
                return Err(err);
            }
        };

    Ok(CrudResult {
        operation: CrudOperation::Update,
        element_id: source_id,
        element_name: format!("Linked {} {} {}", source_name, relation_type, target),
        diffs,
        dry_run,
    })
}

/// Atomically relink a relation: replace one target with another for the same relation type.
///
/// This operation is all-or-nothing from caller perspective. If add fails after remove,
/// in-memory graph state is rolled back to the pre-operation snapshot.
pub fn relink(
    model_manager: &mut ModelManager,
    source: &str,
    relation_type: &str,
    from_target: &str,
    to_target: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    // Resolve source element by name
    let source_element = model_manager
        .graph_registry
        .get_element_by_name(source)
        .ok_or_else(|| {
            ReqvireError::ElementNotFound(format!("Source element '{}' not found", source))
        })?;

    let source_id = source_element.identifier.clone();
    let source_name = source_element.name.clone();

    if from_target == to_target {
        return Err(ReqvireError::RelationError(
            "Relink requires different source and target relation endpoints".to_string(),
        ));
    }

    // Validate existing relation type before mutation.
    let from_target_resolved = if let Some(element) = model_manager
        .graph_registry
        .get_element_by_name(from_target)
    {
        element.identifier.clone()
    } else {
        from_target.to_string()
    };

    let existing_relation = model_manager
        .graph_registry
        .nodes
        .get(&source_id)
        .and_then(|n| {
            n.element
                .relations
                .iter()
                .find(|r| r.target.link.as_str() == from_target_resolved)
        })
        .cloned()
        .ok_or_else(|| {
            ReqvireError::RelationError(format!(
                "No relation found from '{}' to '{}'",
                source, from_target
            ))
        })?;

    if existing_relation.relation_type.name != relation_type {
        return Err(ReqvireError::RelationError(format!(
            "Relation mismatch: '{}' -> '{}' exists as '{}', not '{}'",
            source, from_target, existing_relation.relation_type.name, relation_type
        )));
    }

    let modified_before = snapshot_modified_files(model_manager);
    let registry_snapshot = model_manager.graph_registry.clone();

    let mutation_result = (|| -> Result<(), ReqvireError> {
        let removed = model_manager
            .graph_registry
            .remove_element_relation_full(&source_id, from_target)?;
        if removed.is_none() {
            return Err(ReqvireError::RelationError(format!(
                "No relation found from '{}' to '{}'",
                source, from_target
            )));
        }

        model_manager.graph_registry.add_element_relation_full(
            &source_id,
            to_target,
            relation_type,
            git_root,
        )?;
        enforce_single_root_after_mutation(model_manager)?;
        Ok(())
    })();

    if let Err(err) = mutation_result {
        model_manager.graph_registry = registry_snapshot;
        return Err(err);
    }

    let diffs =
        match finalize_crud_operation(model_manager, &modified_before, git_root, dry_run, None) {
            Ok(diffs) => diffs,
            Err(err) => {
                model_manager.graph_registry = registry_snapshot;
                return Err(err);
            }
        };

    Ok(CrudResult {
        operation: CrudOperation::Update,
        element_id: source_id,
        element_name: format!(
            "Relinked {} {} {} -> {}",
            source_name, relation_type, from_target, to_target
        ),
        diffs,
        dry_run,
    })
}

/// Unlink a relation or attachment between two elements (auto-detects type)
///
/// Searches relations first, then attachments. Only one relation per source-target pair is allowed.
///
/// # Arguments
/// * `model_manager` - The model manager
/// * `source` - Source element name
/// * `target` - Target element name or file path
/// * `git_root` - Git root directory
/// * `dry_run` - If true, don't write changes to disk
pub fn unlink(
    model_manager: &mut ModelManager,
    source: &str,
    target: &str,
    git_root: &Path,
    dry_run: bool,
) -> Result<CrudResult, ReqvireError> {
    // Resolve source element by name
    let source_element = model_manager
        .graph_registry
        .get_element_by_name(source)
        .ok_or_else(|| {
            ReqvireError::ElementNotFound(format!("Source element '{}' not found", source))
        })?;

    let source_id = source_element.identifier.clone();
    let source_name = source_element.name.clone();

    // Track modified files before
    let modified_before = snapshot_modified_files(model_manager);
    let registry_snapshot = model_manager.graph_registry.clone();

    // Try to remove relation via graph_registry
    // This handles element-to-element relations (NOT attachments)
    match model_manager
        .graph_registry
        .remove_element_relation_full(&source_id, target)?
    {
        Some((_modified_file, relation_type, target_display)) => {
            // Relation was found and removed
            // Get newly modified files
            let diffs = match finalize_crud_operation(
                model_manager,
                &modified_before,
                git_root,
                dry_run,
                None,
            ) {
                Ok(diffs) => diffs,
                Err(err) => {
                    model_manager.graph_registry = registry_snapshot;
                    return Err(err);
                }
            };

            Ok(CrudResult {
                operation: CrudOperation::Update,
                element_id: source_id,
                element_name: format!(
                    "Unlinked {} {} {}",
                    source_name, relation_type, target_display
                ),
                diffs,
                dry_run,
            })
        }
        None => {
            // No relation found - check if it's an attachment instead
            // Get fresh source element (in case graph was modified)
            let source_element = model_manager
                .graph_registry
                .get_element_by_name(source)
                .ok_or_else(|| {
                    ReqvireError::ElementNotFound(format!("Source element '{}' not found", source))
                })?;

            // Check if target is an element attachment
            if let Some(target_element) = model_manager.graph_registry.get_element_by_name(target) {
                let target_id = &target_element.identifier;
                let attachment_match = source_element
                    .attachments
                    .iter()
                    .find(|a| a.target.as_str() == target_id.as_str());

                if attachment_match.is_some() {
                    return detach_element(model_manager, source, target, git_root, dry_run);
                }
            }

            // Check if target is a file path attachment
            let cwd = std::env::current_dir().unwrap_or_default();
            let file_exists_cwd = cwd.join(target).exists();
            let file_exists_git_root = git_root.join(target).exists();

            if file_exists_cwd || file_exists_git_root {
                return detach(model_manager, source, target, git_root, dry_run);
            }

            // Check attachments by path string (even if file doesn't exist anymore)
            let attachment_by_path = source_element
                .attachments
                .iter()
                .find(|a| a.target.as_str() == target || a.target.as_str().ends_with(target));

            if attachment_by_path.is_some() {
                return detach(model_manager, source, target, git_root, dry_run);
            }

            // Nothing found
            Err(ReqvireError::RelationError(format!(
                "No relation or attachment found from '{}' to '{}'",
                source, target
            )))
        }
    }
}
