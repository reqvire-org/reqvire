use crate::element::{Element, ElementType};
use crate::graph_registry::GraphRegistry;
use crate::relation::{VERIFICATION_TRACES_RELATIONS, VERIFY_RELATION};
use rustc_hash::FxHashSet;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
pub struct VerificationTracesReport {
    pub files: BTreeMap<String, FileVerifications>,
}

#[derive(Debug, Serialize)]
pub struct FileVerifications {
    pub verifications: Vec<VerificationTrace>,
}

#[derive(Debug, Serialize)]
pub struct VerificationTrace {
    pub identifier: String,
    pub name: String,
    pub file: String,
    #[serde(rename = "type")]
    pub verification_type: String,
    pub directly_verified_requirements: Vec<String>,
    pub trace_tree: TraceTree,
    pub directly_verified_count: usize,
    pub total_requirements_in_tree: usize,
    #[serde(skip)]
    file_order_index: usize,
}

#[derive(Debug, Serialize)]
pub struct TraceTree {
    pub requirements: Vec<RequirementNode>,
}

#[derive(Debug, Serialize, Clone)]
pub struct RequirementNode {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub element_type: String,
    pub is_directly_verified: bool,
    pub children: Vec<RequirementNode>,
}

pub struct VerificationTraceGenerator<'a> {
    registry: &'a GraphRegistry,
}

impl<'a> VerificationTraceGenerator<'a> {
    pub fn new(registry: &'a GraphRegistry) -> Self {
        Self { registry }
    }

    /// Generate verification traces report
    pub fn generate(&self) -> VerificationTracesReport {
        let mut files: BTreeMap<String, FileVerifications> = BTreeMap::new();

        // Find all verification elements
        for element in self.registry.get_all_elements() {
            if matches!(element.element_type, ElementType::Verification(_)) {
                self.process_verification(element, &mut files);
            }
        }

        // Sort verification data for deterministic output
        // BTreeMap keeps files sorted alphabetically automatically
        for file_verifications in files.values_mut() {
            // Sort directly verified requirements alphabetically
            for verification in &mut file_verifications.verifications {
                verification.directly_verified_requirements.sort();
            }
            // Sort verifications by file_order_index for document order
            file_verifications
                .verifications
                .sort_by_key(|v| v.file_order_index);
        }

        VerificationTracesReport { files }
    }

    /// Process a single verification element
    fn process_verification(
        &self,
        verification: &Element,
        files: &mut BTreeMap<String, FileVerifications>,
    ) {
        // Get directly verified requirements
        let directly_verified: Vec<String> = verification
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
            return; // Skip verifications that don't verify anything
        }

        // Build trace tree
        let trace_tree = self.build_trace_tree(&directly_verified);

        // Count total requirements in tree
        let total_count = self.count_requirements_in_tree(&trace_tree);

        // Create verification trace
        let trace = VerificationTrace {
            identifier: verification.identifier.clone(),
            name: verification.name.clone(),
            file: verification.file_path.clone(),
            verification_type: verification.element_type.as_str().to_string(),
            directly_verified_count: directly_verified.len(),
            directly_verified_requirements: directly_verified.clone(),
            trace_tree,
            total_requirements_in_tree: total_count,
            file_order_index: verification.file_order_index,
        };

        // Add to file-level structure
        let file_entry = files
            .entry(verification.file_path.clone())
            .or_insert_with(|| FileVerifications {
                verifications: Vec::new(),
            });

