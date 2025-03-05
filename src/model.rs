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
    
    /// Get a reference to the configuration
    pub fn config(&self) -> &Config {
        &self.config
    }
    
    /// Process diagram generation for markdown files in place (without writing to output)
    /// This method is used when the --generate-diagrams flag is set
    /// Note: This method deliberately skips validation to focus only on diagram generation
    pub fn process_diagrams(&mut self, input_folder: &Path) -> Result<(), ReqFlowError> {
        debug!("Processing diagrams for markdown files in {:?} (skipping validation)", input_folder);
        
        // Set the diagram generation flag in the element registry
        // This will be checked by markdown.rs when processing the files
        self.element_registry.set_diagram_generation_enabled(true);
        // Pass the configuration to the registry
        self.element_registry.set_config(self.config.clone());
        
        // Manually collect key requirements files first, since the automated detection
        // can sometimes have issues with specifications/ as the base path
        let mut files = Vec::new();
        
        // We now process all markdown files as potential element containers
        
        // Instead of hardcoding filenames, find requirements files using the utility functions
        // First, look directly in the input folder for any matching requirement files
        if let Ok(entries) = std::fs::read_dir(input_folder) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                    // Use the proper config-driven utility function
                    if crate::utils::is_requirements_file_by_path(&path, &self.config, input_folder) {
                        debug!("Adding requirements file from input folder: {:?}", path);
                        files.push(path);
                    }
                }
            }
        }
        
        // Then also check specifications/ subfolder if input_folder doesn't already end with 'specifications'
        let input_path_str = input_folder.to_string_lossy().to_lowercase();
        if !input_path_str.ends_with("/specifications") && !input_path_str.ends_with("\\specifications") {
            let specs_folder = input_folder.join(&self.config.paths.specifications_folder);
            if specs_folder.exists() && specs_folder.is_dir() {
                if let Ok(entries) = std::fs::read_dir(&specs_folder) {
                    for entry in entries.filter_map(|e| e.ok()) {
                        let path = entry.path();
                        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                            // Use the proper config-driven utility function 
                            if crate::utils::is_requirements_file_by_path(&path, &self.config, input_folder) {
                                debug!("Adding requirements file from specifications folder: {:?}", path);
                                files.push(path);
                            }
                        }
                    }
                }
            }
        }
        
        // We no longer have a specific system requirements folder
        // All files in specifications subfolders are processed automatically

        // Process external folders
        for external_folder in &self.config.paths.external_folders {
            let folder_path = if Path::new(external_folder).is_absolute() {
                PathBuf::from(external_folder)
            } else {
                input_folder.join(external_folder)
            };

            if folder_path.exists() && folder_path.is_dir() {
                debug!("Processing external folder: {:?}", folder_path);
                for entry in WalkDir::new(&folder_path).into_iter().filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                        // Process all markdown files in external folders
                        debug!("Adding file from external folder: {:?}", path);
                        files.push(path.to_path_buf());
                    }
                }
            } else {
                debug!("External folder not found or not a directory: {:?}", folder_path);
            }
        }
        
        // As a final fallback, also add files detected by the normal mechanism
        let detected_files: Vec<PathBuf> = WalkDir::new(input_folder)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                utils::is_requirements_file_only(
                    e.path(), 
                    &self.config, 
                    input_folder, 
                    self.config.general.verbose
                )
            })
            .map(|e| e.path().to_path_buf())
            .collect();
        
        // Add any additional files found by the normal detection mechanism
        for file in detected_files {
            if !files.contains(&file) {
                debug!("Adding detected requirements file: {:?}", file);
                files.push(file);
            }
        }
            
        debug!("Found {} requirements files to update with diagrams", files.len());
        
        // Note: No need to modify the configuration as diagram generation
        // is triggered by the --generate-diagrams flag in main.rs
        
        // Process files in parallel and track which ones were updated
        let results = files.par_iter().map(|file_path| {
            let result: Result<bool, ReqFlowError> = (|| {
                debug!("Generating diagram for file {:?}", file_path);
                
                let content = utils::read_file(file_path)?;
                let relative_path = utils::get_relative_path(file_path, input_folder)?;
                
                // Remove any existing diagrams first to ensure clean generation
                let mermaid_regex = regex::Regex::new(r"(?s)```mermaid\s*graph (TD|LR);.*?```\s*").unwrap();
                let content_without_diagrams = mermaid_regex.replace_all(&content, "").to_string();
                
                // Pass the diagram_config value to ensure diagrams are generated
                // Update content with mermaid diagrams using our modified config that has diagram generation enabled
                log::debug!("FILE CONTENT BEFORE DIAGRAM GENERATION:");
                log::debug!("{}", &content_without_diagrams);
                
                let updated_content = markdown::replace_relations(
                    &content_without_diagrams,
                    &self.element_registry,
                    &relative_path,
                    false, // Not converting to HTML
                )?;
                
                log::debug!("FILE CONTENT AFTER DIAGRAM GENERATION:");
                log::debug!("{}", &updated_content);
                
                // Always update the file in diagram generation mode, even if there aren't apparent changes
                debug!("Updating {:?} with mermaid diagram", file_path);
                
                // Debug info to compare content before and after
                if content != updated_content {
                    debug!("Content differs, updating file");
                    debug!("Content lengths - before: {}, after: {}", content.len(), updated_content.len());
                    if content.len() >= 100 && updated_content.len() >= 100 {
                        debug!("First 100 chars - before: {}", &content[..100]);
                        debug!("First 100 chars - after: {}", &updated_content[..100]);
                    }
                } else {
                    // Even when content appears the same, force addition of diagram
                    // by appending a newline to make them different
                    let force_updated_content = updated_content + "\n";
                    debug!("No content changes detected, forcing update with newline");
                    utils::write_file(file_path, &force_updated_content)?;
                    return Ok(true);
                }
                
                // Regular case: content differs, just write it
                utils::write_file(file_path, &updated_content)?;
                Ok(true) // File was updated
            })();
            
            match result {
                Ok(updated) => updated,
                Err(e) => {
                    // Log the error but continue processing other files
                    debug!("Error processing diagrams for {:?}: {}", file_path, e);
                    false
                }
            }
        }).collect::<Vec<bool>>();
        
        // Count how many files were updated
        let updated_count = results.into_iter().filter(|&updated| updated).count();
        
        debug!("Finished processing diagrams for all files. Updated {} of {} files.", updated_count, files.len());
        Ok(())
    }
    
    /// Process all files in the input folder and write to the output folder
    pub fn process_files(
        &mut self,
        input_folder: &Path,
        output_folder: &Path,
        convert_to_html: bool,
    ) -> Result<(), ReqFlowError> {
        debug!("Processing files from {:?} to {:?}", input_folder, output_folder);
        
        // Update HTML output setting in config if specified
        if convert_to_html {
            self.config.general.html_output = true;
        }
        
        // Set the diagram generation flag in the element registry
        // This matches the config setting which was already set from the CLI flag
        self.element_registry.set_diagram_generation_enabled(self.config.general.generate_diagrams);
        
        // Create the output folder if it doesn't exist
        utils::create_dir_all(output_folder)?;
        
        // First pass: collect all elements
        self.collect_identifiers(input_folder)?;
        
        // Validate model consistency
        debug!("Validating model consistency...");
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
        debug!("Generating traceability matrix with {} elements", self.element_registry.all_elements().count());
        
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
            debug!("Checking element '{}' for relations - has {} relations", 
                  element.name, element.relations.len());
                  
            if !element.relations.is_empty() {
                elements_with_relations += 1;
                debug!("Element '{}' has the following relations:", element.name);
                
                for relation in &element.relations {
                    debug!("  - {}: {}", relation.relation_type, relation.target);
                    *relation_count.entry(relation.relation_type.clone()).or_insert(0) += 1;
                }
            }
        }
        
        debug!("Found {} elements total, {} with relations", element_count, elements_with_relations);
        
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
        
        // Define regex for level 2 (paragraph) headers
        lazy_static::lazy_static! {
            static ref PARAGRAPH_REGEX: regex::Regex = regex::Regex::new(r"^##\s+(.+)").unwrap();
        }
        
        // First, collect file contents for header detection
        let mut file_contents: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        for (file_path, content) in &self.file_contents {
            file_contents.insert(file_path.clone(), content.clone());
        }
        
        // Map to store paragraphs (by file and section name)
        let mut paragraphs_map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        
        // Parse all files to extract paragraph (level 2) headers
        for (file_path, content) in &file_contents {
            let mut paragraphs = Vec::new();
            
            for line in content.lines() {
                if let Some(captures) = PARAGRAPH_REGEX.captures(line) {
                    if let Some(para_name) = captures.get(1) {
                        paragraphs.push(para_name.as_str().to_string());
                    }
                }
            }
            
            // Store the paragraphs for this file
            if !paragraphs.is_empty() {
                paragraphs_map.insert(file_path.clone(), paragraphs);
            }
        }
        
        // Create the combined map of file+section to elements
        let mut section_element_map: std::collections::HashMap<String, Vec<&element::Element>> = std::collections::HashMap::new();
        
        // First, organize elements by file
        let mut files_with_elements: std::collections::HashMap<String, Vec<&element::Element>> = std::collections::HashMap::new();
        
        for element in self.element_registry.all_elements() {
            files_with_elements.entry(element.file_path.clone())
                .or_insert_with(Vec::new)
                .push(element);
        }
        
        // Now, for each file, detect which elements belong to which section
        for (file_path, elements) in &files_with_elements {
            // If the file has paragraphs, organize by paragraphs
            if let Some(paragraphs) = paragraphs_map.get(file_path) {
                // Get the file content
                if let Some(content) = file_contents.get(file_path) {
                    // Map to store which element belongs to which paragraph
                    let mut paragraph_elements: std::collections::HashMap<String, Vec<&element::Element>> = 
                        std::collections::HashMap::new();
                    
                    // Initialize with empty vectors for each paragraph
                    for paragraph in paragraphs {
                        paragraph_elements.insert(paragraph.clone(), Vec::new());
                    }
                    
                    // Track current paragraph
                    let mut current_paragraph: Option<String> = None;
                    
                    // Go through file line by line
                    for line in content.lines() {
                        // Check if this is a paragraph header
                        if let Some(captures) = PARAGRAPH_REGEX.captures(line) {
                            if let Some(para_name) = captures.get(1) {
                                current_paragraph = Some(para_name.as_str().to_string());
                            }
                        }
                        // Check if this is an element header
                        else if let Some(captures) = Config::element_regex().captures(line) {
                            if let Some(elem_name) = captures.get(1) {
                                let elem_name = elem_name.as_str().trim().to_string();
                                
                                // Find the element
                                if let Some(element) = elements.iter().find(|e| e.name == elem_name) {
                                    // Add to current paragraph if exists, otherwise to "Other"
                                    if let Some(paragraph) = &current_paragraph {
                                        paragraph_elements.entry(paragraph.clone())
                                            .or_insert_with(Vec::new)
                                            .push(*element);
                                    } else {
                                        // Element outside any section goes to file-level
                                        section_element_map.entry(format!("{}/no_section", file_path))
                                            .or_insert_with(Vec::new)
                                            .push(*element);
                                    }
                                }
                            }
                        }
                    }
                    
                    // Add all paragraphs to the main map
                    for (paragraph, para_elements) in paragraph_elements {
                        if !para_elements.is_empty() {
                            section_element_map.insert(format!("{}/{}", file_path, paragraph), para_elements);
                        }
                    }
                }
            } else {
                // No paragraphs in this file, add all elements to file-level
                section_element_map.insert(file_path.clone(), elements.clone());
            }
        }
        
        // Generate diagrams by section
        matrix_content.push_str("## Requirements Relationship Diagrams\n\n");
        matrix_content.push_str("These diagrams show the relationships between requirements organized by file and section.\n\n");
        matrix_content.push_str("* Red nodes: Requirements\n");
        matrix_content.push_str("* Yellow nodes: Elements that satisfy requirements\n");
        matrix_content.push_str("* Green nodes: Verification elements\n");
        matrix_content.push_str("* Light blue nodes: Links to other sections/files\n\n");
        
        // Iterate through sections and create a diagram for each
        for (section_id, elements) in section_element_map.iter() {
            // Only generate diagrams for sections that have elements with relations
            let has_relations = elements.iter().any(|e| !e.relations.is_empty());
            
            if has_relations {
                // Parse the section ID to get file and paragraph
                let parts: Vec<&str> = section_id.split('/').collect();
                let file_path_str = if parts.len() > 1 {
                    parts[0..parts.len()-1].join("/")
                } else {
                    section_id.clone()
                };
                
                let section_name = parts.last().unwrap_or(&"");
                
                // Create a header for this section's diagram
                let file_name = if let Some(idx) = file_path_str.rfind('/') {
                    &file_path_str[idx + 1..]
                } else {
                    &file_path_str
                };
                
                // Use section name or file name as header
                let diagram_header = if *section_name == "no_section" {
                    format!("### Diagram for {}", file_name)
                } else {
                    format!("### Diagram for {} - {}", file_name, section_name)
                };
                
                matrix_content.push_str(&format!("{}\n\n", diagram_header));
                // Use the configured diagram direction
                let direction = &self.config.style.diagram_direction;
                matrix_content.push_str(&format!("```mermaid\ngraph {};\n", direction));
                
                // Add graph styling with updated colors
                matrix_content.push_str("  %% Graph styling\n");
                matrix_content.push_str("  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;\n");
                matrix_content.push_str("  classDef satisfies fill:#fff2cc,stroke:#ffcc00,stroke-width:1px;\n");
                matrix_content.push_str("  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;\n");
                matrix_content.push_str("  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;\n");
                matrix_content.push_str("  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;\n\n");
                
                // Create a set of elements already included in this diagram
                let mut included_elements = std::collections::HashSet::new();
                
                // Get file_path_str as PathBuf for comparison
                let section_file_path = Path::new(&file_path_str);
                
                // Add nodes for this file's elements
                for element in elements {
                    // Safe ID for element - sanitize by replacing all non-alphanumeric chars with underscore
                    let element_id = element.name.chars()
                        .map(|c| if c.is_alphanumeric() { c } else { '_' })
                        .collect::<String>();
                    
                    // Encode the label text properly for Mermaid
                    let label = element.name.replace('"', "&quot;");
                    
                    matrix_content.push_str(&format!("  {}[\"{}\"];\n", element_id, label));
                    
                    // Add click behavior for HTML
                    let html_file = if convert_to_html {
                        element.file_path.replace(".md", ".html")
                    } else {
                        element.file_path.clone()
                    };
                    
                    matrix_content.push_str(&format!("  click {} \"{}\";\n", 
                        element_id, 
                        format!("{}#{}", html_file, element.name.replace(' ', "-").to_lowercase())
                    ));
                    
                    // Apply requirement style to all elements in this file
                    matrix_content.push_str(&format!("  class {} requirement;\n", element_id));
                    
                    // Mark this element as included
                    included_elements.insert(element.name.clone());
                    
                    // Process relations for this element
                    for relation in &element.relations {
                        // Get target element
                        let target = &relation.target;
                        
                        // Generate safe ID for target - sanitize by replacing all non-alphanumeric chars with underscore
                        let target_id = target.chars()
                            .map(|c| if c.is_alphanumeric() { c } else { '_' })
                            .collect::<String>();
                        
                        // Different arrow styles and labels based on relation type
                        let arrow_style = match relation.relation_type.as_str() {
                            "verifiedBy" | "verify" => "-->|verifies|",
                            "satisfiedBy" | "satisfy" => "-->|satisfies|",
                            "derivedFrom" | "derive" => "-.->|derives from|",
                            "refine" => "==>|refines|",
                            "tracedFrom" | "trace" => "-->|traces from|", // Changed from ~~~> which is not valid in mermaid
                            "containedBy" | "contain" => "--o|contains|",
                            _ => "-->|relates to|"
                        };
                        
                        // Add the relationship
                        matrix_content.push_str(&format!("  {} {} {};\n", element_id, arrow_style, target_id));
                        
                        // If the target is not already included in the diagram, add it
                        if !included_elements.contains(target) {
                            // Add the target node
                            matrix_content.push_str(&format!("  {}[\"{}\"];\n", target_id, target.replace('"', "&quot;")));
                            
                            // Try to find the target element in the registry
                            let target_element = self.element_registry.get_element(target);
                            
                            // Check if the target element is from the same file+section
                            let is_external_link = if let Some(target_elem) = target_element {
                                // It's external if it's from a different file 
                                let target_file_path = Path::new(&target_elem.file_path);
                                section_file_path != target_file_path
                            } else {
                                // If we can't find the element, it's external
                                true
                            };
                            
                            // Determine the style class based on relation type and element location
                            let style_class = if is_external_link {
                                // External elements get the blue link style
                                "externalLink"
                            } else if relation.relation_type == "verifiedBy" || relation.relation_type == "verify" {
                                // Verification elements get the green verification style
                                "verification"
                            } else if relation.relation_type == "satisfiedBy" || relation.relation_type == "satisfy" {
                                // Elements that satisfy requirements get the yellow satisfy style
                                "satisfies"
                            } else {
                                // Default style for other elements
                                "default"
                            };
                            
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
                            matrix_content.push_str(&format!("  class {} {};\n", target_id, style_class));
                            
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
            
            debug!("Traceability matrix HTML saved to {:?}", html_path);
        }
        
        debug!("Traceability matrix generated");
        Ok(())
    }
    
    /// Validate the model for consistency
    fn validate_model(&self) -> Result<(), ReqFlowError> {
        use crate::validation::{validate_relations, validate_markdown_structure};
        
        // Validate relations
        let relation_errors = validate_relations(&self.element_registry)?;
        
        if !relation_errors.is_empty() {
            debug!("Found {} relation validation errors", relation_errors.len());
            for error in &relation_errors {
                // Error details need to be visible in normal mode
                debug!("Validation error: {}", error);
            }
            // For now, we'll just log errors but continue processing
            // In a production implementation, we might want to fail or handle differently
        } else {
            debug!("No relation validation errors found");
        }
        
        // Validate markdown structure for each file
        let mut total_structure_errors = 0;
        for element in self.element_registry.all_elements() {
            let file_path = &element.file_path;
            // Group validation by file path to avoid redundant validations
            if let Some(content) = self.file_contents.get(file_path) {
                let structure_errors = validate_markdown_structure(content)?;
                if !structure_errors.is_empty() {
                    debug!("Found {} structure validation errors in {}", structure_errors.len(), file_path);
                    for error in &structure_errors {
                        // Error details need to be visible in normal mode
                        debug!("Structure error: {}", error);
                    }
                    total_structure_errors += structure_errors.len();
                }
            }
        }
        
        if total_structure_errors > 0 {
            debug!("Total of {} structure validation errors found", total_structure_errors);
        } else {
            debug!("No structure validation errors found");
        }
        
        Ok(())
    }
    
    /// Collect identifiers from all markdown files in the input folder
    fn collect_identifiers(&mut self, input_folder: &Path) -> Result<(), ReqFlowError> {
        debug!("Collecting identifiers from markdown files");
        
        // Use existing registry
        let registry = &mut self.element_registry;
        
        // Get the current diagram generation setting
        let diagram_enabled = registry.is_diagram_generation_enabled();
        
        // Clear the registry first to avoid duplicates
        // (not ideal for incremental updates, but ensures clean collection)
        *registry = ElementRegistry::new();
        
        // Restore the diagram generation setting
        registry.set_diagram_generation_enabled(diagram_enabled);
        
        debug!("Scanning directory for all markdown files: {}", input_folder.display());
        debug!("Input folder for scanning: {}", input_folder.display());
        
        // Debug: List all files in the directory
        for entry in walkdir::WalkDir::new(input_folder).max_depth(2).into_iter().filter_map(|e| e.ok()) {
            debug!("Found file in directory: {}", entry.path().display());
        }
        
        for entry in WalkDir::new(input_folder).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            debug!("Processing entry: {}", path.display());
            
            // Process all potentially relevant files to collect identifiers
            if path.is_file() {
                if path.extension().map_or(false, |ext| ext == "md") {
                    // Log all markdown files for debugging
                    debug!("Found Markdown file: {}", path.display());
                    
                    // Check if it's processable
                    let is_proc = path.file_name().map_or(false, |name| utils::is_processable_file(name.to_string_lossy().as_ref()));
                    debug!("File {} is processable: {}", path.display(), is_proc);
                    
                    if !is_proc {
                        // Force processing for DSD files based on path
                        let is_dsd_by_path = path.to_string_lossy().contains(&self.config.paths.design_specifications_folder);
                        if is_dsd_by_path {
                            debug!("Forcing processing of DSD file by path: {}", path.display());
                        } else {
                            continue;
                        }
                    }
                } else {
                    // Not a markdown file
                    continue;
                }
                
                // Check if this file should be excluded by any excluded filename patterns
                let should_exclude = utils::is_excluded_by_patterns(path, &self.config.paths.excluded_filename_patterns, input_folder, self.config.general.verbose);
                
                if should_exclude {
                    debug!("Skipping excluded file: {:?}", path);
                    continue;
                }
                
                debug!("Collecting identifiers from {:?}", path);
                
                let content = utils::read_file(path)?;
                let relative_path = utils::get_relative_path(path, input_folder)?;
                let relative_path_str = relative_path.to_string_lossy().to_string();
                
                // Store file content for later validation and processing
                self.file_contents.insert(relative_path_str.clone(), content.clone());
                
                // Check if this is a Design Specification Document
                // DSDs are identified by being in the design_specifications_folder from config
                let design_specs_folder_name = &self.config.paths.design_specifications_folder;
                
                // Check if this is a Design Specification Document based on its path
                debug!("Checking if '{}' is a DSD file (config DSD folder: '{}')", 
                      relative_path_str, design_specs_folder_name);
                
                // Simple check: is the design specification folder name anywhere in the path?
                let is_dsd = path.to_string_lossy().contains(design_specs_folder_name);
                             
                debug!("is_dsd={} for {} (design_specs_folder={})", 
                       is_dsd, relative_path_str, design_specs_folder_name);
                
                // Create a document element for the file
                let title = if let Some(title_line) = content.lines().find(|line| line.starts_with("# ")) {
                    title_line.trim_start_matches("# ").trim().to_string()
                } else {
                    // Default title if none found
                    Path::new(&relative_path_str)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("Untitled Document")
                        .to_string()
                };
                
                // Normalize the path for consistent registry lookup
                // IMPORTANT: This path is where the file is stored in the registry
                // It must match the path used in relation validation
                let normalized_path = crate::utils::normalize_path(&relative_path_str, &self.config, "");
                debug!("Normalized path for registry: {} -> {}", relative_path_str, normalized_path);
                
                // Log the normalized path for DSD files
                if is_dsd {
                    debug!("DSD file normalized path: {}", normalized_path);
                }
                
                // Add the file as an element for relation validation using the normalized path
                let file_element = crate::element::Element::new(
                    title.clone(),
                    normalized_path,
                );
                
                // Add document to registry for validation
                if let Err(e) = registry.add_element(file_element) {
                    debug!("Error adding file element: {}", e);
                } else {
                        // For DSD files, we've already added the file with its normalized path
                    // We don't need to create hack paths - validation should handle normalization
                    
                    debug!("Successfully added file element for validation: {}", relative_path_str);
                }
                
                if !is_dsd {
                    // Also collect individual elements for non-DSD documents
                    markdown::collect_elements(&content, &relative_path_str, registry)?;
                } else {
                    debug!("Skipping detailed element collection for Design Specification Document: {}", relative_path_str);
                }
            }
        }
        
        Ok(())
    }
    
    /// Public method to collect identifiers only - used for validation, diagram generation and other operations
    /// This method just does basic element collection without performing any validation
    pub fn collect_identifiers_only(&mut self, input_folder: &Path) -> Result<(), ReqFlowError> {
        // When in diagram generation mode, set the flag in the element registry
        // We'll transfer the config setting to the registry
        self.element_registry.set_diagram_generation_enabled(self.config.general.generate_diagrams);
        
        self.collect_identifiers(input_folder)
    }
    
    /// Validate markdown structure of all collected files
    /// Returns a list of validation errors
    pub fn validate_markdown_structure(&self) -> Result<Vec<ReqFlowError>, ReqFlowError> {
        debug!("Validating markdown structure");
        
        let mut all_errors = Vec::new();
        
        // Process each file for markdown structure validation
        for (file_path, content) in &self.file_contents {
            // Skip validation for Design Specification Documents
            // DSDs are identified by being in any folder that matches the design_specifications_folder name
            let design_specs_folder_name = &self.config.paths.design_specifications_folder;
            
            // Check if any component of the path matches the design specs folder name
            let is_dsd = file_path.split('/').any(|component| component == design_specs_folder_name);
            
            if is_dsd {
                debug!("Skipping validation for Design Specification Document: {}", file_path);
                continue;
            }
            
            // Check if this file should be excluded by any excluded filename patterns
            let path_obj = std::path::Path::new(file_path);
            let should_exclude = utils::is_excluded_by_patterns(
                path_obj, 
                &self.config.paths.excluded_filename_patterns, 
                std::path::Path::new(&self.config.paths.specifications_folder),
                false
            );
            
            if should_exclude {
                debug!("Skipping validation for excluded file: {}", file_path);
                continue;
            }
            
            debug!("Validating markdown structure of {}", file_path);
            
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
        
        if !all_errors.is_empty() {
            debug!("Found {} markdown structure validation errors", all_errors.len());
            // Error details need to be visible in normal mode
            for error in &all_errors {
                debug!("Markdown error: {}", error);
            }
        } else {
            debug!("No markdown structure validation errors found");
        }
        
        Ok(all_errors)
    }
    
    /// Validate all relations in the model
    /// Returns a list of validation errors
    pub fn validate_relations(&self) -> Result<Vec<ReqFlowError>, ReqFlowError> {
        debug!("Validating relations");
        
        // Use existing validation function
        let errors = validation::validate_relations(&self.element_registry)?;
        
        if !errors.is_empty() {
            debug!("Found {} relation validation errors", errors.len());
            // Error details need to be visible in normal mode
            for error in &errors {
                debug!("Relation error: {}", error);
            }
        } else {
            debug!("No relation validation errors found");
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
        debug!("Validating filesystem structure in {:?}", input_folder);
        
        let mut errors = Vec::new();
        
        // Get configuration values for folders
        let specs_folder = &config.paths.specifications_folder;
        
        // Get paths from config using helper methods, but use them correctly
        // These paths should all be within the input folder, not absolute or in another repository
        let specs_dir_path = input_folder.join(specs_folder);
        let design_specs_dir_path = specs_dir_path.join(&config.paths.design_specifications_folder);
        
        // Special case for testing - if input_folder is already "specifications", don't add it again
        let actual_specs_dir_path = if input_folder.file_name().map_or(false, |name| name == "specifications") {
            input_folder.to_path_buf()
        } else {
            specs_dir_path.clone()
        };
        
        let actual_design_specs_dir_path = if input_folder.file_name().map_or(false, |name| name == "specifications") {
            input_folder.join(&config.paths.design_specifications_folder)
        } else {
            design_specs_dir_path.clone()
        };
        
        // Always validate the main specifications directory
        if !actual_specs_dir_path.exists() || !actual_specs_dir_path.is_dir() {
            errors.push(ReqFlowError::ValidationError(
                format!("Required specifications directory '{}' is missing", actual_specs_dir_path.display())
            ));
        } else {
            // Design Specifications directory is still required
            if !actual_design_specs_dir_path.exists() || !actual_design_specs_dir_path.is_dir() {
                errors.push(ReqFlowError::ValidationError(
                    format!("Required design specifications directory '{}' is missing", actual_design_specs_dir_path.display())
                ));
            }
            
            // We no longer require a specific SystemRequirements directory
            // Any requirement in a specifications subfolder is treated as a system requirement
        }
        
        // Validate external folders
        for external_folder in &config.paths.external_folders {
            let folder_path = if Path::new(external_folder).is_absolute() {
                PathBuf::from(external_folder)
            } else {
                input_folder.join(external_folder)
            };
            
            if !folder_path.exists() || !folder_path.is_dir() {
                errors.push(ReqFlowError::ValidationError(
                    format!("External folder '{}' is missing or not a directory", folder_path.display())
                ));
            }
        }
        
        // No README validation - README files are optional
        
        // We no longer require a specific SystemRequirements.md file
        // Any markdown file in a specifications subfolder or external folder can be a system requirement
        
        // Check for files in wrong locations within the main specs directory
        for entry in WalkDir::new(input_folder).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            
            // Check if this file is in an external folder - apply folder structure rules to external folders too
            let in_external_folder = config.paths.external_folders.iter().any(|folder| {
                let folder_path = if Path::new(folder).is_absolute() {
                    PathBuf::from(folder)
                } else {
                    input_folder.join(folder)
                };
                path.starts_with(&folder_path)
            });
            
            // For files in external folders, we'll apply standard folder structure rules
            // but with the external folder as the root instead of the specifications folder
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                let file_name = path.file_name().unwrap().to_string_lossy();
                
                // All markdown files are now considered for element extraction
                
                // Determine the root directory for this file (specifications or external folder)
                let (root_dir, design_spec_dir) = if in_external_folder {
                    // Find which external folder contains this file
                    let mut ext_root_dir = PathBuf::new();
                    let mut ext_design_spec_dir = PathBuf::new();
                    
                    for folder in &config.paths.external_folders {
                        let folder_path = if Path::new(folder).is_absolute() {
                            PathBuf::from(folder)
                        } else {
                            input_folder.join(folder)
                        };
                        
                        if path.starts_with(&folder_path) {
                            // This is the external folder containing our file
                            ext_root_dir = folder_path.clone();
                            ext_design_spec_dir = folder_path.join(&config.paths.design_specifications_folder);
                            break;
                        }
                    }
                    
                    (ext_root_dir, ext_design_spec_dir)
                } else {
                    // Use the regular specifications folder
                    (actual_specs_dir_path.clone(), actual_design_specs_dir_path.clone())
                };
                
                let in_specs_root = path.parent().map_or(false, |p| p == root_dir);
                let is_requirements_file = file_name.ends_with(".md");
                
                // User requirements are in specs root, system requirements are in system_requirements_folder
                // A file is considered a User Requirements file if it:
                // 1. Contains the requirements_filename_match string
                // 2. Is directly in the root folder (specifications or external)
                let is_user_req_file = is_requirements_file && in_specs_root;
                
                // A file is considered a System Requirements file if it:
                // 1. Contains the requirements_filename_match string
                // 2. Is NOT directly in the root folder (meaning it's in a subfolder)
                let is_system_req_file = is_requirements_file && !is_user_req_file;
                
                // User Requirements should be in the specifications root folder, NOT in external folders
                if is_user_req_file {
                    if !in_specs_root {
                        errors.push(ReqFlowError::ValidationError(
                            format!("File '{}': User Requirements file must be in the root folder '{}'", 
                                    path.display(), root_dir.display())
                        ));
                    }
                    
                    // Check if User Requirements file is in an external folder
                    if in_external_folder {
                        errors.push(ReqFlowError::ValidationError(
                            format!("File '{}': User Requirements are not allowed in external folders", 
                                    path.display())
                        ));
                    }
                }
                
                // System Requirements can be in any subfolder of specifications or in external folders
                // No need to validate specific directory location for system requirements
                
                // Check if file is in the Design Specifications directory by matching the exact folder name
                let path_str = path.to_string_lossy();
                let design_folder_name = &config.paths.design_specifications_folder;
                
                // The key check is whether the path contains the exact design specifications folder name 
                let is_in_design_specs = path_str.contains(design_folder_name);
                
                if is_in_design_specs {
                    // All files in this directory are considered design specifications
                    // No need for additional validation
                // Only files inside the design specifications folder are considered design specifications
                // We no longer check file names
                }
            }
        }
        
        if !errors.is_empty() {
            debug!("Found {} filesystem structure validation errors", errors.len());
            // Error details need to be visible in normal mode
            for error in &errors {
                debug!("Filesystem error: {}", error);
            }
        } else {
            debug!("No filesystem structure validation errors found");
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
        debug!("Validating cross-component dependencies");
        
        let mut errors = Vec::new();
        
        // Check for circular dependencies
        for element in self.element_registry.all_elements() {
            let mut visited = std::collections::HashSet::new();
            let mut path = Vec::new();
            
            self.check_circular_dependencies(element, &mut visited, &mut path, &mut errors);
        }
        
        // We no longer use a specific system requirements folder
        
        // Check for incomplete dependency chains (e.g., missing derivedFrom relations)
        // For example, system requirements should have derivedFrom relations to user requirements
        for element in self.element_registry.all_elements() {
            // Check for system requirements without any parent relation
            // System requirements must have at least one relation that points to a parent requirement
            
            // Determine if this is a system requirement
            // System requirements are:
            // 1. Any requirement in a specifications subfolder (not in the root)
            // 2. Any requirement in an external folder
            
            // Check if element is in the specifications folder but not at the root level
            let specs_folder = Path::new(&self.config.paths.specifications_folder);
            let specs_folder_str = self.config.paths.specifications_folder.as_str();
            let is_in_specs_subfolder = element.file_path.contains(specs_folder_str) && 
                                        element.file_path.matches('/').count() > 1;
            
            // Check if element is in an external folder
            let is_in_external_folder = self.config.paths.external_folders.iter().any(|ext_folder| {
                element.file_path.contains(ext_folder)
            });
            
            // Check if this is a Design Specification Document - using same logic as in file processing 
            let design_specs_folder_name = &self.config.paths.design_specifications_folder;
            let is_dsd = element.file_path.contains(design_specs_folder_name);
            
            // Any requirement in a subfolder or external folder is a system requirement (except for Design Specifications)
            if (is_in_specs_subfolder || is_in_external_folder) && !is_dsd {
                // Get valid parent relation types from relation module
                let valid_parent_relations = crate::relation::get_parent_relation_types();
                
                // Check if the element has at least one valid parent relation
                let has_parent_relation = element.relations.iter().any(|r| valid_parent_relations.contains(&r.relation_type.as_str()));
                
                if !has_parent_relation {
                    // Format the valid relation types for the error message
                    let relation_types_str = valid_parent_relations.join("', '");
                    
                    errors.push(ReqFlowError::ValidationError(
                        format!("System requirement '{}' has no parent requirement relation (needs '{}') (in file '{}')", 
                               element.name, relation_types_str, element.file_path)
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
        
        if !errors.is_empty() {
            debug!("Found {} cross-component dependency validation errors", errors.len());
            // Error details need to be visible in normal mode
            for error in &errors {
                debug!("Dependency error: {}", error);
            }
        } else {
            debug!("No cross-component dependency validation errors found");
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
        debug!("Processing markdown files");
        
        // Collect files to process based on mode
        let files: Vec<PathBuf>;
        
        if convert_to_html {
            // For HTML conversion, get ALL markdown files in the input folder and its subfolders
            debug!("HTML mode: Processing all markdown files in all subfolders");
            files = WalkDir::new(input_folder)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    let file_path = e.path();
                    if !(file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "md")) {
                        return false;
                    }
                    
                    // Check if this file should be excluded by any excluded filename patterns
                    !utils::is_excluded_by_patterns(file_path, &self.config.paths.excluded_filename_patterns, input_folder, self.config.general.verbose)
                })
                .map(|e| e.path().to_path_buf())
                .collect();
                
            debug!("Found {} markdown files to convert to HTML", files.len());
        } else {
            // Regular mode: Process only requirements files
            debug!("Regular mode: Processing only requirements files");
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
                    // (this already includes the excluded_filename_patterns check)
                    utils::is_requirements_file_by_path(file_path, &self.config, input_folder)
                })
                .map(|e| e.path().to_path_buf())
                .collect();
                
            debug!("Found {} requirements files to process", files.len());
        }
        
        // Process files in parallel
        files.par_iter().try_for_each(|file_path| {
            debug!("Processing file {:?}", file_path);
            
            let content = utils::read_file(file_path)?;
            let relative_path = utils::get_relative_path(file_path, input_folder)?;
            
            // Replace relations with markdown links and add mermaid diagrams if needed
            let updated_content = markdown::replace_relations(
                &content,
                &self.element_registry,
                &relative_path,
                convert_to_html, // Pass the convert_to_html flag
            )?;
            
            // For requirements files, also update the source file with the improved content
            // (with mermaid diagrams and processed relations) but only if diagrams are enabled
            if self.config.general.generate_diagrams && 
               utils::is_requirements_file_by_path(file_path, &self.config, input_folder) {
                utils::write_file(file_path, &updated_content)?;
            }
            
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
        
        debug!("Finished processing all files");
        Ok(())
    }
}
