use crate::constructs::classify::OntologyRestrictionKind;
use crate::constructs::classify::{OntologyConstructBuilder, SourcedQuad};
use crate::vocab::{
    OWL_CARDINALITY, OWL_HAS_VALUE, OWL_MAX_CARDINALITY, OWL_MAX_QUALIFIED_CARDINALITY,
    OWL_MIN_CARDINALITY, OWL_MIN_QUALIFIED_CARDINALITY, OWL_ON_CLASS, OWL_ON_DATA_RANGE,
    OWL_QUALIFIED_CARDINALITY,
};

pub(super) fn classify(builder: &mut OntologyConstructBuilder, sourced_quad: &SourcedQuad) -> bool {
    let quad = &sourced_quad.quad;
    match quad.predicate.as_str() {
        OWL_HAS_VALUE => {
            builder.add_restriction_construct(
                &sourced_quad.source,
                quad,
                OntologyRestrictionKind::HasValue,
                None,
            );
            true
        }
        OWL_CARDINALITY => {
            builder.add_restriction_construct(
                &sourced_quad.source,
                quad,
                OntologyRestrictionKind::Cardinality,
                None,
            );
            true
        }
        OWL_MIN_CARDINALITY => {
            builder.add_restriction_construct(
                &sourced_quad.source,
                quad,
                OntologyRestrictionKind::MinCardinality,
                None,
            );
            true
        }
        OWL_MAX_CARDINALITY => {
            builder.add_restriction_construct(
                &sourced_quad.source,
                quad,
                OntologyRestrictionKind::MaxCardinality,
                None,
            );
            true
        }
        OWL_QUALIFIED_CARDINALITY => {
            builder.add_restriction_construct(
                &sourced_quad.source,
                quad,
                OntologyRestrictionKind::QualifiedCardinality,
                None,
            );
            true
        }
        OWL_MIN_QUALIFIED_CARDINALITY => {
            builder.add_restriction_construct(
                &sourced_quad.source,
                quad,
                OntologyRestrictionKind::MinQualifiedCardinality,
                None,
            );
            true
        }
        OWL_MAX_QUALIFIED_CARDINALITY => {
            builder.add_restriction_construct(
                &sourced_quad.source,
                quad,
                OntologyRestrictionKind::MaxQualifiedCardinality,
                None,
            );
            true
        }
        OWL_ON_CLASS => {
            builder.add_restriction_construct(
                &sourced_quad.source,
                quad,
                OntologyRestrictionKind::OnClass,
                None,
            );
            true
        }
        OWL_ON_DATA_RANGE => {
            builder.add_restriction_construct(
                &sourced_quad.source,
                quad,
                OntologyRestrictionKind::OnDataRange,
                None,
            );
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constructs::classify::OntologyConstructKind;
    use crate::vocab::OWL_MIN_CARDINALITY;
    use oxigraph::model::{NamedNode, Quad, Term};

    #[test]
    fn cardinality_restriction_maps_restriction_construct() {
        let quad = SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: NamedNode::new("https://example.org/p")
                    .expect("subject")
                    .into(),
                predicate: NamedNode::new(OWL_MIN_CARDINALITY).expect("predicate"),
                object: Term::Literal("1".into()),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };

        let projection = super::super::classify_ontology_constructs_with_sources(&[quad]);
        let construct = &projection.constructs[0];
        assert_eq!(construct.kind, OntologyConstructKind::Restriction);
        assert!(construct.restriction_kind.is_some());
    }
}
