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
use crate::semantic_store;
use crate::verification_trace;
use crate::{ModelBuildOptions, ModelManager};
use globset::GlobSet;
use o_kernel::rdf::{subject_iri, term_iri};
use oxigraph::model::{NamedOrBlankNode, Term, Triple};
use oxigraph::sparql::{QueryResults, SparqlEvaluator};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::process::Command;

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
                ("has_reused_contract_context", json!({ "type": "boolean" })),
                (
                    "filter_reused_contract_context",
                    json!({ "type": "string" }),
                ),
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
            "Collect capability, requirement, ontology, concept-scheme, or concept context upstream or downstream.",
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
            "Collect authored OWL/RDF ontology vocabulary only.",
            object_schema(vec![
                (
                    "format",
                    json!({ "type": "string", "enum": ["turtle", "jsonld"], "default": "turtle" }),
                ),
                (
                    "include_external",
                    json!({
                        "type": "boolean",
                        "default": false,
                        "description": "Include only the `reqvire:external-used-subset` graph role from external ontologies; raw external dependency graphs remain hidden."
                    }),
                ),
            ]),
        ),
        read_tool(
            "reqvire.semantic.shapes",
            "Collect semantic-contract SHACL shapes only.",
            object_schema(vec![(
                "format",
                json!({ "type": "string", "enum": ["turtle", "jsonld"], "default": "turtle" }),
            )]),
        ),
        read_tool(
            "reqvire.semantic.concepts",
            "Collect SKOS concept scheme/thesaurus triples only. Native concept schemes own concept_base/concept_prefix directly.",
            object_schema(vec![
                (
                    "format",
                    json!({ "type": "string", "enum": ["turtle", "jsonld"], "default": "turtle" }),
                ),
                (
                    "include_mappings",
                    json!({
                        "type": "boolean",
                        "default": false,
                        "description": "Include structural reqvire:mapsToConcept bridge triples when they point to generated native concepts."
                    }),
                ),
            ]),
        ),
        read_tool(
            "reqvire.concepts.list",
            "List standalone native SKOS concepts generated from Reqvire concept elements.",
            object_schema(vec![
                ("filter", json!({ "type": "string" })),
                ("scheme_iri", json!({ "type": "string" })),
            ]),
        ),
        read_tool(
            "reqvire.concepts.get",
            "Read one standalone native concept or concept scheme by IRI, source identifier, or source element name.",
            object_schema(vec![
                ("iri", json!({ "type": "string" })),
                ("identifier", json!({ "type": "string" })),
                ("name", json!({ "type": "string" })),
            ]),
        ),
        read_tool(
            "reqvire.concept_schemes.list",
            "List standalone native SKOS concept schemes and their concept_base/concept_prefix namespaces.",
            object_schema(vec![("filter", json!({ "type": "string" }))]),
        ),
        read_tool(
            "reqvire.concept_mappings.list",
            "List validated reqvire:mapsToConcept bridge triples from structural ontology terms to generated native SKOS concepts.",
            object_schema(vec![
                ("source_iri", json!({ "type": "string" })),
                ("target_iri", json!({ "type": "string" })),
            ]),
        ),
        read_tool(
            "reqvire.semantic.graph",
            "Collect the combined semantic graph.",
            object_schema(vec![
                (
                    "format",
                    json!({ "type": "string", "enum": ["turtle", "jsonld"], "default": "turtle" }),
                ),
                ("full", json!({ "type": "boolean", "default": false })),
                (
                    "include_external",
                    json!({
                        "type": "boolean",
                        "default": false,
                        "description": "Include only the `reqvire:external-used-subset` graph role from external ontologies; raw external dependency graphs remain hidden."
                    }),
                ),
            ]),
        ),
        read_tool(
            "reqvire.semantic.prefixes",
            "List ontology-defined semantic prefixes and namespaces.",
            object_schema(vec![(
                "include_external",
                json!({
                    "type": "boolean",
                    "default": false,
                    "description": "Include external prefix declarations from the o-kernel used-subset materialization; raw external-vocabulary prefixes remain hidden."
                }),
            )]),
        ),
        read_tool(
            "reqvire.semantic.vocabulary",
            "Page compact semantic vocabulary for SPARQL query construction.",
            object_schema(vec![
                (
                    "section",
                    json!({
                        "type": "string",
                        "enum": [
                            "all",
                            "prefixes",
                            "classes",
                            "properties",
                            "relation_families",
                            "controlled_vocabularies",
                            "concepts",
                            "semantic_contracts",
                            "query_patterns",
                            "source_map",
                            "diagnostics"
                        ],
                        "default": "all"
                    }),
                ),
                ("limit", json!({ "type": "integer", "default": 50 })),
                ("cursor", json!({ "type": "string" })),
                ("filter", json!({ "type": "string" })),
                (
                    "ontology_document",
                    json!({
                        "type": "string",
                        "description": "Exact OWL ontology document IRI used to limit vocabulary items to authored terms defined by that document."
                    }),
                ),
                (
                    "ontology_base",
                    json!({
                        "type": "string",
                        "description": "Alias for ontology_document; the resolved Reqvire ontology_base is the OWL ontology document IRI."
                    }),
                ),
                (
                    "include_source",
                    json!({ "type": "boolean", "default": true }),
                ),
                (
                    "include_examples",
                    json!({ "type": "boolean", "default": false }),
                ),
                (
                    "include_external",
                    json!({
                        "type": "boolean",
                        "default": false,
                        "description": "Include vocabulary terms from the used external ontology subset only; unused raw external dependency terms remain hidden."
                    }),
                ),
            ]),
        ),
        read_tool(
            "reqvire.semantic.sparql",
            "Run a read-only SPARQL query over Reqvire semantic RDF evidence.",
            required_object_schema(
                vec![
                    ("query", json!({ "type": "string" })),
                    ("full", json!({ "type": "boolean", "default": true })),
                    (
                        "include_external",
                        json!({
                                "type": "boolean",
                                "default": false,
                            "description": "Query the graph with the o-kernel external-used-subset layer; raw external dependency graphs are never exposed."
                        }),
                    ),
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
                "Add a relation or reused_contract_context.",
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
                "Remove a relation or reused_contract_context.",
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
        "reqvire.semantic.shapes" => {
            shapes_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.semantic.concepts" => {
            concepts_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.concepts.list" => {
            concepts_list_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.concepts.get" => {
            concept_get_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.concept_schemes.list" => {
            concept_schemes_list_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.concept_mappings.list" => {
            concept_mappings_list_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.semantic.graph" => {
            semantic_graph_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.semantic.prefixes" => {
            semantic_prefixes_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.semantic.vocabulary" => {
            semantic_vocabulary_tool(args, excluded_filename_patterns, with_size_estimates)
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
        bool_arg(args, "has_reused_contract_context", false),
        string_arg(args, "filter_reused_contract_context").as_deref(),
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
    let include_external = bool_arg(args, "include_external", false);
    let graph_layers = semantic_graph_layers(false, include_external);
    let mut serializable_index =
        filtered_semantic_index(&semantic_store.index, OntologyContentFilter::Ontology);
    serializable_index.apply_external_visibility(include_external)?;

    let (format_name, export_format) = match format.as_str() {
        "turtle" => ("turtle", SemanticExportFormat::Turtle),
        "jsonld" => ("jsonld", SemanticExportFormat::JsonLd),
        other => {
            return Err(ReqvireError::ProcessError(format!(
                "Invalid semantic ontology format '{}'. Valid values: turtle, jsonld",
                other
            )))
        }
    };
    let content =
        semantic_store
            .index
            .serialize_ontologies(export_format, include_external, None)?;
    let external_metadata = semantic_contract::external_materialization_metadata(
        &semantic_store.index,
        &serializable_index,
        include_external,
    );

    match format_name {
        "turtle" => Ok(json!({
            "format": format_name,
            "semantic_layer": "ontologies",
            "include_external": include_external,
            "external_materialization": external_metadata["external_materialization"].clone(),
            "external_counts": external_metadata["external_counts"].clone(),
            "graph_layers": graph_layers.clone(),
            "content": content,
            "summary": serializable_index.summary,
            "blocks": serializable_index.blocks,
            "external_blocks": serializable_index.external_blocks,
            "diagnostics": serializable_index.diagnostics,
            "ontology_documents": serializable_index.ontology_documents,
            "ontology_declarations": serializable_index.ontology_declarations,
            "shape_references": serializable_index.shape_references
        })),
        "jsonld" => {
            let jsonld: Value = serde_json::from_str(&content)
                .map_err(|e| ReqvireError::SerializationError(e.to_string()))?;
            Ok(json!({
            "format": format_name,
                "semantic_layer": "ontologies",
                "include_external": include_external,
                "external_materialization": external_metadata["external_materialization"].clone(),
                "external_counts": external_metadata["external_counts"].clone(),
                "graph_layers": graph_layers,
                "content": content,
                "jsonld": jsonld,
                    "summary": serializable_index.summary,
                    "blocks": serializable_index.blocks,
                    "external_blocks": serializable_index.external_blocks,
                    "diagnostics": serializable_index.diagnostics,
                    "ontology_documents": serializable_index.ontology_documents,
                    "ontology_declarations": serializable_index.ontology_declarations,
                    "shape_references": serializable_index.shape_references
                }))
        }
        _ => unreachable!(),
    }
}

fn shapes_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let model = load_model_with_options(excluded_filename_patterns, with_size_estimates)?;
    let semantic_store = model.semantic_store.as_ref().ok_or_else(|| {
        ReqvireError::ProcessError("Parsed model is missing semantic RDF query state".to_string())
    })?;
    let (format_name, export_format) = semantic_tool_format(args, "semantic shapes")?;
    let serializable_index =
        filtered_semantic_index(&semantic_store.index, OntologyContentFilter::Shacl);
    let content = semantic_store.index.serialize_shapes(export_format)?;
    semantic_layer_response(
        format_name,
        "shapes",
        content,
        &serializable_index,
        false,
        semantic_graph_layers(false, false),
    )
}

fn concepts_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let model = load_model_with_options(excluded_filename_patterns, with_size_estimates)?;
    let semantic_store = model.semantic_store.as_ref().ok_or_else(|| {
        ReqvireError::ProcessError("Parsed model is missing semantic RDF query state".to_string())
    })?;
    let (format_name, export_format) = semantic_tool_format(args, "semantic concepts")?;
    let include_mappings = bool_arg(args, "include_mappings", false);
    let content = semantic_store
        .index
        .serialize_concepts(export_format, include_mappings)?;
    let mut serializable_index =
        filtered_semantic_index(&semantic_store.index, OntologyContentFilter::Concepts);
    serializable_index.apply_external_visibility(false)?;
    let prefixes = vocabulary_prefixes(&model, &semantic_store.index);
    let compact_prefixes = compact_vocabulary_prefixes(&prefixes);
    let term_index = collect_term_index(&serializable_index);
    let concepts = concepts_section(
        &model,
        &serializable_index,
        &term_index,
        &compact_prefixes,
        true,
    );
    let mut object = semantic_layer_response(
        format_name,
        "concepts",
        content,
        &serializable_index,
        false,
        semantic_graph_layers(false, false),
    )?;
    if let Some(map) = object.as_object_mut() {
        map.insert("include_mappings".to_string(), json!(include_mappings));
        map.insert("concepts".to_string(), json!(concepts));
    }
    Ok(object)
}

fn concepts_list_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let model = load_model_with_options(excluded_filename_patterns, with_size_estimates)?;
    let filter = string_arg(args, "filter");
    let scheme_iri = string_arg(args, "scheme_iri");
    let concepts = native_concept_items(&model)?
        .into_iter()
        .filter(|item| item.get("kind").and_then(Value::as_str) == Some("concept"))
        .filter(|item| {
            scheme_iri.as_deref().map_or(true, |scheme| {
                item.get("scheme_iri").and_then(Value::as_str) == Some(scheme)
            })
        })
        .filter(|item| concept_item_matches_filter(item, filter.as_deref()))
        .collect::<Vec<_>>();
    let count = concepts.len();
    Ok(json!({
        "concepts": concepts,
        "count": count,
    }))
}

