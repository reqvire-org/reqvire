use std::collections::HashMap;
use crate::relation::{Relation};


#[derive(Debug, Clone, PartialEq, Eq)] 
pub enum RequirementType {
    System,
    User,
}

#[derive(Debug, Clone, PartialEq, Eq)] 
pub enum ElementType {
    Requirement(RequirementType),
    Verification,
    File,
    Other(String),
}


impl ElementType {
    /// Parses a string into an ElementType
    pub fn from_metadata(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "user_requirement" => ElementType::Requirement(RequirementType::User),
            "requirement" => ElementType::Requirement(RequirementType::System),
            "verification" => ElementType::Verification,
            "file" => ElementType::File,
            other => ElementType::Other(other.to_string()),
        }
    }
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
    pub fn new(name: &str, identifier: &str, file_path: &str, element_type: Option<ElementType>) -> Self {
        Self {
            name: name.to_string(),
            content: "".to_string(),
            relations: vec![],
            identifier: identifier.to_string(),
            file_path: file_path.to_string(),
            element_type: element_type.unwrap_or(ElementType::Requirement(RequirementType::System)), 
            metadata: HashMap::new(),
        }
    }

    pub fn add_relation(&mut self, relation: Relation) -> () {
      self.relations.push(relation);
    }
    pub fn add_content(&mut self, content: &str) {
        if !self.content.is_empty() {
            self.content.push('\n');
        }
        self.content.push_str(content);
    }
    
    pub fn set_type_from_metadata(&mut self) {
        if let Some(type_value) = self.metadata.get("type") {
            self.element_type = ElementType::from_metadata(type_value);
        }
     }
   

}

