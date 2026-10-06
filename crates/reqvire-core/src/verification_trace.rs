use crate::element::{Element, ElementType};
use crate::graph_registry::GraphRegistry;
use crate::relation::{LinkType, VERIFICATION_TRACES_RELATIONS, VERIFY_RELATION};
use rustc_hash::FxHashSet;
use serde::Serialize;
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Serialize)]
pub struct VerificationTracesReport {
    pub files: BTreeMap<String, FileVerifications>,
}

#[derive(Debug, Serialize)]
pub struct FileVerifications {
    pub verifications: Vec<VerificationTrace>,
}

#[derive(Debug, Serialize)]
pub struct VerificationTrace {
    pub identifier: String,
    pub name: String,
    pub file: String,
    #[serde(rename = "type")]
    pub verification_type: String,
    pub directly_verified_requirements: Vec<String>,
    pub trace_graph: TraceGraph,
    pub directly_verified_count: usize,
    pub total_requirements_in_tree: usize,
    #[serde(skip)]
    file_order_index: usize,
}

/// A canonical upward graph. Shared ancestors are never expanded into copies.
#[derive(Debug, Serialize)]
pub struct TraceGraph {
    pub nodes: Vec<TraceNode>,
    pub edges: Vec<TraceEdge>,
    #[cfg(test)]
    #[serde(skip)]
    inspected_relations: usize,
    #[cfg(test)]
    #[serde(skip)]
    expanded_nodes: usize,
}

#[derive(Debug, Serialize)]
pub struct TraceNode {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub element_type: String,
    pub is_directly_verified: bool,
}

#[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TraceEdge {
    pub source: String,
    pub relation_type: String,
    pub target: String,
}

pub struct VerificationTraceGenerator<'a> {
    registry: &'a GraphRegistry,
}

impl<'a> VerificationTraceGenerator<'a> {
    pub const fn new(registry: &'a GraphRegistry) -> Self {
        Self { registry }
    }

    /// Generate verification traces report
    pub fn generate(&self) -> VerificationTracesReport {
        let mut files: BTreeMap<String, FileVerifications> = BTreeMap::new();

        // Find all verification elements
        for element in self.registry.get_all_elements() {
            if matches!(element.element_type, ElementType::Verification(_)) {
                self.process_verification(element, &mut files);
            }
        }

        // Sort verification data for deterministic output
        // BTreeMap keeps files sorted alphabetically automatically
        for file_verifications in files.values_mut() {
            // Sort verifications by file_order_index for document order
            file_verifications.verifications.sort_by(|a, b| {
                a.file_order_index
                    .cmp(&b.file_order_index)
                    .then_with(|| a.identifier.cmp(&b.identifier))
            });
        }

        VerificationTracesReport { files }
    }

    /// Process a single verification element
    fn process_verification(
        &self,
        verification: &Element,
        files: &mut BTreeMap<String, FileVerifications>,
    ) {
        let mut directly_verified: Vec<String> = verification
            .relations
            .iter()
            .filter(|rel| rel.relation_type.name == VERIFY_RELATION)
            .filter_map(|rel| match &rel.target.link {
                LinkType::Identifier(id) => self.registry.get_element(id),
                _ => None,
            })
            .filter(|target| matches!(target.element_type, ElementType::Requirement(_)))
            .map(|target| target.identifier.clone())
            .collect();
        directly_verified.sort();
        directly_verified.dedup();
        if directly_verified.is_empty() {
            return;
        }

        let trace_graph = self.build_trace_graph(&directly_verified);
        let total_count = trace_graph
            .nodes
            .iter()
            .filter(|node| node.element_type == "requirement")
            .count();

        // Create verification trace
        let trace = VerificationTrace {
            identifier: verification.identifier.clone(),
            name: verification.name.clone(),
            file: verification.file_path.clone(),
            verification_type: verification.element_type.as_str().to_string(),
            directly_verified_count: directly_verified.len(),
            directly_verified_requirements: directly_verified,
            trace_graph,
            total_requirements_in_tree: total_count,
            file_order_index: verification.file_order_index,
        };

        // Add to file-level structure
        let file_entry = files
            .entry(verification.file_path.clone())
            .or_insert_with(|| FileVerifications {
                verifications: Vec::new(),
            });

        file_entry.verifications.push(trace);
    }