fn concept_schemes_list_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let model = load_model_with_options(excluded_filename_patterns, with_size_estimates)?;
    let filter = string_arg(args, "filter");
    let schemes = native_concept_items(&model)?
        .into_iter()
        .filter(|item| item.get("kind").and_then(Value::as_str) == Some("concept-scheme"))
        .filter(|item| concept_item_matches_filter(item, filter.as_deref()))
        .collect::<Vec<_>>();
    let count = schemes.len();
    Ok(json!({
        "concept_schemes": schemes,
        "count": count,
    }))
}

fn concept_get_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let iri = string_arg(args, "iri");
    let identifier = string_arg(args, "identifier");
    let name = string_arg(args, "name");
    if iri.is_none() && identifier.is_none() && name.is_none() {
        return Err(ReqvireError::ProcessError(
            "reqvire.concepts.get requires one of iri, identifier, or name".to_string(),
        ));
    }

    let model = load_model_with_options(excluded_filename_patterns, with_size_estimates)?;
    let concept = native_concept_items(&model)?.into_iter().find(|item| {
        iri.as_deref()
            .is_some_and(|value| item.get("iri").and_then(Value::as_str) == Some(value))
            || identifier.as_deref().is_some_and(|value| {
                item.get("source_element_identifier")
                    .and_then(Value::as_str)
                    == Some(value)
            })
            || name.as_deref().is_some_and(|value| {
                item.get("source_element_name").and_then(Value::as_str) == Some(value)
                    || item.get("pref_label").and_then(Value::as_str) == Some(value)
            })
    });

    concept
        .map(|concept| json!({ "concept": concept }))
        .ok_or_else(|| ReqvireError::ProcessError("No generated native concept matched the requested selector".to_string()))
}

fn concept_mappings_list_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    const REQVIRE_MAPS_TO_CONCEPT_IRI: &str = "https://www.reqvire.org/ontology#mapsToConcept";

    let source_filter = string_arg(args, "source_iri");
    let target_filter = string_arg(args, "target_iri");
    let model = load_model_with_options(excluded_filename_patterns, with_size_estimates)?;
    let semantic_store = model.semantic_store.as_ref().ok_or_else(|| {
        ReqvireError::ProcessError("Parsed model is missing semantic RDF query state".to_string())
    })?;
    let prefixes = vocabulary_prefixes(&model, &semantic_store.index);
    let compact_prefixes = compact_vocabulary_prefixes(&prefixes);
    let concepts_by_iri = native_concept_items(&model)?
        .into_iter()
        .filter_map(|concept| {
            concept
                .get("iri")
                .and_then(Value::as_str)
                .map(|iri| (iri.to_string(), concept.clone()))
        })
        .collect::<BTreeMap<_, _>>();

    let mut seen = BTreeSet::new();
    let mut mappings = Vec::new();
    for block in &semantic_store.index.blocks {
        for quad in &block.quads {
            if quad.predicate.as_str() != REQVIRE_MAPS_TO_CONCEPT_IRI {
                continue;
            }
            let Some(source_iri) = subject_iri(&quad.subject) else {
                continue;
            };
            let Some(target_iri) = term_iri(&quad.object) else {
                continue;
            };
            if source_filter.as_deref().is_some_and(|value| value != source_iri)
                || target_filter.as_deref().is_some_and(|value| value != target_iri)
            {
                continue;
            }
            let key = format!("{source_iri}\n{target_iri}");
            if !seen.insert(key) {
                continue;
            }
            mappings.push(json!({
                "source_iri": source_iri,
                "source_curie": curie(source_iri, &compact_prefixes),
                "target_iri": target_iri,
                "target_curie": curie(target_iri, &compact_prefixes),
                "predicate_iri": REQVIRE_MAPS_TO_CONCEPT_IRI,
                "predicate_curie": curie(REQVIRE_MAPS_TO_CONCEPT_IRI, &compact_prefixes),
                "source_block": {
                    "source": block.source,
                    "source_name": block.source_name,
                    "file_path": block.file_path,
                    "line_number": block.line_number,
                    "kind": block.kind.as_str(),
                },
                "target_concept": concepts_by_iri.get(target_iri).cloned(),
            }));
        }
    }
    let count = mappings.len();
    Ok(json!({
        "mappings": mappings,
        "count": count,
    }))
}

fn native_concept_items(model: &ModelManager) -> Result<Vec<Value>, ReqvireError> {
    let semantic_store = model.semantic_store.as_ref().ok_or_else(|| {
        ReqvireError::ProcessError("Parsed model is missing semantic RDF query state".to_string())
    })?;
    let mut serializable_index =
        filtered_semantic_index(&semantic_store.index, OntologyContentFilter::Concepts);
    serializable_index.apply_external_visibility(false)?;
    let prefixes = vocabulary_prefixes(model, &semantic_store.index);
    let compact_prefixes = compact_vocabulary_prefixes(&prefixes);
    let term_index = collect_term_index(&serializable_index);
    Ok(concepts_section(
        model,
        &serializable_index,
        &term_index,
        &compact_prefixes,
        true,
    ))
}

