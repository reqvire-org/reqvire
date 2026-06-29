use super::export::{
    build_external_ontology_block, build_model_context_turtle, builtin_external_ontology_block,
    builtin_external_ontology_sources, concept_prefix_block, element_iri_from_identifier,
    element_iri_value, element_type_classes, escape_iri, extract_external_ontology_sources,
    known_class_iris, quads_from_turtle, turtle_literal, turtle_string,
};
use super::prefixes::{
    canonical_ontology_prefix, ontology_document_iri, ontology_term_namespace, parse_turtle_block,
    resolve_ontology_base, resolve_ontology_prefix, turtle_prefix_binding,
    validate_turtle_language,
};
use super::vocabulary::{build_ontology_projection, ontology_term_role};
use super::*;
use crate::concept::concept_local_name;

pub(super) fn concept_reference_iri_value(
    registry: &GraphRegistry,
    source: &Element,
    reference: &ConceptReference,
) -> Option<String> {
    let target_id =
        crate::parser::normalize_concept_reference_target(&source.file_path, &reference.target)
            .ok()?;
    let target = registry.nodes.get(&target_id).map(|node| &node.element)?;
    if !target.element_type.is_concept() {
        return None;
    }
    registry.generated_concept_iri_for_element(target)
}

pub(super) fn build_ontology_document_declarations_turtle(
    declarations: &[OntologyDocumentDeclaration],
) -> String {
    if declarations.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    output.push_str(
        "# -----------------------------------------------------------------------------\n",
    );
    output.push_str("# Reqvire generated ontology document declarations\n\n");
    output.push_str("@prefix owl: <http://www.w3.org/2002/07/owl#> .\n");
    output.push_str("@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n");
    output.push_str("@prefix reqvire: <https://www.reqvire.org/ontology#> .\n\n");

    for declaration in declarations {
        output.push_str(&format!(
            "<{}> a owl:Ontology ;\n",
            escape_iri(&declaration.iri)
        ));
        output.push_str(&format!(
            "  rdfs:label {} ;\n",
            turtle_string(&declaration.ontology_prefix)
        ));
        for element_identifier in &declaration.element_identifiers {
            output.push_str(&format!(
                "  reqvire:ontologyElement {} ;\n",
                element_iri_from_identifier(element_identifier)
            ));
        }
        output.push_str(&format!(
            "  reqvire:ontologyBase {} ;\n",
            turtle_string(&declaration.ontology_base)
        ));
        output.push_str(&format!(
            "  reqvire:ontologyPrefix {} ;\n",
            turtle_string(&declaration.ontology_prefix)
        ));
        output.push_str(&format!(
            "  reqvire:termNamespace {}",
            turtle_string(&declaration.term_namespace)
        ));
        for import_iri in &declaration.imports {
            output.push_str(&format!(" ;\n  owl:imports <{}>", escape_iri(import_iri)));
        }
        output.push_str(" .\n\n");
    }

    output
}

pub(super) fn build_ontology_term_definitions_turtle(
    documents: &[OntologyDocumentDeclaration],
    declarations: &FxHashMap<String, Vec<OntologyTermDeclaration>>,
    blocks: &[SemanticBlock],
) -> String {
    let mut element_documents = FxHashMap::default();
    for document in documents {
        for element_identifier in &document.element_identifiers {
            element_documents.insert(
                element_identifier.as_str(),
                (document.iri.as_str(), document.term_namespace.as_str()),
            );
        }
    }

    let mut edges = BTreeSet::new();
    for (term_iri, term_declarations) in declarations {
        for declaration in term_declarations {
            if declaration.external {
                continue;
            }
            let Some((document_iri, term_namespace)) =
                element_documents.get(declaration.element_identifier.as_str())
            else {
                continue;
            };
            if term_iri == *document_iri || !term_iri.starts_with(*term_namespace) {
                continue;
            }
            edges.insert((term_iri.clone(), (*document_iri).to_string()));
        }
    }
    for (term_iri, document_iri) in authored_ontology_subject_definition_edges(documents, blocks) {
        edges.insert((term_iri, document_iri));
    }

    if edges.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    output.push_str(
        "# -----------------------------------------------------------------------------\n",
    );
    output.push_str("# Reqvire generated ontology term definition links\n\n");
    for (term_iri, document_iri) in edges {
        output.push_str(&format!(
            "<{}> rdfs:isDefinedBy <{}> .\n",
            escape_iri(&term_iri),
            escape_iri(&document_iri)
        ));
    }
    output.push('\n');
    output
}

