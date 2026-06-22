use oxigraph::io::{RdfFormat, RdfParser};
use oxigraph::model::{NamedOrBlankNode, Term};
use std::collections::BTreeSet;
use std::sync::OnceLock;

pub const RDF_NS: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
pub const RDFS_NS: &str = "http://www.w3.org/2000/01/rdf-schema#";
pub const XSD_NS: &str = "http://www.w3.org/2001/XMLSchema#";
pub const OWL_NS: &str = "http://www.w3.org/2002/07/owl#";
pub const SHACL_NS: &str = "http://www.w3.org/ns/shacl#";

pub const RDF_TURTLE: &str = include_str!("standards/rdf.ttl");
pub const RDFS_TURTLE: &str = include_str!("standards/rdfs.ttl");
pub const OWL_TURTLE: &str = include_str!("standards/owl.ttl");
pub const SHACL_TURTLE: &str = include_str!("standards/shacl.ttl");

pub fn is_reserved_namespace_iri(iri: &str) -> bool {
    iri.starts_with(RDF_NS)
        || iri.starts_with(RDFS_NS)
        || iri.starts_with(XSD_NS)
        || iri.starts_with(OWL_NS)
        || iri.starts_with(SHACL_NS)
}

pub fn is_reserved_vocabulary_iri(iri: &str) -> bool {
    is_standard_vocabulary_iri(iri)
        || is_builtin_datatype_iri(iri)
        || is_supported_shacl_datatype_iri(iri)
        || is_builtin_datatype_facet_iri(iri)
}

pub fn is_standard_vocabulary_iri(iri: &str) -> bool {
    standard_vocabulary_iris().contains(iri)
}

pub fn standard_vocabulary_iris() -> &'static BTreeSet<String> {
    static TERMS: OnceLock<BTreeSet<String>> = OnceLock::new();
    TERMS.get_or_init(|| {
        let mut terms = BTreeSet::new();
        collect_turtle_vocabulary_terms(RDF_TURTLE, &mut terms, "RDF standards vocabulary");
        collect_turtle_vocabulary_terms(RDFS_TURTLE, &mut terms, "RDFS standards vocabulary");
        collect_turtle_vocabulary_terms(OWL_TURTLE, &mut terms, "OWL standards vocabulary");
        collect_turtle_vocabulary_terms(SHACL_TURTLE, &mut terms, "SHACL standards vocabulary");
        terms
    })
}

fn collect_turtle_vocabulary_terms(turtle: &str, terms: &mut BTreeSet<String>, label: &str) {
    for parsed in RdfParser::from_format(RdfFormat::Turtle).for_reader(turtle.as_bytes()) {
        let quad = parsed
            .unwrap_or_else(|error| panic!("Bundled {label} failed to parse as Turtle: {error}"));

        if let NamedOrBlankNode::NamedNode(subject) = &quad.subject {
            collect_if_reserved_term(subject.as_str(), terms);
        }
        collect_if_reserved_term(quad.predicate.as_str(), terms);
        if let Term::NamedNode(object) = &quad.object {
            collect_if_reserved_term(object.as_str(), terms);
        }
    }
}

fn collect_if_reserved_term(iri: &str, terms: &mut BTreeSet<String>) {
    if iri.starts_with(RDF_NS)
        || iri.starts_with(RDFS_NS)
        || iri.starts_with(OWL_NS)
        || iri.starts_with(SHACL_NS)
    {
        terms.insert(iri.to_string());
    }
}

pub fn is_supported_datatype_iri(iri: &str) -> bool {
    is_builtin_datatype_iri(iri) || is_supported_shacl_datatype_iri(iri)
}

pub fn is_builtin_datatype_iri(iri: &str) -> bool {
    matches!(
        iri,
        "http://www.w3.org/1999/02/22-rdf-syntax-ns#PlainLiteral"
            | "http://www.w3.org/1999/02/22-rdf-syntax-ns#XMLLiteral"
            | "http://www.w3.org/2000/01/rdf-schema#Literal"
            | "http://www.w3.org/2002/07/owl#real"
            | "http://www.w3.org/2002/07/owl#rational"
            | "http://www.w3.org/2001/XMLSchema#anyURI"
            | "http://www.w3.org/2001/XMLSchema#base64Binary"
            | "http://www.w3.org/2001/XMLSchema#boolean"
            | "http://www.w3.org/2001/XMLSchema#byte"
            | "http://www.w3.org/2001/XMLSchema#dateTime"
            | "http://www.w3.org/2001/XMLSchema#dateTimeStamp"
            | "http://www.w3.org/2001/XMLSchema#decimal"
            | "http://www.w3.org/2001/XMLSchema#double"
            | "http://www.w3.org/2001/XMLSchema#float"
            | "http://www.w3.org/2001/XMLSchema#hexBinary"
            | "http://www.w3.org/2001/XMLSchema#int"
            | "http://www.w3.org/2001/XMLSchema#integer"
            | "http://www.w3.org/2001/XMLSchema#language"
            | "http://www.w3.org/2001/XMLSchema#long"
            | "http://www.w3.org/2001/XMLSchema#Name"
            | "http://www.w3.org/2001/XMLSchema#NCName"
            | "http://www.w3.org/2001/XMLSchema#negativeInteger"
            | "http://www.w3.org/2001/XMLSchema#NMTOKEN"
            | "http://www.w3.org/2001/XMLSchema#nonNegativeInteger"
            | "http://www.w3.org/2001/XMLSchema#nonPositiveInteger"
            | "http://www.w3.org/2001/XMLSchema#normalizedString"
            | "http://www.w3.org/2001/XMLSchema#positiveInteger"
            | "http://www.w3.org/2001/XMLSchema#short"
            | "http://www.w3.org/2001/XMLSchema#string"
            | "http://www.w3.org/2001/XMLSchema#token"
            | "http://www.w3.org/2001/XMLSchema#unsignedByte"
            | "http://www.w3.org/2001/XMLSchema#unsignedInt"
            | "http://www.w3.org/2001/XMLSchema#unsignedLong"
            | "http://www.w3.org/2001/XMLSchema#unsignedShort"
    )
}

