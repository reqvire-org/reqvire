use std::collections::{HashMap, HashSet};
use serde_json::{json, Value};
use std::path::PathBuf;

use crate::element_registry::ElementRegistry;
use crate::element::{Element, ElementType};
use crate::relation::{LinkType, RELATION_TYPES};
use crate::git_commands;
use crate::utils;

/// Enum to specify the matrix format
pub enum MatrixFormat {
    Markdown,
    Json,
    Svg,
}

/// Configuration for the traceability matrix
pub struct MatrixConfig {
    /// Source element type (e.g., "requirement")
    pub source_type: ElementType,
    /// Target element type (e.g., "verification")
    pub target_type: ElementType,
    /// Relation types to trace (e.g., ["verifiedBy"])
    pub relation_types: Vec<&'static str>,
    /// Matrix display format
    pub format: MatrixFormat,
}

impl Default for MatrixConfig {
    fn default() -> Self {
        MatrixConfig {
            source_type: ElementType::Requirement(crate::element::RequirementType::System),
            target_type: ElementType::Verification,
            relation_types: vec!["verifiedBy"],
            format: MatrixFormat::Markdown,
        }
    }
}

/// Generates a traceability matrix based on the provided configuration
pub fn generate_matrix(
    registry: &ElementRegistry,
    config: &MatrixConfig,
) -> String {
    // Get the git repository information for creating links
    let repo_root = match git_commands::repository_root() {
        Ok(root) => root,
        Err(_) => PathBuf::from(""),
    };
    
    let base_url = match git_commands::get_repository_base_url() {
        Ok(url) => url,
        Err(_) => String::from(""),
    };
    
    let commit_hash = match git_commands::get_commit_hash() {
        Ok(hash) => hash,
        Err(_) => String::from("HEAD"),
    };
    // Collect all source and target elements by type
    let source_elements: Vec<&Element> = registry.get_all_elements()
        .into_iter()
        .filter(|elem| matches_element_type(&elem.element_type, &config.source_type))
        .collect();

    let target_elements: Vec<&Element> = registry.get_all_elements()
        .into_iter()
        .filter(|elem| matches_element_type(&elem.element_type, &config.target_type))
        .collect();

    // Determine which relation types to use
    let relation_types: Vec<&str> = config.relation_types.clone();

    // Build the mapping of source to target elements
    let mut matrix_data: HashMap<String, HashSet<String>> = HashMap::new();
    
    for source in &source_elements {
        let source_id = &source.identifier;
        let mut targets = HashSet::new();
        
        // For each relation type we're interested in
        for rel_type in &relation_types {
            // Find all relations of this type in the element
            for relation in &source.relations {
                // Filter only for the specified relation types (e.g., "verifiedBy")
                // Ignore opposites (e.g., "verify") regardless of is_opposite flag
                if relation.relation_type.name == *rel_type {
                    if let LinkType::Identifier(target_id) = &relation.target.link {
                        // Check if the target exists and is of the correct type
                        if let Ok(target) = registry.get_element(target_id) {
                            if matches_element_type(&target.element_type, &config.target_type) {
                                targets.insert(target_id.clone());
                            }
                        }
                    }
                }
            }
        }
        
        if !targets.is_empty() {
            matrix_data.insert(source_id.clone(), targets);
        }
    }
    
    // Format the matrix according to the specified format
    match config.format {
        MatrixFormat::Markdown => generate_markdown_matrix(
            &matrix_data,
            &source_elements,
            &target_elements,
            &relation_types,
            registry,
            &repo_root,
            &base_url,
            &commit_hash,
        ),
        MatrixFormat::Json => generate_json_matrix(
            &matrix_data,
            &source_elements,
            &target_elements,
            &relation_types,
            registry,
            &repo_root,
            &base_url,
            &commit_hash,
        ),
        MatrixFormat::Svg => generate_svg_matrix(
            &matrix_data,
            &source_elements,
            &target_elements,
        ),
    }
}

/// Checks if an element type matches the specified type
fn matches_element_type(element_type: &ElementType, target_type: &ElementType) -> bool {
    match (element_type, target_type) {
        (ElementType::Requirement(_), ElementType::Requirement(_)) => true,
        (ElementType::Verification, ElementType::Verification) => true,
        (ElementType::File, ElementType::File) => true,
        (ElementType::Other(a), ElementType::Other(b)) => a == b,
        _ => false,
    }
}

