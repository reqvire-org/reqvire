use super::index::{
    build_ontology_document_declarations_turtle, build_ontology_term_definitions_turtle,
    concept_reference_iri_value, has_ontology_declaration,
};
use super::prefixes::{
    ontology_term_namespace, parse_external_ontology_block, turtle_prefix_binding,
};
use super::vocabulary::{
    projection_term_key, stable_hash, NormalizedRelationProjection, RelationProjectionDirection,
};
use super::*;

pub(super) fn export_filter_term_namespace(
    namespace_base: &str,
    documents: &[OntologyDocumentDeclaration],
) -> String {
    let trimmed = namespace_base.trim();
    if let Some(document) = documents
        .iter()
        .find(|document| document.ontology_base == trimmed || document.iri == trimmed)
    {
        return document.term_namespace.clone();
    }
    if trimmed.ends_with('#') {
        trimmed.to_string()
    } else {
        ontology_term_namespace(trimmed)
    }
}

pub(super) fn block_declares_subject_in_namespace(block: &SemanticBlock, namespace: &str) -> bool {
    block
        .quads
        .iter()
        .any(|quad| subject_iri(&quad.subject).is_some_and(|iri| iri.starts_with(namespace)))
}

pub fn external_materialization_metadata(
    source: &SemanticIndex,
    visible: &SemanticIndex,
    include_external: bool,
) -> Value {
    let materialized_terms: BTreeSet<String> = visible
        .external_blocks
        .iter()
        .flat_map(materialized_external_subjects)
        .collect();
    let visible_external_declaration_count = visible
        .ontology_declarations
        .values()
        .flat_map(|declarations| declarations.iter())
        .filter(|declaration| declaration.external)
        .count();
    let available_external_declaration_count = source
        .ontology_declarations
        .values()
        .flat_map(|declarations| declarations.iter())
        .filter(|declaration| declaration.external)
        .count();
    let used_external_source_count = source
        .external_sources
        .iter()
        .filter(|source| {
            materialized_terms
                .iter()
                .any(|term| term.starts_with(&source.namespace))
        })
        .count();
    let raw_external_triple_count: usize = source
        .external_blocks
        .iter()
        .map(|block| block.quads.len())
        .sum();
    let materialized_external_triple_count: usize = visible
        .external_blocks
        .iter()
        .map(|block| block.quads.len())
        .sum();

    json!({
        "external_materialization": if include_external { "used_subset" } else { "none" },
        "external_counts": {
            "declared_external_source_count": source.external_sources.len(),
            "visible_external_source_count": if include_external { used_external_source_count } else { 0 },
            "used_external_source_count": if include_external { used_external_source_count } else { 0 },
            "available_external_term_declaration_count": available_external_declaration_count,
            "visible_external_term_declaration_count": if include_external { visible_external_declaration_count } else { 0 },
            "materialized_external_term_count": if include_external { materialized_terms.len() } else { 0 },
            "raw_external_triple_count": raw_external_triple_count,
            "materialized_external_triple_count": if include_external { materialized_external_triple_count } else { 0 }
        }
    })
}

pub(super) fn materialized_external_subjects(block: &SemanticBlock) -> BTreeSet<String> {
    block
        .quads
        .iter()
        .filter_map(|quad| subject_iri(&quad.subject).map(str::to_string))
        .collect()
}

pub(super) fn append_blocks_turtle<'a>(
    output: &mut String,
    blocks: impl IntoIterator<Item = &'a SemanticBlock>,
    seen_quads: &mut BTreeSet<String>,
    prefix_map: &TurtlePrefixMap,
) -> Result<(), ReqvireError> {
    for block in blocks {
        let mut quads = Vec::new();
        for quad in &block.quads {
            let key = quad_key(quad);
            if seen_quads.insert(key) {
                quads.push(quad);
            }
        }
        if quads.is_empty() {
            continue;
        }

        output.push_str(
            "# -----------------------------------------------------------------------------\n",
        );
        output.push_str(&format!("# Source: {}\n", block.source));
        output.push_str(&format!("# Name: {}\n", block.source_name));
        output.push_str(&format!("# Kind: {}\n", block.kind.as_str()));
        output.push_str(&format!("# File: {}\n", block.file_path));
        if block.line_number > 0 {
            output.push_str(&format!("# Line: {}\n", block.line_number));
        }
        output.push('\n');
        let turtle = serialize_quads_turtle_body(&quads, prefix_map)?;
        if !turtle.trim().is_empty() {
            output.push_str(turtle.trim());
            output.push_str("\n\n");
        }
    }
    Ok(())
}

pub(super) fn normalized_export_blocks<'a>(
    blocks: &'a [SemanticBlock],
    seen_quads: &mut BTreeSet<String>,
) -> Vec<(&'a SemanticBlock, Vec<&'a Quad>)> {
    let mut normalized = Vec::new();
    for block in blocks {
        let mut quads = Vec::new();
        for quad in &block.quads {
            let key = quad_key(quad);
            if seen_quads.insert(key) {
                quads.push(quad);
            }
        }
        if !quads.is_empty() {
            normalized.push((block, quads));
        }
    }
    normalized
}

pub(super) fn unique_quads<'a>(
    quads: impl IntoIterator<Item = &'a Quad>,
    seen_quads: &mut BTreeSet<String>,
) -> Vec<&'a Quad> {
    quads
        .into_iter()
        .filter(|quad| seen_quads.insert(quad_key(quad)))
        .collect()
}

pub(super) fn append_used_external_subset_turtle(
    index: &SemanticIndex,
    output: &mut String,
    seen_quads: &mut BTreeSet<String>,
    prefix_map: &TurtlePrefixMap,
) -> Result<(), ReqvireError> {
    let used_external_subset_turtle = index.to_used_external_subset_turtle_string()?;
    let used_external_subset_quads = quads_from_turtle(
        &used_external_subset_turtle,
        "used external ontology subset projection",
    )?;
    let used_external_subset_quads = unique_quads(used_external_subset_quads.iter(), seen_quads);
    if used_external_subset_quads.is_empty() {
        return Ok(());
    }

    output.push_str(
        "# -----------------------------------------------------------------------------\n",
    );
    output.push_str("# Source: reqvire:external-used-subset\n");
    output.push_str("# Name: Reqvire used external ontology subset\n");
    output.push_str("# Kind: external-used-subset\n\n");
    let turtle = serialize_quads_turtle_body(&used_external_subset_quads, prefix_map)?;
    if !turtle.trim().is_empty() {
        output.push_str(turtle.trim());
        output.push_str("\n\n");
    }
    Ok(())
}

pub(super) fn semantic_export_layer_name(layer: &SemanticExportLayer) -> &'static str {
    match layer {
        SemanticExportLayer::Ontologies => "ontologies",
        SemanticExportLayer::Shapes => "shapes",
        SemanticExportLayer::Concepts => "concepts",
        SemanticExportLayer::Model => "model",
        SemanticExportLayer::ExternalUsed => "external-used",
        SemanticExportLayer::Prefixes => "prefixes",
    }
}

pub(super) fn serialize_turtle_as_format(
    turtle: &str,
    format: SemanticExportFormat,
    label: &str,
) -> Result<String, ReqvireError> {
    match format {
        SemanticExportFormat::Turtle => Ok(turtle.to_string()),
        SemanticExportFormat::JsonLd => {
            let mut serializer = RdfSerializer::from_format(RdfFormat::JsonLd {
                profile: JsonLdProfileSet::empty(),
            })
            .for_writer(Vec::new());

            for parsed in RdfParser::from_format(RdfFormat::Turtle).for_reader(turtle.as_bytes()) {
                let quad = parsed.map_err(|error| {
                    ReqvireError::SerializationError(format!(
                        "{} failed to parse as Turtle: {}",
                        label, error
                    ))
                })?;
                serializer.serialize_quad(quad.as_ref())?;
            }

            let bytes = serializer.finish()?;
            String::from_utf8(bytes).map_err(|e| ReqvireError::SerializationError(e.to_string()))
        }
    }
}

pub(super) fn append_turtle_body_unique(
    output: &mut String,
    turtle: &str,
    label: &str,
    seen_quads: &mut BTreeSet<String>,
    prefix_map: &TurtlePrefixMap,
) -> Result<(), ReqvireError> {
    if turtle.trim().is_empty() {
        return Ok(());
    }
    let parse_turtle = prefix_map.to_turtle_block() + turtle;
    let quads = quads_from_turtle(&parse_turtle, label)?;
    let quads = unique_quads(quads.iter(), seen_quads);
    if quads.is_empty() {
        return Ok(());
    }
    let body = serialize_quads_turtle_body(&quads, prefix_map)?;
    if !body.trim().is_empty() {
        output.push_str(body.trim());
        output.push_str("\n\n");
    }
    Ok(())
}

pub(super) fn append_quads_turtle_section(
    output: &mut String,
    header: &str,
    quads: &[&Quad],
    prefix_map: &TurtlePrefixMap,
) -> Result<(), ReqvireError> {
    if quads.is_empty() {
        return Ok(());
    }
    output.push_str(
        "# -----------------------------------------------------------------------------\n",
    );
    output.push_str(header);
    let turtle = serialize_quads_turtle_body(quads, prefix_map)?;
    if !turtle.trim().is_empty() {
        output.push_str(turtle.trim());
        output.push_str("\n\n");
    }
    Ok(())
}

