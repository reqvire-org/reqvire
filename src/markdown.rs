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
        log::info!("Attempting to generate diagram for file: {}", current_file.display());
        
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

/// Generate a mermaid diagram for requirements documents
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
    println!("PATTERN MATCH: {}", &config.paths.requirements_filename_match);
    println!("IS REQUIREMENTS DOC: {}", is_requirements_doc);
    println!("BY PATH: {}", by_path);
    println!("BY TITLE: {}", by_title);
    
    // Use log module for more controlled output
    log::info!("Checking if file is a requirements document: {}", file_path);
    log::info!("Requirements check using config-driven detection: {}", is_requirements_doc);
    log::info!("Title regex match: {}", req_title_regex.is_match(content));
    
    // Force diagram generation for files that contain at least one element with relations
    // This respects the original detection but generates diagrams for any file with relations
    let has_elements_with_relations = content.contains("### ") && content.contains("#### Relations");
    
    if !is_requirements_doc && !has_elements_with_relations {
        log::info!("Skipping diagram generation - file is not a requirements document and has no elements with relations: {}", file_path);
        return Ok(content.to_string());
    }
    
    if has_elements_with_relations && !is_requirements_doc {
        log::info!("File contains elements with relations, proceeding with diagram generation despite not being a requirements document: {}", file_path);
    }
    
    log::info!("Generating diagram for requirements file: {}", file_path);
    
    // Define regex for paragraph headers
    let para_regex = match Regex::new(r"(?m)^##\s+(.+)$") {
        Ok(regex) => regex,
        Err(e) => return Err(ReqFlowError::InvalidRegex(format!("Error compiling regex: {}", e)))
    };
    
    // Find all elements in this file
    let elements_in_file: Vec<&Element> = registry.all_elements()
        .filter(|e| e.file_path == file_path)
        .collect();
    
    // Even if there are no elements, we'll proceed to generate an empty diagram
    // This helps us confirm the diagram generation logic is working
    
    // Only remove ReqFlow-generated diagrams at the top of the document
    // First, let's split the content into lines
    let lines: Vec<&str> = content.lines().collect();
    let mut updated_lines = Vec::new();
    
    // Skip past the title (first line)
    let mut i = 0;
    let mut title_found = false;
    
    // Add the title (first line with # prefix)
    while i < lines.len() {
        let line = lines[i];
        if line.starts_with("# ") {
            updated_lines.push(line);
            title_found = true;
            i += 1;
            break;
        }
        updated_lines.push(line);
        i += 1;
    }
    
    // Skip any blank lines after the title
    while i < lines.len() && lines[i].trim().is_empty() {
        i += 1;
    }
    
    // Now check if there's a ReqFlow-generated diagram (skip past it)
    let mut diagram_found = false;
    if i < lines.len() {
        // Check if next non-empty line starts a mermaid diagram
        if lines[i].trim() == "```mermaid" {
            diagram_found = true;
            
            // Skip past the entire mermaid diagram
            while i < lines.len() && !lines[i].trim().contains("```") {
                i += 1;
            }
            
            // Skip the closing ``` line
            if i < lines.len() {
                i += 1;
            }
            
            // Also skip metadata comment if it exists
            while i < lines.len() && (
                lines[i].trim().is_empty() || 
                lines[i].contains("<!-- ReqFlow-generated diagram")
            ) {
                i += 1;
            }
            
            log::info!("Removed existing mermaid diagram from {}", file_path);
        }
    }
    
    // Add the rest of the document (preserving any other mermaid diagrams)
    while i < lines.len() {
        updated_lines.push(lines[i]);
        i += 1;
    }
    
    // Join the lines back together
    let mut updated_content = updated_lines.join("\n");
    
    // If we removed a diagram, clean up any excess blank lines
    if diagram_found {
        let multiline_regex = match Regex::new(r"\n\n\n+") {
            Ok(regex) => regex,
            Err(e) => return Err(ReqFlowError::InvalidRegex(format!("Error compiling regex: {}", e)))
        };
        
        updated_content = multiline_regex.replace_all(&updated_content, "\n\n").to_string();
    } else {
        log::info!("No existing mermaid diagram found in {}", file_path);
    }
    
    // Add debug logging to understand the content at this point
    println!("UPDATED CONTENT (after cleaning, before new diagram):\n{}", updated_content.lines().take(10).collect::<Vec<&str>>().join("\n"));
    
    // Always generate a diagram, even if just with placeholder nodes for visualization
    let mut diagram = String::from("```mermaid\ngraph LR;\n");
    
    // Add graph styling with the required colors
    diagram.push_str("  %% Graph styling\n");
    diagram.push_str("  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;\n");
    diagram.push_str("  classDef satisfies fill:#fff2cc,stroke:#ffcc00,stroke-width:1px;\n");
    diagram.push_str("  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;\n");
    diagram.push_str("  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;\n");
    diagram.push_str("  classDef paragraph fill:#efefef,stroke:#999999,stroke-width:1px;\n");
    diagram.push_str("  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;\n\n");
    
    // If there are no elements, add a placeholder node so we still get a valid diagram
    if elements_in_file.is_empty() {
        log::info!("No elements found in file, adding placeholder node to diagram");
        diagram.push_str("  placeholder[\"No requirements elements found in this document\"];\n");
        diagram.push_str("  class placeholder default;\n");
    }
    
    // Debug full diagram content to console
    println!("MERMAID DIAGRAM CONTENT:\n{}", diagram);
    println!("ELEMENTS IN FILE: {}", elements_in_file.len());
    
    log::info!("MERMAID DIAGRAM CONTENT:\n{}", diagram);
    log::debug!("Generated base diagram with {} element styles", elements_in_file.len());
    
    // Parse the document to identify paragraphs and elements within each paragraph
    let lines: Vec<&str> = content.lines().collect();
    let mut paragraphs: Vec<String> = Vec::new();
    let mut elements_by_paragraph: HashMap<String, Vec<&Element>> = HashMap::new();
    let mut current_paragraph: Option<String> = None;
    
    for line in &lines {
        if let Some(captures) = para_regex.captures(line) {
            if let Some(para_name) = captures.get(1) {
                let paragraph_name = para_name.as_str().to_string();
                paragraphs.push(paragraph_name.clone());
                current_paragraph = Some(paragraph_name);
            }
        }
    }
    
    // Add "No Section" for elements before any paragraph
    paragraphs.insert(0, "No Section".to_string());
    
    // Now parse the document again to associate elements with paragraphs
    current_paragraph = Some(paragraphs[0].clone()); // Start with "No Section"
    
    for line in &lines {
        if let Some(captures) = para_regex.captures(line) {
            if let Some(para_name) = captures.get(1) {
                current_paragraph = Some(para_name.as_str().to_string());
            }
        } else if let Some(element_name) = crate::config::Config::element_regex()
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
    
    // Create a set of elements already included in this diagram
    let mut included_elements = HashSet::new();
    
    // Add paragraph subgraphs to the diagram
    for (i, paragraph) in paragraphs.iter().enumerate() {
        let paragraph_id = format!("para{}", i);
        
        // Only include paragraphs that have elements
        if let Some(elements) = elements_by_paragraph.get(paragraph) {
            if !elements.is_empty() {
                if paragraph == "No Section" {
                    // For elements not in any paragraph, don't create a subgraph
                    // Just add them directly to the main graph
                    for element in elements {
                        add_element_to_diagram(
                            &mut diagram, 
                            element, 
                            &mut included_elements, 
                            registry, 
                            file_path, 
                            convert_to_html,
                            None
                        )?;
                    }
                } else {
                    // Create a subgraph for this paragraph
                    diagram.push_str(&format!("  subgraph {}[\"{}\"];\n", paragraph_id, paragraph));
                    
                    // Add elements in this paragraph
                    for element in elements {
                        add_element_to_diagram(
                            &mut diagram, 
                            element, 
                            &mut included_elements, 
                            registry, 
                            file_path, 
                            convert_to_html,
                            Some(paragraph_id.clone())
                        )?;
                    }
                    
                    diagram.push_str("  end;\n");
                    diagram.push_str(&format!("  class {} paragraph;\n", paragraph_id));
                }
            }
        }
    }
    
    // Close the mermaid diagram
    diagram.push_str("```\n\n");
    
    // Find the position to insert the diagram
    // It should be after the main title but before the first paragraph heading
    let mut lines = updated_content.lines().collect::<Vec<&str>>();
    let mut insert_pos = 0;
    
    // Skip past the title heading (# Title)
    let title_regex = match Regex::new(r"^#\s+.*") {
        Ok(regex) => regex,
        Err(e) => return Err(ReqFlowError::InvalidRegex(format!("Error compiling regex: {}", e)))
    };
    
    // Log first few lines for debugging
    println!("FIRST 10 LINES OF CONTENT:");
    for (i, line) in lines.iter().enumerate().take(10) {
        println!("Line {}: '{}'", i, line);
    }
    
    log::debug!("First 5 lines of content:");
    for (i, line) in lines.iter().enumerate().take(5) {
        log::debug!("Line {}: '{}'", i, line);
    }
    
    let mut title_found = false;
    for (i, line) in lines.iter().enumerate() {
        if title_regex.is_match(line) {
            println!("Found title at line {}: '{}'", i, line);
            log::debug!("Found title at line {}: '{}'", i, line);
            insert_pos = i + 1;
            title_found = true;
            break;
        }
    }
    
    // If no title was found, insert at the beginning
    if !title_found {
        println!("No title found, using position 0");
        log::debug!("No title found, using position 0");
        insert_pos = 0;
    }
    
    // Find first non-empty line after title
    while insert_pos < lines.len() && lines[insert_pos].trim().is_empty() {
        println!("Skipping empty line at position {}", insert_pos);
        log::debug!("Skipping empty line at position {}", insert_pos);
        insert_pos += 1;
    }
    
    println!("Final insert position: {}", insert_pos);
    log::debug!("Final insert position: {}", insert_pos);
    log::info!("Inserting diagram after title at position {}", insert_pos);
    println!("DIAGRAM TO INSERT (length: {} chars):\n{}", diagram.len(), diagram.lines().take(5).collect::<Vec<&str>>().join("\n"));
    
    // Insert the diagram at the determined position
    // We'll add an extra empty line before and after for better visual separation
    lines.insert(insert_pos, "");
    lines.insert(insert_pos + 1, &diagram);
    lines.insert(insert_pos + 2, "");
    
    // Debug the first few lines after insertion
    println!("AFTER INSERTION - FIRST 15 LINES:");
    for (i, line) in lines.iter().enumerate().take(15) {
        println!("Line {}: '{}'", i, line);
    }
    
    // Log information at appropriate level
    log::debug!("Added diagram to file: {}", file_path);
    log::debug!("Diagram position: {}", insert_pos);
    log::debug!("Diagram content length: {} characters", diagram.len());
    
    // Before joining lines, add a metadata comment right after the closing ``` of the diagram
    // This ensures the comment doesn't break the mermaid syntax
    for (i, line) in lines.iter().enumerate() {
        if i > insert_pos && line.trim() == "```" {
            // Insert the metadata comment after the closing ``` line
            lines.insert(i + 1, "<!-- ReqFlow-generated diagram - do not modify manually -->");
            lines.insert(i + 2, ""); // Add an empty line after the comment for better readability
            break;
        }
    }
    
    // Join the lines with newlines, ensuring we have consistent line endings
    let result = lines.join("\n");
    
    // Final debug of the result (truncated)
    println!("FINAL RESULT (first 500 chars):\n{:.500}...", result);
    println!("RESULT LENGTH: {} characters", result.len());
    
    log::info!("Diagram generation complete for file: {}", file_path);
    
    Ok(result)
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
    
    // Add click behavior for HTML
    let html_file = if convert_to_html {
        element.file_path.replace(".md", ".html")
    } else {
        element.file_path.clone()
    };
    
    diagram.push_str(&format!("{}click {} \"{}#{}\";\n", 
        indent, 
        element_id, 
        html_file,
        element.name.replace(' ', "-").to_lowercase()
    ));
    
    // Apply requirement style to this element
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
        
        // Different arrow styles and labels based on relation type
        let arrow_style = match relation.relation_type.as_str() {
            "verifiedBy" | "verify" => "-->|verifies|",
            "satisfiedBy" | "satisfy" => "-->|satisfies|",
            "derivedFrom" | "derive" => "-.->|derives from|",
            "refine" => "==>|refines|",
            "tracedFrom" | "trace" => "-->|traces from|", // Changed from ~~~> which is not valid in mermaid
            "containedBy" | "contain" => "--o|contains|",
            _ => "-->|relates to|"
        };
        
        // Add the relationship
        diagram.push_str(&format!("{}{} {} {};\n", indent, element_id, arrow_style, target_id));
        
        // If the target is not already included in the diagram, add it
        if !included_elements.contains(target) {
            // Add the target node - always at top level, not inside a subgraph
            // as the target element might be in a different paragraph or file
            diagram.push_str(&format!("  {}[\"{}\"];\n", target_id, target.replace('"', "&quot;")));
            
            // Try to find the target element in the registry
            let target_element = registry.get_element(target);
            
            // Check if the target element is from a different file
            let is_external_link = if let Some(target_elem) = target_element {
                // It's external if it's from a different file
                target_elem.file_path != file_path
            } else {
                // If we can't find the element, it's external
                true
            };
            
            // Determine the style class based on relation type and element location
            let style_class = if is_external_link {
                // External elements get the blue link style
                "externalLink"
            } else if relation.relation_type == "verifiedBy" || relation.relation_type == "verify" {
                // Verification elements get the green verification style
                "verification"
            } else if relation.relation_type == "satisfiedBy" || relation.relation_type == "satisfy" {
                // Elements that satisfy requirements get the yellow satisfy style
                "satisfies"
            } else {
                // Default style for other elements
                "default"
            };
            
            if let Some(target_elem) = target_element {
                // Add click behavior for HTML if we found the target
                let target_html_file = if convert_to_html {
                    target_elem.file_path.replace(".md", ".html")
                } else {
                    target_elem.file_path.clone()
                };
                
                diagram.push_str(&format!("  click {} \"{}#{}\";\n", 
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
                        path.replace(".md", ".html")
                    } else {
                        path
                    }
                } else {
                    let path = parts[0].to_string();
                    if convert_to_html {
                        path.replace(".md", ".html")
                    } else {
                        path
                    }
                };
                
                let element_part = if parts.len() > 1 {
                    parts[parts.len()-1]
                } else {
                    ""
                };
                
                let link = if !element_part.is_empty() {
                    format!("{}#{}", file_part, element_part.replace(' ', "-").to_lowercase())
                } else {
                    file_part
                };
                
                diagram.push_str(&format!("  click {} \"{}\";\n", target_id, link));
            }
            
            // Apply appropriate style class
            diagram.push_str(&format!("  class {} {};\n", target_id, style_class));
            
            // Mark target as included
            included_elements.insert(target.clone());
        }
    }
    
    Ok(())
}