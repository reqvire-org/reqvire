use anyhow::Result;
use log::{debug, info};
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::Config;
use crate::element::{self, ElementRegistry};
use crate::error::ReqFlowError;
use crate::html;
use crate::markdown;
use crate::utils;
use crate::validation;

/// Manages the MBSE model
#[derive(Debug)]
pub struct ModelManager {
    /// Registry of all elements in the model
    element_registry: ElementRegistry,
    
    /// Cache of file contents for validation and processing
    file_contents: HashMap<String, String>,
    
    /// Configuration settings
    config: Config,
}

impl Default for ModelManager {
    fn default() -> Self {
        Self {
            element_registry: ElementRegistry::new(),
            file_contents: HashMap::new(),
            config: Config::default(),
        }
    }
}

impl ModelManager {
    /// Create a new model manager with default configuration
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            element_registry: ElementRegistry::new(),
            file_contents: HashMap::new(),
            config: Config::default(),
        }
    }
    
    /// Create a new model manager with a provided configuration
    pub fn new_with_config(config: Config) -> Self {
        Self {
            element_registry: ElementRegistry::new(),
            file_contents: HashMap::new(),
            config,
        }
    }
    
    /// Process all files in the input folder and write to the output folder
    pub fn process_files(
        &mut self,
        input_folder: &Path,
        output_folder: &Path,
        convert_to_html: bool,
    ) -> Result<(), ReqFlowError> {
        info!("Processing files from {:?} to {:?}", input_folder, output_folder);
        
        // Update HTML output setting in config if specified
        if convert_to_html {
            self.config.general.html_output = true;
        }
        
        // Create the output folder if it doesn't exist
        utils::create_dir_all(output_folder)?;
        
        // First pass: collect all elements
        self.collect_identifiers(input_folder)?;
        
        // Validate model consistency
        info!("Validating model consistency...");
        self.validate_model()?;
        
        // Second pass: process files
        self.process_markdown_files(input_folder, output_folder, self.config.general.html_output)?;
        
        // Note: Traceability matrix is now generated only when explicitly requested
        // with the --generate-matrix flag
        
        Ok(())
    }
    
    /// Generate a traceability matrix showing dependencies between elements
    /// The matrix is saved to the specifications root directory (input_folder)
    /// If convert_to_html is true, HTML will be saved to output_folder
    pub fn generate_traceability_matrix(&self, input_folder: &Path, output_folder: &Path, convert_to_html: bool) -> Result<(), ReqFlowError> {
        // Debug information to help diagnose the issue
        info!("Generating traceability matrix with {} elements", self.element_registry.all_elements().count());
        
        // Create a simple traceability matrix as markdown
        let mut matrix_content = String::from("# Traceability Matrix\n\n");
        matrix_content.push_str("This matrix shows the relationships between elements in the model.\n\n");
        
        // Add summary information
        matrix_content.push_str("## Summary\n\n");
        matrix_content.push_str(&format!("- Total elements: {}\n", self.element_registry.all_elements().count()));
        
        // Count relations by type for summary with detailed debugging
        let mut relation_count: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        let mut element_count = 0;
        let mut elements_with_relations = 0;
        
        for element in self.element_registry.all_elements() {
            element_count += 1;
            info!("Checking element '{}' for relations - has {} relations", 
                  element.name, element.relations.len());
                  
            if !element.relations.is_empty() {
                elements_with_relations += 1;
                info!("Element '{}' has the following relations:", element.name);
                
                for relation in &element.relations {
                    info!("  - {}: {}", relation.relation_type, relation.target);
                    *relation_count.entry(relation.relation_type.clone()).or_insert(0) += 1;
                }
            }
        }
        
        info!("Found {} elements total, {} with relations", element_count, elements_with_relations);
        
        if !relation_count.is_empty() {
            matrix_content.push_str("- Relations by type:\n");
            for (rel_type, count) in relation_count.iter() {
                matrix_content.push_str(&format!("  - {}: {}\n", rel_type, count));
            }
        } else {
            matrix_content.push_str("- No relations found in the model\n");
        }
        matrix_content.push_str("\n");
        
        // List of relation types to include based on ReqFlow documentation
        let relation_types = vec!["verifiedBy", "verify", "satisfiedBy", "satisfy", 
                               "derivedFrom", "derive", "refine", "tracedFrom", 
                               "trace", "containedBy", "contain"];
        
        for relation_type in relation_types {
            matrix_content.push_str(&format!("## {} Relations\n\n", relation_type));
            matrix_content.push_str("|Source Element|Target Element|\n");
            matrix_content.push_str("|-------------|-------------|\n");
            
            let mut has_relations = false;
            
            // Find all relations of this type
            for element in self.element_registry.all_elements() {
                for relation in &element.relations {
                    if relation.relation_type == relation_type {
                        has_relations = true;
                        
                        // Create clickable links for HTML output
                        let source_element = if convert_to_html {
                            let file_path = element.file_path.replace(".md", ".html");
                            let anchor = element.name.replace(' ', "-").to_lowercase();
                            format!("[{}]({}#{})", element.name, file_path, anchor)
                        } else {
                            element.name.clone()
                        };
                        
                        // Format the target with proper links if needed
                        let target = &relation.target;
                        let target_element = if convert_to_html && target.contains('/') {
                            // This is a reference to another file, possibly with an element
                            if target.contains(".md/") {
                                // Handle path/file.md/element format
                                let parts: Vec<&str> = target.split("/").collect();
                                let file_path = parts[0..parts.len()-1].join("/").replace(".md", ".html");
                                let element_name = parts.last().unwrap();
                                let anchor = element_name.replace(' ', "-").to_lowercase();
                                format!("[{}]({}#{})", target, file_path, anchor)
                            } else {
                                // Simple file reference
                                let file_path = target.replace(".md", ".html");
                                format!("[{}]({})", target, file_path)
                            }
                        } else {
                            target.clone()
                        };
                        
                        matrix_content.push_str(&format!("|{}|{}|\n", source_element, target_element));
                    }
                }
            }
            
            if !has_relations {
                matrix_content.push_str("|No relations found||\n");
            }
            
            matrix_content.push_str("\n");
        }
        
        // Organize requirements by file for diagram generation
        let mut files_with_elements: std::collections::HashMap<String, Vec<&element::Element>> = std::collections::HashMap::new();
        
        for element in self.element_registry.all_elements() {
            files_with_elements.entry(element.file_path.clone())
                .or_insert_with(Vec::new)
                .push(element);
        }
        
        // Generate diagrams by file
        matrix_content.push_str("## Requirements Relationship Diagrams\n\n");
        matrix_content.push_str("These diagrams show the relationships between requirements organized by file.\n\n");
        matrix_content.push_str("* Red nodes: Requirements\n");
        matrix_content.push_str("* Green nodes: Verification elements\n");
        matrix_content.push_str("* Gray nodes: Other elements\n\n");
        
        // Iterate through files and create a diagram for each
        for (file_path, elements) in files_with_elements.iter() {
            // Only generate diagrams for files that have elements with relations
            let has_relations = elements.iter().any(|e| !e.relations.is_empty());
            
            if has_relations {
                // Create a header for this file's diagram
                let file_name = if let Some(idx) = file_path.rfind('/') {
                    &file_path[idx + 1..]
                } else {
                    file_path
                };
                
                matrix_content.push_str(&format!("### Diagram for {}\n\n", file_name));
                matrix_content.push_str("```mermaid\ngraph LR;\n");
                
                // Add graph styling
                matrix_content.push_str("  %% Graph styling\n");
                matrix_content.push_str("  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;\n");
                matrix_content.push_str("  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;\n");
                matrix_content.push_str("  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;\n\n");
                
                // Create a set of elements already included in this diagram
                let mut included_elements = std::collections::HashSet::new();
                
                // Add nodes for this file's elements
                for element in elements {
                    // Safe ID for element
                    let element_id = element.name.replace(' ', "_").replace('-', "_")
                        .replace('/', "_").replace(':', "_").replace("'", "")
                        .replace('(', "_").replace(')', "_").replace(".", "_");
                    
                    // Encode the label text properly for Mermaid
                    let label = element.name.replace('"', "&quot;");
                    
                    matrix_content.push_str(&format!("  {}[\"{}\"];\n", element_id, label));
                    
                    // Add click behavior for HTML
                    let html_file = if convert_to_html {
                        file_path.replace(".md", ".html")
                    } else {
                        file_path.clone()
                    };
                    
                    matrix_content.push_str(&format!("  click {} \"{}\";", 
                        element_id, 
                        format!("{}#{}", html_file, element.name.replace(' ', "-").to_lowercase())
                    ));
                    matrix_content.push_str("\n");
                    
                    // Apply requirement style to all elements in this file
                    matrix_content.push_str(&format!("  class {} requirement;\n", element_id));
                    
                    // Mark this element as included
                    included_elements.insert(element.name.clone());
                    
                    // Process relations for this element
                    for relation in &element.relations {
                        // Get target element
                        let target = &relation.target;
                        
                        // Generate safe ID for target
                        let target_id = target.replace(' ', "_").replace('-', "_")
                            .replace('/', "_").replace(':', "_").replace("'", "")
                            .replace('(', "_").replace(')', "_").replace(".", "_");
                        
                        // Different arrow styles and labels based on relation type
                        let (arrow_style, style_class) = match relation.relation_type.as_str() {
                            "verifiedBy" | "verify" => ("-->|verifies|", "verification"),
                            "satisfiedBy" | "satisfy" => ("-->|satisfies|", "default"),
                            "derivedFrom" | "derive" => ("-.->|derives from|", "default"),
                            "refine" => ("==>|refines|", "default"),
                            "tracedFrom" | "trace" => ("~~~>|traces from|", "default"),
                            "containedBy" | "contain" => ("--o|contains|", "default"),
                            _ => ("-->|relates to|", "default")
                        };
                        
                        // Add the relationship
                        matrix_content.push_str(&format!("  {} {} {};\n", element_id, arrow_style, target_id));
                        
                        // If the target is not already included in the diagram, add it
                        if !included_elements.contains(target) {
                            // Add the target node
                            matrix_content.push_str(&format!("  {}[\"{}\"];\n", target_id, target.replace('"', "&quot;")));
                            
                            // Try to find the target element in the registry
                            let target_element = self.element_registry.get_element(target);
                            
                            if let Some(target_elem) = target_element {
                                // Add click behavior for HTML if we found the target
                                let target_html_file = if convert_to_html {
                                    target_elem.file_path.replace(".md", ".html")
                                } else {
                                    target_elem.file_path.clone()
                                };
                                
                                matrix_content.push_str(&format!("  click {} \"{}\";\n", 
                                    target_id, 
                                    format!("{}#{}", target_html_file, target_elem.name.replace(' ', "-").to_lowercase())
                                ));
                            } else if target.contains('/') {
                                // This might be a file reference
                                // Extract file path and element name if present
                                let parts: Vec<&str> = target.split('/').collect();
                                let file_part = if parts.len() > 1 {
                                    let mut path = String::new();
                                    for i in 0..parts.len()-1 {
                                        if i > 0 { path.push('/'); }
                                        path.push_str(parts[i]);
                                    }
                                    if convert_to_html {
                                        path.replace(".md", ".html")
                                    } else {
                                        path
                                    }
                                } else {
                                    let path = parts[0].to_string();
                                    if convert_to_html {
                                        path.replace(".md", ".html")
                                    } else {
                                        path
                                    }
                                };
                                
                                let element_part = if parts.len() > 1 {
                                    parts[parts.len()-1]
                                } else {
                                    ""
                                };
                                
                                let link = if !element_part.is_empty() {
                                    format!("{}#{}", file_part, element_part.replace(' ', "-").to_lowercase())
                                } else {
                                    file_part
                                };
                                
                                matrix_content.push_str(&format!("  click {} \"{}\";\n", target_id, link));
                            }
                            
                            // Apply appropriate style class
                            if style_class == "verification" {
                                matrix_content.push_str(&format!("  class {} verification;\n", target_id));
                            } else {
                                matrix_content.push_str(&format!("  class {} default;\n", target_id));
                            }
                            
                            // Mark target as included
                            included_elements.insert(target.clone());
                        }
                    }
                }
                
                // Close the mermaid diagram
                matrix_content.push_str("```\n\n");
            }
        }
        
        // Write the traceability matrix to the specifications root directory
        let matrix_path = input_folder.join("TraceabilityMatrix.md");
        utils::write_file(&matrix_path, &matrix_content)?;
        
        // Convert to HTML if requested - HTML goes to the output folder
        if convert_to_html {
            let html_content = crate::html::convert_to_html(&matrix_content, "Traceability Matrix")?;
            
            // Create output folder path for HTML
            let html_path = output_folder.join("TraceabilityMatrix.html");
            utils::write_file(&html_path, html_content)?;
            
            info!("Traceability matrix HTML saved to {:?}", html_path);
        }
        
        info!("Traceability matrix generated");
        Ok(())
    }
    
    /// Validate the model for consistency
    fn validate_model(&self) -> Result<(), ReqFlowError> {
        use crate::validation::{validate_relations, validate_markdown_structure};
        
        // Validate relations
        let relation_errors = validate_relations(&self.element_registry)?;
        
        if !relation_errors.is_empty() {
            info!("Found {} relation validation errors", relation_errors.len());
            for error in &relation_errors {
                info!("Validation error: {}", error);
            }
            // For now, we'll just log errors but continue processing
            // In a production implementation, we might want to fail or handle differently
        } else {
            info!("No relation validation errors found");
        }
        
        // Validate markdown structure for each file
        let mut total_structure_errors = 0;
        for element in self.element_registry.all_elements() {
            let file_path = &element.file_path;
            // Group validation by file path to avoid redundant validations
            if let Some(content) = self.file_contents.get(file_path) {
                let structure_errors = validate_markdown_structure(content)?;
                if !structure_errors.is_empty() {
                    info!("Found {} structure validation errors in {}", structure_errors.len(), file_path);
                    for error in &structure_errors {
                        info!("Structure error: {}", error);
                    }
                    total_structure_errors += structure_errors.len();
                }
            }
        }
        
        if total_structure_errors > 0 {
            info!("Total of {} structure validation errors found", total_structure_errors);
        } else {
            info!("No structure validation errors found");
        }
        
        Ok(())
    }
    
    /// Collect identifiers from all markdown files in the input folder
    fn collect_identifiers(&mut self, input_folder: &Path) -> Result<(), ReqFlowError> {
        info!("Collecting identifiers from markdown files");
        
        // Use existing registry
        let registry = &mut self.element_registry;
        
        // Clear the registry first to avoid duplicates
        // (not ideal for incremental updates, but ensures clean collection)
        *registry = ElementRegistry::new();
        
        for entry in WalkDir::new(input_folder).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            
            // Process all potentially relevant files to collect identifiers
            if path.is_file() && path.file_name().map_or(false, |name| utils::is_processable_file(name.to_string_lossy().as_ref())) {
                
                debug!("Collecting identifiers from {:?}", path);
                
                let content = utils::read_file(path)?;
                let relative_path = utils::get_relative_path(path, input_folder)?;
                let relative_path_str = relative_path.to_string_lossy().to_string();
                
                // Store file content for later validation and processing
                self.file_contents.insert(relative_path_str.clone(), content.clone());
                
                // Skip collecting elements from Design Specification Documents
                // DSDs are identified by being in any folder that matches the design_specifications_folder name
                let design_specs_folder_name = &self.config.paths.design_specifications_folder;
                
                // Check if any component of the path matches the design specs folder name
                let is_dsd = relative_path_str.split('/').any(|component| component == design_specs_folder_name);
                
                if !is_dsd {
                    // Only collect elements from non-DSD documents
                    markdown::collect_elements(&content, &relative_path_str, registry)?;
                } else {
                    debug!("Skipping element collection for Design Specification Document: {}", relative_path_str);
                }
            }
        }
        
        Ok(())
    }
    
    /// Public method to collect identifiers only - used for validation
    pub fn collect_identifiers_only(&mut self, input_folder: &Path) -> Result<(), ReqFlowError> {
        self.collect_identifiers(input_folder)
    }
    
    /// Validate markdown structure of all collected files
    /// Returns a list of validation errors
    pub fn validate_markdown_structure(&self) -> Result<Vec<ReqFlowError>, ReqFlowError> {
        info!("Validating markdown structure");
        
        let mut all_errors = Vec::new();
        
        // Process each file for markdown structure validation
        for (file_path, content) in &self.file_contents {
            // Skip validation for Design Specification Documents
            // DSDs are identified by being in any folder that matches the design_specifications_folder name
            let design_specs_folder_name = &self.config.paths.design_specifications_folder;
            
            // Check if any component of the path matches the design specs folder name
            let is_dsd = file_path.split('/').any(|component| component == design_specs_folder_name);
            
            if is_dsd {
                info!("Skipping validation for Design Specification Document: {}", file_path);
                continue;
            }
            
            info!("Validating markdown structure of {}", file_path);
            
            // Use existing validation function
            let errors = validation::validate_markdown_structure(content)?;
            
            // Convert errors to include file path in message
            let errors_with_path: Vec<ReqFlowError> = errors
                .into_iter()
                .map(|error| {
                    match error {
                        ReqFlowError::DuplicateElement(msg) => {
                            ReqFlowError::DuplicateElement(format!("File {}: {}", file_path, msg))
                        },
                        ReqFlowError::DuplicateSubsection(msg) => {
                            ReqFlowError::DuplicateSubsection(format!("File {}: {}", file_path, msg))
                        },
                        ReqFlowError::InvalidMarkdownStructure(msg) => {
                            ReqFlowError::InvalidMarkdownStructure(format!("File {}: {}", file_path, msg))
                        },
                        other => other,
                    }
                })
                .collect();
            
            all_errors.extend(errors_with_path);
        }
        
        if all_errors.is_empty() {
            info!("No markdown structure validation errors found");
        } else {
            info!("Found {} markdown structure validation errors", all_errors.len());
        }
        
        Ok(all_errors)
    }
    
    /// Validate all relations in the model
    /// Returns a list of validation errors
    pub fn validate_relations(&self) -> Result<Vec<ReqFlowError>, ReqFlowError> {
        info!("Validating relations");
        
        // Use existing validation function
        let errors = validation::validate_relations(&self.element_registry)?;
        
        if errors.is_empty() {
            info!("No relation validation errors found");
        } else {
            info!("Found {} relation validation errors", errors.len());
        }
        
        Ok(errors)
    }
    
    /// Validate filesystem structure according to ReqFlow methodology
    /// Checks for:
    /// - Required directories (specifications, etc.)
    /// - Required files
    /// - Proper file organization
    /// Returns a list of validation errors with file paths and line numbers (where applicable)
    pub fn validate_filesystem_structure(&self, input_folder: &Path, config: &Config) -> Result<Vec<ReqFlowError>, ReqFlowError> {
        info!("Validating filesystem structure in {:?}", input_folder);
        
        let mut errors = Vec::new();
        
        // Get configuration values for folders
        let specs_folder = &config.paths.specifications_folder;
        
        // Get paths from config using helper methods, but use them correctly
        // These paths should all be within the input folder, not absolute or in another repository
        let specs_dir_path = input_folder.join(specs_folder);
        let system_reqs_dir_path = specs_dir_path.join(&config.paths.system_requirements_folder);
        let design_specs_dir_path = specs_dir_path.join(&config.paths.design_specifications_folder);
        
        // Special case for testing - if input_folder is already "specifications", don't add it again
        let actual_specs_dir_path = if input_folder.file_name().map_or(false, |name| name == "specifications") {
            input_folder.to_path_buf()
        } else {
            specs_dir_path.clone()
        };
        
        // Special case for testing - adapt system and design specs paths if needed
        let actual_system_reqs_dir_path = if input_folder.file_name().map_or(false, |name| name == "specifications") {
            input_folder.join(&config.paths.system_requirements_folder)
        } else {
            system_reqs_dir_path.clone()  
        };
        
        let actual_design_specs_dir_path = if input_folder.file_name().map_or(false, |name| name == "specifications") {
            input_folder.join(&config.paths.design_specifications_folder)
        } else {
            design_specs_dir_path.clone()
        };
        
        // Check only the primary directories within the input folder
        if !actual_specs_dir_path.exists() || !actual_specs_dir_path.is_dir() {
            errors.push(ReqFlowError::ValidationError(
                format!("Required specifications directory '{}' is missing", actual_specs_dir_path.display())
            ));
        } else {
            // Only check these subdirectories if the main specs directory exists
            if !actual_system_reqs_dir_path.exists() || !actual_system_reqs_dir_path.is_dir() {
                errors.push(ReqFlowError::ValidationError(
                    format!("Required system requirements directory '{}' is missing", actual_system_reqs_dir_path.display())
                ));
            }
            
            if !actual_design_specs_dir_path.exists() || !actual_design_specs_dir_path.is_dir() {
                errors.push(ReqFlowError::ValidationError(
                    format!("Required design specifications directory '{}' is missing", actual_design_specs_dir_path.display())
                ));
            }
        }
        
        // No README validation - README files are optional
        
        // Check SystemRequirements.md file if the system reqs directory exists
        if system_reqs_dir_path.exists() && system_reqs_dir_path.is_dir() {
            let system_reqs_file_path = system_reqs_dir_path.with_extension("md");
            if !system_reqs_file_path.exists() || !system_reqs_file_path.is_file() {
                errors.push(ReqFlowError::ValidationError(
                    format!("Required system requirements file '{}' is missing", system_reqs_file_path.display())
                ));
            }
        }
        
        // Check for files in wrong locations
        for entry in WalkDir::new(input_folder).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                let file_name = path.file_name().unwrap().to_string_lossy();
                
                // Check if the file is a Requirements file - we use the requirements_filename_match parameter from config
                let requirements_match = &config.paths.requirements_filename_match;
                let in_specs_root = path.parent().map_or(false, |p| p == actual_specs_dir_path);
                let is_requirements_file = file_name.contains(requirements_match);
                
                // User requirements are in specs root, system requirements are in system_requirements_folder
                // A file is considered a User Requirements file if it:
                // 1. Contains the requirements_filename_match string
                // 2. Is directly in the specifications root folder 
                let is_user_req_file = is_requirements_file && in_specs_root;
                
                // A file is considered a System Requirements file if it:
                // 1. Contains the requirements_filename_match string
                // 2. Is NOT directly in the specifications root folder (meaning it's either in a subfolder or outside specs)
                let is_system_req_file = is_requirements_file && !is_user_req_file;
                
                // User Requirements should be in the specifications root folder
                if is_user_req_file && !in_specs_root {
                    errors.push(ReqFlowError::ValidationError(
                        format!("File '{}': User Requirements file must be in the specifications root folder '{}'", 
                                path.display(), actual_specs_dir_path.display())
                    ));
                }
                
                // System Requirements should be in the System Requirements directory
                if path.starts_with(&actual_system_reqs_dir_path) {
                    // All files in this directory are considered system requirements
                    // No need for additional validation
                } else if is_system_req_file && !path.starts_with(&actual_system_reqs_dir_path) {
                    // DSD files get special treatment - don't flag them as misplaced system requirements
                    let is_dsd_file = file_name.contains("DSD_") || path.starts_with(&actual_design_specs_dir_path);
                    
                    if !is_dsd_file {
                        // For backward compatibility, flag System Requirements files not in the system requirements directory
                        errors.push(ReqFlowError::ValidationError(
                            format!("File '{}': System Requirements file should be in the System Requirements directory '{}'", 
                                    path.display(), actual_system_reqs_dir_path.display())
                        ));
                    }
                }
                
                // Check if files in the Design Specifications directory
                if path.starts_with(&actual_design_specs_dir_path) {
                    // We don't need to validate that DSD files have the "DSD_" prefix anymore
                    // All files in this directory are considered design specifications
                } else if file_name.contains("DSD_") {
                    // If a file has DSD_ prefix but is not in the design specs directory,
                    // that's still an error for backward compatibility
                    errors.push(ReqFlowError::ValidationError(
                        format!("File '{}': Design specification file is not in the Design Specifications directory '{}'", 
                                path.display(), actual_design_specs_dir_path.display())
                    ));
                }
            }
        }
        
        if errors.is_empty() {
            info!("No filesystem structure validation errors found");
        } else {
            info!("Found {} filesystem structure validation errors", errors.len());
        }
        
        Ok(errors)
    }
    
    /// Validate cross-component dependencies
    /// Checks for:
    /// - Complete dependency chains
    /// - Missing components in dependency chains
    /// - Circular dependencies
    /// Returns a list of validation errors
    pub fn validate_cross_component_dependencies(&self) -> Result<Vec<ReqFlowError>, ReqFlowError> {
        info!("Validating cross-component dependencies");
        
        let mut errors = Vec::new();
        
        // Check for circular dependencies
        for element in self.element_registry.all_elements() {
            let mut visited = std::collections::HashSet::new();
            let mut path = Vec::new();
            
            self.check_circular_dependencies(element, &mut visited, &mut path, &mut errors);
        }
        
        // Get system requirements folder name from config
        let system_reqs_folder = &self.config.paths.system_requirements_folder;
        
        // Check for incomplete dependency chains (e.g., missing derivedFrom relations)
        // For example, system requirements should have derivedFrom relations to user requirements
        for element in self.element_registry.all_elements() {
            // Check for system requirements without any parent relation
            // System requirements must have at least one relation that points to a parent requirement
            if element.file_path.contains(system_reqs_folder) {
                // List of valid parent-child relationship types
                let valid_parent_relations = ["derivedFrom", "tracedFrom", "refine", "containedBy"];
                
                // Check if the element has at least one valid parent relation
                let has_parent_relation = element.relations.iter().any(|r| valid_parent_relations.contains(&r.relation_type.as_str()));
                
                if !has_parent_relation {
                    errors.push(ReqFlowError::ValidationError(
                        format!("System requirement '{}' has no parent requirement relation (needs 'derivedFrom', 'tracedFrom', 'refine', or 'containedBy') (in file '{}')", 
                               element.name, element.file_path)
                    ));
                }
            }
            
            // Check for verification elements without verifiedBy relations
            if element.file_path.contains("Verifications") && 
               !element.relations.iter().any(|r| r.relation_type == "verifiedBy") {
                errors.push(ReqFlowError::ValidationError(
                    format!("Verification element '{}' has no 'verifiedBy' relation (in file '{}')", 
                           element.name, element.file_path)
                ));
            }
        }
        
        if errors.is_empty() {
            info!("No cross-component dependency validation errors found");
        } else {
            info!("Found {} cross-component dependency validation errors", errors.len());
        }
        
        Ok(errors)
    }
    
    /// Helper method to check for circular dependencies
    fn check_circular_dependencies(
        &self, 
        element: &element::Element, 
        visited: &mut std::collections::HashSet<String>,
        path: &mut Vec<String>,
        errors: &mut Vec<ReqFlowError>
    ) {
        let element_id = element.identifier();
        
        // If we've already completely processed this element, skip
        if visited.contains(&element_id) {
            return;
        }
        
        // Check if we've encountered this element before in the current path
        if path.contains(&element_id) {
            // We have a cycle
            let cycle_start = path.iter().position(|id| id == &element_id).unwrap();
            let mut cycle = path[cycle_start..].to_vec();
            cycle.push(element_id.clone());
            
            errors.push(ReqFlowError::ValidationError(
                format!("Circular dependency detected: {} (in file '{}', element '{}')", 
                       cycle.join(" -> "), element.file_path, element.name)
            ));
            return;
        }
        
        // Add this element to the current path
        path.push(element_id.clone());
        
        // Check all relations of this element
        for relation in &element.relations {
            // Only follow certain relation types (those that could create cycles)
            if ["derivedFrom", "dependsOn", "refine", "tracedFrom"].contains(&relation.relation_type.as_str()) {
                // Get the target element
                if let Some(target_element) = self.element_registry.get_element(&relation.target) {
                    self.check_circular_dependencies(target_element, visited, path, errors);
                }
            }
        }
        
        // Mark this element as visited and remove from current path
        visited.insert(element_id);
        path.pop();
    }
    
    /// Process all markdown files and write to the output folder
    fn process_markdown_files(
        &self,
        input_folder: &Path,
        output_folder: &Path,
        convert_to_html: bool,
    ) -> Result<(), ReqFlowError> {
        info!("Processing markdown files");
        
        // Collect files to process based on mode
        let files: Vec<PathBuf>;
        
        if convert_to_html {
            // For HTML conversion, get ALL markdown files in the input folder and its subfolders
            info!("HTML mode: Processing all markdown files in all subfolders");
            files = WalkDir::new(input_folder)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    let file_path = e.path();
                    file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "md")
                })
                .map(|e| e.path().to_path_buf())
                .collect();
                
            info!("Found {} markdown files to convert to HTML", files.len());
        } else {
            // Regular mode: Process only requirements files
            info!("Regular mode: Processing only requirements files");
            // First, gather all primary requirements files and design specifications
            files = WalkDir::new(input_folder)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    let file_path = e.path();
                    
                    if !file_path.is_file() || file_path.extension().map_or(true, |ext| ext != "md") {
                        return false;
                    }
                    
                    // Use the path-based detection that respects configuration
                    utils::is_requirements_file_by_path(file_path, &self.config, input_folder)
                })
                .map(|e| e.path().to_path_buf())
                .collect();
                
            info!("Found {} requirements files to process", files.len());
        }
        
        // Process files in parallel
        files.par_iter().try_for_each(|file_path| {
            debug!("Processing file {:?}", file_path);
            
            let content = utils::read_file(file_path)?;
            let relative_path = utils::get_relative_path(file_path, input_folder)?;
            
            // Replace relations with markdown links
            let updated_content = markdown::replace_relations(
                &content,
                &self.element_registry,
                &relative_path,
                convert_to_html, // Pass the convert_to_html flag
            )?;
            
            // Determine output file path and format
            let output_file = if convert_to_html {
                let file_name = file_path.file_name().unwrap().to_string_lossy();
                let title = file_name.replace(".md", "");
                
                let html_content = html::convert_to_html(&updated_content, &title)?;
                
                let mut html_path = output_folder.join(relative_path);
                html_path.set_extension("html");
                
                utils::write_file(&html_path, html_content)?;
                html_path.clone()
            } else {
                let output_path = output_folder.join(relative_path);
                utils::write_file(&output_path, updated_content)?;
                output_path.clone()
            };
            
            debug!("Wrote output to {:?}", output_file);
            Ok::<(), ReqFlowError>(())
        })?;
        
        info!("Finished processing all files");
        Ok(())
    }
}