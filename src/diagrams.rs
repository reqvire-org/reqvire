use std::collections::{HashMap, HashSet};
use crate::element::Element;
use crate::element_registry::ElementRegistry;
use crate::error::ReqFlowError;
use std::path::PathBuf;
use crate::utils;
use log::debug;
use crate::relation::LinkType;
use rustc_hash::FxHasher;
use std::hash::{Hasher};
use crate::element::ElementType;
use crate::element::RequirementType;

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
        add_element_to_diagram(registry, &mut diagram, element, &mut included_elements, file_path,specification_folder,external_folders)?;
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
) -> Result<(), ReqFlowError> {

    // Convert file path to its parent directory (returns PathBuf)
    let base_dir = PathBuf::from(file_path)
        .parent()
        .map(|p| p.to_path_buf()) 
        .unwrap_or_else(|| PathBuf::from("."));

    let relative_target=utils::to_relative_identifier(
        &element.identifier.clone(),
        &base_dir,
        specification_folder,
        external_folders
    )?;
    
    let element_id = utils::hash_identifier(&element.identifier);   

    if !included_elements.contains(&element.identifier) {
       included_elements.insert(element.identifier.clone());
       
       let label = element.name.replace('"', "&quot;");
       
       let class=match &element.element_type {
           ElementType::Requirement(RequirementType::User)  => "requirement",                    
           ElementType::Requirement(RequirementType::System) =>"requirement",
           ElementType::Verification =>"verification",           
           _ => "default"
       };
           
                  
       diagram.push_str(&format!("  {}[\"{}\"];\n", element_id, label));
       diagram.push_str(&format!("  click {} \"{}\";\n", element_id, relative_target));
       
       diagram.push_str(&format!("  class {} {};\n", element_id,class));
    }



    for relation in &element.relations {
        let label = relation.target.text.clone();
        let target_id = match &relation.target.link {
            LinkType::Identifier(target) => {            
                
                let target_id = utils::hash_identifier(&target);               

                let relative_target = utils::to_relative_identifier(
                    &target,
                    &base_dir,
                    specification_folder,
                    external_folders
                )?;           
            
                if !included_elements.contains(target) {
                    included_elements.insert(target.clone());
                                 
  
                    let class=match registry.get_element(&target) {
                        Ok(existing_element)=>{
                            match existing_element.element_type {
                                ElementType::Requirement(RequirementType::User)  => "requirement",                    
                                ElementType::Requirement(RequirementType::System) =>"requirement",
                                ElementType::Verification =>"verification",           
                                _ => "default"                    
                             }
                        },
                        _ => "default"
                    
                    };
                                                               
                    diagram.push_str(&format!("  {}[\"{}\"];\n", target_id, label));
                    diagram.push_str(&format!("  class {} {};\n", target_id,class));                    
                    diagram.push_str(&format!("  click {} \"{}\";\n", target_id, relative_target));
                }
                target_id
            },
            LinkType::ExternalUrl(url) => {
                // Always add external URLs, regardless of `included_elements`
                let target_id = utils::hash_identifier(url);
                diagram.push_str(&format!("  {}[\"{}\"];\n", target_id, label));
                diagram.push_str(&format!("  class {} {};\n", target_id,"default"));
                diagram.push_str(&format!("  click {} \"{}\";\n", target_id, url));
                
                target_id
                
            }
        };


        let properties = get_relation_properties(relation.relation_type.name);
        let (from_id, to_id) = if properties.reverse_direction {
            (target_id.clone(), element_id.clone())
        } else {
            (element_id.clone(), target_id.clone())
        };

        diagram.push_str(&format!("  {} {}|{}| {};\n", from_id, properties.arrow, properties.label, to_id));
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


/// Struct for relation properties
struct RelationProperties {
    arrow: &'static str,
    label: &'static str,
    reverse_direction: bool,
}



