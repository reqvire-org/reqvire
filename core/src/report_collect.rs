use crate::element::{ElementType, ReusedContractContextTarget};
use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;
use crate::relation;
use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Direction of traversal for content collection
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CollectDirection {
    /// Traverse derivedFrom relations upward to ancestors (default)
    Upstream,
    /// Traverse derive relations downward to descendants
    Downstream,
}

impl std::fmt::Display for CollectDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CollectDirection::Upstream => write!(f, "upstream"),
            CollectDirection::Downstream => write!(f, "downstream"),
        }
    }
}

/// Source type for collected content items
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    /// Content from a model element
    Element,
    /// Content from a contract element (via definedBy relation)
    RefinedByElement,
    /// Content from a contract file (via definedBy relation)
    RefinedByFile,
    /// Content from an reused file
    ReusedContractContextFile,
    /// Content from an reused contract element
    ReusedContractContextElement,
    /// Authored concept references and reachable semantic context
    OntologyContext,
}

/// A single collected content item
#[derive(Debug, Serialize)]
pub struct CollectedItem {
    pub name: String,
    pub identifier: String,
    pub file_path: String,
    pub element_type: String,
    pub content: String,
    pub depth: usize,
    pub source_type: SourceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reused_by: Option<String>,
}

/// Metadata about the collection
#[derive(Debug, Serialize)]
pub struct CollectMetadata {
    pub element_count: usize,
    pub contract_count: usize,
    pub reused_contract_context_count: usize,
    pub ontology_count: usize,
    pub total_items: usize,
}

/// Complete collect report
#[derive(Debug, Serialize)]
pub struct CollectReport {
    pub starting_element: String,
    pub direction: CollectDirection,
    pub items: Vec<CollectedItem>,
    pub metadata: CollectMetadata,
}

/// Generate a collect report for a capability, requirement, or ontology element
pub fn generate_collect_report(
    registry: &GraphRegistry,
    element_name: &str,
    git_root: &Path,
    json_output: bool,
    direction: CollectDirection,
) -> Result<String, ReqvireError> {
    // Find element by name
    let element_id = registry
        .nodes
        .iter()
        .find(|(_, node)| node.element.name == element_name)
        .map(|(id, _)| id.clone());

    let element_id = match element_id {
        Some(id) => id,
        None => {
            return Err(ReqvireError::ElementError(format!(
                "Element with name '{}' not found",
                element_name
            )));
        }
    };

    // Get the element
    let element = registry.get_element(&element_id).ok_or_else(|| {
        ReqvireError::ElementError(format!("Element '{}' not found in registry", element_id))
    })?;

    // Validate element type is supported by collect.
    match &element.element_type {
        ElementType::Capability | ElementType::Requirement(_) | ElementType::Ontology => {}
        _ => {
            return Err(ReqvireError::ElementError(format!(
                "Element '{}' is not a capability, requirement, or ontology type (found: {}). Only capability, requirement, and ontology types are supported.",
                element_name,
                element.element_type.as_str()
            )));
        }
    }

    // Collect chain based on direction
    let chain = match direction {
        CollectDirection::Upstream => collect_upstream_chain(registry, &element_id),
        CollectDirection::Downstream => collect_downstream_chain(registry, &element_id),
    };

    // Build collected items
    let mut items: Vec<CollectedItem> = Vec::new();
    let mut element_count = 0;
    let mut contract_count = 0;
    let mut reused_contract_context_count = 0;
    let mut ontology_count = 0;
    let mut collected_ontology_context: HashSet<String> = HashSet::new();

    // For upstream: chain is start→root, reverse to get root first (depth 0)
    // For downstream: chain is already start→leaves (start at depth 0)
    let ordered_chain: Vec<&String> = match direction {
        CollectDirection::Upstream => chain.iter().rev().collect(),
        CollectDirection::Downstream => chain.iter().collect(),
    };

    for (depth, elem_id) in ordered_chain.iter().enumerate() {
        if let Some(elem) = registry.get_element(elem_id) {
            // Add element content
            items.push(CollectedItem {
                name: elem.name.clone(),
                identifier: elem.identifier.clone(),
                file_path: elem.file_path.clone(),
                element_type: elem.element_type.as_str().to_string(),
                content: elem.content.clone(),
                depth,
                source_type: SourceType::Element,
                reused_by: None,
            });
            element_count += 1;

            // Collect definedBy targets (requirement-owned contract elements and files)
            for rel in &elem.relations {
                if rel.relation_type.name == "definedBy" {
                    if let Some(item) = collect_contract_content(
                        registry,
                        &rel.target,
                        &elem.identifier,
                        depth,
                        git_root,
                    ) {
                        contract_count += 1;
                        items.push(item);
                    }
                }
            }

            // Collect reused_contract_context contents
            for reused_contract_context in &elem.reused_contract_context {
                if let Some(item) = collect_reused_contract_context_content(
                    registry,
                    reused_contract_context,
                    &elem.identifier,
                    depth,
                    git_root,
                ) {
                    if matches!(item.source_type, SourceType::OntologyContext) {
                        if !collected_ontology_context.insert(item.identifier.clone()) {
                            continue;
                        }
                        ontology_count += 1;
                    } else {
                        reused_contract_context_count += 1;
                    }
                    items.push(item);
                }
            }

            let ontology_context =
                registry.build_concept_reference_ontology_context(&elem.identifier);

            for ontology_id in ontology_context {
                if !collected_ontology_context.insert(ontology_id.clone()) {
                    continue;
                }
                if let Some(ontology) = registry.get_element(&ontology_id) {
                    ontology_count += 1;
                    items.push(CollectedItem {
                        name: ontology.name.clone(),
                        identifier: ontology.identifier.clone(),
                        file_path: ontology.file_path.clone(),
                        element_type: ontology.element_type.as_str().to_string(),
                        content: ontology.content.clone(),
                        depth: depth + 1,
                        source_type: SourceType::OntologyContext,
                        reused_by: Some(elem.identifier.clone()),
                    });
                }
            }
        }
    }

    let report = CollectReport {
        starting_element: element_id,
        direction,
        items,
        metadata: CollectMetadata {
            element_count,
            contract_count,
            reused_contract_context_count,
            ontology_count,
            total_items: element_count
                + contract_count
                + reused_contract_context_count
                + ontology_count,
        },
    };

    if json_output {
        serde_json::to_string_pretty(&report)
            .map_err(|e| ReqvireError::SerializationError(e.to_string()))
    } else {
        Ok(generate_text_output(&report))
    }
}

