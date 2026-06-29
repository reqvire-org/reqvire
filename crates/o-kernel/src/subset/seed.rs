use crate::subset::{term_key, SubsetError};
use oxigraph::model::{NamedNode, NamedOrBlankNode, Term};
use oxigraph::store::Store;
use std::collections::BTreeSet;

pub fn collect_reference_terms(
    store: &Store,
    interest_graphs: &[NamedNode],
) -> Result<BTreeSet<String>, SubsetError> {
    let mut terms = BTreeSet::new();

    for graph_name in interest_graphs {
        for quad_result in
            store.quads_for_pattern(None, None, None, Some(graph_name.as_ref().into()))
        {
            let quad = quad_result.map_err(|error| SubsetError::StoreFailure {
                error: error.to_string(),
            })?;

            terms.insert(term_key(&NamedOrBlankNode::NamedNode(quad.predicate)));
            match quad.object {
                Term::NamedNode(node) => {
                    terms.insert(node.as_str().to_string());
                }
                Term::BlankNode(node) => {
                    terms.insert(format!("{}{}", "_:", node.as_str()));
                }
                _ => {}
            }

            terms.insert(term_key(&quad.subject));
        }
    }

    Ok(terms)
}