/// Generates a Markdown representation of the traceability matrix
fn generate_markdown_matrix(
    matrix_data: &HashMap<String, HashSet<String>>,
    source_elements: &[&Element],
    target_elements: &[&Element],
    relation_types: &[&str],
    registry: &ElementRegistry,
    repo_root: &PathBuf,
    base_url: &str,
    commit_hash: &str,
) -> String {
    let mut output = String::new();
    
    // Add title and introduction
    output.push_str("# Traceability Matrix\n\n");
    output.push_str("This matrix shows the relationships between requirements and verification elements, organized by root requirements.\n\n");
    
    // Add relation types used
    output.push_str("## Relation Types Used\n\n");
    for rel_type in relation_types {
        if let Some(info) = RELATION_TYPES.get(rel_type) {
            output.push_str(&format!("- **{}**: {}\n", rel_type, info.description));
        } else {
            output.push_str(&format!("- **{}**\n", rel_type));
        }
    }
    output.push_str("\n");
    
    // Sort target elements by identifier for consistent output
    let mut sorted_target_elements = target_elements.to_vec();
    sorted_target_elements.sort_by(|a, b| a.identifier.cmp(&b.identifier));
    
    // Get requirements grouped by root requirements
    let requirements_by_root = registry.get_requirements_by_root();
    
    // If there are no requirements grouped, generate a single matrix
    if requirements_by_root.is_empty() {
        output.push_str("## All Requirements\n\n");
        
        // Generate table for all source elements
        generate_matrix_table(
            &sorted_target_elements,
            source_elements,
            matrix_data,
            &mut output,
            repo_root,
            base_url,
            commit_hash
        );
    } else {
        // Sort root requirements by identifier for consistent output
        let mut root_ids: Vec<String> = requirements_by_root.keys().cloned().collect();
        root_ids.sort();
        
        // Generate a matrix for each root requirement
        for root_id in &root_ids {
            if let Ok(root) = registry.get_element(root_id) {
                let root_name = &root.name;
                output.push_str(&format!("## {}\n\n", root_name));
                
                if let Some(group_elements) = requirements_by_root.get(root_id) {
                    // Generate table for this root and its children
                    generate_matrix_table(
                        &sorted_target_elements,
                        group_elements,
                        matrix_data,
                        &mut output,
                        repo_root,
                        base_url,
                        commit_hash
                    );
                }
                
                // Add space between matrices
                output.push_str("\n");
            }
        }
    }
    
    // Add legend
    output.push_str("## Legend\n\n");
    output.push_str("- ✅ (in 'Verified' column): Requirement is verified by at least one verification element\n");
    output.push_str("- ❌ (in 'Verified' column): Requirement is not verified by any verification element\n");
    output.push_str("- ✅ (in element columns): Direct relationship exists between requirement and verification\n");
    
    output
}

