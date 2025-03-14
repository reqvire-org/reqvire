use std::collections::HashMap;
use std::path::Path;

use crate::error::ReqFlowError;
use crate::relation::Relation;
use crate::utils;

/// Type of element in the MBSE model
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementType {
    Requirement,
    Verification,
    File,
    Other(String),
}

impl Default for ElementType {
    fn default() -> Self {
        ElementType::Requirement
    }
}

/// Represents an Element in the MBSE model
#[derive(Debug, Clone)]
pub struct Element {
    /// Name of the element (from the level 3 header)
    pub name: String,
    
    /// Content of the element (excluding relations)
    pub content: String,
    
    /// Relations defined within this element
    pub relations: Vec<Relation>,
    
    /// File path relative to the input folder
    pub file_path: String,
    
    /// Type of element (requirement, verification, etc.)
    pub element_type: ElementType,
    
    /// Additional metadata properties for the element
    pub metadata: HashMap<String, String>,
}

impl Element {
    /// Checks if this element requires a parent relation.
    pub fn requires_parent_relation(&self, registry: &ElementRegistry) -> bool {
        //dbg!(self);
        // Never required for File elements or Other
        if self.is_file() || self.is_other() {
            return false;
        }        
        // Always required for Verification type
        if self.is_verification() {
            return true;
        }

        let file_path = Path::new(&self.file_path);

        let is_in_external = utils::is_in_external_folder(file_path, &registry.config());
        let is_in_specifications_subfolder = utils::is_in_specifications_subfolder(file_path, &registry.config());

        // Requirements inside external folders or in subfolders of specifications require a parent
        self.is_requirement() && (is_in_external || is_in_specifications_subfolder)    
    }


    /// Create a new element with the given name and file path
    pub fn new(name: String, file_path: String) -> Self {
        Self {
            name,
            content: String::new(),
            relations: Vec::new(),
            file_path,
            element_type: ElementType::default(),
            metadata: HashMap::new(),
        }
    }
    
    /// Set the element type based on metadata
    pub fn set_type_from_metadata(&mut self) {
        if let Some(type_value) = self.metadata.get("type") {
            match type_value.to_lowercase().as_str() {
                "verification" => self.element_type = ElementType::Verification,
                "requirement" => self.element_type = ElementType::Requirement,
                other => self.element_type = ElementType::Other(other.to_string()),
            }
        }
    }

    /// Check if this element is a file
    pub fn is_file(&self) -> bool {
        matches!(self.element_type, ElementType::File)
    }
    /// Check if this element is a Other
    pub fn is_other(&self) -> bool {
        matches!(self.element_type, ElementType::Other(_))
    }    
    /// Check if this element is a verification
    pub fn is_verification(&self) -> bool {
        matches!(self.element_type, ElementType::Verification)
    }
    
    /// Check if this element is a requirement
    #[allow(dead_code)]
    pub fn is_requirement(&self) -> bool {
        matches!(self.element_type, ElementType::Requirement)
    }

    /// Add content to the element
    pub fn add_content(&mut self, content: &str) {
        self.content.push_str(content);
    }

    /// Add a relation to the element
    pub fn add_relation(&mut self, relation: Relation) {
        self.relations.push(relation);
    }

    /// Get the full identifier for this element (file_path#normalized-element-name)
    /// This matches the format used in GitHub-style fragment references
    pub fn identifier(&self) -> String {
        // Use the common utility function for normalization
        let normalized_fragment = crate::utils::normalize_fragment(&self.name);
        format!("{}#{}", self.file_path, normalized_fragment)
    }

    /// Create an anchor ID for HTML links
    #[allow(dead_code)]
    pub fn anchor_id(&self) -> String {
        // Use the common utility function for normalization
        crate::utils::normalize_fragment(&self.name)
    }
}

/// Collection of elements with operations for finding and managing them
#[derive(Debug)]
pub struct ElementRegistry {
    /// Map of full identifiers to elements
    elements: HashMap<String, Element>,
    
