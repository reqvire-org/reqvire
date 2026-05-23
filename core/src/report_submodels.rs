use crate::element::ElementType;
use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;
use crate::relation::LinkType;
use crate::utils;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[derive(Serialize)]
pub struct SubmodelsReport {
    pub submodels: Vec<SubmodelSummary>,
    pub cross_submodel_couplings: Vec<CrossSubmodelCoupling>,
    pub summary: SubmodelsSummary,
}

#[derive(Serialize)]
pub struct SubmodelSummary {
    pub root_id: String,
    pub root_name: String,
    pub root_type: String,
    pub requirement_count: usize,
}

#[derive(Serialize)]
pub struct CrossSubmodelCoupling {
    pub source_id: String,
    pub source_name: String,
    pub source_root_id: String,
    pub source_root_name: String,
    pub relation_type: String,
    pub target_id: String,
    pub target_name: String,
    pub target_root_id: String,
    pub target_root_name: String,
}

#[derive(Serialize)]
pub struct SubmodelsSummary {
    pub total_submodels: usize,
    pub total_requirements: usize,
    pub total_cross_submodel_couplings: usize,
}

impl SubmodelsReport {
    pub fn to_json_string(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    pub fn format_text(&self) -> String {
        let mut output = String::new();

        output.push_str("## Submodels\n\n");
        output.push_str(
            "Independent feature-rooted subgraphs resolved via feature ownership relations.\n\n",
        );

        if self.submodels.is_empty() {
            output.push_str("*No feature-rooted submodels found.*\n\n");
        } else {
            for submodel in &self.submodels {
                output.push_str(&format!(
                    "### {}\n",
                    format_identifier_link(&submodel.root_id, &submodel.root_name)
                ));
                output.push_str(&format!("  * Type: {}\n", submodel.root_type));
                output.push_str(&format!(
                    "  * Requirements: {}\n",
                    submodel.requirement_count
                ));
                output.push_str("---\n\n");
            }
        }

        output.push_str("## Cross-Submodel Couplings\n\n");
        output.push_str(
            "Requirement-to-requirement relations where source and target belong to different feature roots.\n\n",
        );

        if self.cross_submodel_couplings.is_empty() {
            output.push_str("*No cross-submodel requirement couplings found.*\n\n");
        } else {
            for coupling in &self.cross_submodel_couplings {
                let source = format_identifier_link(&coupling.source_id, &coupling.source_name);
                let target = format_identifier_link(&coupling.target_id, &coupling.target_name);
                output.push_str(&format!(
                    "  * {} --{}--> {} ({} -> {})\n",
                    source,
                    coupling.relation_type,
                    target,
                    coupling.source_root_name,
                    coupling.target_root_name
                ));
            }
            output.push('\n');
        }

        output.push_str("## Summary\n\n");
        output.push_str(&format!(
            "- **Submodels:** {}\n",
            self.summary.total_submodels
        ));
        output.push_str(&format!(
            "- **Requirements:** {}\n",
            self.summary.total_requirements
        ));
        output.push_str(&format!(
            "- **Cross-Submodel Couplings:** {}\n",
            self.summary.total_cross_submodel_couplings
        ));

        output
    }
}

fn format_identifier_link(identifier: &str, name: &str) -> String {
    if let Some(hash_pos) = identifier.rfind('#') {
        let file_part = &identifier[..hash_pos];
        let fragment_part = &identifier[hash_pos..];
        format!("[{}]({}{})", name, file_part, fragment_part)
    } else {
        format!("[{}]({})", name, identifier)
    }
}

fn is_requirement_element(element_type: &ElementType) -> bool {
    matches!(element_type, ElementType::Requirement(_))
}

fn is_feature_element(element_type: &ElementType) -> bool {
    matches!(element_type, ElementType::Feature)
}

fn resolve_target_identifier(
    registry: &GraphRegistry,
    source_file_path: &str,
    target_identifier: &str,
) -> Option<String> {
    if registry.get_element(target_identifier).is_some() {
        return Some(target_identifier.to_string());
    }

    if target_identifier.starts_with('#') {
        let full_identifier = format!("{}{}", source_file_path, target_identifier);
        if registry.get_element(&full_identifier).is_some() {
            return Some(full_identifier);
        }
    }

    let (_, fragment_opt) = utils::extract_path_and_fragment(target_identifier);
    if let Some(fragment) = fragment_opt {
        if registry.get_element(fragment).is_some() {
            return Some(fragment.to_string());
        }
    }

    None
}

fn parent_ids_by_relation_and_type(
    registry: &GraphRegistry,
    element_id: &str,
    relation_name: &str,
    target_predicate: fn(&ElementType) -> bool,
) -> Vec<String> {
    let mut parents = BTreeSet::new();
    let Some(source) = registry.get_element(element_id) else {
        return Vec::new();
    };

    for relation in &source.relations {
        if relation.relation_type.name != relation_name {
            continue;
        }
        let LinkType::Identifier(target_identifier) = &relation.target.link else {
            continue;
        };

        let Some(parent_id) =
            resolve_target_identifier(registry, &source.file_path, target_identifier)
        else {
            continue;
        };

        if registry
            .get_element(&parent_id)
            .is_some_and(|target| target_predicate(&target.element_type))
        {
            parents.insert(parent_id);
        }
    }

    parents.into_iter().collect()
}

fn build_child_map(parent_map: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut child_map: HashMap<String, Vec<String>> = HashMap::new();
    for (child_id, parent_ids) in parent_map {
        for parent_id in parent_ids {
            child_map
                .entry(parent_id.clone())
                .or_default()
                .push(child_id.clone());
        }
    }

    for children in child_map.values_mut() {
        children.sort();
    }

    child_map
}

fn resolve_requirement_subtree_roots(
    requirement_id: &str,
    parent_map: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, BTreeSet<String>>,
    visiting: &mut HashSet<String>,
) -> BTreeSet<String> {
    if let Some(cached) = memo.get(requirement_id) {
        return cached.clone();
    }

    if visiting.contains(requirement_id) {
        return BTreeSet::new();
    }
    visiting.insert(requirement_id.to_string());

    let mut result = BTreeSet::new();
    let parent_ids = parent_map.get(requirement_id).cloned().unwrap_or_default();

    if parent_ids.is_empty() {
        result.insert(requirement_id.to_string());
    } else {
        for parent_id in parent_ids {
            let parent_roots =
                resolve_requirement_subtree_roots(&parent_id, parent_map, memo, visiting);
            for root in parent_roots {
                result.insert(root);
            }
        }
    }

    visiting.remove(requirement_id);
    memo.insert(requirement_id.to_string(), result.clone());
    result
}

fn resolve_feature_roots(
    feature_id: &str,
    feature_parent_map: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, BTreeSet<String>>,
    visiting: &mut HashSet<String>,
) -> BTreeSet<String> {
    if let Some(cached) = memo.get(feature_id) {
        return cached.clone();
    }

    if visiting.contains(feature_id) {
        return BTreeSet::new();
    }
    visiting.insert(feature_id.to_string());

    let mut result = BTreeSet::new();
    let parent_ids = feature_parent_map
        .get(feature_id)
        .cloned()
        .unwrap_or_default();

    if parent_ids.is_empty() {
        result.insert(feature_id.to_string());
    } else {
        for parent_id in parent_ids {
            let parent_roots =
                resolve_feature_roots(&parent_id, feature_parent_map, memo, visiting);
            for root in parent_roots {
                result.insert(root);
            }
        }
    }

    visiting.remove(feature_id);
    memo.insert(feature_id.to_string(), result.clone());
    result
}

fn resolve_requirement_feature_roots(
    requirement_id: &str,
    requirement_parent_map: &HashMap<String, Vec<String>>,
    requirement_specify_map: &HashMap<String, Vec<String>>,
    feature_parent_map: &HashMap<String, Vec<String>>,
    feature_memo: &mut HashMap<String, BTreeSet<String>>,
    requirement_memo: &mut HashMap<String, BTreeSet<String>>,
    visiting: &mut HashSet<String>,
) -> BTreeSet<String> {
    if let Some(cached) = requirement_memo.get(requirement_id) {
        return cached.clone();
    }

    if visiting.contains(requirement_id) {
        return BTreeSet::new();
    }
    visiting.insert(requirement_id.to_string());

    let mut result = BTreeSet::new();

    for feature_id in requirement_specify_map
        .get(requirement_id)
        .cloned()
        .unwrap_or_default()
    {
        let feature_roots = resolve_feature_roots(
            &feature_id,
            feature_parent_map,
            feature_memo,
            &mut HashSet::new(),
        );
        for root in feature_roots {
            result.insert(root);
        }
    }

    for parent_requirement_id in requirement_parent_map
        .get(requirement_id)
        .cloned()
        .unwrap_or_default()
    {
        let parent_roots = resolve_requirement_feature_roots(
            &parent_requirement_id,
            requirement_parent_map,
            requirement_specify_map,
            feature_parent_map,
            feature_memo,
            requirement_memo,
            visiting,
        );
        for root in parent_roots {
            result.insert(root);
        }
    }

    visiting.remove(requirement_id);
    requirement_memo.insert(requirement_id.to_string(), result.clone());
    result
}

fn collect_requirement_descendants(
    start_id: &str,
    requirement_child_map: &HashMap<String, Vec<String>>,
) -> BTreeSet<String> {
    let mut result = BTreeSet::new();
    let mut stack = vec![start_id.to_string()];

    while let Some(current) = stack.pop() {
        if !result.insert(current.clone()) {
            continue;
        }
        if let Some(children) = requirement_child_map.get(&current) {
            for child in children {
                stack.push(child.clone());
            }
        }
    }

    result
}

fn collect_feature_subtree_requirements(
    feature_id: &str,
    feature_child_map: &HashMap<String, Vec<String>>,
    feature_to_requirements_map: &HashMap<String, Vec<String>>,
    requirement_child_map: &HashMap<String, Vec<String>>,
) -> BTreeSet<String> {
    let mut feature_ids = BTreeSet::new();
    let mut stack = vec![feature_id.to_string()];

    while let Some(current) = stack.pop() {
        if !feature_ids.insert(current.clone()) {
            continue;
        }
        if let Some(children) = feature_child_map.get(&current) {
            for child in children {
                stack.push(child.clone());
            }
        }
    }

    let mut requirements = BTreeSet::new();
    for current_feature_id in feature_ids {
        for requirement_id in feature_to_requirements_map
            .get(&current_feature_id)
            .cloned()
            .unwrap_or_default()
        {
            for descendant_id in
                collect_requirement_descendants(&requirement_id, requirement_child_map)
            {
                requirements.insert(descendant_id);
            }
        }
    }

    requirements
}

pub fn generate_submodels_report(
    registry: &GraphRegistry,
    from_name: Option<&str>,
) -> Result<SubmodelsReport, ReqvireError> {
    let feature_ids: Vec<String> = registry
        .get_all_elements()
        .into_iter()
        .filter(|e| is_feature_element(&e.element_type))
        .map(|e| e.identifier.clone())
        .collect();
    let requirement_ids: Vec<String> = registry
        .get_all_elements()
        .into_iter()
        .filter(|e| is_requirement_element(&e.element_type))
        .map(|e| e.identifier.clone())
        .collect();

    let requirement_set: HashSet<String> = requirement_ids.iter().cloned().collect();

    let mut feature_parent_map: HashMap<String, Vec<String>> = HashMap::new();
    for feature_id in &feature_ids {
        feature_parent_map.insert(
            feature_id.clone(),
            parent_ids_by_relation_and_type(
                registry,
                feature_id,
                "derivedFrom",
                is_feature_element,
            ),
        );
    }
    let feature_child_map = build_child_map(&feature_parent_map);

    let mut requirement_parent_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut requirement_specify_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut feature_to_requirements_map: HashMap<String, Vec<String>> = HashMap::new();

    for requirement_id in &requirement_ids {
        requirement_parent_map.insert(
            requirement_id.clone(),
            parent_ids_by_relation_and_type(
                registry,
                requirement_id,
                "derivedFrom",
                is_requirement_element,
            ),
        );

        let specified_features = parent_ids_by_relation_and_type(
            registry,
            requirement_id,
            "specify",
            is_feature_element,
        );
        for feature_id in &specified_features {
            feature_to_requirements_map
                .entry(feature_id.clone())
                .or_default()
                .push(requirement_id.clone());
        }
        requirement_specify_map.insert(requirement_id.clone(), specified_features);
    }

    for requirements in feature_to_requirements_map.values_mut() {
        requirements.sort();
    }
    let requirement_child_map = build_child_map(&requirement_parent_map);

    let mut feature_memo: HashMap<String, BTreeSet<String>> = HashMap::new();
    let mut root_features = BTreeSet::new();
    for feature_id in &feature_ids {
        let roots = resolve_feature_roots(
            feature_id,
            &feature_parent_map,
            &mut feature_memo,
            &mut HashSet::new(),
        );
        for root in roots {
            root_features.insert(root);
        }
    }

    let mut requirement_memo: HashMap<String, BTreeSet<String>> = HashMap::new();
    let mut root_assignment: HashMap<String, String> = HashMap::new();
    let mut root_counts: BTreeMap<String, usize> = BTreeMap::new();

    for root_id in &root_features {
        root_counts.insert(root_id.clone(), 0);
    }

    for requirement_id in &requirement_ids {
        let roots = resolve_requirement_feature_roots(
            requirement_id,
            &requirement_parent_map,
            &requirement_specify_map,
            &feature_parent_map,
            &mut feature_memo,
            &mut requirement_memo,
            &mut HashSet::new(),
        );
        if let Some(root_id) = roots.first().cloned() {
            root_assignment.insert(requirement_id.clone(), root_id.clone());
            *root_counts.entry(root_id).or_insert(0) += 1;
        }
    }

    let submodels: Vec<SubmodelSummary> = root_counts
        .iter()
        .map(|(root_id, count)| {
            let root = registry.get_element(root_id).unwrap();
            SubmodelSummary {
                root_id: root_id.clone(),
                root_name: root.name.clone(),
                root_type: root.element_type.as_str().to_string(),
                requirement_count: *count,
            }
        })
        .collect();

    let mut couplings = Vec::new();
    for source_id in &requirement_ids {
        let Some(source_root_id) = root_assignment.get(source_id) else {
            continue;
        };
        let Some(source) = registry.get_element(source_id) else {
            continue;
        };
        let Some(source_root) = registry.get_element(source_root_id) else {
            continue;
        };

        for relation in &source.relations {
            if !relation.user_created {
                continue;
            }
            let LinkType::Identifier(target_identifier) = &relation.target.link else {
                continue;
            };

            let Some(target_id) =
                resolve_target_identifier(registry, &source.file_path, target_identifier)
            else {
                continue;
            };

            if !requirement_set.contains(&target_id) {
                continue;
            }

            let Some(target_root_id) = root_assignment.get(&target_id) else {
                continue;
            };
            if source_root_id == target_root_id {
                continue;
            }

            let Some(target) = registry.get_element(&target_id) else {
                continue;
            };
            let Some(target_root) = registry.get_element(target_root_id) else {
                continue;
            };

            couplings.push(CrossSubmodelCoupling {
                source_id: source.identifier.clone(),
                source_name: source.name.clone(),
                source_root_id: source_root_id.clone(),
                source_root_name: source_root.name.clone(),
                relation_type: relation.relation_type.name.to_string(),
                target_id: target.identifier.clone(),
                target_name: target.name.clone(),
                target_root_id: target_root_id.clone(),
                target_root_name: target_root.name.clone(),
            });
        }
    }

    couplings.sort_by(|a, b| {
        (&a.source_id, &a.relation_type, &a.target_id).cmp(&(
            &b.source_id,
            &b.relation_type,
            &b.target_id,
        ))
    });

    let mut report = SubmodelsReport {
        summary: SubmodelsSummary {
            total_submodels: submodels.len(),
            total_requirements: requirement_ids.len(),
            total_cross_submodel_couplings: couplings.len(),
        },
        submodels,
        cross_submodel_couplings: couplings,
    };

    if let Some(from_name) = from_name {
        let from_id = match registry.find_element_by_name(from_name) {
            Ok(id) => id,
            Err(ReqvireError::MissingElement(_)) | Err(ReqvireError::ElementNotFound(_)) => {
                return Err(ReqvireError::ElementNotFound(format!(
                    "Submodel root '{}' not found",
                    from_name
                )));
            }
            Err(other) => return Err(other),
        };
        let from_element = registry
            .get_element(&from_id)
            .ok_or_else(|| ReqvireError::ElementNotFound(from_id.clone()))?;

        if matches!(from_element.element_type, ElementType::Feature) {
            let scoped_requirements = collect_feature_subtree_requirements(
                &from_id,
                &feature_child_map,
                &feature_to_requirements_map,
                &requirement_child_map,
            );

            report.submodels = vec![SubmodelSummary {
                root_id: from_element.identifier.clone(),
                root_name: from_element.name.clone(),
                root_type: from_element.element_type.as_str().to_string(),
                requirement_count: scoped_requirements.len(),
            }];

            report.cross_submodel_couplings.retain(|coupling| {
                scoped_requirements.contains(&coupling.source_id)
                    || scoped_requirements.contains(&coupling.target_id)
            });

            report.summary = SubmodelsSummary {
                total_submodels: report.submodels.len(),
                total_requirements: scoped_requirements.len(),
                total_cross_submodel_couplings: report.cross_submodel_couplings.len(),
            };
        } else if matches!(from_element.element_type, ElementType::Requirement(_)) {
            let root_id = from_id;

            let mut scoped_nodes: HashSet<String> = HashSet::new();
            let mut stack = vec![root_id.clone()];
            while let Some(current) = stack.pop() {
                if !scoped_nodes.insert(current.clone()) {
                    continue;
                }
                if let Some(children) = requirement_child_map.get(&current) {
                    for child in children {
                        stack.push(child.clone());
                    }
                }
            }

            // The selected requirement defines scope boundary, but is not a reported submodel entry.
            scoped_nodes.remove(&root_id);

            let mut scoped_parent_map: HashMap<String, Vec<String>> = HashMap::new();
            for node_id in &scoped_nodes {
                let parents = requirement_parent_map
                    .get(node_id)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|p| scoped_nodes.contains(p))
                    .collect::<Vec<_>>();
                scoped_parent_map.insert(node_id.clone(), parents);
            }

            let mut scoped_memo: HashMap<String, BTreeSet<String>> = HashMap::new();
            let mut scoped_root_counts: BTreeMap<String, usize> = BTreeMap::new();
            for node_id in &scoped_nodes {
                let roots = resolve_requirement_subtree_roots(
                    node_id,
                    &scoped_parent_map,
                    &mut scoped_memo,
                    &mut HashSet::new(),
                );
                if let Some(scoped_root_id) = roots.first().cloned() {
                    *scoped_root_counts.entry(scoped_root_id).or_insert(0) += 1;
                }
            }

            report.submodels = scoped_root_counts
                .iter()
                .filter_map(|(scoped_root_id, count)| {
                    registry
                        .get_element(scoped_root_id)
                        .map(|elem| SubmodelSummary {
                            root_id: scoped_root_id.clone(),
                            root_name: elem.name.clone(),
                            root_type: elem.element_type.as_str().to_string(),
                            requirement_count: *count,
                        })
                })
                .collect();

            report.cross_submodel_couplings.retain(|coupling| {
                scoped_nodes.contains(&coupling.source_id)
                    || scoped_nodes.contains(&coupling.target_id)
            });

            let filtered_requirements: usize =
                report.submodels.iter().map(|s| s.requirement_count).sum();
            report.summary = SubmodelsSummary {
                total_submodels: report.submodels.len(),
                total_requirements: filtered_requirements,
                total_cross_submodel_couplings: report.cross_submodel_couplings.len(),
            };
        } else {
            return Err(ReqvireError::InvalidOperation(format!(
                "Submodel scope source '{}' must be a feature or requirement",
                from_name
            )));
        }
    }

    Ok(report)
}
