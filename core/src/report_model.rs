use crate::diagrams::escape_label;
use crate::element;
use crate::element::SizeEstimate;
use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;
use crate::relation;
use crate::utils::hash_identifier;
use serde::Serialize;
use std::collections::HashSet;

/// Model-centric report with nested element structure
#[derive(Debug, Serialize)]
pub struct ModelCentricReport {
    pub elements: Vec<ModelCentricElement>,
    pub metadata: ModelMetadata,
}

/// Direction of traversal for model report
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TraversalDirection {
    Forward, // Root to leaves (derive, satisfiedBy, verifiedBy, trace)
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
    pub attachments: Vec<String>,
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
            // For forward, filter root elements by type
            // First get all elements of the types, then filter to those that are roots
            let type_elements = registry.find_elements_by_type(types.as_slice());
            let hierarchical_relations = relation::get_hierarchical_relation_types();

            type_elements
                .into_iter()
                .filter(|id| {
                    if let Some(element) = registry.get_element(id) {
                        // Check if has any hierarchical parent relation
                        let has_parent = element
                            .relations
                            .iter()
                            .any(|r| hierarchical_relations.contains(&r.relation_type.name));
                        !has_parent
                    } else {
                        false
                    }
                })
                .collect()
        }
    } else if reverse {
        // Reverse without type filter: use leaf elements
        registry.find_leaf_elements(None)
    } else {
        // No filter, use root requirements
        registry.find_root_requirements()
    };

    // Sort for deterministic output
    starting_elements.sort();

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

    let report = ModelCentricReport {
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
    };

    if json_output {
        serde_json::to_string_pretty(&report)
            .map_err(|e| ReqvireError::SerializationError(e.to_string()))
    } else {
        // Generate text with mermaid diagrams
        Ok(generate_model_text(&report, diagram_direction))
    }
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

    // Build attachments list
    let attachments: Vec<String> = element
        .attachments
        .iter()
        .map(|a| a.target.as_str())
        .collect();

    Some(ModelCentricElement {
        identifier: element.identifier.clone(),
        name: element.name.clone(),
        element_type: element.element_type.as_str().to_string(),
        file_path: element.file_path.clone(),
        file_order_index: element.file_order_index,
        size_estimate: element.size_estimate.clone(),
        relations,
        attachments,
    })
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
            "{}  classDef userRequirement fill:#D1C4E9,stroke:#7E57C2,stroke-width:2px;\n",
            indent
        ));
        output.push_str(&format!(
            "{}  classDef systemRequirement fill:#E1D8EE,stroke:#673AB7,stroke-width:1.5px;\n",
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

                // Build label with attachments
                let mut elem_label = escape_label(&elem.name);
                for attachment in &elem.attachments {
                    elem_label.push_str(&format!("<br/>📎 {}", escape_label(attachment)));
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
    if lower == "user-requirement" {
        "userRequirement"
    } else if lower == "requirement" || lower.contains("system") && lower.contains("requirement") {
        "systemRequirement"
    } else if lower.contains("verification") {
        "verification"
    } else {
        "default"
    }
}
