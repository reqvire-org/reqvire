/// Shared trace tree building logic for redundancy detection
///
/// This module provides reusable functionality to build upward trace trees
/// from requirements and detect redundant relations. Used by both:
/// - verification_trace: for redundant verify relations
/// - lint: for both redundant verify and maybe-redundant hierarchical relations

use crate::element::Element;
use crate::graph_registry::GraphRegistry;
use crate::relation::VERIFICATION_TRACES_RELATIONS;
use std::collections::HashSet;

/// Build upward trace tree from a list of directly linked requirements
/// Returns a list of redundant requirement IDs (ancestors that are also directly linked)
///
/// # Arguments
/// * `directly_linked` - IDs of requirements that are directly linked (e.g., verified by a verification)
/// * `registry` - Graph registry to look up elements
///
/// # Returns
/// A sorted vector of redundant requirement IDs (empty if no redundancies found)
pub fn find_redundant_relations(
    directly_linked: &[String],
    registry: &GraphRegistry,
) -> Vec<String> {
    let mut visited = HashSet::new();
    let mut redundant = HashSet::new();
    let directly_linked_set: HashSet<String> = directly_linked.iter().cloned().collect();

    for req_id in directly_linked {
        if let Some(req) = registry.get_element(req_id) {
            find_redundant_in_tree(
                req,
                &mut visited,
                &directly_linked_set,
                &mut redundant,
                registry,
            );
        }
    }

    // Convert to sorted vector for deterministic output
    let mut redundant_vec: Vec<String> = redundant.into_iter().collect();
    redundant_vec.sort();
    redundant_vec
}

/// Recursively traverse upward through parent relations to find redundancies
fn find_redundant_in_tree(
    requirement: &Element,
    visited: &mut HashSet<String>,
    directly_linked_set: &HashSet<String>,
    redundant: &mut HashSet<String>,
    registry: &GraphRegistry,
) {
    // Prevent cycles
    if visited.contains(&requirement.identifier) {
        return;
    }
    visited.insert(requirement.identifier.clone());

    // Find parent relations (those in VERIFICATION_TRACES_RELATIONS)
    for relation in &requirement.relations {
        if VERIFICATION_TRACES_RELATIONS.contains(&relation.relation_type.name) {
            // This is a parent relation, follow it
            if let crate::relation::LinkType::Identifier(parent_id) = &relation.target.link {
                // Check if parent is also directly linked (making it redundant)
                if directly_linked_set.contains(parent_id) {
                    redundant.insert(parent_id.clone());
                }

                if let Some(parent) = registry.get_element(parent_id) {
                    // Clone visited set for this branch to allow multiple paths
                    let mut branch_visited = visited.clone();
                    find_redundant_in_tree(
                        parent,
                        &mut branch_visited,
                        directly_linked_set,
                        redundant,
                        registry,
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_redundant_relations_empty() {
        use crate::GraphRegistry;
        let registry = GraphRegistry::new();
        let redundant = find_redundant_relations(&[], &registry);
        assert_eq!(redundant.len(), 0);
    }
}
