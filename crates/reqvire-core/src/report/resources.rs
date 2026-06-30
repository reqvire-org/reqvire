// Resources Report Module
// Generates reports showing all files referenced by the model through relations and contract_bindings

use crate::element::ContractBindingTarget;
use crate::graph_registry::GraphRegistry;
use crate::relation::LinkType;
use rustc_hash::FxHashMap;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct ResourcesReport {
    pub relations: Vec<FileReferences>,
    pub contract_bindings: Vec<FileReferences>,
    pub summary: ResourcesSummary,
}

#[derive(Serialize)]
pub struct FileReferences {
    pub file_path: String,
    pub references: Vec<Reference>,
}

#[derive(Serialize)]
pub struct Reference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_type: Option<String>,
    pub element_id: String,
    pub element_name: String,
}

#[derive(Serialize)]
pub struct ResourcesSummary {
    pub total_relation_files: usize,
    pub total_contract_bindings_files: usize,
    pub total_relation_references: usize,
    pub total_contract_bindings_references: usize,
}

impl ResourcesReport {
    pub fn to_json_string(&self) -> String {
        serde_json::to_string_pretty(&self).expect("failed to serialize JSON")
    }
}

/// Generate the resources report from the graph registry
pub fn generate_resources_report(registry: &GraphRegistry) -> ResourcesReport {
    let workspace_scope = crate::workspace::WorkspaceScope::discover().ok();
    // Collect InternalPath relations: file_path -> Vec<(relation_type, element_id, element_name)>
    let mut relation_map: FxHashMap<PathBuf, Vec<(String, String, String)>> = FxHashMap::default();

    // Collect FilePath contract_bindings: file_path -> Vec<(element_id, element_name)>
    let mut contract_bindings_map: FxHashMap<PathBuf, Vec<(String, String)>> = FxHashMap::default();

    // Iterate all elements
    for element in registry.get_all_elements() {
        let element_id = element.identifier.clone();
        let element_name = element.name.clone();

        // Process relations with InternalPath targets
        for relation in &element.relations {
            if let LinkType::InternalPath(path) = &relation.target.link {
                if !is_existing_eligible_file(path, workspace_scope.as_ref()) {
                    continue;
                }
                let rel_type = relation.relation_type.name.to_string();
                relation_map.entry(path.clone()).or_default().push((
                    rel_type,
                    element_id.clone(),
                    element_name.clone(),
                ));
            }
        }

        // Process contract_bindings with FilePath targets
        for contract_bindings in &element.contract_bindings {
            if let ContractBindingTarget::FilePath(path) = &contract_bindings.target {
                if !is_existing_eligible_file(path, workspace_scope.as_ref()) {
                    continue;
                }
                contract_bindings_map
                    .entry(path.clone())
                    .or_default()
                    .push((element_id.clone(), element_name.clone()));
            }
            // Skip ElementIdentifier contract_bindings - they reference model elements, not files
        }
    }

    // Build sorted relations list
    let mut relations: Vec<FileReferences> = relation_map
        .into_iter()
        .map(|(path, mut refs)| {
            // Sort references: by relation_type, then by element_id
            refs.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));

            let references: Vec<Reference> = refs
                .into_iter()
                .map(|(rel_type, elem_id, elem_name)| Reference {
                    relation_type: Some(rel_type),
                    element_id: elem_id,
                    element_name: elem_name,
                })
                .collect();

            FileReferences {
                file_path: path.to_string_lossy().to_string(),
                references,
            }
        })
        .collect();

    // Sort by file path
    relations.sort_by(|a, b| a.file_path.cmp(&b.file_path));

    // Build sorted contract_bindings list
    let mut contract_bindings: Vec<FileReferences> = contract_bindings_map
        .into_iter()
        .map(|(path, mut refs)| {
            // Sort references by element_id
            refs.sort_by(|a, b| a.0.cmp(&b.0));

            let references: Vec<Reference> = refs
                .into_iter()
                .map(|(elem_id, elem_name)| Reference {
                    relation_type: None,
                    element_id: elem_id,
                    element_name: elem_name,
                })
                .collect();

            FileReferences {
                file_path: path.to_string_lossy().to_string(),
                references,
            }
        })
        .collect();

    // Sort by file path
    contract_bindings.sort_by(|a, b| a.file_path.cmp(&b.file_path));

    // Calculate totals
    let total_relation_references: usize = relations.iter().map(|f| f.references.len()).sum();
    let total_contract_bindings_references: usize =
        contract_bindings.iter().map(|f| f.references.len()).sum();

    ResourcesReport {
        summary: ResourcesSummary {
            total_relation_files: relations.len(),
            total_contract_bindings_files: contract_bindings.len(),
            total_relation_references,
            total_contract_bindings_references,
        },
        relations,
        contract_bindings,
    }
}

fn is_existing_eligible_file(
    path: &std::path::Path,
    workspace_scope: Option<&crate::workspace::WorkspaceScope>,
) -> bool {
    let Some(scope) = workspace_scope else {
        return false;
    };
    let absolute_path = scope.root.join(path);
    scope.is_eligible_path(&absolute_path)
        && std::fs::metadata(absolute_path)
            .map(|metadata| metadata.is_file())
            .unwrap_or(false)
}
