use std::collections::HashMap;
use std::path::Path;

use crate::error::ReqFlowError;
use crate::relation::Relation;

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
}

impl Element {
    /// Create a new element with the given name and file path
    pub fn new(name: String, file_path: String) -> Self {
        Self {
            name,
            content: String::new(),
            relations: Vec::new(),
            file_path,
        }
    }

    /// Add content to the element
    pub fn add_content(&mut self, content: &str) {
        self.content.push_str(content);
    }

    /// Add a relation to the element
    pub fn add_relation(&mut self, relation: Relation) {
        self.relations.push(relation);
    }

    /// Get the full identifier for this element (file_path/element_name)
    pub fn identifier(&self) -> String {
        format!("{}/{}", self.file_path, self.name)
    }

    /// Create an anchor ID for HTML links
    #[allow(dead_code)]
    pub fn anchor_id(&self) -> String {
        self.name.replace(' ', "-").to_lowercase()
    }
}

/// Collection of elements with operations for finding and managing them
#[derive(Debug, Default)]
pub struct ElementRegistry {
    /// Map of full identifiers to elements
    elements: HashMap<String, Element>,
    
    /// Flag to indicate if diagram generation is enabled
    diagram_generation_enabled: bool,
}

impl ElementRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            diagram_generation_enabled: false,
        }
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
        self.elements.contains_key(identifier)
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