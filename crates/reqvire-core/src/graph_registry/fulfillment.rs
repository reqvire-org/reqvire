use super::*;
use crate::element::ContractBindingTarget;

/// The required contributions shared by implementation coverage and validation.
#[derive(Default)]
pub struct RequirementDependencies {
    pub children: BTreeSet<String>,
    pub contract_consumers: BTreeSet<String>,
}

impl RequirementDependencies {
    pub fn all(&self) -> impl Iterator<Item = &String> {
        self.children.union(&self.contract_consumers)
    }
}

impl GraphRegistry {
    /// Resolve the complete requirement fulfillment graph from normalized relations.
    /// Artifact evidence never changes these edges or exempts a dependency cycle.
    pub(crate) fn requirement_fulfillment_dependencies(
        &self,
    ) -> BTreeMap<String, RequirementDependencies> {
        let mut graph: BTreeMap<String, RequirementDependencies> = self
            .nodes
            .values()
            .filter(|node| matches!(node.element.element_type, ElementType::Requirement(_)))
            .map(|node| {
                (
                    node.element.identifier.clone(),
                    RequirementDependencies::default(),
                )
            })
            .collect();
        let mut owners: FxHashMap<String, Vec<String>> = FxHashMap::default();

        for (id, contributions) in &mut graph {
            for relation in &self.nodes[id].element.relations {
                let LinkType::Identifier(target) = &relation.target.link else {
                    continue;
                };
                let Some(element) = self.get_element(target) else {
                    continue;
                };
                match relation.relation_type.name {
                    "derive" if matches!(element.element_type, ElementType::Requirement(_)) => {
                        contributions.children.insert(target.clone());
                    }
                    "definedBy" if element.element_type.is_requirement_contract() => {
                        owners.entry(target.clone()).or_default().push(id.clone());
                    }
                    _ => {}
                }
            }
        }
        for node in self.nodes.values() {
            if !matches!(node.element.element_type, ElementType::Requirement(_)) {
                continue;
            }
            for binding in &node.element.contract_bindings {
                let ContractBindingTarget::ElementIdentifier(contract) = &binding.target else {
                    continue;
                };
                for owner in owners.get(contract).into_iter().flatten() {
                    graph
                        .get_mut(owner)
                        .expect("contract owner is a requirement")
                        .contract_consumers
                        .insert(node.element.identifier.clone());
                }
            }
        }
        graph
    }

    /// Reject binding-only and mixed hierarchy/binding cycles before reads or writes.
    /// Pure hierarchy cycles remain diagnosed by the existing relation validator.
    pub(crate) fn validate_requirement_fulfillment_cycles(&self) -> Vec<ReqvireError> {
        let graph = self.requirement_fulfillment_dependencies();
        let adjacency: BTreeMap<_, Vec<_>> = graph
            .iter()
            .map(|(id, required)| (id.clone(), required.all().cloned().collect()))
            .collect();
        let mut completed = FxHashSet::default();
        let mut active = FxHashMap::default();
        let mut path: Vec<String> = Vec::new();
        let mut errors = Vec::new();

        // Iterative DFS preserves deterministic traversal without a recursion limit.
        for root in graph.keys() {
            if completed.contains(root) {
                continue;
            }
            active.insert(root.clone(), 0);
            path.push(root.clone());
            let mut stack = vec![(root.clone(), 0)];
            while let Some((id, next)) = stack.last_mut() {
                if *next == adjacency[id].len() {
                    completed.insert(id.clone());
                    active.remove(id);
                    path.pop();
                    stack.pop();
                    continue;
                }
                let target = adjacency[id][*next].clone();
                *next += 1;
                if let Some(&start) = active.get(&target) {
                    let mut cycle = path[start..].to_vec();
                    cycle.push(target);
                    if cycle
                        .windows(2)
                        .any(|edge| graph[&edge[0]].contract_consumers.contains(&edge[1]))
                    {
                        let mut description = cycle[0].clone();
                        for edge in cycle.windows(2) {
                            let kind = if graph[&edge[0]].contract_consumers.contains(&edge[1]) {
                                "contract consumer"
                            } else {
                                "child requirement"
                            };
                            description.push_str(&format!(" -[{kind}]-> {}", edge[1]));
                        }
                        errors.push(ReqvireError::CircularDependencyError(format!(
                            "requirement fulfillment {description}"
                        )));
                    }
                } else if !completed.contains(&target) {
                    active.insert(target.clone(), path.len());
                    path.push(target.clone());
                    stack.push((target, 0));
                }
            }
        }
        errors
    }
}
