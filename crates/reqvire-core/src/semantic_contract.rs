use crate::element::{
    ContractType, Element, ElementType, FencedBlock, ReusedContractContextTarget, VerificationType,
    GOVERNANCE_METADATA_KEYS,
};
use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;
use crate::relation::{self, LinkType};
use o_kernel::diagnostics::is_false;
use o_kernel::{constructs, ontology, shacl, vocab::*};
use oxigraph::io::{JsonLdProfileSet, RdfFormat, RdfParser, RdfSerializer};
use oxigraph::model::{NamedNode, Quad, Term, Triple};
use oxigraph::store::Store;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;
use std::path::{Path, PathBuf};

const GRAPH_AUTHORED_ONTOLOGY: &str = "urn:reqvire:semantic-graph:authored-ontology";
const GRAPH_AUTHORED_MODEL: &str = "urn:reqvire:semantic-graph:authored-model";
const GRAPH_GENERATED: &str = "urn:reqvire:semantic-graph:generated";
const GRAPH_RAW_EXTERNAL_SOURCE: &str = "urn:reqvire:semantic-graph:raw-external-source";
const SKOS_NS: &str = "http://www.w3.org/2004/02/skos/core#";
const SKOS_CONCEPT: &str = "http://www.w3.org/2004/02/skos/core#Concept";
const SKOS_CONCEPT_SCHEME: &str = "http://www.w3.org/2004/02/skos/core#ConceptScheme";
const REQVIRE_MAPS_TO_CONCEPT: &str = "https://www.reqvire.org/ontology#mapsToConcept";

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
    pub ontology_declarations: HashMap<String, Vec<OntologyTermDeclaration>>,
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
        self.blocks
            .iter()
            .filter(|block| matches!(block.kind, SemanticBlockKind::Concepts))
            .flat_map(|block| block.quads.iter())
            .any(|quad| {
                quad.predicate.as_str() == RDF_TYPE
                    && subject_iri(&quad.subject) == Some(iri)
                    && term_iri(&quad.object) == Some(SKOS_CONCEPT)
            })
    }

    pub fn shacl_domain_ontology_index(
        &self,
        ontology_context: &BTreeSet<String>,
    ) -> shacl::DomainOntologyIndex {
        let quads = self.reachable_ontology_context_quads(ontology_context);
        shacl::DomainOntologyIndex::from_quads(&quads)
    }

    pub fn ontology_prefix_map(&self, ontology_context: &[String]) -> HashMap<String, String> {
        let context: BTreeSet<&str> = ontology_context.iter().map(String::as_str).collect();
        let mut prefixes = HashMap::new();

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

        for declaration in &self.ontology_documents {
            if declaration
                .element_identifiers
                .iter()
                .any(|identifier| context.contains(identifier.as_str()))
            {
                prefixes
                    .entry(declaration.ontology_prefix.clone())
                    .or_insert(declaration.term_namespace.clone());
            }
        }

        for source in &self.external_sources {
            if source.builtin || context.contains(source.owner_identifier.as_str()) {
                prefixes
                    .entry(source.prefix.clone())
                    .or_insert(source.namespace.clone());
            }
        }

        prefixes
    }

    pub fn used_ontology_project_namespaces(
        &self,
        ontology_context: &BTreeSet<&str>,
    ) -> UsedOntologyProjectNamespaces {
        let mut by_prefix: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        let mut prefix_by_namespace: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

        for declaration in &self.ontology_documents {
            if !declaration
                .element_identifiers
                .iter()
                .any(|identifier| ontology_context.contains(identifier.as_str()))
            {
                continue;
            }

            by_prefix
                .entry(declaration.ontology_prefix.clone())
                .or_default()
                .insert(declaration.term_namespace.clone());
            prefix_by_namespace
                .entry(declaration.term_namespace.clone())
                .or_default()
                .insert(declaration.ontology_prefix.clone());
        }

        for source in &self.external_sources {
            if !source.builtin && !ontology_context.contains(source.owner_identifier.as_str()) {
                continue;
            }

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

    pub fn to_authored_ontology_turtle_string(&self) -> Result<String, ReqvireError> {
        let ontology_documents_turtle =
            build_ontology_document_declarations_turtle(&self.ontology_documents);
        let mut output = ontology_documents_turtle.clone();
        let mut seen_quads = quad_keys_from_turtle(&ontology_documents_turtle)?;
        append_blocks_turtle(
            &mut output,
            self.blocks.iter().filter(|block| {
                matches!(
                    block.kind,
                    SemanticBlockKind::Ontology | SemanticBlockKind::Concepts
                )
            }),
            &mut seen_quads,
        )?;
        Ok(output)
    }

    pub fn to_ontology_turtle_string_with_external(
        &self,
        include_external: bool,
    ) -> Result<String, ReqvireError> {
        let mut output = String::new();
        output.push_str("# Generated by Reqvire semantic ontology export\n");
        output.push_str(&format!(
            "# Blocks: {} ontology\n\n",
            self.summary.ontology_blocks
        ));
        let ontology_documents_turtle =
            build_ontology_document_declarations_turtle(&self.ontology_documents);
        output.push_str(&ontology_documents_turtle);
        let ontology_term_definitions_turtle = build_ontology_term_definitions_turtle(
            &self.ontology_documents,
            &self.ontology_declarations,
            &self.blocks,
        );
        output.push_str(&ontology_term_definitions_turtle);

        let mut seen_quads = quad_keys_from_turtle(
            &(ontology_documents_turtle + &ontology_term_definitions_turtle),
        )?;
        append_blocks_turtle(
            &mut output,
            self.blocks
                .iter()
                .filter(|block| matches!(block.kind, SemanticBlockKind::Ontology)),
            &mut seen_quads,
        )?;
        if include_external {
            append_used_external_subset_turtle(self, &mut output, &mut seen_quads)?;
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
        let mut output = String::new();
        let mut seen_quads = BTreeSet::new();
        append_blocks_turtle(
            &mut output,
            self.blocks.iter().filter(|block| {
                matches!(
                    block.kind,
                    SemanticBlockKind::Ontology | SemanticBlockKind::Shapes
                )
            }),
            &mut seen_quads,
        )?;
        Ok(output)
    }

    pub fn to_shacl_turtle_string(&self) -> Result<String, ReqvireError> {
        let mut output = String::new();
        let mut seen_quads = BTreeSet::new();
        append_blocks_turtle(
            &mut output,
            self.blocks
                .iter()
                .filter(|block| matches!(block.kind, SemanticBlockKind::Shapes)),
            &mut seen_quads,
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
        let concept_iris = skos_concept_iris(self);
        let mut output = String::new();
        output.push_str("# Generated by Reqvire semantic concept export\n");
        output.push_str(&format!("# Concepts: {}\n\n", concept_iris.len()));
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
            let turtle = serialize_quads_turtle(&concept_quads)?;
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
        build_generated_model_turtle(registry, self)
    }

    pub fn to_raw_external_turtle_string(&self) -> Result<String, ReqvireError> {
        let mut output = String::new();
        let mut seen_quads = BTreeSet::new();
        append_blocks_turtle(&mut output, self.external_blocks.iter(), &mut seen_quads)?;
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
        let mut output = String::new();
        output.push_str("# Generated by Reqvire semantic index\n");
        output.push_str(&format!(
            "# Blocks: {} ontology, {} shapes\n\n",
            self.summary.ontology_blocks, self.summary.shape_blocks
        ));
        let ontology_documents_turtle =
            build_ontology_document_declarations_turtle(&self.ontology_documents);
        output.push_str(&ontology_documents_turtle);
        let ontology_term_definitions_turtle = build_ontology_term_definitions_turtle(
            &self.ontology_documents,
            &self.ontology_declarations,
            &self.blocks,
        );
        output.push_str(&ontology_term_definitions_turtle);

        let mut seen_quads = quad_keys_from_turtle(
            &(ontology_documents_turtle + &ontology_term_definitions_turtle),
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
            let turtle = serialize_quads_turtle(&quads)?;
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
                let turtle = serialize_quads_turtle(&used_external_subset_quads)?;
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
            for parsed in
                RdfParser::from_format(RdfFormat::Turtle).for_reader(generated_turtle.as_bytes())
            {
                let quad = parsed.map_err(|error| {
                    ReqvireError::SerializationError(format!(
                        "Generated ontology declarations failed to parse: {}",
                        error
                    ))
                })?;
                serializer
                    .serialize_quad(quad.as_ref())
                    .map_err(|e| ReqvireError::SerializationError(e.to_string()))?;
            }
        }

        let mut seen_quads = quad_keys_from_turtle(&generated_turtle)?;
        for (_block, quads) in normalized_export_blocks(&self.blocks, &mut seen_quads) {
            for quad in quads {
                serializer
                    .serialize_quad((*quad).as_ref())
                    .map_err(|e| ReqvireError::SerializationError(e.to_string()))?;
            }
        }

        if include_external {
            let used_external_subset_turtle = self.to_used_external_subset_turtle_string()?;
            let used_external_subset_quads = quads_from_turtle(
                &used_external_subset_turtle,
                "used external ontology subset projection",
            )?;
            for quad in unique_quads(used_external_subset_quads.iter(), &mut seen_quads) {
                serializer
                    .serialize_quad(quad.as_ref())
                    .map_err(|e| ReqvireError::SerializationError(e.to_string()))?;
            }
        }

        let bytes = serializer
            .finish()
            .map_err(|e| ReqvireError::SerializationError(e.to_string()))?;
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
        let mut output = self.to_turtle_string_with_external(include_external)?;
        output.push_str(&self.model_context_turtle);
        output.push_str(&build_ontology_projection_turtle(self));
        Ok(output)
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
            serializer
                .serialize_quad(quad.as_ref())
                .map_err(|e| ReqvireError::SerializationError(e.to_string()))?;
        }

        let bytes = serializer
            .finish()
            .map_err(|e| ReqvireError::SerializationError(e.to_string()))?;
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
}

fn export_filter_term_namespace(
    namespace_base: &str,
    documents: &[OntologyDocumentDeclaration],
) -> String {
    let trimmed = namespace_base.trim();
    if let Some(document) = documents
        .iter()
        .find(|document| document.ontology_base == trimmed || document.iri == trimmed)
    {
        return document.term_namespace.clone();
    }
    if trimmed.ends_with('#') {
        trimmed.to_string()
    } else {
        ontology_term_namespace(trimmed)
    }
}

fn block_declares_subject_in_namespace(block: &SemanticBlock, namespace: &str) -> bool {
    block
        .quads
        .iter()
        .any(|quad| subject_iri(&quad.subject).is_some_and(|iri| iri.starts_with(namespace)))
}

pub fn external_materialization_metadata(
    source: &SemanticIndex,
    visible: &SemanticIndex,
    include_external: bool,
) -> Value {
    let materialized_terms: BTreeSet<String> = visible
        .external_blocks
        .iter()
        .flat_map(materialized_external_subjects)
        .collect();
    let visible_external_declaration_count = visible
        .ontology_declarations
        .values()
        .flat_map(|declarations| declarations.iter())
        .filter(|declaration| declaration.external)
        .count();
    let available_external_declaration_count = source
        .ontology_declarations
        .values()
        .flat_map(|declarations| declarations.iter())
        .filter(|declaration| declaration.external)
        .count();
    let used_external_source_count = source
        .external_sources
        .iter()
        .filter(|source| {
            materialized_terms
                .iter()
                .any(|term| term.starts_with(&source.namespace))
        })
        .count();
    let raw_external_triple_count: usize = source
        .external_blocks
        .iter()
        .map(|block| block.quads.len())
        .sum();
    let materialized_external_triple_count: usize = visible
        .external_blocks
        .iter()
        .map(|block| block.quads.len())
        .sum();

    json!({
        "external_materialization": if include_external { "used_subset" } else { "none" },
        "external_counts": {
            "declared_external_source_count": source.external_sources.len(),
            "visible_external_source_count": if include_external { used_external_source_count } else { 0 },
            "used_external_source_count": if include_external { used_external_source_count } else { 0 },
            "available_external_term_declaration_count": available_external_declaration_count,
            "visible_external_term_declaration_count": if include_external { visible_external_declaration_count } else { 0 },
            "materialized_external_term_count": if include_external { materialized_terms.len() } else { 0 },
            "raw_external_triple_count": raw_external_triple_count,
            "materialized_external_triple_count": if include_external { materialized_external_triple_count } else { 0 }
        }
    })
}

fn materialized_external_subjects(block: &SemanticBlock) -> BTreeSet<String> {
    block
        .quads
        .iter()
        .filter_map(|quad| subject_iri(&quad.subject).map(str::to_string))
        .collect()
}

fn append_blocks_turtle<'a>(
    output: &mut String,
    blocks: impl IntoIterator<Item = &'a SemanticBlock>,
    seen_quads: &mut BTreeSet<String>,
) -> Result<(), ReqvireError> {
    for block in blocks {
        let mut quads = Vec::new();
        for quad in &block.quads {
            let key = quad_key(quad);
            if seen_quads.insert(key) {
                quads.push(quad);
            }
        }
        if quads.is_empty() {
            continue;
        }

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
        let turtle = serialize_quads_turtle(&quads)?;
        if !turtle.trim().is_empty() {
            output.push_str(turtle.trim());
            output.push_str("\n\n");
        }
    }
    Ok(())
}

fn normalized_export_blocks<'a>(
    blocks: &'a [SemanticBlock],
    seen_quads: &mut BTreeSet<String>,
) -> Vec<(&'a SemanticBlock, Vec<&'a Quad>)> {
    let mut normalized = Vec::new();
    for block in blocks {
        let mut quads = Vec::new();
        for quad in &block.quads {
            let key = quad_key(quad);
            if seen_quads.insert(key) {
                quads.push(quad);
            }
        }
        if !quads.is_empty() {
            normalized.push((block, quads));
        }
    }
    normalized
}

fn unique_quads<'a>(
    quads: impl IntoIterator<Item = &'a Quad>,
    seen_quads: &mut BTreeSet<String>,
) -> Vec<&'a Quad> {
    quads
        .into_iter()
        .filter(|quad| seen_quads.insert(quad_key(quad)))
        .collect()
}

fn append_used_external_subset_turtle(
    index: &SemanticIndex,
    output: &mut String,
    seen_quads: &mut BTreeSet<String>,
) -> Result<(), ReqvireError> {
    let used_external_subset_turtle = index.to_used_external_subset_turtle_string()?;
    let used_external_subset_quads = quads_from_turtle(
        &used_external_subset_turtle,
        "used external ontology subset projection",
    )?;
    let used_external_subset_quads = unique_quads(used_external_subset_quads.iter(), seen_quads);
    if used_external_subset_quads.is_empty() {
        return Ok(());
    }

    output.push_str(
        "# -----------------------------------------------------------------------------\n",
    );
    output.push_str("# Source: reqvire:external-used-subset\n");
    output.push_str("# Name: Reqvire used external ontology subset\n");
    output.push_str("# Kind: external-used-subset\n\n");
    let turtle = serialize_quads_turtle(&used_external_subset_quads)?;
    if !turtle.trim().is_empty() {
        output.push_str(turtle.trim());
        output.push_str("\n\n");
    }
    Ok(())
}

fn serialize_turtle_as_format(
    turtle: &str,
    format: SemanticExportFormat,
    label: &str,
) -> Result<String, ReqvireError> {
    match format {
        SemanticExportFormat::Turtle => Ok(turtle.to_string()),
        SemanticExportFormat::JsonLd => {
            let mut serializer = RdfSerializer::from_format(RdfFormat::JsonLd {
                profile: JsonLdProfileSet::empty(),
            })
            .for_writer(Vec::new());

            for parsed in RdfParser::from_format(RdfFormat::Turtle).for_reader(turtle.as_bytes()) {
                let quad = parsed.map_err(|error| {
                    ReqvireError::SerializationError(format!(
                        "{} failed to parse as Turtle: {}",
                        label, error
                    ))
                })?;
                serializer
                    .serialize_quad(quad.as_ref())
                    .map_err(|e| ReqvireError::SerializationError(e.to_string()))?;
            }

            let bytes = serializer
                .finish()
                .map_err(|e| ReqvireError::SerializationError(e.to_string()))?;
            String::from_utf8(bytes).map_err(|e| ReqvireError::SerializationError(e.to_string()))
        }
    }
}

