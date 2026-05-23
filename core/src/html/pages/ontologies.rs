use crate::semantic_contract::{SemanticBlockKind, SemanticIndex};
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
const RDFS_CLASS: &str = "http://www.w3.org/2000/01/rdf-schema#Class";
const OWL_CLASS: &str = "http://www.w3.org/2002/07/owl#Class";
const RDF_PROPERTY: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#Property";
const OWL_OBJECT_PROPERTY: &str = "http://www.w3.org/2002/07/owl#ObjectProperty";
const OWL_DATATYPE_PROPERTY: &str = "http://www.w3.org/2002/07/owl#DatatypeProperty";
const SHACL_PREFIX: &str = "http://www.w3.org/ns/shacl#";
const SH_NODE_SHAPE: &str = "http://www.w3.org/ns/shacl#NodeShape";
const SH_PROPERTY_SHAPE: &str = "http://www.w3.org/ns/shacl#PropertyShape";
const SH_PROPERTY: &str = "http://www.w3.org/ns/shacl#property";
const SH_PATH: &str = "http://www.w3.org/ns/shacl#path";
const SH_IN: &str = "http://www.w3.org/ns/shacl#in";

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
    full_uri: String,
    comment: String,
    rdf_types: Vec<String>,
    constraints: Vec<OntologyGraphConstraint>,
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
            div class="flex flex-col md:flex-row md:items-start md:justify-between gap-4 mb-6" {
                div {
                    h1 { "Ontologies" }
                    p class="text-gray-600 mt-2" {
                        "Collected ontology element RDF/Turtle (.ttl) content and semantic-contract SHACL shapes from the graph registry."
                    }
                }
                a href={(nav_prefix) "ontologies.ttl"} class="inline-flex items-center justify-center px-4 py-2 border border-gray-300 rounded text-sm font-medium bg-white hover:bg-gray-50" {
                    "Download .ttl"
                }
            }

            div class="grid grid-cols-1 md:grid-cols-4 gap-3 mb-6" {
                (summary_card("Ontology blocks", report.summary.ontology_blocks))
                (summary_card("Shape blocks", report.summary.shape_blocks))
                (summary_card("RDF quads", report.summary.total_quads))
                (summary_card("Total blocks", report.summary.total_blocks))
            }

            @if report.blocks.is_empty() {
                p class="text-gray-500 italic" { "No ontology or SHACL RDF/Turtle content found." }
            } @else {
                section class="ontology-graph-panel mb-6" aria-label="Ontology graph explorer" {
                    div class="ontology-graph-canvas" {
                        button id="ontology-graph-expand"
                            class="ontology-graph-expand"
                            type="button"
                            onclick="toggleOntologyFullscreen()"
                            aria-label="Expand ontology graph" {
                            "Expand"
                        }
                        div class="ontology-graph-legend" {
                            div class="font-semibold text-gray-900 mb-1" { "Ontology Explorer" }
                            div { span class="ontology-dot ontology-dot-owl" {} " OWL/RDFS term" }
                            div { span class="ontology-dot ontology-dot-shacl" {} " SHACL shape" }
                            div { span class="ontology-dot ontology-dot-generic" {} " RDF resource" }
                            div { span class="ontology-dot ontology-dot-literal" {} " Literal value" }
                            div { span class="ontology-edge-sample" {} " RDF predicate" }
                        }
                        svg id="ontology-graph-svg" role="img" aria-label="Ontology and SHACL relationship graph" {}
                    }
                    aside class="ontology-graph-sidebar" {
                        div class="ontology-search-panel" {
                            input id="ontology-graph-search"
                                type="search"
                                placeholder="Search classes, shapes, or terms"
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
                    }
                }

                div class="mb-4" {
                    input id="ontology-search"
                        type="search"
                        placeholder="Search ontology sources, names, paths, or TTL content"
                        class="w-full border border-gray-300 rounded px-3 py-2 text-sm"
                        oninput="filterOntologyBlocks(this.value)";
                }

                div id="ontology-blocks" class="space-y-4" {
                    @for block in &report.blocks {
                        article class="ontology-block border border-gray-200 rounded bg-white overflow-hidden"
                            data-search={(block.source) " " (block.source_name) " " (block.file_path) " " (block.content)} {
                            div class="flex flex-col md:flex-row md:items-center md:justify-between gap-2 px-4 py-3 bg-gray-50 border-b border-gray-200" {
                                div {
                                    h2 class="text-base font-semibold m-0 border-0 p-0" { (block.source_name) }
                                    div class="text-xs text-gray-500 font-mono break-all mt-1" { (block.source) }
                                }
                                span class={(kind_class(&block.kind))} { (block.kind.as_str()) }
                            }
                            div class="px-4 py-3 text-sm text-gray-600 border-b border-gray-100" {
                                span class="font-medium" { "File: " }
                                span class="font-mono" { (block.file_path) }
                                @if block.line_number > 0 {
                                    span { " line " (block.line_number) }
                                }
                            }
                            pre class="ontology-code-block m-0 p-4 overflow-auto text-xs leading-5"
                                style="background-color:#111827 !important;color:#f9fafb !important;" {
                                code style="background:transparent !important;color:inherit !important;" { (block.content.trim()) }
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
    let mut literal_index = 0usize;
    let internal_property_shapes = collect_internal_property_shapes(report);
    let (rdf_list_nodes, rdf_list_values) = collect_rdf_lists(report);

    for block in &report.blocks {
        for quad in &block.quads {
            let subject_id = subject_id(&quad.subject);
            let predicate = quad.predicate.as_str();
            let predicate_label = clean_uri(predicate);

            if let Some(owner_id) = internal_property_shapes.get(&subject_id) {
                ensure_node(&mut nodes, owner_id, "shacl");
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

            let subject_hint = if matches!(block.kind, SemanticBlockKind::Shapes)
                || predicate.starts_with(SHACL_PREFIX)
            {
                "shacl"
            } else {
                "generic"
            };

            ensure_node(&mut nodes, &subject_id, subject_hint);

            if predicate == RDF_TYPE {
                let type_label = term_label(&quad.object);
                if let Some(node) = nodes.get_mut(&subject_id) {
                    if !node.rdf_types.contains(&type_label) {
                        node.rdf_types.push(type_label);
                    }
                    upgrade_node_type(&mut node.node_type, type_to_node_type(&quad.object));
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

            match &quad.object {
                Term::NamedNode(node) => {
                    if node.as_str() == RDF_NIL {
                        continue;
                    }
                    let target_id = node.as_str().to_string();
                    if !is_boilerplate_label(&clean_uri(&target_id)) {
                        let target_hint = if matches!(block.kind, SemanticBlockKind::Shapes)
                            || predicate.starts_with(SHACL_PREFIX)
                        {
                            "shacl"
                        } else {
                            "generic"
                        };
                        ensure_node(&mut nodes, &target_id, target_hint);
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
                    ensure_node(&mut nodes, &target_id, subject_hint);
                    edges.insert(OntologyGraphEdge {
                        source: subject_id.clone(),
                        target: target_id,
                        label: predicate_label,
                    });
                }
                Term::Literal(literal) => {
                    literal_index += 1;
                    let target_id = format!("literal-{literal_index}");
                    nodes.insert(
                        target_id.clone(),
                        OntologyGraphNode {
                            id: target_id.clone(),
                            label: literal.value().to_string(),
                            node_type: "literal".to_string(),
                            full_uri: "Literal value".to_string(),
                            comment: "Datatype field value.".to_string(),
                            rdf_types: Vec::new(),
                            constraints: Vec::new(),
                        },
                    );
                    edges.insert(OntologyGraphEdge {
                        source: subject_id.clone(),
                        target: target_id,
                        label: predicate_label,
                    });
                }
            }
        }
    }

    apply_blank_node_labels(&mut nodes);

    let nodes: Vec<_> = nodes
        .into_values()
        .filter(|node| !is_boilerplate_label(&node.label))
        .collect();
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

fn ensure_node(nodes: &mut BTreeMap<String, OntologyGraphNode>, id: &str, node_type: &str) {
    nodes
        .entry(id.to_string())
        .and_modify(|node| upgrade_node_type(&mut node.node_type, node_type))
        .or_insert_with(|| OntologyGraphNode {
            id: id.to_string(),
            label: clean_uri(id),
            node_type: node_type.to_string(),
            full_uri: id.to_string(),
            comment: "None specified.".to_string(),
            rdf_types: Vec::new(),
            constraints: Vec::new(),
        });
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

fn type_to_node_type(term: &Term) -> &'static str {
    match term {
        Term::NamedNode(node) => match node.as_str() {
            OWL_CLASS | RDFS_CLASS | RDF_PROPERTY | OWL_OBJECT_PROPERTY | OWL_DATATYPE_PROPERTY => {
                "owl"
            }
            SH_NODE_SHAPE | SH_PROPERTY_SHAPE => "shacl",
            _ => "generic",
        },
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
            | "http://www.w3.org/ns/shacl#datatype"
            | "http://www.w3.org/ns/shacl#targetClass"
            | "http://www.w3.org/ns/shacl#class"
            | "http://www.w3.org/ns/shacl#nodeKind"
            | "http://www.w3.org/ns/shacl#path"
            | "http://www.w3.org/ns/shacl#pattern"
            | "http://www.w3.org/ns/shacl#in"
    )
}

fn is_boilerplate_label(label: &str) -> bool {
    matches!(
        label,
        "Class"
            | "ObjectProperty"
            | "DatatypeProperty"
            | "Property"
            | "NodeShape"
            | "PropertyShape"
            | "NamedIndividual"
            | "Ontology"
            | "Resource"
            | "w3.orgClass"
    )
}

fn summary_card(label: &str, value: usize) -> Markup {
    html! {
        div class="border border-gray-200 rounded bg-gray-50 px-4 py-3" {
            div class="text-xs uppercase tracking-wide text-gray-500" { (label) }
            div class="text-2xl font-semibold text-gray-900 mt-1" { (value) }
        }
    }
}

fn kind_class(kind: &SemanticBlockKind) -> &'static str {
    match kind {
        SemanticBlockKind::Ontology => {
            "inline-flex items-center rounded px-2 py-1 text-xs font-semibold bg-blue-100 text-blue-800"
        }
        SemanticBlockKind::Shapes => {
            "inline-flex items-center rounded px-2 py-1 text-xs font-semibold bg-orange-100 text-orange-800"
        }
    }
}

const ONTOLOGY_GRAPH_CSS: &str = r#"
.ontology-graph-panel {
    display: grid;
    grid-template-columns: minmax(0, 1fr) 340px;
    height: 660px;
    border: 1px solid #d1d5db;
    border-radius: 8px;
    overflow: hidden;
    background: #fff;
}
body.ontology-graph-modal-open {
    overflow: hidden;
}
.ontology-graph-panel.ontology-graph-expanded {
    position: fixed;
    inset: 12px;
    z-index: 2000;
    height: auto;
    margin: 0 !important;
    border-radius: 8px;
    box-shadow: 0 18px 48px rgba(15, 23, 42, 0.28);
}
.ontology-page pre.ontology-code-block,
.bg-white pre.ontology-code-block,
pre.ontology-code-block {
    background: #111827 !important;
    color: #f9fafb !important;
}
.ontology-page pre.ontology-code-block code,
.bg-white pre.ontology-code-block code,
pre.ontology-code-block code {
    color: inherit !important;
    background: transparent !important;
    text-shadow: none;
}
.ontology-page pre.ontology-code-block *,
.bg-white pre.ontology-code-block *,
pre.ontology-code-block * {
    color: inherit !important;
    background: transparent !important;
}
.ontology-page pre.ontology-code-block::selection,
.ontology-page pre.ontology-code-block code::selection,
.bg-white pre.ontology-code-block::selection,
.bg-white pre.ontology-code-block code::selection,
pre.ontology-code-block::selection,
pre.ontology-code-block code::selection {
    background: #1d4ed8;
    color: #ffffff;
}
.ontology-graph-canvas {
    position: relative;
    min-width: 0;
    background: #f8fafc;
}
.ontology-graph-expand {
    position: absolute;
    top: 12px;
    right: 12px;
    z-index: 2;
    border: 1px solid #cbd5e1;
    border-radius: 4px;
    background: rgba(255, 255, 255, 0.94);
    color: #111827;
    cursor: pointer;
    font-size: 12px;
    font-weight: 600;
    line-height: 1;
    padding: 8px 10px;
    box-shadow: 0 2px 8px rgba(15, 23, 42, 0.08);
}
.ontology-graph-expand:hover {
    background: #f8fafc;
    border-color: #94a3b8;
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
    padding: 10px 12px;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.92);
    box-shadow: 0 2px 8px rgba(15, 23, 42, 0.08);
    font-size: 12px;
    line-height: 1.6;
    pointer-events: none;
}
.ontology-dot {
    display: inline-block;
    width: 11px;
    height: 11px;
    margin-right: 4px;
    border-radius: 3px;
    vertical-align: -1px;
}
.ontology-dot-owl { background: #2563eb; }
.ontology-dot-shacl { background: #ea580c; }
.ontology-dot-generic { background: #ccfbf1; border: 1px solid #0f766e; }
.ontology-dot-literal { background: #e5e7eb; border: 1px solid #9ca3af; }
.ontology-edge-sample {
    display: inline-block;
    width: 22px;
    height: 0;
    margin-right: 4px;
    border-top: 2px solid #94a3b8;
    vertical-align: 3px;
}
.ontology-graph-sidebar {
    display: flex;
    min-width: 0;
    flex-direction: column;
    border-left: 1px solid #d1d5db;
    background: #fff;
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
    .ontology-graph-panel.ontology-graph-expanded {
        inset: 8px;
        grid-template-rows: minmax(0, 1fr) 320px;
        height: auto;
    }
    .ontology-graph-canvas {
        height: 520px;
    }
    .ontology-graph-panel.ontology-graph-expanded .ontology-graph-canvas {
        height: auto;
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
    function filterOntologyBlocks(query) {
        const normalized = query.trim().toLowerCase();
        document.querySelectorAll('.ontology-block').forEach(block => {
            const haystack = (block.dataset.search || '').toLowerCase();
            block.style.display = !normalized || haystack.includes(normalized) ? '' : 'none';
        });
    }
    window.filterOntologyBlocks = filterOntologyBlocks;

    if (!window.d3 || !ontologyGraphData || !ontologyGraphData.nodes.length) {
        return;
    }

    const colorByType = {
        owl: { fill: '#2563eb', stroke: '#1d4ed8', text: '#ffffff' },
        shacl: { fill: '#ea580c', stroke: '#c2410c', text: '#ffffff' },
        literal: { fill: '#e5e7eb', stroke: '#9ca3af', text: '#374151' },
        generic: { fill: '#ccfbf1', stroke: '#0f766e', text: '#134e4a' }
    };
    const nodes = ontologyGraphData.nodes.map(node => ({
        ...node,
        width: Math.max(92, Math.min(220, String(node.label || '').length * 7 + 28)),
        height: node.type === 'literal' ? 38 : 36
    }));
    const links = ontologyGraphData.edges.map(edge => ({ ...edge }));
    const nodeById = new Map(nodes.map(node => [node.id, node]));
    const adjacency = new Map(nodes.map(node => [node.id, new Set([node.id])]));
    links.forEach(link => {
        const source = typeof link.source === 'string' ? link.source : link.source.id;
        const target = typeof link.target === 'string' ? link.target : link.target.id;
        if (adjacency.has(source)) adjacency.get(source).add(target);
        if (adjacency.has(target)) adjacency.get(target).add(source);
    });

    const svg = d3.select('#ontology-graph-svg');
    const panel = document.querySelector('.ontology-graph-panel');
    const canvas = document.querySelector('.ontology-graph-canvas');
    const expandButton = document.getElementById('ontology-graph-expand');
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
        .text(d => d.label);

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
        const palette = colorByType[d.type] || colorByType.generic;
        const group = d3.select(this);
        if (d.type === 'literal') {
            group.append('ellipse')
                .attr('rx', d.width / 2)
                .attr('ry', d.height / 2)
                .attr('fill', palette.fill)
                .attr('stroke', palette.stroke);
        } else {
            group.append('rect')
                .attr('x', -d.width / 2)
                .attr('y', -d.height / 2)
                .attr('width', d.width)
                .attr('height', d.height)
                .attr('rx', 4)
                .attr('fill', palette.fill)
                .attr('stroke', palette.stroke);
        }
        group.append('text')
            .attr('fill', palette.text)
            .attr('text-anchor', 'middle')
            .attr('dominant-baseline', 'central')
            .text(truncateLabel(d.label, d.width));
    });

    simulation = d3.forceSimulation(nodes)
        .force('link', d3.forceLink(links).id(d => d.id).distance(d => {
            const labelWeight = Math.min(80, String(d.label || '').length * 5);
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

    function badgeColor(type) {
        if (type === 'shacl') return '#ea580c';
        if (type === 'literal') return '#6b7280';
        if (type === 'owl') return '#2563eb';
        return '#0f766e';
    }

    function renderInspector(nodeData) {
        const title = document.getElementById('ontology-inspector-title');
        const body = document.getElementById('ontology-inspector-body');
        const clear = document.getElementById('ontology-inspector-clear');
        title.textContent = nodeData.label || nodeData.id;
        clear.style.display = 'block';
        const identifierTitle = String(nodeData.full_uri || '').startsWith('_:')
            ? 'Blank Node Identifier'
            : 'Full URI';

        const types = nodeData.rdf_types && nodeData.rdf_types.length
            ? nodeData.rdf_types.map(type => `<span class="ontology-type-pill">${escapeHtml(type)}</span>`).join('')
            : `<span class="text-gray-400 italic">Implicit ${escapeHtml(nodeData.type)} entity</span>`;
        const constraints = nodeData.constraints && nodeData.constraints.length
            ? nodeData.constraints.map(item => `<div class="ontology-constraint"><span>${escapeHtml(item.property)}</span><strong>${escapeHtml(item.value)}</strong></div>`).join('')
            : '<p class="text-gray-400 italic m-0">No structural validation rules mapped.</p>';

        body.innerHTML = `
            <div class="ontology-meta-section">
                <div class="ontology-meta-title">RDF Type</div>
                <div>${types}</div>
            </div>
            <div class="ontology-meta-section">
                <div class="ontology-meta-title">${identifierTitle}</div>
                <div class="ontology-uri-block">${escapeHtml(nodeData.full_uri)}</div>
            </div>
            <div class="ontology-meta-section">
                <div class="ontology-meta-title">Description</div>
                <p class="m-0">${escapeHtml(nodeData.comment || 'None specified.')}</p>
            </div>
            <div class="ontology-meta-section">
                <div class="ontology-meta-title">SHACL / Association Specs</div>
                ${constraints}
            </div>
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

    window.filterOntologyGraph = function (query) {
        const results = document.getElementById('ontology-graph-results');
        const normalized = query.trim().toLowerCase();
        if (!normalized) {
            results.style.display = 'none';
            results.innerHTML = '';
            return;
        }
        const matches = nodes
            .filter(node => `${node.label} ${node.full_uri} ${node.rdf_types.join(' ')}`.toLowerCase().includes(normalized))
            .slice(0, 40);

        if (!matches.length) {
            results.innerHTML = '<li class="ontology-graph-result text-gray-400">No matching nodes found</li>';
            results.style.display = 'block';
            return;
        }

        results.innerHTML = matches.map(node => `
            <li class="ontology-graph-result" onclick="focusOntologyNode('${escapeJsString(node.id)}')">
                <span>${escapeHtml(node.label)}</span>
                <span class="ontology-graph-badge" style="background:${badgeColor(node.type)}">${escapeHtml(node.type)}</span>
            </li>
        `).join('');
        results.style.display = 'block';
    };

    window.focusOntologyNode = function (nodeId) {
        const selected = nodeById.get(nodeId);
        if (!selected) return;
        document.getElementById('ontology-graph-results').style.display = 'none';
        document.getElementById('ontology-graph-search').value = '';
        renderInspector(selected);
        highlightNeighborhood(nodeId);

        node.selectAll('rect, ellipse')
            .attr('stroke-width', d => d.id === nodeId ? 4 : 1.5)
            .attr('stroke', d => d.id === nodeId ? '#0f172a' : (colorByType[d.type] || colorByType.generic).stroke);

        const transform = d3.zoomIdentity
            .translate(width / 2 - selected.x * 1.35, height / 2 - selected.y * 1.35)
            .scale(1.35);
        svg.transition().duration(400).call(zoomBehavior.transform, transform);
    };

    window.clearOntologySelection = function () {
        document.getElementById('ontology-inspector-clear').style.display = 'none';
        document.getElementById('ontology-inspector-title').textContent = 'Node Inspector';
        document.getElementById('ontology-inspector-body').innerHTML = '<p class="text-gray-500 italic m-0">Search or select a graph node to inspect URI, RDF type, comments, and SHACL constraints.</p>';
        clearHighlight();
        node.selectAll('rect, ellipse')
            .attr('stroke-width', 1.5)
            .attr('stroke', d => (colorByType[d.type] || colorByType.generic).stroke);
    };

    window.toggleOntologyFullscreen = function () {
        const expanded = panel.classList.toggle('ontology-graph-expanded');
        document.body.classList.toggle('ontology-graph-modal-open', expanded);
        expandButton.textContent = expanded ? 'Exit' : 'Expand';
        expandButton.setAttribute('aria-label', expanded ? 'Exit expanded ontology graph' : 'Expand ontology graph');
        window.setTimeout(() => resizeGraph(true), 80);
    };

    window.addEventListener('resize', () => resizeGraph(false));
    document.addEventListener('keydown', event => {
        if (event.key === 'Escape' && panel.classList.contains('ontology-graph-expanded')) {
            window.toggleOntologyFullscreen();
        }
    });
})();
"#;