pub(super) fn authored_ontology_subject_definition_edges(
    documents: &[OntologyDocumentDeclaration],
    blocks: &[SemanticBlock],
) -> BTreeSet<(String, String)> {
    let mut element_documents = FxHashMap::default();
    for document in documents {
        for element_identifier in &document.element_identifiers {
            element_documents.insert(
                element_identifier.as_str(),
                (document.iri.as_str(), document.term_namespace.as_str()),
            );
        }
    }

    let mut edges = BTreeSet::new();
    for block in blocks {
        if !matches!(block.kind, SemanticBlockKind::Ontology) {
            continue;
        }
        let Some((document_iri, term_namespace)) = element_documents.get(block.source.as_str())
        else {
            continue;
        };
        for quad in &block.quads {
            let Some(subject) = subject_iri(&quad.subject) else {
                continue;
            };
            if subject == *document_iri || !subject.starts_with(*term_namespace) {
                continue;
            }
            edges.insert((subject.to_string(), (*document_iri).to_string()));
        }
    }
    edges
}

pub fn build_semantic_index(registry: &GraphRegistry) -> SemanticIndex {
    let mut blocks = Vec::new();
    let mut external_blocks = Vec::new();
    let mut external_sources = Vec::new();
    let mut diagnostics = Vec::new();
    let ontology_documents = build_ontology_document_declarations(registry);
    let mut ontology_declarations: FxHashMap<String, Vec<OntologyTermDeclaration>> =
        FxHashMap::default();
    let mut shape_references = Vec::new();
    let mut external_prefixes: FxHashMap<String, String> = FxHashMap::default();
    let mut external_namespaces: FxHashMap<String, String> = FxHashMap::default();

    for source in builtin_external_ontology_sources() {
        external_prefixes.insert(source.prefix.clone(), source.namespace.clone());
        external_namespaces.insert(source.namespace.clone(), source.prefix.clone());
        if let Some(block) = builtin_external_ontology_block(&source, &mut diagnostics) {
            for declaration in ontology_term_declarations_from_quads_with_source(
                &source.owner_identifier,
                &block.quads,
                true,
            ) {
                ontology_declarations
                    .entry(declaration.iri.clone())
                    .or_default()
                    .push(declaration);
            }
            external_blocks.push(block);
        }
        external_sources.push(source);
    }

    for element in registry.get_all_elements() {
        let ontology =
            crate::parser::extract_single_fenced_subsection(&element.content, "Ontology");
        let shapes = crate::parser::extract_single_fenced_subsection(&element.content, "Shapes");
        let query = crate::parser::extract_single_fenced_subsection(&element.content, "Query");

        validate_semantic_sections(element, &ontology, &shapes, &query, &mut diagnostics);

        if element.element_type.is_ontology() {
            let canonical_prefix = canonical_ontology_prefix(registry, &element.identifier);
            if let Some(block) = build_block(
                element,
                SemanticBlockKind::Ontology,
                ontology.first(),
                "Ontology",
                canonical_prefix.as_ref(),
                &mut diagnostics,
            ) {
                validate_ontology_source_contract(
                    element,
                    canonical_prefix.as_ref(),
                    &block,
                    &mut diagnostics,
                );
                let known_class_iris = known_class_iris(&ontology_declarations);
                for declaration in
                    ontology_term_declarations_from_quads(element, &block.quads, &known_class_iris)
                {
                    ontology_declarations
                        .entry(declaration.iri.clone())
                        .or_default()
                        .push(declaration);
                }
                blocks.push(block);
            }

            let (sources, source_diagnostics) = extract_external_ontology_sources(element);
            for message in source_diagnostics {
                diagnostics.push(SemanticDiagnostic {
                    source: element.identifier.clone(),
                    file_path: element.file_path.clone(),
                    line_number: element.line_number,
                    message,
                });
            }
            for source in sources {
                if let Some(existing_namespace) = external_prefixes.get(&source.prefix) {
                    if existing_namespace != &source.namespace {
                        diagnostics.push(SemanticDiagnostic {
                            source: element.identifier.clone(),
                            file_path: element.file_path.clone(),
                            line_number: source.line_number,
                            message: format!(
                                "External Ontology prefix '{}' is already bound to '{}', but this section binds it to '{}'.",
                                source.prefix, existing_namespace, source.namespace
                            ),
                        });
                    }
                } else {
                    external_prefixes.insert(source.prefix.clone(), source.namespace.clone());
                }

                if let Some(existing_prefix) = external_namespaces.get(&source.namespace) {
                    if existing_prefix != &source.prefix {
                        diagnostics.push(SemanticDiagnostic {
                            source: element.identifier.clone(),
                            file_path: element.file_path.clone(),
                            line_number: source.line_number,
                            message: format!(
                                "External Ontology namespace '{}' is already bound to prefix '{}', but this section binds it to '{}'. Prefix aliases are not supported.",
                                source.namespace, existing_prefix, source.prefix
                            ),
                        });
                    }
                } else {
                    external_namespaces.insert(source.namespace.clone(), source.prefix.clone());
                }

                if let Some(block) =
                    build_external_ontology_block(element, &source, &mut diagnostics)
                {
                    for declaration in
                        external_ontology_term_declarations_from_quads(element, &block.quads)
                    {
                        ontology_declarations
                            .entry(declaration.iri.clone())
                            .or_default()
                            .push(declaration);
                    }
                    external_blocks.push(block);
                }
                external_sources.push(source);
            }
            continue;
        }

        if element.element_type.is_semantic_contract() {
            if let Some(block) = build_block(
                element,
                SemanticBlockKind::Shapes,
                shapes.first(),
                "Shapes",
                None,
                &mut diagnostics,
            ) {
                let shacl_registry = shacl::ShaclRegistry::parse(&block.quads);
                for message in shacl_registry.diagnostics_as_messages() {
                    diagnostics.push(SemanticDiagnostic {
                        source: element.identifier.clone(),
                        file_path: element.file_path.clone(),
                        line_number: block.line_number,
                        message: format!("Shapes SHACL sanity validation failed: {}", message),
                    });
                }

                shape_references.extend(shape_iri_references_from_quads(element, &block.quads));
                blocks.push(block);
            }
            continue;
        }

        if element.element_type.is_concept_family() {
            if let Some(block) = build_generated_concept_block(registry, element, &mut diagnostics)
            {
                blocks.push(block);
            }
        }
    }

    let ontology_blocks = blocks
        .iter()
        .filter(|block| matches!(block.kind, SemanticBlockKind::Ontology))
        .count();
    let shape_blocks = blocks
        .iter()
        .filter(|block| matches!(block.kind, SemanticBlockKind::Shapes))
        .count();
    let total_quads = blocks.iter().map(|block| block.quads.len()).sum();
    let ontology_projection = build_ontology_projection(registry, &blocks);

    let mut index = SemanticIndex {
        summary: SemanticIndexSummary {
            ontology_blocks,
            shape_blocks,
            total_blocks: blocks.len(),
            total_quads,
        },
        blocks,
        external_blocks,
        external_sources,
        diagnostics,
        ontology_documents,
        ontology_declarations,
        shape_references,
        ontology_projection,
        model_context: ModelContextGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        },
        model_context_turtle: String::new(),
    };
    index.model_context = build_model_context_graph(registry, &index);
    index.model_context_turtle = build_model_context_turtle(registry, &index);
    index
}