fn skos_concept_iris(index: &SemanticIndex) -> BTreeSet<String> {
    index
        .blocks
        .iter()
        .filter(|block| matches!(block.kind, SemanticBlockKind::Concepts))
        .flat_map(|block| block.quads.iter())
        .filter_map(|quad| {
            if quad.predicate.as_str() == RDF_TYPE
                && matches!(
                    term_iri(&quad.object),
                    Some(SKOS_CONCEPT) | Some(SKOS_CONCEPT_SCHEME)
                )
            {
                subject_iri(&quad.subject).map(str::to_string)
            } else {
                None
            }
        })
        .collect()
}

fn is_concept_layer_quad(
    block_kind: SemanticBlockKind,
    quad: &Quad,
    concept_iris: &BTreeSet<String>,
    include_mappings: bool,
) -> bool {
    let subject = subject_iri(&quad.subject);
    let object = term_iri(&quad.object);
    let predicate = quad.predicate.as_str();

    if predicate == REQVIRE_MAPS_TO_CONCEPT {
        return include_mappings && object.is_some_and(|iri| concept_iris.contains(iri));
    }

    if !matches!(block_kind, SemanticBlockKind::Concepts) {
        return false;
    }

    subject.is_some_and(|iri| concept_iris.contains(iri))
        || (predicate.starts_with(SKOS_NS) && object.is_some_and(|iri| concept_iris.contains(iri)))
}

fn quad_keys_from_turtle(turtle: &str) -> Result<BTreeSet<String>, ReqvireError> {
    let mut keys = BTreeSet::new();
    if turtle.trim().is_empty() {
        return Ok(keys);
    }
    for quad in quads_from_turtle(turtle, "Generated ontology document declarations")? {
        keys.insert(quad_key(&quad));
    }
    Ok(keys)
}

fn quads_from_turtle(turtle: &str, label: &str) -> Result<Vec<Quad>, ReqvireError> {
    let mut quads = Vec::new();
    if turtle.trim().is_empty() {
        return Ok(quads);
    }
    for parsed in RdfParser::from_format(RdfFormat::Turtle).for_reader(turtle.as_bytes()) {
        let quad = parsed.map_err(|error| {
            ReqvireError::SerializationError(format!(
                "{} failed to parse as Turtle: {}",
                label, error
            ))
        })?;
        quads.push(quad);
    }
    Ok(quads)
}

fn serialize_quads_turtle(quads: &[&Quad]) -> Result<String, ReqvireError> {
    let mut serializer = RdfSerializer::from_format(RdfFormat::Turtle).for_writer(Vec::new());
    for quad in quads {
        serializer
            .serialize_quad((*quad).as_ref())
            .map_err(|e| ReqvireError::SerializationError(e.to_string()))?;
    }
    let bytes = serializer
        .finish()
        .map_err(|e| ReqvireError::SerializationError(e.to_string()))?;
    String::from_utf8(bytes).map_err(|e| ReqvireError::SerializationError(e.to_string()))
}

fn materialize_used_external_subset_turtle(index: &SemanticIndex) -> Result<String, ReqvireError> {
    if index.external_sources.is_empty() || index.external_blocks.is_empty() {
        return Ok(String::new());
    }

    let store = build_external_subset_derivation_store(index)?;
    let subset = o_kernel::subset::build_external_dependency_subset(
        &store,
        [
            GRAPH_AUTHORED_ONTOLOGY,
            GRAPH_AUTHORED_MODEL,
            GRAPH_GENERATED,
        ],
        [GRAPH_RAW_EXTERNAL_SOURCE],
    )
    .map_err(|error| {
        ReqvireError::ProcessError(format!(
            "Failed to build external dependency subset: {}",
            error
        ))
    })?;

    let public_subset_quads = subset
        .quads
        .into_iter()
        .filter(|entry| !is_external_source_metadata_quad(&entry.quad))
        .collect::<Vec<_>>();

    if public_subset_quads.is_empty() {
        return Ok(String::new());
    }

    serialize_constructed_subset_turtle(&public_subset_quads)
}

fn is_external_source_metadata_quad(quad: &Quad) -> bool {
    quad.predicate.as_str() == "http://www.w3.org/2000/01/rdf-schema#isDefinedBy"
        || (quad.predicate.as_str() == "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"
            && matches!(
                &quad.object,
                Term::NamedNode(node) if node.as_str() == "http://www.w3.org/2002/07/owl#Ontology"
            ))
}

fn build_external_subset_derivation_store(index: &SemanticIndex) -> Result<Store, ReqvireError> {
    let store = Store::new().map_err(|error| {
        ReqvireError::ProcessError(format!(
            "Failed to create external subset derivation store: {}",
            error
        ))
    })?;

    let authored_ontology_turtle = index.to_authored_ontology_layer_turtle_string()?;
    load_default_graph(
        &store,
        &authored_ontology_turtle,
        "authored ontology graph for external subset derivation",
    )?;
    load_named_graph(
        &store,
        &authored_ontology_turtle,
        GRAPH_AUTHORED_ONTOLOGY,
        "authored ontology graph for external subset derivation",
    )?;

    load_default_graph(
        &store,
        &index.model_context_turtle,
        "model context graph for external subset derivation",
    )?;
    load_named_graph(
        &store,
        &index.model_context_turtle,
        GRAPH_AUTHORED_MODEL,
        "model context graph for external subset derivation",
    )?;

    let ontology_projection_turtle = index.to_ontology_projection_turtle_string();
    load_default_graph(
        &store,
        &ontology_projection_turtle,
        "ontology projection graph for external subset derivation",
    )?;
    load_named_graph(
        &store,
        &ontology_projection_turtle,
        GRAPH_GENERATED,
        "ontology projection graph for external subset derivation",
    )?;

    let raw_external_turtle = index.to_raw_external_turtle_string()?;
    load_named_graph(
        &store,
        &raw_external_turtle,
        GRAPH_RAW_EXTERNAL_SOURCE,
        "raw external source graph for external subset derivation",
    )?;

    Ok(store)
}

fn load_default_graph(store: &Store, turtle: &str, label: &str) -> Result<(), ReqvireError> {
    if turtle.trim().is_empty() {
        return Ok(());
    }
    store
        .load_from_reader(
            RdfParser::from_format(RdfFormat::Turtle).without_named_graphs(),
            turtle.as_bytes(),
        )
        .map_err(|error| {
            ReqvireError::ProcessError(format!("Failed to load {} into Oxigraph: {}", label, error))
        })
}

fn load_named_graph(
    store: &Store,
    turtle: &str,
    graph_iri: &str,
    label: &str,
) -> Result<(), ReqvireError> {
    if turtle.trim().is_empty() {
        return Ok(());
    }
    let graph_name = NamedNode::new(graph_iri).map_err(|error| {
        ReqvireError::ProcessError(format!(
            "Invalid semantic named graph IRI '{}': {}",
            graph_iri, error
        ))
    })?;
    store
        .load_from_reader(
            RdfParser::from_format(RdfFormat::Turtle)
                .without_named_graphs()
                .with_default_graph(graph_name),
            turtle.as_bytes(),
        )
        .map_err(|error| {
            ReqvireError::ProcessError(format!("Failed to load {} into Oxigraph: {}", label, error))
        })
}

fn serialize_constructed_subset_turtle(
    quads: &[o_kernel::subset::ConstructedQuad],
) -> Result<String, ReqvireError> {
    let mut serializer = RdfSerializer::from_format(RdfFormat::Turtle).for_writer(Vec::new());
    let mut seen_quads = BTreeSet::new();
    for quad in quads {
        let key = quad_key(&quad.quad);
        if seen_quads.insert(key) {
            let triple = Triple::new(
                quad.quad.subject.clone(),
                quad.quad.predicate.clone(),
                quad.quad.object.clone(),
            );
            serializer
                .serialize_triple(triple.as_ref())
                .map_err(|e| ReqvireError::SerializationError(e.to_string()))?;
        }
    }
    let bytes = serializer
        .finish()
        .map_err(|e| ReqvireError::SerializationError(e.to_string()))?;
    String::from_utf8(bytes).map_err(|e| ReqvireError::SerializationError(e.to_string()))
}

fn quad_key(quad: &Quad) -> String {
    format!("{:?}", quad)
}

fn build_authored_model_turtle(
    registry: &GraphRegistry,
    index: &SemanticIndex,
) -> Result<String, ReqvireError> {
    let mut output = String::new();
    let mut artifacts = BTreeSet::new();
    let concept_reference_prefixes = full_context_ontology_prefixes(index);
    output.push_str(
        "# -----------------------------------------------------------------------------\n",
    );
    output.push_str("# Reqvire authored model context\n\n");
    output.push_str("@prefix owl: <http://www.w3.org/2002/07/owl#> .\n");
    output.push_str("@prefix reqvire: <https://www.reqvire.org/ontology#> .\n");
    output.push_str("\n");

    let mut nodes: Vec<_> = registry.nodes.values().collect();
    nodes.sort_by(|a, b| a.element.identifier.cmp(&b.element.identifier));

    for node in nodes {
        let element = &node.element;
        let subject = element_iri(element);

        output.push_str(&format!(
            "{} a {} ;\n",
            subject,
            element_type_classes(&element.element_type).join(", ")
        ));
        output.push_str(&format!(
            "  reqvire:elementId {} ;\n",
            turtle_string(&element.id)
        ));
        output.push_str(&format!(
            "  reqvire:elementIdentifier {} ;\n",
            turtle_string(&element.identifier)
        ));
        output.push_str(&format!(
            "  reqvire:elementName {} ;\n",
            turtle_string(&element.name)
        ));
        output.push_str(&format!(
            "  reqvire:elementType {} ;\n",
            turtle_string(&element.element_type.to_metadata_string())
        ));
        output.push_str(&format!(
            "  reqvire:filePath {} ;\n",
            turtle_string(&element.file_path)
        ));
        output.push_str(&format!(
            "  reqvire:lineNumber {} .\n\n",
            element.line_number
        ));

        for key in GOVERNANCE_METADATA_KEYS {
            if let Some(value) = element.metadata.get(*key) {
                output.push_str(&format!(
                    "{} reqvire:{} {} .\n",
                    subject,
                    key,
                    turtle_string(value)
                ));
            }
        }

        if let Some(ontology) = &element.ontology {
            if let Some(block) = &ontology.ontology {
                output.push_str(&format!(
                    "{} reqvire:ontologyText {} .\n",
                    subject,
                    turtle_string(&block.content)
                ));
            }
        }
        if let Some(contract) = &element.semantic_contract {
            output.push_str(&format!(
                "{} reqvire:semanticContractIri {} .\n",
                subject,
                turtle_string(&contract.iri)
            ));
            output.push_str(&format!(
                "{} reqvire:semanticContractKind \"semantic-contract\" .\n",
                subject
            ));
            if let Some(block) = &contract.shapes {
                output.push_str(&format!(
                    "{} reqvire:shapesText {} .\n",
                    subject,
                    turtle_string(&block.content)
                ));
            }
        }

        for relation in &element.relations {
            let Some(target_iri) =
                target_iri_for_link(&relation.target.link, registry, &mut artifacts)
            else {
                continue;
            };
            output.push_str(&format!(
                "{} reqvire:{} {} .\n",
                subject, relation.relation_type.name, target_iri
            ));
        }

        for reused_contract_context in &element.reused_contract_context {
            let Some(target_iri) = reused_contract_context_target_iri(
                &reused_contract_context.target,
                registry,
                &mut artifacts,
            ) else {
                continue;
            };
            output.push_str(&format!(
                "{} reqvire:reusesContract {} .\n",
                subject, target_iri
            ));
        }

        for reference in &element.concept_references {
            let resolved_iri = resolve_full_context_concept_reference_iri(
                &reference.iri,
                &concept_reference_prefixes,
            )
            .unwrap_or_else(|_| reference.iri.clone());
            output.push_str(&format!(
                "{} reqvire:conceptReference <{}> .\n",
                subject,
                escape_iri(&resolved_iri)
            ));
            output.push_str(&format!(
                "{} reqvire:referencesTerm <{}> .\n",
                subject,
                escape_iri(&resolved_iri)
            ));
        }

        if !element.relations.is_empty()
            || !element.reused_contract_context.is_empty()
            || !element.concept_references.is_empty()
        {
            output.push('\n');
        }
    }

    let mut external_sources = index.external_sources.clone();
    external_sources.sort_by(|a, b| {
        a.owner_identifier
            .cmp(&b.owner_identifier)
            .then_with(|| a.prefix.cmp(&b.prefix))
            .then_with(|| a.namespace.cmp(&b.namespace))
    });
    for source in external_sources {
        output.push_str(&external_ontology_source_turtle(&source));
    }

    output.push('\n');
    Ok(output)
}

fn build_generated_model_turtle(
    registry: &GraphRegistry,
    index: &SemanticIndex,
) -> Result<String, ReqvireError> {
    let mut output = String::new();
    output.push_str(
        "# -----------------------------------------------------------------------------\n",
    );
    output.push_str("# Reqvire generated ontology and model context\n\n");
    output.push_str("@prefix owl: <http://www.w3.org/2002/07/owl#> .\n");
    output.push_str("@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n");
    output.push_str("@prefix reqvire: <https://www.reqvire.org/ontology#> .\n");
    output.push_str("\n");
    output.push_str(&build_ontology_document_declarations_turtle(
        &index.ontology_documents,
    ));
    output.push_str(&build_ontology_term_definitions_turtle(
        &index.ontology_documents,
        &index.ontology_declarations,
        &index.blocks,
    ));
    output.push_str(&build_ontology_projection_turtle(index));

    let mut artifacts = BTreeSet::new();
    let mut nodes: Vec<_> = registry.nodes.values().collect();
    nodes.sort_by(|a, b| a.element.identifier.cmp(&b.element.identifier));

    for node in nodes {
        let element = &node.element;
        let subject = element_iri(element);

        for relation in &element.relations {
            let Some(target_iri) =
                target_iri_for_link(&relation.target.link, registry, &mut artifacts)
            else {
                continue;
            };
            append_model_relation_turtle(
                &mut output,
                &subject,
                relation.relation_type.name,
                &target_iri,
                relation.target.link.as_str(),
            );
            append_normalized_relation_family_turtle(
                &mut output,
                &subject,
                &target_iri,
                relation.relation_type.name,
            );
        }

        for reused_contract_context in &element.reused_contract_context {
            let Some(target_iri) = reused_contract_context_target_iri(
                &reused_contract_context.target,
                registry,
                &mut artifacts,
            ) else {
                continue;
            };
            append_model_relation_turtle(
                &mut output,
                &subject,
                "reused_contract_context",
                &target_iri,
                &reused_contract_context.target.as_str(),
            );
            append_normalized_relation_family_turtle(
                &mut output,
                &subject,
                &target_iri,
                "reused_contract_context",
            );
        }
    }

    for artifact in artifacts {
        output.push_str(&artifact);
    }

    let mut declarations: Vec<_> = index
        .ontology_declarations
        .values()
        .flat_map(|declarations| declarations.iter())
        .collect();
    declarations.sort_by(|a, b| {
        a.element_identifier
            .cmp(&b.element_identifier)
            .then_with(|| a.iri.cmp(&b.iri))
            .then_with(|| a.role.cmp(&b.role))
    });
    for declaration in declarations {
        if declaration.external {
            continue;
        }
        if let Some(node) = registry.nodes.get(&declaration.element_identifier) {
            output.push_str(&format!(
                "{} reqvire:declaresTerm <{}> .\n",
                element_iri(&node.element),
                escape_iri(&declaration.iri)
            ));
        }
    }

    let mut shape_references = index.shape_references.clone();
    shape_references.sort();
    for reference in shape_references {
        if let Some(node) = registry.nodes.get(&reference.element_identifier) {
            output.push_str(&format!(
                "{} reqvire:referencesTerm <{}> .\n",
                element_iri(&node.element),
                escape_iri(&reference.iri)
            ));
        }
    }

    output.push('\n');
    Ok(output)
}

