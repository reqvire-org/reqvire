use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{ PathBuf};
use walkdir::WalkDir;
use log::{debug};
use crate::element::Element;
use crate::element_registry::ElementRegistry;
use crate::error::ReqFlowError;
use crate::relation::LinkType;
use crate::relation::{get_parent_relation_types, is_circular_dependency_relation};
use crate::element::ElementType;
use crate::element::RequirementType;


use crate::utils;
use crate::parser::parse_elements;
use globset::GlobSet;

#[derive(Debug)]
pub struct ModelManager {
    /// In-memory registry of elements
    pub element_registry: ElementRegistry,

    /// Stores file content for validation
    file_contents: HashMap<String, String>,

}

impl ModelManager {
    /// Creates a new ModelManager
    pub fn new() -> Self {
        // Initialize empty element registry
        let element_registry = ElementRegistry::new();

        // Initialize empty file contents
        let file_contents = HashMap::new();

        Self {
            element_registry,
            file_contents
        }
    }

    /// Parses and validates all Markdown files while building the registry.
    /// Returns a list of validation errors.
    pub fn parse_and_validate(
        &mut self, 
        specification_folder: &PathBuf, 
        external_folders: &[PathBuf],
        excluded_filename_patterns: &GlobSet
    ) -> Result<Vec<ReqFlowError>, ReqFlowError> {
        debug!("Parsing and validating files in {:?}", specification_folder);
        let mut errors = Vec::new();

        for entry in WalkDir::new(specification_folder)
            .into_iter()
            .filter_map(Result::ok) // Skip errors during directory iteration
            .filter(|e| e.path().is_file()) //  Process only files
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md")) //  Only `.md` files 
            .filter(|e| utils::is_requirements_file_by_path(e.path(), excluded_filename_patterns)) // Only files with elements
        {
            let path = entry.path();

            match (path.file_name(), path) {
                (Some(file_name), file_path) => {
                    debug!("Markdown File found: {}", file_name.to_string_lossy());

                    let file_content = fs::read_to_string(path)?;
                    let relative_path = utils::get_relative_path(path, specification_folder)?;
                    let relative_path_str = relative_path.to_string_lossy().to_string();
    
                    self.file_contents.insert(relative_path_str.clone(), file_content.clone());
    
                    // **Step 1: Markdown Structure Validation**
                    errors.extend(self.validate_markdown_structure(&file_content, &relative_path_str));
    
                    // **Step 2: Parse Elements**
                    match parse_elements(
                        &file_name.to_string_lossy(),
                        &file_content,
                        &file_path.to_path_buf(),
                        specification_folder,
                        external_folders,
                    ) {
                        Ok(elements) => {
                            for element in elements {
                                if let Err(e) = self.element_registry.register_element(element, &relative_path_str) {
                                    errors.push(e);
                                }
                            }
                        }
                        Err(parse_errors) => errors.extend(parse_errors),
                    }
                },
                _ => {                
                    errors.push(ReqFlowError::PathError(
                        format!("File '{}' could not be processed.", path.to_string_lossy()),
                    ));
                }
            }
        }

        // **Step 3: Validate Relations**
        errors.extend(self.validate_relations()?);

        // **Step 4: Validate Cross-Component Dependencies**
        errors.extend(self.validate_cross_component_dependencies()?);

        //self.element_registry.debug_print_registry();
        
        Ok(errors)
    }

    /// Validates relations inside the `ElementRegistry`
    fn validate_relations(&self) -> Result<Vec<ReqFlowError>, ReqFlowError> {
        debug!("Validating relations...");
        let mut errors = Vec::new();

        for element in self.element_registry.get_all_elements() {
            for relation in &element.relations {
            
                // Only validate if the relation target is an Identifier
                if let LinkType::Identifier(ref identifier) = relation.target.link {
                    if self.element_registry.get_element(identifier).is_err() {
                        errors.push(ReqFlowError::MissingRelationTarget(
                            format!("Element '{}' references missing target '{}'", element.identifier, identifier),
                        ));
                    }
                }else{
                    log::debug!("Skipping external target {}",relation.target.link.as_str());
                }            
            }
        }

        if errors.is_empty() {
            debug!("No relation validation errors found.");
        } else {
            debug!("{} relation validation errors found.", errors.len());
        }

        Ok(errors)
    }