pub(super) fn append_turtle_body(
    output: &mut String,
    turtle: &str,
    label: &str,
    prefix_map: &TurtlePrefixMap,
) -> Result<(), ReqvireError> {
    let parse_turtle = prefix_map.to_turtle_block() + turtle;
    let quads = quads_from_turtle(&parse_turtle, label)?;
    let quad_refs = quads.iter().collect::<Vec<_>>();
    let body = serialize_quads_turtle_body(&quad_refs, prefix_map)?;
    if !body.trim().is_empty() {
        output.push_str(body.trim());
        output.push_str("\n\n");
    }
    Ok(())
}

pub(super) fn skos_concept_iris(index: &SemanticIndex) -> BTreeSet<String> {
    index
        .blocks
        .iter()
        .filter(|block| matches!(block.kind, SemanticBlockKind::Concepts))
        .flat_map(|block| block.quads.iter())
        .filter_map(|quad| {
            if quad.predicate.as_str() == RDF_TYPE
                && matches!(
                    term_iri(&quad.object),
                    Some(SKOS_CONCEPT) | Some(SKOS_CONCEPT_SCHEME)
                )
            {
                subject_iri(&quad.subject).map(str::to_string)
            } else {
                None
            }
        })
        .collect()
}

pub(super) fn is_concept_layer_quad(
    block_kind: SemanticBlockKind,
    quad: &Quad,
    concept_iris: &BTreeSet<String>,
    include_mappings: bool,
) -> bool {
    let subject = subject_iri(&quad.subject);
    let object = term_iri(&quad.object);
    let predicate = quad.predicate.as_str();

    if predicate == REQVIRE_MAPS_TO_CONCEPT {
        return include_mappings && object.is_some_and(|iri| concept_iris.contains(iri));
    }

    if !matches!(block_kind, SemanticBlockKind::Concepts) {
        return false;
    }

    subject.is_some_and(|iri| concept_iris.contains(iri))
        || (predicate.starts_with(SKOS_NS) && object.is_some_and(|iri| concept_iris.contains(iri)))
}

pub(super) fn quad_keys_from_turtle_with_prefix_map(
    turtle: &str,
    prefix_map: &TurtlePrefixMap,
    label: &str,
) -> Result<BTreeSet<String>, ReqvireError> {
    let mut keys = BTreeSet::new();
    if turtle.trim().is_empty() {
        return Ok(keys);
    }
    let parse_turtle = prefix_map.to_turtle_block() + turtle;
    for quad in quads_from_turtle(&parse_turtle, label)? {
        keys.insert(quad_key(&quad));
    }
    Ok(keys)
}

pub(super) fn quads_from_turtle(turtle: &str, label: &str) -> Result<Vec<Quad>, ReqvireError> {
    let mut quads = Vec::new();
    if turtle.trim().is_empty() {
        return Ok(quads);
    }
    for parsed in RdfParser::from_format(RdfFormat::Turtle).for_reader(turtle.as_bytes()) {
        let quad = parsed.map_err(|error| {
            ReqvireError::SerializationError(format!(
                "{} failed to parse as Turtle: {}",
                label, error
            ))
        })?;
        quads.push(quad);
    }
    Ok(quads)
}

pub(super) fn serialize_quads_turtle_body(
    quads: &[&Quad],
    prefix_map: &TurtlePrefixMap,
) -> Result<String, ReqvireError> {
    let turtle = serialize_quads_turtle_with_prefixes(quads, prefix_map)?;
    Ok(strip_turtle_prefix_declarations(&turtle))
}

pub(super) fn serialize_quads_turtle_with_prefixes(
    quads: &[&Quad],
    prefix_map: &TurtlePrefixMap,
) -> Result<String, ReqvireError> {
    let mut serializer = prefix_map.serializer()?.for_writer(Vec::new());
    for quad in quads {
        serializer.serialize_quad((*quad).as_ref())?;
    }
    let bytes = serializer.finish()?;
    String::from_utf8(bytes).map_err(|e| ReqvireError::SerializationError(e.to_string()))
}

pub(super) fn serialize_turtle_with_prefix_map(
    turtle: &str,
    prefix_map: &TurtlePrefixMap,
    label: &str,
) -> Result<String, ReqvireError> {
    let quads = quads_from_turtle(turtle, label)?;
    let quad_refs = quads.iter().collect::<Vec<_>>();
    let serialized = serialize_quads_turtle_with_prefixes(&quad_refs, prefix_map)?;
    let body = strip_turtle_prefix_declarations(&serialized);
    let mut output = prefix_map.to_turtle_block();
    output.push_str(body.trim());
    output.push('\n');
    Ok(output)
}

pub(super) fn strip_turtle_prefix_declarations(turtle: &str) -> String {
    let mut output = String::new();
    for line in turtle.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("@prefix ") || trimmed.starts_with("PREFIX ") {
            continue;
        }
        output.push_str(line);
        output.push('\n');
    }
    output.trim_start_matches('\n').to_string()
}

pub(super) fn materialize_used_external_subset_turtle(
    index: &SemanticIndex,
) -> Result<String, ReqvireError> {
    if index.external_sources.is_empty() || index.external_blocks.is_empty() {
        return Ok(String::new());
    }

    let store = build_external_subset_derivation_store(index)?;
    let subset = o_kernel::subset::build_external_dependency_subset(
        &store,
        [
            GRAPH_AUTHORED_ONTOLOGY,
            GRAPH_AUTHORED_MODEL,
            GRAPH_GENERATED,
        ],
        [GRAPH_RAW_EXTERNAL_SOURCE],
    )
    .map_err(|error| {
        ReqvireError::ProcessError(format!(
            "Failed to build external dependency subset: {}",
            error
        ))
    })?;

    let public_subset_quads = subset
        .quads
        .into_iter()
        .filter(|entry| !is_external_source_metadata_quad(&entry.quad))
        .collect::<Vec<_>>();

    if public_subset_quads.is_empty() {
        return Ok(String::new());
    }

    serialize_constructed_subset_turtle(&public_subset_quads)
}

pub(super) fn is_external_source_metadata_quad(quad: &Quad) -> bool {
    quad.predicate.as_str() == o_kernel::vocab::RDFS_IS_DEFINED_BY
        || (quad.predicate.as_str() == o_kernel::vocab::RDF_TYPE
            && matches!(
                &quad.object,
                Term::NamedNode(node) if node.as_str() == o_kernel::vocab::OWL_ONTOLOGY
            ))
}

pub(super) fn build_external_subset_derivation_store(
    index: &SemanticIndex,
) -> Result<Store, ReqvireError> {
    let store = Store::new().map_err(|error| {
        ReqvireError::ProcessError(format!(
            "Failed to create external subset derivation store: {}",
            error
        ))
    })?;

    let authored_ontology_turtle = index.to_authored_ontology_layer_turtle_string()?;
    load_default_graph(
        &store,
        &authored_ontology_turtle,
        "authored ontology graph for external subset derivation",
    )?;
    load_named_graph(
        &store,
        &authored_ontology_turtle,
        GRAPH_AUTHORED_ONTOLOGY,
        "authored ontology graph for external subset derivation",
    )?;

    load_default_graph(
        &store,
        &index.model_context_turtle,
        "model context graph for external subset derivation",
    )?;
    load_named_graph(
        &store,
        &index.model_context_turtle,
        GRAPH_AUTHORED_MODEL,
        "model context graph for external subset derivation",
    )?;

    let ontology_projection_turtle = index.to_ontology_projection_turtle_string();
    load_default_graph(
        &store,
        &ontology_projection_turtle,
        "ontology projection graph for external subset derivation",
    )?;
    load_named_graph(
        &store,
        &ontology_projection_turtle,
        GRAPH_GENERATED,
        "ontology projection graph for external subset derivation",
    )?;

    let raw_external_turtle = index.to_raw_external_turtle_string()?;
    load_named_graph(
        &store,
        &raw_external_turtle,
        GRAPH_RAW_EXTERNAL_SOURCE,
        "raw external source graph for external subset derivation",
    )?;

    Ok(store)
}

pub(super) fn load_default_graph(
    store: &Store,
    turtle: &str,
    label: &str,
) -> Result<(), ReqvireError> {
    if turtle.trim().is_empty() {
        return Ok(());
    }
    store
        .load_from_reader(
            RdfParser::from_format(RdfFormat::Turtle).without_named_graphs(),
            turtle.as_bytes(),
        )
        .map_err(|error| {
            ReqvireError::ProcessError(format!("Failed to load {} into Oxigraph: {}", label, error))
        })
}