fn concept_item_matches_filter(item: &Value, filter: Option<&str>) -> bool {
    let Some(filter) = filter.map(|value| value.to_ascii_lowercase()) else {
        return true;
    };
    [
        "iri",
        "curie",
        "pref_label",
        "definition",
        "source_element_identifier",
        "source_element_name",
        "namespace_base",
        "namespace_prefix",
    ]
    .iter()
    .filter_map(|key| item.get(*key).and_then(Value::as_str))
    .any(|value| value.to_ascii_lowercase().contains(&filter))
}

fn semantic_graph_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let model = load_model_with_options(excluded_filename_patterns, with_size_estimates)?;
    let semantic_store = model.semantic_store.as_ref().ok_or_else(|| {
        ReqvireError::ProcessError("Parsed model is missing semantic RDF query state".to_string())
    })?;
    let (format_name, export_format) = semantic_tool_format(args, "semantic graph")?;
    let full = bool_arg(args, "full", false);
    let include_external = bool_arg(args, "include_external", false);
    let graph_layers = semantic_graph_layers(full, include_external);
    let mut serializable_index = semantic_store.index.clone();
    serializable_index.apply_external_visibility(include_external)?;
    let content =
        serializable_index.serialize_with_options(export_format, full, include_external)?;
    let mut object = semantic_layer_response(
        format_name,
        "graph",
        content,
        &serializable_index,
        include_external,
        graph_layers,
    )?;
    if let Some(map) = object.as_object_mut() {
        map.insert("full".to_string(), json!(full));
    }
    Ok(object)
}

fn semantic_tool_format(
    args: &Value,
    label: &str,
) -> Result<(&'static str, SemanticExportFormat), ReqvireError> {
    let format = string_arg(args, "format").unwrap_or_else(|| "turtle".to_string());
    match format.as_str() {
        "turtle" => Ok(("turtle", SemanticExportFormat::Turtle)),
        "jsonld" => Ok(("jsonld", SemanticExportFormat::JsonLd)),
        other => Err(ReqvireError::ProcessError(format!(
            "Invalid {} format '{}'. Valid values: turtle, jsonld",
            label, other
        ))),
    }
}

fn semantic_layer_response(
    format_name: &str,
    semantic_layer: &str,
    content: String,
    index: &semantic_contract::SemanticIndex,
    include_external: bool,
    graph_layers: Vec<Value>,
) -> Result<Value, ReqvireError> {
    let mut response = json!({
        "format": format_name,
        "semantic_layer": semantic_layer,
        "include_external": include_external,
        "graph_layers": graph_layers,
        "content": content,
        "summary": index.summary,
        "blocks": index.blocks,
        "external_blocks": index.external_blocks,
        "diagnostics": index.diagnostics,
        "ontology_documents": index.ontology_documents,
        "ontology_declarations": index.ontology_declarations,
        "shape_references": index.shape_references
    });
    if format_name == "jsonld" {
        let content = response
            .get("content")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let jsonld: Value = serde_json::from_str(content)
            .map_err(|e| ReqvireError::SerializationError(e.to_string()))?;
        if let Some(map) = response.as_object_mut() {
            map.insert("jsonld".to_string(), jsonld);
        }
    }
    Ok(response)
}

#[derive(Clone, Copy)]
enum OntologyContentFilter {
    Ontology,
    Concepts,
    Shacl,
}

fn filtered_semantic_index(
    source: &semantic_contract::SemanticIndex,
    content_filter: OntologyContentFilter,
) -> semantic_contract::SemanticIndex {
    let mut index = source.clone();
    index.blocks.retain(|block| match content_filter {
        OntologyContentFilter::Ontology => {
            matches!(block.kind, semantic_contract::SemanticBlockKind::Ontology)
        }
        OntologyContentFilter::Concepts => {
            matches!(block.kind, semantic_contract::SemanticBlockKind::Concepts)
        }
        OntologyContentFilter::Shacl => {
            matches!(block.kind, semantic_contract::SemanticBlockKind::Shapes)
        }
    });

    if matches!(
        content_filter,
        OntologyContentFilter::Concepts | OntologyContentFilter::Shacl
    ) {
        index.ontology_documents.clear();
        index.ontology_declarations.clear();
        index.external_blocks.clear();
        index.external_sources.clear();
    }

    if matches!(content_filter, OntologyContentFilter::Shacl) {
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

fn semantic_index_with_external_visibility(
    source: &semantic_contract::SemanticIndex,
    include_external: bool,
) -> Result<semantic_contract::SemanticIndex, ReqvireError> {
    source.with_external_visibility(include_external)
}

fn semantic_prefixes_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let include_external = bool_arg(args, "include_external", false);
    let model = load_model_with_options(excluded_filename_patterns, with_size_estimates)?;
    let semantic_store = model.semantic_store.as_ref().ok_or_else(|| {
        ReqvireError::ProcessError("Parsed model is missing semantic RDF query state".to_string())
    })?;
    let visible_index =
        semantic_index_with_external_visibility(&semantic_store.index, include_external)?;
    let external_metadata = semantic_contract::external_materialization_metadata(
        &semantic_store.index,
        &visible_index,
        include_external,
    );
    let graph_layers = semantic_graph_layers(false, include_external);

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
            "external": false,
            "source": source,
            "concept_schemes": [],
            "contributors": declaration.element_identifiers.iter().zip(declaration.element_names.iter()).map(|(identifier, name)| {
                json!({
                    "element_identifier": identifier,
                    "element_name": name
                })
            }).collect::<Vec<_>>()
        }));
    }

    for entry in concept_scheme_prefix_entries(&model) {
        if let (Some(prefix), Some(namespace)) = (
            entry.get("prefix").and_then(Value::as_str),
            entry.get("namespace").and_then(Value::as_str),
        ) {
            prefix_namespaces
                .entry(prefix.to_string())
                .or_default()
                .insert(namespace.to_string());
        }
        prefixes.push(entry);
    }

    if include_external {
        for source in used_external_sources(&semantic_store.index, &visible_index) {
            prefix_namespaces
                .entry(source.prefix.clone())
                .or_default()
                .insert(source.namespace.clone());

            prefixes.push(json!({
                "prefix": source.prefix,
                "namespace": source.namespace,
                "ontology_base": source.resource,
                "term_namespace": source.namespace,
                "ontology_document_iri": source.resource,
                "external": true,
                "external_materialization": "used_subset",
                "source_declaration": "declared",
                "source": external_ontology_prefix_source(&model, source)
            }));
        }
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
            "external_source_count": if include_external { external_metadata["external_counts"]["used_external_source_count"].as_u64().unwrap_or(0) } else { 0 },
            "conflict_count": conflicts.len()
        },
        "include_external": include_external,
        "external_materialization": external_metadata["external_materialization"].clone(),
        "external_counts": external_metadata["external_counts"].clone(),
        "graph_layers": graph_layers,
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

fn concept_scheme_prefix_entries(model: &ModelManager) -> Vec<Value> {
    let mut entries = model
        .graph_registry
        .get_all_elements()
        .into_iter()
        .filter(|element| element.element_type.is_concept_scheme())
        .filter_map(|element| {
            let payload = element.concept_scheme.as_ref()?;
            let namespace_base = payload.namespace_base.as_ref()?;
            let namespace_prefix = payload.namespace_prefix.as_ref()?;
            let namespace = format!("{}#", namespace_base.trim_end_matches('#'));
            let scheme_iri = format!(
                "{}{}",
                namespace,
                concept_vocabulary_local_name(&element.name)
            );
            let concept_scheme = json!({
                "scheme_element_identifier": element.identifier,
                "scheme_element_name": element.name,
                "file_path": element.file_path,
                "line_number": element.line_number,
                "scheme_iri": scheme_iri
            });
            let source = json!({
                "element_identifier": element.identifier,
                "element_name": element.name,
                "file_path": element.file_path,
                "line_number": element.line_number,
                "content": semantic_prefix_source_content(&element.content)
            });
            Some(json!({
                "prefix": namespace_prefix,
                "namespace": namespace,
                "concept_base": namespace_base,
                "term_namespace": namespace,
                "scheme_iri": scheme_iri,
                "external": false,
                "source": source,
                "concept_schemes": [concept_scheme],
                "contributors": [{
                    "element_identifier": element.identifier,
                    "element_name": element.name
                }]
            }))
        })
        .collect::<Vec<_>>();
    sort_items(&mut entries);
    entries
}

