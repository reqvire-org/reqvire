use std::collections::{HashMap, BTreeSet, VecDeque};

use crate::element::{Element, RequirementType, ElementType};
use crate::relation::{Relation, LinkType, RelationTarget, RELATION_TYPES, RelationDirection};
use crate::element_registry::{ElementNode, RelationNode, ElementRegistry};

#[derive(Debug)]
pub struct GraphRegistry {
    pub nodes: HashMap<String, ElementNode>,
}

impl GraphRegistry {
    pub fn from_registry(registry: &ElementRegistry) -> Self {
        let mut nodes: HashMap<String, ElementNode> = HashMap::new();

        // Clone all elements into ElementNodes first
        for (id, element) in &registry.elements {
            nodes.insert(
                id.clone(),
                ElementNode {
                    element: element.clone(),
                    relations: Vec::new(),
                },
            );
        }

        // Now resolve relations and build the graph structure
        for (id, node) in nodes.clone() {
            let mut relation_nodes = Vec::new();
            for relation in &node.element.relations {
                if let LinkType::Identifier(ref target_id) = relation.target.link {
                    // Only handle valid forward links
                    if relation.relation_type.direction == RelationDirection::Forward {
                        if let Some(target_node) = nodes.get(target_id) {
                            relation_nodes.push(RelationNode {
                                relation_trigger: relation.relation_type.name.to_string(),
                                element_node: target_node.clone(),
                            });
                        }
                    }
                }
            }

            if let Some(entry) = nodes.get_mut(&id) {
                entry.relations = relation_nodes;
            }
        }

        Self { nodes }
    }

    /// Updates an element's identifier and rewires all incoming relations
    pub fn update_identifier(&mut self, old_id: &str, new_id: &str) {
        if let Some(mut node) = self.nodes.remove(old_id) {
            node.element.identifier = new_id.to_string();

            // Update relations within this element (if any self-refs)
            for relation in &mut node.element.relations {
                if let LinkType::Identifier(ref mut link_id) = relation.target.link {
                    if link_id == old_id {
                        *link_id = new_id.to_string();
                    }
                }
            }

            // Reinsert with new ID
            self.nodes.insert(new_id.to_string(), node);

            // Update all relations pointing to this identifier
            for (_id, other_node) in self.nodes.iter_mut() {
                for relation in &mut other_node.element.relations {
                    if let LinkType::Identifier(ref mut link_id) = relation.target.link {
                        if link_id == old_id {
                            *link_id = new_id.to_string();
                        }
                    }
                }

                for relation_node in &mut other_node.relations {
                    if relation_node.element_node.element.identifier == old_id {
                        relation_node.element_node.element.identifier = new_id.to_string();
                    }
                }
            }
        }
    }

    pub fn get_impact_tree(&self, root_id: &str) -> ElementNode {
        let mut visited = BTreeSet::new();
        self.build_impact_tree_recursive(root_id, &mut visited)
    }

