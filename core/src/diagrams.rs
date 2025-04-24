use std::collections::{HashMap, HashSet};
use crate::element::Element;
use crate::element_registry::ElementRegistry;
use crate::error::ReqFlowError;
use std::path::PathBuf;
use crate::utils;
use log::debug;
use crate::relation;
use crate::element::ElementType;
use crate::element::RequirementType;
use crate::git_commands;

/// Generates diagrams grouped by `file_path` and `section`
pub fn generate_diagrams_by_section(
    registry: &ElementRegistry,
    direction: & str,        
    specification_folder: &PathBuf, 
    external_folders: &[PathBuf],        
) -> Result<HashMap<String, String>, ReqFlowError> {
    let mut diagrams: HashMap<String, String> = HashMap::new();

    // Group elements by (file_path, section)
    let mut grouped_elements: HashMap<(String, String), Vec<&Element>> = HashMap::new();
    
    let elements=registry.get_all_elements();
    
    for element in elements {
        grouped_elements
            .entry((element.file_path.clone(), element.section.clone()))
            .or_insert_with(Vec::new)
            .push(element);
    }

    // Generate diagrams for each group
    for ((file_path, section), section_elements) in grouped_elements {
        debug!("Generating diagram for file: {}, section: {}", file_path, section);

        let diagram = generate_section_diagram(registry, &section, &section_elements, &file_path, direction, specification_folder, external_folders)?;
        let diagram_key = format!("{}::{}", file_path, section);
        diagrams.insert(diagram_key, diagram);
    }

    Ok(diagrams)
}

/// Generates a diagram for a single section
fn generate_section_diagram(
    registry: &ElementRegistry,
    _section: &str,
    elements: &[&Element],
    file_path: &str,
    direction: & str,
    specification_folder: &PathBuf, 
    external_folders: &[PathBuf],      
) -> Result<String, ReqFlowError> {
    // Get Git repository information for creating proper links
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

    // Get diagram direction from config (TD or LR)
    let mut diagram = String::from(format!("```mermaid\ngraph {};\n", direction));

    // Define Mermaid graph styles
    diagram.push_str("  %% Graph styling\n");
    diagram.push_str("  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;\n");
    diagram.push_str("  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;\n");
    diagram.push_str("  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;\n");
    diagram.push_str("  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;\n\n");

    let mut included_elements = HashSet::new();

    for element in elements {
        add_element_to_diagram(
            registry, 
            &mut diagram, 
            element, 
            &mut included_elements, 
            file_path,
            specification_folder,
            external_folders,
            &repo_root,
            &base_url,
            &commit_hash
        )?;
    }

    diagram.push_str("```");

    Ok(diagram)
}

