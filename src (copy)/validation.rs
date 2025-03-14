use anyhow::Result;
use std::path::Path;
use std::path::PathBuf;

use crate::element::ElementRegistry;
use crate::error::ReqFlowError;
use crate::relation::Relation;
use crate::config::Config;
use crate::ModelManager;
use crate::relation::get_parent_relation_types;

/// Structure for JSON output of validation results
#[derive(serde::Serialize)]
struct ValidationResult {
    validation_type: String,
    errors: Vec<String>,
    fixed: bool, // Kept for API compatibility but always false now
}

pub fn run_validation_checks(
    model_manager: &mut ModelManager,
    config: &Config,
    input_folder_path: &PathBuf,
) -> Result<i32> {  
    let mut exit_code = 0;

    //  Markdown validation
    if config.validation.validate_markdown || config.validation.validate_all {
        let markdown_errors = model_manager.validate_markdown_structure()?;
        if !markdown_errors.is_empty() {
            exit_code = 1;
            print_validation_results("markdown", &markdown_errors, config.validation.json_output);
        }
    }

    // Relation validation
    if config.validation.validate_relations || config.validation.validate_all {
        let relation_errors = model_manager.validate_relations()?;
        if !relation_errors.is_empty() {
            exit_code = 1;
            print_validation_results("relations", &relation_errors, config.validation.json_output);
        }
    }

    // Filesystem structure validation (Missing Before)
    if config.validation.validate_all {
        let filesystem_errors = model_manager.validate_filesystem_structure(input_folder_path)?;
        if !filesystem_errors.is_empty() {
            exit_code = 1;
            print_validation_results("filesystem", &filesystem_errors, config.validation.json_output);
        }
    }

    // Cross-component dependency validation (Missing Before)
    if config.validation.validate_all {
        let dependency_errors = model_manager.validate_cross_component_dependencies()?;
        if !dependency_errors.is_empty() {
            exit_code = 1;
            print_validation_results("dependencies", &dependency_errors, config.validation.json_output);
        }
    }

    Ok(exit_code)
}

// Helper function to print validation results
fn print_validation_results(validation_type: &str, errors: &[impl ToString], json_output: bool) {
    if json_output {
        let json_result = ValidationResult {
            validation_type: validation_type.to_string(),
            errors: errors.iter().map(|e| e.to_string()).collect(),
            fixed: false,
        };
        println!("{}", serde_json::to_string_pretty(&json_result).unwrap());
    } else {
        println!("❌ {} validation failed with {} errors.", validation_type, errors.len());
        for error in errors {
            println!("  - {}", error.to_string());
        }
    }
}

/// Validate a relation target exists in the element registry
pub fn validate_relation_target(
    relation: &Relation,
    current_path: &Path,
    registry: &ElementRegistry,
) -> Result<(), ReqFlowError> {
    let target = relation.target.trim();
    let config = registry.config();
    let base_path = &config.paths.base_path;

    log::debug!("Validating relation target: '{}' from '{}'", target, current_path.display());

    // **Ignore external HTTP/HTTPS links**
    if target.starts_with("http://") || target.starts_with("https://") {
        log::debug!("Ignoring external link: {}", target);
        return Ok(());
    }

    // **Extract URL from Markdown links `[text](url)`**
    if let Some(start) = target.find("(") {
        if let Some(end) = target.rfind(")") {
            let url = &target[start + 1..end];

            // Ensure resolved_path is stored properly before using
            let resolved_path = crate::utils::normalize_path(url, registry.config(), &registry.config().paths.base_path.display().to_string());

            let (file_path, fragment) = if let Some(pos) = resolved_path.find('#') {
                (&resolved_path[..pos], Some(&resolved_path[pos + 1..]))
            } else {
                (resolved_path.as_ref(), None)
            };            
            
            let normalized_file_path = crate::utils::normalize_path(file_path, registry.config(),"");

            // Step 4: Normalize fragment (if it exists)
            let normalized_fragment = fragment.map(crate::utils::normalize_fragment);

            // Step 5: Construct **correct** full identifier
            let full_identifier = if let Some(fragment) = normalized_fragment {
                if resolved_path.contains('#') {
                    resolved_path.to_string() // Avoid duplicate `#fragment`
                } else {
                    format!("{}#{}", resolved_path, fragment)
                }
            } else {
                resolved_path.to_string()
            };


            log::debug!(
                "Resolved path: {} -> Normalized path: {} -> Final identifier: {}",
                resolved_path,
                normalized_file_path,
                full_identifier
            );

            // Step 6: Check if element exists in registry
            if registry.contains_element(&full_identifier) || registry.contains_element(&normalized_file_path) {
                return Ok(());
            }

            return Err(ReqFlowError::MissingRelationTarget(format!(
                "Referenced identifier not found: {}",
                url
            )));
        }
    }
     // Handle simple file references without `[text](...)` format
    //let resolved_path = crate::utils::resolve_relative_path(current_path, target);
    let normalized_file_path = crate::utils::normalize_path(target, registry.config(), "");
    let normalized_fragment = crate::utils::normalize_fragment(target);

    let full_identifier = format!("{}#{}", normalized_file_path, normalized_fragment);

    if registry.contains_element(&full_identifier) || registry.contains_element(&normalized_file_path) {
        return Ok(());
    }

    Err(ReqFlowError::MissingRelationTarget(target.to_string()))
}