fn build_model_context_turtle(registry: &GraphRegistry, index: &SemanticIndex) -> String {
    let mut output = String::new();
    let mut artifacts = BTreeSet::new();
    let concept_reference_prefixes = full_context_ontology_prefixes(index);
    output.push_str(
        "# -----------------------------------------------------------------------------\n",
    );
    output.push_str("# Reqvire full semantic model context\n\n");
    output.push_str("@prefix owl: <http://www.w3.org/2002/07/owl#> .\n");
    output.push_str("@prefix reqvire: <https://www.reqvire.org/ontology#> .\n\n");

    let mut nodes: Vec<_> = registry.nodes.values().collect();
    nodes.sort_by(|a, b| a.element.identifier.cmp(&b.element.identifier));

    for node in nodes {
        let element = &node.element;

        let subject = element_iri(element);
        output.push_str(&format!(
            "{} a {} ;\n",
            subject,
            element_type_classes(&element.element_type).join(", ")
        ));
        output.push_str(&format!(
            "  reqvire:elementId {} ;\n",
            turtle_string(&element.id)
        ));
        output.push_str(&format!(
            "  reqvire:elementIdentifier {} ;\n",
            turtle_string(&element.identifier)
        ));
        output.push_str(&format!(
            "  reqvire:elementName {} ;\n",
            turtle_string(&element.name)
        ));
        output.push_str(&format!(
            "  reqvire:elementType {} ;\n",
            turtle_string(&element.element_type.to_metadata_string())
        ));
        output.push_str(&format!(
            "  reqvire:filePath {} ;\n",
            turtle_string(&element.file_path)
        ));
        output.push_str(&format!(
            "  reqvire:lineNumber {} .\n\n",
            element.line_number
        ));

        for key in GOVERNANCE_METADATA_KEYS {
            if let Some(value) = element.metadata.get(*key) {
                output.push_str(&format!(
                    "{} reqvire:{} {} .\n",
                    subject,
                    key,
                    turtle_string(value)
                ));
            }
        }

        if let Some(ontology) = &element.ontology {
            if let Some(block) = &ontology.ontology {
                output.push_str(&format!(
                    "{} reqvire:ontologyText {} .\n",
                    subject,
                    turtle_string(&block.content)
                ));
            }
        }
        if let Some(contract) = &element.semantic_contract {
            output.push_str(&format!(
                "{} reqvire:semanticContractIri {} .\n",
                subject,
                turtle_string(&contract.iri)
            ));
            output.push_str(&format!(
                "{} reqvire:semanticContractKind \"semantic-contract\" .\n",
                subject
            ));
            if let Some(block) = &contract.shapes {
                output.push_str(&format!(
                    "{} reqvire:shapesText {} .\n",
                    subject,
                    turtle_string(&block.content)
                ));
            }
        }

        for relation in &element.relations {
            let Some(target_iri) =
                target_iri_for_link(&relation.target.link, registry, &mut artifacts)
            else {
                continue;
            };
            output.push_str(&format!(
                "{} reqvire:{} {} .\n",
                subject, relation.relation_type.name, target_iri
            ));
            append_model_relation_turtle(
                &mut output,
                &subject,
                relation.relation_type.name,
                &target_iri,
                relation.target.link.as_str(),
            );
            output.push_str(&format!(
                "{} reqvire:relationTarget {} .\n",
                subject, target_iri
            ));
            append_normalized_relation_family_turtle(
                &mut output,
                &subject,
                &target_iri,
                relation.relation_type.name,
            );
        }

        for reused_contract_context in &element.reused_contract_context {
            let Some(target_iri) = reused_contract_context_target_iri(
                &reused_contract_context.target,
                registry,
                &mut artifacts,
            ) else {
                continue;
            };
            output.push_str(&format!(
                "{} reqvire:reusesContract {} .\n",
                subject, target_iri
            ));
            let target_identifier = reused_contract_context.target.as_str();
            append_model_relation_turtle(
                &mut output,
                &subject,
                "reused_contract_context",
                &target_iri,
                &target_identifier,
            );
            append_normalized_relation_family_turtle(
                &mut output,
                &subject,
                &target_iri,
                "reused_contract_context",
            );
        }

        for reference in &element.concept_references {
            let resolved_iri = resolve_full_context_concept_reference_iri(
                &reference.iri,
                &concept_reference_prefixes,
            )
            .unwrap_or_else(|_| reference.iri.clone());
            output.push_str(&format!(
                "{} reqvire:conceptReference <{}> .\n",
                subject,
                escape_iri(&resolved_iri)
            ));
            output.push_str(&format!(
                "{} reqvire:referencesTerm <{}> .\n",
                subject,
                escape_iri(&resolved_iri)
            ));
        }

        if !element.relations.is_empty()
            || !element.reused_contract_context.is_empty()
            || !element.concept_references.is_empty()
        {
            output.push('\n');
        }
    }

    for artifact in artifacts {
        output.push_str(&artifact);
    }

    let mut declarations: Vec<_> = index
        .ontology_declarations
        .values()
        .flat_map(|declarations| declarations.iter())
        .collect();
    declarations.sort_by(|a, b| {
        a.element_identifier
            .cmp(&b.element_identifier)
            .then_with(|| a.iri.cmp(&b.iri))
            .then_with(|| a.role.cmp(&b.role))
    });
    for declaration in declarations {
        if declaration.external {
            continue;
        }
        if let Some(node) = registry.nodes.get(&declaration.element_identifier) {
            output.push_str(&format!(
                "{} reqvire:declaresTerm <{}> .\n",
                element_iri(&node.element),
                escape_iri(&declaration.iri)
            ));
        }
    }

    let mut external_sources = index.external_sources.clone();
    external_sources.sort_by(|a, b| {
        a.owner_identifier
            .cmp(&b.owner_identifier)
            .then_with(|| a.prefix.cmp(&b.prefix))
            .then_with(|| a.namespace.cmp(&b.namespace))
    });
    for source in external_sources {
        output.push_str(&external_ontology_source_turtle(&source));
    }

    let mut shape_references = index.shape_references.clone();
    shape_references.sort();
    for reference in shape_references {
        if let Some(node) = registry.nodes.get(&reference.element_identifier) {
            output.push_str(&format!(
                "{} reqvire:referencesTerm <{}> .\n",
                element_iri(&node.element),
                escape_iri(&reference.iri)
            ));
        }
    }

    output.push('\n');
    output
}

fn external_ontology_source_turtle(source: &ExternalOntologySource) -> String {
    let source_iri = external_ontology_source_iri(source);
    let owner_iri = element_iri_from_identifier(&source.owner_identifier);
    let mut output = String::new();
    let source_classes = if source.builtin {
        "reqvire:ExternalOntologySource, reqvire:BuiltInExternalOntologySource"
    } else {
        "reqvire:ExternalOntologySource"
    };
    output.push_str(&format!("{} a {} ;\n", source_iri, source_classes));
    output.push_str(&format!(
        "  reqvire:externalOntologyOwner {} ;\n",
        owner_iri
    ));
    if let Some(resource) = &source.resource {
        output.push_str(&format!(
            "  reqvire:externalOntologyResource <{}> ;\n",
            escape_iri(resource)
        ));
    }
    output.push_str(&format!(
        "  reqvire:externalOntologyPrefix {} ;\n",
        turtle_string(&source.prefix)
    ));
    output.push_str(&format!(
        "  reqvire:externalOntologyNamespace {} ;\n",
        turtle_string(&source.namespace)
    ));
    output.push_str(&format!(
        "  reqvire:externalOntologySourcePath {} ;\n",
        turtle_string(&source.source)
    ));
    if source.builtin {
        output.push_str("  reqvire:builtinExternalOntology true ;\n");
    }
    output.push_str(&format!(
        "  reqvire:externalOntologyFormat {} .\n\n",
        turtle_string(&source.format)
    ));
    output
}

