use crate::element::ElementType;
use crate::element::RequirementType;
use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;

/// Default diagram direction (LR = left-to-right, TD = top-down)
pub const DEFAULT_DIAGRAM_DIRECTION: &str = "TD";

/// Escape special characters in labels for Mermaid
pub fn escape_label(text: &str) -> String {
    text.replace('"', "&quot;")
        .replace('[', "&#91;")
        .replace(']', "&#93;")
        .replace('(', "&#40;")
        .replace(')', "&#41;")
}

/// Generate containment view diagram showing folder/file/element hierarchy
///
/// When `short` is true, shows only root elements (those without hierarchical parents).
/// When `short` is false (default), shows all elements.
pub fn generate_containment_diagram(
    registry: &GraphRegistry,
    short: bool,
) -> Result<String, ReqvireError> {
    // Build containment hierarchy structure
    let hierarchy = crate::containment::ContainmentHierarchy::build(registry, short)?;

    let mut output = String::new();

    // Note about element display mode
    if short {
        output.push_str("*Elements filtered to show only root elements (those without hierarchical parent relations within the same file).*\n\n");
    }
    output.push_str("```mermaid\n");
    output.push_str("graph TD\n");

    // CSS class definitions
    output.push_str("  %% Graph styling\n");
    output.push_str("  classDef capability fill:#BBDEFB,stroke:#1976D2,stroke-width:2.5px;\n");
    output
        .push_str("  classDef systemRequirement fill:#E1D8EE,stroke:#673AB7,stroke-width:1.5px;\n");
    output.push_str("  classDef requirement fill:#ECEFF1,stroke:#673AB7,stroke-width:1.5px;\n");
    output.push_str("  classDef ontology fill:#F4E3A1,stroke:#B08A00,stroke-width:2px;\n");
    output.push_str("  classDef verification fill:#DCEDC8,stroke:#4CAF50,stroke-width:2px;\n");
    output.push_str("  classDef default fill:#F5F5F5,stroke:#424242,stroke-width:1.5px;\n");
    output.push_str("  classDef folder fill:#FAFAFA,stroke:#9E9E9E,stroke-width:2px;\n");
    output.push_str("  classDef file fill:#FFF8E1,stroke:#FFCA28,stroke-width:2px;\n");
    output.push_str(
        "  classDef reused_contract_context fill:#EFEBE9,stroke:#8D6E63,stroke-width:1.5px;\n\n",
    );

    // Define root node
    output.push_str("  root[\"📁 Reqvire root\"]\n");
    output.push_str("  class root folder\n\n");

    // Generate tree structure from hierarchy
    generate_folder_tree(&hierarchy.root_folder, "root", &mut output)?;
    output.push('\n');

    // Collect all elements for styling and links
    let all_elements = collect_all_elements(&hierarchy.root_folder);
    let all_design_docs = collect_all_design_documents(&hierarchy.root_folder);

    // Generate element styling
    output.push_str("  %% Element type styling\n");
    for element in &all_elements {
        let hash_id = generate_element_hash(&element.identifier);
        let class_name = get_element_class_from_type(&element.element_type);
        output.push_str(&format!("  class {} {}\n", hash_id, class_name));
    }

    // Generate clickable links
    output.push_str("\n  %% Clickable links\n");
    // Design documents first
    for doc in &all_design_docs {
        let doc_id = sanitize_design_doc_id(&doc.path);
        output.push_str(&format!("  click {} \"{}\"\n", doc_id, doc.path));
    }
    // Elements
    for element in &all_elements {
        let hash_id = generate_element_hash(&element.identifier);
        let fragment = element.identifier.split('#').nth(1).unwrap_or("");
        output.push_str(&format!(
            "  click {} \"{}#{}\"\n",
            hash_id, element.file_path, fragment
        ));
    }

    output.push_str("```\n");

    Ok(output)
}

/// Generate folder tree structure recursively using containment hierarchy
fn generate_folder_tree(
    folder: &crate::containment::ContainmentFolder,
    parent_id: &str,
    output: &mut String,
) -> Result<(), ReqvireError> {
    // Generate subfolders
    for subfolder in &folder.subfolders {
        let folder_id = sanitize_folder_id(&subfolder.path);
        output.push_str(&format!("  {}[\"📁 {}\"]\n", folder_id, subfolder.name));
        output.push_str(&format!("  {} --> {}\n", parent_id, folder_id));
        output.push_str(&format!("  class {} folder\n", folder_id));

        // Recursively generate subfolder contents
        generate_folder_tree(subfolder, &folder_id, output)?;
    }

    // Generate design documents
    for doc in &folder.design_documents {
        let doc_id = sanitize_design_doc_id(&doc.path);
        output.push_str(&format!("  {}[\"📝 {}\"]\n", doc_id, doc.name));
        output.push_str(&format!("  {} --> {}\n", parent_id, doc_id));
        output.push_str(&format!("  class {} reused_contract_context\n", doc_id));
    }

    // Generate files
    for file in &folder.files {
        let file_id = sanitize_file_id(&file.path);

        // Create subgraph for file containing elements
        output.push_str(&format!("  subgraph {}[\"📄 {}\"]\n", file_id, file.name));
        output.push_str("    direction TB\n");

        // Generate element nodes with reused_contract_context
        for element in &file.elements {
            let hash_id = generate_element_hash(&element.identifier);
            let mut label = escape_label(&element.name);
            // Add reused_contract_context to label
            for reused_contract_context in &element.reused_contract_context {
                label.push_str(&format!(
                    "<br/>📎 {}",
                    escape_label(&reused_contract_context.name)
                ));
            }
            output.push_str(&format!("    {}[\"{}\"]\n", hash_id, label));
        }

        output.push_str("  end\n");
        output.push_str(&format!("  {} --> {}\n", parent_id, file_id));
    }

    Ok(())
}

