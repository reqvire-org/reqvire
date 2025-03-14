use std::collections::HashMap;
use crate::relation::{Relation, is_supported_relation_type};
use crate::identifier::normalize_identifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementType {
    Requirement,
    Verification,
    File,
    Other(String),
}

#[derive(Debug, Clone)]
pub struct Element {
    pub name: String,
    pub content: String,
    pub relations: Vec<Relation>,
    pub identifier: String,
    pub file_path: String,
    pub element_type: ElementType,
    pub metadata: HashMap<String, String>,
}

impl Element {
    pub fn new(name: &str, content: &str, file_path: &str, element_type: ElementType) -> Self {
        let identifier = normalize_identifier(file_path, name);
        Self {
            name: name.to_string(),
            content: content.to_string(),
            relations: vec![],
            identifier,
            file_path: file_path.to_string(),
            element_type,
            metadata: HashMap::new(),
        }
    }

    pub fn add_relation(&mut self, relation_type: &str, target: &str, name: &str) -> Result<(), String> {
        if is_supported_relation_type(relation_type) {
            if let Some(relation) = Relation::new(relation_type, target, name) {
                self.relations.push(relation);
                Ok(())
            } else {
                Err(format!("Invalid relation type: {}", relation_type))
            }
        } else {
            Err(format!("Unsupported relation type: {}", relation_type))
        }
    }
}

