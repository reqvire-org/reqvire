use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::identifier::normalize_identifier;
use crate::config::Config;
use crate::error::ReqFlowError;


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
            description: "Element is source for a derived element",
        });
        m.insert("derive", RelationTypeInfo {
            name: "derive", 
            direction: RelationDirection::Forward, 
            opposite: Some("derivedFrom"),
            description: "Element is derived from another element",
        });
        
        // Refine relation
        m.insert("refine", RelationTypeInfo {
            name: "refine", 
            direction: RelationDirection::Forward, 
            opposite: None,
            description: "Element refines a higher-level element",
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
pub struct Relation {
    pub relation_type: &'static RelationTypeInfo,
    pub target: String,
    pub name: String,
}

impl Relation {
    pub fn new(config: &Config, relation_type: &str, raw_target: &str, name: &str) -> Result<Self, ReqFlowError> {
        let relation_info = RELATION_TYPES.get(relation_type)
            .ok_or_else(|| ReqFlowError::UnsupportedRelationType(relation_type.to_string()))?;

        let normalized_target = normalize_identifier(config, "", raw_target)?;

        Ok(Self {
            relation_type: relation_info,
            target: normalized_target,
            name: name.to_string(),
        })
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

/// Determines if a relation type should be checked for circular dependencies.
pub fn is_circular_dependency_relation(relation_type: &str) -> bool {
    match RELATION_TYPES.get(relation_type) {
        Some(info) => matches!(info.name, "derivedFrom" | "dependsOn" | "refine" | "tracedFrom"),
        None => false,
    }
}