        file_entry.verifications.push(trace);
    }

    /// Build trace tree from directly verified requirements
    fn build_trace_tree(&self, directly_verified: &[String]) -> TraceTree {
        let mut requirements = Vec::new();
        let mut visited = FxHashSet::default();

        for req_id in directly_verified {
            if let Some(req) = self.registry.get_element(req_id) {
                if let Some(node) = self.build_requirement_node(req, true, &mut visited) {
                    requirements.push(node);
                }
            }
        }

        TraceTree { requirements }
    }

    /// Build a requirement node with its parent chain
    fn build_requirement_node(
        &self,
        requirement: &Element,
        is_directly_verified: bool,
        visited: &mut FxHashSet<String>,
    ) -> Option<RequirementNode> {
        // Prevent cycles
        if visited.contains(&requirement.identifier) {
            return None;
        }
        visited.insert(requirement.identifier.clone());

        let mut children = Vec::new();

        // Find parent relations (those in VERIFICATION_TRACES_RELATIONS)
        for relation in &requirement.relations {
            if VERIFICATION_TRACES_RELATIONS.contains(&relation.relation_type.name) {
                // This is a parent relation, follow it
                if let crate::relation::LinkType::Identifier(parent_id) = &relation.target.link {
                    if let Some(parent) = self.registry.get_element(parent_id) {
                        // Clone visited set for this branch to allow multiple paths
                        let mut branch_visited = visited.clone();
                        if let Some(parent_node) =
                            self.build_requirement_node(parent, false, &mut branch_visited)
                        {
                            children.push(parent_node);
                        }
                    }
                }
            }
        }

        Some(RequirementNode {
            id: requirement.identifier.clone(),
            name: requirement.name.clone(),
            element_type: requirement.element_type.as_str().to_string(),
            is_directly_verified,
            children,
        })
    }

    /// Count total requirements in tree
    fn count_requirements_in_tree(&self, tree: &TraceTree) -> usize {
        let mut count = 0;
        let mut visited = FxHashSet::default();

        for req in &tree.requirements {
            count += self.count_node_and_children(req, &mut visited);
        }

        count
    }

    /// Count node and its children recursively
    #[allow(clippy::only_used_in_recursion)]
    fn count_node_and_children(
        &self,
        node: &RequirementNode,
        visited: &mut FxHashSet<String>,
    ) -> usize {
        if visited.contains(&node.id) {
            return 0;
        }
        visited.insert(node.id.clone());

        let mut count = 1;
        for child in &node.children {
            count += self.count_node_and_children(child, visited);
        }
        count
    }
}

/// Valid verification types for --filter-type in traces command
pub const VERIFICATION_TYPES: &[&str] = &[
    "test-verification",
    "formal-proof-verification",
    "analysis-verification",
    "inspection-verification",
    "demonstration-verification",
];

/// Apply filters to verification traces report
pub fn apply_filters(
    mut report: VerificationTracesReport,
    filter_id: Option<&str>,
    filter_name: Option<&str>,
    filter_type: Option<&str>,
) -> Result<VerificationTracesReport, crate::error::ReqvireError> {
    use regex::Regex;

    // Validate verification type if provided
    if let Some(vtype) = filter_type {
        if !VERIFICATION_TYPES.contains(&vtype.to_lowercase().as_str()) {
            return Err(crate::error::ReqvireError::ProcessError(format!(
                "Invalid verification type '{}'. Valid types: {}",
                vtype,
                VERIFICATION_TYPES.join(", ")
            )));
        }
    }

    // Compile regex if name filter is provided
    let name_regex = if let Some(pattern) = filter_name {
        Some(Regex::new(pattern).map_err(crate::error::ReqvireError::from)?)
    } else {
        None
    };

    // Filter verifications in each file
    for file_verifications in report.files.values_mut() {
        file_verifications.verifications.retain(|v| {
            // Filter by ID
            if let Some(id) = filter_id {
                if v.identifier != id {
                    return false;
                }
            }

            // Filter by name regex
            if let Some(ref regex) = name_regex {
                if !regex.is_match(&v.name) {
                    return false;
                }
            }

            // Filter by type
            if let Some(vtype) = filter_type {
                if v.verification_type != vtype {
                    return false;
                }
            }

            true
        });
    }

    // Remove empty files
    report.files.retain(|_, f| !f.verifications.is_empty());

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verification_trace_structure() {
        // Basic structure test
        let report = VerificationTracesReport {
            files: BTreeMap::new(),
        };
        assert_eq!(report.files.len(), 0);
    }
}