/// Collect all elements from folder hierarchy for styling and links
fn collect_all_elements(
    folder: &crate::containment::ContainmentFolder,
) -> Vec<crate::containment::ContainmentElement> {
    let mut elements = Vec::new();

    // Collect from files in this folder
    for file in &folder.files {
        elements.extend(file.elements.clone());
    }

    // Recursively collect from subfolders
    for subfolder in &folder.subfolders {
        elements.extend(collect_all_elements(subfolder));
    }

    // Sort for deterministic output
    elements.sort_by(|a, b| a.identifier.cmp(&b.identifier));

    elements
}

/// Collect all design documents from folder hierarchy for click links
fn collect_all_design_documents(
    folder: &crate::containment::ContainmentFolder,
) -> Vec<crate::containment::DesignDocument> {
    let mut docs = Vec::new();

    // Collect from this folder
    docs.extend(folder.design_documents.clone());

    // Recursively collect from subfolders
    for subfolder in &folder.subfolders {
        docs.extend(collect_all_design_documents(subfolder));
    }

    // Sort for deterministic output
    docs.sort_by(|a, b| a.path.cmp(&b.path));

    docs
}

fn sanitize_folder_id(path: &[String]) -> String {
    path.join("_")
        .replace(".", "")
        .replace("-", "_")
        .replace(" ", "_")
}

fn sanitize_file_id(name: &str) -> String {
    name.replace(".md", "")
        .replace(".", "")
        .replace("-", "_")
        .replace(" ", "_")
}

fn sanitize_design_doc_id(path: &str) -> String {
    path.replace("/", "_")
        .replace(".md", "")
        .replace(".", "")
        .replace("-", "_")
        .replace(" ", "_")
}

fn generate_element_hash(identifier: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    identifier.hash(&mut hasher);
    let hash = hasher.finish();
    format!("{:016x}", hash)
}

fn get_element_class_from_type(element_type: &ElementType) -> &'static str {
    match element_type {
        ElementType::Capability => "capability",
        ElementType::Ontology => "ontology",
        ElementType::Requirement(RequirementType::System) => "systemRequirement",
        ElementType::Verification(_) => "verification",
        _ => "default",
    }
}

/// Generate containment view as D3.js collapsible tree
///
/// Generates a markdown code block with `d3-tree` language containing JSON data
/// that can be rendered as an interactive collapsible tree in the Explorer.
///
/// When `short` is true, shows only root elements (those without hierarchical parents).
/// When `short` is false (default), shows all elements.
pub fn generate_containment_d3_tree(
    registry: &GraphRegistry,
    short: bool,
) -> Result<String, ReqvireError> {
    // Build containment hierarchy structure
    let hierarchy = crate::containment::ContainmentHierarchy::build(registry, short)?;

    // Convert to D3 tree format
    let d3_tree = hierarchy.to_d3_tree();

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&d3_tree).map_err(|e| {
        ReqvireError::SerializationError(format!("Failed to serialize D3 tree: {}", e))
    })?;

    let mut output = String::new();

    // Note about element display mode
    if short {
        output.push_str("*Elements filtered to show only root elements (those without hierarchical parent relations within the same file).*\n\n");
    }

    // Output as d3-tree code block
    output.push_str("```d3-tree\n");
    output.push_str(&json);
    output.push_str("\n```\n");

    Ok(output)
}

/// Generate containment view as D3.js sunburst diagram
///
/// Generates a markdown code block with `d3-sunburst` language containing JSON data
/// that can be rendered as an interactive sunburst diagram in the Explorer.
///
/// Uses the same hierarchical JSON format as the D3 tree.
pub fn generate_containment_d3_sunburst(
    registry: &GraphRegistry,
    short: bool,
) -> Result<String, ReqvireError> {
    // Build containment hierarchy structure
    let hierarchy = crate::containment::ContainmentHierarchy::build(registry, short)?;

    // Convert to D3 tree format (same format works for sunburst)
    let d3_tree = hierarchy.to_d3_tree();

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&d3_tree).map_err(|e| {
        ReqvireError::SerializationError(format!("Failed to serialize D3 sunburst: {}", e))
    })?;

    let mut output = String::new();

    // Note about element display mode
    if short {
        output.push_str("*Elements filtered to show only root elements (those without hierarchical parent relations within the same file).*\n\n");
    }

    // Output as d3-sunburst code block
    output.push_str("```d3-sunburst\n");
    output.push_str(&json);
    output.push_str("\n```\n");

    Ok(output)
}

/// Generate D3.js icicle/partition diagram for containment view
pub fn generate_containment_d3_icicle(
    registry: &GraphRegistry,
    short: bool,
) -> Result<String, ReqvireError> {
    // Build containment hierarchy structure (same as sunburst/tree)
    let hierarchy = crate::containment::ContainmentHierarchy::build(registry, short)?;

    // Convert to D3 tree format (works for icicle too)
    let d3_tree = hierarchy.to_d3_tree();

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&d3_tree).map_err(|e| {
        ReqvireError::SerializationError(format!("Failed to serialize D3 icicle: {}", e))
    })?;

    let mut output = String::new();

    if short {
        output.push_str("*Elements filtered to show only root elements (those without hierarchical parent relations within the same file).*\n\n");
    }

    // Output as d3-icicle code block
    output.push_str("```d3-icicle\n");
    output.push_str(&json);
    output.push_str("\n```\n");

    Ok(output)
}
