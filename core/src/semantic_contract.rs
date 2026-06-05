use crate::element::{
    self, AttachmentTarget, Element, ElementType, FencedBlock, RefinementType, VerificationType,
    GOVERNANCE_METADATA_KEYS,
};
use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;
use crate::relation::LinkType;
use oxigraph::io::{JsonLdProfileSet, RdfFormat, RdfParser, RdfSerializer};
use oxigraph::model::{NamedOrBlankNode, Quad, Term};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt;

const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const RDF_FIRST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#first";
const RDF_REST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#rest";
const RDF_NIL: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#nil";
const RDFS_SUBCLASS_OF: &str = "http://www.w3.org/2000/01/rdf-schema#subClassOf";
const RDFS_DOMAIN: &str = "http://www.w3.org/2000/01/rdf-schema#domain";
const RDFS_RANGE: &str = "http://www.w3.org/2000/01/rdf-schema#range";
const RDFS_LABEL: &str = "http://www.w3.org/2000/01/rdf-schema#label";
const RDFS_COMMENT: &str = "http://www.w3.org/2000/01/rdf-schema#comment";
const RDFS_CLASS: &str = "http://www.w3.org/2000/01/rdf-schema#Class";
const RDFS_DATATYPE: &str = "http://www.w3.org/2000/01/rdf-schema#Datatype";
const OWL_CLASS: &str = "http://www.w3.org/2002/07/owl#Class";
const OWL_NAMED_INDIVIDUAL: &str = "http://www.w3.org/2002/07/owl#NamedIndividual";
const RDF_PROPERTY: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#Property";
const OWL_OBJECT_PROPERTY: &str = "http://www.w3.org/2002/07/owl#ObjectProperty";
const OWL_DATATYPE_PROPERTY: &str = "http://www.w3.org/2002/07/owl#DatatypeProperty";
const OWL_RESTRICTION: &str = "http://www.w3.org/2002/07/owl#Restriction";
const OWL_DISJOINT_WITH: &str = "http://www.w3.org/2002/07/owl#disjointWith";
const OWL_EQUIVALENT_CLASS: &str = "http://www.w3.org/2002/07/owl#equivalentClass";
const OWL_EQUIVALENT_PROPERTY: &str = "http://www.w3.org/2002/07/owl#equivalentProperty";
const OWL_SAME_AS: &str = "http://www.w3.org/2002/07/owl#sameAs";
const OWL_INVERSE_OF: &str = "http://www.w3.org/2002/07/owl#inverseOf";
const OWL_PROPERTY_CHAIN_AXIOM: &str = "http://www.w3.org/2002/07/owl#propertyChainAxiom";
const OWL_FUNCTIONAL_PROPERTY: &str = "http://www.w3.org/2002/07/owl#FunctionalProperty";
const OWL_INVERSE_FUNCTIONAL_PROPERTY: &str =
    "http://www.w3.org/2002/07/owl#InverseFunctionalProperty";