pub(super) fn load_named_graph(
    store: &Store,
    turtle: &str,
    graph_iri: &str,
    label: &str,
) -> Result<(), ReqvireError> {
    if turtle.trim().is_empty() {
        return Ok(());
    }
    let graph_name = NamedNode::new(graph_iri).map_err(|error| {
        ReqvireError::ProcessError(format!(
            "Invalid semantic named graph IRI '{}': {}",
            graph_iri, error
        ))
    })?;
    store
        .load_from_reader(
            RdfParser::from_format(RdfFormat::Turtle)
                .without_named_graphs()
                .with_default_graph(graph_name),
            turtle.as_bytes(),
        )
        .map_err(|error| {
            ReqvireError::ProcessError(format!("Failed to load {} into Oxigraph: {}", label, error))
        })
}

pub(super) fn serialize_constructed_subset_turtle(
    quads: &[o_kernel::subset::ConstructedQuad],
) -> Result<String, ReqvireError> {
    let mut serializer = RdfSerializer::from_format(RdfFormat::Turtle).for_writer(Vec::new());
    let mut seen_quads = BTreeSet::new();
    for quad in quads {
        let key = quad_key(&quad.quad);
        if seen_quads.insert(key) {
            let triple = Triple::new(
                quad.quad.subject.clone(),
                quad.quad.predicate.clone(),
                quad.quad.object.clone(),
            );
            serializer.serialize_triple(triple.as_ref())?;
        }
    }
    let bytes = serializer.finish()?;
    String::from_utf8(bytes).map_err(|e| ReqvireError::SerializationError(e.to_string()))
}

pub(super) fn quad_key(quad: &Quad) -> String {
    format!("{:?}", quad)
}

pub(super) fn build_authored_model_turtle(
    registry: &GraphRegistry,
    index: &SemanticIndex,
) -> Result<String, ReqvireError> {
    let mut output = String::new();
    let mut artifacts = BTreeSet::new();
    output.push_str(
        "# -----------------------------------------------------------------------------\n",
    );
    output.push_str("# Reqvire authored model context\n\n");
    output.push_str("@prefix owl: <http://www.w3.org/2002/07/owl#> .\n");
    output.push_str("@prefix reqvire: <https://www.reqvire.org/ontology#> .\n");
    output.push('\n');

    let mut nodes: Vec<_> = registry.nodes.values().collect();
    nodes.sort_by(|a, b| a.element.identifier.cmp(&b.element.identifier));

    for node in nodes {
        let element = &node.element;
        let subject = element_iri(element);

        output.push_str(&format!(
            "{} a {} ;\n",
            subject,
            element_type_classes(&element.element_type).join(", ")
        ));
        output.push_str(&format!(
            "  reqvire:elementId {} ;\n",
            turtle_string(&element.id)
        ));
        output.push_str(&format!(
            "  reqvire:elementIdentifier {} ;\n",
            turtle_string(&element.identifier)
        ));
        output.push_str(&format!(
            "  reqvire:elementName {} ;\n",
            turtle_string(&element.name)
        ));
        output.push_str(&format!(
            "  reqvire:elementType {} ;\n",
            turtle_string(&element.element_type.to_metadata_string())
        ));
        output.push_str(&format!(
            "  reqvire:filePath {} ;\n",
            turtle_string(&element.file_path)
        ));
        output.push_str(&format!(
            "  reqvire:lineNumber {} .\n\n",
            element.line_number
        ));

        for key in GOVERNANCE_METADATA_KEYS {
            if let Some(value) = element.metadata.get(*key) {
                output.push_str(&format!(
                    "{} reqvire:{} {} .\n",
                    subject,
                    key,
                    turtle_string(value)
                ));
            }
        }

        if let Some(ontology) = &element.ontology {
            if let Some(block) = &ontology.ontology {
                output.push_str(&format!(
                    "{} reqvire:ontologyText {} .\n",
                    subject,
                    turtle_string(&block.content)
                ));
            }
        }
        if let Some(contract) = &element.semantic_contract {
            output.push_str(&format!(
                "{} reqvire:semanticContractIri {} .\n",
                subject,
                turtle_string(&contract.iri)
            ));
            output.push_str(&format!(
                "{} reqvire:semanticContractKind \"semantic-contract\" .\n",
                subject
            ));
            if let Some(block) = &contract.shapes {
                output.push_str(&format!(
                    "{} reqvire:shapesText {} .\n",
                    subject,
                    turtle_string(&block.content)
                ));
            }
        }

        for relation in &element.relations {
            let Some(target_iri) =
                target_iri_for_link(&relation.target.link, registry, &mut artifacts)
            else {
                continue;
            };
            output.push_str(&format!(
                "{} reqvire:{} {} .\n",
                subject, relation.relation_type.name, target_iri
            ));
        }

        for contract_bindings in &element.contract_bindings {
            let Some(target_iri) =
                contract_bindings_target_iri(&contract_bindings.target, registry, &mut artifacts)
            else {
                continue;
            };
            output.push_str(&format!(
                "{} reqvire:bindsContract {} .\n",
                subject, target_iri
            ));
        }

        for reference in &element.concept_references {
            let Some(resolved_iri) = concept_reference_iri_value(registry, element, reference)
            else {
                continue;
            };
            output.push_str(&format!(
                "{} reqvire:conceptReference <{}> .\n",
                subject,
                escape_iri(&resolved_iri)
            ));
            output.push_str(&format!(
                "{} reqvire:referencesTerm <{}> .\n",
                subject,
                escape_iri(&resolved_iri)
            ));
        }

        if !element.relations.is_empty()
            || !element.contract_bindings.is_empty()
            || !element.concept_references.is_empty()
        {
            output.push('\n');
        }
    }

    let mut external_sources = index.external_sources.clone();
    external_sources.sort_by(|a, b| {
        a.owner_identifier
            .cmp(&b.owner_identifier)
            .then_with(|| a.prefix.cmp(&b.prefix))
            .then_with(|| a.namespace.cmp(&b.namespace))
    });
    for source in external_sources {
        output.push_str(&external_ontology_source_turtle(&source));
    }

    output.push('\n');
    Ok(output)
}

pub(super) fn build_generated_model_turtle(
    registry: &GraphRegistry,
    index: &SemanticIndex,
    prefix_map: &TurtlePrefixMap,
) -> Result<String, ReqvireError> {
    let mut output = String::new();
    output.push_str(
        "# -----------------------------------------------------------------------------\n",
    );
    output.push_str("# Reqvire generated ontology and model context\n\n");
    output.push_str("@prefix owl: <http://www.w3.org/2002/07/owl#> .\n");
    output.push_str("@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n");
    output.push_str("@prefix reqvire: <https://www.reqvire.org/ontology#> .\n");
    output.push_str("@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n");
    output.push('\n');
    output.push_str(&build_ontology_document_declarations_turtle(
        &index.ontology_documents,
    ));
    output.push_str(&build_ontology_term_definitions_turtle(
        &index.ontology_documents,
        &index.ontology_declarations,
        &index.blocks,
    ));
    output.push_str(&build_ontology_projection_turtle(index));
    output.push_str(&build_turtle_prefix_projection_turtle(prefix_map));

    let mut artifacts = BTreeSet::new();
    let mut nodes: Vec<_> = registry.nodes.values().collect();
    nodes.sort_by(|a, b| a.element.identifier.cmp(&b.element.identifier));

    for node in nodes {
        let element = &node.element;
        let subject = element_iri(element);

        for relation in &element.relations {
            let Some(target_iri) =
                target_iri_for_link(&relation.target.link, registry, &mut artifacts)
            else {
                continue;
            };
            append_model_relation_turtle(
                &mut output,
                &subject,
                relation.relation_type.name,
                &target_iri,
                relation.target.link.as_str(),
            );
            append_normalized_relation_family_turtle(
                &mut output,
                &subject,
                &target_iri,
                relation.relation_type.name,
            );
        }

        for contract_bindings in &element.contract_bindings {
            let Some(target_iri) =
                contract_bindings_target_iri(&contract_bindings.target, registry, &mut artifacts)
            else {
                continue;
            };
            append_model_relation_turtle(
                &mut output,
                &subject,
                "contract_bindings",
                &target_iri,
                &contract_bindings.target.as_str(),
            );
            append_normalized_relation_family_turtle(
                &mut output,
                &subject,
                &target_iri,
                "contract_bindings",
            );
        }
    }

    for artifact in artifacts {
        output.push_str(&artifact);
    }

    let mut declarations: Vec<_> = index
        .ontology_declarations
        .values()
        .flat_map(|declarations| declarations.iter())
        .collect();
    declarations.sort_by(|a, b| {
        a.element_identifier
            .cmp(&b.element_identifier)
            .then_with(|| a.iri.cmp(&b.iri))
            .then_with(|| a.role.cmp(&b.role))
    });
    for declaration in declarations {
        if declaration.external {
            continue;
        }
        if let Some(node) = registry.nodes.get(&declaration.element_identifier) {
            output.push_str(&format!(
                "{} reqvire:declaresTerm <{}> .\n",
                element_iri(&node.element),
                escape_iri(&declaration.iri)
            ));
        }
    }

    let mut shape_references = index.shape_references.clone();
    shape_references.sort();
    for reference in shape_references {
        if let Some(node) = registry.nodes.get(&reference.element_identifier) {
            output.push_str(&format!(
                "{} reqvire:referencesTerm <{}> .\n",
                element_iri(&node.element),
                escape_iri(&reference.iri)
            ));
        }
    }

    output.push('\n');
    Ok(output)
}

