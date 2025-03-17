use crate::relation::LinkType;
use std::collections::HashMap;
use crate::element::Element;
use crate::element_registry::ElementRegistry;

/// Prints all elements in the registry grouped by file and then by section,
/// and then prints summary counts of elements and relations.
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

    // Print elements in a structured format.
    for (file_path, sections) in &grouped_elements {
        println!("📂 File: {}", file_path);
        for (section, elements) in sections {
            println!("  📖 Section: {}", section);
            for element in elements {
                println!("    🔹 Element: {}", element.identifier);
                println!("      - Name: {}", element.name);
                println!("      - File: {}", element.file_path);
                println!("      - Type: {:?}", element.element_type);

                if element.relations.is_empty() {
                    println!("      - No relations.");
                } else {
                    println!("      - Relations:");
                    for relation in &element.relations {
                        match &relation.target.link {
                            LinkType::Identifier(target_id) => {
                                println!("        ↪ {} (Identifier)", target_id);
                            }
                            LinkType::ExternalUrl(url) => {
                                println!("        🔗 {} (External URL)", url);
                            }
                        }
                    }
                }
                println!(); // Extra spacing for readability.
            }
        }
    }

    // Compute summary counts.
    let total_elements = element_registry.elements.len();
    let mut total_relations = 0;
    let mut element_type_counts: HashMap<String, usize> = HashMap::new();
    let mut relation_type_counts: HashMap<String, usize> = HashMap::new();

    for element in element_registry.elements.values() {
        total_relations += element.relations.len();
        let etype = format!("{:?}", element.element_type);
        *element_type_counts.entry(etype).or_insert(0) += 1;

        for relation in &element.relations {
            let rtype = match &relation.target.link {
                LinkType::Identifier(_) => "Identifier".to_string(),
                LinkType::ExternalUrl(_) => "ExternalUrl".to_string(),
            };
            *relation_type_counts.entry(rtype).or_insert(0) += 1;
        }
    }

    println!("------------------------------------");
    println!("Summary:");
    println!("Total Elements: {}", total_elements);
    println!("Total Relations: {}", total_relations);
    println!("Elements by Type:");
    for (etype, count) in &element_type_counts {
        println!("  {}: {}", etype, count);
    }
    println!("Relations by Type:");
    for (rtype, count) in &relation_type_counts {
        println!("  {}: {}", rtype, count);
    }
}

