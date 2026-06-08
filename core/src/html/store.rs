use crate::element::{AttachmentTarget, Element, GovernanceMetadataSource};
use crate::graph_registry::GraphRegistry;
use crate::ontology_graph::build_graph_data;
use crate::relation::{self, LinkType};
use crate::semantic_contract::SemanticIndex;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path};

const SCHEMA_VERSION: &str = "2026-06-07.project-store.v1";

#[derive(Debug, Clone, Serialize)]
pub struct ExplorerProjectStore {
    pub schema_version: &'static str,
    pub project: ProjectStoreProject,
    pub folders: Vec<ProjectStoreFolder>,
    pub files: Vec<ProjectStoreFile>,
    pub resources: Vec<ProjectStoreResource>,
    pub elements: Vec<ProjectStoreElement>,
    pub relations: Vec<ProjectStoreRelation>,
    pub attachments: Vec<ProjectStoreAttachment>,
    pub concept_refs: Vec<ProjectStoreConceptReference>,
    pub submodels: Value,
    pub traces: Value,
    pub coverage: Value,
    pub ontology: Value,
    pub knowledge_graph: Value,
    pub search: Vec<ProjectStoreSearchDocument>,
    pub summaries: ProjectStoreSummaries,
    pub routes: ProjectStoreRoutes,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectStoreProject {
    pub name: String,
    pub root_label: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectStoreFolder {
    pub path: String,
    pub parent: Option<String>,
    pub children: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectStoreFile {
    pub path: String,
    pub display_path: String,
    pub markdown_content: String,
    pub parent_folder: String,
    pub element_ids: Vec<String>,
    pub resource_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectStoreResource {
    pub id: String,
    pub kind: String,
    pub target: String,
    pub display: String,
    pub file_path: Option<String>,
    pub source_text: Option<String>,
    pub external_url: Option<String>,
    pub referring_element_ids: Vec<String>,
    pub relation_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectStoreElement {
    pub id: String,
    pub name: String,
    pub element_type: String,
    pub type_family: String,
    pub file_path: String,
    pub line_number: usize,
    pub source_anchor: String,
    pub content: String,
    pub metadata: BTreeMap<String, String>,
    pub governance: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectStoreRelation {
    pub id: String,
    pub source_id: String,
    pub target_id: String,
    pub target_kind: String,
    pub relation_type: String,
    pub canonical_relation_type: String,
    pub source_relation_types: Vec<String>,
    pub authored: bool,
    pub generated_opposite: bool,
    pub resource_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectStoreAttachment {
    pub id: String,
    pub source_id: String,
    pub target: String,
    pub target_kind: String,
    pub resource_id: Option<String>,
    pub content_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectStoreConceptReference {
    pub id: String,
    pub source_id: String,
    pub label: String,
    pub iri: String,
    pub line_number: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectStoreSearchDocument {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub route: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectStoreSummaries {
    pub elements: usize,
    pub files: usize,
    pub folders: usize,
    pub resources: usize,
    pub relations: usize,
    pub attachments: usize,
    pub concept_refs: usize,
    pub ontology_blocks: usize,
    pub shape_blocks: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectStoreRoutes {
    pub canonical: Vec<ProjectStoreRoute>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectStoreRoute {
    pub id: String,
    pub pattern: String,
    pub title: String,
}

pub fn build_project_store(
    registry: &GraphRegistry,
    semantic_index: &SemanticIndex,
) -> ExplorerProjectStore {
    let elements = build_elements(registry);
    let (relations, mut resources) = build_relations(registry);
    let attachments = build_attachments(registry, &mut resources);
    let concept_refs = build_concept_refs(registry, &mut resources);
    enrich_resource_sources(&mut resources);
    let (files, folders) = build_files_and_folders(registry, &resources);
    let search = build_search_documents(&elements, &files, &resources);
    let submodels = crate::report_submodels::generate_submodels_report(registry, None)
        .ok()
        .and_then(|report| serde_json::to_value(report).ok())
        .unwrap_or_else(|| json!({"submodels":[],"cross_submodel_couplings":[],"summary":{}}));
    let knowledge_graph = build_knowledge_graph_projection(
        &elements,
        &relations,
        &attachments,
        &concept_refs,
        &resources,
        &submodels,
    );
    let ontology = json!({
        "summary": semantic_index.summary,
        "blocks": semantic_index.blocks,
        "diagnostics": semantic_index.diagnostics,
        "declarations": semantic_index.ontology_declarations,
        "shape_references": semantic_index.shape_references,
        "projection": semantic_index.ontology_projection,
        "graph_data": build_graph_data(semantic_index),
        "ttl_href": "ontologies.ttl"
    });
    let trace_report =
        crate::verification_trace::VerificationTraceGenerator::new(registry, false, None)
            .generate();
    let traces = serde_json::to_value(trace_report).unwrap_or_else(|_| json!({"files":{}}));
    let coverage = serde_json::to_value(crate::report_coverage::generate_coverage_report(registry))
        .unwrap_or_else(|_| json!({}));
    let summaries = ProjectStoreSummaries {
        elements: elements.len(),
        files: files.len(),
        folders: folders.len(),
        resources: resources.len(),
        relations: relations.len(),
        attachments: attachments.len(),
        concept_refs: concept_refs.len(),
        ontology_blocks: semantic_index.summary.ontology_blocks,
        shape_blocks: semantic_index.summary.shape_blocks,
    };

    ExplorerProjectStore {
        schema_version: SCHEMA_VERSION,
        project: ProjectStoreProject {
            name: "Reqvire project".to_string(),
            root_label: "Reqvire root".to_string(),
        },
        folders,
        files,
        resources: resources.into_values().collect(),
        elements,
        relations,
        attachments,
        concept_refs,
        submodels,
        traces,
        coverage,
        ontology,
        knowledge_graph,
        search,
        summaries,
        routes: default_routes(),
    }
}

pub fn project_store_javascript(
    store: &ExplorerProjectStore,
) -> Result<String, crate::error::ReqvireError> {
    let json = serde_json::to_string_pretty(store)
        .map_err(|error| crate::error::ReqvireError::SerializationError(error.to_string()))?;
    let escaped = json
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('&', "\\u0026")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029");

    Ok(format!("window.reqvireProjectStore = {escaped};\n"))
}

fn build_knowledge_graph_projection(
    elements: &[ProjectStoreElement],
    relations: &[ProjectStoreRelation],
    attachments: &[ProjectStoreAttachment],
    concept_refs: &[ProjectStoreConceptReference],
    resources: &BTreeMap<String, ProjectStoreResource>,
    submodels: &Value,
) -> Value {
    let element_ids: BTreeSet<&str> = elements.iter().map(|element| element.id.as_str()).collect();
    let mut nodes = Vec::new();

    for element in elements {
        let metadata = element
            .metadata
            .iter()
            .map(|(name, value)| json!({"name": name, "value": value, "kind": "metadata"}))
            .collect::<Vec<_>>();
        let governance = element
            .governance
            .iter()
            .map(|(name, value)| json!({"name": name, "value": value, "kind": "governance"}))
            .collect::<Vec<_>>();
        let outgoing = relations
            .iter()
            .filter(|relation| relation.source_id == element.id)
            .map(|relation| {
                json!({
                    "name": relation.canonical_relation_type,
                    "value": relation.target_id,
                    "link": relation_link(&relation.target_id, &relation.resource_id),
                    "kind": if relation.authored { "authored" } else { "generated" }
                })
            })
            .collect::<Vec<_>>();
        let incoming = relations
            .iter()
            .filter(|relation| relation.target_id == element.id)
            .map(|relation| {
                json!({
                    "name": relation.canonical_relation_type,
                    "value": relation.source_id,
                    "link": format!("#/elements/{}", relation.source_id),
                    "kind": if relation.authored { "authored" } else { "generated" }
                })
            })
            .collect::<Vec<_>>();
        let attachment_facts = attachments
            .iter()
            .filter(|attachment| attachment.source_id == element.id)
            .map(|attachment| {
                json!({
                    "name": "attaches",
                    "value": attachment.target,
                    "link": relation_link(&attachment.target, &attachment.resource_id),
                    "kind": attachment.target_kind
                })
            })
            .collect::<Vec<_>>();
        let concept_reference_facts = concept_refs
            .iter()
            .filter(|concept_ref| concept_ref.source_id == element.id)
            .map(|concept_ref| {
                json!({
                    "name": concept_ref.label,
                    "value": concept_ref.iri,
                    "kind": "concept-reference"
                })
            })
            .collect::<Vec<_>>();

        nodes.push(json!({
            "id": element.id,
            "identifier": element.id,
            "label": element.name,
            "type": element.type_family,
            "node_type": element.type_family,
            "element_type": element.element_type,
            "file_path": element.file_path,
            "line_number": element.line_number,
            "link": element.source_anchor,
            "description": element.content.lines().find(|line| !line.trim().is_empty()).unwrap_or(""),
            "metadata": metadata,
            "governance": governance,
            "outgoing": outgoing,
            "incoming": incoming,
            "attachments": attachment_facts,
            "concept_references": concept_reference_facts
        }));
    }

    for resource in resources.values() {
        nodes.push(json!({
            "id": resource.id,
            "identifier": resource.id,
            "label": resource.display,
            "type": "resource",
            "node_type": "resource",
            "element_type": resource.kind,
            "file_path": resource.file_path.clone().unwrap_or_default(),
            "line_number": 0,
            "link": resource.external_url.clone().unwrap_or_else(|| resource.target.clone()),
            "description": resource.target,
            "metadata": [],
            "governance": [],
            "outgoing": [],
            "incoming": [],
            "attachments": [],
            "concept_references": []
        }));
    }

    let relation_edges = relations.iter().filter_map(|relation| {
        let target = relation
            .resource_id
            .as_ref()
            .filter(|resource_id| resources.contains_key(resource_id.as_str()))
            .cloned()
            .unwrap_or_else(|| relation.target_id.clone());
        if !element_ids.contains(relation.source_id.as_str())
            || (!element_ids.contains(target.as_str()) && !resources.contains_key(&target))
        {
            return None;
        }
        Some(json!({
            "source": relation.source_id,
            "target": target,
            "label": relation.canonical_relation_type,
            "kind": if relation.authored { "authored" } else { "generated" },
            "authored": relation.authored
        }))
    });

    let attachment_edges = attachments.iter().filter_map(|attachment| {
        let target = attachment
            .resource_id
            .as_ref()
            .filter(|resource_id| resources.contains_key(resource_id.as_str()))
            .cloned()
            .unwrap_or_else(|| attachment.target.clone());
        if !element_ids.contains(attachment.source_id.as_str())
            || (!element_ids.contains(target.as_str()) && !resources.contains_key(&target))
        {
            return None;
        }
        Some(json!({
            "source": attachment.source_id,
            "target": target,
            "label": "attaches",
            "kind": "attachment",
            "authored": true
        }))
    });

    let concept_edges = concept_refs.iter().map(|concept_ref| {
        let concept_id = format!("concept:{}", concept_ref.iri);
        json!({
            "source": concept_ref.source_id,
            "target": concept_id,
            "label": "conceptRef",
            "kind": "concept-reference",
            "authored": true
        })
    });

    let mut concept_nodes: BTreeMap<String, Value> = BTreeMap::new();
    for concept_ref in concept_refs {
        let concept_id = format!("concept:{}", concept_ref.iri);
        concept_nodes.entry(concept_id.clone()).or_insert_with(|| {
            json!({
                "id": concept_id,
                "identifier": concept_ref.iri,
                "label": concept_ref.label,
                "type": "concept",
                "node_type": "concept",
                "element_type": "concept-reference",
                "file_path": "",
                "line_number": concept_ref.line_number,
                "link": "",
                "description": concept_ref.iri,
                "metadata": [],
                "governance": [],
                "outgoing": [],
                "incoming": [],
                "attachments": [],
                "concept_references": []
            })
        });
    }

    nodes.extend(concept_nodes.into_values());
    let edges = relation_edges
        .chain(attachment_edges)
        .chain(concept_edges)
        .collect::<Vec<_>>();

    let submodel_nodes = submodels
        .get("submodels")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let submodel_count = submodel_nodes.len();

    json!({
        "nodes": nodes,
        "edges": edges,
        "submodels": submodel_nodes,
        "summary": {
            "elements": elements.len(),
            "relations": relations.len(),
            "attachments": attachments.len(),
            "concept_references": concept_refs.len(),
            "resources": resources.len(),
            "submodels": submodel_count
        }
    })
}

fn relation_link(target_id: &str, resource_id: &Option<String>) -> String {
    if let Some(resource_id) = resource_id {
        return resource_id.clone();
    }
    if target_id.contains('#') {
        format!("#/elements/{target_id}")
    } else {
        target_id.to_string()
    }
}

fn build_elements(registry: &GraphRegistry) -> Vec<ProjectStoreElement> {
    registry
        .get_all_elements()
        .into_iter()
        .map(|element| {
            let governance = registry
                .resolve_governance_metadata(element)
                .map(|g| {
                    [
                        ("status".to_string(), governance_value(g.status)),
                        ("priority".to_string(), governance_value(g.priority)),
                        ("risk".to_string(), governance_value(g.risk)),
                        ("owner".to_string(), governance_value(g.owner)),
                    ]
                    .into_iter()
                    .collect()
                })
                .unwrap_or_default();
            ProjectStoreElement {
                id: element.identifier.clone(),
                name: element.name.clone(),
                element_type: element.element_type.as_str().to_string(),
                type_family: element.element_type.main_category().to_string(),
                file_path: element.file_path.clone(),
                line_number: element.line_number,
                source_anchor: element_source_anchor(element),
                content: element.content.clone(),
                metadata: element
                    .metadata
                    .iter()
                    .filter(|(key, _)| !key.starts_with('_'))
                    .map(|(key, value)| (key.clone(), value.clone()))
                    .collect(),
                governance,
            }
        })
        .collect()
}

fn governance_value(entry: crate::element::GovernanceMetadataEntry) -> String {
    let source = match entry.source {
        GovernanceMetadataSource::Explicit => "explicit",
        GovernanceMetadataSource::Inherited => "inherited",
        GovernanceMetadataSource::Default => "default",
    };
    if let Some(source_identifier) = entry.source_identifier {
        format!("{} ({source}, from {source_identifier})", entry.value)
    } else {
        format!("{} ({source})", entry.value)
    }
}

fn build_files_and_folders(
    registry: &GraphRegistry,
    resources: &BTreeMap<String, ProjectStoreResource>,
) -> (Vec<ProjectStoreFile>, Vec<ProjectStoreFolder>) {
    let grouped = registry.group_elements_by_location();
    let mut included_paths: BTreeSet<String> = grouped.keys().cloned().collect();
    let mut resource_ids_by_file: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut resource_text_by_file: BTreeMap<String, String> = BTreeMap::new();
    let mut files = Vec::new();
    let mut folder_children: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for resource in resources.values() {
        let Some(file_path) = resource.file_path.as_deref() else {
            continue;
        };
        if !is_existing_local_project_file_path(file_path) {
            continue;
        }
        included_paths.insert(file_path.to_string());
        resource_ids_by_file
            .entry(file_path.to_string())
            .or_default()
            .push(resource.id.clone());
        if let Some(source_text) = resource.source_text.as_ref() {
            resource_text_by_file
                .entry(file_path.to_string())
                .or_insert_with(|| source_text.clone());
        }
    }

    for ids in resource_ids_by_file.values_mut() {
        ids.sort();
        ids.dedup();
    }

    for path in included_paths {
        let elements = grouped.get(&path).cloned().unwrap_or_default();
        let resource_ids = resource_ids_by_file
            .remove(path.as_str())
            .unwrap_or_default();
        if elements.is_empty() && resource_ids.is_empty() {
            continue;
        }
        let folder = Path::new(&path)
            .parent()
            .map(|p| path_string(p))
            .unwrap_or_default();
        folder_children
            .entry(folder.clone())
            .or_default()
            .insert(path.clone());
        add_folder_ancestors(&folder, &mut folder_children);
        let markdown_content = if elements.is_empty() {
            resource_text_by_file.remove(&path).unwrap_or_default()
        } else {
            registry.generate_file_markdown(&path, &elements, true)
        };
        files.push(ProjectStoreFile {
            display_path: path.clone(),
            markdown_content,
            path: path.clone(),
            parent_folder: folder,
            element_ids: elements
                .iter()
                .map(|element| element.identifier.clone())
                .collect(),
            resource_ids,
        });
    }

    files.sort_by(|a, b| a.path.cmp(&b.path));
    let folders = folder_children
        .into_iter()
        .map(|(path, children)| ProjectStoreFolder {
            parent: if path.is_empty() {
                None
            } else {
                Path::new(&path).parent().map(path_string)
            },
            path,
            children: children.into_iter().collect(),
        })
        .collect();
    (files, folders)
}

fn is_local_project_file_path(path: &str) -> bool {
    let path = Path::new(path);
    !path.is_absolute()
        && !path
            .components()
            .any(|component| matches!(component, Component::ParentDir))
}

fn is_existing_local_project_file_path(path: &str) -> bool {
    if !is_local_project_file_path(path) {
        return false;
    }
    let Ok(git_root) = crate::git_commands::get_git_root_dir() else {
        return false;
    };
    fs::metadata(git_root.join(path))
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

fn add_folder_ancestors(folder: &str, folder_children: &mut BTreeMap<String, BTreeSet<String>>) {
    if folder.is_empty() {
        folder_children.entry(String::new()).or_default();
        return;
    }
    let mut current = Path::new(folder);
    while let Some(parent) = current.parent() {
        let parent_key = path_string(parent);
        let current_key = path_string(current);
        folder_children
            .entry(parent_key.clone())
            .or_default()
            .insert(current_key);
        if parent_key.is_empty() {
            break;
        }
        current = parent;
    }
}

fn build_relations(
    registry: &GraphRegistry,
) -> (
    Vec<ProjectStoreRelation>,
    BTreeMap<String, ProjectStoreResource>,
) {
    #[derive(Debug)]
    struct RelationAccumulator {
        source_id: String,
        target_id: String,
        target_kind: String,
        canonical_relation_type: String,
        source_relation_types: BTreeSet<String>,
        authored: bool,
        generated_opposite: bool,
        resource_id: Option<String>,
    }

    let mut relations: BTreeMap<(String, String, String), RelationAccumulator> = BTreeMap::new();
    let mut resources = BTreeMap::new();
    for element in registry.get_all_elements() {
        for relation in &element.relations {
            let (target_id, target_kind, resource_id) = relation_target_store_id(
                &relation.target.link,
                registry,
                &mut resources,
                &element.identifier,
                relation.relation_type.name,
            );
            let (source_id, target_id, canonical_relation_type) = canonical_relation_edge(
                &element.identifier,
                &target_id,
                relation.relation_type.name,
            );
            let key = (
                source_id.clone(),
                target_id.clone(),
                canonical_relation_type.clone(),
            );
            let entry = relations.entry(key).or_insert_with(|| RelationAccumulator {
                source_id,
                target_id,
                target_kind,
                canonical_relation_type,
                source_relation_types: BTreeSet::new(),
                authored: false,
                generated_opposite: false,
                resource_id: resource_id.clone(),
            });
            entry
                .source_relation_types
                .insert(relation.relation_type.name.to_string());
            entry.authored |= relation.user_created;
            entry.generated_opposite |= !relation.user_created;
            if entry.resource_id.is_none() {
                entry.resource_id = resource_id;
            }
        }
    }
    let relations = relations
        .into_values()
        .map(|relation| ProjectStoreRelation {
            id: format!(
                "relation:{}:{}:{}",
                relation.source_id, relation.canonical_relation_type, relation.target_id
            ),
            source_id: relation.source_id,
            target_id: relation.target_id,
            target_kind: relation.target_kind,
            relation_type: relation.canonical_relation_type.clone(),
            canonical_relation_type: relation.canonical_relation_type,
            source_relation_types: relation.source_relation_types.into_iter().collect(),
            authored: relation.authored,
            generated_opposite: relation.generated_opposite,
            resource_id: relation.resource_id,
        })
        .collect();
    (relations, resources)
}

fn build_attachments(
    registry: &GraphRegistry,
    resources: &mut BTreeMap<String, ProjectStoreResource>,
) -> Vec<ProjectStoreAttachment> {
    let mut attachments = Vec::new();
    for element in registry.get_all_elements() {
        for attachment in &element.attachments {
            let (target, target_kind, resource_id) = match &attachment.target {
                AttachmentTarget::ElementIdentifier(id) => {
                    (id.clone(), "element".to_string(), None)
                }
                AttachmentTarget::FilePath(path) => {
                    let target = path.to_string_lossy().to_string();
                    let resource_id = ensure_resource(
                        resources,
                        "attachment-file",
                        &target,
                        &element.identifier,
                        "attaches",
                    );
                    (target, "resource".to_string(), Some(resource_id))
                }
            };
            attachments.push(ProjectStoreAttachment {
                id: format!("attachment:{}:{}", element.identifier, target),
                source_id: element.identifier.clone(),
                target,
                target_kind,
                resource_id,
                content_hash: attachment.content_hash.clone(),
            });
        }
    }
    attachments.sort_by(|a, b| a.id.cmp(&b.id));
    attachments
}

fn build_concept_refs(
    registry: &GraphRegistry,
    resources: &mut BTreeMap<String, ProjectStoreResource>,
) -> Vec<ProjectStoreConceptReference> {
    let mut refs = Vec::new();
    for element in registry.get_all_elements() {
        for reference in &element.concept_references {
            ensure_resource(
                resources,
                "concept-reference",
                &reference.iri,
                &element.identifier,
                "conceptRef",
            );
            refs.push(ProjectStoreConceptReference {
                id: format!("concept-ref:{}:{}", element.identifier, reference.iri),
                source_id: element.identifier.clone(),
                label: reference.label.clone(),
                iri: reference.iri.clone(),
                line_number: reference.line_number,
            });
        }
    }
    refs.sort_by(|a, b| a.id.cmp(&b.id));
    refs
}

fn build_search_documents(
    elements: &[ProjectStoreElement],
    files: &[ProjectStoreFile],
    resources: &BTreeMap<String, ProjectStoreResource>,
) -> Vec<ProjectStoreSearchDocument> {
    let mut docs = Vec::new();
    for element in elements {
        docs.push(ProjectStoreSearchDocument {
            id: element.id.clone(),
            kind: "element".to_string(),
            title: element.name.clone(),
            route: format!("#/elements/{}", element.id),
            text: format!(
                "{} {} {}",
                element.name, element.element_type, element.content
            ),
        });
    }
    for file in files {
        if file.element_ids.is_empty() {
            continue;
        }
        docs.push(ProjectStoreSearchDocument {
            id: file.path.clone(),
            kind: "file".to_string(),
            title: file.display_path.clone(),
            route: format!("#/content/{}", file.path),
            text: file.display_path.clone(),
        });
    }
    for resource in resources.values() {
        if resource.file_path.is_some()
            && !resource
                .file_path
                .as_deref()
                .map(is_existing_local_project_file_path)
                .unwrap_or(false)
        {
            continue;
        }
        docs.push(ProjectStoreSearchDocument {
            id: resource.id.clone(),
            kind: "resource".to_string(),
            title: resource.display.clone(),
            route: format!("#/resources/{}", resource.id),
            text: format!("{} {}", resource.target, resource.relation_types.join(" ")),
        });
    }
    docs.sort_by(|a, b| a.id.cmp(&b.id));
    docs
}

fn relation_target_store_id(
    link: &LinkType,
    registry: &GraphRegistry,
    resources: &mut BTreeMap<String, ProjectStoreResource>,
    source_id: &str,
    relation_type: &str,
) -> (String, String, Option<String>) {
    match link {
        LinkType::Identifier(id) => {
            if registry.nodes.contains_key(id) {
                (id.clone(), "element".to_string(), None)
            } else {
                let resource_id =
                    ensure_resource(resources, "modeled-target", id, source_id, relation_type);
                (id.clone(), "resource".to_string(), Some(resource_id))
            }
        }
        LinkType::InternalPath(path) => {
            let target = path.to_string_lossy().to_string();
            let kind = if relation_type == "satisfiedBy" || relation_type == "satisfy" {
                "evidence-file"
            } else {
                "local-file"
            };
            let resource_id = ensure_resource(resources, kind, &target, source_id, relation_type);
            (target, "resource".to_string(), Some(resource_id))
        }
        LinkType::ExternalUrl(url) => {
            let resource_id =
                ensure_resource(resources, "external-url", url, source_id, relation_type);
            (url.clone(), "resource".to_string(), Some(resource_id))
        }
    }
}

fn ensure_resource(
    resources: &mut BTreeMap<String, ProjectStoreResource>,
    kind: &str,
    target: &str,
    source_id: &str,
    relation_type: &str,
) -> String {
    let id = format!("resource:{}", target);
    let entry = resources
        .entry(id.clone())
        .or_insert_with(|| ProjectStoreResource {
            id: id.clone(),
            kind: kind.to_string(),
            target: target.to_string(),
            display: short_label(target),
            file_path: if kind == "local-file"
                || kind == "attachment-file"
                || kind == "evidence-file"
            {
                Some(target.to_string())
            } else {
                None
            },
            source_text: None,
            external_url: if kind == "external-url" {
                Some(target.to_string())
            } else {
                None
            },
            referring_element_ids: Vec::new(),
            relation_types: Vec::new(),
        });
    push_unique(&mut entry.referring_element_ids, source_id.to_string());
    push_unique(&mut entry.relation_types, relation_type.to_string());
    id
}

fn enrich_resource_sources(resources: &mut BTreeMap<String, ProjectStoreResource>) {
    let Ok(git_root) = crate::git_commands::get_git_root_dir() else {
        return;
    };
    for resource in resources.values_mut() {
        let Some(file_path) = resource.file_path.as_deref() else {
            continue;
        };
        let source_path = Path::new(file_path);
        if source_path.is_absolute() || file_path.contains("..") {
            continue;
        }
        let absolute_path = git_root.join(source_path);
        let Ok(metadata) = fs::metadata(&absolute_path) else {
            continue;
        };
        if !metadata.is_file() || metadata.len() > 512 * 1024 {
            continue;
        }
        if let Ok(source_text) = fs::read_to_string(&absolute_path) {
            resource.source_text = Some(source_text);
        }
    }
}

fn canonical_relation_edge(
    source_id: &str,
    target_id: &str,
    name: &str,
) -> (String, String, String) {
    if relation::DIAGRAM_RELATIONS.contains(&name) {
        return (
            source_id.to_string(),
            target_id.to_string(),
            name.to_string(),
        );
    }
    let canonical = relation::RELATION_TYPES
        .get(name)
        .and_then(|info| info.opposite)
        .filter(|opposite| relation::DIAGRAM_RELATIONS.contains(opposite))
        .unwrap_or(name);
    if canonical == name {
        (
            source_id.to_string(),
            target_id.to_string(),
            name.to_string(),
        )
    } else {
        (
            target_id.to_string(),
            source_id.to_string(),
            canonical.to_string(),
        )
    }
}

fn default_routes() -> ProjectStoreRoutes {
    ProjectStoreRoutes {
        canonical: vec![
            route("model", "#/model", "Model"),
            route("traces", "#/traces", "Traces"),
            route("ontologies", "#/ontologies", "Ontologies"),
            route("coverage", "#/coverage", "Coverage"),
            route("resources", "#/resources", "Resources"),
            route("files", "#/files/<path>", "Files"),
            route("elements", "#/elements/<identifier>", "Elements"),
            route("search", "#/search", "Search"),
        ],
    }
}

fn route(id: &str, pattern: &str, title: &str) -> ProjectStoreRoute {
    ProjectStoreRoute {
        id: id.to_string(),
        pattern: pattern.to_string(),
        title: title.to_string(),
    }
}

fn element_source_anchor(element: &Element) -> String {
    format!("#/content/{}#{}", element.file_path, element.id)
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().trim_matches('/').to_string()
}

fn short_label(value: &str) -> String {
    Path::new(value)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(value)
        .to_string()
}

fn push_unique(values: &mut Vec<String>, value: String) {
    if !values.contains(&value) {
        values.push(value);
        values.sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::{Element, ElementType, RequirementType, VerificationType};
    use crate::relation::Relation;

    #[test]
    fn explorer_files_include_only_model_files_and_registry_linked_resources() {
        let mut registry = GraphRegistry::new();
        registry.register_page(
            "README.md".to_string(),
            "# README\n\nThis page is not part of the registry graph.\n".to_string(),
        );
        registry.register_page(
            "notes/Plan.md".to_string(),
            "# Plan\n\nThis page is not part of the registry graph.\n".to_string(),
        );

        let mut requirement = Element::new(
            "API Requirement",
            "requirements/API.md#api-requirement",
            "requirements/API.md",
            1,
            Some(ElementType::Requirement(RequirementType::System)),
        );
        requirement.add_content("The API shall expose a stable interface.");
        requirement.freeze_content();
        requirement.add_relation(
            Relation::new(
                "satisfiedBy",
                "core/src/html/store.rs".to_string(),
                "core/src/html/store.rs",
                None,
            )
            .expect("relation should be valid"),
        );
        requirement.add_relation(
            Relation::new(
                "trace",
                "src/generated_placeholder.rs".to_string(),
                "src/generated_placeholder.rs",
                None,
            )
            .expect("relation should be valid"),
        );
        registry
            .register_element(requirement, "requirements/API.md")
            .expect("element should register");

        let elements = build_elements(&registry);
        let (_relations, mut resources) = build_relations(&registry);
        enrich_resource_sources(&mut resources);
        let (files, _folders) = build_files_and_folders(&registry, &resources);
        let search = build_search_documents(&elements, &files, &resources);

        let file_paths = files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<BTreeSet<_>>();
        assert!(file_paths.contains("requirements/API.md"));
        assert!(file_paths.contains("core/src/html/store.rs"));
        assert!(!file_paths.contains("src/generated_placeholder.rs"));
        assert!(!file_paths.contains("README.md"));
        assert!(!file_paths.contains("notes/Plan.md"));

        let model_file = files
            .iter()
            .find(|file| file.path == "requirements/API.md")
            .expect("model file should be present because it owns modeled elements");
        assert_eq!(
            model_file.element_ids,
            vec!["requirements/API.md#api-requirement".to_string()]
        );
        let resource_file = files
            .iter()
            .find(|file| file.path == "core/src/html/store.rs")
            .expect("existing relation-backed resource file should be present in the tree");
        assert!(resource_file.element_ids.is_empty());
        assert_eq!(resource_file.parent_folder, "core/src/html");
        assert_eq!(
            resource_file.resource_ids,
            vec!["resource:core/src/html/store.rs".to_string()]
        );
        assert!(resource_file
            .markdown_content
            .contains("build_project_store"));

        let search_ids = search
            .iter()
            .map(|doc| doc.id.as_str())
            .collect::<BTreeSet<_>>();
        assert!(search_ids.contains("requirements/API.md#api-requirement"));
        assert!(search_ids.contains("requirements/API.md"));
        assert!(search_ids.contains("resource:core/src/html/store.rs"));
        assert!(!search_ids.contains("resource:src/generated_placeholder.rs"));
        assert!(!search_ids.contains("core/src/html/store.rs"));
        assert!(!search_ids.contains("src/generated_placeholder.rs"));
        assert!(!search_ids.contains("README.md"));
        assert!(!search_ids.contains("notes/Plan.md"));
    }

    #[test]
    fn knowledge_graph_exports_all_verification_edges() {
        let mut registry = GraphRegistry::new();

        let first_requirement = Element::new(
            "First Requirement",
            "requirements/Checks.md#first-requirement",
            "requirements/Checks.md",
            1,
            Some(ElementType::Requirement(RequirementType::System)),
        );
        registry
            .register_element(first_requirement, "requirements/Checks.md")
            .expect("first requirement should register");

        let second_requirement = Element::new(
            "Second Requirement",
            "requirements/Checks.md#second-requirement",
            "requirements/Checks.md",
            8,
            Some(ElementType::Requirement(RequirementType::System)),
        );
        registry
            .register_element(second_requirement, "requirements/Checks.md")
            .expect("second requirement should register");

        let mut verification = Element::new(
            "Combined Verification",
            "requirements/Verifications.md#combined-verification",
            "requirements/Verifications.md",
            3,
            Some(ElementType::Verification(VerificationType::Test)),
        );
        verification.add_relation(
            Relation::new(
                "satisfiedBy",
                "tests/combined/test.sh".to_string(),
                "tests/combined/test.sh",
                None,
            )
            .expect("satisfiedBy relation should be valid"),
        );
        verification.add_relation(
            Relation::new(
                "verify",
                "requirements/Checks.md#first-requirement".to_string(),
                "requirements/Checks.md#first-requirement",
                None,
            )
            .expect("first verify relation should be valid"),
        );
        verification.add_relation(
            Relation::new(
                "verify",
                "requirements/Checks.md#second-requirement".to_string(),
                "requirements/Checks.md#second-requirement",
                None,
            )
            .expect("second verify relation should be valid"),
        );
        registry
            .register_element(verification, "requirements/Verifications.md")
            .expect("verification should register");

        let elements = build_elements(&registry);
        let (relations, mut resources) = build_relations(&registry);
        let attachments = build_attachments(&registry, &mut resources);
        let concept_refs = build_concept_refs(&registry, &mut resources);
        let graph = build_knowledge_graph_projection(
            &elements,
            &relations,
            &attachments,
            &concept_refs,
            &resources,
            &serde_json::json!({"submodels":[]}),
        );
        let edges = graph
            .get("edges")
            .and_then(Value::as_array)
            .expect("graph edges should be exported");

        let edge_tuples = edges
            .iter()
            .map(|edge| {
                (
                    edge.get("source")
                        .and_then(Value::as_str)
                        .unwrap_or_default(),
                    edge.get("target")
                        .and_then(Value::as_str)
                        .unwrap_or_default(),
                    edge.get("label")
                        .and_then(Value::as_str)
                        .unwrap_or_default(),
                )
            })
            .collect::<BTreeSet<_>>();

        assert!(edge_tuples.contains(&(
            "requirements/Checks.md#first-requirement",
            "requirements/Verifications.md#combined-verification",
            "verifiedBy",
        )));
        assert!(edge_tuples.contains(&(
            "requirements/Checks.md#second-requirement",
            "requirements/Verifications.md#combined-verification",
            "verifiedBy",
        )));
        assert!(edge_tuples.contains(&(
            "requirements/Verifications.md#combined-verification",
            "resource:tests/combined/test.sh",
            "satisfiedBy",
        )));
    }
}
