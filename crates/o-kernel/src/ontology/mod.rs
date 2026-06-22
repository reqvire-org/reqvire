//! Ontology declaration indexing and named-term lookup.

use crate::shacl;
use crate::vocab::reserved as owl_reserved;
use crate::vocab::{RDFS_COMMENT, RDFS_LABEL, SH_DATATYPE, SH_PATH};
use oxigraph::model::{NamedOrBlankNode, Quad};

pub const MODULE: &str = "ontology";

/// Returns the referenced IRI and predicate label for an ontology alignment issue.
pub fn alignment_reference(error: &shacl::AlignmentError) -> Option<(&str, &'static str)> {
    match error {
        shacl::AlignmentError::UndeclaredClass {
            class_node: NamedOrBlankNode::NamedNode(iri),
            predicate,
            ..
        } => Some((iri.as_str(), predicate)),
        shacl::AlignmentError::UndeclaredClass {
            class_node: NamedOrBlankNode::BlankNode(_),
            ..
        } => None,
        shacl::AlignmentError::UndeclaredProperty {
            property_iri,
            predicate,
            ..
        } => Some((property_iri.as_str(), predicate)),
        shacl::AlignmentError::UndeclaredDatatype {
            datatype_iri,
            predicate,
            ..
        } => Some((datatype_iri.as_str(), predicate)),
        shacl::AlignmentError::UndeclaredNode {
            node_iri,
            predicate,
            ..
        } => Some((node_iri.as_str(), predicate)),
        shacl::AlignmentError::InvalidInversePath {
            property_iri,
            predicate,
            ..
        } => Some((property_iri.as_str(), predicate)),
    }
}

/// Extracts referenced SHACL IRIs from a graph, excluding ontology-neutral builtins.
pub fn extract_shape_references(quads: &[Quad]) -> Vec<shacl::ReferencedIri> {
    let mut references = Vec::new();
    for reference in shacl::ShaclRegistry::parse(quads).referenced_iris() {
        let iri = reference.iri.as_str();
        if reference.predicate == SH_DATATYPE && owl_reserved::is_supported_datatype_iri(iri) {
            continue;
        }
        if reference.predicate == SH_PATH && is_builtin_annotation_path(iri) {
            continue;
        }
        references.push(reference);
    }
    references
}

fn is_builtin_annotation_path(iri: &str) -> bool {
    matches!(iri, RDFS_LABEL | RDFS_COMMENT)
}

#[cfg(test)]
mod tests {
    use super::{alignment_reference, extract_shape_references, MODULE};
    use oxigraph::io::{RdfFormat, RdfParser};
    use oxigraph::model::{BlankNode, NamedNode, NamedOrBlankNode};

    const EX: &str = "https://example.org/model#";
    const SH_PATH: &str = "http://www.w3.org/ns/shacl#path";
    const SH_CLASS: &str = "http://www.w3.org/ns/shacl#class";
    const SH_DATATYPE: &str = "http://www.w3.org/ns/shacl#datatype";
    const RDFS_LABEL: &str = "http://www.w3.org/2000/01/rdf-schema#label";

    fn parse_turtle(content: &str) -> Vec<oxigraph::model::Quad> {
        RdfParser::from_format(RdfFormat::Turtle)
            .for_reader(content.as_bytes())
            .map(|quad| quad.expect("test turtle parse"))
            .collect()
    }

    fn node(local: &str) -> NamedNode {
        NamedNode::new(format!("{EX}{local}")).expect("test IRI should be valid")
    }

    #[test]
    fn ontology_extract_shape_references_filters_builtin_terms() {
        let quads = parse_turtle(
            r#"
@prefix ex: <https://example.org/model#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:Shape
  a sh:NodeShape ;
  sh:property [
    sh:path rdfs:label
  ] ;
  sh:property [
    sh:path ex:validPath ;
    sh:class ex:Invoice ;
    sh:datatype xsd:string ;
    sh:lessThan ex:number ;
    sh:lessThan ex:owner
  ] .
"#,
        );

        let references = extract_shape_references(&quads);
        let values = references
            .iter()
            .map(|reference| (reference.iri.as_str(), reference.predicate))
            .collect::<Vec<_>>();

        assert_eq!(
            values.contains(&("http://www.w3.org/2001/XMLSchema#string", SH_DATATYPE)),
            false
        );
        assert_eq!(
            values.contains(&(RDFS_LABEL, SH_PATH)),
            false,
            "annotation path should be filtered"
        );
        assert_eq!(
            values.contains(&("https://example.org/model#Invoice", SH_CLASS)),
            true
        );
        assert_eq!(
            values.contains(&("https://example.org/model#validPath", SH_PATH)),
            true
        );
    }

    #[test]
    fn ontology_alignment_reference_exposes_iri_and_predicate() {
        let shape_id = NamedOrBlankNode::NamedNode(node("Shape"));
        let errors = vec![
            super::shacl::AlignmentError::UndeclaredClass {
                shape_id: shape_id.clone(),
                class_node: NamedOrBlankNode::NamedNode(node("MissingClass")),
                predicate: SH_CLASS,
            },
            super::shacl::AlignmentError::UndeclaredProperty {
                shape_id,
                property_iri: node("MissingProperty"),
                predicate: "http://www.w3.org/ns/shacl#lessThan",
            },
        ];

        let (first_iri, first_predicate) =
            alignment_reference(&errors[0]).expect("class issue mapped");
        assert_eq!(first_iri, "https://example.org/model#MissingClass");
        assert_eq!(first_predicate, SH_CLASS);

        let (second_iri, second_predicate) =
            alignment_reference(&errors[1]).expect("property issue mapped");
        assert_eq!(second_iri, "https://example.org/model#MissingProperty");
        assert_eq!(second_predicate, "http://www.w3.org/ns/shacl#lessThan");
    }

    #[test]
    fn ontology_alignment_reference_skips_blank_node_subjects() {
        let issue = super::shacl::AlignmentError::UndeclaredClass {
            shape_id: NamedOrBlankNode::NamedNode(node("Shape")),
            class_node: NamedOrBlankNode::BlankNode(BlankNode::default()),
            predicate: SH_CLASS,
        };
        assert!(alignment_reference(&issue).is_none());
    }

    #[test]
    fn ontology_module_constant_available() {
        assert_eq!(MODULE, "ontology");
    }
}
