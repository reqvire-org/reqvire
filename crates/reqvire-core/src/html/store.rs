use crate::element::{ContractBindingTarget, Element, GovernanceMetadataSource};
use crate::git_commands;
use crate::graph_registry::GraphRegistry;
use crate::ontology_graph::{build_graph_data, OntologyGraphData};
use crate::relation::{self, LinkType};
use crate::semantic_contract::{
    external_materialization_metadata, materialized_external_subjects, SemanticIndex,
};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

const SCHEMA_VERSION: &str = "2026-06-30.project-store.v4";

#[derive(Debug, Clone, Serialize)]
pub struct ExplorerProjectStore {
    pub schema_version: &'static str,
    pub project: ProjectStoreProject,
    pub folders: Vec<ProjectStoreFolder>,
    pub files: Vec<ProjectStoreFile>,
    pub resources: Vec<ProjectStoreResource>,
    pub elements: Vec<ProjectStoreElement>,
    pub relations: Vec<ProjectStoreRelation>,
    pub contract_bindings: Vec<ProjectStoreContractBindingEntry>,
    pub concept_refs: Vec<ProjectStoreConceptReference>,
    pub thesaurus: ProjectStoreThesaurus,
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
    pub workspace_root: String,
    pub eligible_git_worktrees: Vec<ProjectStoreGitWorktree>,
    pub repository: Option<String>,
    pub branch: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectStoreGitWorktree {
    pub root: String,
    pub workspace_relative_root: String,
    pub head: Option<String>,
    pub dirty: bool,
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
pub struct ProjectStoreContractBindingEntry {
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
    pub target_element_id: String,
    pub label: String,
    pub iri: String,
    pub line_number: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectStoreThesaurus {
    pub schemes: Vec<ProjectStoreThesaurusScheme>,
    pub concepts: Vec<ProjectStoreThesaurusConcept>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectStoreThesaurusScheme {
    pub id: String,
    pub element_id: String,
    pub label: String,
    pub definition: String,
    pub source_href: String,
    pub source_label: String,
    pub concept_base: String,
    pub concept_prefix: String,
    pub top_concept_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectStoreThesaurusConcept {
    pub id: String,
    pub element_id: String,
    pub label: String,
    pub scheme_id: String,
    pub scheme_element_id: String,
    pub scheme_label: String,
    pub parent_id: Option<String>,
    pub child_ids: Vec<String>,
    pub definition: String,
    pub alt_labels: Vec<String>,
    pub scope_note: String,
    pub example_values: Vec<String>,
    pub related_ids: Vec<String>,
    pub exact_match_ids: Vec<String>,
    pub close_match_ids: Vec<String>,
    pub used_by: Vec<ProjectStoreThesaurusUsage>,
    pub maps_to: Vec<ProjectStoreThesaurusUsage>,
    pub source_href: String,
    pub source_label: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectStoreThesaurusUsage {
    pub id: String,
    pub label: String,
    #[serde(rename = "type")]
    pub usage_type: String,
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
    pub contract_bindings: usize,
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
    let contract_bindings = build_contract_bindings(registry, &mut resources);
    let concept_refs = build_concept_refs(registry, &mut resources);
    enrich_resource_sources(&mut resources);
    let (files, folders) = build_files_and_folders(registry, &resources);
    let submodels = crate::report::submodels::generate_submodels_report(registry, None)
        .ok()
        .and_then(|report| serde_json::to_value(report).ok())
        .unwrap_or_else(|| json!({"submodels":[],"cross_submodel_couplings":[],"summary":{}}));
    let knowledge_graph = build_knowledge_graph_projection(
        &elements,
        &relations,
        &contract_bindings,
        &concept_refs,
        &resources,
        &submodels,
    );
    let (visible_semantic_index, external_metadata) =
        explorer_visible_semantic_index(semantic_index);
    let ontology_graph_data = build_graph_data(&visible_semantic_index);
    let thesaurus = build_thesaurus_projection(registry, &concept_refs, &ontology_graph_data);
    let search = build_search_documents(&elements, &files, &resources, &ontology_graph_data);
    let ontology = json!({
        "summary": semantic_index.summary,
        "blocks": visible_semantic_index.blocks,
        "diagnostics": semantic_index.diagnostics,
        "declarations": visible_semantic_index.ontology_declarations,
        "shape_references": semantic_index.shape_references,
        "projection": semantic_index.ontology_projection,
        "graph_data": ontology_graph_data,
        "external_sources": semantic_index.external_sources,
        "external_materialization": external_metadata["external_materialization"].clone(),
        "external_counts": external_metadata["external_counts"].clone(),
        "ttl_href": "ontologies.ttl"
    });
    let trace_report =
        crate::verification_trace::VerificationTraceGenerator::new(registry).generate();
    let traces = serde_json::to_value(trace_report).unwrap_or_else(|_| json!({"files":{}}));
    let coverage =
        serde_json::to_value(crate::report::coverage::generate_coverage_report(registry))
            .unwrap_or_else(|_| json!({}));
    let summaries = ProjectStoreSummaries {
        elements: elements.len(),
        files: files.len(),
        folders: folders.len(),
        resources: resources.len(),
        relations: relations.len(),
        contract_bindings: contract_bindings.len(),
        concept_refs: concept_refs.len(),
        ontology_blocks: semantic_index.summary.ontology_blocks,
        shape_blocks: semantic_index.summary.shape_blocks,
    };

    let project = build_project_metadata();

    ExplorerProjectStore {
        schema_version: SCHEMA_VERSION,
        project,
        folders,
        files,
        resources: resources.into_values().collect(),
        elements,
        relations,
        contract_bindings,
        concept_refs,
        thesaurus,
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

fn build_project_metadata() -> ProjectStoreProject {
    let workspace_root = crate::workspace::workspace_root()
        .map(|root| root.to_string_lossy().to_string())
        .unwrap_or_else(|_| ".".to_string());
    let eligible_git_worktrees = project_store_git_worktrees();
    let repository = resolve_repository_name();
    let branch = git_commands::get_branch_name().ok();
    let name = repository
        .clone()
        .unwrap_or_else(|| "Reqvire project".to_string());
    let root_label = match (&repository, &branch) {
        (Some(repo), Some(branch)) => format!("{repo} @ {branch}"),
        (Some(repo), None) => repo.clone(),
        (None, Some(branch)) => format!("Reqvire project @ {branch}"),
        (None, None) => "Reqvire root".to_string(),
    };

    ProjectStoreProject {
        name,
        root_label,
        workspace_root,
        eligible_git_worktrees,
        repository,
        branch,
    }
}

fn project_store_git_worktrees() -> Vec<ProjectStoreGitWorktree> {
    let Ok(scope) = crate::workspace::WorkspaceScope::discover() else {
        return Vec::new();
    };

    scope
        .git_worktrees
        .iter()
        .map(|root| {
            let workspace_relative_root = root
                .strip_prefix(&scope.root)
                .ok()
                .map(|path| {
                    if path.as_os_str().is_empty() {
                        ".".to_string()
                    } else {
                        path.to_string_lossy().to_string()
                    }
                })
                .unwrap_or_else(|| root.to_string_lossy().to_string());
            let head = git_output_in_dir(root, ["rev-parse", "HEAD"]);
            let status = git_output_in_dir(root, ["status", "--porcelain"]);

            ProjectStoreGitWorktree {
                root: root.to_string_lossy().to_string(),
                workspace_relative_root,
                head,
                dirty: status
                    .as_ref()
                    .is_some_and(|value| !value.trim().is_empty()),
            }
        })
        .collect()
}

fn git_output_in_dir<const N: usize>(dir: &Path, args: [&str; N]) -> Option<String> {
    let output = std::process::Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn resolve_repository_name() -> Option<String> {
    git_commands::get_repository_base_url()
        .ok()
        .and_then(|url| repository_name_from_url(&url))
        .or_else(|| {
            git_commands::get_git_root_dir().ok().and_then(|root| {
                root.file_name()
                    .map(|name| name.to_string_lossy().into_owned())
            })
        })
}

fn repository_name_from_url(url: &str) -> Option<String> {
    url.trim_end_matches('/')
        .rsplit('/')
        .next()
        .map(|name| name.trim_end_matches(".git").to_string())
        .filter(|name| !name.is_empty())
}

fn explorer_visible_semantic_index(source: &SemanticIndex) -> (SemanticIndex, Value) {
    let mut visible = source.clone();
    let used_subset_block = visible.used_external_subset_block().ok().flatten();
    let used_terms = used_subset_block
        .as_ref()
        .map(materialized_external_subjects)
        .unwrap_or_default();

    visible.external_blocks = used_subset_block.into_iter().collect();
    visible.ontology_declarations.retain(|_iri, declarations| {
        declarations.retain_mut(|declaration| {
            if !declaration.external {
                return true;
            }
            let materialized = used_terms.contains(&declaration.iri);
            declaration.materialized_in_used_subset = materialized;
            materialized
        });
        !declarations.is_empty()
    });

    let external_metadata = explorer_external_materialization_metadata(source, &visible);
    (visible, external_metadata)
}

fn explorer_external_materialization_metadata(
    source: &SemanticIndex,
    visible: &SemanticIndex,
) -> Value {
    external_materialization_metadata(source, visible, !source.external_sources.is_empty())
}

pub fn project_store_javascript(
    store: &ExplorerProjectStore,
) -> Result<String, crate::error::ReqvireError> {
    let json = serde_json::to_string_pretty(store)?;
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
    contract_bindings: &[ProjectStoreContractBindingEntry],
    concept_refs: &[ProjectStoreConceptReference],
    resources: &BTreeMap<String, ProjectStoreResource>,
    submodels: &Value,
) -> Value {
    let element_ids: BTreeSet<&str> = elements.iter().map(|element| element.id.as_str()).collect();
    let mut nodes = Vec::new();

    for element in elements {
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
            "description": element.content.lines().find(|line| !line.trim().is_empty()).unwrap_or("")
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
            "description": resource.target
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

    let bound_context_edges = contract_bindings.iter().filter_map(|contract_bindings| {
        let target = contract_bindings
            .resource_id
            .as_ref()
            .filter(|resource_id| resources.contains_key(resource_id.as_str()))
            .cloned()
            .unwrap_or_else(|| contract_bindings.target.clone());
        if !element_ids.contains(contract_bindings.source_id.as_str())
            || (!element_ids.contains(target.as_str()) && !resources.contains_key(&target))
        {
            return None;
        }
        Some(json!({
            "source": contract_bindings.source_id,
            "target": target,
            "label": "binds contract",
            "kind": "contract_bindings",
            "authored": true
        }))
    });

    let concept_edges = concept_refs.iter().filter_map(|concept_ref| {
        if !element_ids.contains(concept_ref.source_id.as_str())
            || !element_ids.contains(concept_ref.target_element_id.as_str())
        {
            return None;
        }
        Some(json!({
            "source": concept_ref.source_id,
            "target": concept_ref.target_element_id,
            "label": "conceptRef",
            "kind": "concept-reference",
            "authored": true
        }))
    });
    let edges = relation_edges
        .chain(bound_context_edges)
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
            "contract_bindings": contract_bindings.len(),
            "concept_references": concept_refs.len(),
            "resources": resources.len(),
            "submodels": submodel_count
        }
    })
}

fn build_thesaurus_projection(
    registry: &GraphRegistry,
    concept_refs: &[ProjectStoreConceptReference],
    ontology_graph_data: &OntologyGraphData,
) -> ProjectStoreThesaurus {
    let all_elements = registry.get_all_elements();
    let concept_iri_by_element_id: BTreeMap<String, String> = all_elements
        .iter()
        .filter_map(|element| {
            element
                .concept
                .as_ref()
                .map(|concept| (element.identifier.clone(), concept.iri.clone()))
        })
        .collect();
    let scheme_element_by_iri: BTreeMap<String, &Element> = all_elements
        .iter()
        .filter_map(|element| {
            element
                .concept_scheme
                .as_ref()
                .map(|scheme| (scheme.iri.clone(), *element))
        })
        .collect();
    let graph_node_by_id: BTreeMap<&str, &crate::ontology_graph::OntologyGraphNode> =
        ontology_graph_data
            .nodes
            .iter()
            .map(|node| (node.id.as_str(), node))
            .collect();
    let mut maps_to_by_concept: BTreeMap<String, Vec<ProjectStoreThesaurusUsage>> = BTreeMap::new();

    for edge in &ontology_graph_data.edges {
        if edge.label != "mapsToConcept" && edge.label != "mappedFrom" {
            continue;
        }
        if concept_iri_by_element_id
            .values()
            .any(|iri| iri == &edge.target)
        {
            if let Some(source) = graph_node_by_id.get(edge.source.as_str()) {
                maps_to_by_concept
                    .entry(edge.target.clone())
                    .or_default()
                    .push(ProjectStoreThesaurusUsage {
                        id: source.id.clone(),
                        label: source.label.clone(),
                        usage_type: source.semantic_type.clone(),
                    });
            }
        } else if concept_iri_by_element_id
            .values()
            .any(|iri| iri == &edge.source)
        {
            if let Some(target) = graph_node_by_id.get(edge.target.as_str()) {
                maps_to_by_concept
                    .entry(edge.source.clone())
                    .or_default()
                    .push(ProjectStoreThesaurusUsage {
                        id: target.id.clone(),
                        label: target.label.clone(),
                        usage_type: target.semantic_type.clone(),
                    });
            }
        }
    }

    let element_by_id: BTreeMap<&str, &Element> = all_elements
        .iter()
        .map(|element| (element.identifier.as_str(), *element))
        .collect();
    let mut used_by_by_concept: BTreeMap<String, Vec<ProjectStoreThesaurusUsage>> = BTreeMap::new();
    for concept_ref in concept_refs {
        if let Some(source) = element_by_id.get(concept_ref.source_id.as_str()) {
            used_by_by_concept
                .entry(concept_ref.iri.clone())
                .or_default()
                .push(ProjectStoreThesaurusUsage {
                    id: source.identifier.clone(),
                    label: source.name.clone(),
                    usage_type: source.element_type.as_str().to_string(),
                });
        }
    }

    let normalized_concept_relations =
        normalized_thesaurus_relations(&all_elements, &concept_iri_by_element_id);

    let schemes = all_elements
        .iter()
        .filter_map(|element| {
            let scheme = element.concept_scheme.as_ref()?;
            Some(ProjectStoreThesaurusScheme {
                id: scheme.iri.clone(),
                element_id: element.identifier.clone(),
                label: scheme.pref_label.clone(),
                definition: scheme.definition.clone().unwrap_or_default(),
                source_href: element_source_anchor(element),
                source_label: source_label(element),
                concept_base: scheme.namespace_base.clone().unwrap_or_default(),
                concept_prefix: scheme.namespace_prefix.clone().unwrap_or_default(),
                top_concept_ids: scheme
                    .top_concepts
                    .iter()
                    .filter_map(|link| concept_link_iri(&link.target, &concept_iri_by_element_id))
                    .collect(),
            })
        })
        .collect();

    let mut concepts: Vec<ProjectStoreThesaurusConcept> = all_elements
        .iter()
        .filter_map(|element| {
            let concept = element.concept.as_ref()?;
            let scheme_id = concept.scheme_iri.clone().unwrap_or_default();
            let scheme_element = scheme_element_by_iri.get(&scheme_id).copied();
            let scheme_payload = scheme_element.and_then(|scheme| scheme.concept_scheme.as_ref());
            Some(ProjectStoreThesaurusConcept {
                id: concept.iri.clone(),
                element_id: element.identifier.clone(),
                label: concept.pref_label.clone(),
                scheme_id: scheme_id.clone(),
                scheme_element_id: scheme_element
                    .map(|scheme| scheme.identifier.clone())
                    .unwrap_or_default(),
                scheme_label: scheme_payload
                    .map(|scheme| scheme.pref_label.clone())
                    .unwrap_or_default(),
                parent_id: normalized_concept_relations
                    .parent_by_concept
                    .get(&concept.iri)
                    .cloned(),
                child_ids: normalized_concept_relations
                    .children_by_concept
                    .get(&concept.iri)
                    .cloned()
                    .unwrap_or_default(),
                definition: concept.definition.clone().unwrap_or_default(),
                alt_labels: concept
                    .labels
                    .iter()
                    .filter(|label| label.kind == "altLabel")
                    .map(|label| label.value.clone())
                    .collect(),
                scope_note: concept.scope_note.clone().unwrap_or_default(),
                example_values: concept
                    .examples
                    .iter()
                    .map(|example| example.value.clone())
                    .collect(),
                related_ids: normalized_concept_relations
                    .related_by_concept
                    .get(&concept.iri)
                    .cloned()
                    .unwrap_or_default(),
                exact_match_ids: normalized_concept_relations
                    .exact_match_by_concept
                    .get(&concept.iri)
                    .cloned()
                    .unwrap_or_default(),
                close_match_ids: normalized_concept_relations
                    .close_match_by_concept
                    .get(&concept.iri)
                    .cloned()
                    .unwrap_or_default(),
                used_by: used_by_by_concept
                    .get(&concept.iri)
                    .cloned()
                    .unwrap_or_default(),
                maps_to: maps_to_by_concept
                    .get(&concept.iri)
                    .cloned()
                    .unwrap_or_default(),
                source_href: element_source_anchor(element),
                source_label: source_label(element),
            })
        })
        .collect();
    concepts.sort_by(|left, right| {
        left.scheme_label
            .cmp(&right.scheme_label)
            .then_with(|| left.label.cmp(&right.label))
    });

    ProjectStoreThesaurus { schemes, concepts }
}

fn concept_link_iri(
    target: &str,
    concept_iri_by_element_id: &BTreeMap<String, String>,
) -> Option<String> {
    concept_iri_by_element_id
        .get(target)
        .cloned()
        .or_else(|| target.starts_with("http").then(|| target.to_string()))
}

#[derive(Debug, Default)]
struct NormalizedThesaurusRelations {
    parent_by_concept: BTreeMap<String, String>,
    children_by_concept: BTreeMap<String, Vec<String>>,
    related_by_concept: BTreeMap<String, Vec<String>>,
    exact_match_by_concept: BTreeMap<String, Vec<String>>,
    close_match_by_concept: BTreeMap<String, Vec<String>>,
}

fn normalized_thesaurus_relations(
    elements: &[&Element],
    concept_iri_by_element_id: &BTreeMap<String, String>,
) -> NormalizedThesaurusRelations {
    let mut relations = NormalizedThesaurusRelations::default();

    for element in elements {
        let element = *element;
        let Some(concept) = element.concept.as_ref() else {
            continue;
        };
        let Some(source_iri) = concept_iri_by_element_id.get(&element.identifier) else {
            continue;
        };

        for link in &concept.broader {
            if let Some(parent_iri) = concept_link_iri(&link.target, concept_iri_by_element_id) {
                relations
                    .parent_by_concept
                    .entry(source_iri.clone())
                    .or_insert_with(|| parent_iri.clone());
                push_unique(
                    relations.children_by_concept.entry(parent_iri).or_default(),
                    source_iri.clone(),
                );
            }
        }

        for link in &concept.narrower {
            if let Some(child_iri) = concept_link_iri(&link.target, concept_iri_by_element_id) {
                relations
                    .parent_by_concept
                    .entry(child_iri.clone())
                    .or_insert_with(|| source_iri.clone());
                push_unique(
                    relations
                        .children_by_concept
                        .entry(source_iri.clone())
                        .or_default(),
                    child_iri,
                );
            }
        }

        for link in &concept.related {
            if let Some(target_iri) = concept_link_iri(&link.target, concept_iri_by_element_id) {
                add_symmetric_thesaurus_relation(
                    &mut relations.related_by_concept,
                    source_iri,
                    &target_iri,
                );
            }
        }

        for link in &concept.exact_match {
            if let Some(target_iri) = concept_link_iri(&link.target, concept_iri_by_element_id) {
                add_symmetric_thesaurus_relation(
                    &mut relations.exact_match_by_concept,
                    source_iri,
                    &target_iri,
                );
            }
        }

        for link in &concept.close_match {
            if let Some(target_iri) = concept_link_iri(&link.target, concept_iri_by_element_id) {
                add_symmetric_thesaurus_relation(
                    &mut relations.close_match_by_concept,
                    source_iri,
                    &target_iri,
                );
            }
        }
    }

    sort_thesaurus_relation_map(&mut relations.children_by_concept);
    sort_thesaurus_relation_map(&mut relations.related_by_concept);
    sort_thesaurus_relation_map(&mut relations.exact_match_by_concept);
    sort_thesaurus_relation_map(&mut relations.close_match_by_concept);
    relations
}

fn add_symmetric_thesaurus_relation(
    relation_map: &mut BTreeMap<String, Vec<String>>,
    source_iri: &str,
    target_iri: &str,
) {
    push_unique(
        relation_map.entry(source_iri.to_string()).or_default(),
        target_iri.to_string(),
    );
    push_unique(
        relation_map.entry(target_iri.to_string()).or_default(),
        source_iri.to_string(),
    );
}

fn sort_thesaurus_relation_map(relation_map: &mut BTreeMap<String, Vec<String>>) {
    for values in relation_map.values_mut() {
        values.sort();
        values.dedup();
    }
}

fn source_label(element: &Element) -> String {
    format!("{}:{}", element.file_path, element.line_number)
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
            .map(path_string)
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
    let Some(project_root) = project_root_dir() else {
        return false;
    };
    let candidate = project_root.join(path);
    let eligible = crate::workspace::WorkspaceScope::discover_from(project_root.clone())
        .map(|scope| scope.is_eligible_path(&candidate))
        .unwrap_or(false);
    if !eligible {
        return false;
    }
    fs::metadata(candidate)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

fn project_root_dir() -> Option<PathBuf> {
    #[cfg(test)]
    {
        project_root_dir_from_manifest().or_else(|| crate::workspace::workspace_root().ok())
    }
    #[cfg(not(test))]
    {
        crate::workspace::workspace_root()
            .ok()
            .or_else(project_root_dir_from_manifest)
    }
}

fn project_root_dir_from_manifest() -> Option<PathBuf> {
    let manifest_dir = option_env!("CARGO_MANIFEST_DIR").map(PathBuf::from)?;
    let parent = manifest_dir.parent()?;
    if parent.file_name().and_then(|name| name.to_str()) == Some("crates") {
        parent.parent().map(Path::to_path_buf)
    } else {
        Some(parent.to_path_buf())
    }
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

fn build_contract_bindings(
    registry: &GraphRegistry,
    resources: &mut BTreeMap<String, ProjectStoreResource>,
) -> Vec<ProjectStoreContractBindingEntry> {
    let mut entries = Vec::new();
    for element in registry.get_all_elements() {
        for entry in &element.contract_bindings {
            let (target, target_kind, resource_id) = match &entry.target {
                ContractBindingTarget::ElementIdentifier(id) => {
                    (id.clone(), "element".to_string(), None)
                }
                ContractBindingTarget::FilePath(path) => {
                    let target = path.to_string_lossy().to_string();
                    let resource_id = ensure_resource(
                        resources,
                        "contract_bindings-file",
                        &target,
                        &element.identifier,
                        "binds contract",
                    );
                    (target, "resource".to_string(), Some(resource_id))
                }
            };
            entries.push(ProjectStoreContractBindingEntry {
                id: format!("contract_bindings:{}:{}", element.identifier, target),
                source_id: element.identifier.clone(),
                target,
                target_kind,
                resource_id,
                content_hash: entry.content_hash.clone(),
            });
        }
    }
    entries.sort_by(|a, b| a.id.cmp(&b.id));
    entries
}

fn build_concept_refs(
    registry: &GraphRegistry,
    _resources: &mut BTreeMap<String, ProjectStoreResource>,
) -> Vec<ProjectStoreConceptReference> {
    let mut refs = Vec::new();
    for element in registry.get_all_elements() {
        for reference in &element.concept_references {
            let Ok(target_id) = crate::parser::normalize_concept_reference_target(
                &element.file_path,
                &reference.target,
            ) else {
                continue;
            };
            let Some(target) = registry
                .nodes
                .get(&target_id)
                .map(|node| &node.element)
                .filter(|target| target.element_type.is_concept())
            else {
                continue;
            };
            let Some(iri) = registry.generated_concept_iri_for_element(target) else {
                continue;
            };
            refs.push(ProjectStoreConceptReference {
                id: format!("concept-ref:{}:{}", element.identifier, target_id),
                source_id: element.identifier.clone(),
                target_element_id: target_id,
                label: reference.label.clone(),
                iri,
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
    ontology_graph_data: &OntologyGraphData,
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
    let included_resource_ids: BTreeSet<&str> = files
        .iter()
        .flat_map(|file| file.resource_ids.iter().map(String::as_str))
        .collect();
    for resource in resources.values() {
        if resource.file_path.is_some() && !included_resource_ids.contains(resource.id.as_str()) {
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
    for node in &ontology_graph_data.nodes {
        let source_text = node
            .sources
            .iter()
            .map(|source| {
                format!(
                    "{} {} {} {}",
                    source.source_name, source.file_path, source.kind, source.source
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        let type_text = node
            .rdf_types
            .iter()
            .chain(node.type_evidence.iter().map(|evidence| &evidence.label))
            .cloned()
            .collect::<Vec<_>>()
            .join(" ");
        let relation_text = node
            .domain
            .iter()
            .chain(node.range.iter())
            .map(|term| format!("{} {} {}", term.label, term.iri, term.kind))
            .collect::<Vec<_>>()
            .join(" ");
        let text = [
            node.id.as_str(),
            node.full_uri.as_str(),
            node.semantic_type.as_str(),
            node.layer.as_str(),
            node.source_kind.as_str(),
            node.comment.as_str(),
            type_text.as_str(),
            source_text.as_str(),
            relation_text.as_str(),
        ]
        .join(" ");
        docs.push(ProjectStoreSearchDocument {
            id: node.id.clone(),
            kind: "ontology".to_string(),
            title: if node.label.is_empty() {
                node.id.clone()
            } else {
                node.label.clone()
            },
            route: "#/ontologies".to_string(),
            text,
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
                || kind == "contract_bindings-file"
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
    let Some(project_root) = project_root_dir() else {
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
        let absolute_path = project_root.join(source_path);
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
    relation::canonical_model_traversal_edge(source_id, target_id, name)
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
    use crate::semantic_contract::{
        ExternalOntologySource, ModelContextGraph, OntologyConstruct, OntologyConstructFamily,
        OntologyConstructKind, OntologyProjectionDerivationMode, OntologyProjectionGraph,
        OntologyProjectionProvenance, OntologyProjectionSource, OntologyProjectionTerm,
        OntologyProjectionTermKind, OntologyTermDeclaration, OntologyTermRole, SemanticBlock,
        SemanticBlockKind, SemanticIndexSummary,
    };
    use crate::test_support::parse_test_quads;
    use rustc_hash::FxHashMap;

    const TEST_RDFS_SUBCLASS_OF: &str = o_kernel::vocab::RDFS_SUBCLASS_OF;

    fn external_block(content: &str) -> SemanticBlock {
        SemanticBlock {
            kind: SemanticBlockKind::ExternalOntology,
            source: "test#external".to_string(),
            source_name: "Test external ontology".to_string(),
            file_path: "system-model/Ontologies/Test.md".to_string(),
            line_number: 1,
            language: "turtle".to_string(),
            external_materialization: None,
            content: content.to_string(),
            quads: parse_test_quads(content),
        }
    }

    fn test_projection_source() -> OntologyProjectionSource {
        OntologyProjectionSource {
            source_block: "test#ontology".to_string(),
            source_element_identifier: "system-model/Ontologies/Test.md#test-ontology".to_string(),
            source_name: "Test Ontology".to_string(),
            file_path: "system-model/Ontologies/Test.md".to_string(),
            line_number: 1,
            block_kind: "ontology".to_string(),
        }
    }

    fn iri_term(iri: &str) -> OntologyProjectionTerm {
        OntologyProjectionTerm {
            kind: OntologyProjectionTermKind::Iri,
            value: iri.to_string(),
            label: iri.to_string(),
        }
    }

    fn projection_graph_with_external_object(iri: &str) -> OntologyProjectionGraph {
        let source = test_projection_source();
        OntologyProjectionGraph {
            id: "urn:reqvire:ontology-projection:test".to_string(),
            derivation_mode: OntologyProjectionDerivationMode::DirectAuthored,
            projections: Vec::new(),
            constructs: vec![OntologyConstruct {
                id: "urn:reqvire:ontology-construct:test".to_string(),
                family: OntologyConstructFamily::SubclassMembership,
                kind: OntologyConstructKind::SubclassInclusion,
                subject: iri_term("https://example.test/local#LocalTerm"),
                predicate: Some(iri_term(TEST_RDFS_SUBCLASS_OF)),
                object: Some(iri_term(iri)),
                property: None,
                members: Vec::new(),
                property_characteristic: None,
                restriction_kind: None,
                class_expression_kind: None,
                shape_overlay_kind: None,
                symbol: None,
                provenance: OntologyProjectionProvenance {
                    derivation_mode: OntologyProjectionDerivationMode::DirectAuthored,
                    source,
                    evidence: Vec::new(),
                },
            }],
            symbols: Vec::new(),
        }
    }

    fn external_declaration(iri: &str) -> OntologyTermDeclaration {
        OntologyTermDeclaration {
            iri: iri.to_string(),
            role: OntologyTermRole::Class,
            element_identifier: "system-model/Ontologies/Test.md#test-ontology".to_string(),
            external: true,
            materialized_in_used_subset: false,
        }
    }

    fn test_semantic_index_with_external_subset() -> SemanticIndex {
        let raw_external = r#"
@prefix ext: <https://example.test/external#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

ext:ProjectedTerm a owl:Class ;
  rdfs:label "Projected term" ;
  rdfs:subClassOf ext:SupportClass .

ext:SupportClass a owl:Class ;
  rdfs:label "Support class" .

ext:UnusedTerm a owl:Class ;
  rdfs:label "Unused term" .
"#;

        SemanticIndex {
            blocks: Vec::new(),
            external_blocks: vec![external_block(raw_external)],
            external_sources: vec![ExternalOntologySource {
                owner_identifier: "system-model/Ontologies/Test.md#test-ontology".to_string(),
                owner_name: "Test Ontology".to_string(),
                prefix: "ext".to_string(),
                namespace: "https://example.test/external#".to_string(),
                resource: Some("https://example.test/external".to_string()),
                source: "references/external.ttl".to_string(),
                format: "turtle".to_string(),
                line_number: 1,
                builtin: false,
            }],
            diagnostics: Vec::new(),
            ontology_documents: Vec::new(),
            ontology_declarations: FxHashMap::from_iter([
                (
                    "https://example.test/external#ProjectedTerm".to_string(),
                    vec![external_declaration(
                        "https://example.test/external#ProjectedTerm",
                    )],
                ),
                (
                    "https://example.test/external#UnusedTerm".to_string(),
                    vec![external_declaration(
                        "https://example.test/external#UnusedTerm",
                    )],
                ),
            ]),
            shape_references: Vec::new(),
            ontology_projection: projection_graph_with_external_object(
                "https://example.test/external#ProjectedTerm",
            ),
            model_context: ModelContextGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
            },
            model_context_turtle: r#"
@prefix reqvire: <https://www.reqvire.org/ontology#> .

<urn:reqvire:external-source:test> a reqvire:ExternalOntologySource ;
  reqvire:externalOntologyNamespace "https://example.test/external#" .

<urn:reqvire:concept:test> reqvire:referencesTerm <https://example.test/external#ProjectedTerm> .
"#
            .to_string(),
            summary: SemanticIndexSummary {
                ontology_blocks: 0,
                shape_blocks: 0,
                total_blocks: 0,
                total_quads: 0,
            },
        }
    }

    #[test]
    fn explorer_visible_semantic_index_materializes_used_external_subset_only() {
        let source = test_semantic_index_with_external_subset();
        let (visible, metadata) = explorer_visible_semantic_index(&source);
        let graph_data = build_graph_data(&visible);
        let graph_node_ids = graph_data
            .nodes
            .iter()
            .map(|node| node.id.as_str())
            .collect::<BTreeSet<_>>();

        assert_eq!(visible.external_blocks.len(), 1);
        assert_eq!(
            visible.external_blocks[0].source,
            "reqvire:external-used-subset"
        );
        assert!(graph_node_ids.contains("https://example.test/external#ProjectedTerm"));
        assert!(graph_node_ids.contains("https://example.test/external#SupportClass"));
        assert!(!graph_node_ids.contains("https://example.test/external#UnusedTerm"));
        assert!(visible
            .ontology_declarations
            .contains_key("https://example.test/external#ProjectedTerm"));
        assert!(!visible
            .ontology_declarations
            .contains_key("https://example.test/external#UnusedTerm"));
        assert!(
            visible.ontology_declarations["https://example.test/external#ProjectedTerm"][0]
                .materialized_in_used_subset
        );
        assert_eq!(metadata["external_materialization"], "used_subset");
        assert_eq!(
            metadata["external_counts"]["declared_external_source_count"],
            1
        );
        assert_eq!(metadata["external_counts"]["used_external_source_count"], 1);
        assert_eq!(
            metadata["external_counts"]["visible_external_term_declaration_count"],
            1
        );
        assert_eq!(
            metadata["external_counts"]["visible_external_source_count"],
            1
        );
        let raw_external_triple_count = metadata["external_counts"]["raw_external_triple_count"]
            .as_u64()
            .expect("raw external triple count should be numeric");
        let materialized_external_triple_count = metadata["external_counts"]
            ["materialized_external_triple_count"]
            .as_u64()
            .expect("materialized external triple count should be numeric");
        assert!(raw_external_triple_count > materialized_external_triple_count);
    }

    #[test]
    fn explorer_search_indexes_visible_ontology_subset_only() {
        let source = test_semantic_index_with_external_subset();
        let (visible, _metadata) = explorer_visible_semantic_index(&source);
        let graph_data = build_graph_data(&visible);
        let docs = build_search_documents(&[], &[], &BTreeMap::new(), &graph_data);
        let ontology_text = docs
            .iter()
            .filter(|doc| doc.kind == "ontology")
            .map(|doc| format!("{} {}", doc.title, doc.text))
            .collect::<Vec<_>>()
            .join("\n");

        assert!(ontology_text.contains("ProjectedTerm"));
        assert!(ontology_text.contains("SupportClass"));
        assert!(!ontology_text.contains("UnusedTerm"));
        assert!(docs
            .iter()
            .all(|doc| doc.kind != "ontology" || doc.route == "#/ontologies"));
    }

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
            "system-model/API.md#api-requirement",
            "system-model/API.md",
            1,
            Some(ElementType::Requirement(RequirementType::System)),
        );
        requirement.add_content("The API shall expose a stable interface.");
        requirement.freeze_content();
        requirement.add_relation(
            Relation::new(
                "satisfiedBy",
                "crates/reqvire-core/src/html/store.rs".to_string(),
                "crates/reqvire-core/src/html/store.rs",
                None,
            )
            .expect("relation should be valid"),
        );
        requirement.add_relation(
            Relation::new(
                "derivedFrom",
                "src/generated_placeholder.rs".to_string(),
                "src/generated_placeholder.rs",
                None,
            )
            .expect("relation should be valid"),
        );
        registry
            .register_element(requirement, "system-model/API.md")
            .expect("element should register");

        let elements = build_elements(&registry);
        let (_relations, mut resources) = build_relations(&registry);
        enrich_resource_sources(&mut resources);
        let (files, _folders) = build_files_and_folders(&registry, &resources);
        let search = build_search_documents(
            &elements,
            &files,
            &resources,
            &OntologyGraphData {
                nodes: Vec::new(),
                edges: Vec::new(),
            },
        );

        let file_paths = files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<BTreeSet<_>>();
        assert!(file_paths.contains("system-model/API.md"));
        assert!(file_paths.contains("crates/reqvire-core/src/html/store.rs"));
        assert!(!file_paths.contains("src/generated_placeholder.rs"));
        assert!(!file_paths.contains("README.md"));
        assert!(!file_paths.contains("notes/Plan.md"));

        let model_file = files
            .iter()
            .find(|file| file.path == "system-model/API.md")
            .expect("model file should be present because it owns modeled elements");
        assert_eq!(
            model_file.element_ids,
            vec!["system-model/API.md#api-requirement".to_string()]
        );
        let resource_file = files
            .iter()
            .find(|file| file.path == "crates/reqvire-core/src/html/store.rs")
            .expect("existing relation-backed resource file should be present in the tree");
        assert!(resource_file.element_ids.is_empty());
        assert_eq!(resource_file.parent_folder, "crates/reqvire-core/src/html");
        assert_eq!(
            resource_file.resource_ids,
            vec!["resource:crates/reqvire-core/src/html/store.rs".to_string()]
        );
        assert!(resource_file
            .markdown_content
            .contains("build_project_store"));

        let search_ids = search
            .iter()
            .map(|doc| doc.id.as_str())
            .collect::<BTreeSet<_>>();
        assert!(search_ids.contains("system-model/API.md#api-requirement"));
        assert!(search_ids.contains("system-model/API.md"));
        assert!(search_ids.contains("resource:crates/reqvire-core/src/html/store.rs"));
        assert!(!search_ids.contains("resource:src/generated_placeholder.rs"));
        assert!(!search_ids.contains("crates/reqvire-core/src/html/store.rs"));
        assert!(!search_ids.contains("src/generated_placeholder.rs"));
        assert!(!search_ids.contains("README.md"));
        assert!(!search_ids.contains("notes/Plan.md"));
    }

    #[test]
    fn knowledge_graph_exports_all_verification_edges() {
        let mut registry = GraphRegistry::new();

        let first_requirement = Element::new(
            "First Requirement",
            "system-model/Checks.md#first-requirement",
            "system-model/Checks.md",
            1,
            Some(ElementType::Requirement(RequirementType::System)),
        );
        registry
            .register_element(first_requirement, "system-model/Checks.md")
            .expect("first requirement should register");

        let second_requirement = Element::new(
            "Second Requirement",
            "system-model/Checks.md#second-requirement",
            "system-model/Checks.md",
            8,
            Some(ElementType::Requirement(RequirementType::System)),
        );
        registry
            .register_element(second_requirement, "system-model/Checks.md")
            .expect("second requirement should register");

        let mut verification = Element::new(
            "Combined Verification",
            "system-model/Verifications.md#combined-verification",
            "system-model/Verifications.md",
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
                "system-model/Checks.md#first-requirement".to_string(),
                "system-model/Checks.md#first-requirement",
                None,
            )
            .expect("first verify relation should be valid"),
        );
        verification.add_relation(
            Relation::new(
                "verify",
                "system-model/Checks.md#second-requirement".to_string(),
                "system-model/Checks.md#second-requirement",
                None,
            )
            .expect("second verify relation should be valid"),
        );
        registry
            .register_element(verification, "system-model/Verifications.md")
            .expect("verification should register");

        let elements = build_elements(&registry);
        let (relations, mut resources) = build_relations(&registry);
        let contract_bindings = build_contract_bindings(&registry, &mut resources);
        let concept_refs = build_concept_refs(&registry, &mut resources);
        let graph = build_knowledge_graph_projection(
            &elements,
            &relations,
            &contract_bindings,
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
            "system-model/Checks.md#first-requirement",
            "system-model/Verifications.md#combined-verification",
            "verifiedBy",
        )));
        assert!(edge_tuples.contains(&(
            "system-model/Checks.md#second-requirement",
            "system-model/Verifications.md#combined-verification",
            "verifiedBy",
        )));
        assert!(edge_tuples.contains(&(
            "system-model/Verifications.md#combined-verification",
            "resource:tests/combined/test.sh",
            "satisfiedBy",
        )));
    }

    #[test]
    fn knowledge_graph_exports_concepts_as_ontology_nodes_and_reference_edges() {
        let mut registry = GraphRegistry::new();

        let mut scheme = Element::new(
            "Traceability Concepts",
            "system-model/Checks.md#traceability-concepts",
            "system-model/Checks.md",
            1,
            Some(ElementType::ConceptScheme),
        );
        scheme.metadata.insert(
            "concept_base".to_string(),
            "https://example.test/concepts".to_string(),
        );
        scheme
            .metadata
            .insert("concept_prefix".to_string(), "concept".to_string());
        scheme.freeze_content();
        registry
            .register_element(scheme, "system-model/Checks.md")
            .expect("concept scheme should register");

        let mut concept = Element::new(
            "Traceability",
            "system-model/Checks.md#traceability",
            "system-model/Checks.md",
            8,
            Some(ElementType::Concept),
        );
        concept.add_content(
            r#"
#### Relations
* derivedFrom: [Traceability Concepts](#traceability-concepts)
"#,
        );
        concept.freeze_content();
        concept.add_relation(
            Relation::new(
                "derivedFrom",
                "system-model/Checks.md#traceability-concepts".to_string(),
                "system-model/Checks.md#traceability-concepts",
                None,
            )
            .expect("concept scheme relation should be valid"),
        );
        registry
            .register_element(concept, "system-model/Checks.md")
            .expect("concept should register");

        let mut requirement = Element::new(
            "Traceability Requirement",
            "system-model/Checks.md#traceability-requirement",
            "system-model/Checks.md",
            16,
            Some(ElementType::Requirement(RequirementType::System)),
        );
        requirement.add_content(
            r#"
#### Concept References
* [Traceability](#traceability)
"#,
        );
        requirement.freeze_content();
        registry
            .register_element(requirement, "system-model/Checks.md")
            .expect("requirement should register");

        let elements = build_elements(&registry);
        let (relations, mut resources) = build_relations(&registry);
        let contract_bindings = build_contract_bindings(&registry, &mut resources);
        let concept_refs = build_concept_refs(&registry, &mut resources);
        assert_eq!(concept_refs.len(), 1);
        assert_eq!(
            concept_refs[0].target_element_id,
            "system-model/Checks.md#traceability"
        );
        let graph = build_knowledge_graph_projection(
            &elements,
            &relations,
            &contract_bindings,
            &concept_refs,
            &resources,
            &serde_json::json!({"submodels":[]}),
        );
        let nodes = graph
            .get("nodes")
            .and_then(Value::as_array)
            .expect("graph nodes should be exported");
        let requirement_node = nodes
            .iter()
            .find(|node| {
                node.get("id").and_then(Value::as_str)
                    == Some("system-model/Checks.md#traceability-requirement")
            })
            .expect("requirement should be exported as a graph node");
        assert!(requirement_node.get("metadata").is_none());
        assert!(requirement_node.get("governance").is_none());
        assert!(requirement_node.get("incoming").is_none());
        assert!(requirement_node.get("outgoing").is_none());
        assert!(requirement_node.get("contract_bindings").is_none());
        assert!(requirement_node.get("concept_references").is_none());
        let concept_node = nodes
            .iter()
            .find(|node| {
                node.get("id").and_then(Value::as_str)
                    == Some("system-model/Checks.md#traceability")
            })
            .expect("concept reference target should be exported as its native concept node");
        assert_eq!(
            concept_node.get("type").and_then(Value::as_str),
            Some("concept")
        );
        assert_eq!(
            concept_node.get("node_type").and_then(Value::as_str),
            Some("concept")
        );
        assert_eq!(
            concept_node.get("element_type").and_then(Value::as_str),
            Some("concept")
        );
        assert!(nodes.iter().all(|node| {
            node.get("type").and_then(Value::as_str) != Some("concept-reference")
                && node.get("element_type").and_then(Value::as_str) != Some("concept-reference")
                && !node
                    .get("id")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .starts_with("concept:")
        }));

        let edges = graph
            .get("edges")
            .and_then(Value::as_array)
            .expect("graph edges should be exported");
        assert!(edges.iter().any(|edge| {
            edge.get("source").and_then(Value::as_str)
                == Some("system-model/Checks.md#traceability-requirement")
                && edge.get("target").and_then(Value::as_str)
                    == Some("system-model/Checks.md#traceability")
                && edge.get("kind").and_then(Value::as_str) == Some("concept-reference")
        }));
    }
}