fn external_ontology_source_iri(source: &ExternalOntologySource) -> String {
    projection_generated_iri(
        "external-ontology-source",
        &format!(
            "{}\n{}\n{}\n{}",
            source.owner_identifier, source.prefix, source.namespace, source.source
        ),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RelationProjectionDirection {
    Forward,
    Inverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NormalizedRelationProjection {
    forward_property: &'static str,
    inverse_property: &'static str,
    direction: RelationProjectionDirection,
}

fn normalized_relation_projection(relation_name: &str) -> Option<NormalizedRelationProjection> {
    let projection = match relation_name {
        "derive" => NormalizedRelationProjection {
            forward_property: "childElement",
            inverse_property: "parentElement",
            direction: RelationProjectionDirection::Forward,
        },
        "derivedFrom" => NormalizedRelationProjection {
            forward_property: "childElement",
            inverse_property: "parentElement",
            direction: RelationProjectionDirection::Inverse,
        },
        "specifiedBy" => NormalizedRelationProjection {
            forward_property: "capabilitySpecifiedByRequirement",
            inverse_property: "requirementSpecifiesCapability",
            direction: RelationProjectionDirection::Forward,
        },
        "specify" => NormalizedRelationProjection {
            forward_property: "capabilitySpecifiedByRequirement",
            inverse_property: "requirementSpecifiesCapability",
            direction: RelationProjectionDirection::Inverse,
        },
        "definedBy" => NormalizedRelationProjection {
            forward_property: "requirementDefinedByContract",
            inverse_property: "contractDefinesRequirement",
            direction: RelationProjectionDirection::Forward,
        },
        "define" => NormalizedRelationProjection {
            forward_property: "requirementDefinedByContract",
            inverse_property: "contractDefinesRequirement",
            direction: RelationProjectionDirection::Inverse,
        },
        "constrainedBy" => NormalizedRelationProjection {
            forward_property: "requirementConstrainedBySemanticContract",
            inverse_property: "semanticContractConstrainsRequirement",
            direction: RelationProjectionDirection::Forward,
        },
        "constrain" => NormalizedRelationProjection {
            forward_property: "requirementConstrainedBySemanticContract",
            inverse_property: "semanticContractConstrainsRequirement",
            direction: RelationProjectionDirection::Inverse,
        },
        "use" => NormalizedRelationProjection {
            forward_property: "semanticContractUsesOntology",
            inverse_property: "ontologyUsedBySemanticContract",
            direction: RelationProjectionDirection::Forward,
        },
        "usedBy" => NormalizedRelationProjection {
            forward_property: "semanticContractUsesOntology",
            inverse_property: "ontologyUsedBySemanticContract",
            direction: RelationProjectionDirection::Inverse,
        },
        "verifiedBy" => NormalizedRelationProjection {
            forward_property: "requirementVerifiedByVerification",
            inverse_property: "verificationVerifiesRequirement",
            direction: RelationProjectionDirection::Forward,
        },
        "verify" => NormalizedRelationProjection {
            forward_property: "requirementVerifiedByVerification",
            inverse_property: "verificationVerifiesRequirement",
            direction: RelationProjectionDirection::Inverse,
        },
        "satisfiedBy" => NormalizedRelationProjection {
            forward_property: "elementSatisfiedByArtifact",
            inverse_property: "artifactSatisfiesElement",
            direction: RelationProjectionDirection::Forward,
        },
        "satisfy" => NormalizedRelationProjection {
            forward_property: "elementSatisfiedByArtifact",
            inverse_property: "artifactSatisfiesElement",
            direction: RelationProjectionDirection::Inverse,
        },
        "reused_contract_context" => NormalizedRelationProjection {
            forward_property: "requirementUsesCrossSubgraphContract",
            inverse_property: "crossSubgraphContractUsedByRequirement",
            direction: RelationProjectionDirection::Forward,
        },
        _ => return None,
    };
    Some(projection)
}

fn append_model_relation_turtle(
    output: &mut String,
    source_iri: &str,
    relation_name: &str,
    target_iri: &str,
    target_identifier: &str,
) {
    let relation_iri = model_relation_iri(source_iri, relation_name, target_iri);
    output.push_str(&format!(
        "{} a reqvire:ModelRelation ;\n  reqvire:relationSource {} ;\n  reqvire:relationTarget {} ;\n  reqvire:relationType {} ;\n  reqvire:relationTargetIdentifier {} .\n",
        relation_iri,
        source_iri,
        target_iri,
        turtle_string(relation_name),
        turtle_string(target_identifier)
    ));
}

fn append_normalized_relation_family_turtle(
    output: &mut String,
    source_iri: &str,
    target_iri: &str,
    relation_name: &str,
) {
    let Some(projection) = normalized_relation_projection(relation_name) else {
        return;
    };
    let (canonical_source, canonical_target) = match projection.direction {
        RelationProjectionDirection::Forward => (source_iri, target_iri),
        RelationProjectionDirection::Inverse => (target_iri, source_iri),
    };
    output.push_str(&format!(
        "{} reqvire:{} {} .\n",
        canonical_source, projection.forward_property, canonical_target
    ));
    output.push_str(&format!(
        "{} reqvire:{} {} .\n",
        canonical_target, projection.inverse_property, canonical_source
    ));
}

fn full_context_ontology_prefixes(index: &SemanticIndex) -> HashMap<String, String> {
    let mut prefixes = HashMap::new();

    for block in &index.blocks {
        if !matches!(
            block.kind,
            SemanticBlockKind::Ontology | SemanticBlockKind::Concepts
        ) {
            continue;
        }
        for (prefix, iri) in parse_turtle_prefix_declarations(&block.content) {
            prefixes.entry(prefix).or_insert(iri);
        }
    }

    for declaration in &index.ontology_documents {
        prefixes
            .entry(declaration.ontology_prefix.clone())
            .or_insert(declaration.term_namespace.clone());
    }

    prefixes
}

pub(crate) fn parse_turtle_prefix_declarations(content: &str) -> Vec<(String, String)> {
    let mut prefixes = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix("@prefix ") else {
            continue;
        };
        let mut parts = rest.split_whitespace();
        let Some(prefix_token) = parts.next() else {
            continue;
        };
        let Some(iri_token) = parts.next() else {
            continue;
        };
        let Some(prefix) = prefix_token.strip_suffix(':') else {
            continue;
        };
        let Some(iri) = iri_token
            .strip_prefix('<')
            .and_then(|value| value.strip_suffix('>'))
        else {
            continue;
        };
        prefixes.push((prefix.to_string(), iri.to_string()));
    }
    prefixes
}

fn resolve_full_context_concept_reference_iri(
    value: &str,
    prefixes: &HashMap<String, String>,
) -> Result<String, String> {
    let trimmed = value.trim();
    if let Some(iri) = trimmed
        .strip_prefix('<')
        .and_then(|value| value.strip_suffix('>'))
    {
        return Ok(iri.to_string());
    }
    if trimmed.starts_with("urn:")
        || trimmed.starts_with("http://")
        || trimmed.starts_with("https://")
    {
        return Ok(trimmed.to_string());
    }

    let Some((prefix, local)) = trimmed.split_once(':') else {
        return Err("expected absolute IRI, <IRI>, or CURIE".to_string());
    };
    let Some(base) = prefixes.get(prefix) else {
        return Err(format!("prefix '{}' is not declared", prefix));
    };
    if local.is_empty() {
        return Err("CURIE local name is empty".to_string());
    }
    Ok(format!("{}{}", base, local))
}

fn build_ontology_projection_turtle(index: &SemanticIndex) -> String {
    let graph = &index.ontology_projection;
    if graph.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    let mut term_resources = BTreeSet::new();
    let mut source_resources = BTreeSet::new();
    let mut provenance_resources = BTreeSet::new();
    let mut evidence_resources = BTreeSet::new();
    let mut member_resources = BTreeSet::new();

    output.push_str(
        "# -----------------------------------------------------------------------------\n",
    );
    output.push_str("# Reqvire generated ontology projection facts\n\n");
    output.push_str("@prefix reqvire: <https://www.reqvire.org/ontology#> .\n");
    output.push_str("\n");

    let graph_iri = projection_resource_iri(&graph.id);
    output.push_str(&format!(
        "{} a reqvire:OntologyProjectionGraph .\n",
        graph_iri
    ));
    output.push_str(&format!(
        "{} reqvire:projectionDerivationMode {} .\n",
        graph_iri,
        turtle_string(graph.derivation_mode.as_str())
    ));

    for projection in &graph.projections {
        let projection_iri = projection_resource_iri(&projection.id);
        output.push_str(&format!(
            "{} reqvire:ontologyConstructProjection {} .\n",
            graph_iri, projection_iri
        ));
    }
    for construct in &graph.constructs {
        output.push_str(&format!(
            "{} reqvire:projectedConstruct {} .\n",
            graph_iri,
            projection_resource_iri(&construct.id)
        ));
    }
    output.push('\n');

    for projection in &graph.projections {
        let projection_iri = projection_resource_iri(&projection.id);
        output.push_str(&format!(
            "{} a reqvire:OntologyConstructProjection .\n",
            projection_iri
        ));
        output.push_str(&format!(
            "{} reqvire:constructFamily {} .\n",
            projection_iri,
            turtle_string(projection.family.as_str())
        ));
        output.push_str(&format!(
            "{} reqvire:projectionDerivationMode {} .\n",
            projection_iri,
            turtle_string(projection.derivation_mode.as_str())
        ));
        for construct_id in &projection.construct_ids {
            output.push_str(&format!(
                "{} reqvire:projectedConstruct {} .\n",
                projection_iri,
                projection_resource_iri(construct_id)
            ));
        }
        output.push('\n');
    }

    for construct in &graph.constructs {
        let construct_iri = projection_resource_iri(&construct.id);
        let source_iri =
            serialize_projection_source(&construct.provenance.source, &mut source_resources);
        let provenance_iri = serialize_projection_provenance(
            &construct.id,
            &construct.provenance,
            &mut term_resources,
            &mut source_resources,
            &mut provenance_resources,
            &mut evidence_resources,
        );
        let subject_iri = serialize_projection_term(&construct.subject, &mut term_resources);

        output.push_str(&format!(
            "{} a reqvire:OntologyConstruct .\n",
            construct_iri
        ));
        output.push_str(&format!(
            "{} reqvire:constructFamily {} .\n",
            construct_iri,
            turtle_string(construct.family.as_str())
        ));
        output.push_str(&format!(
            "{} reqvire:constructKind {} .\n",
            construct_iri,
            turtle_string(construct.kind.as_str())
        ));
        output.push_str(&format!(
            "{} reqvire:projectionDerivationMode {} .\n",
            construct_iri,
            turtle_string(construct.provenance.derivation_mode.as_str())
        ));
        output.push_str(&format!(
            "{} reqvire:constructSourceBlock {} .\n",
            construct_iri, source_iri
        ));
        output.push_str(&format!(
            "{} reqvire:constructProvenance {} .\n",
            construct_iri, provenance_iri
        ));
        output.push_str(&format!(
            "{} reqvire:constructSubject {} .\n",
            construct_iri, subject_iri
        ));

        if let Some(predicate) = &construct.predicate {
            let predicate_iri = serialize_projection_term(predicate, &mut term_resources);
            output.push_str(&format!(
                "{} reqvire:constructPredicate {} .\n",
                construct_iri, predicate_iri
            ));
        }
        if let Some(object) = &construct.object {
            let object_iri = serialize_projection_term(object, &mut term_resources);
            output.push_str(&format!(
                "{} reqvire:constructObject {} .\n",
                construct_iri, object_iri
            ));
        }
        if let Some(property) = &construct.property {
            let property_iri = serialize_projection_term(property, &mut term_resources);
            output.push_str(&format!(
                "{} reqvire:constructProperty {} .\n",
                construct_iri, property_iri
            ));
        }
        if let Some(characteristic) = construct.property_characteristic {
            output.push_str(&format!(
                "{} reqvire:propertyCharacteristic {} .\n",
                construct_iri,
                turtle_string(characteristic.as_str())
            ));
        }
        if let Some(restriction_kind) = construct.restriction_kind {
            output.push_str(&format!(
                "{} reqvire:restrictionKind {} .\n",
                construct_iri,
                turtle_string(restriction_kind.as_str())
            ));
        }
        if let Some(class_expression_kind) = construct.class_expression_kind {
            output.push_str(&format!(
                "{} reqvire:classExpressionKind {} .\n",
                construct_iri,
                turtle_string(class_expression_kind.as_str())
            ));
        }
        if let Some(shape_overlay_kind) = construct.shape_overlay_kind {
            output.push_str(&format!(
                "{} reqvire:shapeOverlayKind {} .\n",
                construct_iri,
                turtle_string(shape_overlay_kind.as_str())
            ));
        }
        for member in &construct.members {
            let member_iri = serialize_construct_member(
                &construct.id,
                member,
                &mut term_resources,
                &mut source_resources,
                &mut member_resources,
            );
            output.push_str(&format!(
                "{} reqvire:constructMember {} .\n",
                construct_iri, member_iri
            ));
        }
        output.push('\n');
    }

    for resource in source_resources {
        output.push_str(&resource);
    }
    for resource in provenance_resources {
        output.push_str(&resource);
    }
    for resource in evidence_resources {
        output.push_str(&resource);
    }
    for resource in member_resources {
        output.push_str(&resource);
    }
    for resource in term_resources {
        output.push_str(&resource);
    }

    output
}

fn build_ontology_document_declarations_turtle(
    declarations: &[OntologyDocumentDeclaration],
) -> String {
    if declarations.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    output.push_str(
        "# -----------------------------------------------------------------------------\n",
    );
    output.push_str("# Reqvire generated ontology document declarations\n\n");
    output.push_str("@prefix owl: <http://www.w3.org/2002/07/owl#> .\n");
    output.push_str("@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n");
    output.push_str("@prefix reqvire: <https://www.reqvire.org/ontology#> .\n\n");

    for declaration in declarations {
        output.push_str(&format!(
            "<{}> a owl:Ontology ;\n",
            escape_iri(&declaration.iri)
        ));
        output.push_str(&format!(
            "  rdfs:label {} ;\n",
            turtle_string(&declaration.ontology_prefix)
        ));
        for element_identifier in &declaration.element_identifiers {
            output.push_str(&format!(
                "  reqvire:ontologyElement {} ;\n",
                element_iri_from_identifier(element_identifier)
            ));
        }
        output.push_str(&format!(
            "  reqvire:ontologyBase {} ;\n",
            turtle_string(&declaration.ontology_base)
        ));
        output.push_str(&format!(
            "  reqvire:ontologyPrefix {} ;\n",
            turtle_string(&declaration.ontology_prefix)
        ));
        output.push_str(&format!(
            "  reqvire:termNamespace {}",
            turtle_string(&declaration.term_namespace)
        ));
        for import_iri in &declaration.imports {
            output.push_str(&format!(" ;\n  owl:imports <{}>", escape_iri(import_iri)));
        }
        output.push_str(" .\n\n");
    }

    output
}

fn build_ontology_term_definitions_turtle(
    documents: &[OntologyDocumentDeclaration],
    declarations: &HashMap<String, Vec<OntologyTermDeclaration>>,
    blocks: &[SemanticBlock],
) -> String {
    let mut element_documents = HashMap::new();
    for document in documents {
        for element_identifier in &document.element_identifiers {
            element_documents.insert(
                element_identifier.as_str(),
                (document.iri.as_str(), document.term_namespace.as_str()),
            );
        }
    }

    let mut edges = BTreeSet::new();
    for (term_iri, term_declarations) in declarations {
        for declaration in term_declarations {
            if declaration.external {
                continue;
            }
            let Some((document_iri, term_namespace)) =
                element_documents.get(declaration.element_identifier.as_str())
            else {
                continue;
            };
            if term_iri == *document_iri || !term_iri.starts_with(*term_namespace) {
                continue;
            }
            edges.insert((term_iri.clone(), (*document_iri).to_string()));
        }
    }
    for (term_iri, document_iri) in authored_ontology_subject_definition_edges(documents, blocks) {
        edges.insert((term_iri, document_iri));
    }

    if edges.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    output.push_str(
        "# -----------------------------------------------------------------------------\n",
    );
    output.push_str("# Reqvire generated ontology term definition links\n\n");
    for (term_iri, document_iri) in edges {
        output.push_str(&format!(
            "<{}> rdfs:isDefinedBy <{}> .\n",
            escape_iri(&term_iri),
            escape_iri(&document_iri)
        ));
    }
    output.push('\n');
    output
}

fn authored_ontology_subject_definition_edges(
    documents: &[OntologyDocumentDeclaration],
    blocks: &[SemanticBlock],
) -> BTreeSet<(String, String)> {
    let mut element_documents = HashMap::new();
    for document in documents {
        for element_identifier in &document.element_identifiers {
            element_documents.insert(
                element_identifier.as_str(),
                (document.iri.as_str(), document.term_namespace.as_str()),
            );
        }
    }

    let mut edges = BTreeSet::new();
    for block in blocks {
        if !matches!(block.kind, SemanticBlockKind::Ontology) {
            continue;
        }
        let Some((document_iri, term_namespace)) = element_documents.get(block.source.as_str())
        else {
            continue;
        };
        for quad in &block.quads {
            let Some(subject) = subject_iri(&quad.subject) else {
                continue;
            };
            if subject == *document_iri || !subject.starts_with(*term_namespace) {
                continue;
            }
            edges.insert((subject.to_string(), (*document_iri).to_string()));
        }
    }
    edges
}

fn serialize_projection_provenance(
    construct_id: &str,
    provenance: &OntologyProjectionProvenance,
    term_resources: &mut BTreeSet<String>,
    source_resources: &mut BTreeSet<String>,
    provenance_resources: &mut BTreeSet<String>,
    evidence_resources: &mut BTreeSet<String>,
) -> String {
    let provenance_iri = projection_generated_iri("ontology-provenance", construct_id);
    let source_iri = serialize_projection_source(&provenance.source, source_resources);
    let mut chunk = String::new();
    chunk.push_str(&format!(
        "{} a reqvire:OntologyProjectionProvenance .\n",
        provenance_iri
    ));
    chunk.push_str(&format!(
        "{} reqvire:projectionDerivationMode {} .\n",
        provenance_iri,
        turtle_string(provenance.derivation_mode.as_str())
    ));
    chunk.push_str(&format!(
        "{} reqvire:provenanceSource {} .\n",
        provenance_iri, source_iri
    ));
    for (index, evidence) in provenance.evidence.iter().enumerate() {
        let evidence_iri = serialize_projection_evidence(
            construct_id,
            index,
            evidence,
            term_resources,
            source_resources,
            evidence_resources,
        );
        chunk.push_str(&format!(
            "{} reqvire:provenanceEvidence {} .\n",
            provenance_iri, evidence_iri
        ));
    }
    chunk.push('\n');
    provenance_resources.insert(chunk);
    provenance_iri
}

fn serialize_projection_evidence(
    construct_id: &str,
    sequence_index: usize,
    evidence: &OntologyProjectionEvidence,
    term_resources: &mut BTreeSet<String>,
    source_resources: &mut BTreeSet<String>,
    evidence_resources: &mut BTreeSet<String>,
) -> String {
    let evidence_key = format!(
        "{}|{}|{}|{}|{}",
        construct_id,
        sequence_index,
        evidence.source.source_block,
        projection_term_key(&evidence.subject),
        projection_term_key(&evidence.object)
    );
    let evidence_iri = projection_generated_iri("ontology-evidence", &evidence_key);
    let source_iri = serialize_projection_source(&evidence.source, source_resources);
    let subject_iri = serialize_projection_term(&evidence.subject, term_resources);
    let predicate_iri = serialize_projection_term(&evidence.predicate, term_resources);
    let object_iri = serialize_projection_term(&evidence.object, term_resources);

    evidence_resources.insert(format!(
        "{} a reqvire:OntologyProjectionEvidence .\n{} reqvire:constructSourceBlock {} .\n{} reqvire:constructSubject {} .\n{} reqvire:constructPredicate {} .\n{} reqvire:constructObject {} .\n\n",
        evidence_iri,
        evidence_iri,
        source_iri,
        evidence_iri,
        subject_iri,
        evidence_iri,
        predicate_iri,
        evidence_iri,
        object_iri
    ));

    evidence_iri
}

fn serialize_construct_member(
    construct_id: &str,
    member: &OntologyConstructMember,
    term_resources: &mut BTreeSet<String>,
    source_resources: &mut BTreeSet<String>,
    member_resources: &mut BTreeSet<String>,
) -> String {
    let member_key = format!(
        "{}|{}|{}",
        construct_id,
        member.sequence_index,
        projection_term_key(&member.term)
    );
    let member_iri = projection_generated_iri("ontology-member", &member_key);
    let term_iri = serialize_projection_term(&member.term, term_resources);
    let source_iri = serialize_projection_source(&member.source, source_resources);

    member_resources.insert(format!(
        "{} a reqvire:OntologyConstructMember .\n{} reqvire:memberTerm {} .\n{} reqvire:constructSourceBlock {} .\n{} reqvire:constructSequenceIndex {} .\n\n",
        member_iri,
        member_iri,
        term_iri,
        member_iri,
        source_iri,
        member_iri,
        member.sequence_index
    ));

    member_iri
}

fn serialize_projection_term(
    term: &OntologyProjectionTerm,
    term_resources: &mut BTreeSet<String>,
) -> String {
    let term_iri = projection_term_iri(term);
    term_resources.insert(format!(
        "{} a reqvire:OntologyTerm .\n{} reqvire:termKind {} .\n{} reqvire:termValue {} .\n{} reqvire:conceptLabel {} .\n\n",
        term_iri,
        term_iri,
        turtle_string(term.kind.as_str()),
        term_iri,
        turtle_string(&term.value),
        term_iri,
        turtle_string(&term.label)
    ));
    term_iri
}

fn serialize_projection_source(
    source: &OntologyProjectionSource,
    source_resources: &mut BTreeSet<String>,
) -> String {
    let source_iri = projection_generated_iri("semantic-block", &source.source_block);
    source_resources.insert(format!(
        "{} a reqvire:SemanticBlock, reqvire:OntologyProjectionSource .\n{} reqvire:sourceBlockId {} .\n{} reqvire:sourceElementIdentifier {} .\n{} reqvire:sourceName {} .\n{} reqvire:filePath {} .\n{} reqvire:lineNumber {} .\n{} reqvire:blockKind {} .\n\n",
        source_iri,
        source_iri,
        turtle_string(&source.source_block),
        source_iri,
        turtle_string(&source.source_element_identifier),
        source_iri,
        turtle_string(&source.source_name),
        source_iri,
        turtle_string(&source.file_path),
        source_iri,
        source.line_number,
        source_iri,
        turtle_string(&source.block_kind)
    ));
    source_iri
}

fn projection_resource_iri(value: &str) -> String {
    format!("<{}>", escape_iri(value))
}

fn projection_generated_iri(kind: &str, canonical: &str) -> String {
    projection_resource_iri(&format!("urn:reqvire:{}:{}", kind, stable_hash(canonical)))
}

fn projection_term_iri(term: &OntologyProjectionTerm) -> String {
    match term.kind {
        OntologyProjectionTermKind::Iri => projection_resource_iri(&term.value),
        OntologyProjectionTermKind::BlankNode | OntologyProjectionTermKind::Literal => {
            projection_generated_iri("ontology-term", &projection_term_key(term))
        }
    }
}

fn element_iri(element: &Element) -> String {
    format!("<{}>", element_iri_value(element))
}

fn element_iri_value(element: &Element) -> String {
    format!("urn:reqvire:element:{}", escape_iri(&element.id))
}

fn element_iri_from_identifier(identifier: &str) -> String {
    let element_id = crate::utils::extract_path_and_fragment(identifier)
        .1
        .unwrap_or(identifier);
    format!("<urn:reqvire:element:{}>", escape_iri(element_id))
}

fn element_type_classes(element_type: &ElementType) -> Vec<&'static str> {
    match element_type {
        ElementType::Capability => {
            vec!["owl:NamedIndividual", "reqvire:Element", "reqvire:Capability"]
        }
        ElementType::Requirement(_) => {
            vec!["owl:NamedIndividual", "reqvire:Element", "reqvire:Requirement"]
        }
        ElementType::Ontology => {
            vec!["owl:NamedIndividual", "reqvire:Element", "reqvire:Ontology"]
        }
        ElementType::ConceptScheme => {
            vec![
                "owl:NamedIndividual",
                "reqvire:Element",
                "reqvire:ConceptScheme",
            ]
        }
        ElementType::Concept => {
            vec!["owl:NamedIndividual", "reqvire:Element", "reqvire:Concept"]
        }
        ElementType::Verification(verification_type) => {
            let subtype = match verification_type {
                VerificationType::Default | VerificationType::Test => "reqvire:TestVerification",
                VerificationType::FormalProof => "reqvire:FormalProofVerification",
                VerificationType::Analysis => "reqvire:AnalysisVerification",
                VerificationType::Inspection => "reqvire:InspectionVerification",
                VerificationType::Demonstration => "reqvire:DemonstrationVerification",
            };
            vec![
                "owl:NamedIndividual",
                "reqvire:Element",
                "reqvire:Verification",
                subtype,
            ]
        }
        ElementType::VerificationObjective => {
            vec![
                "owl:NamedIndividual",
                "reqvire:Element",
                "reqvire:VerificationObjective",
            ]
        }
        ElementType::SemanticContract => {
            vec![
                "owl:NamedIndividual",
                "reqvire:Element",
                "reqvire:SemanticContract",
            ]
        }
        ElementType::Contract(contract_type) => {
            let subtype = match contract_type {
                ContractType::Source => "reqvire:Source",
                ContractType::Constraint => "reqvire:Constraint",
                ContractType::Behavior => "reqvire:Behavior",
                ContractType::Specification => "reqvire:Specification",
                ContractType::State => "reqvire:State",
                ContractType::InputOutput => "reqvire:InputOutput",
            };
            vec![
                "owl:NamedIndividual",
                "reqvire:Element",
                "reqvire:Contract",
                subtype,
            ]
        }
        ElementType::File => {
            vec!["owl:NamedIndividual", "reqvire:Artifact", "reqvire:File"]
        }
        ElementType::Other(_) => {
            vec!["owl:NamedIndividual", "reqvire:Element", "reqvire:CustomElement"]
        }
    }
}

fn target_iri_for_link(
    link: &LinkType,
    registry: &GraphRegistry,
    artifacts: &mut BTreeSet<String>,
) -> Option<String> {
    match link {
        LinkType::Identifier(target_identifier) => registry
            .nodes
            .get(target_identifier)
            .map(|target| element_iri(&target.element)),
        LinkType::InternalPath(path) => {
            let value = path.to_string_lossy();
            let iri = artifact_iri("path", &value);
            artifacts.insert(format!(
                "{} a owl:NamedIndividual, reqvire:Artifact, reqvire:File ;\n  reqvire:filePath {} .\n\n",
                iri,
                turtle_string(&value)
            ));
            Some(iri)
        }
        LinkType::ExternalUrl(url) => {
            let iri = artifact_iri("url", url);
            artifacts.insert(format!(
                "{} a owl:NamedIndividual, reqvire:Artifact ;\n  reqvire:externalUrl {} .\n\n",
                iri,
                turtle_string(url)
            ));
            Some(iri)
        }
    }
}

fn reused_contract_context_target_iri(
    target: &ReusedContractContextTarget,
    registry: &GraphRegistry,
    artifacts: &mut BTreeSet<String>,
) -> Option<String> {
    match target {
        ReusedContractContextTarget::ElementIdentifier(target_identifier) => registry
            .nodes
            .get(target_identifier)
            .map(|target| element_iri(&target.element)),
        ReusedContractContextTarget::FilePath(path) => {
            let value = path.to_string_lossy();
            let iri = artifact_iri("path", &value);
            artifacts.insert(format!(
                "{} a owl:NamedIndividual, reqvire:Artifact, reqvire:File ;\n  reqvire:filePath {} .\n\n",
                iri,
                turtle_string(&value)
            ));
            Some(iri)
        }
    }
}

fn artifact_iri(kind: &str, value: &str) -> String {
    format!("<urn:reqvire:artifact:{}:{}>", kind, escape_iri(value))
}

fn model_relation_iri(source_iri: &str, relation_name: &str, target_iri: &str) -> String {
    let canonical = format!("{}|{}|{}", source_iri, relation_name, target_iri);
    format!("<urn:reqvire:model-relation:{}>", stable_hash(&canonical))
}

fn turtle_string(value: &str) -> String {
    format!(
        "\"{}\"",
        value
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
    )
}

fn escape_iri(value: &str) -> String {
    value
        .replace('\\', "%5C")
        .replace(' ', "%20")
        .replace('<', "%3C")
        .replace('>', "%3E")
        .replace('"', "%22")
        .replace('{', "%7B")
        .replace('}', "%7D")
        .replace('|', "%7C")
        .replace('^', "%5E")
        .replace('`', "%60")
}

fn extract_external_ontology_sources(
    element: &Element,
) -> (Vec<ExternalOntologySource>, Vec<String>) {
    let (parsed_sources, diagnostics) =
        crate::parser::parse_external_ontology_sources(&element.content);
    let sources = parsed_sources
        .into_iter()
        .map(|source| ExternalOntologySource {
            owner_identifier: element.identifier.clone(),
            owner_name: element.name.clone(),
            prefix: source.prefix,
            namespace: source.namespace,
            resource: Some(source.resource),
            source: source.source,
            format: source.format,
            line_number: source.line_number,
            builtin: false,
        })
        .collect();

    (sources, diagnostics)
}

fn builtin_external_ontology_sources() -> Vec<ExternalOntologySource> {
    crate::builtin_external_sources::EXTERNAL_ONTOLOGIES
        .iter()
        .map(|source| ExternalOntologySource {
            owner_identifier: format!("builtin:{}", source.id),
            owner_name: format!("Built-in {}", source.prefix),
            prefix: source.prefix.to_string(),
            namespace: source.namespace.to_string(),
            resource: Some(source.resource.to_string()),
            source: source.source.to_string(),
            format: source.format.to_string(),
            line_number: 0,
            builtin: true,
        })
        .collect()
}

fn builtin_external_ontology_source_content(
    source: &ExternalOntologySource,
) -> Option<&'static str> {
    crate::builtin_external_sources::EXTERNAL_ONTOLOGIES
        .iter()
        .find(|builtin| builtin.source == source.source)
        .map(|builtin| builtin.content)
}

