use crate::semantic_contract::{
    OntologyClassExpressionKind, OntologyConstruct, OntologyConstructFamily, OntologyConstructKind,
    OntologyProjectionSource, OntologyProjectionTerm, OntologyProjectionTermKind,
    OntologyPropertyCharacteristic, OntologyRestrictionKind, OntologyShapeOverlayKind,
    OntologySymbol, SemanticBlock, SemanticBlockKind, SemanticIndex,
};
use maud::{html, Markup, PreEscaped};
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
struct OntologyGraphData {
    nodes: Vec<OntologyGraphNode>,
    edges: Vec<OntologyGraphEdge>,
}

#[derive(Debug, Clone, Serialize)]
struct OntologyGraphNode {
    id: String,
    label: String,
    #[serde(rename = "type")]
    node_type: String,
    semantic_type: String,
    full_uri: String,
    comment: String,
    rdf_types: Vec<String>,
    type_evidence: Vec<OntologyGraphTypeEvidence>,
    sources: Vec<OntologyGraphSource>,
    constraints: Vec<OntologyGraphConstraint>,
    badges: Vec<OntologyGraphBadge>,
    equivalence_group: String,
    inverse_properties: Vec<String>,
    property_chains: Vec<OntologyGraphPropertyChain>,
    domain: Vec<OntologyGraphTermRef>,
    range: Vec<OntologyGraphTermRef>,
    literal_values: Vec<OntologyGraphLiteralValue>,
    slot_facets: Vec<OntologyGraphSlotFacet>,
    constructs: Vec<OntologyGraphConstructDetail>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct OntologyGraphTermRef {
    label: String,
    iri: String,
    kind: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct OntologyGraphLiteralValue {
    predicate: String,
    value: String,
    source: OntologyGraphSource,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct OntologyGraphBadge {
    kind: String,
    symbol: String,
    code_point: String,
    label: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct OntologyGraphPropertyChain {
    id: String,
    members: Vec<String>,
    source: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct OntologyGraphSlotFacet {
    slot_label: String,
    slot_iri: String,
    slot_kind: String,
    target_class_label: String,
    target_class_iri: String,
    source_shape_label: String,
    source_shape_iri: String,
    source: OntologyGraphSource,
    facets: Vec<OntologyGraphSlotFacetValue>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct OntologyGraphSlotFacetValue {
    name: String,
    value: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct OntologyGraphConstructDetail {
    id: String,
    family: String,
    kind: String,
    label: String,
    subject: String,
    predicate: String,
    object: String,
    property: String,
    members: Vec<String>,
    source: OntologyGraphSource,
    badge: Option<OntologyGraphBadge>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct OntologyGraphTypeEvidence {
    iri: String,
    label: String,
    source: OntologyGraphSource,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct OntologyGraphSource {
    source: String,
    source_name: String,
    file_path: String,
    line_number: usize,
    kind: String,
    link: String,
}

#[derive(Debug, Clone, Serialize)]
struct OntologyGraphConstraint {
    property: String,
    value: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct OntologyGraphEdge {
    source: String,
    target: String,
    label: String,
}

pub fn render(report: &SemanticIndex, nav_prefix: &str) -> Markup {
    let graph_data = build_graph_data(report);
    let graph_json = serde_json::to_string(&graph_data)
        .unwrap_or_else(|_| "{\"nodes\":[],\"edges\":[]}".to_string())
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('&', "\\u0026");

    let content = html! {
        style {
            (PreEscaped(ONTOLOGY_GRAPH_CSS))
        }
        div class="ontology-page" {
            @if report.blocks.is_empty() {
                div class="ontology-empty-state" {
                    p class="text-gray-500 italic" { "No ontology or SHACL RDF/Turtle content found." }
                    a href={(nav_prefix) "ontologies.ttl"} class="ontology-empty-download-link" {
                        "Download .ttl"
                    }
                }
            } @else {
                section class="ontology-graph-panel" aria-label="Ontology graph explorer" {
                    div class="ontology-graph-canvas" {
                        div class="ontology-graph-legend" aria-label="Ontology graph legend and filters" {
                            div class="ontology-legend-title" { "Types" }
                            div class="ontology-legend-grid ontology-color-key" {
                                div class="ontology-legend-key-item" { span class="ontology-dot ontology-dot-class" {} "Class" }
                                div class="ontology-legend-key-item" { span class="ontology-dot ontology-dot-object-property" {} "Object prop." }
                                div class="ontology-legend-key-item" { span class="ontology-dot ontology-dot-datatype-property" {} "Datatype prop." }
                                div class="ontology-legend-key-item" { span class="ontology-dot ontology-dot-rdf-property" {} "RDF prop." }
                                div class="ontology-legend-key-item" { span class="ontology-dot ontology-dot-named-individual" {} "Individual" }
                                div class="ontology-legend-key-item" { span class="ontology-dot ontology-dot-datatype" {} "Datatype" }
                                div class="ontology-legend-key-item" { span class="ontology-dot ontology-dot-restriction" {} "Restriction" }
                                div class="ontology-legend-key-item" { span class="ontology-dot ontology-dot-class-expression" {} "Class expr." }
                                div class="ontology-legend-key-item" { span class="ontology-dot ontology-dot-node-shape" {} "Node shape" }
                                div class="ontology-legend-key-item" { span class="ontology-dot ontology-dot-property-shape" {} "Property shape" }
                                div class="ontology-legend-key-item" { span class="ontology-dot ontology-dot-resource" {} "Resource" }
                            }
                            div class="ontology-legend-title ontology-legend-title-secondary" { "Show" }
                            div class="ontology-legend-grid" {
                                button type="button" class="ontology-legend-item ontology-filter-toggle is-active" data-filter-category="role" data-filter-value="ontology-term" aria-pressed="true" { span class="ontology-dot ontology-dot-class" {} "Terms" }
                                button type="button" class="ontology-legend-item ontology-filter-toggle is-active" data-filter-category="role" data-filter-value="property" aria-pressed="true" { span class="ontology-dot ontology-dot-object-property" {} "Properties" }
                                button type="button" class="ontology-legend-item ontology-filter-toggle is-active" data-filter-category="role" data-filter-value="shacl-shape" aria-pressed="true" { span class="ontology-dot ontology-dot-node-shape" {} "SHACL shapes" }
                                button type="button" class="ontology-legend-item ontology-filter-toggle is-active" data-filter-category="role" data-filter-value="resource" aria-pressed="true" { span class="ontology-dot ontology-dot-resource" {} "Resources" }
                                button type="button" class="ontology-legend-item ontology-filter-toggle is-active" data-filter-category="role" data-filter-value="relation" aria-pressed="true" { span class="ontology-edge-sample" {} "Relation" }
                                button type="button" class="ontology-legend-item ontology-filter-toggle" data-filter-category="role" data-filter-value="external-reference" aria-pressed="false" { span class="ontology-dot ontology-dot-external-reference" {} "External refs" }
                            }
                            div class="ontology-legend-title ontology-legend-title-secondary" { "Constructs" }
                            div class="ontology-legend-grid" {
                                button type="button" class="ontology-legend-item ontology-filter-toggle is-active" data-filter-category="construct" data-filter-value="domain-range" aria-pressed="true" { span class="ontology-legend-symbol" { "D/R" } "Domain/range" }
                                button type="button" class="ontology-legend-item ontology-filter-toggle is-active" data-filter-category="construct" data-filter-value="subclass" aria-pressed="true" { span class="ontology-legend-symbol" { "⊆" } "Subclass" }
                                button type="button" class="ontology-legend-item ontology-filter-toggle is-active" data-filter-category="construct" data-filter-value="membership" aria-pressed="true" { span class="ontology-legend-symbol" { "∈" } "Membership" }
                                button type="button" class="ontology-legend-item ontology-filter-toggle is-active" data-filter-category="construct" data-filter-value="disjoint" aria-pressed="true" { span class="ontology-legend-symbol" { "⟂" } "Disjoint" }
                                button type="button" class="ontology-legend-item ontology-filter-toggle is-active" data-filter-category="construct" data-filter-value="equivalence" aria-pressed="true" { span class="ontology-legend-symbol" { "⇔" } "Equivalence" }
                                button type="button" class="ontology-legend-item ontology-filter-toggle is-active" data-filter-category="construct" data-filter-value="inverse" aria-pressed="true" { span class="ontology-legend-symbol" { "⟲" } "Inverse" }
                                button type="button" class="ontology-legend-item ontology-filter-toggle is-active" data-filter-category="construct" data-filter-value="property-chain" aria-pressed="true" { span class="ontology-legend-symbol" { "∘" } "Property chain" }
                                button type="button" class="ontology-legend-item ontology-filter-toggle is-active" data-filter-category="construct" data-filter-value="property-characteristic" aria-pressed="true" { span class="ontology-legend-symbol" { "→" } "Property char." }
                                button type="button" class="ontology-legend-item ontology-filter-toggle is-active" data-filter-category="construct" data-filter-value="restriction" aria-pressed="true" { span class="ontology-legend-symbol" { "∀" } "Restriction" }
                                button type="button" class="ontology-legend-item ontology-filter-toggle is-active" data-filter-category="construct" data-filter-value="class-expression" aria-pressed="true" { span class="ontology-legend-symbol" { "∩" } "Class expr." }
                                button type="button" class="ontology-legend-item ontology-filter-toggle is-active" data-filter-category="construct" data-filter-value="shape-overlay" aria-pressed="true" { span class="ontology-legend-symbol" { "SH" } "SHACL overlay" }
                            }
                            div class="ontology-legend-title ontology-legend-title-secondary" { "Origin" }
                            div class="ontology-legend-grid ontology-legend-origin-grid" {
                                button type="button" class="ontology-legend-item ontology-filter-toggle is-active" data-filter-category="origin" data-filter-value="authored" aria-pressed="true" { span class="ontology-origin-dot ontology-origin-authored" {} "Defined" }
                                button type="button" class="ontology-legend-item ontology-filter-toggle is-active" data-filter-category="origin" data-filter-value="registry" aria-pressed="true" { span class="ontology-origin-dot ontology-origin-registry" {} "Registry" }
                                button type="button" class="ontology-legend-item ontology-filter-toggle is-active" data-filter-category="origin" data-filter-value="construct" aria-pressed="true" { span class="ontology-origin-dot ontology-origin-construct" {} "Constructs" }
                            }
                        }
                        svg id="ontology-graph-svg" role="img" aria-label="Ontology and SHACL relationship graph" {}
                    }
                    aside class="ontology-graph-sidebar" {
                        div class="ontology-search-panel" {
                            input id="ontology-graph-search"
                                type="search"
                                placeholder="Search kind, domain/range, sources, SHACL, badges"
                                class="ontology-graph-search"
                                oninput="filterOntologyGraph(this.value)";
                            ul id="ontology-graph-results" class="ontology-graph-results" {}
                        }
                        div class="ontology-inspector-header" {
                            h2 id="ontology-inspector-title" { "Node Inspector" }
                            button id="ontology-inspector-clear" type="button" onclick="clearOntologySelection()" aria-label="Clear selection" { "x" }
                        }
                        div id="ontology-inspector-body" class="ontology-inspector-body" {
                            p class="text-gray-500 italic m-0" {
                                "Search or select a graph node to inspect URI, RDF type, comments, and SHACL constraints."
                            }
                        }
                        div class="ontology-sidebar-summary" aria-label="Ontology graph summary" {
                            span class="ontology-summary-entry" { "Ont " strong { (report.summary.ontology_blocks) } }
                            span class="ontology-summary-entry" { "Shapes " strong { (report.summary.shape_blocks) } }
                            span class="ontology-summary-entry" { "Quads " strong { (report.summary.total_quads) } }
                            span class="ontology-summary-entry" { "Blocks " strong { (report.summary.total_blocks) } }
                            a href={(nav_prefix) "ontologies.ttl"} class="ontology-summary-entry ontology-footer-download" title="Download ontologies.ttl" {
                                "Download .ttl"
                            }
                        }
                    }
                }
            }
        }
        script src="https://d3js.org/d3.v7.min.js" {}
        script {
            "const ontologyGraphData = ";
            (PreEscaped(graph_json));
            ";"
        }
        script {
            (PreEscaped(ONTOLOGY_GRAPH_JS))
        }
    };

    crate::html::layouts::base("Ontologies", content, nav_prefix)
}

fn build_graph_data(report: &SemanticIndex) -> OntologyGraphData {
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
        kind: source.block_kind.clone(),
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
        "literal" => 9,
        "node-shape" | "property-shape" => 8,
        "object-property" | "datatype-property" => 7,
        "class" => 6,
        "rdf-property" => 5,
        "named-individual" => 4,
        "restriction" | "class-expression" => 4,
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

fn clean_uri(value: &str) -> String {
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

fn is_boilerplate_label(label: &str) -> bool {
    matches!(label, "w3.orgClass")
}

fn is_primary_graph_node(node: &OntologyGraphNode) -> bool {
    if is_hidden_primary_graph_iri(&node.id) || is_boilerplate_label(&node.label) {
        return false;
    }

    if node.semantic_type == "literal" || node.node_type == "literal" {
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

const ONTOLOGY_GRAPH_CSS: &str = r#"
body:has(.ontology-page) {
    overflow: hidden;
}
body:has(.ontology-page) > div.w-full {
    height: calc(100vh - 50px);
    max-width: none !important;
    margin: 0 !important;
    padding: 0 !important;
}
body:has(.ontology-page) > div.w-full > div.bg-white {
    height: 100%;
    border: 0 !important;
    border-radius: 0 !important;
    background: #fff;
    box-shadow: none !important;
    padding: 0 !important;
}
body:has(.ontology-page) footer {
    display: none;
}
.ontology-page {
    height: 100%;
    min-height: 0;
}
.ontology-graph-panel {
    display: grid;
    grid-template-columns: minmax(0, 1fr) 420px;
    height: 100%;
    border: 0;
    border-radius: 0;
    overflow: hidden;
    background: #fff;
}
.ontology-graph-canvas {
    position: relative;
    min-width: 0;
    background: #f8fafc;
}
#ontology-graph-svg {
    width: 100%;
    height: 100%;
    display: block;
}
.ontology-graph-legend {
    position: absolute;
    top: 12px;
    left: 12px;
    z-index: 1;
    max-width: 190px;
    max-height: calc(100% - 24px);
    overflow-y: auto;
    padding: 8px 10px;
    border: 1px solid #cbd5e1;
    border-radius: 6px;
    background: rgba(248, 250, 252, 0.94);
    box-shadow: 0 2px 6px rgba(15, 23, 42, 0.08);
    color: #334155;
    font-size: 11px;
    line-height: 1.25;
    pointer-events: auto;
}
.ontology-legend-title {
    margin-bottom: 6px;
    color: #0f172a;
    font-size: 12px;
    font-weight: 700;
}
.ontology-legend-title-secondary {
    margin-top: 8px;
}
.ontology-legend-grid {
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    gap: 4px;
    align-items: center;
}
.ontology-legend-origin-grid {
    grid-template-columns: minmax(0, 1fr);
}
.ontology-legend-item,
.ontology-legend-key-item {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    min-width: 100%;
    border-radius: 4px;
    padding: 2px 4px;
    text-align: left;
    white-space: nowrap;
}
.ontology-legend-item {
    border: 1px solid transparent;
    background: transparent;
    color: #334155;
    cursor: pointer;
    font: inherit;
}
.ontology-legend-key-item {
    color: #475569;
}
.ontology-legend-item:hover {
    background: #e2e8f0;
}
.ontology-legend-item:focus-visible {
    outline: 2px solid #2563eb;
    outline-offset: 1px;
}
.ontology-legend-item.is-active {
    border-color: #bfdbfe;
    background: #eff6ff;
    color: #1e3a8a;
}
.ontology-legend-item:not(.is-active) {
    opacity: 0.45;
}
.ontology-dot {
    display: inline-flex;
    flex: 0 0 auto;
    width: 11px;
    height: 11px;
    border-radius: 3px;
}
.ontology-dot-class { background: #f59e0b; border: 1px solid #b45309; }
.ontology-dot-object-property { background: #2563eb; }
.ontology-dot-datatype-property { background: #0891b2; }
.ontology-dot-rdf-property { background: #0f766e; }
.ontology-dot-named-individual { background: #7c3aed; }
.ontology-dot-datatype { background: #e0f2fe; border: 1px solid #0284c7; }
.ontology-dot-restriction { background: #f5d0fe; border: 1px solid #a21caf; }
.ontology-dot-class-expression { background: #fce7f3; border: 1px solid #be185d; }
.ontology-dot-node-shape { background: #dc2626; }
.ontology-dot-property-shape { background: #be123c; }
.ontology-dot-resource { background: #ccfbf1; border: 1px solid #0f766e; }
.ontology-dot-external-reference { background: #f8fafc; border: 1px dashed #64748b; }
.ontology-edge-sample {
    display: inline-flex;
    flex: 0 0 auto;
    width: 22px;
    height: 0;
    border-top: 2px solid #94a3b8;
}
.ontology-legend-symbol {
    display: inline-flex;
    flex: 0 0 auto;
    min-width: 22px;
    justify-content: center;
    color: #1d4ed8;
    font-size: 13px;
    font-weight: 700;
    line-height: 1;
}
.ontology-origin-dot {
    display: inline-flex;
    flex: 0 0 auto;
    width: 10px;
    height: 10px;
    border-radius: 999px;
}
.ontology-origin-authored { background: #14b8a6; }
.ontology-origin-registry { background: #64748b; }
.ontology-origin-construct { background: #8b5cf6; }
.ontology-filtered-out {
    display: none;
}
.ontology-graph-sidebar {
    display: flex;
    min-width: 0;
    flex-direction: column;
    overflow: hidden;
    border-left: 1px solid #d1d5db;
    background: #fff;
}
.ontology-empty-download-link {
    color: #334155;
    font-size: 12px;
    font-weight: 700;
    text-decoration: underline;
    text-underline-offset: 2px;
}
.ontology-sidebar-summary {
    flex: 0 0 auto;
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: 0;
    overflow: hidden;
    border-top: 1px solid #e5e7eb;
    background: #f8fafc;
    color: #64748b;
    padding: 5px 8px;
    font-size: 10px;
    line-height: 1.2;
    white-space: nowrap;
}
.ontology-summary-entry + .ontology-summary-entry::before {
    content: "|";
    color: #cbd5e1;
    padding: 0 6px;
}
.ontology-summary-entry strong {
    color: #111827;
    font-size: 11px;
    font-weight: 700;
}
.ontology-footer-download {
    color: #334155;
    text-decoration: none;
    font-size: 10px;
    font-weight: 700;
    line-height: inherit;
}
.ontology-footer-download:hover {
    color: #111827;
    text-decoration: underline;
}
.ontology-search-panel {
    border-bottom: 1px solid #e5e7eb;
    padding: 14px;
}
.ontology-graph-search {
    width: 100%;
    box-sizing: border-box;
    border: 1px solid #cbd5e1;
    border-radius: 4px;
    padding: 8px 10px;
    font-size: 13px;
}
.ontology-graph-search:focus {
    outline: none;
    border-color: #2563eb;
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.14);
}
.ontology-graph-results {
    display: none;
    max-height: 150px;
    overflow-y: auto;
    margin: 10px 0 0;
    padding: 0;
    list-style: none;
    border: 1px solid #e5e7eb;
    border-radius: 4px;
}
.ontology-graph-result {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    padding: 8px 10px;
    border-bottom: 1px solid #f1f5f9;
    cursor: pointer;
    font-size: 13px;
}
.ontology-graph-result:last-child {
    border-bottom: 0;
}
.ontology-graph-result:hover {
    background: #f8fafc;
}
.ontology-graph-badge {
    flex: 0 0 auto;
    border-radius: 999px;
    padding: 2px 7px;
    color: #fff;
    font-size: 10px;
    font-weight: 700;
}
.ontology-inspector-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    padding: 14px;
    border-bottom: 1px solid #e5e7eb;
    background: #f8fafc;
}
.ontology-inspector-header h2 {
    margin: 0;
    padding: 0;
    border: 0;
    color: #111827;
    font-size: 16px;
    line-height: 1.3;
}
#ontology-inspector-clear {
    display: none;
    border: 0;
    background: transparent;
    color: #6b7280;
    cursor: pointer;
    font-size: 18px;
}
.ontology-inspector-body {
    flex: 1;
    min-height: 0;
    overflow: auto;
    padding: 14px;
    color: #374151;
    font-size: 13px;
    line-height: 1.5;
}
.ontology-meta-section {
    margin-top: 14px;
    padding-top: 12px;
    border-top: 1px solid #f1f5f9;
}
.ontology-meta-section:first-child {
    margin-top: 0;
    padding-top: 0;
    border-top: 0;
}
.ontology-meta-title {
    margin-bottom: 6px;
    color: #111827;
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0;
    text-transform: uppercase;
}
.ontology-uri-block {
    overflow-wrap: anywhere;
    border: 1px solid #e5e7eb;
    border-radius: 4px;
    background: #f8fafc;
    color: #1d4ed8;
    padding: 8px;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", monospace;
    font-size: 11px;
}
.ontology-type-pill {
    display: inline-block;
    margin: 2px 4px 2px 0;
    border: 1px solid #bbf7d0;
    border-radius: 4px;
    background: #f0fdf4;
    color: #166534;
    padding: 2px 6px;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", monospace;
    font-size: 11px;
}
.ontology-constraint {
    display: flex;
    justify-content: space-between;
    gap: 8px;
    margin-bottom: 4px;
    border: 1px solid #fed7aa;
    border-radius: 4px;
    background: #fff7ed;
    color: #c2410c;
    padding: 4px 7px;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", monospace;
    font-size: 11px;
}
.ontology-literal-value {
    display: flex;
    justify-content: space-between;
    gap: 8px;
    margin-bottom: 4px;
    border: 1px solid #cbd5e1;
    border-radius: 4px;
    background: #f8fafc;
    color: #334155;
    padding: 4px 7px;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", monospace;
    font-size: 11px;
}
.ontology-literal-value strong {
    text-align: right;
    overflow-wrap: anywhere;
}
.ontology-slot-facet {
    margin-bottom: 7px;
    border: 1px solid #bae6fd;
    border-radius: 4px;
    background: #f0f9ff;
    color: #0c4a6e;
    padding: 7px 8px;
    font-size: 11px;
    line-height: 1.45;
}
.ontology-slot-facet-title {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    margin-bottom: 5px;
}
.ontology-slot-facet-title strong {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", monospace;
    color: #075985;
}
.ontology-slot-facet-source {
    color: #0369a1;
    font-size: 10px;
}
.ontology-slot-facet-target {
    margin-bottom: 5px;
    color: #475569;
    font-size: 10px;
}
.ontology-slot-facet-values {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
}
.ontology-slot-facet-pill {
    display: inline-flex;
    gap: 4px;
    border: 1px solid #7dd3fc;
    border-radius: 999px;
    background: #e0f2fe;
    color: #075985;
    padding: 1px 7px;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", monospace;
}
.ontology-slot-facet-pill span {
    color: #0369a1;
}
.ontology-badge-row {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-bottom: 12px;
}
.ontology-badge {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    border: 1px solid #c7d2fe;
    border-radius: 999px;
    background: #eef2ff;
    color: #3730a3;
    padding: 2px 8px;
    font-size: 11px;
    line-height: 1.4;
}
.ontology-badge-symbol {
    font-size: 13px;
    line-height: 1;
}
.ontology-rel-pill {
    display: inline-block;
    margin: 2px 4px 2px 0;
    border: 1px solid #e5e7eb;
    border-radius: 4px;
    background: #f8fafc;
    color: #334155;
    padding: 2px 6px;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", monospace;
    font-size: 11px;
}
.ontology-kind-pill {
    display: inline-block;
    border: 1px solid #bfdbfe;
    border-radius: 999px;
    background: #eff6ff;
    color: #1e40af;
    padding: 2px 9px;
    font-size: 11px;
    font-weight: 600;
}
.ontology-term-ref {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    margin: 2px 4px 2px 0;
    border: 1px solid #e5e7eb;
    border-radius: 4px;
    background: #f8fafc;
    color: #334155;
    padding: 2px 4px 2px 7px;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", monospace;
    font-size: 11px;
}
.ontology-term-kind {
    border-radius: 3px;
    background: #e2e8f0;
    color: #475569;
    padding: 0 4px;
    font-size: 9px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    letter-spacing: 0.03em;
    text-transform: uppercase;
}
.ontology-term-class .ontology-term-kind {
    background: #dbeafe;
    color: #1e40af;
}
.ontology-term-datatype .ontology-term-kind,
.ontology-term-literal .ontology-term-kind {
    background: #fef3c7;
    color: #92400e;
}
.ontology-source {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 8px;
    margin-bottom: 4px;
    font-size: 11px;
    color: #475569;
}
.ontology-source a {
    color: inherit;
    text-decoration: none;
}
.ontology-source a:hover {
    color: #1d4ed8;
    text-decoration: underline;
}
.ontology-source-kind {
    margin-left: 6px;
    border-radius: 3px;
    background: #f1f5f9;
    color: #64748b;
    padding: 0 5px;
    font-size: 9px;
    text-transform: uppercase;
    letter-spacing: 0.03em;
}
.ontology-source-loc {
    flex: 0 1 auto;
    color: #64748b;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", monospace;
    overflow-wrap: anywhere;
    text-align: right;
}
.ontology-chain {
    margin-bottom: 6px;
    border: 1px solid #ddd6fe;
    border-radius: 4px;
    background: #f5f3ff;
    color: #5b21b6;
    padding: 6px 8px;
    font-size: 11px;
    line-height: 1.5;
}
.ontology-chain-sep {
    color: #a78bfa;
    padding: 0 2px;
}
.ontology-chain-source {
    margin-top: 4px;
    color: #6d28d9;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", monospace;
    font-size: 10px;
    overflow-wrap: anywhere;
}
.ontology-construct {
    margin-bottom: 6px;
    border: 1px solid #c7d2fe;
    border-radius: 4px;
    background: #eef2ff;
    color: #312e81;
    padding: 6px 8px;
    font-size: 11px;
    line-height: 1.5;
}
.ontology-construct-title {
    display: flex;
    align-items: center;
    gap: 5px;
    font-weight: 700;
}
.ontology-construct-meta {
    margin-top: 4px;
    color: #4338ca;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", monospace;
    overflow-wrap: anywhere;
}
.ontology-construct-symbol {
    font-size: 13px;
    line-height: 1;
}
.ontology-construct-members,
.ontology-construct-usages {
    display: flex;
    flex-direction: column;
    gap: 5px;
}
.ontology-construct-member,
.ontology-construct-usage {
    border: 1px solid #dbeafe;
    border-radius: 4px;
    background: #f8fafc;
    color: #1e3a8a;
    padding: 5px 7px;
    font-size: 11px;
    overflow-wrap: anywhere;
}
.ontology-construct-usage strong {
    color: #1e40af;
}
.ontology-raw-details {
    margin-top: 14px;
    padding-top: 12px;
    border-top: 1px solid #f1f5f9;
}
.ontology-raw-details summary {
    cursor: pointer;
    color: #475569;
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
}
.ontology-raw-details .ontology-uri-block {
    margin-top: 8px;
}
.ontology-node-badges {
    font-size: 12px;
    fill: #1e293b;
    pointer-events: none;
}
.ontology-node rect,
.ontology-node ellipse {
    stroke-width: 1.5px;
    cursor: pointer;
}
.ontology-node text {
    pointer-events: none;
    font-size: 11px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
}
.ontology-edge {
    stroke: #cbd5e1;
    stroke-width: 1.4px;
    fill: none;
}
.ontology-edge-label {
    fill: #475569;
    font-size: 9px;
    paint-order: stroke;
    stroke: #fff;
    stroke-width: 3px;
}
@media (max-width: 900px) {
    .ontology-graph-panel {
        grid-template-columns: 1fr;
        height: auto;
    }
    .ontology-graph-canvas {
        height: 520px;
    }
    .ontology-graph-sidebar {
        min-height: 360px;
        border-left: 0;
        border-top: 1px solid #d1d5db;
    }
}
"#;

const ONTOLOGY_GRAPH_JS: &str = r#"
(function () {
    if (!window.d3 || !ontologyGraphData || !ontologyGraphData.nodes.length) {
        return;
    }

    const colorBySemanticType = {
        class: { fill: '#f59e0b', stroke: '#b45309', text: '#111827' },
        'object-property': { fill: '#2563eb', stroke: '#1d4ed8', text: '#ffffff' },
        'datatype-property': { fill: '#0891b2', stroke: '#0e7490', text: '#ffffff' },
        'rdf-property': { fill: '#0f766e', stroke: '#115e59', text: '#ffffff' },
        'named-individual': { fill: '#7c3aed', stroke: '#6d28d9', text: '#ffffff' },
        datatype: { fill: '#e0f2fe', stroke: '#0284c7', text: '#0c4a6e' },
        restriction: { fill: '#f5d0fe', stroke: '#a21caf', text: '#581c87' },
        'class-expression': { fill: '#fce7f3', stroke: '#be185d', text: '#831843' },
        'node-shape': { fill: '#dc2626', stroke: '#b91c1c', text: '#ffffff' },
        'property-shape': { fill: '#be123c', stroke: '#9f1239', text: '#ffffff' },
        resource: { fill: '#ccfbf1', stroke: '#0f766e', text: '#134e4a' }
    };
    function nodePalette(nodeData) {
        return colorBySemanticType[nodeData.semantic_type] || colorBySemanticType.resource;
    }
    const nodes = ontologyGraphData.nodes.map(node => {
        const displayLabel = graphNodeDisplayLabel(node);
        return {
            ...node,
            display_label: displayLabel,
            width: Math.max(92, Math.min(260, String(displayLabel || '').length * 7 + 28)),
            height: 36
        };
    });
    const links = ontologyGraphData.edges.map(edge => ({ ...edge }));
    const nodeById = new Map(nodes.map(node => [node.id, node]));
    const adjacency = new Map(nodes.map(node => [node.id, new Set([node.id])]));
    links.forEach(link => {
        const source = typeof link.source === 'string' ? link.source : link.source.id;
        const target = typeof link.target === 'string' ? link.target : link.target.id;
        if (adjacency.has(source)) adjacency.get(source).add(target);
        if (adjacency.has(target)) adjacency.get(target).add(source);
    });
    const filterState = {
        role: new Set([
            'ontology-term',
            'property',
            'shacl-shape',
            'resource',
            'relation'
        ]),
        construct: new Set([
            'domain-range',
            'subclass',
            'membership',
            'disjoint',
            'equivalence',
            'inverse',
            'property-chain',
            'property-characteristic',
            'restriction',
            'class-expression',
            'shape-overlay'
        ]),
        origin: new Set(['authored', 'registry', 'construct'])
    };
    nodes.forEach(node => {
        node._ontologyRoles = nodeRoleValues(node);
        node._ontologyConstructs = nodeConstructValues(node);
        node._ontologyOrigins = nodeOriginValues(node);
    });
    links.forEach(link => {
        link._ontologyRoles = edgeRoleValues(link);
        link._ontologyConstructs = edgeConstructValues(link);
        link._ontologyOrigins = edgeOriginValues(link);
    });
    let visibleNodeIds = new Set(nodes.map(node => node.id));
    let selectedNodeId = null;

    const svg = d3.select('#ontology-graph-svg');
    const canvas = document.querySelector('.ontology-graph-canvas');
    let width = 0;
    let height = 0;
    let simulation;

    function measureGraph() {
        width = Math.max(480, canvas.clientWidth || 900);
        height = Math.max(420, canvas.clientHeight || 640);
        svg.attr('viewBox', [0, 0, width, height]);
    }
    measureGraph();

    svg.append('defs').append('marker')
        .attr('id', 'ontology-arrow')
        .attr('viewBox', '0 -5 10 10')
        .attr('refX', 9)
        .attr('refY', 0)
        .attr('markerWidth', 7)
        .attr('markerHeight', 7)
        .attr('orient', 'auto')
        .append('path')
        .attr('d', 'M0,-5L10,0L0,5')
        .attr('fill', '#cbd5e1');

    const viewport = svg.append('g');
    const edgeLayer = viewport.append('g');
    const labelLayer = viewport.append('g');
    const nodeLayer = viewport.append('g');

    const zoomBehavior = d3.zoom()
        .scaleExtent([0.25, 3])
        .on('zoom', event => viewport.attr('transform', event.transform));
    svg.call(zoomBehavior);

    const edge = edgeLayer.selectAll('path')
        .data(links)
        .join('path')
        .attr('class', 'ontology-edge')
        .attr('marker-end', 'url(#ontology-arrow)');

    const edgeLabel = labelLayer.selectAll('text')
        .data(links)
        .join('text')
        .attr('class', 'ontology-edge-label')
        .attr('text-anchor', 'middle')
            .text(d => d.display_label || graphNodeDisplayLabel(d));

    const node = nodeLayer.selectAll('g')
        .data(nodes)
        .join('g')
        .attr('class', 'ontology-node')
        .call(d3.drag()
            .on('start', dragStarted)
            .on('drag', dragged)
            .on('end', dragEnded))
        .on('click', (event, d) => {
            event.stopPropagation();
            focusOntologyNode(d.id);
        })
        .on('mouseenter', (event, d) => highlightNeighborhood(d.id))
        .on('mouseleave', clearHighlight);

    node.each(function (d) {
        const palette = nodePalette(d);
        const group = d3.select(this);
        group.append('rect')
            .attr('x', -d.width / 2)
            .attr('y', -d.height / 2)
            .attr('width', d.width)
            .attr('height', d.height)
            .attr('rx', 4)
            .attr('fill', palette.fill)
            .attr('stroke', palette.stroke);
        group.append('text')
            .attr('fill', palette.text)
            .attr('text-anchor', 'middle')
            .attr('dominant-baseline', 'central')
            .text(truncateLabel(d.display_label || graphNodeDisplayLabel(d), d.width));
        const badgeSymbols = visibleBadgeSymbols(d);
        if (badgeSymbols) {
            group.append('text')
                .attr('class', 'ontology-node-badges')
                .attr('text-anchor', 'middle')
                .attr('x', 0)
                .attr('y', -d.height / 2 - 5)
                .text(badgeSymbols);
        }
        group.append('title').text(nodeAccessibleSummary(d));
    });

    simulation = d3.forceSimulation(nodes)
        .force('link', d3.forceLink(links).id(d => d.id).distance(d => {
            const labelWeight = Math.min(80, String(d.label || d.id || '').length * 5);
            return 120 + labelWeight;
        }))
        .force('charge', d3.forceManyBody().strength(-520))
        .force('center', d3.forceCenter(width / 2, height / 2))
        .force('collision', d3.forceCollide().radius(d => Math.max(d.width, d.height) / 2 + 18))
        .on('tick', ticked);

    function resizeGraph(restart) {
        measureGraph();
        simulation.force('center', d3.forceCenter(width / 2, height / 2));
        if (restart) {
            simulation.alpha(0.35).restart();
        }
    }

    function ticked() {
        edge.attr('d', d => {
            const source = d.source;
            const target = d.target;
            const dx = target.x - source.x;
            const dy = target.y - source.y;
            const dr = Math.sqrt(dx * dx + dy * dy) * 0.8;
            return `M${source.x},${source.y}A${dr},${dr} 0 0,1 ${target.x},${target.y}`;
        });
        edgeLabel
            .attr('x', d => (d.source.x + d.target.x) / 2)
            .attr('y', d => (d.source.y + d.target.y) / 2);
        node.attr('transform', d => `translate(${d.x},${d.y})`);
    }

    function dragStarted(event, d) {
        if (!event.active) simulation.alphaTarget(0.25).restart();
        d.fx = d.x;
        d.fy = d.y;
    }

    function dragged(event, d) {
        d.fx = event.x;
        d.fy = event.y;
    }

    function dragEnded(event, d) {
        if (!event.active) simulation.alphaTarget(0);
        d.fx = null;
        d.fy = null;
    }

    function truncateLabel(value, width) {
        const text = String(value || '');
        const max = Math.max(8, Math.floor((width - 20) / 7));
        return text.length > max ? `${text.slice(0, max - 1)}...` : text;
    }

    function escapeHtml(value) {
        return String(value || '')
            .replaceAll('&', '&amp;')
            .replaceAll('<', '&lt;')
            .replaceAll('>', '&gt;')
            .replaceAll('"', '&quot;')
            .replaceAll("'", '&#39;');
    }

    function escapeJsString(value) {
        return String(value || '').replaceAll('\\', '\\\\').replaceAll("'", "\\'");
    }

    function searchBadgeStyle(nodeData) {
        const palette = nodePalette(nodeData);
        return `background:${palette.fill};color:${palette.text};border:1px solid ${palette.stroke}`;
    }

    function shortLabel(value) {
        const text = String(value || '');
        const parts = text.split(/[\/#]/).filter(Boolean);
        return parts.length ? parts[parts.length - 1] : text;
    }

    function classExpressionMembersLabel(nodeData) {
        const expression = (nodeData.constructs || []).find(item => item.kind === 'class-expression');
        const members = expression ? (expression.members || []) : [];
        if (!members.length) {
            return nodeData.label || 'Class expression';
        }
        const joiner = expression.label === 'Intersection'
            ? ' ∩ '
            : expression.label === 'Complement'
            ? ' ∖ '
            : ' ∪ ';
        return members.map(shortLabel).join(joiner);
    }

    function graphNodeDisplayLabel(nodeData) {
        if (!nodeData || nodeData.semantic_type !== 'class-expression') {
            return (nodeData && (nodeData.label || nodeData.id)) || '';
        }
        const usage = (nodeData.constructs || []).find(item =>
            item.kind === 'property-domain' || item.kind === 'property-range'
        );
        const expression = classExpressionMembersLabel(nodeData);
        if (!usage) {
            return expression;
        }
        const property = shortLabel(usage.subject || usage.property || '');
        const role = usage.kind === 'property-domain' ? 'domain' : 'range';
        return property ? `${property} ${role}: ${expression}` : `${role}: ${expression}`;
    }

    function badgeAriaLabel(badge) {
        return `${badge.label} (symbol ${badge.symbol})`;
    }

    function nodeAccessibleSummary(nodeData) {
        const label = graphNodeDisplayLabel(nodeData) || nodeData.id;
        const badges = (nodeData.badges || []).map(badgeAriaLabel).join('; ');
        return badges ? `${label} - ${badges}` : label;
    }

    function renderBadges(badges) {
        if (!badges || !badges.length) {
            return '';
        }
        const items = badges.map(badge => {
            const aria = escapeHtml(badgeAriaLabel(badge));
            return `<span class="ontology-badge" role="img" aria-label="${aria}" title="${aria}">`
                + `<span class="ontology-badge-symbol" aria-hidden="true">${escapeHtml(badge.symbol)}</span>`
                + `<span class="ontology-badge-text">${escapeHtml(badge.label)}</span>`
                + `</span>`;
        }).join('');
        return `<div class="ontology-badge-row">${items}</div>`;
    }

    function badgeFilterValue(badge) {
        const kind = String((badge && badge.kind) || '');
        if (kind === 'subset-or-equal') return 'subclass';
        if (kind === 'member-of') return 'membership';
        if (kind === 'disjointness') return 'disjoint';
        if (kind === 'logical-equivalence') return 'equivalence';
        if (kind === 'inverse-property') return 'inverse';
        if (['functional', 'inverse-functional', 'symmetric', 'asymmetric', 'reflexive', 'irreflexive', 'transitive'].includes(kind)) {
            return 'property-characteristic';
        }
        if (kind === 'universal-restriction' || kind === 'existential-restriction') return 'restriction';
        if (kind === 'intersection' || kind === 'union' || kind === 'set-difference') return 'class-expression';
        return '';
    }

    function visibleBadges(nodeData) {
        return (nodeData.badges || []).filter(badge => {
            const value = badgeFilterValue(badge);
            return value ? filterState.construct.has(value) : true;
        });
    }

    function visibleBadgeSymbols(nodeData) {
        return visibleBadges(nodeData).map(badge => badge.symbol).join(' ');
    }

    function visibleConstructDetails(nodeData) {
        return (nodeData.constructs || []).filter(item => {
            const values = constructFilterValues(item);
            return values.size ? hasAny(values, filterState.construct) : true;
        });
    }

    function visibleSlotFacets(nodeData) {
        return filterState.construct.has('shape-overlay') ? (nodeData.slot_facets || []) : [];
    }

    function equivalenceMembers(group) {
        const raw = String(group || '');
        const body = raw.startsWith('equivalence:') ? raw.slice('equivalence:'.length) : raw;
        return body ? body.split('|').filter(Boolean) : [];
    }

    function renderRelationPills(ids) {
        if (!ids || !ids.length) {
            return '<span class="text-gray-400 italic">None</span>';
        }
        return ids.map(id => `<span class="ontology-rel-pill" title="${escapeHtml(id)}">${escapeHtml(shortLabel(id))}</span>`).join('');
    }

    const SEMANTIC_TYPE_LABELS = {
        'object-property': 'Object property',
        'datatype-property': 'Datatype property',
        'rdf-property': 'RDF property',
        'class': 'Class',
        'named-individual': 'Named individual',
        'node-shape': 'SHACL node shape',
        'property-shape': 'SHACL property shape',
        'restriction': 'OWL restriction',
        'class-expression': 'Class expression',
        'datatype': 'Datatype',
        'literal': 'Literal',
        'resource': 'Resource'
    };

    function humanizeSemanticType(value) {
        return SEMANTIC_TYPE_LABELS[value] || (value || 'Resource');
    }

    function renderTermRefs(terms, emptyLabel) {
        if (!terms || !terms.length) {
            return `<span class="text-gray-400 italic">${escapeHtml(emptyLabel)}</span>`;
        }
        return terms.map(term => {
            const kind = String(term.kind || 'class');
            const title = escapeHtml(term.iri || term.label || '');
            return `<span class="ontology-term-ref ontology-term-${escapeHtml(kind)}" title="${title}">`
                + `${escapeHtml(term.label || term.iri || '')}`
                + `<span class="ontology-term-kind">${escapeHtml(kind)}</span>`
                + `</span>`;
        }).join('');
    }

    function renderLiteralValues(values) {
        if (!values || !values.length) {
            return '';
        }
        return `<div class="ontology-meta-section">
            <div class="ontology-meta-title">Literal Values</div>
            ${values.map(item => (
                `<div class="ontology-literal-value"><span>${escapeHtml(item.predicate || 'value')}</span><strong>${escapeHtml(item.value || '')}</strong></div>`
            )).join('')}
        </div>`;
    }

    function renderSources(sources) {
        if (!sources || !sources.length) {
            return '<span class="text-gray-400 italic">No source recorded.</span>';
        }
        return sources.map(src => {
            const name = src.source_name || src.source || src.file_path || 'source';
            const loc = src.line_number ? `${src.file_path}:${src.line_number}` : (src.file_path || '');
            const kind = src.kind ? `<span class="ontology-source-kind">${escapeHtml(src.kind)}</span>` : '';
            const href = src.link ? escapeHtml(src.link) : '';
            const title = escapeHtml(src.source || loc || name);
            const nameHtml = href
                ? `<a class="ontology-source-name" href="${href}" title="${title}">${escapeHtml(name)}</a>`
                : `<span>${escapeHtml(name)}</span>`;
            const locHtml = href
                ? `<a class="ontology-source-loc" href="${href}" title="${title}">${escapeHtml(loc)}</a>`
                : `<span class="ontology-source-loc">${escapeHtml(loc)}</span>`;
            return `<div class="ontology-source"><span>${nameHtml}${kind}</span>${locHtml}</div>`;
        }).join('');
    }

    function renderPropertyChain(chain) {
        const members = (chain.members || []).map(member => escapeHtml(shortLabel(member)));
        const body = members.length
            ? members.join('<span class="ontology-chain-sep">∘</span>')
            : '<span class="text-gray-400 italic">Empty chain</span>';
        const source = chain.source
            ? `<div class="ontology-chain-source">${escapeHtml(chain.source)}</div>`
            : '';
        return `<div class="ontology-chain"><div>${body}</div>${source}</div>`;
    }

    function renderConstructDetail(item) {
        if (item.kind === 'class-expression') {
            return renderClassExpressionConstructDetail(item);
        }
        const badge = item.badge
            ? `<span class="ontology-construct-symbol" title="${escapeHtml(badgeAriaLabel(item.badge))}">${escapeHtml(item.badge.symbol)}</span>`
            : '';
        const fields = [];
        if (item.subject) fields.push(`subject=${item.subject}`);
        if (item.predicate) fields.push(`predicate=${item.predicate}`);
        if (item.property) fields.push(`property=${item.property}`);
        if (item.object) fields.push(`object=${item.object}`);
        if (item.members && item.members.length) fields.push(`members=${item.members.join(' -> ')}`);
        const source = item.source && (item.source.source_name || item.source.source || item.source.file_path)
            ? `source=${item.source.source_name || item.source.source || item.source.file_path}${item.source.line_number ? ':' + item.source.line_number : ''}`
            : '';
        if (source) fields.push(source);
        const meta = fields.length
            ? `<div class="ontology-construct-meta">${escapeHtml(fields.join(' | '))}</div>`
            : '';
        return `<div class="ontology-construct">`
            + `<div class="ontology-construct-title">${badge}<span>${escapeHtml(item.label || item.kind || 'Construct')}</span></div>`
            + meta
            + `</div>`;
    }

    function renderClassExpressionConstructDetail(item) {
        const badge = item.badge
            ? `<span class="ontology-construct-symbol" title="${escapeHtml(badgeAriaLabel(item.badge))}">${escapeHtml(item.badge.symbol)}</span>`
            : '';
        const members = (item.members || []).length
            ? `<div class="ontology-construct-members">${item.members.map((member, index) => (
                `<div class="ontology-construct-member"><strong>${index + 1}.</strong> ${escapeHtml(shortLabel(member))}</div>`
            )).join('')}</div>`
            : '<span class="text-gray-400 italic">No members recorded.</span>';
        const source = item.source && (item.source.source_name || item.source.source || item.source.file_path)
            ? `<div class="ontology-construct-meta">source=${escapeHtml(item.source.source_name || item.source.source || item.source.file_path)}${item.source.line_number ? ':' + escapeHtml(item.source.line_number) : ''}</div>`
            : '';
        return `<div class="ontology-construct">`
            + `<div class="ontology-construct-title">${badge}<span>${escapeHtml(item.label || 'Class expression')}</span></div>`
            + members
            + source
            + `</div>`;
    }

    function isBlankNodeIdentifier(value) {
        return String(value || '').startsWith('_:');
    }

    function classExpressionConstructs(nodeData) {
        return visibleConstructDetails(nodeData).filter(item => item.kind === 'class-expression');
    }

    function graphUsagesForNode(nodeData) {
        const id = nodeData.id;
        return links
            .filter(link => endpointId(link.source) === id || endpointId(link.target) === id)
            .map(link => {
                const sourceId = endpointId(link.source);
                const targetId = endpointId(link.target);
                const source = nodeById.get(sourceId);
                const target = nodeById.get(targetId);
                const direction = sourceId === id ? 'to' : 'from';
                const other = sourceId === id ? target : source;
                return {
                    label: link.label || 'relation',
                    direction,
                    other: other ? (other.label || other.id) : (direction === 'to' ? targetId : sourceId),
                };
            });
    }

    function renderClassExpressionSummary(nodeData) {
        if (nodeData.semantic_type !== 'class-expression') {
            return '';
        }
        const expressionConstructs = classExpressionConstructs(nodeData);
        const memberSection = expressionConstructs.length
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">Expression Members</div>
                ${expressionConstructs.map(renderClassExpressionConstructDetail).join('')}
            </div>`
            : '';
        const usages = graphUsagesForNode(nodeData)
            .filter(usage => !['union', 'intersection', 'complement'].includes(String(usage.label || '')));
        const usageSection = usages.length
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">Used As</div>
                <div class="ontology-construct-usages">${usages.map(usage => (
                    `<div class="ontology-construct-usage"><strong>${escapeHtml(usage.label)}</strong> ${escapeHtml(usage.direction)} ${escapeHtml(usage.other)}</div>`
                )).join('')}</div>
            </div>`
            : '';
        return memberSection + usageSection;
    }

    function renderIdentifierSection(nodeData, title) {
        const value = nodeData.full_uri || '';
        if (isBlankNodeIdentifier(value) && nodeData.semantic_type === 'class-expression') {
            return `<details class="ontology-raw-details">
                <summary>Raw Details</summary>
                <div class="ontology-meta-title">Blank Node Identifier</div>
                <div class="ontology-uri-block">${escapeHtml(value)}</div>
            </details>`;
        }
        return `<div class="ontology-meta-section">
            <div class="ontology-meta-title">${escapeHtml(title)}</div>
            <div class="ontology-uri-block">${escapeHtml(value)}</div>
        </div>`;
    }

    function renderSlotFacet(facet, contextKind) {
        const isPropertyContext = contextKind === 'property';
        const values = (facet.facets || []).length
            ? `<div class="ontology-slot-facet-values">${facet.facets.map(item => (
                `<span class="ontology-slot-facet-pill"><span>${escapeHtml(item.name)}</span>${escapeHtml(item.value)}</span>`
            )).join('')}</div>`
            : '<span class="text-gray-400 italic">No explicit facets.</span>';
        const slotLabel = facet.slot_label || facet.slot_iri || 'slot';
        const targetLabel = facet.target_class_label || facet.target_class_iri || 'target class';
        const titleLabel = isPropertyContext ? `Class ${targetLabel}` : slotLabel;
        const titleIri = isPropertyContext
            ? (facet.target_class_iri || facet.target_class_label || '')
            : (facet.slot_iri || facet.slot_label || '');
        const target = isPropertyContext
            ? `<div class="ontology-slot-facet-target">Path ${escapeHtml(slotLabel)}</div>`
            : facet.target_class_label
            ? `<div class="ontology-slot-facet-target">Class ${escapeHtml(facet.target_class_label)}</div>`
            : '';
        return `<div class="ontology-slot-facet">`
            + `<div class="ontology-slot-facet-title"><strong title="${escapeHtml(titleIri)}">${escapeHtml(titleLabel)}</strong><span class="ontology-slot-facet-source">${escapeHtml(facet.source_shape_label || 'shape')}</span></div>`
            + target
            + values
            + `</div>`;
    }

    function ontologyNodeHaystack(node) {
        const badges = (node.badges || [])
            .map(badge => `${badge.label} ${badge.symbol} ${badge.code_point}`)
            .join(' ');
        const chains = (node.property_chains || [])
            .map(chain => `${(chain.members || []).join(' ')} ${chain.source || ''}`)
            .join(' ');
        const terms = []
            .concat(node.domain || [], node.range || [])
            .map(term => `${term.label || ''} ${term.iri || ''} ${term.kind || ''}`)
            .join(' ');
        const constraints = (node.constraints || [])
            .map(item => `${item.property || ''} ${item.value || ''}`)
            .join(' ');
        const sources = (node.sources || [])
            .map(src => `${src.source || ''} ${src.source_name || ''} ${src.file_path || ''} ${src.kind || ''}`)
            .join(' ');
        const constructs = (node.constructs || [])
            .map(item => [
                item.id,
                item.family,
                item.kind,
                item.label,
                item.subject,
                item.predicate,
                item.object,
                item.property,
                (item.members || []).join(' '),
                item.source ? `${item.source.source || ''} ${item.source.source_name || ''} ${item.source.file_path || ''}` : '',
                item.badge ? `${item.badge.label || ''} ${item.badge.symbol || ''} ${item.badge.code_point || ''}` : ''
            ].join(' '))
            .join(' ');
        const slots = (node.slot_facets || [])
            .map(slot => [
                slot.slot_label,
                slot.slot_iri,
                slot.target_class_label,
                slot.target_class_iri,
                slot.source_shape_label,
                slot.source_shape_iri,
                (slot.facets || []).map(facet => `${facet.name} ${facet.value}`).join(' ')
            ].join(' '))
            .join(' ');
        const literalValues = (node.literal_values || [])
            .map(item => `${item.predicate || ''} ${item.value || ''} ${item.source ? (item.source.source_name || item.source.source || item.source.file_path || '') : ''}`)
            .join(' ');
        return [
            graphNodeDisplayLabel(node),
            node.label,
            node.full_uri,
            node.type || '',
            node.semantic_type || '',
            (node.rdf_types || []).join(' '),
            badges,
            node.equivalence_group || '',
            (node.inverse_properties || []).join(' '),
            chains,
            terms,
            constraints,
            literalValues,
            slots,
            sources,
            constructs
        ].join(' ').toLowerCase();
    }

    function renderInspector(nodeData) {
        const title = document.getElementById('ontology-inspector-title');
        const body = document.getElementById('ontology-inspector-body');
        const clear = document.getElementById('ontology-inspector-clear');
        title.textContent = graphNodeDisplayLabel(nodeData) || nodeData.id;
        clear.style.display = 'block';
        const identifierTitle = String(nodeData.full_uri || '').startsWith('_:')
            ? 'Blank Node Identifier'
            : 'Full URI';

        const types = nodeData.rdf_types && nodeData.rdf_types.length
            ? nodeData.rdf_types.map(type => `<span class="ontology-type-pill">${escapeHtml(type)}</span>`).join('')
            : `<span class="text-gray-400 italic">Implicit ${escapeHtml(nodeData.type)} entity</span>`;
        const rawShaclEvidence = nodeData.constraints && nodeData.constraints.length
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">Raw SHACL Evidence</div>
                ${nodeData.constraints.map(item => `<div class="ontology-constraint"><span>${escapeHtml(item.property)}</span><strong>${escapeHtml(item.value)}</strong></div>`).join('')}
            </div>`
            : '';

        const badges = renderBadges(visibleBadges(nodeData));

        const equivalence = nodeData.equivalence_group
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">Equivalence Group</div>
                <div>${renderRelationPills(equivalenceMembers(nodeData.equivalence_group))}</div>
            </div>`
            : '';

        const inverses = nodeData.inverse_properties && nodeData.inverse_properties.length
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">Inverse Properties</div>
                <div>${renderRelationPills(nodeData.inverse_properties)}</div>
            </div>`
            : '';

        const chains = nodeData.property_chains && nodeData.property_chains.length
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">Property Chains</div>
                ${nodeData.property_chains.map(renderPropertyChain).join('')}
            </div>`
            : '';

        const visibleConstructs = visibleConstructDetails(nodeData).filter(item => {
            if (nodeData.semantic_type !== 'class-expression') {
                return true;
            }
            return !['class-expression', 'property-domain', 'property-range'].includes(item.kind);
        });
        const constructs = visibleConstructs.length
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">Projection Constructs</div>
                ${visibleConstructs.map(renderConstructDetail).join('')}
            </div>`
            : '';
        const classExpressionSummary = renderClassExpressionSummary(nodeData);
        const literalValues = renderLiteralValues(nodeData.literal_values);
        const identifier = renderIdentifierSection(nodeData, identifierTitle);

        const isProperty = String(nodeData.semantic_type || '').endsWith('property')
            || (nodeData.domain && nodeData.domain.length)
            || (nodeData.range && nodeData.range.length);

        const visibleSlots = visibleSlotFacets(nodeData);
        const slots = visibleSlots.length
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">${isProperty ? 'Used As Slot / Facets' : 'Slots / Facets'}</div>
                ${visibleSlots.map(facet => renderSlotFacet(facet, isProperty ? 'property' : 'class')).join('')}
            </div>`
            : '';

        const domain = isProperty
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">Domain</div>
                <div>${renderTermRefs(nodeData.domain, 'Any (unconstrained)')}</div>
            </div>`
            : '';

        const range = isProperty
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">Range</div>
                <div>${renderTermRefs(nodeData.range, 'Any (unconstrained)')}</div>
            </div>`
            : '';

        body.innerHTML = `
            ${badges}
            <div class="ontology-meta-section">
                <div class="ontology-meta-title">Kind</div>
                <div><span class="ontology-kind-pill">${escapeHtml(humanizeSemanticType(nodeData.semantic_type))}</span></div>
            </div>
            <div class="ontology-meta-section">
                <div class="ontology-meta-title">RDF Type</div>
                <div>${types}</div>
            </div>
            ${identifier}
            <div class="ontology-meta-section">
                <div class="ontology-meta-title">Description</div>
                <p class="m-0">${escapeHtml(nodeData.comment || 'None specified.')}</p>
            </div>
            ${literalValues}
            ${classExpressionSummary}
            ${domain}
            ${range}
            ${slots}
            ${equivalence}
            ${inverses}
            ${chains}
            ${constructs}
            <div class="ontology-meta-section">
                <div class="ontology-meta-title">Sources</div>
                ${renderSources(nodeData.sources)}
            </div>
            ${rawShaclEvidence}
        `;
    }

    function highlightNeighborhood(nodeId) {
        const related = adjacency.get(nodeId) || new Set([nodeId]);
        node.style('opacity', d => related.has(d.id) ? 1 : 0.18);
        edge.style('opacity', d => related.has(d.source.id) && related.has(d.target.id) ? 1 : 0.12)
            .style('stroke', d => d.source.id === nodeId || d.target.id === nodeId ? '#2563eb' : '#cbd5e1')
            .style('stroke-width', d => d.source.id === nodeId || d.target.id === nodeId ? 2.4 : 1.4);
        edgeLabel.style('opacity', d => d.source.id === nodeId || d.target.id === nodeId ? 1 : 0.08);
    }

    function clearHighlight() {
        node.style('opacity', 1);
        edge.style('opacity', 1).style('stroke', '#cbd5e1').style('stroke-width', 1.4);
        edgeLabel.style('opacity', 1);
    }

    function endpointId(value) {
        return typeof value === 'string' ? value : (value && value.id ? value.id : '');
    }

    function hasAny(values, activeValues) {
        for (const value of values || []) {
            if (activeValues.has(value)) {
                return true;
            }
        }
        return false;
    }

    function nodeRoleValues(nodeData) {
        return new Set([nodeRoleFilterValue(nodeData)]);
    }

    function nodeRoleFilterValue(nodeData) {
        if (isExternalReferenceNode(nodeData)) {
            return 'external-reference';
        }
        const semanticType = nodeData.semantic_type || 'resource';
        if (['object-property', 'datatype-property', 'rdf-property'].includes(semanticType)) {
            return 'property';
        }
        if (['node-shape', 'property-shape'].includes(semanticType)) {
            return 'shacl-shape';
        }
        if (semanticType === 'resource') {
            return 'resource';
        }
        return 'ontology-term';
    }

    function isExternalReferenceNode(nodeData) {
        const iri = String(nodeData.full_uri || nodeData.id || '');
        return iri.startsWith('http://www.w3.org/2001/XMLSchema#')
            || iri.startsWith('http://www.w3.org/1999/02/22-rdf-syntax-ns#')
            || iri.startsWith('http://www.w3.org/2000/01/rdf-schema#')
            || iri.startsWith('http://www.w3.org/2002/07/owl#')
            || iri.startsWith('http://www.w3.org/ns/shacl#');
    }

    function nodeConstructValues(nodeData) {
        const constructs = new Set();
        (nodeData.constructs || []).forEach(item => {
            constructFilterValues(item).forEach(value => constructs.add(value));
        });
        if ((nodeData.inverse_properties || []).length) {
            constructs.add('inverse');
        }
        if ((nodeData.property_chains || []).length) {
            constructs.add('property-chain');
        }
        if ((nodeData.domain || []).length || (nodeData.range || []).length) {
            constructs.add('domain-range');
        }
        if ((nodeData.slot_facets || []).length) {
            constructs.add('shape-overlay');
        }
        return constructs;
    }

    function edgeRoleValues(edgeData) {
        return new Set(['relation']);
    }

    function edgeConstructValues(edgeData) {
        const constructs = new Set();
        const label = String(edgeData.label || '');
        if (label === 'domain' || label === 'range') constructs.add('domain-range');
        if (label === 'subclass') constructs.add('subclass');
        if (label === 'member') constructs.add('membership');
        if (label === 'disjoint') constructs.add('disjoint');
        if (label === 'inverse') constructs.add('inverse');
        return constructs;
    }

    function constructFilterValues(construct) {
        const kind = String((construct && construct.kind) || '');
        const family = String((construct && construct.family) || '');
        const values = new Set();
        if (kind === 'property-domain' || kind === 'property-range' || family === 'property-domain-range') values.add('domain-range');
        if (kind === 'subclass-inclusion') values.add('subclass');
        if (kind === 'membership') values.add('membership');
        if (kind === 'disjointness') values.add('disjoint');
        if (kind === 'equivalence-group') values.add('equivalence');
        if (kind === 'inverse-property') values.add('inverse');
        if (kind === 'property-chain' || family === 'property-chain') values.add('property-chain');
        if (kind === 'property-characteristic' || family === 'property-characteristic') values.add('property-characteristic');
        if (kind === 'restriction' || family === 'restriction') values.add('restriction');
        if (kind === 'class-expression' || family === 'class-expression') values.add('class-expression');
        if (kind === 'shape-overlay' || family === 'shape-overlay') values.add('shape-overlay');
        return values;
    }

    function nodeOriginValues(nodeData) {
        const origins = new Set();
        const sources = nodeData.sources || [];
        if (sources.some(source => source.kind === 'ontology' || source.kind === 'shapes')) {
            origins.add('authored');
        }
        if (sources.length || (nodeData.rdf_types || []).length || (nodeData.type_evidence || []).length) {
            origins.add('registry');
        }
        if ((nodeData.constructs || []).length
            || (nodeData.badges || []).length
            || (nodeData.inverse_properties || []).length
            || (nodeData.property_chains || []).length
            || (nodeData.domain || []).length
            || (nodeData.range || []).length
            || (nodeData.slot_facets || []).length) {
            origins.add('construct');
        }
        if (!origins.size) {
            origins.add('registry');
        }
        return origins;
    }

    function edgeOriginValues(edgeData) {
        const origins = new Set();
        const label = String(edgeData.label || '');
        if (['domain', 'range', 'subclass', 'member', 'disjoint', 'inverse', 'restriction', 'class expression'].includes(label)) {
            origins.add('construct');
        } else {
            origins.add('registry');
        }
        return origins;
    }

    function nodePassesOwnFilters(nodeData) {
        return hasAny(nodeData._ontologyRoles, filterState.role)
            && hasAny(nodeData._ontologyOrigins, filterState.origin);
    }

    function edgePassesFilters(edgeData) {
        const hasConstructRole = edgeData._ontologyConstructs && edgeData._ontologyConstructs.size > 0;
        const roleVisible = hasConstructRole
            ? hasAny(edgeData._ontologyConstructs, filterState.construct)
            : hasAny(edgeData._ontologyRoles, filterState.role);
        return roleVisible && hasAny(edgeData._ontologyOrigins, filterState.origin);
    }

    function computeVisibleNodeIds() {
        const visible = new Set();
        nodes.forEach(nodeData => {
            if (nodePassesOwnFilters(nodeData)) {
                visible.add(nodeData.id);
            }
        });
        return visible;
    }

    function isEdgeVisible(edgeData) {
        const sourceId = endpointId(edgeData.source);
        const targetId = endpointId(edgeData.target);
        return edgePassesFilters(edgeData)
            && visibleNodeIds.has(sourceId)
            && visibleNodeIds.has(targetId);
    }

    function applyGraphFilters() {
        visibleNodeIds = computeVisibleNodeIds();
        node.classed('ontology-filtered-out', d => !visibleNodeIds.has(d.id));
        edge.classed('ontology-filtered-out', d => !isEdgeVisible(d));
        edgeLabel.classed('ontology-filtered-out', d => !isEdgeVisible(d));
        node.select('.ontology-node-badges')
            .text(d => visibleBadgeSymbols(d))
            .classed('ontology-filtered-out', d => !visibleBadgeSymbols(d));

        document.querySelectorAll('.ontology-filter-toggle').forEach(button => {
            const category = button.dataset.filterCategory;
            const value = button.dataset.filterValue;
            const active = Boolean(filterState[category] && filterState[category].has(value));
            button.classList.toggle('is-active', active);
            button.setAttribute('aria-pressed', active ? 'true' : 'false');
        });

        const search = document.getElementById('ontology-graph-search');
        if (search && search.value.trim()) {
            window.filterOntologyGraph(search.value);
        }
        if (selectedNodeId && !visibleNodeIds.has(selectedNodeId)) {
            window.clearOntologySelection();
        } else if (selectedNodeId && nodeById.has(selectedNodeId)) {
            renderInspector(nodeById.get(selectedNodeId));
        }
    }

    function initializeOntologyFilters() {
        document.querySelectorAll('.ontology-filter-toggle').forEach(button => {
            button.addEventListener('click', event => {
                event.preventDefault();
                event.stopPropagation();
                const category = button.dataset.filterCategory;
                const value = button.dataset.filterValue;
                if (!filterState[category] || !value) {
                    return;
                }
                if (filterState[category].has(value)) {
                    filterState[category].delete(value);
                } else {
                    filterState[category].add(value);
                }
                applyGraphFilters();
            });
        });
    }

    window.filterOntologyGraph = function (query) {
        const results = document.getElementById('ontology-graph-results');
        const normalized = query.trim().toLowerCase();
        if (!normalized) {
            results.style.display = 'none';
            results.innerHTML = '';
            return;
        }
        const matches = nodes
            .filter(node => visibleNodeIds.has(node.id) && ontologyNodeHaystack(node).includes(normalized))
            .slice(0, 40);

        if (!matches.length) {
            results.innerHTML = '<li class="ontology-graph-result text-gray-400">No matching nodes found</li>';
            results.style.display = 'block';
            return;
        }

        results.innerHTML = matches.map(node => `
            <li class="ontology-graph-result" onclick="focusOntologyNode('${escapeJsString(node.id)}')">
                <span>${escapeHtml(graphNodeDisplayLabel(node))}</span>
                <span class="ontology-graph-badge" style="${searchBadgeStyle(node)}">${escapeHtml(humanizeSemanticType(node.semantic_type))}</span>
            </li>
        `).join('');
        results.style.display = 'block';
    };

    window.focusOntologyNode = function (nodeId) {
        const selected = nodeById.get(nodeId);
        if (!selected) return;
        if (!visibleNodeIds.has(nodeId)) return;
        selectedNodeId = nodeId;
        document.getElementById('ontology-graph-results').style.display = 'none';
        document.getElementById('ontology-graph-search').value = '';
        renderInspector(selected);
        highlightNeighborhood(nodeId);

        node.selectAll('rect, ellipse')
            .attr('stroke-width', d => d.id === nodeId ? 4 : 1.5)
            .attr('stroke', d => d.id === nodeId ? '#0f172a' : nodePalette(d).stroke);

        const transform = d3.zoomIdentity
            .translate(width / 2 - selected.x * 1.35, height / 2 - selected.y * 1.35)
            .scale(1.35);
        svg.transition().duration(400).call(zoomBehavior.transform, transform);
    };

    window.clearOntologySelection = function () {
        selectedNodeId = null;
        document.getElementById('ontology-inspector-clear').style.display = 'none';
        document.getElementById('ontology-inspector-title').textContent = 'Node Inspector';
        document.getElementById('ontology-inspector-body').innerHTML = '<p class="text-gray-500 italic m-0">Search or select a graph node to inspect URI, RDF type, comments, and SHACL constraints.</p>';
        clearHighlight();
        node.selectAll('rect, ellipse')
            .attr('stroke-width', 1.5)
            .attr('stroke', d => nodePalette(d).stroke);
    };

    initializeOntologyFilters();
    applyGraphFilters();
    window.addEventListener('resize', () => resizeGraph(false));
})();
"#;
