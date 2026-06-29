use super::*;
use crate::concept::concept_local_name;
use crate::Element;

pub(crate) fn semantic_export_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let layers = semantic_export_layers_arg(args)?;
    semantic_export_layers_tool(
        args,
        excluded_filename_patterns,
        with_size_estimates,
        layers,
        "export",
        None,
    )
}

pub(crate) fn ontologies_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    semantic_export_layers_tool(
        args,
        excluded_filename_patterns,
        with_size_estimates,
        vec![SemanticExportLayer::Ontologies],
        "ontologies",
        Some(OntologyContentFilter::Ontology),
    )
}

pub(crate) fn shapes_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    semantic_export_layers_tool(
        args,
        excluded_filename_patterns,
        with_size_estimates,
        vec![SemanticExportLayer::Shapes],
        "shapes",
        Some(OntologyContentFilter::Shacl),
    )
}

pub(crate) fn concepts_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    let model = load_model_with_options(excluded_filename_patterns, with_size_estimates)?;
    let semantic_store = model.semantic_store.as_ref().ok_or_else(|| {
        ReqvireError::ProcessError("Parsed model is missing semantic RDF query state".to_string())
    })?;
    let (format_name, export_format) = semantic_tool_format(args, "semantic concepts")?;
    let content = semantic_store.index.serialize_export_layers(
        export_format,
        &[SemanticExportLayer::Concepts],
        None,
    )?;
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
        semantic_graph_layers_for_export(&[SemanticExportLayer::Concepts]),
    )?;
    if let Some(map) = object.as_object_mut() {
        map.insert(
            "layers".to_string(),
            semantic_export_layer_values(&[SemanticExportLayer::Concepts]),
        );
        map.insert("concepts".to_string(), json!(concepts));
    }
    Ok(object)
}

pub(crate) fn semantic_model_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    semantic_export_layers_tool(
        args,
        excluded_filename_patterns,
        with_size_estimates,
        vec![SemanticExportLayer::Model],
        "model",
        None,
    )
}

pub(crate) fn semantic_graph_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Value, ReqvireError> {
    semantic_export_layers_tool(
        args,
        excluded_filename_patterns,
        with_size_estimates,
        Vec::new(),
        "graph",
        None,
    )
}

fn semantic_export_layers_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
    requested_layers: Vec<SemanticExportLayer>,
    semantic_layer: &str,
    filter: Option<OntologyContentFilter>,
) -> Result<Value, ReqvireError> {
    let model = load_model_with_options(excluded_filename_patterns, with_size_estimates)?;
    let semantic_store = model.semantic_store.as_ref().ok_or_else(|| {
        ReqvireError::ProcessError("Parsed model is missing semantic RDF query state".to_string())
    })?;
    let (format_name, export_format) = semantic_tool_format(args, semantic_layer)?;
    let namespace_base = string_arg(args, "namespace_base");
    let content = semantic_store.index.serialize_export_layers(
        export_format,
        &requested_layers,
        namespace_base.as_deref(),
    )?;
    let effective_layers = if requested_layers.is_empty() {
        SemanticExportLayer::default_layers()
    } else {
        requested_layers.clone()
    };
    let include_external = effective_layers.contains(&SemanticExportLayer::ExternalUsed);
    let mut serializable_index = if let Some(filter) = filter {
        filtered_semantic_index(&semantic_store.index, filter)
    } else {
        semantic_store.index.clone()
    };
    serializable_index.apply_external_visibility(include_external)?;
    let external_metadata = semantic_contract::external_materialization_metadata(
        &semantic_store.index,
        &serializable_index,
        include_external,
    );
    let mut object = semantic_layer_response(
        format_name,
        semantic_layer,
        content,
        &serializable_index,
        include_external,
        semantic_graph_layers_for_export(&effective_layers),
    )?;
    if let Some(map) = object.as_object_mut() {
        map.insert(
            "layers".to_string(),
            semantic_export_layer_values(&effective_layers),
        );
        map.insert(
            "external_materialization".to_string(),
            external_metadata["external_materialization"].clone(),
        );
        map.insert(
            "external_counts".to_string(),
            external_metadata["external_counts"].clone(),
        );
        if let Some(namespace_base) = namespace_base {
            map.insert("namespace_base".to_string(), json!(namespace_base));
        }
    }
    Ok(object)
}