fn builtin_external_ontology_block(
    source: &ExternalOntologySource,
    diagnostics: &mut Vec<SemanticDiagnostic>,
) -> Option<SemanticBlock> {
    let Some(content) = builtin_external_ontology_source_content(source) else {
        diagnostics.push(SemanticDiagnostic {
            source: source.owner_identifier.clone(),
            file_path: source.source.clone(),
            line_number: source.line_number,
            message: format!(
                "Built-in External Ontology '{}' source '{}' is not registered.",
                source.prefix, source.source
            ),
        });
        return None;
    };

    let format = match ExternalOntologyFormat::parse(&source.format) {
        Some(format) => format,
        None => {
            diagnostics.push(SemanticDiagnostic {
                source: source.owner_identifier.clone(),
                file_path: source.source.clone(),
                line_number: source.line_number,
                message: format!(
                    "Built-in External Ontology '{}' uses unsupported format '{}'.",
                    source.prefix, source.format
                ),
            });
            return None;
        }
    };

    let quads = match parse_external_ontology_block(content, format) {
        Ok(quads) => quads,
        Err(message) => {
            diagnostics.push(SemanticDiagnostic {
                source: source.owner_identifier.clone(),
                file_path: source.source.clone(),
                line_number: source.line_number,
                message: format!(
                    "Built-in External Ontology '{}' source '{}' failed to parse as {}: {}.",
                    source.prefix,
                    source.source,
                    format.display_name(),
                    message
                ),
            });
            return None;
        }
    };

    Some(SemanticBlock {
        kind: SemanticBlockKind::ExternalOntology,
        source: external_ontology_block_source(source),
        source_name: format!("Built-in external ontology {}", source.prefix),
        file_path: source.source.clone(),
        line_number: source.line_number,
        language: format.language().to_string(),
        external_materialization: None,
        content: content.to_string(),
        quads,
    })
}

fn resolve_external_source_path(element: &Element, source: &str) -> Result<PathBuf, String> {
    if source.starts_with("http://") || source.starts_with("https://") {
        return Err("source must be a local path; network fetches are not supported".to_string());
    }

    let source_path = Path::new(source);
    if source_path.is_absolute() {
        return Ok(source_path.to_path_buf());
    }

    let git_root = crate::git_commands::get_git_root_dir()
        .or_else(|_| {
            std::env::current_dir().map_err(|error| ReqvireError::PathError(error.to_string()))
        })
        .map_err(|error| error.to_string())?;

    let root_relative = git_root.join(source_path);
    if root_relative.exists() {
        return Ok(root_relative);
    }

    Ok(git_root
        .join(
            Path::new(&element.file_path)
                .parent()
                .unwrap_or_else(|| Path::new("")),
        )
        .join(source_path))
}

fn build_external_ontology_block(
    element: &Element,
    source: &ExternalOntologySource,
    diagnostics: &mut Vec<SemanticDiagnostic>,
) -> Option<SemanticBlock> {
    let format = match ExternalOntologyFormat::parse(&source.format) {
        Some(format) => format,
        None => {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: source.line_number,
                message: format!(
                    "External Ontology '{}' uses unsupported format '{}'. Supported formats: turtle, ttl, rdf, rdfxml, rdf+xml, jsonld.",
                    source.prefix, source.format
                ),
            });
            return None;
        }
    };

    let path = match resolve_external_source_path(element, &source.source) {
        Ok(path) => path,
        Err(message) => {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: source.line_number,
                message: format!(
                    "External Ontology '{}' source '{}' cannot be resolved: {}.",
                    source.prefix, source.source, message
                ),
            });
            return None;
        }
    };

    let content = match std::fs::read_to_string(&path) {
        Ok(content) => content,
        Err(error) => {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: source.line_number,
                message: format!(
                    "External Ontology '{}' source '{}' cannot be read: {}.",
                    source.prefix,
                    path.display(),
                    error
                ),
            });
            return None;
        }
    };

    if matches!(format, ExternalOntologyFormat::Turtle)
        && turtle_prefix_binding(&content, &source.prefix) != Some(source.namespace.clone())
    {
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: source.line_number,
            message: format!(
                "External Ontology '{}' source '{}' must explicitly declare `@prefix {}: <{}> .`.",
                source.prefix, source.source, source.prefix, source.namespace
            ),
        });
    }

    let quads = match parse_external_ontology_block(&content, format) {
        Ok(quads) => quads,
        Err(message) => {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: source.line_number,
                message: format!(
                    "External Ontology '{}' source '{}' failed to parse as {}: {}.",
                    source.prefix,
                    source.source,
                    format.display_name(),
                    message
                ),
            });
            return None;
        }
    };

    if let Some(resource) = &source.resource {
        if !has_ontology_declaration(&quads, resource) {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: source.line_number,
                message: format!(
                    "External Ontology '{}' source '{}' must declare <{}> a owl:Ontology.",
                    source.prefix, source.source, resource
                ),
            });
        }
    }

    if !graph_mentions_namespace(&quads, &source.namespace) {
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: source.line_number,
            message: format!(
                "External Ontology '{}' source '{}' does not declare or reference any term in namespace '{}'.",
                source.prefix, source.source, source.namespace
            ),
        });
    }

    Some(SemanticBlock {
        kind: SemanticBlockKind::ExternalOntology,
        source: external_ontology_block_source(source),
        source_name: format!("{} external ontology {}", element.name, source.prefix),
        file_path: path.to_string_lossy().to_string(),
        line_number: source.line_number,
        language: format.language().to_string(),
        external_materialization: None,
        content,
        quads,
    })
}

fn external_ontology_block_source(source: &ExternalOntologySource) -> String {
    if source.builtin {
        format!("builtin:external-ontology-{}", source.prefix)
    } else {
        format!(
            "{}#external-ontology-{}",
            source.owner_identifier, source.prefix
        )
    }
}

fn known_class_iris(
    declarations: &HashMap<String, Vec<OntologyTermDeclaration>>,
) -> BTreeSet<String> {
    declarations
        .iter()
        .filter(|(_, declarations)| {
            declarations
                .iter()
                .any(|declaration| declaration.role == OntologyTermRole::Class)
        })
        .map(|(iri, _)| iri.clone())
        .collect()
}

fn graph_mentions_namespace(quads: &[Quad], namespace: &str) -> bool {
    quads.iter().any(|quad| {
        subject_iri(&quad.subject).is_some_and(|iri| iri.starts_with(namespace))
            || quad.predicate.as_str().starts_with(namespace)
            || term_iri(&quad.object).is_some_and(|iri| iri.starts_with(namespace))
    })
}

pub fn build_semantic_index(registry: &GraphRegistry) -> SemanticIndex {
    let mut blocks = Vec::new();
    let mut external_blocks = Vec::new();
    let mut external_sources = Vec::new();
    let mut diagnostics = Vec::new();
    let ontology_documents = build_ontology_document_declarations(registry);
    let mut ontology_declarations: HashMap<String, Vec<OntologyTermDeclaration>> = HashMap::new();
    let mut shape_references = Vec::new();
    let mut external_prefixes: HashMap<String, String> = HashMap::new();
    let mut external_namespaces: HashMap<String, String> = HashMap::new();

    for source in builtin_external_ontology_sources() {
        external_prefixes.insert(source.prefix.clone(), source.namespace.clone());
        external_namespaces.insert(source.namespace.clone(), source.prefix.clone());
        if let Some(block) = builtin_external_ontology_block(&source, &mut diagnostics) {
            for declaration in ontology_term_declarations_from_quads_with_source(
                &source.owner_identifier,
                &block.quads,
                true,
            ) {
                ontology_declarations
                    .entry(declaration.iri.clone())
                    .or_default()
                    .push(declaration);
            }
            external_blocks.push(block);
        }
        external_sources.push(source);
    }

    for element in registry.get_all_elements() {
        let ontology =
            crate::parser::extract_single_fenced_subsection(&element.content, "Ontology");
        let shapes = crate::parser::extract_single_fenced_subsection(&element.content, "Shapes");
        let query = crate::parser::extract_single_fenced_subsection(&element.content, "Query");

        validate_semantic_sections(element, &ontology, &shapes, &query, &mut diagnostics);

        if element.element_type.is_ontology() {
            let canonical_prefix = canonical_ontology_prefix(registry, &element.identifier);
            if let Some(block) = build_block(
                element,
                SemanticBlockKind::Ontology,
                ontology.first(),
                "Ontology",
                canonical_prefix.as_ref(),
                &mut diagnostics,
            ) {
                validate_ontology_source_contract(
                    element,
                    canonical_prefix.as_ref(),
                    &block,
                    &mut diagnostics,
                );
                let known_class_iris = known_class_iris(&ontology_declarations);
                for declaration in
                    ontology_term_declarations_from_quads(element, &block.quads, &known_class_iris)
                {
                    ontology_declarations
                        .entry(declaration.iri.clone())
                        .or_default()
                        .push(declaration);
                }
                blocks.push(block);
            }

            let (sources, source_diagnostics) = extract_external_ontology_sources(element);
            for message in source_diagnostics {
                diagnostics.push(SemanticDiagnostic {
                    source: element.identifier.clone(),
                    file_path: element.file_path.clone(),
                    line_number: element.line_number,
                    message,
                });
            }
            for source in sources {
                if let Some(existing_namespace) = external_prefixes.get(&source.prefix) {
                    if existing_namespace != &source.namespace {
                        diagnostics.push(SemanticDiagnostic {
                            source: element.identifier.clone(),
                            file_path: element.file_path.clone(),
                            line_number: source.line_number,
                            message: format!(
                                "External Ontology prefix '{}' is already bound to '{}', but this section binds it to '{}'.",
                                source.prefix, existing_namespace, source.namespace
                            ),
                        });
                    }
                } else {
                    external_prefixes.insert(source.prefix.clone(), source.namespace.clone());
                }

                if let Some(existing_prefix) = external_namespaces.get(&source.namespace) {
                    if existing_prefix != &source.prefix {
                        diagnostics.push(SemanticDiagnostic {
                            source: element.identifier.clone(),
                            file_path: element.file_path.clone(),
                            line_number: source.line_number,
                            message: format!(
                                "External Ontology namespace '{}' is already bound to prefix '{}', but this section binds it to '{}'. Prefix aliases are not supported.",
                                source.namespace, existing_prefix, source.prefix
                            ),
                        });
                    }
                } else {
                    external_namespaces.insert(source.namespace.clone(), source.prefix.clone());
                }

                if let Some(block) =
                    build_external_ontology_block(element, &source, &mut diagnostics)
                {
                    for declaration in
                        external_ontology_term_declarations_from_quads(element, &block.quads)
                    {
                        ontology_declarations
                            .entry(declaration.iri.clone())
                            .or_default()
                            .push(declaration);
                    }
                    external_blocks.push(block);
                }
                external_sources.push(source);
            }
            continue;
        }

        if element.element_type.is_semantic_contract() {
            if let Some(block) = build_block(
                element,
                SemanticBlockKind::Shapes,
                shapes.first(),
                "Shapes",
                None,
                &mut diagnostics,
            ) {
                let shacl_registry = shacl::ShaclRegistry::parse(&block.quads);
                for message in shacl_registry.diagnostics_as_messages() {
                    diagnostics.push(SemanticDiagnostic {
                        source: element.identifier.clone(),
                        file_path: element.file_path.clone(),
                        line_number: block.line_number,
                        message: format!("Shapes SHACL sanity validation failed: {}", message),
                    });
                }

                shape_references.extend(shape_iri_references_from_quads(element, &block.quads));
                blocks.push(block);
            }
            continue;
        }

        if element.element_type.is_concept_family() {
            if let Some(block) = build_generated_concept_block(registry, element, &mut diagnostics)
            {
                blocks.push(block);
            }
        }
    }

    let ontology_blocks = blocks
        .iter()
        .filter(|block| matches!(block.kind, SemanticBlockKind::Ontology))
        .count();
    let shape_blocks = blocks
        .iter()
        .filter(|block| matches!(block.kind, SemanticBlockKind::Shapes))
        .count();
    let total_quads = blocks.iter().map(|block| block.quads.len()).sum();
    let ontology_projection = build_ontology_projection(registry, &blocks);

    let mut index = SemanticIndex {
        summary: SemanticIndexSummary {
            ontology_blocks,
            shape_blocks,
            total_blocks: blocks.len(),
            total_quads,
        },
        blocks,
        external_blocks,
        external_sources,
        diagnostics,
        ontology_documents,
        ontology_declarations,
        shape_references,
        ontology_projection,
        model_context: ModelContextGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        },
        model_context_turtle: String::new(),
    };
    index.model_context = build_model_context_graph(registry, &index);
    index.model_context_turtle = build_model_context_turtle(registry, &index);
    index
}