pub(super) fn build_generated_concept_block(
    registry: &GraphRegistry,
    element: &Element,
    diagnostics: &mut Vec<SemanticDiagnostic>,
) -> Option<SemanticBlock> {
    let content = match generated_concept_turtle(registry, element) {
        Ok(Some(content)) => content,
        Ok(None) => return None,
        Err(message) => {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: element.line_number,
                message,
            });
            return None;
        }
    };
    let quads = match quads_from_turtle(&content, "Generated native concept RDF") {
        Ok(quads) => quads,
        Err(error) => {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: element.line_number,
                message: error.to_string(),
            });
            return None;
        }
    };
    Some(SemanticBlock {
        kind: SemanticBlockKind::Concepts,
        source: element.identifier.clone(),
        source_name: element.name.clone(),
        file_path: element.file_path.clone(),
        line_number: element.line_number,
        language: "turtle".to_string(),
        external_materialization: None,
        content,
        quads,
    })
}

pub(super) fn generated_concept_turtle(
    registry: &GraphRegistry,
    element: &Element,
) -> Result<Option<String>, String> {
    if element.element_type.is_concept_scheme() {
        let Some(scheme) = &element.concept_scheme else {
            return Ok(None);
        };
        let Some((prefix, namespace)) = concept_namespace_context(registry, element) else {
            return Err(format!(
                "Concept scheme '{}' must define concept_base and concept_prefix metadata.",
                element.name
            ));
        };
        let subject = concept_curie(&prefix, element);
        let mut turtle = concept_prefix_block(&prefix, &namespace);
        turtle.push_str(&format!("{} a skos:ConceptScheme ;\n", subject));
        turtle.push_str(&format!(
            "  skos:prefLabel \"{}\"",
            turtle_literal(&scheme.pref_label)
        ));
        if let Some(definition) = &scheme.definition {
            turtle.push_str(&format!(
                " ;\n  skos:definition \"{}\"",
                turtle_literal(definition)
            ));
        }
        for top in &scheme.top_concepts {
            turtle.push_str(&format!(
                " ;\n  skos:hasTopConcept {}",
                concept_link_object(registry, &prefix, &top.target, &top.label)
            ));
        }
        turtle.push_str(" .\n");
        return Ok(Some(turtle));
    }

    if element.element_type.is_concept() {
        let Some(concept) = &element.concept else {
            return Ok(None);
        };
        let Some((prefix, namespace)) = concept_namespace_context(registry, element) else {
            return Err(format!(
                "Concept '{}' must derive scheme and namespace context from a concept-scheme element.",
                element.name
            ));
        };
        let Some(scheme) = concept_scheme_context(registry, element) else {
            return Err(format!(
                "Concept '{}' must derive from a concept-scheme or another concept with scheme context.",
                element.name
            ));
        };
        let subject = concept_curie(&prefix, element);
        let mut turtle = concept_prefix_block(&prefix, &namespace);
        turtle.push_str(&format!("{} a skos:Concept ;\n", subject));
        turtle.push_str(&format!(
            "  skos:inScheme {}",
            concept_curie(&prefix, scheme)
        ));
        turtle.push_str(&format!(
            " ;\n  skos:prefLabel \"{}\"",
            turtle_literal(&concept.pref_label)
        ));
        for label in &concept.labels {
            if matches!(label.kind.as_str(), "altLabel" | "hiddenLabel") {
                turtle.push_str(&format!(
                    " ;\n  skos:{} \"{}\"",
                    label.kind,
                    turtle_literal(&label.value)
                ));
            }
        }
        if let Some(definition) = &concept.definition {
            turtle.push_str(&format!(
                " ;\n  skos:definition \"{}\"",
                turtle_literal(definition)
            ));
        }
        if let Some(scope_note) = &concept.scope_note {
            turtle.push_str(&format!(
                " ;\n  skos:scopeNote \"{}\"",
                turtle_literal(scope_note)
            ));
        }
        for example in &concept.examples {
            turtle.push_str(&format!(
                " ;\n  skos:example \"{}\"",
                turtle_literal(&example.value)
            ));
        }
        for (predicate, object) in normalized_concept_relation_objects(registry, element, &prefix) {
            turtle.push_str(&format!(" ;\n  skos:{} {}", predicate, object));
        }
        turtle.push_str(" .\n");
        for (subject, predicate, object) in
            external_symmetric_concept_relation_triples(element, &prefix)
        {
            turtle.push_str(&format!("{} skos:{} {} .\n", subject, predicate, object));
        }
        return Ok(Some(turtle));
    }

    Ok(None)
}