    fn build_trace_graph(&self, directly_verified: &[String]) -> TraceGraph {
        let direct: FxHashSet<&str> = directly_verified.iter().map(String::as_str).collect();
        let mut scheduled = direct.clone();
        let mut pending: VecDeque<&str> = directly_verified.iter().map(String::as_str).collect();
        let mut nodes = Vec::new();
        let mut edges = FxHashSet::default();
        #[cfg(test)]
        let mut inspected_relations = 0;
        #[cfg(test)]
        let mut expanded_nodes = 0;

        while let Some(id) = pending.pop_front() {
            let Some(element) = self.registry.get_element(id) else {
                continue;
            };
            #[cfg(test)]
            {
                expanded_nodes += 1;
            }
            nodes.push(TraceNode {
                id: id.to_owned(),
                name: element.name.clone(),
                element_type: element.element_type.as_str().to_owned(),
                is_directly_verified: direct.contains(id),
            });
            for relation in &element.relations {
                #[cfg(test)]
                {
                    inspected_relations += 1;
                }
                let LinkType::Identifier(target_id) = &relation.target.link else {
                    continue;
                };
                let Some(target) = self.registry.get_element(target_id) else {
                    continue;
                };
                let kind = relation.relation_type.name;
                let hierarchy = VERIFICATION_TRACES_RELATIONS.contains(&kind)
                    && matches!(
                        (&element.element_type, &target.element_type),
                        (ElementType::Requirement(_), ElementType::Requirement(_))
                            | (ElementType::Capability, ElementType::Capability)
                    );
                let ownership = kind == "specify"
                    && matches!(element.element_type, ElementType::Requirement(_))
                    && matches!(target.element_type, ElementType::Capability);
                if !hierarchy && !ownership {
                    continue;
                }

                // Always retain the relation, including edges into an already scheduled
                // ancestor. Only expansion is deduplicated, never split/merge paths.
                edges.insert(TraceEdge {
                    source: id.to_owned(),
                    relation_type: kind.to_owned(),
                    target: target_id.clone(),
                });
                if scheduled.insert(target_id.as_str()) {
                    pending.push_back(target_id);
                }
            }
        }
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        let mut edges: Vec<_> = edges.into_iter().collect();
        edges.sort();
        TraceGraph {
            nodes,
            edges,
            #[cfg(test)]
            inspected_relations,
            #[cfg(test)]
            expanded_nodes,
        }
    }
}

/// Valid verification types for --filter-type in traces command
pub const VERIFICATION_TYPES: &[&str] = &[
    "test-verification",
    "formal-proof-verification",
    "analysis-verification",
    "inspection-verification",
    "demonstration-verification",
];

