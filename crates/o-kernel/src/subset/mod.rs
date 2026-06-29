//! External ontology dependency subset construction service.

pub mod closure;
pub mod construct;
pub mod reference;
pub mod seed;

use crate::rdf::named_or_blank_node_key;
use crate::vocab::{
    OWL_ALL_VALUES_FROM, OWL_CARDINALITY, OWL_COMPLEMENT_OF, OWL_DISJOINT_WITH,
    OWL_EQUIVALENT_CLASS, OWL_EQUIVALENT_PROPERTY, OWL_HAS_VALUE, OWL_INTERSECTION_OF,
    OWL_INVERSE_OF, OWL_MAX_CARDINALITY, OWL_MAX_QUALIFIED_CARDINALITY, OWL_MIN_CARDINALITY,
    OWL_MIN_QUALIFIED_CARDINALITY, OWL_ON_CLASS, OWL_ON_DATA_RANGE, OWL_ON_PROPERTY,
    OWL_QUALIFIED_CARDINALITY, OWL_SOME_VALUES_FROM, OWL_UNION_OF,
};
use crate::vocab::{
    RDFS_COMMENT, RDFS_DOMAIN, RDFS_LABEL, RDFS_RANGE, RDFS_SUBCLASS_OF, RDF_FIRST, RDF_REST,
    RDF_TYPE, SH_AND, SH_CLASS, SH_CLOSED, SH_DISJOINT, SH_EQUALS, SH_HAS_VALUE,
    SH_IGNORED_PROPERTIES, SH_IN, SH_LESS_THAN, SH_LESS_THAN_OR_EQUALS, SH_NODE, SH_NOT, SH_OR,
    SH_PATH, SH_PATTERN, SH_QUALIFIED_VALUE_SHAPE, SH_TARGET, SH_TARGET_CLASS, SH_TARGET_NODE,
    SH_TARGET_OBJECTS_OF, SH_TARGET_SUBJECTS_OF, SH_XONE,
};
use oxigraph::model::{GraphName, NamedNode, NamedOrBlankNode, Quad, Term};
use oxigraph::store::Store;
use std::collections::BTreeSet;
use thiserror::Error;

pub const MODULE: &str = "subset";

pub const EXTERNAL_SUBSET_SUPPORT_DEPTH: usize = 2;

pub const EXTERNAL_SUBSET_SUPPORT_PREDICATES: &[&str] = &[
    RDF_TYPE,
    RDFS_SUBCLASS_OF,
    RDFS_DOMAIN,
    RDFS_RANGE,
    "http://www.w3.org/2000/01/rdf-schema#subPropertyOf",
    OWL_EQUIVALENT_CLASS,
    OWL_EQUIVALENT_PROPERTY,
    OWL_DISJOINT_WITH,
    OWL_INVERSE_OF,
    OWL_ON_PROPERTY,
    OWL_ALL_VALUES_FROM,
    OWL_SOME_VALUES_FROM,
    OWL_HAS_VALUE,
    OWL_ON_CLASS,
    OWL_ON_DATA_RANGE,
    OWL_MAX_QUALIFIED_CARDINALITY,
    OWL_MIN_QUALIFIED_CARDINALITY,
    OWL_MIN_CARDINALITY,
    OWL_MAX_CARDINALITY,
    OWL_CARDINALITY,
    OWL_QUALIFIED_CARDINALITY,
    OWL_INTERSECTION_OF,
    OWL_UNION_OF,
    OWL_COMPLEMENT_OF,
    SH_AND,
    SH_OR,
    SH_XONE,
    SH_NOT,
    SH_NODE,
    SH_CLASS,
    "http://www.w3.org/ns/shacl#datatype",
    SH_PATH,
    SH_TARGET_CLASS,
    SH_TARGET_NODE,
    SH_TARGET,
    SH_TARGET_SUBJECTS_OF,
    SH_TARGET_OBJECTS_OF,
    SH_QUALIFIED_VALUE_SHAPE,
    SH_IN,
    "http://www.w3.org/ns/shacl#minCount",
    "http://www.w3.org/ns/shacl#maxCount",
    SH_LESS_THAN,
    SH_LESS_THAN_OR_EQUALS,
    SH_EQUALS,
    SH_DISJOINT,
    SH_HAS_VALUE,
    "http://www.w3.org/ns/shacl#in",
    "http://www.w3.org/ns/shacl#property",
    SH_CLOSED,
    SH_IGNORED_PROPERTIES,
    SH_PATTERN,
    "http://www.w3.org/ns/shacl#not",
];

