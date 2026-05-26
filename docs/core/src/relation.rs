use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::error::ReqvireError;
use serde::Serialize;
use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;
use crate::utils::EXTERNAL_SCHEMES;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct RelationTypeInfo {
    pub name: &'static str,
    pub opposite: Option<&'static str>,
    pub description: &'static str,
    pub arrow: &'static str,
    pub label: &'static str,
}

lazy_static! {
    pub static ref RELATION_TYPES: HashMap<&'static str, RelationTypeInfo> = {
        let mut m = HashMap::new();

        // Derive relations
        m.insert("derivedFrom", RelationTypeInfo {
            name: "derivedFrom",
            opposite: Some("derive"),
            description: "Element is derived from another element",
            arrow: "-.->",
            label: "derivedFrom",
        });
        m.insert("derive", RelationTypeInfo {
            name: "derive",
            opposite: Some("derivedFrom"),
            description: "Element is source for a derived element",
            arrow: "-.->",
            label: "deriveReqT",
        });

        // Satisfy relations
        m.insert("satisfiedBy", RelationTypeInfo {
            name: "satisfiedBy",
            opposite: Some("satisfy"),
            description: "A souce element being satisfied by other element.",
            arrow: "-->",
            label: "satisfiedBy",
        });
        m.insert("satisfy", RelationTypeInfo {
            name: "satisfy",
            opposite: Some("satisfiedBy"),
            description: "Element satisfies another element",
            arrow: "-->",
            label: "satisfies",
        });
        
        // Verify relations
        m.insert("verifiedBy", RelationTypeInfo {
            name: "verifiedBy",
            opposite: Some("verify"),
            description: "A souce element being verified by other element.",
            arrow: "-.->",
            label: "verifiedBy",
        });
        m.insert("verify", RelationTypeInfo {
            name: "verify",
            opposite: Some("verifiedBy"),
            description: "Element verifies another element",
            arrow: "-.->",
            label: "verifies",
        });

        // Trace relations
        m.insert("trace", RelationTypeInfo {
            name: "trace",
            opposite: None,
            description: "Element is related to another element in a non-directional way",
            arrow: "-.->",
            label: "trace",
        });

        m
    };
}

/// Relations to show in diagrams (one from each pair to avoid duplicates)
/// These are typically the "forward" relations from the old direction system
pub const DIAGRAM_RELATIONS: &[&str] = &[
    "derive",        // Not derivedFrom
    "satisfiedBy",   // Not satisfy
    "verifiedBy",    // Not verify
    "trace"
];

/// Relations that propagate changes in impact analysis
/// When these relations exist, changes to the source affect the target
pub const IMPACT_PROPAGATION_RELATIONS: &[&str] = &[
    "derive",        // Source changes affect derived elements
    "satisfiedBy",   // Requirement changes affect implementations
    "verifiedBy",    // Requirement changes invalidate verifications
];

/// Backward relations for reverse model traversal (opposite of DIAGRAM_RELATIONS)
/// These traverse from leaves upward to roots
pub const BACKWARD_RELATIONS: &[&str] = &[
    "derivedFrom",   // Opposite of derive
    "satisfy",       // Opposite of satisfiedBy
    "verify",        // Opposite of verifiedBy
];


/// Relation type for verification
pub const VERIFY_RELATION: &str = "verify";

/// Relations for satisfaction connections (refinement-to-requirement ownership)
/// Used to determine if refinements are connected and find defining requirements
pub const SATISFACTION_RELATIONS: &[&str] = &[
    "satisfy",       // Refinement satisfies requirement (forward from refinement)
    "satisfiedBy",   // Requirement satisfied by refinement (forward from requirement)
];

/// Relations that trace verification propagation in verification traces
pub const VERIFICATION_TRACES_RELATIONS: &[&str] = &[
    "derivedFrom",
];

#[derive(Debug, Clone, Serialize)]
pub struct RelationTarget {
    pub text: String,
    pub link: LinkType,
    /// Stable Element ID (fragment) for Identifier links, None for external/path links
    /// This is the globally unique, location-independent identifier used for change detection
    pub element_id: Option<String>,
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
    InternalPath(PathBuf), // Internal Path, e.g., "../core/src/digrams.rs"    
}
impl LinkType {
    /// Converts `LinkType` into a string representation.
    pub fn as_str(&self) -> &str {
        match self {
            LinkType::Identifier(id) => id,
            LinkType::ExternalUrl(url) => url,
            LinkType::InternalPath(path) =>  path.to_str()
                    .expect(&format!("InternalPath is not valid UTF-8: {:?}", path))
        }
    }
}