fn build_generated_concept_block(
    registry: &GraphRegistry,
    element: &Element,
    diagnostics: &mut Vec<SemanticDiagnostic>,
) -> Option<SemanticBlock> {
    let content = match generated_concept_turtle(registry, element) {
        Ok(Some(content)) => content,
        Ok(None) => return None,
        Err(message) => {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: element.line_number,
                message,
            });
            return None;
        }
    };
    let quads = match quads_from_turtle(&content, "Generated native concept RDF") {
        Ok(quads) => quads,
        Err(error) => {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: element.line_number,
                message: error.to_string(),
            });
            return None;
        }
    };
    Some(SemanticBlock {
        kind: SemanticBlockKind::Concepts,
        source: element.identifier.clone(),
        source_name: element.name.clone(),
        file_path: element.file_path.clone(),
        line_number: element.line_number,
        language: "turtle".to_string(),
        external_materialization: None,
        content,
        quads,
    })
}

fn generated_concept_turtle(
    registry: &GraphRegistry,
    element: &Element,
) -> Result<Option<String>, String> {
    if element.element_type.is_concept_scheme() {
        let Some(scheme) = &element.concept_scheme else {
            return Ok(None);
        };
        let Some((prefix, namespace)) = concept_namespace_context(registry, element) else {
            return Err(format!(
                "Concept scheme '{}' must define concept_base and concept_prefix metadata.",
                element.name
            ));
        };
        let subject = concept_curie(&prefix, element);
        let mut turtle = concept_prefix_block(&prefix, &namespace);
        turtle.push_str(&format!("{} a skos:ConceptScheme ;\n", subject));
        turtle.push_str(&format!(
            "  skos:prefLabel \"{}\"",
            turtle_literal(&scheme.pref_label)
        ));
        if let Some(definition) = &scheme.definition {
            turtle.push_str(&format!(
                " ;\n  skos:definition \"{}\"",
                turtle_literal(definition)
            ));
        }
        for top in &scheme.top_concepts {
            turtle.push_str(&format!(
                " ;\n  skos:hasTopConcept {}",
                concept_link_object(registry, &prefix, &top.target, &top.label)
            ));
        }
        turtle.push_str(" .\n");
        return Ok(Some(turtle));
    }

    if element.element_type.is_concept() {
        let Some(concept) = &element.concept else {
            return Ok(None);
        };
        let Some((prefix, namespace)) = concept_namespace_context(registry, element) else {
            return Err(format!(
                "Concept '{}' must derive scheme and namespace context from a concept-scheme element.",
                element.name
            ));
        };
        let Some(scheme) = concept_scheme_context(registry, element) else {
            return Err(format!(
                "Concept '{}' must derive from a concept-scheme or another concept with scheme context.",
                element.name
            ));
        };
        let subject = concept_curie(&prefix, element);
        let mut turtle = concept_prefix_block(&prefix, &namespace);
        turtle.push_str(&format!("{} a skos:Concept ;\n", subject));
        turtle.push_str(&format!(
            "  skos:inScheme {}",
            concept_curie(&prefix, scheme)
        ));
        turtle.push_str(&format!(
            " ;\n  skos:prefLabel \"{}\"",
            turtle_literal(&concept.pref_label)
        ));
        for label in &concept.labels {
            if matches!(label.kind.as_str(), "altLabel" | "hiddenLabel") {
                turtle.push_str(&format!(
                    " ;\n  skos:{} \"{}\"",
                    label.kind,
                    turtle_literal(&label.value)
                ));
            }
        }
        if let Some(definition) = &concept.definition {
            turtle.push_str(&format!(
                " ;\n  skos:definition \"{}\"",
                turtle_literal(definition)
            ));
        }
        if let Some(scope_note) = &concept.scope_note {
            turtle.push_str(&format!(
                " ;\n  skos:scopeNote \"{}\"",
                turtle_literal(scope_note)
            ));
        }
        for example in &concept.examples {
            turtle.push_str(&format!(
                " ;\n  skos:example \"{}\"",
                turtle_literal(&example.value)
            ));
        }
        for (predicate, object) in normalized_concept_relation_objects(registry, element, &prefix) {
            turtle.push_str(&format!(" ;\n  skos:{} {}", predicate, object));
        }
        turtle.push_str(" .\n");
        for (subject, predicate, object) in external_symmetric_concept_relation_triples(element, &prefix) {
            turtle.push_str(&format!("{} skos:{} {} .\n", subject, predicate, object));
        }
        return Ok(Some(turtle));
    }

    Ok(None)
}

fn concept_prefix_block(prefix: &str, namespace: &str) -> String {
    format!(
        "@prefix {}: <{}> .\n@prefix skos: <{}> .\n\n",
        prefix, namespace, SKOS_NS
    )
}

fn concept_namespace_context(
    registry: &GraphRegistry,
    element: &Element,
) -> Option<(String, String)> {
    let scheme = if element.element_type.is_concept_scheme() {
        Some(element)
    } else {
        concept_scheme_context(registry, element)
    }?;
    let base = scheme
        .metadata
        .get("concept_base")
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())?;
    let prefix = scheme
        .metadata
        .get("concept_prefix")
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())?;
    Some((prefix.to_string(), ontology_term_namespace(base)))
}

fn concept_scheme_context<'a>(
    registry: &'a GraphRegistry,
    element: &'a Element,
) -> Option<&'a Element> {
    if element.element_type.is_concept_scheme() {
        return Some(element);
    }
    let mut seen = BTreeSet::new();
    concept_scheme_context_recursive(registry, element, &mut seen)
}

fn concept_scheme_context_recursive<'a>(
    registry: &'a GraphRegistry,
    element: &'a Element,
    seen: &mut BTreeSet<String>,
) -> Option<&'a Element> {
    if !seen.insert(element.identifier.clone()) {
        return None;
    }
    for relation in &element.relations {
        if relation.relation_type.name != "derivedFrom" {
            continue;
        }
        let LinkType::Identifier(target_id) = &relation.target.link else {
            continue;
        };
        let target = registry.nodes.get(target_id).map(|node| &node.element)?;
        if target.element_type.is_concept_scheme() {
            return Some(target);
        }
        if target.element_type.is_concept() {
            if let Some(scheme) = concept_scheme_context_recursive(registry, target, seen) {
                return Some(scheme);
            }
        }
    }
    None
}

fn normalized_concept_relation_objects(
    registry: &GraphRegistry,
    element: &Element,
    prefix: &str,
) -> BTreeSet<(String, String)> {
    let mut output = BTreeSet::new();
    let current_id = element.identifier.as_str();

    for candidate in registry.get_all_elements() {
        let Some(concept) = candidate.concept.as_ref() else {
            continue;
        };
        let candidate_id = candidate.identifier.as_str();

        for link in &concept.broader {
            if candidate_id == current_id {
                output.insert((
                    "broader".to_string(),
                    concept_link_object(registry, prefix, &link.target, &link.label),
                ));
            }
            if concept_link_target_element(registry, &link.target)
                .is_some_and(|target| target.identifier.as_str() == current_id)
            {
                output.insert(("narrower".to_string(), concept_curie(prefix, candidate)));
            }
        }

        for link in &concept.narrower {
            if candidate_id == current_id {
                output.insert((
                    "narrower".to_string(),
                    concept_link_object(registry, prefix, &link.target, &link.label),
                ));
            }
            if concept_link_target_element(registry, &link.target)
                .is_some_and(|target| target.identifier.as_str() == current_id)
            {
                output.insert(("broader".to_string(), concept_curie(prefix, candidate)));
            }
        }

        append_symmetric_concept_relation_objects(
            registry,
            &mut output,
            current_id,
            candidate,
            "related",
            &concept.related,
            prefix,
        );
        append_symmetric_concept_relation_objects(
            registry,
            &mut output,
            current_id,
            candidate,
            "exactMatch",
            &concept.exact_match,
            prefix,
        );
        append_symmetric_concept_relation_objects(
            registry,
            &mut output,
            current_id,
            candidate,
            "closeMatch",
            &concept.close_match,
            prefix,
        );
    }

    output
}

fn append_symmetric_concept_relation_objects(
    registry: &GraphRegistry,
    output: &mut BTreeSet<(String, String)>,
    current_id: &str,
    candidate: &Element,
    predicate: &str,
    links: &[crate::element::ConceptLink],
    prefix: &str,
) {
    let candidate_id = candidate.identifier.as_str();
    for link in links {
        if candidate_id == current_id {
            output.insert((
                predicate.to_string(),
                concept_link_object(registry, prefix, &link.target, &link.label),
            ));
        }
        if concept_link_target_element(registry, &link.target)
            .is_some_and(|target| target.identifier.as_str() == current_id)
        {
            output.insert((predicate.to_string(), concept_curie(prefix, candidate)));
        }
    }
}

fn external_symmetric_concept_relation_triples(
    element: &Element,
    prefix: &str,
) -> BTreeSet<(String, String, String)> {
    let mut triples = BTreeSet::new();
    let Some(concept) = element.concept.as_ref() else {
        return triples;
    };
    let object = concept_curie(prefix, element);
    for (predicate, links) in [
        ("exactMatch", &concept.exact_match),
        ("closeMatch", &concept.close_match),
    ] {
        for link in links {
            if link.target.starts_with("http://") || link.target.starts_with("https://") {
                triples.insert((
                    format!("<{}>", link.target),
                    predicate.to_string(),
                    object.clone(),
                ));
            }
        }
    }
    triples
}

fn concept_link_target_element<'a>(
    registry: &'a GraphRegistry,
    target: &str,
) -> Option<&'a Element> {
    registry
        .nodes
        .get(target)
        .map(|node| &node.element)
        .or_else(|| {
            registry
                .nodes
                .values()
                .find(|node| node.element.identifier.ends_with(target))
                .map(|node| &node.element)
        })
}

fn concept_link_object(
    registry: &GraphRegistry,
    prefix: &str,
    target: &str,
    label: &str,
) -> String {
    if target.starts_with("http://") || target.starts_with("https://") {
        return format!("<{}>", target);
    }
    if let Some(element) = concept_link_target_element(registry, target) {
        return concept_curie(prefix, element);
    }
    format!("{}:{}", prefix, concept_local_name(label))
}

fn concept_curie(prefix: &str, element: &Element) -> String {
    format!("{}:{}", prefix, concept_local_name(&element.name))
}

fn concept_local_name(name: &str) -> String {
    let mut local = String::new();
    for part in name
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
    {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            local.push(first.to_ascii_uppercase());
            for ch in chars {
                local.push(ch);
            }
        }
    }
    if local.is_empty() {
        "Concept".to_string()
    } else {
        local
    }
}

fn turtle_literal(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn build_model_context_graph(registry: &GraphRegistry, index: &SemanticIndex) -> ModelContextGraph {
    let mut nodes: Vec<_> = registry
        .nodes
        .values()
        .map(|node| {
            let element = &node.element;
            ModelContextNode {
                id: element_iri_value(element),
                label: element.name.clone(),
                identifier: element.identifier.clone(),
                element_type: element.element_type.to_metadata_string(),
                rdf_types: element_type_classes(&element.element_type)
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                file_path: element.file_path.clone(),
                line_number: element.line_number,
            }
        })
        .collect();
    nodes.sort_by(|a, b| a.id.cmp(&b.id));

    let mut edges = BTreeSet::new();
    for node in registry.nodes.values() {
        let source = element_iri_value(&node.element);
        for relation in &node.element.relations {
            let Some(target) = model_context_relation_target_iri(relation, registry) else {
                continue;
            };
            let (source, target, label) = canonical_model_context_relation_edge(
                &source,
                &target,
                relation.relation_type.name,
            );
            edges.insert(ModelContextEdge {
                source,
                target,
                label,
            });
        }
    }

    let concept_reference_prefixes = full_context_ontology_prefixes(index);
    for node in registry.nodes.values() {
        let source = element_iri_value(&node.element);
        for reference in &node.element.concept_references {
            let target = resolve_full_context_concept_reference_iri(
                &reference.iri,
                &concept_reference_prefixes,
            )
            .unwrap_or_else(|_| reference.iri.clone());
            edges.insert(ModelContextEdge {
                source: source.clone(),
                target,
                label: "referencesTerm".to_string(),
            });
        }
    }

    for declarations in index.ontology_declarations.values() {
        for declaration in declarations {
            let Some(node) = registry.nodes.get(&declaration.element_identifier) else {
                continue;
            };
            edges.insert(ModelContextEdge {
                source: element_iri_value(&node.element),
                target: declaration.iri.clone(),
                label: "declaresTerm".to_string(),
            });
        }
    }

    for reference in &index.shape_references {
        let Some(node) = registry.nodes.get(&reference.element_identifier) else {
            continue;
        };
        edges.insert(ModelContextEdge {
            source: element_iri_value(&node.element),
            target: reference.iri.clone(),
            label: "referencesTerm".to_string(),
        });
    }

    ModelContextGraph {
        nodes,
        edges: edges.into_iter().collect(),
    }
}

fn model_context_relation_target_iri(
    relation: &crate::relation::Relation,
    registry: &GraphRegistry,
) -> Option<String> {
    if let Some(target_id) = relation.target.element_id.as_deref() {
        if let Some(target) = registry.nodes.get(target_id) {
            return Some(element_iri_value(&target.element));
        }
    }
    match &relation.target.link {
        LinkType::Identifier(target_id) => registry
            .nodes
            .get(target_id)
            .map(|target| element_iri_value(&target.element)),
        _ => None,
    }
}

fn canonical_model_context_relation_edge(
    source: &str,
    target: &str,
    relation_name: &str,
) -> (String, String, String) {
    if relation::DIAGRAM_RELATIONS.contains(&relation_name) {
        return (
            source.to_string(),
            target.to_string(),
            relation_name.to_string(),
        );
    }
    let canonical = relation::RELATION_TYPES
        .get(relation_name)
        .and_then(|info| info.opposite)
        .filter(|opposite| relation::DIAGRAM_RELATIONS.contains(opposite))
        .unwrap_or(relation_name);
    if canonical == relation_name {
        (
            source.to_string(),
            target.to_string(),
            relation_name.to_string(),
        )
    } else {
        (
            target.to_string(),
            source.to_string(),
            canonical.to_string(),
        )
    }
}

fn build_ontology_document_declarations(
    registry: &GraphRegistry,
) -> Vec<OntologyDocumentDeclaration> {
    let mut memo = HashMap::new();
    let mut prefix_memo = HashMap::new();
    let mut declarations: BTreeMap<String, OntologyDocumentAccumulator> = BTreeMap::new();
    let mut ontology_ids: Vec<String> = registry
        .nodes
        .values()
        .filter(|node| node.element.element_type.is_ontology())
        .map(|node| node.element.identifier.clone())
        .collect();
    ontology_ids.sort();

    for ontology_id in ontology_ids {
        let Some(node) = registry.nodes.get(&ontology_id) else {
            continue;
        };
        let Some(ontology_base) = resolve_ontology_base(registry, &ontology_id, &mut memo) else {
            continue;
        };
        let Some(ontology_prefix) =
            resolve_ontology_prefix(registry, &ontology_id, &mut prefix_memo)
        else {
            continue;
        };
        let term_namespace = ontology_term_namespace(&ontology_base);

        let declaration = declarations
            .entry(ontology_base.clone())
            .or_insert_with(|| OntologyDocumentAccumulator {
                ontology_base: ontology_base.clone(),
                ontology_prefix: ontology_prefix.clone(),
                term_namespace: term_namespace.clone(),
                element_identifiers: BTreeSet::new(),
                element_names: BTreeSet::new(),
                imports: BTreeSet::new(),
            });
        declaration
            .element_identifiers
            .insert(node.element.identifier.clone());
        declaration.element_names.insert(node.element.name.clone());
    }

    declarations
        .into_values()
        .map(|declaration| OntologyDocumentDeclaration {
            iri: ontology_document_iri(&declaration.ontology_base),
            ontology_base: declaration.ontology_base,
            ontology_prefix: declaration.ontology_prefix,
            term_namespace: declaration.term_namespace,
            element_identifiers: declaration.element_identifiers.into_iter().collect(),
            element_names: declaration.element_names.into_iter().collect(),
            imports: declaration.imports.into_iter().collect(),
        })
        .collect()
}

fn canonical_ontology_prefix(
    registry: &GraphRegistry,
    ontology_id: &str,
) -> Option<CanonicalOntologyPrefix> {
    let mut base_memo = HashMap::new();
    let mut prefix_memo = HashMap::new();
    let ontology_base = resolve_ontology_base(registry, ontology_id, &mut base_memo)?;
    let prefix = resolve_ontology_prefix(registry, ontology_id, &mut prefix_memo)?;
    let explicit_boundary = registry
        .nodes
        .get(ontology_id)
        .and_then(|node| node.element.metadata.get("ontology_base"))
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false);
    let ontology_document_iri = ontology_document_iri(&ontology_base);
    let required_imports = required_ontology_imports(registry, ontology_id, &ontology_base);
    Some(CanonicalOntologyPrefix {
        prefix,
        namespace: ontology_term_namespace(&ontology_base),
        ontology_base,
        ontology_document_iri,
        explicit_boundary,
        required_imports,
    })
}

