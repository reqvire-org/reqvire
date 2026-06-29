use super::*;

pub(crate) fn workspace_status(
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let model = crate::operations::load_model_with_options(
        excluded_filename_patterns,
        with_size_estimates,
    )?;
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

pub(crate) fn model_revision(
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let model = crate::operations::load_model_with_options(
        excluded_filename_patterns,
        with_size_estimates,
    )?;
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

pub(crate) fn read_element(
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

    let model = crate::operations::load_model_with_options(
        excluded_filename_patterns,
        with_size_estimates,
    )?;
    let element = crate::operations::read_element(
        &model.graph_registry,
        identifier.as_deref(),
        name.as_deref(),
    )?;

    serde_json::to_value(element).map_err(ReqvireError::from)
}

pub(crate) fn search_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let model = crate::operations::load_model(excluded_filename_patterns)?;
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
        bool_arg(args, "has_contract_bindings", false),
        string_arg(args, "filter_contract_bindings").as_deref(),
    )?;
    parse_json_string(crate::operations::search_report(
        &model.graph_registry,
        &filters,
        true,
        bool_arg(args, "short", false),
    )?)
}

pub(crate) fn model_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let model = crate::operations::load_model_with_options(
        excluded_filename_patterns,
        with_size_estimates,
    )?;
    let filter_type = string_arg(args, "filter_type");
    let type_filter: Option<Vec<&str>> = filter_type
        .as_deref()
        .map(|s| s.split(',').map(|t| t.trim()).collect());
    parse_json_string(crate::operations::model_report(
        &model.graph_registry,
        string_arg(args, "from").as_deref(),
        bool_arg(args, "reverse", false),
        type_filter,
    )?)
}

pub(crate) fn containment_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let model = crate::operations::load_model(excluded_filename_patterns)?;
    let hierarchy = crate::operations::containment_hierarchy(
        &model.graph_registry,
        bool_arg(args, "short", false),
    )?;
    serde_json::to_value(hierarchy).map_err(ReqvireError::from)
}

pub(crate) fn collect_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let model = crate::operations::load_model(excluded_filename_patterns)?;
    let direction = match string_arg(args, "direction")
        .unwrap_or_else(|| "UPSTREAM".to_string())
        .to_uppercase()
        .as_str()
    {
        "UPSTREAM" => report::collect::CollectDirection::Upstream,
        "DOWNSTREAM" => report::collect::CollectDirection::Downstream,
        other => {
            return Err(ReqvireError::ProcessError(format!(
                "Invalid direction '{}'. Valid values: UPSTREAM, DOWNSTREAM",
                other
            )));
        }
    };
    parse_json_string(crate::operations::collect_report(
        &model.graph_registry,
        &required_string_arg(args, "element_name")?,
        true,
        direction,
    )?)
}

pub(crate) fn submodels_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let model = crate::operations::load_model(excluded_filename_patterns)?;
    let report = crate::operations::submodels_report(
        &model.graph_registry,
        string_arg(args, "from").as_deref(),
    )?;
    parse_json_string(report.to_json_string())
}

pub(crate) fn sparql_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let query = required_string_arg(args, "query")?;
    let full = bool_arg(args, "full", true);
    let include_external = bool_arg(args, "include_external", false);
    let model = crate::operations::load_model_with_options(
        excluded_filename_patterns,
        with_size_estimates,
    )?;
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

pub(crate) fn lint_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let model = crate::operations::load_model_lenient(excluded_filename_patterns)?;
    let report = crate::operations::lint_report(&model.graph_registry);
    parse_json_string(report.to_json_string(
        bool_arg(args, "fixable", false),
        bool_arg(args, "auditable", false),
    ))
}

pub(crate) fn coverage_tool(excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let model = crate::operations::load_model(excluded_filename_patterns)?;
    let report = crate::operations::coverage_report(&model.graph_registry);
    parse_json_string(report.to_json_string())
}

pub(crate) fn traces_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let model = crate::operations::load_model(excluded_filename_patterns)?;
    let report = crate::operations::traces_report(
        &model.graph_registry,
        string_arg(args, "filter_id").as_deref(),
        string_arg(args, "filter_name").as_deref(),
        string_arg(args, "filter_type").as_deref(),
    )?;
    serde_json::to_value(report).map_err(ReqvireError::from)
}

pub(crate) fn resources_tool(excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let model = crate::operations::load_model(excluded_filename_patterns)?;
    let report = crate::operations::resources_report(&model.graph_registry);
    parse_json_string(report.to_json_string())
}

pub(crate) fn change_impact_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let git_commit = string_arg(args, "git_commit").unwrap_or_else(|| "HEAD".to_string());
    let model = crate::operations::load_model(excluded_filename_patterns)?;
    let (report, base_url, current_commit) = crate::operations::change_impact_report(
        &model.graph_registry,
        &git_commit,
        excluded_filename_patterns,
    )?;
    parse_json_string(report.to_json_string(&base_url, &current_commit, &git_commit))
}

pub(crate) fn format_tool(
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
    let model = crate::operations::load_model(excluded_filename_patterns)?;
    parse_json_string(crate::operations::format_report(
        &model.graph_registry,
        fix,
        bool_arg(args, "with_full_relations", false),
    )?)
}
