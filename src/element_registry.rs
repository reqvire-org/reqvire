use std::collections::HashMap;
use crate::element::Element;
use crate::error::ReqFlowError;


#[derive(Debug)]
pub struct ElementRegistry {
    /// Map of full identifiers to elements
    pub elements: HashMap<String, Element>
}

impl ElementRegistry {
    /// Creates a new registry with a given configuration
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    /// Registers an element, ensuring identifier uniqueness
    pub fn register_element(&mut self, element: Element, _file_path: &str) -> Result<(), ReqFlowError> {
   
        if self.elements.contains_key(&element.identifier) {
            return Err(ReqFlowError::DuplicateElement(element.identifier));
        }

        let identifier=element.identifier.clone();
        self.elements.insert(identifier, element);
        Ok(())
    }

    /// Retrieves an element by its identifier
    pub fn get_element(&self, identifier: &str) -> Result<&Element, ReqFlowError> {
        self.elements
            .get(identifier)
            .ok_or_else(|| ReqFlowError::MissingElement(identifier.to_string()))
    }

    /// Retrieves all elements
    pub fn get_all_elements(&self) -> Vec<&Element> {
        self.elements.values().collect()
    }

    /// Checks if an element exists in the registry
    pub fn contains_element(&self, identifier: &str) -> bool {
        self.elements.contains_key(identifier)
    }

    /// Registers multiple elements at once
    pub fn register_elements(&mut self, elements: Vec<Element>, file_path: &str) -> Vec<ReqFlowError> {
        let mut errors = vec![];

        for element in elements {
            if let Err(e) = self.register_element(element, file_path) {
                errors.push(e);
            }
        }

        errors
    }

}

