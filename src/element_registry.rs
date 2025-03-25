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
    pub fn change_impact_with_relation(&self, changed: &crate::element::Element) -> BTreeMap<String, BTreeSet<String>> {
        let mut impacted: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
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
                                // Insert the impacted element with the relation name into the map
                                impacted
                                    .entry(target_id.clone())
                                    .or_insert_with(BTreeSet::new)
                                    .insert(relation.relation_type.name.to_string());

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

    /// Builds the change impact tree recursively using `ElementNode` and keep only forward relations
pub fn build_change_impact_tree(
    &self,
    current: &ElementRegistry,
    element_id: String,
    visited: &mut BTreeSet<String>,
) -> ElementNode {
    // Fetch the current element
    let element = current.elements.get(&element_id).expect("Element not found").clone();

    // Create an empty vector to hold relation nodes
    let mut relations = Vec::new();

    // Get impacted elements using the change impact analysis function
    let impacted_map = self.change_impact_with_relation(&element);

    for (impacted_id, relation_triggers) in impacted_map {
        // Skip if the relation was already visited to prevent cycles
        if !visited.insert(impacted_id.clone()) {
            continue;
        }

        // Recursively build the child node
        let child_node = self.build_change_impact_tree(current, impacted_id.clone(), visited);

        // Create a RelationNode object with the trigger relation, filtering only forward relations
        for trigger in relation_triggers {
            // Fetch the relation type from the static map
            if let Some(relation_type_info) = crate::relation::RELATION_TYPES.get(trigger.as_str()) {
                // Only include Forward relations
                if relation_type_info.direction == crate::relation::RelationDirection::Forward {
                    let relation_node = RelationNode {
                        relation_trigger: trigger.clone(),
                        element_node: child_node.clone(),

                    };
                    relations.push(relation_node);
                }
            }
        }
    }

    // Construct and return the ElementNode
    ElementNode {
        element,
        relations,
    }
}
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    /// Helper function to create a simple element.
    fn create_element(identifier: &str, name: &str, content: &str) -> Element {
        Element {
            identifier: identifier.to_string(),
            name: name.to_string(),
            content: content.to_string(),
            relations: Vec::new(),
            hash_impact_content: String::new(),
        }
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
        add_relation(
            &mut element_b,
            &RelationTypeInfo {
                name: "derive",
                direction: RelationDirection::Forward,
                opposite: Some("derivedFrom"),
                description: "Element B derives from A",
            },
            "A",
        );

        my_struct.elements.insert("A".to_string(), element_a.clone());
        my_struct.elements.insert("B".to_string(), element_b.clone());

        let impact = my_struct.change_impact_with_relation(&element_b);
        assert_eq!(impact.len(), 1);
        assert!(impact.contains_key("A"));
        assert!(impact.get("A").unwrap().contains("derive"));
    }

    #[test]
    fn test_change_impact_with_relation_multiple_relations() {
        let mut my_struct = ElementRegistry::new();
        let element_a = create_element("A", "Element A", "Content A");
        let mut element_b = create_element("B", "Element B", "Content B");
        let mut element_c = create_element("C", "Element C", "Content C");

        // Add multiple relations.
        add_relation(
            &mut element_b,
            &RelationTypeInfo {
                name: "contain",
                direction: RelationDirection::Forward,
                opposite: Some("containedBy"),
                description: "Element B contains A",
            },
            "A",
        );

        add_relation(
            &mut element_c,
            &RelationTypeInfo {
                name: "derive",
                direction: RelationDirection::Forward,
                opposite: Some("derivedFrom"),
                description: "Element C derives from B",
            },
            "B",
        );

        my_struct.elements.insert("A".to_string(), element_a.clone());
        my_struct.elements.insert("B".to_string(), element_b.clone());
        my_struct.elements.insert("C".to_string(), element_c.clone());

        let impact = my_struct.change_impact_with_relation(&element_c);
        assert_eq!(impact.len(), 1);
        assert!(impact.contains_key("B"));
        assert!(impact.get("B").unwrap().contains("derive"));
    }

    #[test]
    fn test_build_change_impact_tree() {
        let mut my_struct = ElementRegistry::new();
        let element_a = create_element("A", "Element A", "Content A");
        let mut element_b = create_element("B", "Element B", "Content B");

        // Define a forward relation from B to A.
        add_relation(
            &mut element_b,
            &RelationTypeInfo {
                name: "derive",
                direction: RelationDirection::Forward,
                opposite: Some("derivedFrom"),
                description: "Element B derives from A",
            },
            "A",
        );

        my_struct.elements.insert("A".to_string(), element_a.clone());
        my_struct.elements.insert("B".to_string(), element_b.clone());

        let mut visited = BTreeSet::new();
        visited.insert("B".to_string());

        let tree = my_struct.build_change_impact_tree(
            &element_registry::ElementRegistry {
                elements: my_struct.elements.clone(),
            },
            "B".to_string(),
            &mut visited,
        );

        assert_eq!(tree.element.identifier, "B");
        assert_eq!(tree.relations.len(), 1);
        // Access the child node via element_node.
        assert_eq!(
            tree.relations[0].element_node.element.identifier,
            "A"
        );
        assert_eq!(tree.relations[0].relation_trigger, "derive");
    }

    #[test]
    fn test_tree_with_cycle() {
        let mut my_struct = ElementRegistry::new();
        let mut element_a = create_element("A", "Element A", "Content A");
        let mut element_b = create_element("B", "Element B", "Content B");

        // Create a cycle: A -> B -> A.
        add_relation(
            &mut element_a,
            &RelationTypeInfo {
                name: "contain",
                direction: RelationDirection::Forward,
                opposite: Some("containedBy"),
                description: "Element A contains B",
            },
            "B",
        );

        add_relation(
            &mut element_b,
            &RelationTypeInfo {
                name: "derive",
                direction: RelationDirection::Forward,
                opposite: Some("derivedFrom"),
                description: "Element B derives from A",
            },
            "A",
        );

        my_struct.elements.insert("A".to_string(), element_a.clone());
        my_struct.elements.insert("B".to_string(), element_b.clone());

        let mut visited = BTreeSet::new();
        visited.insert("A".to_string());

        let tree = my_struct.build_change_impact_tree(
            &element_registry::ElementRegistry {
                elements: my_struct.elements.clone(),
            },
            "A".to_string(),
            &mut visited,
        );

        // Check that the cycle is correctly handled and not infinite.
        assert_eq!(tree.element.identifier, "A");
        assert_eq!(tree.relations.len(), 1);
        // For the relation from A to B.
        assert_eq!(
            tree.relations[0].element_node.element.identifier,
            "B"
        );
        assert_eq!(tree.relations[0].relation_trigger, "contain");
        // Check the child node for B.
        assert_eq!(tree.relations[0].element_node.relations.len(), 1);
        // For the child relation of B (from B back to A), verify the target via B's original element field.
        assert_eq!(
            tree.relations[0].element_node.element.relations.len(),
            1
        );
        assert_eq!(
            tree.relations[0].element_node.element.relations[0].target.text,
            "A"
        );
    }
}



