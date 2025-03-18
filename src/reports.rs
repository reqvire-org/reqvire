use crate::relation::LinkType;
use std::collections::HashMap;
use crate::element::{Element, ElementType, RequirementType};
use crate::element_registry::ElementRegistry;

/// Prints all elements in the registry grouped by file and then by section,
/// and displays relation details including the relation type name and target.
pub fn print_registry_summary(element_registry: &ElementRegistry) {
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

