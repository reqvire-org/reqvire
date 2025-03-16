use std::collections::{HashMap, HashSet};
use crate::element::Element;
use crate::element_registry::ElementRegistry;
use crate::error::ReqFlowError;
use log::debug;

/// Generates diagrams grouped by `file_path` and `section`
pub fn generate_diagrams_by_section(
    registry: &ElementRegistry,
    direction: & str,           
    convert_to_html: bool,
) -> Result<HashMap<String, String>, ReqFlowError> {
    let mut diagrams: HashMap<String, String> = HashMap::new();

    // Step 1: Group elements by (file_path, section)
    let mut grouped_elements: HashMap<(String, String), Vec<&Element>> = HashMap::new();
    
    let elements=registry.get_all_elements();
    
    for element in elements {
        grouped_elements
            .entry((element.file_path.clone(), element.section.clone()))
            .or_insert_with(Vec::new)
            .push(element);
    }

    // Step 2: Generate diagrams for each group
    for ((file_path, section), section_elements) in grouped_elements {
        debug!("Generating diagram for file: {}, section: {}", file_path, section);

        let diagram = generate_section_diagram(&section, &section_elements, registry, &file_path, direction, convert_to_html)?;
        let diagram_key = format!("{}::{}", file_path, section);
        diagrams.insert(diagram_key, diagram);
    }

    Ok(diagrams)
}

/// Generates a diagram for a single section
fn generate_section_diagram(
    _section: &str,
    elements: &[&Element],
    registry: &ElementRegistry,
    file_path: &str,
    direction: & str,               
    convert_to_html: bool
) -> Result<String, ReqFlowError> {


    // Get diagram direction from config (TD or LR)
    let mut diagram = String::from(format!("```mermaid\ngraph {};\n", direction));

    // Define Mermaid graph styles
    diagram.push_str("  %% Graph styling\n");
    diagram.push_str("  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;\n");
    diagram.push_str("  classDef satisfies fill:#fff2cc,stroke:#ffcc00,stroke-width:1px;\n");
    diagram.push_str("  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;\n");
    diagram.push_str("  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;\n");
    diagram.push_str("  classDef paragraph fill:#efefef,stroke:#999999,stroke-width:1px;\n");
    diagram.push_str("  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;\n\n");

    let mut included_elements = HashSet::new();

    for element in elements {
        add_element_to_diagram(&mut diagram, element, &mut included_elements, registry, file_path, convert_to_html)?;
    }

    diagram.push_str("```\n");
    Ok(diagram)
}

/// Adds an element and its relations to the diagram
fn add_element_to_diagram(
    diagram: &mut String,
    element: &Element,
    included_elements: &mut HashSet<String>,
    registry: &ElementRegistry,
    file_path: &str,
    convert_to_html: bool,
) -> Result<(), ReqFlowError> {
    let element_id = sanitize_id(&element.identifier);
    let label = element.name.replace('"', "&quot;");

    diagram.push_str(&format!("  {}[\"{}\"];\n", element_id, label));

    let html_file = if convert_to_html {
        convert_path_to_html_link(&element.file_path)
    } else {
        element.file_path.clone()
    };

    diagram.push_str(&format!("  click {} \"{}#{}\";\n", element_id, html_file, element.name.replace(' ', "-").to_lowercase()));
    diagram.push_str(&format!("  class {} requirement;\n", element_id));

    included_elements.insert(element.identifier.clone());

    for relation in &element.relations {
        let target_id = sanitize_id(relation.target.link.as_str());

        let properties = get_relation_properties(relation.relation_type.name);

        let (from_id, to_id) = if properties.reverse_direction {
            (target_id.clone(), element_id.clone())
        } else {
            (element_id.clone(), target_id.clone())
        };

        diagram.push_str(&format!("  {} {}|{}| {};\n", from_id, properties.arrow, properties.label, to_id));

        if !included_elements.contains(&relation.target.text) {
            add_target_to_diagram(diagram, &relation.target.text, &target_id, registry, file_path, convert_to_html)?;
            included_elements.insert(relation.target.text.clone());
        }
    }

    Ok(())
}

/// Adds a relation target to the diagram
fn add_target_to_diagram(
    diagram: &mut String,
    target_text: &str,
    target_id: &str,
    registry: &ElementRegistry,
    file_path: &str,
    convert_to_html: bool,
) -> Result<(), ReqFlowError> {
    let display_text = target_text.replace('"', "&quot;");
    diagram.push_str(&format!("  {}[\"{}\"];\n", target_id, display_text));

    if let Ok(target_elem) = registry.get_element(target_text) {
        let target_html_file = if convert_to_html {
            convert_path_to_html_link(&target_elem.file_path)
        } else {
            target_elem.file_path.clone()
        };

        diagram.push_str(&format!("  click {} \"{}#{}\";\n", target_id, target_html_file, target_elem.name.replace(' ', "-").to_lowercase()));
    } else {
        let doc_link = if convert_to_html {
            convert_path_to_html_link(file_path)
        } else {
            file_path.to_string()
        };
        let target_fragment = target_text.replace(' ', "-").to_lowercase();
        diagram.push_str(&format!("  click {} \"{}#{}\";\n", target_id, doc_link, target_fragment));
    }

    Ok(())
}

/// Returns relation properties (arrow, label, direction)
fn get_relation_properties(relation_type: &str) -> RelationProperties {
    match relation_type {
        "verifiedBy" => RelationProperties { arrow: "-->", label: "verifies", reverse_direction: true },
        "satisfiedBy" => RelationProperties { arrow: "-->", label: "satisfies", reverse_direction: true },
        "derivedFrom" => RelationProperties { arrow: "-.->", label: "deriveReqT", reverse_direction: false },
        "tracedFrom" => RelationProperties { arrow: "-->", label: "traces", reverse_direction: true },
        "containedBy" => RelationProperties { arrow: "--o", label: "contains", reverse_direction: true },
        "verify" => RelationProperties { arrow: "-->", label: "verifies", reverse_direction: false },
        "satisfy" => RelationProperties { arrow: "-->", label: "satisfies", reverse_direction: false },
        "derive" => RelationProperties { arrow: "-.->", label: "deriveReqT", reverse_direction: true },
        "refine" => RelationProperties { arrow: "==>", label: "refines", reverse_direction: false },
        "trace" => RelationProperties { arrow: "-->", label: "traces", reverse_direction: false },
        "contain" => RelationProperties { arrow: "--o", label: "contains", reverse_direction: false },
        _ => RelationProperties { arrow: "-->", label: "relates to", reverse_direction: false },
    }
}

/// Sanitizes identifiers for Mermaid compatibility
fn sanitize_id(identifier: &str) -> String {
    identifier.chars().map(|c| if c.is_alphanumeric() { c } else { '_' }).collect()
}

/// Converts markdown paths to HTML links
fn convert_path_to_html_link(path: &str) -> String {
    if path.ends_with(".md") {
        path.replace(".md", ".html")
    } else {
        path.to_string()
    }
}

/// Struct for relation properties
struct RelationProperties {
    arrow: &'static str,
    label: &'static str,
    reverse_direction: bool,
}