pub(super) fn concept_namespace_context(
    registry: &GraphRegistry,
    element: &Element,
) -> Option<(String, String)> {
    let scheme = if element.element_type.is_concept_scheme() {
        Some(element)
    } else {
        concept_scheme_context(registry, element)
    }?;
    let base = scheme
        .metadata
        .get("concept_base")
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())?;
    let prefix = scheme
        .metadata
        .get("concept_prefix")
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())?;
    Some((prefix.to_string(), ontology_term_namespace(base)))
}

pub(super) fn concept_scheme_context<'a>(
    registry: &'a GraphRegistry,
    element: &'a Element,
) -> Option<&'a Element> {
    registry.concept_scheme_context_element(&element.identifier)
}

pub(super) fn normalized_concept_relation_objects(
    registry: &GraphRegistry,
    element: &Element,
    prefix: &str,
) -> BTreeSet<(String, String)> {
    let mut output = BTreeSet::new();
    let current_id = element.identifier.as_str();

    for candidate in registry.get_all_elements() {
        let Some(concept) = candidate.concept.as_ref() else {
            continue;
        };
        let candidate_id = candidate.identifier.as_str();

        for link in &concept.broader {
            if candidate_id == current_id {
                output.insert((
                    "broader".to_string(),
                    concept_link_object(registry, prefix, &link.target, &link.label),
                ));
            }
            if concept_link_target_element(registry, &link.target)
                .is_some_and(|target| target.identifier.as_str() == current_id)
            {
                output.insert(("narrower".to_string(), concept_curie(prefix, candidate)));
            }
        }

        for link in &concept.narrower {
            if candidate_id == current_id {
                output.insert((
                    "narrower".to_string(),
                    concept_link_object(registry, prefix, &link.target, &link.label),
                ));
            }
            if concept_link_target_element(registry, &link.target)
                .is_some_and(|target| target.identifier.as_str() == current_id)
            {
                output.insert(("broader".to_string(), concept_curie(prefix, candidate)));
            }
        }

        append_symmetric_concept_relation_objects(
            registry,
            &mut output,
            current_id,
            candidate,
            "related",
            &concept.related,
            prefix,
        );
        append_symmetric_concept_relation_objects(
            registry,
            &mut output,
            current_id,
            candidate,
            "exactMatch",
            &concept.exact_match,
            prefix,
        );
        append_symmetric_concept_relation_objects(
            registry,
            &mut output,
            current_id,
            candidate,
            "closeMatch",
            &concept.close_match,
            prefix,
        );
    }

    output
}