/// Collect upstream context.
///
/// Capability starts traverse capability parents only.
/// Requirement starts traverse requirement parents, then cross to owning capability and capability parents.
/// Ontology starts traverse ontology parents.
fn collect_upstream_chain(registry: &GraphRegistry, start_id: &str) -> Vec<String> {
    let Some(start) = registry.get_element(start_id) else {
        return Vec::new();
    };

    match &start.element_type {
        ElementType::Capability => {
            collect_parent_chain_by_type(registry, start_id, ElementTypeKind::Capability)
        }
        ElementType::Requirement(_) => {
            let mut chain =
                collect_parent_chain_by_type(registry, start_id, ElementTypeKind::Requirement);
            if let Some(owner_capability) = find_owning_capability(registry, start_id) {
                let capability_chain = collect_parent_chain_by_type(
                    registry,
                    &owner_capability,
                    ElementTypeKind::Capability,
                );
                for id in capability_chain {
                    if !chain.contains(&id) {
                        chain.push(id);
                    }
                }
            }
            chain
        }
        ElementType::Ontology => {
            collect_parent_chain_by_type(registry, start_id, ElementTypeKind::Ontology)
        }
        _ => Vec::new(),
    }
}

#[derive(Clone, Copy)]
enum ElementTypeKind {
    Capability,
    Requirement,
    Ontology,
}

