use std::collections::HashMap;
use crate::element;
use crate::relation;
use crate::error::ReqvireError;

use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::path::PathBuf;
use serde::Serialize;


/// A node for tree relation

#[derive(Debug,Clone, Serialize)]
pub struct RelationNode {
    pub relation_trigger: String,
    pub element_node: ElementNode,
}


#[derive(Debug, Clone, Serialize)]
pub struct ElementNode {
    pub element: element::Element,
    pub relations: Vec<RelationNode>,
}


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
    pub fn register_element(&mut self, element: element::Element, _file_path: &str) -> Result<(), ReqvireError> {
   
        if self.elements.contains_key(&element.identifier) {
            return Err(ReqvireError::DuplicateElement(element.identifier));
        }

        let identifier=element.identifier.clone();
        self.elements.insert(identifier, element);
        Ok(())
    }

    /// Retrieves an element by its identifier
    pub fn get_element(&self, identifier: &str) -> Result<&element::Element, ReqvireError> {
        self.elements
            .get(identifier)
            .ok_or_else(|| ReqvireError::MissingElement(identifier.to_string()))
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
    pub fn register_elements(&mut self, elements: Vec<element::Element>, file_path: &str) -> Vec<ReqvireError> {
        let mut errors = vec![];

        for element in elements {
            if let Err(e) = self.register_element(element, file_path) {
                errors.push(e);
            }
        }

        errors
    }
    
    /// Gets all elements of a specific type
    pub fn get_elements_by_type(&self, element_type_str: &str) -> Vec<&element::Element> {
        self.elements.values()
            .filter(|e| match &e.element_type {
                element::ElementType::Requirement(_) if element_type_str == "requirement" => true,
                element::ElementType::Verification(_) if element_type_str == "verification" => true,
                element::ElementType::File if element_type_str == "file" => true,
                element::ElementType::Other(ref t) if t == element_type_str => true,
                _ => false,
            })
            .collect()
    }
    
    /// Gets all root requirements (requirements that don't have any parent relations)
    pub fn get_root_requirements(&self) -> Vec<&element::Element> {
        let parent_relation_types = relation::get_parent_relation_types();
        
        self.elements.values()
            .filter(|e| match e.element_type {
                element::ElementType::Requirement(_) => true,
                _ => false,
            })
            .filter(|e| {
                // Check if it doesn't have any parent relations
                !e.relations.iter().any(|r| 
                    parent_relation_types.contains(&r.relation_type.name)
                )
            })
            .collect()
    }
    
    /// Gets all requirements grouped by root requirements
    /// Returns a map where keys are root requirement IDs and values are the requirements in that group
    pub fn get_requirements_by_root(&self) -> HashMap<String, Vec<&element::Element>> {
        let mut result = HashMap::new();
        let root_requirements = self.get_root_requirements();
        
        // Add each root as its own group initially
        for root in &root_requirements {
            result.insert(root.identifier.clone(), vec![*root]);
        }
        
        // If no root requirements found, return empty map
        if root_requirements.is_empty() {
            return result;
        }
        
        // For any non-root requirement, find its root parent
        for element in self.elements.values() {
            // Check if this element is a root requirement
            let is_root = root_requirements.iter().any(|root| root.identifier == element.identifier);
            if is_root {
                continue; // Skip root requirements as they're already included
            }
            
            if let element::ElementType::Requirement(_) = element.element_type {
                // Find which root is the parent of this requirement
                for root in &root_requirements {
                    if self.is_child_of(element, root) {
                        result.entry(root.identifier.clone())
                            .or_insert_with(Vec::new)
                            .push(element);
                        break;
                    }
                }
            }
        }
        
        result
    }
    
    /// Checks if an element is a child of another element through parent relation types
    fn is_child_of(&self, potential_child: &element::Element, potential_parent: &element::Element) -> bool {
        let parent_relation_types = relation::get_parent_relation_types();
        
        // Direct relation check
        for relation in &potential_child.relations {
            if parent_relation_types.contains(&relation.relation_type.name) {
                if let relation::LinkType::Identifier(ref target_id) = relation.target.link {
                    if target_id == &potential_parent.identifier {
                        return true;
                    }
                    
                    // Recursive check through the target of this relation
                    if let Ok(intermediate) = self.get_element(target_id) {
                        if self.is_child_of(intermediate, potential_parent) {
                            return true;
                        }
                    }
                }
            }
        }
        
        false
    }

    /// Returns all relation targets that:
    /// - Refer to internal paths
    pub fn get_internal_path_targets(&self) -> HashSet<PathBuf> {
        let mut local_files = HashSet::new();

        for element in self.elements.values() {
            for rel in &element.relations {
                if let relation::LinkType::InternalPath(ref path) = rel.target.link {
                    local_files.insert(path.clone());
                }
            }
        }

        local_files
    }
            
    /// To return both the impacted element and the trigger relation.
    pub fn change_impact_with_relation(
        &self,
        changed: &element::Element,
    ) -> BTreeMap<String, BTreeSet<relation::Relation>> {
        let mut impacted: BTreeMap<String, BTreeSet<relation::Relation>> = BTreeMap::new();
        let mut worklist: VecDeque<(String, Option<String>)> = VecDeque::new();

        // Start with the changed element’s identifier (with no trigger at the root).
        worklist.push_back((changed.identifier.clone(), None));

        while let Some((current_id, _prev_relation)) = worklist.pop_front() {
            for (_id, elem) in &self.elements {
                for relation in &elem.relations {                     
                    
                    if matches!(relation.target.link, relation::LinkType::Identifier(_) | relation::LinkType::InternalPath(_)) {
                        let target_key = relation.target.link.as_str().to_string();

                        if relation.relation_type.direction == relation::RelationDirection::Forward {
                            if elem.identifier == current_id {
                                impacted
                                    .entry(target_key.clone())
                                    .or_insert_with(BTreeSet::new)
                                    .insert(relation.clone());

                                if !impacted.contains_key(&target_key) {
                                    let cloned_key = target_key.clone();
                                    worklist.push_back((cloned_key, Some(relation.relation_type.name.to_string())));
                                }
                            }
                        }
                    }                
                }
            }
        }

        impacted
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::Element;
    use crate::relation::{RelationTypeInfo, Relation, RelationTarget, RelationDirection};
    use std::collections::BTreeSet;

    /// Helper function to create a simple element.
    fn create_element(identifier: &str, name: &str, content: &str) -> Element {
        let mut element = Element::new(
            name,
            identifier,
            "test.md",
            "TestSection",
            Some(crate::element::ElementType::Requirement(crate::element::RequirementType::System))
        );
        element.content = content.to_string();
        element
    }

    /// Helper function to add a relation to an element.
    fn add_relation(element: &mut Element, relation_type: &'static RelationTypeInfo, target_id: &str) {
        element.relations.push(Relation {
            relation_type,
            target: RelationTarget {
                text: target_id.to_string(),
                link: relation::LinkType::Identifier(target_id.to_string()),
            },
            is_opposite: false,
        });
    }

    #[test]
    fn test_change_impact_with_relation_single_relation() {
        let mut my_struct = ElementRegistry::new();
        let element_a = create_element("A", "Element A", "Content A");
        let mut element_b = create_element("B", "Element B", "Content B");

        // Define a forward relation from B to A.       
        let rel = Relation {
            relation_type: &RelationTypeInfo {
                name: "derive",
                direction: RelationDirection::Forward,
                opposite: Some("derivedFrom"),
                description: "Element B derives from A",
                arrow: "-->",
                label: "label"
            },
            target: RelationTarget {
                text: "A".to_string(),
                link: relation::LinkType::Identifier("A".to_string()),
            },
            is_opposite: false,
        };
        element_b.relations.push(rel.clone());
                

        my_struct.elements.insert("A".to_string(), element_a.clone());
        my_struct.elements.insert("B".to_string(), element_b.clone());

        let impact = my_struct.change_impact_with_relation(&element_b);
        assert_eq!(impact.len(), 1);
        assert!(impact.contains_key("A"));
        assert!(impact.get("A").unwrap().contains(&rel));
    }

    #[test]
    fn test_change_impact_with_relation_multiple_relations() {
        let mut my_struct = ElementRegistry::new();
        let element_a = create_element("A", "Element A", "Content A");
        let mut element_b = create_element("B", "Element B", "Content B");
        let mut element_c = create_element("C", "Element C", "Content C");

        // Add multiple relations.
        let rela = Relation {
            relation_type: &RelationTypeInfo {
                name: "contain",
                direction: RelationDirection::Forward,
                opposite: Some("containedBy"),
                description: "Element B contains A",
                arrow: "-->",
                label: "label"                
            },
            target: RelationTarget {
                text: "A".to_string(),
                link: relation::LinkType::Identifier("A".to_string()),
            },
            is_opposite: false,
        };
        element_b.relations.push(rela.clone());
              
        let relb = Relation {
            relation_type: &RelationTypeInfo {
                name: "derive",
                direction: RelationDirection::Forward,
                opposite: Some("derivedFrom"),
                description: "Element C derives from B",
                arrow: "-->",
                label: "label"                
            },
            target: RelationTarget {
                text: "B".to_string(),
                link: relation::LinkType::Identifier("B".to_string()),
            },
            is_opposite: false,
        };   
        element_c.relations.push(relb.clone());             

        my_struct.elements.insert("A".to_string(), element_a.clone());
        my_struct.elements.insert("B".to_string(), element_b.clone());
        my_struct.elements.insert("C".to_string(), element_c.clone());

        let impact = my_struct.change_impact_with_relation(&element_c);
        assert_eq!(impact.len(), 1);
        assert!(impact.contains_key("B"));
        assert!(impact.get("B").unwrap().contains(&relb));
    }


}



