//! Generic RDF, OWL, and SHACL ontology kernel primitives.

pub mod constructs;
pub mod describe;
pub mod diagnostics;
pub mod ontology;
pub mod owl_reserved;
pub mod prelude;
pub mod rdf;
pub mod shacl;
pub mod stable;
pub mod subset;
pub mod vocab;

#[cfg(test)]
mod tests {
    use super::{constructs, describe, diagnostics, ontology, prelude, rdf, stable, subset, vocab};

    #[test]
    fn public_module_paths_compile() {
        let _ = (
            core::any::type_name_of_val(&vocab::MODULE),
            core::any::type_name_of_val(&rdf::MODULE),
            core::any::type_name_of_val(&diagnostics::MODULE),
            core::any::type_name_of_val(&ontology::MODULE),
            core::any::type_name_of_val(&constructs::MODULE),
            core::any::type_name_of_val(&describe::MODULE),
            core::any::type_name_of_val(&subset::MODULE),
            core::any::type_name_of_val(&prelude::MODULE),
            core::any::type_name_of_val(&stable::stable_hash),
        );
    }
}