fn semantic_export_layers_arg(args: &Value) -> Result<Vec<SemanticExportLayer>, ReqvireError> {
    let Some(value) = args.get("layers") else {
        return Ok(Vec::new());
    };
    let Some(items) = value.as_array() else {
        return Err(ReqvireError::ProcessError(
            "semantic export layers must be an array of layer names".to_string(),
        ));
    };
    let mut layers = Vec::new();
    for item in items {
        let Some(layer) = item.as_str() else {
            return Err(ReqvireError::ProcessError(
                "semantic export layers must be strings".to_string(),
            ));
        };
        layers.push(match layer {
            "ontologies" => SemanticExportLayer::Ontologies,
            "shapes" => SemanticExportLayer::Shapes,
            "concepts" => SemanticExportLayer::Concepts,
            "model" => SemanticExportLayer::Model,
            "external-used" => SemanticExportLayer::ExternalUsed,
            "prefixes" => SemanticExportLayer::Prefixes,
            other => {
                return Err(ReqvireError::ProcessError(format!(
                    "Invalid semantic export layer '{}'. Valid values: ontologies, shapes, concepts, model, external-used, prefixes",
                    other
                )))
            }
        });
    }
    Ok(layers)
}

pub(crate) fn concepts_list_tool(
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
            scheme_iri
                .as_deref()
                .is_none_or(|scheme| item.get("scheme_iri").and_then(Value::as_str) == Some(scheme))
        })
        .filter(|item| concept_item_matches_filter(item, filter.as_deref()))
        .collect::<Vec<_>>();
    let count = concepts.len();
    Ok(json!({
        "concepts": concepts,
        "count": count,
    }))
}

pub(crate) fn concept_schemes_list_tool(
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

pub(crate) fn concept_get_tool(
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
        .ok_or_else(|| {
            ReqvireError::ProcessError(
                "No generated native concept matched the requested selector".to_string(),
            )
        })
}

pub(crate) fn concept_mappings_list_tool(
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
            if source_filter
                .as_deref()
                .is_some_and(|value| value != source_iri)
                || target_filter
                    .as_deref()
                    .is_some_and(|value| value != target_iri)
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
        let jsonld: Value = serde_json::from_str(content)?;
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

pub(crate) fn semantic_index_with_external_visibility(
    source: &semantic_contract::SemanticIndex,
    include_external: bool,
) -> Result<semantic_contract::SemanticIndex, ReqvireError> {
    source.with_external_visibility(include_external)
}

pub(crate) fn semantic_prefixes_tool(
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
    let mut entries = concept_scheme_prefix_records(model)
        .into_iter()
        .map(|record| {
            json!({
                "prefix": record.namespace_prefix,
                "namespace": record.namespace,
                "concept_base": record.namespace_base,
                "term_namespace": record.namespace,
                "scheme_iri": record.scheme_iri,
                "external": false,
                "source": concept_scheme_prefix_source(record.element),
                "concept_schemes": [{
                    "scheme_element_identifier": record.element.identifier,
                    "scheme_element_name": record.element.name,
                    "file_path": record.element.file_path,
                    "line_number": record.element.line_number,
                    "scheme_iri": record.scheme_iri
                }],
                "contributors": [{
                    "element_identifier": record.element.identifier,
                    "element_name": record.element.name
                }]
            })
        })
        .collect::<Vec<_>>();
    sort_items(&mut entries);
    entries
}

fn concept_scheme_vocabulary_prefixes(model: &ModelManager) -> Vec<Value> {
    let mut prefixes = concept_scheme_prefix_records(model)
        .into_iter()
        .map(|record| {
            json!({
                "prefix": record.namespace_prefix,
                "namespace": record.namespace,
                "concept_base": record.namespace_base,
                "term_namespace": record.namespace,
                "external": false,
                "source": concept_scheme_prefix_source(record.element)
            })
        })
        .collect::<Vec<_>>();
    sort_items(&mut prefixes);
    prefixes
}

struct ConceptSchemePrefixRecord<'a> {
    element: &'a Element,
    namespace_base: &'a str,
    namespace_prefix: &'a str,
    namespace: String,
    scheme_iri: String,
}

fn concept_scheme_prefix_records(model: &ModelManager) -> Vec<ConceptSchemePrefixRecord<'_>> {
    model
        .graph_registry
        .get_all_elements()
        .into_iter()
        .filter(|element| element.element_type.is_concept_scheme())
        .filter_map(|element| {
            let payload = element.concept_scheme.as_ref()?;
            let namespace_base = payload.namespace_base.as_deref()?;
            let namespace_prefix = payload.namespace_prefix.as_deref()?;
            let namespace = format!("{}#", namespace_base.trim_end_matches('#'));
            let scheme_iri = format!("{}{}", namespace, concept_local_name(&element.name));
            Some(ConceptSchemePrefixRecord {
                element,
                namespace_base,
                namespace_prefix,
                namespace,
                scheme_iri,
            })
        })
        .collect()
}

