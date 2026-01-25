/// Model quality analysis and relation redundancy detection
///
/// This module provides linting functionality to detect issues in requirements relations:
/// - Redundant verify relations (auto-fixable)
/// - Redundant hierarchical relations (auto-fixable)

use crate::element::ElementType;
use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;
use crate::relation::{VERIFY_RELATION, VERIFICATION_TRACES_RELATIONS};
use crate::trace_tree_builder;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Clone)]
pub struct LintReport {
    pub auto_fixable: Vec<AutoFixableIssue>,
    pub needs_manual_review: Vec<ManualReviewIssue>,
}

#[derive(Debug, Serialize, Clone)]
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

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type")]
pub enum ManualReviewIssue {
    #[serde(rename = "maybe_redundant_hierarchical_relations")]
    MaybeRedundantHierarchicalRelations {
        element: ElementInfo,
        potentially_redundant_relations: Vec<RelationInfo>,
        rationale: String,
    },
    #[serde(rename = "multi_branch_convergence")]
    MultiBranchConvergence {
        element: ElementInfo,
        common_ancestor: String,
        branch_paths: Vec<String>,
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
            // Filter the report based on flags before serializing
            let filtered_report = if show_only_fixable {
                LintReport {
                    auto_fixable: self.auto_fixable.clone(),
                    needs_manual_review: vec![],
                }
            } else if show_only_auditable {
                LintReport {
                    auto_fixable: vec![],
                    needs_manual_review: self.needs_manual_review.clone(),
                }
            } else {
                // Show both
                self.clone()
            };
            println!("{}", serde_json::to_string_pretty(&filtered_report).unwrap());
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
                    ManualReviewIssue::MultiBranchConvergence {
                        element,
                        common_ancestor,
                        branch_paths,
                        rationale,
                    } => {
                        println!("### Multi-Branch Convergence\n");
                        println!("**Element: {}**", element.name);
                        println!("File: [{}]({})\n", element.identifier, element.identifier);
                        println!("This element reaches a common ancestor through multiple distinct branch paths:");
                        println!("  * Common ancestor: [{}]({})", common_ancestor, common_ancestor);
                        println!("  * Branch paths:");
                        for path in branch_paths {
                            println!("    - Via: {}", path);
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

    // Detect multi-branch convergence (needs manual review)
    needs_manual_review.extend(detect_multi_branch_convergence(registry));

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
            ManualReviewIssue::MultiBranchConvergence { element, .. } => &element.identifier,
        };
        let id_b = match b {
            ManualReviewIssue::MaybeRedundantHierarchicalRelations { element, .. } => &element.identifier,
            ManualReviewIssue::MultiBranchConvergence { element, .. } => &element.identifier,
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
/// Returns (auto_fixable_issues, unused_manual_review_issues)
/// An element has a redundant hierarchical relation if it has a direct parent relation
/// to both a requirement and an ancestor of that requirement.
///
/// All redundant relations are safe to auto-remove: If an element has a direct relation
/// to a parent AND also reaches that parent through other paths (whether single or multiple),
/// the direct relation is redundant and can be safely removed.
fn detect_hierarchical_redundancies(
    registry: &GraphRegistry,
) -> (Vec<AutoFixableIssue>, Vec<ManualReviewIssue>) {
    let mut safe_issues = Vec::new();
    let unsafe_issues = Vec::new();
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

            // All redundant parents are safe to auto-remove
            // If an element has both a direct relation to a parent AND reaches that parent
            // through other intermediate elements, the direct relation is redundant regardless
            // of whether there's one path or multiple convergent paths
            let mut safe_redundant = redundant_parents.clone();
            // Sort for deterministic output
            safe_redundant.sort();

            // Create safe auto-fixable issues
            if !safe_redundant.is_empty() {
                // Find which intermediate parents lead to each redundant parent for detailed rationale
                let rationale = if safe_redundant.len() == 1 {
                    let redundant_id = &safe_redundant[0];
                    let intermediate_paths: Vec<String> = intermediate_parents
                        .iter()
                        .filter(|parent_id| {
                            path_exists_to_target(
                                parent_id,
                                redundant_id,
                                registry,
                                &mut HashSet::new(),
                            )
                        })
                        .cloned()
                        .collect();

                    if intermediate_paths.len() == 1 {
                        format!(
                            "This element reaches '{}' through a path via '{}'. The direct relation to the ancestor is redundant and safe to remove.",
                            redundant_id,
                            intermediate_paths[0]
                        )
                    } else if intermediate_paths.len() >= 2 {
                        format!(
                            "This element reaches '{}' through multiple paths (via '{}' and '{}'). The direct relation to the ancestor is redundant and safe to remove.",
                            redundant_id,
                            intermediate_paths[0],
                            intermediate_paths[1]
                        )
                    } else {
                        format!(
                            "This element reaches '{}' through intermediate elements. The direct relation to the ancestor is redundant and safe to remove.",
                            redundant_id
                        )
                    }
                } else {
                    let ancestors_list = safe_redundant.join("', '");
                    let intermediates_list = intermediate_parents.join("', '");
                    format!(
                        "This element reaches ancestor requirements ('{}') via intermediate requirements ('{}'). The direct relations to ancestors are redundant and safe to remove.",
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

/// Detect multi-branch convergence where an element reaches a common ancestor
/// through multiple distinct branch paths WITHOUT a direct relation to the ancestor.
///
/// This is different from redundant hierarchical relations (which have a direct relation
/// that can be auto-removed). Multi-branch convergence requires manual review to determine
/// if all branches are semantically necessary or if one represents a modeling error.
fn detect_multi_branch_convergence(registry: &GraphRegistry) -> Vec<ManualReviewIssue> {
    let mut issues = Vec::new();

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

        // Need at least 2 parents to have potential convergence
        if direct_parents.len() < 2 {
            continue;
        }

        // For each direct parent, collect all its ancestors
        let mut parent_ancestors: HashMap<String, HashSet<String>> = HashMap::new();
        for parent_id in &direct_parents {
            let ancestors = collect_ancestors(parent_id, registry);
            parent_ancestors.insert(parent_id.clone(), ancestors);
        }

        // Find common ancestors that are reached through multiple branches
        // We need to find ancestors that appear in multiple parent's ancestor sets
        // BUT we skip ancestors that are direct parents (those are handled by redundant hierarchical detection)
        let mut ancestor_counts: HashMap<String, Vec<String>> = HashMap::new();
        for (parent_id, ancestors) in &parent_ancestors {
            for ancestor_id in ancestors {
                // Skip if this ancestor is also a direct parent
                // (that case is handled by redundant hierarchical relations detection)
                if direct_parents.contains(ancestor_id) {
                    continue;
                }

                ancestor_counts
                    .entry(ancestor_id.clone())
                    .or_insert_with(Vec::new)
                    .push(parent_id.clone());
            }
        }

        // Find ancestors reached through multiple branches (2 or more)
        // Only report the NEAREST common ancestor (fewest hops from element)
        let mut candidates: Vec<(String, Vec<String>)> = ancestor_counts
            .iter()
            .filter(|(_, branches)| branches.len() >= 2)
            .map(|(ancestor, branches)| (ancestor.clone(), branches.clone()))
            .collect();

        // Sort candidates by distance (number of hops) - closest first
        // We want to report only the nearest common ancestor to avoid noise
        candidates.sort_by_key(|(ancestor_id, _)| {
            // Calculate minimum distance from element to this ancestor through any branch
            let mut min_distance = usize::MAX;
            for parent in &direct_parents {
                if let Some(dist) = distance_to_ancestor(parent, ancestor_id, registry) {
                    min_distance = min_distance.min(dist + 1); // +1 for edge from element to parent
                }
            }
            min_distance
        });

        // Find the first candidate that meets all criteria
        for (ancestor_id, branch_parents) in &candidates {
            // Only report if the ancestor is NOT a direct parent
            // (if it is a direct parent, that's handled by redundant hierarchical relations detection)
            if direct_parents.contains(ancestor_id) {
                continue;
            }

            // Get all ancestors of the common ancestor
            let ancestor_ancestors = collect_ancestors(ancestor_id, registry);

            // Filter branch paths to only include those that are DIRECT parents
            // AND that are NOT themselves ancestors of the common ancestor
            // (we want leaf branches, not intermediate ancestors)
            let direct_branch_paths: Vec<String> = branch_parents
                .iter()
                .filter(|branch| {
                    // Must be a direct parent
                    if !direct_parents.contains(*branch) {
                        return false;
                    }
                    // Must NOT be an ancestor of the common ancestor
                    // (check if this branch appears in the common ancestor's ancestors)
                    if ancestor_ancestors.contains(*branch) {
                        return false;  // Skip this branch - it's an ancestor of the common ancestor
                    }
                    true
                })
                .cloned()
                .collect();

            // Only report if there are at least 2 direct branch paths
            if direct_branch_paths.len() < 2 {
                continue;
            }

            // Check if ALL branch paths have the common ancestor as a direct parent
            // (this is the ideal case - all branches are immediate children of the common ancestor)
            // If NOT all branches have it as a direct parent, skip and look for a closer ancestor
            let all_branches_are_direct_children = direct_branch_paths.iter().all(|branch_path| {
                if let Some(branch_element) = registry.get_element(branch_path) {
                    for rel in &branch_element.relations {
                        if VERIFICATION_TRACES_RELATIONS.contains(&rel.relation_type.name) {
                            if let crate::relation::LinkType::Identifier(parent_id) = &rel.target.link {
                                if parent_id == ancestor_id {
                                    return true;  // This branch has the ancestor as a direct parent
                                }
                            }
                        }
                    }
                }
                false  // This branch does NOT have the ancestor as a direct parent
            });

            if !all_branches_are_direct_children {
                continue;  // Skip this candidate - not all branches are direct children
            }

            // This is a valid multi-branch convergence - report it
            let mut branch_paths = direct_branch_paths.clone();
            branch_paths.sort();

            issues.push(ManualReviewIssue::MultiBranchConvergence {
                element: ElementInfo {
                    identifier: element.identifier.clone(),
                    name: element.name.clone(),
                    file: element.file_path.clone(),
                },
                common_ancestor: ancestor_id.clone(),
                branch_paths,
                rationale: "This element reaches the common ancestor through multiple branches. Both branches may be semantically necessary (element derives from ancestor in multiple contexts), OR one branch may represent a modeling error. Human review is required to determine if all branches are valid.".to_string(),
            });

            // Only report the nearest valid common ancestor
            break;
        }
    }

    issues
}

/// Calculate the distance (number of hops) from source to target through hierarchical relations
/// Returns None if no path exists
fn distance_to_ancestor(
    current_id: &str,
    target_id: &str,
    registry: &GraphRegistry,
) -> Option<usize> {
    let mut visited = HashSet::new();
    distance_to_ancestor_recursive(current_id, target_id, registry, &mut visited, 0)
}

/// Recursive helper for distance calculation
fn distance_to_ancestor_recursive(
    current_id: &str,
    target_id: &str,
    registry: &GraphRegistry,
    visited: &mut HashSet<String>,
    current_distance: usize,
) -> Option<usize> {
    // Prevent cycles
    if visited.contains(current_id) {
        return None;
    }
    visited.insert(current_id.to_string());

    // Check if we've reached the target
    if current_id == target_id {
        return Some(current_distance);
    }

    // Get current element and check its parents
    if let Some(element) = registry.get_element(current_id) {
        let mut min_distance: Option<usize> = None;

        for relation in &element.relations {
            if VERIFICATION_TRACES_RELATIONS.contains(&relation.relation_type.name) {
                if let crate::relation::LinkType::Identifier(parent_id) = &relation.target.link {
                    if let Some(dist) = distance_to_ancestor_recursive(
                        parent_id,
                        target_id,
                        registry,
                        visited,
                        current_distance + 1,
                    ) {
                        min_distance = Some(min_distance.map_or(dist, |d| d.min(dist)));
                    }
                }
            }
        }

        return min_distance;
    }

    None
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
