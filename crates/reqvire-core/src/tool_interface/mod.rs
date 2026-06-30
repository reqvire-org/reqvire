use crate::crud;
use crate::diff::render_crud_json;
use crate::error::ReqvireError;
use crate::git_commands;
use crate::report;
use crate::search;
use crate::semantic_contract::{self, SemanticExportFormat, SemanticExportLayer};
use crate::semantic_store;
use crate::{ModelBuildOptions, ModelManager};
use globset::GlobSet;
use o_kernel::rdf::{subject_iri, term_iri};
use o_kernel::vocab;
use o_kernel::vocab::reserved;
use oxigraph::model::{NamedOrBlankNode, Term, Triple};
use oxigraph::sparql::{QueryResults, SparqlEvaluator};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::process::Command;

mod arg_helpers;
use arg_helpers::{
    bool_arg, current_dir_path, current_dir_string, git_state, load_model, load_model_with_options,
    model_fingerprint, parse_json_string, required_string_arg, string_arg, string_array_arg,
    usize_arg,
};

mod definitions;
use definitions::{mutation_tool_names, resource_contents, tool_exists};
pub use definitions::{resource_definitions, tool_definitions, validate_tool_arguments};

mod read_tools;
use read_tools::{
    change_impact_tool, collect_tool, containment_tool, coverage_tool, format_tool, lint_tool,
    model_revision, model_tool, read_element, resources_tool, search_tool, sparql_tool,
    submodels_tool, traces_tool, workspace_status,
};

mod semantic_tools;
use semantic_tools::{
    concept_get_tool, concept_mappings_list_tool, concept_schemes_list_tool, concepts_list_tool,
    concepts_tool, ontologies_tool, semantic_export_tool, semantic_graph_layers,
    semantic_graph_tool, semantic_index_with_external_visibility, semantic_model_tool,
    semantic_prefixes_tool, semantic_vocabulary_tool, shapes_tool,
};

mod mutation_tools;
use mutation_tools::{
    add_element_tool, link_tool, merge_elements_tool, move_asset_tool, move_element_tool,
    move_file_tool, move_folder_tool, relink_tool, remove_asset_tool, remove_element_tool,
    rename_element_tool, unlink_tool,
};

mod dispatch;
use dispatch::dispatch_tool;

pub const MCP_PROTOCOL_VERSION: &str = "2025-11-25";
pub const TOOL_CONTRACT_VERSION: &str = "2";

pub struct ReqvireToolRegistry<'a> {
    enable_mutations: bool,
    with_size_estimates: bool,
    excluded_filename_patterns: &'a GlobSet,
}

impl<'a> ReqvireToolRegistry<'a> {
    pub fn new(enable_mutations: bool, excluded_filename_patterns: &'a GlobSet) -> Self {
        Self::new_with_options(enable_mutations, false, excluded_filename_patterns)
    }

    pub fn new_with_options(
        enable_mutations: bool,
        with_size_estimates: bool,
        excluded_filename_patterns: &'a GlobSet,
    ) -> Self {
        Self {
            enable_mutations,
            with_size_estimates,
            excluded_filename_patterns,
        }
    }

    pub fn mutation_tools_enabled(&self) -> bool {
        self.enable_mutations
    }

    pub fn tool_definitions(&self) -> Vec<Value> {
        tool_definitions(self.enable_mutations)
    }

    pub fn resource_definitions(&self) -> Vec<Value> {
        resource_definitions()
    }

    pub fn tool_exists(&self, name: &str) -> bool {
        tool_exists(name, self.enable_mutations)
    }

    pub fn is_mutation_tool(&self, name: &str) -> bool {
        is_mutation_tool_name(name)
    }

    pub fn validate_tool_arguments(
        &self,
        tool_name: &str,
        arguments: &Value,
    ) -> Result<(), String> {
        validate_tool_arguments(tool_name, arguments, self.enable_mutations)
    }

    pub fn call_tool(&self, name: &str, args: &Value) -> Result<Value, ReqvireError> {
        dispatch_tool(
            name,
            args,
            self.enable_mutations,
            self.with_size_estimates,
            self.excluded_filename_patterns,
        )
    }

