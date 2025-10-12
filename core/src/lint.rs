/// Model quality analysis and relation redundancy detection
///
/// This module provides linting functionality to detect issues in requirements relations:
/// - Redundant verify relations (auto-fixable)
/// - Maybe-redundant hierarchical relations (manual review needed)

use crate::element::ElementType;
use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;
use crate::relation::{VERIFY_RELATION, VERIFICATION_TRACES_RELATIONS};
use crate::trace_tree_builder;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize)]
pub struct LintReport {
    pub auto_fixable: Vec<AutoFixableIssue>,
    pub needs_manual_review: Vec<ManualReviewIssue>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum AutoFixableIssue {
    #[serde(rename = "redundant_verify_relations")]
    RedundantVerifyRelations {
        verification: ElementInfo,
        redundant_relations: Vec<RelationInfo>,
        rationale: String,
    },
    #[serde(rename = "safe_redundant_hierarchical_relations")]
    SafeRedundantHierarchicalRelations {
        element: ElementInfo,
        redundant_relations: Vec<RelationInfo>,
        rationale: String,
    },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ManualReviewIssue {
    #[serde(rename = "maybe_redundant_hierarchical_relations")]
    MaybeRedundantHierarchicalRelations {
        element: ElementInfo,
        potentially_redundant_relations: Vec<RelationInfo>,
        rationale: String,
    },
}

#[derive(Debug, Serialize, Clone)]
pub struct ElementInfo {
    pub identifier: String,
    pub name: String,
    pub file: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct RelationInfo {
    pub relation_type: String,
    pub target: String,
}

impl LintReport {
    pub fn print(&self, json: bool, show_only_fixable: bool, show_only_auditable: bool) {
        if json {
            println!("{}", serde_json::to_string_pretty(self).unwrap());
        } else {
            self.print_text(show_only_fixable, show_only_auditable);
        }
    }

    fn print_text(&self, show_only_fixable: bool, show_only_auditable: bool) {
        let show_fixable = !show_only_auditable;
        let show_auditable = !show_only_fixable;

        // Auto-fixable Issues
        if show_fixable && !self.auto_fixable.is_empty() {
            println!("## Auto-fixable Issues\n");
            for issue in &self.auto_fixable {
                match issue {
                    AutoFixableIssue::RedundantVerifyRelations {
                        verification,
                        redundant_relations,
                        rationale,
                    } => {
                        println!("### Redundant Verify Relations\n");
                        println!("**Verification: {}**", verification.name);
                        println!("File: [{}]({})\n", verification.identifier, verification.identifier);
                        println!("Redundant verify relations (these can be automatically removed):");
                        for rel in redundant_relations {
                            println!("  * {}: [{}]({})", rel.relation_type, rel.target, rel.target);
                        }
                        println!("\nReason: {}\n", rationale);
                        println!("---\n");
                    }
                    AutoFixableIssue::SafeRedundantHierarchicalRelations {
                        element,
                        redundant_relations,
                        rationale,
                    } => {
                        println!("### Safe Redundant Hierarchical Relations\n");
                        println!("**Element: {}**", element.name);
                        println!("File: [{}]({})\n", element.identifier, element.identifier);
                        println!("Safe redundant {} relations (these can be automatically removed):",
                            if redundant_relations.len() == 1 {
                                redundant_relations[0].relation_type.as_str()
                            } else {
                                "hierarchical"
                            }
                        );
                        for rel in redundant_relations {
                            println!("  * {}: [{}]({})", rel.relation_type, rel.target, rel.target);
                        }
                        println!("\nReason: {}\n", rationale);
                        println!("---\n");
                    }
                }
            }
        }

        // Needs Manual Review
        if show_auditable && !self.needs_manual_review.is_empty() {
            println!("## Needs Manual Review\n");
            for issue in &self.needs_manual_review {
                match issue {
                    ManualReviewIssue::MaybeRedundantHierarchicalRelations {
                        element,
                        potentially_redundant_relations,
                        rationale,
                    } => {
                        println!("### Maybe-Redundant Hierarchical Relations\n");
                        println!("**Element: {}**", element.name);
                        println!("File: [{}]({})\n", element.identifier, element.identifier);
                        println!("Potentially redundant {} relations detected:",
                            if potentially_redundant_relations.len() == 1 {
                                potentially_redundant_relations[0].relation_type.as_str()
                            } else {
                                "hierarchical"
                            }
                        );
                        for rel in potentially_redundant_relations {
                            println!("  * {}: [{}]({})", rel.relation_type, rel.target, rel.target);
                        }
                        println!("\nReason: {}\n", rationale);
                        println!("---\n");
                    }
                }
            }
        }
    }

    /// Apply automatic fixes for auto-fixable issues
    /// Processes RedundantVerifyRelations and SafeRedundantHierarchicalRelations issues
    /// Returns the number of relations removed
    pub fn apply_fixes(&self, registry: &mut GraphRegistry) -> Result<usize, ReqvireError> {
        let mut relations_removed = 0;

        // Process all auto-fixable issues
        for issue in &self.auto_fixable {
            match issue {
                AutoFixableIssue::RedundantVerifyRelations {
                    verification,
                    redundant_relations,
                    rationale: _,
                } => {
                    // Remove each redundant verify relation
                    for rel in redundant_relations {
                        match registry.remove_element_relation(
                            &verification.identifier,
                            &rel.target,
                            &rel.relation_type,
                        ) {
                            Ok(()) => {
                                relations_removed += 1;
                            }
                            Err(e) => {
                                // Log error but continue with other relations
                                eprintln!(
                                    "Warning: Failed to remove relation '{}' from '{}' to '{}': {}",
                                    rel.relation_type, verification.identifier, rel.target, e
                                );
                            }
                        }
                    }
                }
                AutoFixableIssue::SafeRedundantHierarchicalRelations {
                    element,
                    redundant_relations,
                    rationale: _,
                } => {
                    // Remove each safe redundant hierarchical relation
                    for rel in redundant_relations {
                        match registry.remove_element_relation(
                            &element.identifier,
                            &rel.target,
                            &rel.relation_type,
                        ) {
                            Ok(()) => {
                                relations_removed += 1;
                            }
                            Err(e) => {
                                // Log error but continue with other relations
                                eprintln!(
                                    "Warning: Failed to remove relation '{}' from '{}' to '{}': {}",
                                    rel.relation_type, element.identifier, rel.target, e
                                );
                            }
                        }
                    }
                }
            }
        }

        Ok(relations_removed)
    }
}

/// Analyze model for quality issues
pub fn analyze_model(registry: &GraphRegistry) -> LintReport {
    let mut auto_fixable = Vec::new();
    let mut needs_manual_review = Vec::new();

    // Detect redundant verify relations
    auto_fixable.extend(detect_redundant_verify_relations(registry));

    // Detect hierarchical relation redundancies (both safe and unsafe)
    let (safe_issues, unsafe_issues) = detect_hierarchical_redundancies(registry);
    auto_fixable.extend(safe_issues);
    needs_manual_review.extend(unsafe_issues);

    // Sort issues by element identifier for deterministic output
    auto_fixable.sort_by(|a, b| {
        let id_a = match a {
            AutoFixableIssue::RedundantVerifyRelations { verification, .. } => &verification.identifier,
            AutoFixableIssue::SafeRedundantHierarchicalRelations { element, .. } => &element.identifier,
        };
        let id_b = match b {
            AutoFixableIssue::RedundantVerifyRelations { verification, .. } => &verification.identifier,
            AutoFixableIssue::SafeRedundantHierarchicalRelations { element, .. } => &element.identifier,
        };
        id_a.cmp(id_b)
    });

    needs_manual_review.sort_by(|a, b| {
        let id_a = match a {
            ManualReviewIssue::MaybeRedundantHierarchicalRelations { element, .. } => &element.identifier,
        };
        let id_b = match b {
            ManualReviewIssue::MaybeRedundantHierarchicalRelations { element, .. } => &element.identifier,
        };
        id_a.cmp(id_b)
    });

    LintReport {
        auto_fixable,
        needs_manual_review,
    }
}

/// Detect redundant verify relations in verifications
fn detect_redundant_verify_relations(registry: &GraphRegistry) -> Vec<AutoFixableIssue> {
    let mut issues = Vec::new();

    // Find all verification elements
    for element in registry.get_all_elements() {
        if matches!(element.element_type, ElementType::Verification(_)) {
            // Get directly verified requirements
            let directly_verified: Vec<String> = element
                .relations
                .iter()
                .filter(|rel| rel.relation_type.name == VERIFY_RELATION)
                .filter_map(|rel| {
                    if let crate::relation::LinkType::Identifier(id) = &rel.target.link {
                        Some(id.clone())
                    } else {
                        None
                    }
                })
                .collect();

            if directly_verified.is_empty() {
                continue;
            }

            // Use shared trace tree builder to find redundant relations
            let redundant_ids = trace_tree_builder::find_redundant_relations(&directly_verified, registry);

            if !redundant_ids.is_empty() {
                // Find the leaf requirement(s) for rationale
                let leaf_reqs: Vec<String> = directly_verified
                    .iter()
                    .filter(|id| !redundant_ids.contains(id))
                    .cloned()
                    .collect();

                // Get the hierarchical relation name from VERIFICATION_TRACES_RELATIONS
                let hierarchical_relation = VERIFICATION_TRACES_RELATIONS[0];

                let rationale = if leaf_reqs.len() == 1 {
                    format!(
                        "This verification already verifies '{}' which derives from '{}'. Verification traces automatically roll up through {} relations, so verifying the leaf requirement is sufficient.",
                        leaf_reqs[0],
                        redundant_ids[0],
                        hierarchical_relation
                    )
                } else {
                    format!(
                        "This verification verifies leaf requirements which derive from parent requirements also directly verified. Verification traces automatically roll up through {} relations, so verifying the leaf requirements is sufficient.",
                        hierarchical_relation
                    )
                };

                issues.push(AutoFixableIssue::RedundantVerifyRelations {
                    verification: ElementInfo {
                        identifier: element.identifier.clone(),
                        name: element.name.clone(),
                        file: element.file_path.clone(),
                    },
                    redundant_relations: redundant_ids
                        .iter()
                        .map(|id| RelationInfo {
                            relation_type: VERIFY_RELATION.to_string(),
                            target: id.clone(),
                        })
                        .collect(),
                    rationale,
                });
            }
        }
    }

    issues
}

/// Detect hierarchical relation redundancies
/// Returns (safe_auto_fixable_issues, unsafe_manual_review_issues)
/// An element has a redundant hierarchical relation if it has a direct parent relation
/// to both a requirement and an ancestor of that requirement.
///
/// Safe to auto-remove (single chain): Element → Parent → Ancestor with direct Element → Ancestor
/// Unsafe (multi-path): Element reaches Ancestor through multiple different paths
fn detect_hierarchical_redundancies(
    registry: &GraphRegistry,
) -> (Vec<AutoFixableIssue>, Vec<ManualReviewIssue>) {
    let mut safe_issues = Vec::new();
    let mut unsafe_issues = Vec::new();
    let hierarchical_relation = VERIFICATION_TRACES_RELATIONS[0];

    // Check each element
    for element in registry.get_all_elements() {
        // Skip verifications and files
        if matches!(element.element_type, ElementType::Verification(_))
            || matches!(element.element_type, ElementType::File)
        {
            continue;
        }

        // Get all direct parent relations for this element
        let direct_parents: Vec<String> = element
            .relations
            .iter()
            .filter(|rel| VERIFICATION_TRACES_RELATIONS.contains(&rel.relation_type.name))
            .filter_map(|rel| {
                if let crate::relation::LinkType::Identifier(id) = &rel.target.link {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect();

        // Need at least 2 parents to have potential redundancy
        if direct_parents.len() < 2 {
            continue;
        }

        // For each direct parent, collect all its ancestors
        let mut parent_ancestors: HashMap<String, HashSet<String>> = HashMap::new();
        for parent_id in &direct_parents {
            let ancestors = collect_ancestors(parent_id, registry);
            parent_ancestors.insert(parent_id.clone(), ancestors);
        }

        // Check if any direct parent is an ancestor of another direct parent
        let mut redundant_parents = Vec::new();
        for parent_id in &direct_parents {
            // Check if this parent appears as an ancestor of any other parent
            for (other_parent_id, other_ancestors) in &parent_ancestors {
                if parent_id != other_parent_id && other_ancestors.contains(parent_id) {
                    // This parent is redundant because we already have a relation to a descendant
                    if !redundant_parents.contains(parent_id) {
                        redundant_parents.push(parent_id.clone());
                    }
                }
            }
        }
        // Sort for deterministic output
        redundant_parents.sort();

        if !redundant_parents.is_empty() {
            // Find non-redundant parents (intermediate elements) for the rationale
            let mut intermediate_parents: Vec<String> = direct_parents
                .iter()
                .filter(|id| !redundant_parents.contains(id))
                .cloned()
                .collect();
            // Sort for deterministic output
            intermediate_parents.sort();

            // For each redundant parent, check if it's safe to auto-remove (single chain)
            // or needs manual review (multi-path)
            let mut safe_redundant = Vec::new();
            let mut unsafe_redundant = Vec::new();

            for redundant_parent_id in &redundant_parents {
                // Count paths from element to this redundant parent through intermediates
                let path_count = count_paths_to_target(
                    &element.identifier,
                    redundant_parent_id,
                    &direct_parents,
                    registry,
                );

                if path_count == 1 {
                    // Single chain - safe to auto-remove
                    safe_redundant.push(redundant_parent_id.clone());
                } else {
                    // Multiple paths - needs manual review
                    unsafe_redundant.push(redundant_parent_id.clone());
                }
            }
            // Sort for deterministic output
            safe_redundant.sort();
            unsafe_redundant.sort();

            // Create safe auto-fixable issues
            if !safe_redundant.is_empty() {
                let rationale = if intermediate_parents.len() == 1 && safe_redundant.len() == 1 {
                    format!(
                        "This element reaches '{}' through a single chain via '{}'. The direct relation to the ancestor is redundant and safe to remove.",
                        safe_redundant[0],
                        intermediate_parents[0]
                    )
                } else {
                    let ancestors_list = safe_redundant.join("', '");
                    let intermediates_list = intermediate_parents.join("', '");
                    format!(
                        "This element reaches ancestor requirements ('{}') through single chains via intermediate requirements ('{}'). The direct relations to ancestors are redundant and safe to remove.",
                        ancestors_list,
                        intermediates_list
                    )
                };

                safe_issues.push(AutoFixableIssue::SafeRedundantHierarchicalRelations {
                    element: ElementInfo {
                        identifier: element.identifier.clone(),
                        name: element.name.clone(),
                        file: element.file_path.clone(),
                    },
                    redundant_relations: safe_redundant
                        .iter()
                        .map(|id| RelationInfo {
                            relation_type: hierarchical_relation.to_string(),
                            target: id.clone(),
                        })
                        .collect(),
                    rationale,
                });
            }

            // Create unsafe manual review issues
            if !unsafe_redundant.is_empty() {
                // Find which intermediate parents lead to this redundant parent
                let intermediate_paths: Vec<String> = intermediate_parents
                    .iter()
                    .filter(|parent_id| {
                        path_exists_to_target(
                            parent_id,
                            &unsafe_redundant[0],
                            registry,
                            &mut HashSet::new(),
                        )
                    })
                    .cloned()
                    .collect();

                let rationale = if intermediate_paths.len() >= 2 && unsafe_redundant.len() == 1 {
                    format!(
                        "This element reaches '{}' through multiple paths (via '{}' and '{}'). Multiple paths may have semantic meaning. Manual review required to determine if the direct relation should be removed.",
                        unsafe_redundant[0],
                        intermediate_paths.get(0).unwrap_or(&"unknown".to_string()),
                        intermediate_paths.get(1).unwrap_or(&"unknown".to_string())
                    )
                } else {
                    format!(
                        "This element reaches ancestor requirements through multiple paths. Multiple paths may have semantic meaning. Manual review required to determine if the direct relations should be removed."
                    )
                };

                unsafe_issues.push(ManualReviewIssue::MaybeRedundantHierarchicalRelations {
                    element: ElementInfo {
                        identifier: element.identifier.clone(),
                        name: element.name.clone(),
                        file: element.file_path.clone(),
                    },
                    potentially_redundant_relations: unsafe_redundant
                        .iter()
                        .map(|id| RelationInfo {
                            relation_type: hierarchical_relation.to_string(),
                            target: id.clone(),
                        })
                        .collect(),
                    rationale,
                });
            }
        }
    }

    (safe_issues, unsafe_issues)
}

/// Collect all ancestors of a requirement by traversing upward through hierarchical relations
fn collect_ancestors(requirement_id: &str, registry: &GraphRegistry) -> HashSet<String> {
    let mut ancestors = HashSet::new();
    let mut visited = HashSet::new();
    collect_ancestors_recursive(requirement_id, registry, &mut ancestors, &mut visited);
    ancestors
}

/// Recursively collect ancestors
fn collect_ancestors_recursive(
    requirement_id: &str,
    registry: &GraphRegistry,
    ancestors: &mut HashSet<String>,
    visited: &mut HashSet<String>,
) {
    // Prevent cycles
    if visited.contains(requirement_id) {
        return;
    }
    visited.insert(requirement_id.to_string());

    if let Some(requirement) = registry.get_element(requirement_id) {
        // Find all parent relations
        for relation in &requirement.relations {
            if VERIFICATION_TRACES_RELATIONS.contains(&relation.relation_type.name) {
                if let crate::relation::LinkType::Identifier(parent_id) = &relation.target.link {
                    ancestors.insert(parent_id.clone());
                    // Recursively collect ancestors of this parent
                    collect_ancestors_recursive(parent_id, registry, ancestors, visited);
                }
            }
        }
    }
}

/// Count the number of distinct paths from source to target through intermediate elements
/// Returns the count of unique paths found
fn count_paths_to_target(
    _source_id: &str,
    target_id: &str,
    direct_parents: &[String],
    registry: &GraphRegistry,
) -> usize {
    let mut path_count = 0;
    let visited = HashSet::new();

    // Check each direct parent to see if it leads to the target
    for parent_id in direct_parents {
        if parent_id == target_id {
            // Direct connection doesn't count as a path through intermediate
            continue;
        }

        // Check if this parent leads to the target
        if path_exists_to_target(parent_id, target_id, registry, &mut visited.clone()) {
            path_count += 1;
        }
    }

    path_count
}

/// Check if a path exists from source to target through hierarchical relations
fn path_exists_to_target(
    current_id: &str,
    target_id: &str,
    registry: &GraphRegistry,
    visited: &mut HashSet<String>,
) -> bool {
    // Prevent cycles
    if visited.contains(current_id) {
        return false;
    }
    visited.insert(current_id.to_string());

    // Check if we've reached the target
    if current_id == target_id {
        return true;
    }

    // Get current element and check its parents
    if let Some(element) = registry.get_element(current_id) {
        for relation in &element.relations {
            if VERIFICATION_TRACES_RELATIONS.contains(&relation.relation_type.name) {
                if let crate::relation::LinkType::Identifier(parent_id) = &relation.target.link {
                    if path_exists_to_target(parent_id, target_id, registry, visited) {
                        return true;
                    }
                }
            }
        }
    }

    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lint_report_structure() {
        let report = LintReport {
            auto_fixable: vec![],
            needs_manual_review: vec![],
        };
        assert_eq!(report.auto_fixable.len(), 0);
        assert_eq!(report.needs_manual_review.len(), 0);
    }
}
