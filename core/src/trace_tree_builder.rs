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
    // Track which ancestors are reachable from each directly-linked requirement
    let mut reachable_from: std::collections::HashMap<String, HashSet<String>> =
        std::collections::HashMap::new();

    // Build reachability map from each directly-linked requirement
    for req_id in directly_linked {
        if let Some(req) = registry.get_element(req_id) {
            let mut visited = HashSet::new();
            let mut ancestors = HashSet::new();
            collect_ancestors(req, &mut visited, &mut ancestors, registry);
            reachable_from.insert(req_id.clone(), ancestors);
        }
    }

    // Find redundant requirements: a directly-linked requirement is redundant
    // if it's reachable from MULTIPLE (≥2) OTHER directly-linked requirements
    // This detects "branching redundancy" where multiple paths converge on the same node
    let mut redundant = HashSet::new();
    for req_id in directly_linked {
        // Count how many OTHER directly-linked requirements can reach this one
        let mut reachable_count = 0;
        for (other_id, ancestors) in &reachable_from {
            if other_id != req_id && ancestors.contains(req_id) {
                reachable_count += 1;
            }
        }

        // Redundant if reachable from 2 or more other directly-linked requirements
        if reachable_count >= 2 {
            redundant.insert(req_id.clone());
        }
    }

    // Convert to sorted vector for deterministic output
    let mut redundant_vec: Vec<String> = redundant.into_iter().collect();
    redundant_vec.sort();
    redundant_vec
}

/// Recursively collect all ancestor IDs reachable from a requirement
fn collect_ancestors(
    requirement: &Element,
    visited: &mut HashSet<String>,
    ancestors: &mut HashSet<String>,
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
                // Add this parent to ancestors set
                ancestors.insert(parent_id.clone());

                // Recursively collect ancestors of this parent
                if let Some(parent) = registry.get_element(parent_id) {
                    collect_ancestors(parent, visited, ancestors, registry);
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