pub(super) fn build_model_context_turtle(
    registry: &GraphRegistry,
    index: &SemanticIndex,
) -> String {
    let mut output = String::new();
    let mut artifacts = BTreeSet::new();
    output.push_str(
        "# -----------------------------------------------------------------------------\n",
    );
    output.push_str("# Reqvire full semantic model context\n\n");
    output.push_str("@prefix owl: <http://www.w3.org/2002/07/owl#> .\n");
    output.push_str("@prefix reqvire: <https://www.reqvire.org/ontology#> .\n\n");

    let mut nodes: Vec<_> = registry.nodes.values().collect();
    nodes.sort_by(|a, b| a.element.identifier.cmp(&b.element.identifier));

    for node in nodes {
        let element = &node.element;

        let subject = element_iri(element);
        output.push_str(&format!(
            "{} a {} ;\n",
            subject,
            element_type_classes(&element.element_type).join(", ")
        ));
        output.push_str(&format!(
            "  reqvire:elementId {} ;\n",
            turtle_string(&element.id)
        ));
        output.push_str(&format!(
            "  reqvire:elementIdentifier {} ;\n",
            turtle_string(&element.identifier)
        ));
        output.push_str(&format!(
            "  reqvire:elementName {} ;\n",
            turtle_string(&element.name)
        ));
        output.push_str(&format!(
            "  reqvire:elementType {} ;\n",
            turtle_string(&element.element_type.to_metadata_string())
        ));
        output.push_str(&format!(
            "  reqvire:filePath {} ;\n",
            turtle_string(&element.file_path)
        ));
        output.push_str(&format!(
            "  reqvire:lineNumber {} .\n\n",
            element.line_number
        ));

        for key in GOVERNANCE_METADATA_KEYS {
            if let Some(value) = element.metadata.get(*key) {
                output.push_str(&format!(
                    "{} reqvire:{} {} .\n",
                    subject,
                    key,
                    turtle_string(value)
                ));
            }
        }

        if let Some(ontology) = &element.ontology {
            if let Some(block) = &ontology.ontology {
                output.push_str(&format!(
                    "{} reqvire:ontologyText {} .\n",
                    subject,
                    turtle_string(&block.content)
                ));
            }
        }
        if let Some(contract) = &element.semantic_contract {
            output.push_str(&format!(
                "{} reqvire:semanticContractIri {} .\n",
                subject,
                turtle_string(&contract.iri)
            ));
            output.push_str(&format!(
                "{} reqvire:semanticContractKind \"semantic-contract\" .\n",
                subject
            ));
            if let Some(block) = &contract.shapes {
                output.push_str(&format!(
                    "{} reqvire:shapesText {} .\n",
                    subject,
                    turtle_string(&block.content)
                ));
            }
        }

        for relation in &element.relations {
            let Some(target_iri) =
                target_iri_for_link(&relation.target.link, registry, &mut artifacts)
            else {
                continue;
            };
            output.push_str(&format!(
                "{} reqvire:{} {} .\n",
                subject, relation.relation_type.name, target_iri
            ));
            append_model_relation_turtle(
                &mut output,
                &subject,
                relation.relation_type.name,
                &target_iri,
                relation.target.link.as_str(),
            );
            output.push_str(&format!(
                "{} reqvire:relationTarget {} .\n",
                subject, target_iri
            ));
            append_normalized_relation_family_turtle(
                &mut output,
                &subject,
                &target_iri,
                relation.relation_type.name,
            );
        }

        for contract_bindings in &element.contract_bindings {
            let Some(target_iri) =
                contract_bindings_target_iri(&contract_bindings.target, registry, &mut artifacts)
            else {
                continue;
            };
            output.push_str(&format!(
                "{} reqvire:bindsContract {} .\n",
                subject, target_iri
            ));
            let target_identifier = contract_bindings.target.as_str();
            append_model_relation_turtle(
                &mut output,
                &subject,
                "contract_bindings",
                &target_iri,
                &target_identifier,
            );
            append_normalized_relation_family_turtle(
                &mut output,
                &subject,
                &target_iri,
                "contract_bindings",
            );
        }

        for reference in &element.concept_references {
            let Some(resolved_iri) = concept_reference_iri_value(registry, element, reference)
            else {
                continue;
            };
            output.push_str(&format!(
                "{} reqvire:conceptReference <{}> .\n",
                subject,
                escape_iri(&resolved_iri)
            ));
            output.push_str(&format!(
                "{} reqvire:referencesTerm <{}> .\n",
                subject,
                escape_iri(&resolved_iri)
            ));
        }

        if !element.relations.is_empty()
            || !element.contract_bindings.is_empty()
            || !element.concept_references.is_empty()
        {
            output.push('\n');
        }
    }

    for artifact in artifacts {
        output.push_str(&artifact);
    }

    let mut declarations: Vec<_> = index
        .ontology_declarations
        .values()
        .flat_map(|declarations| declarations.iter())
        .collect();
    declarations.sort_by(|a, b| {
        a.element_identifier
            .cmp(&b.element_identifier)
            .then_with(|| a.iri.cmp(&b.iri))
            .then_with(|| a.role.cmp(&b.role))
    });
    for declaration in declarations {
        if declaration.external {
            continue;
        }
        if let Some(node) = registry.nodes.get(&declaration.element_identifier) {
            output.push_str(&format!(
                "{} reqvire:declaresTerm <{}> .\n",
                element_iri(&node.element),
                escape_iri(&declaration.iri)
            ));
        }
    }

    let mut external_sources = index.external_sources.clone();
    external_sources.sort_by(|a, b| {
        a.owner_identifier
            .cmp(&b.owner_identifier)
            .then_with(|| a.prefix.cmp(&b.prefix))
            .then_with(|| a.namespace.cmp(&b.namespace))
    });
    for source in external_sources {
        output.push_str(&external_ontology_source_turtle(&source));
    }

    let mut shape_references = index.shape_references.clone();
    shape_references.sort();
    for reference in shape_references {
        if let Some(node) = registry.nodes.get(&reference.element_identifier) {
            output.push_str(&format!(
                "{} reqvire:referencesTerm <{}> .\n",
                element_iri(&node.element),
                escape_iri(&reference.iri)
            ));
        }
    }

    output.push('\n');
    output
}

pub(super) fn external_ontology_source_turtle(source: &ExternalOntologySource) -> String {
    let source_iri = external_ontology_source_iri(source);
    let owner_iri = element_iri_from_identifier(&source.owner_identifier);
    let mut output = String::new();
    let source_classes = if source.builtin {
        "reqvire:ExternalOntologySource, reqvire:BuiltInExternalOntologySource"
    } else {
        "reqvire:ExternalOntologySource"
    };
    output.push_str(&format!("{} a {} ;\n", source_iri, source_classes));
    output.push_str(&format!(
        "  reqvire:externalOntologyOwner {} ;\n",
        owner_iri
    ));
    if let Some(resource) = &source.resource {
        output.push_str(&format!(
            "  reqvire:externalOntologyResource <{}> ;\n",
            escape_iri(resource)
        ));
    }
    output.push_str(&format!(
        "  reqvire:externalOntologyPrefix {} ;\n",
        turtle_string(&source.prefix)
    ));
    output.push_str(&format!(
        "  reqvire:externalOntologyNamespace {} ;\n",
        turtle_string(&source.namespace)
    ));
    output.push_str(&format!(
        "  reqvire:externalOntologySourcePath {} ;\n",
        turtle_string(&source.source)
    ));
    if source.builtin {
        output.push_str("  reqvire:builtinExternalOntology true ;\n");
    }
    output.push_str(&format!(
        "  reqvire:externalOntologyFormat {} .\n\n",
        turtle_string(&source.format)
    ));
    output
}

pub(super) fn external_ontology_source_iri(source: &ExternalOntologySource) -> String {
    projection_generated_iri(
        "external-ontology-source",
        &format!(
            "{}\n{}\n{}\n{}",
            source.owner_identifier, source.prefix, source.namespace, source.source
        ),
    )
}

