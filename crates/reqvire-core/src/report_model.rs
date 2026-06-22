use crate::diagrams::escape_label;
use crate::element;
use crate::element::SizeEstimate;
use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;
use crate::relation;
use crate::utils::hash_identifier;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashSet};

/// Model-centric report with nested element structure
#[derive(Debug, Serialize)]
pub struct ModelCentricReport {
    pub elements: Vec<ModelCentricElement>,
    pub metadata: ModelMetadata,
}

/// Direction of traversal for model report
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TraversalDirection {
    Forward, // Root to leaves through diagram traversal relations.
    Reverse, // Leaves to roots (derivedFrom, satisfy, verify)
}

/// Element in model-centric view with nested relations
#[derive(Debug, Serialize)]
pub struct ModelCentricElement {
    pub identifier: String,
    pub name: String,
    pub element_type: String,
    pub file_path: String,
    pub file_order_index: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_estimate: Option<SizeEstimate>,
    pub relations: Vec<ModelCentricRelation>,
    pub reused_contract_context: Vec<String>,
    #[serde(skip)]
    pub reused_contract_context_labels: Vec<String>,
}

/// Relation in model-centric view with target details
#[derive(Debug, Serialize)]
pub struct ModelCentricRelation {
    pub relation_type: String,
    #[serde(flatten)]
    pub target: RelationTarget,
}

/// Target of a relation (element, file, or external)
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum RelationTarget {
    Element {
        element: Box<ModelCentricElement>,
    },
    File {
        path: String,
        #[serde(rename = "type")]
        target_type: String,
    },
    External {
        url: String,
        #[serde(rename = "type")]
        target_type: String,
    },
}

/// Metadata about the model
#[derive(Debug, Serialize)]
pub struct ModelMetadata {
    pub total_elements: usize,
    pub total_relations: usize,
    pub filtered_from: Option<String>,
    pub direction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_filter: Option<Vec<String>>,
}

/// Generate model-centric report
pub fn generate_model_report(
    registry: &GraphRegistry,
    root_element_name: Option<&str>,
    reverse: bool,
    type_filter: Option<Vec<&str>>,
    json_output: bool,
    diagram_direction: &str, // "LR" or "TD"
) -> Result<String, ReqvireError> {
    let report = build_model_report(registry, root_element_name, reverse, type_filter)?;

    if json_output {
        serde_json::to_string_pretty(&report)
            .map_err(|e| ReqvireError::SerializationError(e.to_string()))
    } else {
        // Generate text with mermaid diagrams
        Ok(generate_model_text(&report, diagram_direction))
    }
}

/// Generate pure Mermaid output for the model-centric report.
pub fn generate_model_mmd(
    registry: &GraphRegistry,
    root_element_name: Option<&str>,
    reverse: bool,
    type_filter: Option<Vec<&str>>,
    diagram_direction: &str,
) -> Result<String, ReqvireError> {
    let report = build_model_report(registry, root_element_name, reverse, type_filter)?;
    Ok(generate_model_mmd_text(
        registry,
        &report,
        diagram_direction,
    ))
}

