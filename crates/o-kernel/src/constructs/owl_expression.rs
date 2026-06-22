use crate::constructs::classify::{
    symbol, OntologyClassExpressionKind, OntologyConstructBuilder, OntologyRestrictionKind,
    SourcedQuad,
};
use crate::vocab::{
    OWL_ALL_VALUES_FROM, OWL_COMPLEMENT_OF, OWL_INTERSECTION_OF, OWL_SOME_VALUES_FROM, OWL_UNION_OF,
};

pub(super) fn classify(builder: &mut OntologyConstructBuilder, sourced_quad: &SourcedQuad) -> bool {
    let quad = &sourced_quad.quad;
    match quad.predicate.as_str() {
        OWL_ALL_VALUES_FROM => {
            builder.add_restriction_construct(
                &sourced_quad.source,
                quad,
                OntologyRestrictionKind::Universal,
                Some(symbol("universal-restriction")),
            );
            true
        }
        OWL_SOME_VALUES_FROM => {
            builder.add_restriction_construct(
                &sourced_quad.source,
                quad,
                OntologyRestrictionKind::Existential,
                Some(symbol("existential-restriction")),
            );
            true
        }
        OWL_INTERSECTION_OF => {
            builder.add_class_expression_construct(
                &sourced_quad.source,
                quad,
                OntologyClassExpressionKind::Intersection,
                Some(symbol("intersection")),
            );
            true
        }
        OWL_UNION_OF => {
            builder.add_class_expression_construct(
                &sourced_quad.source,
                quad,
                OntologyClassExpressionKind::Union,
                Some(symbol("union")),
            );
            true
        }
        OWL_COMPLEMENT_OF => {
            builder.add_class_expression_construct(
                &sourced_quad.source,
                quad,
                OntologyClassExpressionKind::Complement,
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
    use crate::vocab::{OWL_UNION_OF, RDF_FIRST, RDF_REST};
    use oxigraph::model::{BlankNode, NamedNode, Quad, Term};

    #[test]
    fn class_expression_preserves_list_members() {
        let head = BlankNode::new("list").expect("blank node");
        let first = BlankNode::new("first").expect("blank node");
        let second = BlankNode::new("second").expect("blank node");

        let expression = SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: NamedNode::new("https://example.org/expr")
                    .expect("predicate subject")
                    .into(),
                predicate: NamedNode::new(OWL_UNION_OF).expect("predicate"),
                object: Term::BlankNode(head.clone()),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };
        let a = SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: head.clone().into(),
                predicate: NamedNode::new(RDF_FIRST).expect(""),
                object: Term::NamedNode(NamedNode::new("https://example.org/a").expect("")),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };
        let b = SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: head.clone().into(),
                predicate: NamedNode::new(RDF_REST).expect(""),
                object: Term::BlankNode(first.clone()),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };
        let c = SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: first.clone().into(),
                predicate: NamedNode::new(RDF_FIRST).expect(""),
                object: Term::NamedNode(NamedNode::new("https://example.org/b").expect("")),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };
        let d = SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: first.clone().into(),
                predicate: NamedNode::new(RDF_REST).expect(""),
                object: Term::BlankNode(second.clone()),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };
        let e = SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: second.clone().into(),
                predicate: NamedNode::new(RDF_FIRST).expect(""),
                object: Term::NamedNode(NamedNode::new("https://example.org/c").expect("")),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };
        let f = SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: second.clone().into(),
                predicate: NamedNode::new(RDF_REST).expect(""),
                object: Term::NamedNode(
                    oxigraph::model::NamedNode::new(crate::vocab::RDF_NIL).expect(""),
                ),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };

        let projection = super::super::classify_ontology_constructs_with_sources(&[
            expression, a, b, c, d, e, f,
        ]);

        assert_eq!(projection.constructs.len(), 1);
        let members = &projection.constructs[0].members;
        assert_eq!(members.len(), 3);
        assert_eq!(members[0].term.value, "https://example.org/a");
        assert_eq!(members[1].term.value, "https://example.org/b");
        assert_eq!(members[2].term.value, "https://example.org/c");
    }
}
