use crate::semantic_contract::{
    OntologyClassExpressionKind, OntologyConstruct, OntologyConstructFamily, OntologyConstructKind,
    OntologyProjectionSource, OntologyProjectionTerm, OntologyProjectionTermKind,
    OntologyPropertyCharacteristic, OntologyRestrictionKind, OntologyShapeOverlayKind,
    OntologySymbol, SemanticBlock, SemanticBlockKind, SemanticIndex,
};
use oxigraph::model::{NamedOrBlankNode, Term};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const RDF_FIRST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#first";
const RDF_REST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#rest";
const RDF_NIL: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#nil";
const RDFS_LABEL: &str = "http://www.w3.org/2000/01/rdf-schema#label";
const RDFS_COMMENT: &str = "http://www.w3.org/2000/01/rdf-schema#comment";
const RDFS_DOMAIN: &str = "http://www.w3.org/2000/01/rdf-schema#domain";
const RDFS_RANGE: &str = "http://www.w3.org/2000/01/rdf-schema#range";
const RDFS_SUBCLASS_OF: &str = "http://www.w3.org/2000/01/rdf-schema#subClassOf";
const RDFS_CLASS: &str = "http://www.w3.org/2000/01/rdf-schema#Class";
const RDFS_DATATYPE: &str = "http://www.w3.org/2000/01/rdf-schema#Datatype";
const RDFS_LITERAL: &str = "http://www.w3.org/2000/01/rdf-schema#Literal";
const RDFS_RESOURCE: &str = "http://www.w3.org/2000/01/rdf-schema#Resource";
const OWL_CLASS: &str = "http://www.w3.org/2002/07/owl#Class";
const OWL_NAMED_INDIVIDUAL: &str = "http://www.w3.org/2002/07/owl#NamedIndividual";
const OWL_ONTOLOGY: &str = "http://www.w3.org/2002/07/owl#Ontology";
const OWL_THING: &str = "http://www.w3.org/2002/07/owl#Thing";
const RDF_PROPERTY: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#Property";
const RDF_LIST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#List";
const OWL_OBJECT_PROPERTY: &str = "http://www.w3.org/2002/07/owl#ObjectProperty";
const OWL_DATATYPE_PROPERTY: &str = "http://www.w3.org/2002/07/owl#DatatypeProperty";
const OWL_DISJOINT_WITH: &str = "http://www.w3.org/2002/07/owl#disjointWith";
const OWL_EQUIVALENT_CLASS: &str = "http://www.w3.org/2002/07/owl#equivalentClass";
const OWL_EQUIVALENT_PROPERTY: &str = "http://www.w3.org/2002/07/owl#equivalentProperty";
const OWL_SAME_AS: &str = "http://www.w3.org/2002/07/owl#sameAs";
const OWL_INVERSE_OF: &str = "http://www.w3.org/2002/07/owl#inverseOf";
const OWL_PROPERTY_CHAIN_AXIOM: &str = "http://www.w3.org/2002/07/owl#propertyChainAxiom";
const OWL_ON_PROPERTY: &str = "http://www.w3.org/2002/07/owl#onProperty";
const OWL_ALL_VALUES_FROM: &str = "http://www.w3.org/2002/07/owl#allValuesFrom";
const OWL_SOME_VALUES_FROM: &str = "http://www.w3.org/2002/07/owl#someValuesFrom";
const OWL_HAS_VALUE: &str = "http://www.w3.org/2002/07/owl#hasValue";
const OWL_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#cardinality";
const OWL_MIN_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#minCardinality";
const OWL_MAX_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#maxCardinality";
const OWL_QUALIFIED_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#qualifiedCardinality";
const OWL_MIN_QUALIFIED_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#minQualifiedCardinality";
const OWL_MAX_QUALIFIED_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#maxQualifiedCardinality";
const OWL_ON_CLASS: &str = "http://www.w3.org/2002/07/owl#onClass";
const OWL_ON_DATA_RANGE: &str = "http://www.w3.org/2002/07/owl#onDataRange";
const OWL_INTERSECTION_OF: &str = "http://www.w3.org/2002/07/owl#intersectionOf";
const OWL_UNION_OF: &str = "http://www.w3.org/2002/07/owl#unionOf";
const OWL_COMPLEMENT_OF: &str = "http://www.w3.org/2002/07/owl#complementOf";
const SHACL_PREFIX: &str = "http://www.w3.org/ns/shacl#";
const SH_NODE_SHAPE: &str = "http://www.w3.org/ns/shacl#NodeShape";
const SH_PROPERTY_SHAPE: &str = "http://www.w3.org/ns/shacl#PropertyShape";
const SH_PROPERTY: &str = "http://www.w3.org/ns/shacl#property";
const SH_PATH: &str = "http://www.w3.org/ns/shacl#path";
const SH_TARGET_CLASS: &str = "http://www.w3.org/ns/shacl#targetClass";
const SH_DATATYPE: &str = "http://www.w3.org/ns/shacl#datatype";
const SH_CLASS: &str = "http://www.w3.org/ns/shacl#class";
const SH_NODE_KIND: &str = "http://www.w3.org/ns/shacl#nodeKind";
const SH_MIN_COUNT: &str = "http://www.w3.org/ns/shacl#minCount";
const SH_MAX_COUNT: &str = "http://www.w3.org/ns/shacl#maxCount";
const SH_PATTERN: &str = "http://www.w3.org/ns/shacl#pattern";
const SH_IN: &str = "http://www.w3.org/ns/shacl#in";
const XSD_PREFIX: &str = "http://www.w3.org/2001/XMLSchema#";

