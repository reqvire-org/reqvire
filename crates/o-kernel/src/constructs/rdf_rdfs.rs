use crate::constructs::classify::{
    symbol, OntologyConstructBuilder, OntologyConstructFamily, OntologyConstructKind, SourcedQuad,
};
use crate::vocab::{RDFS_DOMAIN, RDFS_RANGE, RDFS_SUBCLASS_OF};

pub(super) fn classify(builder: &mut OntologyConstructBuilder, sourced_quad: &SourcedQuad) -> bool {
    let quad = &sourced_quad.quad;

    match quad.predicate.as_str() {
        RDFS_DOMAIN => {
            builder.add_direct_construct(
                &sourced_quad.source,
                quad,
                OntologyConstructFamily::PropertyDomainRange,
                OntologyConstructKind::PropertyDomain,
                None,
                None,
                None,
                None,
                Vec::new(),
            );
            true
        }
        RDFS_RANGE => {
            builder.add_direct_construct(
                &sourced_quad.source,
                quad,
                OntologyConstructFamily::PropertyDomainRange,
                OntologyConstructKind::PropertyRange,
                None,
                None,
                None,
                None,
                Vec::new(),
            );
            true
        }
        RDFS_SUBCLASS_OF => {
            builder.add_direct_construct(
                &sourced_quad.source,
                quad,
                OntologyConstructFamily::SubclassMembership,
                OntologyConstructKind::SubclassInclusion,
                Some(symbol("subset-or-equal")),
                None,
                None,
                None,
                Vec::new(),
            );
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vocab::RDFS_DOMAIN;
    use oxigraph::model::{NamedNode, Quad, Term};

    #[test]
    fn rdfs_domain_becomes_property_domain_construct() {
        let quad = SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: NamedNode::new("https://example.org/p")
                    .expect("subject iri")
                    .into(),
                predicate: NamedNode::new(RDFS_DOMAIN).expect("predicate iri"),
                object: Term::NamedNode(
                    NamedNode::new("https://example.org/o").expect("object iri"),
                ),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };

        let mut builder = OntologyConstructBuilder::new(Default::default(), Default::default());
        let handled = classify(&mut builder, &quad);
        let projection = builder.finish();

        assert!(handled);
        assert_eq!(
            projection.constructs[0].kind,
            OntologyConstructKind::PropertyDomain
        );
        assert_eq!(
            projection.constructs[0].subject.value,
            "https://example.org/p"
        );
    }

    #[test]
    fn unrelated_predicates_do_not_match() {
        let quad = SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: NamedNode::new("https://example.org/p")
                    .expect("subject iri")
                    .into(),
                predicate: NamedNode::new("http://example.org/other").expect("predicate iri"),
                object: Term::NamedNode(
                    NamedNode::new("https://example.org/o").expect("object iri"),
                ),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };
        let mut builder = OntologyConstructBuilder::new(Default::default(), Default::default());
        assert!(!classify(&mut builder, &quad));
    }
}
