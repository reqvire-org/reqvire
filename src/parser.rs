use crate::config::Config;
use crate::identifier::normalize_identifier;
use crate::element::{Element, ElementType};
use crate::relation::Relation;
use crate::error::ReqFlowError;
use crate::utils;
use log::debug;
use regex::Regex;


use crate::element::{Element, ElementType};
use crate::relation::Relation;
use crate::error::ReqFlowError;
use crate::utils;
use log::debug;

/// Parses a markdown document and extracts elements with metadata and relations.
pub fn parse_elements(content: &str, file_path: &str) -> Result<Vec<Element>, ReqFlowError> {
    let mut elements = Vec::new();
    let mut current_element: Option<Element> = None;
    let mut in_relations_section = false;
    let mut in_metadata_section = false;
    
    // Reserved subsections that should not be treated as standalone elements
    let reserved_subsections = vec!["Relations", "Details", "Properties", "Metadata"];

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("### ") {
            // Save previous element
            if let Some(element) = current_element.take() {
                elements.push(element);
            }
            
            // Create a new element
            let element_name = trimmed[4..].trim().to_string();
            let identifier = utils::normalize_identifier(file_path, &element_name);
            current_element = Some(Element::new(element_name.clone(), identifier, file_path.to_string()));

            debug!("Found element: {}", element_name);
            in_relations_section = false;
            in_metadata_section = false;
        } else if trimmed.starts_with("#### ") && current_element.is_some() {
            // Level 4 headers are subsections within elements
            let subsection_name = trimmed[5..].trim().to_string();

            if reserved_subsections.contains(&subsection_name.as_str()) {
                match subsection_name.as_str() {
                    "Relations" => {
                        in_relations_section = true;
                        in_metadata_section = false;
                    }
                    "Metadata" => {
                        in_metadata_section = true;
                        in_relations_section = false;
                    }
                    _ => {
                        in_relations_section = false;
                        in_metadata_section = false;
                    }
                }
            }

            if let Some(element) = &mut current_element {
                element.add_content(&format!("{}\n", line));
            }
        } else if in_metadata_section {
            // Parse metadata properties
            if let Some(element) = &mut current_element {
                if let Some((key, value)) = utils::parse_metadata_line(trimmed) {
                    element.metadata.insert(key.clone(), value.clone());

                    // If "type" is specified, update the element type
                    if key == "type" {
                        element.set_type_from_metadata();
                    }
                }
                element.add_content(&format!("{}\n", line));
            }
        } else if in_relations_section {
            // Parse relations
            if let Some(element) = &mut current_element {
                if let Some((relation_type, target)) = utils::parse_relation_line(trimmed) {
                    let normalized_target = utils::normalize_identifier(file_path, &target);
                    let relation = Relation::new(relation_type.clone(), normalized_target.clone(), target.clone());

                    // Prevent duplicate relations
                    if !element.relations.iter().any(|r| r.relation_type == relation_type && r.target == normalized_target) {
                        element.add_relation(relation);
                    }
                }
                element.add_content(&format!("{}\n", line));
            }
        } else {
            // Regular content
            if let Some(element) = &mut current_element {
                element.add_content(&format!("{}\n", line));
            }
        }
    }

    // Add last element if exists
    if let Some(element) = current_element.take() {
        elements.push(element);
    }

    Ok(elements)
}

