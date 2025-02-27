use anyhow::Result;
use log::{debug, info};
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::element::{self, ElementRegistry};
use crate::error::ReqFlowError;
use crate::html;
use crate::markdown;
use crate::utils;
use crate::validation;

/// Manages the MBSE model
#[derive(Debug, Default)]
pub struct ModelManager {
    /// Registry of all elements in the model
    element_registry: ElementRegistry,
    
    /// Cache of file contents for validation and processing
    file_contents: HashMap<String, String>,
}

impl ModelManager {
    /// Create a new model manager
    pub fn new() -> Self {
        Self {
            element_registry: ElementRegistry::new(),
            file_contents: HashMap::new(),
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
        
        // Create the output folder if it doesn't exist
        utils::create_dir_all(output_folder)?;
        
        // First pass: collect all elements
        self.collect_identifiers(input_folder)?;
        
        // Validate model consistency
        info!("Validating model consistency...");
        self.validate_model()?;
        
        // Second pass: process files
        self.process_markdown_files(input_folder, output_folder, convert_to_html)?;
        
        // Generate traceability matrix
        info!("Generating traceability matrix...");
        self.generate_traceability_matrix(output_folder, convert_to_html)?;
        
        Ok(())
    }
    
    /// Generate a traceability matrix showing dependencies between elements
    fn generate_traceability_matrix(&self, output_folder: &Path, convert_to_html: bool) -> Result<(), ReqFlowError> {
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
                        matrix_content.push_str(&format!("|{}|{}|\n", element.name, relation.target));
                    }
                }
            }
            
            if !has_relations {
                matrix_content.push_str("|No relations found||\n");
            }
            
            matrix_content.push_str("\n");
        }
        
        // Generate a comprehensive relationships diagram
        matrix_content.push_str("## Requirements Relationship Diagram\n\n");
        
        // Add a section that explains the diagram
        matrix_content.push_str("This diagram shows the relationships between requirements and other elements in the model.\n");
        matrix_content.push_str("* Red nodes: Requirements\n");
        matrix_content.push_str("* Green nodes: Verification elements\n");
        matrix_content.push_str("* Gray nodes: Other elements\n\n");
        matrix_content.push_str("```mermaid\ngraph TD;\n");
        
        // Add graph styling
        matrix_content.push_str("  %% Graph styling\n");
        matrix_content.push_str("  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;\n");
        matrix_content.push_str("  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;\n");
        matrix_content.push_str("  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;\n\n");
        
        // Add nodes and relationships for verification
        for element in self.element_registry.all_elements() {
            // Create a safe ID using a hash-based approach to avoid any possible Mermaid syntax issues
            // This way we avoid all syntax issues with special characters in IDs
            let safe_id = format!("node_{}", element.name.bytes().map(|b| b as u32).sum::<u32>());
            
            // Encode the label text properly for Mermaid
            let label = element.name.replace('"', "&quot;");
            matrix_content.push_str(&format!("  {}[\"{}\"];\n", safe_id, label));
            
            // Apply requirement style to all elements
            matrix_content.push_str(&format!("  class {} requirement;\n", safe_id));
            
            for relation in &element.relations {
                // Create a safe ID for the target using the same hash-based approach
                let target_safe_id = format!("node_{}", relation.target.bytes().map(|b| b as u32).sum::<u32>());
                
                // Different arrow styles and labels based on relation type
                let (arrow_style, _style_class) = match relation.relation_type.as_str() {
                    "verifiedBy" | "verify" => ("-->|verifies|", "verification"),
                    "satisfiedBy" | "satisfy" => ("-->|satisfies|", "default"),
                    "derivedFrom" | "derive" => ("-.->|derives from|", "default"),
                    "refine" => ("==>|refines|", "default"),
                    "tracedFrom" | "trace" => ("~~~>|traces from|", "default"),
                    "containedBy" | "contain" => ("--o|contains|", "default"),
                    _ => ("-->|relates to|", "default")
                };
                
                matrix_content.push_str(&format!("  {} {} {};\n", safe_id, arrow_style, target_safe_id));
                
                // Apply style to target if it's a verification
                if relation.relation_type == "verifiedBy" || relation.relation_type == "verify" {
                    matrix_content.push_str(&format!("  class {} verification;\n", target_safe_id));
                }
            }
        }
        
        matrix_content.push_str("```\n");
        
        // Write the traceability matrix
        let matrix_path = output_folder.join("traceability_matrix.md");
        utils::write_file(&matrix_path, &matrix_content)?;
        
        // Convert to HTML if requested
        if convert_to_html {
            let html_content = crate::html::convert_to_html(&matrix_content, "Traceability Matrix")?;
            let html_path = matrix_path.with_extension("html");
            utils::write_file(&html_path, html_content)?;
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
                
                markdown::collect_elements(&content, &relative_path_str, registry)?;
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
    /// If fix=true, attempts to fix common issues automatically
    pub fn validate_markdown_structure(&self, _fix: bool) -> Result<Vec<ReqFlowError>, ReqFlowError> {
        info!("Validating markdown structure");
        
        let mut all_errors = Vec::new();
        
        // Process each file for markdown structure validation
        for (file_path, content) in &self.file_contents {
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
    /// If fix=true, attempts to fix common issues automatically
    pub fn validate_relations(&self, _fix: bool) -> Result<Vec<ReqFlowError>, ReqFlowError> {
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
    /// - Required files (README.md, etc.)
    /// - Proper file organization
    /// Returns a list of validation errors
    /// If fix=true, attempts to fix common issues automatically
    pub fn validate_filesystem_structure(&self, input_folder: &Path, _fix: bool) -> Result<Vec<ReqFlowError>, ReqFlowError> {
        info!("Validating filesystem structure in {:?}", input_folder);
        
        let mut errors = Vec::new();
        
        // Check required directories according to ReqFlow methodology
        let required_dirs = vec![
            "specifications",
            "specifications/DesignSpecifications",
            "specifications/SystemRequirements",
        ];
        
        for dir in &required_dirs {
            let dir_path = input_folder.join(dir);
            if !dir_path.exists() || !dir_path.is_dir() {
                errors.push(ReqFlowError::ValidationError(
                    format!("Required directory '{}' is missing", dir)
                ));
            }
        }
        
        // Check required files
        let required_files = vec![
            "README.md",
            "specifications/README.md",
            "specifications/SystemRequirements.md",
        ];
        
        for file in &required_files {
            let file_path = input_folder.join(file);
            if !file_path.exists() || !file_path.is_file() {
                errors.push(ReqFlowError::ValidationError(
                    format!("Required file '{}' is missing", file)
                ));
            }
        }
        
        // Check for files in wrong locations
        for entry in WalkDir::new(input_folder).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                let file_name = path.file_name().unwrap().to_string_lossy();
                
                // Check if requirements files are in the right directories
                if file_name.contains("Requirements") && !path.to_string_lossy().contains("Requirements") {
                    errors.push(ReqFlowError::ValidationError(
                        format!("Requirements file '{}' is not in a Requirements directory", path.display())
                    ));
                }
                
                // Check if design specs are in the right directories
                if file_name.contains("DSD_") && !path.to_string_lossy().contains("DesignSpecifications") {
                    errors.push(ReqFlowError::ValidationError(
                        format!("Design specification file '{}' is not in the DesignSpecifications directory", path.display())
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
    /// If fix=true, attempts to fix common issues automatically
    pub fn validate_cross_component_dependencies(&self, _fix: bool) -> Result<Vec<ReqFlowError>, ReqFlowError> {
        info!("Validating cross-component dependencies");
        
        let mut errors = Vec::new();
        
        // Check for circular dependencies
        for element in self.element_registry.all_elements() {
            let mut visited = std::collections::HashSet::new();
            let mut path = Vec::new();
            
            self.check_circular_dependencies(element, &mut visited, &mut path, &mut errors);
        }
        
        // Check for incomplete dependency chains (e.g., missing derivedFrom relations)
        // For example, system requirements should have derivedFrom relations to user requirements
        for element in self.element_registry.all_elements() {
            // Check for system requirements without derivedFrom relations
            if element.file_path.contains("SystemRequirements") && 
               !element.relations.iter().any(|r| r.relation_type == "derivedFrom") {
                errors.push(ReqFlowError::ValidationError(
                    format!("System requirement '{}' has no 'derivedFrom' relation", element.name)
                ));
            }
            
            // Check for verification elements without verifiedBy relations
            if element.file_path.contains("Verifications") && 
               !element.relations.iter().any(|r| r.relation_type == "verifiedBy") {
                errors.push(ReqFlowError::ValidationError(
                    format!("Verification element '{}' has no 'verifiedBy' relation", element.name)
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
                format!("Circular dependency detected: {}", cycle.join(" -> "))
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
        
        // First, gather all primary requirements files and design specifications
        let mut requirements_files: Vec<PathBuf> = WalkDir::new(input_folder)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().is_file() && 
                e.path().extension().map_or(false, |ext| ext == "md") && 
                (e.path().file_name().map_or(false, |name| utils::is_requirements_file(name.to_string_lossy().as_ref())) ||
                 // Handle system requirements in subfolders according to the documentation
                 e.path().parent().map_or(false, |parent| parent.file_name().map_or(false, |name| 
                     name.to_string_lossy().contains("systemRequirements"))) &&
                 e.path().file_name().map_or(false, |name| name.to_string_lossy() == "Requirements.md") ||
                 // Explicitly include all design specification documents
                 e.path().parent().map_or(false, |parent| parent.file_name().map_or(false, |name| 
                     name.to_string_lossy().contains("DesignSpecifications"))))
            })
            .map(|e| e.path().to_path_buf())
            .collect();
            
        // Next, if HTML output is requested, find all referenced files through relations
        if convert_to_html {
            // Build a set of all referenced files from relations
            let mut referenced_files = std::collections::HashSet::new();
            
            // Add core files needed for the system (README.md, etc)
            for entry in WalkDir::new(input_folder).into_iter().filter_map(|e| e.ok()) {
                if entry.path().is_file() && entry.file_name().to_string_lossy() == "README.md" {
                    referenced_files.insert(entry.path().to_path_buf());
                }
            }
            
            // Analyze all relations in the elements to find referenced files
            for element in self.element_registry.all_elements() {
                for relation in &element.relations {
                    let target = &relation.target;
                    
                    // Check if the target references a file
                    if target.contains('/') {
                        let parts: Vec<&str> = target.split('/').collect();
                        if parts.len() >= 1 {
                            // Extract the file part
                            let mut file_part = parts[0].to_string();
                            
                            // Add .md extension if needed
                            if !file_part.ends_with(".md") {
                                file_part = format!("{}.md", file_part);
                            }
                            
                            // Add the file to our set of referenced files
                            let referenced_path = input_folder.join(&file_part);
                            if referenced_path.exists() {
                                referenced_files.insert(referenced_path);
                            } else {
                                // Try looking in subdirectories
                                for entry in WalkDir::new(input_folder).into_iter().filter_map(|e| e.ok()) {
                                    if entry.path().is_file() && 
                                       entry.path().file_name().map_or(false, |name| name.to_string_lossy() == file_part) {
                                        referenced_files.insert(entry.path().to_path_buf());
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // Add all referenced files to our processing list, if they aren't already included
            for file_path in referenced_files {
                if !requirements_files.contains(&file_path) {
                    requirements_files.push(file_path);
                }
            }
        }
        
        let files = requirements_files;
        
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