/// Validate that all relation targets exist in the model
pub fn validate_relations(registry: &ElementRegistry) -> Result<Vec<ReqFlowError>, ReqFlowError> {
    let mut errors = Vec::new();
    
    // Log all elements in the registry to understand what's available
    log::debug!("Registry contains the following elements:");
    for element in registry.all_elements() {
        log::debug!("  Element: {}/{}", element.file_path, element.name);
        // Log metadata if available
        for (key, value) in &element.metadata {
            log::debug!("    Metadata: {} = {}", key, value);
        }
    }
    
    for element in registry.all_elements() {
        let element_path = Path::new(&element.file_path);
        
        // Check if this element requires a parent relation
        if element.requires_parent_relation(registry) {
            let has_parent = element.relations.iter().any(|relation| {
                get_parent_relation_types().contains(&relation.relation_type.as_str())
            });

            if !has_parent {
                errors.push(ReqFlowError::MissingParentRelation(format!(
                    "In file '{}', Element '{}' requires a parent relation but none found.",
                    element.file_path, element.name
                )));
            }
        }        
        // Check for duplicate relations within the same element
        let mut relation_map = std::collections::HashMap::new();
        
        for (index, relation) in element.relations.iter().enumerate() {
            // First validate relation type format
            if let Err(err) = relation.validate_type(element.file_path.clone(),element.name.clone()) {
                errors.push(err);
                continue; // Skip target validation if type is invalid
            }
            
            // Then validate relation target
            if let Err(err) = validate_relation_target(relation, element_path, registry) {
                // Add file path to the error message for better debugging
                if let ReqFlowError::MissingRelationTarget(target) = err {
                    errors.push(ReqFlowError::MissingRelationTarget(
                        format!("In file '{}', element '{}': {}", element.file_path, element.name, target)
                    ));
                } else {
                    errors.push(err);
                }
            }
            
            // Check for duplicate relations (same type and target)
            // Normalize the relation type and target by trimming whitespace
            let normalized_type = relation.relation_type.trim().to_string();
            let normalized_target = relation.target.trim().to_string();
            
            let relation_key = format!("{}:{}", normalized_type, normalized_target);
            if let Some(prev_index) = relation_map.get(&relation_key) {
                errors.push(ReqFlowError::DuplicateRelation(format!(
                    "Duplicate relation '{}' in element '{}' (previous at relation #{}, duplicated at #{})",
                    relation_key,
                    element.name,
                    prev_index + 1,
                    index + 1
                )));
            } else {
                relation_map.insert(relation_key, index);
            }
        }
    }
    
    Ok(errors)
}

/// Validate the structure of a markdown document
#[allow(dead_code)]
pub fn validate_markdown_structure(content: &str) -> Result<Vec<ReqFlowError>, ReqFlowError> {
    let mut errors = Vec::new();
    let mut element_names = Vec::new();
    let mut current_element: Option<String> = None;
    let mut current_element_subsections: Vec<String> = Vec::new();
    
    // Validate element headers are unique, considering subsections within their parent context
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        
        if trimmed.starts_with("### ") {
            // New level 3 header (element) - check for uniqueness among elements
            let name = trimmed[4..].trim().to_string();
            
            // Check if this element name is a duplicate at the same level
            if element_names.contains(&name) {
                errors.push(ReqFlowError::DuplicateElement(format!(
                    "Duplicate element '{}' found at line {}",
                    name,
                    line_num + 1
                )));
            } else {
                // Record this as a new element
                element_names.push(name.clone());
                
                // Update current element tracking
                current_element = Some(name);
                
                // Reset subsections for the new element
                current_element_subsections.clear();
            }
        } else if trimmed.starts_with("#### ") && current_element.is_some() {
            // Level 4 header (subsection) - check for uniqueness within current element
            let subsection_name = trimmed[5..].trim().to_string();
            
            // Check if this subsection is a duplicate within this element
            if current_element_subsections.contains(&subsection_name) {
                errors.push(ReqFlowError::DuplicateSubsection(format!(
                    "Duplicate subsection '{}' within element '{}' found at line {}",
                    subsection_name,
                    current_element.as_ref().unwrap(),
                    line_num + 1
                )));
            } else {
                // Record this subsection for the current element
                current_element_subsections.push(subsection_name);
            }
        } else if trimmed.starts_with("## ") || trimmed.starts_with("# ") {
            // Higher level heading - reset current element tracking
            current_element = None;
            current_element_subsections.clear();
        }
    }
    
    Ok(errors)
}
