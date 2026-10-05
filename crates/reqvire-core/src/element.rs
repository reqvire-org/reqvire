use crate::relation::Relation;
use crate::utils;
use rustc_hash::FxHashMap;
use serde::Serialize;
use std::path::PathBuf;

pub const GOVERNANCE_METADATA_KEYS: &[&str] = &["status", "priority", "risk", "owner"];
pub const GOVERNANCE_STATUS_VALUES: &[&str] = &["draft", "review", "approved"];
pub const GOVERNANCE_PRIORITY_VALUES: &[&str] = &["low", "medium", "high", "critical"];
pub const GOVERNANCE_RISK_VALUES: &[&str] = &["low", "medium", "high", "critical"];
pub const CONTRACT_BINDINGS_SECTION: &str = "Contract Bindings";
pub const CONTRACT_REFERENCES_SECTION: &str = "Contract References";
// Remove this legacy rejection list when the matching migration paths are removed.
// These names are intentionally invalid in canonical Markdown; they remain named
// only so validation can tell authors to run `reqvire migrate --fix`.
pub const LEGACY_CONTRACT_BINDINGS_SECTION_NAMES: &[&str] =
    &["Attachments", "Reused Contract Context"];

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

pub fn is_contract_bindings_section(value: &str) -> bool {
    value == CONTRACT_BINDINGS_SECTION
}