    pub fn read_resource(&self, uri: &str) -> Result<Value, ReqvireError> {
        let value = match uri {
            "reqvire://workspace/status" => {
                workspace_status(self.excluded_filename_patterns, self.with_size_estimates)
                    .map(|v| resource_contents(uri, v))
            }
            "reqvire://workspace/model-revision" => {
                model_revision(self.excluded_filename_patterns, self.with_size_estimates)
                    .map(|v| resource_contents(uri, v))
            }
            "reqvire://tools/contract" => Ok(resource_contents(
                uri,
                json!({
                    "mcp_protocol_version": MCP_PROTOCOL_VERSION,
                    "tool_contract_version": TOOL_CONTRACT_VERSION,
                    "mutation_tools_enabled": self.enable_mutations,
                    "size_estimates_enabled": self.with_size_estimates,
                    "tools": self.tool_definitions()
                }),
            )),
            _ => Err(ReqvireError::ProcessError(format!(
                "Unknown MCP resource URI '{}'",
                uri
            ))),
        }?;

        Ok(value)
    }
}

pub fn validate_startup(excluded_filename_patterns: &GlobSet) -> Result<(), ReqvireError> {
    validate_startup_with_options(excluded_filename_patterns, false)
}

pub fn validate_startup_with_options(
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<(), ReqvireError> {
    load_model_with_options(excluded_filename_patterns, with_size_estimates)
        .map(|_| ())
        .map_err(|err| match err {
            ReqvireError::ValidationError(errors) => {
                let mut messages: Vec<String> = errors.iter().map(ToString::to_string).collect();
                messages.sort();
                ReqvireError::ProcessError(format!(
                    "MCP startup validation failed for Reqvire {} using MCP protocol {}:\n{}",
                    env!("CARGO_PKG_VERSION"),
                    MCP_PROTOCOL_VERSION,
                    messages
                        .iter()
                        .enumerate()
                        .map(|(idx, msg)| format!("{}. {}", idx + 1, msg))
                        .collect::<Vec<_>>()
                        .join("\n")
                ))
            }
            ReqvireError::ValidationDiagnostics { related_errors, .. } => {
                let mut messages: Vec<String> =
                    related_errors.iter().map(ToString::to_string).collect();
                messages.sort();
                ReqvireError::ProcessError(format!(
                    "MCP startup validation failed for Reqvire {} using MCP protocol {}:\n{}",
                    env!("CARGO_PKG_VERSION"),
                    MCP_PROTOCOL_VERSION,
                    messages
                        .iter()
                        .enumerate()
                        .map(|(idx, msg)| format!("{}. {}", idx + 1, msg))
                        .collect::<Vec<_>>()
                        .join("\n")
                ))
            }
            other => ReqvireError::ProcessError(format!(
                "MCP startup failed for Reqvire {} using MCP protocol {}: {}",
                env!("CARGO_PKG_VERSION"),
                MCP_PROTOCOL_VERSION,
                other
            )),
        })
}

pub fn is_mutation_tool_name(name: &str) -> bool {
    mutation_tool_names().contains(&name)
}

pub fn request_requires_write_tool(tool_name: &str, arguments: Option<&Value>) -> bool {
    is_mutation_tool_name(tool_name)
        || (tool_name == "reqvire.format"
            && arguments
                .and_then(|args| args.get("fix"))
                .and_then(Value::as_bool)
                .unwrap_or(false))
}

#[cfg(test)]
mod tests {
    use super::*;
    use globset::{Glob, GlobSetBuilder};
    use serde_json::json;

    #[test]
    fn in_process_registry_discovers_protocol_neutral_read_only_tools() {
        let ignored = ignored_patterns();
        let registry = ReqvireToolRegistry::new(false, &ignored);

        let tools = registry.tool_definitions();
        assert!(tools.iter().any(|tool| tool["name"] == "reqvire.search"));
        assert!(!tools
            .iter()
            .any(|tool| tool["name"] == "reqvire.add_element"));

        let contract = registry
            .call_tool("reqvire.tool_contract", &json!({}))
            .expect("tool contract should execute through library registry");
        assert_eq!(contract["mutation_tools_enabled"], false);
        assert!(contract["tools"]
            .as_array()
            .expect("contract tools array")
            .iter()
            .any(|tool| tool["name"] == "reqvire.search"));
    }

    fn ignored_patterns() -> globset::GlobSet {
        let mut builder = GlobSetBuilder::new();
        for pattern in ["output/**", "fixtures/**", "expected/**"] {
            builder.add(Glob::new(pattern).expect("valid ignore glob"));
        }
        builder.build().expect("ignore glob set builds")
    }
}
