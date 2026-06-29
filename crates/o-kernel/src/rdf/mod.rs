//! Standards-generic RDF helpers over Oxigraph-native data types.

use oxigraph::model::NamedOrBlankNode;

pub use crate::vocab::*;
pub const MODULE: &str = "rdf";

pub(crate) fn named_or_blank_node_key(term: &NamedOrBlankNode) -> String {
    match term {
        NamedOrBlankNode::NamedNode(node) => node.as_str().to_string(),
        NamedOrBlankNode::BlankNode(node) => format!("_:{}", node.as_str()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxigraph::model::{NamedNode, NamedOrBlankNode, Term};

    #[test]
    fn subject_iri_handles_expanded_named_and_blank_nodes() {
        let subject = NamedOrBlankNode::NamedNode(
            NamedNode::new("https://example.org/terms/subject").expect("expanded IRI should parse"),
        );
        let blank = NamedOrBlankNode::BlankNode(oxigraph::model::BlankNode::default());

        assert_eq!(
            subject_iri(&subject),
            Some("https://example.org/terms/subject")
        );
        assert_eq!(subject_iri(&blank), None);
    }

    #[test]
    fn term_iri_reports_only_named_node_iris() {
        let named = Term::NamedNode(
            NamedNode::new("https://example.org/terms/object").expect("expanded IRI should parse"),
        );

        assert_eq!(term_iri(&named), Some("https://example.org/terms/object"));
        assert_eq!(term_iri(&Term::Literal("value".into())), None);
    }
}