#[derive(Debug, Clone, Serialize)]
pub struct OntologyGraphData {
    pub nodes: Vec<OntologyGraphNode>,
    pub edges: Vec<OntologyGraphEdge>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OntologyGraphNode {
    pub id: String,
    pub label: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub semantic_type: String,
    pub full_uri: String,
    pub comment: String,
    pub rdf_types: Vec<String>,
    pub type_evidence: Vec<OntologyGraphTypeEvidence>,
    pub sources: Vec<OntologyGraphSource>,
    pub constraints: Vec<OntologyGraphConstraint>,
    pub badges: Vec<OntologyGraphBadge>,
    pub equivalence_group: String,
    pub inverse_properties: Vec<String>,
    pub property_chains: Vec<OntologyGraphPropertyChain>,
    pub domain: Vec<OntologyGraphTermRef>,
    pub range: Vec<OntologyGraphTermRef>,
    pub literal_values: Vec<OntologyGraphLiteralValue>,
    pub slot_facets: Vec<OntologyGraphSlotFacet>,
    pub constructs: Vec<OntologyGraphConstructDetail>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct OntologyGraphTermRef {
    pub label: String,
    pub iri: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OntologyGraphLiteralValue {
    pub predicate: String,
    pub value: String,
    pub source: OntologyGraphSource,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OntologyGraphBadge {
    pub kind: String,
    pub symbol: String,
    pub code_point: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OntologyGraphPropertyChain {
    pub id: String,
    pub members: Vec<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OntologyGraphSlotFacet {
    pub slot_label: String,
    pub slot_iri: String,
    pub slot_kind: String,
    pub target_class_label: String,
    pub target_class_iri: String,
    pub source_shape_label: String,
    pub source_shape_iri: String,
    pub source: OntologyGraphSource,
    pub facets: Vec<OntologyGraphSlotFacetValue>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct OntologyGraphSlotFacetValue {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OntologyGraphConstructDetail {
    pub id: String,
    pub family: String,
    pub kind: String,
    pub label: String,
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub property: String,
    pub members: Vec<String>,
    pub source: OntologyGraphSource,
    pub badge: Option<OntologyGraphBadge>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OntologyGraphTypeEvidence {
    pub iri: String,
    pub label: String,
    pub source: OntologyGraphSource,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OntologyGraphSource {
    pub source: String,
    pub source_name: String,
    pub file_path: String,
    pub line_number: usize,
    pub kind: String,
    pub link: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OntologyGraphConstraint {
    pub property: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct OntologyGraphEdge {
    pub source: String,
    pub target: String,
    pub label: String,
}

pub fn build_graph_data(report: &SemanticIndex) -> OntologyGraphData {
    let mut nodes: BTreeMap<String, OntologyGraphNode> = BTreeMap::new();
    let mut edges: BTreeSet<OntologyGraphEdge> = BTreeSet::new();
    let internal_property_shapes = collect_internal_property_shapes(report);
    let (rdf_list_nodes, rdf_list_values) = collect_rdf_lists(report);
    let primary_blank_nodes =
        collect_primary_blank_nodes(report, &internal_property_shapes, &rdf_list_nodes);

    for block in &report.blocks {
        for quad in &block.quads {
            let subject_id = subject_id(&quad.subject);
            let predicate = quad.predicate.as_str();
            let predicate_label = clean_uri(predicate);

            if let Some(owner_id) = internal_property_shapes.get(&subject_id) {
                ensure_node(&mut nodes, owner_id, "shacl", block);
                append_property_shape_constraint(
                    &mut nodes,
                    owner_id,
                    predicate,
                    &quad.object,
                    &rdf_list_values,
                );
                continue;
            }

            if rdf_list_nodes.contains(&subject_id) {
                continue;
            }

            if is_blank_node_id(&subject_id) && !primary_blank_nodes.contains_key(&subject_id) {
                continue;
            }

            let subject_hint = if matches!(block.kind, SemanticBlockKind::Shapes)
                || predicate.starts_with(SHACL_PREFIX)
            {
                "shacl"
            } else {
                "generic"
            };

            ensure_node(&mut nodes, &subject_id, subject_hint, block);

            if predicate == RDF_TYPE {
                if let Some(node) = nodes.get_mut(&subject_id) {
                    record_type_evidence(node, &quad.object, block);
                }
                continue;
            }

            if predicate == RDFS_LABEL {
                if let Term::Literal(literal) = &quad.object {
                    if let Some(node) = nodes.get_mut(&subject_id) {
                        node.label = literal.value().to_string();
                    }
                }
                continue;
            }

            if predicate == RDFS_COMMENT {
                if let Term::Literal(literal) = &quad.object {
                    if let Some(node) = nodes.get_mut(&subject_id) {
                        node.comment = literal.value().to_string();
                    }
                }
                continue;
            }

            if is_constraint_predicate(predicate) {
                let value = term_label_with_lists(&quad.object, &rdf_list_values);
                if let Some(node) = nodes.get_mut(&subject_id) {
                    node.constraints.push(OntologyGraphConstraint {
                        property: predicate_label.clone(),
                        value,
                    });
                }
                continue;
            }

            if is_projection_construct_predicate(predicate) {
                continue;
            }

            match &quad.object {
                Term::NamedNode(node) => {
                    if node.as_str() == RDF_NIL {
                        continue;
                    }
                    let target_id = node.as_str().to_string();
                    if !is_hidden_primary_graph_iri(&target_id) {
                        let target_hint = if predicate == SH_DATATYPE || is_datatype_iri(&target_id)
                        {
                            "generic"
                        } else if matches!(block.kind, SemanticBlockKind::Shapes)
                            || predicate.starts_with(SHACL_PREFIX)
                        {
                            "shacl"
                        } else {
                            "generic"
                        };
                        ensure_node(&mut nodes, &target_id, target_hint, block);
                        if predicate == SH_DATATYPE {
                            if let Some(target_node) = nodes.get_mut(&target_id) {
                                upgrade_semantic_type_string(
                                    &mut target_node.semantic_type,
                                    "datatype",
                                );
                            }
                        }
                        edges.insert(OntologyGraphEdge {
                            source: subject_id.clone(),
                            target: target_id,
                            label: predicate_label,
                        });
                    }
                }
                Term::BlankNode(node) => {
                    let target_id = node.to_string();
                    if predicate == SH_PROPERTY && internal_property_shapes.contains_key(&target_id)
                    {
                        continue;
                    }
                    if rdf_list_nodes.contains(&target_id) {
                        continue;
                    }
                    if !primary_blank_nodes.contains_key(&target_id) {
                        continue;
                    }
                    ensure_node(&mut nodes, &target_id, subject_hint, block);
                    edges.insert(OntologyGraphEdge {
                        source: subject_id.clone(),
                        target: target_id,
                        label: predicate_label,
                    });
                }
                Term::Literal(literal) => {
                    if let Some(node) = nodes.get_mut(&subject_id) {
                        node.literal_values.push(OntologyGraphLiteralValue {
                            predicate: predicate_label,
                            value: literal.value().to_string(),
                            source: source_metadata(block),
                        });
                    }
                }
            }
        }
    }

    apply_blank_node_labels(&mut nodes);
    populate_construct_metadata(&mut nodes, &mut edges, report);
    apply_shape_slot_facets(&mut nodes, report, &rdf_list_values);
    promote_typed_named_individuals(&mut nodes);

    for node in nodes.values_mut() {
        node.node_type = visual_node_type_for_semantic_type(&node.semantic_type).to_string();
        node.domain.sort();
        node.domain.dedup();
        node.range.sort();
        node.range.dedup();
        node.literal_values.sort_by(|a, b| {
            (
                a.predicate.as_str(),
                a.value.as_str(),
                a.source.source.as_str(),
                a.source.file_path.as_str(),
                a.source.line_number,
            )
                .cmp(&(
                    b.predicate.as_str(),
                    b.value.as_str(),
                    b.source.source.as_str(),
                    b.source.file_path.as_str(),
                    b.source.line_number,
                ))
        });
        node.literal_values.dedup();
        node.slot_facets.sort_by(|a, b| {
            (
                a.target_class_iri.as_str(),
                a.slot_iri.as_str(),
                a.source_shape_iri.as_str(),
            )
                .cmp(&(
                    b.target_class_iri.as_str(),
                    b.slot_iri.as_str(),
                    b.source_shape_iri.as_str(),
                ))
        });
        node.slot_facets.dedup_by(|a, b| {
            a.target_class_iri == b.target_class_iri
                && a.slot_iri == b.slot_iri
                && a.source_shape_iri == b.source_shape_iri
                && a.facets == b.facets
        });
        node.constructs.sort_by(|a, b| a.id.cmp(&b.id));
        node.constructs.dedup_by(|a, b| a.id == b.id);
    }

    let nodes: Vec<_> = nodes.into_values().filter(is_primary_graph_node).collect();
    let node_ids: BTreeSet<_> = nodes.iter().map(|node| node.id.as_str()).collect();
    let edges = edges
        .into_iter()
        .filter(|edge| {
            node_ids.contains(edge.source.as_str()) && node_ids.contains(edge.target.as_str())
        })
        .collect();

    OntologyGraphData { nodes, edges }
}

fn collect_internal_property_shapes(report: &SemanticIndex) -> BTreeMap<String, String> {
    let mut property_shapes = BTreeMap::new();
    for block in &report.blocks {
        for quad in &block.quads {
            if quad.predicate.as_str() != SH_PROPERTY {
                continue;
            }
            if let Term::BlankNode(node) = &quad.object {
                property_shapes.insert(node.to_string(), subject_id(&quad.subject));
            }
        }
    }
    property_shapes
}

fn apply_shape_slot_facets(
    nodes: &mut BTreeMap<String, OntologyGraphNode>,
    report: &SemanticIndex,
    rdf_list_values: &BTreeMap<String, Vec<String>>,
) {
    let mut target_classes: BTreeMap<String, Vec<OntologyGraphTermRef>> = BTreeMap::new();
    let mut property_shape_owners: BTreeMap<String, (String, OntologyGraphSource)> =
        BTreeMap::new();
    let mut property_shape_quads: BTreeMap<String, Vec<(String, Term)>> = BTreeMap::new();

    for block in &report.blocks {
        for quad in &block.quads {
            let predicate = quad.predicate.as_str();
            let subject = subject_id(&quad.subject);

            if predicate == SH_TARGET_CLASS {
                if let Some(target) = shacl_target_ref(&quad.object, "class") {
                    target_classes
                        .entry(subject.clone())
                        .or_default()
                        .push(target);
                }
            }

            if predicate == SH_PROPERTY {
                if let Some(property_shape_id) = term_node_id(&quad.object) {
                    property_shape_owners
                        .insert(property_shape_id, (subject.clone(), source_metadata(block)));
                }
            }

            if is_slot_facet_predicate(predicate) {
                property_shape_quads
                    .entry(subject)
                    .or_default()
                    .push((predicate.to_string(), quad.object.clone()));
            }
        }
    }

    for targets in target_classes.values_mut() {
        targets.sort();
        targets.dedup();
    }

    for (property_shape_id, (owner_shape_id, source)) in property_shape_owners {
        let Some(targets) = target_classes.get(&owner_shape_id) else {
            continue;
        };
        let Some(quads) = property_shape_quads.get(&property_shape_id) else {
            continue;
        };
        let Some(path) = quads
            .iter()
            .find(|(predicate, _)| predicate == SH_PATH)
            .and_then(|(_, term)| shacl_target_ref(term, "property"))
        else {
            continue;
        };

        let mut facets: Vec<_> = quads
            .iter()
            .filter(|(predicate, _)| predicate != SH_PATH)
            .filter_map(|(predicate, term)| slot_facet_value(predicate, term, rdf_list_values))
            .collect();
        facets.sort();
        facets.dedup();

        for target in targets {
            let facet = OntologyGraphSlotFacet {
                slot_label: path.label.clone(),
                slot_iri: path.iri.clone(),
                slot_kind: path.kind.clone(),
                target_class_label: target.label.clone(),
                target_class_iri: target.iri.clone(),
                source_shape_label: clean_uri(&owner_shape_id),
                source_shape_iri: owner_shape_id.clone(),
                source: source.clone(),
                facets: facets.clone(),
            };

            if let Some(target_node) = nodes.get_mut(&target.iri) {
                push_slot_facet(target_node, facet.clone());
            }

            if !path.iri.is_empty() {
                if let Some(property_node) = nodes.get_mut(&path.iri) {
                    push_slot_facet(property_node, facet);
                }
            }
        }
    }
}

fn push_slot_facet(node: &mut OntologyGraphNode, facet: OntologyGraphSlotFacet) {
    if !node.slot_facets.contains(&facet) {
        node.slot_facets.push(facet);
    }
}

fn shacl_target_ref(term: &Term, kind: &'static str) -> Option<OntologyGraphTermRef> {
    match term {
        Term::NamedNode(node) => Some(OntologyGraphTermRef {
            label: clean_uri(node.as_str()),
            iri: node.as_str().to_string(),
            kind: kind.to_string(),
        }),
        Term::BlankNode(node) => Some(OntologyGraphTermRef {
            label: "anonymous node".to_string(),
            iri: node.to_string(),
            kind: kind.to_string(),
        }),
        Term::Literal(_) => None,
    }
}

fn term_node_id(term: &Term) -> Option<String> {
    match term {
        Term::NamedNode(node) => Some(node.as_str().to_string()),
        Term::BlankNode(node) => Some(node.to_string()),
        Term::Literal(_) => None,
    }
}

fn is_slot_facet_predicate(predicate: &str) -> bool {
    matches!(
        predicate,
        SH_PATH
            | SH_DATATYPE
            | SH_CLASS
            | SH_NODE_KIND
            | SH_MIN_COUNT
            | SH_MAX_COUNT
            | SH_PATTERN
            | SH_IN
    )
}

fn slot_facet_value(
    predicate: &str,
    term: &Term,
    rdf_list_values: &BTreeMap<String, Vec<String>>,
) -> Option<OntologyGraphSlotFacetValue> {
    let name = match predicate {
        SH_DATATYPE => "datatype",
        SH_CLASS => "class",
        SH_NODE_KIND => "nodeKind",
        SH_MIN_COUNT => "minCount",
        SH_MAX_COUNT => "maxCount",
        SH_PATTERN => "pattern",
        SH_IN => "allowed values",
        _ => return None,
    };
    Some(OntologyGraphSlotFacetValue {
        name: name.to_string(),
        value: term_label_with_lists(term, rdf_list_values),
    })
}

fn collect_rdf_lists(report: &SemanticIndex) -> (BTreeSet<String>, BTreeMap<String, Vec<String>>) {
    let mut list_nodes = BTreeSet::new();
    let mut first_values = BTreeMap::new();
    let mut rest_targets = BTreeMap::new();

    for block in &report.blocks {
        for quad in &block.quads {
            let subject = subject_id(&quad.subject);
            match quad.predicate.as_str() {
                RDF_FIRST => {
                    list_nodes.insert(subject.clone());
                    first_values.insert(subject, term_label(&quad.object));
                }
                RDF_REST => {
                    list_nodes.insert(subject.clone());
                    match &quad.object {
                        Term::NamedNode(node) if node.as_str() == RDF_NIL => {
                            rest_targets.insert(subject, RDF_NIL.to_string());
                        }
                        Term::BlankNode(node) => {
                            let target = node.to_string();
                            list_nodes.insert(target.clone());
                            rest_targets.insert(subject, target);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    let mut list_values = BTreeMap::new();
    for head in &list_nodes {
        let mut values = Vec::new();
        let mut current = head.as_str();
        let mut seen = BTreeSet::new();

        while seen.insert(current.to_string()) {
            if let Some(value) = first_values.get(current) {
                values.push(value.clone());
            }

            let Some(next) = rest_targets.get(current) else {
                break;
            };
            if next == RDF_NIL {
                break;
            }
            current = next;
        }

        if !values.is_empty() {
            list_values.insert(head.clone(), values);
        }
    }

    (list_nodes, list_values)
}

fn collect_primary_blank_nodes(
    report: &SemanticIndex,
    internal_property_shapes: &BTreeMap<String, String>,
    rdf_list_nodes: &BTreeSet<String>,
) -> BTreeMap<String, &'static str> {
    let mut blank_nodes = BTreeMap::new();
    for block in &report.blocks {
        for quad in &block.quads {
            if quad.predicate.as_str() != RDF_TYPE {
                continue;
            }
            let NamedOrBlankNode::BlankNode(subject) = &quad.subject else {
                continue;
            };
            let subject_id = subject.to_string();
            if internal_property_shapes.contains_key(&subject_id)
                || rdf_list_nodes.contains(&subject_id)
            {
                continue;
            }

            let Some(semantic_type) = semantic_type_from_rdf_type(&quad.object) else {
                continue;
            };
            if semantic_type != "node-shape" && semantic_type != "property-shape" {
                continue;
            }

            blank_nodes
                .entry(subject_id)
                .and_modify(|current| upgrade_semantic_type(current, semantic_type))
                .or_insert(semantic_type);
        }
    }
    blank_nodes
}

fn ensure_node(
    nodes: &mut BTreeMap<String, OntologyGraphNode>,
    id: &str,
    node_type: &str,
    block: &SemanticBlock,
) {
    nodes
        .entry(id.to_string())
        .and_modify(|node| {
            upgrade_node_type(&mut node.node_type, node_type);
            add_node_source(node, block);
            if is_datatype_iri(id) {
                upgrade_semantic_type_string(&mut node.semantic_type, "datatype");
            }
        })
        .or_insert_with(|| OntologyGraphNode {
            id: id.to_string(),
            label: clean_uri(id),
            node_type: node_type.to_string(),
            semantic_type: default_semantic_type(id, node_type).to_string(),
            full_uri: id.to_string(),
            comment: "None specified.".to_string(),
            rdf_types: Vec::new(),
            type_evidence: Vec::new(),
            sources: vec![source_metadata(block)],
            constraints: Vec::new(),
            badges: Vec::new(),
            equivalence_group: String::new(),
            inverse_properties: Vec::new(),
            property_chains: Vec::new(),
            domain: Vec::new(),
            range: Vec::new(),
            literal_values: Vec::new(),
            slot_facets: Vec::new(),
            constructs: Vec::new(),
        });
}

fn populate_construct_metadata(
    nodes: &mut BTreeMap<String, OntologyGraphNode>,
    edges: &mut BTreeSet<OntologyGraphEdge>,
    report: &SemanticIndex,
) {
    for construct in &report.ontology_projection.constructs {
        apply_projection_construct(nodes, edges, construct);
    }

    for node in nodes.values_mut() {
        node.badges.sort_by(|a, b| a.kind.cmp(&b.kind));
        node.badges.dedup_by(|a, b| a.kind == b.kind);
        node.inverse_properties.sort();
        node.inverse_properties.dedup();
        node.property_chains.sort_by(|a, b| a.id.cmp(&b.id));
        node.property_chains.dedup_by(|a, b| a.id == b.id);
    }
}

fn apply_projection_construct(
    nodes: &mut BTreeMap<String, OntologyGraphNode>,
    edges: &mut BTreeSet<OntologyGraphEdge>,
    construct: &OntologyConstruct,
) {
    let Some(subject_id) = projection_term_node_id(&construct.subject) else {
        return;
    };
    ensure_projection_node(
        nodes,
        &construct.subject,
        subject_semantic_hint(construct),
        &construct.provenance.source,
    );

    if let Some(node) = nodes.get_mut(&subject_id) {
        push_construct_detail(node, construct);
        add_construct_badge(node, construct);
    }

    match construct.kind {
        OntologyConstructKind::PropertyDomain | OntologyConstructKind::PropertyRange => {
            apply_property_domain_range_construct(nodes, edges, construct);
        }
        OntologyConstructKind::SubclassInclusion => {
            apply_binary_projection_edge(nodes, edges, construct, "subclass", Some("class"));
        }
        OntologyConstructKind::Membership => {
            apply_binary_projection_edge(nodes, edges, construct, "member", Some("class"));
        }
        OntologyConstructKind::Disjointness => {
            apply_binary_projection_edge(nodes, edges, construct, "disjoint", Some("class"));
        }
        OntologyConstructKind::EquivalenceGroup => {
            apply_equivalence_group_construct(nodes, construct);
        }
        OntologyConstructKind::InverseProperty => {
            apply_inverse_property_construct(nodes, edges, construct);
        }
        OntologyConstructKind::PropertyChain => {
            apply_property_chain_construct(nodes, construct);
        }
        OntologyConstructKind::PropertyCharacteristic => {
            if let Some(node) = nodes.get_mut(&subject_id) {
                upgrade_semantic_type_string(&mut node.semantic_type, "rdf-property");
                upgrade_node_type(&mut node.node_type, "owl");
            }
        }
        OntologyConstructKind::Restriction => {
            apply_restriction_construct(nodes, edges, construct);
        }
        OntologyConstructKind::ClassExpression => {
            apply_class_expression_construct(nodes, edges, construct);
        }
        OntologyConstructKind::ShapeOverlay => {
            apply_shape_overlay_construct(nodes, edges, construct);
        }
    }
}

fn apply_property_domain_range_construct(
    nodes: &mut BTreeMap<String, OntologyGraphNode>,
    edges: &mut BTreeSet<OntologyGraphEdge>,
    construct: &OntologyConstruct,
) {
    let Some(subject_id) = projection_term_node_id(&construct.subject) else {
        return;
    };
    let Some(object) = &construct.object else {
        return;
    };
    let Some(term_ref) = projection_term_ref(object) else {
        return;
    };

    if let Some(node) = nodes.get_mut(&subject_id) {
        upgrade_semantic_type_string(&mut node.semantic_type, "rdf-property");
        upgrade_node_type(&mut node.node_type, "owl");
        let bucket = if construct.kind == OntologyConstructKind::PropertyDomain {
            &mut node.domain
        } else {
            &mut node.range
        };
        if !bucket.contains(&term_ref) {
            bucket.push(term_ref.clone());
        }
    }

    if let Some(target_id) = projection_term_node_id(object) {
        let should_link_target = term_ref.kind == "class"
            || matches!(object.kind, OntologyProjectionTermKind::BlankNode);
        if should_link_target {
            let hint = if term_ref.kind == "class" {
                Some("class")
            } else {
                None
            };
            ensure_projection_node(nodes, object, hint, &construct.provenance.source);
            if matches!(object.kind, OntologyProjectionTermKind::BlankNode) {
                if let Some(target_node) = nodes.get_mut(&target_id) {
                    push_construct_detail(target_node, construct);
                }
            }
            edges.insert(OntologyGraphEdge {
                source: subject_id,
                target: target_id,
                label: if construct.kind == OntologyConstructKind::PropertyDomain {
                    "domain".to_string()
                } else {
                    "range".to_string()
                },
            });
        }
    }
}

fn apply_equivalence_group_construct(
    nodes: &mut BTreeMap<String, OntologyGraphNode>,
    construct: &OntologyConstruct,
) {
    let mut member_ids: Vec<_> = construct
        .members
        .iter()
        .filter_map(|member| projection_term_node_id(&member.term))
        .collect();
    member_ids.sort();
    member_ids.dedup();
    if member_ids.len() < 2 {
        return;
    }

    let group_id = format!("equivalence:{}", member_ids.join("|"));
    for member in &construct.members {
        let Some(member_id) = projection_term_node_id(&member.term) else {
            continue;
        };
        ensure_projection_node(nodes, &member.term, None, &member.source);
        if let Some(node) = nodes.get_mut(&member_id) {
            node.equivalence_group = group_id.clone();
            push_construct_detail(node, construct);
            add_construct_badge(node, construct);
        }
    }
}

fn apply_inverse_property_construct(
    nodes: &mut BTreeMap<String, OntologyGraphNode>,
    edges: &mut BTreeSet<OntologyGraphEdge>,
    construct: &OntologyConstruct,
) {
    let Some(subject_id) = projection_term_node_id(&construct.subject) else {
        return;
    };
    let Some(object) = &construct.object else {
        return;
    };
    let Some(object_id) = projection_term_node_id(object) else {
        return;
    };

    ensure_projection_node(
        nodes,
        object,
        Some("rdf-property"),
        &construct.provenance.source,
    );
    for (id, inverse) in [(&subject_id, &object_id), (&object_id, &subject_id)] {
        if let Some(node) = nodes.get_mut(id) {
            upgrade_semantic_type_string(&mut node.semantic_type, "rdf-property");
            upgrade_node_type(&mut node.node_type, "owl");
            node.inverse_properties.push(inverse.clone());
            push_construct_detail(node, construct);
            add_construct_badge(node, construct);
        }
    }

    edges.insert(OntologyGraphEdge {
        source: subject_id,
        target: object_id,
        label: "inverse".to_string(),
    });
}

fn apply_property_chain_construct(
    nodes: &mut BTreeMap<String, OntologyGraphNode>,
    construct: &OntologyConstruct,
) {
    let Some(subject_id) = projection_term_node_id(&construct.subject) else {
        return;
    };
    let members = construct
        .members
        .iter()
        .map(|member| projection_term_display(&member.term))
        .collect::<Vec<_>>();
    let chain = OntologyGraphPropertyChain {
        id: construct.id.clone(),
        members,
        source: construct
            .provenance
            .source
            .source_element_identifier
            .clone(),
    };

    if let Some(node) = nodes.get_mut(&subject_id) {
        upgrade_semantic_type_string(&mut node.semantic_type, "rdf-property");
        upgrade_node_type(&mut node.node_type, "owl");
        if !node.property_chains.contains(&chain) {
            node.property_chains.push(chain);
        }
    }
}

fn apply_restriction_construct(
    nodes: &mut BTreeMap<String, OntologyGraphNode>,
    edges: &mut BTreeSet<OntologyGraphEdge>,
    construct: &OntologyConstruct,
) {
    let Some(subject_id) = projection_term_node_id(&construct.subject) else {
        return;
    };
    if let Some(node) = nodes.get_mut(&subject_id) {
        upgrade_semantic_type_string(&mut node.semantic_type, "restriction");
        upgrade_node_type(&mut node.node_type, "owl");
    }

    if let Some(property) = &construct.property {
        if let Some(property_id) = projection_term_node_id(property) {
            ensure_projection_node(
                nodes,
                property,
                Some("rdf-property"),
                &construct.provenance.source,
            );
            edges.insert(OntologyGraphEdge {
                source: subject_id.clone(),
                target: property_id,
                label: "on property".to_string(),
            });
        }
    }

    if let Some(object) = &construct.object {
        let label = construct
            .restriction_kind
            .map(restriction_kind_name)
            .unwrap_or("restriction");
        add_projection_object_edge(nodes, edges, &subject_id, object, label, construct);
    }
}

fn apply_class_expression_construct(
    nodes: &mut BTreeMap<String, OntologyGraphNode>,
    edges: &mut BTreeSet<OntologyGraphEdge>,
    construct: &OntologyConstruct,
) {
    let Some(subject_id) = projection_term_node_id(&construct.subject) else {
        return;
    };
    if let Some(node) = nodes.get_mut(&subject_id) {
        upgrade_semantic_type_string(&mut node.semantic_type, "class-expression");
        upgrade_node_type(&mut node.node_type, "owl");
        node.label = construct
            .class_expression_kind
            .map(|kind| format!("{} class expression", class_expression_kind_label(kind)))
            .unwrap_or_else(|| "Class expression".to_string());
    }

    let label = construct
        .class_expression_kind
        .map(class_expression_kind_name)
        .unwrap_or("class-expression");
    for member in &construct.members {
        add_projection_object_edge(nodes, edges, &subject_id, &member.term, label, construct);
    }
}

fn apply_shape_overlay_construct(
    nodes: &mut BTreeMap<String, OntologyGraphNode>,
    edges: &mut BTreeSet<OntologyGraphEdge>,
    construct: &OntologyConstruct,
) {
    let Some(subject_id) = projection_term_node_id(&construct.subject) else {
        return;
    };
    if let Some(shape_kind) = construct.shape_overlay_kind {
        if let Some(node) = nodes.get_mut(&subject_id) {
            upgrade_semantic_type_string(
                &mut node.semantic_type,
                shape_overlay_kind_name(shape_kind),
            );
            upgrade_node_type(&mut node.node_type, "shacl");
        }
    }

    if let Some(object) = &construct.object {
        let target_hint = projection_term_semantic_hint(object);
        if let Some(object_id) = projection_term_node_id(object) {
            ensure_projection_node(nodes, object, target_hint, &construct.provenance.source);
            if let Some(node) = nodes.get_mut(&object_id) {
                push_construct_detail(node, construct);
            }
            edges.insert(OntologyGraphEdge {
                source: subject_id,
                target: object_id,
                label: construct_label(construct).to_lowercase(),
            });
        }
    }
}

fn apply_binary_projection_edge(
    nodes: &mut BTreeMap<String, OntologyGraphNode>,
    edges: &mut BTreeSet<OntologyGraphEdge>,
    construct: &OntologyConstruct,
    label: &str,
    object_hint: Option<&'static str>,
) {
    let Some(subject_id) = projection_term_node_id(&construct.subject) else {
        return;
    };
    let Some(object) = &construct.object else {
        return;
    };
    add_projection_object_edge(nodes, edges, &subject_id, object, label, construct);
    if let Some(object_id) = projection_term_node_id(object) {
        ensure_projection_node(nodes, object, object_hint, &construct.provenance.source);
        if let Some(node) = nodes.get_mut(&object_id) {
            push_construct_detail(node, construct);
            if mirrors_construct_badge_on_object(construct.kind) {
                add_construct_badge(node, construct);
            }
        }
    }
}

fn add_projection_object_edge(
    nodes: &mut BTreeMap<String, OntologyGraphNode>,
    edges: &mut BTreeSet<OntologyGraphEdge>,
    subject_id: &str,
    object: &OntologyProjectionTerm,
    label: &str,
    construct: &OntologyConstruct,
) {
    let Some(object_id) = projection_term_node_id(object) else {
        return;
    };
    let target_hint = projection_term_semantic_hint(object);
    ensure_projection_node(nodes, object, target_hint, &construct.provenance.source);
    edges.insert(OntologyGraphEdge {
        source: subject_id.to_string(),
        target: object_id,
        label: label.to_string(),
    });
}

fn ensure_projection_node(
    nodes: &mut BTreeMap<String, OntologyGraphNode>,
    term: &OntologyProjectionTerm,
    semantic_hint: Option<&'static str>,
    source: &OntologyProjectionSource,
) {
    let Some(id) = projection_term_node_id(term) else {
        return;
    };
    let source = source_metadata_from_projection(source);
    let node_type = semantic_hint
        .map(visual_node_type_for_semantic_type)
        .unwrap_or_else(|| {
            if is_datatype_iri(&id) {
                "generic"
            } else {
                "owl"
            }
        });
    nodes
        .entry(id.clone())
        .and_modify(|node| {
            upgrade_node_type(&mut node.node_type, node_type);
            if let Some(semantic_hint) = semantic_hint {
                upgrade_semantic_type_string(&mut node.semantic_type, semantic_hint);
            } else if is_datatype_iri(&id) {
                upgrade_semantic_type_string(&mut node.semantic_type, "datatype");
            }
            if !node.sources.contains(&source) {
                node.sources.push(source.clone());
            }
        })
        .or_insert_with(|| {
            let semantic_type = semantic_hint
                .or_else(|| projection_term_semantic_hint(term))
                .unwrap_or_else(|| default_semantic_type(&id, node_type))
                .to_string();
            OntologyGraphNode {
                id: id.clone(),
                label: projection_node_label(term, semantic_hint),
                node_type: visual_node_type_for_semantic_type(&semantic_type).to_string(),
                semantic_type,
                full_uri: id,
                comment: "None specified.".to_string(),
                rdf_types: Vec::new(),
                type_evidence: Vec::new(),
                sources: vec![source.clone()],
                constraints: Vec::new(),
                badges: Vec::new(),
                equivalence_group: String::new(),
                inverse_properties: Vec::new(),
                property_chains: Vec::new(),
                domain: Vec::new(),
                range: Vec::new(),
                literal_values: Vec::new(),
                slot_facets: Vec::new(),
                constructs: Vec::new(),
            }
        });
}

fn push_construct_detail(node: &mut OntologyGraphNode, construct: &OntologyConstruct) {
    let detail = construct_detail(construct);
    if !node
        .constructs
        .iter()
        .any(|existing| existing.id == detail.id)
    {
        node.constructs.push(detail);
    }
}

fn add_construct_badge(node: &mut OntologyGraphNode, construct: &OntologyConstruct) {
    let Some(badge) = construct_badge(construct) else {
        return;
    };
    if !node
        .badges
        .iter()
        .any(|existing| existing.kind == badge.kind)
    {
        node.badges.push(badge);
    }
}

fn mirrors_construct_badge_on_object(kind: OntologyConstructKind) -> bool {
    matches!(kind, OntologyConstructKind::Disjointness)
}

fn construct_detail(construct: &OntologyConstruct) -> OntologyGraphConstructDetail {
    OntologyGraphConstructDetail {
        id: construct.id.clone(),
        family: construct_family_name(construct.family).to_string(),
        kind: construct_kind_name(construct.kind).to_string(),
        label: construct_label(construct),
        subject: projection_term_display(&construct.subject),
        predicate: construct
            .predicate
            .as_ref()
            .map(projection_term_display)
            .unwrap_or_default(),
        object: construct
            .object
            .as_ref()
            .map(projection_term_display)
            .unwrap_or_default(),
        property: construct
            .property
            .as_ref()
            .map(projection_term_display)
            .unwrap_or_default(),
        members: construct
            .members
            .iter()
            .map(|member| projection_term_display(&member.term))
            .collect(),
        source: source_metadata_from_projection(&construct.provenance.source),
        badge: construct_badge(construct),
    }
}

fn source_metadata_from_projection(source: &OntologyProjectionSource) -> OntologyGraphSource {
    OntologyGraphSource {
        source: source.source_element_identifier.clone(),
        source_name: source.source_name.clone(),
        file_path: source.file_path.clone(),
        line_number: source.line_number,
        kind: source.source_block.clone(),
        link: source_html_link(&source.source_element_identifier, &source.file_path),
    }
}

fn symbol_badge(symbol: &OntologySymbol) -> OntologyGraphBadge {
    OntologyGraphBadge {
        kind: symbol.concept_name.clone(),
        symbol: symbol.rendered_unicode_character.clone(),
        code_point: symbol.raw_unicode_code_point.clone(),
        label: if symbol.accessible_label.is_empty() {
            symbol.tooltip.clone()
        } else {
            symbol.accessible_label.clone()
        },
    }
}

fn construct_badge(construct: &OntologyConstruct) -> Option<OntologyGraphBadge> {
    let symbol = construct.symbol.as_ref()?;
    let mut badge = symbol_badge(symbol);
    match construct.kind {
        OntologyConstructKind::SubclassInclusion => {
            badge.label = "Subclass".to_string();
        }
        OntologyConstructKind::Membership => {
            badge.label = "Member of".to_string();
        }
        _ => {}
    }
    Some(badge)
}

fn subject_semantic_hint(construct: &OntologyConstruct) -> Option<&'static str> {
    match construct.kind {
        OntologyConstructKind::PropertyDomain
        | OntologyConstructKind::PropertyRange
        | OntologyConstructKind::InverseProperty
        | OntologyConstructKind::PropertyChain
        | OntologyConstructKind::PropertyCharacteristic => Some("rdf-property"),
        OntologyConstructKind::SubclassInclusion | OntologyConstructKind::Disjointness => {
            Some("class")
        }
        OntologyConstructKind::Restriction => Some("restriction"),
        OntologyConstructKind::ClassExpression => Some("class-expression"),
        OntologyConstructKind::ShapeOverlay => {
            construct.shape_overlay_kind.map(shape_overlay_kind_name)
        }
        OntologyConstructKind::Membership | OntologyConstructKind::EquivalenceGroup => None,
    }
}

fn projection_term_node_id(term: &OntologyProjectionTerm) -> Option<String> {
    match term.kind {
        OntologyProjectionTermKind::Iri | OntologyProjectionTermKind::BlankNode => {
            Some(term.value.clone())
        }
        OntologyProjectionTermKind::Literal => None,
    }
}

fn projection_term_display(term: &OntologyProjectionTerm) -> String {
    if !term.label.is_empty() && term.label != "anonymous node" {
        term.label.clone()
    } else if term.kind == OntologyProjectionTermKind::Iri {
        clean_uri(&term.value)
    } else {
        term.value.clone()
    }
}

fn projection_node_label(
    term: &OntologyProjectionTerm,
    semantic_hint: Option<&'static str>,
) -> String {
    match (term.kind, semantic_hint) {
        (OntologyProjectionTermKind::BlankNode, Some("restriction")) => "Restriction".to_string(),
        (OntologyProjectionTermKind::BlankNode, Some("class-expression")) => {
            "Class expression".to_string()
        }
        (OntologyProjectionTermKind::BlankNode, Some("node-shape")) => "Node shape".to_string(),
        (OntologyProjectionTermKind::BlankNode, Some("property-shape")) => {
            "Property shape".to_string()
        }
        _ => projection_term_display(term),
    }
}

fn projection_term_semantic_hint(term: &OntologyProjectionTerm) -> Option<&'static str> {
    match term.kind {
        OntologyProjectionTermKind::Literal => Some("literal"),
        OntologyProjectionTermKind::Iri if is_datatype_iri(&term.value) => Some("datatype"),
        _ => None,
    }
}

fn projection_term_ref(term: &OntologyProjectionTerm) -> Option<OntologyGraphTermRef> {
    match term.kind {
        OntologyProjectionTermKind::Iri => Some(OntologyGraphTermRef {
            label: projection_term_display(term),
            iri: term.value.clone(),
            kind: classify_named_term(&term.value).to_string(),
        }),
        OntologyProjectionTermKind::Literal => Some(OntologyGraphTermRef {
            label: projection_term_display(term),
            iri: String::new(),
            kind: "literal".to_string(),
        }),
        OntologyProjectionTermKind::BlankNode => Some(OntologyGraphTermRef {
            label: projection_term_display(term),
            iri: term.value.clone(),
            kind: "resource".to_string(),
        }),
    }
}

fn construct_family_name(family: OntologyConstructFamily) -> &'static str {
    match family {
        OntologyConstructFamily::PropertyDomainRange => "property-domain-range",
        OntologyConstructFamily::SubclassMembership => "subclass-membership",
        OntologyConstructFamily::DisjointEquivalenceInverse => "disjoint-equivalence-inverse",
        OntologyConstructFamily::PropertyChain => "property-chain",
        OntologyConstructFamily::PropertyCharacteristic => "property-characteristic",
        OntologyConstructFamily::Restriction => "restriction",
        OntologyConstructFamily::ClassExpression => "class-expression",
        OntologyConstructFamily::ShapeOverlay => "shape-overlay",
    }
}

fn construct_kind_name(kind: OntologyConstructKind) -> &'static str {
    match kind {
        OntologyConstructKind::PropertyDomain => "property-domain",
        OntologyConstructKind::PropertyRange => "property-range",
        OntologyConstructKind::SubclassInclusion => "subclass-inclusion",
        OntologyConstructKind::Membership => "membership",
        OntologyConstructKind::Disjointness => "disjointness",
        OntologyConstructKind::EquivalenceGroup => "equivalence-group",
        OntologyConstructKind::InverseProperty => "inverse-property",
        OntologyConstructKind::PropertyChain => "property-chain",
        OntologyConstructKind::PropertyCharacteristic => "property-characteristic",
        OntologyConstructKind::Restriction => "restriction",
        OntologyConstructKind::ClassExpression => "class-expression",
        OntologyConstructKind::ShapeOverlay => "shape-overlay",
    }
}

fn construct_label(construct: &OntologyConstruct) -> String {
    match construct.kind {
        OntologyConstructKind::PropertyDomain => "Property domain".to_string(),
        OntologyConstructKind::PropertyRange => "Property range".to_string(),
        OntologyConstructKind::SubclassInclusion => "Subclass inclusion".to_string(),
        OntologyConstructKind::Membership => "Membership".to_string(),
        OntologyConstructKind::Disjointness => "Disjointness".to_string(),
        OntologyConstructKind::EquivalenceGroup => "Equivalence group".to_string(),
        OntologyConstructKind::InverseProperty => "Inverse property".to_string(),
        OntologyConstructKind::PropertyChain => "Property chain".to_string(),
        OntologyConstructKind::PropertyCharacteristic => construct
            .property_characteristic
            .map(property_characteristic_label)
            .unwrap_or("Property characteristic")
            .to_string(),
        OntologyConstructKind::Restriction => construct
            .restriction_kind
            .map(restriction_kind_label)
            .unwrap_or("Restriction")
            .to_string(),
        OntologyConstructKind::ClassExpression => construct
            .class_expression_kind
            .map(class_expression_kind_label)
            .unwrap_or("Class expression")
            .to_string(),
        OntologyConstructKind::ShapeOverlay => construct
            .shape_overlay_kind
            .map(shape_overlay_kind_label)
            .unwrap_or("SHACL overlay")
            .to_string(),
    }
}

fn property_characteristic_label(characteristic: OntologyPropertyCharacteristic) -> &'static str {
    match characteristic {
        OntologyPropertyCharacteristic::Functional => "Functional property",
        OntologyPropertyCharacteristic::InverseFunctional => "Inverse functional property",
        OntologyPropertyCharacteristic::Symmetric => "Symmetric property",
        OntologyPropertyCharacteristic::Asymmetric => "Asymmetric property",
        OntologyPropertyCharacteristic::Reflexive => "Reflexive property",
        OntologyPropertyCharacteristic::Irreflexive => "Irreflexive property",
        OntologyPropertyCharacteristic::Transitive => "Transitive property",
    }
}

fn restriction_kind_name(kind: OntologyRestrictionKind) -> &'static str {
    match kind {
        OntologyRestrictionKind::Universal => "universal",
        OntologyRestrictionKind::Existential => "existential",
        OntologyRestrictionKind::HasValue => "has-value",
        OntologyRestrictionKind::Cardinality => "cardinality",
        OntologyRestrictionKind::MinCardinality => "min-cardinality",
        OntologyRestrictionKind::MaxCardinality => "max-cardinality",
        OntologyRestrictionKind::QualifiedCardinality => "qualified-cardinality",
        OntologyRestrictionKind::MinQualifiedCardinality => "min-qualified-cardinality",
        OntologyRestrictionKind::MaxQualifiedCardinality => "max-qualified-cardinality",
        OntologyRestrictionKind::OnClass => "on-class",
        OntologyRestrictionKind::OnDataRange => "on-data-range",
    }
}

fn restriction_kind_label(kind: OntologyRestrictionKind) -> &'static str {
    match kind {
        OntologyRestrictionKind::Universal => "Universal restriction",
        OntologyRestrictionKind::Existential => "Existential restriction",
        OntologyRestrictionKind::HasValue => "Has-value restriction",
        OntologyRestrictionKind::Cardinality => "Cardinality restriction",
        OntologyRestrictionKind::MinCardinality => "Minimum cardinality restriction",
        OntologyRestrictionKind::MaxCardinality => "Maximum cardinality restriction",
        OntologyRestrictionKind::QualifiedCardinality => "Qualified cardinality restriction",
        OntologyRestrictionKind::MinQualifiedCardinality => {
            "Minimum qualified cardinality restriction"
        }
        OntologyRestrictionKind::MaxQualifiedCardinality => {
            "Maximum qualified cardinality restriction"
        }
        OntologyRestrictionKind::OnClass => "Restriction target class",
        OntologyRestrictionKind::OnDataRange => "Restriction target data range",
    }
}

fn class_expression_kind_name(kind: OntologyClassExpressionKind) -> &'static str {
    match kind {
        OntologyClassExpressionKind::Intersection => "intersection",
        OntologyClassExpressionKind::Union => "union",
        OntologyClassExpressionKind::Complement => "complement",
    }
}

fn class_expression_kind_label(kind: OntologyClassExpressionKind) -> &'static str {
    match kind {
        OntologyClassExpressionKind::Intersection => "Intersection",
        OntologyClassExpressionKind::Union => "Union",
        OntologyClassExpressionKind::Complement => "Complement",
    }
}

fn shape_overlay_kind_name(kind: OntologyShapeOverlayKind) -> &'static str {
    match kind {
        OntologyShapeOverlayKind::NodeShape => "node-shape",
        OntologyShapeOverlayKind::PropertyShape => "property-shape",
    }
}

fn shape_overlay_kind_label(kind: OntologyShapeOverlayKind) -> &'static str {
    match kind {
        OntologyShapeOverlayKind::NodeShape => "SHACL node shape overlay",
        OntologyShapeOverlayKind::PropertyShape => "SHACL property shape overlay",
    }
}

fn add_node_source(node: &mut OntologyGraphNode, block: &SemanticBlock) {
    let source = source_metadata(block);
    if !node.sources.contains(&source) {
        node.sources.push(source);
    }
}

fn source_metadata(block: &SemanticBlock) -> OntologyGraphSource {
    OntologyGraphSource {
        source: block.source.clone(),
        source_name: block.source_name.clone(),
        file_path: block.file_path.clone(),
        line_number: block.line_number,
        kind: block.kind.as_str().to_string(),
        link: source_html_link(&block.source, &block.file_path),
    }
}

fn source_html_link(source: &str, file_path: &str) -> String {
    let target = if source.contains(".md") {
        source
    } else if !file_path.is_empty() {
        file_path
    } else {
        source
    };
    target.replace(".md", ".html")
}

fn record_type_evidence(node: &mut OntologyGraphNode, term: &Term, block: &SemanticBlock) {
    let label = term_label(term);
    if !node.rdf_types.contains(&label) {
        node.rdf_types.push(label.clone());
    }

    let evidence = OntologyGraphTypeEvidence {
        iri: term_iri(term).unwrap_or_else(|| label.clone()),
        label,
        source: source_metadata(block),
    };
    if !node.type_evidence.contains(&evidence) {
        node.type_evidence.push(evidence);
    }

    if let Some(semantic_type) = semantic_type_from_rdf_type(term) {
        upgrade_semantic_type_string(&mut node.semantic_type, semantic_type);
        upgrade_node_type(
            &mut node.node_type,
            visual_node_type_for_semantic_type(semantic_type),
        );
    }
}

fn promote_typed_named_individuals(nodes: &mut BTreeMap<String, OntologyGraphNode>) {
    let declared_classes: BTreeSet<String> = nodes
        .iter()
        .filter(|(_, node)| node.semantic_type == "class")
        .map(|(id, _)| id.clone())
        .collect();

    if declared_classes.is_empty() {
        return;
    }

    for node in nodes.values_mut() {
        if node.semantic_type != "resource" || is_blank_node_id(&node.id) {
            continue;
        }

        if node
            .type_evidence
            .iter()
            .any(|evidence| declared_classes.contains(&evidence.iri))
        {
            node.semantic_type = "named-individual".to_string();
        }
    }
}

fn append_property_shape_constraint(
    nodes: &mut BTreeMap<String, OntologyGraphNode>,
    owner_id: &str,
    predicate: &str,
    object: &Term,
    rdf_list_values: &BTreeMap<String, Vec<String>>,
) {
    let Some(node) = nodes.get_mut(owner_id) else {
        return;
    };

    let property = match predicate {
        SH_PATH => "property",
        SH_IN => "allowed values",
        _ if is_constraint_predicate(predicate) => "property rule",
        RDF_TYPE => return,
        _ => return,
    };
    let value = match predicate {
        SH_PATH => format!("{} shape", term_label(object)),
        SH_IN => term_label_with_lists(object, rdf_list_values),
        _ => format!(
            "{} {}",
            clean_uri(predicate),
            term_label_with_lists(object, rdf_list_values)
        ),
    };

    if !node
        .constraints
        .iter()
        .any(|constraint| constraint.property == property && constraint.value == value)
    {
        node.constraints.push(OntologyGraphConstraint {
            property: property.to_string(),
            value,
        });
    }
}

fn apply_blank_node_labels(nodes: &mut BTreeMap<String, OntologyGraphNode>) {
    for node in nodes.values_mut() {
        if !node.id.starts_with("_:") || !node.label.starts_with("_:") {
            continue;
        }

        if let Some(path) = node
            .constraints
            .iter()
            .find(|constraint| constraint.property == "path")
            .map(|constraint| constraint.value.as_str())
        {
            node.label = format!("{path} property shape");
            node.comment = "Anonymous SHACL property shape.".to_string();
        } else if node
            .rdf_types
            .iter()
            .any(|rdf_type| rdf_type == "PropertyShape")
        {
            node.label = "Property shape".to_string();
            node.comment = "Anonymous SHACL property shape.".to_string();
        } else if node.node_type == "shacl" {
            node.label = "Anonymous SHACL node".to_string();
        } else {
            node.label = "Anonymous RDF node".to_string();
        }
    }
}

fn upgrade_node_type(current: &mut String, candidate: &str) {
    if node_type_rank(candidate) > node_type_rank(current) {
        *current = candidate.to_string();
    }
}

fn node_type_rank(node_type: &str) -> u8 {
    match node_type {
        "literal" => 4,
        "shacl" => 3,
        "owl" => 2,
        "generic" => 1,
        _ => 0,
    }
}

fn semantic_type_from_rdf_type(term: &Term) -> Option<&'static str> {
    match term {
        Term::NamedNode(node) => match node.as_str() {
            OWL_CLASS | RDFS_CLASS => Some("class"),
            OWL_OBJECT_PROPERTY => Some("object-property"),
            OWL_DATATYPE_PROPERTY => Some("datatype-property"),
            RDF_PROPERTY => Some("rdf-property"),
            OWL_NAMED_INDIVIDUAL => Some("named-individual"),
            SH_NODE_SHAPE => Some("node-shape"),
            SH_PROPERTY_SHAPE => Some("property-shape"),
            RDFS_DATATYPE => Some("datatype"),
            _ => None,
        },
        _ => None,
    }
}

fn default_semantic_type(id: &str, node_type: &str) -> &'static str {
    if node_type == "literal" {
        "literal"
    } else if is_datatype_iri(id) {
        "datatype"
    } else {
        "resource"
    }
}

fn upgrade_semantic_type(current: &mut &'static str, candidate: &'static str) {
    if semantic_type_rank(candidate) > semantic_type_rank(*current) {
        *current = candidate;
    }
}

fn upgrade_semantic_type_string(current: &mut String, candidate: &'static str) {
    if semantic_type_rank(candidate) > semantic_type_rank(current.as_str()) {
        *current = candidate.to_string();
    }
}

fn semantic_type_rank(semantic_type: &str) -> u8 {
    match semantic_type {
        "literal" => 10,
        "node-shape" | "property-shape" => 9,
        "restriction" | "class-expression" => 8,
        "object-property" | "datatype-property" => 7,
        "class" => 6,
        "rdf-property" => 5,
        "named-individual" => 4,
        "datatype" => 3,
        "resource" => 1,
        _ => 0,
    }
}

fn visual_node_type_for_semantic_type(semantic_type: &str) -> &'static str {
    match semantic_type {
        "literal" => "literal",
        "node-shape" | "property-shape" => "shacl",
        "class" | "rdf-property" | "object-property" | "datatype-property" | "named-individual"
        | "restriction" | "class-expression" => "owl",
        _ => "generic",
    }
}

fn subject_id(subject: &NamedOrBlankNode) -> String {
    match subject {
        NamedOrBlankNode::NamedNode(node) => node.as_str().to_string(),
        NamedOrBlankNode::BlankNode(node) => node.to_string(),
    }
}

fn term_label(term: &Term) -> String {
    match term {
        Term::NamedNode(node) => clean_uri(node.as_str()),
        Term::BlankNode(_) => "anonymous node".to_string(),
        Term::Literal(literal) => literal.value().to_string(),
    }
}

fn term_iri(term: &Term) -> Option<String> {
    match term {
        Term::NamedNode(node) => Some(node.as_str().to_string()),
        Term::BlankNode(node) => Some(node.to_string()),
        Term::Literal(_) => None,
    }
}

fn classify_named_term(iri: &str) -> &'static str {
    if is_datatype_iri(iri) || iri == RDFS_LITERAL {
        "datatype"
    } else {
        "class"
    }
}

fn term_label_with_lists(term: &Term, rdf_list_values: &BTreeMap<String, Vec<String>>) -> String {
    match term {
        Term::BlankNode(node) => rdf_list_values
            .get(&node.to_string())
            .map(|values| values.join(", "))
            .unwrap_or_else(|| "anonymous node".to_string()),
        _ => term_label(term),
    }
}

pub fn clean_uri(value: &str) -> String {
    value
        .trim_matches(|c| c == '<' || c == '>')
        .rsplit(['/', '#'])
        .find(|part| !part.is_empty())
        .unwrap_or(value)
        .to_string()
}

fn is_constraint_predicate(predicate: &str) -> bool {
    matches!(
        predicate,
        "http://www.w3.org/ns/shacl#minCount"
            | "http://www.w3.org/ns/shacl#maxCount"
            | SH_DATATYPE
            | "http://www.w3.org/ns/shacl#targetClass"
            | "http://www.w3.org/ns/shacl#class"
            | "http://www.w3.org/ns/shacl#nodeKind"
            | "http://www.w3.org/ns/shacl#path"
            | "http://www.w3.org/ns/shacl#pattern"
            | "http://www.w3.org/ns/shacl#in"
    )
}

fn is_projection_construct_predicate(predicate: &str) -> bool {
    matches!(
        predicate,
        RDFS_DOMAIN
            | RDFS_RANGE
            | RDFS_SUBCLASS_OF
            | OWL_DISJOINT_WITH
            | OWL_EQUIVALENT_CLASS
            | OWL_EQUIVALENT_PROPERTY
            | OWL_SAME_AS
            | OWL_INVERSE_OF
            | OWL_PROPERTY_CHAIN_AXIOM
            | OWL_ON_PROPERTY
            | OWL_ALL_VALUES_FROM
            | OWL_SOME_VALUES_FROM
            | OWL_HAS_VALUE
            | OWL_CARDINALITY
            | OWL_MIN_CARDINALITY
            | OWL_MAX_CARDINALITY
            | OWL_QUALIFIED_CARDINALITY
            | OWL_MIN_QUALIFIED_CARDINALITY
            | OWL_MAX_QUALIFIED_CARDINALITY
            | OWL_ON_CLASS
            | OWL_ON_DATA_RANGE
            | OWL_INTERSECTION_OF
            | OWL_UNION_OF
            | OWL_COMPLEMENT_OF
    )
}

fn is_primary_graph_node(node: &OntologyGraphNode) -> bool {
    if is_hidden_primary_graph_iri(&node.id) || is_boilerplate_label(&node.label) {
        return false;
    }

    if node.semantic_type == "literal" || node.node_type == "literal" {
        return false;
    }

    if is_datatype_iri(&node.id) {
        return false;
    }

    if is_blank_node_id(&node.id)
        && !matches!(
            node.semantic_type.as_str(),
            "node-shape" | "property-shape" | "restriction" | "class-expression"
        )
    {
        return false;
    }

    true
}

fn is_boilerplate_label(label: &str) -> bool {
    matches!(label, "w3.orgClass")
}

fn is_hidden_primary_graph_iri(iri: &str) -> bool {
    matches!(
        iri,
        RDFS_CLASS
            | RDFS_DATATYPE
            | RDFS_LITERAL
            | RDFS_RESOURCE
            | OWL_CLASS
            | OWL_NAMED_INDIVIDUAL
            | OWL_ONTOLOGY
            | OWL_THING
            | RDF_PROPERTY
            | RDF_LIST
            | OWL_OBJECT_PROPERTY
            | OWL_DATATYPE_PROPERTY
            | SH_NODE_SHAPE
            | SH_PROPERTY_SHAPE
    )
}

fn is_blank_node_id(id: &str) -> bool {
    id.starts_with("_:")
}

fn is_datatype_iri(iri: &str) -> bool {
    iri.starts_with(XSD_PREFIX) || iri == RDFS_DATATYPE
}
