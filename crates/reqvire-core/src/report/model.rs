use crate::element;
use crate::element::SizeEstimate;
use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;
use crate::relation;
use rustc_hash::FxHashSet;
use serde::Serialize;

/// Model-centric report with nested element structure
#[derive(Debug, Serialize)]
pub struct ModelCentricReport {
    pub elements: Vec<ModelCentricElement>,
    pub metadata: ModelMetadata,
}

/// Direction of traversal for model report
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TraversalDirection {
    Forward, // Root to leaves through diagram traversal relations.
    Reverse, // Leaves to roots (derivedFrom, satisfy, verify)
}

/// Element in model-centric view with nested relations
#[derive(Debug, Serialize)]
pub struct ModelCentricElement {
    pub identifier: String,
    pub name: String,
    pub element_type: String,
    pub file_path: String,
    pub file_order_index: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_estimate: Option<SizeEstimate>,
    pub relations: Vec<ModelCentricRelation>,
    pub contract_bindings: Vec<String>,
    #[serde(skip)]
    pub contract_bindings_labels: Vec<String>,
}

/// Relation in model-centric view with target details
#[derive(Debug, Serialize)]
pub struct ModelCentricRelation {
    pub relation_type: String,
    #[serde(flatten)]
    pub target: RelationTarget,
}

/// Target of a relation (element, file, or external)
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum RelationTarget {
    Element {
        element: Box<ModelCentricElement>,
    },
    File {
        path: String,
        #[serde(rename = "type")]
        target_type: String,
    },
    External {
        url: String,
        #[serde(rename = "type")]
        target_type: String,
    },
}

/// Metadata about the model
#[derive(Debug, Serialize)]
pub struct ModelMetadata {
    pub total_elements: usize,
    pub total_relations: usize,
    pub filtered_from: Option<String>,
    pub direction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_filter: Option<Vec<String>>,
}

/// Generate model-centric report
pub fn generate_model_report(
    registry: &GraphRegistry,
    root_element_name: Option<&str>,
    reverse: bool,
    type_filter: Option<Vec<&str>>,
) -> Result<String, ReqvireError> {
    let report = build_model_report(registry, root_element_name, reverse, type_filter)?;
    serde_json::to_string_pretty(&report).map_err(ReqvireError::from)
}

fn build_model_report(
    registry: &GraphRegistry,
    root_element_name: Option<&str>,
    reverse: bool,
    type_filter: Option<Vec<&str>>,
) -> Result<ModelCentricReport, ReqvireError> {
    use rustc_hash::FxHashSet;

    // Validate type filter if provided
    if let Some(ref types) = type_filter {
        for t in types {
            if !element::is_valid_element_type(t) {
                return Err(ReqvireError::ProcessError(format!(
                    "Invalid element type '{}'. Valid types: {}",
                    t,
                    element::element_types_help()
                )));
            }
        }
    }

    let direction = if reverse {
        TraversalDirection::Reverse
    } else {
        TraversalDirection::Forward
    };

    // Determine starting elements
    let mut starting_elements = if let Some(name) = root_element_name {
        // Find element by name
        let found = registry
            .nodes
            .iter()
            .find(|(_, node)| node.element.name == name);

        match found {
            Some((id, _)) => vec![id.clone()],
            None => {
                eprintln!("❌ Element with name '{}' not found", name);
                return Err(ReqvireError::ElementError(format!(
                    "Element with name '{}' not found",
                    name
                )));
            }
        }
    } else if let Some(ref types) = type_filter {
        // Filter by element types
        if reverse {
            // For reverse, filter leaf elements by type
            registry.find_leaf_elements(Some(types.as_slice()))
        } else {
            types
                .iter()
                .flat_map(|element_type| {
                    find_model_starting_elements_by_type(registry, element_type)
                })
                .collect()
        }
    } else if reverse {
        // Reverse without type filter: use leaf elements
        registry.find_leaf_elements(None)
    } else {
        // No filter: start from ontology roots, concept roots, and capability roots.
        find_default_model_roots(registry)
    };

    if root_element_name.is_some() || type_filter.is_some() || reverse {
        starting_elements.sort();
    }

    // Build report
    let mut elements = Vec::new();

    for start_id in &starting_elements {
        // Each starting element gets its own visited set to allow independent traversals
        // This is important for reverse mode where multiple leaves may trace to common ancestors
        let mut visited = FxHashSet::default();
        if let Some(element) = build_element_recursive(registry, start_id, &mut visited, direction)
        {
            elements.push(element);
        }
    }

    // Count metadata
    let total_elements = starting_elements.len();
    let total_relations = count_relations(registry, &starting_elements, direction);

    Ok(ModelCentricReport {
        elements,
        metadata: ModelMetadata {
            total_elements,
            total_relations,
            filtered_from: root_element_name.map(|s| s.to_string()),
            direction: if reverse {
                "Reverse".to_string()
            } else {
                "Forward".to_string()
            },
            type_filter: type_filter.map(|v| v.into_iter().map(|s| s.to_string()).collect()),
        },
    })
}