pub(super) fn append_symmetric_concept_relation_objects(
    registry: &GraphRegistry,
    output: &mut BTreeSet<(String, String)>,
    current_id: &str,
    candidate: &Element,
    predicate: &str,
    links: &[crate::element::ConceptLink],
    prefix: &str,
) {
    let candidate_id = candidate.identifier.as_str();
    for link in links {
        if candidate_id == current_id {
            output.insert((
                predicate.to_string(),
                concept_link_object(registry, prefix, &link.target, &link.label),
            ));
        }
        if concept_link_target_element(registry, &link.target)
            .is_some_and(|target| target.identifier.as_str() == current_id)
        {
            output.insert((predicate.to_string(), concept_curie(prefix, candidate)));
        }
    }
}

pub(super) fn external_symmetric_concept_relation_triples(
    element: &Element,
    prefix: &str,
) -> BTreeSet<(String, String, String)> {
    let mut triples = BTreeSet::new();
    let Some(concept) = element.concept.as_ref() else {
        return triples;
    };
    let object = concept_curie(prefix, element);
    for (predicate, links) in [
        ("exactMatch", &concept.exact_match),
        ("closeMatch", &concept.close_match),
    ] {
        for link in links {
            if link.target.starts_with("http://") || link.target.starts_with("https://") {
                triples.insert((
                    format!("<{}>", link.target),
                    predicate.to_string(),
                    object.clone(),
                ));
            }
        }
    }
    triples
}

pub(super) fn concept_link_target_element<'a>(
    registry: &'a GraphRegistry,
    target: &str,
) -> Option<&'a Element> {
    registry
        .resolve_concept_element_id(target)
        .and_then(|target_id| registry.nodes.get(&target_id).map(|node| &node.element))
}

pub(super) fn concept_link_object(
    registry: &GraphRegistry,
    prefix: &str,
    target: &str,
    label: &str,
) -> String {
    if target.starts_with("http://") || target.starts_with("https://") {
        return format!("<{}>", target);
    }
    if let Some(element) = concept_link_target_element(registry, target) {
        return concept_curie(prefix, element);
    }
    format!("{}:{}", prefix, concept_local_name(label))
}

pub(super) fn concept_curie(prefix: &str, element: &Element) -> String {
    format!("{}:{}", prefix, concept_local_name(&element.name))
}

pub(super) fn build_model_context_graph(
    registry: &GraphRegistry,
    index: &SemanticIndex,
) -> ModelContextGraph {
    let mut nodes: Vec<_> = registry
        .nodes
        .values()
        .map(|node| {
            let element = &node.element;
            ModelContextNode {
                id: element_iri_value(element),
                label: element.name.clone(),
                identifier: element.identifier.clone(),
                element_type: element.element_type.to_metadata_string(),
                rdf_types: element_type_classes(&element.element_type)
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                file_path: element.file_path.clone(),
                line_number: element.line_number,
            }
        })
        .collect();
    nodes.sort_by(|a, b| a.id.cmp(&b.id));

    let mut edges = BTreeSet::new();
    for node in registry.nodes.values() {
        let source = element_iri_value(&node.element);
        for relation in &node.element.relations {
            let Some(target) = model_context_relation_target_iri(relation, registry) else {
                continue;
            };
            let (source, target, label) = canonical_model_context_relation_edge(
                &source,
                &target,
                relation.relation_type.name,
            );
            edges.insert(ModelContextEdge {
                source,
                target,
                label,
            });
        }
    }

    for node in registry.nodes.values() {
        let source = element_iri_value(&node.element);
        for reference in &node.element.concept_references {
            let Some(target) = concept_reference_iri_value(registry, &node.element, reference)
            else {
                continue;
            };
            edges.insert(ModelContextEdge {
                source: source.clone(),
                target,
                label: "referencesTerm".to_string(),
            });
        }
    }

    for declarations in index.ontology_declarations.values() {
        for declaration in declarations {
            let Some(node) = registry.nodes.get(&declaration.element_identifier) else {
                continue;
            };
            edges.insert(ModelContextEdge {
                source: element_iri_value(&node.element),
                target: declaration.iri.clone(),
                label: "declaresTerm".to_string(),
            });
        }
    }

    for reference in &index.shape_references {
        let Some(node) = registry.nodes.get(&reference.element_identifier) else {
            continue;
        };
        edges.insert(ModelContextEdge {
            source: element_iri_value(&node.element),
            target: reference.iri.clone(),
            label: "referencesTerm".to_string(),
        });
    }

    ModelContextGraph {
        nodes,
        edges: edges.into_iter().collect(),
    }
}

pub(super) fn model_context_relation_target_iri(
    relation: &crate::relation::Relation,
    registry: &GraphRegistry,
) -> Option<String> {
    if let Some(target_id) = relation.target.element_id.as_deref() {
        if let Some(target) = registry.nodes.get(target_id) {
            return Some(element_iri_value(&target.element));
        }
    }
    match &relation.target.link {
        LinkType::Identifier(target_id) => registry
            .nodes
            .get(target_id)
            .map(|target| element_iri_value(&target.element)),
        _ => None,
    }
}

