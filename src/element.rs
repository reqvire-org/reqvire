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
        // Use DEBUG level instead of INFO for most logs
        log::debug!("Checking for element: {}", identifier);
        
        // Direct lookup - fastest path
        if self.elements.contains_key(identifier) {
            log::debug!("Direct element match found: {}", identifier);
            return true;
        }
        
        // Check if this might be a file reference without an element identifier
        if identifier.ends_with(".md") {
            // Check if any element in registry has this file path
            for (id, elem) in &self.elements {
                if elem.file_path == identifier {
                    log::debug!("Found matching file path: {} -> {}", identifier, id);
                    return true;
                }
            }
        }
        
        // Handle fragment references with special logic
        if identifier.contains("#") {
            // Split the identifier into file path and fragment
            let parts: Vec<&str> = identifier.split('#').collect();
            if parts.len() == 2 {
                let file_path = parts[0];
                let fragment = parts[1];
                
                log::debug!("Checking fragment reference with path '{}' and fragment '{}'", file_path, fragment);
                
                // Normalize the fragment for comparison
                let normalized_fragment = crate::utils::normalize_fragment(fragment);
                
                // Check all elements for matches based on fragment
                for (id, elem) in &self.elements {
                    // Extract element's fragment for comparison
                    if let Some(elem_fragment_idx) = id.find('#') {
                        let elem_fragment = &id[elem_fragment_idx+1..];
                        let elem_path = &id[0..elem_fragment_idx];
                        
                        let elem_normalized = crate::utils::normalize_fragment(elem_fragment);
                        let paths_match = if !file_path.is_empty() {
                            // Check for path match when file_path is provided
                            elem_path.ends_with(file_path) || 
                            file_path.ends_with(elem_path) ||
                            elem.file_path.contains(file_path) ||
                            file_path.contains(&elem.file_path)
                        } else {
                            // No path specified, so any path matches
                            true
                        };
                        
                        // For fragment matching, compare the normalized versions with various cases
                        let fragments_match = normalized_fragment == elem_normalized || 
                            elem.name.to_lowercase().contains(&fragment.to_lowercase());
                        
                        if paths_match && fragments_match {
                            log::debug!("Found matching element through fragment identification: {} matches {}", identifier, id);
                            return true;
                        }
                    }
                }
            }
            
            // Enhanced logging for debugging fragment references
            log::debug!("Fragment identifier not found directly: {}", identifier);
            // Log some similar keys for comparison
            for (id, _) in self.elements.iter().filter(|(k, _)| k.contains("#") || k.contains(&identifier.split("#").next().unwrap_or(""))) {
                log::debug!("  Similar registry element: {}", id);
            }
        }
        
        log::debug!("Element not found: {}", identifier);
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
            if elem.file_path == file_path {
                log::debug!("Found file in registry: {}", file_path);
                return true;
            }
        }
        
        log::debug!("File not found in registry: {}", file_path);
        false
    }
}