pub fn is_legacy_contract_bindings_section(value: &str) -> bool {
    LEGACY_CONTRACT_BINDINGS_SECTION_NAMES.contains(&value)
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
    "concept-scheme",
    "concept",
    "test-verification",          // VerificationType::Test/Default
    "formal-proof-verification",  // VerificationType::FormalProof
    "analysis-verification",      // VerificationType::Analysis
    "inspection-verification",    // VerificationType::Inspection
    "demonstration-verification", // VerificationType::Demonstration
    "verification-objective",     // ElementType::VerificationObjective
    "source",                     // ContractType::Source
    "semantic-query",
    "semantic-contract",          // ElementType::SemanticContract
    "constraint",                 // ContractType::Constraint
    "behavior",                   // ContractType::Behavior
    "specification",              // ContractType::Specification
    "state",                      // ContractType::State
    "input-output",               // ContractType::InputOutput
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

/// Represents the target of a contract binding - either a file path or an element identifier
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ContractBindingTarget {
    /// File path contract_bindings (git-root-relative, normalized)
    FilePath(PathBuf),
    /// Element identifier contract_bindings (must point to a Contract element)
    ElementIdentifier(String),
}

impl ContractBindingTarget {
    /// Returns a string representation of the contract_bindings target
    pub fn as_str(&self) -> String {
        match self {
            Self::FilePath(path) => path.to_string_lossy().to_string(),
            Self::ElementIdentifier(id) => id.clone(),
        }
    }

    /// Returns true if this is a file path contract_bindings
    pub const fn is_file_path(&self) -> bool {
        matches!(self, Self::FilePath(_))
    }

    /// Returns true if this is an element identifier contract_bindings
    pub const fn is_element_identifier(&self) -> bool {
        matches!(self, Self::ElementIdentifier(_))
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ContractBindingEntry {
    pub target: ContractBindingTarget,
    /// Content hash for file contract_bindings (FilePath only).
    /// For ElementIdentifier contract_bindings, the hash is looked up from registry.
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
pub struct ConceptLabel {
    pub kind: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConceptLiteral {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConceptLink {
    pub relation: String,
    pub label: String,
    pub target: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConceptScheme {
    pub iri: String,
    pub pref_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub top_concepts: Vec<ConceptLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_prefix: Option<String>,
    pub source_element_identifier: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Concept {
    pub iri: String,
    pub pref_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme_iri: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<ConceptLabel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_note: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<ConceptLiteral>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub broader: Vec<ConceptLink>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub narrower: Vec<ConceptLink>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<ConceptLink>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exact_match: Vec<ConceptLink>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub close_match: Vec<ConceptLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_prefix: Option<String>,
    pub source_element_identifier: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConceptReference {
    pub label: String,
    pub target: String,
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
    ContractBinding,
    ContractReference,
    ConceptReferences,
    ScopeNote,
    Labels,
    Examples,
    Mappings,
    ExternalOntology,
}
impl SubSection {
    pub fn name(&self) -> &str {
        match self {
            Self::Requirement => "Requirement",
            Self::Relations => "Relations",
            Self::Metadata => "Metadata",
            Self::Details => "Details",
            Self::Properties => "Properties",
            Self::ContractBinding => CONTRACT_BINDINGS_SECTION,
            Self::ContractReference => CONTRACT_REFERENCES_SECTION,
            Self::ConceptReferences => "Concept References",
            Self::ScopeNote => "Scope Note",
            Self::Labels => "Labels",
            Self::Examples => "Examples",
            Self::Mappings => "Mappings",
            Self::ExternalOntology => "External Ontology",
            Self::Other(name) => name.as_str(),
        }
    }

    pub const fn is_repeatable(&self) -> bool {
        matches!(self, Self::ExternalOntology)
    }

    pub const fn is_content_bearing(&self) -> bool {
        matches!(
            self,
            Self::Details
                | Self::ConceptReferences
                | Self::ScopeNote
                | Self::Labels
                | Self::Examples
                | Self::Mappings
                | Self::ExternalOntology
                | Self::Other(_)
        )
    }

    pub fn parse(s: &str) -> Self {
        match s {
            "Requirement" => Self::Requirement,
            "Relations" => Self::Relations,
            "Metadata" => Self::Metadata,
            "Details" => Self::Details,
            "Properties" => Self::Properties,
            CONTRACT_BINDINGS_SECTION => Self::ContractBinding,
            CONTRACT_REFERENCES_SECTION => Self::ContractReference,
            "Concept References" => Self::ConceptReferences,
            "Scope Note" => Self::ScopeNote,
            "Labels" => Self::Labels,
            "Examples" => Self::Examples,
            "Mappings" => Self::Mappings,
            "External Ontology" => Self::ExternalOntology,
            other => Self::Other(other.to_string()),
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
pub enum ContractType {
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
    ConceptScheme,
    Concept,
    SemanticContract,
    SemanticQuery,
    VerificationObjective,
    Verification(VerificationType),
    Contract(ContractType),
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
            Self::Capability => "capability",
            Self::Requirement(req) => match req {
                RequirementType::System => "requirement",
            },
            Self::Ontology => "ontology",
            Self::ConceptScheme => "concept-scheme",
            Self::Concept => "concept",
            Self::SemanticQuery => "semantic-query",
            Self::SemanticContract => "semantic-contract",
            Self::VerificationObjective => "verification-objective",
            Self::Verification(ver) => match ver {
                VerificationType::Default => "test-verification",
                VerificationType::Test => "test-verification",
                VerificationType::FormalProof => "formal-proof-verification",
                VerificationType::Analysis => "analysis-verification",
                VerificationType::Inspection => "inspection-verification",
                VerificationType::Demonstration => "demonstration-verification",
            },
            Self::Contract(ref_type) => match ref_type {
                ContractType::Source => "source",
                ContractType::Constraint => "constraint",
                ContractType::Behavior => "behavior",
                ContractType::Specification => "specification",
                ContractType::State => "state",
                ContractType::InputOutput => "input-output",
            },
            Self::File => "file",
            Self::Other(s) => s.as_str(),
        }
    }

    /// Returns the canonical `type:` metadata string for this element type.
    ///
    /// This is the inverse of `from_metadata`: unlike `as_str`, custom types
    /// keep their `other-` prefix (e.g. "other-open-point"), so the value
    /// round-trips when an element is re-serialized to Markdown and re-validated.
    pub fn to_metadata_string(&self) -> String {
        match self {
            Self::Other(custom_type) => format!("other-{}", custom_type),
            _ => self.as_str().to_string(),
        }
    }

    /// Parses a string into an ElementType
    pub fn from_metadata(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "capability" => Self::Capability,
            "requirement" | "system-requirement" => Self::Requirement(RequirementType::System),
            "ontology" => Self::Ontology,
            "concept-scheme" => Self::ConceptScheme,
            "concept" => Self::Concept,
            "verification-objective" => Self::VerificationObjective,

            // Different verification types
            "verification" => Self::Verification(VerificationType::Test),
            "test-verification" => Self::Verification(VerificationType::Test),
            "formal-proof-verification" => Self::Verification(VerificationType::FormalProof),
            "analysis-verification" => Self::Verification(VerificationType::Analysis),
            "inspection-verification" => Self::Verification(VerificationType::Inspection),
            "demonstration-verification" => Self::Verification(VerificationType::Demonstration),

            "semantic-query" => Self::SemanticQuery,
            "semantic-contract" => Self::SemanticContract,

            // Contract types
            "source" => Self::Contract(ContractType::Source),
            "constraint" => Self::Contract(ContractType::Constraint),
            "behavior" => Self::Contract(ContractType::Behavior),
            "specification" => Self::Contract(ContractType::Specification),
            "state" => Self::Contract(ContractType::State),
            "input-output" => Self::Contract(ContractType::InputOutput),

            "file" => Self::File,
            other if other.starts_with("other-") && other.len() > 6 => {
                Self::Other(other[6..].to_string())
            }
            other => Self::Other(other.to_string()),
        }
    }

    /// Returns true if this element type is a requirement-owned contract type.
    pub const fn is_contract(&self) -> bool {
        matches!(self, Self::Contract(_))
    }

    pub const fn is_capability(&self) -> bool {
        matches!(self, Self::Capability)
    }

    pub const fn is_requirement(&self) -> bool {
        matches!(self, Self::Requirement(_))
    }

    pub const fn is_ontology(&self) -> bool {
        matches!(self, Self::Ontology)
    }

    pub const fn is_concept_scheme(&self) -> bool {
        matches!(self, Self::ConceptScheme)
    }

    pub const fn is_concept(&self) -> bool {
        matches!(self, Self::Concept)
    }

    pub const fn is_concept_family(&self) -> bool {
        self.is_concept_scheme() || self.is_concept()
    }

    pub const fn is_governance_bearing(&self) -> bool {
        self.is_capability() || self.is_requirement()
    }

    pub const fn is_capability_contract(&self) -> bool {
        false
    }

    pub const fn is_requirement_contract(&self) -> bool {
        matches!(
            self,
            Self::Contract(
                ContractType::Source
                    | ContractType::Constraint
                    | ContractType::Behavior
                    | ContractType::Specification
                    | ContractType::State
                    | ContractType::InputOutput
            )
        )
    }

    pub const fn is_semantic_query(&self) -> bool { matches!(self, Self::SemanticQuery) }

    pub const fn is_semantic_contract(&self) -> bool {
        matches!(self, Self::SemanticContract)
    }

    pub const fn is_verification_objective(&self) -> bool {
        matches!(self, Self::VerificationObjective)
    }

    /// Returns the main type category for merge compatibility
    pub const fn main_category(&self) -> &'static str {
        match self {
            Self::Capability => "capability",
            Self::Requirement(_) => "requirement",
            Self::Ontology => "ontology",
            Self::ConceptScheme | Self::Concept => "concept",
            Self::SemanticQuery => "semantic-query",
            Self::SemanticContract => "semantic-contract",
            Self::VerificationObjective => "verification-objective",
            Self::Verification(_) => "verification",
            Self::Contract(_) => "contract",
            Self::File => "file",
            Self::Other(_) => "other",
        }
    }

    /// Check if two element types are merge-compatible
    /// Elements are merge-compatible if they belong to the same main type category
    pub fn is_merge_compatible(&self, other: &Self) -> bool {
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
    pub metadata: FxHashMap<String, String>,
    //
    // hash of content that is taken into impact change detection
    pub hash_impact_content: String,
    //
    pub changed_since_commit: bool,
    //
    // Order index within the file (used for preserving original order)
    pub file_order_index: usize,
    //
    // Contract Bindings - external documents linked to this element
    pub contract_bindings: Vec<ContractBindingEntry>,
    /// Content dependencies for review; excluded from implementation fulfillment.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contract_references: Vec<ContractBindingEntry>,
    //
    // Optional model-build metadata for JSON evidence consumers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_estimate: Option<SizeEstimate>,
    // Parsed ADT for semantic-contract elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_contract: Option<SemanticContract>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_query: Option<crate::semantic_contract::queries::QuerySource>,
    #[serde(skip)]
    pub query_line_number: Option<usize>,
    // Parsed ADT for ontology elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontology: Option<Ontology>,
    // Parsed ADT for concept-scheme elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concept_scheme: Option<ConceptScheme>,
    // Parsed ADT for concept elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concept: Option<Concept>,
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
            metadata: FxHashMap::default(),
            changed_since_commit: false,
            file_order_index: 0, // Will be set during parsing
            contract_bindings: vec![],
            contract_references: vec![],
            size_estimate: None,
            semantic_contract: None,
            semantic_query: None,
            query_line_number: None,
            ontology: None,
            concept_scheme: None,
            concept: None,
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
        self.semantic_query = self.element_type.is_semantic_query().then(|| crate::semantic_contract::queries::QuerySource::parse(&self.content));
        if self.element_type.is_semantic_query() { self.hash_impact_content = utils::hash_content(&self.content); }
        self.populate_ontology();
        self.populate_semantic_contract();
        self.populate_concept_scheme();
        self.populate_concept();
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

    pub fn concept_iri(&self) -> String {
        format!("urn:reqvire:concept:{}", self.id)
    }

    fn populate_ontology(&mut self) {
        self.ontology = None;
        if !self.element_type.is_ontology() {
            return;
        }

        let ontology = crate::parser::extract_single_fenced_subsection(&self.content, "Ontology");
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

        let shapes = crate::parser::extract_single_fenced_subsection(&self.content, "Shapes");
        if shapes.len() <= 1 && !shapes.is_empty() {
            self.semantic_contract = Some(SemanticContract {
                iri: self.semantic_contract_iri(),
                shapes: shapes.into_iter().next(),
            });
        }
    }

    fn populate_concept_scheme(&mut self) {
        self.concept_scheme = None;
        if !self.element_type.is_concept_scheme() {
            return;
        }

        self.concept_scheme = Some(ConceptScheme {
            iri: self.concept_iri(),
            pref_label: self.name.clone(),
            definition: extract_main_body_definition(&self.content),
            top_concepts: Vec::new(),
            namespace_base: None,
            namespace_prefix: None,
            source_element_identifier: self.identifier.clone(),
        });
    }

    fn populate_concept(&mut self) {
        self.concept = None;
        if !self.element_type.is_concept() {
            return;
        }

        self.concept = Some(Concept {
            iri: self.concept_iri(),
            pref_label: self.name.clone(),
            definition: extract_main_body_definition(&self.content),
            scheme_iri: None,
            labels: extract_concept_labels(&self.content),
            scope_note: extract_plain_subsection(&self.content, "Scope Note"),
            examples: extract_bullet_literals(&self.content, "Examples"),
            broader: concept_links_from_relations(&self.relations, "broader"),
            narrower: concept_links_from_relations(&self.relations, "narrower"),
            related: concept_links_from_relations(&self.relations, "related"),
            exact_match: merge_concept_links(
                concept_links_from_relations(&self.relations, "exactMatch"),
                extract_mapping_links(&self.content, "exactMatch"),
            ),
            close_match: merge_concept_links(
                concept_links_from_relations(&self.relations, "closeMatch"),
                extract_mapping_links(&self.content, "closeMatch"),
            ),
            namespace_base: None,
            namespace_prefix: None,
            source_element_identifier: self.identifier.clone(),
        });
    }

    fn populate_concept_references(&mut self) {
        self.concept_references = crate::parser::extract_concept_references(&self.content).0;
    }
}

fn extract_main_body_definition(content: &str) -> Option<String> {
    let mut lines = Vec::new();
    for line in content.lines() {
        if line.trim_start().starts_with("#### ") {
            break;
        }
        lines.push(line);
    }
    let definition = lines.join("\n").trim().to_string();
    if definition.is_empty() {
        None
    } else {
        Some(definition)
    }
}

fn extract_plain_subsection(content: &str, subsection: &str) -> Option<String> {
    let lines = collect_subsection_lines(content, subsection);
    let value = lines.join("\n").trim().to_string();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn extract_bullet_literals(content: &str, subsection: &str) -> Vec<ConceptLiteral> {
    collect_subsection_lines(content, subsection)
        .into_iter()
        .filter_map(|line| {
            line.trim()
                .strip_prefix("* ")
                .or_else(|| line.trim().strip_prefix("- "))
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| ConceptLiteral {
                    value: value.to_string(),
                })
        })
        .collect()
}

fn extract_concept_labels(content: &str) -> Vec<ConceptLabel> {
    collect_subsection_lines(content, "Labels")
        .into_iter()
        .filter_map(|line| {
            let entry = line
                .trim()
                .strip_prefix("* ")
                .or_else(|| line.trim().strip_prefix("- "))?;
            let (kind, value) = entry.split_once(':')?;
            let kind = kind.trim();
            let value = value.trim();
            if kind.is_empty() || value.is_empty() {
                None
            } else {
                Some(ConceptLabel {
                    kind: kind.to_string(),
                    value: value.to_string(),
                })
            }
        })
        .collect()
}

fn concept_links_from_relations(relations: &[Relation], relation_name: &str) -> Vec<ConceptLink> {
    relations
        .iter()
        .filter(|relation| relation.relation_type.name == relation_name)
        .map(|relation| ConceptLink {
            relation: relation_name.to_string(),
            label: relation.target.text.clone(),
            target: relation.target.link.as_str().to_string(),
        })
        .collect()
}

fn extract_mapping_links(content: &str, relation_name: &str) -> Vec<ConceptLink> {
    collect_subsection_lines(content, "Mappings")
        .into_iter()
        .filter_map(|line| {
            let entry = line
                .trim()
                .strip_prefix("* ")
                .or_else(|| line.trim().strip_prefix("- "))?;
            let (kind, target) = entry.split_once(':')?;
            if kind.trim() != relation_name {
                return None;
            }
            let target = target
                .trim()
                .trim_start_matches('<')
                .trim_end_matches('>')
                .to_string();
            if target.is_empty() {
                None
            } else {
                Some(ConceptLink {
                    relation: relation_name.to_string(),
                    label: target.clone(),
                    target,
                })
            }
        })
        .collect()
}

fn merge_concept_links(mut left: Vec<ConceptLink>, right: Vec<ConceptLink>) -> Vec<ConceptLink> {
    left.extend(right);
    left
}

fn collect_subsection_lines(content: &str, subsection: &str) -> Vec<String> {
    let header = format!("#### {}", subsection);
    let mut lines = Vec::new();
    let mut in_section = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("#### ") {
            in_section = trimmed == header;
            continue;
        }
        if in_section {
            lines.push(line.to_string());
        }
    }

    lines
}
