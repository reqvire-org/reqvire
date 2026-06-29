use crate::element::{
    ConceptReference, ContractBindingTarget, ContractType, Element, ElementType, FencedBlock,
    VerificationType, GOVERNANCE_METADATA_KEYS,
};
use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;
use crate::relation::{self, LinkType};
use o_kernel::diagnostics::is_false;
use o_kernel::{constructs, ontology, shacl, vocab::*};
use oxigraph::io::{JsonLdProfileSet, RdfFormat, RdfParser, RdfSerializer};
use oxigraph::model::{Quad, Triple};
use oxigraph::store::Store;
use rustc_hash::FxHashMap;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};

const GRAPH_AUTHORED_ONTOLOGY: &str = "urn:reqvire:semantic-graph:authored-ontology";
const GRAPH_AUTHORED_MODEL: &str = "urn:reqvire:semantic-graph:authored-model";
const GRAPH_GENERATED: &str = "urn:reqvire:semantic-graph:generated";
const GRAPH_RAW_EXTERNAL_SOURCE: &str = "urn:reqvire:semantic-graph:raw-external-source";
const REQVIRE_NS: &str = "https://www.reqvire.org/ontology#";
pub(crate) const REQVIRE_MAPS_TO_CONCEPT: &str = "https://www.reqvire.org/ontology#mapsToConcept";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OntologyTermRole {
    Class,
    Datatype,
    Property,
    AnnotationProperty,
    ObjectProperty,
    DatatypeProperty,
    NamedIndividual,
}

impl OntologyTermRole {
    pub fn conflicts_with(self, other: Self) -> bool {
        matches!(
            (self, other),
            (
                Self::Class,
                Self::Property
                    | Self::AnnotationProperty
                    | Self::ObjectProperty
                    | Self::DatatypeProperty
            ) | (
                Self::Property
                    | Self::AnnotationProperty
                    | Self::ObjectProperty
                    | Self::DatatypeProperty,
                Self::Class
            ) | (Self::ObjectProperty, Self::DatatypeProperty)
                | (Self::DatatypeProperty, Self::ObjectProperty)
                | (
                    Self::AnnotationProperty,
                    Self::ObjectProperty | Self::DatatypeProperty
                )
                | (
                    Self::ObjectProperty | Self::DatatypeProperty,
                    Self::AnnotationProperty
                )
        )
    }
}

