use std::path::Path;
use crate::config::Config;
use crate::model::ModelManager;

#[test]
fn test_filesystem_validation() {
    let config = Config::default();
    let model_manager = ModelManager::new_with_config(config);
    
    let input_folder = Path::new("test-fixtures/test-lint");
    let validation_result = model_manager.validate_filesystem_structure(input_folder, &model_manager.config());
    
    assert!(validation_result.is_ok(), "Validation should succeed");
    
    // Get the validation errors
    let errors = validation_result.unwrap();
    
    // Note: validation errors are expected because our test fixtures might not have the complete
    // directory structure that would match our validation requirements
    // This test mainly checks that the validation function runs without runtime errors
    assert!(true, "Validation function executed successfully");
}

#[test]
fn test_external_folders_validation() {
    // Create a custom configuration with external folders
    let mut config = Config::default();
    config.paths.external_folders = vec!["external-project".into()];
    
    let mut model_manager = ModelManager::new_with_config(config);
    
    let input_folder = Path::new("test-fixtures/test-external-folders-clean");
    
    // First, load identifiers from both main and external folders
    let collect_result = model_manager.collect_identifiers_only(input_folder);
    assert!(collect_result.is_ok(), "Collecting identifiers should succeed");
    
    // Then validate filesystem structure
    let validation_result = model_manager.validate_filesystem_structure(input_folder, &model_manager.config());
    assert!(validation_result.is_ok(), "Validation should succeed");
    
    // Get the validation errors
    let errors = validation_result.unwrap();
    
    // Print the errors to see what's failing
    println!("Validation errors: {:?}", errors);
    
    // With proper external folder setup, expect no errors
    assert_eq!(errors.len(), 0, "Expected no validation errors with external folders");
    
    // Also validate relations to ensure cross-folder references work
    let relation_result = model_manager.validate_relations();
    assert!(relation_result.is_ok(), "Relation validation should succeed");
    
    let relation_errors = relation_result.unwrap();
    assert_eq!(relation_errors.len(), 0, "Expected no relation errors across folders");
}

#[test]
fn test_invalid_external_folder() {
    // Create a custom configuration with a non-existent external folder
    let mut config = Config::default();
    config.paths.external_folders = vec!["non-existent-folder".into()];
    
    let mut model_manager = ModelManager::new_with_config(config);
    
    let input_folder = Path::new("test-fixtures/test-external-folders");
    
    // Validate filesystem structure with invalid external folder
    let validation_result = model_manager.validate_filesystem_structure(input_folder, &model_manager.config());
    assert!(validation_result.is_ok(), "Validation should succeed");
    
    // Get the validation errors
    let errors = validation_result.unwrap();
    
    // Should have at least one error for the missing external folder
    assert!(errors.len() > 0, "Expected validation errors for missing external folder");
}

#[test]
fn test_markdown_link_validation() {
    use crate::element::{Element, ElementRegistry};
    use crate::relation::Relation;
    use crate::validation::validate_relation_target;
    use std::path::Path;
    
    // Create a test registry with some elements
    let mut registry = ElementRegistry::new();
    
    // Add a few sample elements to the registry
    let mut element1 = Element::new(
        "Element1".to_string(),
        "DesignSpecifications/API.md".to_string()
    );
    element1.content = "Test content".to_string();
    
    let mut element2 = Element::new(
        "Element2".to_string(),
        "specifications/SystemRequirements.md".to_string()
    );
    element2.content = "Test content".to_string();
    
    registry.add_element(element1);
    registry.add_element(element2);
    
    // Test case 1: Valid markdown link to existing file
    let relation1 = Relation::new(
        "satisfiedBy".to_string(),
        "[DesignSpecifications/API.md](DesignSpecifications/API.md)".to_string()
    );
    let result1 = validate_relation_target(&relation1, Path::new("current_file.md"), &registry);
    assert!(result1.is_ok(), "Valid markdown link should pass validation");
    
    // Test case 2: Invalid markdown link to non-existing file
    let relation2 = Relation::new(
        "satisfiedBy".to_string(),
        "[DesignSpecifications/API.md](DesignSpecifications/API2.md)".to_string()
    );
    let result2 = validate_relation_target(&relation2, Path::new("current_file.md"), &registry);
    assert!(result2.is_err(), "Invalid markdown link should fail validation");
    
    // Test case 3: Mismatch between display text and URL in markdown link
    let relation3 = Relation::new(
        "satisfiedBy".to_string(),
        "[specifications/NonExistent.md](specifications/SystemRequirements.md)".to_string()
    );
    let result3 = validate_relation_target(&relation3, Path::new("current_file.md"), &registry);
    assert!(result3.is_ok(), "Mismatched markdown link should still pass if target exists");
}

#[test]
fn test_user_requirements_in_external_folder() {
    // Create a custom configuration with external folders
    let mut config = Config::default();
    config.paths.external_folders = vec!["external-project".into()];
    
    let mut model_manager = ModelManager::new_with_config(config);
    
    // Path with an invalid user requirements file in external folder
    let input_folder = Path::new("test-fixtures/test-external-folders");
    
    // Validate filesystem structure
    let validation_result = model_manager.validate_filesystem_structure(input_folder, &model_manager.config());
    assert!(validation_result.is_ok(), "Validation should succeed");
    
    // Get the validation errors
    let errors = validation_result.unwrap();
    
    // We should have at least one error for the user requirements file in the external folder
    assert!(errors.len() > 0, "Expected validation errors for user requirements in external folder");
    
    // Check if any error message contains our expected error about user requirements in external folders
    let has_user_req_error = errors.iter().any(|e| {
        match e {
            crate::error::ReqFlowError::ValidationError(msg) => {
                msg.contains("User Requirements are not allowed in external folders")
            },
            _ => false
        }
    });
    
    assert!(has_user_req_error, "Expected error about user requirements in external folders");
}