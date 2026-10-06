use super::*;
use std::sync::LazyLock;

struct ToolCatalog {
    definitions: Vec<Value>,
    by_name: BTreeMap<String, usize>,
}

impl ToolCatalog {
    fn new(enable_mutations: bool) -> Self {
        let definitions = build_tool_definitions(enable_mutations);
        let by_name = definitions
            .iter()
            .enumerate()
            .map(|(index, tool)| {
                (
                    tool["name"]
                        .as_str()
                        .expect("tool definition name")
                        .to_owned(),
                    index,
                )
            })
            .collect();
        Self {
            definitions,
            by_name,
        }
    }

    fn get(&self, name: &str) -> Option<&Value> {
        self.by_name
            .get(name)
            .map(|index| &self.definitions[*index])
    }
}

fn catalog(enable_mutations: bool) -> &'static ToolCatalog {
    static READ_ONLY: LazyLock<ToolCatalog> = LazyLock::new(|| ToolCatalog::new(false));
    static MUTATIONS: LazyLock<ToolCatalog> = LazyLock::new(|| ToolCatalog::new(true));
    if enable_mutations {
        &MUTATIONS
    } else {
        &READ_ONLY
    }
}

pub fn tool_definitions(enable_mutations: bool) -> Vec<Value> {
    catalog(enable_mutations).definitions.clone()
}

fn element_reference_schema(domain: &str) -> Value {
    json!({
        "type": "string",
        "description": format!("Exact element name or canonical element identifier in the selected model context. {domain}")
    })
}

