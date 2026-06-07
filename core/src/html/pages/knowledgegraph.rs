use crate::element::{AttachmentTarget, Element, RequirementGovernanceMetadata};
use crate::graph_registry::GraphRegistry;
use crate::relation::{self, LinkType};
use crate::report_submodels;
use crate::utils::extract_path_and_fragment;
use maud::{html, Markup, PreEscaped};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
struct KnowledgeGraphData {
    nodes: Vec<KnowledgeGraphNode>,
    edges: Vec<KnowledgeGraphEdge>,
    submodels: Vec<KnowledgeGraphSubmodel>,
    summary: KnowledgeGraphSummary,
}

#[derive(Debug, Clone, Serialize)]
struct KnowledgeGraphSummary {
    elements: usize,
    relations: usize,
    attachments: usize,
    concept_references: usize,
    resources: usize,
    submodels: usize,
}

#[derive(Debug, Clone, Serialize)]
struct KnowledgeGraphSubmodel {
    root_id: String,
    root_name: String,
    root_type: String,
    requirement_count: usize,
}

#[derive(Debug, Clone, Serialize)]
struct KnowledgeGraphNode {
    id: String,
    label: String,
    #[serde(rename = "type")]
    node_type: String,
    element_type: String,
    identifier: String,
    file_path: String,
    line_number: usize,
    link: String,
    description: String,
    metadata: Vec<KnowledgeGraphFact>,
    governance: Vec<KnowledgeGraphFact>,
    outgoing: Vec<KnowledgeGraphFact>,
    incoming: Vec<KnowledgeGraphFact>,
    attachments: Vec<KnowledgeGraphFact>,
    concept_references: Vec<KnowledgeGraphFact>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct KnowledgeGraphFact {
    name: String,
    value: String,
    link: String,
    kind: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct KnowledgeGraphEdge {
    source: String,
    target: String,
    label: String,
    kind: String,
    authored: bool,
}

pub fn render(registry: &GraphRegistry, nav_prefix: &str) -> Markup {
    let graph_data = build_graph_data(registry);
    let graph_json = project_graph_json_from_data(&graph_data);

    let content = html! {
        style { (PreEscaped(KNOWLEDGE_GRAPH_CSS)) }
        section class="knowledge-graph-page" aria-label="Project knowledge graph explorer" {
            div class="knowledge-graph-canvas" {
                div class="knowledge-graph-legend" aria-label="Knowledge graph filters" {
                    div class="kg-legend-title" { "Show" }
                    div class="kg-legend-grid" {
                        button type="button" class="kg-filter-toggle is-active" data-filter-value="capability" aria-pressed="true" { span class="kg-dot kg-dot-capability" {} "Capabilities" }
                        button type="button" class="kg-filter-toggle is-active" data-filter-value="requirement" aria-pressed="true" { span class="kg-dot kg-dot-requirement" {} "Requirements" }
                        button type="button" class="kg-filter-toggle is-active" data-filter-value="refinement" aria-pressed="true" { span class="kg-dot kg-dot-refinement" {} "Refinements" }
                        button type="button" class="kg-filter-toggle is-active" data-filter-value="verification" aria-pressed="true" { span class="kg-dot kg-dot-verification" {} "Verifications" }
                        button type="button" class="kg-filter-toggle is-active" data-filter-value="ontology" aria-pressed="true" { span class="kg-dot kg-dot-ontology" {} "Ontologies" }
                        button type="button" class="kg-filter-toggle" data-filter-value="resource" aria-pressed="false" { span class="kg-dot kg-dot-resource" {} "Resources" }
                        button type="button" class="kg-filter-toggle is-active" data-filter-value="other" aria-pressed="true" { span class="kg-dot kg-dot-other" {} "Other" }
                    }
                    div class="kg-legend-title kg-legend-title-secondary" { "Overlays" }
                    div class="kg-legend-grid" {
                        button type="button" class="kg-overlay-toggle" data-overlay-value="cross" aria-pressed="false" { span class="kg-line-dot" {} "Attachments / concepts" }
                        button type="button" class="kg-overlay-toggle" data-overlay-value="verification" aria-pressed="false" { span class="kg-line-dot" {} "Verification / satisfy" }
                        button type="button" class="kg-overlay-toggle" data-overlay-value="trace" aria-pressed="false" { span class="kg-line-dot" {} "Trace" }
                    }
                }
                div id="knowledge-graph-container" role="img" aria-label="Actual project elements and facts graph" {}
            }
            aside class="knowledge-graph-sidebar" {
                div class="kg-search-panel" {
                    input id="knowledge-graph-search"
                        type="search"
                        placeholder="Search elements, facts, files, relations, concept references"
                        class="kg-search"
                        oninput="filterKnowledgeGraph(this.value)";
                    ul id="knowledge-graph-results" class="kg-results" {}
                }
                div class="kg-inspector-header" {
                    h2 id="knowledge-inspector-title" { "Fact Inspector" }
                    button id="knowledge-inspector-clear" type="button" onclick="clearKnowledgeSelection()" aria-label="Clear selection" { "x" }
                }
                div id="knowledge-inspector-body" class="kg-inspector-body" {
                    p class="kg-empty" {
                        "Search or select a node to inspect actual project facts: element type, relations, attachments, governance, concept references, and source location."
                    }
                }
                div class="kg-sidebar-summary" aria-label="Knowledge graph summary" {
                    span class="kg-summary-entry" { "Submodels " strong { (graph_data.summary.submodels) } }
                    span class="kg-summary-entry" { "Elements " strong { (graph_data.summary.elements) } }
                    span class="kg-summary-entry" { "Relations " strong { (graph_data.summary.relations) } }
                    span class="kg-summary-entry" { "Attachments " strong { (graph_data.summary.attachments) } }
                    span class="kg-summary-entry" { "Concept refs " strong { (graph_data.summary.concept_references) } }
                    span class="kg-summary-entry" { "Resources " strong { (graph_data.summary.resources) } }
                }
            }
        }
        script {
            "const knowledgeGraphData = ";
            (PreEscaped(graph_json));
            ";"
        }
        script type="module" { (PreEscaped(KNOWLEDGE_GRAPH_JS)) }
    };

    crate::html::layouts::base("Knowledge Graph", content, nav_prefix)
}

pub(crate) fn project_graph_json(registry: &GraphRegistry) -> String {
    project_graph_json_from_data(&build_graph_data(registry))
}

fn project_graph_json_from_data(graph_data: &KnowledgeGraphData) -> String {
    serde_json::to_string(graph_data)
        .unwrap_or_else(|_| {
            "{\"nodes\":[],\"edges\":[],\"submodels\":[],\"summary\":{\"elements\":0,\"relations\":0,\"attachments\":0,\"concept_references\":0,\"resources\":0,\"submodels\":0}}".to_string()
        })
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('&', "\\u0026")
}

fn build_graph_data(registry: &GraphRegistry) -> KnowledgeGraphData {
    let mut nodes = BTreeMap::new();
    let mut edges = BTreeMap::new();
    let mut attachment_count = 0;
    let mut concept_reference_count = 0;
    let element_ids = element_identifier_index(registry);
    let submodels = report_submodels::generate_submodels_report(registry, None)
        .map(|report| {
            report
                .submodels
                .into_iter()
                .map(|submodel| KnowledgeGraphSubmodel {
                    root_id: submodel.root_id,
                    root_name: submodel.root_name,
                    root_type: submodel.root_type,
                    requirement_count: submodel.requirement_count,
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    for element in registry.get_all_elements() {
        let id = element.identifier.clone();
        nodes.insert(id.clone(), element_node(registry, element));

        for relation in &element.relations {
            let (target_id, target_label, target_kind, target_link) =
                relation_target_node(registry, &element_ids, &relation.target.link);
            ensure_resource_node(
                &mut nodes,
                &target_id,
                &target_label,
                &target_kind,
                &target_link,
            );
            if let Some((source, target, label)) =
                canonical_knowledge_relation(&id, &target_id, relation.relation_type.name)
            {
                upsert_edge(
                    &mut edges,
                    KnowledgeGraphEdge {
                        source,
                        target,
                        label,
                        kind: if relation.user_created {
                            "authored"
                        } else {
                            "generated"
                        }
                        .to_string(),
                        authored: relation.user_created,
                    },
                );
            }
        }

        for attachment in &element.attachments {
            attachment_count += 1;
            let (target_id, target_label, target_kind, target_link) =
                attachment_target_node(registry, &attachment.target);
            ensure_resource_node(
                &mut nodes,
                &target_id,
                &target_label,
                &target_kind,
                &target_link,
            );
            upsert_edge(
                &mut edges,
                KnowledgeGraphEdge {
                    source: id.clone(),
                    target: target_id,
                    label: "attaches".to_string(),
                    kind: "attachment".to_string(),
                    authored: true,
                },
            );
        }

        for reference in &element.concept_references {
            concept_reference_count += 1;
            let target_id = format!("concept:{}", reference.iri);
            ensure_resource_node(
                &mut nodes,
                &target_id,
                &reference.label,
                "resource",
                &reference.iri,
            );
            upsert_edge(
                &mut edges,
                KnowledgeGraphEdge {
                    source: id.clone(),
                    target: target_id,
                    label: "conceptRef".to_string(),
                    kind: "concept-reference".to_string(),
                    authored: true,
                },
            );
        }
    }

    let edge_set: BTreeSet<_> = edges.into_values().collect();

    populate_node_facts(&mut nodes, &edge_set);

    let resources = nodes
        .values()
        .filter(|node| node.node_type.as_str() == "resource")
        .count();
    let summary = KnowledgeGraphSummary {
        elements: registry.nodes.len(),
        relations: edge_set
            .iter()
            .filter(|edge| edge.label != "attaches" && edge.label != "conceptRef")
            .count(),
        attachments: attachment_count,
        concept_references: concept_reference_count,
        resources,
        submodels: submodels.len(),
    };

    KnowledgeGraphData {
        nodes: nodes.into_values().collect(),
        edges: edge_set.into_iter().collect(),
        submodels,
        summary,
    }
}

fn upsert_edge(
    edges: &mut BTreeMap<(String, String, String), KnowledgeGraphEdge>,
    edge: KnowledgeGraphEdge,
) {
    let key = (edge.source.clone(), edge.target.clone(), edge.label.clone());
    if let Some(existing) = edges.get_mut(&key) {
        if edge.authored {
            existing.authored = true;
            existing.kind = edge.kind;
        }
        return;
    }
    edges.insert(key, edge);
}

fn canonical_knowledge_relation(
    source_id: &str,
    target_id: &str,
    relation_name: &str,
) -> Option<(String, String, String)> {
    if relation::DIAGRAM_RELATIONS.contains(&relation_name) {
        let label = relation::RELATION_TYPES
            .get(relation_name)
            .map(|info| info.label)
            .unwrap_or(relation_name);
        return Some((
            source_id.to_string(),
            target_id.to_string(),
            label.to_string(),
        ));
    }

    let opposite = relation::RELATION_TYPES
        .get(relation_name)
        .and_then(|info| info.opposite)?;
    if !relation::DIAGRAM_RELATIONS.contains(&opposite) {
        return None;
    }
    let label = relation::RELATION_TYPES
        .get(opposite)
        .map(|info| info.label)
        .unwrap_or(opposite);
    Some((
        target_id.to_string(),
        source_id.to_string(),
        label.to_string(),
    ))
}

fn element_node(registry: &GraphRegistry, element: &Element) -> KnowledgeGraphNode {
    let mut metadata: Vec<_> = element
        .metadata
        .iter()
        .filter(|(key, _)| !key.starts_with('_'))
        .map(|(key, value)| KnowledgeGraphFact {
            name: key.clone(),
            value: value.clone(),
            link: String::new(),
            kind: "metadata".to_string(),
        })
        .collect();
    metadata.sort();

    KnowledgeGraphNode {
        id: element.identifier.clone(),
        label: element.name.clone(),
        node_type: element_node_type(element),
        element_type: element.element_type.as_str().to_string(),
        identifier: element.identifier.clone(),
        file_path: element.file_path.clone(),
        line_number: element.line_number,
        link: element_link(element),
        description: element_description(element),
        metadata,
        governance: governance_facts(registry.resolve_governance_metadata(element)),
        outgoing: Vec::new(),
        incoming: Vec::new(),
        attachments: attachment_facts(registry, element),
        concept_references: concept_reference_facts(element),
    }
}

fn populate_node_facts(
    nodes: &mut BTreeMap<String, KnowledgeGraphNode>,
    edges: &BTreeSet<KnowledgeGraphEdge>,
) {
    let labels: BTreeMap<String, String> = nodes
        .iter()
        .map(|(id, node)| (id.clone(), node.label.clone()))
        .collect();
    let links: BTreeMap<String, String> = nodes
        .iter()
        .map(|(id, node)| (id.clone(), node.link.clone()))
        .collect();

    for edge in edges {
        let target_label = labels
            .get(&edge.target)
            .cloned()
            .unwrap_or_else(|| edge.target.clone());
        let source_label = labels
            .get(&edge.source)
            .cloned()
            .unwrap_or_else(|| edge.source.clone());

        if let Some(source) = nodes.get_mut(&edge.source) {
            source.outgoing.push(KnowledgeGraphFact {
                name: edge.label.clone(),
                value: target_label,
                link: links.get(&edge.target).cloned().unwrap_or_default(),
                kind: edge.kind.clone(),
            });
        }
        if let Some(target) = nodes.get_mut(&edge.target) {
            target.incoming.push(KnowledgeGraphFact {
                name: edge.label.clone(),
                value: source_label,
                link: links.get(&edge.source).cloned().unwrap_or_default(),
                kind: edge.kind.clone(),
            });
        }
    }

    for node in nodes.values_mut() {
        node.outgoing.sort();
        node.incoming.sort();
    }
}

fn element_node_type(element: &Element) -> String {
    let element_type = element.element_type.as_str();
    if element_type == "capability" {
        "capability"
    } else if element_type == "requirement" {
        "requirement"
    } else if element_type == "ontology" {
        "ontology"
    } else if element_type.contains("verification") {
        "verification"
    } else if matches!(
        element_type,
        "source"
            | "semantic-contract"
            | "semantic-query-contract"
            | "constraint"
            | "behavior"
            | "specification"
            | "state"
            | "input-output"
    ) {
        "refinement"
    } else {
        "other"
    }
    .to_string()
}

fn element_description(element: &Element) -> String {
    element
        .content
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with("####"))
        .unwrap_or("No details specified.")
        .chars()
        .take(260)
        .collect()
}

fn element_link(element: &Element) -> String {
    let (_, fragment) = extract_path_and_fragment(&element.identifier);
    let html_path = markdown_path_to_html(&element.file_path);
    match fragment {
        Some(fragment) => format!("{}#{}", html_path, fragment),
        None => html_path,
    }
}

fn markdown_path_to_html(path: &str) -> String {
    Path::new(path)
        .with_extension("html")
        .to_string_lossy()
        .to_string()
}

fn governance_facts(governance: Option<RequirementGovernanceMetadata>) -> Vec<KnowledgeGraphFact> {
    let Some(governance) = governance else {
        return Vec::new();
    };

    vec![
        governance_fact("status", governance.status),
        governance_fact("priority", governance.priority),
        governance_fact("risk", governance.risk),
        governance_fact("owner", governance.owner),
    ]
}

fn governance_fact(
    name: &str,
    entry: crate::element::GovernanceMetadataEntry,
) -> KnowledgeGraphFact {
    let source = match entry.source {
        crate::element::GovernanceMetadataSource::Explicit => "explicit",
        crate::element::GovernanceMetadataSource::Inherited => "inherited",
        crate::element::GovernanceMetadataSource::Default => "default",
    };
    let value = if let Some(source_identifier) = entry.source_identifier {
        format!("{} ({}, from {})", entry.value, source, source_identifier)
    } else {
        format!("{} ({})", entry.value, source)
    };

    KnowledgeGraphFact {
        name: name.to_string(),
        value,
        link: String::new(),
        kind: "governance".to_string(),
    }
}

fn attachment_facts(registry: &GraphRegistry, element: &Element) -> Vec<KnowledgeGraphFact> {
    let mut facts = Vec::new();
    for attachment in &element.attachments {
        let (_, label, kind, link) = attachment_target_node(registry, &attachment.target);
        facts.push(KnowledgeGraphFact {
            name: kind,
            value: label,
            link,
            kind: "attachment".to_string(),
        });
    }
    facts.sort();
    facts
}

fn concept_reference_facts(element: &Element) -> Vec<KnowledgeGraphFact> {
    let mut facts = Vec::new();
    for reference in &element.concept_references {
        facts.push(KnowledgeGraphFact {
            name: reference.label.clone(),
            value: reference.iri.clone(),
            link: reference.iri.clone(),
            kind: "concept-reference".to_string(),
        });
    }
    facts.sort();
    facts
}

fn element_identifier_index(registry: &GraphRegistry) -> BTreeMap<String, String> {
    let mut index = BTreeMap::new();
    for element in registry.get_all_elements() {
        index.insert(element.identifier.clone(), element.identifier.clone());
        index.insert(element.id.clone(), element.identifier.clone());
        if let Some(fragment) = extract_path_and_fragment(&element.identifier).1 {
            index.insert(fragment.to_string(), element.identifier.clone());
        }
    }
    index
}

fn relation_target_node(
    registry: &GraphRegistry,
    element_ids: &BTreeMap<String, String>,
    target: &LinkType,
) -> (String, String, String, String) {
    match target {
        LinkType::Identifier(identifier) => {
            if let Some(resolved) = resolve_identifier(element_ids, identifier) {
                if let Some(element) = registry.get_element(&resolved) {
                    return (
                        resolved,
                        element.name.clone(),
                        element_node_type(element),
                        element_link(element),
                    );
                }
            }
            (
                format!("resource:{}", identifier),
                short_resource_label(identifier),
                "resource".to_string(),
                identifier.clone(),
            )
        }
        LinkType::InternalPath(path) => {
            let path = path.to_string_lossy().to_string();
            (
                format!("file:{}", path),
                short_resource_label(&path),
                "resource".to_string(),
                markdown_path_to_html(&path),
            )
        }
        LinkType::ExternalUrl(url) => (
            format!("external:{}", url),
            short_resource_label(url),
            "resource".to_string(),
            url.clone(),
        ),
    }
}

fn attachment_target_node(
    registry: &GraphRegistry,
    target: &AttachmentTarget,
) -> (String, String, String, String) {
    match target {
        AttachmentTarget::ElementIdentifier(identifier) => {
            if let Some(element) = registry.get_element(identifier) {
                return (
                    element.identifier.clone(),
                    element.name.clone(),
                    element_node_type(element),
                    element_link(element),
                );
            }
            (
                format!("resource:{}", identifier),
                short_resource_label(identifier),
                "resource".to_string(),
                identifier.clone(),
            )
        }
        AttachmentTarget::FilePath(path) => {
            let path = path.to_string_lossy().to_string();
            (
                format!("file:{}", path),
                short_resource_label(&path),
                "resource".to_string(),
                markdown_path_to_html(&path),
            )
        }
    }
}

fn resolve_identifier(element_ids: &BTreeMap<String, String>, identifier: &str) -> Option<String> {
    element_ids.get(identifier).cloned().or_else(|| {
        extract_path_and_fragment(identifier)
            .1
            .and_then(|fragment| element_ids.get(fragment).cloned())
    })
}

fn ensure_resource_node(
    nodes: &mut BTreeMap<String, KnowledgeGraphNode>,
    id: &str,
    label: &str,
    node_type: &str,
    link: &str,
) {
    if nodes.contains_key(id) {
        return;
    }
    nodes.insert(
        id.to_string(),
        KnowledgeGraphNode {
            id: id.to_string(),
            label: label.to_string(),
            node_type: node_type.to_string(),
            element_type: node_type.to_string(),
            identifier: id.to_string(),
            file_path: String::new(),
            line_number: 0,
            link: link.to_string(),
            description: "Referenced project fact target.".to_string(),
            metadata: Vec::new(),
            governance: Vec::new(),
            outgoing: Vec::new(),
            incoming: Vec::new(),
            attachments: Vec::new(),
            concept_references: Vec::new(),
        },
    );
}

fn short_resource_label(value: &str) -> String {
    if value.starts_with("http://") || value.starts_with("https://") {
        return value
            .trim_end_matches('/')
            .rsplit(['/', '#'])
            .next()
            .unwrap_or(value)
            .to_string();
    }
    extract_path_and_fragment(value)
        .1
        .map(title_from_fragment)
        .unwrap_or_else(|| {
            Path::new(value)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(value)
                .to_string()
        })
}

fn title_from_fragment(fragment: &str) -> String {
    fragment
        .split(['-', '_'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) const KNOWLEDGE_GRAPH_CSS: &str = r#"
body:has(.knowledge-graph-page) > div.w-full {
  max-width: none;
  width: 100%;
  height: calc(100vh - 50px);
  margin: 0;
  padding: 0;
}
body:has(.knowledge-graph-page) > div.w-full > div.bg-white {
  height: 100%;
  padding: 0;
  border: 0;
  border-radius: 0;
  box-shadow: none;
  background: #d8dfdc;
}
.knowledge-graph-page {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 390px;
  height: calc(100vh - 50px);
  min-height: 520px;
  background: var(--reqvire-surface-base);
  color: #172027;
}
.knowledge-graph-canvas {
  position: relative;
  min-width: 0;
  min-height: 520px;
  height: 100%;
  background: var(--reqvire-surface-canvas);
  overflow: hidden;
}
#knowledge-graph-container {
  width: 100%;
  height: 100%;
  min-height: 520px;
  display: block;
}
.knowledge-graph-sidebar {
  display: flex;
  flex-direction: column;
  min-height: 0;
  border-left: 1px solid #c7c7bf;
  background: var(--reqvire-surface-base);
  overflow: hidden;
}
.knowledge-graph-legend {
  position: absolute;
  top: 12px;
  left: 12px;
  z-index: 3;
  box-sizing: border-box;
  width: 220px;
  max-height: calc(100% - 24px);
  overflow-y: auto;
  padding: 10px;
  border: 1px solid #c7c7bf;
  border-radius: 6px;
  background: var(--reqvire-surface-card);
  box-shadow: 0 2px 6px rgba(28, 28, 28, 0.10);
  color: #334155;
  font-size: 11px;
  line-height: 1.25;
  pointer-events: auto;
}
.kg-legend-title {
  margin-bottom: 6px;
  color: #0f172a;
  font-weight: 700;
  font-size: 12px;
}
.kg-legend-title-secondary {
  margin: 10px 0 5px;
  color: #52605b;
  font-size: 10px;
  letter-spacing: 0;
  text-transform: uppercase;
}
.kg-legend-grid {
  display: grid;
  gap: 4px;
}
.kg-filter-toggle,
.kg-overlay-toggle {
  display: flex;
  align-items: center;
  gap: 5px;
  width: 100%;
  padding: 2px 4px;
  border: 1px solid transparent;
  border-radius: 4px;
  background: transparent;
  color: #334155;
  font: inherit;
  text-align: left;
  cursor: pointer;
}
.kg-filter-toggle:hover,
.kg-overlay-toggle:hover {
  background: var(--reqvire-surface-hover);
}
.kg-filter-toggle:focus-visible,
.kg-overlay-toggle:focus-visible {
  outline: 2px solid #2563eb;
  outline-offset: 1px;
}
.kg-filter-toggle.is-active,
.kg-overlay-toggle.is-active {
  border-color: var(--reqvire-surface-active);
  background: var(--reqvire-surface-active);
  color: var(--reqvire-surface-active-text);
  font-weight: 800;
}
.kg-filter-toggle.is-active::before,
.kg-overlay-toggle.is-active::before {
  content: "✓";
  display: inline-block;
  flex: 0 0 10px;
  width: 10px;
  color: inherit;
  font-weight: 900;
  line-height: 1;
}
.kg-filter-toggle:not(.is-active),
.kg-overlay-toggle:not(.is-active) {
  opacity: 0.45;
}
.kg-line-dot {
  width: 14px;
  height: 2px;
  border-radius: 999px;
  display: inline-block;
  background: #53636b;
}
.kg-dot {
  width: 11px;
  height: 11px;
  border-radius: 3px;
  display: inline-block;
  border: 1px solid rgba(0, 0, 0, 0.24);
}
.kg-dot-capability { background: #1976D2; border-color: #0f4d8a; border-radius: 50%; }
.kg-dot-requirement { background: #673AB7; border-color: #452480; border-radius: 50%; }
.kg-dot-refinement { background: #673AB7; border-color: #452480; }
.kg-dot-verification { background: #4CAF50; border-color: #2f6f32; }
.kg-dot-ontology { background: #B08A00; border-color: #775d00; }
.kg-dot-resource { background: #FFCA28; border-color: #b88c00; }
.kg-dot-other { background: #424242; border-color: #232323; }
.kg-search-panel {
  flex: 0 0 auto;
  padding: 14px;
  border-bottom: 1px solid #d8d8d2;
}
.kg-search {
  width: 100%;
  box-sizing: border-box;
  padding: 8px 10px;
  border: 1px solid #bfc3c7;
  border-radius: 4px;
  background: #fff;
  color: #172027;
  font-size: 13px;
}
.kg-results {
  max-height: 140px;
  margin: 8px 0 0;
  padding: 0;
  overflow: auto;
  list-style: none;
}
.kg-results button {
  display: flex;
  align-items: center;
  gap: 7px;
  width: 100%;
  padding: 6px 8px;
  border: 0;
  border-bottom: 1px solid rgba(37, 48, 58, 0.11);
  background: transparent;
  color: #172027;
  text-align: left;
}
.kg-results button:hover {
  background: var(--reqvire-surface-hover);
}
.kg-result-swatch {
  flex: 0 0 auto;
  width: 11px;
  height: 11px;
  border: 1px solid rgba(0, 0, 0, 0.24);
  border-radius: 3px;
}
.kg-result-swatch[data-node-type="capability"],
.kg-result-swatch[data-node-type="requirement"] {
  border-radius: 50%;
}
.kg-result-label {
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
}
.kg-inspector-header {
  flex: 0 0 auto;
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 14px;
  border-bottom: 1px solid var(--reqvire-surface-active);
  background: var(--reqvire-surface-active);
  color: var(--reqvire-surface-active-text);
}
.kg-inspector-header h2 {
  display: flex;
  align-items: center;
  justify-content: center;
  flex: 1 1 auto;
  margin: 0;
  padding: 0 32px;
  border: 0 !important;
  color: var(--reqvire-surface-active-text) !important;
  font-size: 16px;
  line-height: 1.3;
  font-weight: 800;
  text-align: center;
  text-decoration: none;
}
.kg-inspector-header button {
  position: absolute;
  right: 14px;
  top: 50%;
  transform: translateY(-50%);
  border: 0;
  background: transparent;
  font-size: 22px;
  color: var(--reqvire-surface-active-text);
}
.kg-inspector-header button:hover {
  color: #ffffff;
}
.kg-inspector-body {
  flex: 1;
  min-height: 0;
  overflow: auto;
  padding: 14px;
  background: var(--reqvire-surface-base);
  color: #374151;
  font-size: 13px;
  line-height: 1.5;
}
.kg-empty {
  margin: 0;
  color: #58646b;
  font-style: italic;
}
.kg-section {
  padding: 14px 0;
  border-top: 1px solid rgba(37, 48, 58, 0.12);
}
.kg-section:first-child {
  border-top: 0;
  padding-top: 0;
}
.kg-section h3 {
  margin: 0 0 8px;
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.04em;
}
.kg-pill {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  max-width: 100%;
  margin: 0 6px 6px 0;
  padding: 5px 8px;
  border: 1px solid #9bb7d2;
  border-radius: 5px;
  background: var(--reqvire-surface-muted);
  color: #173353;
  font-size: 12px;
  line-height: 1.25;
  overflow-wrap: anywhere;
}
.kg-fact {
  display: grid;
  grid-template-columns: minmax(0, 1fr);
  gap: 3px;
  width: 100%;
  box-sizing: border-box;
  margin: 0 0 6px;
  padding: 7px 9px;
  border: 1px solid #bdc8c4;
  border-radius: 5px;
  background: var(--reqvire-surface-muted);
  border-color: var(--reqvire-surface-border);
  color: #1f2a31;
}
.kg-fact-name {
  color: #52605b;
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 0.04em;
  line-height: 1.2;
  text-transform: uppercase;
}
.kg-fact-value {
  min-width: 0;
  overflow-wrap: anywhere;
  word-break: break-word;
}
.kg-fact a {
  color: #174a75;
  text-decoration: none;
}
.kg-mono {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
  overflow-wrap: anywhere;
}
.kg-sidebar-summary {
  flex: 0 0 auto;
  display: flex;
  gap: 8px;
  align-items: center;
  justify-content: center;
  overflow-x: auto;
  padding: 5px 8px;
  border-top: 1px solid #d8d8d2;
  background: var(--reqvire-surface-hover);
  color: #64748b;
  font-size: 10px;
  line-height: 1.2;
  white-space: nowrap;
}
.kg-summary-entry strong {
  color: #111827;
  font-size: 11px;
  font-weight: 700;
}
.kg-render-notice {
  position: absolute;
  left: 260px;
  top: 72px;
  z-index: 2;
  max-width: 480px;
  padding: 10px 12px;
  border: 1px solid #bfc7c2;
  border-radius: 5px;
  background: var(--reqvire-surface-card);
  color: #334155;
  font-size: 13px;
}
"#;

pub(crate) const KNOWLEDGE_GRAPH_JS: &str = r#"
import Graph from 'https://cdn.jsdelivr.net/npm/graphology@0.25.4/+esm';
import { Sigma } from 'https://cdn.jsdelivr.net/npm/sigma@3.0.0/+esm';
import forceAtlas2 from 'https://cdn.jsdelivr.net/npm/graphology-layout-forceatlas2@0.10.1/+esm';

const KG_LOG_PREFIX = '[Reqvire KG]';
const kgLog = (...args) => console.info(KG_LOG_PREFIX, ...args);
const kgWarn = (...args) => console.warn(KG_LOG_PREFIX, ...args);
const kgError = (...args) => console.error(KG_LOG_PREFIX, ...args);

// Prefer the central Explorer Project Store knowledge_graph projection when the SPA
// runtime is present; embedded data remains available for projection rendering tests.
function resolveKnowledgeGraphSource() {
  try {
    const store = window.ReqvireExplorerStore
      && typeof window.ReqvireExplorerStore.getStore === 'function'
      && window.ReqvireExplorerStore.getStore();
    const projection = store && store.knowledge_graph;
    if (projection && Array.isArray(projection.nodes) && Array.isArray(projection.edges)) {
      kgLog('using central project store knowledge_graph projection');
      return projection;
    }
  } catch (error) {
    kgWarn('central store projection unavailable; using page-local knowledgeGraphData', error);
  }
  return knowledgeGraphData;
}

const container = document.getElementById('knowledge-graph-container');
const sourceGraph = resolveKnowledgeGraphSource();
const data = {
  nodes: sourceGraph.nodes.map(d => ({ ...d, node_type: d.node_type || d.type || 'other' })),
  edges: sourceGraph.edges.map(d => ({ ...d })),
  submodels: sourceGraph.submodels || []
};
const nodeById = new Map(data.nodes.map(d => [d.id, d]));
const originalEdgeCount = data.edges.length;
data.edges = data.edges.filter(d => nodeById.has(d.source) && nodeById.has(d.target));
const activeTypes = new Set(['capability', 'requirement', 'refinement', 'verification', 'ontology', 'other']);
const activeOverlays = new Set();
let selectedId = null;
let hoveredId = null;
let suppressNextStageClear = false;
let searchTerm = '';
let renderer = null;
let graph = null;

window.filterKnowledgeGraph = function(value) {
  searchTerm = (value || '').trim().toLowerCase();
  renderSearchResults();
  applyFilters();
};
window.clearKnowledgeSelection = clearKnowledgeSelection;

try {
  renderKnowledgeGraph();
} catch (error) {
  kgError('Sigma/Graphology renderer failed', error);
  showGraphNotice('Knowledge Graph renderer failed. Check the browser console for Sigma/Graphology errors.');
}

function renderKnowledgeGraph() {
  if (!container) {
    kgError('container element not found');
    return;
  }
  ensureCanvasSize();
  if (!data.nodes.length) {
    kgWarn('no nodes exported');
    showGraphNotice('No project graph nodes were exported.');
    return;
  }

  kgLog('bootstrap', {
    renderer: 'sigma-graphology',
    nodes: data.nodes.length,
    edges: data.edges.length,
    droppedEdges: originalEdgeCount - data.edges.length,
    containerWidth: container.clientWidth,
    containerHeight: container.clientHeight
  });

  graph = new Graph({ type: 'directed', multi: true, allowSelfLoops: true });
  assignInitialPositions(data.nodes);

  data.nodes.forEach(node => {
    graph.addNode(node.id, {
      ...node,
      type: 'circle',
      reqvireType: node.type,
      label: truncate(node.label, nodeLabelLimit(node)),
      fullLabel: node.label,
      x: node.x,
      y: node.y,
      size: nodeSize(node),
      color: graphNodeFill(node),
      hidden: !visibleNode(node)
    });
  });

  data.edges.forEach((edge, index) => {
    if (edgeParticipatesInLayout(edge)) addKnowledgeEdge(edge, index);
  });

  applyForceAtlas2Layout();

  data.edges.forEach((edge, index) => {
    if (!edgeParticipatesInLayout(edge)) addKnowledgeEdge(edge, index);
  });

  renderer = new Sigma(graph, container, {
    allowInvalidContainer: true,
    defaultEdgeType: 'arrow',
    renderEdgeLabels: true,
    labelDensity: 0.12,
    labelGridCellSize: 80,
    labelRenderedSizeThreshold: 9,
    nodeReducer: (node, attributes) => {
      const result = { ...attributes };
      const focusIds = activeFocusIds();
      result.focused = focusIds.includes(node);
      result.inFocusNeighborhood = focusIds.length > 0 && isInAnyFocusNeighborhood(node, focusIds);
      if (result.focused) {
        result.label = attributes.fullLabel || attributes.label || '';
        result.forceLabel = true;
      }
      if (focusIds.length > 0 && !result.inFocusNeighborhood) {
        result.color = dimNodeColor(attributes.color || '#8da0ae', 0.2);
        result.label = '';
        result.forceLabel = false;
      }
      if (result.inFocusNeighborhood) {
        result.forceLabel = true;
      }
      return result;
    },
    edgeReducer: (_edge, attributes) => {
      const result = { ...attributes };
      const focusIds = activeFocusIds();
      if (focusIds.length === 0 || attributes.hidden) {
        result.hidden = true;
        return result;
      }
      if (!isIncidentToAnyFocus(attributes, focusIds)) {
        result.hidden = true;
        result.label = '';
        result.forceLabel = false;
      } else {
        result.hidden = false;
        result.color = '#53636b';
        result.size = Math.max(1.1, (attributes.size || 1) * 1.15);
        result.forceLabel = true;
      }
      return result;
    }
  });

  renderer.on('clickNode', event => {
    suppressNextStageClear = true;
    selectNode(event.node);
  });
  renderer.on('clickStage', () => {
    if (suppressNextStageClear) {
      suppressNextStageClear = false;
      return;
    }
    clearKnowledgeSelection();
  });
  renderer.on('enterNode', event => {
    hoveredId = event.node;
    renderer.refresh();
  });
  renderer.on('leaveNode', event => {
    if (hoveredId === event.node) {
      hoveredId = null;
      renderer.refresh();
    }
  });

  document.querySelectorAll('.kg-filter-toggle').forEach(button => {
    button.addEventListener('click', () => {
      const value = button.dataset.filterValue;
      if (activeTypes.has(value)) {
        activeTypes.delete(value);
        button.classList.remove('is-active');
        button.setAttribute('aria-pressed', 'false');
      } else {
        activeTypes.add(value);
        button.classList.add('is-active');
        button.setAttribute('aria-pressed', 'true');
      }
      applyFilters();
    });
  });

  document.querySelectorAll('.kg-overlay-toggle').forEach(button => {
    button.addEventListener('click', () => {
      const value = button.dataset.overlayValue;
      if (activeOverlays.has(value)) {
        activeOverlays.delete(value);
        button.classList.remove('is-active');
        button.setAttribute('aria-pressed', 'false');
      } else {
        activeOverlays.add(value);
        button.classList.add('is-active');
        button.setAttribute('aria-pressed', 'true');
      }
      applyFilters();
    });
  });

  applyFilters();
  fitVisibleGraph();
  kgLog('sigma renderer started', {
    order: graph.order,
    size: graph.size,
    visibleNodes: countVisibleNodes(),
    visibleEdges: countVisibleEdges()
  });
}

function addKnowledgeEdge(edge, index) {
  const key = `e${index}`;
  if (graph.hasEdge(key)) return;
  graph.addDirectedEdgeWithKey(key, edge.source, edge.target, {
    ...edge,
    type: 'arrow',
    reqvireType: edge.type,
    relCategory: relationCategory(edge),
    label: edge.label,
    size: edgeSize(edge),
    color: '#6d7b83',
    hidden: !visibleEdge(edge)
  });
}

function applyForceAtlas2Layout() {
  try {
    const settings = forceAtlas2.inferSettings(graph);
    forceAtlas2.assign(graph, {
      iterations: graph.order > 650 ? 260 : 180,
      settings: {
        ...settings,
        adjustSizes: true,
        barnesHutOptimize: true,
        gravity: 1.6,
        scalingRatio: 18,
        slowDown: 2
      }
    });
    separateOverlappingNodes();
    kgLog('forceatlas2 layout assigned', { order: graph.order, size: graph.size });
  } catch (error) {
    kgError('forceatlas2 layout failed; using deterministic initial positions', error);
  }
}

function ensureCanvasSize() {
  const minimumHeight = Math.max(window.innerHeight - 50, 520);
  if (!container.clientHeight || container.clientHeight < 20) {
    container.style.height = `${minimumHeight}px`;
  }
}

function assignInitialPositions(nodes) {
  const typeBuckets = new Map();
  nodes.forEach(node => {
    const bucket = node.node_type || 'other';
    if (!typeBuckets.has(bucket)) typeBuckets.set(bucket, []);
    typeBuckets.get(bucket).push(node);
  });
  const centers = {
    capability: [-8, -5],
    requirement: [-1, -2],
    refinement: [3, 2],
    verification: [8, 5],
    ontology: [-5, -8],
    resource: [10, 8],
    other: [0, 0]
  };
  for (const [type, bucket] of typeBuckets.entries()) {
    const [cx, cy] = centers[type] || centers.other;
    const radius = Math.max(2.5, Math.sqrt(bucket.length) * 0.9);
    bucket.forEach((node, index) => {
      const angle = (index / Math.max(bucket.length, 1)) * Math.PI * 2;
      const ring = radius * (0.45 + (index % 11) / 11);
      node.x = cx + Math.cos(angle) * ring;
      node.y = cy + Math.sin(angle) * ring;
    });
  }
}

function separateOverlappingNodes() {
  const seen = new Map();
  graph.forEachNode((node, attributes) => {
    const key = `${Math.round(attributes.x * 10)}:${Math.round(attributes.y * 10)}`;
    const count = seen.get(key) || 0;
    seen.set(key, count + 1);
    if (count > 0) {
      graph.mergeNodeAttributes(node, {
        x: attributes.x + Math.cos(count) * count * 0.12,
        y: attributes.y + Math.sin(count) * count * 0.12
      });
    }
  });
}

function visibleNode(d) {
  if (!activeTypes.has(d.node_type)) return false;
  if (!searchTerm) return true;
  return searchCorpus(d).includes(searchTerm);
}

function visibleEdge(edge) {
  const source = nodeById.get(edge.source);
  const target = nodeById.get(edge.target);
  return source && target && visibleNode(source) && visibleNode(target) && edgeVisibleByOverlay(edge);
}

function edgeVisibleByOverlay(edge) {
  const category = relationCategory(edge);
  if (category === 'attach' || category === 'concept-reference') return activeOverlays.has('cross');
  if (category === 'verify' || category === 'satisfy') return activeOverlays.has('verification');
  if (category === 'trace') return activeOverlays.has('trace');
  return true;
}

function edgeParticipatesInLayout(edge) {
  return isSubmodelStructureCategory(relationCategory(edge));
}

function isSubmodelStructureCategory(category) {
  return category === 'derive' || category === 'specify' || category === 'refine';
}

function relationCategory(edge) {
  const label = String(edge.label || '').toLowerCase();
  const kind = String(edge.kind || '').toLowerCase();
  if (kind === 'attachment' || label === 'attaches') return 'attach';
  if (kind === 'concept-reference' || label === 'conceptref') return 'concept-reference';
  if (label.includes('derive')) return 'derive';
  if (label.includes('specif')) return 'specify';
  if (label.includes('refine')) return 'refine';
  if (label.includes('verif')) return 'verify';
  if (label.includes('satisf')) return 'satisfy';
  if (label.includes('trace')) return 'trace';
  return 'trace';
}

function activeFocusIds() {
  const ids = [];
  if (selectedId) ids.push(selectedId);
  if (hoveredId && hoveredId !== selectedId) ids.push(hoveredId);
  return ids;
}

function isAdjacentToFocus(node, focusId) {
  if (!focusId) return false;
  return data.edges.some(edge =>
    edgeVisibleByOverlay(edge)
      && ((edge.source === focusId && edge.target === node) || (edge.target === focusId && edge.source === node))
  );
}

function isInAnyFocusNeighborhood(node, focusIds) {
  return focusIds.some(focusId => node === focusId || isAdjacentToFocus(node, focusId));
}

function isIncidentToAnyFocus(edge, focusIds) {
  return focusIds.some(focusId => edge.source === focusId || edge.target === focusId);
}

function dimNodeColor(color, alpha) {
  if (!color || !color.startsWith('#')) return color;
  const foreground = parseHexColor(color);
  const surfaceBase = getComputedStyle(document.documentElement).getPropertyValue('--reqvire-surface-base').trim();
  const background = parseHexColor(surfaceBase);
  if (!foreground || !background) return color;
  const r = Math.round(foreground.r * alpha + background.r * (1 - alpha));
  const g = Math.round(foreground.g * alpha + background.g * (1 - alpha));
  const b = Math.round(foreground.b * alpha + background.b * (1 - alpha));
  return rgbToHex(r, g, b);
}

function parseHexColor(color) {
  const hex = color.slice(1);
  const value = hex.length === 3
    ? hex.split('').map(part => part + part).join('')
    : hex.padEnd(6, '0').slice(0, 6);
  const r = parseInt(value.slice(0, 2), 16);
  const g = parseInt(value.slice(2, 4), 16);
  const b = parseInt(value.slice(4, 6), 16);
  if ([r, g, b].some(component => Number.isNaN(component))) return null;
  return { r, g, b };
}

function rgbToHex(r, g, b) {
  return `#${[r, g, b].map(component => component.toString(16).padStart(2, '0')).join('')}`;
}

function applyFilters() {
  if (!graph) return;
  data.nodes.forEach(node => {
    if (graph.hasNode(node.id)) {
      graph.setNodeAttribute(node.id, 'hidden', !visibleNode(node));
    }
  });
  data.edges.forEach((_edge, index) => {
    const key = `e${index}`;
    if (graph.hasEdge(key)) {
      graph.setEdgeAttribute(key, 'hidden', !visibleEdge(_edge));
    }
  });
  if (renderer) renderer.refresh();
  kgLog('filters applied', {
    activeTypes: Array.from(activeTypes).sort(),
    searchTerm,
    visibleNodes: countVisibleNodes(),
    visibleEdges: countVisibleEdges()
  });
}

function fitVisibleGraph() {
  if (!renderer || !graph) return;
  const visible = [];
  graph.forEachNode((node, attributes) => {
    if (!attributes.hidden) visible.push(node);
  });
  if (!visible.length) return;
  renderer.getCamera().animatedReset({ duration: 250 });
}

function countVisibleNodes() {
  if (!graph) return 0;
  let count = 0;
  graph.forEachNode((_node, attributes) => {
    if (!attributes.hidden) count += 1;
  });
  return count;
}

function countVisibleEdges() {
  if (!graph) return 0;
  let count = 0;
  graph.forEachEdge((_edge, attributes) => {
    if (!attributes.hidden) count += 1;
  });
  return count;
}

function renderSearchResults() {
  const results = document.getElementById('knowledge-graph-results');
  if (!results) return;
  results.innerHTML = '';
  if (!searchTerm) return;
  data.nodes.filter(visibleNode).slice(0, 20).forEach(d => {
    const item = document.createElement('li');
    const button = document.createElement('button');
    button.type = 'button';
    const swatch = document.createElement('span');
    swatch.className = 'kg-result-swatch';
    swatch.dataset.nodeType = d.node_type || 'other';
    swatch.style.background = nodeFill(d);
    const label = document.createElement('span');
    label.className = 'kg-result-label';
    label.textContent = d.label;
    button.appendChild(swatch);
    button.appendChild(label);
    button.onclick = () => selectNode(d.id);
    item.appendChild(button);
    results.appendChild(item);
  });
}

function selectNode(id) {
  selectedId = id;
  const selected = data.nodes.find(d => d.id === id);
  if (renderer) renderer.refresh();
  if (!selected) return;
  centerOnNode(id);
  document.getElementById('knowledge-inspector-title').textContent = selected.label;
  document.getElementById('knowledge-inspector-body').innerHTML = inspectorHtml(selected);
}

function centerOnNode(id) {
  if (!renderer || !graph || !graph.hasNode(id)) return;
  const display = renderer.getNodeDisplayData(id);
  if (!display) return;
  const camera = renderer.getCamera();
  const state = camera.getState();
  camera.animate(
    { x: display.x, y: display.y, ratio: Math.min(state.ratio, 0.9) },
    { duration: 280 }
  );
}

function clearKnowledgeSelection() {
  selectedId = null;
  if (renderer) renderer.refresh();
  document.getElementById('knowledge-inspector-title').textContent = 'Fact Inspector';
  document.getElementById('knowledge-inspector-body').innerHTML = '<p class="kg-empty">Search or select a node to inspect actual project facts: element type, relations, attachments, governance, concept references, and source location.</p>';
}

function inspectorHtml(d) {
  return [
    section('Kind', kindPillHtml(d)),
    section('Identifier', fieldHtml('id', `<span class="kg-mono">${escapeHtml(d.identifier)}</span>`)),
    d.file_path ? section('Source', fieldHtml('file', `<a href="${escapeAttr(d.link)}">${escapeHtml(d.file_path)}:${d.line_number}</a>`)) : '',
    section('Description', `<p>${escapeHtml(d.description || 'None specified.')}</p>`),
    factsSection('Governance', d.governance),
    factsSection('Metadata', d.metadata),
    factsSection('Outgoing Facts', d.outgoing),
    factsSection('Incoming Facts', d.incoming),
    factsSection('Attachments', d.attachments),
    factsSection('Concept References', d.concept_references)
  ].filter(Boolean).join('');
}

function kindPillHtml(d) {
  const background = nodeFill(d);
  const color = readableTextColor(background);
  return `<span class="kg-pill" style="background:${background};border-color:${nodeBorder(d)};color:${color}">${escapeHtml(d.element_type || d.node_type)}</span>`;
}

function factsSection(title, facts) {
  if (!facts || facts.length === 0) return '';
  return section(title, facts.map(f => {
    const value = f.link ? `<a href="${escapeAttr(f.link)}">${escapeHtml(f.value)}</a>` : escapeHtml(f.value);
    return fieldHtml(f.name, value);
  }).join(''));
}

function fieldHtml(name, value) {
  return `<div class="kg-fact"><span class="kg-fact-name">${escapeHtml(name)}</span><span class="kg-fact-value">${value}</span></div>`;
}

function section(title, body) {
  return `<div class="kg-section"><h3>${escapeHtml(title)}</h3>${body}</div>`;
}

function searchCorpus(d) {
  const facts = [].concat(d.metadata || [], d.governance || [], d.outgoing || [], d.incoming || [], d.attachments || [], d.concept_references || []);
  return [d.label, d.element_type, d.identifier, d.file_path, d.description]
    .concat(facts.flatMap(f => [f.name, f.value, f.kind]))
    .join(' ').toLowerCase();
}

function nodeLabelLimit(node) {
  return ['capability', 'requirement', 'ontology'].includes(node.node_type) ? 26 : 34;
}

function nodeSize(node) {
  const degree = data.edges.filter(edge => edge.source === node.id || edge.target === node.id).length;
  return Math.min(16, 4 + Math.sqrt(degree + 1) * 1.6);
}

function edgeSize(edge) {
  return edge.kind === 'attachment' || edge.kind === 'concept-reference' ? 0.8 : 1.1;
}

function nodeFill(d) {
  return ({
    capability: '#1976D2',
    requirement: '#673AB7',
    refinement: '#673AB7',
    verification: '#4CAF50',
    ontology: '#B08A00',
    resource: '#FFCA28',
    other: '#424242'
  })[d.node_type] || '#424242';
}

function graphNodeFill(d) {
  return nodeFill(d);
}

function nodeBorder(d) {
  return ({
    capability: '#0f4d8a',
    requirement: '#452480',
    refinement: '#452480',
    verification: '#2f6f32',
    ontology: '#775d00',
    resource: '#b88c00',
    other: '#232323'
  })[d.node_type] || '#232323';
}

function readableTextColor(background) {
  if (!background || !background.startsWith('#')) return '#172027';
  const value = background.slice(1).padEnd(6, '0').slice(0, 6);
  const r = parseInt(value.slice(0, 2), 16);
  const g = parseInt(value.slice(2, 4), 16);
  const b = parseInt(value.slice(4, 6), 16);
  const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;
  return luminance > 0.55 ? '#172027' : '#ffffff';
}

function truncate(value, max) {
  value = value || '';
  return value.length > max ? value.slice(0, Math.max(1, max - 1)) + '…' : value;
}

function escapeHtml(value) {
  return String(value ?? '').replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
}

function escapeAttr(value) {
  return escapeHtml(value).replace(/`/g, '&#96;');
}

function showGraphNotice(message) {
  if (!container) return;
  const notice = document.createElement('div');
  notice.className = 'kg-render-notice';
  notice.textContent = message;
  container.appendChild(notice);
}
"#;
