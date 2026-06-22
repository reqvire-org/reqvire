use crate::diagnostics::{
    Diagnostic, DiagnosticCode, DiagnosticSeverity, CODE_SHACL_INVALID_CONSTRAINT,
    CODE_SHACL_INVALID_PATH, CODE_SHACL_INVALID_SHAPE_REFERENCE, CODE_SHACL_MISSING_SHAPE_NODES,
};
use crate::vocab::reserved as owl_reserved;
use crate::vocab::*;
use oxigraph::model::{NamedNode, NamedOrBlankNode, Quad, Term};
use std::collections::{HashMap, HashSet};

const SH_NODE_KINDS: &[(&str, NodeKindVariant)] = &[
    (
        "http://www.w3.org/ns/shacl#BlankNode",
        NodeKindVariant::BlankNode,
    ),
    ("http://www.w3.org/ns/shacl#IRI", NodeKindVariant::Iri),
    (
        "http://www.w3.org/ns/shacl#Literal",
        NodeKindVariant::Literal,
    ),
    (
        "http://www.w3.org/ns/shacl#BlankNodeOrIRI",
        NodeKindVariant::BlankNodeOrIri,
    ),
    (
        "http://www.w3.org/ns/shacl#BlankNodeOrLiteral",
        NodeKindVariant::BlankNodeOrLiteral,
    ),
    (
        "http://www.w3.org/ns/shacl#IRIOrLiteral",
        NodeKindVariant::IriOrLiteral,
    ),
];

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Node(NodeShape),
    Property(PropertyShape),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TargetIdentifier {
    Class(NamedOrBlankNode),
    Node(Term),
    SubjectsOf(NamedNode),
    ObjectsOf(NamedNode),
    Custom(NamedOrBlankNode),
    ImplicitClass(NamedOrBlankNode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstPath {
    Iri(NamedNode),
    Inverse(NamedNode),
    Sequence(Vec<AstPath>),
    Alternative(Vec<AstPath>),
    ZeroOrMore(Box<AstPath>),
    OneOrMore(Box<AstPath>),
    ZeroOrOne(Box<AstPath>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstConstraint {
    pub predicate: NamedNode,
    pub value: Term,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeShape {
    pub id: NamedOrBlankNode,
    pub targets: Vec<TargetIdentifier>,
    pub constraints: Vec<SyntaxConstraint>,
    pub raw_constraints: Vec<AstConstraint>,
    pub property_shapes: Vec<NamedOrBlankNode>,
    pub is_closed: bool,
    pub ignored_properties: Vec<NamedNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyShape {
    pub id: NamedOrBlankNode,
    pub path: Option<AstPath>,
    pub targets: Vec<TargetIdentifier>,
    pub constraints: Vec<SyntaxConstraint>,
    pub raw_constraints: Vec<AstConstraint>,
    pub property_shapes: Vec<NamedOrBlankNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReferencedIri {
    pub iri: NamedNode,
    pub predicate: &'static str,
}

impl ReferencedIri {
    pub fn predicate_label(&self) -> String {
        predicate_label(self.predicate)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxConstraint {
    Class {
        class_node: NamedOrBlankNode,
    },
    Datatype {
        datatype_iri: NamedNode,
    },
    NodeKind {
        kind: NodeKindVariant,
    },
    MinCount(u32),
    MaxCount(u32),
    MinExclusive(Term),
    MinInclusive(Term),
    MaxExclusive(Term),
    MaxInclusive(Term),
    MinLength(u32),
    MaxLength(u32),
    Pattern {
        expression: String,
        flags: Option<String>,
    },
    LanguageIn(Vec<String>),
    UniqueLang(bool),
    Equals {
        property_iri: NamedNode,
    },
    Disjoint {
        property_iri: NamedNode,
    },
    LessThan {
        property_iri: NamedNode,
    },
    LessThanOrEquals {
        property_iri: NamedNode,
    },
    And(Vec<NamedOrBlankNode>),
    Or(Vec<NamedOrBlankNode>),
    Not(NamedOrBlankNode),
    Xone(Vec<NamedOrBlankNode>),
    Node(NamedOrBlankNode),
    QualifiedValueShape {
        shape_id: NamedOrBlankNode,
        min_count: Option<u32>,
        max_count: Option<u32>,
    },
    HasValue(Term),
    In(Vec<Term>),
    Sparql {
        query: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKindVariant {
    BlankNode,
    Iri,
    Literal,
    BlankNodeOrIri,
    BlankNodeOrLiteral,
    IriOrLiteral,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShaclParseIssue {
    MissingShapeNodes,
    InvalidShapeReference {
        shape_id: NamedOrBlankNode,
        predicate: &'static str,
    },
    InvalidPath {
        shape_id: NamedOrBlankNode,
        message: String,
    },
    InvalidConstraint {
        shape_id: NamedOrBlankNode,
        message: String,
    },
}

impl ShaclParseIssue {
    pub const fn code(&self) -> DiagnosticCode {
        match self {
            Self::MissingShapeNodes => CODE_SHACL_MISSING_SHAPE_NODES,
            Self::InvalidShapeReference { .. } => CODE_SHACL_INVALID_SHAPE_REFERENCE,
            Self::InvalidPath { .. } => CODE_SHACL_INVALID_PATH,
            Self::InvalidConstraint { .. } => CODE_SHACL_INVALID_CONSTRAINT,
        }
    }

    #[must_use]
    pub const fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    #[must_use]
    pub fn diagnostic(&self) -> Diagnostic {
        Diagnostic::new(self.code(), self.severity(), self.message())
    }

    pub fn message(&self) -> String {
        match self {
            Self::MissingShapeNodes => {
                "Shapes graph must contain at least one sh:NodeShape, sh:PropertyShape, sh:Shape, or targeted shape node".to_string()
            }
            Self::InvalidShapeReference {
                shape_id,
                predicate,
            } => format!("SHACL shape {shape_id} {predicate} must point to shape nodes"),
            Self::InvalidPath { shape_id, message } => {
                format!("SHACL shape {shape_id} {message}")
            }
            Self::InvalidConstraint { shape_id, message } => {
                format!("SHACL shape {shape_id} {message}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlignmentError {
    UndeclaredClass {
        shape_id: NamedOrBlankNode,
        class_node: NamedOrBlankNode,
        predicate: &'static str,
    },
    UndeclaredProperty {
        shape_id: NamedOrBlankNode,
        property_iri: NamedNode,
        predicate: &'static str,
    },
    UndeclaredDatatype {
        shape_id: NamedOrBlankNode,
        datatype_iri: NamedNode,
        predicate: &'static str,
    },
    UndeclaredNode {
        shape_id: NamedOrBlankNode,
        node_iri: NamedNode,
        predicate: &'static str,
    },
    InvalidInversePath {
        shape_id: NamedOrBlankNode,
        property_iri: NamedNode,
        predicate: &'static str,
    },
}

#[derive(Debug, Clone, Default)]
pub struct DomainOntologyIndex {
    pub declared_classes: HashSet<NamedOrBlankNode>,
    pub declared_properties: HashSet<NamedNode>,
    pub declared_datatypes: HashSet<NamedNode>,
    pub available_terms: HashSet<NamedNode>,
}

impl DomainOntologyIndex {
    pub fn from_quads(quads: &[Quad]) -> Self {
        let mut index = Self::default();

        for iri in [RDFS_LABEL, RDFS_COMMENT] {
            let named_node = NamedNode::new_unchecked(iri);
            index.available_terms.insert(named_node.clone());
            index.declared_properties.insert(named_node);
        }

        for quad in quads {
            if let NamedOrBlankNode::NamedNode(subject) = &quad.subject {
                index.available_terms.insert(subject.clone());
            }
            index.available_terms.insert(quad.predicate.clone());
            if let Term::NamedNode(object) = &quad.object {
                index.available_terms.insert(object.clone());
            }

            if quad.predicate.as_str() != RDF_TYPE {
                continue;
            }

            let Some(role_iri) = term_iri(&quad.object) else {
                continue;
            };

            match role_iri {
                OWL_CLASS | RDFS_CLASS => {
                    index.declared_classes.insert(quad.subject.clone());
                }
                RDFS_DATATYPE => {
                    if let NamedOrBlankNode::NamedNode(subject) = &quad.subject {
                        index.declared_datatypes.insert(subject.clone());
                    }
                }
                RDF_PROPERTY
                | OWL_ANNOTATION_PROPERTY
                | OWL_OBJECT_PROPERTY
                | OWL_DATATYPE_PROPERTY => {
                    if let NamedOrBlankNode::NamedNode(subject) = &quad.subject {
                        index.declared_properties.insert(subject.clone());
                    }
                }
                _ => {}
            }
        }

        index
    }
}

pub struct OntologyAligner<'a> {
    ontology: &'a DomainOntologyIndex,
}

impl<'a> OntologyAligner<'a> {
    pub fn new(ontology: &'a DomainOntologyIndex) -> Self {
        Self { ontology }
    }

    pub fn cross_check_shapes(
        &self,
        shapes: &HashMap<NamedOrBlankNode, Shape>,
    ) -> Vec<AlignmentError> {
        let mut errors = Vec::new();

        for (id, shape) in shapes {
            match shape {
                Shape::Node(node) => {
                    self.verify_targets(id, &node.targets, &mut errors);
                    self.verify_constraints(id, &node.constraints, &mut errors);
                }
                Shape::Property(property) => {
                    self.verify_targets(id, &property.targets, &mut errors);
                    if let Some(path) = &property.path {
                        self.verify_path(id, path, &mut errors);
                    }
                    self.verify_constraints(id, &property.constraints, &mut errors);
                }
            }
        }

        errors
    }

    fn verify_targets(
        &self,
        id: &NamedOrBlankNode,
        targets: &[TargetIdentifier],
        errors: &mut Vec<AlignmentError>,
    ) {
        for target in targets {
            match target {
                TargetIdentifier::Class(class) | TargetIdentifier::ImplicitClass(class) => {
                    if !self.ontology.declared_classes.contains(class) {
                        errors.push(AlignmentError::UndeclaredClass {
                            shape_id: id.clone(),
                            class_node: class.clone(),
                            predicate: SH_TARGET_CLASS,
                        });
                    }
                }
                TargetIdentifier::SubjectsOf(property) => {
                    if !self.ontology.declared_properties.contains(property) {
                        errors.push(AlignmentError::UndeclaredProperty {
                            shape_id: id.clone(),
                            property_iri: property.clone(),
                            predicate: SH_TARGET_SUBJECTS_OF,
                        });
                    }
                }
                TargetIdentifier::ObjectsOf(property) => {
                    if !self.ontology.declared_properties.contains(property) {
                        errors.push(AlignmentError::UndeclaredProperty {
                            shape_id: id.clone(),
                            property_iri: property.clone(),
                            predicate: SH_TARGET_OBJECTS_OF,
                        });
                    }
                }
                TargetIdentifier::Node(Term::NamedNode(node_iri)) => {
                    if !self.ontology.available_terms.contains(node_iri) {
                        errors.push(AlignmentError::UndeclaredNode {
                            shape_id: id.clone(),
                            node_iri: node_iri.clone(),
                            predicate: SH_TARGET_NODE,
                        });
                    }
                }
                TargetIdentifier::Node(_) | TargetIdentifier::Custom(_) => {}
            }
        }
    }

    fn verify_path(&self, id: &NamedOrBlankNode, path: &AstPath, errors: &mut Vec<AlignmentError>) {
        match path {
            AstPath::Iri(iri) => {
                if !self.ontology.declared_properties.contains(iri) {
                    errors.push(AlignmentError::UndeclaredProperty {
                        shape_id: id.clone(),
                        property_iri: iri.clone(),
                        predicate: SH_PATH,
                    });
                }
            }
            AstPath::Inverse(iri) => {
                if !self.ontology.declared_properties.contains(iri) {
                    errors.push(AlignmentError::InvalidInversePath {
                        shape_id: id.clone(),
                        property_iri: iri.clone(),
                        predicate: SH_INVERSE_PATH,
                    });
                }
            }
            AstPath::Sequence(nested) | AstPath::Alternative(nested) => {
                for item in nested {
                    self.verify_path(id, item, errors);
                }
            }
            AstPath::ZeroOrMore(inner) | AstPath::OneOrMore(inner) | AstPath::ZeroOrOne(inner) => {
                self.verify_path(id, inner, errors)
            }
        }
    }

    fn verify_constraints(
        &self,
        id: &NamedOrBlankNode,
        constraints: &[SyntaxConstraint],
        errors: &mut Vec<AlignmentError>,
    ) {
        for constraint in constraints {
            match constraint {
                SyntaxConstraint::Class { class_node } => {
                    if !self.ontology.declared_classes.contains(class_node) {
                        errors.push(AlignmentError::UndeclaredClass {
                            shape_id: id.clone(),
                            class_node: class_node.clone(),
                            predicate: SH_CLASS,
                        });
                    }
                }
                SyntaxConstraint::Datatype { datatype_iri } => {
                    if !self.ontology.declared_datatypes.contains(datatype_iri)
                        && !owl_reserved::is_supported_datatype_iri(datatype_iri.as_str())
                    {
                        errors.push(AlignmentError::UndeclaredDatatype {
                            shape_id: id.clone(),
                            datatype_iri: datatype_iri.clone(),
                            predicate: SH_DATATYPE,
                        });
                    }
                }
                SyntaxConstraint::Equals { property_iri } => {
                    if !self.ontology.declared_properties.contains(property_iri) {
                        errors.push(AlignmentError::UndeclaredProperty {
                            shape_id: id.clone(),
                            property_iri: property_iri.clone(),
                            predicate: SH_EQUALS,
                        });
                    }
                }
                SyntaxConstraint::Disjoint { property_iri } => {
                    if !self.ontology.declared_properties.contains(property_iri) {
                        errors.push(AlignmentError::UndeclaredProperty {
                            shape_id: id.clone(),
                            property_iri: property_iri.clone(),
                            predicate: SH_DISJOINT,
                        });
                    }
                }
                SyntaxConstraint::LessThan { property_iri } => {
                    if !self.ontology.declared_properties.contains(property_iri) {
                        errors.push(AlignmentError::UndeclaredProperty {
                            shape_id: id.clone(),
                            property_iri: property_iri.clone(),
                            predicate: SH_LESS_THAN,
                        });
                    }
                }
                SyntaxConstraint::LessThanOrEquals { property_iri } => {
                    if !self.ontology.declared_properties.contains(property_iri) {
                        errors.push(AlignmentError::UndeclaredProperty {
                            shape_id: id.clone(),
                            property_iri: property_iri.clone(),
                            predicate: SH_LESS_THAN_OR_EQUALS,
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ShaclRegistry {
    pub compiled_shapes: HashMap<NamedOrBlankNode, Shape>,
    pub diagnostics: Vec<ShaclParseIssue>,
}

impl ShaclRegistry {
    pub fn parse(quads: &[Quad]) -> Self {
        let mut parser = ShaclParser::new(quads);
        parser.parse()
    }

    pub fn diagnostics_as_messages(&self) -> Vec<String> {
        self.diagnostics
            .iter()
            .map(ShaclParseIssue::message)
            .collect()
    }

    pub fn diagnostics_as_structs(&self) -> Vec<Diagnostic> {
        self.diagnostics
            .iter()
            .map(ShaclParseIssue::diagnostic)
            .collect()
    }

    pub fn referenced_iris(&self) -> Vec<ReferencedIri> {
        let mut references = HashSet::new();
        for shape in self.compiled_shapes.values() {
            match shape {
                Shape::Node(node) => {
                    collect_target_references(&node.targets, &mut references);
                    collect_constraint_references(&node.constraints, &mut references);
                }
                Shape::Property(property) => {
                    collect_target_references(&property.targets, &mut references);
                    if let Some(path) = &property.path {
                        collect_path_references(path, &mut references);
                    }
                    collect_constraint_references(&property.constraints, &mut references);
                }
            }
        }
        references.into_iter().collect()
    }
}

struct ShaclParser<'a> {
    quads: &'a [Quad],
    diagnostics: Vec<ShaclParseIssue>,
}

impl<'a> ShaclParser<'a> {
    fn new(quads: &'a [Quad]) -> Self {
        Self {
            quads,
            diagnostics: Vec::new(),
        }
    }

    fn parse(&mut self) -> ShaclRegistry {
        let mut candidates = self.shape_candidates();
        if candidates.is_empty() {
            self.diagnostics.push(ShaclParseIssue::MissingShapeNodes);
            return ShaclRegistry {
                compiled_shapes: HashMap::new(),
                diagnostics: self.diagnostics.clone(),
            };
        }

        let mut compiled_shapes = HashMap::new();
        for id in candidates.drain() {
            let shape = if self.is_property_shape(&id) {
                Shape::Property(self.parse_property_shape(id.clone()))
            } else {
                Shape::Node(self.parse_node_shape(id.clone()))
            };
            compiled_shapes.insert(id, shape);
        }

        ShaclRegistry {
            compiled_shapes,
            diagnostics: self.diagnostics.clone(),
        }
    }

    fn shape_candidates(&self) -> HashSet<NamedOrBlankNode> {
        let mut candidates = HashSet::new();
        for quad in self.quads {
            if quad.predicate.as_str() == RDF_TYPE {
                if matches!(
                    term_iri(&quad.object),
                    Some(SH_NODE_SHAPE | SH_PROPERTY_SHAPE | SH_SHAPE)
                ) {
                    candidates.insert(quad.subject.clone());
                }
            }
            if matches!(
                quad.predicate.as_str(),
                SH_TARGET_CLASS
                    | SH_TARGET_NODE
                    | SH_TARGET_SUBJECTS_OF
                    | SH_TARGET_OBJECTS_OF
                    | SH_TARGET
                    | SH_PATH
            ) {
                candidates.insert(quad.subject.clone());
            }
            if quad.predicate.as_str() == SH_PROPERTY {
                if let Some(property_shape) = term_as_node(&quad.object) {
                    candidates.insert(property_shape);
                }
            }
        }
        candidates
    }

    fn is_property_shape(&self, id: &NamedOrBlankNode) -> bool {
        has_type(self.quads, id, SH_PROPERTY_SHAPE)
            || !objects_for(self.quads, id, SH_PATH).is_empty()
            || self.quads.iter().any(|quad| {
                quad.predicate.as_str() == SH_PROPERTY
                    && term_as_node(&quad.object).as_ref() == Some(id)
            })
    }

    fn parse_node_shape(&mut self, id: NamedOrBlankNode) -> NodeShape {
        let property_shapes = self.shape_refs(&id, SH_PROPERTY);
        let constraints = self.constraints(&id);
        let raw_constraints = self.raw_constraints(&id);
        self.validate_cardinality_bounds(&id, &constraints);
        NodeShape {
            id: id.clone(),
            targets: self.targets(&id),
            constraints,
            raw_constraints,
            property_shapes,
            is_closed: boolean_object(self.quads, &id, SH_CLOSED).unwrap_or(false),
            ignored_properties: self.ignored_properties(&id),
        }
    }

    fn parse_property_shape(&mut self, id: NamedOrBlankNode) -> PropertyShape {
        let path_terms = objects_for(self.quads, &id, SH_PATH);
        let path = match path_terms.as_slice() {
            [] => {
                self.diagnostics.push(ShaclParseIssue::InvalidPath {
                    shape_id: id.clone(),
                    message: "property shape must define exactly one sh:path".to_string(),
                });
                None
            }
            [term] => match self.parse_path(term) {
                Ok(path) => Some(path),
                Err(message) => {
                    self.diagnostics.push(ShaclParseIssue::InvalidPath {
                        shape_id: id.clone(),
                        message,
                    });
                    None
                }
            },
            _ => {
                self.diagnostics.push(ShaclParseIssue::InvalidPath {
                    shape_id: id.clone(),
                    message: "property shape must define exactly one sh:path".to_string(),
                });
                None
            }
        };
        let constraints = self.constraints(&id);
        let raw_constraints = self.raw_constraints(&id);
        self.validate_cardinality_bounds(&id, &constraints);

        PropertyShape {
            id: id.clone(),
            path,
            targets: self.targets(&id),
            constraints,
            raw_constraints,
            property_shapes: self.shape_refs(&id, SH_PROPERTY),
        }
    }

    fn targets(&mut self, id: &NamedOrBlankNode) -> Vec<TargetIdentifier> {
        let mut targets = Vec::new();
        for term in objects_for(self.quads, id, SH_TARGET_CLASS) {
            match term_as_node(term) {
                Some(target) => targets.push(TargetIdentifier::Class(target)),
                None => self.invalid_constraint(id, "sh:targetClass must be an IRI or blank node"),
            }
        }
        for term in objects_for(self.quads, id, SH_TARGET_NODE) {
            targets.push(TargetIdentifier::Node(term.clone()));
        }
        for term in objects_for(self.quads, id, SH_TARGET_SUBJECTS_OF) {
            match term {
                Term::NamedNode(target) => {
                    targets.push(TargetIdentifier::SubjectsOf(target.clone()))
                }
                _ => self.invalid_constraint(id, "sh:targetSubjectsOf must be an IRI"),
            }
        }
        for term in objects_for(self.quads, id, SH_TARGET_OBJECTS_OF) {
            match term {
                Term::NamedNode(target) => {
                    targets.push(TargetIdentifier::ObjectsOf(target.clone()))
                }
                _ => self.invalid_constraint(id, "sh:targetObjectsOf must be an IRI"),
            }
        }
        for term in objects_for(self.quads, id, SH_TARGET) {
            match term_as_node(term) {
                Some(target) => targets.push(TargetIdentifier::Custom(target)),
                None => self.invalid_constraint(id, "sh:target must be an IRI or blank node"),
            }
        }
        targets
    }

    fn constraints(&mut self, id: &NamedOrBlankNode) -> Vec<SyntaxConstraint> {
        let mut constraints = Vec::new();
        for term in objects_for(self.quads, id, SH_CLASS) {
            match term_as_node(term) {
                Some(class_node) => constraints.push(SyntaxConstraint::Class { class_node }),
                None => self.invalid_constraint(id, "sh:class must be an IRI or blank node"),
            }
        }
        for term in objects_for(self.quads, id, SH_DATATYPE) {
            match term {
                Term::NamedNode(datatype_iri) => constraints.push(SyntaxConstraint::Datatype {
                    datatype_iri: datatype_iri.clone(),
                }),
                _ => self.invalid_constraint(id, "sh:datatype must be an IRI"),
            }
        }
        for term in objects_for(self.quads, id, SH_NODE_KIND) {
            match term_iri(term).and_then(node_kind_variant) {
                Some(kind) => constraints.push(SyntaxConstraint::NodeKind { kind }),
                None => self
                    .invalid_constraint(id, "sh:nodeKind must be a supported SHACL node kind IRI"),
            }
        }
        self.push_u32(
            id,
            &mut constraints,
            SH_MIN_COUNT,
            SyntaxConstraint::MinCount,
        );
        self.push_u32(
            id,
            &mut constraints,
            SH_MAX_COUNT,
            SyntaxConstraint::MaxCount,
        );
        self.push_u32(
            id,
            &mut constraints,
            SH_MIN_LENGTH,
            SyntaxConstraint::MinLength,
        );
        self.push_u32(
            id,
            &mut constraints,
            SH_MAX_LENGTH,
            SyntaxConstraint::MaxLength,
        );
        self.push_term(
            id,
            &mut constraints,
            SH_MIN_EXCLUSIVE,
            SyntaxConstraint::MinExclusive,
        );
        self.push_term(
            id,
            &mut constraints,
            SH_MIN_INCLUSIVE,
            SyntaxConstraint::MinInclusive,
        );
        self.push_term(
            id,
            &mut constraints,
            SH_MAX_EXCLUSIVE,
            SyntaxConstraint::MaxExclusive,
        );
        self.push_term(
            id,
            &mut constraints,
            SH_MAX_INCLUSIVE,
            SyntaxConstraint::MaxInclusive,
        );
        for term in objects_for(self.quads, id, SH_PATTERN) {
            match literal_string(term) {
                Some(expression) => constraints.push(SyntaxConstraint::Pattern {
                    expression,
                    flags: literal_object(self.quads, id, SH_FLAGS),
                }),
                None => self.invalid_constraint(id, "sh:pattern must be a literal"),
            }
        }
        for term in objects_for(self.quads, id, SH_LANGUAGE_IN) {
            match rdf_list_terms(self.quads, term) {
                Ok(terms) => {
                    let mut languages = Vec::new();
                    for item in terms {
                        match literal_string(&item) {
                            Some(language) => languages.push(language),
                            None => self.invalid_constraint(
                                id,
                                "sh:languageIn list values must be literals",
                            ),
                        }
                    }
                    constraints.push(SyntaxConstraint::LanguageIn(languages));
                }
                Err(message) => self.invalid_constraint(id, &message),
            }
        }
        for value in boolean_objects(self.quads, id, SH_UNIQUE_LANG) {
            constraints.push(SyntaxConstraint::UniqueLang(value));
        }
        self.push_property_iri(id, &mut constraints, SH_EQUALS, |property_iri| {
            SyntaxConstraint::Equals { property_iri }
        });
        self.push_property_iri(id, &mut constraints, SH_DISJOINT, |property_iri| {
            SyntaxConstraint::Disjoint { property_iri }
        });
        self.push_property_iri(id, &mut constraints, SH_LESS_THAN, |property_iri| {
            SyntaxConstraint::LessThan { property_iri }
        });
        self.push_property_iri(
            id,
            &mut constraints,
            SH_LESS_THAN_OR_EQUALS,
            |property_iri| SyntaxConstraint::LessThanOrEquals { property_iri },
        );
        self.push_shape_list(id, &mut constraints, SH_AND, SyntaxConstraint::And);
        self.push_shape_list(id, &mut constraints, SH_OR, SyntaxConstraint::Or);
        self.push_shape_list(id, &mut constraints, SH_XONE, SyntaxConstraint::Xone);
        for shape in self.shape_refs(id, SH_NOT) {
            constraints.push(SyntaxConstraint::Not(shape));
        }
        for shape in self.shape_refs(id, SH_NODE) {
            constraints.push(SyntaxConstraint::Node(shape));
        }
        for shape_id in self.shape_refs(id, SH_QUALIFIED_VALUE_SHAPE) {
            constraints.push(SyntaxConstraint::QualifiedValueShape {
                shape_id,
                min_count: first_u32_object(self.quads, id, SH_QUALIFIED_MIN_COUNT),
                max_count: first_u32_object(self.quads, id, SH_QUALIFIED_MAX_COUNT),
            });
        }
        for term in objects_for(self.quads, id, SH_HAS_VALUE) {
            constraints.push(SyntaxConstraint::HasValue(term.clone()));
        }
        for term in objects_for(self.quads, id, SH_IN) {
            match rdf_list_terms(self.quads, term) {
                Ok(values) => constraints.push(SyntaxConstraint::In(values)),
                Err(message) => self.invalid_constraint(id, &format!("sh:in {message}")),
            }
        }
        for sparql_node in self.shape_refs(id, SH_SPARQL) {
            for term in objects_for(self.quads, &sparql_node, SH_SELECT) {
                match literal_string(term) {
                    Some(query) => constraints.push(SyntaxConstraint::Sparql { query }),
                    None => self.invalid_constraint(id, "sh:sparql sh:select must be a literal"),
                }
            }
        }
        constraints
    }

    fn raw_constraints(&self, id: &NamedOrBlankNode) -> Vec<AstConstraint> {
        self.quads
            .iter()
            .filter(|quad| {
                &quad.subject == id && is_raw_constraint_predicate(quad.predicate.as_str())
            })
            .map(|quad| AstConstraint {
                predicate: quad.predicate.clone(),
                value: quad.object.clone(),
            })
            .collect()
    }

    fn parse_path(&mut self, term: &Term) -> Result<AstPath, String> {
        match term {
            Term::NamedNode(iri) => Ok(AstPath::Iri(iri.clone())),
            Term::BlankNode(_) => {
                let node = term_as_node(term).expect("blank term is node");
                if let Some(inverse) = first_named_object(self.quads, &node, SH_INVERSE_PATH) {
                    return Ok(AstPath::Inverse(inverse));
                }
                if let Some(sequence) = rdf_list_terms(self.quads, term).ok() {
                    let mut elements = Vec::new();
                    for item in sequence {
                        elements.push(self.parse_path(&item)?);
                    }
                    return Ok(AstPath::Sequence(elements));
                }
                if let Some(alternative) =
                    objects_for(self.quads, &node, SH_ALTERNATIVE_PATH).first()
                {
                    let alternatives = rdf_list_terms(self.quads, alternative)?
                        .into_iter()
                        .map(|item| self.parse_path(&item))
                        .collect::<Result<Vec<_>, _>>()?;
                    return Ok(AstPath::Alternative(alternatives));
                }
                for (predicate, builder) in [
                    (
                        SH_ZERO_OR_MORE_PATH,
                        AstPath::ZeroOrMore as fn(Box<AstPath>) -> AstPath,
                    ),
                    (SH_ONE_OR_MORE_PATH, AstPath::OneOrMore),
                    (SH_ZERO_OR_ONE_PATH, AstPath::ZeroOrOne),
                ] {
                    if let Some(inner) = objects_for(self.quads, &node, predicate).first() {
                        let parsed = self.parse_path(inner)?;
                        if matches!(parsed, AstPath::Sequence(_)) {
                            return Err("SHACL path repetition operators must wrap exactly one path element".to_string());
                        }
                        return Ok(builder(Box::new(parsed)));
                    }
                }
                Err(
                    "sh:path blank node does not define a supported SHACL property path"
                        .to_string(),
                )
            }
            Term::Literal(_) => Err("sh:path must be an IRI or SHACL path node".to_string()),
        }
    }

    fn shape_refs(
        &mut self,
        id: &NamedOrBlankNode,
        predicate: &'static str,
    ) -> Vec<NamedOrBlankNode> {
        let mut refs = Vec::new();
        for term in objects_for(self.quads, id, predicate) {
            match term_as_node(term) {
                Some(shape_id) => refs.push(shape_id),
                None => self
                    .diagnostics
                    .push(ShaclParseIssue::InvalidShapeReference {
                        shape_id: id.clone(),
                        predicate,
                    }),
            }
        }
        refs
    }

    fn ignored_properties(&mut self, id: &NamedOrBlankNode) -> Vec<NamedNode> {
        let mut ignored = Vec::new();
        for term in objects_for(self.quads, id, SH_IGNORED_PROPERTIES) {
            match rdf_list_terms(self.quads, term) {
                Ok(terms) => {
                    for item in terms {
                        match item {
                            Term::NamedNode(iri) => ignored.push(iri),
                            _ => self.invalid_constraint(
                                id,
                                "sh:ignoredProperties list values must be IRIs",
                            ),
                        }
                    }
                }
                Err(message) => self.invalid_constraint(id, &message),
            }
        }
        ignored
    }

    fn push_u32(
        &mut self,
        id: &NamedOrBlankNode,
        constraints: &mut Vec<SyntaxConstraint>,
        predicate: &'static str,
        builder: fn(u32) -> SyntaxConstraint,
    ) {
        for term in objects_for(self.quads, id, predicate) {
            match parse_u32(term) {
                Some(value) => constraints.push(builder(value)),
                None => self.invalid_constraint(
                    id,
                    &format!(
                        "{} must be a non-negative integer literal",
                        predicate_label(predicate)
                    ),
                ),
            }
        }
    }

    fn push_term(
        &mut self,
        id: &NamedOrBlankNode,
        constraints: &mut Vec<SyntaxConstraint>,
        predicate: &'static str,
        builder: fn(Term) -> SyntaxConstraint,
    ) {
        for term in objects_for(self.quads, id, predicate) {
            constraints.push(builder(term.clone()));
        }
    }

    fn push_property_iri(
        &mut self,
        id: &NamedOrBlankNode,
        constraints: &mut Vec<SyntaxConstraint>,
        predicate: &'static str,
        builder: impl Fn(NamedNode) -> SyntaxConstraint,
    ) {
        for term in objects_for(self.quads, id, predicate) {
            match term {
                Term::NamedNode(iri) => constraints.push(builder(iri.clone())),
                _ => self.invalid_constraint(
                    id,
                    &format!("{} must be an IRI", predicate_label(predicate)),
                ),
            }
        }
    }

    fn push_shape_list(
        &mut self,
        id: &NamedOrBlankNode,
        constraints: &mut Vec<SyntaxConstraint>,
        predicate: &'static str,
        builder: fn(Vec<NamedOrBlankNode>) -> SyntaxConstraint,
    ) {
        for term in objects_for(self.quads, id, predicate) {
            match rdf_list_terms(self.quads, term) {
                Ok(terms) => {
                    let mut shapes = Vec::new();
                    for item in terms {
                        match term_as_node(&item) {
                            Some(shape) => shapes.push(shape),
                            None => self.invalid_constraint(
                                id,
                                &format!(
                                    "{} list values must be shape nodes",
                                    predicate_label(predicate)
                                ),
                            ),
                        }
                    }
                    constraints.push(builder(shapes));
                }
                Err(message) => self.invalid_constraint(id, &message),
            }
        }
    }

    fn invalid_constraint(&mut self, id: &NamedOrBlankNode, message: &str) {
        self.diagnostics.push(ShaclParseIssue::InvalidConstraint {
            shape_id: id.clone(),
            message: message.to_string(),
        });
    }

    fn validate_cardinality_bounds(
        &mut self,
        id: &NamedOrBlankNode,
        constraints: &[SyntaxConstraint],
    ) {
        let min = constraints.iter().find_map(|constraint| match constraint {
            SyntaxConstraint::MinCount(value) => Some(*value),
            _ => None,
        });
        let max = constraints.iter().find_map(|constraint| match constraint {
            SyntaxConstraint::MaxCount(value) => Some(*value),
            _ => None,
        });
        if let (Some(min), Some(max)) = (min, max) {
            if max < min {
                self.invalid_constraint(
                    id,
                    "sh:maxCount must be greater than or equal to sh:minCount",
                );
            }
        }
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

fn has_type(graph: &[Quad], subject: &NamedOrBlankNode, type_iri: &str) -> bool {
    graph.iter().any(|quad| {
        &quad.subject == subject
            && quad.predicate.as_str() == RDF_TYPE
            && term_iri(&quad.object) == Some(type_iri)
    })
}

fn first_named_object(
    graph: &[Quad],
    subject: &NamedOrBlankNode,
    predicate: &str,
) -> Option<NamedNode> {
    objects_for(graph, subject, predicate)
        .into_iter()
        .find_map(|term| match term {
            Term::NamedNode(node) => Some(node.clone()),
            _ => None,
        })
}

fn first_u32_object(graph: &[Quad], subject: &NamedOrBlankNode, predicate: &str) -> Option<u32> {
    objects_for(graph, subject, predicate)
        .into_iter()
        .find_map(parse_u32)
}

fn parse_u32(term: &Term) -> Option<u32> {
    let Term::Literal(literal) = term else {
        return None;
    };
    let value = literal.value().parse::<i64>().ok()?;
    if value < 0 {
        return None;
    }
    u32::try_from(value).ok()
}

fn literal_string(term: &Term) -> Option<String> {
    match term {
        Term::Literal(literal) => Some(literal.value().to_string()),
        _ => None,
    }
}

fn literal_object(graph: &[Quad], subject: &NamedOrBlankNode, predicate: &str) -> Option<String> {
    objects_for(graph, subject, predicate)
        .into_iter()
        .find_map(literal_string)
}

fn boolean_objects(graph: &[Quad], subject: &NamedOrBlankNode, predicate: &str) -> Vec<bool> {
    objects_for(graph, subject, predicate)
        .into_iter()
        .filter_map(boolean_term)
        .collect()
}

fn boolean_object(graph: &[Quad], subject: &NamedOrBlankNode, predicate: &str) -> Option<bool> {
    boolean_objects(graph, subject, predicate)
        .into_iter()
        .next()
}

fn boolean_term(term: &Term) -> Option<bool> {
    let Term::Literal(literal) = term else {
        return None;
    };
    match literal.value() {
        "true" | "1" => Some(true),
        "false" | "0" => Some(false),
        _ => None,
    }
}

fn rdf_list_terms(graph: &[Quad], head: &Term) -> Result<Vec<Term>, String> {
    let mut terms = Vec::new();
    let mut seen = HashSet::new();
    let mut cursor = head.clone();
    loop {
        if term_iri(&cursor) == Some(RDF_NIL) {
            return Ok(terms);
        }
        let Some(node) = term_as_node(&cursor) else {
            return Err("RDF list must point to a list node or rdf:nil".to_string());
        };
        if matches!(node, NamedOrBlankNode::NamedNode(_)) {
            return Err("RDF list nodes must be blank nodes or rdf:nil".to_string());
        }
        if !seen.insert(node.to_string()) {
            return Err("RDF list contains a cycle".to_string());
        }
        let first = objects_for(graph, &node, RDF_FIRST);
        let rest = objects_for(graph, &node, RDF_REST);
        if first.len() != 1 || rest.len() != 1 {
            return Err("RDF list nodes must have exactly one rdf:first and rdf:rest".to_string());
        }
        terms.push(first[0].clone());
        cursor = rest[0].clone();
    }
}

fn node_kind_variant(iri: &str) -> Option<NodeKindVariant> {
    SH_NODE_KINDS
        .iter()
        .find_map(|(candidate, kind)| (*candidate == iri).then_some(*kind))
}

fn collect_target_references(
    targets: &[TargetIdentifier],
    references: &mut HashSet<ReferencedIri>,
) {
    for target in targets {
        match target {
            TargetIdentifier::Class(NamedOrBlankNode::NamedNode(iri)) => {
                references.insert(ReferencedIri {
                    iri: iri.clone(),
                    predicate: SH_TARGET_CLASS,
                });
            }
            TargetIdentifier::SubjectsOf(iri) => {
                references.insert(ReferencedIri {
                    iri: iri.clone(),
                    predicate: SH_TARGET_SUBJECTS_OF,
                });
            }
            TargetIdentifier::ObjectsOf(iri) => {
                references.insert(ReferencedIri {
                    iri: iri.clone(),
                    predicate: SH_TARGET_OBJECTS_OF,
                });
            }
            TargetIdentifier::Node(Term::NamedNode(iri)) => {
                references.insert(ReferencedIri {
                    iri: iri.clone(),
                    predicate: SH_TARGET_NODE,
                });
            }
            TargetIdentifier::Node(_)
            | TargetIdentifier::Custom(_)
            | TargetIdentifier::Class(NamedOrBlankNode::BlankNode(_))
            | TargetIdentifier::ImplicitClass(_) => {}
        }
    }
}

fn collect_path_references(path: &AstPath, references: &mut HashSet<ReferencedIri>) {
    match path {
        AstPath::Iri(iri) => {
            references.insert(ReferencedIri {
                iri: iri.clone(),
                predicate: SH_PATH,
            });
        }
        AstPath::Inverse(iri) => {
            references.insert(ReferencedIri {
                iri: iri.clone(),
                predicate: SH_INVERSE_PATH,
            });
        }
        AstPath::Sequence(nested) | AstPath::Alternative(nested) => {
            for item in nested {
                collect_path_references(item, references);
            }
        }
        AstPath::ZeroOrMore(inner) | AstPath::OneOrMore(inner) | AstPath::ZeroOrOne(inner) => {
            collect_path_references(inner, references)
        }
    }
}

fn collect_constraint_references(
    constraints: &[SyntaxConstraint],
    references: &mut HashSet<ReferencedIri>,
) {
    for constraint in constraints {
        match constraint {
            SyntaxConstraint::Class {
                class_node: NamedOrBlankNode::NamedNode(iri),
            } => {
                references.insert(ReferencedIri {
                    iri: iri.clone(),
                    predicate: SH_CLASS,
                });
            }
            SyntaxConstraint::Datatype { datatype_iri } => {
                references.insert(ReferencedIri {
                    iri: datatype_iri.clone(),
                    predicate: SH_DATATYPE,
                });
            }
            SyntaxConstraint::Equals { property_iri } => {
                references.insert(ReferencedIri {
                    iri: property_iri.clone(),
                    predicate: SH_EQUALS,
                });
            }
            SyntaxConstraint::Disjoint { property_iri } => {
                references.insert(ReferencedIri {
                    iri: property_iri.clone(),
                    predicate: SH_DISJOINT,
                });
            }
            SyntaxConstraint::LessThan { property_iri } => {
                references.insert(ReferencedIri {
                    iri: property_iri.clone(),
                    predicate: SH_LESS_THAN,
                });
            }
            SyntaxConstraint::LessThanOrEquals { property_iri } => {
                references.insert(ReferencedIri {
                    iri: property_iri.clone(),
                    predicate: SH_LESS_THAN_OR_EQUALS,
                });
            }
            _ => {}
        }
    }
}

fn is_raw_constraint_predicate(predicate: &str) -> bool {
    predicate.starts_with("http://www.w3.org/ns/shacl#")
        && !matches!(
            predicate,
            SH_TARGET_CLASS
                | SH_TARGET_NODE
                | SH_TARGET_SUBJECTS_OF
                | SH_TARGET_OBJECTS_OF
                | SH_TARGET
                | SH_PATH
                | SH_PROPERTY
        )
}

pub fn predicate_label(iri: &str) -> String {
    iri.strip_prefix("http://www.w3.org/ns/shacl#")
        .map(|local| format!("sh:{local}"))
        .unwrap_or_else(|| iri.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxigraph::io::{RdfFormat, RdfParser};

    const EX: &str = "https://example.org/model#";
    const XSD_STRING: &str = "http://www.w3.org/2001/XMLSchema#string";

    fn parse_turtle(turtle: &str) -> Vec<Quad> {
        RdfParser::from_format(RdfFormat::Turtle)
            .for_reader(turtle.as_bytes())
            .map(|quad| quad.expect("test Turtle should parse"))
            .collect()
    }

    fn iri(local: &str) -> NamedNode {
        NamedNode::new(format!("{EX}{local}")).expect("test IRI should be valid")
    }

    fn node(local: &str) -> NamedOrBlankNode {
        NamedOrBlankNode::NamedNode(iri(local))
    }

    fn contains_class_target(targets: &[TargetIdentifier], local: &str) -> bool {
        targets
            .iter()
            .any(|target| matches!(target, TargetIdentifier::Class(value) if value == &node(local)))
    }

    fn contains_subjects_of_target(targets: &[TargetIdentifier], local: &str) -> bool {
        targets.iter().any(
            |target| matches!(target, TargetIdentifier::SubjectsOf(value) if value == &iri(local)),
        )
    }

    fn contains_objects_of_target(targets: &[TargetIdentifier], local: &str) -> bool {
        targets.iter().any(
            |target| matches!(target, TargetIdentifier::ObjectsOf(value) if value == &iri(local)),
        )
    }

    #[test]
    fn shacl_parser_discovers_targets_nested_properties_and_constraints() {
        let quads = parse_turtle(
            r#"
@prefix ex: <https://example.org/model#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:InvoiceShape
  a sh:NodeShape ;
  sh:targetClass ex:Invoice ;
  sh:targetSubjectsOf ex:status ;
  sh:targetObjectsOf ex:owner ;
  sh:closed true ;
  sh:ignoredProperties ( ex:ignored ) ;
  sh:property ex:InvoiceNumberShape .

ex:InvoiceNumberShape
  a sh:PropertyShape ;
  sh:path ex:number ;
  sh:datatype xsd:string ;
  sh:minCount 1 ;
  sh:maxCount 1 ;
  sh:pattern "^[A-Z]+$" ;
  sh:flags "i" ;
  sh:nodeKind sh:Literal ;
  sh:in ( "A" "B" ) ;
  sh:sparql [ sh:select "SELECT $this WHERE { $this ex:number ?value }" ] .
"#,
        );

        let registry = ShaclRegistry::parse(&quads);
        assert_eq!(registry.diagnostics_as_messages(), Vec::<String>::new());
        assert_eq!(registry.compiled_shapes.len(), 2);

        let Shape::Node(invoice_shape) = registry
            .compiled_shapes
            .get(&node("InvoiceShape"))
            .expect("invoice shape should be compiled")
        else {
            panic!("invoice shape should be a node shape");
        };
        assert!(contains_class_target(&invoice_shape.targets, "Invoice"));
        assert!(contains_subjects_of_target(
            &invoice_shape.targets,
            "status"
        ));
        assert!(contains_objects_of_target(&invoice_shape.targets, "owner"));
        assert!(invoice_shape.is_closed);
        assert_eq!(invoice_shape.ignored_properties, vec![iri("ignored")]);
        assert_eq!(
            invoice_shape.property_shapes,
            vec![node("InvoiceNumberShape")]
        );

        let Shape::Property(number_shape) = registry
            .compiled_shapes
            .get(&node("InvoiceNumberShape"))
            .expect("number shape should be compiled")
        else {
            panic!("number shape should be a property shape");
        };
        assert_eq!(number_shape.path, Some(AstPath::Iri(iri("number"))));
        assert!(number_shape
            .raw_constraints
            .iter()
            .any(|constraint| constraint.predicate.as_str() == SH_DATATYPE
                && constraint.value == Term::NamedNode(NamedNode::new(XSD_STRING).unwrap())));
        assert!(number_shape
            .raw_constraints
            .iter()
            .any(|constraint| constraint.predicate.as_str() == SH_MIN_COUNT));
        assert!(number_shape
            .constraints
            .contains(&SyntaxConstraint::Datatype {
                datatype_iri: NamedNode::new(XSD_STRING).unwrap(),
            }));
        assert!(number_shape
            .constraints
            .contains(&SyntaxConstraint::NodeKind {
                kind: NodeKindVariant::Literal,
            }));
        assert!(number_shape
            .constraints
            .contains(&SyntaxConstraint::MinCount(1)));
        assert!(number_shape
            .constraints
            .contains(&SyntaxConstraint::MaxCount(1)));
        assert!(number_shape.constraints.iter().any(
            |constraint| matches!(constraint, SyntaxConstraint::Pattern { expression, flags }
                if expression == "^[A-Z]+$" && flags.as_deref() == Some("i"))
        ));
        assert!(number_shape.constraints.iter().any(
            |constraint| matches!(constraint, SyntaxConstraint::In(values) if values.len() == 2)
        ));
        assert!(number_shape
            .constraints
            .iter()
            .any(|constraint| matches!(constraint, SyntaxConstraint::Sparql { query } if query.contains("SELECT"))));
    }

    #[test]
    fn shacl_parser_supports_target_only_shapes_and_complex_paths() {
        let quads = parse_turtle(
            r#"
@prefix ex: <https://example.org/model#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

ex:StatusShape
  sh:targetSubjectsOf ex:status .

ex:ComplexPropertyShape
  a sh:PropertyShape ;
  sh:path (
    ex:first
    [ sh:inversePath ex:parent ]
    [ sh:alternativePath ( ex:a ex:b ) ]
    [ sh:zeroOrMorePath ex:tag ]
    [ sh:oneOrMorePath ex:item ]
    [ sh:zeroOrOnePath ex:optional ]
  ) .
"#,
        );

        let registry = ShaclRegistry::parse(&quads);
        assert_eq!(registry.diagnostics_as_messages(), Vec::<String>::new());

        let Shape::Node(status_shape) = registry
            .compiled_shapes
            .get(&node("StatusShape"))
            .expect("target-only shape should be compiled")
        else {
            panic!("target-only shape should be a node shape");
        };
        assert!(contains_subjects_of_target(&status_shape.targets, "status"));

        let Shape::Property(complex_shape) = registry
            .compiled_shapes
            .get(&node("ComplexPropertyShape"))
            .expect("complex property shape should be compiled")
        else {
            panic!("complex path should be a property shape");
        };
        let Some(AstPath::Sequence(path)) = &complex_shape.path else {
            panic!("complex path should preserve the sequence node");
        };
        assert_eq!(path.len(), 6);
        assert_eq!(path[0], AstPath::Iri(iri("first")));
        assert_eq!(path[1], AstPath::Inverse(iri("parent")));
        assert!(matches!(&path[2], AstPath::Alternative(options)
            if options == &vec![AstPath::Iri(iri("a")), AstPath::Iri(iri("b"))]));
        assert!(matches!(&path[3], AstPath::ZeroOrMore(inner)
            if inner.as_ref() == &AstPath::Iri(iri("tag"))));
        assert!(matches!(&path[4], AstPath::OneOrMore(inner)
            if inner.as_ref() == &AstPath::Iri(iri("item"))));
        assert!(matches!(&path[5], AstPath::ZeroOrOne(inner)
            if inner.as_ref() == &AstPath::Iri(iri("optional"))));
    }

    #[test]
    fn shacl_parser_reports_deterministic_structure_diagnostics() {
        let quads = parse_turtle(
            r#"
@prefix ex: <https://example.org/model#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

ex:BadShape
  a sh:NodeShape ;
  sh:in ex:notAList ;
  sh:property ex:MissingPathShape .

ex:MissingPathShape
  a sh:PropertyShape .
"#,
        );

        let messages = ShaclRegistry::parse(&quads).diagnostics_as_messages();
        assert!(
            messages
                .iter()
                .any(|message| message
                    .contains("sh:in RDF list nodes must be blank nodes or rdf:nil"))
        );
        assert!(messages
            .iter()
            .any(|message| message.contains("property shape must define exactly one sh:path")));
    }

    #[test]
    fn shacl_aligner_accepts_supplied_ontology_and_builtin_datatypes() {
        let quads = parse_turtle(
            r#"
@prefix ex: <https://example.org/model#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:InvoiceShape
  a sh:NodeShape ;
  sh:targetClass ex:Invoice ;
  sh:targetNode ex:InvoiceExample ;
  sh:property ex:NumberShape .

ex:NumberShape
  a sh:PropertyShape ;
  sh:path ex:number ;
  sh:class ex:InvoiceNumber ;
  sh:datatype xsd:string ;
  sh:equals ex:canonicalNumber .
"#,
        );

        let registry = ShaclRegistry::parse(&quads);
        assert_eq!(registry.diagnostics_as_messages(), Vec::<String>::new());

        let mut domain = DomainOntologyIndex::default();
        domain.declared_classes.insert(node("Invoice"));
        domain.declared_classes.insert(node("InvoiceNumber"));
        domain.available_terms.insert(iri("InvoiceExample"));
        domain.declared_properties.insert(iri("number"));
        domain.declared_properties.insert(iri("canonicalNumber"));

        let errors = OntologyAligner::new(&domain).cross_check_shapes(&registry.compiled_shapes);
        assert_eq!(errors, Vec::<AlignmentError>::new());
    }

    #[test]
    fn shacl_aligner_accepts_annotation_property_paths() {
        let ontology_quads = parse_turtle(
            r#"
@prefix ex: <https://example.org/model#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:layer a owl:AnnotationProperty .
"#,
        );
        let shacl_quads = parse_turtle(
            r#"
@prefix ex: <https://example.org/model#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

ex:LayerShape
  a sh:PropertyShape ;
  sh:path ex:layer .
"#,
        );

        let registry = ShaclRegistry::parse(&shacl_quads);
        assert_eq!(registry.diagnostics_as_messages(), Vec::<String>::new());

        let domain = DomainOntologyIndex::from_quads(&ontology_quads);
        let errors = OntologyAligner::new(&domain).cross_check_shapes(&registry.compiled_shapes);
        assert_eq!(errors, Vec::<AlignmentError>::new());
    }

    #[test]
    fn shacl_aligner_reports_predicate_specific_reference_errors() {
        let quads = parse_turtle(
            r#"
@prefix ex: <https://example.org/model#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

ex:BadShape
  a sh:NodeShape ;
  sh:targetClass ex:MissingClass ;
  sh:targetNode ex:MissingNode ;
  sh:targetSubjectsOf ex:missingSubjectPredicate ;
  sh:property ex:BadPropertyShape .

ex:BadPropertyShape
  a sh:PropertyShape ;
  sh:path [ sh:inversePath ex:missingInverse ] ;
  sh:datatype ex:MissingDatatype ;
  sh:lessThan ex:missingLessThan .
"#,
        );

        let registry = ShaclRegistry::parse(&quads);
        assert_eq!(registry.diagnostics_as_messages(), Vec::<String>::new());
        let errors = OntologyAligner::new(&DomainOntologyIndex::default())
            .cross_check_shapes(&registry.compiled_shapes);

        assert!(errors.iter().any(|error| matches!(error,
            AlignmentError::UndeclaredClass {
                class_node: NamedOrBlankNode::NamedNode(class_node),
                predicate: SH_TARGET_CLASS,
                ..
            } if class_node == &iri("MissingClass")
        )));
        assert!(errors.iter().any(|error| matches!(error,
            AlignmentError::UndeclaredProperty {
                property_iri,
                predicate: SH_TARGET_SUBJECTS_OF,
                ..
            } if property_iri == &iri("missingSubjectPredicate")
        )));
        assert!(errors.iter().any(|error| matches!(error,
            AlignmentError::UndeclaredNode {
                node_iri,
                predicate: SH_TARGET_NODE,
                ..
            } if node_iri == &iri("MissingNode")
        )));
        assert!(errors.iter().any(|error| matches!(error,
            AlignmentError::InvalidInversePath {
                property_iri,
                predicate: SH_INVERSE_PATH,
                ..
            } if property_iri == &iri("missingInverse")
        )));
        assert!(errors.iter().any(|error| matches!(error,
            AlignmentError::UndeclaredDatatype {
                datatype_iri,
                predicate: SH_DATATYPE,
                ..
            } if datatype_iri == &iri("MissingDatatype")
        )));
        assert!(errors.iter().any(|error| matches!(error,
            AlignmentError::UndeclaredProperty {
                property_iri,
                predicate: SH_LESS_THAN,
                ..
            } if property_iri == &iri("missingLessThan")
        )));
    }
}
