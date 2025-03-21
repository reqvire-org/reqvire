use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::error::ReqFlowError;
use serde::Serialize;
use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum RelationDirection {
    Forward,
    Backward,
    Neutral,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct RelationTypeInfo {
    pub name: &'static str,
    pub direction: RelationDirection,
    pub opposite: Option<&'static str>,
    pub description: &'static str,
}

lazy_static! {
    pub static ref RELATION_TYPES: HashMap<&'static str, RelationTypeInfo> = {
        let mut m = HashMap::new();
        
        // Containment relations
        m.insert("containedBy", RelationTypeInfo {
            name: "containedBy", 
            direction: RelationDirection::Backward, 
            opposite: Some("contain"),
            description: "Element is contained by another element",
        });
        m.insert("contain", RelationTypeInfo {
            name: "contain", 
            direction: RelationDirection::Forward, 
            opposite: Some("containedBy"),
            description: "Element contains another element",
        });
        
        // Derive relations
        m.insert("derivedFrom", RelationTypeInfo {
            name: "derivedFrom", 
            direction: RelationDirection::Backward, 
            opposite: Some("derive"),
            description: "Element is derived from another element",
        });
        m.insert("derive", RelationTypeInfo {
            name: "derive", 
            direction: RelationDirection::Forward, 
            opposite: Some("derivedFrom"),
            description: "Element is source for a derived element",
        });
        
        // Refine relation
        m.insert("refine", RelationTypeInfo {
            name: "refine", 
            direction: RelationDirection::Backward,
            opposite: Some("refinedBy"),
            description: "Element refines a higher-level element",
        });
        
        // Refine relation
        m.insert("refinedBy", RelationTypeInfo {
            name: "refinedBy", 
            direction: RelationDirection::Forward,
            opposite: Some("refine"),
            description: "A souce element being refined by other element.",
        });        
        
        // Satisfy relations
        m.insert("satisfiedBy", RelationTypeInfo {
            name: "satisfiedBy", 
            direction: RelationDirection::Forward, 
            opposite: Some("satisfy"),
            description: "A souce element being satisfied by other element.",
        });
        m.insert("satisfy", RelationTypeInfo {
            name: "satisfy", 
            direction: RelationDirection::Backward, 
            opposite: Some("satisfiedBy"),
            description: "Element satisfies another element",
        });
        
        // Verify relations
        m.insert("verifiedBy", RelationTypeInfo {
            name: "verifiedBy", 
            direction: RelationDirection::Forward, 
            opposite: Some("verify"),
            description: "A souce element being verified by other element.",
        });
        m.insert("verify", RelationTypeInfo {
            name: "verify", 
            direction: RelationDirection::Backward, 
            opposite: Some("verifiedBy"),
            description: "Element verifies another element",
        });
        
        // Trace relations
        m.insert("trace", RelationTypeInfo {
            name: "trace", 
            direction: RelationDirection::Neutral, 
            opposite: None,
            description: "Element is related to another element in a non-directional way",
        });

        m
    };
}

#[derive(Debug, Clone, Serialize)]
pub struct RelationTarget {
    pub text: String,
    pub link: LinkType,
}

impl PartialEq for RelationTarget {
    fn eq(&self, other: &Self) -> bool {
        self.link.as_str() == other.link.as_str()
    }
}

impl Eq for RelationTarget {}

impl Ord for RelationTarget {
    fn cmp(&self, other: &Self) -> Ordering {
        self.link.as_str().cmp(&other.link.as_str())
    }
}

impl PartialOrd for RelationTarget {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for RelationTarget {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.link.as_str().hash(state);
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum LinkType {
    Identifier(String), // Internal reference, e.g., "some-identifier"
    ExternalUrl(String), // External URL, e.g., "https://example.com"
}
impl LinkType {
    /// Converts `LinkType` into a string representation.
    pub fn as_str(&self) -> &str {
        match self {
            LinkType::Identifier(id) => id,
            LinkType::ExternalUrl(url) => url,
        }
    }
}



#[derive(Debug, Clone, Serialize)]
pub struct Relation {
    pub relation_type: &'static RelationTypeInfo,
    pub target: RelationTarget,
    pub is_opposite: bool
}

impl PartialEq for Relation {
    fn eq(&self, other: &Self) -> bool {
        self.relation_type.name == other.relation_type.name && self.target == other.target
    }
}

impl Eq for Relation {}


impl Ord for Relation {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare relation types by name first
        let relation_cmp = self.relation_type.name.cmp(&other.relation_type.name);