pub(super) fn normalized_relation_projection(
    relation_name: &str,
) -> Option<NormalizedRelationProjection> {
    let projection = match relation_name {
        "derive" => NormalizedRelationProjection {
            forward_property: "childElement",
            inverse_property: "parentElement",
            direction: RelationProjectionDirection::Forward,
        },
        "derivedFrom" => NormalizedRelationProjection {
            forward_property: "childElement",
            inverse_property: "parentElement",
            direction: RelationProjectionDirection::Inverse,
        },
        "specifiedBy" => NormalizedRelationProjection {
            forward_property: "capabilitySpecifiedByRequirement",
            inverse_property: "requirementSpecifiesCapability",
            direction: RelationProjectionDirection::Forward,
        },
        "specify" => NormalizedRelationProjection {
            forward_property: "capabilitySpecifiedByRequirement",
            inverse_property: "requirementSpecifiesCapability",
            direction: RelationProjectionDirection::Inverse,
        },
        "definedBy" => NormalizedRelationProjection {
            forward_property: "requirementDefinedByContract",
            inverse_property: "contractDefinesRequirement",
            direction: RelationProjectionDirection::Forward,
        },
        "define" => NormalizedRelationProjection {
            forward_property: "requirementDefinedByContract",
            inverse_property: "contractDefinesRequirement",
            direction: RelationProjectionDirection::Inverse,
        },
        "constrainedBy" => NormalizedRelationProjection {
            forward_property: "requirementConstrainedBySemanticContract",
            inverse_property: "semanticContractConstrainsRequirement",
            direction: RelationProjectionDirection::Forward,
        },
        "constrain" => NormalizedRelationProjection {
            forward_property: "requirementConstrainedBySemanticContract",
            inverse_property: "semanticContractConstrainsRequirement",
            direction: RelationProjectionDirection::Inverse,
        },
        "use" => NormalizedRelationProjection {
            forward_property: "semanticContractUsesOntology",
            inverse_property: "ontologyUsedBySemanticContract",
            direction: RelationProjectionDirection::Forward,
        },
        "usedBy" => NormalizedRelationProjection {
            forward_property: "semanticContractUsesOntology",
            inverse_property: "ontologyUsedBySemanticContract",
            direction: RelationProjectionDirection::Inverse,
        },
        "verifiedBy" => NormalizedRelationProjection {
            forward_property: "requirementVerifiedByVerification",
            inverse_property: "verificationVerifiesRequirement",
            direction: RelationProjectionDirection::Forward,
        },
        "verify" => NormalizedRelationProjection {
            forward_property: "requirementVerifiedByVerification",
            inverse_property: "verificationVerifiesRequirement",
            direction: RelationProjectionDirection::Inverse,
        },
        "satisfiedBy" => NormalizedRelationProjection {
            forward_property: "elementSatisfiedByArtifact",
            inverse_property: "artifactSatisfiesElement",
            direction: RelationProjectionDirection::Forward,
        },
        "satisfy" => NormalizedRelationProjection {
            forward_property: "elementSatisfiedByArtifact",
            inverse_property: "artifactSatisfiesElement",
            direction: RelationProjectionDirection::Inverse,
        },
        "contract_bindings" => NormalizedRelationProjection {
            forward_property: "requirementBindsContract",
            inverse_property: "contractBoundBy",
            direction: RelationProjectionDirection::Forward,
        },
        _ => return None,
    };
    Some(projection)
}

pub(super) fn append_model_relation_turtle(
    output: &mut String,
    source_iri: &str,
    relation_name: &str,
    target_iri: &str,
    target_identifier: &str,
) {
    let relation_iri = model_relation_iri(source_iri, relation_name, target_iri);
    output.push_str(&format!(
        "{} a reqvire:ModelRelation ;\n  reqvire:relationSource {} ;\n  reqvire:relationTarget {} ;\n  reqvire:relationType {} ;\n  reqvire:relationTargetIdentifier {} .\n",
        relation_iri,
        source_iri,
        target_iri,
        turtle_string(relation_name),
        turtle_string(target_identifier)
    ));
}

pub(super) fn append_normalized_relation_family_turtle(
    output: &mut String,
    source_iri: &str,
    target_iri: &str,
    relation_name: &str,
) {
    let Some(projection) = normalized_relation_projection(relation_name) else {
        return;
    };
    let (canonical_source, canonical_target) = match projection.direction {
        RelationProjectionDirection::Forward => (source_iri, target_iri),
        RelationProjectionDirection::Inverse => (target_iri, source_iri),
    };
    output.push_str(&format!(
        "{} reqvire:{} {} .\n",
        canonical_source, projection.forward_property, canonical_target
    ));
    output.push_str(&format!(
        "{} reqvire:{} {} .\n",
        canonical_target, projection.inverse_property, canonical_source
    ));
}

pub(super) fn build_semantic_term_context_turtle(index: &SemanticIndex) -> String {
    let mut output = String::new();
    let mut emitted = BTreeSet::new();

    let mut declarations: Vec<_> = index
        .ontology_declarations
        .values()
        .flat_map(|declarations| declarations.iter())
        .filter(|declaration| !declaration.external)
        .collect();
    declarations.sort_by(|a, b| {
        a.element_identifier
            .cmp(&b.element_identifier)
            .then_with(|| a.iri.cmp(&b.iri))
            .then_with(|| a.role.cmp(&b.role))
    });

    let mut shape_references = index.shape_references.clone();
    shape_references.sort();

    if declarations.is_empty() && shape_references.is_empty() {
        return output;
    }

    output.push_str(
        "# -----------------------------------------------------------------------------\n",
    );
    output.push_str("# Reqvire generated semantic term context\n\n");
    output.push_str("@prefix reqvire: <https://www.reqvire.org/ontology#> .\n\n");

    for declaration in declarations {
        let subject = element_iri_from_identifier(&declaration.element_identifier);
        let statement = format!(
            "{} reqvire:declaresTerm <{}> .\n",
            subject,
            escape_iri(&declaration.iri)
        );
        if emitted.insert(statement.clone()) {
            output.push_str(&statement);
        }
    }

    for reference in shape_references {
        let subject = element_iri_from_identifier(&reference.element_identifier);
        let statement = format!(
            "{} reqvire:referencesTerm <{}> .\n",
            subject,
            escape_iri(&reference.iri)
        );
        if emitted.insert(statement.clone()) {
            output.push_str(&statement);
        }
    }

    output.push('\n');
    output
}

pub(super) fn build_ontology_projection_turtle(index: &SemanticIndex) -> String {
    let graph = &index.ontology_projection;
    if graph.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    let mut term_resources = BTreeSet::new();
    let mut source_resources = BTreeSet::new();
    let mut provenance_resources = BTreeSet::new();
    let mut evidence_resources = BTreeSet::new();
    let mut member_resources = BTreeSet::new();

    output.push_str(
        "# -----------------------------------------------------------------------------\n",
    );
    output.push_str("# Reqvire generated ontology projection facts\n\n");
    output.push_str("@prefix reqvire: <https://www.reqvire.org/ontology#> .\n");
    output.push('\n');

    let graph_iri = projection_resource_iri(&graph.id);
    output.push_str(&format!(
        "{} a reqvire:OntologyProjectionGraph .\n",
        graph_iri
    ));
    output.push_str(&format!(
        "{} reqvire:projectionDerivationMode {} .\n",
        graph_iri,
        turtle_string(graph.derivation_mode.as_str())
    ));

    for projection in &graph.projections {
        let projection_iri = projection_resource_iri(&projection.id);
        output.push_str(&format!(
            "{} reqvire:ontologyConstructProjection {} .\n",
            graph_iri, projection_iri
        ));
    }
    for construct in &graph.constructs {
        output.push_str(&format!(
            "{} reqvire:projectedConstruct {} .\n",
            graph_iri,
            projection_resource_iri(&construct.id)
        ));
    }
    output.push('\n');

    for projection in &graph.projections {
        let projection_iri = projection_resource_iri(&projection.id);
        output.push_str(&format!(
            "{} a reqvire:OntologyConstructProjection .\n",
            projection_iri
        ));
        output.push_str(&format!(
            "{} reqvire:constructFamily {} .\n",
            projection_iri,
            turtle_string(projection.family.as_str())
        ));
        output.push_str(&format!(
            "{} reqvire:projectionDerivationMode {} .\n",
            projection_iri,
            turtle_string(projection.derivation_mode.as_str())
        ));
        for construct_id in &projection.construct_ids {
            output.push_str(&format!(
                "{} reqvire:projectedConstruct {} .\n",
                projection_iri,
                projection_resource_iri(construct_id)
            ));
        }
        output.push('\n');
    }

    for construct in &graph.constructs {
        let construct_iri = projection_resource_iri(&construct.id);
        let source_iri =
            serialize_projection_source(&construct.provenance.source, &mut source_resources);
        let provenance_iri = serialize_projection_provenance(
            &construct.id,
            &construct.provenance,
            &mut term_resources,
            &mut source_resources,
            &mut provenance_resources,
            &mut evidence_resources,
        );
        let subject_iri = serialize_projection_term(&construct.subject, &mut term_resources);

        output.push_str(&format!(
            "{} a reqvire:OntologyConstruct .\n",
            construct_iri
        ));
        output.push_str(&format!(
            "{} reqvire:constructFamily {} .\n",
            construct_iri,
            turtle_string(construct.family.as_str())
        ));
        output.push_str(&format!(
            "{} reqvire:constructKind {} .\n",
            construct_iri,
            turtle_string(construct.kind.as_str())
        ));
        output.push_str(&format!(
            "{} reqvire:projectionDerivationMode {} .\n",
            construct_iri,
            turtle_string(construct.provenance.derivation_mode.as_str())
        ));
        output.push_str(&format!(
            "{} reqvire:constructSourceBlock {} .\n",
            construct_iri, source_iri
        ));
        output.push_str(&format!(
            "{} reqvire:constructProvenance {} .\n",
            construct_iri, provenance_iri
        ));
        output.push_str(&format!(
            "{} reqvire:constructSubject {} .\n",
            construct_iri, subject_iri
        ));

        if let Some(predicate) = &construct.predicate {
            let predicate_iri = serialize_projection_term(predicate, &mut term_resources);
            output.push_str(&format!(
                "{} reqvire:constructPredicate {} .\n",
                construct_iri, predicate_iri
            ));
        }
        if let Some(object) = &construct.object {
            let object_iri = serialize_projection_term(object, &mut term_resources);
            output.push_str(&format!(
                "{} reqvire:constructObject {} .\n",
                construct_iri, object_iri
            ));
        }
        if let Some(property) = &construct.property {
            let property_iri = serialize_projection_term(property, &mut term_resources);
            output.push_str(&format!(
                "{} reqvire:constructProperty {} .\n",
                construct_iri, property_iri
            ));
        }
        if let Some(characteristic) = construct.property_characteristic {
            output.push_str(&format!(
                "{} reqvire:propertyCharacteristic {} .\n",
                construct_iri,
                turtle_string(characteristic.as_str())
            ));
        }
        if let Some(restriction_kind) = construct.restriction_kind {
            output.push_str(&format!(
                "{} reqvire:restrictionKind {} .\n",
                construct_iri,
                turtle_string(restriction_kind.as_str())
            ));
        }
        if let Some(class_expression_kind) = construct.class_expression_kind {
            output.push_str(&format!(
                "{} reqvire:classExpressionKind {} .\n",
                construct_iri,
                turtle_string(class_expression_kind.as_str())
            ));
        }
        if let Some(shape_overlay_kind) = construct.shape_overlay_kind {
            output.push_str(&format!(
                "{} reqvire:shapeOverlayKind {} .\n",
                construct_iri,
                turtle_string(shape_overlay_kind.as_str())
            ));
        }
        for member in &construct.members {
            let member_iri = serialize_construct_member(
                &construct.id,
                member,
                &mut term_resources,
                &mut source_resources,
                &mut member_resources,
            );
            output.push_str(&format!(
                "{} reqvire:constructMember {} .\n",
                construct_iri, member_iri
            ));
        }
        output.push('\n');
    }

    for resource in source_resources {
        output.push_str(&resource);
    }
    for resource in provenance_resources {
        output.push_str(&resource);
    }
    for resource in evidence_resources {
        output.push_str(&resource);
    }
    for resource in member_resources {
        output.push_str(&resource);
    }
    for resource in term_resources {
        output.push_str(&resource);
    }

    output
}