pub fn is_supported_shacl_datatype_iri(iri: &str) -> bool {
    matches!(
        iri,
        "http://www.w3.org/2001/XMLSchema#date"
            | "http://www.w3.org/2001/XMLSchema#time"
            | "http://www.w3.org/2001/XMLSchema#duration"
            | "http://www.w3.org/2001/XMLSchema#dayTimeDuration"
            | "http://www.w3.org/2001/XMLSchema#yearMonthDuration"
            | "http://www.w3.org/2001/XMLSchema#gDay"
            | "http://www.w3.org/2001/XMLSchema#gMonth"
            | "http://www.w3.org/2001/XMLSchema#gMonthDay"
            | "http://www.w3.org/2001/XMLSchema#gYear"
            | "http://www.w3.org/2001/XMLSchema#gYearMonth"
    )
}

pub fn is_builtin_datatype_facet_iri(iri: &str) -> bool {
    matches!(
        iri,
        "http://www.w3.org/2001/XMLSchema#length"
            | "http://www.w3.org/2001/XMLSchema#maxExclusive"
            | "http://www.w3.org/2001/XMLSchema#maxInclusive"
            | "http://www.w3.org/2001/XMLSchema#maxLength"
            | "http://www.w3.org/2001/XMLSchema#minExclusive"
            | "http://www.w3.org/2001/XMLSchema#minInclusive"
            | "http://www.w3.org/2001/XMLSchema#minLength"
            | "http://www.w3.org/2001/XMLSchema#pattern"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_owl_reserved_datatypes_by_expanded_iri() {
        assert!(is_builtin_datatype_iri(
            "http://www.w3.org/2001/XMLSchema#string"
        ));
        assert!(is_builtin_datatype_iri(
            "http://www.w3.org/2000/01/rdf-schema#Literal"
        ));
        assert!(is_builtin_datatype_iri(
            "http://www.w3.org/2002/07/owl#real"
        ));
        assert!(!is_builtin_datatype_iri(
            "http://www.w3.org/2001/XMLSchema#date"
        ));
        assert!(!is_builtin_datatype_iri(
            "http://www.w3.org/2001/XMLSchema#pattern"
        ));
        assert!(!is_builtin_datatype_iri(
            "http://www.w3.org/2001/XMLSchema#notARealDatatype"
        ));
    }

    #[test]
    fn recognizes_supported_shacl_datatypes_separately_from_owl_builtins() {
        assert!(is_supported_shacl_datatype_iri(
            "http://www.w3.org/2001/XMLSchema#date"
        ));
        assert!(is_supported_datatype_iri(
            "http://www.w3.org/2001/XMLSchema#date"
        ));
        assert!(is_supported_datatype_iri(
            "http://www.w3.org/2001/XMLSchema#string"
        ));
        assert!(!is_supported_datatype_iri(
            "http://www.w3.org/2001/XMLSchema#pattern"
        ));
    }

    #[test]
    fn recognizes_owl_reserved_facets_separately_from_datatypes() {
        assert!(is_builtin_datatype_facet_iri(
            "http://www.w3.org/2001/XMLSchema#pattern"
        ));
        assert!(is_reserved_vocabulary_iri(
            "http://www.w3.org/2001/XMLSchema#pattern"
        ));
    }

    #[test]
    fn recognizes_expanded_reserved_namespace_and_vocabulary_iris() {
        assert!(is_reserved_namespace_iri(
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"
        ));
        assert!(is_reserved_namespace_iri(
            "http://www.w3.org/2000/01/rdf-schema#Class"
        ));
        assert!(is_reserved_namespace_iri(
            "http://www.w3.org/2001/XMLSchema#string"
        ));
        assert!(is_reserved_vocabulary_iri(
            "http://www.w3.org/2001/XMLSchema#string"
        ));
        assert!(is_reserved_vocabulary_iri(
            "http://www.w3.org/2002/07/owl#topObjectProperty"
        ));
        assert!(is_reserved_namespace_iri(
            "http://www.w3.org/ns/shacl#NodeShape"
        ));
        assert!(is_reserved_vocabulary_iri(
            "http://www.w3.org/ns/shacl#NodeShape"
        ));
        assert!(!is_reserved_vocabulary_iri(
            "https://example.org/custom#MyType"
        ));
        assert!(!is_reserved_vocabulary_iri(
            "http://www.w3.org/2002/07/owl#notARealOwlTerm"
        ));
    }

    #[test]
    fn standards_vocabulary_is_derived_from_bundled_graphs() {
        let terms = standard_vocabulary_iris();

        assert!(terms.contains("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"));
        assert!(terms.contains("http://www.w3.org/2000/01/rdf-schema#Class"));
        assert!(terms.contains("http://www.w3.org/2002/07/owl#Thing"));
        assert!(terms.contains("http://www.w3.org/ns/shacl#NodeShape"));
        assert!(!terms.contains("http://www.w3.org/2001/XMLSchema#string"));
        assert!(!terms.contains("http://www.w3.org/2002/07/owl#notARealOwlTerm"));
    }
}
