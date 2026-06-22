//! Built-in external ontology sources shipped by Reqvire.

pub const MODULE: &str = "builtin_external_sources";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuiltinExternalOntology {
    pub id: &'static str,
    pub prefix: &'static str,
    pub namespace: &'static str,
    pub resource: &'static str,
    pub source: &'static str,
    pub format: &'static str,
    pub content: &'static str,
}

pub const SKOS: BuiltinExternalOntology = BuiltinExternalOntology {
    id: "skos",
    prefix: "skos",
    namespace: "http://www.w3.org/2004/02/skos/core#",
    resource: "http://www.w3.org/2004/02/skos/core",
    source: "builtin:skos.rdf",
    format: "rdfxml",
    content: include_str!("builtin_external_sources/skos.rdf"),
};

pub const EXTERNAL_ONTOLOGIES: &[BuiltinExternalOntology] = &[SKOS];