/// Apply filters to verification traces report
pub fn apply_filters(
    mut report: VerificationTracesReport,
    filter_id: Option<&str>,
    filter_name: Option<&str>,
    filter_type: Option<&str>,
) -> Result<VerificationTracesReport, crate::error::ReqvireError> {
    use regex::Regex;

    // Validate verification type if provided
    if let Some(vtype) = filter_type {
        if !VERIFICATION_TYPES.contains(&vtype.to_lowercase().as_str()) {
            return Err(crate::error::ReqvireError::ProcessError(format!(
                "Invalid verification type '{}'. Valid types: {}",
                vtype,
                VERIFICATION_TYPES.join(", ")
            )));
        }
    }

    // Compile regex if name filter is provided
    let name_regex = if let Some(pattern) = filter_name {
        Some(Regex::new(pattern).map_err(crate::error::ReqvireError::from)?)
    } else {
        None
    };

    // Filter verifications in each file
    for file_verifications in report.files.values_mut() {
        file_verifications.verifications.retain(|v| {
            // Filter by ID
            if let Some(id) = filter_id {
                if v.identifier != id {
                    return false;
                }
            }

            // Filter by name regex
            if let Some(ref regex) = name_regex {
                if !regex.is_match(&v.name) {
                    return false;
                }
            }

            // Filter by type
            if let Some(vtype) = filter_type {
                if v.verification_type != vtype {
                    return false;
                }
            }

            true
        });
    }

    // Remove empty files
    report.files.retain(|_, f| !f.verifications.is_empty());

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::{RequirementType, VerificationType};
    use crate::graph_registry::RegistryNode;
    use crate::relation::{LinkType, Relation, RelationTarget, RELATION_TYPES};

    fn node(registry: &mut GraphRegistry, id: &str, kind: ElementType) {
        registry.nodes.insert(
            id.into(),
            RegistryNode {
                element: Element::new(id, id, "trace.md", 1, Some(kind)),
                relations: Vec::new(),
            },
        );
    }

    fn requirement(registry: &mut GraphRegistry, id: &str) {
        node(
            registry,
            id,
            ElementType::Requirement(RequirementType::System),
        );
    }

    fn edge(registry: &mut GraphRegistry, source: &str, kind: &'static str, target: &str) {
        registry
            .nodes
            .get_mut(source)
            .expect("test fixture operation should succeed")
            .element
            .relations
            .push(Relation {
                relation_type: RELATION_TYPES
                    .get(kind)
                    .expect("test fixture operation should succeed"),
                target: RelationTarget {
                    text: target.into(),
                    link: LinkType::Identifier(target.into()),
                    element_id: None,
                },
                user_created: true,
            });
    }

    fn diamond() -> GraphRegistry {
        let mut registry = GraphRegistry::new();
        for id in ["leaf", "left", "right", "root"] {
            requirement(&mut registry, id);
        }
        for id in ["cap", "cap-root"] {
            node(&mut registry, id, ElementType::Capability);
        }
        node(
            &mut registry,
            "check",
            ElementType::Verification(VerificationType::Test),
        );
        for (source, kind, target) in [
            ("check", "verify", "leaf"),
            ("check", "verify", "root"),
            ("leaf", "derivedFrom", "left"),
            ("leaf", "derivedFrom", "right"),
            ("left", "derivedFrom", "root"),
            ("right", "derivedFrom", "root"),
            ("root", "specify", "cap"),
            ("cap", "derivedFrom", "cap-root"),
        ] {
            edge(&mut registry, source, kind, target);
        }
        registry
    }

    #[test]
    fn reconvergent_trace_has_unique_nodes_all_edges_and_capability_context() {
        // Keep this pre-fix reproduction small: the old algorithm expands paths exponentially.
        let registry = diamond();
        let report = VerificationTraceGenerator::new(&registry).generate();
        let trace = serde_json::to_value(&report.files["trace.md"].verifications[0])
            .expect("test fixture operation should succeed");
        assert_eq!(trace["directly_verified_count"], 2);
        assert_eq!(trace["total_requirements_in_tree"], 4);
        let graph = &trace["trace_graph"];
        let nodes = graph["nodes"]
            .as_array()
            .expect("traces must use a normalized graph");
        assert_eq!(nodes.len(), 6);
        assert_eq!(
            graph["edges"]
                .as_array()
                .expect("expected an array in the test response")
                .len(),
            6
        );
        assert_eq!(
            nodes
                .iter()
                .filter(|n| n["is_directly_verified"] == true)
                .count(),
            2
        );
        assert!(nodes.iter().all(|n| n.get("children").is_none()));
        assert!(trace.get("trace_tree").is_none());
    }
    #[test]
    fn traversal_and_output_are_bounded_on_layered_reconvergent_graphs() {
        for layers in [20, 40] {
            let mut registry = GraphRegistry::new();
            node(
                &mut registry,
                "check",
                ElementType::Verification(VerificationType::Test),
            );
            let mut expected_edges = std::collections::BTreeSet::new();
            for layer in 0..layers {
                for side in ["a", "b"] {
                    let id = format!("{layer:02}-{side}");
                    requirement(&mut registry, &id);
                    if layer > 0 {
                        for parent_side in ["a", "b"] {
                            let parent = format!("{:02}-{parent_side}", layer - 1);
                            edge(&mut registry, &id, "derivedFrom", &parent);
                            expected_edges.insert((id.clone(), "derivedFrom".to_string(), parent));
                        }
                    }
                }
            }
            for side in ["a", "b"] {
                edge(
                    &mut registry,
                    "check",
                    "verify",
                    &format!("{:02}-{side}", layers - 1),
                );
            }
            let report = VerificationTraceGenerator::new(&registry).generate();
            let trace = &report.files["trace.md"].verifications[0];
            let graph = &trace.trace_graph;
            assert_eq!(trace.total_requirements_in_tree, 2 * layers);
            assert_eq!(trace.directly_verified_count, 2);
            assert_eq!(graph.nodes.len(), 2 * layers);
            assert_eq!(graph.expanded_nodes, graph.nodes.len());
            assert_eq!(graph.inspected_relations, 4 * (layers - 1));
            assert_eq!(graph.edges.len(), expected_edges.len());
            assert_eq!(
                graph
                    .edges
                    .iter()
                    .map(|e| (e.source.clone(), e.relation_type.clone(), e.target.clone()))
                    .collect::<std::collections::BTreeSet<_>>(),
                expected_edges
            );
            assert!(
                serde_json::to_vec(trace)
                    .expect("test fixture operation should succeed")
                    .len()
                    < 400 * (graph.nodes.len() + graph.edges.len())
            );
        }
    }

    #[test]
    fn direct_flags_duplicates_and_order_do_not_depend_on_the_traversal_path() {
        let registry = diamond();
        let expected = serde_json::to_value(VerificationTraceGenerator::new(&registry).generate())
            .expect("test fixture operation should succeed");
        let mut shuffled = registry;
        // Directly verified root is also shared by both branches. Duplicate edges
        // and reverse insertion order must not alter any graph record or counter.
        edge(&mut shuffled, "check", "verify", "root");
        edge(&mut shuffled, "right", "derivedFrom", "root");
        for node in shuffled.nodes.values_mut() {
            node.element.relations.reverse();
        }
        let actual = serde_json::to_value(VerificationTraceGenerator::new(&shuffled).generate())
            .expect("test fixture operation should succeed");
        assert_eq!(actual, expected);
        let nodes = actual["files"]["trace.md"]["verifications"][0]["trace_graph"]["nodes"]
            .as_array()
            .expect("expected an array in the test response");
        let direct: Vec<_> = nodes
            .iter()
            .filter(|n| n["is_directly_verified"] == true)
            .map(|n| {
                n["id"]
                    .as_str()
                    .expect("expected a string in the test response")
            })
            .collect();
        assert_eq!(direct, ["leaf", "root"]);
    }

    #[test]
    fn cycles_keep_closing_edges_and_invalid_targets_do_not_leak_into_traces() {
        let mut registry = diamond();
        edge(&mut registry, "root", "derivedFrom", "leaf");
        edge(&mut registry, "root", "derivedFrom", "root");
        edge(&mut registry, "root", "derivedFrom", "missing");
        edge(&mut registry, "root", "derivedFrom", "cap");
        edge(&mut registry, "leaf", "satisfiedBy", "check");
        edge(&mut registry, "check", "verify", "cap");
        edge(&mut registry, "check", "verify", "missing");
        node(
            &mut registry,
            "orphan",
            ElementType::Verification(VerificationType::Test),
        );
        node(
            &mut registry,
            "unresolved",
            ElementType::Verification(VerificationType::Test),
        );
        edge(&mut registry, "unresolved", "verify", "missing");
        node(
            &mut registry,
            "objective",
            ElementType::VerificationObjective,
        );
        edge(&mut registry, "objective", "verify", "root");
        let report = VerificationTraceGenerator::new(&registry).generate();
        let rows = &report.files["trace.md"].verifications;
        assert_eq!(rows.len(), 1);
        let trace = &rows[0];
        assert_eq!(trace.directly_verified_count, 2);
        assert_eq!(trace.total_requirements_in_tree, 4);
        assert_eq!(trace.trace_graph.nodes.len(), 6);
        assert_eq!(trace.trace_graph.expanded_nodes, 6);
        assert_eq!(trace.trace_graph.edges.len(), 8);
        for target in ["leaf", "root"] {
            assert!(trace
                .trace_graph
                .edges
                .iter()
                .any(|e| e.source == "root" && e.target == target));
        }
        assert!(trace.trace_graph.edges.iter().all(|edge| trace
            .trace_graph
            .nodes
            .iter()
            .any(|n| n.id == edge.source)
            && trace.trace_graph.nodes.iter().any(|n| n.id == edge.target)));
    }

    #[test]
    fn deep_traces_do_not_use_recursive_construction_counting_or_serialization() {
        let mut registry = GraphRegistry::new();
        let depth = 10_000;
        for index in 0..depth {
            let id = format!("r{index:05}");
            requirement(&mut registry, &id);
            if index > 0 {
                edge(
                    &mut registry,
                    &id,
                    "derivedFrom",
                    &format!("r{:05}", index - 1),
                );
            }
        }
        node(
            &mut registry,
            "check",
            ElementType::Verification(VerificationType::Test),
        );
        edge(
            &mut registry,
            "check",
            "verify",
            &format!("r{:05}", depth - 1),
        );
        let report = VerificationTraceGenerator::new(&registry).generate();
        let trace = &report.files["trace.md"].verifications[0];
        assert_eq!(trace.total_requirements_in_tree, depth);
        assert_eq!(trace.trace_graph.expanded_nodes, depth);
        assert_eq!(trace.trace_graph.inspected_relations, depth - 1);
        assert_eq!(trace.trace_graph.edges.len(), depth - 1);
        assert!(
            serde_json::to_vec(&report)
                .expect("test fixture operation should succeed")
                .len()
                < depth * 400
        );
    }

    #[test]
    fn empty_registry_and_orphans_have_no_trace_files() {
        let mut registry = GraphRegistry::new();
        assert!(VerificationTraceGenerator::new(&registry)
            .generate()
            .files
            .is_empty());
        node(
            &mut registry,
            "orphan",
            ElementType::Verification(VerificationType::Inspection),
        );
        assert!(VerificationTraceGenerator::new(&registry)
            .generate()
            .files
            .is_empty());
    }
}
