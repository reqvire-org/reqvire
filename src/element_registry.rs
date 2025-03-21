use std::collections::HashMap;
use crate::element;
use crate::relation;
use crate::error::ReqFlowError;

use std::collections::HashSet;

#[derive(Debug)]
pub struct ElementRegistry {
    /// Map of full identifiers to elements
    pub elements: HashMap<String, element::Element>
}

impl ElementRegistry {
    /// Creates a new registry with a given configuration
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    /// Registers an element, ensuring identifier uniqueness
    pub fn register_element(&mut self, element: element::Element, _file_path: &str) -> Result<(), ReqFlowError> {
   
        if self.elements.contains_key(&element.identifier) {
            return Err(ReqFlowError::DuplicateElement(element.identifier));
        }

        let identifier=element.identifier.clone();
        self.elements.insert(identifier, element);
        Ok(())
    }

    /// Retrieves an element by its identifier
    pub fn get_element(&self, identifier: &str) -> Result<&element::Element, ReqFlowError> {
        self.elements
            .get(identifier)
            .ok_or_else(|| ReqFlowError::MissingElement(identifier.to_string()))
    }

    /// Retrieves all elements
    pub fn get_all_elements(&self) -> Vec<&element::Element> {
        self.elements.values().collect()
    }

    /// Checks if an element exists in the registry
    pub fn contains_element(&self, identifier: &str) -> bool {
        self.elements.contains_key(identifier)
    }

    /// Registers multiple elements at once
    pub fn register_elements(&mut self, elements: Vec<element::Element>, file_path: &str) -> Vec<ReqFlowError> {
        let mut errors = vec![];

        for element in elements {
            if let Err(e) = self.register_element(element, file_path) {
                errors.push(e);
            }
        }

        errors
    }

        
    // to return both the impacted element and the trigger relation.
    pub fn change_impact_with_relation(&self, changed: &crate::element::Element) -> HashSet<(String, String)> {
        let mut impacted = HashSet::new();
        let mut worklist = Vec::new();

        // Start with the changed element’s identifier (with no trigger at the root).
        worklist.push((&changed.identifier, None));

        while let Some((current_id, _prev_relation)) = worklist.pop() {
            for (_id, elem) in &self.elements {
                for relation in &elem.relations {
                    if let relation::LinkType::Identifier(ref target_id) = relation.target.link {
                            match relation.relation_type.direction {
                                relation::RelationDirection::Forward => {
                                    if &elem.identifier == current_id {
                                        if !impacted.contains(&(target_id.clone(), relation.relation_type.name.to_string())) {
                                            if self.elements.contains_key(target_id) {
                                                impacted.insert((target_id.clone(), relation.relation_type.name.to_string()));
                                                worklist.push((target_id, Some(relation.relation_type.name.to_string())));
                                            }
                                        }
                                    }
                                },
                                
                                relation::RelationDirection::Backward => {
                                    if target_id == current_id {
                                        if !impacted.contains(&(elem.identifier.clone(), relation.relation_type.name.to_string())) {
                                            impacted.insert((elem.identifier.clone(), relation.relation_type.name.to_string()));
                                            worklist.push((&elem.identifier, Some(relation.relation_type.name.to_string())));
                                        }
                                    }
                                },
                                
                                //relation::RelationDirection::Backward  => { /* Do nothing */ }
                                relation::RelationDirection::Neutral => { /* Do nothing */ }
                            }
                    }
                }
            }
        }
        impacted
    }

}