fn build_model_report(
    registry: &GraphRegistry,
    root_element_name: Option<&str>,
    reverse: bool,
    type_filter: Option<Vec<&str>>,
) -> Result<ModelCentricReport, ReqvireError> {
    use std::collections::HashSet;

    // Validate type filter if provided
    if let Some(ref types) = type_filter {
        for t in types {
            if !element::is_valid_element_type(t) {
                return Err(ReqvireError::ProcessError(format!(
                    "Invalid element type '{}'. Valid types: {}",
                    t,
                    element::element_types_help()
                )));
            }
        }
    }

    let direction = if reverse {
        TraversalDirection::Reverse
    } else {
        TraversalDirection::Forward
    };

    // Determine starting elements
    let mut starting_elements = if let Some(name) = root_element_name {
        // Find element by name
        let found = registry
            .nodes
            .iter()
            .find(|(_, node)| node.element.name == name);

        match found {
            Some((id, _)) => vec![id.clone()],
            None => {
                eprintln!("❌ Element with name '{}' not found", name);
                return Err(ReqvireError::ElementError(format!(
                    "Element with name '{}' not found",
                    name
                )));
            }
        }
    } else if let Some(ref types) = type_filter {
        // Filter by element types
        if reverse {
            // For reverse, filter leaf elements by type
            registry.find_leaf_elements(Some(types.as_slice()))
        } else {
            types
                .iter()
                .flat_map(|element_type| {
                    find_model_starting_elements_by_type(registry, element_type)
                })
                .collect()
        }
    } else if reverse {
        // Reverse without type filter: use leaf elements
        registry.find_leaf_elements(None)
    } else {
        // No filter: start from ontology roots, concept roots, and capability roots.
        find_default_model_roots(registry)
    };

    if root_element_name.is_some() || type_filter.is_some() || reverse {
        starting_elements.sort();
    }

    // Build report
    let mut elements = Vec::new();

    for start_id in &starting_elements {
        // Each starting element gets its own visited set to allow independent traversals
        // This is important for reverse mode where multiple leaves may trace to common ancestors
        let mut visited = HashSet::new();
        if let Some(element) = build_element_recursive(registry, start_id, &mut visited, direction)
        {
            elements.push(element);
        }
    }

    // Count metadata
    let total_elements = starting_elements.len();
    let total_relations = count_relations(registry, &starting_elements, direction);

    Ok(ModelCentricReport {
        elements,
        metadata: ModelMetadata {
            total_elements,
            total_relations,
            filtered_from: root_element_name.map(|s| s.to_string()),
            direction: if reverse {
                "Reverse".to_string()
            } else {
                "Forward".to_string()
            },
            type_filter: type_filter.map(|v| v.into_iter().map(|s| s.to_string()).collect()),
        },
    })
}

fn find_default_model_roots(registry: &GraphRegistry) -> Vec<String> {
    let mut roots = registry.find_root_elements_by_type("ontology");
    let mut seen: HashSet<String> = roots.iter().cloned().collect();
    for concept_scheme_id in registry.find_root_elements_by_type("concept-scheme") {
        if seen.insert(concept_scheme_id.clone()) {
            roots.push(concept_scheme_id);
        }
    }
    for capability_id in registry.find_root_capabilities() {
        if seen.insert(capability_id.clone()) {
            roots.push(capability_id);
        }
    }
    roots
}

fn find_model_starting_elements_by_type(
    registry: &GraphRegistry,
    element_type: &str,
) -> Vec<String> {
    if element_type == "concept" {
        let mut concepts = registry
            .nodes
            .values()
            .filter(|node| node.element.element_type.as_str() == element_type)
            .map(|node| node.element.identifier.clone())
            .collect::<Vec<_>>();
        concepts.sort();
        return concepts;
    }
    registry.find_root_elements_by_type(element_type)
}

