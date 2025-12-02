use std::collections::HashMap;
use std::path::PathBuf;
use crate::relation::Relation;
use crate::utils;
use serde::Serialize;

/// Represents the target of an attachment - either a file path or an element identifier
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum AttachmentTarget {
    /// File path attachment (git-root-relative, normalized)
    FilePath(PathBuf),
    /// Element identifier attachment (must point to a Refinement element)
    ElementIdentifier(String),
}

impl AttachmentTarget {
    /// Returns a string representation of the attachment target
    pub fn as_str(&self) -> String {
        match self {
            AttachmentTarget::FilePath(path) => path.to_string_lossy().to_string(),
            AttachmentTarget::ElementIdentifier(id) => id.clone(),
        }
    }

    /// Returns true if this is a file path attachment
    pub fn is_file_path(&self) -> bool {
        matches!(self, AttachmentTarget::FilePath(_))
    }

    /// Returns true if this is an element identifier attachment
    pub fn is_element_identifier(&self) -> bool {
        matches!(self, AttachmentTarget::ElementIdentifier(_))
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Attachment {
    pub target: AttachmentTarget,
    /// Content hash for file attachments (FilePath only).
    /// For ElementIdentifier attachments, the hash is looked up from registry.
    pub content_hash: Option<String>,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub enum SubSection {
    Other(String),
    Requirement,
    Relations,
    Metadata,
    Details,
    Properties,
    Attachments,
}
impl SubSection {
    pub fn name(&self) -> &str {
        match self {
            SubSection::Requirement => "Requirement",
            SubSection::Relations => "Relations",
            SubSection::Metadata => "Metadata",
            SubSection::Details => "Details",
            SubSection::Properties => "Properties",
            SubSection::Attachments => "Attachments",
            SubSection::Other(name) => name.as_str(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Requirement" => SubSection::Requirement,
            "Relations" => SubSection::Relations,
            "Metadata" => SubSection::Metadata,
            "Details" =>   SubSection::Details,
            "Properties" => SubSection::Properties,
            "Attachments" => SubSection::Attachments,
            other => SubSection::Other(other.to_string()),
        }
    }
}




#[derive(Debug, Clone, PartialEq, Eq, Serialize)] 
pub enum RequirementType {
    System,
    User,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum VerificationType {
    Default,
    Test,
    Analysis,
    Inspection,
    Demonstration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum RefinementType {
    Constraint,
    Behavior,
    Specification,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ElementType {
    Requirement(RequirementType),
    Verification(VerificationType),
    Refinement(RefinementType),
    File,
    Other(String),
}


impl ElementType {
    /// Returns the metadata key corresponding to this ElementType,
    /// e.g. "user_requirement", "analysis-verification", or the
    /// raw string for Other.
    pub fn as_str(&self) -> &str {
        match self {
            ElementType::Requirement(req) => match req {
                RequirementType::User   => "user-requirement",
                RequirementType::System => "requirement",
            },
            ElementType::Verification(ver) => match ver {
                VerificationType::Default       => "test-verification",
                VerificationType::Test          => "test-verification",
                VerificationType::Analysis      => "analysis-verification",
                VerificationType::Inspection    => "inspection-verification",
                VerificationType::Demonstration => "demonstration-verification",
            },
            ElementType::Refinement(ref_type) => match ref_type {
                RefinementType::Constraint    => "constraint",
                RefinementType::Behavior      => "behavior",
                RefinementType::Specification => "specification",
            },
            ElementType::File => "file",
            ElementType::Other(s) => s.as_str(),
        }
    }

    
    /// Parses a string into an ElementType
    pub fn from_metadata(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "user-requirement" => ElementType::Requirement(RequirementType::User),
            "requirement" | "system-requirement" => ElementType::Requirement(RequirementType::System),

            // Different verification types
            "verification" => ElementType::Verification(VerificationType::Test),
            "test-verification" => ElementType::Verification(VerificationType::Test),
            "analysis-verification" => ElementType::Verification(VerificationType::Analysis),
            "inspection-verification" => ElementType::Verification(VerificationType::Inspection),
            "demonstration-verification" => ElementType::Verification(VerificationType::Demonstration),

            // Refinement types
            "constraint" => ElementType::Refinement(RefinementType::Constraint),
            "behavior" => ElementType::Refinement(RefinementType::Behavior),
            "specification" => ElementType::Refinement(RefinementType::Specification),

            "file" => ElementType::File,
            other => ElementType::Other(other.to_string()),
        }
    }

    /// Returns true if this element type is a Refinement type
    pub fn is_refinement(&self) -> bool {
        matches!(self, ElementType::Refinement(_))
    }

    /// Returns the main type category for merge compatibility
    pub fn main_category(&self) -> &'static str {
        match self {
            ElementType::Requirement(_) => "requirement",
            ElementType::Verification(_) => "verification",
            ElementType::Refinement(_) => "refinement",
            ElementType::File => "file",
            ElementType::Other(_) => "other",
        }
    }

    /// Check if two element types are merge-compatible
    /// Elements are merge-compatible if they belong to the same main type category
    pub fn is_merge_compatible(&self, other: &ElementType) -> bool {
        self.main_category() == other.main_category()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Element {
    pub name: String,
    /// Stable Element ID - globally unique, location-independent identifier
    /// This is the normalized element name that remains unchanged across relocations
    #[serde(skip)]
    pub id: String,
    pub content: String,
    pub relations: Vec<Relation>,
    pub identifier: String,
    pub file_path: String,
    pub line_number: usize,
    pub element_type: ElementType,
    pub metadata: HashMap<String, String>,
    //
    // hash of content that is taken into impact change detection
    pub hash_impact_content: String,
    //
    pub changed_since_commit: bool,
    //
    // Order index within the file (used for preserving original order)
    pub file_order_index: usize,
    //
    // Attachments - external documents linked to this element
    pub attachments: Vec<Attachment>,
}



impl Element {
    pub fn new(name: &str, identifier: &str, file_path: &str, line_number: usize, element_type: Option<ElementType>) -> Self {
        // Extract stable ID (fragment) from identifier
        let id = utils::extract_path_and_fragment(identifier).1
            .unwrap_or(identifier)
            .to_string();

        Self {
            name: name.to_string(),
            id,
            content: "".to_string(),
            hash_impact_content: "".to_string(),
            relations: vec![],
            identifier: identifier.to_string(),
            file_path: file_path.to_string(),
            line_number,
            element_type: element_type.unwrap_or(ElementType::Requirement(RequirementType::System)),
            metadata: HashMap::new(),
            changed_since_commit: false,
            file_order_index: 0, // Will be set during parsing
            attachments: vec![],
        }
    }

    pub fn add_relation(&mut self, relation: Relation) -> () {
    
    
      self.relations.push(relation);
    }

    pub fn add_content(&mut self, content: &str) {
        self.content.push_str(content);
    }

    pub fn freeze_content(&mut self) {
        // Trim newlines and tabs from the beginning and end.
        let trimmed = self.content.trim_matches(&['\n', '\t'][..]);     
                
        // Normalize content by removing all whitespace (spaces, tabs, newlines, etc.)
        let normalized: String = trimmed.chars().filter(|c| !c.is_whitespace()).collect();

        self.content=trimmed.to_string();
        self.hash_impact_content=utils::hash_content(&normalized);
    }
        
    pub fn set_type_from_metadata(&mut self) {
        if let Some(type_value) = self.metadata.get("type") {
            self.element_type = ElementType::from_metadata(type_value);
        }
     }
   
    pub fn extract_fragment(&self) -> String {
        match self.identifier.split_once('#') {
            Some((_, fragment)) => fragment.to_string(),
            None => "".to_string(),
        }
    }



}
