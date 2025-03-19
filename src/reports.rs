use crate::relation::LinkType;
use std::collections::HashMap;
use crate::element::{Element, ElementType, RequirementType};
use crate::element_registry::ElementRegistry;
use serde_json::json;
use serde_json::Value;


pub fn print_registry_summary(element_registry: &ElementRegistry, json:bool) {
    if json{
        print_registry_summary_json(element_registry);
    }else{
        print_registry_summary_text(element_registry);    
    }

}
/// Prints all elements in the registry grouped by file and then by section,
/// and displays relation details including the relation type name and target.
fn print_registry_summary_text(element_registry: &ElementRegistry) {
    println!("--- MBSE Model summary ---");

    // Group elements by file_path and then by section.
    let mut grouped_elements: HashMap<String, HashMap<String, Vec<&Element>>> = HashMap::new();

    for element in element_registry.elements.values() {
        grouped_elements
            .entry(element.file_path.clone())
            .or_insert_with(HashMap::new)
            .entry(element.section.clone())
            .or_insert_with(Vec::new)
            .push(element);
    }

    // Global counters.
    let mut total_elements = 0;
    let mut total_system_requirements = 0;
    let mut total_not_verified = 0;
    let mut total_not_satisfied = 0;

    // Print elements in a structured format.
    for (file_path, sections) in grouped_elements {
        println!("📂 File: {}", file_path);

        for (section, elements) in sections {
            println!("  📖 Section: {}", section);

            for element in elements {
                total_elements += 1;
                println!("    🔹 Element: {}", element.identifier);
                println!("      - Name: {}", element.name);
                println!("      - File: {}", element.file_path);
                println!("      - Type: {:?}", element.element_type);
                println!("      - Content: {:?}", element.content);                

                // If the element is a System Requirement, count its verification/satisfaction.
                if let ElementType::Requirement(ref req_type) = element.element_type {
                    if let RequirementType::System = req_type {
                        total_system_requirements += 1;
                        let verified_count = element.relations.iter()
                            .filter(|r| r.relation_type.name.eq_ignore_ascii_case("verifiedBy") ||
                                        r.relation_type.name.eq_ignore_ascii_case("verify"))
                            .count();
                        let satisfied_count = element.relations.iter()
                            .filter(|r| r.relation_type.name.eq_ignore_ascii_case("satisfiedBy") ||
                                        r.relation_type.name.eq_ignore_ascii_case("satisfy"))
                            .count();
                        println!("      - Verified relations count: {}", verified_count);
                        println!("      - Satisfied relations count: {}", satisfied_count);
                        if verified_count == 0 {
                            total_not_verified += 1;
                        }
                        if satisfied_count == 0 {
                            total_not_satisfied += 1;
                        }
                    }
                }
                
                if element.relations.is_empty() {
                    println!("      - No relations.");
                } else {
                    println!("      - Relations:");
                    for relation in &element.relations {
                        let rel_type = relation.relation_type.name;
                        match &relation.target.link {
                            LinkType::Identifier(target_id) => {
                                println!("        ↪ {}: {} (Identifier)", rel_type, target_id);
                            }
                            LinkType::ExternalUrl(url) => {
                                println!("        ↪ {}: {} (External URL)", rel_type, url);
                            }
                        }
                    }
                }
                println!(); // Add spacing for readability.
            }
        }
    }

    println!("------------------------------------");
    println!("Total elements: {}", total_elements);
    println!("Total System Requirements: {}", total_system_requirements);
    println!("System Requirements not verified: {}", total_not_verified);
    println!("System Requirements not satisfied: {}", total_not_satisfied);
}


/// Prints a JSON summary of all elements in the registry grouped by file and then by section,
/// including relation details with relation type names and targets.
pub fn print_registry_summary_json(element_registry: &ElementRegistry) {
    // Group elements by file_path and then by section.
    let mut grouped_elements: HashMap<String, HashMap<String, Vec<&Element>>> = HashMap::new();

    for element in element_registry.elements.values() {
        grouped_elements
            .entry(element.file_path.clone())
            .or_insert_with(HashMap::new)
            .entry(element.section.clone())
            .or_insert_with(Vec::new)
            .push(element);
    }

    // Global counters.
    let mut total_elements = 0;
    let mut total_system_requirements = 0;
    let mut total_not_verified = 0;
    let mut total_not_satisfied = 0;

    // Build a JSON map for files.
    let mut files_json = serde_json::Map::new();

    for (file_path, sections) in grouped_elements {
        let mut sections_json = serde_json::Map::new();

        for (section, elements) in sections {
            let mut elements_json = Vec::new();

            for element in elements {
                total_elements += 1;

                // Counters for System Requirements.
                let mut verified_count = 0;
                let mut satisfied_count = 0;
                if let ElementType::Requirement(ref req_type) = element.element_type {
                    if let RequirementType::System = req_type {
                        total_system_requirements += 1;
                        verified_count = element
                            .relations
                            .iter()
                            .filter(|r| {
                                r.relation_type.name.eq_ignore_ascii_case("verifiedBy")
                                    || r.relation_type.name.eq_ignore_ascii_case("verify")
                            })
                            .count();
                        satisfied_count = element
                            .relations
                            .iter()
                            .filter(|r| {
                                r.relation_type.name.eq_ignore_ascii_case("satisfiedBy")
                                    || r.relation_type.name.eq_ignore_ascii_case("satisfy")
                            })
                            .count();

                        if verified_count == 0 {
                            total_not_verified += 1;
                        }
                        if satisfied_count == 0 {
                            total_not_satisfied += 1;
                        }
                    }
                }

                // Build relations JSON.
                let relations_json: Vec<Value> = if element.relations.is_empty() {
                    vec![]
                } else {
                    element
                        .relations
                        .iter()
                        .map(|relation| {
                            let rel_type = relation.relation_type.name.clone();
                            let target = match &relation.target.link {
                                LinkType::Identifier(target_id) => json!({
                                    "target": target_id,
                                    "type": "Identifier"
                                }),
                                LinkType::ExternalUrl(url) => json!({
                                    "target": url,
                                    "type": "External URL"
                                }),
                            };
                            json!({
                                "relation_type": rel_type,
                                "target": target,
                            })
                        })
                        .collect()
                };

                // Build element JSON object.
                let element_json = json!({
                    "identifier": element.identifier,
                    "name": element.name,
                    "file": element.file_path,
                    "type": format!("{:?}", element.element_type),
                    "content": element.content,
                    "verified_relations_count": verified_count,
                    "satisfied_relations_count": satisfied_count,
                    "relations": relations_json,
                });

                elements_json.push(element_json);
            }

            sections_json.insert(section, json!(elements_json));
        }

        files_json.insert(file_path, json!(sections_json));
    }

    let summary = json!({
        "model_summary": {
            "files": files_json,
            "global_counters": {
                "total_elements": total_elements,
                "total_system_requirements": total_system_requirements,
                "system_requirements_not_verified": total_not_verified,
                "system_requirements_not_satisfied": total_not_satisfied,
            }
        }
    });

    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
}