fn concept_scheme_vocabulary_prefixes(model: &ModelManager) -> Vec<Value> {
    let mut prefixes = model
        .graph_registry
        .get_all_elements()
        .into_iter()
        .filter(|element| element.element_type.is_concept_scheme())
        .filter_map(|element| {
            let payload = element.concept_scheme.as_ref()?;
            let namespace_base = payload.namespace_base.as_ref()?;
            let namespace_prefix = payload.namespace_prefix.as_ref()?;
            let namespace = format!("{}#", namespace_base.trim_end_matches('#'));
            Some(json!({
                "prefix": namespace_prefix,
                "namespace": namespace,
                "concept_base": namespace_base,
                "term_namespace": namespace,
                "external": false,
                "source": {
                    "element_identifier": element.identifier,
                    "element_name": element.name,
                    "file_path": element.file_path,
                    "line_number": element.line_number,
                    "content": semantic_prefix_source_content(&element.content)
                }
            }))
        })
        .collect::<Vec<_>>();
    sort_items(&mut prefixes);
    prefixes
}

fn concept_vocabulary_local_name(name: &str) -> String {
    let mut local = String::new();
    for part in name
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
    {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            local.push(first.to_ascii_uppercase());
            for ch in chars {
                local.push(ch);
            }
        }
    }
    if local.is_empty() {
        "Concept".to_string()
    } else {
        local
    }
}

fn external_ontology_prefix_source(
    model: &ModelManager,
    source: &semantic_contract::ExternalOntologySource,
) -> Value {
    let owner = model.graph_registry.get_element(&source.owner_identifier);
    match owner {
        Some(element) => json!({
            "element_identifier": element.identifier,
            "element_name": element.name,
            "file_path": element.file_path,
            "line_number": source.line_number,
            "content": semantic_prefix_source_content(&element.content),
            "external_source": {
                "resource": source.resource,
                "source": source.source,
                "format": source.format
            }
        }),
        None => json!({
            "element_identifier": source.owner_identifier,
            "element_name": source.owner_name,
            "file_path": null,
            "line_number": source.line_number,
            "content": "",
            "external_source": {
                "resource": source.resource,
                "source": source.source,
                "format": source.format
            }
        }),
    }
}

fn semantic_prefix_source_content(content: &str) -> String {
    let mut result = Vec::new();
    let mut skip_semantic_section = false;

    for line in content.lines() {
        if let Some(section) = line.trim().strip_prefix("#### ") {
            skip_semantic_section =
                matches!(section.trim(), "Ontology" | "Shapes" | "External Ontology");
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

#[derive(Clone)]
struct VocabularyPrefix {
    prefix: String,
    namespace: String,
}

fn semantic_vocabulary_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let section = string_arg(args, "section").unwrap_or_else(|| "all".to_string());
    let limit = usize_arg(args, "limit", 50).clamp(1, 200);
    let offset = string_arg(args, "cursor")
        .and_then(|cursor| cursor.parse::<usize>().ok())
        .unwrap_or(0);
    let filter = string_arg(args, "filter").map(|value| value.to_lowercase());
    let ontology_document_filter =
        string_arg(args, "ontology_document").or_else(|| string_arg(args, "ontology_base"));
    let include_source = bool_arg(args, "include_source", true);
    let include_examples = bool_arg(args, "include_examples", false);
    let include_external = bool_arg(args, "include_external", false);

    let model = load_model_with_options(excluded_filename_patterns, with_size_estimates)?;
    let semantic_store = model.semantic_store.as_ref().ok_or_else(|| {
        ReqvireError::ProcessError("Parsed model is missing semantic RDF query state".to_string())
    })?;
    let semantic_index =
        semantic_index_with_external_visibility(&semantic_store.index, include_external)?;
    let external_metadata = semantic_contract::external_materialization_metadata(
        &semantic_store.index,
        &semantic_index,
        include_external,
    );
    let graph_layers = semantic_graph_layers(false, include_external);

    let prefixes = vocabulary_prefixes(&model, &semantic_index);
    let compact_prefixes = compact_vocabulary_prefixes(&prefixes);
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

    let vocabulary = build_vocabulary_sections(
        &model,
        &semantic_index,
        &compact_prefixes,
        include_source,
        include_examples,
    );
    let vocabulary =
        filter_vocabulary_by_ontology_document(vocabulary, ontology_document_filter.as_deref());

    if section == "all" {
        let sections = vocabulary
            .iter()
            .map(|(name, items)| {
                json!({
                    "name": name,
                    "count": items.len(),
                    "cursor": if items.is_empty() { Value::Null } else { json!("0") }
                })
            })
            .collect::<Vec<_>>();
        let summary = vocabulary
            .iter()
            .map(|(name, items)| (name.clone(), json!(items.len())))
            .collect::<serde_json::Map<_, _>>();

        return Ok(json!({
            "section": "all",
            "prefixes": prefixes,
            "sparql_prefix_block": sparql_prefix_block,
            "summary": summary,
            "sections": sections,
            "paging": {
                "limit": limit,
                "next_cursor": Value::Null,
                "has_more": false
            },
            "diagnostics": semantic_store.index.diagnostics,
            "include_external": include_external,
            "ontology_document_filter": ontology_document_filter,
            "external_materialization": external_metadata["external_materialization"].clone(),
            "external_counts": external_metadata["external_counts"].clone(),
            "graph_layers": graph_layers.clone(),
            "model_fingerprint": model_fingerprint(&model)
        }));
    }

    let Some(items) = vocabulary.get(&section) else {
        return Err(ReqvireError::ProcessError(format!(
            "Invalid semantic vocabulary section '{}'",
            section
        )));
    };

    let filtered_items = filter_items(items, filter.as_deref());
    let total = filtered_items.len();
    let page_items = filtered_items
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect::<Vec<_>>();
    let next_offset = offset + page_items.len();
    let has_more = next_offset < total;

    Ok(json!({
        "section": section,
        "items": page_items,
        "prefixes": prefixes,
        "sparql_prefix_block": sparql_prefix_block,
        "paging": {
            "limit": limit,
            "cursor": if offset == 0 { Value::Null } else { json!(offset.to_string()) },
            "next_cursor": if has_more { json!(next_offset.to_string()) } else { Value::Null },
            "has_more": has_more,
            "total": total
        },
        "diagnostics": semantic_store.index.diagnostics,
        "include_external": include_external,
        "ontology_document_filter": ontology_document_filter,
        "external_materialization": external_metadata["external_materialization"].clone(),
        "external_counts": external_metadata["external_counts"].clone(),
        "graph_layers": graph_layers,
        "model_fingerprint": model_fingerprint(&model)
    }))
}

fn compact_vocabulary_prefixes(prefixes: &[Value]) -> Vec<VocabularyPrefix> {
    prefixes
        .iter()
        .filter_map(|entry| {
            Some(VocabularyPrefix {
                prefix: entry.get("prefix")?.as_str()?.to_string(),
                namespace: entry.get("namespace")?.as_str()?.to_string(),
            })
        })
        .collect()
}

fn filter_vocabulary_by_ontology_document(
    sections: BTreeMap<String, Vec<Value>>,
    ontology_document_filter: Option<&str>,
) -> BTreeMap<String, Vec<Value>> {
    let Some(ontology_document_filter) = ontology_document_filter else {
        return sections;
    };

    sections
        .into_iter()
        .map(|(section, items)| {
            let filtered_items = items
                .into_iter()
                .filter(|item| item_ontology_document_matches(item, ontology_document_filter))
                .collect();
            (section, filtered_items)
        })
        .collect()
}

fn item_ontology_document_matches(item: &Value, ontology_document_filter: &str) -> bool {
    item.get("ontology_document")
        .or_else(|| item.get("ontology_document_iri"))
        .and_then(Value::as_str)
        .is_some_and(|ontology_document| ontology_document == ontology_document_filter)
}

fn vocabulary_prefixes(
    model: &ModelManager,
    index: &semantic_contract::SemanticIndex,
) -> Vec<Value> {
    let mut prefixes = Vec::new();
    for declaration in &index.ontology_documents {
        let source = ontology_prefix_source(model, declaration);
        prefixes.push(json!({
            "prefix": declaration.ontology_prefix,
            "namespace": declaration.term_namespace,
            "ontology_base": declaration.ontology_base,
            "term_namespace": declaration.term_namespace,
            "ontology_document_iri": declaration.iri,
            "external": false,
            "source": source
        }));
    }
    prefixes.extend(concept_scheme_vocabulary_prefixes(model));
    for source in &index.external_sources {
        let ontology_document_iri = source.resource.as_deref().unwrap_or(&source.namespace);
        prefixes.push(json!({
            "prefix": source.prefix,
            "namespace": source.namespace,
            "ontology_base": ontology_document_iri,
            "term_namespace": source.namespace,
            "ontology_document_iri": ontology_document_iri,
            "external": true,
            "external_materialization": "used_subset",
            "source_declaration": "declared",
            "source": external_ontology_prefix_source(model, source)
        }));
    }
    prefixes.sort_by(|left, right| {
        left.get("prefix")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .cmp(
                right
                    .get("prefix")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
            )
    });
    prefixes
}

fn used_external_sources<'a>(
    source: &'a semantic_contract::SemanticIndex,
    visible: &semantic_contract::SemanticIndex,
) -> Vec<&'a semantic_contract::ExternalOntologySource> {
    let materialized_terms: BTreeSet<String> = visible
        .external_blocks
        .iter()
        .flat_map(|block| {
            block.quads.iter().filter_map(|quad| match &quad.subject {
                oxigraph::model::NamedOrBlankNode::NamedNode(node) => {
                    Some(node.as_str().to_string())
                }
                oxigraph::model::NamedOrBlankNode::BlankNode(_) => None,
            })
        })
        .collect();

    source
        .external_sources
        .iter()
        .filter(|external_source| {
            materialized_terms
                .iter()
                .any(|term| term.starts_with(&external_source.namespace))
        })
        .collect()
}