/// Adds an element and its relations to the diagram
fn add_element_to_diagram(
    registry: &ElementRegistry,
    diagram: &mut String,
    element: &Element,
    included_elements: &mut HashSet<String>,
    file_path: &str,
    specification_folder: &PathBuf, 
    external_folders: &[PathBuf],
    repo_root: &PathBuf,
    base_url: &str,
    commit_hash: &str
) -> Result<(), ReqFlowError> {

    // Convert file path to its parent directory (returns PathBuf)
    let base_dir = PathBuf::from(file_path)
        .parent()
        .map(|p| p.to_path_buf()) 
        .unwrap_or_else(|| PathBuf::from("."));

    // Create a stable GitHub link for the element if we have the git info
    let has_git_info = !base_url.is_empty() && !commit_hash.is_empty() && !repo_root.as_os_str().is_empty();
    
    // Get HTML-style relative ID for local navigation
    let relative_target = utils::to_relative_identifier(
        &element.identifier.clone(),
        &base_dir,
        specification_folder,
        external_folders
    )?;
    
    // Get a GitHub link if we have git info
    let git_link = if has_git_info {
        // Get repository-relative path
        let relative_id = match utils::get_relative_path_from_root(&PathBuf::from(&element.identifier), &repo_root) {
            Ok(rel_path) => rel_path.to_string_lossy().to_string(),
            Err(_) => element.identifier.clone(),
        };
        
        // Create a git link for the element
        format!("{}/blob/{}/{}", base_url, commit_hash, relative_id)
    } else {
        // Fall back to the relative link for local navigation
        relative_target.clone()
    };
    
    let element_id = utils::hash_identifier(&element.identifier);   

    if !included_elements.contains(&element.identifier) {
       included_elements.insert(element.identifier.clone());
       
       let label = element.name.replace('"', "&quot;");
       
       let class=match &element.element_type {
           ElementType::Requirement(RequirementType::User)  => "requirement",                    
           ElementType::Requirement(RequirementType::System) =>"requirement",
           ElementType::Verification(_) =>"verification",           
           _ => "default"
       };
           
                  
       // Add the element node
       diagram.push_str(&format!("  {}[\"{}\"];\n", element_id, label));
       
       // Use GitHub link if available, otherwise fall back to local link
       let click_target = if has_git_info { git_link } else { relative_target };
       diagram.push_str(&format!("  click {} \"{}\";\n", element_id, click_target));
       
       diagram.push_str(&format!("  class {} {};\n", element_id, class));
    }



    for relation in &element.relations {
        if !relation.is_opposite {
        
        
        let label = relation.target.text.clone();
        let target_id = match &relation.target.link {
            relation::LinkType::Identifier(target) => {            
                
                let target_id = utils::hash_identifier(&target);               

                // Get HTML-style relative ID for local navigation
                let relative_target = utils::to_relative_identifier(
                    &target,
                    &base_dir,
                    specification_folder,
                    external_folders
                )?;
                
                // Get a GitHub link if we have git info
                let git_link = if has_git_info {
                    // Get repository-relative path
                    let relative_id = match utils::get_relative_path_from_root(&PathBuf::from(target), &repo_root) {
                        Ok(rel_path) => rel_path.to_string_lossy().to_string(),
                        Err(_) => target.clone(),
                    };
                    
                    // Create a git link for the target element
                    format!("{}/blob/{}/{}", base_url, commit_hash, relative_id)
                } else {
                    // Fall back to the relative link for local navigation
                    relative_target.clone()
                };
                
                if !included_elements.contains(target) {
                    included_elements.insert(target.clone());
                                 
                    let class = match registry.get_element(&target) {
                        Ok(existing_element)=>{
                            match existing_element.element_type {
                                ElementType::Requirement(RequirementType::User)  => "requirement",                    
                                ElementType::Requirement(RequirementType::System) => "requirement",
                                ElementType::Verification(_) => "verification",           
                                _ => "default"                    
                             }
                        },
                        _ => "default"
                    };
                                                               
                    diagram.push_str(&format!("  {}[\"{}\"];\n", target_id, label));
                    diagram.push_str(&format!("  class {} {};\n", target_id, class));                    
                    
                    // Use GitHub link if available, otherwise fall back to local link
                    let click_target = if has_git_info { git_link } else { relative_target };
                    diagram.push_str(&format!("  click {} \"{}\";\n", target_id, click_target));
                }
                target_id
            },
            relation::LinkType::ExternalUrl(url) => {
                // Always add external URLs, regardless of `included_elements`
                let target_id = utils::hash_identifier(url);
                diagram.push_str(&format!("  {}[\"{}\"];\n", target_id, label));
                diagram.push_str(&format!("  class {} {};\n", target_id,"default"));
                diagram.push_str(&format!("  click {} \"{}\";\n", target_id, url));
                
                target_id
                
            }
        };


        if let Some(info) = relation::RELATION_TYPES.get(relation.relation_type.name) {
            // pick from/to based on semantic direction
            let (from_id, to_id) = match info.direction {
                relation::RelationDirection::Forward => (target_id.clone(), element_id.clone()),
                _                           => (element_id.clone(), target_id.clone()),
            };

            diagram.push_str(&format!(
                "  {} {}|{}| {};\n",
                from_id,
                info.arrow,
                info.label,
                to_id,
            ));
        } else {
            // fallback: unknown relation
            diagram.push_str(&format!(
                "  {} -->|relates to| {};\n",
                element_id, target_id,
            ));
        }
        }
    }

    Ok(())
}


