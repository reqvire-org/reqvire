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
    namespace: o_kernel::vocab::SKOS_NS,
    resource: o_kernel::vocab::SKOS_ONTOLOGY,
    source: "builtin:skos.rdf",
    format: "rdfxml",
    content: include_str!("builtin_external_sources/skos.rdf"),
};

pub const EXTERNAL_ONTOLOGIES: &[BuiltinExternalOntology] = &[SKOS];