/// Helper function to generate a matrix table for a group of requirements
fn generate_matrix_table(
    all_targets: &[&Element],
    source_elements: &[&Element],
    matrix_data: &HashMap<String, HashSet<String>>,
    output: &mut String,
    repo_root: &PathBuf,
    base_url: &str,
    commit_hash: &str
) {
    // If no source elements, return early
    if source_elements.is_empty() {
        output.push_str("No requirements found for this group.\n");
        return;
    }
    
    // Filter targets that verify elements in this group
    let relevant_targets: Vec<&Element> = all_targets.iter()
        .filter(|target| {
            source_elements.iter().any(|source| {
                matrix_data.get(&source.identifier)
                    .map_or(false, |targets| targets.contains(&target.identifier))
            })
        })
        .cloned()
        .collect();
    
    // Start the table
    output.push_str("| Requirement | Verified |");
    
    // If no verifications for this group
    if relevant_targets.is_empty() {
        output.push_str("\n\n⚠️ No verification elements found for these requirements.\n");
        return;
    }
    
    // Add column headers (target elements)
    for target in &relevant_targets {
        let short_name = get_short_element_name(target);
        
        // Get a relative identifier for the target element
        let relative_id = match utils::get_relative_path_from_root(&PathBuf::from(&target.identifier), &repo_root) {
            Ok(rel_path) => rel_path.to_string_lossy().to_string(),
            Err(_) => target.identifier.clone(),
        };
        
        // Create a git link for the target element
        let target_url = format!("{}/blob/{}/{}", base_url, commit_hash, relative_id);
        output.push_str(&format!(" [{}]({}) |", short_name, target_url));
    }
    output.push_str("\n|");
    
    // Add header separator row
    for _ in 0..relevant_targets.len() + 2 { // +2 for requirement column and verified status column
        output.push_str(" --- |");
    }
    output.push_str("\n");
    
    // Organize source elements by hierarchy
    // This helps represent the parent-child relationships in the matrix
    let mut hierarchical_elements = Vec::new();
    
    // First, build a map from element ID to its children
    let mut parent_to_children: HashMap<String, Vec<&Element>> = HashMap::new();
    let parent_relation_types = crate::relation::get_parent_relation_types();
    
    // Identify parent-child relationships
    for source in source_elements {
        // Find the parent of this element, if any
        let mut has_parent = false;
        for relation in &source.relations {
            if parent_relation_types.contains(&relation.relation_type.name) {
                if let LinkType::Identifier(parent_id) = &relation.target.link {
                    // Add this element as a child of its parent
                    parent_to_children.entry(parent_id.clone())
                        .or_insert_with(Vec::new)
                        .push(source);
                    has_parent = true;
                    break;
                }
            }
        }
        
        // If no parent, this is a top-level element
        if !has_parent {
            hierarchical_elements.push((source, 0)); // 0 = top level (no indentation)
        }
    }
    
    // Function to recursively add children with proper indentation levels
    fn add_children<'a>(
        element: &'a Element,
        level: usize, 
        result: &mut Vec<(&'a Element, usize)>,
        parent_to_children: &HashMap<String, Vec<&'a Element>>
    ) {
        // Add the current element
        result.push((element, level));
        
        // Add all its children recursively
        if let Some(children) = parent_to_children.get(&element.identifier) {
            let mut sorted_children = children.to_vec();
            sorted_children.sort_by(|a, b| a.name.cmp(&b.name));
            
            for child in sorted_children {
                add_children(child, level + 1, result, parent_to_children);
            }
        }
    }
    
    // Sort the top-level elements by name for consistency
    let mut sorted_roots = hierarchical_elements.clone();
    sorted_roots.sort_by(|(a, _), (b, _)| a.name.cmp(&b.name));
    
    // Process all top-level elements and their children recursively
    let mut sorted_hierarchical = Vec::new();
    for (element, level) in sorted_roots {
        add_children(element, level, &mut sorted_hierarchical, &parent_to_children);
    }
    
    // Add rows
    for (source, indent_level) in sorted_hierarchical.iter() {
        let source_id = &source.identifier;
        let short_name = get_short_element_name(source);
        
        // Get a relative identifier for the source element
        let relative_id = match utils::get_relative_path_from_root(&PathBuf::from(&source.identifier), &repo_root) {
            Ok(rel_path) => rel_path.to_string_lossy().to_string(),
            Err(_) => source.identifier.clone(),
        };
        
        // Create an indented name with visible characters that won't break table formatting
        let indentation = match *indent_level {
            0 => "",
            1 => "↳ ",
            2 => "__↳ ",
            3 => "____↳ ",
            _ => "______↳ ", // For very deep levels, cap the indentation
        };
        let indented_name = format!("{}{}", indentation, short_name);
        
        // Create a git link for the source element
        let source_url = format!("{}/blob/{}/{}", base_url, commit_hash, relative_id);
        output.push_str(&format!("| [{}]({}) |", indented_name, source_url));
        
        // Add verification status column
        let is_verified = matrix_data.get(source_id).map_or(false, |targets| !targets.is_empty());
        if is_verified {
            output.push_str(" ✅ |"); // Green checkmark if verified by at least one element
        } else {
            output.push_str(" ❌ |"); // X mark if not verified
        }
        
        // Add relationship cells
        for target in &relevant_targets {
            let target_id = &target.identifier;
            if matrix_data.get(source_id).map_or(false, |targets| targets.contains(target_id)) {
                output.push_str(" ✅ |"); // Green checkmark for relationship exists
            } else {
                output.push_str("   |"); // Empty cell
            }
        }
        output.push_str("\n");
    }
}

