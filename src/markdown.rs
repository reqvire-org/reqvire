use crate::element_registry::ElementRegistry;
use std::path::Path;
use crate::error::ReqFlowError;
use crate::element::Element;
use regex::Regex;
use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::relation::LinkType;


/// TODO: Move this into utils and reuse it everywhere
pub struct RegexPatterns {
    pub mermaid: Regex,
    pub paragraph: Regex,
    pub element: Regex,
    pub subsection: Regex,
    pub metadata: Regex,
    pub relation: Regex,
    pub markdown_link: Regex,
}

impl RegexPatterns {
    /// Initializes and returns a static reference to the compiled regex patterns.
    pub fn get() -> &'static RegexPatterns {
        lazy_static! {
            static ref PATTERNS: RegexPatterns = RegexPatterns {
                mermaid: Regex::new(r"(?s)```mermaid\s*graph (TD|LR);.*?```\s*").unwrap(),
                paragraph: Regex::new(r"^##\s+(.+)").unwrap(),
                element: Regex::new(r"^###\s+(.+)").unwrap(),
                subsection: Regex::new(r"^####\s+(.+)").unwrap(),
                metadata: Regex::new(r"[\*\-]\s+(\w+):\s*(.+)").unwrap(),
                relation: Regex::new(r"^\s*(?:\*|-)\s+(\w+):\s*(.+)").unwrap(),
                markdown_link: Regex::new(r"\[.+?\]\(.+?\)").unwrap(),
            };
        }
        &PATTERNS
    }
}

/// Replace relations in markdown with proper links
pub fn replace_relations(
    content: &str,
    registry: &ElementRegistry,
    current_file: &Path,
    direction: & str,       
    should_convert_links: bool,
) -> Result<String, ReqFlowError> {

    let patterns = RegexPatterns::get();

    // Process relations first
    let processed_content = process_relations(content, current_file, should_convert_links)?;

    log::debug!("Generating diagram for file: {}", current_file.display());
    let updated_content = generate_requirements_diagram(
        &processed_content, 
        registry, 
        &current_file.to_string_lossy(), 
        direction,
        should_convert_links
    )?;
    log::debug!("Diagram generation completed for file: {}", current_file.display());
    Ok(updated_content)

}