fn find_default_model_roots(registry: &GraphRegistry) -> Vec<String> {
    let mut roots = registry.find_root_elements_by_type("ontology");
    let mut seen: FxHashSet<String> = roots.iter().cloned().collect();
    for concept_scheme_id in registry.find_root_elements_by_type("concept-scheme") {
        if seen.insert(concept_scheme_id.clone()) {
            roots.push(concept_scheme_id);
        }
    }
    for capability_id in registry.find_root_capabilities() {
        if seen.insert(capability_id.clone()) {
            roots.push(capability_id);
        }
    }
    roots
}

fn find_model_starting_elements_by_type(
    registry: &GraphRegistry,
    element_type: &str,
) -> Vec<String> {
    if element_type == "concept" {
        let mut concepts = registry
            .nodes
            .values()
            .filter(|node| node.element.element_type.as_str() == element_type)
            .map(|node| node.element.identifier.clone())
            .collect::<Vec<_>>();
        concepts.sort();
        return concepts;
    }
    registry.find_root_elements_by_type(element_type)
}

/// Build element recursively with nested relations
fn build_element_recursive(
    registry: &GraphRegistry,
    element_id: &str,
    visited: &mut FxHashSet<String>,
    direction: TraversalDirection,
) -> Option<ModelCentricElement> {
    // Prevent infinite recursion
    if visited.contains(element_id) {
        return None;
    }
    visited.insert(element_id.to_string());

    let element = registry.get_element(element_id)?;

    // Choose which relations to follow based on direction
    let allowed_relations = match direction {
        TraversalDirection::Forward => relation::DIAGRAM_RELATIONS,
        TraversalDirection::Reverse => relation::BACKWARD_RELATIONS,
    };

    // Sort element relations for deterministic iteration
    let mut sorted_relations = element.relations.clone();
    sorted_relations.sort_by(|a, b| {
        // First sort by relation type
        let type_cmp = a.relation_type.name.cmp(b.relation_type.name);
        if type_cmp != std::cmp::Ordering::Equal {
            return type_cmp;
        }

        // Then sort by target
        let a_target = match &a.target.link {
            relation::LinkType::Identifier(id) => id.as_str(),
            relation::LinkType::ExternalUrl(url) => url.as_str(),
            relation::LinkType::InternalPath(path) => path.to_str().unwrap_or(""),
        };
        let b_target = match &b.target.link {
            relation::LinkType::Identifier(id) => id.as_str(),
            relation::LinkType::ExternalUrl(url) => url.as_str(),
            relation::LinkType::InternalPath(path) => path.to_str().unwrap_or(""),
        };
        a_target.cmp(b_target)
    });

    // Build relations with nested targets
    let mut relations = Vec::new();
    for relation in &sorted_relations {
        // Only include relations matching the direction
        if !allowed_relations.contains(&relation.relation_type.name) {
            continue;
        }

        let target = match &relation.target.link {
            relation::LinkType::Identifier(target_id) => {
                let Some(target_element) = registry.get_element(target_id) else {
                    continue;
                };
                if is_model_namespace_context_relation(
                    element,
                    relation.relation_type.name,
                    target_element,
                ) {
                    continue;
                }
                // Recursive element
                if let Some(target_element) =
                    build_element_recursive(registry, target_id, visited, direction)
                {
                    RelationTarget::Element {
                        element: Box::new(target_element),
                    }
                } else {
                    continue; // Skip cyclic or missing
                }
            }
            relation::LinkType::InternalPath(path) => RelationTarget::File {
                path: path.to_string_lossy().to_string(),
                target_type: "file".to_string(),
            },
            relation::LinkType::ExternalUrl(url) => RelationTarget::External {
                url: url.clone(),
                target_type: "external".to_string(),
            },
        };

        relations.push(ModelCentricRelation {
            relation_type: relation.relation_type.name.to_string(),
            target,
        });
    }

    // Relations are already sorted from sorted_relations above

    // Build contract_bindings list
    let contract_bindings: Vec<String> = element
        .contract_bindings
        .iter()
        .map(|a| a.target.as_str())
        .collect();
    let contract_bindings_labels: Vec<String> = element
        .contract_bindings
        .iter()
        .map(|a| {
            let target_id = a.target.as_str();
            registry
                .get_element(&target_id)
                .map(|target| target.name.clone())
                .unwrap_or_else(|| contract_bindings_target_label(&target_id))
        })
        .collect();

    Some(ModelCentricElement {
        identifier: element.identifier.clone(),
        name: element.name.clone(),
        element_type: element.element_type.as_str().to_string(),
        file_path: element.file_path.clone(),
        file_order_index: element.file_order_index,
        size_estimate: element.size_estimate.clone(),
        relations,
        contract_bindings,
        contract_bindings_labels,
    })
}