fn build_vocabulary_sections(
    model: &ModelManager,
    index: &semantic_contract::SemanticIndex,
    prefixes: &[VocabularyPrefix],
    include_source: bool,
    include_examples: bool,
) -> BTreeMap<String, Vec<Value>> {
    let term_index = collect_term_index(index);
    let ontology_document_by_term = ontology_document_by_term(index);
    let mut sections = BTreeMap::new();

    sections.insert(
        "prefixes".to_string(),
        vocabulary_prefixes(model, index)
            .into_iter()
            .collect::<Vec<_>>(),
    );
    sections.insert(
        "classes".to_string(),
        ontology_terms_section(
            model,
            index,
            &term_index,
            &ontology_document_by_term,
            prefixes,
            include_source,
            |role| role == "class",
        ),
    );
    sections.insert(
        "properties".to_string(),
        ontology_terms_section(
            model,
            index,
            &term_index,
            &ontology_document_by_term,
            prefixes,
            include_source,
            |role| role != "class",
        ),
    );
    sections.insert(
        "relation_families".to_string(),
        relation_families_section(index, &term_index, prefixes, include_source),
    );
    sections.insert(
        "controlled_vocabularies".to_string(),
        controlled_vocabularies_section(index, &term_index, prefixes, include_source),
    );
    sections.insert(
        "concepts".to_string(),
        concepts_section(model, index, &term_index, prefixes, include_source),
    );
    sections.insert(
        "semantic_contracts".to_string(),
        semantic_contracts_section(model, index, prefixes, include_source),
    );
    sections.insert(
        "query_patterns".to_string(),
        query_patterns_section(include_examples),
    );
    sections.insert(
        "source_map".to_string(),
        source_map_section(model, index, prefixes),
    );
    sections.insert(
        "diagnostics".to_string(),
        index
            .diagnostics
            .iter()
            .map(|diagnostic| json!(diagnostic))
            .collect(),
    );

    sections
}

fn ontology_document_by_term(index: &semantic_contract::SemanticIndex) -> BTreeMap<String, String> {
    let mut document_by_element = BTreeMap::new();
    for document in &index.ontology_documents {
        for element_identifier in &document.element_identifiers {
            document_by_element.insert(element_identifier.as_str(), document.iri.as_str());
        }
    }

    let mut document_by_term = BTreeMap::new();
    for declarations in index.ontology_declarations.values() {
        for declaration in declarations {
            if declaration.external {
                if let Some(source) = index
                    .external_sources
                    .iter()
                    .find(|source| declaration.iri.starts_with(&source.namespace))
                {
                    let document = source.resource.as_deref().unwrap_or(&source.namespace);
                    document_by_term.insert(declaration.iri.clone(), document.to_string());
                }
                continue;
            }

            let Some(document_iri) =
                document_by_element.get(declaration.element_identifier.as_str())
            else {
                continue;
            };
            document_by_term.insert(declaration.iri.clone(), (*document_iri).to_string());
        }
    }
    document_by_term
}

#[derive(Clone, Default)]
struct TermInfo {
    label: Option<String>,
    comment: Option<String>,
    types: BTreeSet<String>,
    string_properties: BTreeMap<String, Vec<String>>,
    iri_properties: BTreeMap<String, Vec<String>>,
    source_block: Option<TermSourceRef>,
}

#[derive(Clone, Copy)]
struct TermSourceRef {
    external: bool,
    block_index: usize,
}

fn collect_term_index(index: &semantic_contract::SemanticIndex) -> BTreeMap<String, TermInfo> {
    let mut terms = BTreeMap::new();
    let blocks = index
        .blocks
        .iter()
        .enumerate()
        .map(|(block_index, block)| (false, block_index, block))
        .chain(
            index
                .external_blocks
                .iter()
                .enumerate()
                .map(|(block_index, block)| (true, block_index, block)),
        );
    for (external, block_index, block) in blocks {
        for quad in &block.quads {
            let Some(subject) = subject_iri(&quad.subject) else {
                continue;
            };
            let entry = terms
                .entry(subject.to_string())
                .or_insert_with(TermInfo::default);
            entry.source_block.get_or_insert(TermSourceRef {
                external,
                block_index,
            });
            match quad.predicate.as_str() {
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#type" => {
                    if let Some(iri) = term_iri(&quad.object) {
                        entry.types.insert(iri.to_string());
                    }
                }
                "http://www.w3.org/2000/01/rdf-schema#label" => {
                    if let Some(value) = literal_value(&quad.object) {
                        entry.label = Some(value.to_string());
                    }
                }
                "http://www.w3.org/2000/01/rdf-schema#comment" => {
                    if let Some(value) = literal_value(&quad.object) {
                        entry.comment = Some(value.to_string());
                    }
                }
                predicate => {
                    if let Some(value) = literal_value(&quad.object) {
                        entry
                            .string_properties
                            .entry(predicate.to_string())
                            .or_default()
                            .push(value.to_string());
                    } else if let Some(iri) = term_iri(&quad.object) {
                        entry
                            .iri_properties
                            .entry(predicate.to_string())
                            .or_default()
                            .push(iri.to_string());
                    }
                }
            }
        }
    }
    terms
}

fn ontology_terms_section(
    model: &ModelManager,
    index: &semantic_contract::SemanticIndex,
    term_index: &BTreeMap<String, TermInfo>,
    ontology_document_by_term: &BTreeMap<String, String>,
    prefixes: &[VocabularyPrefix],
    include_source: bool,
    include_role: impl Fn(&str) -> bool,
) -> Vec<Value> {
    let mut items = Vec::new();
    for declarations in index.ontology_declarations.values() {
        for declaration in declarations {
            let role = declaration.role.to_string();
            if !include_role(&role) {
                continue;
            }
            let info = term_index.get(&declaration.iri);
            let mut item = serde_json::Map::new();
            item.insert("iri".to_string(), json!(declaration.iri));
            item.insert(
                "curie".to_string(),
                json!(curie(&declaration.iri, prefixes)),
            );
            item.insert("role".to_string(), json!(role));
            item.insert("external".to_string(), json!(declaration.external));
            if let Some(ontology_document) = ontology_document_by_term.get(&declaration.iri) {
                item.insert("ontology_document".to_string(), json!(ontology_document));
            }
            if declaration.external {
                item.insert("external_materialization".to_string(), json!("used_subset"));
                item.insert(
                    "materialized_in_used_subset".to_string(),
                    json!(declaration.materialized_in_used_subset),
                );
            }
            item.insert(
                "label".to_string(),
                json!(info.and_then(|entry| entry.label.clone())),
            );
            item.insert(
                "comment".to_string(),
                json!(info.and_then(|entry| entry.comment.clone())),
            );
            if let Some(info) = info {
                if let Some(domain) = first_iri_property(
                    info,
                    "http://www.w3.org/2000/01/rdf-schema#domain",
                    prefixes,
                ) {
                    item.insert("domain".to_string(), json!(domain));
                }
                if let Some(range) =
                    first_iri_property(info, "http://www.w3.org/2000/01/rdf-schema#range", prefixes)
                {
                    item.insert("range".to_string(), json!(range));
                }
            }
            if include_source {
                let source = if declaration.external {
                    info.map(|entry| source_for_term(index, entry))
                        .unwrap_or(Value::Null)
                } else {
                    source_for_element_identifier(model, &declaration.element_identifier)
                };
                item.insert("source".to_string(), source);
            }
            items.push(Value::Object(item));
        }
    }
    sort_items(&mut items);
    items
}