pub(super) fn canonical_model_context_relation_edge(
    source: &str,
    target: &str,
    relation_name: &str,
) -> (String, String, String) {
    relation::canonical_model_traversal_edge(source, target, relation_name)
}

pub(super) fn build_ontology_document_declarations(
    registry: &GraphRegistry,
) -> Vec<OntologyDocumentDeclaration> {
    let mut memo = FxHashMap::default();
    let mut prefix_memo = FxHashMap::default();
    let mut declarations: BTreeMap<String, OntologyDocumentAccumulator> = BTreeMap::new();
    let mut ontology_ids: Vec<String> = registry
        .nodes
        .values()
        .filter(|node| node.element.element_type.is_ontology())
        .map(|node| node.element.identifier.clone())
        .collect();
    ontology_ids.sort();

    for ontology_id in ontology_ids {
        let Some(node) = registry.nodes.get(&ontology_id) else {
            continue;
        };
        let Some(ontology_base) = resolve_ontology_base(registry, &ontology_id, &mut memo) else {
            continue;
        };
        let Some(ontology_prefix) =
            resolve_ontology_prefix(registry, &ontology_id, &mut prefix_memo)
        else {
            continue;
        };
        let term_namespace = ontology_term_namespace(&ontology_base);

        let declaration = declarations
            .entry(ontology_base.clone())
            .or_insert_with(|| OntologyDocumentAccumulator {
                ontology_base: ontology_base.clone(),
                ontology_prefix: ontology_prefix.clone(),
                term_namespace: term_namespace.clone(),
                element_identifiers: BTreeSet::new(),
                element_names: BTreeSet::new(),
                imports: BTreeSet::new(),
            });
        declaration
            .element_identifiers
            .insert(node.element.identifier.clone());
        declaration.element_names.insert(node.element.name.clone());
    }

    declarations
        .into_values()
        .map(|declaration| OntologyDocumentDeclaration {
            iri: ontology_document_iri(&declaration.ontology_base),
            ontology_base: declaration.ontology_base,
            ontology_prefix: declaration.ontology_prefix,
            term_namespace: declaration.term_namespace,
            element_identifiers: declaration.element_identifiers.into_iter().collect(),
            element_names: declaration.element_names.into_iter().collect(),
            imports: declaration.imports.into_iter().collect(),
        })
        .collect()
}

pub(super) fn validate_semantic_sections(
    element: &Element,
    ontology: &[FencedBlock],
    shapes: &[FencedBlock],
    query: &[FencedBlock],
    diagnostics: &mut Vec<SemanticDiagnostic>,
) {
    let has_query_section = crate::parser::has_subsection(&element.content, "Query");
    let has_external_ontology_section =
        crate::parser::has_subsection(&element.content, "External Ontology");
    let query_line_number = query
        .first()
        .map(|block| block.line_number)
        .unwrap_or(element.line_number);

    if element.element_type.is_ontology() {
        if ontology.len() != 1 {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: element.line_number,
                message: format!(
                    "Ontology element '{}' must contain exactly one #### Ontology fenced Turtle block.",
                    element.name
                ),
            });
        }
        if !shapes.is_empty() {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: shapes[0].line_number,
                message: format!(
                    "Ontology element '{}' must not contain a #### Shapes section. SHACL profiles belong in semantic-contract elements.",
                    element.name
                ),
            });
        }
    } else if element.element_type.is_semantic_contract() {
        if !ontology.is_empty() {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: ontology[0].line_number,
                message: format!(
                    "Semantic contract '{}' must not contain a #### Ontology section. Semantic contracts are shapes-only profiles over reachable ontology elements.",
                    element.name
                ),
            });
        }
        if shapes.len() != 1 {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: element.line_number,
                message: format!(
                    "Semantic contract '{}' must contain exactly one #### Shapes fenced Turtle block.",
                    element.name
                ),
            });
        }
    } else {
        if !ontology.is_empty() {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: ontology[0].line_number,
                message: format!(
                    "Element '{}' is type '{}' and must not contain a #### Ontology section. Use type: ontology.",
                    element.name,
                    element.element_type.as_str()
                ),
            });
        }
        if !shapes.is_empty() {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: shapes[0].line_number,
                message: format!(
                    "Element '{}' is type '{}' and must not contain a #### Shapes section. Use type: semantic-contract.",
                    element.name,
                    element.element_type.as_str()
                ),
            });
        }
    }

    if has_external_ontology_section && !element.element_type.is_ontology() {
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: element.line_number,
            message: format!(
                "Element '{}' is type '{}' and must not contain a #### External Ontology section. External ontology sources belong on ontology elements.",
                element.name,
                element.element_type.as_str()
            ),
        });
    }

    if has_query_section {
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: query_line_number,
            message: format!(
                "Element '{}' is type '{}' and must not contain a #### Query section. No Reqvire element type currently supports this reserved subsection.",
                element.name,
                element.element_type.as_str()
            ),
        });
    }

    if ontology.len() > 1 {
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: ontology[1].line_number,
            message: format!(
                "Element '{}' must contain at most one #### Ontology fenced Turtle block.",
                element.name
            ),
        });
    }

    if shapes.len() > 1 {
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: shapes[1].line_number,
            message: format!(
                "Element '{}' must contain at most one #### Shapes fenced Turtle block.",
                element.name
            ),
        });
    }

    if query.len() > 1 {
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: query[1].line_number,
            message: format!(
                "Element '{}' must contain at most one #### Query fenced block.",
                element.name
            ),
        });
    }
}