#[derive(Debug, Clone, Serialize)]
pub struct Relation {
    pub relation_type: &'static RelationTypeInfo,
    pub target: RelationTarget,
    pub user_created: bool
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
        self.user_created.hash(state);
    }
}

impl Relation {
    pub fn new(relation_type: &str, text: String, normalized_target: &str, element_id: Option<String>) -> Result<Self, ReqvireError> {
        let link=Self::parse_link_type(normalized_target);

        let relation_info = RELATION_TYPES.get(relation_type)
            .ok_or_else(|| ReqvireError::UnsupportedRelationType(relation_type.to_string()))?;
        Ok(Self {
            relation_type: relation_info,
            target: RelationTarget{text: text, link: link, element_id},
            user_created: true,  // Relations created via parsing are user-created
        })
    }
    
    /// Determines if the link should be treated as an identifier, internal path or an external URL.
    fn parse_link_type(link: &str) -> LinkType {
        if EXTERNAL_SCHEMES.iter().any(|scheme| link.starts_with(scheme)) {
            LinkType::ExternalUrl(link.to_string())
        } else if link.contains('#') {
            LinkType::Identifier(link.to_string())
        } else {
            LinkType::InternalPath(PathBuf::from(link))
        }
    }    

    pub fn update_target_identifier_link_url(&mut self, url: &str)  {
        match self.target.link {
            LinkType::Identifier(_) =>  self.target.link=LinkType::Identifier(url.to_string()),
            _ =>{}
        };  
    }


