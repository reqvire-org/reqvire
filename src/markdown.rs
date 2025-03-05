use anyhow::Result;
use log::debug;
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
    
    // Track if we're in a metadata section
    let mut in_metadata_section = false;
    
    for line in content.lines() {
        if line.trim() == "### Relations" {
            // Special case: "### Relations" could be mistaken for an element but should be treated as a Relations subsection
            if current_element.is_some() {
                in_relations_section = true;
                in_metadata_section = false;
                
                // Add the relations header to the element content
                if let Some(element) = &mut current_element {
                    element.add_content(&format!("{}\n", line));
                }
            } else {
                // We found a Relations header outside of an element - log it but don't create an element for it
                debug!("Found '### Relations' outside of an element in {}", file_path);
            }
        } else if line.trim() == "#### Metadata" {
            // Special case: Metadata subsection
            if current_element.is_some() {
                in_metadata_section = true;
                in_relations_section = false;
                
                // Add the metadata header to the element content
                if let Some(element) = &mut current_element {
                    element.add_content(&format!("{}\n", line));
                }
            } else {
                // We found a Metadata subsection outside of an element
                debug!("Found 'Metadata' subsection outside of an element in {}", file_path);
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
                    // Set flags based on subsection type
                    if subsection_name == "Relations" {
                        in_relations_section = true;
                        in_metadata_section = false;
                    } else if subsection_name == "Metadata" {
                        in_metadata_section = true;
                        in_relations_section = false;
                    } else {
                        in_relations_section = false;
                        in_metadata_section = false;
                    }
                    
                    // Add the subsection header to the element content
                    if let Some(element) = &mut current_element {
                        element.add_content(&format!("{}\n", line));
                    }
                } else {
                    // We found a subsection header outside of an element - this is unusual
                    // but we shouldn't create an element for it
                    debug!("Found {} subsection header outside of an element in {}", subsection_name, file_path);
                }
            } else {
                // Not a reserved subsection, treat as normal content
                if let Some(element) = &mut current_element {
                    element.add_content(&format!("{}\n", line));
                }
            }
        } else if in_metadata_section && (line.trim().starts_with("* ") || line.trim().starts_with("- ")) {
            // Parse metadata property
            let metadata_regex = Regex::new(r"[\*\-]\s+(\w+):\s*(.+)").unwrap();
            
            if let Some(captures) = metadata_regex.captures(line) {
                let property_name = captures.get(1).unwrap().as_str().to_string();
                let property_value = captures.get(2).unwrap().as_str().trim().to_string();
                
                debug!("Found metadata property: {} = {} in element '{}'", 
                      property_name, property_value,
                      current_element.as_ref().map_or("Unknown", |e| &e.name));
                
                // Add metadata to the element
                if let Some(element) = &mut current_element {
                    element.metadata.insert(property_name, property_value);
                    element.add_content(&format!("{}\n", line));
                    
                    // If we've added a type, update the element type
                    element.set_type_from_metadata();
                }
            } else {
                // If it doesn't match the metadata pattern, still add it as content
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
                    debug!("Trying flexible relation parsing for line: {}", line);
                    alt_regex2.captures(line)
                })
            } else {
                captures
            };
            
            if let Some(captures) = captures {
                let relation_type = captures.get(1).unwrap().as_str().to_string();
                let target = captures.get(2).unwrap().as_str().trim().to_string();
                
                // More detailed logging to help diagnose issues
                debug!("Found relation: {} -> {} in element '{}'", relation_type, target, 
                      current_element.as_ref().map_or("Unknown", |e| &e.name));
                let relation = Relation::new(relation_type, target);
                
                if let Some(element) = &mut current_element {
                    // Check for duplicates in the current element's relations
                    let is_duplicate = element.relations.iter().any(|r| 
                        r.relation_type == relation.relation_type && r.target == relation.target
                    );
                    
                    if is_duplicate {
                        debug!("Found duplicate relation: {} -> {} in element '{}'", 
                              relation.relation_type, relation.target, element.name);
                    }
                    
                    // Add it regardless for later validation
                    element.add_relation(relation);
                    element.add_content(&format!("{}\n", line));
                    // Log total relations after adding
                    debug!("Element '{}' now has {} relations", element.name, element.relations.len());
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
                in_metadata_section = false;
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
    
    // Log all elements before adding to registry
    log::debug!("Collecting elements from file: {}", file_path);
    for element in &elements {
        log::debug!("  Element: {} from file {}", element.name, file_path);
        // Log custom IDs if available
        if let Some(id_value) = element.metadata.get("id") {
            log::debug!("    Custom ID: {}", id_value);
        }
    }
    
    for element in elements {
        registry.add_element(element)?;
    }
    
    Ok(())
}

/// Replace relations in markdown with proper links
pub fn replace_relations(
    content: &str,
    registry: &ElementRegistry,
    current_file: &Path,
    convert_to_html: bool,
) -> Result<String, ReqFlowError> {
    use crate::relation::process_relations;
    use crate::config::Config;
    
    // For diagram generation, we need to check if we're in the diagram generation context
    // This is set by main.rs when using the --generate-diagrams flag
    // We'll check this instead of reloading the config
    
    // When we're in diagram generation mode but not HTML conversion mode,
    // we don't want to replace markdown links with HTML links
    let diagrams_mode = registry.is_diagram_generation_enabled();
    let should_convert_links = if diagrams_mode && !convert_to_html {
        // In diagram-only mode, keep markdown links as markdown
        false
    } else {
        // Otherwise respect the convert_to_html parameter
        convert_to_html
    };
    
    // Process the content to replace relations with markdown links
    let processed_content = process_relations(content, current_file, should_convert_links)?;
    
    // Generate diagrams if in diagrams mode
    if diagrams_mode {
        // Always attempt to generate a diagram for requirements files
        log::debug!("Attempting to generate diagram for file: {}", current_file.display());
        
        // Generate and add mermaid diagram regardless of file type
        let updated_content = generate_requirements_diagram(&processed_content, registry, &current_file.to_string_lossy(), convert_to_html)?;
        log::debug!("Diagram generation completed for file: {}", current_file.display());
        Ok(updated_content)
    } else {
        // Skip diagram generation if not explicitly enabled
        log::debug!("Diagram generation skipped (not enabled)");
        Ok(processed_content)
    }
}

/// Generate mermaid diagrams for requirements documents
/// This function creates a separate diagram for each section (level 2 heading)
pub fn generate_requirements_diagram(
    content: &str,
    registry: &ElementRegistry,
    file_path: &str,
    convert_to_html: bool,
) -> Result<String, ReqFlowError> {
    use regex::Regex;
    use std::collections::{HashMap, HashSet};
    
    // Check if this is a requirements document by file path or content
    let req_title_regex = match Regex::new(r"(?m)^#\s+.*Requirements") {
        Ok(regex) => regex,
        Err(e) => return Err(ReqFlowError::InvalidRegex(format!("Error compiling regex: {}", e)))
    };
    
    // Get the config to properly use configuration-driven detection
    let config = crate::config::Config::load();
    
    // Use the built-in utils function to check if this is a requirements file
    // This will use the proper config-driven approach, not hardcoded values
    let path_obj = std::path::Path::new(file_path);
    let base_path = std::path::Path::new("./");
    let by_path = crate::utils::is_requirements_file_by_path(path_obj, &config, base_path);
    let by_title = req_title_regex.is_match(content);
    let is_requirements_doc = by_path || by_title;
    
    // Debug information - print to console to make sure it appears with additional path details
    let path_filename = path_obj.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default();
    println!("FILE PATH: {}", file_path);
    println!("FILENAME: {}", path_filename);
    println!("Checking requirements file by path...");
    println!("IS REQUIREMENTS DOC: {}", is_requirements_doc);
    println!("BY PATH: {}", by_path);
    println!("BY TITLE: {}", by_title);
    
    // Use log module for more controlled output
    log::debug!("Checking if file is a requirements document: {}", file_path);
    log::debug!("Requirements check using config-driven detection: {}", is_requirements_doc);
    log::debug!("Title regex match: {}", req_title_regex.is_match(content));
    
    // Force diagram generation for files that contain at least one element with relations
    // This respects the original detection but generates diagrams for any file with relations
    let has_elements_with_relations = content.contains("### ") && content.contains("#### Relations");
    
    if !is_requirements_doc && !has_elements_with_relations {
        log::debug!("Skipping diagram generation - file is not a requirements document and has no elements with relations: {}", file_path);
        return Ok(content.to_string());
    }
    
    if has_elements_with_relations && !is_requirements_doc {
        log::debug!("File contains elements with relations, proceeding with diagram generation despite not being a requirements document: {}", file_path);
    }
    
    log::debug!("Generating diagrams for requirements file: {}", file_path);
    
    // Define regex for paragraph headers
    let para_regex = match Regex::new(r"(?m)^##\s+(.+)$") {
        Ok(regex) => regex,
        Err(e) => return Err(ReqFlowError::InvalidRegex(format!("Error compiling regex: {}", e)))
    };
    
    // Find all elements in this file
    let elements_in_file: Vec<&Element> = registry.all_elements()
        .filter(|e| e.file_path == file_path)
        .collect();
    
    // Split the content into lines
    let lines: Vec<&str> = content.lines().collect();
    let mut result_lines = Vec::new();
    
    // First, remove any existing ReqFlow-generated diagrams and cleanup markers
    let mut i = 0;
    let mut in_diagram = false;
    let mut skipping_diagram = false;
    
    while i < lines.len() {
        let line = lines[i];
        
        // Skip any standalone ReqFlow marker comments
        if line.trim().contains("<!-- ReqFlow-generated diagram") {
            i += 1;
            continue;
        }
        
        // Check for start of mermaid diagram
        if line.trim() == "```mermaid" {
            // Look ahead to see if this is a ReqFlow-generated diagram
            let mut j = i + 1;
            
            while j < lines.len() && !lines[j].trim().ends_with("```") {
                j += 1;
            }
            
            // Look for the typical ReqFlow-generated diagram pattern
            // ReqFlow diagrams always contain specific class definitions
            let mut contains_req_styling = false;
            for k in i + 1..j {
                if lines[k].contains("classDef requirement") && 
                   lines[k].contains("fill:#f9d6d6") {
                    contains_req_styling = true;
                    break;
                }
            }
            
            if contains_req_styling {
                // This is likely a ReqFlow-generated diagram, skip it
                skipping_diagram = true;
                in_diagram = true;
                i += 1;
                continue;
            }
        }
        
        // If we're skipping a diagram, check for end of diagram
        if in_diagram {
            if line.trim().ends_with("```") {
                in_diagram = false;
                i += 1;
                
                // Skip any marker comments that might follow the diagram
                while i < lines.len() && 
                      (lines[i].trim().is_empty() || 
                       lines[i].trim().contains("<!-- ReqFlow-generated diagram")) {
                    i += 1;
                }
                continue;
            }
            
            if skipping_diagram {
                i += 1;
                continue;
            }
        }
        
        // Add this line to result
        result_lines.push(line.to_string());
        i += 1;
    }
    
    // Clean up any excess newlines
    let mut final_content = result_lines.join("\n");
    let multiline_regex = match Regex::new(r"\n\n\n+") {
        Ok(regex) => regex,
        Err(e) => return Err(ReqFlowError::InvalidRegex(format!("Error compiling regex: {}", e)))
    };
    
    final_content = multiline_regex.replace_all(&final_content, "\n\n").to_string();
    
    // Parse the document to identify paragraphs and elements within each paragraph
    // We'll generate a separate diagram for each paragraph
    let lines: Vec<&str> = final_content.lines().collect();
    let mut paragraphs: Vec<(usize, String)> = Vec::new(); // (line index, paragraph name)
    let mut elements_by_paragraph: HashMap<String, Vec<&Element>> = HashMap::new();
    let mut current_paragraph: Option<String> = None;
    
    // First, find all paragraph headings and their line positions
    for (i, line) in lines.iter().enumerate() {
        if let Some(captures) = para_regex.captures(line) {
            if let Some(para_name) = captures.get(1) {
                let paragraph_name = para_name.as_str().to_string();
                paragraphs.push((i, paragraph_name.clone()));
                current_paragraph = Some(paragraph_name);
            }
        }
    }
    
    // Add a special "Introduction" section for any elements before the first paragraph
    if !paragraphs.is_empty() {
        paragraphs.insert(0, (0, "Document Overview".to_string()));
    } else {
        // If no paragraphs, use a single "Document" section
        paragraphs.push((0, "Document".to_string()));
    }
    
    // Associate elements with paragraphs
    current_paragraph = Some(paragraphs[0].1.clone());
    let mut current_para_idx = 0;
    
    for (i, line) in lines.iter().enumerate() {
        // Check if we've reached a new paragraph
        if current_para_idx < paragraphs.len() - 1 && i >= paragraphs[current_para_idx + 1].0 {
            current_para_idx += 1;
            current_paragraph = Some(paragraphs[current_para_idx].1.clone());
        }
        
        // Check if line contains an element heading
        if let Some(element_name) = crate::config::Config::element_regex()
                                     .captures(line)
                                     .and_then(|caps| caps.get(1))
                                     .map(|m| m.as_str().trim().to_string()) {
            // Found an element, associate it with the current paragraph
            if let Some(paragraph) = &current_paragraph {
                if let Some(element) = elements_in_file.iter().find(|e| e.name == element_name) {
                    elements_by_paragraph.entry(paragraph.clone())
                        .or_insert_with(Vec::new)
                        .push(element);
                }
            }
        }
    }
    
    // Now create a new version of the document with diagrams inserted after each paragraph heading
    let mut result_lines = Vec::new();
    let mut in_first_intro_section = true;
    
    for (i, line) in lines.iter().enumerate() {
        let is_paragraph_heading = para_regex.is_match(line);
        
        // Add the line
        result_lines.push(line.to_string());
        
        // If this is a paragraph heading, insert a diagram after it
        if is_paragraph_heading || in_first_intro_section {
            in_first_intro_section = false;
            
            // Figure out which paragraph this is
            let para_name = if is_paragraph_heading {
                para_regex.captures(line)
                    .and_then(|caps| caps.get(1))
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_else(|| "Unknown".to_string())
            } else {
                paragraphs[0].1.clone() // Introduction section
            };
            
            // Only generate a diagram if the paragraph has elements
            if let Some(elements) = elements_by_paragraph.get(&para_name) {
                if !elements.is_empty() {
                    // Generate a diagram for this paragraph
                    let diagram = generate_diagram_for_paragraph(
                        &para_name,
                        elements,
                        registry,
                        file_path,
                        convert_to_html
                    )?;
                    
                    // Add an empty line and the diagram
                    result_lines.push("".to_string());
                    result_lines.push(diagram);
                    result_lines.push("".to_string());
                }
            }
        }
    }
    
    let result = result_lines.join("\n");
    
    log::debug!("Diagram generation complete for file: {}", file_path);
    
    Ok(result)
}

/// Generate a mermaid diagram for a specific paragraph
fn generate_diagram_for_paragraph(
    paragraph_name: &str,
    elements: &[&Element],
    registry: &ElementRegistry,
    file_path: &str,
    convert_to_html: bool
) -> Result<String, ReqFlowError> {
    use std::collections::HashSet;
    
    // Get diagram direction from config (TD or LR)
    let direction = &registry.config().style.diagram_direction;
    let mut diagram = String::from(format!("```mermaid\ngraph {};\n", direction));
    
    // Add graph styling
    diagram.push_str("  %% Graph styling\n");
    diagram.push_str("  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;\n");
    diagram.push_str("  classDef satisfies fill:#fff2cc,stroke:#ffcc00,stroke-width:1px;\n");
    diagram.push_str("  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;\n");
    diagram.push_str("  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;\n");
    diagram.push_str("  classDef paragraph fill:#efefef,stroke:#999999,stroke-width:1px;\n");
    diagram.push_str("  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;\n\n");
    
    // If there are no elements, add a placeholder node
    if elements.is_empty() {
        diagram.push_str(&format!("  placeholder[\"No elements in '{}' section\"];\n", paragraph_name));
        diagram.push_str("  class placeholder default;\n");
    } else {
        // Create a set to track included elements
        let mut included_elements = HashSet::new();
        

        
        // Add each element in this paragraph
        for element in elements {
            add_element_to_diagram(
                &mut diagram,
                element,
                &mut included_elements,
                registry,
                file_path,
                convert_to_html,
                Some("para".to_string())
            )?;
        }
        

    }
    
    // Close the diagram
    diagram.push_str("```");
    
    Ok(diagram)
}

/// Add an element and its relationships to the diagram
fn add_element_to_diagram(
    diagram: &mut String,
    element: &Element,
    included_elements: &mut std::collections::HashSet<String>,
    registry: &ElementRegistry,
    file_path: &str,
    convert_to_html: bool,
    subgraph_id: Option<String>,
) -> Result<(), ReqFlowError> {
    // Safe ID for element - sanitize by replacing all non-alphanumeric chars with underscore
    let element_id = element.name.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>();
    
    // Encode the label text properly for Mermaid
    let label = element.name.replace('"', "&quot;");
    
    // Indentation for subgraph elements
    let indent = if subgraph_id.is_some() { "    " } else { "  " };
    
    diagram.push_str(&format!("{}{}[\"{}\"];\n", indent, element_id, label));
    
    // Add click behavior for HTML - only convert to HTML if we're in HTML mode
    let html_file = if convert_to_html {
        // In HTML mode, convert .md to .html
        convert_path_to_html_link(&element.file_path)
    } else {
        // In markdown mode, keep as .md
        element.file_path.clone()
    };
    
    diagram.push_str(&format!("{}click {} \"{}#{}\";\n", 
        indent, 
        element_id, 
        html_file,
        element.name.replace(' ', "-").to_lowercase()
    ));
    
    // Apply requirement style to this element - all direct elements in document are requirements
    diagram.push_str(&format!("{}class {} requirement;\n", indent, element_id));
    
    // Mark this element as included
    included_elements.insert(element.name.clone());
    
    // Process relations for this element
    for relation in &element.relations {
        // Get target element
        let target = &relation.target;
        
        // Generate safe ID for target - sanitize by replacing all non-alphanumeric chars with underscore
        let target_id = target.chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect::<String>();
        
        // Define a small struct to represent the relationship properties
        struct RelationProperties {
            arrow: &'static str,      // Arrow style
            label: &'static str,      // Displayed label
            reverse_direction: bool,  // Whether to reverse the arrow direction
        }
        
        // Get relation properties based on relation type
        let properties = match relation.relation_type.as_str() {
            // Relations originating from requirements ("By" suffix) - reverse direction
            "verifiedBy" => RelationProperties { arrow: "-->", label: "verifies", reverse_direction: true },
            "satisfiedBy" => RelationProperties { arrow: "-->", label: "satisfies", reverse_direction: true },
            // For derivedFrom, we DON'T reverse the direction to get the opposite effect
            "derivedFrom" => RelationProperties { arrow: "-.->", label: "deriveReqT", reverse_direction: false },  
            "tracedFrom" => RelationProperties { arrow: "-->", label: "traces", reverse_direction: true },  // Changed to "traces"
            "containedBy" => RelationProperties { arrow: "--o", label: "contains", reverse_direction: true },
            
            // Relations originating from other elements - normal direction
            "verify" => RelationProperties { arrow: "-->", label: "verifies", reverse_direction: false },
            "satisfy" => RelationProperties { arrow: "-->", label: "satisfies", reverse_direction: false },
            // For derive, we reverse the direction to get the opposite effect
            "derive" => RelationProperties { arrow: "-.->", label: "deriveReqT", reverse_direction: true },
            "refine" => RelationProperties { arrow: "==>", label: "refines", reverse_direction: false },
            "trace" => RelationProperties { arrow: "-->", label: "traces", reverse_direction: false },  // Changed to "traces"
            "contain" => RelationProperties { arrow: "--o", label: "contains", reverse_direction: false },
            
            // Handle any relation type that ends with "By" - reverse direction
            rel_type if rel_type.ends_with("By") => {
                // Extract the base relation name by removing the "By" suffix
                let base_len = rel_type.len() - 2;  // Length without "By"
                let base_name = &rel_type[0..base_len];
                
                // Determine the arrow style and label based on the base relation
                let (arrow, label, should_reverse) = if base_name.contains("derive") {
                    // For derive relations, DON'T reverse the direction
                    ("-.->", "deriveReqT", false)  // Special label for derive relations
                } else if base_name.contains("refine") {
                    ("==>", "refines", true)
                } else if base_name.contains("contain") {
                    ("--o", "contains", true) 
                } else if base_name.contains("trace") {
                    ("-->", "traces", true)
                } else if base_name.contains("verify") {
                    ("-->", "verifies", true)
                } else if base_name.contains("satisfy") {
                    ("-->", "satisfies", true)
                } else {
                    ("-->", "relates to", true)
                };
                
                RelationProperties { 
                    arrow,
                    label,
                    reverse_direction: should_reverse  // Not all "By" relations should reverse direction
                }
            },
            
            // Default for any other relation type
            _ => RelationProperties { arrow: "-->", label: "relates to", reverse_direction: false },
        };
        
        // Determine the source and target IDs based on direction
        // Clone the strings to avoid ownership issues
        let (from_id, to_id) = if properties.reverse_direction {
            (target_id.clone(), element_id.clone())  // Reversed: from target to element
        } else {
            (element_id.clone(), target_id.clone())  // Normal: from element to target
        };
        
        // Add the relationship with the correct directionality, arrow style, and label
        diagram.push_str(&format!("{}{} {}|{}| {};\n", 
            indent,
            from_id,
            properties.arrow, 
            properties.label,
            to_id
        ));
        
        // If the target is not already included in the diagram, add it
        if !included_elements.contains(target) {
            // Add the target node without markdown link syntax to avoid rendering issues
            let display_text = if target.contains('[') && target.contains(']') && target.contains('(') && target.contains(')') {
                // This is a markdown link [text](url) format, extract just the text part
                let link_text_start = target.find('[').unwrap() + 1;
                let link_text_end = target.find(']').unwrap();
                target[link_text_start..link_text_end].replace('"', "&quot;")
            } else if target.contains('#') {
                // For anchor links like Element 1#element-1, extract the element name
                let parts: Vec<&str> = target.split('#').collect();
                parts[0].trim().replace('"', "&quot;")
            } else {
                target.replace('"', "&quot;")
            };
            diagram.push_str(&format!("{}{}[\"{}\"];\n", indent, target_id, display_text));
            
            // Try to find the target element in the registry
            let target_element = registry.get_element(target);
            
            // We previously checked if the target was external, but now we color strictly by relation type
            // We'll keep the check for debugging purposes
            let _is_external = if let Some(target_elem) = target_element {
                // It's external if it's from a different file
                target_elem.file_path != file_path
            } else {
                // If we can't find the element, it's external
                true
            };
            
            // Always determine style class based purely on relation type
            // This ensures consistent coloring based on relation
            let style_class = if relation.relation_type == "verifiedBy" || relation.relation_type == "verify" {
                // Verification relations get the green verification style
                "verification"
            } else if relation.relation_type == "satisfiedBy" || relation.relation_type == "satisfy" {
                // Satisfy relations get the yellow satisfy style
                "satisfies"
            } else {
                // Default style for other relation types - use requirement style (red)
                "requirement"
            };
            
            // Note: We're no longer overriding style based on external link status
            // This ensures color is based on relation type, not location
            
            if let Some(target_elem) = target_element {
                // Add click behavior for HTML if we found the target
                let target_html_file = if convert_to_html {
                    // In HTML mode, convert .md to .html
                    convert_path_to_html_link(&target_elem.file_path)
                } else {
                    // In markdown mode, keep as .md
                    target_elem.file_path.clone()
                };
                
                // Simple plain URL format without markdown link syntax
                diagram.push_str(&format!("{}click {} \"{}#{}\";\n", 
                    indent,
                    target_id, 
                    target_html_file,
                    target_elem.name.replace(' ', "-").to_lowercase()
                ));
            } else if target.contains('/') {
                // This might be a file reference
                // Extract file path and element name if present
                let parts: Vec<&str> = target.split('/').collect();
                let file_part = if parts.len() > 1 {
                    let mut path = String::new();
                    for i in 0..parts.len()-1 {
                        if i > 0 { path.push('/'); }
                        path.push_str(parts[i]);
                    }
                    if convert_to_html {
                        // In HTML mode, convert .md to .html
                        convert_path_to_html_link(&path)
                    } else {
                        // In markdown mode, keep path as is
                        path
                    }
                } else {
                    let path = parts[0].to_string();
                    if convert_to_html {
                        // In HTML mode, convert .md to .html
                        convert_path_to_html_link(&path)
                    } else {
                        // In markdown mode, keep path as is
                        path
                    }
                };
                
                let element_part = if parts.len() > 1 {
                    parts[parts.len()-1]
                } else {
                    ""
                };
                
                let link = if !element_part.is_empty() {
                    // Determine if this is a path to a file or a fragment identifier
                    let is_file_path = element_part.contains(".md") || element_part.contains(".html");
                    
                    if is_file_path {
                        // For file paths, use slash separator
                        if element_part.ends_with(".html") {
                            format!("{}/{}", file_part, element_part)
                        } else {
                            // For markdown links, convert if needed
                            let element_file = if convert_to_html && element_part.ends_with(".md") {
                                element_part.replace(".md", ".html")
                            } else {
                                element_part.to_string()
                            };
                            format!("{}/{}", file_part, element_file)
                        }
                    } else {
                        // For fragment identifiers, use hash separator
                        // Check if the element part already has .html suffix, and remove it if present
                        let element_anchor = if element_part.ends_with(".html") {
                            element_part[..element_part.len()-5].replace(' ', "-").to_lowercase()
                        } else {
                            element_part.replace(' ', "-").to_lowercase()
                        };
                        format!("{}#{}", file_part, element_anchor)
                    }
                } else {
                    file_part
                };
                
                // Use direct URL without markdown formatting for diagram click links
                // Make sure no markdown links are included
                let sanitized_link = if link.contains('[') && link.contains(']') && link.contains('(') && link.contains(')') {
                    // Extract just the URL part from a markdown link
                    let url_start = link.find('(').unwrap() + 1;
                    let url_end = link.rfind(')').unwrap();
                    link[url_start..url_end].to_string()
                } else {
                    link
                };
                diagram.push_str(&format!("{}click {} \"{}\";\n", indent, target_id, sanitized_link));
            } else if let Some(url) = &relation.url {
                // Use the URL extracted from the markdown link in the Relation struct
                // This ensures we always have the correct URL, not the markdown syntax
                
                // Add a click event with the right URL
                let mut click_url = if convert_to_html && url.ends_with(".md") {
                    url.replace(".md", ".html")
                } else {
                    url.to_string()
                };
                
                // Fix any paths where '#' was incorrectly used instead of '/'
                // This is a common issue when links look like "DesignSpecifications#Status.md"
                if click_url.contains('#') && !click_url.starts_with('#') {
                    // Check if this looks like a file path with a hash instead of a slash
                    if click_url.contains('#') && 
                       click_url.split('#').last().map_or(false, |last| last.contains('.')) {
                        // This is likely a path with a hash that should be a slash
                        click_url = click_url.replace('#', "/");
                    }
                }
                
                // Use just the URL for the click event, don't include markdown syntax
                // Make sure no markdown links are included
                let sanitized_url = if click_url.contains('[') && click_url.contains(']') && click_url.contains('(') && click_url.contains(')') {
                    // Extract just the URL part from a markdown link
                    let url_start = click_url.find('(').unwrap() + 1;
                    let url_end = click_url.rfind(')').unwrap();
                    click_url[url_start..url_end].to_string()
                } else {
                    click_url
                };
                diagram.push_str(&format!("{}click {} \"{}\";\n", indent, target_id, sanitized_url));
            } else if target.contains('(') && target.contains(')') && target.contains('[') && target.contains(']') {
                // This is a markdown link [text](url) format but wasn't extracted in the Relation struct
                // This is a fallback case
                // Extract the URL part
                let url_start = target.find('(').unwrap() + 1;
                let url_end = target.find(')').unwrap();
                let url = &target[url_start..url_end];
                
                // Add a click event with the right URL
                let mut click_url = if convert_to_html && url.ends_with(".md") {
                    url.replace(".md", ".html")
                } else {
                    url.to_string()
                };
                
                // Fix any paths where '#' was incorrectly used instead of '/'
                if click_url.contains('#') && !click_url.starts_with('#') {
                    // Check if this looks like a file path with a hash instead of a slash
                    if click_url.contains('#') && 
                       click_url.split('#').last().map_or(false, |last| last.contains('.')) {
                        // This is likely a path with a hash that should be a slash
                        click_url = click_url.replace('#', "/");
                    }
                }
                
                // Use just the URL for the click event, don't include markdown syntax
                // IMPORTANT: Make sure target_id is used for node ID, click_url for URL
                // Make sure no markdown links are included
                let sanitized_url = if click_url.contains('[') && click_url.contains(']') && click_url.contains('(') && click_url.contains(')') {
                    // Extract just the URL part from a markdown link
                    let url_start = click_url.find('(').unwrap() + 1;
                    let url_end = click_url.rfind(')').unwrap();
                    click_url[url_start..url_end].to_string()
                } else {
                    click_url
                };
                diagram.push_str(&format!("{}click {} \"{}\";\n", indent, target_id, sanitized_url));
            } else {
                // For completely external elements without a known link,
                // Create a generic link back to the document with the element as fragment
                let doc_link = if convert_to_html {
                    convert_path_to_html_link(file_path)
                } else {
                    file_path.to_string()
                };
                
                // Assume we can link back to original document with target as fragment
                let target_fragment = target.replace(' ', "-").to_lowercase();
                diagram.push_str(&format!("{}click {} \"{}#{}\";\n", 
                    indent, 
                    target_id, 
                    doc_link,
                    target_fragment
                ));
            }
            
            // Apply appropriate style class - maintain consistent indentation
            diagram.push_str(&format!("{}class {} {};\n", indent, target_id, style_class));
            
            // Mark target as included
            included_elements.insert(target.clone());
        }
    }
    
    Ok(())
}

/// Helper function to convert path to HTML format with proper extension handling
fn convert_path_to_html_link(path: &str) -> String {
    if path.ends_with(".md") {
        // Replace .md with .html
        path.replace(".md", ".html")
    } else if !path.contains('.') {
        // If no extension, add .html
        format!("{}.html", path)
    } else {
        // Otherwise keep as is
        path.to_string()
    }
}
