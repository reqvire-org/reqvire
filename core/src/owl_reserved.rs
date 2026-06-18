pub const RDF_NS: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
pub const RDFS_NS: &str = "http://www.w3.org/2000/01/rdf-schema#";
pub const XSD_NS: &str = "http://www.w3.org/2001/XMLSchema#";
pub const OWL_NS: &str = "http://www.w3.org/2002/07/owl#";

pub fn is_reserved_namespace_iri(iri: &str) -> bool {
    iri.starts_with(RDF_NS)
        || iri.starts_with(RDFS_NS)
        || iri.starts_with(XSD_NS)
        || iri.starts_with(OWL_NS)
}

pub fn is_reserved_vocabulary_iri(iri: &str) -> bool {
    is_builtin_datatype_iri(iri)
        || is_builtin_datatype_facet_iri(iri)
        || matches!(
            iri,
            "http://www.w3.org/2002/07/owl#backwardCompatibleWith"
                | "http://www.w3.org/2002/07/owl#bottomDataProperty"
                | "http://www.w3.org/2002/07/owl#bottomObjectProperty"
                | "http://www.w3.org/2002/07/owl#deprecated"
                | "http://www.w3.org/2002/07/owl#incompatibleWith"
                | "http://www.w3.org/2002/07/owl#Nothing"
                | "http://www.w3.org/2002/07/owl#priorVersion"
                | "http://www.w3.org/2002/07/owl#Thing"
                | "http://www.w3.org/2002/07/owl#topDataProperty"
                | "http://www.w3.org/2002/07/owl#topObjectProperty"
                | "http://www.w3.org/2002/07/owl#versionInfo"
                | "http://www.w3.org/1999/02/22-rdf-syntax-ns#langRange"
                | "http://www.w3.org/2000/01/rdf-schema#comment"
                | "http://www.w3.org/2000/01/rdf-schema#isDefinedBy"
                | "http://www.w3.org/2000/01/rdf-schema#label"
                | "http://www.w3.org/2000/01/rdf-schema#seeAlso"
        )
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
            "http://www.w3.org/2001/XMLSchema#pattern"
        ));
        assert!(!is_builtin_datatype_iri(
            "http://www.w3.org/2001/XMLSchema#notARealDatatype"
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
}