pub(super) fn build_turtle_prefix_projection_turtle(prefix_map: &TurtlePrefixMap) -> String {
    if prefix_map.declarations.is_empty() {
        return String::new();
    }

    let export_iri = "<urn:reqvire:semantic-export:prefixed-turtle>";
    let map_iri = "<urn:reqvire:semantic-export:turtle-prefix-map:top-level-export>";
    let mut output = String::new();
    output.push_str(
        "# -----------------------------------------------------------------------------\n",
    );
    output.push_str("# Reqvire generated Turtle prefix projection facts\n\n");
    output.push_str("@prefix reqvire: <https://www.reqvire.org/ontology#> .\n");
    output.push_str("@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n\n");
    output.push_str(&format!(
        "{} a reqvire:PrefixedTurtleExport ;\n  reqvire:turtlePrefixMap {} .\n\n",
        export_iri, map_iri
    ));
    output.push_str(&format!(
        "{} a reqvire:TurtlePrefixMap ;\n  reqvire:turtlePrefixScope \"top-level-export\"",
        map_iri
    ));
    for declaration in &prefix_map.declarations {
        output.push_str(&format!(
            " ;\n  reqvire:turtlePrefixDeclaration {}",
            turtle_prefix_declaration_iri(&declaration.prefix)
        ));
    }
    output.push_str(" .\n\n");

    for (index, declaration) in prefix_map.declarations.iter().enumerate() {
        output.push_str(&format!(
            "{} a reqvire:TurtlePrefixDeclaration ;\n",
            turtle_prefix_declaration_iri(&declaration.prefix)
        ));
        output.push_str(&format!(
            "  reqvire:turtlePrefixName {} ;\n",
            turtle_string(&declaration.prefix)
        ));
        output.push_str(&format!(
            "  reqvire:turtlePrefixNamespace {}^^xsd:anyURI ;\n",
            turtle_string(&declaration.namespace)
        ));
        output.push_str(&format!(
            "  reqvire:turtlePrefixSourceKind {} ;\n",
            turtle_string(turtle_prefix_source_kind(declaration.source_rank))
        ));
        output.push_str(&format!(
            "  reqvire:turtlePrefixReserved {} ;\n",
            turtle_prefix_is_reserved(declaration)
        ));
        output.push_str(&format!("  reqvire:turtlePrefixOrder {} .\n\n", index));
    }

    output
}

pub(super) fn turtle_prefix_declaration_iri(prefix: &str) -> String {
    format!(
        "<urn:reqvire:semantic-export:turtle-prefix:{}>",
        escape_iri(prefix)
    )
}

pub(super) fn turtle_prefix_is_reserved(declaration: &TurtlePrefixDeclaration) -> bool {
    (declaration.prefix == "reqvire" && declaration.namespace == REQVIRE_NS)
        || (declaration.source_rank == 0)
}

pub(super) fn turtle_prefix_source_kind(source_rank: u8) -> &'static str {
    match source_rank {
        0 => "built-in",
        10 => "authored-ontology",
        20 => "concept-scheme",
        _ => "external-source",
    }
}

pub(super) fn serialize_projection_provenance(
    construct_id: &str,
    provenance: &OntologyProjectionProvenance,
    term_resources: &mut BTreeSet<String>,
    source_resources: &mut BTreeSet<String>,
    provenance_resources: &mut BTreeSet<String>,
    evidence_resources: &mut BTreeSet<String>,
) -> String {
    let provenance_iri = projection_generated_iri("ontology-provenance", construct_id);
    let source_iri = serialize_projection_source(&provenance.source, source_resources);
    let mut chunk = String::new();
    chunk.push_str(&format!(
        "{} a reqvire:OntologyProjectionProvenance .\n",
        provenance_iri
    ));
    chunk.push_str(&format!(
        "{} reqvire:projectionDerivationMode {} .\n",
        provenance_iri,
        turtle_string(provenance.derivation_mode.as_str())
    ));
    chunk.push_str(&format!(
        "{} reqvire:provenanceSource {} .\n",
        provenance_iri, source_iri
    ));
    for (index, evidence) in provenance.evidence.iter().enumerate() {
        let evidence_iri = serialize_projection_evidence(
            construct_id,
            index,
            evidence,
            term_resources,
            source_resources,
            evidence_resources,
        );
        chunk.push_str(&format!(
            "{} reqvire:provenanceEvidence {} .\n",
            provenance_iri, evidence_iri
        ));
    }
    chunk.push('\n');
    provenance_resources.insert(chunk);
    provenance_iri
}

pub(super) fn serialize_projection_evidence(
    construct_id: &str,
    sequence_index: usize,
    evidence: &OntologyProjectionEvidence,
    term_resources: &mut BTreeSet<String>,
    source_resources: &mut BTreeSet<String>,
    evidence_resources: &mut BTreeSet<String>,
) -> String {
    let evidence_key = format!(
        "{}|{}|{}|{}|{}",
        construct_id,
        sequence_index,
        evidence.source.source_block,
        projection_term_key(&evidence.subject),
        projection_term_key(&evidence.object)
    );
    let evidence_iri = projection_generated_iri("ontology-evidence", &evidence_key);
    let source_iri = serialize_projection_source(&evidence.source, source_resources);
    let subject_iri = serialize_projection_term(&evidence.subject, term_resources);
    let predicate_iri = serialize_projection_term(&evidence.predicate, term_resources);
    let object_iri = serialize_projection_term(&evidence.object, term_resources);

    evidence_resources.insert(format!(
        "{} a reqvire:OntologyProjectionEvidence .\n{} reqvire:constructSourceBlock {} .\n{} reqvire:constructSubject {} .\n{} reqvire:constructPredicate {} .\n{} reqvire:constructObject {} .\n\n",
        evidence_iri,
        evidence_iri,
        source_iri,
        evidence_iri,
        subject_iri,
        evidence_iri,
        predicate_iri,
        evidence_iri,
        object_iri
    ));

    evidence_iri
}

pub(super) fn serialize_construct_member(
    construct_id: &str,
    member: &OntologyConstructMember,
    term_resources: &mut BTreeSet<String>,
    source_resources: &mut BTreeSet<String>,
    member_resources: &mut BTreeSet<String>,
) -> String {
    let member_key = format!(
        "{}|{}|{}",
        construct_id,
        member.sequence_index,
        projection_term_key(&member.term)
    );
    let member_iri = projection_generated_iri("ontology-member", &member_key);
    let term_iri = serialize_projection_term(&member.term, term_resources);
    let source_iri = serialize_projection_source(&member.source, source_resources);

    member_resources.insert(format!(
        "{} a reqvire:OntologyConstructMember .\n{} reqvire:memberTerm {} .\n{} reqvire:constructSourceBlock {} .\n{} reqvire:constructSequenceIndex {} .\n\n",
        member_iri,
        member_iri,
        term_iri,
        member_iri,
        source_iri,
        member_iri,
        member.sequence_index
    ));

    member_iri
}

