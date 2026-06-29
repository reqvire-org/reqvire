//! Bounded RDF term description construction.

use crate::vocab::{RDFS_COMMENT, RDFS_LABEL, RDF_FIRST, RDF_NIL, RDF_REST};
use oxigraph::model::{NamedOrBlankNode, Quad, Term};
use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};

pub const MODULE: &str = "describe";

pub const STANDARD_ANNOTATION_PREDICATES: &[&str] = &[
    RDFS_LABEL,
    RDFS_COMMENT,
    "http://www.w3.org/2004/02/skos/core#prefLabel",
    "http://www.w3.org/2004/02/skos/core#definition",
    "http://purl.org/dc/terms/description",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescribePolicy {
    pub support_predicates: Vec<String>,
    pub annotation_predicates: Vec<String>,
    pub support_depth: usize,
    pub include_rdf_list_closure: bool,
}

impl DescribePolicy {
    pub fn standard(support_predicates: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            support_predicates: support_predicates.into_iter().map(Into::into).collect(),
            annotation_predicates: STANDARD_ANNOTATION_PREDICATES
                .iter()
                .map(|predicate| (*predicate).to_string())
                .collect(),
            support_depth: 1,
            include_rdf_list_closure: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescriptionQuadKind {
    Direct,
    Support,
    Annotation,
    ListClosure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescriptionQuad {
    pub quad: Quad,
    pub kind: DescriptionQuadKind,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DescriptionMetadata {
    pub selected_term_count: usize,
    pub support_term_count: usize,
    pub annotation_term_count: usize,
    pub list_term_count: usize,
    pub direct_quad_count: usize,
    pub support_quad_count: usize,
    pub annotation_quad_count: usize,
    pub list_quad_count: usize,
    pub depth_boundary_term_count: usize,
    pub support_depth: usize,
    pub support_terms: Vec<String>,
    pub annotation_terms: Vec<String>,
    pub list_terms: Vec<String>,
    pub depth_boundary_terms: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Description {
    pub quads: Vec<DescriptionQuad>,
    pub metadata: DescriptionMetadata,
}

pub fn describe_terms(
    quads: impl IntoIterator<Item = Quad>,
    selected_terms: &[NamedOrBlankNode],
    policy: DescribePolicy,
) -> Description {
    let source_quads = quads.into_iter().collect::<Vec<_>>();
    let selected = selected_terms.to_vec();
    let selected_keys = selected.iter().map(term_key).collect::<BTreeSet<_>>();
    let support_predicates = policy
        .support_predicates
        .iter()
        .map(String::as_str)
        .collect::<HashSet<_>>();
    let annotation_predicates = policy
        .annotation_predicates
        .iter()
        .map(String::as_str)
        .collect::<HashSet<_>>();

    let mut result = BTreeMap::<String, DescriptionQuad>::new();
    let mut described_terms = selected.clone();
    let mut support_terms = BTreeSet::new();
    let mut boundary_terms = BTreeSet::new();
    let mut queue = selected
        .iter()
        .cloned()
        .map(|term| (term, 0usize))
        .collect::<VecDeque<_>>();
    let mut visited = selected_keys;

    for term in &selected {
        for quad in source_quads.iter().filter(|quad| &quad.subject == term) {
            add_quad(&mut result, quad.clone(), DescriptionQuadKind::Direct);
        }
    }

    while let Some((term, depth)) = queue.pop_front() {
        for quad in source_quads.iter().filter(|quad| quad.subject == term) {
            if !support_predicates.contains(quad.predicate.as_str()) {
                continue;
            }
            let Some(target) = object_resource(&quad.object) else {
                continue;
            };
            let target_key = term_key(&target);
            let next_depth = depth + 1;
            if next_depth <= policy.support_depth {
                add_quad(&mut result, quad.clone(), DescriptionQuadKind::Support);
                support_terms.insert(target_key.clone());
                if visited.insert(target_key) {
                    queue.push_back((target.clone(), next_depth));
                    described_terms.push(target);
                }
            } else {
                boundary_terms.insert(target_key);
            }
        }
    }

    let described_keys = described_terms
        .iter()
        .map(term_key)
        .collect::<BTreeSet<_>>();
    let mut annotation_terms = BTreeSet::new();
    for term in &described_terms {
        for quad in source_quads.iter().filter(|quad| &quad.subject == term) {
            if annotation_predicates.contains(quad.predicate.as_str()) {
                annotation_terms.insert(term_key(term));
                add_quad(&mut result, quad.clone(), DescriptionQuadKind::Annotation);
            }
        }
    }

    let mut list_terms = BTreeSet::new();
    if policy.include_rdf_list_closure {
        let list_roots = result
            .values()
            .filter_map(|entry| object_resource(&entry.quad.object))
            .filter(|term| {
                source_quads.iter().any(|quad| {
                    &quad.subject == term
                        && (quad.predicate.as_str() == RDF_FIRST
                            || quad.predicate.as_str() == RDF_REST)
                })
            })
            .collect::<Vec<_>>();
        collect_list_closure(&source_quads, list_roots, &mut result, &mut list_terms);
    }

    let mut out = result.into_values().collect::<Vec<_>>();
    out.sort_by_key(|entry| quad_key(&entry.quad));

    let metadata = DescriptionMetadata {
        selected_term_count: selected_terms.len(),
        support_term_count: support_terms.len(),
        annotation_term_count: annotation_terms.len(),
        list_term_count: list_terms.len(),
        direct_quad_count: out
            .iter()
            .filter(|entry| entry.kind == DescriptionQuadKind::Direct)
            .count(),
        support_quad_count: out
            .iter()
            .filter(|entry| entry.kind == DescriptionQuadKind::Support)
            .count(),
        annotation_quad_count: out
            .iter()
            .filter(|entry| entry.kind == DescriptionQuadKind::Annotation)
            .count(),
        list_quad_count: out
            .iter()
            .filter(|entry| entry.kind == DescriptionQuadKind::ListClosure)
            .count(),
        depth_boundary_term_count: boundary_terms.len(),
        support_depth: policy.support_depth,
        support_terms: support_terms.into_iter().collect(),
        annotation_terms: annotation_terms.into_iter().collect(),
        list_terms: list_terms.into_iter().collect(),
        depth_boundary_terms: boundary_terms.into_iter().collect(),
    };

    let _ = described_keys;
    Description {
        quads: out,
        metadata,
    }
}

fn collect_list_closure(
    source_quads: &[Quad],
    roots: Vec<NamedOrBlankNode>,
    result: &mut BTreeMap<String, DescriptionQuad>,
    list_terms: &mut BTreeSet<String>,
) {
    let mut queue = roots.into_iter().collect::<VecDeque<_>>();
    let mut visited = BTreeSet::new();
    while let Some(term) = queue.pop_front() {
        if !visited.insert(term_key(&term)) {
            continue;
        }
        for quad in source_quads.iter().filter(|quad| quad.subject == term) {
            if quad.predicate.as_str() != RDF_FIRST && quad.predicate.as_str() != RDF_REST {
                continue;
            }
            list_terms.insert(term_key(&term));
            add_quad(result, quad.clone(), DescriptionQuadKind::ListClosure);
            if quad.predicate.as_str() == RDF_REST {
                if let Some(next) = object_resource(&quad.object) {
                    if !matches!(&next, NamedOrBlankNode::NamedNode(node) if node.as_str() == RDF_NIL)
                    {
                        queue.push_back(next);
                    }
                }
            }
        }
    }
}

fn object_resource(term: &Term) -> Option<NamedOrBlankNode> {
    match term {
        Term::NamedNode(node) => Some(NamedOrBlankNode::NamedNode(node.clone())),
        Term::BlankNode(node) => Some(NamedOrBlankNode::BlankNode(node.clone())),
        _ => None,
    }
}

fn add_quad(result: &mut BTreeMap<String, DescriptionQuad>, quad: Quad, kind: DescriptionQuadKind) {
    let key = quad_key(&quad);
    if let Some(existing) = result.get_mut(&key) {
        if existing.kind == DescriptionQuadKind::Direct && kind != DescriptionQuadKind::Direct {
            existing.kind = kind;
        }
    } else {
        result.insert(key, DescriptionQuad { quad, kind });
    }
}

fn term_key(term: &NamedOrBlankNode) -> String {
    match term {
        NamedOrBlankNode::NamedNode(node) => node.as_str().to_string(),
        NamedOrBlankNode::BlankNode(node) => format!("_:{}", node.as_str()),
    }
}

fn quad_key(quad: &Quad) -> String {
    format!(
        "{} {} {} {}",
        term_key(&quad.subject),
        quad.predicate.as_str(),
        quad.object,
        quad.graph_name
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vocab::{RDFS_COMMENT, RDFS_LABEL, RDFS_SUBCLASS_OF, RDF_FIRST, RDF_REST};
    use oxigraph::model::{BlankNode, GraphName, Literal, NamedNode};

    fn iri(value: &str) -> NamedNode {
        NamedNode::new(value).expect("test iri")
    }

    fn quad(subject: NamedOrBlankNode, predicate: &str, object: Term) -> Quad {
        Quad {
            subject,
            predicate: iri(predicate),
            object,
            graph_name: GraphName::DefaultGraph,
        }
    }

    #[test]
    fn selected_terms_produce_direct_description_quads() {
        let term = NamedOrBlankNode::NamedNode(iri("https://example.test/Term"));
        let quads = vec![quad(
            term.clone(),
            RDFS_LABEL,
            Term::Literal(Literal::new_simple_literal("Term label")),
        )];

        let description = describe_terms(
            quads,
            &[term],
            DescribePolicy::standard(Vec::<String>::new()),
        );

        assert_eq!(description.metadata.selected_term_count, 1);
        assert_eq!(description.metadata.annotation_quad_count, 1);
        assert_eq!(description.quads.len(), 1);
    }

    #[test]
    fn support_predicates_follow_bounded_resource_descriptions() {
        let selected = NamedOrBlankNode::NamedNode(iri("https://example.test/Selected"));
        let support = NamedOrBlankNode::NamedNode(iri("https://example.test/Support"));
        let boundary = NamedOrBlankNode::NamedNode(iri("https://example.test/Boundary"));
        let quads = vec![
            quad(
                selected.clone(),
                RDFS_SUBCLASS_OF,
                Term::NamedNode(iri("https://example.test/Support")),
            ),
            quad(
                support.clone(),
                RDFS_SUBCLASS_OF,
                Term::NamedNode(iri("https://example.test/Boundary")),
            ),
            quad(
                support,
                RDFS_COMMENT,
                Term::Literal(Literal::new_simple_literal("support comment")),
            ),
            quad(
                boundary,
                RDFS_LABEL,
                Term::Literal(Literal::new_simple_literal("boundary label")),
            ),
        ];

        let description = describe_terms(
            quads,
            &[selected],
            DescribePolicy::standard([RDFS_SUBCLASS_OF]),
        );

        assert!(description
            .metadata
            .support_terms
            .contains(&"https://example.test/Support".to_string()));
        assert!(description
            .metadata
            .depth_boundary_terms
            .contains(&"https://example.test/Boundary".to_string()));
        assert_eq!(description.metadata.annotation_term_count, 1);
    }

    #[test]
    fn rdf_list_closure_preserves_reached_list_nodes() {
        let selected = NamedOrBlankNode::NamedNode(iri("https://example.test/Selected"));
        let head = BlankNode::new("head").expect("blank");
        let tail = BlankNode::new("tail").expect("blank");
        let quads = vec![
            quad(
                selected.clone(),
                RDFS_SUBCLASS_OF,
                Term::BlankNode(head.clone()),
            ),
            quad(
                head.clone().into(),
                RDF_FIRST,
                Term::NamedNode(iri("https://example.test/A")),
            ),
            quad(head.clone().into(), RDF_REST, Term::BlankNode(tail.clone())),
            quad(
                tail.clone().into(),
                RDF_FIRST,
                Term::NamedNode(iri("https://example.test/B")),
            ),
            quad(
                tail.into(),
                RDF_REST,
                Term::NamedNode(iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#nil")),
            ),
        ];

        let description = describe_terms(
            quads,
            &[selected],
            DescribePolicy::standard([RDFS_SUBCLASS_OF]),
        );

        assert_eq!(description.metadata.list_quad_count, 4);
        assert_eq!(description.metadata.list_term_count, 2);
    }
}