fn relation_families_section(
    index: &semantic_contract::SemanticIndex,
    term_index: &BTreeMap<String, TermInfo>,
    prefixes: &[VocabularyPrefix],
    include_source: bool,
) -> Vec<Value> {
    let relation_family_type = "https://www.reqvire.org/ontology#RelationFamily";
    let relation_rule_type = "https://www.reqvire.org/ontology#RelationRule";
    let mut rule_items_by_family: BTreeMap<String, Vec<Value>> = BTreeMap::new();

    for (iri, info) in term_index {
        if !info.types.contains(relation_rule_type) {
            continue;
        }
        let Some(family) = first_iri(info, "https://www.reqvire.org/ontology#relationFamily")
        else {
            continue;
        };
        let rule_item = json!({
            "name": first_string(info, "https://www.reqvire.org/ontology#relationName"),
            "direction": first_string(info, "https://www.reqvire.org/ontology#relationDirection"),
            "allowed_source_type": strings(info, "https://www.reqvire.org/ontology#allowedSourceType"),
            "allowed_target_type": strings(info, "https://www.reqvire.org/ontology#allowedTargetType"),
            "iri": iri,
            "curie": curie(iri, prefixes),
            "external": term_info_external(info)
        });
        rule_items_by_family
            .entry(family.to_string())
            .or_default()
            .push(rule_item);
    }

    let mut items = Vec::new();
    for (iri, info) in term_index {
        if !info.types.contains(relation_family_type) {
            continue;
        }
        let mut raw_relations = rule_items_by_family.remove(iri).unwrap_or_default();
        sort_items(&mut raw_relations);
        let mut item = serde_json::Map::new();
        item.insert(
            "name".to_string(),
            json!(first_string(
                info,
                "https://www.reqvire.org/ontology#relationFamilyName"
            )),
        );
        item.insert("iri".to_string(), json!(iri));
        item.insert("curie".to_string(), json!(curie(iri, prefixes)));
        item.insert("external".to_string(), json!(term_info_external(info)));
        item.insert(
            "meaning".to_string(),
            json!(first_string(
                info,
                "https://www.reqvire.org/ontology#relationFamilyMeaning"
            )),
        );
        item.insert(
            "forward_property".to_string(),
            json!(first_iri_property(
                info,
                "https://www.reqvire.org/ontology#relationFamilyForwardProperty",
                prefixes
            )),
        );
        item.insert(
            "inverse_property".to_string(),
            json!(first_iri_property(
                info,
                "https://www.reqvire.org/ontology#relationFamilyInverseProperty",
                prefixes
            )),
        );
        item.insert("raw_relations".to_string(), json!(raw_relations));
        item.insert(
            "transitive".to_string(),
            json!(
                first_string(info, "https://www.reqvire.org/ontology#relationFamilyName")
                    .as_deref()
                    == Some("hierarchy")
            ),
        );
        if include_source {
            item.insert("source".to_string(), source_for_term(index, info));
        }
        items.push(Value::Object(item));
    }
    sort_items(&mut items);
    items
}

fn concepts_section(
    model: &ModelManager,
    index: &semantic_contract::SemanticIndex,
    term_index: &BTreeMap<String, TermInfo>,
    prefixes: &[VocabularyPrefix],
    include_source: bool,
) -> Vec<Value> {
    const SKOS_CONCEPT: &str = "http://www.w3.org/2004/02/skos/core#Concept";
    const SKOS_CONCEPT_SCHEME: &str = "http://www.w3.org/2004/02/skos/core#ConceptScheme";
    const SKOS_PREF_LABEL: &str = "http://www.w3.org/2004/02/skos/core#prefLabel";
    const SKOS_ALT_LABEL: &str = "http://www.w3.org/2004/02/skos/core#altLabel";
    const SKOS_HIDDEN_LABEL: &str = "http://www.w3.org/2004/02/skos/core#hiddenLabel";
    const SKOS_DEFINITION: &str = "http://www.w3.org/2004/02/skos/core#definition";
    const SKOS_SCOPE_NOTE: &str = "http://www.w3.org/2004/02/skos/core#scopeNote";
    const SKOS_EXAMPLE: &str = "http://www.w3.org/2004/02/skos/core#example";
    const SKOS_IN_SCHEME: &str = "http://www.w3.org/2004/02/skos/core#inScheme";
    const SKOS_HAS_TOP_CONCEPT: &str = "http://www.w3.org/2004/02/skos/core#hasTopConcept";
    const SKOS_BROADER: &str = "http://www.w3.org/2004/02/skos/core#broader";
    const SKOS_NARROWER: &str = "http://www.w3.org/2004/02/skos/core#narrower";
    const SKOS_RELATED: &str = "http://www.w3.org/2004/02/skos/core#related";
    const SKOS_EXACT_MATCH: &str = "http://www.w3.org/2004/02/skos/core#exactMatch";
    const SKOS_CLOSE_MATCH: &str = "http://www.w3.org/2004/02/skos/core#closeMatch";

    let mut items = Vec::new();
    for (iri, info) in term_index {
        let is_scheme = info.types.contains(SKOS_CONCEPT_SCHEME);
        let is_concept = info.types.contains(SKOS_CONCEPT);
        if !is_scheme && !is_concept {
            continue;
        }
        let generated_from_markdown =
            term_has_type_in_concept_block(index, iri, SKOS_CONCEPT_SCHEME)
                || term_has_type_in_concept_block(index, iri, SKOS_CONCEPT);
        if !generated_from_markdown {
            continue;
        }

        let mut item = serde_json::Map::new();
        item.insert("iri".to_string(), json!(iri));
        item.insert("curie".to_string(), json!(curie(iri, prefixes)));
        item.insert(
            "kind".to_string(),
            json!(if is_scheme {
                "concept-scheme"
            } else {
                "concept"
            }),
        );
        item.insert("external".to_string(), json!(term_info_external(info)));
        item.insert(
            "generated_from_markdown".to_string(),
            json!(generated_from_markdown),
        );
        if let Some(ontology_document) = ontology_document_for_iri(index, iri) {
            item.insert("ontology_document".to_string(), json!(ontology_document));
        }
        item.insert(
            "pref_label".to_string(),
            json!(first_string(info, SKOS_PREF_LABEL)),
        );
        item.insert(
            "definition".to_string(),
            json!(first_string(info, SKOS_DEFINITION)),
        );
        item.insert(
            "alt_labels".to_string(),
            json!(strings(info, SKOS_ALT_LABEL)),
        );
        item.insert(
            "hidden_labels".to_string(),
            json!(strings(info, SKOS_HIDDEN_LABEL)),
        );
        item.insert(
            "scope_notes".to_string(),
            json!(strings(info, SKOS_SCOPE_NOTE)),
        );
        item.insert("examples".to_string(), json!(strings(info, SKOS_EXAMPLE)));
        item.insert(
            "in_scheme".to_string(),
            json!(iris_as_curies(info, SKOS_IN_SCHEME, prefixes)),
        );
        item.insert(
            "top_concepts".to_string(),
            json!(iris_as_curies(info, SKOS_HAS_TOP_CONCEPT, prefixes)),
        );
        item.insert(
            "broader".to_string(),
            json!(iris_as_curies(info, SKOS_BROADER, prefixes)),
        );
        item.insert(
            "narrower".to_string(),
            json!(iris_as_curies(info, SKOS_NARROWER, prefixes)),
        );
        item.insert(
            "related".to_string(),
            json!(iris_as_curies(info, SKOS_RELATED, prefixes)),
        );
        item.insert(
            "exact_match".to_string(),
            json!(iris_as_curies(info, SKOS_EXACT_MATCH, prefixes)),
        );
        item.insert(
            "close_match".to_string(),
            json!(iris_as_curies(info, SKOS_CLOSE_MATCH, prefixes)),
        );
        if include_source {
            let source = source_for_term(index, info);
            if let Some(source_identifier) =
                source.get("element_identifier").and_then(Value::as_str)
            {
                insert_native_concept_source_fields(
                    &mut item,
                    model,
                    source_identifier,
                    iri,
                    is_scheme,
                );
            }
            if let Some(source_element) = source
                .get("element_identifier")
                .and_then(Value::as_str)
                .map(|identifier| source_for_element_identifier(model, identifier))
                .filter(|source| !source.is_null())
            {
                item.insert("source_element".to_string(), source_element);
            }
            item.insert("source".to_string(), source);
        }
        items.push(Value::Object(item));
    }
    sort_items(&mut items);
    items
}

