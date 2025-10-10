use crate::element::{Element, ElementType};
use crate::git_commands;
use crate::graph_registry::GraphRegistry;
use crate::relation::{VERIFY_RELATION, VERIFICATION_TRACES_RELATIONS};
use crate::utils;
use serde::Serialize;
use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct VerificationTracesReport {
    pub files: BTreeMap<String, FileVerifications>,
}

#[derive(Debug, Serialize)]
pub struct FileVerifications {
    pub sections: BTreeMap<String, SectionVerifications>,
}

#[derive(Debug, Serialize)]
pub struct SectionVerifications {
    pub verifications: Vec<VerificationTrace>,
}

#[derive(Debug, Serialize)]
pub struct VerificationTrace {
    pub identifier: String,
    pub name: String,
    pub section: String,
    pub file: String,
    #[serde(rename = "type")]
    pub verification_type: String,
    pub directly_verified_requirements: Vec<String>,
    pub trace_tree: TraceTree,
    pub directly_verified_count: usize,
    pub total_requirements_in_tree: usize,
    #[serde(skip)]
    section_order_index: usize,
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

// Internal structure that includes relation type for diagram generation
#[derive(Debug, Clone)]
struct RequirementNodeWithRelation {
    pub id: String,
    pub name: String,
    pub element_type: String,
    pub is_directly_verified: bool,
    pub children: Vec<(String, RequirementNodeWithRelation)>, // (relation_type, node)
}

pub struct VerificationTraceGenerator<'a> {
    registry: &'a GraphRegistry,
    diagrams_with_blobs: bool,
    from_folder: Option<String>,
}

impl<'a> VerificationTraceGenerator<'a> {
    pub fn new(registry: &'a GraphRegistry, diagrams_with_blobs: bool, from_folder: Option<String>) -> Self {
        Self { registry, diagrams_with_blobs, from_folder }
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
        // BTreeMap keeps files and sections sorted alphabetically automatically
        for file_verifications in files.values_mut() {
            for section_verifications in file_verifications.sections.values_mut() {
                // Sort directly verified requirements alphabetically
                for verification in &mut section_verifications.verifications {
                    verification.directly_verified_requirements.sort();
                }
                // Sort verifications alphabetically by identifier
                section_verifications.verifications.sort_by(|a, b| a.identifier.cmp(&b.identifier));
            }
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
            section: verification.section.clone(),
            file: verification.file_path.clone(),
            verification_type: verification.element_type.as_str().to_string(),
            directly_verified_count: directly_verified.len(),
            directly_verified_requirements: directly_verified.clone(),
            trace_tree,
            total_requirements_in_tree: total_count,
            section_order_index: verification.section_order_index,
        };

        // Add to hierarchical structure
        let file_entry = files
            .entry(verification.file_path.clone())
            .or_insert_with(|| FileVerifications {
                sections: BTreeMap::new(),
            });

        let section_entry = file_entry
            .sections
            .entry(verification.section.clone())
            .or_insert_with(|| SectionVerifications {
                verifications: Vec::new(),
            });

        section_entry.verifications.push(trace);
    }

    /// Build trace tree from directly verified requirements
    fn build_trace_tree(&self, directly_verified: &[String]) -> TraceTree {
        let mut requirements = Vec::new();
        let mut visited = HashSet::new();

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
        visited: &mut HashSet<String>,
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
        let mut visited = HashSet::new();

        for req in &tree.requirements {
            count += self.count_node_and_children(req, &mut visited);
        }

        count
    }