/// Generate mermaid diagrams for documents with relations and elements.
/// This function creates a separate diagram for each section (level 2 heading).
pub fn generate_requirements_diagram(
    content: &str,
    registry: &ElementRegistry,
    file_path: &str,
    direction: & str,   
    convert_to_html: bool,
) -> Result<String, ReqFlowError> {
    log::debug!("Generating diagrams for file: {}", file_path);

    let patterns = RegexPatterns::get();

    let elements_in_file: Vec<&Element> = registry.get_all_elements()
    .iter()
    .filter(|e| e.file_path == file_path)
    .map(|e| *e) // Explicitly dereference
    .collect();
    
    let lines: Vec<&str> = content.lines().collect();
    let mut result_lines = Vec::new();

    let mut in_diagram = false;
    let mut skipping_diagram = false;

    for line in lines {
        if line.trim().contains("<!-- ReqFlow-generated diagram") {
            continue; // Skip generated diagram markers
        }

        if line.trim() == "```mermaid" {
            skipping_diagram = true;
            in_diagram = true;
            continue;
        }

        if in_diagram {
            if line.trim().ends_with("```") {
                in_diagram = false;
            }
            continue;
        }

        result_lines.push(line.to_string());
    }

    let final_content = patterns.mermaid.replace_all(&result_lines.join("\n"), "").to_string();

    let mut paragraphs: Vec<(usize, String)> = Vec::new();
    let mut elements_by_paragraph: HashMap<String, Vec<&Element>> = HashMap::new();

    let lines: Vec<&str> = final_content.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if let Some(caps) = patterns.paragraph.captures(line) {
            if let Some(para_name) = caps.get(1) {
                paragraphs.push((i, para_name.as_str().to_string()));
            }
        }
    }

    if paragraphs.is_empty() {
        paragraphs.push((0, "Document".to_string()));
    }

    let mut current_paragraph: Option<String> = Some(paragraphs[0].1.clone());
    let mut current_para_idx = 0;

    for (_i, line) in lines.iter().enumerate() {
        if current_para_idx < paragraphs.len() - 1 && _i >= paragraphs[current_para_idx + 1].0 {
            current_para_idx += 1;
            current_paragraph = Some(paragraphs[current_para_idx].1.clone());
        }

        if let Some(element_name) = patterns.element.captures(line)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim().to_string())
        {
            if let Some(paragraph) = &current_paragraph {
                if let Some(element) = elements_in_file.iter().find(|e| e.name == element_name) {
                    elements_by_paragraph.entry(paragraph.clone())
                        .or_insert_with(Vec::new)
                        .push(element);
                }
            }
        }
    }

    let mut result_lines = Vec::new();
    let mut in_first_intro_section = true;

    for (_i, line) in lines.iter().enumerate() {
        let is_paragraph_heading = patterns.paragraph.is_match(line);
        result_lines.push(line.to_string());

        if is_paragraph_heading || in_first_intro_section {
            in_first_intro_section = false;
            let para_name = patterns.paragraph.captures(line)
                .and_then(|caps| caps.get(1))
                .map(|m| m.as_str().to_string())
                .unwrap_or_else(|| "Unknown".to_string());

            if let Some(elements) = elements_by_paragraph.get(&para_name) {
                if !elements.is_empty() {
                    let diagram = generate_diagram_for_paragraph(
                        &para_name, elements, registry, file_path, direction, convert_to_html
                    )?;
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
    direction: & str,    
    convert_to_html: bool
) -> Result<String, ReqFlowError> {
    use std::collections::HashSet;


    // Get diagram direction from config (TD or LR)
    let mut diagram = String::from(format!("```mermaid\ngraph {};\n", direction));



    // Add graph styling for different relation types
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
        let mut included_elements = HashSet::new();

        // Process each element and add it to the diagram
        for element in elements {
            add_element_to_diagram(
                &mut diagram,
                element,
                &mut included_elements,
                registry,
                file_path,
                convert_to_html,
                Some(paragraph_name.to_string())
            )?;
        }
    }

    // Close the Mermaid diagram
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
    // Generate a safe ID for the element by replacing non-alphanumeric characters with underscores
    let element_id = element.name.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>();

    // Encode the label text properly for Mermaid
    let label = element.name.replace('"', "&quot;");

    // Indentation for subgraph elements
    let indent = if subgraph_id.is_some() { "    " } else { "  " };

    // Add the element to the diagram
    diagram.push_str(&format!("{}{}[\"{}\"];\n", indent, element_id, label));

    // Determine the correct file path for links
    let html_file = if convert_to_html {
        convert_path_to_html_link(&element.file_path)
    } else {
        element.file_path.clone()
    };

    // Add clickable behavior to navigate to the element
    diagram.push_str(&format!(
        "{}click {} \"{}#{}\";\n",
        indent,
        element_id,
        html_file,
        element.name.replace(' ', "-").to_lowercase()
    ));

    // Apply requirement style to this element (default)
    diagram.push_str(&format!("{}class {} requirement;\n", indent, element_id));

    // Mark this element as included
    included_elements.insert(element.name.clone());

    // Process each relation of the element
    for relation in &element.relations {
        // Extract target name and link type
        let target = &relation.target;
        let target_id = target.text.chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect::<String>();

        struct RelationProperties {
            arrow: &'static str,      // Arrow style
            label: &'static str,      // Displayed label
            reverse_direction: bool,  // Whether to reverse the arrow direction
        }

        // Define relation styles based on type
        let properties = match relation.relation_type.name {
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

            rel_type if rel_type.ends_with("By") => {
                let base_name = &rel_type[..rel_type.len() - 2];

                let (arrow, label, should_reverse) = if base_name.contains("derive") {
                    ("-.->", "deriveReqT", false)
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

                RelationProperties { arrow, label, reverse_direction: should_reverse }
            }

            _ => RelationProperties { arrow: "-->", label: "relates to", reverse_direction: false },
        };

        // Determine relation direction
        let (from_id, to_id) = if properties.reverse_direction {
            (target_id.clone(), element_id.clone())
        } else {
            (element_id.clone(), target_id.clone())
        };

        // Add the relationship to the diagram
        diagram.push_str(&format!(
            "{}{} {}|{}| {};\n",
            indent, from_id, properties.arrow, properties.label, to_id
        ));

        // If the target is not included yet, add it
        if !included_elements.contains(&target.text) {
            let display_text = target.text.replace('"', "&quot;");

            diagram.push_str(&format!("{}{}[\"{}\"];\n", indent, target_id, display_text));

            // Determine if the target element exists in the registry
            if let Ok(target_elem) = registry.get_element(&target.text) {
                let target_html_file = if convert_to_html {
                    convert_path_to_html_link(&target_elem.file_path)
                } else {
                    target_elem.file_path.clone()
                };

                diagram.push_str(&format!(
                    "{}click {} \"{}#{}\";\n",
                    indent,
                    target_id,
                    target_html_file,
                    target_elem.name.replace(' ', "-").to_lowercase()
                ));
            } else if let LinkType::ExternalUrl(url) = &target.link {
                diagram.push_str(&format!("{}click {} \"{}\";\n", indent, target_id, url));
            } else {
                let doc_link = if convert_to_html {
                    convert_path_to_html_link(file_path)
                } else {
                    file_path.to_string()
                };
                let target_fragment = target.text.replace(' ', "-").to_lowercase();
                diagram.push_str(&format!(
                    "{}click {} \"{}#{}\";\n",
                    indent,
                    target_id,
                    doc_link,
                    target_fragment
                ));
            }

            diagram.push_str(&format!(
                "{}class {} {};\n",
                indent,
                target_id,
                match relation.relation_type.name {
                    "verifiedBy" | "verify" => "verification",
                    "satisfiedBy" | "satisfy" => "satisfies",
                    _ => "requirement",
                }
            ));

            included_elements.insert(target.text.clone());
        }
    }

    Ok(())
}

/// Converts a file path to an appropriate HTML link format
fn convert_path_to_html_link(path: &str) -> String {
    if path.ends_with(".md") {
        path.replace(".md", ".html")
    } else if !path.contains('.') {
        format!("{}.html", path)
    } else {
        path.to_string()
    }
}

/// Process markdown relations and update their references
fn process_relations(
    content: &str,
    current_file: &Path,
    should_convert_links: bool,
) -> Result<String, ReqFlowError> {
    let patterns = RegexPatterns::get();

    let mut updated_content = content.to_string();

    // Replace markdown links within content
    for cap in patterns.markdown_link.captures_iter(content) {
        if let Some(full_match) = cap.get(0) {
            let replacement = convert_markdown_link(full_match.as_str(), current_file, should_convert_links);
            updated_content = updated_content.replace(full_match.as_str(), &replacement);
        }
    }

    Ok(updated_content)
}

/// Converts markdown links based on configuration (for HTML output)
fn convert_markdown_link(link: &str, current_file: &Path, should_convert_links: bool) -> String {
    // Extract the text and URL from the markdown link format: [text](url)
    if let Some(start) = link.find('(') {
        if let Some(end) = link.find(')') {
            let text_part = &link[..start].trim_matches(|c| c == '[' || c == ']');
            let url_part = &link[start + 1..end];

            let new_url = if should_convert_links {
                convert_path_to_html_link(url_part)
            } else {
                url_part.to_string()
            };

            return format!("[{}]({})", text_part, new_url);
        }
    }
    link.to_string()
}


