use std::collections::HashMap;
use crate::element;
use crate::relation;
use crate::error::ReqFlowError;

use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::collections::BTreeMap;

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

        
    /// To return both the impacted element and the trigger relation.
    pub fn change_impact_with_relation(
        &self,
        changed: &element::Element,
    ) -> BTreeMap<String, BTreeSet<relation::Relation>> {
        let mut impacted: BTreeMap<String, BTreeSet<relation::Relation>> = BTreeMap::new();
        let mut worklist = VecDeque::new();

        // Start with the changed element’s identifier (with no trigger at the root).
        worklist.push_back((&changed.identifier, None));

        while let Some((current_id, _prev_relation)) = worklist.pop_front() {
            for (_id, elem) in &self.elements {
                for relation in &elem.relations {
                    if let relation::LinkType::Identifier(ref target_id) = relation.target.link {
                        // Ensure we only consider Forward relations
                        if relation.relation_type.direction == relation::RelationDirection::Forward {
                            if &elem.identifier == current_id {
                                // Insert the impacted element with the full relation object into the map
                                impacted
                                    .entry(target_id.clone())
                                    .or_insert_with(BTreeSet::new)
                                    .insert(relation.clone());

                                // Add to worklist if not already impacted
                                if !impacted.contains_key(target_id) {
                                    worklist.push_back((target_id, Some(relation.relation_type.name.to_string())));
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