fn build_tool_definitions(enable_mutations: bool) -> Vec<Value> {
    #[cfg(test)]
    tests::CATALOG_BUILDS.with(|count| count.set(count.get() + 1));
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
            "Report the current workspace and canonical v2 SHA-256 model revision (64 lowercase hexadecimal characters).",
            object_schema(vec![]),
        ),
        read_tool(
            "reqvire.read_element",
            "Read one authoritative model element by identifier or name.",
            object_schema(vec![
                ("identifier", json!({ "type": "string", "description": "Canonical element identifier only; multiple explicit selectors must agree." })),
                ("name", element_reference_schema("Multiple explicit selectors must select the same element.")),
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
                ("has_contract_references", json!({ "type": "boolean" })),
                ("filter_contract_references", json!({ "type": "string", "description": "Glob matching normalized Contract Reference target identifiers." })),
            ]),
        ),
        read_tool(
            "reqvire.model",
            "Generate model-centric structure.",
            object_schema(vec![
                ("from", element_reference_schema("Omit for the whole model; operation-specific root type checks apply.")),
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
            "Collect capability, requirement, ontology, semantic-query, concept-scheme, or concept context upstream or downstream.",
            required_object_schema(
                vec![
                    ("element_name", element_reference_schema("")),
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
            object_schema(vec![("from", element_reference_schema("Omit for the whole model; operation-specific root type checks apply."))]),
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
                        "items": { "type": "string", "enum": ["ontologies", "shapes", "concepts", "model", "external-used", "prefixes", "queries"] },
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
                ("identifier", json!({ "type": "string", "description": "Canonical element identifier only; multiple explicit selectors must agree." })),
                ("name", element_reference_schema("Multiple explicit selectors must select the same element.")),
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
            "reqvire.semantic.queries",
            "Discover and validate native SPARQL artifacts without execution.",
            object_schema(vec![
                ("name", element_reference_schema("Select a semantic-query source element; mutually exclusive with iri.")),
                ("iri", json!({"type":"string"})),
                ("namespace_base", json!({"type":"string"})),
                ("include_content", json!({"type":"boolean", "default":false})),
            ]),
        ),
        read_tool(
            "reqvire.semantic.queries.validate",
            "Discover and validate native SPARQL artifacts without execution.",
            object_schema(vec![
                ("name", element_reference_schema("Select a semantic-query source element; mutually exclusive with iri.")),
                ("iri", json!({"type":"string"})),
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
            object_schema(vec![("from", element_reference_schema("Select a capability subtree; omitted selects the whole model."))]),
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
                        ("element_name", element_reference_schema("")),
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
                        ("element_name", element_reference_schema("")),
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
                        ("element_name", element_reference_schema("")),
                        ("new_name", json!({ "type": "string", "description": "Literal new authored element name." })),
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
                        ("target", element_reference_schema("Select the target receiving merged content; merge type compatibility applies.")),
                        (
                            "sources",
                            json!({ "type": "array", "items": element_reference_schema("Select a source element to merge.") }),
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
                "Add a relation, bindContract implementation obligation, or referenceContract content dependency. A requirement cannot combine Contract Bindings and Contract References.",
                required_object_schema(
                    vec![
                        ("source", element_reference_schema("")),
                        ("relation_type", json!({ "type": "string" })),
                        ("target", element_reference_schema("Relation endpoints also accept existing file paths and URLs according to relation rules; contracts retain their type checks.")),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["source", "relation_type", "target"],
                ),
            ),
            mutation_tool(
                "reqvire.unlink",
                "Remove a relation, contract binding, or contract reference by target.",
                required_object_schema(
                    vec![
                        ("source", element_reference_schema("")),
                        ("target", element_reference_schema("Relation endpoints also accept existing file paths and URLs according to relation rules; contracts retain their type checks.")),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["source", "target"],
                ),
            ),
            mutation_tool(
                "reqvire.relink",
                "Replace an existing relation or referenceContract target atomically.",
                required_object_schema(
                    vec![
                        ("source", element_reference_schema("")),
                        ("relation_type", json!({ "type": "string" })),
                        ("from_target", element_reference_schema("Existing relation target; file paths and URLs retain their domains.")),
                        ("to_target", element_reference_schema("Replacement relation target; file paths and URLs retain their domains.")),
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
    let tool = catalog(enable_mutations)
        .get(tool_name)
        .ok_or_else(|| format!("Unknown tool '{}'", tool_name))?;
    let schema = tool
        .get("inputSchema")
        .ok_or_else(|| format!("Tool '{}' has no inputSchema", tool_name))?;
    validate_object_schema(arguments, schema)
}

pub fn tool_exists(name: &str, enable_mutations: bool) -> bool {
    catalog(enable_mutations).get(name).is_some()
}

pub const fn mutation_tool_names() -> &'static [&'static str] {
    &[
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
        "outputSchema": output_schema(name),
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

fn output_schema(name: &str) -> Value {
    let mut schema = json!({
        "type": "object",
        "additionalProperties": true
    });
    let fingerprint = json!({
        "type": "string",
        "pattern": "^[0-9a-f]{64}$",
        "description": "SHA-256 of canonical parsed-element model encoding reqvire.model-revision.v2. Excludes Git state and external file contents."
    });
    match name {
        "reqvire.workspace_status" => {
            schema["properties"] = json!({
                "model": {"type": "object", "properties": {"fingerprint": fingerprint}, "additionalProperties": true}
            })
        }
        "reqvire.model_revision"
        | "reqvire.semantic.prefixes"
        | "reqvire.semantic.vocabulary"
        | "reqvire.semantic.sparql" => {
            schema["properties"] = json!({"model_fingerprint": fingerprint});
        }
        _ => {}
    }
    schema
}

pub fn resource_contents(uri: &str, value: Value) -> Value {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    thread_local! {
        pub(super) static CATALOG_BUILDS: Cell<usize> = const { Cell::new(0) };
    }

    #[test]
    fn catalog_validation_reuses_definitions_after_warmup() {
        let args = json!({"short":true});
        for enabled in [false, true] {
            validate_tool_arguments("reqvire.search", &args, enabled)
                .expect("test fixture operation should succeed");
            let before = CATALOG_BUILDS.with(Cell::get);
            for _ in 0..20 {
                validate_tool_arguments("reqvire.search", &args, enabled)
                    .expect("test fixture operation should succeed");
                assert!(validate_tool_arguments("reqvire.not_a_tool", &args, enabled).is_err());
            }
            assert_eq!(
                CATALOG_BUILDS.with(Cell::get),
                before,
                "validation must not rebuild catalogs"
            );
        }
    }

    #[test]
    fn cached_catalogs_preserve_mode_schemas_and_diagnostics() {
        for enabled in [false, true] {
            let definitions = tool_definitions(enabled);
            assert_eq!(
                definitions
                    .iter()
                    .any(|tool| tool["name"] == "reqvire.add_element"),
                enabled
            );
            for (name, args, diagnostic) in [
                (
                    "reqvire.not_a_tool",
                    json!({}),
                    "Unknown tool 'reqvire.not_a_tool'",
                ),
                (
                    "reqvire.search",
                    json!([]),
                    "Tool arguments must be a JSON object",
                ),
                (
                    "reqvire.search",
                    json!({"alien":true}),
                    "Unknown argument 'alien'",
                ),
                (
                    "reqvire.collect",
                    json!({}),
                    "Missing required argument 'element_name'",
                ),
                (
                    "reqvire.search",
                    json!({"short":"yes"}),
                    "Argument 'short' must be a boolean",
                ),
                (
                    "reqvire.collect",
                    json!({"element_name":"Root", "direction":"sideways"}),
                    "Argument 'direction' has unsupported value '\"sideways\"'",
                ),
                (
                    "reqvire.semantic.export",
                    json!({"layers":[42]}),
                    "Argument 'layers' must contain only strings",
                ),
            ] {
                assert_eq!(
                    validate_tool_arguments(name, &args, enabled)
                        .expect_err("test fixture operation should fail"),
                    diagnostic
                );
            }
        }
        assert!(validate_tool_arguments("reqvire.format", &json!({"fix":true}), false).is_err());
        assert!(validate_tool_arguments("reqvire.format", &json!({"fix":true}), true).is_ok());
        assert!(validate_tool_arguments(
            "reqvire.add_element",
            &json!({"file":"Model.md","content":"content"}),
            false
        )
        .is_err());
        assert!(validate_tool_arguments(
            "reqvire.add_element",
            &json!({"file":"Model.md","content":"content"}),
            true
        )
        .is_ok());
        // Consumers own their discovery output and cannot mutate the cached schema.
        let mut altered = tool_definitions(false);
        altered[0]["inputSchema"]["properties"]["alien"] = json!({"type":"boolean"});
        assert!(
            validate_tool_arguments("reqvire.workspace_status", &json!({"alien":true}), false)
                .is_err()
        );
    }

    #[test]
    fn existing_element_discovery_preserves_argument_domains() {
        let definitions = tool_definitions(true);
        for (tool, selectors) in [
            ("reqvire.read_element", vec!["name"]),
            ("reqvire.model", vec!["from"]),
            ("reqvire.submodels", vec!["from"]),
            ("reqvire.coverage", vec!["from"]),
            ("reqvire.collect", vec!["element_name"]),
            ("reqvire.concepts.get", vec!["name"]),
            ("reqvire.semantic.queries", vec!["name"]),
            ("reqvire.semantic.queries.validate", vec!["name"]),
            ("reqvire.remove_element", vec!["element_name"]),
            ("reqvire.move_element", vec!["element_name"]),
            ("reqvire.rename_element", vec!["element_name"]),
            ("reqvire.merge_elements", vec!["target", "sources"]),
            ("reqvire.link", vec!["source", "target"]),
            ("reqvire.unlink", vec!["source", "target"]),
            ("reqvire.relink", vec!["source", "from_target", "to_target"]),
        ] {
            let definition = definitions
                .iter()
                .find(|item| item["name"] == tool)
                .expect("public tool");
            for selector in selectors {
                let property = &definition["inputSchema"]["properties"][selector];
                let schema = if property["type"] == "array" {
                    &property["items"]
                } else {
                    property
                };
                let description = schema["description"]
                    .as_str()
                    .expect("selector description");
                assert!(
                    description.contains("Exact element name")
                        && description.contains("canonical element identifier"),
                    "{tool} {selector}: {description}"
                );
            }
        }
        let read = definitions
            .iter()
            .find(|item| item["name"] == "reqvire.read_element")
            .expect("read");
        assert!(
            read["inputSchema"]["properties"]["identifier"]["description"]
                .as_str()
                .expect("explicit ID")
                .contains("identifier only")
        );
        let rename = definitions
            .iter()
            .find(|item| item["name"] == "reqvire.rename_element")
            .expect("rename");
        assert!(
            rename["inputSchema"]["properties"]["new_name"]["description"]
                .as_str()
                .expect("literal")
                .contains("Literal")
        );
    }
}