fn collect_parent_chain_by_type(
    registry: &GraphRegistry,
    start_id: &str,
    kind: ElementTypeKind,
) -> Vec<String> {
    let mut chain = Vec::new();
    let mut visited = HashSet::new();
    let mut current_level = vec![start_id.to_string()];

    while !current_level.is_empty() {
        // Sort for deterministic ordering
        let mut sorted_level = current_level.clone();
        sorted_level.sort();

        for elem_id in &sorted_level {
            if visited.contains(elem_id) {
                continue;
            }
            visited.insert(elem_id.clone());
            chain.push(elem_id.clone());
        }

        // Find next level (parents via derivedFrom)
        let mut next_level = Vec::new();
        for elem_id in &sorted_level {
            if let Some(elem) = registry.get_element(elem_id) {
                for rel in &elem.relations {
                    if rel.relation_type.name == "derivedFrom" {
                        if let relation::LinkType::Identifier(target_id) = &rel.target.link {
                            if !visited.contains(target_id)
                                && element_matches_kind(registry, target_id, kind)
                            {
                                next_level.push(target_id.clone());
                            }
                        }
                    }
                }
            }
        }

        current_level = next_level;
    }

    chain
}

/// Collect downstream context.
///
/// Capability starts traverse child capabilities and requirements that specify each capability.
/// Requirement starts traverse requirement descendants only.
/// Ontology starts traverse ontology descendants and semantic contracts that use reachable ontology.
fn collect_downstream_chain(registry: &GraphRegistry, start_id: &str) -> Vec<String> {
    let Some(start) = registry.get_element(start_id) else {
        return Vec::new();
    };

    match &start.element_type {
        ElementType::Capability => collect_capability_downstream_chain(registry, start_id),
        ElementType::Requirement(_) => collect_requirement_downstream_chain(registry, start_id),
        ElementType::Ontology => collect_ontology_downstream_chain(registry, start_id),
        _ => Vec::new(),
    }
}

fn collect_requirement_downstream_chain(registry: &GraphRegistry, start_id: &str) -> Vec<String> {
    let mut chain = Vec::new();
    let mut visited = HashSet::new();
    let mut current_level = vec![start_id.to_string()];

    while !current_level.is_empty() {
        // Sort for deterministic ordering
        let mut sorted_level = current_level.clone();
        sorted_level.sort();

        for elem_id in &sorted_level {
            if visited.contains(elem_id) {
                continue;
            }
            visited.insert(elem_id.clone());
            chain.push(elem_id.clone());
        }

        // Find next level (children via derive)
        let mut next_level = Vec::new();
        for elem_id in &sorted_level {
            if let Some(elem) = registry.get_element(elem_id) {
                for rel in &elem.relations {
                    if rel.relation_type.name == "derive" {
                        if let relation::LinkType::Identifier(target_id) = &rel.target.link {
                            if !visited.contains(target_id)
                                && element_matches_kind(
                                    registry,
                                    target_id,
                                    ElementTypeKind::Requirement,
                                )
                            {
                                next_level.push(target_id.clone());
                            }
                        }
                    }
                }
            }
        }

        current_level = next_level;
    }

    chain
}