fn insert_native_concept_source_fields(
    item: &mut serde_json::Map<String, Value>,
    model: &ModelManager,
    source_identifier: &str,
    iri: &str,
    is_scheme: bool,
) {
    let Some(element) = model.graph_registry.get_element(source_identifier) else {
        return;
    };

    item.insert(
        "source_element_identifier".to_string(),
        json!(element.identifier),
    );
    item.insert("source_element_name".to_string(), json!(element.name));
    item.insert(
        "source_element_type".to_string(),
        json!(element.element_type.as_str()),
    );

    if is_scheme {
        item.insert("scheme_iri".to_string(), json!(iri));
        item.insert(
            "scheme_element_identifier".to_string(),
            json!(element.identifier),
        );
        item.insert("scheme_element_name".to_string(), json!(element.name));
        if let Some(payload) = &element.concept_scheme {
            insert_namespace_fields(
                item,
                payload.namespace_base.as_deref(),
                payload.namespace_prefix.as_deref(),
            );
        }
        return;
    }

    item.insert("concept_iri".to_string(), json!(iri));
    if let Some(payload) = &element.concept {
        if let Some(scheme_iri) = &payload.scheme_iri {
            item.insert("scheme_iri".to_string(), json!(scheme_iri));
            if let Some(scheme_element) = concept_scheme_element_by_iri(model, scheme_iri) {
                item.insert(
                    "scheme_element_identifier".to_string(),
                    json!(scheme_element.identifier),
                );
                item.insert(
                    "scheme_element_name".to_string(),
                    json!(scheme_element.name),
                );
                if let Some(scheme_payload) = &scheme_element.concept_scheme {
                    insert_namespace_fields(
                        item,
                        scheme_payload.namespace_base.as_deref(),
                        scheme_payload.namespace_prefix.as_deref(),
                    );
                    return;
                }
            }
        }
        insert_namespace_fields(
            item,
            payload.namespace_base.as_deref(),
            payload.namespace_prefix.as_deref(),
        );
    }
}

fn concept_scheme_element_by_iri<'a>(
    model: &'a ModelManager,
    scheme_iri: &str,
) -> Option<&'a crate::element::Element> {
    model
        .graph_registry
        .get_all_elements()
        .into_iter()
        .find(|element| {
            element
                .concept_scheme
                .as_ref()
                .is_some_and(|payload| payload.iri == scheme_iri)
        })
}

fn insert_namespace_fields(
    item: &mut serde_json::Map<String, Value>,
    namespace_base: Option<&str>,
    namespace_prefix: Option<&str>,
) {
    if let Some(base) = namespace_base {
        item.insert("namespace_base".to_string(), json!(base));
        item.insert(
            "namespace_iri".to_string(),
            json!(format!("{}#", base.trim_end_matches('#'))),
        );
    }
    if let Some(prefix) = namespace_prefix {
        item.insert("namespace_prefix".to_string(), json!(prefix));
    }
}

fn controlled_vocabularies_section(
    index: &semantic_contract::SemanticIndex,
    term_index: &BTreeMap<String, TermInfo>,
    prefixes: &[VocabularyPrefix],
    include_source: bool,
) -> Vec<Value> {
    let named_individual = "http://www.w3.org/2002/07/owl#NamedIndividual";
    let excluded_types = BTreeSet::from([
        "https://www.reqvire.org/ontology#RelationFamily",
        "https://www.reqvire.org/ontology#RelationRule",
    ]);
    let mut items = Vec::new();
    for (iri, info) in term_index {
        if !info.types.contains(named_individual) {
            continue;
        }
        let semantic_types: Vec<String> = info
            .types
            .iter()
            .filter(|kind| {
                kind.as_str() != named_individual && !excluded_types.contains(kind.as_str())
            })
            .map(|kind| curie(kind, prefixes))
            .collect();
        if semantic_types.is_empty() {
            continue;
        }
        let mut item = serde_json::Map::new();
        item.insert("iri".to_string(), json!(iri));
        item.insert("curie".to_string(), json!(curie(iri, prefixes)));
        item.insert("external".to_string(), json!(term_info_external(info)));
        item.insert("types".to_string(), json!(semantic_types));
        item.insert("label".to_string(), json!(info.label));
        item.insert("comment".to_string(), json!(info.comment));
        if include_source {
            item.insert("source".to_string(), source_for_term(index, info));
        }
        items.push(Value::Object(item));
    }
    sort_items(&mut items);
    items
}

fn semantic_contracts_section(
    model: &ModelManager,
    index: &semantic_contract::SemanticIndex,
    prefixes: &[VocabularyPrefix],
    include_source: bool,
) -> Vec<Value> {
    index
        .blocks
        .iter()
        .filter(|block| matches!(block.kind, semantic_contract::SemanticBlockKind::Shapes))
        .map(|block| {
            let shape_references = index
                .shape_references
                .iter()
                .filter(|reference| reference.element_identifier == block.source)
                .map(|reference| {
                    json!({
                        "iri": reference.iri,
                        "curie": curie(&reference.iri, prefixes),
                        "kind": reference.kind
                    })
                })
                .collect::<Vec<_>>();
            let mut item = serde_json::Map::new();
            item.insert("element_identifier".to_string(), json!(block.source));
            item.insert("element_name".to_string(), json!(block.source_name));
            item.insert("file_path".to_string(), json!(block.file_path));
            item.insert("line_number".to_string(), json!(block.line_number));
            item.insert("shape_references".to_string(), json!(shape_references));
            if include_source {
                item.insert(
                    "source".to_string(),
                    source_for_element_identifier(model, &block.source),
                );
            }
            Value::Object(item)
        })
        .collect()
}

fn source_map_section(
    model: &ModelManager,
    index: &semantic_contract::SemanticIndex,
    prefixes: &[VocabularyPrefix],
) -> Vec<Value> {
    let term_index = collect_term_index(index);
    index
        .ontology_declarations
        .values()
        .flat_map(|declarations| declarations.iter())
        .map(|declaration| {
            let source = if declaration.external {
                term_index
                    .get(&declaration.iri)
                    .map(|info| source_for_term(index, info))
                    .unwrap_or(Value::Null)
            } else {
                source_for_element_identifier(model, &declaration.element_identifier)
            };
            json!({
                "term": curie(&declaration.iri, prefixes),
                "iri": declaration.iri,
                "role": declaration.role.to_string(),
                "external": declaration.external,
                "external_materialization": if declaration.external { Value::String("used_subset".to_string()) } else { Value::Null },
                "materialized_in_used_subset": declaration.external && declaration.materialized_in_used_subset,
                "source": source
            })
        })
        .collect()
}

fn query_patterns_section(include_examples: bool) -> Vec<Value> {
    let mut patterns = vec![
        json!({
            "id": "discover_relation_families",
            "title": "Discover relation families",
            "preferred_classes": ["reqvire:RelationFamily"],
            "preferred_properties": ["reqvire:relationFamilyName", "reqvire:relationFamilyForwardProperty", "reqvire:relationFamilyInverseProperty"]
        }),
        json!({
            "id": "verified_requirements",
            "title": "Requirements verified by verification elements",
            "preferred_property": "reqvire:requirementVerifiedByVerification"
        }),
        json!({
            "id": "cross_subgraph_contract_context",
            "title": "Requirements using Reused Contract Context",
            "preferred_property": "reqvire:requirementUsesCrossSubgraphContract"
        }),
    ];

    if include_examples {
        if let Some(Value::Object(pattern)) = patterns.get_mut(0) {
            pattern.insert(
                "sparql".to_string(),
                json!("SELECT ?family ?name ?forward ?inverse WHERE { ?family a reqvire:RelationFamily ; reqvire:relationFamilyName ?name . OPTIONAL { ?family reqvire:relationFamilyForwardProperty ?forward } OPTIONAL { ?family reqvire:relationFamilyInverseProperty ?inverse } } ORDER BY ?name"),
            );
        }
        if let Some(Value::Object(pattern)) = patterns.get_mut(1) {
            pattern.insert(
                "sparql".to_string(),
                json!("SELECT ?requirement ?verification WHERE { ?requirement a reqvire:Requirement ; reqvire:requirementVerifiedByVerification ?verification . } ORDER BY ?requirement ?verification"),
            );
        }
        if let Some(Value::Object(pattern)) = patterns.get_mut(2) {
            pattern.insert(
                "sparql".to_string(),
                json!("SELECT ?requirement ?contract WHERE { ?requirement a reqvire:Requirement ; reqvire:requirementUsesCrossSubgraphContract ?contract . } ORDER BY ?requirement ?contract"),
            );
        }
    }

    patterns
}

fn filter_items(items: &[Value], filter: Option<&str>) -> Vec<Value> {
    let Some(filter) = filter else {
        return items.to_vec();
    };
    items
        .iter()
        .filter(|item| item.to_string().to_lowercase().contains(filter))
        .cloned()
        .collect()
}

fn source_for_element_identifier(model: &ModelManager, identifier: &str) -> Value {
    match model.graph_registry.get_element(identifier) {
        Some(element) => json!({
            "element_identifier": element.identifier,
            "element_name": element.name,
            "element_type": element.element_type.as_str(),
            "file_path": element.file_path,
            "line_number": element.line_number,
            "content": semantic_prefix_source_content(&element.content)
        }),
        None => Value::Null,
    }
}

