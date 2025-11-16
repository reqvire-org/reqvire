use crate::graph_registry::GraphRegistry;
use crate::relation;
use crate::error::ReqvireError;
use crate::diagrams::escape_label;
use crate::utils::hash_identifier;
use serde::Serialize;
use std::collections::HashSet;

/// Model-centric report with nested element structure
#[derive(Debug, Serialize)]
pub struct ModelCentricReport {
    pub elements: Vec<ModelCentricElement>,
    pub metadata: ModelMetadata,
}

/// Element in model-centric view with nested relations
#[derive(Debug, Serialize)]
pub struct ModelCentricElement {
    pub identifier: String,
    pub name: String,
    pub element_type: String,
    pub file_path: String,
    pub section: String,
    pub section_index: usize,
    pub relations: Vec<ModelCentricRelation>,
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
}

/// Generate model-centric report
pub fn generate_model_report(
    registry: &GraphRegistry,
    root_element_name: Option<&str>,
    json_output: bool,
    diagram_direction: &str,  // "LR" or "TD"
) -> Result<String, ReqvireError> {
    use std::collections::HashSet;

    // Determine starting elements
    let mut starting_elements = if let Some(name) = root_element_name {
        // Find element by name
        let found = registry.nodes.iter()
            .find(|(_, node)| node.element.name == name);

        match found {
            Some((id, _)) => vec![id.clone()],
            None => {
                eprintln!("❌ Element with name '{}' not found", name);
                return Err(ReqvireError::ElementError(format!("Element with name '{}' not found", name)));
            }
        }
    } else {
        // No filter, use root requirements
        registry.find_root_requirements()
    };

    // Sort for deterministic output
    starting_elements.sort();

    // Build report
    let mut visited = HashSet::new();
    let mut elements = Vec::new();

    for start_id in &starting_elements {
        if let Some(element) = build_element_recursive(registry, start_id, &mut visited) {
            elements.push(element);
        }
    }

    // Count metadata
    let total_elements = starting_elements.len();
    let total_relations = count_relations(registry, &starting_elements);

    let report = ModelCentricReport {
        elements,
        metadata: ModelMetadata {
            total_elements,
            total_relations,
            filtered_from: root_element_name.map(|s| s.to_string()),
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
) -> Option<ModelCentricElement> {

    // Prevent infinite recursion
    if visited.contains(element_id) {
        return None;
    }
    visited.insert(element_id.to_string());

    let element = registry.get_element(element_id)?;

    // Sort element relations for deterministic iteration
    let mut sorted_relations = element.relations.clone();
    sorted_relations.sort_by(|a, b| {
        // First sort by relation type
        let type_cmp = a.relation_type.name.cmp(&b.relation_type.name);
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
        // Only include forward relations (diagram relations)
        if !relation::DIAGRAM_RELATIONS.contains(&relation.relation_type.name) {
            continue;
        }

        let target = match &relation.target.link {
            relation::LinkType::Identifier(target_id) => {
                // Recursive element
                if let Some(target_element) = build_element_recursive(registry, target_id, visited) {
                    RelationTarget::Element {
                        element: Box::new(target_element),
                    }
                } else {
                    continue; // Skip cyclic or missing
                }
            },
            relation::LinkType::InternalPath(path) => {
                RelationTarget::File {
                    path: path.to_string_lossy().to_string(),
                    target_type: "file".to_string(),
                }
            },
            relation::LinkType::ExternalUrl(url) => {
                RelationTarget::External {
                    url: url.clone(),
                    target_type: "external".to_string(),
                }
            },
        };

        relations.push(ModelCentricRelation {
            relation_type: relation.relation_type.name.to_string(),
            target,
        });
    }

    // Relations are already sorted from sorted_relations above

    Some(ModelCentricElement {
        identifier: element.identifier.clone(),
        name: element.name.clone(),
        element_type: element.element_type.as_str().to_string(),
        file_path: element.file_path.clone(),
        section: element.section.clone(),
        section_index: element.section_order_index,
        relations,
    })
}

/// Count total relations from starting elements
fn count_relations(registry: &GraphRegistry, starting_elements: &[String]) -> usize {
    let mut count = 0;
    let mut visited = HashSet::new();

    for start_id in starting_elements {
        count_relations_recursive(registry, start_id, &mut visited, &mut count);
    }

    count
}

/// Count relations recursively
fn count_relations_recursive(registry: &GraphRegistry, element_id: &str, visited: &mut HashSet<String>, count: &mut usize) {

    if visited.contains(element_id) {
        return;
    }
    visited.insert(element_id.to_string());

    if let Some(element) = registry.get_element(element_id) {
        for relation in &element.relations {
            if relation::DIAGRAM_RELATIONS.contains(&relation.relation_type.name) {
                *count += 1;

                // Recurse for identifier targets
                if let relation::LinkType::Identifier(target_id) = &relation.target.link {
                    count_relations_recursive(registry, &target_id, visited, count);
                }
            }
        }
    }
}

/// Generate text output for model-centric report with mermaid diagrams
fn generate_model_text(report: &ModelCentricReport, diagram_direction: &str) -> String {
    let mut output = String::new();

    output.push_str("# Model Structure\n\n");

    // Metadata
    output.push_str(&format!("**Total Elements**: {}\n", report.metadata.total_elements));
    output.push_str(&format!("**Total Relations**: {}\n", report.metadata.total_relations));
    if let Some(ref filtered) = report.metadata.filtered_from {
        output.push_str(&format!("**Filtered From**: {}\n", filtered));
    }
    output.push_str("\n");

    // Elements with mermaid diagrams
    for element in &report.elements {
        output.push_str(&generate_element_text(element, 0, diagram_direction));
    }

    output
}

/// Generate text for an element with mermaid diagram showing its relations
fn generate_element_text(element: &ModelCentricElement, depth: usize, diagram_direction: &str) -> String {
    let indent = "  ".repeat(depth);
    let mut output = String::new();

    // Element header
    output.push_str(&format!("{}## {}\n\n", indent, element.name));
    output.push_str(&format!("{}**Type**: {}\n", indent, element.element_type));
    output.push_str(&format!("{}**File**: [{}]({})\n", indent, element.file_path, element.file_path));
    output.push_str(&format!("{}**Section**: {}\n\n", indent, element.section));

    // Mermaid diagram for this element's relations
    if !element.relations.is_empty() {
        output.push_str(&format!("{}```mermaid\n", indent));
        output.push_str(&format!("{}graph {}\n", indent, diagram_direction));

        // Add CSS class definitions for colors
        output.push_str(&format!("{}  classDef userRequirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;\n", indent));
        output.push_str(&format!("{}  classDef systemRequirement fill:#fce4e4,stroke:#e68a8a,stroke-width:1px;\n", indent));
        output.push_str(&format!("{}  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;\n", indent));
        output.push_str(&format!("{}  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;\n", indent));
        output.push_str(&format!("{}\n", indent));

        output.push_str(&generate_mermaid_for_element(element, &indent));
        output.push_str(&format!("{}```\n\n", indent));
    }

    output
}

/// Generate mermaid diagram content for an element's relations recursively
fn generate_mermaid_for_element(element: &ModelCentricElement, indent: &str) -> String {
    let mut visited = HashSet::new();
    generate_mermaid_for_element_recursive(element, indent, &mut visited)
}

fn generate_mermaid_for_element_recursive(
    element: &ModelCentricElement,
    indent: &str,
    visited: &mut HashSet<String>,
) -> String {
    // Prevent infinite recursion
    if visited.contains(&element.identifier) {
        return String::new();
    }
    visited.insert(element.identifier.clone());

    let mut output = String::new();
    let element_id = hash_identifier(&element.identifier);

    // Determine CSS class based on element type
    let element_class = get_element_class(&element.element_type);

    // Add element node with class and click handler
    output.push_str(&format!("{}  {}[\"{}\"];\n", indent, element_id, escape_label(&element.name)));
    output.push_str(&format!("{}  class {} {};\n", indent, element_id, element_class));
    output.push_str(&format!("{}  click {} \"{}\";\n", indent, element_id, &element.identifier));

    for (idx, relation) in element.relations.iter().enumerate() {
        match &relation.target {
            RelationTarget::Element { element: target } => {
                let target_id = hash_identifier(&target.identifier);
                let target_class = get_element_class(&target.element_type);

                output.push_str(&format!("{}  {}[\"{}\"];\n", indent, target_id, escape_label(&target.name)));
                output.push_str(&format!("{}  class {} {};\n", indent, target_id, target_class));
                output.push_str(&format!("{}  click {} \"{}\";\n", indent, target_id, &target.identifier));
                output.push_str(&format!("{}  {} -->|{}| {};\n", indent, element_id, relation.relation_type, target_id));

                // Recursively show target element's relations
                output.push_str(&generate_mermaid_for_element_recursive(target, indent, visited));
            },
            RelationTarget::File { path, .. } => {
                let file_id = hash_identifier(&format!("file:{}:{}", element.identifier, idx));
                output.push_str(&format!("{}  {}[\"{}\"];\n", indent, file_id, escape_label(path)));
                output.push_str(&format!("{}  class {} default;\n", indent, file_id));
                output.push_str(&format!("{}  click {} \"{}\";\n", indent, file_id, path));
                output.push_str(&format!("{}  {} -->|{}| {};\n", indent, element_id, relation.relation_type, file_id));
            },
            RelationTarget::External { url, .. } => {
                let ext_id = hash_identifier(&format!("external:{}:{}", element.identifier, idx));
                output.push_str(&format!("{}  {}[\"{}\"];\n", indent, ext_id, escape_label(url)));
                output.push_str(&format!("{}  class {} default;\n", indent, ext_id));
                output.push_str(&format!("{}  click {} \"{}\";\n", indent, ext_id, url));
                output.push_str(&format!("{}  {} -->|{}| {};\n", indent, element_id, relation.relation_type, ext_id));
            },
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