    /// Validates cross-component dependencies for circular dependencies and missing links.
    fn validate_cross_component_dependencies(&self) -> Result<Vec<ReqFlowError>, ReqFlowError> {
        debug!("Validating cross-component dependencies...");
        let mut errors = Vec::new();
        let mut visited = HashSet::new();

        // Check for circular dependencies
        for element in self.element_registry.get_all_elements() {
            let mut path = Vec::new();
            self.check_circular_dependencies(element, &mut visited, &mut path, &mut errors);
        }

        // Check for missing parent relations
        let valid_parent_relations = get_parent_relation_types();
        for element in self.element_registry.get_all_elements() {
            // Important: Only system requirements needs parent
            if let ElementType::Requirement(req_type) = &element.element_type {
                match req_type {
                    RequirementType::User => continue,                    
                    RequirementType::System =>{
                    
                        let has_parent_relation = element.relations.iter()
                            .any(|r| valid_parent_relations.contains(&r.relation_type.name));

                        if !has_parent_relation {
                            errors.push(ReqFlowError::MissingParentRelation(
                                format!("Element '{}' has no parent relation (needs one of: {:?})", element.name, valid_parent_relations),
                            ));
                        }                    
                    
                    }
                }
            }

        }

        if errors.is_empty() {
            debug!("No cross-component dependency validation errors found.");
        } else {
            debug!("{} cross-component validation errors found.", errors.len());
        }

        Ok(errors)
    }
    /// Validates the Markdown structure of a document, ensuring:
    /// - Element (`###`) names are unique.
    /// - Subsection (`####`) names are unique within each element.
    /// - Provides error messages with line numbers.
    fn validate_markdown_structure(&self, content: &str, file_path: &str) -> Vec<ReqFlowError> {
        let mut errors = Vec::new();
        let mut element_names = Vec::new();
        let mut current_element: Option<String> = None;
        let mut current_element_subsections: Vec<String> = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            if trimmed.starts_with("### ") {
                // Level 3 header (element)
                let name = trimmed[4..].trim().to_string();

                if element_names.contains(&name) {
                    errors.push(ReqFlowError::DuplicateElement(format!(
                        "File '{}': Duplicate element '{}' found at line {}",
                        file_path, name, line_num + 1
                    )));
                } else {
                    element_names.push(name.clone());
                    current_element = Some(name);
                    current_element_subsections.clear();
                }
            } else if trimmed.starts_with("#### ") && current_element.is_some() {
                // Level 4 header (subsection)
                let subsection_name = trimmed[5..].trim().to_string();

                if current_element_subsections.contains(&subsection_name) {
                    errors.push(ReqFlowError::DuplicateSubsection(format!(
                        "File '{}': Duplicate subsection '{}' within element '{}' found at line {}",
                        file_path,
                        subsection_name,
                        current_element.as_ref().unwrap(),
                        line_num + 1
                    )));
                } else {
                    current_element_subsections.push(subsection_name);
                }
            } else if trimmed.starts_with("## ") || trimmed.starts_with("# ") {
                // Reset current element tracking on higher-level headers
                current_element = None;
                current_element_subsections.clear();
            }
        }

       if errors.is_empty() {
            debug!("No markdown validation errors found.");
        } else {
            debug!("{} markdown validation errors found.", errors.len());
        }
        errors
    }    

    
    /// Recursive function to check for circular dependencies, but only for Identifier links.
    fn check_circular_dependencies(
        &self,
        element: &Element,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
        errors: &mut Vec<ReqFlowError>,
    ) {
        let element_id = element.identifier.clone();
        if visited.contains(&element_id) {
            return;
        }

        if path.contains(&element_id) {
            let cycle_start = path.iter().position(|id| id == &element_id).unwrap();
            let cycle = path[cycle_start..].join(" -> ");
            errors.push(ReqFlowError::ValidationError(
                format!("Circular dependency detected: {}", cycle),
            ));
            return;
        }

        path.push(element_id.clone());

        for relation in &element.relations {
            // Only check relations that have an Identifier as the target
            if let LinkType::Identifier(ref identifier) = relation.target.link {
                if is_circular_dependency_relation(relation.relation_type.name) {
                    if let Ok(target_element) = self.element_registry.get_element(identifier) {
                        self.check_circular_dependencies(target_element, visited, path, errors);
                    }
                }
            }
        }

        visited.insert(element_id);
        path.pop();
    }

}

