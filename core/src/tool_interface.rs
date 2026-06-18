use crate::change_impact;
use crate::containment::ContainmentHierarchy;
use crate::crud;
use crate::diff::render_crud_json;
use crate::error::ReqvireError;
use crate::format::{format_files, render_diff_json};
use crate::git_commands;
use crate::lint;
use crate::report_collect;
use crate::report_coverage;
use crate::report_model;
use crate::report_resources;
use crate::report_submodels;
use crate::search;
use crate::semantic_contract::{self, SemanticExportFormat};
use crate::verification_trace;
use crate::{ModelBuildOptions, ModelManager};
use globset::GlobSet;
use oxigraph::model::{NamedOrBlankNode, Term, Triple};
use oxigraph::sparql::{QueryResults, SparqlEvaluator};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::process::Command;

pub const MCP_PROTOCOL_VERSION: &str = "2025-11-25";
pub const TOOL_CONTRACT_VERSION: &str = "1";

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

pub fn tool_definitions(enable_mutations: bool) -> Vec<Value> {
    let mut tools = vec![
        read_tool(
            "reqvire.workspace_status",
            "Report workspace, git, and model status.",
            object_schema(vec![]),
        ),
        read_tool(
            "reqvire.tool_contract",
            "Return the Reqvire tool contract.",
            object_schema(vec![]),
        ),
        read_tool(
            "reqvire.model_revision",
            "Report the current workspace and model revision.",
            object_schema(vec![]),
        ),
        read_tool(
            "reqvire.read_element",
            "Read one authoritative model element by identifier or name.",
            object_schema(vec![
                ("identifier", json!({ "type": "string" })),
                ("name", json!({ "type": "string" })),
            ]),
        ),
        read_tool(
            "reqvire.search",
            "Search and filter model elements.",
            object_schema(vec![
                ("short", json!({ "type": "boolean" })),
                ("filter_file", json!({ "type": "string" })),
                ("filter_name", json!({ "type": "string" })),
                ("filter_type", json!({ "type": "string" })),
                ("filter_status", json!({ "type": "string" })),
                ("filter_priority", json!({ "type": "string" })),
                ("filter_risk", json!({ "type": "string" })),
                ("filter_owner", json!({ "type": "string" })),
                ("filter_content", json!({ "type": "string" })),
                ("filter_page_content", json!({ "type": "string" })),
                ("have_relations", json!({ "type": "string" })),
                ("not_have_relations", json!({ "type": "string" })),
                ("has_attachments", json!({ "type": "boolean" })),
                ("filter_attachment", json!({ "type": "string" })),
            ]),
        ),
        read_tool(
            "reqvire.model",
            "Generate model-centric structure.",
            object_schema(vec![
                ("from", json!({ "type": "string" })),
                ("reverse", json!({ "type": "boolean" })),
                ("filter_type", json!({ "type": "string" })),
            ]),
        ),
        read_tool(
            "reqvire.containment",
            "Generate folder/file/element containment hierarchy.",
            object_schema(vec![("short", json!({ "type": "boolean" }))]),
        ),
        read_tool(
            "reqvire.collect",
            "Collect capability, requirement, or ontology context upstream or downstream.",
            required_object_schema(
                vec![
                    ("element_name", json!({ "type": "string" })),
                    (
                        "direction",
                        json!({ "type": "string", "enum": ["UPSTREAM", "DOWNSTREAM"] }),
                    ),
                ],
                vec!["element_name"],
            ),
        ),
        read_tool(
            "reqvire.submodels",
            "Analyze independent capability and requirement submodels.",
            object_schema(vec![("from", json!({ "type": "string" }))]),
        ),
        read_tool(
            "reqvire.semantic.ontologies",
            "Collect ontology RDF and semantic-contract SHACL shapes.",
            object_schema(vec![
                (
                    "format",
                    json!({ "type": "string", "enum": ["turtle", "jsonld"], "default": "turtle" }),
                ),
                (
                    "content",
                    json!({ "type": "string", "enum": ["rdf", "shacl", "both"], "default": "both" }),
                ),
                ("full", json!({ "type": "boolean", "default": false })),
            ]),
        ),
        read_tool(
            "reqvire.semantic.prefixes",
            "List ontology-defined semantic prefixes and namespaces.",
            object_schema(vec![]),
        ),
        read_tool(
            "reqvire.semantic.sparql",
            "Run a read-only SPARQL query over Reqvire semantic RDF evidence.",
            required_object_schema(
                vec![
                    ("query", json!({ "type": "string" })),
                    ("full", json!({ "type": "boolean", "default": true })),
                ],
                vec!["query"],
            ),
        ),
        read_tool(
            "reqvire.lint",
            "Analyze model quality without applying fixes.",
            object_schema(vec![
                ("fixable", json!({ "type": "boolean" })),
                ("auditable", json!({ "type": "boolean" })),
            ]),
        ),
        read_tool(
            "reqvire.coverage",
            "Generate verification and implementation coverage.",
            object_schema(vec![]),
        ),
        read_tool(
            "reqvire.traces",
            "Generate verification traces.",
            object_schema(vec![
                ("from_folder", json!({ "type": "string" })),
                ("links_with_blobs", json!({ "type": "boolean" })),
                ("filter_id", json!({ "type": "string" })),
                ("filter_name", json!({ "type": "string" })),
                ("filter_type", json!({ "type": "string" })),
            ]),
        ),
        read_tool(
            "reqvire.resources",
            "Report files referenced by the model.",
            object_schema(vec![]),
        ),
        read_tool(
            "reqvire.change_impact",
            "Analyze change impact against a git commit.",
            object_schema(vec![(
                "git_commit",
                json!({ "type": "string", "default": "HEAD" }),
            )]),
        ),
    ];

    if enable_mutations {
        tools.push(conditional_tool(
            "reqvire.format",
            "Preview formatting, or apply formatting when mutation mode is enabled and fix=true.",
            object_schema(vec![
                ("fix", json!({ "type": "boolean", "default": false })),
                (
                    "with_full_relations",
                    json!({ "type": "boolean", "default": false }),
                ),
            ]),
        ));
    } else {
        tools.push(read_tool(
            "reqvire.format",
            "Preview formatting without applying changes.",
            object_schema(vec![
                (
                    "fix",
                    json!({ "type": "boolean", "enum": [false], "default": false }),
                ),
                (
                    "with_full_relations",
                    json!({ "type": "boolean", "default": false }),
                ),
            ]),
        ));
    }

    if enable_mutations {
        tools.extend(vec![
            mutation_tool(
                "reqvire.add_element",
                "Add a new element from Markdown content.",
                required_object_schema(
                    vec![
                        ("file", json!({ "type": "string" })),
                        ("content", json!({ "type": "string" })),
                        (
                            "override_existing",
                            json!({ "type": "boolean", "default": false }),
                        ),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["file", "content"],
                ),
            ),
            mutation_tool(
                "reqvire.remove_element",
                "Remove an element.",
                required_object_schema(
                    vec![
                        ("element_name", json!({ "type": "string" })),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["element_name"],
                ),
            ),
            mutation_tool(
                "reqvire.move_element",
                "Move an element to another file.",
                required_object_schema(
                    vec![
                        ("element_name", json!({ "type": "string" })),
                        ("file", json!({ "type": "string" })),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["element_name", "file"],
                ),
            ),
            mutation_tool(
                "reqvire.rename_element",
                "Rename an element.",
                required_object_schema(
                    vec![
                        ("element_name", json!({ "type": "string" })),
                        ("new_name", json!({ "type": "string" })),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["element_name", "new_name"],
                ),
            ),
            mutation_tool(
                "reqvire.merge_elements",
                "Merge source elements into a target element.",
                required_object_schema(
                    vec![
                        ("target", json!({ "type": "string" })),
                        (
                            "sources",
                            json!({ "type": "array", "items": { "type": "string" } }),
                        ),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["target", "sources"],
                ),
            ),
            mutation_tool(
                "reqvire.move_file",
                "Move a model file and its elements.",
                required_object_schema(
                    vec![
                        ("source_file", json!({ "type": "string" })),
                        ("target_file", json!({ "type": "string" })),
                        ("squash", json!({ "type": "boolean", "default": false })),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["source_file", "target_file"],
                ),
            ),
            mutation_tool(
                "reqvire.link",
                "Add a relation or attachment.",
                required_object_schema(
                    vec![
                        ("source", json!({ "type": "string" })),
                        ("relation_type", json!({ "type": "string" })),
                        ("target", json!({ "type": "string" })),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["source", "relation_type", "target"],
                ),
            ),
            mutation_tool(
                "reqvire.unlink",
                "Remove a relation or attachment.",
                required_object_schema(
                    vec![
                        ("source", json!({ "type": "string" })),
                        ("target", json!({ "type": "string" })),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["source", "target"],
                ),
            ),
            mutation_tool(
                "reqvire.relink",
                "Replace an existing relation target.",
                required_object_schema(
                    vec![
                        ("source", json!({ "type": "string" })),
                        ("relation_type", json!({ "type": "string" })),
                        ("from_target", json!({ "type": "string" })),
                        ("to_target", json!({ "type": "string" })),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["source", "relation_type", "from_target", "to_target"],
                ),
            ),
            mutation_tool(
                "reqvire.move_asset",
                "Move an asset and update references.",
                required_object_schema(
                    vec![
                        ("old_path", json!({ "type": "string" })),
                        ("new_path", json!({ "type": "string" })),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["old_path", "new_path"],
                ),
            ),
            mutation_tool(
                "reqvire.remove_asset",
                "Remove an asset and update references.",
                required_object_schema(
                    vec![
                        ("file_path", json!({ "type": "string" })),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["file_path"],
                ),
            ),
        ]);
    }

    tools
}

pub fn resource_definitions() -> Vec<Value> {
    vec![
        json!({
            "uri": "reqvire://workspace/status",
            "name": "Reqvire workspace status",
            "mimeType": "application/json",
            "description": "Workspace, git, and model status."
        }),
        json!({
            "uri": "reqvire://workspace/model-revision",
            "name": "Reqvire model revision",
            "mimeType": "application/json",
            "description": "Current workspace revision metadata."
        }),
        json!({
            "uri": "reqvire://tools/contract",
            "name": "Reqvire tool contract",
            "mimeType": "application/json",
            "description": "Tool definitions and Reqvire contract metadata."
        }),
    ]
}

pub fn validate_tool_arguments(
    tool_name: &str,
    arguments: &Value,
    enable_mutations: bool,
) -> Result<(), String> {
    let tools = tool_definitions(enable_mutations);
    let tool = tools
        .iter()
        .find(|tool| tool.get("name").and_then(Value::as_str) == Some(tool_name))
        .ok_or_else(|| format!("Unknown tool '{}'", tool_name))?;
    let schema = tool
        .get("inputSchema")
        .ok_or_else(|| format!("Tool '{}' has no inputSchema", tool_name))?;
    validate_object_schema(arguments, schema)
}

fn dispatch_tool(
    name: &str,
    args: &Value,
    enable_mutations: bool,
    with_size_estimates: bool,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    match name {
        "reqvire.workspace_status" => {
            workspace_status(excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.tool_contract" => Ok(json!({
            "mcp_protocol_version": MCP_PROTOCOL_VERSION,
            "tool_contract_version": TOOL_CONTRACT_VERSION,
            "mutation_tools_enabled": enable_mutations,
            "size_estimates_enabled": with_size_estimates,
            "tools": tool_definitions(enable_mutations)
        })),
        "reqvire.model_revision" => model_revision(excluded_filename_patterns, with_size_estimates),
        "reqvire.read_element" => {
            read_element(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.search" => search_tool(args, excluded_filename_patterns),
        "reqvire.model" => model_tool(args, excluded_filename_patterns, with_size_estimates),
        "reqvire.containment" => containment_tool(args, excluded_filename_patterns),
        "reqvire.collect" => collect_tool(args, excluded_filename_patterns),
        "reqvire.submodels" => submodels_tool(args, excluded_filename_patterns),
        "reqvire.semantic.ontologies" => {
            ontologies_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.semantic.prefixes" => {
            semantic_prefixes_tool(excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.semantic.sparql" => {
            sparql_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.lint" => lint_tool(args, excluded_filename_patterns),
        "reqvire.coverage" => coverage_tool(excluded_filename_patterns),
        "reqvire.traces" => traces_tool(args, excluded_filename_patterns),
        "reqvire.resources" => resources_tool(excluded_filename_patterns),
        "reqvire.change_impact" => change_impact_tool(args, excluded_filename_patterns),
        "reqvire.format" => format_tool(args, enable_mutations, excluded_filename_patterns),
        "reqvire.add_element" => add_element_tool(args, excluded_filename_patterns),
        "reqvire.remove_element" => remove_element_tool(args, excluded_filename_patterns),
        "reqvire.move_element" => move_element_tool(args, excluded_filename_patterns),
        "reqvire.rename_element" => rename_element_tool(args, excluded_filename_patterns),
        "reqvire.merge_elements" => merge_elements_tool(args, excluded_filename_patterns),
        "reqvire.move_file" => move_file_tool(args, excluded_filename_patterns),
        "reqvire.link" => link_tool(args, excluded_filename_patterns),
        "reqvire.unlink" => unlink_tool(args, excluded_filename_patterns),
        "reqvire.relink" => relink_tool(args, excluded_filename_patterns),
        "reqvire.move_asset" => move_asset_tool(args, excluded_filename_patterns),
        "reqvire.remove_asset" => remove_asset_tool(args, excluded_filename_patterns),
        _ => Err(ReqvireError::ProcessError(format!(
            "Unknown Reqvire tool '{}'",
            name
        ))),
    }
}

fn workspace_status(
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let model = load_model_with_options(excluded_filename_patterns, with_size_estimates)?;
    let mut files = BTreeSet::new();
    for element in model.graph_registry.get_all_elements() {
        files.insert(element.file_path.clone());
    }
    for file in model.graph_registry.pages.keys() {
        files.insert(file.clone());
    }

    Ok(json!({
        "workspace_root": current_dir_string(),
        "git": git_state(),
        "reqvire_version": env!("CARGO_PKG_VERSION"),
        "mcp_protocol_version": MCP_PROTOCOL_VERSION,
        "tool_contract_version": TOOL_CONTRACT_VERSION,
        "size_estimates_enabled": with_size_estimates,
        "model": {
            "valid": true,
            "fingerprint": model_fingerprint(&model),
            "element_count": model.graph_registry.nodes.len(),
            "file_count": files.len()
        }
    }))
}

fn model_revision(
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let model = load_model_with_options(excluded_filename_patterns, with_size_estimates)?;
    Ok(json!({
        "workspace_root": current_dir_string(),
        "git": git_state(),
        "reqvire_version": env!("CARGO_PKG_VERSION"),
        "mcp_protocol_version": MCP_PROTOCOL_VERSION,
        "tool_contract_version": TOOL_CONTRACT_VERSION,
        "size_estimates_enabled": with_size_estimates,
        "model_fingerprint": model_fingerprint(&model)
    }))
}

fn read_element(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let identifier = string_arg(args, "identifier");
    let name = string_arg(args, "name");
    if identifier.is_none() && name.is_none() {
        return Err(ReqvireError::ProcessError(
            "read_element requires 'identifier' or 'name'".to_string(),
        ));
    }

    let model = load_model_with_options(excluded_filename_patterns, with_size_estimates)?;
    let element = if let Some(identifier) = identifier {
        model.graph_registry.get_element(&identifier)
    } else {
        model.graph_registry.get_element_by_name(&name.unwrap())
    }
    .ok_or_else(|| ReqvireError::ElementNotFound("Element not found".to_string()))?;

    Ok(serde_json::to_value(element)
        .map_err(|e| ReqvireError::SerializationError(e.to_string()))?)
}

fn search_tool(args: &Value, excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let model = load_model(excluded_filename_patterns)?;
    let filters = search::SearchFilters::new(
        string_arg(args, "filter_file").as_deref(),
        string_arg(args, "filter_name").as_deref(),
        string_arg(args, "filter_type").as_deref(),
        string_arg(args, "filter_status").as_deref(),
        string_arg(args, "filter_priority").as_deref(),
        string_arg(args, "filter_risk").as_deref(),
        string_arg(args, "filter_owner").as_deref(),
        string_arg(args, "filter_content").as_deref(),
        string_arg(args, "filter_page_content").as_deref(),
        string_arg(args, "have_relations").as_deref(),
        string_arg(args, "not_have_relations").as_deref(),
        bool_arg(args, "has_attachments", false),
        string_arg(args, "filter_attachment").as_deref(),
    )?;
    parse_json_string(search::generate_search_report(
        &model.graph_registry,
        &filters,
        true,
        bool_arg(args, "short", false),
    )?)
}

fn model_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let model = load_model_with_options(excluded_filename_patterns, with_size_estimates)?;
    let filter_type = string_arg(args, "filter_type");
    let type_filter: Option<Vec<&str>> = filter_type
        .as_deref()
        .map(|s| s.split(',').map(|t| t.trim()).collect());
    parse_json_string(report_model::generate_model_report(
        &model.graph_registry,
        string_arg(args, "from").as_deref(),
        bool_arg(args, "reverse", false),
        type_filter,
        true,
        "LR",
    )?)
}

fn containment_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let model = load_model(excluded_filename_patterns)?;
    let hierarchy =
        ContainmentHierarchy::build(&model.graph_registry, bool_arg(args, "short", false))?;
    Ok(serde_json::to_value(hierarchy)
        .map_err(|e| ReqvireError::SerializationError(e.to_string()))?)
}

fn collect_tool(args: &Value, excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let model = load_model(excluded_filename_patterns)?;
    let direction = match string_arg(args, "direction")
        .unwrap_or_else(|| "UPSTREAM".to_string())
        .to_uppercase()
        .as_str()
    {
        "UPSTREAM" => report_collect::CollectDirection::Upstream,
        "DOWNSTREAM" => report_collect::CollectDirection::Downstream,
        other => {
            return Err(ReqvireError::ProcessError(format!(
                "Invalid direction '{}'. Valid values: UPSTREAM, DOWNSTREAM",
                other
            )));
        }
    };
    let git_root = git_commands::get_git_root_dir()?;
    parse_json_string(report_collect::generate_collect_report(
        &model.graph_registry,
        &required_string_arg(args, "element_name")?,
        &git_root,
        true,
        direction,
    )?)
}

fn submodels_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let model = load_model(excluded_filename_patterns)?;
    let report = report_submodels::generate_submodels_report(
        &model.graph_registry,
        string_arg(args, "from").as_deref(),
    )?;
    parse_json_string(report.to_json_string())
}

fn ontologies_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let model = load_model_with_options(excluded_filename_patterns, with_size_estimates)?;
    let semantic_store = model.semantic_store.as_ref().ok_or_else(|| {
        ReqvireError::ProcessError("Parsed model is missing semantic RDF query state".to_string())
    })?;
    let format = string_arg(args, "format").unwrap_or_else(|| "turtle".to_string());
    let content_filter = ontology_content_filter(args)?;
    let full = bool_arg(args, "full", false);
    let index = filtered_semantic_index(&semantic_store.index, content_filter);
    match format.as_str() {
        "turtle" => Ok(json!({
            "format": "turtle",
            "content_filter": content_filter.as_str(),
            "full": full,
            "content": if full {
                index.serialize_full(SemanticExportFormat::Turtle, &model.graph_registry)?
            } else {
                index.serialize(SemanticExportFormat::Turtle)?
            },
            "summary": index.summary,
            "blocks": index.blocks,
            "diagnostics": index.diagnostics,
            "ontology_documents": index.ontology_documents,
            "ontology_declarations": index.ontology_declarations,
            "shape_references": index.shape_references
        })),
        "jsonld" => {
            let content = if full {
                index.serialize_full(SemanticExportFormat::JsonLd, &model.graph_registry)?
            } else {
                index.serialize(SemanticExportFormat::JsonLd)?
            };
            let jsonld: Value = serde_json::from_str(&content)
                .map_err(|e| ReqvireError::SerializationError(e.to_string()))?;
            Ok(json!({
                "format": "jsonld",
                "content_filter": content_filter.as_str(),
                "full": full,
                "content": content,
                "jsonld": jsonld,
                "summary": index.summary,
                "blocks": index.blocks,
                "diagnostics": index.diagnostics,
                "ontology_documents": index.ontology_documents,
                "ontology_declarations": index.ontology_declarations,
                "shape_references": index.shape_references
            }))
        }
        other => Err(ReqvireError::ProcessError(format!(
            "Invalid ontology format '{}'. Valid values: turtle, jsonld",
            other
        ))),
    }
}

#[derive(Clone, Copy)]
enum OntologyContentFilter {
    Rdf,
    Shacl,
    Both,
}

impl OntologyContentFilter {
    fn as_str(self) -> &'static str {
        match self {
            Self::Rdf => "rdf",
            Self::Shacl => "shacl",
            Self::Both => "both",
        }
    }
}

fn ontology_content_filter(args: &Value) -> Result<OntologyContentFilter, ReqvireError> {
    match string_arg(args, "content")
        .unwrap_or_else(|| "both".to_string())
        .as_str()
    {
        "rdf" => Ok(OntologyContentFilter::Rdf),
        "shacl" => Ok(OntologyContentFilter::Shacl),
        "both" => Ok(OntologyContentFilter::Both),
        other => Err(ReqvireError::ProcessError(format!(
            "Invalid ontology content filter '{}'. Valid values: rdf, shacl, both",
            other
        ))),
    }
}

fn filtered_semantic_index(
    source: &semantic_contract::SemanticIndex,
    content_filter: OntologyContentFilter,
) -> semantic_contract::SemanticIndex {
    if matches!(content_filter, OntologyContentFilter::Both) {
        return source.clone();
    }

    let mut index = source.clone();
    index.blocks.retain(|block| match content_filter {
        OntologyContentFilter::Rdf => {
            matches!(block.kind, semantic_contract::SemanticBlockKind::Ontology)
        }
        OntologyContentFilter::Shacl => {
            matches!(block.kind, semantic_contract::SemanticBlockKind::Shapes)
        }
        OntologyContentFilter::Both => true,
    });

    if matches!(content_filter, OntologyContentFilter::Shacl) {
        index.ontology_documents.clear();
        index.ontology_declarations.clear();
        index.ontology_projection = semantic_contract::OntologyProjectionGraph {
            id: "urn:reqvire:ontology-projection:empty".to_string(),
            derivation_mode: semantic_contract::OntologyProjectionDerivationMode::DirectAuthored,
            projections: Vec::new(),
            constructs: Vec::new(),
            symbols: Vec::new(),
        };
    } else {
        index.shape_references.clear();
    }

    let ontology_blocks = index
        .blocks
        .iter()
        .filter(|block| matches!(block.kind, semantic_contract::SemanticBlockKind::Ontology))
        .count();
    let shape_blocks = index
        .blocks
        .iter()
        .filter(|block| matches!(block.kind, semantic_contract::SemanticBlockKind::Shapes))
        .count();
    let total_quads = index.blocks.iter().map(|block| block.quads.len()).sum();
    index.summary = semantic_contract::SemanticIndexSummary {
        ontology_blocks,
        shape_blocks,
        total_blocks: index.blocks.len(),
        total_quads,
    };

    index
}

fn semantic_prefixes_tool(
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let model = load_model_with_options(excluded_filename_patterns, with_size_estimates)?;
    let semantic_store = model.semantic_store.as_ref().ok_or_else(|| {
        ReqvireError::ProcessError("Parsed model is missing semantic RDF query state".to_string())
    })?;

    let mut prefixes = Vec::new();
    let mut prefix_namespaces: std::collections::BTreeMap<String, BTreeSet<String>> =
        std::collections::BTreeMap::new();

    for declaration in &semantic_store.index.ontology_documents {
        prefix_namespaces
            .entry(declaration.ontology_prefix.clone())
            .or_default()
            .insert(declaration.term_namespace.clone());

        let source = ontology_prefix_source(&model, declaration);
        prefixes.push(json!({
            "prefix": declaration.ontology_prefix,
            "namespace": declaration.term_namespace,
            "ontology_base": declaration.ontology_base,
            "term_namespace": declaration.term_namespace,
            "ontology_document_iri": declaration.iri,
            "source": source,
            "contributors": declaration.element_identifiers.iter().zip(declaration.element_names.iter()).map(|(identifier, name)| {
                json!({
                    "element_identifier": identifier,
                    "element_name": name
                })
            }).collect::<Vec<_>>()
        }));
    }

    prefixes.sort_by(|left, right| {
        let left_prefix = left
            .get("prefix")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let right_prefix = right
            .get("prefix")
            .and_then(Value::as_str)
            .unwrap_or_default();
        left_prefix.cmp(right_prefix).then_with(|| {
            left.get("namespace")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .cmp(
                    right
                        .get("namespace")
                        .and_then(Value::as_str)
                        .unwrap_or_default(),
                )
        })
    });

    let conflicts: Vec<Value> = prefix_namespaces
        .iter()
        .filter(|(_prefix, namespaces)| namespaces.len() > 1)
        .map(|(prefix, namespaces)| {
            json!({
                "prefix": prefix,
                "namespaces": namespaces.iter().cloned().collect::<Vec<_>>()
            })
        })
        .collect();

    let sparql_prefix_block = prefixes
        .iter()
        .filter_map(|entry| {
            Some(format!(
                "PREFIX {}: <{}>",
                entry.get("prefix")?.as_str()?,
                entry.get("namespace")?.as_str()?
            ))
        })
        .collect::<Vec<_>>()
        .join("\n");
    let sparql_prefix_block = if sparql_prefix_block.is_empty() {
        String::new()
    } else {
        format!("{}\n", sparql_prefix_block)
    };

    let namespace_count: usize = prefix_namespaces
        .values()
        .flat_map(|namespaces| namespaces.iter())
        .collect::<BTreeSet<_>>()
        .len();

    Ok(json!({
        "prefixes": prefixes,
        "sparql_prefix_block": sparql_prefix_block,
        "conflicts": conflicts,
        "summary": {
            "prefix_count": prefix_namespaces.len(),
            "namespace_count": namespace_count,
            "ontology_document_count": semantic_store.index.ontology_documents.len(),
            "conflict_count": conflicts.len()
        },
        "diagnostics": semantic_store.index.diagnostics,
        "model_fingerprint": model_fingerprint(&model)
    }))
}

fn ontology_prefix_source(
    model: &ModelManager,
    declaration: &semantic_contract::OntologyDocumentDeclaration,
) -> Value {
    let source_element = declaration
        .element_identifiers
        .iter()
        .filter_map(|identifier| model.graph_registry.get_element(identifier))
        .find(|element| {
            element.metadata.get("ontology_base") == Some(&declaration.ontology_base)
                && element.metadata.get("ontology_prefix") == Some(&declaration.ontology_prefix)
        })
        .or_else(|| {
            declaration
                .element_identifiers
                .iter()
                .filter_map(|identifier| model.graph_registry.get_element(identifier))
                .find(|element| {
                    element.metadata.contains_key("ontology_base")
                        || element.metadata.contains_key("ontology_prefix")
                })
        })
        .or_else(|| {
            declaration
                .element_identifiers
                .iter()
                .find_map(|identifier| model.graph_registry.get_element(identifier))
        });

    match source_element {
        Some(element) => json!({
            "element_identifier": element.identifier,
            "element_name": element.name,
            "file_path": element.file_path,
            "line_number": element.line_number,
            "content": semantic_prefix_source_content(&element.content)
        }),
        None => json!({
            "element_identifier": null,
            "element_name": null,
            "file_path": null,
            "line_number": null,
            "content": ""
        }),
    }
}

fn semantic_prefix_source_content(content: &str) -> String {
    let mut result = Vec::new();
    let mut skip_semantic_section = false;

    for line in content.lines() {
        if let Some(section) = line.trim().strip_prefix("#### ") {
            skip_semantic_section = matches!(section.trim(), "Ontology" | "Shapes");
            if skip_semantic_section {
                continue;
            }
        }
        if !skip_semantic_section {
            result.push(line);
        }
    }

    result.join("\n").trim().to_string()
}

fn sparql_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let query = required_string_arg(args, "query")?;
    let full = bool_arg(args, "full", true);
    let model = load_model_with_options(excluded_filename_patterns, with_size_estimates)?;
    let semantic_store = model.semantic_store.as_ref().ok_or_else(|| {
        ReqvireError::ProcessError("Parsed model is missing semantic RDF query state".to_string())
    })?;

    let results = SparqlEvaluator::new()
        .parse_query(&query)
        .map_err(|error| ReqvireError::ProcessError(format!("Invalid SPARQL query: {}", error)))?
        .on_store(semantic_store.store(full))
        .execute()
        .map_err(|error| ReqvireError::ProcessError(format!("SPARQL query failed: {}", error)))?;

    let mut result = match results {
        QueryResults::Solutions(mut solutions) => {
            let variables: Vec<String> = solutions
                .variables()
                .iter()
                .map(|variable| variable.as_str().to_string())
                .collect();
            let mut bindings = Vec::new();
            for solution in &mut solutions {
                let solution = solution.map_err(|error| {
                    ReqvireError::ProcessError(format!(
                        "SPARQL solution evaluation failed: {}",
                        error
                    ))
                })?;
                let mut binding = serde_json::Map::new();
                for variable in &variables {
                    if let Some(term) = solution.get(variable.as_str()) {
                        binding.insert(variable.clone(), rdf_term_json(term));
                    }
                }
                bindings.push(Value::Object(binding));
            }
            json!({
                "result_type": "select",
                "variables": variables,
                "bindings": bindings,
                "row_count": bindings.len()
            })
        }
        QueryResults::Boolean(value) => json!({
            "result_type": "ask",
            "boolean": value
        }),
        QueryResults::Graph(triples) => {
            let mut rows = Vec::new();
            for triple in triples {
                let triple = triple.map_err(|error| {
                    ReqvireError::ProcessError(format!("SPARQL graph evaluation failed: {}", error))
                })?;
                rows.push(rdf_triple_json(&triple));
            }
            json!({
                "result_type": "graph",
                "triples": rows,
                "triple_count": rows.len()
            })
        }
    };

    if let Value::Object(ref mut object) = result {
        object.insert("format".to_string(), json!("sparql"));
        object.insert("full".to_string(), json!(full));
        object.insert("summary".to_string(), json!(semantic_store.index.summary));
        object.insert(
            "diagnostics".to_string(),
            json!(semantic_store.index.diagnostics),
        );
        object.insert(
            "model_fingerprint".to_string(),
            json!(model_fingerprint(&model)),
        );
    }

    Ok(result)
}

fn rdf_triple_json(triple: &Triple) -> Value {
    json!({
        "subject": rdf_subject_json(&triple.subject),
        "predicate": {
            "kind": "iri",
            "value": triple.predicate.as_str(),
            "iri": triple.predicate.as_str()
        },
        "object": rdf_term_json(&triple.object)
    })
}

fn rdf_subject_json(subject: &NamedOrBlankNode) -> Value {
    match subject {
        NamedOrBlankNode::NamedNode(node) => json!({
            "kind": "iri",
            "value": node.as_str(),
            "iri": node.as_str()
        }),
        NamedOrBlankNode::BlankNode(node) => json!({
            "kind": "blank-node",
            "value": node.as_str(),
            "id": node.as_str()
        }),
    }
}

fn rdf_term_json(term: &Term) -> Value {
    match term {
        Term::NamedNode(node) => json!({
            "kind": "iri",
            "value": node.as_str(),
            "iri": node.as_str()
        }),
        Term::BlankNode(node) => json!({
            "kind": "blank-node",
            "value": node.as_str(),
            "id": node.as_str()
        }),
        Term::Literal(literal) => {
            let mut value = json!({
                "kind": "literal",
                "value": literal.value(),
                "datatype": literal.datatype().as_str()
            });
            if let Some(language) = literal.language() {
                if let Value::Object(ref mut object) = value {
                    object.insert("language".to_string(), json!(language));
                }
            }
            value
        }
    }
}

fn lint_tool(args: &Value, excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let model = load_model_lenient(excluded_filename_patterns)?;
    let report = lint::analyze_model(&model.graph_registry);
    parse_json_string(report.to_json_string(
        bool_arg(args, "fixable", false),
        bool_arg(args, "auditable", false),
    ))
}

fn coverage_tool(excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let model = load_model(excluded_filename_patterns)?;
    let report = report_coverage::generate_coverage_report(&model.graph_registry);
    parse_json_string(report.to_json_string())
}

fn traces_tool(args: &Value, excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let model = load_model(excluded_filename_patterns)?;
    let generator = verification_trace::VerificationTraceGenerator::new(
        &model.graph_registry,
        bool_arg(args, "links_with_blobs", false),
        string_arg(args, "from_folder"),
    );
    let mut report = generator.generate();
    if args.get("filter_id").is_some()
        || args.get("filter_name").is_some()
        || args.get("filter_type").is_some()
    {
        report = verification_trace::apply_filters(
            report,
            string_arg(args, "filter_id").as_deref(),
            string_arg(args, "filter_name").as_deref(),
            string_arg(args, "filter_type").as_deref(),
        )?;
    }
    Ok(
        serde_json::to_value(report)
            .map_err(|e| ReqvireError::SerializationError(e.to_string()))?,
    )
}

fn resources_tool(excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let model = load_model(excluded_filename_patterns)?;
    let report = report_resources::generate_resources_report(&model.graph_registry);
    parse_json_string(report.to_json_string())
}

fn change_impact_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let git_commit = string_arg(args, "git_commit").unwrap_or_else(|| "HEAD".to_string());
    let model = load_model(excluded_filename_patterns)?;
    let mut reference_model = ModelManager::new();
    match reference_model.parse_and_validate_with_mode(
        Some(&git_commit),
        excluded_filename_patterns,
        false,
    ) {
        Ok(_) => {}
        Err(ReqvireError::ValidationError(_)) => {
            reference_model.parse_and_validate_with_mode(
                Some(&git_commit),
                excluded_filename_patterns,
                true,
            )?;
        }
        Err(e) => return Err(e),
    }

    let base_url = git_commands::get_repository_base_url()?;
    let current_commit = git_commands::get_commit_hash()?;
    let report = change_impact::compute_change_impact(
        &model.graph_registry,
        &reference_model.graph_registry,
    )
    .map_err(|e| ReqvireError::ProcessError(format!("{:?}", e)))?;
    parse_json_string(report.to_json_string(&base_url, &current_commit, &git_commit))
}

fn format_tool(
    args: &Value,
    enable_mutations: bool,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let fix = bool_arg(args, "fix", false);
    if fix && !enable_mutations {
        return Err(ReqvireError::ProcessError(
            "format with fix=true requires --enable-mutations".to_string(),
        ));
    }
    let model = load_model(excluded_filename_patterns)?;
    let result = format_files(
        &model.graph_registry,
        !fix,
        bool_arg(args, "with_full_relations", false),
    )?;
    parse_json_string(render_diff_json(&result))
}

fn add_element_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::add_element(
        &mut model,
        &required_string_arg(args, "content")?,
        &required_string_arg(args, "file")?,
        excluded_filename_patterns,
        &current_dir_path(),
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
        bool_arg(args, "override_existing", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn remove_element_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let element_id = model
        .graph_registry
        .find_element_by_name(&required_string_arg(args, "element_name")?)?;
    let result = crud::remove_element(
        &mut model,
        &element_id,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn move_element_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let element_id = model
        .graph_registry
        .find_element_by_name(&required_string_arg(args, "element_name")?)?;
    let result = crud::move_element(
        &mut model,
        &element_id,
        &required_string_arg(args, "file")?,
        excluded_filename_patterns,
        &current_dir_path(),
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn rename_element_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let element_id = model
        .graph_registry
        .find_element_by_name(&required_string_arg(args, "element_name")?)?;
    let result = crud::rename_element(
        &mut model,
        &element_id,
        &required_string_arg(args, "new_name")?,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn merge_elements_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::merge_elements(
        &mut model,
        &required_string_arg(args, "target")?,
        &string_array_arg(args, "sources")?,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn move_file_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::move_file(
        &mut model,
        &required_string_arg(args, "source_file")?,
        &required_string_arg(args, "target_file")?,
        &current_dir_path(),
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
        bool_arg(args, "squash", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn link_tool(args: &Value, excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let source = required_string_arg(args, "source")?;
    let relation_type = required_string_arg(args, "relation_type")?;
    let target = required_string_arg(args, "target")?;
    let git_root = git_commands::get_git_root_dir()?;
    let result = if relation_type == "attaching" {
        if crate::utils::is_external_url(&target) {
            return Err(ReqvireError::ProcessError(
                "External URLs cannot be attached. Use a relation type such as trace instead."
                    .to_string(),
            ));
        }
        crud::attach_element_identifier(
            &mut model,
            &source,
            &target,
            &git_root,
            bool_arg(args, "dry_run", false),
        )?
    } else {
        crud::link(
            &mut model,
            &source,
            &relation_type,
            &target,
            &git_root,
            bool_arg(args, "dry_run", false),
        )?
    };
    parse_json_string(render_crud_json(&result))
}

fn unlink_tool(args: &Value, excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::unlink(
        &mut model,
        &required_string_arg(args, "source")?,
        &required_string_arg(args, "target")?,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn relink_tool(args: &Value, excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::relink(
        &mut model,
        &required_string_arg(args, "source")?,
        &required_string_arg(args, "relation_type")?,
        &required_string_arg(args, "from_target")?,
        &required_string_arg(args, "to_target")?,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn move_asset_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::mv_asset(
        &mut model,
        &required_string_arg(args, "old_path")?,
        &required_string_arg(args, "new_path")?,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn remove_asset_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::rm_asset(
        &mut model,
        &required_string_arg(args, "file_path")?,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn load_model(excluded_filename_patterns: &GlobSet) -> Result<ModelManager, ReqvireError> {
    load_model_with_options(excluded_filename_patterns, false)
}

fn load_model_with_options(
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<ModelManager, ReqvireError> {
    let mut model = ModelManager::new();
    model.parse_and_validate_with_options(
        None,
        excluded_filename_patterns,
        ModelBuildOptions {
            lenient: false,
            with_size_estimates,
        },
    )?;
    Ok(model)
}

fn load_model_lenient(excluded_filename_patterns: &GlobSet) -> Result<ModelManager, ReqvireError> {
    let mut model = ModelManager::new();
    model.parse_and_validate_with_mode(None, excluded_filename_patterns, true)?;
    Ok(model)
}

fn parse_json_string(json_str: String) -> Result<Value, ReqvireError> {
    serde_json::from_str(&json_str).map_err(|e| ReqvireError::SerializationError(e.to_string()))
}

fn required_string_arg(args: &Value, name: &str) -> Result<String, ReqvireError> {
    string_arg(args, name).ok_or_else(|| {
        ReqvireError::ProcessError(format!("Missing required string argument '{}'", name))
    })
}

fn string_arg(args: &Value, name: &str) -> Option<String> {
    args.get(name)
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn bool_arg(args: &Value, name: &str, default: bool) -> bool {
    args.get(name).and_then(Value::as_bool).unwrap_or(default)
}

fn string_array_arg(args: &Value, name: &str) -> Result<Vec<String>, ReqvireError> {
    let values = args.get(name).and_then(Value::as_array).ok_or_else(|| {
        ReqvireError::ProcessError(format!("Missing required string array argument '{}'", name))
    })?;
    values
        .iter()
        .map(|value| {
            value.as_str().map(ToString::to_string).ok_or_else(|| {
                ReqvireError::ProcessError(format!("Argument '{}' must contain only strings", name))
            })
        })
        .collect()
}

fn tool_exists(name: &str, enable_mutations: bool) -> bool {
    read_tool_names().contains(&name)
        || conditional_tool_names().contains(&name)
        || (enable_mutations && mutation_tool_names().contains(&name))
}

fn read_tool_names() -> Vec<&'static str> {
    vec![
        "reqvire.workspace_status",
        "reqvire.tool_contract",
        "reqvire.model_revision",
        "reqvire.read_element",
        "reqvire.search",
        "reqvire.model",
        "reqvire.containment",
        "reqvire.collect",
        "reqvire.submodels",
        "reqvire.semantic.ontologies",
        "reqvire.semantic.prefixes",
        "reqvire.semantic.sparql",
        "reqvire.lint",
        "reqvire.coverage",
        "reqvire.traces",
        "reqvire.resources",
        "reqvire.change_impact",
    ]
}

fn conditional_tool_names() -> Vec<&'static str> {
    vec!["reqvire.format"]
}

fn mutation_tool_names() -> Vec<&'static str> {
    vec![
        "reqvire.add_element",
        "reqvire.remove_element",
        "reqvire.move_element",
        "reqvire.rename_element",
        "reqvire.merge_elements",
        "reqvire.move_file",
        "reqvire.link",
        "reqvire.unlink",
        "reqvire.relink",
        "reqvire.move_asset",
        "reqvire.remove_asset",
    ]
}

fn read_tool(name: &str, description: &str, input_schema: Value) -> Value {
    tool(name, description, input_schema, true, false)
}

fn conditional_tool(name: &str, description: &str, input_schema: Value) -> Value {
    tool(name, description, input_schema, false, false)
}

fn mutation_tool(name: &str, description: &str, input_schema: Value) -> Value {
    tool(name, description, input_schema, false, true)
}

fn tool(
    name: &str,
    description: &str,
    input_schema: Value,
    read_only: bool,
    destructive: bool,
) -> Value {
    json!({
        "name": name,
        "description": description,
        "inputSchema": input_schema,
        "outputSchema": generic_output_schema(),
        "annotations": {
            "title": name,
            "readOnlyHint": read_only,
            "destructiveHint": destructive,
            "openWorldHint": false
        }
    })
}

fn object_schema(properties: Vec<(&str, Value)>) -> Value {
    required_object_schema(properties, Vec::new())
}

fn required_object_schema(properties: Vec<(&str, Value)>, required: Vec<&str>) -> Value {
    let mut map = serde_json::Map::new();
    for (name, schema) in properties {
        map.insert(name.to_string(), schema);
    }
    json!({
        "type": "object",
        "properties": map,
        "required": required,
        "additionalProperties": false
    })
}

fn generic_output_schema() -> Value {
    json!({
        "type": "object",
        "additionalProperties": true
    })
}

fn resource_contents(uri: &str, value: Value) -> Value {
    let text = serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string());
    json!({
        "contents": [{
            "uri": uri,
            "mimeType": "application/json",
            "text": text
        }]
    })
}

fn validate_object_schema(arguments: &Value, schema: &Value) -> Result<(), String> {
    let object = arguments
        .as_object()
        .ok_or_else(|| "Tool arguments must be a JSON object".to_string())?;
    let properties = schema
        .get("properties")
        .and_then(Value::as_object)
        .ok_or_else(|| "inputSchema.properties must be an object".to_string())?;

    if let Some(required) = schema.get("required").and_then(Value::as_array) {
        for item in required {
            let name = item
                .as_str()
                .ok_or_else(|| "inputSchema.required must contain strings".to_string())?;
            if !object.contains_key(name) {
                return Err(format!("Missing required argument '{}'", name));
            }
        }
    }

    if schema
        .get("additionalProperties")
        .and_then(Value::as_bool)
        .is_some_and(|allowed| !allowed)
    {
        for name in object.keys() {
            if !properties.contains_key(name) {
                return Err(format!("Unknown argument '{}'", name));
            }
        }
    }

    for (name, value) in object {
        if let Some(property_schema) = properties.get(name) {
            validate_property_type(name, value, property_schema)?;
            validate_property_enum(name, value, property_schema)?;
        }
    }

    Ok(())
}

fn validate_property_type(name: &str, value: &Value, schema: &Value) -> Result<(), String> {
    match schema.get("type").and_then(Value::as_str) {
        Some("string") if !value.is_string() => {
            Err(format!("Argument '{}' must be a string", name))
        }
        Some("boolean") if !value.is_boolean() => {
            Err(format!("Argument '{}' must be a boolean", name))
        }
        Some("array") => {
            let values = value
                .as_array()
                .ok_or_else(|| format!("Argument '{}' must be an array", name))?;
            if schema
                .get("items")
                .and_then(|items| items.get("type"))
                .and_then(Value::as_str)
                == Some("string")
                && values.iter().any(|item| !item.is_string())
            {
                return Err(format!("Argument '{}' must contain only strings", name));
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn validate_property_enum(name: &str, value: &Value, schema: &Value) -> Result<(), String> {
    if let Some(allowed) = schema.get("enum").and_then(Value::as_array) {
        if !allowed.iter().any(|allowed_value| allowed_value == value) {
            return Err(format!(
                "Argument '{}' has unsupported value '{}'",
                name, value
            ));
        }
    }
    Ok(())
}

fn git_state() -> Value {
    let head = git_output(["rev-parse", "HEAD"]);
    let status = git_output(["status", "--porcelain"]);
    json!({
        "head": head,
        "dirty": status.as_ref().is_some_and(|s| !s.trim().is_empty())
    })
}

fn git_output<const N: usize>(args: [&str; N]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn current_dir_path() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn current_dir_string() -> String {
    current_dir_path().to_string_lossy().to_string()
}

fn model_fingerprint(model: &ModelManager) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let mut elements = model.graph_registry.get_all_elements();
    elements.sort_by(|a, b| a.identifier.cmp(&b.identifier));
    for element in elements {
        element.identifier.hash(&mut hasher);
        element.name.hash(&mut hasher);
        element.element_type.as_str().hash(&mut hasher);
        element.content.hash(&mut hasher);
        element.file_path.hash(&mut hasher);
        for relation in &element.relations {
            relation.relation_type.name.hash(&mut hasher);
            relation.target.link.as_str().hash(&mut hasher);
        }
        for attachment in &element.attachments {
            attachment.target.as_str().hash(&mut hasher);
        }
    }

    format!("{:016x}", hasher.finish())
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