fn collect_capability_downstream_chain(registry: &GraphRegistry, start_id: &str) -> Vec<String> {
    let mut chain = Vec::new();
    let mut visited = HashSet::new();
    let mut current_level = vec![start_id.to_string()];

    while !current_level.is_empty() {
        let mut sorted_level = current_level.clone();
        sorted_level.sort();

        for elem_id in &sorted_level {
            if visited.contains(elem_id) {
                continue;
            }
            visited.insert(elem_id.clone());
            chain.push(elem_id.clone());
        }

        let mut next_level = Vec::new();
        for elem_id in &sorted_level {
            if let Some(elem) = registry.get_element(elem_id) {
                match &elem.element_type {
                    ElementType::Capability => {
                        for rel in &elem.relations {
                            if matches!(rel.relation_type.name, "derive" | "specifiedBy") {
                                if let relation::LinkType::Identifier(target_id) = &rel.target.link
                                {
                                    if !visited.contains(target_id) {
                                        next_level.push(target_id.clone());
                                    }
                                }
                            }
                        }
                    }
                    ElementType::Requirement(_) => {
                        for rel in &elem.relations {
                            if rel.relation_type.name == "derive" {
                                if let relation::LinkType::Identifier(target_id) = &rel.target.link
                                {
                                    if !visited.contains(target_id)
                                        && element_matches_kind(
                                            registry,
                                            target_id,
                                            ElementTypeKind::Requirement,
                                        )
                                    {
                                        next_level.push(target_id.clone());
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        current_level = next_level;
    }

    chain
}

fn collect_ontology_downstream_chain(registry: &GraphRegistry, start_id: &str) -> Vec<String> {
    let mut chain = Vec::new();
    let mut visited = HashSet::new();
    let mut current_level = vec![start_id.to_string()];

    while !current_level.is_empty() {
        let mut sorted_level = current_level.clone();
        sorted_level.sort();

        let mut next_level = Vec::new();
        for elem_id in &sorted_level {
            if visited.contains(elem_id) {
                continue;
            }
            visited.insert(elem_id.clone());
            chain.push(elem_id.clone());

            let Some(elem) = registry.get_element(elem_id) else {
                continue;
            };

            if elem.element_type.is_ontology() {
                for rel in &elem.relations {
                    if rel.relation_type.name == "derive" {
                        if let relation::LinkType::Identifier(target_id) = &rel.target.link {
                            if !visited.contains(target_id)
                                && element_matches_kind(
                                    registry,
                                    target_id,
                                    ElementTypeKind::Ontology,
                                )
                            {
                                next_level.push(target_id.clone());
                            }
                        }
                    }
                }

                for contract_id in semantic_contracts_using_ontology(registry, elem_id) {
                    if !visited.contains(&contract_id) {
                        next_level.push(contract_id);
                    }
                }
            }
        }

        current_level = next_level;
    }

    chain
}

fn semantic_contracts_using_ontology(registry: &GraphRegistry, ontology_id: &str) -> Vec<String> {
    let mut contracts = Vec::new();
    let mut seen = HashSet::new();

    if let Some(ontology) = registry.get_element(ontology_id) {
        for rel in &ontology.relations {
            if rel.relation_type.name == "usedBy" {
                if let relation::LinkType::Identifier(target_id) = &rel.target.link {
                    if registry
                        .get_element(target_id)
                        .is_some_and(|element| element.element_type.is_semantic_contract())
                        && seen.insert(target_id.clone())
                    {
                        contracts.push(target_id.clone());
                    }
                }
            }
        }
    }

    for element in registry.get_all_elements() {
        if !element.element_type.is_semantic_contract() {
            continue;
        }
        let uses_ontology = element.relations.iter().any(|rel| {
            rel.relation_type.name == "use"
                && matches!(&rel.target.link, relation::LinkType::Identifier(target_id) if target_id == ontology_id)
        });
        if uses_ontology && seen.insert(element.identifier.clone()) {
            contracts.push(element.identifier.clone());
        }
    }

    contracts.sort();
    contracts
}

fn element_matches_kind(registry: &GraphRegistry, element_id: &str, kind: ElementTypeKind) -> bool {
    registry
        .get_element(element_id)
        .is_some_and(|element| match kind {
            ElementTypeKind::Capability => matches!(element.element_type, ElementType::Capability),
            ElementTypeKind::Requirement => {
                matches!(element.element_type, ElementType::Requirement(_))
            }
            ElementTypeKind::Ontology => matches!(element.element_type, ElementType::Ontology),
        })
}

fn find_owning_capability(registry: &GraphRegistry, requirement_id: &str) -> Option<String> {
    let mut visited = HashSet::new();
    let mut current_level = vec![requirement_id.to_string()];

    while !current_level.is_empty() {
        let mut next_level = Vec::new();
        for elem_id in &current_level {
            if !visited.insert(elem_id.clone()) {
                continue;
            }
            let Some(elem) = registry.get_element(elem_id) else {
                continue;
            };

            for rel in &elem.relations {
                if rel.relation_type.name == "specify" {
                    if let relation::LinkType::Identifier(target_id) = &rel.target.link {
                        if element_matches_kind(registry, target_id, ElementTypeKind::Capability) {
                            return Some(target_id.clone());
                        }
                    }
                }
            }

            for rel in &elem.relations {
                if rel.relation_type.name == "derivedFrom" {
                    if let relation::LinkType::Identifier(target_id) = &rel.target.link {
                        if element_matches_kind(registry, target_id, ElementTypeKind::Requirement) {
                            next_level.push(target_id.clone());
                        }
                    }
                }
            }
        }

        current_level = next_level;
    }

    None
}

/// Collect content from an reused_contract_context
fn collect_reused_contract_context_content(
    registry: &GraphRegistry,
    reused_contract_context: &crate::element::ReusedContractContextEntry,
    parent_identifier: &str,
    depth: usize,
    git_root: &Path,
) -> Option<CollectedItem> {
    match &reused_contract_context.target {
        ReusedContractContextTarget::FilePath(path) => {
            let full_path = git_root.join(path);
            let path_str = path.to_string_lossy().to_string();

            // Check if it's a markdown file
            if path_str.ends_with(".md") {
                // Try to read file content
                match fs::read_to_string(&full_path) {
                    Ok(content) => Some(CollectedItem {
                        name: path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| path_str.clone()),
                        identifier: path_str,
                        file_path: path.to_string_lossy().to_string(),
                        element_type: "reused_contract_context".to_string(),
                        content,
                        depth,
                        source_type: SourceType::ReusedContractContextFile,
                        reused_by: Some(parent_identifier.to_string()),
                    }),
                    Err(_) => {
                        // File not found - create link instead
                        Some(CollectedItem {
                            name: path
                                .file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_else(|| path_str.clone()),
                            identifier: path_str.clone(),
                            file_path: path.to_string_lossy().to_string(),
                            element_type: "reused_contract_context".to_string(),
                            content: format!("[{}]({})", path_str, path_str),
                            depth,
                            source_type: SourceType::ReusedContractContextFile,
                            reused_by: Some(parent_identifier.to_string()),
                        })
                    }
                }
            } else {
                // Non-markdown file - create link
                Some(CollectedItem {
                    name: path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| path_str.clone()),
                    identifier: path_str.clone(),
                    file_path: path.to_string_lossy().to_string(),
                    element_type: "reused_contract_context".to_string(),
                    content: format!("[{}]({})", path_str, path_str),
                    depth,
                    source_type: SourceType::ReusedContractContextFile,
                    reused_by: Some(parent_identifier.to_string()),
                })
            }
        }
        ReusedContractContextTarget::ElementIdentifier(elem_id) => {
            // Look up element content from registry
            registry.get_element(elem_id).map(|elem| {
                let source_type = if elem.element_type.is_ontology() {
                    SourceType::OntologyContext
                } else {
                    SourceType::ReusedContractContextElement
                };
                CollectedItem {
                    name: elem.name.clone(),
                    identifier: elem.identifier.clone(),
                    file_path: elem.file_path.clone(),
                    element_type: elem.element_type.as_str().to_string(),
                    content: elem.content.clone(),
                    depth,
                    source_type,
                    reused_by: Some(parent_identifier.to_string()),
                }
            })
        }
    }
}

/// Collect content from a definedBy relation target
fn collect_contract_content(
    registry: &GraphRegistry,
    target: &relation::RelationTarget,
    parent_identifier: &str,
    depth: usize,
    git_root: &Path,
) -> Option<CollectedItem> {
    match &target.link {
        relation::LinkType::Identifier(elem_id) => {
            // Element identifier - look up contract element content
            registry.get_element(elem_id).map(|elem| CollectedItem {
                name: elem.name.clone(),
                identifier: elem.identifier.clone(),
                file_path: elem.file_path.clone(),
                element_type: elem.element_type.as_str().to_string(),
                content: elem.content.clone(),
                depth,
                source_type: SourceType::RefinedByElement,
                reused_by: Some(parent_identifier.to_string()),
            })
        }
        relation::LinkType::InternalPath(path) => {
            // File path - read file content (same logic as reused_contract_context file handling)
            let full_path = git_root.join(path);
            let path_str = path.to_string_lossy().to_string();

            if path_str.ends_with(".md") {
                match fs::read_to_string(&full_path) {
                    Ok(content) => Some(CollectedItem {
                        name: path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| path_str.clone()),
                        identifier: path_str,
                        file_path: path.to_string_lossy().to_string(),
                        element_type: "contract".to_string(),
                        content,
                        depth,
                        source_type: SourceType::RefinedByFile,
                        reused_by: Some(parent_identifier.to_string()),
                    }),
                    Err(_) => Some(CollectedItem {
                        name: path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| path_str.clone()),
                        identifier: path_str.clone(),
                        file_path: path.to_string_lossy().to_string(),
                        element_type: "contract".to_string(),
                        content: format!("[{}]({})", path_str, path_str),
                        depth,
                        source_type: SourceType::RefinedByFile,
                        reused_by: Some(parent_identifier.to_string()),
                    }),
                }
            } else {
                // Non-markdown file - create link
                Some(CollectedItem {
                    name: path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| path_str.clone()),
                    identifier: path_str.clone(),
                    file_path: path.to_string_lossy().to_string(),
                    element_type: "contract".to_string(),
                    content: format!("[{}]({})", path_str, path_str),
                    depth,
                    source_type: SourceType::RefinedByFile,
                    reused_by: Some(parent_identifier.to_string()),
                })
            }
        }
        relation::LinkType::ExternalUrl(_) => {
            // Skip external URLs
            None
        }
    }
}

/// Generate text output with source citations
fn generate_text_output(report: &CollectReport) -> String {
    let mut output = String::new();

    for item in &report.items {
        // Add content
        output.push_str(&item.content);
        output.push_str("\n\n");

        // Add source citation based on source type
        match item.source_type {
            SourceType::Element => {
                output.push_str(&format!("— Source: [{}]({})\n", item.name, item.identifier));
            }
            SourceType::RefinedByElement => {
                if let Some(ref parent) = item.reused_by {
                    output.push_str(&format!(
                        "— Source: [{}]({}) refining [{}]({})\n",
                        item.name,
                        item.identifier,
                        extract_element_name(parent),
                        parent
                    ));
                } else {
                    output.push_str(&format!("— Source: [{}]({})\n", item.name, item.identifier));
                }
            }
            SourceType::RefinedByFile => {
                if let Some(ref parent) = item.reused_by {
                    output.push_str(&format!(
                        "— Source: [{}]({}) refining [{}]({})\n",
                        item.name,
                        item.identifier,
                        extract_element_name(parent),
                        parent
                    ));
                } else {
                    output.push_str(&format!("— Source: [{}]({})\n", item.name, item.identifier));
                }
            }
            SourceType::ReusedContractContextFile => {
                if let Some(ref parent) = item.reused_by {
                    output.push_str(&format!(
                        "— Source: [{}]({}) reused to [{}]({})\n",
                        item.name,
                        item.identifier,
                        extract_element_name(parent),
                        parent
                    ));
                } else {
                    output.push_str(&format!("— Source: [{}]({})\n", item.name, item.identifier));
                }
            }
            SourceType::ReusedContractContextElement => {
                if let Some(ref parent) = item.reused_by {
                    output.push_str(&format!(
                        "— Source: [{}]({}) reused to [{}]({})\n",
                        item.name,
                        item.identifier,
                        extract_element_name(parent),
                        parent
                    ));
                } else {
                    output.push_str(&format!("— Source: [{}]({})\n", item.name, item.identifier));
                }
            }
            SourceType::OntologyContext => {
                if let Some(ref parent) = item.reused_by {
                    output.push_str(&format!(
                        "— Source: [{}]({}) ontology context for [{}]({})\n",
                        item.name,
                        item.identifier,
                        extract_element_name(parent),
                        parent
                    ));
                } else {
                    output.push_str(&format!("— Source: [{}]({})\n", item.name, item.identifier));
                }
            }
        }

        // Add separator after citation
        output.push_str("\n---\n\n");
    }

    output
}

/// Extract element name from identifier (text after #)
fn extract_element_name(identifier: &str) -> String {
    if let Some(pos) = identifier.rfind('#') {
        // Convert fragment to title case
        let fragment = &identifier[pos + 1..];
        fragment
            .split('-')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        identifier.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_direction_display() {
        assert_eq!(format!("{}", CollectDirection::Upstream), "upstream");
        assert_eq!(format!("{}", CollectDirection::Downstream), "downstream");
    }

    #[test]
    fn test_extract_element_name() {
        assert_eq!(
            extract_element_name("file.md#some-element-name"),
            "Some Element Name"
        );
        assert_eq!(extract_element_name("path/to/file.md#simple"), "Simple");
        assert_eq!(extract_element_name("no-fragment"), "no-fragment");
    }
}
