use std::collections::HashMap;
use crate::element::Element;
use crate::error::ReqFlowError;
use crate::relation::LinkType;


#[derive(Debug)]
pub struct ElementRegistry {
    /// Map of full identifiers to elements
    elements: HashMap<String, Element>
}

impl ElementRegistry {
    /// Creates a new registry with a given configuration
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    /// Registers an element, ensuring identifier uniqueness
    pub fn register_element(&mut self, element: Element, _file_path: &str) -> Result<(), ReqFlowError> {
   
        if self.elements.contains_key(&element.identifier) {
            return Err(ReqFlowError::DuplicateElement(element.identifier));
        }

        let identifier=element.identifier.clone();
        self.elements.insert(identifier, element);
        Ok(())
    }

    /// Retrieves an element by its identifier
    pub fn get_element(&self, identifier: &str) -> Result<&Element, ReqFlowError> {
        self.elements
            .get(identifier)
            .ok_or_else(|| ReqFlowError::MissingElement(identifier.to_string()))
    }

    /// Retrieves all elements
    pub fn get_all_elements(&self) -> Vec<&Element> {
        self.elements.values().collect()
    }

    /// Checks if an element exists in the registry
    pub fn contains_element(&self, identifier: &str) -> bool {
        self.elements.contains_key(identifier)
    }

    /// Registers multiple elements at once
    pub fn register_elements(&mut self, elements: Vec<Element>, file_path: &str) -> Vec<ReqFlowError> {
        let mut errors = vec![];

        for element in elements {
            if let Err(e) = self.register_element(element, file_path) {
                errors.push(e);
            }
        }

        errors
    }
    /// Prints all elements in the registry grouped by file and then by section
    pub fn print_registry(&self) {
        println!("--- Element Registry Debug Print ---");

        // Group elements by file_path and then by section
        let mut grouped_elements: HashMap<String, HashMap<String, Vec<&Element>>> = HashMap::new();

        for element in self.elements.values() {
            grouped_elements
                .entry(element.file_path.clone()) // Group by file path
                .or_insert_with(HashMap::new)
                .entry(element.section.clone()) // Group by section inside file
                .or_insert_with(Vec::new)
                .push(element);
        }

        // Print elements in a structured format
        for (file_path, sections) in grouped_elements {
            println!("📂 File: {}", file_path);

            for (section, elements) in sections {
                println!("  📖 Section: {}", section);

                for element in elements {
                    println!("    🔹 Element: {}", element.identifier);
                    println!("      - Name: {}", element.name);
                    println!("      - File: {}", element.file_path); // Include file of the element
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
                    println!(); // Add spacing for readability
                }
            }
        }

        println!("------------------------------------");
    }
}