/// Build element recursively with nested relations
fn build_element_recursive(
    registry: &GraphRegistry,
    element_id: &str,
    visited: &mut HashSet<String>,
    direction: TraversalDirection,
) -> Option<ModelCentricElement> {
    // Prevent infinite recursion
    if visited.contains(element_id) {
        return None;
    }
    visited.insert(element_id.to_string());

    let element = registry.get_element(element_id)?;

    // Choose which relations to follow based on direction
    let allowed_relations = match direction {
        TraversalDirection::Forward => relation::DIAGRAM_RELATIONS,
        TraversalDirection::Reverse => relation::BACKWARD_RELATIONS,
    };

    // Sort element relations for deterministic iteration
    let mut sorted_relations = element.relations.clone();
    sorted_relations.sort_by(|a, b| {
        // First sort by relation type
        let type_cmp = a.relation_type.name.cmp(b.relation_type.name);
        if type_cmp != std::cmp::Ordering::Equal {
            return type_cmp;
        }

        // Then sort by target
        let a_target = match &a.target.link {
            relation::LinkType::Identifier(id) => id.as_str(),
            relation::LinkType::ExternalUrl(url) => url.as_str(),
            relation::LinkType::InternalPath(path) => path.to_str().unwrap_or(""),
        };
        let b_target = match &b.target.link {
            relation::LinkType::Identifier(id) => id.as_str(),
            relation::LinkType::ExternalUrl(url) => url.as_str(),
            relation::LinkType::InternalPath(path) => path.to_str().unwrap_or(""),
        };
        a_target.cmp(b_target)
    });

    // Build relations with nested targets
    let mut relations = Vec::new();
    for relation in &sorted_relations {
        // Only include relations matching the direction
        if !allowed_relations.contains(&relation.relation_type.name) {
            continue;
        }

        let target = match &relation.target.link {
            relation::LinkType::Identifier(target_id) => {
                let Some(target_element) = registry.get_element(target_id) else {
                    continue;
                };
                if is_model_namespace_context_relation(
                    element,
                    relation.relation_type.name,
                    target_element,
                ) {
                    continue;
                }
                // Recursive element
                if let Some(target_element) =
                    build_element_recursive(registry, target_id, visited, direction)
                {
                    RelationTarget::Element {
                        element: Box::new(target_element),
                    }
                } else {
                    continue; // Skip cyclic or missing
                }
            }
            relation::LinkType::InternalPath(path) => RelationTarget::File {
                path: path.to_string_lossy().to_string(),
                target_type: "file".to_string(),
            },
            relation::LinkType::ExternalUrl(url) => RelationTarget::External {
                url: url.clone(),
                target_type: "external".to_string(),
            },
        };

        relations.push(ModelCentricRelation {
            relation_type: relation.relation_type.name.to_string(),
            target,
        });
    }

    // Relations are already sorted from sorted_relations above

    // Build reused_contract_context list
    let reused_contract_context: Vec<String> = element
        .reused_contract_context
        .iter()
        .map(|a| a.target.as_str())
        .collect();
    let reused_contract_context_labels: Vec<String> = element
        .reused_contract_context
        .iter()
        .map(|a| {
            let target_id = a.target.as_str();
            registry
                .get_element(&target_id)
                .map(|target| target.name.clone())
                .unwrap_or_else(|| reused_contract_context_target_label(&target_id))
        })
        .collect();

    Some(ModelCentricElement {
        identifier: element.identifier.clone(),
        name: element.name.clone(),
        element_type: element.element_type.as_str().to_string(),
        file_path: element.file_path.clone(),
        file_order_index: element.file_order_index,
        size_estimate: element.size_estimate.clone(),
        relations,
        reused_contract_context,
        reused_contract_context_labels,
    })
}