fn required_ontology_imports(
    registry: &GraphRegistry,
    ontology_id: &str,
    ontology_base: &str,
) -> Vec<String> {
    let Some(node) = registry.nodes.get(ontology_id) else {
        return Vec::new();
    };
    let mut memo = HashMap::new();
    let mut imports = BTreeSet::new();

    for relation in node
        .element
        .relations
        .iter()
        .filter(|relation| relation.relation_type.name == "derivedFrom")
    {
        let LinkType::Identifier(target_id) = &relation.target.link else {
            continue;
        };
        let Some(target) = registry.nodes.get(target_id) else {
            continue;
        };
        if !target.element.element_type.is_ontology() {
            continue;
        }
        let Some(target_base) = resolve_ontology_base(registry, target_id, &mut memo) else {
            continue;
        };
        if target_base != ontology_base {
            imports.insert(ontology_document_iri(&target_base));
        }
    }

    imports.into_iter().collect()
}

fn resolve_ontology_base(
    registry: &GraphRegistry,
    ontology_id: &str,
    memo: &mut HashMap<String, Option<String>>,
) -> Option<String> {
    if let Some(cached) = memo.get(ontology_id) {
        return cached.clone();
    }
    memo.insert(ontology_id.to_string(), None);

    let Some(node) = registry.nodes.get(ontology_id) else {
        return None;
    };
    if let Some(base) = node
        .element
        .metadata
        .get("ontology_base")
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
    {
        let base = base.to_string();
        memo.insert(ontology_id.to_string(), Some(base.clone()));
        return Some(base);
    }

    let mut parent_ids: Vec<String> = node
        .element
        .relations
        .iter()
        .filter(|relation| relation.relation_type.name == "derivedFrom")
        .filter_map(|relation| match &relation.target.link {
            LinkType::Identifier(target_id) => registry
                .nodes
                .get(target_id)
                .filter(|target| target.element.element_type.is_ontology())
                .map(|_| target_id.clone()),
            _ => None,
        })
        .collect();
    parent_ids.sort();

    for parent_id in parent_ids {
        if let Some(base) = resolve_ontology_base(registry, &parent_id, memo) {
            memo.insert(ontology_id.to_string(), Some(base.clone()));
            return Some(base);
        }
    }

    None
}

fn resolve_ontology_prefix(
    registry: &GraphRegistry,
    ontology_id: &str,
    memo: &mut HashMap<String, Option<String>>,
) -> Option<String> {
    if let Some(cached) = memo.get(ontology_id) {
        return cached.clone();
    }
    memo.insert(ontology_id.to_string(), None);

    let Some(node) = registry.nodes.get(ontology_id) else {
        return None;
    };
    if let Some(prefix) = node
        .element
        .metadata
        .get("ontology_prefix")
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
    {
        let prefix = prefix.to_string();
        memo.insert(ontology_id.to_string(), Some(prefix.clone()));
        return Some(prefix);
    }

    let mut parent_ids: Vec<String> = node
        .element
        .relations
        .iter()
        .filter(|relation| relation.relation_type.name == "derivedFrom")
        .filter_map(|relation| match &relation.target.link {
            LinkType::Identifier(target_id) => registry
                .nodes
                .get(target_id)
                .filter(|target| target.element.element_type.is_ontology())
                .map(|_| target_id.clone()),
            _ => None,
        })
        .collect();
    parent_ids.sort();

    for parent_id in parent_ids {
        if let Some(prefix) = resolve_ontology_prefix(registry, &parent_id, memo) {
            memo.insert(ontology_id.to_string(), Some(prefix.clone()));
            return Some(prefix);
        }
    }

    None
}

fn ontology_document_iri(ontology_base: &str) -> String {
    ontology_base.trim_end_matches('/').to_string()
}

fn ontology_term_namespace(ontology_base: &str) -> String {
    format!("{}#", ontology_base.trim_end_matches('#'))
}

fn validate_semantic_sections(
    element: &Element,
    ontology: &[FencedBlock],
    shapes: &[FencedBlock],
    query: &[FencedBlock],
    diagnostics: &mut Vec<SemanticDiagnostic>,
) {
    let has_query_section = crate::parser::has_subsection(&element.content, "Query");
    let has_external_ontology_section =
        crate::parser::has_subsection(&element.content, "External Ontology");
    let query_line_number = query
        .first()
        .map(|block| block.line_number)
        .unwrap_or(element.line_number);

    if element.element_type.is_ontology() {
        if ontology.len() != 1 {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: element.line_number,
                message: format!(
                    "Ontology element '{}' must contain exactly one #### Ontology fenced Turtle block.",
                    element.name
                ),
            });
        }
        if !shapes.is_empty() {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: shapes[0].line_number,
                message: format!(
                    "Ontology element '{}' must not contain a #### Shapes section. SHACL profiles belong in semantic-contract elements.",
                    element.name
                ),
            });
        }
    } else if element.element_type.is_semantic_contract() {
        if !ontology.is_empty() {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: ontology[0].line_number,
                message: format!(
                    "Semantic contract '{}' must not contain a #### Ontology section. Semantic contracts are shapes-only profiles over reachable ontology elements.",
                    element.name
                ),
            });
        }
        if shapes.len() != 1 {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: element.line_number,
                message: format!(
                    "Semantic contract '{}' must contain exactly one #### Shapes fenced Turtle block.",
                    element.name
                ),
            });
        }
    } else {
        if !ontology.is_empty() {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: ontology[0].line_number,
                message: format!(
                    "Element '{}' is type '{}' and must not contain a #### Ontology section. Use type: ontology.",
                    element.name,
                    element.element_type.as_str()
                ),
            });
        }
        if !shapes.is_empty() {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: shapes[0].line_number,
                message: format!(
                    "Element '{}' is type '{}' and must not contain a #### Shapes section. Use type: semantic-contract.",
                    element.name,
                    element.element_type.as_str()
                ),
            });
        }
    }

    if has_external_ontology_section && !element.element_type.is_ontology() {
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: element.line_number,
            message: format!(
                "Element '{}' is type '{}' and must not contain a #### External Ontology section. External ontology sources belong on ontology elements.",
                element.name,
                element.element_type.as_str()
            ),
        });
    }

    if has_query_section {
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: query_line_number,
            message: format!(
                "Element '{}' is type '{}' and must not contain a #### Query section. No Reqvire element type currently supports this reserved subsection.",
                element.name,
                element.element_type.as_str()
            ),
        });
    }

    if ontology.len() > 1 {
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: ontology[1].line_number,
            message: format!(
                "Element '{}' must contain at most one #### Ontology fenced Turtle block.",
                element.name
            ),
        });
    }

    if shapes.len() > 1 {
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: shapes[1].line_number,
            message: format!(
                "Element '{}' must contain at most one #### Shapes fenced Turtle block.",
                element.name
            ),
        });
    }

    if query.len() > 1 {
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: query[1].line_number,
            message: format!(
                "Element '{}' must contain at most one #### Query fenced block.",
                element.name
            ),
        });
    }
}

fn build_block(
    element: &Element,
    kind: SemanticBlockKind,
    block: Option<&FencedBlock>,
    section_name: &str,
    canonical_prefix: Option<&CanonicalOntologyPrefix>,
    diagnostics: &mut Vec<SemanticDiagnostic>,
) -> Option<SemanticBlock> {
    let block = block?;
    if !validate_turtle_language(&block.language) {
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: block.line_number,
            message: format!(
                "{} fenced block must use language tag 'turtle' or 'ttl'.",
                section_name
            ),
        });
        return None;
    }

    let parse_content = match canonical_turtle_content(&block.content, canonical_prefix) {
        Ok(content) => content,
        Err(message) => {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: block.line_number,
                message,
            });
            return None;
        }
    };

    match parse_turtle_block(&parse_content) {
        Ok(quads) => Some(SemanticBlock {
            kind,
            source: element.identifier.clone(),
            source_name: element.name.clone(),
            file_path: element.file_path.clone(),
            line_number: block.line_number,
            language: block.language.clone(),
            external_materialization: None,
            content: block.content.clone(),
            quads,
        }),
        Err(message) => {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: block.line_number,
                message: format!("{} Turtle validation failed: {}", section_name, message),
            });
            None
        }
    }
}

fn canonical_turtle_content(
    content: &str,
    canonical_prefix: Option<&CanonicalOntologyPrefix>,
) -> Result<String, String> {
    let Some(canonical_prefix) = canonical_prefix else {
        return Ok(content.to_string());
    };

    if let Some(namespace) = turtle_prefix_binding(content, &canonical_prefix.prefix) {
        if namespace == canonical_prefix.namespace {
            return Ok(content.to_string());
        }
        return Err(format!(
            "Ontology Turtle prefix '{}' maps to '{}', but inherited ontology metadata requires '{}'.",
            canonical_prefix.prefix, namespace, canonical_prefix.namespace
        ));
    }

    Err(format!(
        "Ontology Turtle block must explicitly declare prefix '{}' as '{}'.",
        canonical_prefix.prefix, canonical_prefix.namespace
    ))
}

fn validate_ontology_source_contract(
    element: &Element,
    canonical_prefix: Option<&CanonicalOntologyPrefix>,
    block: &SemanticBlock,
    diagnostics: &mut Vec<SemanticDiagnostic>,
) {
    let Some(context) = canonical_prefix else {
        return;
    };

    if context.explicit_boundary
        && !has_ontology_declaration(&block.quads, &context.ontology_document_iri)
    {
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: block.line_number,
            message: format!(
                "Ontology boundary element '{}' with ontology_base '{}' must explicitly declare <{}> a owl:Ontology.",
                element.name, context.ontology_base, context.ontology_document_iri
            ),
        });
    }

    for import_iri in &context.required_imports {
        if has_ontology_import(&block.quads, &context.ontology_document_iri, import_iri) {
            continue;
        }
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: block.line_number,
            message: format!(
                "Ontology element '{}' has a derivedFrom relation crossing ontology boundaries and must explicitly declare <{}> owl:imports <{}>.",
                element.name, context.ontology_document_iri, import_iri
            ),
        });
    }

    let authored_terms = authored_ontology_subjects(
        &block.quads,
        &context.ontology_document_iri,
        &context.namespace,
    );
    for quad in &block.quads {
        let Some(subject) = subject_iri(&quad.subject) else {
            continue;
        };
        if quad.predicate.as_str() != RDFS_IS_DEFINED_BY || !authored_terms.contains(subject) {
            continue;
        }
        if term_iri(&quad.object) == Some(&context.ontology_document_iri) {
            continue;
        }
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: block.line_number,
            message: format!(
                "Ontology term <{}> is authored in ontology_base '{}' but has conflicting rdfs:isDefinedBy target. Authored Reqvire ontology terms must be defined by <{}>.",
                subject, context.ontology_base, context.ontology_document_iri
            ),
        });
    }
}

fn authored_ontology_subjects(
    quads: &[Quad],
    ontology_document_iri: &str,
    term_namespace: &str,
) -> BTreeSet<String> {
    quads
        .iter()
        .filter_map(|quad| subject_iri(&quad.subject))
        .filter(|subject| *subject != ontology_document_iri && subject.starts_with(term_namespace))
        .map(ToString::to_string)
        .collect()
}

fn has_ontology_declaration(quads: &[Quad], ontology_iri: &str) -> bool {
    quads.iter().any(|quad| {
        subject_iri(&quad.subject) == Some(ontology_iri)
            && quad.predicate.as_str() == RDF_TYPE
            && term_iri(&quad.object) == Some(OWL_ONTOLOGY)
    })
}

fn has_ontology_import(quads: &[Quad], ontology_iri: &str, import_iri: &str) -> bool {
    quads.iter().any(|quad| {
        subject_iri(&quad.subject) == Some(ontology_iri)
            && quad.predicate.as_str() == OWL_IMPORTS
            && term_iri(&quad.object) == Some(import_iri)
    })
}

fn turtle_prefix_binding(content: &str, expected_prefix: &str) -> Option<String> {
    for line in content.lines() {
        let Some((prefix, namespace)) = parse_turtle_prefix_line(line.trim()) else {
            continue;
        };
        if prefix == expected_prefix {
            return Some(namespace);
        }
    }
    None
}

fn parse_turtle_prefix_line(line: &str) -> Option<(String, String)> {
    let rest = line
        .strip_prefix("@prefix ")
        .or_else(|| line.strip_prefix("@PREFIX "))
        .or_else(|| line.strip_prefix("PREFIX "))?;
    let rest = rest.trim_start();
    let (prefix, rest) = rest.split_once(':')?;
    let rest = rest.trim_start();
    let namespace_start = rest.find('<')? + 1;
    let namespace_end = rest[namespace_start..].find('>')? + namespace_start;
    Some((
        prefix.trim().to_string(),
        rest[namespace_start..namespace_end].to_string(),
    ))
}

fn validate_turtle_language(language: &str) -> bool {
    matches!(language.to_ascii_lowercase().as_str(), "turtle" | "ttl")
}

fn parse_turtle_block(content: &str) -> Result<Vec<Quad>, String> {
    let mut graph = Vec::new();

    for parsed in RdfParser::from_format(RdfFormat::Turtle).for_reader(content.as_bytes()) {
        graph.push(parsed.map_err(|error| error.to_string())?);
    }

    if graph.is_empty() {
        return Err("Turtle block has no RDF statements".to_string());
    }

    Ok(graph)
}

fn parse_external_ontology_block(
    content: &str,
    format: ExternalOntologyFormat,
) -> Result<Vec<Quad>, String> {
    let mut graph = Vec::new();

    let parser = match format {
        ExternalOntologyFormat::Turtle => RdfParser::from_format(RdfFormat::Turtle),
        ExternalOntologyFormat::RdfXml => RdfParser::from_format(RdfFormat::RdfXml),
        ExternalOntologyFormat::JsonLd => RdfParser::from_format(RdfFormat::JsonLd {
            profile: JsonLdProfileSet::empty(),
        }),
    };

    for parsed in parser.for_reader(content.as_bytes()) {
        graph.push(parsed.map_err(|error| error.to_string())?);
    }

    if graph.is_empty() {
        return Err(format!(
            "{} block has no RDF statements",
            format.display_name()
        ));
    }

    Ok(graph)
}

fn ontology_term_role(type_iri: &str) -> Option<OntologyTermRole> {
    match type_iri {
        OWL_CLASS | RDFS_CLASS => Some(OntologyTermRole::Class),
        RDFS_DATATYPE => Some(OntologyTermRole::Datatype),
        RDF_PROPERTY => Some(OntologyTermRole::Property),
        OWL_ANNOTATION_PROPERTY => Some(OntologyTermRole::AnnotationProperty),
        OWL_OBJECT_PROPERTY => Some(OntologyTermRole::ObjectProperty),
        OWL_DATATYPE_PROPERTY => Some(OntologyTermRole::DatatypeProperty),
        OWL_NAMED_INDIVIDUAL => Some(OntologyTermRole::NamedIndividual),
        _ => None,
    }
}

fn build_ontology_projection(
    _registry: &GraphRegistry,
    blocks: &[SemanticBlock],
) -> OntologyProjectionGraph {
    let mut sourced_quads = Vec::new();
    let mut source_lookup = BTreeMap::new();

    for block in blocks {
        let source_key = ontology_projection_source_key(block);
        source_lookup.insert(source_key.clone(), projection_source(block));
        for quad in &block.quads {
            sourced_quads.push(constructs::SourcedQuad {
                source: source_key.clone(),
                quad: quad.clone(),
            });
        }
    }

    let projection = constructs::classify_ontology_constructs_with_sources(&sourced_quads);
    kernel_projection_to_reqvire_graph(&projection, &source_lookup)
}