    /// Flag to indicate if diagram generation is enabled
    diagram_generation_enabled: bool,
    
    /// Configuration for the registry
    config: crate::config::Config,
}

impl ElementRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            diagram_generation_enabled: false,
            config: crate::config::Config::default(),
        }
    }
    
    /// Get a reference to the configuration
    pub fn config(&self) -> &crate::config::Config {
        &self.config
    }
    
    /// Set the configuration
    pub fn set_config(&mut self, config: crate::config::Config) {
        self.config = config;
    }
    
    /// Set the diagram generation flag
    pub fn set_diagram_generation_enabled(&mut self, enabled: bool) {
        self.diagram_generation_enabled = enabled;
    }
    
    /// Check if diagram generation is enabled
    pub fn is_diagram_generation_enabled(&self) -> bool {
        self.diagram_generation_enabled
    }

    /// Add an element to the registry
    pub fn add_element(&mut self, element: Element) -> Result<(), ReqFlowError> {
        let identifier = element.identifier();
        if self.elements.contains_key(&identifier) {
            log::warn!("Duplicate element found: {}", identifier);
            // Just skip it instead of returning an error
            return Ok(());
        }
        self.elements.insert(identifier, element);
        Ok(())
    }

    /// Add a file element to the registry
    pub fn add_file_element(&mut self, mut element: Element) -> Result<(), ReqFlowError> {
        element.element_type = ElementType::File;    
        self.add_element(element)
    }    
    /// Get an element by its identifier
    #[allow(dead_code)]
    pub fn get_element(&self, identifier: &str) -> Option<&Element> {
        self.elements.get(identifier)
    }

    /// Get all elements in the registry
    #[allow(dead_code)]
    pub fn all_elements(&self) -> impl Iterator<Item = &Element> {
        self.elements.values()
    }

    /// Check if an element exists in the registry
pub fn contains_element(&self, identifier: &str) -> bool {
    let normalized_identifier = crate::utils::normalize_path(identifier, self.config(), "");
    
    log::debug!("Checking for element: {}", normalized_identifier);
    
    if self.elements.contains_key(&normalized_identifier) {
        log::debug!("Direct element match found: {}", normalized_identifier);
        return true;
    }

    if identifier.ends_with(".md") {
        for (id, elem) in &self.elements {
            if elem.file_path == normalized_identifier {
                log::debug!("Found matching file path: {} -> {}", identifier, id);
                return true;
            }
        }
    }

    false
}



    /// Get elements from a specific file
    #[allow(dead_code)]
    pub fn elements_in_file<P: AsRef<Path>>(&self, file_path: P) -> Vec<&Element> {
        let file_path_str = file_path.as_ref().to_string_lossy();
        self.elements
            .values()
            .filter(|e| e.file_path == file_path_str)
            .collect()
    }
    
    /// Check if a file exists in the registry (by checking if any element has this file path)
    #[allow(dead_code)]
    pub fn contains_file(&self, file_path: &str) -> bool {
        log::debug!("Checking if file exists in registry: {}", file_path);
        
        // Check all elements to see if any are from this file
        for elem in self.elements.values() {
            // Direct match
            if elem.file_path == file_path {
                log::debug!("Found file in registry: {}", file_path);
                return true;
            }
            
            // Check if the element's file path ends with the search path
            // This helps when a file is referenced with a relative path
            if elem.file_path.ends_with(file_path) {
                log::debug!("Found matching file in registry (via suffix): {} matches {}", 
                            elem.file_path, file_path);
                return true;
            }
        }
        
        log::debug!("File not found in registry: {}", file_path);
        false
    }
    
    /// Check if a file is excluded based on the configuration's excluded_filename_patterns
    pub fn is_file_excluded(&self, file_path: &str) -> bool {
        let path = std::path::Path::new(file_path);
        crate::utils::is_excluded_by_patterns(
            path,
            &self.config,
            false
        )
    }
}