impl fmt::Display for OntologyTermRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Class => write!(f, "class"),
            Self::Datatype => write!(f, "datatype"),
            Self::Property => write!(f, "property"),
            Self::AnnotationProperty => write!(f, "annotation-property"),
            Self::ObjectProperty => write!(f, "object-property"),
            Self::DatatypeProperty => write!(f, "datatype-property"),
            Self::NamedIndividual => write!(f, "named-individual"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OntologyTermDeclaration {
    pub iri: String,
    pub role: OntologyTermRole,
    pub element_identifier: String,
    pub external: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub materialized_in_used_subset: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct OntologyDocumentDeclaration {
    pub iri: String,
    pub ontology_base: String,
    pub ontology_prefix: String,
    pub term_namespace: String,
    pub element_identifiers: Vec<String>,
    pub element_names: Vec<String>,
    pub imports: Vec<String>,
}

#[derive(Debug, Clone)]
struct OntologyDocumentAccumulator {
    ontology_base: String,
    ontology_prefix: String,
    term_namespace: String,
    element_identifiers: BTreeSet<String>,
    element_names: BTreeSet<String>,
    imports: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct ShapeIriReference {
    pub iri: String,
    pub kind: String,
    pub element_identifier: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SemanticBlockKind {
    Ontology,
    Shapes,
    Concepts,
    ExternalOntology,
}

impl SemanticBlockKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ontology => "ontology",
            Self::Shapes => "shapes",
            Self::Concepts => "concepts",
            Self::ExternalOntology => "external-ontology",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SemanticBlock {
    pub kind: SemanticBlockKind,
    pub source: String,
    pub source_name: String,
    pub file_path: String,
    pub line_number: usize,
    pub language: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_materialization: Option<String>,
    pub content: String,
    #[serde(skip)]
    pub quads: Vec<Quad>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SemanticDiagnostic {
    pub source: String,
    pub file_path: String,
    pub line_number: usize,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExternalOntologySource {
    pub owner_identifier: String,
    pub owner_name: String,
    pub prefix: String,
    pub namespace: String,
    pub resource: Option<String>,
    pub source: String,
    pub format: String,
    pub line_number: usize,
    #[serde(skip_serializing_if = "is_false")]
    pub builtin: bool,
}

pub struct UsedOntologyProjectNamespaces {
    by_prefix: BTreeMap<String, BTreeSet<String>>,
    prefix_by_namespace: BTreeMap<String, BTreeSet<String>>,
}

impl UsedOntologyProjectNamespaces {
    pub fn is_empty(&self) -> bool {
        self.by_prefix.is_empty()
    }

    pub fn namespaces_for_iri(&self, iri: &str) -> Vec<&str> {
        self.prefix_by_namespace
            .keys()
            .filter(|namespace| iri.starts_with(namespace.as_str()))
            .map(String::as_str)
            .collect()
    }

    pub fn prefix_for_namespace(&self, namespace: &str) -> Option<&str> {
        self.prefix_by_namespace
            .get(namespace)?
            .iter()
            .next()
            .map(String::as_str)
    }

    pub fn namespaces_for_prefix(&self, prefix: &str) -> Option<&BTreeSet<String>> {
        self.by_prefix.get(prefix)
    }
}

#[derive(Debug, Clone)]
struct CanonicalOntologyPrefix {
    prefix: String,
    namespace: String,
    ontology_base: String,
    ontology_document_iri: String,
    explicit_boundary: bool,
    required_imports: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SemanticIndexSummary {
    pub ontology_blocks: usize,
    pub shape_blocks: usize,
    pub total_blocks: usize,
    pub total_quads: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OntologyProjectionDerivationMode {
    DirectAuthored,
}

impl OntologyProjectionDerivationMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::DirectAuthored => "direct-authored",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OntologyConstructFamily {
    PropertyDomainRange,
    SubclassMembership,
    DisjointEquivalenceInverse,
    PropertyChain,
    PropertyCharacteristic,
    Restriction,
    ClassExpression,
    ShapeOverlay,
}

impl OntologyConstructFamily {
    fn as_str(self) -> &'static str {
        match self {
            Self::PropertyDomainRange => "property-domain-range",
            Self::SubclassMembership => "subclass-membership",
            Self::DisjointEquivalenceInverse => "disjoint-equivalence-inverse",
            Self::PropertyChain => "property-chain",
            Self::PropertyCharacteristic => "property-characteristic",
            Self::Restriction => "restriction",
            Self::ClassExpression => "class-expression",
            Self::ShapeOverlay => "shape-overlay",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OntologyConstructKind {
    PropertyDomain,
    PropertyRange,
    SubclassInclusion,
    Membership,
    Disjointness,
    EquivalenceGroup,
    InverseProperty,
    PropertyChain,
    PropertyCharacteristic,
    Restriction,
    ClassExpression,
    ShapeOverlay,
}

impl OntologyConstructKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::PropertyDomain => "property-domain",
            Self::PropertyRange => "property-range",
            Self::SubclassInclusion => "subclass-inclusion",
            Self::Membership => "membership",
            Self::Disjointness => "disjointness",
            Self::EquivalenceGroup => "equivalence-group",
            Self::InverseProperty => "inverse-property",
            Self::PropertyChain => "property-chain",
            Self::PropertyCharacteristic => "property-characteristic",
            Self::Restriction => "restriction",
            Self::ClassExpression => "class-expression",
            Self::ShapeOverlay => "shape-overlay",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OntologyProjectionTermKind {
    Iri,
    BlankNode,
    Literal,
}

impl OntologyProjectionTermKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Iri => "iri",
            Self::BlankNode => "blank-node",
            Self::Literal => "literal",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct OntologyProjectionTerm {
    pub kind: OntologyProjectionTermKind,
    pub value: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct OntologyProjectionSource {
    pub source_block: String,
    pub source_element_identifier: String,
    pub source_name: String,
    pub file_path: String,
    pub line_number: usize,
    pub block_kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct OntologyProjectionEvidence {
    pub source: OntologyProjectionSource,
    pub subject: OntologyProjectionTerm,
    pub predicate: OntologyProjectionTerm,
    pub object: OntologyProjectionTerm,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OntologyProjectionProvenance {
    pub derivation_mode: OntologyProjectionDerivationMode,
    pub source: OntologyProjectionSource,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<OntologyProjectionEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct OntologyConstructMember {
    pub sequence_index: usize,
    pub term: OntologyProjectionTerm,
    pub source: OntologyProjectionSource,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct OntologySymbol {
    pub concept_name: String,
    pub raw_unicode_code_point: String,
    pub rendered_unicode_character: String,
    pub tooltip: String,
    pub accessible_label: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OntologyPropertyCharacteristic {
    Functional,
    InverseFunctional,
    Symmetric,
    Asymmetric,
    Reflexive,
    Irreflexive,
    Transitive,
}

impl OntologyPropertyCharacteristic {
    fn as_str(self) -> &'static str {
        match self {
            Self::Functional => "functional",
            Self::InverseFunctional => "inverse-functional",
            Self::Symmetric => "symmetric",
            Self::Asymmetric => "asymmetric",
            Self::Reflexive => "reflexive",
            Self::Irreflexive => "irreflexive",
            Self::Transitive => "transitive",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OntologyRestrictionKind {
    Universal,
    Existential,
    HasValue,
    Cardinality,
    MinCardinality,
    MaxCardinality,
    QualifiedCardinality,
    MinQualifiedCardinality,
    MaxQualifiedCardinality,
    OnClass,
    OnDataRange,
}

impl OntologyRestrictionKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Universal => "universal",
            Self::Existential => "existential",
            Self::HasValue => "has-value",
            Self::Cardinality => "cardinality",
            Self::MinCardinality => "min-cardinality",
            Self::MaxCardinality => "max-cardinality",
            Self::QualifiedCardinality => "qualified-cardinality",
            Self::MinQualifiedCardinality => "min-qualified-cardinality",
            Self::MaxQualifiedCardinality => "max-qualified-cardinality",
            Self::OnClass => "on-class",
            Self::OnDataRange => "on-data-range",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OntologyClassExpressionKind {
    Intersection,
    Union,
    Complement,
}

impl OntologyClassExpressionKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Intersection => "intersection",
            Self::Union => "union",
            Self::Complement => "complement",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OntologyShapeOverlayKind {
    NodeShape,
    PropertyShape,
}

impl OntologyShapeOverlayKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::NodeShape => "node-shape",
            Self::PropertyShape => "property-shape",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OntologyConstruct {
    pub id: String,
    pub family: OntologyConstructFamily,
    pub kind: OntologyConstructKind,
    pub subject: OntologyProjectionTerm,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<OntologyProjectionTerm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<OntologyProjectionTerm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OntologyProjectionTerm>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<OntologyConstructMember>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_characteristic: Option<OntologyPropertyCharacteristic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restriction_kind: Option<OntologyRestrictionKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_expression_kind: Option<OntologyClassExpressionKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape_overlay_kind: Option<OntologyShapeOverlayKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<OntologySymbol>,
    pub provenance: OntologyProjectionProvenance,
}

#[derive(Debug, Clone, Serialize)]
pub struct OntologyConstructProjection {
    pub id: String,
    pub family: OntologyConstructFamily,
    pub derivation_mode: OntologyProjectionDerivationMode,
    pub construct_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OntologyProjectionGraph {
    pub id: String,
    pub derivation_mode: OntologyProjectionDerivationMode,
    pub projections: Vec<OntologyConstructProjection>,
    pub constructs: Vec<OntologyConstruct>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub symbols: Vec<OntologySymbol>,
}

impl OntologyProjectionGraph {
    pub fn is_empty(&self) -> bool {
        self.constructs.is_empty()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ModelContextGraph {
    pub nodes: Vec<ModelContextNode>,
    pub edges: Vec<ModelContextEdge>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ModelContextNode {
    pub id: String,
    pub label: String,
    pub identifier: String,
    pub element_type: String,
    pub rdf_types: Vec<String>,
    pub file_path: String,
    pub line_number: usize,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModelContextEdge {
    pub source: String,
    pub target: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SemanticIndex {
    pub blocks: Vec<SemanticBlock>,
    pub external_blocks: Vec<SemanticBlock>,
    pub external_sources: Vec<ExternalOntologySource>,
    pub diagnostics: Vec<SemanticDiagnostic>,
    pub ontology_documents: Vec<OntologyDocumentDeclaration>,
    pub ontology_declarations: FxHashMap<String, Vec<OntologyTermDeclaration>>,
    pub shape_references: Vec<ShapeIriReference>,
    pub ontology_projection: OntologyProjectionGraph,
    pub model_context: ModelContextGraph,
    #[serde(skip)]
    pub model_context_turtle: String,
    pub summary: SemanticIndexSummary,
}

#[derive(Debug, Clone, Copy)]
pub enum SemanticExportFormat {
    Turtle,
    JsonLd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SemanticExportLayer {
    Ontologies,
    Shapes,
    Concepts,
    Model,
    ExternalUsed,
    Prefixes,
}

impl SemanticExportLayer {
    pub fn default_layers() -> Vec<Self> {
        vec![
            Self::Ontologies,
            Self::Shapes,
            Self::Concepts,
            Self::Model,
            Self::ExternalUsed,
            Self::Prefixes,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ontologies => "ontologies",
            Self::Shapes => "shapes",
            Self::Concepts => "concepts",
            Self::Model => "model",
            Self::ExternalUsed => "external-used",
            Self::Prefixes => "prefixes",
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ExternalOntologyFormat {
    Turtle,
    RdfXml,
    JsonLd,
}

impl ExternalOntologyFormat {
    fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "turtle" | "ttl" => Some(Self::Turtle),
            "rdf" | "rdfxml" | "rdf-xml" | "rdf_xml" | "rdf+xml" => Some(Self::RdfXml),
            "jsonld" | "json-ld" | "json_ld" => Some(Self::JsonLd),
            _ => None,
        }
    }

    fn display_name(self) -> &'static str {
        match self {
            Self::Turtle => "Turtle",
            Self::RdfXml => "RDF/XML",
            Self::JsonLd => "JSON-LD",
        }
    }

    fn language(self) -> &'static str {
        match self {
            Self::Turtle => "turtle",
            Self::RdfXml => "rdfxml",
            Self::JsonLd => "jsonld",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct TurtlePrefixDeclaration {
    prefix: String,
    namespace: String,
    source_rank: u8,
}

#[derive(Debug, Clone)]
struct TurtlePrefixMap {
    declarations: Vec<TurtlePrefixDeclaration>,
}

struct TurtlePrefixMapBuilder {
    by_prefix: BTreeMap<String, TurtlePrefixDeclaration>,
    prefix_by_namespace: BTreeMap<String, String>,
}

impl TurtlePrefixMapBuilder {
    fn new() -> Self {
        Self {
            by_prefix: BTreeMap::new(),
            prefix_by_namespace: BTreeMap::new(),
        }
    }

    fn add(&mut self, prefix: &str, namespace: &str, source_rank: u8) -> Result<(), ReqvireError> {
        let prefix = prefix.trim();
        let namespace = namespace.trim();
        if prefix.is_empty() || namespace.is_empty() {
            return Ok(());
        }

        if let Some(existing) = self.by_prefix.get_mut(prefix) {
            if existing.namespace != namespace {
                return Err(ReqvireError::ProcessError(format!(
                    "Turtle prefix '{}' is bound to both '{}' and '{}'.",
                    prefix, existing.namespace, namespace
                )));
            }
            return Ok(());
        }

        if let Some(existing_prefix) = self.prefix_by_namespace.get(namespace) {
            if existing_prefix != prefix {
                return Err(ReqvireError::ProcessError(format!(
                    "Turtle namespace '{}' is bound to both prefix '{}' and prefix '{}'. Prefix aliases are not supported.",
                    namespace, existing_prefix, prefix
                )));
            }
        }

        self.by_prefix.insert(
            prefix.to_string(),
            TurtlePrefixDeclaration {
                prefix: prefix.to_string(),
                namespace: namespace.to_string(),
                source_rank,
            },
        );
        self.prefix_by_namespace
            .insert(namespace.to_string(), prefix.to_string());
        Ok(())
    }

    fn add_reqvire_fallback_if_missing(&mut self) -> Result<(), ReqvireError> {
        self.add("reqvire", REQVIRE_NS, 0)
    }

    fn add_local(&mut self, prefix: &str, namespace: &str, source_rank: u8) {
        let prefix = prefix.trim();
        let namespace = namespace.trim();
        if prefix.is_empty() || namespace.is_empty() {
            return;
        }

        if let Some(existing) = self.by_prefix.get(prefix) {
            if existing.namespace != namespace {
                return;
            }
            return;
        }

        if self.prefix_by_namespace.contains_key(namespace) {
            return;
        }

        self.by_prefix.insert(
            prefix.to_string(),
            TurtlePrefixDeclaration {
                prefix: prefix.to_string(),
                namespace: namespace.to_string(),
                source_rank,
            },
        );
        self.prefix_by_namespace
            .insert(namespace.to_string(), prefix.to_string());
    }

    fn finish(self) -> TurtlePrefixMap {
        let mut declarations = self.by_prefix.into_values().collect::<Vec<_>>();
        declarations.sort_by(|a, b| {
            turtle_prefix_sort_group(a)
                .cmp(&turtle_prefix_sort_group(b))
                .then_with(|| builtin_prefix_order(&a.prefix).cmp(&builtin_prefix_order(&b.prefix)))
                .then_with(|| a.prefix.cmp(&b.prefix))
                .then_with(|| a.namespace.cmp(&b.namespace))
        });
        TurtlePrefixMap { declarations }
    }
}

fn turtle_prefix_sort_group(declaration: &TurtlePrefixDeclaration) -> u8 {
    if declaration.prefix == "reqvire" && declaration.namespace == REQVIRE_NS {
        return 0;
    }

    match declaration.source_rank {
        0 => 1,
        10 => 2,
        20 => 3,
        30 => 4,
        _ => 5,
    }
}

fn builtin_prefix_order(prefix: &str) -> u8 {
    match prefix {
        "rdf" => 0,
        "rdfs" => 1,
        "owl" => 2,
        "xsd" => 3,
        "sh" => 4,
        "skos" => 5,
        _ => u8::MAX,
    }
}

impl TurtlePrefixMap {
    fn to_turtle_block(&self) -> String {
        if self.declarations.is_empty() {
            return String::new();
        }
        let mut output = String::new();
        for declaration in &self.declarations {
            output.push_str(&format!(
                "@prefix {}: <{}> .\n",
                declaration.prefix,
                escape_iri(&declaration.namespace)
            ));
        }
        output.push('\n');
        output
    }

    fn serializer(&self) -> Result<RdfSerializer, ReqvireError> {
        let mut serializer = RdfSerializer::from_format(RdfFormat::Turtle);
        for declaration in &self.declarations {
            serializer = serializer
                .with_prefix(declaration.prefix.as_str(), declaration.namespace.as_str())
                .map_err(|error| {
                    ReqvireError::SerializationError(format!(
                        "Invalid Turtle prefix '{}: <{}>': {}",
                        declaration.prefix, declaration.namespace, error
                    ))
                })?;
        }
        Ok(serializer)
    }
}

impl SemanticIndex {
    pub fn with_namespace_base_filter(
        &self,
        namespace_base: Option<&str>,
    ) -> Result<Self, ReqvireError> {
        let Some(namespace_base) = namespace_base
            .map(str::trim)
            .filter(|value| !value.is_empty())
        else {
            return Ok(self.clone());
        };

        let namespace = export_filter_term_namespace(namespace_base, &self.ontology_documents);
        let mut index = self.clone();

        index
            .ontology_documents
            .retain(|document| document.term_namespace == namespace);
        let retained_sources: BTreeSet<String> = index
            .ontology_documents
            .iter()
            .flat_map(|document| document.element_identifiers.iter().cloned())
            .collect();

        index.blocks.retain(|block| match block.kind {
            SemanticBlockKind::Ontology => {
                retained_sources.contains(&block.source)
                    || block_declares_subject_in_namespace(block, &namespace)
            }
            SemanticBlockKind::Concepts => block_declares_subject_in_namespace(block, &namespace),
            SemanticBlockKind::Shapes => block_declares_subject_in_namespace(block, &namespace),
            SemanticBlockKind::ExternalOntology => false,
        });

        index.ontology_declarations.retain(|iri, declarations| {
            declarations.retain(|declaration| {
                !declaration.external
                    && iri.starts_with(&namespace)
                    && retained_sources.contains(&declaration.element_identifier)
            });
            !declarations.is_empty()
        });
        index
            .shape_references
            .retain(|reference| reference.iri.starts_with(&namespace));
        index.summary = SemanticIndexSummary {
            ontology_blocks: index
                .blocks
                .iter()
                .filter(|block| matches!(block.kind, SemanticBlockKind::Ontology))
                .count(),
            shape_blocks: index
                .blocks
                .iter()
                .filter(|block| matches!(block.kind, SemanticBlockKind::Shapes))
                .count(),
            total_blocks: index.blocks.len(),
            total_quads: index.blocks.iter().map(|block| block.quads.len()).sum(),
        };

        Ok(index)
    }

    pub fn with_external_visibility(&self, include_external: bool) -> Result<Self, ReqvireError> {
        let mut index = self.clone();
        index.apply_external_visibility(include_external)?;
        Ok(index)
    }

    pub fn apply_external_visibility(
        &mut self,
        include_external: bool,
    ) -> Result<(), ReqvireError> {
        if include_external {
            let used_subset_block = self.used_external_subset_block()?;
            let used_terms = used_subset_block
                .as_ref()
                .map(materialized_external_subjects)
                .unwrap_or_default();
            self.external_blocks = used_subset_block.into_iter().collect();
            self.external_sources.retain(|source| {
                used_terms
                    .iter()
                    .any(|term| term.starts_with(&source.namespace))
            });
            self.ontology_declarations.retain(|_iri, declarations| {
                declarations.retain_mut(|declaration| {
                    if !declaration.external {
                        return true;
                    }
                    let materialized = used_terms.contains(&declaration.iri);
                    declaration.materialized_in_used_subset = materialized;
                    materialized
                });
                !declarations.is_empty()
            });
            return Ok(());
        }

        self.external_blocks.clear();
        self.external_sources.clear();
        self.ontology_declarations.retain(|_iri, declarations| {
            declarations.retain(|declaration| !declaration.external);
            !declarations.is_empty()
        });
        Ok(())
    }

    pub fn reachable_ontology_context_quads(
        &self,
        ontology_context: &BTreeSet<String>,
    ) -> Vec<Quad> {
        let external_block_sources: BTreeSet<String> = self
            .external_sources
            .iter()
            .filter(|source| source.builtin || ontology_context.contains(&source.owner_identifier))
            .map(external_ontology_block_source)
            .collect();

        let mut quads = Vec::new();
        for block in &self.blocks {
            if matches!(block.kind, SemanticBlockKind::Ontology)
                && ontology_context.contains(&block.source)
            {
                quads.extend(block.quads.iter().cloned());
            }
        }
        for block in &self.external_blocks {
            if external_block_sources.contains(&block.source) {
                quads.extend(block.quads.iter().cloned());
            }
        }

        quads
    }

    pub fn is_skos_concept_iri(&self, iri: &str) -> bool {
        self.concept_layer_subject_has_type(iri, SKOS_CONCEPT)
    }

    pub fn concept_layer_subject_has_type(&self, iri: &str, type_iri: &str) -> bool {
        self.blocks
            .iter()
            .filter(|block| matches!(block.kind, SemanticBlockKind::Concepts))
            .flat_map(|block| block.quads.iter())
            .any(|quad| {
                quad.predicate.as_str() == RDF_TYPE
                    && subject_iri(&quad.subject) == Some(iri)
                    && term_iri(&quad.object) == Some(type_iri)
            })
    }

    pub fn shacl_domain_ontology_index(
        &self,
        ontology_context: &BTreeSet<String>,
    ) -> shacl::DomainOntologyIndex {
        let quads = self.reachable_ontology_context_quads(ontology_context);
        shacl::DomainOntologyIndex::from_quads(&quads)
    }

    pub fn ontology_prefix_map(&self, ontology_context: &[String]) -> FxHashMap<String, String> {
        let context: BTreeSet<&str> = ontology_context.iter().map(String::as_str).collect();
        let mut prefixes = FxHashMap::default();

        for block in &self.blocks {
            if !matches!(block.kind, SemanticBlockKind::Ontology)
                || !context.contains(block.source.as_str())
            {
                continue;
            }

            for (prefix, iri) in parse_turtle_prefix_declarations(&block.content) {
                prefixes.entry(prefix).or_insert(iri);
            }
        }

        for block in &self.blocks {
            if !matches!(block.kind, SemanticBlockKind::Concepts) {
                continue;
            }

            for (prefix, iri) in parse_turtle_prefix_declarations(&block.content) {
                prefixes.entry(prefix).or_insert(iri);
            }
        }

        for declaration in self.ontology_documents_for_context(&context) {
            prefixes
                .entry(declaration.ontology_prefix.clone())
                .or_insert(declaration.term_namespace.clone());
        }

        for source in self.external_sources_for_context(&context) {
            prefixes
                .entry(source.prefix.clone())
                .or_insert(source.namespace.clone());
        }

        prefixes
    }

    pub fn used_ontology_project_namespaces(
        &self,
        ontology_context: &BTreeSet<&str>,
    ) -> UsedOntologyProjectNamespaces {
        let mut by_prefix: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        let mut prefix_by_namespace: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

        for declaration in self.ontology_documents_for_context(ontology_context) {
            by_prefix
                .entry(declaration.ontology_prefix.clone())
                .or_default()
                .insert(declaration.term_namespace.clone());
            prefix_by_namespace
                .entry(declaration.term_namespace.clone())
                .or_default()
                .insert(declaration.ontology_prefix.clone());
        }

        for source in self.external_sources_for_context(ontology_context) {
            by_prefix
                .entry(source.prefix.clone())
                .or_default()
                .insert(source.namespace.clone());
            prefix_by_namespace
                .entry(source.namespace.clone())
                .or_default()
                .insert(source.prefix.clone());
        }

        UsedOntologyProjectNamespaces {
            by_prefix,
            prefix_by_namespace,
        }
    }

    fn ontology_documents_for_context<'a>(
        &'a self,
        ontology_context: &BTreeSet<&str>,
    ) -> Vec<&'a OntologyDocumentDeclaration> {
        self.ontology_documents
            .iter()
            .filter(|declaration| {
                declaration
                    .element_identifiers
                    .iter()
                    .any(|identifier| ontology_context.contains(identifier.as_str()))
            })
            .collect()
    }

    fn external_sources_for_context<'a>(
        &'a self,
        ontology_context: &BTreeSet<&str>,
    ) -> Vec<&'a ExternalOntologySource> {
        self.external_sources
            .iter()
            .filter(|source| {
                source.builtin || ontology_context.contains(source.owner_identifier.as_str())
            })
            .collect()
    }

    pub fn ontology_document_by_element(&self) -> BTreeMap<&str, &str> {
        let mut document_by_element = BTreeMap::new();
        for document in &self.ontology_documents {
            for element_identifier in &document.element_identifiers {
                document_by_element.insert(element_identifier.as_str(), document.iri.as_str());
            }
        }
        document_by_element
    }

    pub fn to_authored_ontology_turtle_string(&self) -> Result<String, ReqvireError> {
        let prefix_map = self.turtle_prefix_map(false)?;
        let ontology_documents_turtle =
            build_ontology_document_declarations_turtle(&self.ontology_documents);
        let mut output = String::new();
        output.push_str(&prefix_map.to_turtle_block());
        append_turtle_body(
            &mut output,
            &ontology_documents_turtle,
            "Generated ontology document declarations",
            &prefix_map,
        )?;
        let mut seen_quads = quad_keys_from_turtle_with_prefix_map(
            &ontology_documents_turtle,
            &prefix_map,
            "Generated ontology document declarations",
        )?;
        append_blocks_turtle(
            &mut output,
            self.blocks.iter().filter(|block| {
                matches!(
                    block.kind,
                    SemanticBlockKind::Ontology | SemanticBlockKind::Concepts
                )
            }),
            &mut seen_quads,
            &prefix_map,
        )?;
        Ok(output)
    }

    pub fn to_ontology_turtle_string_with_external(
        &self,
        include_external: bool,
    ) -> Result<String, ReqvireError> {
        let prefix_map = self.turtle_prefix_map(include_external)?;
        let mut output = String::new();
        output.push_str("# Generated by Reqvire semantic ontology export\n");
        output.push_str(&format!(
            "# Blocks: {} ontology\n\n",
            self.summary.ontology_blocks
        ));
        output.push_str(&prefix_map.to_turtle_block());
        let ontology_documents_turtle =
            build_ontology_document_declarations_turtle(&self.ontology_documents);
        append_turtle_body(
            &mut output,
            &ontology_documents_turtle,
            "Generated ontology document declarations",
            &prefix_map,
        )?;
        let ontology_term_definitions_turtle = build_ontology_term_definitions_turtle(
            &self.ontology_documents,
            &self.ontology_declarations,
            &self.blocks,
        );
        append_turtle_body(
            &mut output,
            &ontology_term_definitions_turtle,
            "Generated ontology term definition links",
            &prefix_map,
        )?;

        let mut seen_quads = quad_keys_from_turtle_with_prefix_map(
            &(ontology_documents_turtle + &ontology_term_definitions_turtle),
            &prefix_map,
            "Generated ontology declarations and definition links",
        )?;
        append_blocks_turtle(
            &mut output,
            self.blocks
                .iter()
                .filter(|block| matches!(block.kind, SemanticBlockKind::Ontology)),
            &mut seen_quads,
            &prefix_map,
        )?;
        if include_external {
            append_used_external_subset_turtle(self, &mut output, &mut seen_quads, &prefix_map)?;
        }
        Ok(output)
    }

    pub fn serialize_ontologies(
        &self,
        format: SemanticExportFormat,
        include_external: bool,
        namespace_base: Option<&str>,
    ) -> Result<String, ReqvireError> {
        let mut index = self.with_namespace_base_filter(namespace_base)?;
        index.apply_external_visibility(include_external)?;
        let turtle = index.to_ontology_turtle_string_with_external(include_external)?;
        serialize_turtle_as_format(&turtle, format, "Semantic ontology export")
    }

    pub fn to_authored_ontology_layer_turtle_string(&self) -> Result<String, ReqvireError> {
        self.to_selected_layer_turtle_string(&[
            SemanticBlockKind::Ontology,
            SemanticBlockKind::Shapes,
        ])
    }

    pub fn to_shacl_turtle_string(&self) -> Result<String, ReqvireError> {
        self.to_selected_layer_turtle_string(&[SemanticBlockKind::Shapes])
    }

    fn to_selected_layer_turtle_string(
        &self,
        selected_layers: &[SemanticBlockKind],
    ) -> Result<String, ReqvireError> {
        let prefix_map = self.turtle_prefix_map(false)?;
        let mut output = String::new();
        output.push_str(&prefix_map.to_turtle_block());
        let mut seen_quads = BTreeSet::new();
        append_blocks_turtle(
            &mut output,
            self.blocks
                .iter()
                .filter(|block| selected_layers.contains(&block.kind)),
            &mut seen_quads,
            &prefix_map,
        )?;
        Ok(output)
    }

    pub fn serialize_shapes(&self, format: SemanticExportFormat) -> Result<String, ReqvireError> {
        let turtle = self.to_shacl_turtle_string()?;
        serialize_turtle_as_format(&turtle, format, "Semantic shapes export")
    }

    pub fn to_concepts_turtle_string(
        &self,
        include_mappings: bool,
    ) -> Result<String, ReqvireError> {
        let prefix_map = self.turtle_prefix_map(false)?;
        let concept_iris = skos_concept_iris(self);
        let mut output = String::new();
        output.push_str("# Generated by Reqvire semantic concept export\n");
        output.push_str(&format!("# Concepts: {}\n\n", concept_iris.len()));
        output.push_str(&prefix_map.to_turtle_block());
        let mut seen_quads = BTreeSet::new();
        let mut concept_quads = Vec::new();
        for block in &self.blocks {
            if !matches!(
                block.kind,
                SemanticBlockKind::Ontology | SemanticBlockKind::Concepts
            ) {
                continue;
            }
            for quad in &block.quads {
                if is_concept_layer_quad(block.kind, quad, &concept_iris, include_mappings) {
                    let key = quad_key(quad);
                    if seen_quads.insert(key) {
                        concept_quads.push(quad);
                    }
                }
            }
        }
        if !concept_quads.is_empty() {
            let turtle = serialize_quads_turtle_body(&concept_quads, &prefix_map)?;
            output.push_str(turtle.trim());
            output.push_str("\n\n");
        }
        Ok(output)
    }

    pub fn serialize_concepts(
        &self,
        format: SemanticExportFormat,
        include_mappings: bool,
    ) -> Result<String, ReqvireError> {
        let turtle = self.to_concepts_turtle_string(include_mappings)?;
        serialize_turtle_as_format(&turtle, format, "Semantic concept export")
    }

    pub fn to_authored_model_layer_turtle_string(
        &self,
        registry: &GraphRegistry,
    ) -> Result<String, ReqvireError> {
        build_authored_model_turtle(registry, self)
    }

    pub fn to_generated_layer_turtle_string(
        &self,
        registry: &GraphRegistry,
    ) -> Result<String, ReqvireError> {
        let prefix_map = self.turtle_prefix_map(true)?;
        build_generated_model_turtle(registry, self, &prefix_map)
    }

    pub fn to_raw_external_turtle_string(&self) -> Result<String, ReqvireError> {
        let prefix_map = self.turtle_prefix_map(true)?;
        let mut output = String::new();
        output.push_str(&prefix_map.to_turtle_block());
        let mut seen_quads = BTreeSet::new();
        append_blocks_turtle(
            &mut output,
            self.external_blocks.iter(),
            &mut seen_quads,
            &prefix_map,
        )?;
        Ok(output)
    }

    pub fn to_ontology_projection_turtle_string(&self) -> String {
        build_ontology_projection_turtle(self)
    }

    pub fn to_used_external_subset_turtle_string(&self) -> Result<String, ReqvireError> {
        materialize_used_external_subset_turtle(self)
    }

    pub fn used_external_subset_block(&self) -> Result<Option<SemanticBlock>, ReqvireError> {
        let content = self.to_used_external_subset_turtle_string()?;
        let quads = quads_from_turtle(&content, "used external ontology subset projection")?;
        if quads.is_empty() {
            return Ok(None);
        }

        Ok(Some(SemanticBlock {
            kind: SemanticBlockKind::ExternalOntology,
            source: "reqvire:external-used-subset".to_string(),
            source_name: "Reqvire used external ontology subset".to_string(),
            file_path: String::new(),
            line_number: 0,
            language: "turtle".to_string(),
            external_materialization: Some("used_subset".to_string()),
            content,
            quads,
        }))
    }

    pub fn to_turtle_string(&self) -> Result<String, ReqvireError> {
        self.to_turtle_string_with_external(false)
    }

    pub fn to_turtle_string_with_external(
        &self,
        include_external: bool,
    ) -> Result<String, ReqvireError> {
        let prefix_map = self.turtle_prefix_map(include_external)?;
        let mut output = String::new();
        output.push_str("# Generated by Reqvire semantic index\n");
        output.push_str(&format!(
            "# Blocks: {} ontology, {} shapes\n\n",
            self.summary.ontology_blocks, self.summary.shape_blocks
        ));
        output.push_str(&prefix_map.to_turtle_block());
        let ontology_documents_turtle =
            build_ontology_document_declarations_turtle(&self.ontology_documents);
        append_turtle_body(
            &mut output,
            &ontology_documents_turtle,
            "Generated ontology document declarations",
            &prefix_map,
        )?;
        let ontology_term_definitions_turtle = build_ontology_term_definitions_turtle(
            &self.ontology_documents,
            &self.ontology_declarations,
            &self.blocks,
        );
        append_turtle_body(
            &mut output,
            &ontology_term_definitions_turtle,
            "Generated ontology term definition links",
            &prefix_map,
        )?;

        let mut seen_quads = quad_keys_from_turtle_with_prefix_map(
            &(ontology_documents_turtle + &ontology_term_definitions_turtle),
            &prefix_map,
            "Generated ontology declarations and definition links",
        )?;
        for (block, quads) in normalized_export_blocks(&self.blocks, &mut seen_quads) {
            output.push_str(
                "# -----------------------------------------------------------------------------\n",
            );
            output.push_str(&format!("# Source: {}\n", block.source));
            output.push_str(&format!("# Name: {}\n", block.source_name));
            output.push_str(&format!("# Kind: {}\n", block.kind.as_str()));
            output.push_str(&format!("# File: {}\n", block.file_path));
            if block.line_number > 0 {
                output.push_str(&format!("# Line: {}\n", block.line_number));
            }
            output.push('\n');
            let turtle = serialize_quads_turtle_body(&quads, &prefix_map)?;
            if !turtle.trim().is_empty() {
                output.push_str(turtle.trim());
                output.push_str("\n\n");
            }
        }

        if include_external {
            let used_external_subset_turtle = self.to_used_external_subset_turtle_string()?;
            let used_external_subset_quads = quads_from_turtle(
                &used_external_subset_turtle,
                "used external ontology subset projection",
            )?;
            let used_external_subset_quads =
                unique_quads(used_external_subset_quads.iter(), &mut seen_quads);
            if !used_external_subset_quads.is_empty() {
                output.push_str(
                    "# -----------------------------------------------------------------------------\n",
                );
                output.push_str("# Source: reqvire:external-used-subset\n");
                output.push_str("# Name: Reqvire used external ontology subset\n");
                output.push_str("# Kind: external-used-subset\n\n");
                output.push('\n');
                let turtle = serialize_quads_turtle_body(&used_external_subset_quads, &prefix_map)?;
                if !turtle.trim().is_empty() {
                    output.push_str(turtle.trim());
                    output.push_str("\n\n");
                }
            }
        }

        Ok(output)
    }

    pub fn to_jsonld_string(&self) -> Result<String, ReqvireError> {
        self.to_jsonld_string_with_external(false)
    }

    pub fn to_jsonld_string_with_external(
        &self,
        include_external: bool,
    ) -> Result<String, ReqvireError> {
        let prefix_map = self.turtle_prefix_map(include_external)?;
        let mut serializer = RdfSerializer::from_format(RdfFormat::JsonLd {
            profile: JsonLdProfileSet::empty(),
        })
        .for_writer(Vec::new());

        let ontology_documents_turtle =
            build_ontology_document_declarations_turtle(&self.ontology_documents);
        let ontology_term_definitions_turtle = build_ontology_term_definitions_turtle(
            &self.ontology_documents,
            &self.ontology_declarations,
            &self.blocks,
        );
        let generated_turtle = ontology_documents_turtle + &ontology_term_definitions_turtle;
        if !generated_turtle.trim().is_empty() {
            let parse_turtle = prefix_map.to_turtle_block() + &generated_turtle;
            for parsed in
                RdfParser::from_format(RdfFormat::Turtle).for_reader(parse_turtle.as_bytes())
            {
                let quad = parsed.map_err(|error| {
                    ReqvireError::SerializationError(format!(
                        "Generated ontology declarations failed to parse: {}",
                        error
                    ))
                })?;
                serializer.serialize_quad(quad.as_ref())?;
            }
        }

        let mut seen_quads = quad_keys_from_turtle_with_prefix_map(
            &generated_turtle,
            &prefix_map,
            "Generated ontology declarations and definition links",
        )?;
        for (_block, quads) in normalized_export_blocks(&self.blocks, &mut seen_quads) {
            for quad in quads {
                serializer.serialize_quad((*quad).as_ref())?;
            }
        }

        if include_external {
            let used_external_subset_turtle = self.to_used_external_subset_turtle_string()?;
            let used_external_subset_quads = quads_from_turtle(
                &used_external_subset_turtle,
                "used external ontology subset projection",
            )?;
            for quad in unique_quads(used_external_subset_quads.iter(), &mut seen_quads) {
                serializer.serialize_quad(quad.as_ref())?;
            }
        }

        let bytes = serializer.finish()?;
        String::from_utf8(bytes).map_err(|e| ReqvireError::SerializationError(e.to_string()))
    }

    pub fn serialize(&self, format: SemanticExportFormat) -> Result<String, ReqvireError> {
        match format {
            SemanticExportFormat::Turtle => self.to_turtle_string(),
            SemanticExportFormat::JsonLd => self.to_jsonld_string(),
        }
    }

    pub fn to_full_turtle_string(&self) -> Result<String, ReqvireError> {
        self.to_full_turtle_string_with_external(false)
    }

    pub fn to_full_turtle_string_with_external(
        &self,
        include_external: bool,
    ) -> Result<String, ReqvireError> {
        let prefix_map = self.turtle_prefix_map(include_external)?;
        let mut output = self.to_turtle_string_with_external(include_external)?;
        output.push_str(&self.model_context_turtle);
        output.push_str(&build_semantic_term_context_turtle(self));
        output.push_str(&build_ontology_projection_turtle(self));
        output.push_str(&build_turtle_prefix_projection_turtle(&prefix_map));
        serialize_turtle_with_prefix_map(&output, &prefix_map, "Full semantic Turtle projection")
    }

    pub fn to_full_jsonld_string(&self) -> Result<String, ReqvireError> {
        self.to_full_jsonld_string_with_external(false)
    }

    pub fn to_full_jsonld_string_with_external(
        &self,
        include_external: bool,
    ) -> Result<String, ReqvireError> {
        let turtle = self.to_full_turtle_string_with_external(include_external)?;
        let mut serializer = RdfSerializer::from_format(RdfFormat::JsonLd {
            profile: JsonLdProfileSet::empty(),
        })
        .for_writer(Vec::new());

        for parsed in RdfParser::from_format(RdfFormat::Turtle).for_reader(turtle.as_bytes()) {
            let quad = parsed.map_err(|error| {
                ReqvireError::SerializationError(format!(
                    "Full semantic Turtle projection failed to parse: {}",
                    error
                ))
            })?;
            serializer.serialize_quad(quad.as_ref())?;
        }

        let bytes = serializer.finish()?;
        String::from_utf8(bytes).map_err(|e| ReqvireError::SerializationError(e.to_string()))
    }

    pub fn serialize_full(&self, format: SemanticExportFormat) -> Result<String, ReqvireError> {
        match format {
            SemanticExportFormat::Turtle => self.to_full_turtle_string(),
            SemanticExportFormat::JsonLd => self.to_full_jsonld_string(),
        }
    }

    pub fn serialize_with_options(
        &self,
        format: SemanticExportFormat,
        full: bool,
        include_external: bool,
    ) -> Result<String, ReqvireError> {
        match (format, full) {
            (SemanticExportFormat::Turtle, false) => {
                self.to_turtle_string_with_external(include_external)
            }
            (SemanticExportFormat::JsonLd, false) => {
                self.to_jsonld_string_with_external(include_external)
            }
            (SemanticExportFormat::Turtle, true) => {
                self.to_full_turtle_string_with_external(include_external)
            }
            (SemanticExportFormat::JsonLd, true) => {
                self.to_full_jsonld_string_with_external(include_external)
            }
        }
    }

    pub fn serialize_with_options_and_filter(
        &self,
        format: SemanticExportFormat,
        full: bool,
        include_external: bool,
        namespace_base: Option<&str>,
    ) -> Result<String, ReqvireError> {
        if full && namespace_base.is_some_and(|value| !value.trim().is_empty()) {
            return Err(ReqvireError::ProcessError(
                "--namespace-base filters clean authored semantic exports; it cannot be combined with --full model-context projection.".to_string(),
            ));
        }

        self.with_namespace_base_filter(namespace_base)?
            .serialize_with_options(format, full, include_external)
    }

    pub fn serialize_export_layers(
        &self,
        format: SemanticExportFormat,
        layers: &[SemanticExportLayer],
        namespace_base: Option<&str>,
    ) -> Result<String, ReqvireError> {
        let selected_layers = if layers.is_empty() {
            SemanticExportLayer::default_layers()
        } else {
            layers
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect()
        };
        let has_model = selected_layers.contains(&SemanticExportLayer::Model);
        if has_model && namespace_base.is_some_and(|value| !value.trim().is_empty()) {
            return Err(ReqvireError::ProcessError(
                "--namespace-base filters clean authored semantic exports; it cannot be combined with the model layer.".to_string(),
            ));
        }

        let include_external = selected_layers.contains(&SemanticExportLayer::ExternalUsed);
        let mut index = self.with_namespace_base_filter(namespace_base)?;
        index.apply_external_visibility(include_external)?;
        let turtle = index.to_export_layers_turtle_string(&selected_layers)?;
        serialize_turtle_as_format(&turtle, format, "Semantic layered export")
    }

    fn to_export_layers_turtle_string(
        &self,
        layers: &[SemanticExportLayer],
    ) -> Result<String, ReqvireError> {
        let include_external = layers.contains(&SemanticExportLayer::ExternalUsed);
        let prefix_map = self.turtle_prefix_map(include_external)?;
        let mut output = String::new();
        output.push_str("# Generated by Reqvire semantic export\n");
        output.push_str(&format!(
            "# Layers: {}\n\n",
            layers
                .iter()
                .map(SemanticExportLayer::as_str)
                .collect::<Vec<_>>()
                .join(", ")
        ));
        output.push_str(&prefix_map.to_turtle_block());
        let mut seen_quads = BTreeSet::new();

        if layers.contains(&SemanticExportLayer::Ontologies) {
            let ontology_documents_turtle =
                build_ontology_document_declarations_turtle(&self.ontology_documents);
            append_turtle_body_unique(
                &mut output,
                &ontology_documents_turtle,
                "Generated ontology document declarations",
                &mut seen_quads,
                &prefix_map,
            )?;
            let ontology_term_definitions_turtle = build_ontology_term_definitions_turtle(
                &self.ontology_documents,
                &self.ontology_declarations,
                &self.blocks,
            );
            append_turtle_body_unique(
                &mut output,
                &ontology_term_definitions_turtle,
                "Generated ontology term definition links",
                &mut seen_quads,
                &prefix_map,
            )?;
            append_blocks_turtle(
                &mut output,
                self.blocks
                    .iter()
                    .filter(|block| matches!(block.kind, SemanticBlockKind::Ontology)),
                &mut seen_quads,
                &prefix_map,
            )?;
        }

        if layers.contains(&SemanticExportLayer::Shapes) {
            append_blocks_turtle(
                &mut output,
                self.blocks
                    .iter()
                    .filter(|block| matches!(block.kind, SemanticBlockKind::Shapes)),
                &mut seen_quads,
                &prefix_map,
            )?;
        }

        if layers.contains(&SemanticExportLayer::Concepts) {
            let concept_iris = skos_concept_iris(self);
            let mut concept_quads = Vec::new();
            for block in &self.blocks {
                if !matches!(
                    block.kind,
                    SemanticBlockKind::Ontology | SemanticBlockKind::Concepts
                ) {
                    continue;
                }
                for quad in &block.quads {
                    if is_concept_layer_quad(block.kind, quad, &concept_iris, false)
                        && seen_quads.insert(quad_key(quad))
                    {
                        concept_quads.push(quad);
                    }
                }
            }
            append_quads_turtle_section(
                &mut output,
                "# Source: reqvire:concept-layer\n# Name: Reqvire SKOS concept layer\n# Kind: concepts\n\n",
                &concept_quads,
                &prefix_map,
            )?;
        }

        if layers.contains(&SemanticExportLayer::Model) {
            append_turtle_body_unique(
                &mut output,
                &self.model_context_turtle,
                "Generated model context",
                &mut seen_quads,
                &prefix_map,
            )?;
            append_turtle_body_unique(
                &mut output,
                &build_semantic_term_context_turtle(self),
                "Generated semantic term context",
                &mut seen_quads,
                &prefix_map,
            )?;
            append_turtle_body_unique(
                &mut output,
                &build_ontology_projection_turtle(self),
                "Generated ontology projection",
                &mut seen_quads,
                &prefix_map,
            )?;
        }

        if layers.contains(&SemanticExportLayer::Prefixes) {
            append_turtle_body_unique(
                &mut output,
                &build_turtle_prefix_projection_turtle(&prefix_map),
                "Generated Turtle prefix projection",
                &mut seen_quads,
                &prefix_map,
            )?;
        }

        if include_external {
            append_used_external_subset_turtle(self, &mut output, &mut seen_quads, &prefix_map)?;
        }

        Ok(output)
    }

    fn turtle_prefix_map(&self, include_external: bool) -> Result<TurtlePrefixMap, ReqvireError> {
        let mut builder = TurtlePrefixMapBuilder::new();
        builder.add("rdf", RDF_NS, 0)?;
        builder.add("rdfs", RDFS_NS, 0)?;
        builder.add("owl", OWL_NS, 0)?;
        builder.add("xsd", XSD_NS, 0)?;
        builder.add("sh", SHACL_NS, 0)?;
        builder.add("skos", SKOS_NS, 0)?;

        for declaration in &self.ontology_documents {
            builder.add(
                &declaration.ontology_prefix,
                &declaration.term_namespace,
                10,
            )?;
        }

        for block in &self.blocks {
            let rank = match block.kind {
                SemanticBlockKind::Ontology | SemanticBlockKind::Shapes => 10,
                SemanticBlockKind::Concepts => 20,
                SemanticBlockKind::ExternalOntology => 30,
            };
            for (prefix, namespace) in parse_turtle_prefix_declarations(&block.content) {
                builder.add_local(&prefix, &namespace, rank);
            }
        }

        if include_external {
            for source in &self.external_sources {
                builder.add(&source.prefix, &source.namespace, 30)?;
            }
            for block in &self.external_blocks {
                for (prefix, namespace) in parse_turtle_prefix_declarations(&block.content) {
                    builder.add_local(&prefix, &namespace, 30);
                }
            }
        }

        builder.add_reqvire_fallback_if_missing()?;

        Ok(builder.finish())
    }
}

mod export;
mod index;
mod prefixes;
mod vocabulary;

use export::*;
use index::*;

pub use export::external_materialization_metadata;
pub(crate) use export::materialized_external_subjects;
pub use index::build_semantic_index;
pub(crate) use prefixes::{parse_turtle_prefix_declarations, parse_turtle_prefix_line};

#[cfg(test)]
mod tests {
    use super::export::normalized_relation_projection;
    use super::vocabulary::RelationProjectionDirection;
    use super::*;
    use crate::test_support::parse_test_quads;
    use oxigraph::model::Term;

    fn external_block(content: &str) -> SemanticBlock {
        SemanticBlock {
            kind: SemanticBlockKind::ExternalOntology,
            source: "test#external".to_string(),
            source_name: "Test external ontology".to_string(),
            file_path: "system-model/Ontologies/Test.md".to_string(),
            line_number: 1,
            language: "turtle".to_string(),
            external_materialization: None,
            content: content.to_string(),
            quads: parse_test_quads(content),
        }
    }

    fn ontology_block(content: &str) -> SemanticBlock {
        SemanticBlock {
            kind: SemanticBlockKind::Ontology,
            source: "test#ontology".to_string(),
            source_name: "Test ontology".to_string(),
            file_path: "system-model/Ontologies/Test.md".to_string(),
            line_number: 1,
            language: "turtle".to_string(),
            external_materialization: None,
            content: content.to_string(),
            quads: parse_test_quads(content),
        }
    }

    fn test_projection_source() -> OntologyProjectionSource {
        OntologyProjectionSource {
            source_block: "test#ontology".to_string(),
            source_element_identifier: "system-model/Ontologies/Test.md#test-ontology".to_string(),
            source_name: "Test Ontology".to_string(),
            file_path: "system-model/Ontologies/Test.md".to_string(),
            line_number: 1,
            block_kind: "ontology".to_string(),
        }
    }

    #[test]
    fn contract_ownership_projection_uses_definition_predicate_names() {
        let defined_by =
            normalized_relation_projection("definedBy").expect("definedBy should project");
        assert_eq!(defined_by.forward_property, "requirementDefinedByContract");
        assert_eq!(defined_by.inverse_property, "contractDefinesRequirement");
        assert_eq!(defined_by.direction, RelationProjectionDirection::Forward);

        let define = normalized_relation_projection("define").expect("define should project");
        assert_eq!(define.forward_property, "requirementDefinedByContract");
        assert_eq!(define.inverse_property, "contractDefinesRequirement");
        assert_eq!(define.direction, RelationProjectionDirection::Inverse);

        let contract_bindings = normalized_relation_projection("contract_bindings")
            .expect("contract_bindings should project");
        assert_eq!(contract_bindings.forward_property, "bindsContract");
        assert_eq!(contract_bindings.inverse_property, "boundByContract");
        assert_eq!(
            contract_bindings.direction,
            RelationProjectionDirection::Forward
        );
    }

    fn iri_term(iri: &str) -> OntologyProjectionTerm {
        OntologyProjectionTerm {
            kind: OntologyProjectionTermKind::Iri,
            value: iri.to_string(),
            label: iri.to_string(),
        }
    }

    fn projection_graph_with_external_object(iri: &str) -> OntologyProjectionGraph {
        let source = test_projection_source();
        OntologyProjectionGraph {
            id: "urn:reqvire:ontology-projection:test".to_string(),
            derivation_mode: OntologyProjectionDerivationMode::DirectAuthored,
            projections: Vec::new(),
            constructs: vec![OntologyConstruct {
                id: "urn:reqvire:ontology-construct:test".to_string(),
                family: OntologyConstructFamily::SubclassMembership,
                kind: OntologyConstructKind::SubclassInclusion,
                subject: iri_term("https://example.test/local#LocalTerm"),
                predicate: Some(iri_term(RDFS_SUBCLASS_OF)),
                object: Some(iri_term(iri)),
                property: None,
                members: Vec::new(),
                property_characteristic: None,
                restriction_kind: None,
                class_expression_kind: None,
                shape_overlay_kind: None,
                symbol: None,
                provenance: OntologyProjectionProvenance {
                    derivation_mode: OntologyProjectionDerivationMode::DirectAuthored,
                    source,
                    evidence: Vec::new(),
                },
            }],
            symbols: Vec::new(),
        }
    }

    fn test_index() -> SemanticIndex {
        let raw_external = r#"
@prefix ext: <https://example.test/external#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

ext:PathTerm a owl:ObjectProperty ;
  rdfs:label "Path term" ;
  rdfs:domain ext:SupportClass .

ext:DeclaredTerm a owl:Class ;
  rdfs:label "Declared term" .

ext:ConceptTerm a owl:Class ;
  rdfs:label "Concept term" .

ext:ProjectedTerm a owl:Class ;
  rdfs:label "Projected term" ;
  rdfs:subClassOf ext:SupportClass .

ext:SupportClass a owl:Class ;
  rdfs:label "Support class" ;
  rdfs:subClassOf ext:ParentSupport .

ext:ParentSupport a owl:Class ;
  rdfs:label "Parent support" .

ext:UnusedTerm a owl:Class ;
  rdfs:label "Unused term" .
"#;

        SemanticIndex {
            blocks: Vec::new(),
            external_blocks: vec![external_block(raw_external)],
            external_sources: vec![ExternalOntologySource {
                owner_identifier: "system-model/Ontologies/Test.md#test-ontology".to_string(),
                owner_name: "Test Ontology".to_string(),
                prefix: "ext".to_string(),
                namespace: "https://example.test/external#".to_string(),
                resource: Some("https://example.test/external".to_string()),
                source: "references/external.ttl".to_string(),
                format: "turtle".to_string(),
                line_number: 1,
                builtin: false,
            }],
            diagnostics: Vec::new(),
            ontology_documents: Vec::new(),
            ontology_declarations: FxHashMap::from_iter([
                (
                    "https://example.test/external#ProjectedTerm".to_string(),
                    vec![OntologyTermDeclaration {
                        iri: "https://example.test/external#ProjectedTerm".to_string(),
                        role: OntologyTermRole::Class,
                        element_identifier: "system-model/Ontologies/Test.md#test-ontology"
                            .to_string(),
                        external: true,
                        materialized_in_used_subset: false,
                    }],
                ),
                (
                    "https://example.test/external#UnusedTerm".to_string(),
                    vec![OntologyTermDeclaration {
                        iri: "https://example.test/external#UnusedTerm".to_string(),
                        role: OntologyTermRole::Class,
                        element_identifier: "system-model/Ontologies/Test.md#test-ontology"
                            .to_string(),
                        external: true,
                        materialized_in_used_subset: false,
                    }],
                ),
            ]),
            shape_references: Vec::new(),
            ontology_projection: projection_graph_with_external_object(
                "https://example.test/external#ProjectedTerm",
            ),
            model_context: ModelContextGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
            },
            model_context_turtle: r#"
@prefix reqvire: <https://www.reqvire.org/ontology#> .

<urn:reqvire:external-source:test> a reqvire:ExternalOntologySource ;
  reqvire:externalOntologyNamespace "https://example.test/external#" .

<urn:reqvire:shape:test> reqvire:referencesTerm <https://example.test/external#PathTerm> .
<urn:reqvire:declaration:test> reqvire:declaresTerm <https://example.test/external#DeclaredTerm> .
<urn:reqvire:concept:test> reqvire:conceptReference <https://example.test/external#ConceptTerm> .
"#
            .to_string(),
            summary: SemanticIndexSummary {
                ontology_blocks: 0,
                shape_blocks: 0,
                total_blocks: 0,
                total_quads: 0,
            },
        }
    }

    fn materialized_triples(index: &SemanticIndex) -> BTreeSet<(String, String, String)> {
        parse_test_quads(&index.to_used_external_subset_turtle_string().unwrap())
            .into_iter()
            .map(|quad| {
                (
                    quad.subject.to_string(),
                    quad.predicate.to_string(),
                    quad.object.to_string(),
                )
            })
            .collect()
    }

    #[test]
    fn used_external_subset_materializes_sparql_seed_sources_and_closure() {
        let triples = materialized_triples(&test_index());

        assert!(triples.contains(&(
            "<https://example.test/external#PathTerm>".to_string(),
            "<http://www.w3.org/2000/01/rdf-schema#label>".to_string(),
            "\"Path term\"".to_string()
        )));
        assert!(triples.contains(&(
            "<https://example.test/external#DeclaredTerm>".to_string(),
            "<http://www.w3.org/2000/01/rdf-schema#label>".to_string(),
            "\"Declared term\"".to_string()
        )));
        assert!(triples.contains(&(
            "<https://example.test/external#ConceptTerm>".to_string(),
            "<http://www.w3.org/2000/01/rdf-schema#label>".to_string(),
            "\"Concept term\"".to_string()
        )));
        assert!(triples.contains(&(
            "<https://example.test/external#ProjectedTerm>".to_string(),
            "<http://www.w3.org/2000/01/rdf-schema#label>".to_string(),
            "\"Projected term\"".to_string()
        )));
        assert!(triples.contains(&(
            "<https://example.test/external#SupportClass>".to_string(),
            "<http://www.w3.org/2000/01/rdf-schema#subClassOf>".to_string(),
            "<https://example.test/external#ParentSupport>".to_string()
        )));
        assert!(triples.contains(&(
            "<https://example.test/external#SupportClass>".to_string(),
            "<http://www.w3.org/2000/01/rdf-schema#label>".to_string(),
            "\"Support class\"".to_string()
        )));
    }

    #[test]
    fn used_external_subset_omits_unused_raw_external_terms() {
        let output = test_index()
            .to_turtle_string_with_external(true)
            .expect("include_external Turtle should serialize");

        assert!(output.contains("Projected term"));
        assert!(output.contains("Support class"));
        assert!(!output.contains("Unused term"));
        assert!(!output.contains("UnusedTerm"));
    }

    #[test]
    fn used_external_subset_block_marks_materialization_without_raw_terms() {
        let block = test_index()
            .used_external_subset_block()
            .expect("used subset should materialize")
            .expect("used subset block should be present");
        let subjects = block
            .quads
            .iter()
            .filter_map(|quad| subject_iri(&quad.subject).map(str::to_string))
            .collect::<BTreeSet<_>>();

        assert_eq!(
            block.external_materialization.as_deref(),
            Some("used_subset")
        );
        assert!(subjects.contains("https://example.test/external#ProjectedTerm"));
        assert!(!subjects.contains("https://example.test/external#UnusedTerm"));
    }

    #[test]
    fn builtin_skos_external_source_is_registered_and_parsed() {
        let source = builtin_external_ontology_sources()
            .into_iter()
            .find(|source| source.prefix == "skos")
            .expect("SKOS built-in external source should be registered");
        let mut diagnostics = Vec::new();
        let block = builtin_external_ontology_block(&source, &mut diagnostics)
            .expect("SKOS built-in source should parse");
        assert!(diagnostics.is_empty());
        assert!(source.builtin);
        assert_eq!(source.source, "builtin:skos.rdf");

        let triples = block
            .quads
            .iter()
            .map(|quad| {
                (
                    quad.subject.to_string(),
                    quad.predicate.to_string(),
                    quad.object.to_string(),
                )
            })
            .collect::<BTreeSet<_>>();

        assert!(triples.contains(&(
            "<http://www.w3.org/2004/02/skos/core#Concept>".to_string(),
            "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>".to_string(),
            "<http://www.w3.org/2002/07/owl#Class>".to_string()
        )));
        assert!(triples.contains(&(
            "<http://www.w3.org/2004/02/skos/core#prefLabel>".to_string(),
            "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>".to_string(),
            "<http://www.w3.org/1999/02/22-rdf-syntax-ns#Property>".to_string()
        )));
    }

    #[test]
    fn generated_model_turtle_declares_builtin_prefixes_when_it_uses_them() {
        let registry = GraphRegistry::new();
        let index = SemanticIndex {
            blocks: Vec::new(),
            external_blocks: Vec::new(),
            external_sources: Vec::new(),
            diagnostics: Vec::new(),
            ontology_documents: vec![OntologyDocumentDeclaration {
                iri: "https://example.test/ontology".to_string(),
                ontology_base: "https://example.test/ontology".to_string(),
                ontology_prefix: "ex".to_string(),
                term_namespace: "https://example.test/ontology#".to_string(),
                element_identifiers: vec![
                    "system-model/Ontologies/Test.md#test-ontology".to_string()
                ],
                element_names: vec!["Test ontology".to_string()],
                imports: Vec::new(),
            }],
            ontology_declarations: FxHashMap::from_iter([(
                "https://example.test/ontology#TestTerm".to_string(),
                vec![OntologyTermDeclaration {
                    iri: "https://example.test/ontology#TestTerm".to_string(),
                    role: OntologyTermRole::Class,
                    element_identifier: "system-model/Ontologies/Test.md#test-ontology".to_string(),
                    external: false,
                    materialized_in_used_subset: false,
                }],
            )]),
            shape_references: Vec::new(),
            ontology_projection: OntologyProjectionGraph {
                id: "urn:reqvire:ontology-projection:test".to_string(),
                derivation_mode: OntologyProjectionDerivationMode::DirectAuthored,
                projections: Vec::new(),
                constructs: Vec::new(),
                symbols: Vec::new(),
            },
            model_context: ModelContextGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
            },
            model_context_turtle: String::new(),
            summary: SemanticIndexSummary {
                ontology_blocks: 0,
                shape_blocks: 0,
                total_blocks: 0,
                total_quads: 0,
            },
        };

        let turtle = index
            .to_generated_layer_turtle_string(&registry)
            .expect("generated layer Turtle should serialize");

        assert!(turtle.contains("@prefix owl: <http://www.w3.org/2002/07/owl#> ."));
        assert!(turtle.contains("@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> ."));
        assert!(turtle.contains("@prefix reqvire: <https://www.reqvire.org/ontology#> ."));
        assert!(turtle.contains("rdfs:isDefinedBy"));
    }

    #[test]
    fn used_external_subset_materializes_only_referenced_builtin_skos_terms() {
        let source = builtin_external_ontology_sources()
            .into_iter()
            .find(|source| source.prefix == "skos")
            .expect("SKOS built-in external source should be registered");
        let mut diagnostics = Vec::new();
        let external_block = builtin_external_ontology_block(&source, &mut diagnostics)
            .expect("SKOS built-in source should parse");
        assert!(diagnostics.is_empty());

        let authored = r#"
@prefix concept: <https://example.test/concepts#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix skos: <http://www.w3.org/2004/02/skos/core#> .

<https://example.test/concepts> a owl:Ontology .

concept:TraceabilityConstruct a owl:Class ;
  rdfs:range skos:Concept .
"#;

        let index = SemanticIndex {
            blocks: vec![ontology_block(authored)],
            external_blocks: vec![external_block],
            external_sources: vec![source],
            diagnostics: Vec::new(),
            ontology_documents: Vec::new(),
            ontology_declarations: FxHashMap::default(),
            shape_references: Vec::new(),
            ontology_projection: OntologyProjectionGraph {
                id: "urn:reqvire:ontology-projection:test".to_string(),
                derivation_mode: OntologyProjectionDerivationMode::DirectAuthored,
                projections: Vec::new(),
                constructs: Vec::new(),
                symbols: Vec::new(),
            },
            model_context: ModelContextGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
            },
            model_context_turtle: String::new(),
            summary: SemanticIndexSummary {
                ontology_blocks: 1,
                shape_blocks: 0,
                total_blocks: 1,
                total_quads: 0,
            },
        };

        let default_output = index
            .to_turtle_string()
            .expect("default Turtle should serialize");
        assert!(!default_output.contains("An idea or notion; a unit of thought."));

        let external_output = index
            .to_turtle_string_with_external(true)
            .expect("include_external Turtle should serialize");
        let external_quads = parse_test_quads(&external_output);
        assert!(external_output.contains("An idea or notion; a unit of thought."));
        assert!(external_quads.iter().any(|quad| {
            matches!(
                &quad.subject,
                oxigraph::model::NamedOrBlankNode::NamedNode(node) if node.as_str() == SKOS_CONCEPT
            ) || matches!(
                &quad.object,
                Term::NamedNode(node) if node.as_str() == SKOS_CONCEPT
            )
        }));
        assert!(!external_output.contains("Ordered Collection"));
        assert!(!external_output.contains(
            "An ordered collection of concepts, where both the grouping and the ordering are meaningful."
        ));
    }
}
