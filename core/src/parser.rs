use crate::element::{Element, SubSection, ElementType, RequirementType};
use crate::relation::Relation;
use crate::error::ReqFlowError;
use crate::utils;
use log::debug;
use std::collections::HashSet;
use std::path::PathBuf;

/// Parses a markdown document and extracts elements with metadata and relations.
pub fn parse_elements(
    file: &str,
    content: &str,
    file_path: &PathBuf,
    specifications_folder: &PathBuf,
    external_folders_refs: &[PathBuf],
) -> (Vec<Element>, Vec<ReqFlowError>) {
    let mut elements = Vec::new();
    let mut current_element: Option<Element> = None;
    let mut errors = Vec::new();
    let mut seen_identifiers = HashSet::new();
    let mut skip_current_element = false;
    let mut seen_subsections = HashSet::new();
    let mut in_details_block = false; 


    let mut current_subsection = SubSection::Other("".to_string());
    let mut current_section_name = "Requirements";

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();


        if in_details_block {
            if !skip_current_element {
                if let Some(element) = &mut current_element {
                    element.add_content(&format!("{}\n", line));
                }
            }
        
            if trimmed.starts_with("</details>") {
                in_details_block = false;
            }
        
            continue; // Skip any further processing while in <details>        

        }else if trimmed == "---" {
            current_subsection = SubSection::Other("".to_string());

        } else if trimmed.starts_with("## ") {
            current_section_name = trimmed[3..].trim();
            current_subsection = SubSection::Other("".to_string());

        } else if trimmed.starts_with("### ") {
            current_subsection = SubSection::Requirement;

            if let Some(mut element) = current_element.take() {
                if !skip_current_element {
                    element.freeze_content();
                    elements.push(element);
                }
            }

            skip_current_element = false;
            seen_subsections.clear();

            let element_name = trimmed[4..].trim().to_string();

            match file_path.parent() {
                Some(file_folder) => {
                    let identifier = format!("{}#{}", file, element_name);

                    match utils::normalize_identifier(
                        &identifier,
                        &file_folder.to_path_buf(),
                        specifications_folder,
                        external_folders_refs,
                    ) {
                        Ok(identifier) => {
                            if seen_identifiers.contains(&identifier) {
                                let msg = format!(
                                    "'{}' already seen (file: {}, line {})",
                                    element_name,
                                    file_path.display(),
                                    line_num + 1
                                );
                                errors.push(ReqFlowError::DuplicateElement(msg.clone()));
                                debug!("Error: {}", msg);
                                skip_current_element = true;
                            } else {
                                seen_identifiers.insert(identifier.clone());

                                let element_type = if utils::is_in_specification_root(
                                    &file_folder.to_path_buf(),
                                    specifications_folder,
                                ) {
                                    ElementType::Requirement(RequirementType::User)
                                } else {
                                    ElementType::Requirement(RequirementType::System)
                                };

                                current_element = Some(Element::new(
                                    &element_name,
                                    &identifier,
                                    &file_path.to_string_lossy(),
                                    &current_section_name,
                                    Some(element_type),
                                ));
                                debug!("Found element: {}", element_name);
                            }
                        }
                        Err(e) => {
                            let msg = format!(
                                "Failed to normalize identifier for '{}': {} (file: {}, line {})",
                                element_name,
                                e,
                                file_path.display(),
                                line_num + 1
                            );
                            errors.push(ReqFlowError::InvalidIdentifier(msg.clone()));
                            debug!("Error: {}", msg);
                            skip_current_element = true;
                        }
                    }
                }
                None => {
                    let msg = format!(
                        "Failed to normalize identifier for '{}': {} (file: {}, line {})",
                        element_name,
                        "File folder not accessible.",
                        file_path.display(),
                        line_num + 1
                    );
                    errors.push(ReqFlowError::InvalidIdentifier(msg.clone()));
                    debug!("Error: {}", msg);
                    skip_current_element = true;
                }
            }

        } else if trimmed.starts_with("#### ") && current_element.is_some() {
            let subsection = SubSection::from_str(&trimmed[5..].trim());

            if !skip_current_element {
                if seen_subsections.contains(&subsection) {
                    let msg = format!(
                        "Duplicate subsection '{}' in element '{}' (file: {}, line {})",
                        subsection.name(),
                        current_element.as_ref().unwrap().name,
                        file_path.display(),
                        line_num + 1
                    );
                    errors.push(ReqFlowError::DuplicateSubsection(msg.clone()));
                    debug!("Error: {}", msg);
                } else {
                    seen_subsections.insert(subsection.clone());
                }
            }

            current_subsection = subsection;

        } else if (current_subsection == SubSection::Requirement || current_subsection == SubSection::Details)
            && !skip_current_element
        {
            if let Some(element) = &mut current_element {
                if trimmed.starts_with("<details") {
                    in_details_block = true;
                }
        
                element.add_content(&format!("{}\n", line));

            }

        } else if in_details_block && !skip_current_element {
            // Still inside <details> block under 'Details' subsection
            if let Some(element) = &mut current_element {
                element.add_content(&format!("{}\n", line));
            }

        } else if current_subsection == SubSection::Metadata && !skip_current_element {
            if trimmed.is_empty() {
                continue;
            }
            if let Some(element) = &mut current_element {
                if let Some((key, value)) = utils::parse_metadata_line(trimmed) {
                    element.metadata.insert(key.clone(), value.clone());

                    if key.eq_ignore_ascii_case("type") {
                        element.set_type_from_metadata();
                    }
                } else {
                    let msg = format!(
                        "Element '{}' has invalid metadata format: '{}' (file: {}, line {})",
                        element.name, trimmed, file, line_num + 1
                    );
                    errors.push(ReqFlowError::InvalidMetadataFormat(msg.clone()));
                    debug!("Error: {}", msg);
                    current_subsection = SubSection::Other("".to_string());
                }
            }

        } else if current_subsection == SubSection::Relations && !skip_current_element {
            if let Some(element) = &mut current_element {
                if trimmed.starts_with("* ") {
                    match utils::parse_relation_line(trimmed) {
                        Ok((relation_type, (text, link))) => {
                            let final_link = if link.starts_with('#') {
                                format!("{}{}", file, link)
                            } else {
                                link
                            };

                            match file_path.parent() {
                                Some(file_folder) => {
                                    match utils::normalize_identifier(
                                        &final_link,
                                        &file_folder.to_path_buf(),
                                        specifications_folder,
                                        external_folders_refs,
                                    ) {
                                        Ok(normalized_target) => {
                                            match Relation::new(&relation_type, text, &normalized_target) {
                                                Ok(relation) => {
                                                    element.add_relation(relation);
                                                }
                                                Err(_) => {
                                                    let msg = format!(
                                                        "'{}' in element '{}': (file: {}, line {})",
                                                        relation_type, element.name, file, line_num + 1
                                                    );
                                                    errors.push(ReqFlowError::UnsupportedRelationType(msg.clone()));
                                                    debug!("Error: {}", msg);
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            let msg = format!(
                                                "Failed to normalize identifier for '{}': {} (file: {}, line {})",
                                                element.name, e, file, line_num + 1
                                            );
                                            errors.push(ReqFlowError::InvalidIdentifier(msg.clone()));
                                            debug!("Error: {}", msg);
                                        }
                                    }
                                }
                                None => {
                                    let msg = format!(
                                        "Failed to normalize identifier for '{}': {} (file: {}, line {})",
                                        trimmed,
                                        "File folder not accessible.",
                                        file_path.display(),
                                        line_num + 1
                                    );
                                    errors.push(ReqFlowError::InvalidIdentifier(msg.clone()));
                                    debug!("Error: {}", msg);
                                }
                            }
                        }
                        Err(_) => {
                            let msg = format!(
                                "Element '{}' has invalid relation format: '{}'. (file: {}, line {})",
                                element.name, trimmed, file, line_num + 1
                            );
                            errors.push(ReqFlowError::UnsupportedRelationType(msg.clone()));
                            debug!("Error: {}", msg);
                        }
                    }
                } else if trimmed.is_empty() {
                    // Ignore
                } else {
                    let msg = format!(
                        "Element '{}' has invalid relations format: '{}' (file: {}, line {})",
                        element.name, trimmed, file, line_num + 1
                    );
                    errors.push(ReqFlowError::InvalidRelationFormat(msg.clone()));
                    debug!("Error: {}", msg);
                    current_subsection = SubSection::Other("".to_string());
                }
            }

        } else if matches!(current_subsection, SubSection::Other(_)) {
            // Skip
        }
    }

    // Final element
    if let Some(mut element) = current_element.take() {
        if !skip_current_element {
            element.freeze_content();
            elements.push(element);
        }
    }

    (elements, errors)
}


