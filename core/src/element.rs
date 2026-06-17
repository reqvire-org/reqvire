use crate::relation::Relation;
use crate::utils;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;

pub const GOVERNANCE_METADATA_KEYS: &[&str] = &["status", "priority", "risk", "owner"];
pub const GOVERNANCE_STATUS_VALUES: &[&str] = &["draft", "review", "approved"];
pub const GOVERNANCE_PRIORITY_VALUES: &[&str] = &["low", "medium", "high", "critical"];
pub const GOVERNANCE_RISK_VALUES: &[&str] = &["low", "medium", "high", "critical"];

pub fn is_governance_metadata_key(key: &str) -> bool {
    GOVERNANCE_METADATA_KEYS.contains(&key)
}

pub fn is_valid_governance_status(value: &str) -> bool {
    GOVERNANCE_STATUS_VALUES.contains(&value)
}

pub fn is_valid_governance_priority(value: &str) -> bool {
    GOVERNANCE_PRIORITY_VALUES.contains(&value)
}

pub fn is_valid_governance_risk(value: &str) -> bool {
    GOVERNANCE_RISK_VALUES.contains(&value)
}

/// All valid element types that can be used in --filter-type arguments.
/// These values match what ElementType::as_str() returns for each variant.
///
/// MAINTENANCE NOTE: If you add a new ElementType variant, add its string here too.
/// The values must match exactly what ElementType::as_str() returns.
pub const ELEMENT_TYPES: &[&str] = &[
    "capability",
    "requirement",
    "ontology",
    "test-verification",          // VerificationType::Test/Default
    "formal-proof-verification",  // VerificationType::FormalProof
    "analysis-verification",      // VerificationType::Analysis
    "inspection-verification",    // VerificationType::Inspection
    "demonstration-verification", // VerificationType::Demonstration
    "source",                     // RefinementType::Source
    "semantic-contract",          // ElementType::SemanticContract
    "constraint",                 // RefinementType::Constraint
    "behavior",                   // RefinementType::Behavior
    "specification",              // RefinementType::Specification
    "state",                      // RefinementType::State
    "input-output",               // RefinementType::InputOutput
];

/// Element type aliases that are also accepted (mapped to canonical types)
/// These match the aliases in ElementType::from_metadata()
pub const ELEMENT_TYPE_ALIASES: &[&str] = &[
    "system-requirement", // alias for "requirement"
    "verification",       // alias for "test-verification"
];

/// Returns true if the given type string is a valid element type
/// Valid types are:
/// - Standard types (capability, requirement, test-verification, etc.)
/// - Aliases (system-requirement, verification)
/// - Custom types following the pattern "other-TYPENAME" (e.g., other-use-case, other-actor)
pub fn is_valid_element_type(type_str: &str) -> bool {
    let lower = type_str.to_lowercase();
    // Check standard types and aliases
    if ELEMENT_TYPES.contains(&lower.as_str()) || ELEMENT_TYPE_ALIASES.contains(&lower.as_str()) {
        return true;
    }
    // Check custom type pattern: other-TYPENAME
    if lower.starts_with("other-") && lower.len() > 6 {
        return true;
    }
    false
}

/// Helper function to get element types as a comma-separated string for CLI help
pub fn element_types_list() -> String {
    ELEMENT_TYPES.join(", ")
}