/// Generates a JSON representation of the traceability matrix
fn generate_json_matrix(
    matrix_data: &HashMap<String, HashSet<String>>,
    source_elements: &[&Element],
    target_elements: &[&Element],
    relation_types: &[&str],
    registry: &ElementRegistry,
    repo_root: &PathBuf,
    base_url: &str,
    commit_hash: &str,
) -> String {
    // Create metadata section
    let source_type = if let Some(first_source) = source_elements.get(0) {
        format!("{:?}", first_source.element_type)
    } else {
        "Unknown".to_string()
    };
    
    let target_type = if let Some(first_target) = target_elements.get(0) {
        format!("{:?}", first_target.element_type)
    } else {
        "Unknown".to_string()
    };
    
    let timestamp = format!("{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs());
        
    let metadata = json!({
        "sourceType": source_type,
        "targetType": target_type,
        "relationTypes": relation_types,
        "totalSources": source_elements.len(),
        "totalTargets": target_elements.len(),
        "timestamp": timestamp,
    });
    
    // Organize source elements by hierarchy (similar to markdown output)
    let mut hierarchical_elements = Vec::new();
    let mut parent_to_children: HashMap<String, Vec<&Element>> = HashMap::new();
    let parent_relation_types = crate::relation::get_parent_relation_types();
    
    // Identify parent-child relationships
    for source in source_elements {
        let mut has_parent = false;
        for relation in &source.relations {
            if parent_relation_types.contains(&relation.relation_type.name) {
                if let LinkType::Identifier(parent_id) = &relation.target.link {
                    parent_to_children.entry(parent_id.clone())
                        .or_insert_with(Vec::new)
                        .push(source);
                    has_parent = true;
                    break;
                }
            }
        }
        
        if !has_parent {
            hierarchical_elements.push((source, 0));
        }
    }
    
    // Function to recursively add children with proper indentation levels
    fn add_children_json<'a>(
        element: &'a Element,
        level: usize, 
        result: &mut Vec<(&'a Element, usize)>,
        parent_to_children: &HashMap<String, Vec<&'a Element>>
    ) {
        result.push((element, level));
        
        if let Some(children) = parent_to_children.get(&element.identifier) {
            let mut sorted_children = children.to_vec();
            sorted_children.sort_by(|a, b| a.name.cmp(&b.name));
            
            for child in sorted_children {
                add_children_json(child, level + 1, result, parent_to_children);
            }
        }
    }
    
    // Sort the top-level elements by name for consistency
    let mut sorted_roots = hierarchical_elements.clone();
    sorted_roots.sort_by(|(a, _), (b, _)| a.name.cmp(&b.name));
    
    let mut sorted_hierarchical = Vec::new();
    for (element, level) in sorted_roots {
        add_children_json(element, level, &mut sorted_hierarchical, &parent_to_children);
    }

    // Create sources section with identifiers, names, hierarchy level, and git repository links
    let sources = sorted_hierarchical.iter().map(|(e, level)| {
        // Get a relative identifier for the source element
        let relative_id = match utils::get_relative_path_from_root(&PathBuf::from(&e.identifier), &repo_root) {
            Ok(rel_path) => rel_path.to_string_lossy().to_string(),
            Err(_) => e.identifier.clone(),
        };
        
        let source_url = format!("{}/blob/{}/{}", base_url, commit_hash, relative_id);
        json!({
            "id": relative_id,
            "name": e.name,
            "hierarchy_level": level,
            "type": format!("{:?}", e.element_type),
            "url": source_url,
        })
    }).collect::<Vec<Value>>();
    
    // Create targets section with identifiers, names, and git repository links
    let targets = target_elements.iter().map(|e| {
        // Get a relative identifier for the target element
        let relative_id = match utils::get_relative_path_from_root(&PathBuf::from(&e.identifier), &repo_root) {
            Ok(rel_path) => rel_path.to_string_lossy().to_string(),
            Err(_) => e.identifier.clone(),
        };
        
        let target_url = format!("{}/blob/{}/{}", base_url, commit_hash, relative_id);
        json!({
            "id": relative_id,
            "name": e.name,
            "type": format!("{:?}", e.element_type),
            "url": target_url,
        })
    }).collect::<Vec<Value>>();
    
    // Create matrix section with relationships using relative paths
    let matrix = matrix_data.iter().map(|(source_id, target_ids)| {
        // Convert source ID to relative path
        let rel_source_id = match utils::get_relative_path_from_root(&PathBuf::from(source_id), &repo_root) {
            Ok(rel_path) => rel_path.to_string_lossy().to_string(),
            Err(_) => source_id.clone(),
        };
        
        // Convert all target IDs to relative paths
        let rel_target_ids: Vec<String> = target_ids.iter().map(|target_id| {
            match utils::get_relative_path_from_root(&PathBuf::from(target_id), &repo_root) {
                Ok(rel_path) => rel_path.to_string_lossy().to_string(),
                Err(_) => target_id.clone(),
            }
        }).collect();
        
        (rel_source_id, json!(rel_target_ids))
    }).collect::<HashMap<String, Value>>();
    
    // Create verification status section with relative IDs
    let verification_status = source_elements.iter().map(|e| {
        let is_verified = matrix_data.get(&e.identifier).map_or(false, |targets| !targets.is_empty());
        
        // Convert to relative path
        let rel_id = match utils::get_relative_path_from_root(&PathBuf::from(&e.identifier), &repo_root) {
            Ok(rel_path) => rel_path.to_string_lossy().to_string(),
            Err(_) => e.identifier.clone(),
        };
        
        (rel_id, json!(is_verified))
    }).collect::<HashMap<String, Value>>();
    
    // Get requirements grouped by root requirements
    let requirements_by_root = registry.get_requirements_by_root();
    
    // Create a groups section mapping root requirements to their children using relative paths
    let mut groups = serde_json::Map::new();
    
    for (root_id, elements) in &requirements_by_root {
        // Convert root ID to relative path
        let rel_root_id = match utils::get_relative_path_from_root(&PathBuf::from(root_id), &repo_root) {
            Ok(rel_path) => rel_path.to_string_lossy().to_string(),
            Err(_) => root_id.clone(),
        };
        
        // Convert all element IDs to relative paths
        let element_ids: Vec<String> = elements.iter()
            .map(|e| {
                match utils::get_relative_path_from_root(&PathBuf::from(&e.identifier), &repo_root) {
                    Ok(rel_path) => rel_path.to_string_lossy().to_string(),
                    Err(_) => e.identifier.clone(),
                }
            })
            .collect();
        
        groups.insert(rel_root_id, json!(element_ids));
    }
    
    // Combine all sections
    let output = json!({
        "metadata": metadata,
        "sources": sources,
        "targets": targets,
        "matrix": matrix,
        "verificationStatus": verification_status,
        "groups": groups,
    });
    
    serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string())
}

