use super::*;

pub(crate) fn parse_turtle_prefix_declarations(content: &str) -> Vec<(String, String)> {
    let mut prefixes = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix("@prefix ") else {
            continue;
        };
        let mut parts = rest.split_whitespace();
        let Some(prefix_token) = parts.next() else {
            continue;
        };
        let Some(iri_token) = parts.next() else {
            continue;
        };
        let Some(prefix) = prefix_token.strip_suffix(':') else {
            continue;
        };
        let Some(iri) = iri_token
            .strip_prefix('<')
            .and_then(|value| value.strip_suffix('>'))
        else {
            continue;
        };
        prefixes.push((prefix.to_string(), iri.to_string()));
    }
    prefixes
}

pub(super) fn canonical_ontology_prefix(
    registry: &GraphRegistry,
    ontology_id: &str,
) -> Option<CanonicalOntologyPrefix> {
    let mut base_memo = FxHashMap::default();
    let mut prefix_memo = FxHashMap::default();
    let ontology_base = resolve_ontology_base(registry, ontology_id, &mut base_memo)?;
    let prefix = resolve_ontology_prefix(registry, ontology_id, &mut prefix_memo)?;
    let explicit_boundary = registry
        .nodes
        .get(ontology_id)
        .and_then(|node| node.element.metadata.get("ontology_base"))
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false);
    let ontology_document_iri = ontology_document_iri(&ontology_base);
    let required_imports = required_ontology_imports(registry, ontology_id, &ontology_base);
    Some(CanonicalOntologyPrefix {
        prefix,
        namespace: ontology_term_namespace(&ontology_base),
        ontology_base,
        ontology_document_iri,
        explicit_boundary,
        required_imports,
    })
}

pub(super) fn required_ontology_imports(
    registry: &GraphRegistry,
    ontology_id: &str,
    ontology_base: &str,
) -> Vec<String> {
    let Some(node) = registry.nodes.get(ontology_id) else {
        return Vec::new();
    };
    let mut memo = FxHashMap::default();
    let mut imports = BTreeSet::new();

    for relation in node
        .element
        .relations
        .iter()
        .filter(|relation| relation.relation_type.name == "derivedFrom")
    {
        let LinkType::Identifier(target_id) = &relation.target.link else {
            continue;
        };
        let Some(target) = registry.nodes.get(target_id) else {
            continue;
        };
        if !target.element.element_type.is_ontology() {
            continue;
        }
        let Some(target_base) = resolve_ontology_base(registry, target_id, &mut memo) else {
            continue;
        };
        if target_base != ontology_base {
            imports.insert(ontology_document_iri(&target_base));
        }
    }

    imports.into_iter().collect()
}

pub(super) fn resolve_ontology_base(
    registry: &GraphRegistry,
    ontology_id: &str,
    memo: &mut FxHashMap<String, Option<String>>,
) -> Option<String> {
    resolve_inherited_ontology_metadata(registry, ontology_id, "ontology_base", memo)
}

pub(super) fn resolve_ontology_prefix(
    registry: &GraphRegistry,
    ontology_id: &str,
    memo: &mut FxHashMap<String, Option<String>>,
) -> Option<String> {
    resolve_inherited_ontology_metadata(registry, ontology_id, "ontology_prefix", memo)
}

fn resolve_inherited_ontology_metadata(
    registry: &GraphRegistry,
    ontology_id: &str,
    metadata_key: &str,
    memo: &mut FxHashMap<String, Option<String>>,
) -> Option<String> {
    if let Some(cached) = memo.get(ontology_id) {
        return cached.clone();
    }
    memo.insert(ontology_id.to_string(), None);

    let node = registry.nodes.get(ontology_id)?;
    if let Some(value) = node
        .element
        .metadata
        .get(metadata_key)
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
    {
        let value = value.to_string();
        memo.insert(ontology_id.to_string(), Some(value.clone()));
        return Some(value);
    }

    let mut parent_ids: Vec<String> = node
        .element
        .relations
        .iter()
        .filter(|relation| relation.relation_type.name == "derivedFrom")
        .filter_map(|relation| match &relation.target.link {
            LinkType::Identifier(target_id) => registry
                .nodes
                .get(target_id)
                .filter(|target| target.element.element_type.is_ontology())
                .map(|_| target_id.clone()),
            _ => None,
        })
        .collect();
    parent_ids.sort();

    for parent_id in parent_ids {
        if let Some(value) =
            resolve_inherited_ontology_metadata(registry, &parent_id, metadata_key, memo)
        {
            memo.insert(ontology_id.to_string(), Some(value.clone()));
            return Some(value);
        }
    }

    None
}

pub(super) fn ontology_document_iri(ontology_base: &str) -> String {
    ontology_base.trim_end_matches('/').to_string()
}

pub(super) fn ontology_term_namespace(ontology_base: &str) -> String {
    format!("{}#", ontology_base.trim_end_matches('#'))
}

pub(super) fn turtle_prefix_binding(content: &str, expected_prefix: &str) -> Option<String> {
    for line in content.lines() {
        let Some((prefix, namespace)) = parse_turtle_prefix_line(line.trim()) else {
            continue;
        };
        if prefix == expected_prefix {
            return Some(namespace);
        }
    }
    None
}

pub(crate) fn parse_turtle_prefix_line(line: &str) -> Option<(String, String)> {
    let rest = line
        .strip_prefix("@prefix ")
        .or_else(|| line.strip_prefix("@PREFIX "))
        .or_else(|| line.strip_prefix("PREFIX "))?;
    let rest = rest.trim_start();
    let (prefix, rest) = rest.split_once(':')?;
    let rest = rest.trim_start();
    let namespace_start = rest.find('<')? + 1;
    let namespace_end = rest[namespace_start..].find('>')? + namespace_start;
    Some((
        prefix.trim().to_string(),
        rest[namespace_start..namespace_end].to_string(),
    ))
}

pub(super) fn validate_turtle_language(language: &str) -> bool {
    matches!(language.to_ascii_lowercase().as_str(), "turtle" | "ttl")
}

pub(super) fn parse_turtle_block(content: &str) -> Result<Vec<Quad>, String> {
    let mut graph = Vec::new();

    for parsed in RdfParser::from_format(RdfFormat::Turtle).for_reader(content.as_bytes()) {
        graph.push(parsed.map_err(|error| error.to_string())?);
    }

    if graph.is_empty() {
        return Err("Turtle block has no RDF statements".to_string());
    }

    Ok(graph)
}

pub(super) fn parse_external_ontology_block(
    content: &str,
    format: ExternalOntologyFormat,
) -> Result<Vec<Quad>, String> {
    let mut graph = Vec::new();

    let parser = match format {
        ExternalOntologyFormat::Turtle => RdfParser::from_format(RdfFormat::Turtle),
        ExternalOntologyFormat::RdfXml => RdfParser::from_format(RdfFormat::RdfXml),
        ExternalOntologyFormat::JsonLd => RdfParser::from_format(RdfFormat::JsonLd {
            profile: JsonLdProfileSet::empty(),
        }),
    };

    for parsed in parser.for_reader(content.as_bytes()) {
        graph.push(parsed.map_err(|error| error.to_string())?);
    }

    if graph.is_empty() {
        return Err(format!(
            "{} block has no RDF statements",
            format.display_name()
        ));
    }

    Ok(graph)
}
