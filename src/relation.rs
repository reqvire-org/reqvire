use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::error::ReqFlowError;
use crate::element;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationDirection {
    Forward,
    Backward,
    Neutral,
}

#[derive(Debug, Clone)]
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
            direction: RelationDirection::Forward, 
            opposite: Some("refinedBy"),
            description: "Element refines a higher-level element",
        });
        
        // Refine relation
        m.insert("refinedBy", RelationTypeInfo {
            name: "refinedBy", 
            direction: RelationDirection::Backward, 
            opposite: Some("refine"),
            description: "Element is source for a element refining it",
        });        
        
        // Satisfy relations
        m.insert("satisfiedBy", RelationTypeInfo {
            name: "satisfiedBy", 
            direction: RelationDirection::Backward, 
            opposite: Some("satisfy"),
            description: "Element is satisfied by another element",
        });
        m.insert("satisfy", RelationTypeInfo {
            name: "satisfy", 
            direction: RelationDirection::Forward, 
            opposite: Some("satisfiedBy"),
            description: "Element satisfies another element",
        });
        
        // Verify relations
        m.insert("verifiedBy", RelationTypeInfo {
            name: "verifiedBy", 
            direction: RelationDirection::Backward, 
            opposite: Some("verify"),
            description: "Element is verified by another element",
        });
        m.insert("verify", RelationTypeInfo {
            name: "verify", 
            direction: RelationDirection::Forward, 
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

#[derive(Debug, Clone)]
pub struct RelationTarget {
    pub text: String,
    pub link: LinkType,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Relation {
    pub relation_type: &'static RelationTypeInfo,
    pub target: RelationTarget,
}

impl Relation {
    pub fn new(relation_type: &str, text: String, normalized_target: &str) -> Result<Self, ReqFlowError> {   
        let link=Self::parse_link_type(normalized_target);
               
        let relation_info = RELATION_TYPES.get(relation_type)
            .ok_or_else(|| ReqFlowError::UnsupportedRelationType(relation_type.to_string()))?;
        Ok(Self {
            relation_type: relation_info,
            target: RelationTarget{text: text, link: link}
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
                        link: LinkType::Identifier(identifier.to_string()),
                    },
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