pub(super) fn serialize_projection_term(
    term: &OntologyProjectionTerm,
    term_resources: &mut BTreeSet<String>,
) -> String {
    let term_iri = projection_term_iri(term);
    term_resources.insert(format!(
        "{} a reqvire:OntologyTerm .\n{} reqvire:termKind {} .\n{} reqvire:termValue {} .\n{} reqvire:conceptLabel {} .\n\n",
        term_iri,
        term_iri,
        turtle_string(term.kind.as_str()),
        term_iri,
        turtle_string(&term.value),
        term_iri,
        turtle_string(&term.label)
    ));
    term_iri
}

pub(super) fn serialize_projection_source(
    source: &OntologyProjectionSource,
    source_resources: &mut BTreeSet<String>,
) -> String {
    let source_iri = projection_generated_iri("semantic-block", &source.source_block);
    source_resources.insert(format!(
        "{} a reqvire:SemanticBlock, reqvire:OntologyProjectionSource .\n{} reqvire:sourceBlockId {} .\n{} reqvire:sourceElementIdentifier {} .\n{} reqvire:sourceName {} .\n{} reqvire:filePath {} .\n{} reqvire:lineNumber {} .\n{} reqvire:blockKind {} .\n\n",
        source_iri,
        source_iri,
        turtle_string(&source.source_block),
        source_iri,
        turtle_string(&source.source_element_identifier),
        source_iri,
        turtle_string(&source.source_name),
        source_iri,
        turtle_string(&source.file_path),
        source_iri,
        source.line_number,
        source_iri,
        turtle_string(&source.block_kind)
    ));
    source_iri
}

pub(super) fn projection_resource_iri(value: &str) -> String {
    format!("<{}>", escape_iri(value))
}

pub(super) fn projection_generated_iri(kind: &str, canonical: &str) -> String {
    projection_resource_iri(&format!("urn:reqvire:{}:{}", kind, stable_hash(canonical)))
}

pub(super) fn projection_term_iri(term: &OntologyProjectionTerm) -> String {
    match term.kind {
        OntologyProjectionTermKind::Iri => projection_resource_iri(&term.value),
        OntologyProjectionTermKind::BlankNode | OntologyProjectionTermKind::Literal => {
            projection_generated_iri("ontology-term", &projection_term_key(term))
        }
    }
}

pub(super) fn element_iri(element: &Element) -> String {
    format!("<{}>", element_iri_value(element))
}

pub(super) fn element_iri_value(element: &Element) -> String {
    format!("urn:reqvire:element:{}", escape_iri(&element.id))
}

pub(super) fn element_iri_from_identifier(identifier: &str) -> String {
    let element_id = crate::utils::extract_path_and_fragment(identifier)
        .1
        .unwrap_or(identifier);
    format!("<urn:reqvire:element:{}>", escape_iri(element_id))
}

pub(super) fn element_type_classes(element_type: &ElementType) -> Vec<&'static str> {
    match element_type {
        ElementType::Capability => {
            vec![
                "owl:NamedIndividual",
                "reqvire:Element",
                "reqvire:Capability",
            ]
        }
        ElementType::Requirement(_) => {
            vec![
                "owl:NamedIndividual",
                "reqvire:Element",
                "reqvire:Requirement",
            ]
        }
        ElementType::Ontology => {
            vec!["owl:NamedIndividual", "reqvire:Element", "reqvire:Ontology"]
        }
        ElementType::ConceptScheme => {
            vec![
                "owl:NamedIndividual",
                "reqvire:Element",
                "reqvire:ConceptScheme",
            ]
        }
        ElementType::Concept => {
            vec!["owl:NamedIndividual", "reqvire:Element", "reqvire:Concept"]
        }
        ElementType::Verification(verification_type) => {
            let subtype = match verification_type {
                VerificationType::Default | VerificationType::Test => "reqvire:TestVerification",
                VerificationType::FormalProof => "reqvire:FormalProofVerification",
                VerificationType::Analysis => "reqvire:AnalysisVerification",
                VerificationType::Inspection => "reqvire:InspectionVerification",
                VerificationType::Demonstration => "reqvire:DemonstrationVerification",
            };
            vec![
                "owl:NamedIndividual",
                "reqvire:Element",
                "reqvire:Verification",
                subtype,
            ]
        }
        ElementType::VerificationObjective => {
            vec![
                "owl:NamedIndividual",
                "reqvire:Element",
                "reqvire:VerificationObjective",
            ]
        }
        ElementType::SemanticContract => {
            vec![
                "owl:NamedIndividual",
                "reqvire:Element",
                "reqvire:SemanticContract",
            ]
        }
        ElementType::Contract(contract_type) => {
            let subtype = match contract_type {
                ContractType::Source => "reqvire:Source",
                ContractType::Constraint => "reqvire:Constraint",
                ContractType::Behavior => "reqvire:Behavior",
                ContractType::Specification => "reqvire:Specification",
                ContractType::State => "reqvire:State",
                ContractType::InputOutput => "reqvire:InputOutput",
            };
            vec![
                "owl:NamedIndividual",
                "reqvire:Element",
                "reqvire:Contract",
                subtype,
            ]
        }
        ElementType::File => {
            vec!["owl:NamedIndividual", "reqvire:Artifact", "reqvire:File"]
        }
        ElementType::Other(_) => {
            vec![
                "owl:NamedIndividual",
                "reqvire:Element",
                "reqvire:CustomElement",
            ]
        }
    }
}

pub(super) fn target_iri_for_link(
    link: &LinkType,
    registry: &GraphRegistry,
    artifacts: &mut BTreeSet<String>,
) -> Option<String> {
    match link {
        LinkType::Identifier(target_identifier) => registry
            .nodes
            .get(target_identifier)
            .map(|target| element_iri(&target.element)),
        LinkType::InternalPath(path) => {
            let value = path.to_string_lossy();
            let iri = artifact_iri("path", &value);
            artifacts.insert(format!(
                "{} a owl:NamedIndividual, reqvire:Artifact, reqvire:File ;\n  reqvire:filePath {} .\n\n",
                iri,
                turtle_string(&value)
            ));
            Some(iri)
        }
        LinkType::ExternalUrl(url) => {
            let iri = artifact_iri("url", url);
            artifacts.insert(format!(
                "{} a owl:NamedIndividual, reqvire:Artifact ;\n  reqvire:externalUrl {} .\n\n",
                iri,
                turtle_string(url)
            ));
            Some(iri)
        }
    }
}

pub(super) fn contract_bindings_target_iri(
    target: &ContractBindingTarget,
    registry: &GraphRegistry,
    artifacts: &mut BTreeSet<String>,
) -> Option<String> {
    match target {
        ContractBindingTarget::ElementIdentifier(target_identifier) => registry
            .nodes
            .get(target_identifier)
            .map(|target| element_iri(&target.element)),
        ContractBindingTarget::FilePath(path) => {
            let value = path.to_string_lossy();
            let iri = artifact_iri("path", &value);
            artifacts.insert(format!(
                "{} a owl:NamedIndividual, reqvire:Artifact, reqvire:File ;\n  reqvire:filePath {} .\n\n",
                iri,
                turtle_string(&value)
            ));
            Some(iri)
        }
    }
}

pub(super) fn artifact_iri(kind: &str, value: &str) -> String {
    format!("<urn:reqvire:artifact:{}:{}>", kind, escape_iri(value))
}

pub(super) fn model_relation_iri(
    source_iri: &str,
    relation_name: &str,
    target_iri: &str,
) -> String {
    let canonical = format!("{}|{}|{}", source_iri, relation_name, target_iri);
    format!("<urn:reqvire:model-relation:{}>", stable_hash(&canonical))
}

pub(super) fn turtle_string(value: &str) -> String {
    format!(
        "\"{}\"",
        value
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
    )
}

pub(super) fn escape_iri(value: &str) -> String {
    value
        .replace('\\', "%5C")
        .replace(' ', "%20")
        .replace('<', "%3C")
        .replace('>', "%3E")
        .replace('"', "%22")
        .replace('{', "%7B")
        .replace('}', "%7D")
        .replace('|', "%7C")
        .replace('^', "%5E")
        .replace('`', "%60")
}

pub(super) fn extract_external_ontology_sources(
    element: &Element,
) -> (Vec<ExternalOntologySource>, Vec<String>) {
    let (parsed_sources, diagnostics) =
        crate::parser::parse_external_ontology_sources(&element.content);
    let sources = parsed_sources
        .into_iter()
        .map(|source| ExternalOntologySource {
            owner_identifier: element.identifier.clone(),
            owner_name: element.name.clone(),
            prefix: source.prefix,
            namespace: source.namespace,
            resource: Some(source.resource),
            source: source.source,
            format: source.format,
            line_number: source.line_number,
            builtin: false,
        })
        .collect();

    (sources, diagnostics)
}

