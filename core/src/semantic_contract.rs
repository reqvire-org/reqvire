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
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt;

const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const RDF_FIRST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#first";
const RDF_REST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#rest";
const RDF_NIL: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#nil";
const RDFS_CLASS: &str = "http://www.w3.org/2000/01/rdf-schema#Class";
const OWL_CLASS: &str = "http://www.w3.org/2002/07/owl#Class";
const RDF_PROPERTY: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#Property";
const OWL_OBJECT_PROPERTY: &str = "http://www.w3.org/2002/07/owl#ObjectProperty";
const OWL_DATATYPE_PROPERTY: &str = "http://www.w3.org/2002/07/owl#DatatypeProperty";
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

#[derive(Debug, Clone, Serialize)]
pub struct SemanticIndex {
    pub blocks: Vec<SemanticBlock>,
    pub diagnostics: Vec<SemanticDiagnostic>,
    pub ontology_declarations: HashMap<String, Vec<OntologyTermDeclaration>>,
    pub shape_references: Vec<ShapeIriReference>,
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

        validate_semantic_sections(element, &ontology, &shapes, &mut diagnostics);

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
    }
}

fn validate_semantic_sections(
    element: &Element,
    ontology: &[FencedBlock],
    shapes: &[FencedBlock],
    diagnostics: &mut Vec<SemanticDiagnostic>,
) {
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
                    "Ontology element '{}' must not contain a #### Shapes section. SHACL profiles belong in capability-owned or requirement-owned semantic-contract elements.",
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
        references.insert(ShapeIriReference {
            iri: iri.to_string(),
            kind: kind.to_string(),
            element_identifier: element.identifier.clone(),
        });
    }

    references.into_iter().collect()
}