    fn build_impact_tree_recursive(&self, current_id: &str, visited: &mut BTreeSet<String>) -> ElementNode {
        if !visited.insert(current_id.to_string()) {
            // Already visited, stop recursion to prevent cycles
            return self.nodes.get(current_id).unwrap().clone();
        }

        let current_node = self.nodes.get(current_id).unwrap();
        let mut child_nodes = Vec::new();

        for relation_node in &current_node.relations {
            let target_id = &relation_node.element_node.element.identifier;
            let subtree = self.build_impact_tree_recursive(target_id, visited);
            child_nodes.push(RelationNode {
                relation_trigger: relation_node.relation_trigger.clone(),
                element_node: subtree,
            });
        }

        ElementNode {
            element: current_node.element.clone(),
            relations: child_nodes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::{Element, ElementType, RequirementType};
    use crate::relation::{RelationTypeInfo, Relation, RelationTarget, RelationDirection, LinkType};
    use crate::element_registry::ElementRegistry;

    fn make_element(id: &str, name: &str) -> Element {
        let mut element = Element::new(
            name,
            id,
            "file.md",
            "TestSection",
            Some(ElementType::Requirement(RequirementType::System)),
        );
        element.content = format!("This is {}", name);
        element.freeze_content();
        element
    }

    fn add_relation(from: &mut Element, relation_type: &'static str, to_id: &str) {
        let relation_info = RELATION_TYPES.get(relation_type).unwrap();
        from.relations.push(Relation {
            relation_type: relation_info,
            target: RelationTarget {
                text: to_id.to_string(),
                link: LinkType::Identifier(to_id.to_string()),
            },
            is_opposite: false,
        });
    }

    #[test]
    fn test_graph_from_registry_resolves_forward_links() {
        let mut registry = ElementRegistry::new();
        let mut a = make_element("A", "Element A");
        let b = make_element("B", "Element B");

        add_relation(&mut a, "contain", "B");

        registry.register_element(a.clone(), "file.md").unwrap();
        registry.register_element(b.clone(), "file.md").unwrap();

        let graph = GraphRegistry::from_registry(&registry);

        let a_node = graph.nodes.get("A").unwrap();
        assert_eq!(a_node.relations.len(), 1);
        assert_eq!(a_node.relations[0].relation_trigger, "contain");
        assert_eq!(a_node.relations[0].element_node.element.identifier, "B");
    }

    #[test]
    fn test_update_identifier_updates_links_and_graph() {
        let mut registry = ElementRegistry::new();
        let mut a = make_element("A", "Element A");
        let b = make_element("B", "Element B");

        add_relation(&mut a, "contain", "B");

        registry.register_element(a.clone(), "file.md").unwrap();
        registry.register_element(b.clone(), "file.md").unwrap();

        let mut graph = GraphRegistry::from_registry(&registry);
        graph.update_identifier("B", "B_NEW");

        // B should no longer exist, B_NEW should
        assert!(graph.nodes.get("B").is_none());
        assert!(graph.nodes.get("B_NEW").is_some());

        // A’s relation should now point to B_NEW
        let a_node = graph.nodes.get("A").unwrap();
        assert_eq!(a_node.relations.len(), 1);
        assert_eq!(a_node.relations[0].element_node.element.identifier, "B_NEW");
    }

    #[test]
    fn test_get_impact_tree_traverses_correctly() {
        let mut registry = ElementRegistry::new();
        let mut a = make_element("A", "Element A");
        let mut b = make_element("B", "Element B");
        let c = make_element("C", "Element C");

        add_relation(&mut a, "contain", "B");
        add_relation(&mut b, "derive", "C");

        registry.register_element(a.clone(), "file.md").unwrap();
        registry.register_element(b.clone(), "file.md").unwrap();
        registry.register_element(c.clone(), "file.md").unwrap();

        let graph = GraphRegistry::from_registry(&registry);
        let tree = graph.get_impact_tree("A");

        assert_eq!(tree.element.identifier, "A");
        assert_eq!(tree.relations.len(), 1);

        let b_node = &tree.relations[0].element_node;
        assert_eq!(b_node.element.identifier, "B");
        assert_eq!(b_node.relations.len(), 1);
        assert_eq!(b_node.relations[0].element_node.element.identifier, "C");
    }

    #[test]
    fn test_cycle_is_handled_gracefully() {
        let mut registry = ElementRegistry::new();
        let mut a = make_element("A", "Element A");
        let mut b = make_element("B", "Element B");

        // A -> B and B -> A (cycle)
        add_relation(&mut a, "contain", "B");
        add_relation(&mut b, "derive", "A");

        registry.register_element(a.clone(), "file.md").unwrap();
        registry.register_element(b.clone(), "file.md").unwrap();

        let graph = GraphRegistry::from_registry(&registry);
        let tree = graph.get_impact_tree("A");

        assert_eq!(tree.element.identifier, "A");
        assert_eq!(tree.relations.len(), 1);
        assert_eq!(tree.relations[0].element_node.element.identifier, "B");

        // Because of cycle protection, B should not recurse into A again
        assert_eq!(tree.relations[0].element_node.relations.len(), 0);
    }
}