pub(super) fn build_block(
    element: &Element,
    kind: SemanticBlockKind,
    block: Option<&FencedBlock>,
    section_name: &str,
    canonical_prefix: Option<&CanonicalOntologyPrefix>,
    diagnostics: &mut Vec<SemanticDiagnostic>,
) -> Option<SemanticBlock> {
    let block = block?;
    if !validate_turtle_language(&block.language) {
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: block.line_number,
            message: format!(
                "{} fenced block must use language tag 'turtle' or 'ttl'.",
                section_name
            ),
        });
        return None;
    }

    let parse_content = match canonical_turtle_content(&block.content, canonical_prefix) {
        Ok(content) => content,
        Err(message) => {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: block.line_number,
                message,
            });
            return None;
        }
    };

    match parse_turtle_block(&parse_content) {
        Ok(quads) => Some(SemanticBlock {
            kind,
            source: element.identifier.clone(),
            source_name: element.name.clone(),
            file_path: element.file_path.clone(),
            line_number: block.line_number,
            language: block.language.clone(),
            external_materialization: None,
            content: block.content.clone(),
            quads,
        }),
        Err(message) => {
            diagnostics.push(SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: block.line_number,
                message: format!("{} Turtle validation failed: {}", section_name, message),
            });
            None
        }
    }
}

pub(super) fn canonical_turtle_content(
    content: &str,
    canonical_prefix: Option<&CanonicalOntologyPrefix>,
) -> Result<String, String> {
    let Some(canonical_prefix) = canonical_prefix else {
        return Ok(content.to_string());
    };

    if let Some(namespace) = turtle_prefix_binding(content, &canonical_prefix.prefix) {
        if namespace == canonical_prefix.namespace {
            return Ok(content.to_string());
        }
        return Err(format!(
            "Ontology Turtle prefix '{}' maps to '{}', but inherited ontology metadata requires '{}'.",
            canonical_prefix.prefix, namespace, canonical_prefix.namespace
        ));
    }

    Err(format!(
        "Ontology Turtle block must explicitly declare prefix '{}' as '{}'.",
        canonical_prefix.prefix, canonical_prefix.namespace
    ))
}

pub(super) fn validate_ontology_source_contract(
    element: &Element,
    canonical_prefix: Option<&CanonicalOntologyPrefix>,
    block: &SemanticBlock,
    diagnostics: &mut Vec<SemanticDiagnostic>,
) {
    let Some(context) = canonical_prefix else {
        return;
    };

    if context.explicit_boundary
        && !has_ontology_declaration(&block.quads, &context.ontology_document_iri)
    {
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: block.line_number,
            message: format!(
                "Ontology boundary element '{}' with ontology_base '{}' must explicitly declare <{}> a owl:Ontology.",
                element.name, context.ontology_base, context.ontology_document_iri
            ),
        });
    }

    for import_iri in &context.required_imports {
        if has_ontology_import(&block.quads, &context.ontology_document_iri, import_iri) {
            continue;
        }
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: block.line_number,
            message: format!(
                "Ontology element '{}' has a derivedFrom relation crossing ontology boundaries and must explicitly declare <{}> owl:imports <{}>.",
                element.name, context.ontology_document_iri, import_iri
            ),
        });
    }

    let authored_terms = authored_ontology_subjects(
        &block.quads,
        &context.ontology_document_iri,
        &context.namespace,
    );
    for quad in &block.quads {
        let Some(subject) = subject_iri(&quad.subject) else {
            continue;
        };
        if quad.predicate.as_str() != RDFS_IS_DEFINED_BY || !authored_terms.contains(subject) {
            continue;
        }
        if term_iri(&quad.object) == Some(&context.ontology_document_iri) {
            continue;
        }
        diagnostics.push(SemanticDiagnostic {
            source: element.identifier.clone(),
            file_path: element.file_path.clone(),
            line_number: block.line_number,
            message: format!(
                "Ontology term <{}> is authored in ontology_base '{}' but has conflicting rdfs:isDefinedBy target. Authored Reqvire ontology terms must be defined by <{}>.",
                subject, context.ontology_base, context.ontology_document_iri
            ),
        });
    }
}

