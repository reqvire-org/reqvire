//! Generic RDF/RDFS/OWL/SHACL construct classification types and dispatch helpers.

pub mod classify;
pub mod owl_expression;
pub mod owl_property;
pub mod rdf_rdfs;
pub mod restriction;
pub mod shacl_overlay;

pub use classify::{
    classify_ontology_constructs, classify_ontology_constructs_with_sources,
    OntologyClassExpressionKind, OntologyConstruct, OntologyConstructEvidence,
    OntologyConstructFamily, OntologyConstructKind, OntologyConstructMember,
    OntologyConstructProjection, OntologyConstructSource, OntologyConstructTerm,
    OntologyConstructTermKind, OntologyProjection, OntologyPropertyCharacteristic,
    OntologyRestrictionKind, OntologyShapeOverlayKind, OntologySymbol, SourcedQuad,
};
pub const MODULE: &str = "constructs";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_exports_include_known_construct_types() {
        let _ = core::mem::size_of::<OntologyConstruct>();
        let _ = core::mem::size_of::<OntologyProjection>();
    }

    #[test]
    fn module_is_compilable() {
        assert!(MODULE.len() > 0);
    }
}