fn contract_bindings_target_label(target: &str) -> String {
    let fragment_or_path = target.rsplit('#').next().unwrap_or(target);
    let basename = fragment_or_path
        .rsplit('/')
        .next()
        .unwrap_or(fragment_or_path);
    if basename.is_empty() {
        target.to_string()
    } else {
        basename.to_string()
    }
}

fn is_model_namespace_context_relation(
    source: &crate::element::Element,
    relation_name: &str,
    target: &crate::element::Element,
) -> bool {
    (relation_name == "derive"
        && source.element_type.is_ontology()
        && target.element_type.is_concept_scheme())
        || (relation_name == "derivedFrom"
            && source.element_type.is_concept_scheme()
            && target.element_type.is_ontology())
}

/// Count total relations from starting elements
fn count_relations(
    registry: &GraphRegistry,
    starting_elements: &[String],
    direction: TraversalDirection,
) -> usize {
    let mut count = 0;
    let mut visited = FxHashSet::default();

    for start_id in starting_elements {
        count_relations_recursive(registry, start_id, &mut visited, &mut count, direction);
    }

    count
}

/// Count relations recursively
fn count_relations_recursive(
    registry: &GraphRegistry,
    element_id: &str,
    visited: &mut FxHashSet<String>,
    count: &mut usize,
    direction: TraversalDirection,
) {
    if visited.contains(element_id) {
        return;
    }
    visited.insert(element_id.to_string());

    let allowed_relations = match direction {
        TraversalDirection::Forward => relation::DIAGRAM_RELATIONS,
        TraversalDirection::Reverse => relation::BACKWARD_RELATIONS,
    };

    if let Some(element) = registry.get_element(element_id) {
        for relation in &element.relations {
            if allowed_relations.contains(&relation.relation_type.name) {
                if let relation::LinkType::Identifier(target_id) = &relation.target.link {
                    if let Some(target) = registry.get_element(target_id) {
                        if is_model_namespace_context_relation(
                            element,
                            relation.relation_type.name,
                            target,
                        ) {
                            continue;
                        }
                    }
                }
                *count += 1;

                // Recurse for identifier targets
                if let relation::LinkType::Identifier(target_id) = &relation.target.link {
                    count_relations_recursive(registry, target_id, visited, count, direction);
                }
            }
        }
    }
}
