use crate::config::Config;
use crate::relation::{Relation, is_supported_relation_type};
use crate::identifier::normalize_identifier;
use regex::Regex;

pub fn parse_markdown(config: &Config, content: &str, file_path: &str) -> Vec<Element> {
    let mut elements = Vec::new();
    let mut current_element: Option<Element> = None;

    let element_re = Regex::new(r"^###\s+(.+)$").unwrap();
    let relation_re = Regex::new(r"^\s*\*\s*([\w]+):\s*\[([^\]]+)\]\(([^\)]+)\)$").unwrap();

    for line in content.lines() {
        if let Some(caps) = element_re.captures(line) {
            if let Some(element) = current_element.take() {
                elements.push(element);
            }
            let name = caps.get(1).unwrap().as_str().trim().to_string();
            current_element = Some(Element::new(&name, "", file_path, ElementType::Requirement));
        } else if let Some(caps) = relation_re.captures(line) {
            let rel_type = caps.get(1).unwrap().as_str();
            let rel_name = caps.get(2).unwrap().as_str();
            let rel_target = caps.get(3).unwrap().as_str();
            if let Some(ref mut element) = current_element {
                if let Some(relation) = Relation::new(config, rel_type, rel_target, rel_name) {
                    element.relations.push(relation);
                }
            }
        }
    }

    if let Some(element) = current_element {
        elements.push(element);
    }

    elements
}

