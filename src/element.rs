use std::collections::HashMap;
use std::path::Path;

use crate::error::ReqFlowError;
use crate::relation::Relation;

/// Type of element in the MBSE model
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementType {
    Requirement,
    Verification,
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
    
    /// Check if this element is a verification
    pub fn is_verification(&self) -> bool {
        matches!(self.element_type, ElementType::Verification)
    }
    
    /// Check if this element is a requirement
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
    #[allow(dead_code)]
    pub fn contains_element(&self, identifier: &str) -> bool {
        // Enhanced debugging - use INFO level for better visibility
        log::info!("Checking for element: {}", identifier);
        
        // Direct lookup - fastest path
        if self.elements.contains_key(identifier) {
            log::info!("Direct element match found: {}", identifier);
            return true;
        }
        
        // Check if this might be a file reference without an element identifier
        if identifier.ends_with(".md") {
            // Check if any element in registry has this file path
            for (id, elem) in &self.elements {
                if elem.file_path == identifier {
                    log::info!("Found matching file path: {} -> {}", identifier, id);
                    return true;
                }
            }
        }
        
        // Enhanced logging for debugging fragment references
        if identifier.contains("#") {
            log::info!("Fragment identifier not found directly: {}", identifier);
            // Log some similar keys for comparison
            for (id, _) in self.elements.iter().filter(|(k, _)| k.contains("#") || k.contains(&identifier.split("#").next().unwrap_or(""))) {
                log::info!("  Similar registry element: {}", id);
            }
        }
        
        log::info!("Element not found: {}", identifier);
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
}