fn source_for_term(index: &semantic_contract::SemanticIndex, info: &TermInfo) -> Value {
    info.source_block
        .and_then(|source| {
            if source.external {
                index.external_blocks.get(source.block_index)
            } else {
                index.blocks.get(source.block_index)
            }
        })
        .map(|block| {
            let external = matches!(
                block.kind,
                semantic_contract::SemanticBlockKind::ExternalOntology
            );
            let external_materialization = block
                .external_materialization
                .as_ref()
                .map(|materialization| Value::String(materialization.clone()))
                .unwrap_or(Value::Null);
            json!({
                "element_identifier": block.source,
                "element_name": block.source_name,
                "file_path": block.file_path,
                "line_number": block.line_number,
                "external": external,
                "external_materialization": external_materialization
            })
        })
        .unwrap_or(Value::Null)
}

fn term_info_external(info: &TermInfo) -> bool {
    matches!(info.source_block, Some(source) if source.external)
}

fn term_has_type_in_concept_block(
    index: &semantic_contract::SemanticIndex,
    iri: &str,
    type_iri: &str,
) -> bool {
    index
        .blocks
        .iter()
        .filter(|block| matches!(block.kind, semantic_contract::SemanticBlockKind::Concepts))
        .flat_map(|block| block.quads.iter())
        .any(|quad| {
            quad.predicate.as_str() == "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"
                && subject_iri(&quad.subject) == Some(iri)
                && term_iri(&quad.object) == Some(type_iri)
        })
}

fn ontology_document_for_iri<'a>(
    index: &'a semantic_contract::SemanticIndex,
    iri: &str,
) -> Option<&'a str> {
    index
        .ontology_documents
        .iter()
        .find(|document| iri.starts_with(&document.term_namespace))
        .map(|document| document.iri.as_str())
}

fn first_string(info: &TermInfo, predicate: &str) -> Option<String> {
    info.string_properties
        .get(predicate)
        .and_then(|values| values.first())
        .cloned()
}

fn strings(info: &TermInfo, predicate: &str) -> Vec<String> {
    info.string_properties
        .get(predicate)
        .cloned()
        .unwrap_or_default()
}

fn first_iri<'a>(info: &'a TermInfo, predicate: &str) -> Option<&'a str> {
    info.iri_properties
        .get(predicate)
        .and_then(|values| values.first())
        .map(String::as_str)
}

fn first_iri_property(
    info: &TermInfo,
    predicate: &str,
    prefixes: &[VocabularyPrefix],
) -> Option<String> {
    first_iri(info, predicate).map(|iri| curie(iri, prefixes))
}

fn iris_as_curies(info: &TermInfo, predicate: &str, prefixes: &[VocabularyPrefix]) -> Vec<String> {
    info.iri_properties
        .get(predicate)
        .map(|values| values.iter().map(|iri| curie(iri, prefixes)).collect())
        .unwrap_or_default()
}

fn sort_items(items: &mut [Value]) {
    items.sort_by(|left, right| left.to_string().cmp(&right.to_string()));
}

fn literal_value(term: &Term) -> Option<&str> {
    match term {
        Term::Literal(literal) => Some(literal.value()),
        _ => None,
    }
}

fn semantic_graph_layers(full: bool, include_external: bool) -> Vec<Value> {
    let layers = vec![
        graph_layer(
            "default",
            "urn:reqvire:semantic-graph:default",
            true,
            "default graph compatibility projection",
        ),
        graph_layer(
            "authored-ontology",
            semantic_store::GRAPH_AUTHORED_ONTOLOGY,
            true,
            "reqvire-authored ontology and concept graph",
        ),
        graph_layer(
            "authored-model",
            semantic_store::GRAPH_AUTHORED_MODEL,
            full,
            "reqvire model context graph",
        ),
        graph_layer(
            "generated",
            semantic_store::GRAPH_GENERATED,
            full,
            "reqvire generated ontology and model helper graph",
        ),
        graph_layer(
            "external-used-subset",
            semantic_store::GRAPH_EXTERNAL_USED_SUBSET,
            include_external,
            "used external subset produced by o-kernel",
        ),
        graph_layer(
            "raw-external-source",
            "urn:reqvire:semantic-graph:raw-external-source",
            false,
            "raw external dependency graph (internal only)",
        ),
    ];
    layers
}

fn graph_layer(role: &str, graph_iri: &str, included: bool, source: &str) -> Value {
    json!({
        "role": role,
        "graph_iri": graph_iri,
        "included": included,
        "source": source
    })
}

fn curie(iri: &str, prefixes: &[VocabularyPrefix]) -> String {
    const BUILTIN_PREFIXES: &[(&str, &str)] = &[
        ("reqvire", "https://www.reqvire.org/ontology#"),
        ("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
        ("rdfs", "http://www.w3.org/2000/01/rdf-schema#"),
        ("owl", "http://www.w3.org/2002/07/owl#"),
        ("sh", "http://www.w3.org/ns/shacl#"),
        ("skos", "http://www.w3.org/2004/02/skos/core#"),
    ];
    let mut best: Option<&VocabularyPrefix> = None;
    for prefix in prefixes {
        if iri.starts_with(&prefix.namespace)
            && best
                .as_ref()
                .is_none_or(|current| prefix.namespace.len() > current.namespace.len())
        {
            best = Some(prefix);
        }
    }
    let mut builtin_best: Option<(&str, &str)> = None;
    for (prefix, namespace) in BUILTIN_PREFIXES {
        if iri.starts_with(namespace)
            && builtin_best
                .as_ref()
                .is_none_or(|(_, current)| namespace.len() > current.len())
        {
            builtin_best = Some((*prefix, *namespace));
        }
    }

    match (best, builtin_best) {
        (Some(prefix), Some((builtin_prefix, builtin_namespace)))
            if builtin_namespace.len() > prefix.namespace.len() =>
        {
            format!("{}:{}", builtin_prefix, &iri[builtin_namespace.len()..])
        }
        (Some(prefix), _) => format!("{}:{}", prefix.prefix, &iri[prefix.namespace.len()..]),
        (None, Some((builtin_prefix, builtin_namespace))) => {
            format!("{}:{}", builtin_prefix, &iri[builtin_namespace.len()..])
        }
        (None, None) => iri.to_string(),
    }
}

fn sparql_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let query = required_string_arg(args, "query")?;
    let full = bool_arg(args, "full", true);
    let include_external = bool_arg(args, "include_external", false);
    let model = load_model_with_options(excluded_filename_patterns, with_size_estimates)?;
    let semantic_store = model.semantic_store.as_ref().ok_or_else(|| {
        ReqvireError::ProcessError("Parsed model is missing semantic RDF query state".to_string())
    })?;
    let visible_index =
        semantic_index_with_external_visibility(&semantic_store.index, include_external)?;
    let external_metadata = semantic_contract::external_materialization_metadata(
        &semantic_store.index,
        &visible_index,
        include_external,
    );
    let graph_layers = semantic_graph_layers(full, include_external);

    let results = SparqlEvaluator::new()
        .parse_query(&query)
        .map_err(|error| ReqvireError::ProcessError(format!("Invalid SPARQL query: {}", error)))?
        .on_store(semantic_store.store(full, include_external))
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
        object.insert("include_external".to_string(), json!(include_external));
        object.insert(
            "external_materialization".to_string(),
            external_metadata["external_materialization"].clone(),
        );
        object.insert(
            "external_counts".to_string(),
            external_metadata["external_counts"].clone(),
        );
        object.insert("summary".to_string(), json!(semantic_store.index.summary));
        object.insert(
            "diagnostics".to_string(),
            json!(semantic_store.index.diagnostics),
        );
        object.insert(
            "model_fingerprint".to_string(),
            json!(model_fingerprint(&model)),
        );
        object.insert("graph_layers".to_string(), json!(graph_layers));
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
    let result = if relation_type == "reusesContract" {
        if crate::utils::is_external_url(&target) {
            return Err(ReqvireError::ProcessError(
                "External URLs cannot be reused as contract context. Use a semantically specific relation only when the URL is valid evidence for that relation."
                    .to_string(),
            ));
        }
        crud::reuse_contract_element_identifier(
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

fn usize_arg(args: &Value, name: &str, default: usize) -> usize {
    args.get(name)
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
        .unwrap_or(default)
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
        "reqvire.semantic.shapes",
        "reqvire.semantic.concepts",
        "reqvire.concepts.list",
        "reqvire.concepts.get",
        "reqvire.concept_schemes.list",
        "reqvire.concept_mappings.list",
        "reqvire.semantic.graph",
        "reqvire.semantic.prefixes",
        "reqvire.semantic.vocabulary",
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
        for reused_contract_context in &element.reused_contract_context {
            reused_contract_context.target.as_str().hash(&mut hasher);
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
