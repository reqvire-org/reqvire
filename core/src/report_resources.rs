// Resources Report Module
// Generates reports showing all files referenced by the model through relations and reused_contract_context

use crate::element::{ReusedContractContextTarget, REUSED_CONTRACT_CONTEXT_SECTION};
use crate::graph_registry::GraphRegistry;
use crate::relation::LinkType;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct ResourcesReport {
    pub relations: Vec<FileReferences>,
    pub reused_contract_context: Vec<FileReferences>,
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
    pub total_reused_contract_context_files: usize,
    pub total_relation_references: usize,
    pub total_reused_contract_context_references: usize,
}

/// Helper function to format an identifier as a markdown link
fn format_identifier_link(identifier: &str, name: &str) -> String {
    if let Some(hash_pos) = identifier.rfind('#') {
        let file_part = &identifier[..hash_pos];
        let fragment_part = &identifier[hash_pos..];
        format!("[{}]({}{})", name, file_part, fragment_part)
    } else {
        format!("[{}]({})", name, identifier)
    }
}

impl ResourcesReport {
    pub fn to_json_string(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    pub fn print(&self, json_output: bool) {
        if json_output {
            println!("{}", self.to_json_string());
        } else {
            print!("{}", self.format_text());
        }
    }

    pub fn format_text(&self) -> String {
        let mut output = String::new();

        // Relations Section
        output.push_str("## Relations\n\n");
        output.push_str("Files referenced via relations such as satisfiedBy:\n\n");

        if self.relations.is_empty() {
            output.push_str("*No files referenced via relations.*\n\n");
        } else {
            for file_ref in &self.relations {
                output.push_str(&format!("### {}\n", file_ref.file_path));
                for reference in &file_ref.references {
                    let link =
                        format_identifier_link(&reference.element_id, &reference.element_name);
                    if let Some(rel_type) = &reference.relation_type {
                        output.push_str(&format!("  * {} (via {})\n", link, rel_type));
                    } else {
                        output.push_str(&format!("  * {}\n", link));
                    }
                }
                output.push_str("---\n\n");
            }
        }

        // Reused Contract Context Section
        output.push_str("## ");
        output.push_str(REUSED_CONTRACT_CONTEXT_SECTION);
        output.push_str("\n\n");
        output.push_str("Files referenced via reused_contract_context:\n\n");

        if self.reused_contract_context.is_empty() {
            output.push_str("*No files referenced via reused_contract_context.*\n\n");
        } else {
            for file_ref in &self.reused_contract_context {
                output.push_str(&format!("### {}\n", file_ref.file_path));
                for reference in &file_ref.references {
                    let link =
                        format_identifier_link(&reference.element_id, &reference.element_name);
                    output.push_str(&format!("  * {}\n", link));
                }
                output.push_str("---\n\n");
            }
        }

        // Summary
        output.push_str("## Summary\n\n");
        output.push_str(&format!(
            "- **Relation Files:** {} ({} references)\n",
            self.summary.total_relation_files, self.summary.total_relation_references
        ));
        output.push_str(&format!(
            "- **ReusedContractContextEntry Files:** {} ({} references)\n",
            self.summary.total_reused_contract_context_files,
            self.summary.total_reused_contract_context_references
        ));

        output
    }
}

/// Generate the resources report from the graph registry
pub fn generate_resources_report(registry: &GraphRegistry) -> ResourcesReport {
    // Collect InternalPath relations: file_path -> Vec<(relation_type, element_id, element_name)>
    let mut relation_map: HashMap<PathBuf, Vec<(String, String, String)>> = HashMap::new();

    // Collect FilePath reused_contract_context: file_path -> Vec<(element_id, element_name)>
    let mut reused_context_map: HashMap<PathBuf, Vec<(String, String)>> = HashMap::new();

    // Iterate all elements
    for element in registry.get_all_elements() {
        let element_id = element.identifier.clone();
        let element_name = element.name.clone();

        // Process relations with InternalPath targets
        for relation in &element.relations {
            if let LinkType::InternalPath(path) = &relation.target.link {
                let rel_type = relation.relation_type.name.to_string();
                relation_map.entry(path.clone()).or_default().push((
                    rel_type,
                    element_id.clone(),
                    element_name.clone(),
                ));
            }
        }

        // Process reused_contract_context with FilePath targets
        for reused_contract_context in &element.reused_contract_context {
            if let ReusedContractContextTarget::FilePath(path) = &reused_contract_context.target {
                reused_context_map
                    .entry(path.clone())
                    .or_default()
                    .push((element_id.clone(), element_name.clone()));
            }
            // Skip ElementIdentifier reused_contract_context - they reference model elements, not files
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

    // Build sorted reused_contract_context list
    let mut reused_contract_context: Vec<FileReferences> = reused_context_map
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
    reused_contract_context.sort_by(|a, b| a.file_path.cmp(&b.file_path));

    // Calculate totals
    let total_relation_references: usize = relations.iter().map(|f| f.references.len()).sum();
    let total_reused_contract_context_references: usize = reused_contract_context
        .iter()
        .map(|f| f.references.len())
        .sum();

    ResourcesReport {
        summary: ResourcesSummary {
            total_relation_files: relations.len(),
            total_reused_contract_context_files: reused_contract_context.len(),
            total_relation_references,
            total_reused_contract_context_references,
        },
        relations,
        reused_contract_context,
    }
}
