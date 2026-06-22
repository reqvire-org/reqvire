use crate::constructs::classify::{OntologyConstructBuilder, SourcedQuad};
use crate::vocab::{
    SH_CLASS, SH_DATATYPE, SH_IN, SH_MAX_COUNT, SH_MIN_COUNT, SH_NODE_KIND, SH_PATH, SH_PATTERN,
    SH_PROPERTY, SH_TARGET_CLASS,
};

pub(super) fn classify(builder: &mut OntologyConstructBuilder, sourced_quad: &SourcedQuad) -> bool {
    let quad = &sourced_quad.quad;
    match quad.predicate.as_str() {
        SH_TARGET_CLASS | SH_PROPERTY | SH_PATH | SH_DATATYPE | SH_CLASS | SH_NODE_KIND
        | SH_MIN_COUNT | SH_MAX_COUNT | SH_PATTERN | SH_IN => {
            builder.add_shape_overlay_construct(&sourced_quad.source, quad);
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constructs::classify::{OntologyConstructFamily, OntologyConstructKind};
    use crate::vocab::SH_PROPERTY;
    use oxigraph::model::{BlankNode, NamedNode, Quad, Term};

    #[test]
    fn shacl_overlay_predicate_classifies_overlay_construct() {
        let quad = SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: NamedNode::new("https://example.org/sh")
                    .expect("subject")
                    .into(),
                predicate: NamedNode::new(SH_PROPERTY).expect("predicate"),
                object: Term::BlankNode(BlankNode::new("shape-property").expect("blank node")),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };

        let projection = super::super::classify_ontology_constructs_with_sources(&[quad]);
        let construct = &projection.constructs[0];
        assert_eq!(construct.kind, OntologyConstructKind::ShapeOverlay);
        assert_eq!(construct.family, OntologyConstructFamily::ShapeOverlay);
    }
}