pub const EXTERNAL_SUBSET_ANNOTATION_PREDICATES: &[&str] = &[
    RDFS_LABEL,
    RDFS_COMMENT,
    "http://www.w3.org/2004/02/skos/core#prefLabel",
    "http://www.w3.org/2004/02/skos/core#definition",
    "http://purl.org/dc/terms/description",
];

pub const EXTERNAL_SUBSET_LIST_PREDICATES: &[&str] = &[RDF_FIRST, RDF_REST];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstructedQuadKind {
    Direct,
    Support,
    Annotation,
    ListClosure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructedQuad {
    pub quad: Quad,
    pub kind: ConstructedQuadKind,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExternalSubsetMetadata {
    pub referenced_term_count: usize,
    pub seed_term_count: usize,
    pub support_term_count: usize,
    pub annotation_term_count: usize,
    pub list_term_count: usize,
    pub depth_boundary_term_count: usize,
    pub direct_quad_count: usize,
    pub support_quad_count: usize,
    pub annotation_quad_count: usize,
    pub list_quad_count: usize,
    pub depth_boundary_quad_count: usize,
    pub support_depth: usize,
    pub referenced_dependency_terms: Vec<String>,
    pub seed_dependency_terms: Vec<String>,
    pub support_terms: Vec<String>,
    pub annotation_terms: Vec<String>,
    pub list_terms: Vec<String>,
    pub depth_boundary_terms: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalSubsetResult {
    pub quads: Vec<ConstructedQuad>,
    pub metadata: ExternalSubsetMetadata,
}

#[derive(Debug, Error)]
pub enum SubsetError {
    #[error("Invalid named graph IRI '{value}': {error}")]
    InvalidGraph { value: String, error: String },
    #[error("Store query failed: {error}")]
    StoreFailure { error: String },
}

pub fn build_external_dependency_subset(
    store: &Store,
    ontology_graphs_of_interest: impl IntoIterator<Item = impl AsRef<str>>,
    dependency_graphs: impl IntoIterator<Item = impl AsRef<str>>,
) -> Result<ExternalSubsetResult, SubsetError> {
    let ontology_graphs = parse_named_graphs(ontology_graphs_of_interest)?;
    let dependency_graphs = parse_named_graphs(dependency_graphs)?;

    let seed_terms = seed::collect_reference_terms(store, &ontology_graphs)?;
    let referenced_dependency_terms =
        reference::select_dependency_terms(store, &dependency_graphs, &seed_terms)?;
    let closure = closure::bounded_support_closure(
        store,
        &dependency_graphs,
        &referenced_dependency_terms,
        EXTERNAL_SUBSET_SUPPORT_DEPTH,
        EXTERNAL_SUBSET_SUPPORT_PREDICATES,
    )?;
    let constructed = construct::construct_dependency_subset(
        store,
        &dependency_graphs,
        &referenced_dependency_terms,
        &closure.support_terms,
        &closure.support_edges,
        EXTERNAL_SUBSET_ANNOTATION_PREDICATES,
    )?;

    let direct_quad_count = constructed
        .quads
        .iter()
        .filter(|entry| entry.kind == ConstructedQuadKind::Direct)
        .count();
    let support_quad_count = constructed
        .quads
        .iter()
        .filter(|entry| entry.kind == ConstructedQuadKind::Support)
        .count();
    let annotation_quad_count = constructed
        .quads
        .iter()
        .filter(|entry| entry.kind == ConstructedQuadKind::Annotation)
        .count();
    let list_quad_count = constructed
        .quads
        .iter()
        .filter(|entry| entry.kind == ConstructedQuadKind::ListClosure)
        .count();

    let metadata = ExternalSubsetMetadata {
        referenced_term_count: referenced_dependency_terms.len(),
        seed_term_count: seed_terms.len(),
        support_term_count: closure.support_terms.len(),
        annotation_term_count: constructed.annotation_terms.len(),
        list_term_count: constructed.list_terms.len(),
        depth_boundary_term_count: closure.boundary_terms.len(),
        direct_quad_count,
        support_quad_count,
        annotation_quad_count,
        list_quad_count,
        depth_boundary_quad_count: closure.depth_boundary_quad_count,
        support_depth: EXTERNAL_SUBSET_SUPPORT_DEPTH,
        referenced_dependency_terms: sort_named_nodes(&referenced_dependency_terms),
        seed_dependency_terms: seed_terms.iter().cloned().collect(),
        support_terms: sorted_named_or_blank_keys(closure.support_terms.iter()),
        annotation_terms: constructed.annotation_terms.iter().cloned().collect(),
        list_terms: constructed.list_terms.iter().cloned().collect(),
        depth_boundary_terms: sorted_named_or_blank_keys(closure.boundary_terms.iter()),
    };

    Ok(ExternalSubsetResult {
        quads: constructed.quads,
        metadata,
    })
}

fn parse_named_graphs<I>(values: I) -> Result<Vec<NamedNode>, SubsetError>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut graphs = Vec::new();
    for value in values {
        let value = value.as_ref();
        let graph = NamedNode::new(value).map_err(|error| SubsetError::InvalidGraph {
            value: value.to_string(),
            error: error.to_string(),
        })?;
        graphs.push(graph);
    }
    Ok(graphs)
}

fn quad_key(quad: &Quad) -> String {
    format!(
        "{} {} {} {}",
        named_or_blank_node_key(&quad.subject),
        quad.predicate.as_str(),
        term_to_key(&quad.object),
        graph_key(&quad.graph_name),
    )
}

fn graph_key(graph_name: &GraphName) -> String {
    match graph_name {
        GraphName::DefaultGraph => String::from("DEFAULT"),
        GraphName::NamedNode(node) => node.as_str().to_string(),
        GraphName::BlankNode(node) => format!("_:{}", node.as_str()),
    }
}

fn term_to_key(term: &Term) -> String {
    match term {
        Term::BlankNode(node) => format!("_:{}", node.as_str()),
        Term::NamedNode(node) => node.as_str().to_string(),
        Term::Literal(literal) => format!("\"{}\"", literal.value()),
    }
}

fn sort_named_nodes(names: &BTreeSet<NamedNode>) -> Vec<String> {
    names.iter().map(|name| name.as_str().to_string()).collect()
}

fn sorted_named_or_blank_keys<'a, I>(names: I) -> Vec<String>
where
    I: Iterator<Item = &'a NamedOrBlankNode>,
{
    let mut values = names.map(named_or_blank_node_key).collect::<Vec<_>>();
    values.sort();
    values
}

pub(crate) fn term_key(term: &NamedOrBlankNode) -> String {
    named_or_blank_node_key(term)
}

pub(crate) fn quad_key_for_sort(quad: &Quad) -> String {
    quad_key(quad)
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxigraph::io::{RdfFormat, RdfParser};

    fn load_turtle(graph_iri: &str, payload: &str, store: &Store) {
        let graph_name = NamedNode::new(graph_iri).expect("graph IRI");
        store
            .load_from_reader(
                RdfParser::from_format(RdfFormat::Turtle)
                    .without_named_graphs()
                    .with_default_graph(graph_name.as_ref()),
                payload.as_bytes(),
            )
            .expect("graph loads");
    }

    fn quad_has_subject(quad: &ConstructedQuad, subject: &str) -> bool {
        match &quad.quad.subject {
            NamedOrBlankNode::NamedNode(node) => node.as_str() == subject,
            NamedOrBlankNode::BlankNode(node) => subject == format!("_:{}", node.as_str()),
        }
    }

    fn has_term(set: &[String], expected: &str) -> bool {
        set.iter().any(|term| term == expected)
    }

    #[test]
    fn reference_extraction_tracks_subject_predicate_object_and_list_members() {
        let store = Store::new().expect("store creates");
        load_turtle(
            "urn:o-kernel:test:authored",
            r#"
@prefix ex: <https://example.test/> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
ex:Subject ex:predicate ex:Object .
ex:ListHead rdf:first ex:ListMember ; rdf:rest rdf:nil .
"#,
            &store,
        );

        let result = build_external_dependency_subset(
            &store,
            vec!["urn:o-kernel:test:authored"],
            Vec::<String>::new(),
        )
        .expect("subset build succeeds");

        assert!(has_term(
            &result.metadata.seed_dependency_terms,
            "https://example.test/Subject"
        ));
        assert!(has_term(
            &result.metadata.seed_dependency_terms,
            "https://example.test/predicate"
        ));
        assert!(has_term(
            &result.metadata.seed_dependency_terms,
            "https://example.test/Object"
        ));
        assert!(has_term(
            &result.metadata.seed_dependency_terms,
            "https://example.test/ListMember"
        ));
        assert!(has_term(
            &result.metadata.seed_dependency_terms,
            "https://example.test/ListHead"
        ));
    }

    #[test]
    fn support_expansion_respects_depth_and_reports_boundary_terms() {
        let store = Store::new().expect("store creates");
        load_turtle(
            "urn:o-kernel:test:authored",
            r#"
@prefix ex: <https://example.test/> .
ex:Seed ex:seeded "seed" .
"#,
            &store,
        );
        load_turtle(
            "urn:o-kernel:test:dependency",
            r#"
@prefix ex: <https://example.test/> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
ex:Seed rdfs:subClassOf ex:Support1 .
ex:Support1 rdfs:subClassOf ex:Boundary2 .
ex:Boundary2 rdfs:subClassOf ex:Boundary3 .
ex:Support1 rdfs:label "support" .
"#,
            &store,
        );

        let result = build_external_dependency_subset(
            &store,
            vec!["urn:o-kernel:test:authored"],
            vec!["urn:o-kernel:test:dependency"],
        )
        .expect("subset build succeeds");

        assert!(has_term(
            &result.metadata.referenced_dependency_terms,
            "https://example.test/Seed"
        ));
        assert!(has_term(
            &result.metadata.support_terms,
            "https://example.test/Support1"
        ));
        assert!(result.metadata.depth_boundary_term_count > 0);
        assert!(result.metadata.depth_boundary_quad_count > 0);
    }

    #[test]
    fn annotation_triples_are_included_for_selected_and_support_terms() {
        let store = Store::new().expect("store creates");
        load_turtle(
            "urn:o-kernel:test:authored",
            r#"
@prefix ex: <https://example.test/> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
ex:Seed rdfs:subClassOf ex:Support .
"#,
            &store,
        );
        load_turtle(
            "urn:o-kernel:test:dependency",
            r#"
@prefix ex: <https://example.test/> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
ex:Seed rdfs:label "seed" ;
  rdfs:subClassOf ex:Support .
ex:Support rdfs:label "support" .
"#,
            &store,
        );

        let result = build_external_dependency_subset(
            &store,
            vec!["urn:o-kernel:test:authored"],
            vec!["urn:o-kernel:test:dependency"],
        )
        .expect("subset build succeeds");

        assert!(result.metadata.annotation_term_count >= 2);
        assert!(result
            .quads
            .iter()
            .any(|entry| entry.kind == ConstructedQuadKind::Annotation));
    }

    #[test]
    fn list_closure_preserves_order_and_terminal_nil() {
        let store = Store::new().expect("store creates");
        load_turtle(
            "urn:o-kernel:test:authored",
            r#"
@prefix ex: <https://example.test/> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
ex:Seed rdfs:subClassOf ex:Head .
"#,
            &store,
        );
        load_turtle(
            "urn:o-kernel:test:dependency",
            r#"
@prefix ex: <https://example.test/> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
ex:Seed rdfs:subClassOf ex:Head .
ex:Head rdf:first ex:A ;
  rdf:rest _:cell .
_:cell rdf:first ex:B ;
  rdf:rest rdf:nil .
"#,
            &store,
        );

        let result = build_external_dependency_subset(
            &store,
            vec!["urn:o-kernel:test:authored"],
            vec!["urn:o-kernel:test:dependency"],
        )
        .expect("subset build succeeds");

        let list_quads = result
            .quads
            .iter()
            .filter(|entry| entry.kind == ConstructedQuadKind::ListClosure)
            .count();
        assert!(list_quads >= 4);
        assert!(result.quads.iter().any(|entry| {
            quad_has_subject(entry, "https://example.test/Head")
                && entry.quad.predicate.as_str()
                    == "http://www.w3.org/1999/02/22-rdf-syntax-ns#rest"
                && matches!(entry.quad.object, Term::BlankNode(_))
        }));
        assert!(result
            .metadata
            .list_terms
            .iter()
            .any(|term| term.starts_with("_:")));
    }
}
