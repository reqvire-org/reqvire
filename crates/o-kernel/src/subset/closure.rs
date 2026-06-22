use crate::subset::{
    quad_key_for_sort, term_key, ConstructedQuad, ConstructedQuadKind, SubsetError,
};
use crate::vocab::{RDF_FIRST, RDF_NIL, RDF_REST};
use oxigraph::model::{NamedNode, NamedOrBlankNode, Term};
use oxigraph::store::Store;
use std::collections::{BTreeSet, HashSet, VecDeque};

#[derive(Debug, Clone, Default)]
pub struct SupportClosure {
    pub support_terms: HashSet<NamedOrBlankNode>,
    pub boundary_terms: HashSet<NamedOrBlankNode>,
    pub support_edges: Vec<ConstructedQuad>,
    pub depth_boundary_quad_count: usize,
}

#[derive(Debug, Clone, Default)]
pub struct ListClosure {
    pub list_terms: HashSet<NamedOrBlankNode>,
    pub list_edges: Vec<ConstructedQuad>,
}

pub fn bounded_support_closure(
    store: &Store,
    dependency_graphs: &[NamedNode],
    referenced_terms: &BTreeSet<NamedNode>,
    support_depth: usize,
    support_predicates: &[&str],
) -> Result<SupportClosure, SubsetError> {
    if referenced_terms.is_empty() || dependency_graphs.is_empty() {
        return Ok(SupportClosure::default());
    }

    let mut closure = SupportClosure::default();
    let mut support_queue = VecDeque::new();
    let mut seen_terms = HashSet::new();
    let mut seen_support_edges = BTreeSet::new();
    let mut seen_boundary_edges = BTreeSet::new();

    for term in referenced_terms {
        let term = NamedOrBlankNode::NamedNode(term.clone());
        if seen_terms.insert(term_key(&term)) {
            support_queue.push_back((term, 0usize));
        }
    }

    while let Some((term, depth)) = support_queue.pop_front() {
        let edges =
            collect_term_support_edges(store, dependency_graphs, &term, support_predicates)?;

        for edge in edges {
            let edge_key = quad_key_for_sort(&edge.quad);

            if depth < support_depth {
                if seen_support_edges.insert(edge_key.clone()) {
                    closure.support_edges.push(edge.clone());
                }
            } else if seen_boundary_edges.insert(edge_key.clone()) {
                closure.depth_boundary_quad_count += 1;
            }

            if let Some(target) = target_term(&edge.quad.object) {
                let next_depth = depth + 1;
                if next_depth <= support_depth {
                    let target_key = term_key(&target);
                    if seen_terms.insert(target_key) {
                        closure.support_terms.insert(target.clone());
                        support_queue.push_back((target, next_depth));
                    }
                } else {
                    closure.boundary_terms.insert(target);
                }
            }
        }
    }

    closure
        .support_edges
        .sort_by_key(|edge| quad_key_for_sort(&edge.quad));
    Ok(closure)
}

fn collect_term_support_edges(
    store: &Store,
    dependency_graphs: &[NamedNode],
    term: &NamedOrBlankNode,
    support_predicates: &[&str],
) -> Result<Vec<ConstructedQuad>, SubsetError> {
    let mut edges = Vec::new();
    for graph_name in dependency_graphs {
        for quad_result in store.quads_for_pattern(
            Some(term.as_ref().into()),
            None,
            None,
            Some(graph_name.as_ref().into()),
        ) {
            let quad = quad_result.map_err(|error| SubsetError::StoreFailure {
                error: error.to_string(),
            })?;

            if !support_predicates.contains(&quad.predicate.as_str()) {
                continue;
            }

            if !matches!(quad.object, Term::NamedNode(_) | Term::BlankNode(_)) {
                continue;
            }

            edges.push(ConstructedQuad {
                quad,
                kind: ConstructedQuadKind::Support,
            });
        }
    }

    edges.sort_by_key(|edge| quad_key_for_sort(&edge.quad));
    edges.dedup_by_key(|edge| quad_key_for_sort(&edge.quad));
    Ok(edges)
}

pub fn collect_list_closure(
    store: &Store,
    dependency_graphs: &[NamedNode],
    roots: &[NamedOrBlankNode],
) -> Result<ListClosure, SubsetError> {
    let mut list = ListClosure::default();
    let mut worklist = VecDeque::new();
    let mut visited = HashSet::new();
    let mut seen_edges = BTreeSet::new();

    for root in roots {
        worklist.push_back(root.clone());
    }

    while let Some(term) = worklist.pop_front() {
        let key = term_key(&term);
        if !visited.insert(key) {
            continue;
        }

        let mut edges = collect_list_edges_for_term(store, dependency_graphs, &term)?;
        if edges.is_empty() {
            continue;
        }

        list.list_terms.insert(term.clone());

        edges.sort_by_key(|edge| quad_key_for_sort(&edge.quad));
        for edge in edges {
            let edge_key = quad_key_for_sort(&edge.quad);
            if !seen_edges.insert(edge_key) {
                continue;
            }

            if edge.quad.predicate.as_str() == RDF_REST {
                if let Some(rest_term) = target_term(&edge.quad.object) {
                    if let NamedOrBlankNode::NamedNode(node) = &rest_term {
                        if node.as_str() != RDF_NIL {
                            worklist.push_back(rest_term);
                        }
                    } else {
                        worklist.push_back(rest_term);
                    }
                }
            }

            list.list_edges.push(edge);
        }
    }

    list.list_edges
        .sort_by_key(|edge| quad_key_for_sort(&edge.quad));
    Ok(list)
}

fn collect_list_edges_for_term(
    store: &Store,
    dependency_graphs: &[NamedNode],
    term: &NamedOrBlankNode,
) -> Result<Vec<ConstructedQuad>, SubsetError> {
    let mut edges = Vec::new();
    for graph_name in dependency_graphs {
        for quad_result in store.quads_for_pattern(
            Some(term.as_ref().into()),
            None,
            None,
            Some(graph_name.as_ref().into()),
        ) {
            let quad = quad_result.map_err(|error| SubsetError::StoreFailure {
                error: error.to_string(),
            })?;

            if quad.predicate.as_str() == RDF_FIRST || quad.predicate.as_str() == RDF_REST {
                edges.push(ConstructedQuad {
                    quad,
                    kind: ConstructedQuadKind::ListClosure,
                });
            }
        }
    }

    edges.sort_by_key(|edge| quad_key_for_sort(&edge.quad));
    edges.dedup_by_key(|edge| quad_key_for_sort(&edge.quad));
    Ok(edges)
}

fn target_term(term: &Term) -> Option<NamedOrBlankNode> {
    match term {
        Term::NamedNode(node) => Some(NamedOrBlankNode::NamedNode(node.clone())),
        Term::BlankNode(node) => Some(NamedOrBlankNode::BlankNode(node.clone())),
        _ => None,
    }
}
