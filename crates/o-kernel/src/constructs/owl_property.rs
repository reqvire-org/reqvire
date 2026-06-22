use crate::constructs::classify::{
    symbol, OntologyConstructBuilder, OntologyConstructFamily, OntologyConstructKind, SourcedQuad,
};
use crate::vocab::{
    OWL_DISJOINT_WITH, OWL_EQUIVALENT_CLASS, OWL_EQUIVALENT_PROPERTY, OWL_INVERSE_OF,
    OWL_PROPERTY_CHAIN_AXIOM, OWL_SAME_AS, RDF_TYPE,
};

pub(super) fn classify(builder: &mut OntologyConstructBuilder, sourced_quad: &SourcedQuad) -> bool {
    let quad = &sourced_quad.quad;

    match quad.predicate.as_str() {
        RDF_TYPE => {
            builder.add_type_construct(sourced_quad);
            true
        }
        OWL_DISJOINT_WITH => {
            builder.add_direct_construct(
                &sourced_quad.source,
                quad,
                OntologyConstructFamily::DisjointEquivalenceInverse,
                OntologyConstructKind::Disjointness,
                Some(symbol("disjointness")),
                None,
                None,
                None,
                Vec::new(),
            );
            true
        }
        OWL_EQUIVALENT_CLASS | OWL_EQUIVALENT_PROPERTY | OWL_SAME_AS => {
            builder.record_equivalence(sourced_quad);
            true
        }
        OWL_INVERSE_OF => {
            builder.add_direct_construct(
                &sourced_quad.source,
                quad,
                OntologyConstructFamily::DisjointEquivalenceInverse,
                OntologyConstructKind::InverseProperty,
                Some(symbol("inverse-property")),
                None,
                None,
                None,
                Vec::new(),
            );
            true
        }
        OWL_PROPERTY_CHAIN_AXIOM => {
            let members = builder.members_for_list_term(&quad.object);
            builder.add_direct_construct(
                &sourced_quad.source,
                quad,
                OntologyConstructFamily::PropertyChain,
                OntologyConstructKind::PropertyChain,
                Some(symbol("logical-implication")),
                None,
                None,
                None,
                members,
            );
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vocab::{OWL_EQUIVALENT_CLASS, OWL_INVERSE_OF};
    use oxigraph::model::{NamedNode, Quad, Term};

    fn quad_for(predicate: &str) -> SourcedQuad {
        SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: NamedNode::new("https://example.org/p").expect("").into(),
                predicate: NamedNode::new(predicate).expect(""),
                object: Term::NamedNode(NamedNode::new("https://example.org/q").expect("")),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        }
    }

    #[test]
    fn inverse_property_is_classified() {
        let quad = quad_for(OWL_INVERSE_OF);
        let mut builder = OntologyConstructBuilder::new(Default::default(), Default::default());
        let handled = classify(&mut builder, &quad);
        let projection = builder.finish();

        assert!(handled);
        assert_eq!(
            projection.constructs[0].kind,
            OntologyConstructKind::InverseProperty
        );
    }

    #[test]
    fn equivalent_classification_records_equivalence_edge() {
        let quad = quad_for(OWL_EQUIVALENT_CLASS);
        let mut builder = OntologyConstructBuilder::new(Default::default(), Default::default());
        let handled = classify(&mut builder, &quad);
        let projection = builder.finish();

        assert!(handled);
        assert_eq!(projection.constructs.len(), 1);
        assert_eq!(
            projection.constructs[0].kind,
            OntologyConstructKind::EquivalenceGroup
        );
        assert_eq!(
            projection.constructs[0].family,
            OntologyConstructFamily::DisjointEquivalenceInverse
        );
    }
}