const OWL_TRANSITIVE_PROPERTY: &str = "http://www.w3.org/2002/07/owl#TransitiveProperty";
const OWL_SYMMETRIC_PROPERTY: &str = "http://www.w3.org/2002/07/owl#SymmetricProperty";
const OWL_ASYMMETRIC_PROPERTY: &str = "http://www.w3.org/2002/07/owl#AsymmetricProperty";
const OWL_REFLEXIVE_PROPERTY: &str = "http://www.w3.org/2002/07/owl#ReflexiveProperty";
const OWL_IRREFLEXIVE_PROPERTY: &str = "http://www.w3.org/2002/07/owl#IrreflexiveProperty";
const OWL_ON_PROPERTY: &str = "http://www.w3.org/2002/07/owl#onProperty";
const OWL_ALL_VALUES_FROM: &str = "http://www.w3.org/2002/07/owl#allValuesFrom";
const OWL_SOME_VALUES_FROM: &str = "http://www.w3.org/2002/07/owl#someValuesFrom";
const OWL_HAS_VALUE: &str = "http://www.w3.org/2002/07/owl#hasValue";
const OWL_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#cardinality";
const OWL_MIN_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#minCardinality";
const OWL_MAX_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#maxCardinality";
const OWL_QUALIFIED_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#qualifiedCardinality";
const OWL_MIN_QUALIFIED_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#minQualifiedCardinality";
const OWL_MAX_QUALIFIED_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#maxQualifiedCardinality";
const OWL_ON_CLASS: &str = "http://www.w3.org/2002/07/owl#onClass";
const OWL_ON_DATA_RANGE: &str = "http://www.w3.org/2002/07/owl#onDataRange";
const OWL_INTERSECTION_OF: &str = "http://www.w3.org/2002/07/owl#intersectionOf";
const OWL_UNION_OF: &str = "http://www.w3.org/2002/07/owl#unionOf";
const OWL_COMPLEMENT_OF: &str = "http://www.w3.org/2002/07/owl#complementOf";
const SH_NODE_SHAPE: &str = "http://www.w3.org/ns/shacl#NodeShape";
const SH_PROPERTY_SHAPE: &str = "http://www.w3.org/ns/shacl#PropertyShape";
const SH_TARGET_CLASS: &str = "http://www.w3.org/ns/shacl#targetClass";
const SH_PROPERTY: &str = "http://www.w3.org/ns/shacl#property";
const SH_PATH: &str = "http://www.w3.org/ns/shacl#path";
const SH_MIN_COUNT: &str = "http://www.w3.org/ns/shacl#minCount";
const SH_MAX_COUNT: &str = "http://www.w3.org/ns/shacl#maxCount";
const SH_DATATYPE: &str = "http://www.w3.org/ns/shacl#datatype";
const SH_CLASS: &str = "http://www.w3.org/ns/shacl#class";
const SH_NODE_KIND: &str = "http://www.w3.org/ns/shacl#nodeKind";
const SH_PATTERN: &str = "http://www.w3.org/ns/shacl#pattern";
const SH_IN: &str = "http://www.w3.org/ns/shacl#in";
const SH_NODE_KINDS: &[&str] = &[
    "http://www.w3.org/ns/shacl#IRI",
    "http://www.w3.org/ns/shacl#BlankNode",
    "http://www.w3.org/ns/shacl#Literal",
    "http://www.w3.org/ns/shacl#BlankNodeOrIRI",
    "http://www.w3.org/ns/shacl#BlankNodeOrLiteral",
    "http://www.w3.org/ns/shacl#IRIOrLiteral",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OntologyTermRole {
    Class,
    Property,
    ObjectProperty,
    DatatypeProperty,
}

impl OntologyTermRole {
    pub fn conflicts_with(self, other: Self) -> bool {
        matches!(
            (self, other),
            (
                Self::Class,
                Self::Property | Self::ObjectProperty | Self::DatatypeProperty
            ) | (
                Self::Property | Self::ObjectProperty | Self::DatatypeProperty,
                Self::Class
            ) | (Self::ObjectProperty, Self::DatatypeProperty)
                | (Self::DatatypeProperty, Self::ObjectProperty)
        )
    }
}

impl fmt::Display for OntologyTermRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Class => write!(f, "class"),
            Self::Property => write!(f, "property"),
            Self::ObjectProperty => write!(f, "object-property"),
            Self::DatatypeProperty => write!(f, "datatype-property"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OntologyTermDeclaration {
    pub iri: String,
    pub role: OntologyTermRole,
    pub element_identifier: String,
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
}

impl SemanticBlockKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ontology => "ontology",
            Self::Shapes => "shapes",
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_contract_iri: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_contract_iri: Option<String>,
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
pub struct SemanticIndex {
    pub blocks: Vec<SemanticBlock>,
    pub diagnostics: Vec<SemanticDiagnostic>,
    pub ontology_declarations: HashMap<String, Vec<OntologyTermDeclaration>>,
    pub shape_references: Vec<ShapeIriReference>,
    pub ontology_projection: OntologyProjectionGraph,
    pub summary: SemanticIndexSummary,
}

#[derive(Debug, Clone, Copy)]
pub enum SemanticExportFormat {
    Turtle,
    JsonLd,
}

impl SemanticIndex {
    pub fn to_turtle_string(&self) -> String {
        let mut output = String::new();
        output.push_str("# Generated by Reqvire semantic index\n");
        output.push_str(&format!(
            "# Blocks: {} ontology, {} shapes\n\n",
            self.summary.ontology_blocks, self.summary.shape_blocks
        ));

        for block in &self.blocks {
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
            output.push_str(block.content.trim());
            output.push_str("\n\n");
        }

        output
    }

    pub fn to_jsonld_string(&self) -> Result<String, ReqvireError> {
        let mut serializer = RdfSerializer::from_format(RdfFormat::JsonLd {
            profile: JsonLdProfileSet::empty(),
        })
        .for_writer(Vec::new());

        for block in &self.blocks {
            for quad in &block.quads {
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
            SemanticExportFormat::Turtle => Ok(self.to_turtle_string()),
            SemanticExportFormat::JsonLd => self.to_jsonld_string(),
        }
    }

    pub fn to_full_turtle_string(&self, registry: &GraphRegistry) -> String {
        let mut output = self.to_turtle_string();
        output.push_str(&build_model_context_turtle(registry, self));
        output.push_str(&build_ontology_projection_turtle(self));
        output
    }

    pub fn to_full_jsonld_string(&self, registry: &GraphRegistry) -> Result<String, ReqvireError> {
        let turtle = self.to_full_turtle_string(registry);
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

    pub fn serialize_full(
        &self,
        format: SemanticExportFormat,
        registry: &GraphRegistry,
    ) -> Result<String, ReqvireError> {
        match format {
            SemanticExportFormat::Turtle => Ok(self.to_full_turtle_string(registry)),
            SemanticExportFormat::JsonLd => self.to_full_jsonld_string(registry),
        }
    }
}

fn build_model_context_turtle(registry: &GraphRegistry, index: &SemanticIndex) -> String {
    let mut output = String::new();
    let mut artifacts = BTreeSet::new();
    output.push_str(
        "# -----------------------------------------------------------------------------\n",
    );
    output.push_str("# Reqvire full semantic model context\n\n");
    output.push_str("@prefix reqvire: <https://www.reqvire.org/ontology#> .\n\n");

    let mut nodes: Vec<_> = registry.nodes.values().collect();
    nodes.sort_by(|a, b| a.element.identifier.cmp(&b.element.identifier));

    for node in nodes {
        let element = &node.element;
        if element.element_type.is_semantic_query_contract() {
            continue;
        }

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
            turtle_string(&element_type_token(&element.element_type))
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
            output.push_str(&format!(
                "{} reqvire:relationTarget {} .\n",
                subject, target_iri
            ));
        }

        for attachment in &element.attachments {
            let Some(target_iri) =
                attachment_target_iri(&attachment.target, registry, &mut artifacts)
            else {
                continue;
            };
            output.push_str(&format!("{} reqvire:attaches {} .\n", subject, target_iri));
        }

        for reference in &element.concept_references {
            output.push_str(&format!(
                "{} reqvire:conceptReference <{}> .\n",
                subject,
                escape_iri(&reference.iri)
            ));
            output.push_str(&format!(
                "{} reqvire:referencesTerm <{}> .\n",
                subject,
                escape_iri(&reference.iri)
            ));
        }

        if !element.relations.is_empty()
            || !element.attachments.is_empty()
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
    output
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
    let mut symbol_resources = BTreeSet::new();

    output.push_str(
        "# -----------------------------------------------------------------------------\n",
    );
    output.push_str("# Reqvire generated ontology projection facts\n\n");
    output.push_str("@prefix reqvire: <https://www.reqvire.org/ontology#> .\n\n");

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
    for symbol in &graph.symbols {
        let symbol_iri = serialize_projection_symbol(symbol, &mut symbol_resources);
        output.push_str(&format!(
            "{} reqvire:ontologySymbol {} .\n",
            graph_iri, symbol_iri
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
        if let Some(pattern_contract_iri) = &projection.pattern_contract_iri {
            output.push_str(&format!(
                "{} reqvire:constructQueryContract {} .\n",
                projection_iri,
                projection_resource_iri(pattern_contract_iri)
            ));
        }
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
        if let Some(symbol) = &construct.symbol {
            let symbol_iri = serialize_projection_symbol(symbol, &mut symbol_resources);
            output.push_str(&format!(
                "{} reqvire:constructSymbol {} .\n",
                construct_iri, symbol_iri
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
        if let Some(pattern_contract_iri) = &construct.provenance.pattern_contract_iri {
            output.push_str(&format!(
                "{} reqvire:constructQueryContract {} .\n",
                construct_iri,
                projection_resource_iri(pattern_contract_iri)
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
    for resource in symbol_resources {
        output.push_str(&resource);
    }

    output
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
    if let Some(pattern_contract_iri) = &provenance.pattern_contract_iri {
        chunk.push_str(&format!(
            "{} reqvire:constructQueryContract {} .\n",
            provenance_iri,
            projection_resource_iri(pattern_contract_iri)
        ));
    }
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

fn serialize_projection_symbol(
    symbol: &OntologySymbol,
    symbol_resources: &mut BTreeSet<String>,
) -> String {
    let symbol_iri = projection_generated_iri("ontology-symbol", &symbol.concept_name);
    symbol_resources.insert(format!(
        "{} a reqvire:OntologySymbol .\n{} reqvire:symbolConceptName {} .\n{} reqvire:rawUnicodeCodePoint {} .\n{} reqvire:renderedUnicodeCharacter {} .\n{} reqvire:symbolTooltip {} .\n{} reqvire:accessibleLabel {} .\n\n",
        symbol_iri,
        symbol_iri,
        turtle_string(&symbol.concept_name),
        symbol_iri,
        turtle_string(&symbol.raw_unicode_code_point),
        symbol_iri,
        turtle_string(&symbol.rendered_unicode_character),
        symbol_iri,
        turtle_string(&symbol.tooltip),
        symbol_iri,
        turtle_string(&symbol.accessible_label)
    ));
    symbol_iri
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
    format!("<urn:reqvire:element:{}>", escape_iri(&element.id))
}

fn element_type_classes(element_type: &ElementType) -> Vec<&'static str> {
    match element_type {
        ElementType::Capability => vec!["reqvire:Element", "reqvire:Capability"],
        ElementType::Requirement(_) => vec!["reqvire:Element", "reqvire:Requirement"],
        ElementType::Ontology => vec!["reqvire:Element", "reqvire:Ontology"],
        ElementType::Verification(verification_type) => {
            let subtype = match verification_type {
                VerificationType::Default | VerificationType::Test => "reqvire:TestVerification",
                VerificationType::FormalProof => "reqvire:FormalProofVerification",
                VerificationType::Analysis => "reqvire:AnalysisVerification",
                VerificationType::Inspection => "reqvire:InspectionVerification",
                VerificationType::Demonstration => "reqvire:DemonstrationVerification",
            };
            vec!["reqvire:Element", "reqvire:Verification", subtype]
        }
        ElementType::Refinement(refinement_type) => {
            let subtype = match refinement_type {
                RefinementType::Source => "reqvire:Source",
                RefinementType::SemanticContract => "reqvire:SemanticContract",
                RefinementType::SemanticQueryContract => "reqvire:SemanticQueryContract",
                RefinementType::Constraint => "reqvire:Constraint",
                RefinementType::Behavior => "reqvire:Behavior",
                RefinementType::Specification => "reqvire:Specification",
                RefinementType::State => "reqvire:State",
                RefinementType::InputOutput => "reqvire:InputOutput",
            };
            vec!["reqvire:Element", "reqvire:Refinement", subtype]
        }
        ElementType::File => vec!["reqvire:Artifact", "reqvire:File"],
        ElementType::Other(_) => vec!["reqvire:Element", "reqvire:CustomElement"],
    }
}

fn element_type_token(element_type: &ElementType) -> String {
    match element_type {
        ElementType::Other(custom_type) => format!("other-{}", custom_type),
        _ => element_type.as_str().to_string(),
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
            .filter(|target| !target.element.element_type.is_semantic_query_contract())
            .map(|target| element_iri(&target.element)),
        LinkType::InternalPath(path) => {
            let value = path.to_string_lossy();
            let iri = artifact_iri("path", &value);
            artifacts.insert(format!(
                "{} a reqvire:Artifact, reqvire:File ;\n  reqvire:filePath {} .\n\n",
                iri,
                turtle_string(&value)
            ));
            Some(iri)
        }
        LinkType::ExternalUrl(url) => {
            let iri = artifact_iri("url", url);
            artifacts.insert(format!(
                "{} a reqvire:Artifact ;\n  reqvire:externalUrl {} .\n\n",
                iri,
                turtle_string(url)
            ));
            Some(iri)
        }
    }
}

fn attachment_target_iri(
    target: &AttachmentTarget,
    registry: &GraphRegistry,
    artifacts: &mut BTreeSet<String>,
) -> Option<String> {
    match target {
        AttachmentTarget::ElementIdentifier(target_identifier) => registry
            .nodes
            .get(target_identifier)
            .filter(|target| !target.element.element_type.is_semantic_query_contract())
            .map(|target| element_iri(&target.element)),
        AttachmentTarget::FilePath(path) => {
            let value = path.to_string_lossy();
            let iri = artifact_iri("path", &value);
            artifacts.insert(format!(
                "{} a reqvire:Artifact, reqvire:File ;\n  reqvire:filePath {} .\n\n",
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

pub fn build_semantic_index(registry: &GraphRegistry) -> SemanticIndex {
    let mut blocks = Vec::new();
    let mut diagnostics = Vec::new();
    let mut ontology_declarations: HashMap<String, Vec<OntologyTermDeclaration>> = HashMap::new();
    let mut shape_references = Vec::new();

    for element in registry.get_all_elements() {
        let ontology = element::extract_single_fenced_subsection(&element.content, "Ontology");
        let shapes = element::extract_single_fenced_subsection(&element.content, "Shapes");
        let query = element::extract_single_fenced_subsection(&element.content, "Query");

        validate_semantic_sections(element, &ontology, &shapes, &query, &mut diagnostics);

        if element.element_type.is_ontology() {
            if let Some(block) = build_block(
                element,
                SemanticBlockKind::Ontology,
                ontology.first(),
                "Ontology",
                &mut diagnostics,
            ) {
                for declaration in ontology_term_declarations_from_quads(element, &block.quads) {
                    ontology_declarations
                        .entry(declaration.iri.clone())
                        .or_default()
                        .push(declaration);
                }
                blocks.push(block);
            }
            continue;
        }

        if element.element_type.is_semantic_contract() {
            if let Some(block) = build_block(
                element,
                SemanticBlockKind::Shapes,
                shapes.first(),
                "Shapes",
                &mut diagnostics,
            ) {
                for message in validate_shacl_sanity(&block.quads) {
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

    SemanticIndex {
        summary: SemanticIndexSummary {
            ontology_blocks,
            shape_blocks,
            total_blocks: blocks.len(),
            total_quads,
        },
        blocks,
        diagnostics,
        ontology_declarations,
        shape_references,
        ontology_projection,
    }
}

fn validate_semantic_sections(
    element: &Element,
    ontology: &[FencedBlock],
    shapes: &[FencedBlock],
    query: &[FencedBlock],
    diagnostics: &mut Vec<SemanticDiagnostic>,
) {
    let has_query_section = element::has_subsection(&element.content, "Query");
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
                    "Ontology element '{}' must not contain a #### Shapes section. SHACL profiles belong in requirement-owned semantic-contract elements.",
                    element.name
                ),
            });
        }
        if has_query_section {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: query_line_number,
                message: format!(
                    "Ontology element '{}' must not contain a #### Query section. SPARQL queries belong in requirement-owned semantic-query-contract elements.",
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
        if has_query_section {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: query_line_number,
                message: format!(
                    "Semantic contract '{}' must not contain a #### Query section. Use type: semantic-query-contract.",
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
    } else if element.element_type.is_semantic_query_contract() {
        if !ontology.is_empty() {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: ontology[0].line_number,
                message: format!(
                    "Semantic query contract '{}' must not contain a #### Ontology section. Ontology declarations belong in ontology elements.",
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
                    "Semantic query contract '{}' must not contain a #### Shapes section. SHACL profiles belong in semantic-contract elements.",
                    element.name
                ),
            });
        }
        if query.len() != 1 {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: element.line_number,
                message: format!(
                    "Semantic query contract '{}' must contain exactly one #### Query fenced SPARQL block.",
                    element.name
                ),
            });
        } else if !validate_sparql_language(&query[0].language) {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: query[0].line_number,
                message: "Query fenced block must use language tag 'sparql'.".to_string(),
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
        if has_query_section {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: query_line_number,
                message: format!(
                    "Element '{}' is type '{}' and must not contain a #### Query section. Use type: semantic-query-contract.",
                    element.name,
                    element.element_type.as_str()
                ),
            });
        }
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
                "Element '{}' must contain at most one #### Query fenced SPARQL block.",
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

    match parse_turtle_block(&block.content) {
        Ok(quads) => Some(SemanticBlock {
            kind,
            source: element.identifier.clone(),
            source_name: element.name.clone(),
            file_path: element.file_path.clone(),
            line_number: block.line_number,
            language: block.language.clone(),
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

fn validate_turtle_language(language: &str) -> bool {
    matches!(language.to_ascii_lowercase().as_str(), "turtle" | "ttl")
}

fn validate_sparql_language(language: &str) -> bool {
    language == "sparql"
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

fn subject_iri(subject: &NamedOrBlankNode) -> Option<&str> {
    match subject {
        NamedOrBlankNode::NamedNode(node) => Some(node.as_str()),
        NamedOrBlankNode::BlankNode(_) => None,
    }
}

fn term_iri(term: &Term) -> Option<&str> {
    match term {
        Term::NamedNode(node) => Some(node.as_str()),
        _ => None,
    }
}

fn ontology_term_role(type_iri: &str) -> Option<OntologyTermRole> {
    match type_iri {
        OWL_CLASS | RDFS_CLASS => Some(OntologyTermRole::Class),
        RDF_PROPERTY => Some(OntologyTermRole::Property),
        OWL_OBJECT_PROPERTY => Some(OntologyTermRole::ObjectProperty),
        OWL_DATATYPE_PROPERTY => Some(OntologyTermRole::DatatypeProperty),
        _ => None,
    }
}

fn term_as_node(term: &Term) -> Option<NamedOrBlankNode> {
    match term {
        Term::NamedNode(node) => Some(NamedOrBlankNode::NamedNode(node.clone())),
        Term::BlankNode(node) => Some(NamedOrBlankNode::BlankNode(node.clone())),
        Term::Literal(_) => None,
    }
}

fn objects_for<'a>(
    graph: &'a [Quad],
    subject: &NamedOrBlankNode,
    predicate: &str,
) -> Vec<&'a Term> {
    graph
        .iter()
        .filter(|quad| &quad.subject == subject && quad.predicate.as_str() == predicate)
        .map(|quad| &quad.object)
        .collect()
}

fn subjects_with_type(graph: &[Quad], type_iri: &str) -> Vec<NamedOrBlankNode> {
    graph
        .iter()
        .filter(|quad| {
            quad.predicate.as_str() == RDF_TYPE && term_iri(&quad.object) == Some(type_iri)
        })
        .map(|quad| quad.subject.clone())
        .collect()
}

fn parse_non_negative_count(term: &Term, name: &str) -> Result<u64, String> {
    let Term::Literal(literal) = term else {
        return Err(format!("{name} must be a non-negative integer literal"));
    };

    let value = literal
        .value()
        .parse::<i64>()
        .map_err(|_| format!("{name} must be a non-negative integer literal"))?;
    if value < 0 {
        return Err(format!("{name} must be a non-negative integer literal"));
    }
    Ok(value as u64)
}

fn validate_rdf_list(
    graph: &[Quad],
    head: &Term,
    seen: &mut HashSet<String>,
) -> Result<(), String> {
    if term_iri(head) == Some(RDF_NIL) {
        return Ok(());
    }

    let Some(node) = term_as_node(head) else {
        return Err("sh:in must point to an RDF list node or rdf:nil".to_string());
    };

    if matches!(node, NamedOrBlankNode::NamedNode(_)) {
        return Err("sh:in RDF list nodes must be blank nodes or rdf:nil".to_string());
    }

    if !seen.insert(node.to_string()) {
        return Err("sh:in RDF list contains a cycle".to_string());
    }

    let first = objects_for(graph, &node, RDF_FIRST);
    let rest = objects_for(graph, &node, RDF_REST);
    if first.len() != 1 || rest.len() != 1 {
        return Err(
            "sh:in RDF list nodes must have exactly one rdf:first and rdf:rest".to_string(),
        );
    }

    validate_rdf_list(graph, rest[0], seen)
}

fn validate_property_shape(
    property_shape: &NamedOrBlankNode,
    _ontology: &[Quad],
    shapes: &[Quad],
) -> Vec<String> {
    let mut errors = Vec::new();
    let shape_name = property_shape.to_string();
    let paths = objects_for(shapes, property_shape, SH_PATH);

    if paths.len() != 1 {
        errors.push(format!(
            "SHACL property shape {shape_name} must define exactly one sh:path"
        ));
    } else if term_iri(paths[0]).is_some() {
    } else {
        errors.push(format!(
            "SHACL property shape {shape_name} sh:path must be an IRI"
        ));
    }

    let mut min_count = None;
    for term in objects_for(shapes, property_shape, SH_MIN_COUNT) {
        match parse_non_negative_count(term, "sh:minCount") {
            Ok(value) => min_count = Some(value),
            Err(message) => errors.push(format!("SHACL property shape {shape_name} {message}")),
        }
    }

    let mut max_count = None;
    for term in objects_for(shapes, property_shape, SH_MAX_COUNT) {
        match parse_non_negative_count(term, "sh:maxCount") {
            Ok(value) => max_count = Some(value),
            Err(message) => errors.push(format!("SHACL property shape {shape_name} {message}")),
        }
    }

    if let (Some(min), Some(max)) = (min_count, max_count) {
        if max < min {
            errors.push(format!(
                "SHACL property shape {shape_name} sh:maxCount must be greater than or equal to sh:minCount"
            ));
        }
    }

    for term in objects_for(shapes, property_shape, SH_DATATYPE) {
        if term_iri(term).is_none() {
            errors.push(format!(
                "SHACL property shape {shape_name} sh:datatype must be an IRI"
            ));
        }
    }

    for term in objects_for(shapes, property_shape, SH_CLASS) {
        if term_iri(term).is_none() {
            errors.push(format!(
                "SHACL property shape {shape_name} sh:class must be an IRI"
            ));
        }
    }

    for term in objects_for(shapes, property_shape, SH_NODE_KIND) {
        if !matches!(term_iri(term), Some(iri) if SH_NODE_KINDS.contains(&iri)) {
            errors.push(format!(
                "SHACL property shape {shape_name} sh:nodeKind must be a supported SHACL node kind IRI"
            ));
        }
    }

    for term in objects_for(shapes, property_shape, SH_PATTERN) {
        if !matches!(term, Term::Literal(_)) {
            errors.push(format!(
                "SHACL property shape {shape_name} sh:pattern must be a string literal"
            ));
        }
    }

    for term in objects_for(shapes, property_shape, SH_IN) {
        if let Err(message) = validate_rdf_list(shapes, term, &mut HashSet::new()) {
            errors.push(format!("SHACL property shape {shape_name} {message}"));
        }
    }

    errors
}

fn validate_shacl_sanity(shapes: &[Quad]) -> Vec<String> {
    let mut errors = Vec::new();
    let node_shapes = subjects_with_type(shapes, SH_NODE_SHAPE);
    let property_shapes = subjects_with_type(shapes, SH_PROPERTY_SHAPE);

    if node_shapes.is_empty() && property_shapes.is_empty() {
        errors.push(
            "Shapes graph must contain at least one sh:NodeShape or sh:PropertyShape".to_string(),
        );
        return errors;
    }

    let mut validated_property_shapes = HashSet::new();
    for node_shape in &node_shapes {
        let shape_name = node_shape.to_string();
        let target_classes = objects_for(shapes, node_shape, SH_TARGET_CLASS);
        if target_classes.is_empty() {
            errors.push(format!(
                "SHACL node shape {shape_name} must define at least one sh:targetClass"
            ));
        }

        for target in target_classes {
            if term_iri(target).is_none() {
                errors.push(format!(
                    "SHACL node shape {shape_name} sh:targetClass must be an IRI"
                ));
            }
        }

        for property in objects_for(shapes, node_shape, SH_PROPERTY) {
            if let Some(property_shape) = term_as_node(property) {
                validated_property_shapes.insert(property_shape.to_string());
                errors.extend(validate_property_shape(&property_shape, &[], shapes));
            } else {
                errors.push(format!(
                    "SHACL node shape {shape_name} sh:property must point to a property shape"
                ));
            }
        }
    }

    for property_shape in &property_shapes {
        if validated_property_shapes.insert(property_shape.to_string()) {
            errors.extend(validate_property_shape(property_shape, &[], shapes));
        }
    }

    errors
}

fn build_ontology_projection(
    registry: &GraphRegistry,
    blocks: &[SemanticBlock],
) -> OntologyProjectionGraph {
    let pattern_contracts = ontology_projection_pattern_contracts(registry);
    let rdf_lists = collect_projection_rdf_lists(blocks);
    let object_index = collect_projection_object_index(blocks);
    let mut builder = OntologyProjectionBuilder::new(pattern_contracts, rdf_lists, object_index);

    for block in blocks {
        for quad in &block.quads {
            builder.visit_quad(block, quad);
        }
    }

    builder.finish()
}

fn ontology_projection_pattern_contracts(
    registry: &GraphRegistry,
) -> BTreeMap<OntologyConstructFamily, String> {
    let mut contracts = BTreeMap::new();
    for element in registry.get_all_elements() {
        if !element.element_type.is_semantic_query_contract() {
            continue;
        }

        let family = match element.name.as_str() {
            "Direct OWL Construct Projection Query Contract" => {
                OntologyConstructFamily::DisjointEquivalenceInverse
            }
            "RDF List OWL Construct Projection Query Contract" => {
                OntologyConstructFamily::PropertyChain
            }
            "OWL Property Metadata Projection Query Contract" => {
                OntologyConstructFamily::PropertyDomainRange
            }
            _ => continue,
        };

        if let Some(contract) = &element.semantic_query_contract {
            contracts.insert(family, contract.iri.clone());
        }
    }

    contracts
}

struct OntologyProjectionBuilder {
    constructs: BTreeMap<String, OntologyConstruct>,
    equivalence_edges: Vec<EquivalenceEdge>,
    pattern_contracts: BTreeMap<OntologyConstructFamily, String>,
    rdf_lists: BTreeMap<String, Vec<OntologyConstructMember>>,
    object_index: BTreeMap<(String, String), Vec<OntologyProjectionTerm>>,
}

impl OntologyProjectionBuilder {
    fn new(
        pattern_contracts: BTreeMap<OntologyConstructFamily, String>,
        rdf_lists: BTreeMap<String, Vec<OntologyConstructMember>>,
        object_index: BTreeMap<(String, String), Vec<OntologyProjectionTerm>>,
    ) -> Self {
        Self {
            constructs: BTreeMap::new(),
            equivalence_edges: Vec::new(),
            pattern_contracts,
            rdf_lists,
            object_index,
        }
    }

    fn visit_quad(&mut self, block: &SemanticBlock, quad: &Quad) {
        match quad.predicate.as_str() {
            RDFS_DOMAIN => {
                self.add_direct_construct(
                    block,
                    quad,
                    OntologyConstructFamily::PropertyDomainRange,
                    OntologyConstructKind::PropertyDomain,
                    None,
                    None,
                    None,
                    None,
                    Vec::new(),
                );
            }
            RDFS_RANGE => {
                self.add_direct_construct(
                    block,
                    quad,
                    OntologyConstructFamily::PropertyDomainRange,
                    OntologyConstructKind::PropertyRange,
                    None,
                    None,
                    None,
                    None,
                    Vec::new(),
                );
            }
            RDFS_SUBCLASS_OF => {
                self.add_direct_construct(
                    block,
                    quad,
                    OntologyConstructFamily::SubclassMembership,
                    OntologyConstructKind::SubclassInclusion,
                    symbol("subset-or-equal"),
                    None,
                    None,
                    None,
                    Vec::new(),
                );
            }
            RDF_TYPE => self.add_type_construct(block, quad),
            OWL_DISJOINT_WITH => {
                self.add_direct_construct(
                    block,
                    quad,
                    OntologyConstructFamily::DisjointEquivalenceInverse,
                    OntologyConstructKind::Disjointness,
                    symbol("disjointness"),
                    None,
                    None,
                    None,
                    Vec::new(),
                );
            }
            OWL_EQUIVALENT_CLASS | OWL_EQUIVALENT_PROPERTY | OWL_SAME_AS => {
                self.record_equivalence_edge(block, quad);
            }
            OWL_INVERSE_OF => {
                self.add_direct_construct(
                    block,
                    quad,
                    OntologyConstructFamily::DisjointEquivalenceInverse,
                    OntologyConstructKind::InverseProperty,
                    symbol("inverse-property"),
                    None,
                    None,
                    None,
                    Vec::new(),
                );
            }
            OWL_PROPERTY_CHAIN_AXIOM => {
                let members = self.members_for_list_term(&quad.object);
                self.add_direct_construct(
                    block,
                    quad,
                    OntologyConstructFamily::PropertyChain,
                    OntologyConstructKind::PropertyChain,
                    symbol("logical-implication"),
                    None,
                    None,
                    None,
                    members,
                );
            }
            OWL_ALL_VALUES_FROM => self.add_restriction_construct(
                block,
                quad,
                OntologyRestrictionKind::Universal,
                symbol("universal-restriction"),
            ),
            OWL_SOME_VALUES_FROM => self.add_restriction_construct(
                block,
                quad,
                OntologyRestrictionKind::Existential,
                symbol("existential-restriction"),
            ),
            OWL_HAS_VALUE => {
                self.add_restriction_construct(block, quad, OntologyRestrictionKind::HasValue, None)
            }
            OWL_CARDINALITY => self.add_restriction_construct(
                block,
                quad,
                OntologyRestrictionKind::Cardinality,
                None,
            ),
            OWL_MIN_CARDINALITY => self.add_restriction_construct(
                block,
                quad,
                OntologyRestrictionKind::MinCardinality,
                None,
            ),
            OWL_MAX_CARDINALITY => self.add_restriction_construct(
                block,
                quad,
                OntologyRestrictionKind::MaxCardinality,
                None,
            ),
            OWL_QUALIFIED_CARDINALITY => self.add_restriction_construct(
                block,
                quad,
                OntologyRestrictionKind::QualifiedCardinality,
                None,
            ),
            OWL_MIN_QUALIFIED_CARDINALITY => self.add_restriction_construct(
                block,
                quad,
                OntologyRestrictionKind::MinQualifiedCardinality,
                None,
            ),
            OWL_MAX_QUALIFIED_CARDINALITY => self.add_restriction_construct(
                block,
                quad,
                OntologyRestrictionKind::MaxQualifiedCardinality,
                None,
            ),
            OWL_ON_CLASS => {
                self.add_restriction_construct(block, quad, OntologyRestrictionKind::OnClass, None)
            }
            OWL_ON_DATA_RANGE => self.add_restriction_construct(
                block,
                quad,
                OntologyRestrictionKind::OnDataRange,
                None,
            ),
            OWL_INTERSECTION_OF => self.add_class_expression_construct(
                block,
                quad,
                OntologyClassExpressionKind::Intersection,
                symbol("intersection"),
            ),
            OWL_UNION_OF => self.add_class_expression_construct(
                block,
                quad,
                OntologyClassExpressionKind::Union,
                symbol("union"),
            ),
            OWL_COMPLEMENT_OF => self.add_class_expression_construct(
                block,
                quad,
                OntologyClassExpressionKind::Complement,
                None,
            ),
            SH_TARGET_CLASS | SH_PROPERTY | SH_PATH | SH_DATATYPE | SH_CLASS | SH_NODE_KIND
            | SH_MIN_COUNT | SH_MAX_COUNT | SH_PATTERN | SH_IN => {
                self.add_shape_overlay_construct(block, quad)
            }
            _ => {}
        }
    }

    fn record_equivalence_edge(&mut self, block: &SemanticBlock, quad: &Quad) {
        let source = projection_source(block);
        let subject = projection_term_from_subject(&quad.subject);
        let predicate = projection_term_from_predicate(quad.predicate.as_str());
        let object = projection_term_from_term(&quad.object);
        self.equivalence_edges.push(EquivalenceEdge {
            evidence: OntologyProjectionEvidence {
                source: source.clone(),
                subject: subject.clone(),
                predicate: predicate.clone(),
                object: object.clone(),
            },
            subject,
            object,
        });
    }

    fn add_type_construct(&mut self, block: &SemanticBlock, quad: &Quad) {
        let Some(object_iri) = term_iri(&quad.object) else {
            return;
        };

        if let Some(characteristic) = property_characteristic_for_type(object_iri) {
            self.add_direct_construct(
                block,
                quad,
                OntologyConstructFamily::PropertyCharacteristic,
                OntologyConstructKind::PropertyCharacteristic,
                symbol(characteristic.as_str()),
                Some(characteristic),
                None,
                None,
                Vec::new(),
            );
            return;
        }

        if is_declaration_type(object_iri) || object_iri == OWL_RESTRICTION {
            if object_iri == SH_NODE_SHAPE || object_iri == SH_PROPERTY_SHAPE {
                self.add_shape_overlay_construct(block, quad);
            }
            return;
        }

        self.add_direct_construct(
            block,
            quad,
            OntologyConstructFamily::SubclassMembership,
            OntologyConstructKind::Membership,
            symbol("member-of"),
            None,
            None,
            None,
            Vec::new(),
        );
    }

    fn add_restriction_construct(
        &mut self,
        block: &SemanticBlock,
        quad: &Quad,
        restriction_kind: OntologyRestrictionKind,
        symbol: Option<OntologySymbol>,
    ) {
        let property = self
            .objects_for_subject(&quad.subject, OWL_ON_PROPERTY)
            .first()
            .cloned();
        self.add_direct_construct(
            block,
            quad,
            OntologyConstructFamily::Restriction,
            OntologyConstructKind::Restriction,
            symbol,
            None,
            Some(restriction_kind),
            None,
            Vec::new(),
        )
        .with_property(property);
    }

    fn add_class_expression_construct(
        &mut self,
        block: &SemanticBlock,
        quad: &Quad,
        expression_kind: OntologyClassExpressionKind,
        symbol: Option<OntologySymbol>,
    ) {
        let members = self.members_for_list_term(&quad.object);
        self.add_direct_construct(
            block,
            quad,
            OntologyConstructFamily::ClassExpression,
            OntologyConstructKind::ClassExpression,
            symbol,
            None,
            None,
            Some(expression_kind),
            members,
        );
    }

    fn add_shape_overlay_construct(&mut self, block: &SemanticBlock, quad: &Quad) {
        let shape_overlay_kind = match quad.predicate.as_str() {
            RDF_TYPE if term_iri(&quad.object) == Some(SH_NODE_SHAPE) => {
                Some(OntologyShapeOverlayKind::NodeShape)
            }
            RDF_TYPE if term_iri(&quad.object) == Some(SH_PROPERTY_SHAPE) => {
                Some(OntologyShapeOverlayKind::PropertyShape)
            }
            _ => self.shape_overlay_kind_for_subject(&quad.subject),
        };

        self.add_direct_construct(
            block,
            quad,
            OntologyConstructFamily::ShapeOverlay,
            OntologyConstructKind::ShapeOverlay,
            None,
            None,
            None,
            None,
            Vec::new(),
        )
        .with_shape_overlay_kind(shape_overlay_kind);
    }

    #[allow(clippy::too_many_arguments)]
    fn add_direct_construct(
        &mut self,
        block: &SemanticBlock,
        quad: &Quad,
        family: OntologyConstructFamily,
        kind: OntologyConstructKind,
        symbol: Option<OntologySymbol>,
        property_characteristic: Option<OntologyPropertyCharacteristic>,
        restriction_kind: Option<OntologyRestrictionKind>,
        class_expression_kind: Option<OntologyClassExpressionKind>,
        members: Vec<OntologyConstructMember>,
    ) -> ConstructMut<'_> {
        let source = projection_source(block);
        let evidence = OntologyProjectionEvidence {
            source: source.clone(),
            subject: projection_term_from_subject(&quad.subject),
            predicate: projection_term_from_predicate(quad.predicate.as_str()),
            object: projection_term_from_term(&quad.object),
        };
        let provenance = OntologyProjectionProvenance {
            derivation_mode: OntologyProjectionDerivationMode::DirectAuthored,
            source,
            pattern_contract_iri: self.pattern_contract_for_family(family),
            evidence: vec![evidence],
        };
        let subject = projection_term_from_subject(&quad.subject);
        let predicate = projection_term_from_predicate(quad.predicate.as_str());
        let object = projection_term_from_term(&quad.object);
        let id = construct_id(kind, &subject, Some(&predicate), Some(&object), &members);
        let construct = OntologyConstruct {
            id: id.clone(),
            family,
            kind,
            subject,
            predicate: Some(predicate),
            object: Some(object),
            property: None,
            members,
            property_characteristic,
            restriction_kind,
            class_expression_kind,
            shape_overlay_kind: None,
            symbol,
            provenance,
        };

        self.constructs.entry(id.clone()).or_insert(construct);
        ConstructMut {
            construct: self.constructs.get_mut(&id),
        }
    }

    fn pattern_contract_for_family(&self, family: OntologyConstructFamily) -> Option<String> {
        self.pattern_contracts
            .get(&family)
            .or_else(|| match family {
                OntologyConstructFamily::PropertyDomainRange => self
                    .pattern_contracts
                    .get(&OntologyConstructFamily::PropertyDomainRange),
                OntologyConstructFamily::SubclassMembership
                | OntologyConstructFamily::Restriction => self
                    .pattern_contracts
                    .get(&OntologyConstructFamily::DisjointEquivalenceInverse),
                OntologyConstructFamily::ClassExpression => self
                    .pattern_contracts
                    .get(&OntologyConstructFamily::PropertyChain),
                OntologyConstructFamily::PropertyChain => self
                    .pattern_contracts
                    .get(&OntologyConstructFamily::PropertyChain),
                OntologyConstructFamily::PropertyCharacteristic => self
                    .pattern_contracts
                    .get(&OntologyConstructFamily::PropertyDomainRange),
                OntologyConstructFamily::ShapeOverlay
                | OntologyConstructFamily::DisjointEquivalenceInverse => None,
            })
            .cloned()
    }

    fn members_for_list_term(&self, term: &Term) -> Vec<OntologyConstructMember> {
        match term {
            Term::BlankNode(node) => self
                .rdf_lists
                .get(&node.to_string())
                .cloned()
                .unwrap_or_default(),
            Term::NamedNode(node) if node.as_str() == RDF_NIL => Vec::new(),
            _ => Vec::new(),
        }
    }

    fn objects_for_subject(
        &self,
        subject: &NamedOrBlankNode,
        predicate: &str,
    ) -> Vec<OntologyProjectionTerm> {
        self.object_index
            .get(&(subject_key(subject), predicate.to_string()))
            .cloned()
            .unwrap_or_default()
    }

    fn shape_overlay_kind_for_subject(
        &self,
        subject: &NamedOrBlankNode,
    ) -> Option<OntologyShapeOverlayKind> {
        let subject_types = self.objects_for_subject(subject, RDF_TYPE);
        if subject_types.iter().any(|term| term.value == SH_NODE_SHAPE) {
            return Some(OntologyShapeOverlayKind::NodeShape);
        }
        if subject_types
            .iter()
            .any(|term| term.value == SH_PROPERTY_SHAPE)
        {
            return Some(OntologyShapeOverlayKind::PropertyShape);
        }

        let subject_term = projection_term_from_subject(subject);
        self.constructs
            .values()
            .filter(|construct| construct.subject == subject_term)
            .find_map(|construct| construct.shape_overlay_kind)
    }

    fn finish(mut self) -> OntologyProjectionGraph {
        self.materialize_equivalence_groups();

        let pattern_contracts = self.pattern_contracts;
        let mut constructs: Vec<_> = self.constructs.into_values().collect();
        constructs.sort_by(|a, b| {
            a.family
                .cmp(&b.family)
                .then_with(|| a.kind.cmp(&b.kind))
                .then_with(|| a.id.cmp(&b.id))
        });

        let mut projections_by_family: BTreeMap<OntologyConstructFamily, Vec<String>> =
            BTreeMap::new();
        let mut symbols = BTreeSet::new();
        for construct in &constructs {
            projections_by_family
                .entry(construct.family)
                .or_default()
                .push(construct.id.clone());
            if let Some(symbol) = &construct.symbol {
                symbols.insert(symbol.clone());
            }
        }

        let projections = projections_by_family
            .into_iter()
            .map(|(family, mut construct_ids)| {
                construct_ids.sort();
                construct_ids.dedup();
                OntologyConstructProjection {
                    id: format!(
                        "urn:reqvire:ontology-projection:{}:{}",
                        family.as_str(),
                        stable_hash(&construct_ids.join("|"))
                    ),
                    family,
                    derivation_mode: OntologyProjectionDerivationMode::DirectAuthored,
                    pattern_contract_iri: pattern_contract_for_family(&pattern_contracts, family),
                    construct_ids,
                }
            })
            .collect();

        OntologyProjectionGraph {
            id: "urn:reqvire:ontology-projection:graph:direct-authored".to_string(),
            derivation_mode: OntologyProjectionDerivationMode::DirectAuthored,
            projections,
            constructs,
            symbols: symbols.into_iter().collect(),
        }
    }

    fn materialize_equivalence_groups(&mut self) {
        if self.equivalence_edges.is_empty() {
            return;
        }

        let mut parent: BTreeMap<String, String> = BTreeMap::new();
        let mut terms: BTreeMap<String, OntologyProjectionTerm> = BTreeMap::new();
        for edge in &self.equivalence_edges {
            let subject_key = projection_term_key(&edge.subject);
            let object_key = projection_term_key(&edge.object);
            terms.insert(subject_key.clone(), edge.subject.clone());
            terms.insert(object_key.clone(), edge.object.clone());
            uf_union_strings(&mut parent, &subject_key, &object_key);
        }

        let members: Vec<String> = parent.keys().cloned().collect();
        let mut groups: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for member in members {
            let root = uf_find_string(&mut parent, &member);
            groups.entry(root).or_default().insert(member);
        }

        for group_members in groups.values().filter(|members| members.len() > 1) {
            let Some(first_member_key) = group_members.iter().next() else {
                continue;
            };
            let Some(subject) = terms.get(first_member_key).cloned() else {
                continue;
            };
            let mut evidence: Vec<_> = self
                .equivalence_edges
                .iter()
                .filter(|edge| {
                    group_members.contains(&projection_term_key(&edge.subject))
                        && group_members.contains(&projection_term_key(&edge.object))
                })
                .map(|edge| edge.evidence.clone())
                .collect();
            evidence.sort();
            evidence.dedup();

            let Some(source) = evidence.first().map(|evidence| evidence.source.clone()) else {
                continue;
            };

            let mut members = Vec::new();
            for member_key in group_members {
                if let Some(term) = terms.get(member_key) {
                    members.push(OntologyConstructMember {
                        sequence_index: members.len(),
                        term: term.clone(),
                        source: source.clone(),
                    });
                }
            }

            let id = construct_id(
                OntologyConstructKind::EquivalenceGroup,
                &subject,
                None,
                None,
                &members,
            );
            let construct = OntologyConstruct {
                id: id.clone(),
                family: OntologyConstructFamily::DisjointEquivalenceInverse,
                kind: OntologyConstructKind::EquivalenceGroup,
                subject,
                predicate: None,
                object: None,
                property: None,
                members,
                property_characteristic: None,
                restriction_kind: None,
                class_expression_kind: None,
                shape_overlay_kind: None,
                symbol: symbol("logical-equivalence"),
                provenance: OntologyProjectionProvenance {
                    derivation_mode: OntologyProjectionDerivationMode::DirectAuthored,
                    source,
                    pattern_contract_iri: self.pattern_contract_for_family(
                        OntologyConstructFamily::DisjointEquivalenceInverse,
                    ),
                    evidence,
                },
            };
            self.constructs.entry(id).or_insert(construct);
        }
    }
}

#[derive(Debug, Clone)]
struct EquivalenceEdge {
    subject: OntologyProjectionTerm,
    object: OntologyProjectionTerm,
    evidence: OntologyProjectionEvidence,
}

fn pattern_contract_for_family(
    pattern_contracts: &BTreeMap<OntologyConstructFamily, String>,
    family: OntologyConstructFamily,
) -> Option<String> {
    pattern_contracts
        .get(&family)
        .or_else(|| match family {
            OntologyConstructFamily::PropertyDomainRange => {
                pattern_contracts.get(&OntologyConstructFamily::PropertyDomainRange)
            }
            OntologyConstructFamily::SubclassMembership | OntologyConstructFamily::Restriction => {
                pattern_contracts.get(&OntologyConstructFamily::DisjointEquivalenceInverse)
            }
            OntologyConstructFamily::ClassExpression | OntologyConstructFamily::PropertyChain => {
                pattern_contracts.get(&OntologyConstructFamily::PropertyChain)
            }
            OntologyConstructFamily::PropertyCharacteristic => {
                pattern_contracts.get(&OntologyConstructFamily::PropertyDomainRange)
            }
            OntologyConstructFamily::ShapeOverlay
            | OntologyConstructFamily::DisjointEquivalenceInverse => None,
        })
        .cloned()
}

struct ConstructMut<'a> {
    construct: Option<&'a mut OntologyConstruct>,
}

impl ConstructMut<'_> {
    fn with_property(self, property: Option<OntologyProjectionTerm>) {
        if let Some(construct) = self.construct {
            construct.property = property;
        }
    }

    fn with_shape_overlay_kind(self, shape_overlay_kind: Option<OntologyShapeOverlayKind>) {
        if let Some(construct) = self.construct {
            construct.shape_overlay_kind = shape_overlay_kind;
        }
    }
}

fn collect_projection_rdf_lists(
    blocks: &[SemanticBlock],
) -> BTreeMap<String, Vec<OntologyConstructMember>> {
    let mut first_values: BTreeMap<String, (OntologyProjectionTerm, OntologyProjectionSource)> =
        BTreeMap::new();
    let mut rest_targets: BTreeMap<String, String> = BTreeMap::new();
    let mut list_nodes = BTreeSet::new();

    for block in blocks {
        for quad in &block.quads {
            let subject = subject_key(&quad.subject);
            match quad.predicate.as_str() {
                RDF_FIRST => {
                    list_nodes.insert(subject.clone());
                    first_values.insert(
                        subject,
                        (
                            projection_term_from_term(&quad.object),
                            projection_source(block),
                        ),
                    );
                }
                RDF_REST => {
                    list_nodes.insert(subject.clone());
                    match &quad.object {
                        Term::BlankNode(node) => {
                            let target = node.to_string();
                            list_nodes.insert(target.clone());
                            rest_targets.insert(subject, target);
                        }
                        Term::NamedNode(node) if node.as_str() == RDF_NIL => {
                            rest_targets.insert(subject, RDF_NIL.to_string());
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    let mut lists = BTreeMap::new();
    for head in list_nodes {
        let mut members = Vec::new();
        let mut current = head.as_str();
        let mut seen = BTreeSet::new();

        while seen.insert(current.to_string()) {
            if let Some((term, source)) = first_values.get(current) {
                members.push(OntologyConstructMember {
                    sequence_index: members.len(),
                    term: term.clone(),
                    source: source.clone(),
                });
            }

            let Some(next) = rest_targets.get(current) else {
                break;
            };
            if next == RDF_NIL {
                break;
            }
            current = next;
        }

        if !members.is_empty() {
            lists.insert(head, members);
        }
    }

    lists
}

fn collect_projection_object_index(
    blocks: &[SemanticBlock],
) -> BTreeMap<(String, String), Vec<OntologyProjectionTerm>> {
    let mut object_index: BTreeMap<(String, String), Vec<OntologyProjectionTerm>> = BTreeMap::new();

    for block in blocks {
        for quad in &block.quads {
            let key = (
                subject_key(&quad.subject),
                quad.predicate.as_str().to_string(),
            );
            let value = projection_term_from_term(&quad.object);
            object_index.entry(key).or_default().push(value);
        }
    }

    for values in object_index.values_mut() {
        values.sort();
        values.dedup();
    }

    object_index
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

fn projection_term_from_subject(subject: &NamedOrBlankNode) -> OntologyProjectionTerm {
    match subject {
        NamedOrBlankNode::NamedNode(node) => projection_iri_term(node.as_str()),
        NamedOrBlankNode::BlankNode(node) => OntologyProjectionTerm {
            kind: OntologyProjectionTermKind::BlankNode,
            value: node.to_string(),
            label: "anonymous node".to_string(),
        },
    }
}

fn projection_term_from_predicate(predicate: &str) -> OntologyProjectionTerm {
    projection_iri_term(predicate)
}

fn projection_term_from_term(term: &Term) -> OntologyProjectionTerm {
    match term {
        Term::NamedNode(node) => projection_iri_term(node.as_str()),
        Term::BlankNode(node) => OntologyProjectionTerm {
            kind: OntologyProjectionTermKind::BlankNode,
            value: node.to_string(),
            label: "anonymous node".to_string(),
        },
        Term::Literal(literal) => OntologyProjectionTerm {
            kind: OntologyProjectionTermKind::Literal,
            value: literal.value().to_string(),
            label: literal.value().to_string(),
        },
    }
}

fn projection_iri_term(iri: &str) -> OntologyProjectionTerm {
    OntologyProjectionTerm {
        kind: OntologyProjectionTermKind::Iri,
        value: iri.to_string(),
        label: compact_iri_label(iri),
    }
}

fn compact_iri_label(value: &str) -> String {
    value
        .trim_matches(|c| c == '<' || c == '>')
        .rsplit(['/', '#'])
        .find(|part| !part.is_empty())
        .unwrap_or(value)
        .to_string()
}

fn subject_key(subject: &NamedOrBlankNode) -> String {
    match subject {
        NamedOrBlankNode::NamedNode(node) => node.as_str().to_string(),
        NamedOrBlankNode::BlankNode(node) => node.to_string(),
    }
}

fn projection_term_key(term: &OntologyProjectionTerm) -> String {
    format!("{}:{}", term.kind.as_str(), term.value)
}

fn uf_find_string(parent: &mut BTreeMap<String, String>, id: &str) -> String {
    let mut root = id.to_string();
    while let Some(next) = parent.get(&root) {
        if next == &root {
            break;
        }
        root = next.clone();
    }

    let mut current = id.to_string();
    while current != root {
        let next = parent
            .get(&current)
            .cloned()
            .unwrap_or_else(|| root.clone());
        parent.insert(current, root.clone());
        current = next;
    }

    root
}

fn uf_union_strings(parent: &mut BTreeMap<String, String>, a: &str, b: &str) {
    parent.entry(a.to_string()).or_insert_with(|| a.to_string());
    parent.entry(b.to_string()).or_insert_with(|| b.to_string());
    let root_a = uf_find_string(parent, a);
    let root_b = uf_find_string(parent, b);
    if root_a != root_b {
        parent.insert(root_a, root_b);
    }
}

fn property_characteristic_for_type(type_iri: &str) -> Option<OntologyPropertyCharacteristic> {
    match type_iri {
        OWL_FUNCTIONAL_PROPERTY => Some(OntologyPropertyCharacteristic::Functional),
        OWL_INVERSE_FUNCTIONAL_PROPERTY => Some(OntologyPropertyCharacteristic::InverseFunctional),
        OWL_SYMMETRIC_PROPERTY => Some(OntologyPropertyCharacteristic::Symmetric),
        OWL_ASYMMETRIC_PROPERTY => Some(OntologyPropertyCharacteristic::Asymmetric),
        OWL_REFLEXIVE_PROPERTY => Some(OntologyPropertyCharacteristic::Reflexive),
        OWL_IRREFLEXIVE_PROPERTY => Some(OntologyPropertyCharacteristic::Irreflexive),
        OWL_TRANSITIVE_PROPERTY => Some(OntologyPropertyCharacteristic::Transitive),
        _ => None,
    }
}

fn is_declaration_type(type_iri: &str) -> bool {
    matches!(
        type_iri,
        OWL_CLASS
            | RDFS_CLASS
            | RDF_PROPERTY
            | OWL_OBJECT_PROPERTY
            | OWL_DATATYPE_PROPERTY
            | OWL_NAMED_INDIVIDUAL
            | RDFS_DATATYPE
            | SH_NODE_SHAPE
            | SH_PROPERTY_SHAPE
    )
}

fn symbol(concept_name: &str) -> Option<OntologySymbol> {
    let (raw_unicode_code_point, rendered_unicode_character, tooltip) = match concept_name {
        "disjointness" => ("U+27C2", "⟂", "Disjointness"),
        "intersection" => ("U+2229", "∩", "Intersection"),
        "union" => ("U+222A", "∪", "Union"),
        "logical-implication" => ("U+21D2", "⇒", "Implies"),
        "logical-equivalence" => ("U+21D4", "⇔", "Equivalent"),
        "universal-restriction" => ("U+2200", "∀", "Universal restriction"),
        "existential-restriction" => ("U+2203", "∃", "Existential restriction"),
        "member-of" => ("U+2208", "∈", "Member of"),
        "subset-or-equal" => ("U+2286", "⊆", "Subset or equal"),
        "symmetric" => ("U+2194", "↔", "Symmetric property"),
        "inverse-property" => ("U+27F2", "⟲", "Inverse property"),
        "reflexive" => ("U+25CB", "○", "Reflexive property"),
        "irreflexive" => ("U+2209", "∉", "Irreflexive property"),
        "transitive" => ("U+25B3", "△", "Transitive property"),
        "asymmetric" => ("U+21AE", "↮", "Asymmetric property"),
        "inverse-functional" => ("U+2190", "←", "Inverse functional property"),
        "functional" => ("U+2192", "→", "Functional property"),
        _ => return None,
    };

    Some(OntologySymbol {
        concept_name: concept_name.to_string(),
        raw_unicode_code_point: raw_unicode_code_point.to_string(),
        rendered_unicode_character: rendered_unicode_character.to_string(),
        tooltip: tooltip.to_string(),
        accessible_label: tooltip.to_string(),
    })
}

fn construct_id(
    kind: OntologyConstructKind,
    subject: &OntologyProjectionTerm,
    predicate: Option<&OntologyProjectionTerm>,
    object: Option<&OntologyProjectionTerm>,
    members: &[OntologyConstructMember],
) -> String {
    let mut canonical = format!(
        "{}|{}:{}",
        kind.as_str(),
        subject.kind.as_str(),
        subject.value
    );
    if let Some(predicate) = predicate {
        canonical.push_str(&format!(
            "|p:{}:{}",
            predicate.kind.as_str(),
            predicate.value
        ));
    }
    if let Some(object) = object {
        canonical.push_str(&format!("|o:{}:{}", object.kind.as_str(), object.value));
    }
    for member in members {
        canonical.push_str(&format!(
            "|m:{}:{}:{}",
            member.sequence_index,
            member.term.kind.as_str(),
            member.term.value
        ));
    }

    format!(
        "urn:reqvire:ontology-construct:{}:{}",
        kind.as_str(),
        stable_hash(&canonical)
    )
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
) -> Vec<OntologyTermDeclaration> {
    quads
        .iter()
        .filter(|quad| quad.predicate.as_str() == RDF_TYPE)
        .filter_map(|quad| {
            let iri = subject_iri(&quad.subject)?;
            let role = ontology_term_role(term_iri(&quad.object)?)?;
            Some(OntologyTermDeclaration {
                iri: iri.to_string(),
                role,
                element_identifier: element.identifier.clone(),
            })
        })
        .collect()
}

fn shape_iri_references_from_quads(element: &Element, quads: &[Quad]) -> Vec<ShapeIriReference> {
    let mut references = BTreeSet::new();
    for quad in quads {
        let kind = match quad.predicate.as_str() {
            SH_TARGET_CLASS => "sh:targetClass",
            SH_PATH => "sh:path",
            SH_CLASS => "sh:class",
            _ => continue,
        };
        let Some(iri) = term_iri(&quad.object) else {
            continue;
        };
        if kind == "sh:path" && is_builtin_annotation_path(iri) {
            continue;
        }
        references.insert(ShapeIriReference {
            iri: iri.to_string(),
            kind: kind.to_string(),
            element_identifier: element.identifier.clone(),
        });
    }

    references.into_iter().collect()
}

fn is_builtin_annotation_path(iri: &str) -> bool {
    matches!(iri, RDFS_LABEL | RDFS_COMMENT)
}