    /// Count node and its children recursively
    fn count_node_and_children(&self, node: &RequirementNode, visited: &mut HashSet<String>) -> usize {
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

    /// Generate Mermaid diagram for a verification trace
    pub fn generate_mermaid_diagram(&self, trace: &VerificationTrace) -> String {
        let mut diagram = String::new();

        // Get Git repository information for creating proper links
        let repo_root = match git_commands::get_git_root_dir() {
            Ok(root) => root,
            Err(_) => PathBuf::from(""),
        };

        let base_url = match git_commands::get_repository_base_url() {
            Ok(url) => url,
            Err(_) => String::new(),
        };

        let commit_hash = match git_commands::get_commit_hash() {
            Ok(hash) => hash,
            Err(_) => String::new(),
        };

        let has_git_info = !repo_root.as_os_str().is_empty()
            && !base_url.is_empty()
            && !commit_hash.is_empty();

        // Header with CSS classes (reused from diagrams.rs pattern)
        diagram.push_str("```mermaid\n");
        diagram.push_str("graph TD\n");
        diagram.push_str("  classDef verified fill:#90EE90,stroke:#000,stroke-width:2px;\n");
        diagram.push_str("  classDef requirement fill:#87CEEB,stroke:#000,stroke-width:1px;\n");
        diagram.push_str("  classDef verification fill:#FFD700,stroke:#000,stroke-width:2px;\n");
        diagram.push_str("\n");

        // Add verification node at the top
        let verification_id = utils::hash_identifier(&trace.identifier);
        diagram.push_str(&format!(
            "  {}[\"{}<br>({})\"]:::verification\n",
            verification_id, trace.name, trace.verification_type
        ));

        // Add click handler for verification node
        let verification_click_target = if self.diagrams_with_blobs && has_git_info {
            // Use GitHub blob URLs when diagrams_with_blobs is enabled
            let relative_id = match utils::get_relative_path(&PathBuf::from(&trace.identifier)) {
                Ok(rel_path) => rel_path.to_string_lossy().to_string(),
                Err(_) => trace.identifier.clone(),
            };
            format!("{}/blob/{}/{}", base_url, commit_hash, relative_id)
        } else if let Some(ref from_folder) = self.from_folder {
            // Special case: "/" means reqvire root (git root), use identifier as-is
            if from_folder == "/" {
                trace.identifier.clone()
            } else {
                // Calculate relative path from from_folder to the identifier
                let from_folder_path = repo_root.join(from_folder);
                match utils::to_relative_identifier(&trace.identifier, &from_folder_path, false) {
                    Ok(rel_path) => rel_path,
                    Err(_) => trace.identifier.clone(),
                }
            }
        } else {
            // Use identifier as-is (default behavior)
            trace.identifier.clone()
        };
        diagram.push_str(&format!("  click {} \"{}\";\n", verification_id, verification_click_target));

        // Build tree with relation information
        let mut visited = HashSet::new();
        let mut tree_with_relations = Vec::new();

        for req_id in &trace.directly_verified_requirements {
            if let Some(req) = self.registry.get_element(req_id) {
                if let Some(node) = self.build_node_with_relations(req, true, &mut visited) {
                    tree_with_relations.push(node);
                }
            }
        }

        // Add all requirements from trace tree
        let mut visited_nodes = HashSet::new();
        let mut visited_edges = HashSet::new();
        for node in &tree_with_relations {
            self.add_node_to_diagram_with_relations(
                node,
                &verification_id,
                VERIFY_RELATION, // Connection from verification to verified requirement
                &mut diagram,
                &mut visited_nodes,
                &mut visited_edges,
                &repo_root,
                &base_url,
                &commit_hash,
                has_git_info,
            );
        }

        diagram.push_str("```\n");
        diagram
    }

    /// Build a requirement node with relation information
    fn build_node_with_relations(
        &self,
        requirement: &Element,
        is_directly_verified: bool,
        visited: &mut HashSet<String>,
    ) -> Option<RequirementNodeWithRelation> {
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
                        let mut branch_visited = visited.clone();
                        if let Some(parent_node) =
                            self.build_node_with_relations(parent, false, &mut branch_visited)
                        {
                            // Store relation type with the node
                            children.push((relation.relation_type.name.to_string(), parent_node));
                        }
                    }
                }
            }
        }

        Some(RequirementNodeWithRelation {
            id: requirement.identifier.clone(),
            name: requirement.name.clone(),
            element_type: requirement.element_type.as_str().to_string(),
            is_directly_verified,
            children,
        })
    }

    /// Add requirement node and its children to diagram with proper relation arrows
    fn add_node_to_diagram_with_relations(
        &self,
        node: &RequirementNodeWithRelation,
        source_id: &str,
        relation_type_name: &str,
        diagram: &mut String,
        visited_nodes: &mut HashSet<String>,
        visited_edges: &mut HashSet<(String, String, String)>,
        repo_root: &PathBuf,
        base_url: &str,
        commit_hash: &str,
        has_git_info: bool,
    ) {
        let node_id = utils::hash_identifier(&node.id);

        // Add node if not already visited
        if !visited_nodes.contains(&node.id) {
            visited_nodes.insert(node.id.clone());

            let class = if node.is_directly_verified {
                "verified"
            } else {
                "requirement"
            };

            diagram.push_str(&format!(
                "  {}[\"{}<br>({})\"]:::{}\n",
                node_id, node.name, node.element_type, class
            ));

            // Add click handler for requirement node
            let click_target = if self.diagrams_with_blobs && has_git_info {
                let relative_id = match utils::get_relative_path(&PathBuf::from(&node.id)) {
                    Ok(rel_path) => rel_path.to_string_lossy().to_string(),
                    Err(_) => node.id.clone(),
                };
                format!("{}/blob/{}/{}", base_url, commit_hash, relative_id)
            } else if let Some(ref from_folder) = self.from_folder {
                // Special case: "/" means reqvire root (git root), use identifier as-is
                if from_folder == "/" {
                    node.id.clone()
                } else {
                    // Calculate relative path from from_folder to the identifier
                    let from_folder_path = repo_root.join(from_folder);
                    match utils::to_relative_identifier(&node.id, &from_folder_path, false) {
                        Ok(rel_path) => rel_path,
                        Err(_) => node.id.clone(),
                    }
                }
            } else {
                node.id.clone()
            };
            diagram.push_str(&format!("  click {} \"{}\";\n", node_id, click_target));
        }

        // Add link from source to this node using proper relation metadata
        // Track edges to avoid duplicates (source, relation_type, target)
        let edge_key = (source_id.to_string(), relation_type_name.to_string(), node_id.clone());
        if !visited_edges.contains(&edge_key) {
            visited_edges.insert(edge_key);

            if let Some(info) = crate::relation::RELATION_TYPES.get(relation_type_name) {
                // Always render as element → target
                diagram.push_str(&format!(
                    "  {} {}|{}| {};\n",
                    source_id,
                    info.arrow,
                    info.label,
                    node_id,
                ));
            }
        }

        // Recursively add children
        for (child_relation_type, child) in &node.children {
            self.add_node_to_diagram_with_relations(
                child,
                &node_id,
                child_relation_type,
                diagram,
                visited_nodes,
                visited_edges,
                repo_root,
                base_url,
                commit_hash,
                has_git_info,
            );
        }
    }

    /// Generate Markdown report with Mermaid diagrams
    pub fn generate_markdown(&self, report: &VerificationTracesReport) -> String {
        let mut markdown = String::new();

        markdown.push_str("# Verification Traces Report\n\n");

        // Files are already sorted alphabetically by BTreeMap
        for (file_path, file_verifications) in &report.files {
            markdown.push_str(&format!("## File: {}\n\n", file_path));

            // Sort sections by section_order from registry for deterministic output
            let mut sorted_sections: Vec<_> = file_verifications.sections.iter().collect();
            sorted_sections.sort_by_key(|(section_name, _)| {
                let section_key = crate::graph_registry::SectionKey::new(file_path.clone(), section_name.to_string());
                self.registry.sections.get(&section_key).map(|s| s.section_order).unwrap_or(0)
            });

            for (section_name, section_verifications) in sorted_sections {
                markdown.push_str(&format!("### Section: {}\n\n", section_name));

                // Sort verifications by document order (section_order_index)
                let mut sorted_verifications: Vec<_> = section_verifications.verifications.iter().collect();
                sorted_verifications.sort_by_key(|v| v.section_order_index);

                for trace in sorted_verifications {
                    markdown.push_str(&format!("#### {}\n\n", trace.name));
                    markdown.push_str(&format!("- **Type**: {}\n", trace.verification_type));
                    markdown.push_str(&format!(
                        "- **Directly Verified**: {} requirements\n",
                        trace.directly_verified_count
                    ));
                    markdown.push_str(&format!(
                        "- **Total in Tree**: {} requirements\n\n",
                        trace.total_requirements_in_tree
                    ));

                    // Add Mermaid diagram
                    markdown.push_str(&self.generate_mermaid_diagram(trace));
                    markdown.push_str("\n\n");
                }
            }
        }

        markdown
    }
}

/// Apply filters to verification traces report
pub fn apply_filters(
    mut report: VerificationTracesReport,
    filter_id: Option<&str>,
    filter_name: Option<&str>,
    filter_type: Option<&str>,
) -> Result<VerificationTracesReport, crate::error::ReqvireError> {
    use regex::Regex;

    // Compile regex if name filter is provided
    let name_regex = if let Some(pattern) = filter_name {
        Some(Regex::new(pattern)
            .map_err(|e| crate::error::ReqvireError::InvalidRegex(e.to_string()))?)
    } else {
        None
    };

    // Filter verifications in each file/section
    for file_verifications in report.files.values_mut() {
        for section_verifications in file_verifications.sections.values_mut() {
            section_verifications.verifications.retain(|v| {
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

        // Remove empty sections
        file_verifications.sections.retain(|_, s| !s.verifications.is_empty());
    }

    // Remove empty files
    report.files.retain(|_, f| !f.sections.is_empty());

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