/// Helper function to get a short display name for an element
fn get_short_element_name(element: &Element) -> String {
    let name_parts: Vec<&str> = element.name.split('/').collect();
    if let Some(last_part) = name_parts.last() {
        if last_part.len() > 30 {
            format!("{}...", &last_part[0..27])
        } else {
            last_part.to_string()
        }
    } else {
        // Fallback to the full name if splitting didn't work
        if element.name.len() > 30 {
            format!("{}...", &element.name[0..27])
        } else {
            element.name.clone()
        }
    }
}

/// Generates an SVG representation of the traceability matrix
fn generate_svg_matrix(
    matrix_data: &HashMap<String, HashSet<String>>,
    source_elements: &[&Element],
    target_elements: &[&Element],
) -> String {
    // SVG constants for styling
    const CELL_HEIGHT: i32 = 40;
    const HEADER_HEIGHT: i32 = 60;
    const FONT_SIZE: i32 = 14;
    const PADDING: i32 = 10;
    const MIN_CELL_WIDTH: i32 = 200;
    // Width per character (approximate for proportional fonts)
    const CHAR_WIDTH: i32 = 8;
    
    // Colors
    const HEADER_FILL: &str = "#f0f0f0";
    const CELL_FILL: &str = "#ffffff";
    const BORDER_COLOR: &str = "#d0d0d0";
    const TEXT_COLOR: &str = "#333333";
    const VERIFIED_COLOR: &str = "#4CAF50";
    const UNVERIFIED_COLOR: &str = "#F44336";
    
    // Begin SVG
    let mut svg = String::new();
    svg.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n");
    
    // Organize source elements by hierarchy (similar to markdown output)
    let mut hierarchical_elements = Vec::new();
    let mut parent_to_children: HashMap<String, Vec<&Element>> = HashMap::new();
    let parent_relation_types = crate::relation::get_parent_relation_types();
    
    // Identify parent-child relationships
    for source in source_elements {
        let mut has_parent = false;
        for relation in &source.relations {
            if parent_relation_types.contains(&relation.relation_type.name) {
                if let LinkType::Identifier(parent_id) = &relation.target.link {
                    parent_to_children.entry(parent_id.clone())
                        .or_insert_with(Vec::new)
                        .push(source);
                    has_parent = true;
                    break;
                }
            }
        }
        
        if !has_parent {
            hierarchical_elements.push((source, 0));
        }
    }
    
    // Function to recursively add children with proper indentation levels
    fn add_children_svg<'a>(
        element: &'a Element,
        level: usize, 
        result: &mut Vec<(&'a Element, usize)>,
        parent_to_children: &HashMap<String, Vec<&'a Element>>
    ) {
        result.push((element, level));
        
        if let Some(children) = parent_to_children.get(&element.identifier) {
            let mut sorted_children = children.to_vec();
            sorted_children.sort_by(|a, b| a.name.cmp(&b.name));
            
            for child in sorted_children {
                add_children_svg(child, level + 1, result, parent_to_children);
            }
        }
    }
    
    // Sort the top-level elements by name for consistency
    let mut sorted_roots = hierarchical_elements.clone();
    sorted_roots.sort_by(|(a, _), (b, _)| a.name.cmp(&b.name));
    
    let mut sorted_hierarchical = Vec::new();
    for (element, level) in sorted_roots {
        add_children_svg(element, level, &mut sorted_hierarchical, &parent_to_children);
    }
    
    // Filter relevant target elements 
    let relevant_targets: Vec<&Element> = target_elements.iter()
        .filter(|target| {
            sorted_hierarchical.iter().any(|(source, _)| {
                matrix_data.get(&source.identifier)
                    .map_or(false, |targets| targets.contains(&target.identifier))
            })
        })
        .cloned()
        .collect();
    
    // Calculate optimal column widths based on element name lengths
    // For requirement column, find the longest requirement name including indentation
    let mut req_column_width = MIN_CELL_WIDTH;
    for (source, indent_level) in &sorted_hierarchical {
        // Calculate indentation text length
        let indent_length = match *indent_level {
            0 => 0,
            1 => 2, // "↳ " = 2 chars
            2 => 4, // "__↳ " = 4 chars
            3 => 6, // "____↳ " = 6 chars
            _ => 8, // "______↳ " = 8 chars
        };
        
        // Calculate total length including indentation
        let total_length = indent_length + source.name.len() as i32;
        let needed_width = total_length * CHAR_WIDTH + PADDING * 2;
        
        if needed_width > req_column_width {
            req_column_width = needed_width;
        }
    }
    
    // For target columns, find the longest verification name
    let mut target_column_width = MIN_CELL_WIDTH;
    for target in &relevant_targets {
        let needed_width = target.name.len() as i32 * CHAR_WIDTH + PADDING * 2;
        if needed_width > target_column_width {
            target_column_width = needed_width;
        }
    }
    
    // Verified column can be narrower since it only contains checkmarks
    let verified_column_width = 80; // Narrower width for the verified column
    
    // Calculate SVG dimensions
    let width = req_column_width + verified_column_width + (target_column_width * relevant_targets.len() as i32);
    let height = HEADER_HEIGHT + (CELL_HEIGHT * sorted_hierarchical.len() as i32) + PADDING * 4; // Extra padding for legend
    
    svg.push_str(&format!(
        "<svg width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">\n",
        width, height, width, height
    ));
    
    // Add title
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" font-family=\"Arial\" font-size=\"{}\" font-weight=\"bold\">{}</text>\n",
        PADDING, PADDING * 2, FONT_SIZE + 4, "Traceability Matrix"
    ));
    
    // Add header row
    svg.push_str("<g>\n"); // Header group
    
    // Requirement column header
    svg.push_str(&format!(
        "<rect x=\"0\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"{}\" />\n",
        HEADER_HEIGHT - CELL_HEIGHT, req_column_width, CELL_HEIGHT, HEADER_FILL, BORDER_COLOR
    ));
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" font-family=\"Arial\" font-size=\"{}\" font-weight=\"bold\">{}</text>\n",
        PADDING, HEADER_HEIGHT - CELL_HEIGHT/2 + FONT_SIZE/2, FONT_SIZE, "Requirement"
    ));
    
    // Verified column header
    let verified_x = req_column_width;
    svg.push_str(&format!(
        "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"{}\" />\n",
        verified_x, HEADER_HEIGHT - CELL_HEIGHT, verified_column_width, CELL_HEIGHT, HEADER_FILL, BORDER_COLOR
    ));
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" font-family=\"Arial\" font-size=\"{}\" font-weight=\"bold\">{}</text>\n",
        verified_x + PADDING, HEADER_HEIGHT - CELL_HEIGHT/2 + FONT_SIZE/2, FONT_SIZE, "Verified"
    ));
    
    // Target columns headers
    for (i, target) in relevant_targets.iter().enumerate() {
        let x = req_column_width + verified_column_width + (i as i32 * target_column_width);
        svg.push_str(&format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"{}\" />\n",
            x, HEADER_HEIGHT - CELL_HEIGHT, target_column_width, CELL_HEIGHT, HEADER_FILL, BORDER_COLOR
        ));
        
        // Use the full name for target elements in SVG
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" font-family=\"Arial\" font-size=\"{}\" font-weight=\"bold\">{}</text>\n",
            x + PADDING, HEADER_HEIGHT - CELL_HEIGHT/2 + FONT_SIZE/2, FONT_SIZE, target.name
        ));
    }
    svg.push_str("</g>\n");
    
    // Add data rows
    for (row_idx, (source, indent_level)) in sorted_hierarchical.iter().enumerate() {
        let y = HEADER_HEIGHT + (row_idx as i32 * CELL_HEIGHT);
        let source_id = &source.identifier;
        
        // Requirement cell
        svg.push_str(&format!(
            "<rect x=\"0\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"{}\" />\n",
            y, req_column_width, CELL_HEIGHT, CELL_FILL, BORDER_COLOR
        ));
        
        // Create indentation string for hierarchy
        let indent_text = match *indent_level {
            0 => "",
            1 => "↳ ",
            2 => "__↳ ",
            3 => "____↳ ",
            _ => "______↳ ",
        };
        
        // Display the full element name (no truncation in SVG)
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" font-family=\"Arial\" font-size=\"{}\" fill=\"{}\">{}{}</text>\n",
            PADDING, y + CELL_HEIGHT/2 + FONT_SIZE/2, FONT_SIZE, TEXT_COLOR, indent_text, source.name
        ));
        
        // Verified status cell
        let verified_x = req_column_width;
        svg.push_str(&format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"{}\" />\n",
            verified_x, y, verified_column_width, CELL_HEIGHT, CELL_FILL, BORDER_COLOR
        ));
        
        // Add verification status
        let is_verified = matrix_data.get(source_id).map_or(false, |targets| !targets.is_empty());
        if is_verified {
            // Green checkmark for verified
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" font-family=\"Arial\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
                verified_x + PADDING, y + CELL_HEIGHT/2 + FONT_SIZE/2, FONT_SIZE + 2, VERIFIED_COLOR, "✅"
            ));
        } else {
            // Red X for not verified
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" font-family=\"Arial\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
                verified_x + PADDING, y + CELL_HEIGHT/2 + FONT_SIZE/2, FONT_SIZE + 2, UNVERIFIED_COLOR, "❌"
            ));
        }
        
        // Relationship cells for targets
        for (i, target) in relevant_targets.iter().enumerate() {
            let x = req_column_width + verified_column_width + (i as i32 * target_column_width);
            
            svg.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"{}\" />\n",
                x, y, target_column_width, CELL_HEIGHT, CELL_FILL, BORDER_COLOR
            ));
            
            let target_id = &target.identifier;
            if matrix_data.get(source_id).map_or(false, |targets| targets.contains(target_id)) {
                // Green checkmark for relationship
                svg.push_str(&format!(
                    "<text x=\"{}\" y=\"{}\" font-family=\"Arial\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
                    x + PADDING, y + CELL_HEIGHT/2 + FONT_SIZE/2, FONT_SIZE + 2, VERIFIED_COLOR, "✅"
                ));
            }
        }
    }
    
    // Add legend
    let legend_y = HEADER_HEIGHT + (sorted_hierarchical.len() as i32 * CELL_HEIGHT) + PADDING * 2;
    
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" font-family=\"Arial\" font-size=\"{}\" font-weight=\"bold\">{}</text>\n",
        PADDING, legend_y, FONT_SIZE + 2, "Legend:"
    ));
    
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" font-family=\"Arial\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
        PADDING, legend_y + FONT_SIZE + PADDING, FONT_SIZE, TEXT_COLOR, "✅ - Element is verified or relationship exists"
    ));
    
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" font-family=\"Arial\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
        PADDING, legend_y + (FONT_SIZE + PADDING) * 2, FONT_SIZE, TEXT_COLOR, "❌ - Element is not verified"
    ));
    
    // Close SVG
    svg.push_str("</svg>\n");
    
    svg
}