fn concept_scheme_prefix_source(element: &Element) -> Value {
    json!({
        "element_identifier": element.identifier,
        "element_name": element.name,
        "file_path": element.file_path,
        "line_number": element.line_number,
        "content": semantic_prefix_source_content(&element.content)
    })
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

pub(crate) fn semantic_vocabulary_tool(
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
    let document_by_element = index.ontology_document_by_element();
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
                vocab::RDF_TYPE => {
                    if let Some(iri) = term_iri(&quad.object) {
                        entry.types.insert(iri.to_string());
                    }
                }
                vocab::RDFS_LABEL => {
                    if let Some(value) = literal_value(&quad.object) {
                        entry.label = Some(value.to_string());
                    }
                }
                vocab::RDFS_COMMENT => {
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
            let mut item = base_term_item(&declaration.iri, prefixes, info, declaration.external);
            item.insert("role".to_string(), json!(role));
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
            if let Some(info) = info {
                if let Some(domain) = first_iri_property(info, vocab::RDFS_DOMAIN, prefixes) {
                    item.insert("domain".to_string(), json!(domain));
                }
                if let Some(range) = first_iri_property(info, vocab::RDFS_RANGE, prefixes) {
                    item.insert("range".to_string(), json!(range));
                }
            }
            if include_source {
                item.insert(
                    "source".to_string(),
                    source_for_ontology_declaration(model, index, term_index, declaration),
                );
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
    let mut items = Vec::new();
    for (iri, info) in term_index {
        let is_scheme = info.types.contains(vocab::SKOS_CONCEPT_SCHEME);
        let is_concept = info.types.contains(vocab::SKOS_CONCEPT);
        if !is_scheme && !is_concept {
            continue;
        }
        let generated_from_markdown = index
            .concept_layer_subject_has_type(iri, vocab::SKOS_CONCEPT_SCHEME)
            || index.concept_layer_subject_has_type(iri, vocab::SKOS_CONCEPT);
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
            json!(first_string(info, vocab::SKOS_PREF_LABEL)),
        );
        item.insert(
            "definition".to_string(),
            json!(first_string(info, vocab::SKOS_DEFINITION)),
        );
        item.insert(
            "alt_labels".to_string(),
            json!(strings(info, vocab::SKOS_ALT_LABEL)),
        );
        item.insert(
            "hidden_labels".to_string(),
            json!(strings(info, vocab::SKOS_HIDDEN_LABEL)),
        );
        item.insert(
            "scope_notes".to_string(),
            json!(strings(info, vocab::SKOS_SCOPE_NOTE)),
        );
        item.insert(
            "examples".to_string(),
            json!(strings(info, vocab::SKOS_EXAMPLE)),
        );
        item.insert(
            "in_scheme".to_string(),
            json!(iris_as_curies(info, vocab::SKOS_IN_SCHEME, prefixes)),
        );
        item.insert(
            "top_concepts".to_string(),
            json!(iris_as_curies(info, vocab::SKOS_HAS_TOP_CONCEPT, prefixes)),
        );
        item.insert(
            "broader".to_string(),
            json!(iris_as_curies(info, vocab::SKOS_BROADER, prefixes)),
        );
        item.insert(
            "narrower".to_string(),
            json!(iris_as_curies(info, vocab::SKOS_NARROWER, prefixes)),
        );
        item.insert(
            "related".to_string(),
            json!(iris_as_curies(info, vocab::SKOS_RELATED, prefixes)),
        );
        item.insert(
            "exact_match".to_string(),
            json!(iris_as_curies(info, vocab::SKOS_EXACT_MATCH, prefixes)),
        );
        item.insert(
            "close_match".to_string(),
            json!(iris_as_curies(info, vocab::SKOS_CLOSE_MATCH, prefixes)),
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
    let named_individual = vocab::OWL_NAMED_INDIVIDUAL;
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
        let mut item = base_term_item(iri, prefixes, Some(info), term_info_external(info));
        item.insert("types".to_string(), json!(semantic_types));
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
            source_map_declaration_item(model, index, &term_index, declaration, prefixes)
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
            "title": "Requirements using Contract Bindings",
            "preferred_property": "reqvire:bindsContract"
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
                json!("SELECT ?requirement ?contract WHERE { ?requirement a reqvire:Requirement ; reqvire:bindsContract ?contract . } ORDER BY ?requirement ?contract"),
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

fn base_term_item(
    iri: &str,
    prefixes: &[VocabularyPrefix],
    info: Option<&TermInfo>,
    external: bool,
) -> serde_json::Map<String, Value> {
    let mut item = serde_json::Map::new();
    item.insert("iri".to_string(), json!(iri));
    item.insert("curie".to_string(), json!(curie(iri, prefixes)));
    item.insert("external".to_string(), json!(external));
    item.insert(
        "label".to_string(),
        json!(info.and_then(|entry| entry.label.clone())),
    );
    item.insert(
        "comment".to_string(),
        json!(info.and_then(|entry| entry.comment.clone())),
    );
    item
}

fn source_for_ontology_declaration(
    model: &ModelManager,
    index: &semantic_contract::SemanticIndex,
    term_index: &BTreeMap<String, TermInfo>,
    declaration: &semantic_contract::OntologyTermDeclaration,
) -> Value {
    if declaration.external {
        term_index
            .get(&declaration.iri)
            .map(|info| source_for_term(index, info))
            .unwrap_or(Value::Null)
    } else {
        source_for_element_identifier(model, &declaration.element_identifier)
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

fn source_map_declaration_item(
    model: &ModelManager,
    index: &semantic_contract::SemanticIndex,
    term_index: &BTreeMap<String, TermInfo>,
    declaration: &semantic_contract::OntologyTermDeclaration,
    prefixes: &[VocabularyPrefix],
) -> Value {
    json!({
        "term": curie(&declaration.iri, prefixes),
        "iri": declaration.iri,
        "role": declaration.role.to_string(),
        "external": declaration.external,
        "external_materialization": if declaration.external { Value::String("used_subset".to_string()) } else { Value::Null },
        "materialized_in_used_subset": declaration.external && declaration.materialized_in_used_subset,
        "source": source_for_ontology_declaration(model, index, term_index, declaration)
    })
}

fn term_info_external(info: &TermInfo) -> bool {
    matches!(info.source_block, Some(source) if source.external)
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
    items.sort_by_key(|left| left.to_string());
}

fn semantic_export_layer_values(layers: &[SemanticExportLayer]) -> Value {
    json!(layers
        .iter()
        .map(SemanticExportLayer::as_str)
        .collect::<Vec<_>>())
}

fn literal_value(term: &Term) -> Option<&str> {
    match term {
        Term::Literal(literal) => Some(literal.value()),
        _ => None,
    }
}

fn semantic_graph_layers_for_export(layers: &[SemanticExportLayer]) -> Vec<Value> {
    let includes = |layer| layers.contains(&layer);
    vec![
        graph_layer(
            "ontologies",
            semantic_store::GRAPH_AUTHORED_ONTOLOGY,
            includes(SemanticExportLayer::Ontologies),
            "authored ontology RDF layer",
        ),
        graph_layer(
            "shapes",
            semantic_store::GRAPH_AUTHORED_ONTOLOGY,
            includes(SemanticExportLayer::Shapes),
            "authored semantic-contract SHACL layer",
        ),
        graph_layer(
            "concepts",
            semantic_store::GRAPH_AUTHORED_ONTOLOGY,
            includes(SemanticExportLayer::Concepts),
            "generated native SKOS concept layer",
        ),
        graph_layer(
            "model",
            semantic_store::GRAPH_AUTHORED_MODEL,
            includes(SemanticExportLayer::Model),
            "generated Reqvire model facts layer",
        ),
        graph_layer(
            "external-used",
            semantic_store::GRAPH_EXTERNAL_USED_SUBSET,
            includes(SemanticExportLayer::ExternalUsed),
            "used external subset produced by o-kernel",
        ),
        graph_layer(
            "prefixes",
            semantic_store::GRAPH_GENERATED,
            includes(SemanticExportLayer::Prefixes),
            "generated Turtle prefix projection layer",
        ),
        graph_layer(
            "raw-external-source",
            "urn:reqvire:semantic-graph:raw-external-source",
            false,
            "raw external dependency graph (internal only)",
        ),
    ]
}

pub(crate) fn semantic_graph_layers(full: bool, include_external: bool) -> Vec<Value> {
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
        ("rdf", reserved::RDF_NS),
        ("rdfs", reserved::RDFS_NS),
        ("owl", reserved::OWL_NS),
        ("sh", reserved::SHACL_NS),
        ("skos", vocab::SKOS_NS),
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
