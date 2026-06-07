use crate::element::{Element, ElementType};
use crate::git_commands;
use crate::graph_registry::GraphRegistry;
use crate::relation::{VERIFICATION_TRACES_RELATIONS, VERIFY_RELATION};
use crate::utils;
use serde::Serialize;
use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

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

// Internal structure that includes relation type for diagram generation
#[derive(Debug, Clone)]
struct RequirementNodeWithRelation {
    pub id: String,
    pub name: String,
    pub element_type: String, // CSS class name based on element type
    pub children: Vec<(String, RequirementNodeWithRelation)>, // (relation_type, node)
    pub attachment_labels: Vec<String>,
}

pub struct VerificationTraceGenerator<'a> {
    registry: &'a GraphRegistry,
    diagrams_with_blobs: bool,
    from_folder: Option<String>,
}

impl<'a> VerificationTraceGenerator<'a> {
    pub fn new(
        registry: &'a GraphRegistry,
        diagrams_with_blobs: bool,
        from_folder: Option<String>,
    ) -> Self {
        Self {
            registry,
            diagrams_with_blobs,
            from_folder,
        }
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
    fn count_node_and_children(
        &self,
        node: &RequirementNode,
        visited: &mut HashSet<String>,
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

    /// Generate Mermaid diagram for a verification trace with containment structure
    pub fn generate_mermaid_diagram(&self, trace: &VerificationTrace) -> String {
        use std::collections::HashMap;

        let mut diagram = String::new();

        // Get Git repository information for creating proper links
        let repo_root = match git_commands::get_git_root_dir() {
            Ok(root) => root,
            Err(_) => PathBuf::from(""),
        };

        let base_url = git_commands::get_repository_base_url().unwrap_or_default();

        let commit_hash = git_commands::get_commit_hash().unwrap_or_default();

        let has_git_info =
            !repo_root.as_os_str().is_empty() && !base_url.is_empty() && !commit_hash.is_empty();

        // Build tree with relation information first to collect all elements
        let mut visited = HashSet::new();
        let mut tree_with_relations = Vec::new();

        for req_id in &trace.directly_verified_requirements {
            if let Some(req) = self.registry.get_element(req_id) {
                if let Some(node) = self.build_node_with_relations(req, &mut visited) {
                    tree_with_relations.push(node);
                }
            }
        }

        // Collect all elements that will be in the diagram
        // (id, name, element_type, attachment_labels) - element_type is used for CSS class
        let mut all_elements: Vec<(String, String, String, Vec<String>)> = Vec::new();
        let mut collected_ids: HashSet<String> = HashSet::new();

        // Add verification element
        all_elements.push((
            trace.identifier.clone(),
            trace.name.clone(),
            "verification".to_string(),
            vec![],
        ));
        collected_ids.insert(trace.identifier.clone());

        // Collect all requirements from tree
        self.collect_elements_from_tree(
            &tree_with_relations,
            &mut all_elements,
            &mut collected_ids,
        );

        // Group elements by folder -> file for containment structure
        #[allow(clippy::type_complexity)]
        let mut folders: HashMap<
            String,
            HashMap<String, Vec<(String, String, String, Vec<String>)>>,
        > = HashMap::new();

        for (elem_id, elem_name, elem_type, attachments) in all_elements {
            // Extract folder and file from identifier (format: path/to/File.md#element-name)
            let id_without_fragment = elem_id.split('#').next().unwrap_or(&elem_id);
            let path = PathBuf::from(id_without_fragment);
            let folder = path
                .parent()
                .and_then(|p| p.to_str())
                .unwrap_or("")
                .to_string();
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_else(|| {
                    id_without_fragment
                        .rsplit('/')
                        .next()
                        .unwrap_or(id_without_fragment)
                })
                .to_string();

            folders
                .entry(folder)
                .or_default()
                .entry(file_name)
                .or_default()
                .push((elem_id, elem_name, elem_type, attachments));
        }

        // Header with CSS classes (MBSE color scheme - matching other diagrams)
        diagram.push_str("```mermaid\n");
        diagram.push_str("graph TD\n");
        diagram.push_str("  classDef capability fill:#BBDEFB,stroke:#1976D2,stroke-width:2.5px;\n");
        diagram.push_str(
            "  classDef systemRequirement fill:#E1D8EE,stroke:#673AB7,stroke-width:1.5px;\n",
        );
        diagram
            .push_str("  classDef requirement fill:#ECEFF1,stroke:#673AB7,stroke-width:1.5px;\n");
        diagram.push_str("  classDef verification fill:#DCEDC8,stroke:#4CAF50,stroke-width:2px;\n");
        diagram.push_str("  classDef folder fill:#FAFAFA,stroke:#9E9E9E,stroke-width:3px;\n");
        diagram.push_str("  classDef file fill:#FFF8E1,stroke:#FFCA28,stroke-width:2px;\n");
        diagram.push_str("  classDef default fill:#F5F5F5,stroke:#424242,stroke-width:1.5px;\n");
        diagram.push('\n');

        // Sort folders for deterministic output
        let mut folder_names: Vec<&String> = folders.keys().collect();
        folder_names.sort();

        // Generate subgraphs for folders and files (containment structure)
        for folder_name in folder_names {
            let files = &folders[folder_name];
            let folder_id = utils::hash_identifier(&format!("folder:{}", folder_name));
            let folder_display = if folder_name.is_empty() {
                "root"
            } else {
                folder_name
            };

            diagram.push_str(&format!(
                "  subgraph {}[\"📁 {}\"]\n",
                folder_id, folder_display
            ));

            // Sort files for deterministic output
            let mut file_names: Vec<&String> = files.keys().collect();
            file_names.sort();

            for file_name in file_names {
                let file_elements = &files[file_name];
                let file_id =
                    utils::hash_identifier(&format!("file:{}:{}", folder_name, file_name));

                diagram.push_str(&format!("    subgraph {}[\"📄 {}\"]\n", file_id, file_name));

                // Sort elements for deterministic output
                let mut sorted_elements: Vec<&(String, String, String, Vec<String>)> =
                    file_elements.iter().collect();
                sorted_elements.sort_by(|a, b| a.0.cmp(&b.0));

                for (elem_id, elem_name, elem_type, attachment_labels) in sorted_elements {
                    let node_id = utils::hash_identifier(elem_id);

                    // Build label with attachments
                    let mut node_label = escape_mermaid_label(elem_name);
                    for attachment in attachment_labels {
                        node_label
                            .push_str(&format!("<br/>📎 {}", escape_mermaid_label(attachment)));
                    }

                    // Use the element type directly for CSS class
                    let class = elem_type.as_str();

                    diagram.push_str(&format!(
                        "      {}[\"{}\"]:::{}\n",
                        node_id, node_label, class
                    ));

                    // Add click handler
                    let click_target = self.get_click_target(
                        elem_id,
                        &repo_root,
                        &base_url,
                        &commit_hash,
                        has_git_info,
                    );
                    diagram.push_str(&format!("      click {} \"{}\";\n", node_id, click_target));
                }

                diagram.push_str("    end\n");
            }

            diagram.push_str("  end\n");
        }

        // Add all relations
        let verification_id = utils::hash_identifier(&trace.identifier);
        let mut visited_edges: HashSet<(String, String, String)> = HashSet::new();

        // Add verify relations from verification to directly verified requirements
        for req_id in &trace.directly_verified_requirements {
            let req_node_id = utils::hash_identifier(req_id);
            let edge_key = (
                verification_id.clone(),
                VERIFY_RELATION.to_string(),
                req_node_id.clone(),
            );
            if !visited_edges.contains(&edge_key) {
                visited_edges.insert(edge_key);
                if let Some(info) = crate::relation::RELATION_TYPES.get(VERIFY_RELATION) {
                    diagram.push_str(&format!(
                        "  {} {}|{}| {};\n",
                        verification_id, info.arrow, info.label, req_node_id,
                    ));
                }
            }
        }

        // Add parent relations from tree
        self.add_relations_from_tree(&tree_with_relations, &mut diagram, &mut visited_edges);

        diagram.push_str("```\n");
        diagram
    }

    /// Collect all elements from the tree for containment grouping
    fn collect_elements_from_tree(
        &self,
        nodes: &[RequirementNodeWithRelation],
        all_elements: &mut Vec<(String, String, String, Vec<String>)>,
        collected_ids: &mut HashSet<String>,
    ) {
        for node in nodes {
            // Only add if not already collected (avoid duplicates from multiple paths)
            if !collected_ids.contains(&node.id) {
                collected_ids.insert(node.id.clone());
                all_elements.push((
                    node.id.clone(),
                    node.name.clone(),
                    node.element_type.clone(),
                    node.attachment_labels.clone(),
                ));
            }
            // Recursively collect children
            let children: Vec<RequirementNodeWithRelation> =
                node.children.iter().map(|(_, n)| n.clone()).collect();
            self.collect_elements_from_tree(&children, all_elements, collected_ids);
        }
    }

    /// Add relations from tree to diagram
    fn add_relations_from_tree(
        &self,
        nodes: &[RequirementNodeWithRelation],
        diagram: &mut String,
        visited_edges: &mut HashSet<(String, String, String)>,
    ) {
        for node in nodes {
            let node_id = utils::hash_identifier(&node.id);

            for (relation_type, child) in &node.children {
                let child_id = utils::hash_identifier(&child.id);
                let edge_key = (node_id.clone(), relation_type.clone(), child_id.clone());

                if !visited_edges.contains(&edge_key) {
                    visited_edges.insert(edge_key);
                    if let Some(info) = crate::relation::RELATION_TYPES.get(relation_type.as_str())
                    {
                        diagram.push_str(&format!(
                            "  {} {}|{}| {};\n",
                            node_id, info.arrow, info.label, child_id,
                        ));
                    }
                }

                // Recursively add relations for children
                self.add_relations_from_tree(std::slice::from_ref(child), diagram, visited_edges);
            }
        }
    }

    /// Get click target for an element
    fn get_click_target(
        &self,
        elem_id: &str,
        repo_root: &Path,
        base_url: &str,
        commit_hash: &str,
        has_git_info: bool,
    ) -> String {
        if self.diagrams_with_blobs && has_git_info {
            let relative_id = match utils::get_relative_path(&PathBuf::from(elem_id)) {
                Ok(rel_path) => rel_path.to_string_lossy().to_string(),
                Err(_) => elem_id.to_string(),
            };
            format!("{}/blob/{}/{}", base_url, commit_hash, relative_id)
        } else if let Some(ref from_folder) = self.from_folder {
            if from_folder == "/" {
                elem_id.to_string()
            } else {
                let from_folder_path = repo_root.join(from_folder);
                match utils::to_relative_identifier(elem_id, &from_folder_path, false) {
                    Ok(rel_path) => rel_path,
                    Err(_) => elem_id.to_string(),
                }
            }
        } else {
            elem_id.to_string()
        }
    }

    /// Build a requirement node with relation information
    fn build_node_with_relations(
        &self,
        requirement: &Element,
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
                            self.build_node_with_relations(parent, &mut branch_visited)
                        {
                            // Store relation type with the node
                            children.push((relation.relation_type.name.to_string(), parent_node));
                        }
                    }
                }
            }
        }

        // Determine CSS class based on element type (matching other diagrams)
        let element_type = match &requirement.element_type {
            ElementType::Capability => "capability",
            ElementType::Requirement(crate::element::RequirementType::System) => {
                "systemRequirement"
            }
            ElementType::Verification(_) => "verification",
            _ => "requirement",
        };

        Some(RequirementNodeWithRelation {
            id: requirement.identifier.clone(),
            name: requirement.name.clone(),
            element_type: element_type.to_string(),
            children,
            attachment_labels: requirement
                .attachments
                .iter()
                .map(|a| match &a.target {
                    crate::element::AttachmentTarget::FilePath(path) => path
                        .file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_else(|| path.to_string_lossy().into_owned()),
                    crate::element::AttachmentTarget::ElementIdentifier(id) => self
                        .registry
                        .get_element(id)
                        .map(|target| target.name.clone())
                        .unwrap_or_else(|| attachment_target_label(id)),
                })
                .collect(),
        })
    }

    /// Generate Markdown report with Mermaid diagrams
    pub fn generate_markdown(&self, report: &VerificationTracesReport) -> String {
        let mut markdown = String::new();

        // Files are already sorted alphabetically by BTreeMap
        for (file_path, file_verifications) in &report.files {
            markdown.push_str(&format!("## File: {}\n\n", file_path));

            // Verifications are already sorted by file_order_index in generate()
            for trace in &file_verifications.verifications {
                // Make element name a clickable link to its definition
                let element_fragment = trace
                    .identifier
                    .rfind('#')
                    .map(|pos| &trace.identifier[pos..])
                    .unwrap_or("");
                markdown.push_str(&format!(
                    "### [{}]({}{})\n\n",
                    trace.name, trace.file, element_fragment
                ));
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

        markdown
    }
}

fn attachment_target_label(target: &str) -> String {
    let fragment_or_path = target.rsplit('#').next().unwrap_or(target);
    let basename = fragment_or_path
        .rsplit('/')
        .next()
        .unwrap_or(fragment_or_path);
    if basename.is_empty() {
        target.to_string()
    } else {
        basename.to_string()
    }
}

fn escape_mermaid_label(label: &str) -> String {
    label
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', " ")
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
        Some(
            Regex::new(pattern)
                .map_err(|e| crate::error::ReqvireError::InvalidRegex(e.to_string()))?,
        )
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

/// Sankey diagram data structures
#[derive(Debug, Serialize)]
pub struct SankeyData {
    pub nodes: Vec<SankeyNode>,
    pub links: Vec<SankeyLink>,
}

#[derive(Debug, Serialize)]
pub struct SankeyNode {
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SankeyLink {
    pub source: String,
    pub target: String,
    pub value: usize,
}

/// Generate Sankey diagram data from verification traces
/// Shows flow from requirements to verifications
pub fn generate_sankey_data(report: &VerificationTracesReport) -> SankeyData {
    let mut nodes: Vec<SankeyNode> = Vec::new();
    let mut links: Vec<SankeyLink> = Vec::new();
    let mut node_names: HashSet<String> = HashSet::new();

    // Collect all requirements and verifications
    for file_verifications in report.files.values() {
        for trace in &file_verifications.verifications {
            // Add verification node
            if !node_names.contains(&trace.name) {
                nodes.push(SankeyNode {
                    name: trace.name.clone(),
                    node_type: "verification".to_string(),
                    link: Some(trace.identifier.clone()),
                });
                node_names.insert(trace.name.clone());
            }

            // Add requirement nodes and links from trace tree
            add_requirements_from_tree(
                &trace.trace_tree.requirements,
                &trace.name,
                &mut nodes,
                &mut links,
                &mut node_names,
            );
        }
    }

    SankeyData { nodes, links }
}

fn add_requirements_from_tree(
    requirements: &[RequirementNode],
    verification_name: &str,
    nodes: &mut Vec<SankeyNode>,
    links: &mut Vec<SankeyLink>,
    node_names: &mut HashSet<String>,
) {
    for req in requirements {
        // Add requirement node if not already present
        if !node_names.contains(&req.name) {
            nodes.push(SankeyNode {
                name: req.name.clone(),
                node_type: req.element_type.clone(),
                link: Some(req.id.clone()),
            });
            node_names.insert(req.name.clone());
        }

        // Add link from requirement to verification (for directly verified)
        if req.is_directly_verified {
            links.push(SankeyLink {
                source: req.name.clone(),
                target: verification_name.to_string(),
                value: 1,
            });
        }

        // Process children (parent requirements in the derivedFrom hierarchy)
        for child in &req.children {
            // Add child node
            if !node_names.contains(&child.name) {
                nodes.push(SankeyNode {
                    name: child.name.clone(),
                    node_type: child.element_type.clone(),
                    link: Some(child.id.clone()),
                });
                node_names.insert(child.name.clone());
            }

            // Add link from child to parent requirement
            links.push(SankeyLink {
                source: child.name.clone(),
                target: req.name.clone(),
                value: 1,
            });

            // Recursively process grandchildren
            add_requirements_from_tree(
                &child.children,
                verification_name,
                nodes,
                links,
                node_names,
            );
        }
    }
}

/// Generate Sankey markdown block for traces report
pub fn generate_sankey_markdown(report: &VerificationTracesReport) -> String {
    let sankey_data = generate_sankey_data(report);

    let json = serde_json::to_string_pretty(&sankey_data).unwrap_or_else(|_| "{}".to_string());

    format!("```d3-sankey\n{}\n```\n", json)
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
