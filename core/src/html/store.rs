use crate::element::{AttachmentTarget, Element, GovernanceMetadataSource};
use crate::graph_registry::GraphRegistry;
use crate::relation::{self, LinkType};
use crate::semantic_contract::SemanticIndex;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

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
    pub html_path: String,
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
    let (files, folders) = build_files_and_folders(registry);
    let (relations, mut resources) = build_relations(registry);
    let attachments = build_attachments(registry, &mut resources);
    let concept_refs = build_concept_refs(registry, &mut resources);
    let search = build_search_documents(&elements, &files, &resources);
    let submodels = crate::report_submodels::generate_submodels_report(registry, None)
        .ok()
        .and_then(|report| serde_json::to_value(report).ok())
        .unwrap_or_else(|| json!({"submodels":[],"cross_submodel_couplings":[],"summary":{}}));
    let knowledge_graph = serde_json::from_str(
        &crate::html::pages::knowledgegraph::project_graph_json(registry),
    )
    .unwrap_or_else(|_| json!({"nodes":[],"edges":[],"submodels":[],"summary":{}}));
    let ontology = json!({
        "summary": semantic_index.summary,
        "blocks": semantic_index.blocks,
        "diagnostics": semantic_index.diagnostics,
        "declarations": semantic_index.ontology_declarations,
        "shape_references": semantic_index.shape_references,
        "projection": semantic_index.ontology_projection,
        "graph_data": crate::html::pages::ontologies::graph_data_json(semantic_index),
        "graph_renderer": crate::html::pages::ontologies::graph_renderer_assets_json(),
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

pub fn project_store_script(
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

    // The exported/served Explorer is the compiled Vite/React/Radix bundle
    // (see core/src/export.rs::write_explorer_index). This function emits only
    // the immutable Project Store seed script the bundle reads at boot; route
    // rendering and view modules live in the React bundle, not here.
    Ok(format!(
        r##"<script id="reqvire-project-store">const reqvireProjectStore = {escaped};</script>"##
    ))
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
) -> (Vec<ProjectStoreFile>, Vec<ProjectStoreFolder>) {
    let grouped = registry.group_elements_by_location();
    let mut all_paths: BTreeSet<String> = registry.pages.keys().cloned().collect();
    all_paths.extend(grouped.keys().cloned());
    let mut files = Vec::new();
    let mut folder_children: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for path in all_paths {
        let elements = grouped.get(&path).cloned().unwrap_or_default();
        let folder = Path::new(&path)
            .parent()
            .map(|p| path_string(p))
            .unwrap_or_default();
        folder_children
            .entry(folder.clone())
            .or_default()
            .insert(path.clone());
        add_folder_ancestors(&folder, &mut folder_children);
        files.push(ProjectStoreFile {
            html_path: markdown_to_html_path(&path),
            display_path: path.clone(),
            path,
            parent_folder: folder,
            element_ids: elements
                .iter()
                .map(|element| element.identifier.clone())
                .collect(),
            resource_ids: Vec::new(),
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
        docs.push(ProjectStoreSearchDocument {
            id: file.path.clone(),
            kind: "file".to_string(),
            title: file.display_path.clone(),
            route: format!("#/files/{}", file.path),
            text: file.display_path.clone(),
        });
    }
    for resource in resources.values() {
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
            route("knowledge-graph", "#/knowledge-graph", "Knowledge Graph"),
            route("kn2", "#/kn2", "KN2"),
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
    format!(
        "{}#{}",
        markdown_to_html_path(&element.file_path),
        element.id
    )
}

fn markdown_to_html_path(path: &str) -> String {
    if let Some(stripped) = path.strip_suffix(".md") {
        format!("{stripped}.html")
    } else {
        path.to_string()
    }
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
