use super::*;

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
                ("has_contract_bindings", json!({ "type": "boolean" })),
                (
                    "filter_contract_bindings",
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
            "reqvire.semantic.export",
            "Export selected semantic RDF layers with the same layer contract as the CLI semantic export command.",
            object_schema(vec![
                (
                    "format",
                    json!({ "type": "string", "enum": ["turtle", "jsonld"], "default": "turtle" }),
                ),
                (
                    "layers",
                    json!({
                        "type": "array",
                        "items": { "type": "string", "enum": ["ontologies", "shapes", "concepts", "model", "external-used", "prefixes"] },
                        "description": "Semantic export layers to include. Omit or pass an empty array to export all public layers."
                    }),
                ),
                (
                    "namespace_base",
                    json!({
                        "type": "string",
                        "description": "Filter clean authored exports to one ontology base or term namespace. Cannot be combined with the model layer."
                    }),
                ),
            ]),
        ),
        read_tool(
            "reqvire.semantic.ontologies",
            "Collect authored OWL/RDF ontology vocabulary only.",
            object_schema(vec![(
                "format",
                json!({ "type": "string", "enum": ["turtle", "jsonld"], "default": "turtle" }),
            )]),
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
            object_schema(vec![(
                "format",
                json!({ "type": "string", "enum": ["turtle", "jsonld"], "default": "turtle" }),
            )]),
        ),
        read_tool(
            "reqvire.semantic.model",
            "Collect generated Reqvire model RDF facts for elements, relations, concept references, semantic term context, and ontology projection facts.",
            object_schema(vec![(
                "format",
                json!({ "type": "string", "enum": ["turtle", "jsonld"], "default": "turtle" }),
            )]),
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
            "Collect the combined public semantic export graph. Equivalent to reqvire.semantic.export with omitted layers.",
            object_schema(vec![(
                "format",
                json!({ "type": "string", "enum": ["turtle", "jsonld"], "default": "turtle" }),
            )]),
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
                "reqvire.move_folder",
                "Move or rename a folder subtree and update model references.",
                required_object_schema(
                    vec![
                        ("source_folder", json!({ "type": "string" })),
                        ("target_folder", json!({ "type": "string" })),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["source_folder", "target_folder"],
                ),
            ),
            mutation_tool(
                "reqvire.link",
                "Add a relation or contract_bindings.",
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
                "Remove a relation or contract_bindings.",
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

pub(crate) fn tool_exists(name: &str, enable_mutations: bool) -> bool {
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
        "reqvire.semantic.export",
        "reqvire.semantic.ontologies",
        "reqvire.semantic.shapes",
        "reqvire.semantic.concepts",
        "reqvire.semantic.model",
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

pub(crate) fn mutation_tool_names() -> Vec<&'static str> {
    vec![
        "reqvire.add_element",
        "reqvire.remove_element",
        "reqvire.move_element",
        "reqvire.rename_element",
        "reqvire.merge_elements",
        "reqvire.move_file",
        "reqvire.move_folder",
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

pub(crate) fn resource_contents(uri: &str, value: Value) -> Value {
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