fn reused_contract_context_target_label(target: &str) -> String {
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

fn is_model_namespace_context_relation(
    source: &crate::element::Element,
    relation_name: &str,
    target: &crate::element::Element,
) -> bool {
    (relation_name == "derive"
        && source.element_type.is_ontology()
        && target.element_type.is_concept_scheme())
        || (relation_name == "derivedFrom"
            && source.element_type.is_concept_scheme()
            && target.element_type.is_ontology())
}

/// Count total relations from starting elements
fn count_relations(
    registry: &GraphRegistry,
    starting_elements: &[String],
    direction: TraversalDirection,
) -> usize {
    let mut count = 0;
    let mut visited = HashSet::new();

    for start_id in starting_elements {
        count_relations_recursive(registry, start_id, &mut visited, &mut count, direction);
    }

    count
}

/// Count relations recursively
fn count_relations_recursive(
    registry: &GraphRegistry,
    element_id: &str,
    visited: &mut HashSet<String>,
    count: &mut usize,
    direction: TraversalDirection,
) {
    if visited.contains(element_id) {
        return;
    }
    visited.insert(element_id.to_string());

    let allowed_relations = match direction {
        TraversalDirection::Forward => relation::DIAGRAM_RELATIONS,
        TraversalDirection::Reverse => relation::BACKWARD_RELATIONS,
    };

    if let Some(element) = registry.get_element(element_id) {
        for relation in &element.relations {
            if allowed_relations.contains(&relation.relation_type.name) {
                if let relation::LinkType::Identifier(target_id) = &relation.target.link {
                    if let Some(target) = registry.get_element(target_id) {
                        if is_model_namespace_context_relation(
                            element,
                            relation.relation_type.name,
                            target,
                        ) {
                            continue;
                        }
                    }
                }
                *count += 1;

                // Recurse for identifier targets
                if let relation::LinkType::Identifier(target_id) = &relation.target.link {
                    count_relations_recursive(registry, target_id, visited, count, direction);
                }
            }
        }
    }
}

/// Generate text output for model-centric report with mermaid diagrams
fn generate_model_text(report: &ModelCentricReport, diagram_direction: &str) -> String {
    let mut output = String::new();

    // Metadata
    output.push_str(&format!(
        "**Total Elements**: {}\n",
        report.metadata.total_elements
    ));
    output.push_str(&format!(
        "**Total Relations**: {}\n",
        report.metadata.total_relations
    ));
    output.push_str(&format!("**Direction**: {}\n", report.metadata.direction));
    if let Some(ref filtered) = report.metadata.filtered_from {
        output.push_str(&format!("**Filtered From**: {}\n", filtered));
    }
    if let Some(ref type_filter) = report.metadata.type_filter {
        output.push_str(&format!("**Type Filter**: {}\n", type_filter.join(", ")));
    }
    output.push('\n');

    // Elements with mermaid diagrams
    for element in &report.elements {
        output.push_str(&generate_element_text(element, 0, diagram_direction));
    }

    output
}

#[derive(Clone)]
struct MmdNode {
    identifier: String,
    name: String,
    element_type: String,
    reused_contract_context: Vec<String>,
}

fn generate_model_mmd_text(
    registry: &GraphRegistry,
    report: &ModelCentricReport,
    diagram_direction: &str,
) -> String {
    let mut nodes: BTreeMap<String, MmdNode> = BTreeMap::new();
    let mut node_order: Vec<String> = Vec::new();
    let mut edges: BTreeSet<(String, String, String)> = BTreeSet::new();

    for element in &report.elements {
        collect_mmd_nodes_and_edges(element, &mut nodes, &mut node_order, &mut edges);
    }

    let discovered_nodes: Vec<MmdNode> = nodes.values().cloned().collect();
    for node in discovered_nodes {
        for reused_context_id in &node.reused_contract_context {
            if let Some(target) = registry.get_element(reused_context_id) {
                insert_mmd_node(
                    &mut nodes,
                    &mut node_order,
                    MmdNode {
                        identifier: target.identifier.clone(),
                        name: target.name.clone(),
                        element_type: target.element_type.as_str().to_string(),
                        reused_contract_context: target
                            .reused_contract_context
                            .iter()
                            .map(|reused_contract_context| reused_contract_context.target.as_str())
                            .collect(),
                    },
                );
                edges.insert((
                    node.identifier.clone(),
                    "reuses contract".to_string(),
                    target.identifier.clone(),
                ));
            }
        }
    }

    let mut output = String::new();
    output.push_str(&format!("graph {}\n", diagram_direction));
    output.push_str("  classDef capability fill:#BBDEFB,stroke:#1976D2,stroke-width:2.5px;\n");
    output
        .push_str("  classDef systemRequirement fill:#E1D8EE,stroke:#673AB7,stroke-width:1.5px;\n");
    output.push_str("  classDef ontology fill:#F4E3A1,stroke:#B08A00,stroke-width:2px;\n");
    output.push_str("  classDef concept fill:#D7CCC8,stroke:#6D4C41,stroke-width:2px;\n");
    output.push_str("  classDef verification fill:#DCEDC8,stroke:#4CAF50,stroke-width:2px;\n");
    output.push_str("  classDef default fill:#F5F5F5,stroke:#424242,stroke-width:1.5px;\n\n");

    for node_id_key in &node_order {
        let Some(node) = nodes.get(node_id_key) else {
            continue;
        };
        let node_id = hash_identifier(&node.identifier);
        output.push_str(&format!(
            "  {}[\"{}\"];\n",
            node_id,
            escape_label(&node.name)
        ));
        output.push_str(&format!(
            "  class {} {};\n",
            node_id,
            get_element_class(&node.element_type)
        ));
        output.push_str(&format!(
            "  click {} \"{}\";\n",
            node_id,
            escape_label(&node.identifier)
        ));
    }

    if !edges.is_empty() {
        output.push('\n');
    }

    for (source, relation, target) in edges {
        output.push_str(&format!(
            "  {} -->|{}| {};\n",
            hash_identifier(&source),
            escape_label(&relation),
            hash_identifier(&target)
        ));
    }

    output
}

fn collect_mmd_nodes_and_edges(
    element: &ModelCentricElement,
    nodes: &mut BTreeMap<String, MmdNode>,
    node_order: &mut Vec<String>,
    edges: &mut BTreeSet<(String, String, String)>,
) {
    insert_mmd_node(
        nodes,
        node_order,
        MmdNode {
            identifier: element.identifier.clone(),
            name: element.name.clone(),
            element_type: element.element_type.clone(),
            reused_contract_context: element.reused_contract_context.clone(),
        },
    );

    for (idx, relation) in element.relations.iter().enumerate() {
        match &relation.target {
            RelationTarget::Element { element: target } => {
                edges.insert((
                    element.identifier.clone(),
                    relation.relation_type.clone(),
                    target.identifier.clone(),
                ));
                collect_mmd_nodes_and_edges(target, nodes, node_order, edges);
            }
            RelationTarget::File { path, .. } => {
                let target_id = format!("file:{}:{}", element.identifier, idx);
                insert_mmd_node(
                    nodes,
                    node_order,
                    MmdNode {
                        identifier: target_id.clone(),
                        name: path.clone(),
                        element_type: "file".to_string(),
                        reused_contract_context: Vec::new(),
                    },
                );
                edges.insert((
                    element.identifier.clone(),
                    relation.relation_type.clone(),
                    target_id,
                ));
            }
            RelationTarget::External { url, .. } => {
                let target_id = format!("external:{}:{}", element.identifier, idx);
                insert_mmd_node(
                    nodes,
                    node_order,
                    MmdNode {
                        identifier: target_id.clone(),
                        name: url.clone(),
                        element_type: "external".to_string(),
                        reused_contract_context: Vec::new(),
                    },
                );
                edges.insert((
                    element.identifier.clone(),
                    relation.relation_type.clone(),
                    target_id,
                ));
            }
        }
    }
}

fn insert_mmd_node(
    nodes: &mut BTreeMap<String, MmdNode>,
    node_order: &mut Vec<String>,
    node: MmdNode,
) {
    let identifier = node.identifier.clone();
    if nodes.insert(identifier.clone(), node).is_none() {
        node_order.push(identifier);
    }
}

/// Generate text for an element with mermaid diagram showing its relations
fn generate_element_text(
    element: &ModelCentricElement,
    depth: usize,
    diagram_direction: &str,
) -> String {
    let indent = "  ".repeat(depth);
    let mut output = String::new();

    // Element header - make name a link to the element
    let element_fragment = element
        .identifier
        .rfind('#')
        .map(|pos| &element.identifier[pos..])
        .unwrap_or("");
    output.push_str(&format!(
        "{}## [{}]({}{})\n\n",
        indent, element.name, element.file_path, element_fragment
    ));
    output.push_str(&format!("{}**Type**: {}\n", indent, element.element_type));
    output.push_str(&format!(
        "{}**File**: [{}]({})\n\n",
        indent, element.file_path, element.file_path
    ));

    // Mermaid diagram for this element's relations
    if !element.relations.is_empty() {
        output.push_str(&format!("{}```mermaid\n", indent));
        output.push_str(&format!("{}graph {}\n", indent, diagram_direction));

        // Add CSS class definitions for colors (MBSE color scheme)
        output.push_str(&format!(
            "{}  classDef capability fill:#BBDEFB,stroke:#1976D2,stroke-width:2.5px;\n",
            indent
        ));
        output.push_str(&format!(
            "{}  classDef systemRequirement fill:#E1D8EE,stroke:#673AB7,stroke-width:1.5px;\n",
            indent
        ));
        output.push_str(&format!(
            "{}  classDef ontology fill:#F4E3A1,stroke:#B08A00,stroke-width:2px;\n",
            indent
        ));
        output.push_str(&format!(
            "{}  classDef concept fill:#D7CCC8,stroke:#6D4C41,stroke-width:2px;\n",
            indent
        ));
        output.push_str(&format!(
            "{}  classDef verification fill:#DCEDC8,stroke:#4CAF50,stroke-width:2px;\n",
            indent
        ));
        output.push_str(&format!(
            "{}  classDef default fill:#F5F5F5,stroke:#424242,stroke-width:1.5px;\n",
            indent
        ));
        output.push_str(&format!("{}\n", indent));

        output.push_str(&generate_mermaid_for_element(element, &indent));
        output.push_str(&format!("{}```\n\n", indent));
    }

    output
}

/// Generate mermaid diagram content for an element's relations with containment structure
fn generate_mermaid_for_element(element: &ModelCentricElement, indent: &str) -> String {
    use std::collections::HashMap;
    use std::path::Path;

    // First, collect all elements that will be in the diagram
    let mut all_elements: Vec<&ModelCentricElement> = Vec::new();
    let mut visited_collect = HashSet::new();
    collect_all_elements_recursive(element, &mut all_elements, &mut visited_collect);

    // Group elements by folder -> file
    let mut folders: HashMap<String, HashMap<String, Vec<&ModelCentricElement>>> = HashMap::new();

    for elem in &all_elements {
        let file_path = &elem.file_path;
        let folder = Path::new(file_path)
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .to_string();
        let file_name = Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(file_path)
            .to_string();

        folders
            .entry(folder)
            .or_default()
            .entry(file_name)
            .or_default()
            .push(*elem);
    }

    let mut output = String::new();

    // Add folder styling
    output.push_str(&format!(
        "{}  classDef folder fill:#FAFAFA,stroke:#9E9E9E,stroke-width:3px;\n",
        indent
    ));
    output.push_str(&format!(
        "{}  classDef file fill:#FFF8E1,stroke:#FFCA28,stroke-width:2px;\n",
        indent
    ));
    output.push_str(&format!("{}\n", indent));

    // Sort folders for deterministic output
    let mut folder_names: Vec<&String> = folders.keys().collect();
    folder_names.sort();

    // Generate subgraphs for folders and files
    for folder_name in folder_names {
        let files = &folders[folder_name];
        let folder_id = hash_identifier(&format!("folder:{}", folder_name));
        let folder_display = if folder_name.is_empty() {
            "root"
        } else {
            folder_name
        };

        output.push_str(&format!(
            "{}  subgraph {}[\"📁 {}\"]\n",
            indent,
            folder_id,
            escape_label(folder_display)
        ));

        // Sort files for deterministic output
        let mut file_names: Vec<&String> = files.keys().collect();
        file_names.sort();

        for file_name in file_names {
            let elements = &files[file_name];
            let file_id = hash_identifier(&format!("file:{}:{}", folder_name, file_name));

            output.push_str(&format!(
                "{}    subgraph {}[\"📄 {}\"]\n",
                indent,
                file_id,
                escape_label(file_name)
            ));

            // Sort elements for deterministic output
            let mut sorted_elements: Vec<&&ModelCentricElement> = elements.iter().collect();
            sorted_elements.sort_by(|a, b| a.identifier.cmp(&b.identifier));

            for elem in sorted_elements {
                let elem_id = hash_identifier(&elem.identifier);
                let elem_class = get_element_class(&elem.element_type);

                // Build label with reused_contract_context
                let mut elem_label = escape_label(&elem.name);
                for reused_contract_context in &elem.reused_contract_context_labels {
                    elem_label.push_str(&format!(
                        "<br/>📎 {}",
                        escape_label(reused_contract_context)
                    ));
                }

                output.push_str(&format!(
                    "{}      {}[\"{}\"];\n",
                    indent, elem_id, elem_label
                ));
                output.push_str(&format!(
                    "{}      class {} {};\n",
                    indent, elem_id, elem_class
                ));
                output.push_str(&format!(
                    "{}      click {} \"{}\";\n",
                    indent, elem_id, &elem.identifier
                ));
            }

            output.push_str(&format!("{}    end\n", indent));
        }

        output.push_str(&format!("{}  end\n", indent));
    }

    // Now add all relations
    let mut visited_relations = HashSet::new();
    output.push_str(&generate_relations_recursive(
        element,
        indent,
        &mut visited_relations,
    ));

    output
}

/// Collect all elements recursively for containment grouping
fn collect_all_elements_recursive<'a>(
    element: &'a ModelCentricElement,
    all_elements: &mut Vec<&'a ModelCentricElement>,
    visited: &mut HashSet<String>,
) {
    if visited.contains(&element.identifier) {
        return;
    }
    visited.insert(element.identifier.clone());
    all_elements.push(element);

    for relation in &element.relations {
        if let RelationTarget::Element { element: target } = &relation.target {
            collect_all_elements_recursive(target, all_elements, visited);
        }
    }
}

