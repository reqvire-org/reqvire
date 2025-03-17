use anyhow::Result;
use crate::element::{Element};
use crate::relation::{Relation};
use crate::element::ElementType;
use crate::element::RequirementType;

use crate::error::ReqFlowError;
use crate::utils;
use log::debug;
use std::path::PathBuf;


/// Parses a markdown document and extracts elements with metadata and relations.
pub fn parse_elements(file: &str, content: &str, file_path: &PathBuf,  specifications_folder: &PathBuf,external_folders_refs:&[PathBuf]) -> Result<Vec<Element>, Vec<ReqFlowError>> {
    let mut elements = Vec::new();
    let mut current_element: Option<Element> = None;
    let mut errors = Vec::new();
    let mut in_relations_section = false;
    let mut in_metadata_section = false;
    
    let mut current_section_name = "Requirements";

    
    // Reserved subsections that should not be treated as standalone elements
    let reserved_subsections = vec!["Relations", "Details", "Metadata"];

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("## ") {
            // Extract section name and update tracking variable
            current_section_name = trimmed[3..].trim();

            
        }else if trimmed.starts_with("### ") {
            // Save previous element
            if let Some(element) = current_element.take() {
                elements.push(element);
            }
            
            // Create a new element
            let element_name = trimmed[4..].trim().to_string();
            
            
            match file_path.parent(){
                Some(file_folder)=>{
                    let identifier=format!("{}#{}",file,element_name);
            
                    match utils::normalize_identifier(
                        &identifier, 
                        &file_folder.to_path_buf(),
                        specifications_folder, 
                        external_folders_refs
                    ) {
                        Ok(identifier) => {
                        
                            // Important: specifications root folder will auto-auseme user requirements
                            let element_type = if utils::is_in_specification_root(&file_folder.to_path_buf(),specifications_folder) {
                               ElementType::Requirement(RequirementType::User)
                            } else {
                               ElementType::Requirement(RequirementType::System)
                            };

                            current_element = Some(Element::new(&element_name.clone(), &identifier.clone(), &file_path.to_string_lossy().into_owned(),&current_section_name, Some(element_type)));
                            debug!("Found element: {}", element_name);
                        },
                        Err(e) => {
                            let msg=format!("Failed to normalize identifier for '{}': {} (file: {})", element_name, e, file_path.display());
                            errors.push(ReqFlowError::InvalidIdentifier(msg.clone()));
                            debug!("Error: {}",msg);
                        }
                    }                            
                },
                _ =>{

                        let msg=format!("Failed to normalize identifier for '{}': {} (file: {})", element_name, "File folder not accessible.", file_path.display());
                        errors.push(ReqFlowError::InvalidIdentifier(msg.clone()));
                        debug!("Error: {}",msg);
                }
            }
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
            // Strictly allow only metadata lines (e.g., "* key: value")
            if !trimmed.starts_with("* ") {
                in_metadata_section = false;
            } else if let Some(element) = &mut current_element {
                if let Some((key, value)) = utils::parse_metadata_line(trimmed) {
                    element.metadata.insert(key.clone(), value.clone());

                    // If "type" is specified, update the element type
                    if key.eq_ignore_ascii_case("type") {
                        element.set_type_from_metadata();
                    }
                } else {                                        
                    let msg = format!("{}: {} (file: {})", element.name.clone(), trimmed, file.to_string());
                    errors.push(ReqFlowError::InvalidMetadataFormat(msg.clone()));
                    debug!("Error: {}", msg);    
                                    
                    in_metadata_section = false; // Exit metadata section
                }
            }        
        } else if in_relations_section {
        
            // Strictly allow only relation lines (e.g., "* derivedFrom: identifier")
            if !trimmed.starts_with("* ") {
                in_relations_section = false;
            } else if let Some(element) = &mut current_element {            
                if let Ok((relation_type, (text, link))) = utils::parse_relation_line(trimmed) {
            
                    // Important: Fragment in file has to get a filename of the current document!
                    let final_link = if link.starts_with('#') {
                        format!("{}{}", file, link) 
                    } else {
                        link
                    };

                    match file_path.parent() {
                        Some(file_folder) => {
                            match utils::normalize_identifier(&final_link, &file_folder.to_path_buf(), specifications_folder, external_folders_refs) {
                                Ok(normalized_target) => {
                                    let relation_result = Relation::new(&relation_type, text, &normalized_target);
                                    match relation_result {
                                        Ok(relation) => {
                                            element.add_relation(relation);
                                        }
                                        Err(_) => {
                                            let msg = format!("{}: {} (file: {})", element.name.clone(), relation_type, file.to_string());
                                            errors.push(ReqFlowError::UnsupportedRelationType(msg.clone()));
                                            debug!("Error: {}", msg);
                                        }
                                    }                                                           
                                }
                                Err(e) => {
                                    let msg = format!("Failed to normalize identifier for '{}': {} (file: {})", element.name.clone(), e, file);
                                    errors.push(ReqFlowError::InvalidIdentifier(msg.clone()));
                                    debug!("Error: {}", msg);
                                }
                            }
                        }
                        _ => {
                            let msg = format!("Failed to normalize identifier for '{}': {} (file: {})", trimmed, "File folder not accessible.", file_path.display());
                            errors.push(ReqFlowError::InvalidIdentifier(msg.clone()));
                            debug!("Error: {}", msg);
                        }                                              
                    }
                } else {
                    debug!("Invalid relation format, stopping relation parsing.");
                    in_relations_section = false;  // Exit the relations section
                }
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

    if errors.is_empty(){
        Ok(elements)
    }else{
        Err(errors)
    }

}

#[cfg(test)]
mod tests {
    use super::*;


}