pub(super) fn builtin_external_ontology_sources() -> Vec<ExternalOntologySource> {
    crate::builtin_external_sources::EXTERNAL_ONTOLOGIES
        .iter()
        .map(|source| ExternalOntologySource {
            owner_identifier: format!("builtin:{}", source.id),
            owner_name: format!("Built-in {}", source.prefix),
            prefix: source.prefix.to_string(),
            namespace: source.namespace.to_string(),
            resource: Some(source.resource.to_string()),
            source: source.source.to_string(),
            format: source.format.to_string(),
            line_number: 0,
            builtin: true,
        })
        .collect()
}

pub(super) fn builtin_external_ontology_source_content(
    source: &ExternalOntologySource,
) -> Option<&'static str> {
    crate::builtin_external_sources::EXTERNAL_ONTOLOGIES
        .iter()
        .find(|builtin| builtin.source == source.source)
        .map(|builtin| builtin.content)
}

pub(super) fn builtin_external_ontology_block(
    source: &ExternalOntologySource,
    diagnostics: &mut Vec<SemanticDiagnostic>,
) -> Option<SemanticBlock> {
    let Some(content) = builtin_external_ontology_source_content(source) else {
        diagnostics.push(SemanticDiagnostic {
            source: source.owner_identifier.clone(),
            file_path: source.source.clone(),
            line_number: source.line_number,
            message: format!(
                "Built-in External Ontology '{}' source '{}' is not registered.",
                source.prefix, source.source
            ),
        });
        return None;
    };

    let format = match ExternalOntologyFormat::parse(&source.format) {
        Some(format) => format,
        None => {
            diagnostics.push(SemanticDiagnostic {
                source: source.owner_identifier.clone(),
                file_path: source.source.clone(),
                line_number: source.line_number,
                message: format!(
                    "Built-in External Ontology '{}' uses unsupported format '{}'.",
                    source.prefix, source.format
                ),
            });
            return None;
        }
    };

    let quads = match parse_external_ontology_block(content, format) {
        Ok(quads) => quads,
        Err(message) => {
            diagnostics.push(SemanticDiagnostic {
                source: source.owner_identifier.clone(),
                file_path: source.source.clone(),
                line_number: source.line_number,
                message: format!(
                    "Built-in External Ontology '{}' source '{}' failed to parse as {}: {}.",
                    source.prefix,
                    source.source,
                    format.display_name(),
                    message
                ),
            });
            return None;
        }
    };

    Some(SemanticBlock {
        kind: SemanticBlockKind::ExternalOntology,
        source: external_ontology_block_source(source),
        source_name: format!("Built-in external ontology {}", source.prefix),
        file_path: source.source.clone(),
        line_number: source.line_number,
        language: format.language().to_string(),
        external_materialization: None,
        content: content.to_string(),
        quads,
    })
}

pub(super) fn resolve_external_source_path(
    element: &Element,
    source: &str,
) -> Result<PathBuf, String> {
    if source.starts_with("http://") || source.starts_with("https://") {
        return Err("source must be a local path; network fetches are not supported".to_string());
    }

    let source_path = Path::new(source);
    if source_path.is_absolute() {
        return Ok(source_path.to_path_buf());
    }

    let git_root = crate::git_commands::get_git_root_dir()
        .or_else(|_| {
            std::env::current_dir().map_err(|error| ReqvireError::PathError(error.to_string()))
        })
        .map_err(|error| error.to_string())?;

    let root_relative = git_root.join(source_path);
    if root_relative.exists() {
        return Ok(root_relative);
    }

    Ok(git_root
        .join(
            Path::new(&element.file_path)
                .parent()
                .unwrap_or_else(|| Path::new("")),
        )
        .join(source_path))
}

pub(super) fn build_external_ontology_block(
    element: &Element,
    source: &ExternalOntologySource,
    diagnostics: &mut Vec<SemanticDiagnostic>,
) -> Option<SemanticBlock> {
    let format = match ExternalOntologyFormat::parse(&source.format) {
        Some(format) => format,
        None => {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: source.line_number,
                message: format!(
                    "External Ontology '{}' uses unsupported format '{}'. Supported formats: turtle, ttl, rdf, rdfxml, rdf+xml, jsonld.",
                    source.prefix, source.format
                ),
            });
            return None;
        }
    };

    let path = match resolve_external_source_path(element, &source.source) {
        Ok(path) => path,
        Err(message) => {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: source.line_number,
                message: format!(
                    "External Ontology '{}' source '{}' cannot be resolved: {}.",
                    source.prefix, source.source, message
                ),
            });
            return None;
        }
    };

    let content = match std::fs::read_to_string(&path) {
        Ok(content) => content,
        Err(error) => {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: source.line_number,
                message: format!(
                    "External Ontology '{}' source '{}' cannot be read: {}.",
                    source.prefix,
                    path.display(),
                    error
                ),
            });
            return None;
        }
    };

    if matches!(format, ExternalOntologyFormat::Turtle)
        && turtle_prefix_binding(&content, &source.prefix) != Some(source.namespace.clone())
    {
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: source.line_number,
            message: format!(
                "External Ontology '{}' source '{}' must explicitly declare `@prefix {}: <{}> .`.",
                source.prefix, source.source, source.prefix, source.namespace
            ),
        });
    }

    let quads = match parse_external_ontology_block(&content, format) {
        Ok(quads) => quads,
        Err(message) => {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: source.line_number,
                message: format!(
                    "External Ontology '{}' source '{}' failed to parse as {}: {}.",
                    source.prefix,
                    source.source,
                    format.display_name(),
                    message
                ),
            });
            return None;
        }
    };

    if let Some(resource) = &source.resource {
        if !has_ontology_declaration(&quads, resource) {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: source.line_number,
                message: format!(
                    "External Ontology '{}' source '{}' must declare <{}> a owl:Ontology.",
                    source.prefix, source.source, resource
                ),
            });
        }
    }

    if !graph_mentions_namespace(&quads, &source.namespace) {
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: source.line_number,
            message: format!(
                "External Ontology '{}' source '{}' does not declare or reference any term in namespace '{}'.",
                source.prefix, source.source, source.namespace
            ),
        });
    }

    Some(SemanticBlock {
        kind: SemanticBlockKind::ExternalOntology,
        source: external_ontology_block_source(source),
        source_name: format!("{} external ontology {}", element.name, source.prefix),
        file_path: path.to_string_lossy().to_string(),
        line_number: source.line_number,
        language: format.language().to_string(),
        external_materialization: None,
        content,
        quads,
    })
}

pub(super) fn external_ontology_block_source(source: &ExternalOntologySource) -> String {
    if source.builtin {
        format!("builtin:external-ontology-{}", source.prefix)
    } else {
        format!(
            "{}#external-ontology-{}",
            source.owner_identifier, source.prefix
        )
    }
}

pub(super) fn known_class_iris(
    declarations: &FxHashMap<String, Vec<OntologyTermDeclaration>>,
) -> BTreeSet<String> {
    declarations
        .iter()
        .filter(|(_, declarations)| {
            declarations
                .iter()
                .any(|declaration| declaration.role == OntologyTermRole::Class)
        })
        .map(|(iri, _)| iri.clone())
        .collect()
}

pub(super) fn graph_mentions_namespace(quads: &[Quad], namespace: &str) -> bool {
    quads.iter().any(|quad| {
        subject_iri(&quad.subject).is_some_and(|iri| iri.starts_with(namespace))
            || quad.predicate.as_str().starts_with(namespace)
            || term_iri(&quad.object).is_some_and(|iri| iri.starts_with(namespace))
    })
}

pub(super) fn concept_prefix_block(prefix: &str, namespace: &str) -> String {
    format!(
        "@prefix {}: <{}> .\n@prefix skos: <{}> .\n\n",
        prefix, namespace, SKOS_NS
    )
}

pub(super) fn turtle_literal(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_iri_percent_encodes_reserved_characters() {
        assert_eq!(escape_iri("a b<c>\""), "a%20b%3Cc%3E%22");
        assert_eq!(escape_iri(""), "");
        assert_eq!(escape_iri("\\"), "%5C");
        assert_eq!(escape_iri("a`b"), "a%60b");
        assert_eq!(escape_iri("{|}^"), "%7B%7C%7D%5E");
    }

    #[test]
    fn strip_turtle_prefix_declarations_removes_prefix_lines() {
        let input = "@prefix foo: <urn:x> .\nbody line one\nPREFIX bar: <urn:y> .\nbody line two\n";
        let stripped = strip_turtle_prefix_declarations(input);
        assert!(
            !stripped.contains("@prefix"),
            "expected @prefix lines removed, got: {}",
            stripped
        );
        assert!(
            !stripped.contains("PREFIX"),
            "expected PREFIX lines removed, got: {}",
            stripped
        );
        assert!(stripped.contains("body line one"));
        assert!(stripped.contains("body line two"));
    }

    #[test]
    fn strip_turtle_prefix_declarations_passes_through_empty_input() {
        assert_eq!(strip_turtle_prefix_declarations(""), "");
    }

    #[test]
    fn turtle_literal_escapes_special_characters() {
        assert_eq!(turtle_literal("a\"b\n"), "a\\\"b\\n");
        assert_eq!(turtle_literal("\\"), "\\\\");
        assert_eq!(turtle_literal("plain"), "plain");
        assert_eq!(turtle_literal("line1\r\nline2"), "line1\\r\\nline2");
    }
}
