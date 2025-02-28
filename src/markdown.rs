use anyhow::Result;
use log::info;
use regex::Regex;
use std::path::Path;

use crate::config::Config;
use crate::element::{Element, ElementRegistry};
use crate::error::ReqFlowError;
use crate::relation::Relation;

/// Parse a markdown document and extract elements
pub fn parse_elements(content: &str, file_path: &str) -> Result<Vec<Element>, ReqFlowError> {
    let mut elements = Vec::new();
    let mut current_element: Option<Element> = None;
    let mut in_relations_section = false;
    
    // Use the config's subsection regex
    
    // List of reserved subsections that should never be treated as elements
    let reserved_subsections = vec!["Relations", "Details", "Properties", "Metadata"];
    
    for line in content.lines() {
        if line.trim() == "### Relations" {
            // Special case: "### Relations" could be mistaken for an element but should be treated as a Relations subsection
            if current_element.is_some() {
                in_relations_section = true;
                
                // Add the relations header to the element content
                if let Some(element) = &mut current_element {
                    element.add_content(&format!("{}\n", line));
                }
            } else {
                // We found a Relations header outside of an element - log it but don't create an element for it
                info!("Found '### Relations' outside of an element in {}", file_path);
            }
        } else if Config::element_regex().is_match(line) {
            // Save previous element if exists
            if let Some(element) = current_element.take() {
                elements.push(element);
            }
            
            // Create new element from this header
            let element_name = Config::element_regex()
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .trim()
                .to_string();
            
            current_element = Some(Element::new(element_name, file_path.to_string()));
            in_relations_section = false;
            
            // Add the header line to the element content
            if let Some(element) = &mut current_element {
                element.add_content(&format!("{}\n", line));
            }
        } else if let Some(subsection_match) = Config::subsection_regex().captures(line) {
            // Got a level 4 header (subsection)
            let subsection_name = subsection_match.get(1).unwrap().as_str().trim();
            
            // Check if this is a reserved subsection
            if reserved_subsections.contains(&subsection_name) {
                // Check that we're inside an element
                if current_element.is_some() {
                    // Set in_relations_section flag if this is a Relations subsection
                    if subsection_name == "Relations" {
                        in_relations_section = true;
                    } else {
                        in_relations_section = false;
                    }
                    
                    // Add the subsection header to the element content
                    if let Some(element) = &mut current_element {
                        element.add_content(&format!("{}\n", line));
                    }
                } else {
                    // We found a subsection header outside of an element - this is unusual
                    // but we shouldn't create an element for it
                    info!("Found {} subsection header outside of an element in {}", subsection_name, file_path);
                }
            } else {
                // Not a reserved subsection, treat as normal content
                if let Some(element) = &mut current_element {
                    element.add_content(&format!("{}\n", line));
                }
            }
        } else if in_relations_section && (line.trim().starts_with("* ") || line.trim().starts_with("  * ") || 
                                          line.trim().starts_with("- ") || line.trim().starts_with("  - ")) {
            // Parse relation - be more flexible to match ReqFlow format
            // First attempt with the precise regex
            let captures = Config::relation_regex().captures(line);
            
            // If that doesn't work, try a more forgiving approach
            let captures = if captures.is_none() {
                // First attempt with single-character bullet + space
                let bullet_type = if line.trim().starts_with("-") { r"\-" } else { r"\*" };
                let alt_regex1 = Regex::new(&format!(r"{}\s+(\w+):\s*(.+)", bullet_type)).unwrap();
                
                // Second attempt with just looking for type: target pattern anywhere in the line
                let alt_regex2 = Regex::new(r"(\w+):\s*(.+)").unwrap();
                
                // Try both alternatives
                alt_regex1.captures(line).or_else(|| {
                    // Log an attempt with alt_regex2
                    info!("Trying flexible relation parsing for line: {}", line);
                    alt_regex2.captures(line)
                })
            } else {
                captures
            };
            
            if let Some(captures) = captures {
                let relation_type = captures.get(1).unwrap().as_str().to_string();
                let target = captures.get(2).unwrap().as_str().trim().to_string();
                
                // More detailed logging to help diagnose issues
                info!("Found relation: {} -> {} in element '{}'", relation_type, target, 
                      current_element.as_ref().map_or("Unknown", |e| &e.name));
                let relation = Relation::new(relation_type, target);
                
                if let Some(element) = &mut current_element {
                    // Check for duplicates in the current element's relations
                    let is_duplicate = element.relations.iter().any(|r| 
                        r.relation_type == relation.relation_type && r.target == relation.target
                    );
                    
                    if is_duplicate {
                        info!("Found duplicate relation: {} -> {} in element '{}'", 
                              relation.relation_type, relation.target, element.name);
                    }
                    
                    // Add it regardless for later validation
                    element.add_relation(relation);
                    element.add_content(&format!("{}\n", line));
                    // Log total relations after adding
                    info!("Element '{}' now has {} relations", element.name, element.relations.len());
                }
            } else {
                // If it doesn't match the relation pattern, still add it as content
                if let Some(element) = &mut current_element {
                    element.add_content(&format!("{}\n", line));
                }
            }
        } else {
            // Regular content line
            if line.trim().starts_with("###") && !line.trim().starts_with("####") {
                in_relations_section = false;
            }
            
            if let Some(element) = &mut current_element {
                element.add_content(&format!("{}\n", line));
            }
        }
    }
    
    // Add the last element if exists
    if let Some(element) = current_element.take() {
        elements.push(element);
    }
    
    Ok(elements)
}

/// Collect all elements from a document and add them to the registry
pub fn collect_elements(
    content: &str,
    file_path: &str,
    registry: &mut ElementRegistry,
) -> Result<(), ReqFlowError> {
    let elements = parse_elements(content, file_path)?;
    
    for element in elements {
        registry.add_element(element)?;
    }
    
    Ok(())
}

/// Replace relations in markdown with proper links
pub fn replace_relations(
    content: &str,
    _registry: &ElementRegistry,
    current_file: &Path,
    convert_to_html: bool,
) -> Result<String, ReqFlowError> {
    use crate::relation::process_relations;
    
    // Process the content to replace relations with markdown links
    let updated_content = process_relations(content, current_file, convert_to_html)?;
    
    Ok(updated_content)
}