pub(super) fn authored_ontology_subjects(
    quads: &[Quad],
    ontology_document_iri: &str,
    term_namespace: &str,
) -> BTreeSet<String> {
    quads
        .iter()
        .filter_map(|quad| subject_iri(&quad.subject))
        .filter(|subject| *subject != ontology_document_iri && subject.starts_with(term_namespace))
        .map(ToString::to_string)
        .collect()
}

pub(super) fn has_ontology_declaration(quads: &[Quad], ontology_iri: &str) -> bool {
    quads.iter().any(|quad| {
        subject_iri(&quad.subject) == Some(ontology_iri)
            && quad.predicate.as_str() == RDF_TYPE
            && term_iri(&quad.object) == Some(OWL_ONTOLOGY)
    })
}

pub(super) fn has_ontology_import(quads: &[Quad], ontology_iri: &str, import_iri: &str) -> bool {
    quads.iter().any(|quad| {
        subject_iri(&quad.subject) == Some(ontology_iri)
            && quad.predicate.as_str() == OWL_IMPORTS
            && term_iri(&quad.object) == Some(import_iri)
    })
}

pub(super) fn ontology_term_declarations_from_quads(
    element: &Element,
    quads: &[Quad],
    known_class_iris: &BTreeSet<String>,
) -> Vec<OntologyTermDeclaration> {
    collect_ontology_term_declarations(&element.identifier, quads, false, known_class_iris)
}

pub(super) fn external_ontology_term_declarations_from_quads(
    element: &Element,
    quads: &[Quad],
) -> Vec<OntologyTermDeclaration> {
    collect_ontology_term_declarations(&element.identifier, quads, true, &BTreeSet::new())
}

pub(super) fn ontology_term_declarations_from_quads_with_source(
    source_identifier: &str,
    quads: &[Quad],
    external: bool,
) -> Vec<OntologyTermDeclaration> {
    collect_ontology_term_declarations(source_identifier, quads, external, &BTreeSet::new())
}

pub(super) fn collect_ontology_term_declarations(
    source_identifier: &str,
    quads: &[Quad],
    external: bool,
    known_class_iris: &BTreeSet<String>,
) -> Vec<OntologyTermDeclaration> {
    let mut declarations = Vec::new();
    let mut seen = BTreeSet::new();
    let mut class_iris = known_class_iris.clone();

    for quad in quads
        .iter()
        .filter(|quad| quad.predicate.as_str() == RDF_TYPE)
    {
        let Some(iri) = subject_iri(&quad.subject) else {
            continue;
        };
        let Some(type_iri) = term_iri(&quad.object) else {
            continue;
        };
        let Some(role) = ontology_term_role(type_iri) else {
            continue;
        };
        if role == OntologyTermRole::Class {
            class_iris.insert(iri.to_string());
        }
        push_ontology_term_declaration(
            &mut declarations,
            &mut seen,
            source_identifier,
            iri,
            role,
            external,
        );
    }

    for quad in quads
        .iter()
        .filter(|quad| quad.predicate.as_str() == RDF_TYPE)
    {
        let Some(iri) = subject_iri(&quad.subject) else {
            continue;
        };
        let Some(type_iri) = term_iri(&quad.object) else {
            continue;
        };
        if ontology_term_role(type_iri).is_some() || !class_iris.contains(type_iri) {
            continue;
        }
        push_ontology_term_declaration(
            &mut declarations,
            &mut seen,
            source_identifier,
            iri,
            OntologyTermRole::NamedIndividual,
            external,
        );
    }

    declarations
}

pub(super) fn push_ontology_term_declaration(
    declarations: &mut Vec<OntologyTermDeclaration>,
    seen: &mut BTreeSet<(String, OntologyTermRole)>,
    source_identifier: &str,
    iri: &str,
    role: OntologyTermRole,
    external: bool,
) {
    if !seen.insert((iri.to_string(), role)) {
        return;
    }
    declarations.push(OntologyTermDeclaration {
        iri: iri.to_string(),
        role,
        element_identifier: source_identifier.to_string(),
        external,
        materialized_in_used_subset: false,
    });
}

pub(super) fn shape_iri_references_from_quads(
    element: &Element,
    quads: &[Quad],
) -> Vec<ShapeIriReference> {
    let mut references = BTreeSet::new();
    for reference in ontology::extract_shape_references(quads) {
        let iri = reference.iri.as_str();
        let kind = reference.predicate_label();
        references.insert(ShapeIriReference {
            iri: iri.to_string(),
            kind,
            element_identifier: element.identifier.clone(),
        });
    }

    references.into_iter().collect()
}