/// Generate only the relations (edges) between elements
fn generate_relations_recursive(
    element: &ModelCentricElement,
    indent: &str,
    visited: &mut HashSet<String>,
) -> String {
    if visited.contains(&element.identifier) {
        return String::new();
    }
    visited.insert(element.identifier.clone());

    let mut output = String::new();
    let element_id = hash_identifier(&element.identifier);

    for (idx, relation) in element.relations.iter().enumerate() {
        match &relation.target {
            RelationTarget::Element { element: target } => {
                let target_id = hash_identifier(&target.identifier);
                output.push_str(&format!(
                    "{}  {} -->|{}| {};\n",
                    indent, element_id, relation.relation_type, target_id
                ));

                // Recursively add relations for target
                output.push_str(&generate_relations_recursive(target, indent, visited));
            }
            RelationTarget::File { path, .. } => {
                let file_id = hash_identifier(&format!("file:{}:{}", element.identifier, idx));
                output.push_str(&format!(
                    "{}  {}[\"{}\"];\n",
                    indent,
                    file_id,
                    escape_label(path)
                ));
                output.push_str(&format!("{}  class {} default;\n", indent, file_id));
                output.push_str(&format!("{}  click {} \"{}\";\n", indent, file_id, path));
                output.push_str(&format!(
                    "{}  {} -->|{}| {};\n",
                    indent, element_id, relation.relation_type, file_id
                ));
            }
            RelationTarget::External { url, .. } => {
                let ext_id = hash_identifier(&format!("external:{}:{}", element.identifier, idx));
                output.push_str(&format!(
                    "{}  {}[\"{}\"];\n",
                    indent,
                    ext_id,
                    escape_label(url)
                ));
                output.push_str(&format!("{}  class {} default;\n", indent, ext_id));
                output.push_str(&format!("{}  click {} \"{}\";\n", indent, ext_id, url));
                output.push_str(&format!(
                    "{}  {} -->|{}| {};\n",
                    indent, element_id, relation.relation_type, ext_id
                ));
            }
        }
    }

    output
}

/// Determine CSS class name based on element type string
fn get_element_class(element_type: &str) -> &'static str {
    let lower = element_type.to_lowercase();
    if lower == "capability" {
        "capability"
    } else if lower == "ontology" {
        "ontology"
    } else if lower == "concept-scheme" || lower == "concept" {
        "concept"
    } else if lower == "requirement" || lower.contains("system") && lower.contains("requirement") {
        "systemRequirement"
    } else if lower.contains("verification") {
        "verification"
    } else {
        "default"
    }
}
