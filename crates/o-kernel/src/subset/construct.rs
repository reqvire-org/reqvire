use crate::subset::closure::collect_list_closure;
use crate::subset::{
    quad_key_for_sort, term_key, ConstructedQuad, ConstructedQuadKind, SubsetError,
};
use oxigraph::model::{NamedNode, NamedOrBlankNode, Quad};
use oxigraph::store::Store;
use std::collections::{BTreeMap, BTreeSet, HashSet};

#[derive(Debug, Default, Clone)]
pub struct ConstructResult {
    pub quads: Vec<ConstructedQuad>,
    pub annotation_terms: BTreeSet<String>,
    pub list_terms: BTreeSet<String>,
}

pub fn construct_dependency_subset(
    store: &Store,
    dependency_graphs: &[NamedNode],
    referenced_terms: &BTreeSet<NamedNode>,
    support_terms: &HashSet<NamedOrBlankNode>,
    support_edges: &[ConstructedQuad],
    annotation_predicates: &[&str],
) -> Result<ConstructResult, SubsetError> {
    let mut quads = BTreeMap::<String, ConstructedQuad>::new();
    let mut annotation_terms = BTreeSet::new();

    let mut annotation_roots = Vec::new();
    let mut direct_roots = Vec::new();
    let mut dedupe_roots = HashSet::new();

    for term in referenced_terms {
        let root = NamedOrBlankNode::NamedNode(term.clone());
        let key = term_key(&root);
        if dedupe_roots.insert(key.clone()) {
            annotation_roots.push(root.clone());
            direct_roots.push(root.clone());
        }
        add_direct_quads(store, dependency_graphs, root, &mut quads)?;
    }

    for term in support_terms {
        let key = term_key(term);
        if dedupe_roots.insert(key) {
            annotation_roots.push(term.clone());
        }
    }

    for edge in support_edges {
        add_quad(&mut quads, edge.quad.clone(), ConstructedQuadKind::Support)?;
    }

    for term in &annotation_roots {
        let mut has_annotation = false;
        for graph_name in dependency_graphs {
            for quad_result in store.quads_for_pattern(
                Some(term.as_ref()),
                None,
                None,
                Some(graph_name.as_ref().into()),
            ) {
                let quad = quad_result.map_err(|error| SubsetError::StoreFailure {
                    error: error.to_string(),
                })?;

                if annotation_predicates.contains(&quad.predicate.as_str()) {
                    has_annotation = true;
                    add_quad(&mut quads, quad, ConstructedQuadKind::Annotation)?;
                }
            }
        }

        if has_annotation {
            annotation_terms.insert(term_key(term));
        }
    }

    let mut list_roots = direct_roots;
    list_roots.extend(support_terms.iter().cloned());
    list_roots.sort_by_key(term_key);
    list_roots.dedup_by_key(|term| term_key(term));

    let list_closure = collect_list_closure(store, dependency_graphs, &list_roots)?;
    for edge in list_closure.list_edges {
        add_quad(&mut quads, edge.quad, ConstructedQuadKind::ListClosure)?;
    }

    let mut out = quads.into_values().collect::<Vec<_>>();
    out.sort_by_key(|entry| quad_key_for_sort(&entry.quad));

    Ok(ConstructResult {
        quads: out,
        annotation_terms,
        list_terms: list_closure
            .list_terms
            .into_iter()
            .map(|term| term_key(&term))
            .collect(),
    })
}

fn add_direct_quads(
    store: &Store,
    dependency_graphs: &[NamedNode],
    term: NamedOrBlankNode,
    quads: &mut BTreeMap<String, ConstructedQuad>,
) -> Result<(), SubsetError> {
    for graph_name in dependency_graphs {
        for quad_result in store.quads_for_pattern(
            Some(term.as_ref()),
            None,
            None,
            Some(graph_name.as_ref().into()),
        ) {
            let quad = quad_result.map_err(|error| SubsetError::StoreFailure {
                error: error.to_string(),
            })?;
            add_quad(quads, quad, ConstructedQuadKind::Direct)?;
        }
    }
    Ok(())
}

fn add_quad(
    quads: &mut BTreeMap<String, ConstructedQuad>,
    quad: Quad,
    kind: ConstructedQuadKind,
) -> Result<(), SubsetError> {
    let key = quad_key_for_sort(&quad);
    if let Some(existing) = quads.get_mut(&key) {
        if existing.kind == ConstructedQuadKind::Direct && kind != ConstructedQuadKind::Direct {
            existing.kind = kind;
        }
    } else {
        quads.insert(key, ConstructedQuad { quad, kind });
    }
    Ok(())
}
