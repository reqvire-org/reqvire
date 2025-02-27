use anyhow::Result;
use std::path::Path;

use crate::element::ElementRegistry;
use crate::error::ReqFlowError;
use crate::relation::Relation;

/// Validate a relation target exists in the element registry
#[allow(dead_code)]
pub fn validate_relation_target(
    relation: &Relation,
    current_path: &Path,
    registry: &ElementRegistry,
) -> Result<(), ReqFlowError> {
    let target = relation.target.trim();
    
    // Handle different types of targets
    if target.contains('/') {
        // External document or specific element reference
        if target.ends_with(".md") {
            // Just a file reference, ensure file exists
            // In a full implementation, we'd check if the file exists
            // For now, we'll assume it's valid
            Ok(())
        } else {
            // Element reference in another file
            // First, check if we have this element in the registry
            if registry.contains_element(target) {
                Ok(())
            } else {
                Err(ReqFlowError::MissingRelationTarget(target.to_string()))
            }
        }
    } else {
        // Same-document element reference
        // Construct the full identifier by combining current path with fragment
        let current_file = current_path.to_string_lossy();
        let full_identifier = format!("{}/{}", current_file, target);
        
        if registry.contains_element(&full_identifier) {
            Ok(())
        } else {
            Err(ReqFlowError::MissingRelationTarget(full_identifier))
        }
    }
}

/// Validate that all relation targets exist in the model
#[allow(dead_code)]
pub fn validate_relations(registry: &ElementRegistry) -> Result<Vec<ReqFlowError>, ReqFlowError> {
    let mut errors = Vec::new();
    
    for element in registry.all_elements() {
        let element_path = Path::new(&element.file_path);
        
        for relation in &element.relations {
            // First validate relation type format
            if let Err(err) = relation.validate_type() {
                errors.push(err);
                continue; // Skip target validation if type is invalid
            }
            
            // Then validate relation target
            if let Err(err) = validate_relation_target(relation, element_path, registry) {
                errors.push(err);
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
    
    // Validate element headers are unique
    for (line_num, line) in content.lines().enumerate() {
        if line.trim().starts_with("### ") {
            let name = line.trim()[4..].trim().to_string();
            
            if element_names.contains(&name) {
                errors.push(ReqFlowError::DuplicateElement(format!(
                    "Duplicate element '{}' found at line {}",
                    name,
                    line_num + 1
                )));
            } else {
                element_names.push(name);
            }
        }
    }
    
    Ok(errors)
}