        // If relation types are equal, compare targets
        if relation_cmp == Ordering::Equal {
            self.target.cmp(&other.target)
        } else {
            relation_cmp
        }
    }
}

impl PartialOrd for Relation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Relation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.relation_type.name.hash(state);
        self.target.hash(state);
        self.is_opposite.hash(state);
    }
}

impl Relation {
    pub fn new(relation_type: &str, text: String, normalized_target: &str) -> Result<Self, ReqFlowError> {   
        let link=Self::parse_link_type(normalized_target);
               
        let relation_info = RELATION_TYPES.get(relation_type)
            .ok_or_else(|| ReqFlowError::UnsupportedRelationType(relation_type.to_string()))?;
        Ok(Self {
            relation_type: relation_info,
            target: RelationTarget{text: text, link: link},
            is_opposite: false,
        })
    }
    
    /// Determines if the link should be treated as an identifier or an external URL.
    fn parse_link_type(link: &str) -> LinkType {
        if Self::is_path_reference(link) {
            LinkType::Identifier(link.to_string())
        } else {
            LinkType::ExternalUrl(link.to_string())
        }
    }

    pub fn update_target_identifier_link_url(&mut self, url: &str)  {
        match self.target.link {
            LinkType::Identifier(_) =>  self.target.link=LinkType::Identifier(url.to_string()),
            _ =>{}
        };  
    }


    /// Determines whether the given string is a path reference (i.e., identifier).
    fn is_path_reference(link: &str) -> bool {
        // Common external protocols used in documentation and software
        let external_protocols = [
            "http://", "https://", // Web
            "file://",  // Local file paths
            "ftp://",   // FTP links
            "mailto:",  // Email links
            "ssh://",   // SSH links
            "git://",   // Git repository links
            "data:",    // Data URIs
        ];

        // Check if the link starts with any of these external protocols
        !external_protocols.iter().any(|&proto| link.starts_with(proto))
    } 
    

    /// Creates an opposite relation if possible for given target
    pub fn to_opposite(&self, name: &str, identifier: &str) -> Option<Relation> {
        if let Some(opposite_name) = self.relation_type.opposite {
            if let Some(opposite_info) = RELATION_TYPES.get(opposite_name) {
                Some(Relation {
                    relation_type: opposite_info,
                    target: RelationTarget {
                        text: name.to_string(),
                        link: LinkType::Identifier(identifier.to_string())
                    },
                    is_opposite: true                    
                })
            } else {
                None
            }
        } else {
            None
        }
    }   
}



/// Check if a relation type is supported according to the DSD
pub fn is_supported_relation_type(relation_type: &str) -> bool {
    RELATION_TYPES.contains_key(relation_type)
}

/// Check if revalidation is needed
pub fn needs_revalidation(relation_type: &str) -> bool {
    if RELATION_TYPES.contains_key(relation_type) {
        relation_type == "verifiedBy"
    } else {
        false
    }
}
/// Check if review is needed
pub fn needs_review(relation_type: &str) -> bool {
    if RELATION_TYPES.contains_key(relation_type) {
        relation_type == "satisfiedBy"
    } else {
        false
    }
}

/// Get the list of all supported relation types
pub fn get_supported_relation_types() -> Vec<&'static str> {
    RELATION_TYPES.keys().cloned().collect()
}

/// Get the list of valid parent relation types (hierarchical relationships).
pub fn get_parent_relation_types() -> Vec<&'static str> {
    RELATION_TYPES
        .iter()
        .filter(|(_, info)| info.direction == RelationDirection::Backward)
        .map(|(name, _)| *name)
        .collect()
}





#[cfg(test)]
mod tests {
    use super::*;

}