/// Helper function to get element types help with custom type explanation
pub fn element_types_help() -> String {
    format!(
        "{}. For custom types use: other-TYPENAME",
        ELEMENT_TYPES.join(", ")
    )
}

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SizeEstimate {
    pub content_bytes: usize,
    pub rendered_context_bytes: usize,
    pub estimated_tokens: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FencedBlock {
    pub language: String,
    pub content: String,
    pub line_number: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SemanticContract {
    pub iri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shapes: Option<FencedBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Ontology {
    pub iri: String,
    pub ontology: Option<FencedBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConceptReference {
    pub label: String,
    pub iri: String,
    pub line_number: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GovernanceMetadataSource {
    Explicit,
    Inherited,
    Default,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GovernanceMetadataEntry {
    pub value: String,
    pub source: GovernanceMetadataSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RequirementGovernanceMetadata {
    pub status: GovernanceMetadataEntry,
    pub priority: GovernanceMetadataEntry,
    pub risk: GovernanceMetadataEntry,
    pub owner: GovernanceMetadataEntry,
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
    ConceptReferences,
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
            SubSection::ConceptReferences => "Concept References",
            SubSection::Other(name) => name.as_str(),
        }
    }

    pub fn parse(s: &str) -> Self {
        match s {
            "Requirement" => SubSection::Requirement,
            "Relations" => SubSection::Relations,
            "Metadata" => SubSection::Metadata,
            "Details" => SubSection::Details,
            "Properties" => SubSection::Properties,
            "Attachments" => SubSection::Attachments,
            "Concept References" => SubSection::ConceptReferences,
            other => SubSection::Other(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum RequirementType {
    System,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum VerificationType {
    Default,
    Test,
    FormalProof,
    Analysis,
    Inspection,
    Demonstration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum RefinementType {
    Source,
    Constraint,
    Behavior,
    Specification,
    State,
    InputOutput,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ElementType {
    Capability,
    Requirement(RequirementType),
    Ontology,
    SemanticContract,
    Verification(VerificationType),
    Refinement(RefinementType),
    File,
    Other(String),
}

impl ElementType {
    /// Returns the metadata key corresponding to this ElementType,
    /// e.g. "requirement", "analysis-verification", or the
    /// raw string for Other.
    ///
    /// Note: for `Other`, this is the unprefixed name (e.g. "open-point").
    /// Use `to_metadata_string` for the canonical form (with the `other-`
    /// prefix) that is written back to Markdown.
    pub fn as_str(&self) -> &str {
        match self {
            ElementType::Capability => "capability",
            ElementType::Requirement(req) => match req {
                RequirementType::System => "requirement",
            },
            ElementType::Ontology => "ontology",
            ElementType::SemanticContract => "semantic-contract",
            ElementType::Verification(ver) => match ver {
                VerificationType::Default => "test-verification",
                VerificationType::Test => "test-verification",
                VerificationType::FormalProof => "formal-proof-verification",
                VerificationType::Analysis => "analysis-verification",
                VerificationType::Inspection => "inspection-verification",
                VerificationType::Demonstration => "demonstration-verification",
            },
            ElementType::Refinement(ref_type) => match ref_type {
                RefinementType::Source => "source",
                RefinementType::Constraint => "constraint",
                RefinementType::Behavior => "behavior",
                RefinementType::Specification => "specification",
                RefinementType::State => "state",
                RefinementType::InputOutput => "input-output",
            },
            ElementType::File => "file",
            ElementType::Other(s) => s.as_str(),
        }
    }

    /// Returns the canonical `type:` metadata string for this element type.
    ///
    /// This is the inverse of `from_metadata`: unlike `as_str`, custom types
    /// keep their `other-` prefix (e.g. "other-open-point"), so the value
    /// round-trips when an element is re-serialized to Markdown and re-validated.
    pub fn to_metadata_string(&self) -> String {
        match self {
            ElementType::Other(custom_type) => format!("other-{}", custom_type),
            _ => self.as_str().to_string(),
        }
    }

    /// Parses a string into an ElementType
    pub fn from_metadata(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "capability" => ElementType::Capability,
            "requirement" | "system-requirement" => {
                ElementType::Requirement(RequirementType::System)
            }
            "ontology" => ElementType::Ontology,

            // Different verification types
            "verification" => ElementType::Verification(VerificationType::Test),
            "test-verification" => ElementType::Verification(VerificationType::Test),
            "formal-proof-verification" => ElementType::Verification(VerificationType::FormalProof),
            "analysis-verification" => ElementType::Verification(VerificationType::Analysis),
            "inspection-verification" => ElementType::Verification(VerificationType::Inspection),
            "demonstration-verification" => {
                ElementType::Verification(VerificationType::Demonstration)
            }

            "semantic-contract" => ElementType::SemanticContract,

            // Refinement types
            "source" => ElementType::Refinement(RefinementType::Source),
            "constraint" => ElementType::Refinement(RefinementType::Constraint),
            "behavior" => ElementType::Refinement(RefinementType::Behavior),
            "specification" => ElementType::Refinement(RefinementType::Specification),
            "state" => ElementType::Refinement(RefinementType::State),
            "input-output" => ElementType::Refinement(RefinementType::InputOutput),

            "file" => ElementType::File,
            other if other.starts_with("other-") && other.len() > 6 => {
                ElementType::Other(other[6..].to_string())
            }
            other => ElementType::Other(other.to_string()),
        }
    }

    /// Returns true if this element type is a requirement-owned refinement type.
    pub fn is_refinement(&self) -> bool {
        matches!(self, ElementType::Refinement(_))
    }

    pub fn is_capability(&self) -> bool {
        matches!(self, ElementType::Capability)
    }

    pub fn is_requirement(&self) -> bool {
        matches!(self, ElementType::Requirement(_))
    }

    pub fn is_ontology(&self) -> bool {
        matches!(self, ElementType::Ontology)
    }

    pub fn is_governance_bearing(&self) -> bool {
        self.is_capability() || self.is_requirement()
    }

    pub fn is_capability_refinement(&self) -> bool {
        false
    }

    pub fn is_requirement_refinement(&self) -> bool {
        matches!(
            self,
            ElementType::Refinement(
                RefinementType::Source
                    | RefinementType::Constraint
                    | RefinementType::Behavior
                    | RefinementType::Specification
                    | RefinementType::State
                    | RefinementType::InputOutput
            )
        )
    }

    pub fn is_semantic_contract(&self) -> bool {
        matches!(self, ElementType::SemanticContract)
    }

    /// Returns the main type category for merge compatibility
    pub fn main_category(&self) -> &'static str {
        match self {
            ElementType::Capability => "capability",
            ElementType::Requirement(_) => "requirement",
            ElementType::Ontology => "ontology",
            ElementType::SemanticContract => "semantic-contract",
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
    //
    // Optional model-build metadata for JSON evidence consumers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_estimate: Option<SizeEstimate>,
    // Parsed ADT for semantic-contract elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_contract: Option<SemanticContract>,
    // Parsed ADT for ontology elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontology: Option<Ontology>,
    // Parsed concept references from human-readable labels to ontology terms.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub concept_references: Vec<ConceptReference>,
}

impl Element {
    pub fn new(
        name: &str,
        identifier: &str,
        file_path: &str,
        line_number: usize,
        element_type: Option<ElementType>,
    ) -> Self {
        // Extract stable ID (fragment) from identifier
        let id = utils::extract_path_and_fragment(identifier)
            .1
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
            size_estimate: None,
            semantic_contract: None,
            ontology: None,
            concept_references: Vec::new(),
        }
    }

    pub fn add_relation(&mut self, relation: Relation) {
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

        self.content = trimmed.to_string();
        self.hash_impact_content = utils::hash_content(&normalized);
        self.populate_ontology();
        self.populate_semantic_contract();
        self.populate_concept_references();
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

    pub fn semantic_contract_iri(&self) -> String {
        format!("urn:reqvire:semantic-contract:{}", self.id)
    }

    pub fn ontology_iri(&self) -> String {
        format!("urn:reqvire:ontology:{}", self.id)
    }

    fn populate_ontology(&mut self) {
        self.ontology = None;
        if !self.element_type.is_ontology() {
            return;
        }

        let ontology = extract_single_fenced_subsection(&self.content, "Ontology");
        if ontology.len() <= 1 && !ontology.is_empty() {
            self.ontology = Some(Ontology {
                iri: self.ontology_iri(),
                ontology: ontology.into_iter().next(),
            });
        }
    }

    fn populate_semantic_contract(&mut self) {
        self.semantic_contract = None;
        if !self.element_type.is_semantic_contract() {
            return;
        }

        let shapes = extract_single_fenced_subsection(&self.content, "Shapes");
        if shapes.len() <= 1 && !shapes.is_empty() {
            self.semantic_contract = Some(SemanticContract {
                iri: self.semantic_contract_iri(),
                shapes: shapes.into_iter().next(),
            });
        }
    }

    fn populate_concept_references(&mut self) {
        self.concept_references = extract_concept_references(&self.content).0;
    }
}

pub fn extract_single_fenced_subsection(content: &str, subsection: &str) -> Vec<FencedBlock> {
    let header = format!("#### {}", subsection);
    let mut blocks = Vec::new();
    let mut in_section = false;
    let mut in_fence = false;
    let mut language = String::new();
    let mut block_content = String::new();
    let mut fence_line_number = 0;

    for (line_index, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("#### ") {
            if in_fence {
                blocks.push(FencedBlock {
                    language: language.clone(),
                    content: block_content.trim_end().to_string(),
                    line_number: fence_line_number,
                });
                in_fence = false;
                language.clear();
                block_content.clear();
                fence_line_number = 0;
            }
            in_section = trimmed == header;
            continue;
        }

        if !in_section {
            continue;
        }

        if trimmed.starts_with("```") {
            if in_fence {
                blocks.push(FencedBlock {
                    language: language.clone(),
                    content: block_content.trim_end().to_string(),
                    line_number: fence_line_number,
                });
                in_fence = false;
                language.clear();
                block_content.clear();
                fence_line_number = 0;
            } else {
                in_fence = true;
                language = trimmed.trim_start_matches("```").trim().to_string();
                fence_line_number = line_index + 1;
            }
            continue;
        }

        if in_fence {
            block_content.push_str(line);
            block_content.push('\n');
        }
    }

    if in_fence {
        blocks.push(FencedBlock {
            language,
            content: block_content.trim_end().to_string(),
            line_number: fence_line_number,
        });
    }

    blocks
}

pub fn has_subsection(content: &str, subsection: &str) -> bool {
    let header = format!("#### {}", subsection);
    content
        .lines()
        .any(|line| line.trim() == header || line.trim().starts_with(&(header.clone() + " ")))
}

pub fn extract_concept_references(content: &str) -> (Vec<ConceptReference>, Vec<String>) {
    let mut references = Vec::new();
    let mut diagnostics = Vec::new();
    let mut in_section = false;

    for (line_index, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("#### ") {
            in_section = trimmed == "#### Concept References";
            continue;
        }

        if !in_section || trimmed.is_empty() {
            continue;
        }

        let Some(entry) = trimmed.strip_prefix("* ") else {
            diagnostics.push(format!(
                "Concept References line {} must use '* Label: IRI_OR_CURIE' syntax.",
                line_index + 1
            ));
            continue;
        };

        let Some((label, iri)) = entry.split_once(':') else {
            diagnostics.push(format!(
                "Concept References line {} must contain a label and IRI separated by ':'.",
                line_index + 1
            ));
            continue;
        };

        let label = label.trim();
        let iri = iri.trim();
        if label.is_empty() || iri.is_empty() {
            diagnostics.push(format!(
                "Concept References line {} must contain a non-empty label and IRI.",
                line_index + 1
            ));
            continue;
        }

        references.push(ConceptReference {
            label: label.to_string(),
            iri: iri.to_string(),
            line_number: line_index + 1,
        });
    }

    (references, diagnostics)
}