fn ontology_projection_source_key(block: &SemanticBlock) -> String {
    format!(
        "{}#{}:{}",
        block.source,
        block.kind.as_str(),
        block.line_number
    )
}

fn kernel_projection_to_reqvire_graph(
    projection: &constructs::OntologyProjection,
    source_lookup: &BTreeMap<String, OntologyProjectionSource>,
) -> OntologyProjectionGraph {
    let mut constructs = Vec::new();
    for construct in &projection.constructs {
        constructs.push(kernel_projection_construct_to_reqvire(
            construct,
            source_lookup,
        ));
    }
    constructs.sort_by(|a, b| {
        a.family
            .cmp(&b.family)
            .then_with(|| a.kind.cmp(&b.kind))
            .then_with(|| a.id.cmp(&b.id))
    });

    let mut projections = Vec::new();
    for projection in &projection.projections {
        projections.push(OntologyConstructProjection {
            id: projection.id.clone(),
            family: to_reqvire_construct_family(projection.family),
            derivation_mode: to_reqvire_projection_derivation_mode(projection.derivation_mode),
            construct_ids: projection.construct_ids.clone(),
        });
    }
    projections.sort_by_key(|projection| projection.id.clone());

    OntologyProjectionGraph {
        id: "urn:reqvire:ontology-projection:graph:direct-authored".to_string(),
        derivation_mode: OntologyProjectionDerivationMode::DirectAuthored,
        projections,
        constructs,
        symbols: projection.symbols.iter().map(to_reqvire_symbol).collect(),
    }
}

fn kernel_projection_construct_to_reqvire(
    construct: &constructs::OntologyConstruct,
    source_lookup: &BTreeMap<String, OntologyProjectionSource>,
) -> OntologyConstruct {
    OntologyConstruct {
        id: construct.id.clone(),
        family: to_reqvire_construct_family(construct.family),
        kind: to_reqvire_construct_kind(construct.kind),
        subject: to_reqvire_projection_term(&construct.subject),
        predicate: construct.predicate.as_ref().map(to_reqvire_projection_term),
        object: construct.object.as_ref().map(to_reqvire_projection_term),
        property: construct.property.as_ref().map(to_reqvire_projection_term),
        members: construct
            .members
            .iter()
            .map(|member| OntologyConstructMember {
                sequence_index: member.sequence_index,
                term: to_reqvire_projection_term(&member.term),
                source: projection_source_for_key(&member.source, source_lookup),
            })
            .collect(),
        property_characteristic: construct
            .property_characteristic
            .map(to_reqvire_property_characteristic),
        restriction_kind: construct.restriction_kind.map(to_reqvire_restriction_kind),
        class_expression_kind: construct
            .class_expression_kind
            .map(to_reqvire_class_expression_kind),
        shape_overlay_kind: construct
            .shape_overlay_kind
            .map(to_reqvire_shape_overlay_kind),
        symbol: construct.symbol.as_ref().map(to_reqvire_symbol),
        provenance: OntologyProjectionProvenance {
            derivation_mode: OntologyProjectionDerivationMode::DirectAuthored,
            source: projection_source_for_key(&construct.source, source_lookup),
            evidence: construct
                .evidence
                .iter()
                .map(|evidence| OntologyProjectionEvidence {
                    source: projection_source_for_key(&evidence.source, source_lookup),
                    subject: to_reqvire_projection_term(&evidence.subject),
                    predicate: to_reqvire_projection_term(&evidence.predicate),
                    object: to_reqvire_projection_term(&evidence.object),
                })
                .collect(),
        },
    }
}

fn projection_source_for_key(
    key: &str,
    source_lookup: &BTreeMap<String, OntologyProjectionSource>,
) -> OntologyProjectionSource {
    if let Some(source) = source_lookup.get(key) {
        return source.clone();
    }

    let (source_element_identifier, block_kind, line_number) = parse_projection_source_key(key);
    OntologyProjectionSource {
        source_block: key.to_string(),
        source_element_identifier,
        source_name: "Unknown source".to_string(),
        file_path: String::new(),
        line_number,
        block_kind,
    }
}

fn parse_projection_source_key(key: &str) -> (String, String, usize) {
    let (source_element_identifier, suffix) = key.rsplit_once('#').unwrap_or((key, ""));
    let (block_kind, line_number_text) = suffix.rsplit_once(':').unwrap_or((suffix, "0"));
    let line_number = line_number_text.parse::<usize>().unwrap_or(0);
    (
        source_element_identifier.to_string(),
        block_kind.to_string(),
        line_number,
    )
}

fn to_reqvire_projection_derivation_mode(mode: &str) -> OntologyProjectionDerivationMode {
    match mode {
        "direct-authored" => OntologyProjectionDerivationMode::DirectAuthored,
        _ => OntologyProjectionDerivationMode::DirectAuthored,
    }
}

fn to_reqvire_projection_term(term: &constructs::OntologyConstructTerm) -> OntologyProjectionTerm {
    OntologyProjectionTerm {
        kind: match term.kind {
            constructs::OntologyConstructTermKind::Iri => OntologyProjectionTermKind::Iri,
            constructs::OntologyConstructTermKind::BlankNode => {
                OntologyProjectionTermKind::BlankNode
            }
            constructs::OntologyConstructTermKind::Literal => OntologyProjectionTermKind::Literal,
        },
        value: term.value.clone(),
        label: term.label.clone(),
    }
}

fn to_reqvire_construct_family(
    family: constructs::OntologyConstructFamily,
) -> OntologyConstructFamily {
    match family {
        constructs::OntologyConstructFamily::PropertyDomainRange => {
            OntologyConstructFamily::PropertyDomainRange
        }
        constructs::OntologyConstructFamily::SubclassMembership => {
            OntologyConstructFamily::SubclassMembership
        }
        constructs::OntologyConstructFamily::DisjointEquivalenceInverse => {
            OntologyConstructFamily::DisjointEquivalenceInverse
        }
        constructs::OntologyConstructFamily::PropertyChain => {
            OntologyConstructFamily::PropertyChain
        }
        constructs::OntologyConstructFamily::PropertyCharacteristic => {
            OntologyConstructFamily::PropertyCharacteristic
        }
        constructs::OntologyConstructFamily::Restriction => OntologyConstructFamily::Restriction,
        constructs::OntologyConstructFamily::ClassExpression => {
            OntologyConstructFamily::ClassExpression
        }
        constructs::OntologyConstructFamily::ShapeOverlay => OntologyConstructFamily::ShapeOverlay,
    }
}

fn to_reqvire_construct_kind(kind: constructs::OntologyConstructKind) -> OntologyConstructKind {
    match kind {
        constructs::OntologyConstructKind::PropertyDomain => OntologyConstructKind::PropertyDomain,
        constructs::OntologyConstructKind::PropertyRange => OntologyConstructKind::PropertyRange,
        constructs::OntologyConstructKind::SubclassInclusion => {
            OntologyConstructKind::SubclassInclusion
        }
        constructs::OntologyConstructKind::Membership => OntologyConstructKind::Membership,
        constructs::OntologyConstructKind::Disjointness => OntologyConstructKind::Disjointness,
        constructs::OntologyConstructKind::EquivalenceGroup => {
            OntologyConstructKind::EquivalenceGroup
        }
        constructs::OntologyConstructKind::InverseProperty => {
            OntologyConstructKind::InverseProperty
        }
        constructs::OntologyConstructKind::PropertyChain => OntologyConstructKind::PropertyChain,
        constructs::OntologyConstructKind::PropertyCharacteristic => {
            OntologyConstructKind::PropertyCharacteristic
        }
        constructs::OntologyConstructKind::Restriction => OntologyConstructKind::Restriction,
        constructs::OntologyConstructKind::ClassExpression => {
            OntologyConstructKind::ClassExpression
        }
        constructs::OntologyConstructKind::ShapeOverlay => OntologyConstructKind::ShapeOverlay,
    }
}

fn to_reqvire_property_characteristic(
    characteristic: constructs::OntologyPropertyCharacteristic,
) -> OntologyPropertyCharacteristic {
    match characteristic {
        constructs::OntologyPropertyCharacteristic::Functional => {
            OntologyPropertyCharacteristic::Functional
        }
        constructs::OntologyPropertyCharacteristic::InverseFunctional => {
            OntologyPropertyCharacteristic::InverseFunctional
        }
        constructs::OntologyPropertyCharacteristic::Symmetric => {
            OntologyPropertyCharacteristic::Symmetric
        }
        constructs::OntologyPropertyCharacteristic::Asymmetric => {
            OntologyPropertyCharacteristic::Asymmetric
        }
        constructs::OntologyPropertyCharacteristic::Reflexive => {
            OntologyPropertyCharacteristic::Reflexive
        }
        constructs::OntologyPropertyCharacteristic::Irreflexive => {
            OntologyPropertyCharacteristic::Irreflexive
        }
        constructs::OntologyPropertyCharacteristic::Transitive => {
            OntologyPropertyCharacteristic::Transitive
        }
    }
}

fn to_reqvire_restriction_kind(
    restriction: constructs::OntologyRestrictionKind,
) -> OntologyRestrictionKind {
    match restriction {
        constructs::OntologyRestrictionKind::Universal => OntologyRestrictionKind::Universal,
        constructs::OntologyRestrictionKind::Existential => OntologyRestrictionKind::Existential,
        constructs::OntologyRestrictionKind::HasValue => OntologyRestrictionKind::HasValue,
        constructs::OntologyRestrictionKind::Cardinality => OntologyRestrictionKind::Cardinality,
        constructs::OntologyRestrictionKind::MinCardinality => {
            OntologyRestrictionKind::MinCardinality
        }
        constructs::OntologyRestrictionKind::MaxCardinality => {
            OntologyRestrictionKind::MaxCardinality
        }
        constructs::OntologyRestrictionKind::QualifiedCardinality => {
            OntologyRestrictionKind::QualifiedCardinality
        }
        constructs::OntologyRestrictionKind::MinQualifiedCardinality => {
            OntologyRestrictionKind::MinQualifiedCardinality
        }
        constructs::OntologyRestrictionKind::MaxQualifiedCardinality => {
            OntologyRestrictionKind::MaxQualifiedCardinality
        }
        constructs::OntologyRestrictionKind::OnClass => OntologyRestrictionKind::OnClass,
        constructs::OntologyRestrictionKind::OnDataRange => OntologyRestrictionKind::OnDataRange,
    }
}

fn to_reqvire_class_expression_kind(
    class_expression: constructs::OntologyClassExpressionKind,
) -> OntologyClassExpressionKind {
    match class_expression {
        constructs::OntologyClassExpressionKind::Intersection => {
            OntologyClassExpressionKind::Intersection
        }
        constructs::OntologyClassExpressionKind::Union => OntologyClassExpressionKind::Union,
        constructs::OntologyClassExpressionKind::Complement => {
            OntologyClassExpressionKind::Complement
        }
    }
}

fn to_reqvire_shape_overlay_kind(
    overlay: constructs::OntologyShapeOverlayKind,
) -> OntologyShapeOverlayKind {
    match overlay {
        constructs::OntologyShapeOverlayKind::NodeShape => OntologyShapeOverlayKind::NodeShape,
        constructs::OntologyShapeOverlayKind::PropertyShape => {
            OntologyShapeOverlayKind::PropertyShape
        }
    }
}

fn to_reqvire_symbol(symbol: &constructs::OntologySymbol) -> OntologySymbol {
    OntologySymbol {
        concept_name: symbol.concept_name.clone(),
        raw_unicode_code_point: symbol.raw_unicode_code_point.clone(),
        rendered_unicode_character: symbol.rendered_unicode_character.clone(),
        tooltip: symbol.tooltip.clone(),
        accessible_label: symbol.accessible_label.clone(),
    }
}

fn projection_source(block: &SemanticBlock) -> OntologyProjectionSource {
    OntologyProjectionSource {
        source_block: format!(
            "{}#{}:{}",
            block.source,
            block.kind.as_str(),
            block.line_number
        ),
        source_element_identifier: block.source.clone(),
        source_name: block.source_name.clone(),
        file_path: block.file_path.clone(),
        line_number: block.line_number,
        block_kind: block.kind.as_str().to_string(),
    }
}

fn projection_term_key(term: &OntologyProjectionTerm) -> String {
    format!("{}:{}", term.kind.as_str(), term.value)
}

fn stable_hash(value: &str) -> String {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash = FNV_OFFSET;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    format!("{hash:016x}")
}

fn ontology_term_declarations_from_quads(
    element: &Element,
    quads: &[Quad],
    known_class_iris: &BTreeSet<String>,
) -> Vec<OntologyTermDeclaration> {
    collect_ontology_term_declarations(&element.identifier, quads, false, known_class_iris)
}

fn external_ontology_term_declarations_from_quads(
    element: &Element,
    quads: &[Quad],
) -> Vec<OntologyTermDeclaration> {
    collect_ontology_term_declarations(&element.identifier, quads, true, &BTreeSet::new())
}

fn ontology_term_declarations_from_quads_with_source(
    source_identifier: &str,
    quads: &[Quad],
    external: bool,
) -> Vec<OntologyTermDeclaration> {
    collect_ontology_term_declarations(source_identifier, quads, external, &BTreeSet::new())
}

fn collect_ontology_term_declarations(
    source_identifier: &str,
    quads: &[Quad],
    external: bool,
    known_class_iris: &BTreeSet<String>,
) -> Vec<OntologyTermDeclaration> {
    let mut declarations = Vec::new();
    let mut seen = BTreeSet::new();
    let mut class_iris = known_class_iris.clone();

    for quad in quads
        .iter()
        .filter(|quad| quad.predicate.as_str() == RDF_TYPE)
    {
        let Some(iri) = subject_iri(&quad.subject) else {
            continue;
        };
        let Some(type_iri) = term_iri(&quad.object) else {
            continue;
        };
        let Some(role) = ontology_term_role(type_iri) else {
            continue;
        };
        if role == OntologyTermRole::Class {
            class_iris.insert(iri.to_string());
        }
        push_ontology_term_declaration(
            &mut declarations,
            &mut seen,
            source_identifier,
            iri,
            role,
            external,
        );
    }

    for quad in quads
        .iter()
        .filter(|quad| quad.predicate.as_str() == RDF_TYPE)
    {
        let Some(iri) = subject_iri(&quad.subject) else {
            continue;
        };
        let Some(type_iri) = term_iri(&quad.object) else {
            continue;
        };
        if ontology_term_role(type_iri).is_some() || !class_iris.contains(type_iri) {
            continue;
        }
        push_ontology_term_declaration(
            &mut declarations,
            &mut seen,
            source_identifier,
            iri,
            OntologyTermRole::NamedIndividual,
            external,
        );
    }

    declarations
}

fn push_ontology_term_declaration(
    declarations: &mut Vec<OntologyTermDeclaration>,
    seen: &mut BTreeSet<(String, OntologyTermRole)>,
    source_identifier: &str,
    iri: &str,
    role: OntologyTermRole,
    external: bool,
) {
    if !seen.insert((iri.to_string(), role)) {
        return;
    }
    declarations.push(OntologyTermDeclaration {
        iri: iri.to_string(),
        role,
        element_identifier: source_identifier.to_string(),
        external,
        materialized_in_used_subset: false,
    });
}

fn shape_iri_references_from_quads(element: &Element, quads: &[Quad]) -> Vec<ShapeIriReference> {
    let mut references = BTreeSet::new();
    for reference in ontology::extract_shape_references(quads) {
        let iri = reference.iri.as_str();
        let kind = reference.predicate_label();
        references.insert(ShapeIriReference {
            iri: iri.to_string(),
            kind,
            element_identifier: element.identifier.clone(),
        });
    }

    references.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_test_quads(turtle: &str) -> Vec<Quad> {
        RdfParser::from_format(RdfFormat::Turtle)
            .for_reader(turtle.as_bytes())
            .map(|quad| quad.expect("test Turtle should parse"))
            .collect()
    }

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
            ontology_declarations: HashMap::from([
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
                element_identifiers: vec!["system-model/Ontologies/Test.md#test-ontology".to_string()],
                element_names: vec!["Test ontology".to_string()],
                imports: Vec::new(),
            }],
            ontology_declarations: HashMap::from([(
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
            ontology_declarations: HashMap::new(),
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
        assert!(external_output.contains("An idea or notion; a unit of thought."));
        assert!(external_output.contains("http://www.w3.org/2004/02/skos/core#Concept"));
        assert!(!external_output.contains("Ordered Collection"));
        assert!(!external_output.contains(
            "An ordered collection of concepts, where both the grouping and the ordering are meaningful."
        ));
    }
}
