use crate::subset::SubsetError;
use oxigraph::model::{NamedNode, NamedOrBlankNode};
use oxigraph::store::Store;
use std::collections::BTreeSet;

pub fn select_dependency_terms(
    store: &Store,
    dependency_graphs: &[NamedNode],
    seed_terms: &BTreeSet<String>,
) -> Result<BTreeSet<NamedNode>, SubsetError> {
    let mut candidates = BTreeSet::new();
    for term in seed_terms {
        if !term.starts_with("_:") {
            candidates.insert(term.clone());
        }
    }

    let mut selected = BTreeSet::new();
    if candidates.is_empty() || dependency_graphs.is_empty() {
        return Ok(selected);
    }

    for graph_name in dependency_graphs {
        for quad_result in
            store.quads_for_pattern(None, None, None, Some(graph_name.as_ref().into()))
        {
            let quad = quad_result.map_err(|error| SubsetError::StoreFailure {
                error: error.to_string(),
            })?;

            if let NamedOrBlankNode::NamedNode(subject) = quad.subject {
                if candidates.contains(subject.as_str()) {
                    selected.insert(subject);
                }
            }
        }
    }

    Ok(selected)
}