    /// Creates an opposite relation if possible for given target
    pub fn to_opposite(&self, name: &str, identifier: &str, element_id: &str) -> Option<Relation> {
        if let Some(opposite_name) = self.relation_type.opposite {
            match RELATION_TYPES.get(opposite_name) {
                Some(opposite_info) => {
                    Some(Relation {
                        relation_type: opposite_info,
                        target: RelationTarget {
                            text: name.to_string(),
                            link: LinkType::Identifier(identifier.to_string()),
                            element_id: Some(element_id.to_string()),
                        },
                        user_created: false,  // Auto-generated opposite relations are not user-created
                    })
                }
                None => {
                    None
                }
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

/// Get a formatted string of all supported relation types for error messages
pub fn supported_relation_types_list() -> String {
    let mut types: Vec<&str> = RELATION_TYPES.keys().cloned().collect();
    types.sort();
    types.join(", ")
}

/// Get the list of general parent relation types (backward dependencies).
/// These are the "backward" pointing relations where an element refers to something it depends on.
/// Includes hierarchical (derivedFrom), satisfaction (satisfy), and verification (verify) parents.
pub fn get_parent_relation_types() -> Vec<&'static str> {
    vec!["derivedFrom", "satisfy", "verify"]
}

/// Get the list of hierarchical relation types only.
/// These define the derivation hierarchy in the model (parent-child relationships).
/// Per specifications, only derivedFrom is hierarchical.
pub fn get_hierarchical_relation_types() -> Vec<&'static str> {
    vec!["derivedFrom"]
}

/// Returns whether the relation is a verification-related type
pub fn is_verification_relation(rtype: &RelationTypeInfo) -> bool {
    matches!(rtype.name, "verifiedBy" | "verify")
}

/// Returns whether the relation is a satisfaction-related type
pub fn is_satisfaction_relation(rtype: &RelationTypeInfo) -> bool {
    matches!(rtype.name, "satisfiedBy" | "satisfy")
}




/// Validates if the element types are appropriate for a given relation type
/// Returns true if the types are compatible, false otherwise
///
/// Element Type Relation Compatibility Matrix:
/// - derivedFrom/derive: Capability, requirement, and ontology hierarchies can use these within the same family
/// - verifiedBy: Source must be capability or requirement, target must be verification
/// - verify: Source must be verification, target must be capability or requirement
/// - satisfiedBy: Source must be requirement or test-verification, target must be file
/// - satisfy: Inverse of satisfiedBy (auto-generated)
/// - trace: Any non-refinement element type can use trace
/// - Refinement types (constraint, behavior, specification): Can only have satisfy relations
/// - Other type: Can only use trace relations
pub fn validate_relation_element_types(
    relation_type: &str,
    source_type: &crate::element::ElementType,
    target_type: &crate::element::ElementType
) -> bool {
    use crate::element::ElementType;

    // First check: source element type restrictions based on relation type
    // Refinement types cannot have ANY relations (this is checked elsewhere in parser)
    // Other type can only use trace relations
    if let ElementType::Other(type_str) = source_type {
        // "other" type (explicit) can only use trace
        if type_str == "other" && relation_type != "trace" {
            return false;
        }
    }

    match relation_type {
        "derivedFrom" => {
            // Capability, requirement, or ontology hierarchy families can use derivedFrom
            // Source must be requirement, target must be capability or requirement
            matches!(source_type, ElementType::Requirement(_)) &&
            matches!(target_type, ElementType::Requirement(_))
        },
        "derive" => {
            // Capability, requirement, or ontology hierarchy families can use derive
            // Source must be requirement, target must be capability or requirement
            matches!(source_type, ElementType::Requirement(_)) &&
            matches!(target_type, ElementType::Requirement(_))
        },
        "verifiedBy" => {
            // Source must be a requirement and target must be a verification
            matches!(source_type, ElementType::Requirement(_)) &&
            matches!(target_type, ElementType::Verification(_))
        },
        "verify" => {
            // Source must be a verification and target must be a requirement
            matches!(source_type, ElementType::Verification(_)) &&
            matches!(target_type, ElementType::Requirement(_))
        },
        "satisfiedBy" => {
            // Source must be requirement or test-verification
            // Target can be a file (implementation) or refinement element (constraint, behavior, specification)
            // Note: non-test-verification satisfiedBy is checked separately in graph_registry
            let source_valid = match source_type {
                ElementType::Requirement(_) => true,
                ElementType::Verification(vtype) => {
                    matches!(vtype, crate::element::VerificationType::Default | crate::element::VerificationType::Test)
                },
                _ => false
            };
            // For target, we allow File type, Other (for implementation files), or Refinement types
            let target_valid = matches!(target_type, ElementType::File | ElementType::Other(_) | ElementType::Refinement(_));
            source_valid && target_valid
        },
        "satisfy" => {
            // Source should be a file/implementation or refinement, target should be a requirement or test-verification
            // Refinement elements can explicitly define satisfy relations to requirements
            let source_valid = matches!(source_type, ElementType::File | ElementType::Other(_) | ElementType::Refinement(_));
            let target_valid = match target_type {
                ElementType::Requirement(_) => true,
                ElementType::Verification(vtype) => {
                    matches!(vtype, crate::element::VerificationType::Default | crate::element::VerificationType::Test)
                },
                _ => false
            };
            source_valid && target_valid
        },
        "trace" => {
            // Trace is allowed for any non-refinement element type
            // Refinement types cannot have relations at all (checked in parser)
            !matches!(source_type, ElementType::Refinement(_))
        },
        // For other relation types, no specific element type validation
        _ => true
    }
}

/// Gets a detailed description of the expected element types for a relation
/// Returns None if the relation type has no specific type restrictions
pub fn get_relation_element_type_description(relation_type: &str) -> Option<String> {
    match relation_type {
        "derivedFrom" => Some("'derivedFrom' can only be used within capability, requirement, or ontology hierarchy families".to_string()),
        "derive" => Some("'derive' can only be used within capability, requirement, or ontology hierarchy families".to_string()),
        "verifiedBy" => Some("'verifiedBy' should connect a capability or requirement to a verification element".to_string()),
        "verify" => Some("'verify' should connect a verification element to a capability or requirement".to_string()),
        "satisfiedBy" => Some("'satisfiedBy' should connect a requirement or test-verification to an implementation file or refinement element".to_string()),
        "satisfy" => Some("'satisfy' should connect an implementation file or refinement element to a requirement or test-verification".to_string()),
        "trace" => Some("'trace' can be used by any element type except refinement types".to_string()),
        _ => None
    }
}

