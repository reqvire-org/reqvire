use crate::element;
use crate::element::AttachmentTarget;
use crate::graph_registry::GraphRegistry;
use crate::relation;
use serde::Serialize;
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Serialize)]
pub struct CoverageReport {
    summary: CoverageSummary,
    verified_leaf_requirements: RequirementsByFile,
    unverified_leaf_requirements: RequirementsByFile,
    satisfied_test_verifications: VerificationsByFile,
    unsatisfied_test_verifications: VerificationsByFile,
    orphaned_verifications: VerificationsByFile,
    covered_requirements: CoveredRequirementsByFile,
    uncovered_requirements: UncoveredRequirementsByFile,
    capability_coverage: CapabilityCoverageByCapability,
}

#[derive(Serialize)]
struct CoverageSummary {
    // Leaf requirements metrics
    total_leaf_requirements: usize,
    verified_leaf_requirements: usize,
    unverified_leaf_requirements: usize,
    leaf_requirements_coverage_percentage: f64,

    // Test verifications metrics
    total_test_verifications: usize,
    satisfied_test_verifications: usize,
    unsatisfied_test_verifications: usize,
    test_verifications_satisfaction_percentage: f64,

    // Orphaned verifications metrics
    total_verifications: usize,
    orphaned_verifications: usize,
    orphaned_verifications_percentage: f64,

    // Verification types breakdown
    verification_types: VerificationTypeCounts,

    // Implementation coverage metrics
    total_requirements_in_scope: usize,
    covered_requirements: usize,
    uncovered_requirements: usize,
    implementation_coverage_percentage: f64,
    coverage_sources: CoverageSourceCounts,
}

#[derive(Serialize)]
struct VerificationTypeCounts {
    test: usize,
    formal_proof: usize,
    analysis: usize,
    inspection: usize,
    demonstration: usize,
}

#[derive(Serialize)]
struct CoverageSourceCounts {
    direct_satisfied: usize,
    refinement_contract_satisfied_via_attachment: usize,
    refinement_contract_satisfied_via_child: usize,
}

#[derive(Serialize)]
struct CapabilityCoverageByCapability {
    capabilities: Vec<CapabilityCoverageDetails>,
}

#[derive(Serialize, Clone)]
struct CapabilityCoverageDetails {
    identifier: String,
    name: String,
    local_leaf_requirements: usize,
    local_verified_leaf_requirements: usize,
    aggregate_leaf_requirements: usize,
    aggregate_verified_leaf_requirements: usize,
    verification_coverage_percentage: f64,
    local_requirements: usize,
    local_covered_requirements: usize,
    aggregate_requirements: usize,
    aggregate_covered_requirements: usize,
    implementation_coverage_percentage: f64,
    mark: String,
}

#[derive(Serialize)]
struct RequirementsByFile {
    files: HashMap<String, Vec<RequirementDetails>>,
}
#[derive(Serialize)]
struct VerificationsByFile {
    files: HashMap<String, Vec<VerificationDetails>>,
}
#[derive(Serialize)]
struct CoveredRequirementsByFile {
    files: HashMap<String, Vec<ImplementationCoveredRequirementDetails>>,
}
#[derive(Serialize)]
struct UncoveredRequirementsByFile {
    files: HashMap<String, Vec<ImplementationUncoveredRequirementDetails>>,
}

#[derive(Serialize, Clone)]
struct RequirementDetails {
    identifier: String,
    name: String,
    verified_by: Vec<String>,
}
#[derive(Serialize, Clone)]
struct VerificationDetails {
    identifier: String,
    name: String,
    verification_type: String,
    satisfied_by: Vec<String>,
}

#[derive(Serialize, Clone)]
struct ImplementationCoveredRequirementDetails {
    identifier: String,
    name: String,
    coverage_source: String,
    evidence: Vec<String>,
}

#[derive(Serialize, Clone)]
struct ImplementationUncoveredRequirementDetails {
    identifier: String,
    name: String,
}

/// Helper function to format an identifier as a markdown link
/// Splits identifier like "path/file.md#fragment" into proper link format
fn format_identifier_link(identifier: &str) -> String {
    if let Some(hash_pos) = identifier.rfind('#') {
        let file_part = &identifier[..hash_pos];
        let fragment_part = &identifier[hash_pos..];
        format!("[{}]({}{})", identifier, file_part, fragment_part)
    } else {
        format!("[{}]({})", identifier, identifier)
    }
}

fn round_to_two_decimals(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

fn find_directly_satisfied_descendant(
    start_requirement: &str,
    children_by_requirement: &HashMap<String, Vec<String>>,
    direct_satisfaction: &HashMap<String, Vec<String>>,
) -> Option<String> {
    let mut stack: Vec<String> = children_by_requirement
        .get(start_requirement)
        .cloned()
        .unwrap_or_default();

    // DFS traversal with deterministic ordering (children vectors are pre-sorted).
    while let Some(current) = stack.pop() {
        if direct_satisfaction.contains_key(&current) {
            return Some(current);
        }

        if let Some(children) = children_by_requirement.get(&current) {
            for child in children.iter().rev() {
                stack.push(child.clone());
            }
        }
    }

    None
}

impl CoverageReport {
    pub fn to_json_string(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    pub fn print(&self, json_output: bool) {
        if json_output {
            println!("{}", self.to_json_string());
        } else {
            print!("{}", self.format_text());
        }
    }

    pub fn format_text(&self) -> String {
        let mut output = String::new();

        // Summary
        output.push_str("## Summary\n\n");

        // Leaf Requirements Summary
        output.push_str("### Leaf Requirements\n\n");
        output.push_str(&format!(
            "- **Total Leaf Requirements:** {}\n",
            self.summary.total_leaf_requirements
        ));
        output.push_str(&format!(
            "- **Verified Leaf Requirements:** {} ({:.1}%)\n",
            self.summary.verified_leaf_requirements,
            self.summary.leaf_requirements_coverage_percentage
        ));
        output.push_str(&format!(
            "- **Unverified Leaf Requirements:** {}\n\n",
            self.summary.unverified_leaf_requirements
        ));

        // Test Verifications Summary
        output.push_str("### Test Verifications\n\n");
        output.push_str(&format!(
            "- **Total Test Verifications:** {}\n",
            self.summary.total_test_verifications
        ));
        output.push_str(&format!(
            "- **Satisfied Test Verifications:** {} ({:.1}%)\n",
            self.summary.satisfied_test_verifications,
            self.summary.test_verifications_satisfaction_percentage
        ));
        output.push_str(&format!(
            "- **Unsatisfied Test Verifications:** {}\n\n",
            self.summary.unsatisfied_test_verifications
        ));

        // Orphaned Verifications Summary
        output.push_str("### Orphaned Verifications\n\n");
        output.push_str(&format!(
            "- **Total Verifications:** {}\n",
            self.summary.total_verifications
        ));
        output.push_str(&format!(
            "- **Orphaned Verifications:** {} ({:.1}%)\n\n",
            self.summary.orphaned_verifications, self.summary.orphaned_verifications_percentage
        ));

        output.push_str("### Verification Types\n\n");
        output.push_str(&format!(
            "- Test: {}\n",
            self.summary.verification_types.test
        ));
        output.push_str(&format!(
            "- Formal Proof: {}\n",
            self.summary.verification_types.formal_proof
        ));
        output.push_str(&format!(
            "- Analysis: {}\n",
            self.summary.verification_types.analysis
        ));
        output.push_str(&format!(
            "- Inspection: {}\n",
            self.summary.verification_types.inspection
        ));
        output.push_str(&format!(
            "- Demonstration: {}\n\n",
            self.summary.verification_types.demonstration
        ));

        // Verified leaf requirements
        if !self.verified_leaf_requirements.files.is_empty() {
            output.push_str("## Verified Leaf Requirements\n\n");
            let mut sorted_files: Vec<_> = self.verified_leaf_requirements.files.iter().collect();
            sorted_files.sort_by_key(|(file, _)| *file);

            for (file, requirements) in sorted_files {
                output.push_str(&format!("### [{}]({})\n\n", file, file));
                let mut sorted_requirements = requirements.clone();
                sorted_requirements.sort_by(|a, b| a.name.cmp(&b.name));

                for requirement in sorted_requirements {
                    output.push_str(&format!(
                        "- ✅ **[{}]({})**\n",
                        requirement.name, requirement.identifier
                    ));
                    if !requirement.verified_by.is_empty() {
                        output.push_str("  - Verified by:\n");
                        for id in &requirement.verified_by {
                            output.push_str(&format!("    - {}\n", format_identifier_link(id)));
                        }
                    }
                }
                output.push('\n');
            }
        }

        // Unverified leaf requirements
        if !self.unverified_leaf_requirements.files.is_empty() {
            output.push_str("## Unverified Leaf Requirements\n\n");
            let mut sorted_files: Vec<_> = self.unverified_leaf_requirements.files.iter().collect();
            sorted_files.sort_by_key(|(file, _)| *file);

            for (file, requirements) in sorted_files {
                output.push_str(&format!("### [{}]({})\n\n", file, file));
                let mut sorted_requirements = requirements.clone();
                sorted_requirements.sort_by(|a, b| a.name.cmp(&b.name));

                for requirement in sorted_requirements {
                    output.push_str(&format!(
                        "- ❌ **[{}]({})**\n",
                        requirement.name, requirement.identifier
                    ));
                }
                output.push('\n');
            }
        }

        // Satisfied test verifications
        if !self.satisfied_test_verifications.files.is_empty() {
            output.push_str("## Satisfied Test Verifications\n\n");
            let mut sorted_files: Vec<_> = self.satisfied_test_verifications.files.iter().collect();
            sorted_files.sort_by_key(|(file, _)| *file);

            for (file, verifications) in sorted_files {
                output.push_str(&format!("### [{}]({})\n\n", file, file));
                let mut sorted_verifications = verifications.clone();
                sorted_verifications.sort_by(|a, b| a.name.cmp(&b.name));

                for verification in sorted_verifications {
                    output.push_str(&format!(
                        "- ✅ **[{}]({})** ({})\n",
                        verification.name, verification.identifier, verification.verification_type
                    ));
                    if !verification.satisfied_by.is_empty() {
                        output.push_str("  - Satisfied by:\n");
                        for id in &verification.satisfied_by {
                            output.push_str(&format!("    - {}\n", format_identifier_link(id)));
                        }
                    }
                }
                output.push('\n');
            }
        }

        // Unsatisfied test verifications
        if !self.unsatisfied_test_verifications.files.is_empty() {
            output.push_str("## Unsatisfied Test Verifications\n\n");
            let mut sorted_files: Vec<_> =
                self.unsatisfied_test_verifications.files.iter().collect();
            sorted_files.sort_by_key(|(file, _)| *file);

            for (file, verifications) in sorted_files {
                output.push_str(&format!("### [{}]({})\n\n", file, file));
                let mut sorted_verifications = verifications.clone();
                sorted_verifications.sort_by(|a, b| a.name.cmp(&b.name));

                for verification in sorted_verifications {
                    output.push_str(&format!(
                        "- ❌ **[{}]({})** ({})\n",
                        verification.name, verification.identifier, verification.verification_type
                    ));
                }
                output.push('\n');
            }
        }

        // Orphaned verifications
        if !self.orphaned_verifications.files.is_empty() {
            output.push_str("## Orphaned Verifications\n\n");
            let mut sorted_files: Vec<_> = self.orphaned_verifications.files.iter().collect();
            sorted_files.sort_by_key(|(file, _)| *file);

            for (file, verifications) in sorted_files {
                output.push_str(&format!("### [{}]({})\n\n", file, file));
                let mut sorted_verifications = verifications.clone();
                sorted_verifications.sort_by(|a, b| a.name.cmp(&b.name));

                for verification in sorted_verifications {
                    output.push_str(&format!(
                        "- ⚠️  **[{}]({})** ({})\n",
                        verification.name, verification.identifier, verification.verification_type
                    ));
                }
                output.push('\n');
            }
        }

        // Requirement implementation coverage
        output.push_str("### Requirement Implementation Coverage\n\n");
        output.push_str(&format!(
            "- **Total Requirements in Scope:** {}\n",
            self.summary.total_requirements_in_scope
        ));
        output.push_str(&format!(
            "- **Covered Requirements:** {} ({:.1}%)\n",
            self.summary.covered_requirements, self.summary.implementation_coverage_percentage
        ));
        output.push_str(&format!(
            "- **Uncovered Requirements:** {}\n\n",
            self.summary.uncovered_requirements
        ));

        output.push_str("#### Coverage Sources\n\n");
        output.push_str(&format!(
            "- direct_satisfied: {}\n",
            self.summary.coverage_sources.direct_satisfied
        ));
        output.push_str(&format!(
            "- refinement_contract_satisfied_via_attachment: {}\n",
            self.summary
                .coverage_sources
                .refinement_contract_satisfied_via_attachment
        ));
        output.push_str(&format!(
            "- refinement_contract_satisfied_via_child: {}\n\n",
            self.summary
                .coverage_sources
                .refinement_contract_satisfied_via_child
        ));

        if !self.covered_requirements.files.is_empty() {
            output.push_str("## Covered Requirements\n\n");
            let mut sorted_files: Vec<_> = self.covered_requirements.files.iter().collect();
            sorted_files.sort_by_key(|(file, _)| *file);

            for (file, requirements) in sorted_files {
                output.push_str(&format!("### [{}]({})\n\n", file, file));
                let mut sorted_requirements = requirements.clone();
                sorted_requirements.sort_by(|a, b| a.name.cmp(&b.name));

                for requirement in sorted_requirements {
                    output.push_str(&format!(
                        "- ✅ **[{}]({})** ({})\n",
                        requirement.name, requirement.identifier, requirement.coverage_source
                    ));
                    if !requirement.evidence.is_empty() {
                        output.push_str("  - Evidence:\n");
                        for id in &requirement.evidence {
                            output.push_str(&format!("    - {}\n", format_identifier_link(id)));
                        }
                    }
                }
                output.push('\n');
            }
        }

        if !self.uncovered_requirements.files.is_empty() {
            output.push_str("## Uncovered Requirements\n\n");
            let mut sorted_files: Vec<_> = self.uncovered_requirements.files.iter().collect();
            sorted_files.sort_by_key(|(file, _)| *file);

            for (file, requirements) in sorted_files {
                output.push_str(&format!("### [{}]({})\n\n", file, file));
                let mut sorted_requirements = requirements.clone();
                sorted_requirements.sort_by(|a, b| a.name.cmp(&b.name));

                for requirement in sorted_requirements {
                    output.push_str(&format!(
                        "- ❌ **[{}]({})**\n",
                        requirement.name, requirement.identifier
                    ));
                }
            }
        }

        if !self.capability_coverage.capabilities.is_empty() {
            output.push_str("\n## Capability Coverage\n\n");
            for capability in &self.capability_coverage.capabilities {
                output.push_str(&format!(
                    "- **[{}]({})**: {} verification {:.1}% ({}/{} leaf), implementation {:.1}% ({}/{} requirements)\n",
                    capability.name,
                    capability.identifier,
                    capability.mark,
                    capability.verification_coverage_percentage,
                    capability.aggregate_verified_leaf_requirements,
                    capability.aggregate_leaf_requirements,
                    capability.implementation_coverage_percentage,
                    capability.aggregate_covered_requirements,
                    capability.aggregate_requirements
                ));
            }
        }

        output
    }
}

pub fn generate_coverage_report(registry: &GraphRegistry) -> CoverageReport {
    // Initialize counters and data structures
    let mut total_leaf_requirements = 0;
    let mut verified_leaf_requirements = 0;
    let mut total_test_verifications = 0;
    let mut satisfied_test_verifications = 0;
    let mut total_verifications = 0;
    let mut orphaned_verifications_count = 0;
    let mut verification_types = VerificationTypeCounts {
        test: 0,
        formal_proof: 0,
        analysis: 0,
        inspection: 0,
        demonstration: 0,
    };

    let mut verified_leaf_files: HashMap<String, Vec<RequirementDetails>> = HashMap::new();
    let mut unverified_leaf_files: HashMap<String, Vec<RequirementDetails>> = HashMap::new();
    let mut verified_leaf_ids: HashSet<String> = HashSet::new();
    let mut unverified_leaf_ids: HashSet<String> = HashSet::new();
    let mut satisfied_test_files: HashMap<String, Vec<VerificationDetails>> = HashMap::new();
    let mut unsatisfied_test_files: HashMap<String, Vec<VerificationDetails>> = HashMap::new();
    let mut orphaned_verifications_files: HashMap<String, Vec<VerificationDetails>> =
        HashMap::new();
    let mut covered_requirements_files: HashMap<
        String,
        Vec<ImplementationCoveredRequirementDetails>,
    > = HashMap::new();
    let mut uncovered_requirements_files: HashMap<
        String,
        Vec<ImplementationUncoveredRequirementDetails>,
    > = HashMap::new();

    // First pass: collect all verification counts
    for element in registry.get_all_elements() {
        if let element::ElementType::Verification(verification_type) = &element.element_type {
            total_verifications += 1;

            // Check if this verification has any verify relations
            let verify_relations: Vec<String> = element
                .relations
                .iter()
                .filter(|r| r.relation_type.name == "verify")
                .map(|r| match &r.target.link {
                    relation::LinkType::Identifier(id) => id.clone(),
                    relation::LinkType::ExternalUrl(url) => url.clone(),
                    relation::LinkType::InternalPath(path) => path.to_string_lossy().to_string(),
                })
                .collect();

            // Count by verification type
            match verification_type {
                element::VerificationType::Default
                | element::VerificationType::Test
                | element::VerificationType::FormalProof => {
                    if matches!(verification_type, element::VerificationType::FormalProof) {
                        verification_types.formal_proof += 1;
                    } else {
                        verification_types.test += 1;
                    }
                    total_test_verifications += 1;

                    // For test verifications, check if they have satisfiedBy relations
                    let satisfied_by: Vec<String> = element
                        .relations
                        .iter()
                        .filter(|r| relation::is_satisfaction_relation(r.relation_type))
                        .map(|r| match &r.target.link {
                            relation::LinkType::Identifier(id) => id.clone(),
                            relation::LinkType::ExternalUrl(url) => url.clone(),
                            relation::LinkType::InternalPath(path) => {
                                path.to_string_lossy().to_string()
                            }
                        })
                        .collect();

                    let verification_details = VerificationDetails {
                        identifier: element.identifier.clone(),
                        name: element.name.clone(),
                        verification_type: element.element_type.as_str().to_string(),
                        satisfied_by: satisfied_by.clone(),
                    };

                    if satisfied_by.is_empty() {
                        // Unsatisfied test verification
                        unsatisfied_test_files
                            .entry(element.file_path.clone())
                            .or_default()
                            .push(verification_details);
                    } else {
                        // Satisfied test verification
                        satisfied_test_verifications += 1;
                        satisfied_test_files
                            .entry(element.file_path.clone())
                            .or_default()
                            .push(verification_details);
                    }
                }
                element::VerificationType::Analysis => {
                    verification_types.analysis += 1;
                }
                element::VerificationType::Inspection => {
                    verification_types.inspection += 1;
                }
                element::VerificationType::Demonstration => {
                    verification_types.demonstration += 1;
                }
            }

            // Check if this verification is orphaned (no verify relations)
            if verify_relations.is_empty() {
                orphaned_verifications_count += 1;
                let orphaned_details = VerificationDetails {
                    identifier: element.identifier.clone(),
                    name: element.name.clone(),
                    verification_type: element.element_type.as_str().to_string(),
                    satisfied_by: vec![], // Orphaned verifications don't need satisfied_by info here
                };
                orphaned_verifications_files
                    .entry(element.file_path.clone())
                    .or_default()
                    .push(orphaned_details);
            }
        }
    }

    // Third pass: implementation coverage (direct / refinement-contract via attachment / via child)
    let requirements: Vec<&element::Element> = registry
        .get_all_elements()
        .into_iter()
        .filter(|e| {
            matches!(
                e.element_type,
                element::ElementType::Requirement(element::RequirementType::System)
            )
        })
        .collect();

    let mut owned_refinements: HashMap<String, Vec<String>> = HashMap::new();
    let mut children_by_requirement: HashMap<String, Vec<String>> = HashMap::new();
    let mut attached_refinement_consumers: HashMap<String, Vec<String>> = HashMap::new();
    let mut direct_satisfaction: HashMap<String, Vec<String>> = HashMap::new();

    for req in &requirements {
        // Direct implementation evidence
        let mut satisfied_by_targets: Vec<String> = req
            .relations
            .iter()
            .filter(|r| relation::is_satisfaction_relation(r.relation_type))
            .map(|r| match &r.target.link {
                relation::LinkType::Identifier(id) => id.clone(),
                relation::LinkType::ExternalUrl(url) => url.clone(),
                relation::LinkType::InternalPath(path) => path.to_string_lossy().to_string(),
            })
            .collect();
        satisfied_by_targets.sort();
        satisfied_by_targets.dedup();
        if !satisfied_by_targets.is_empty() {
            direct_satisfaction.insert(req.identifier.clone(), satisfied_by_targets);
        }

        // Hierarchy edges for local child-evidence rule
        let mut children: Vec<String> = req
            .relations
            .iter()
            .filter(|r| r.relation_type.name == "derive")
            .filter_map(|r| match &r.target.link {
                relation::LinkType::Identifier(id) => Some(id.clone()),
                _ => None,
            })
            .collect();
        children.sort();
        children.dedup();
        children_by_requirement.insert(req.identifier.clone(), children);

        // Owned requirement contracts
        let mut refinements: Vec<String> = req
            .relations
            .iter()
            .filter(|r| r.relation_type.name == "definedBy")
            .filter_map(|r| match &r.target.link {
                relation::LinkType::Identifier(id) => Some(id.clone()),
                _ => None,
            })
            .collect();
        refinements.sort();
        refinements.dedup();
        owned_refinements.insert(req.identifier.clone(), refinements);

        // Refinement identifier attachments (consumer -> contract)
        for attachment in &req.attachments {
            if let AttachmentTarget::ElementIdentifier(id) = &attachment.target {
                attached_refinement_consumers
                    .entry(id.clone())
                    .or_default()
                    .push(req.identifier.clone());
            }
        }
    }

    for consumers in attached_refinement_consumers.values_mut() {
        consumers.sort();
        consumers.dedup();
    }

    let mut impl_coverage: HashMap<String, CoverageState> = HashMap::new();
    for req in &requirements {
        // direct_satisfied
        if let Some(evidence) = direct_satisfaction.get(&req.identifier) {
            impl_coverage.insert(
                req.identifier.clone(),
                CoverageState {
                    source: "direct_satisfied".to_string(),
                    evidence: evidence.clone(),
                },
            );
            continue;
        }

        let owns_refinement = owned_refinements
            .get(&req.identifier)
            .map(|v| !v.is_empty())
            .unwrap_or(false);
        if !owns_refinement {
            continue;
        }

        // refinement contract covered via attachment by directly satisfied requirement
        if let Some(refinements) = owned_refinements.get(&req.identifier) {
            let mut matched_consumer: Option<String> = None;
            for contract_id in refinements {
                if let Some(consumers) = attached_refinement_consumers.get(contract_id) {
                    if let Some(consumer) = consumers
                        .iter()
                        .find(|consumer_id| {
                            *consumer_id != &req.identifier
                                && direct_satisfaction.contains_key(*consumer_id)
                        })
                        .cloned()
                    {
                        matched_consumer = Some(consumer);
                        break;
                    }
                }
            }

            if let Some(consumer_id) = matched_consumer {
                impl_coverage.insert(
                    req.identifier.clone(),
                    CoverageState {
                        source: "refinement_contract_satisfied_via_attachment".to_string(),
                        evidence: vec![consumer_id],
                    },
                );
                continue;
            }
        }

        // refinement contract covered via directly satisfied descendant requirement
        if let Some(descendant_id) = find_directly_satisfied_descendant(
            &req.identifier,
            &children_by_requirement,
            &direct_satisfaction,
        ) {
            impl_coverage.insert(
                req.identifier.clone(),
                CoverageState {
                    source: "refinement_contract_satisfied_via_child".to_string(),
                    evidence: vec![descendant_id],
                },
            );
        }
    }

    let total_requirements_in_scope = requirements.len();
    let covered_requirements = impl_coverage.len();
    let uncovered_requirements = total_requirements_in_scope.saturating_sub(covered_requirements);

    let mut coverage_sources = CoverageSourceCounts {
        direct_satisfied: 0,
        refinement_contract_satisfied_via_attachment: 0,
        refinement_contract_satisfied_via_child: 0,
    };

    for req in &requirements {
        if let Some(state) = impl_coverage.get(&req.identifier) {
            match state.source.as_str() {
                "direct_satisfied" => coverage_sources.direct_satisfied += 1,
                "refinement_contract_satisfied_via_attachment" => {
                    coverage_sources.refinement_contract_satisfied_via_attachment += 1
                }
                "refinement_contract_satisfied_via_child" => {
                    coverage_sources.refinement_contract_satisfied_via_child += 1
                }
                _ => {}
            }

            covered_requirements_files
                .entry(req.file_path.clone())
                .or_default()
                .push(ImplementationCoveredRequirementDetails {
                    identifier: req.identifier.clone(),
                    name: req.name.clone(),
                    coverage_source: state.source.clone(),
                    evidence: state.evidence.clone(),
                });
        } else {
            uncovered_requirements_files
                .entry(req.file_path.clone())
                .or_default()
                .push(ImplementationUncoveredRequirementDetails {
                    identifier: req.identifier.clone(),
                    name: req.name.clone(),
                });
        }
    }

    // Second pass: identify leaf requirements and check their verification
    for element in registry.get_all_elements() {
        // Only process requirement-type elements
        if matches!(element.element_type, element::ElementType::Requirement(_)) {
            // Check if this is a leaf requirement (no forward relations to other requirements)
            let has_forward_relations = element.relations.iter().any(|relation| {
                // Check if relation is a forward relation to another requirement
                match relation.relation_type.name {
                    "contain" | "derive" | "definedBy" => {
                        // These are forward relations - check if target is a requirement
                        if let relation::LinkType::Identifier(_) = &relation.target.link {
                            // Assume it's a requirement if it's an identifier link
                            // This is a simplified check - in practice you'd resolve the target
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            });

            if !has_forward_relations {
                // This is a leaf requirement
                total_leaf_requirements += 1;

                // Check if it has verifiedBy relations
                let verified_by: Vec<String> = element
                    .relations
                    .iter()
                    .filter(|r| relation::is_verification_relation(r.relation_type))
                    .map(|r| match &r.target.link {
                        relation::LinkType::Identifier(id) => id.clone(),
                        relation::LinkType::ExternalUrl(url) => url.clone(),
                        relation::LinkType::InternalPath(path) => {
                            path.to_string_lossy().to_string()
                        }
                    })
                    .collect();

                let requirement_details = RequirementDetails {
                    identifier: element.identifier.clone(),
                    name: element.name.clone(),
                    verified_by: verified_by.clone(),
                };

                if verified_by.is_empty() {
                    // Unverified leaf requirement
                    unverified_leaf_ids.insert(element.identifier.clone());
                    unverified_leaf_files
                        .entry(element.file_path.clone())
                        .or_default()
                        .push(requirement_details);
                } else {
                    // Verified leaf requirement
                    verified_leaf_requirements += 1;
                    verified_leaf_ids.insert(element.identifier.clone());
                    verified_leaf_files
                        .entry(element.file_path.clone())
                        .or_default()
                        .push(requirement_details);
                }
            }
        }
    }

    // Calculate percentages
    let leaf_requirements_coverage_percentage = if total_leaf_requirements > 0 {
        (verified_leaf_requirements as f64 / total_leaf_requirements as f64) * 100.0
    } else {
        0.0
    };

    let test_verifications_satisfaction_percentage = if total_test_verifications > 0 {
        (satisfied_test_verifications as f64 / total_test_verifications as f64) * 100.0
    } else {
        0.0
    };

    let orphaned_verifications_percentage = if total_verifications > 0 {
        (orphaned_verifications_count as f64 / total_verifications as f64) * 100.0
    } else {
        0.0
    };

    let implementation_coverage_percentage = if total_requirements_in_scope > 0 {
        (covered_requirements as f64 / total_requirements_in_scope as f64) * 100.0
    } else {
        0.0
    };

    let capability_coverage = build_capability_coverage(
        registry,
        &children_by_requirement,
        &verified_leaf_ids,
        &unverified_leaf_ids,
        &impl_coverage,
    );

    let leaf_requirements_coverage_percentage =
        round_to_two_decimals(leaf_requirements_coverage_percentage);
    let test_verifications_satisfaction_percentage =
        round_to_two_decimals(test_verifications_satisfaction_percentage);
    let orphaned_verifications_percentage =
        round_to_two_decimals(orphaned_verifications_percentage);
    let implementation_coverage_percentage =
        round_to_two_decimals(implementation_coverage_percentage);

    CoverageReport {
        summary: CoverageSummary {
            total_leaf_requirements,
            verified_leaf_requirements,
            unverified_leaf_requirements: total_leaf_requirements - verified_leaf_requirements,
            leaf_requirements_coverage_percentage,

            total_test_verifications,
            satisfied_test_verifications,
            unsatisfied_test_verifications: total_test_verifications - satisfied_test_verifications,
            test_verifications_satisfaction_percentage,

            total_verifications,
            orphaned_verifications: orphaned_verifications_count,
            orphaned_verifications_percentage,

            verification_types,
            total_requirements_in_scope,
            covered_requirements,
            uncovered_requirements,
            implementation_coverage_percentage,
            coverage_sources,
        },
        verified_leaf_requirements: RequirementsByFile {
            files: verified_leaf_files,
        },
        unverified_leaf_requirements: RequirementsByFile {
            files: unverified_leaf_files,
        },
        satisfied_test_verifications: VerificationsByFile {
            files: satisfied_test_files,
        },
        unsatisfied_test_verifications: VerificationsByFile {
            files: unsatisfied_test_files,
        },
        orphaned_verifications: VerificationsByFile {
            files: orphaned_verifications_files,
        },
        covered_requirements: CoveredRequirementsByFile {
            files: covered_requirements_files,
        },
        uncovered_requirements: UncoveredRequirementsByFile {
            files: uncovered_requirements_files,
        },
        capability_coverage: CapabilityCoverageByCapability {
            capabilities: capability_coverage,
        },
    }
}

fn build_capability_coverage(
    registry: &GraphRegistry,
    children_by_requirement: &HashMap<String, Vec<String>>,
    verified_leaf_ids: &HashSet<String>,
    unverified_leaf_ids: &HashSet<String>,
    impl_coverage: &HashMap<String, CoverageState>,
) -> Vec<CapabilityCoverageDetails> {
    let mut capability_children: HashMap<String, Vec<String>> = HashMap::new();
    let mut capability_requirements: HashMap<String, Vec<String>> = HashMap::new();

    for element in registry.get_all_elements() {
        if !matches!(element.element_type, element::ElementType::Capability) {
            continue;
        }

        let mut child_capabilities = Vec::new();
        let mut specified_requirements = Vec::new();

        for relation in &element.relations {
            let relation::LinkType::Identifier(target_id) = &relation.target.link else {
                continue;
            };

            match relation.relation_type.name {
                "derive" => {
                    if registry.get_element(target_id).is_some_and(|target| {
                        matches!(target.element_type, element::ElementType::Capability)
                    }) {
                        child_capabilities.push(target_id.clone());
                    }
                }
                "specifiedBy" => {
                    if registry.get_element(target_id).is_some_and(|target| {
                        matches!(target.element_type, element::ElementType::Requirement(_))
                    }) {
                        specified_requirements.push(target_id.clone());
                    }
                }
                _ => {}
            }
        }

        child_capabilities.sort();
        child_capabilities.dedup();
        specified_requirements.sort();
        specified_requirements.dedup();

        capability_children.insert(element.identifier.clone(), child_capabilities);
        capability_requirements.insert(element.identifier.clone(), specified_requirements);
    }

    let mut result = Vec::new();
    for capability in registry.get_all_elements() {
        if !matches!(capability.element_type, element::ElementType::Capability) {
            continue;
        }

        let local_requirements = collect_requirement_subtree_ids(
            capability_requirements.get(&capability.identifier),
            children_by_requirement,
        );
        let aggregate_capabilities =
            collect_capability_subtree_ids(&capability.identifier, &capability_children);

        let mut aggregate_requirements = BTreeSet::new();
        for capability_id in &aggregate_capabilities {
            for req_id in collect_requirement_subtree_ids(
                capability_requirements.get(capability_id),
                children_by_requirement,
            ) {
                aggregate_requirements.insert(req_id);
            }
        }

        let local_leaf_requirements = count_leaf_requirements(
            local_requirements.iter(),
            verified_leaf_ids,
            unverified_leaf_ids,
        );
        let local_verified_leaf_requirements = local_requirements
            .iter()
            .filter(|id| verified_leaf_ids.contains(*id))
            .count();
        let aggregate_leaf_requirements = count_leaf_requirements(
            aggregate_requirements.iter(),
            verified_leaf_ids,
            unverified_leaf_ids,
        );
        let aggregate_verified_leaf_requirements = aggregate_requirements
            .iter()
            .filter(|id| verified_leaf_ids.contains(*id))
            .count();

        let local_covered_requirements = local_requirements
            .iter()
            .filter(|id| impl_coverage.contains_key(*id))
            .count();
        let aggregate_covered_requirements = aggregate_requirements
            .iter()
            .filter(|id| impl_coverage.contains_key(*id))
            .count();

        let verification_coverage_percentage = if aggregate_leaf_requirements > 0 {
            round_to_two_decimals(
                (aggregate_verified_leaf_requirements as f64 / aggregate_leaf_requirements as f64)
                    * 100.0,
            )
        } else {
            0.0
        };

        let implementation_coverage_percentage = if !aggregate_requirements.is_empty() {
            round_to_two_decimals(
                (aggregate_covered_requirements as f64 / aggregate_requirements.len() as f64)
                    * 100.0,
            )
        } else {
            0.0
        };

        let mark = if aggregate_leaf_requirements == 0 && aggregate_requirements.is_empty() {
            "not-applicable"
        } else if aggregate_leaf_requirements > 0
            && aggregate_verified_leaf_requirements == aggregate_leaf_requirements
            && aggregate_covered_requirements == aggregate_requirements.len()
        {
            "covered"
        } else if aggregate_verified_leaf_requirements > 0 || aggregate_covered_requirements > 0 {
            "partial"
        } else {
            "uncovered"
        };

        result.push(CapabilityCoverageDetails {
            identifier: capability.identifier.clone(),
            name: capability.name.clone(),
            local_leaf_requirements,
            local_verified_leaf_requirements,
            aggregate_leaf_requirements,
            aggregate_verified_leaf_requirements,
            verification_coverage_percentage,
            local_requirements: local_requirements.len(),
            local_covered_requirements,
            aggregate_requirements: aggregate_requirements.len(),
            aggregate_covered_requirements,
            implementation_coverage_percentage,
            mark: mark.to_string(),
        });
    }

    result.sort_by(|a, b| a.identifier.cmp(&b.identifier));
    result
}

#[derive(Clone)]
struct CoverageState {
    source: String,
    evidence: Vec<String>,
}

fn collect_requirement_subtree_ids(
    roots: Option<&Vec<String>>,
    children_by_requirement: &HashMap<String, Vec<String>>,
) -> BTreeSet<String> {
    let mut result = BTreeSet::new();
    let mut stack = roots.cloned().unwrap_or_default();
    stack.sort();

    while let Some(current) = stack.pop() {
        if !result.insert(current.clone()) {
            continue;
        }
        if let Some(children) = children_by_requirement.get(&current) {
            for child in children.iter().rev() {
                stack.push(child.clone());
            }
        }
    }

    result
}

fn collect_capability_subtree_ids(
    root: &str,
    capability_children: &HashMap<String, Vec<String>>,
) -> BTreeSet<String> {
    let mut result = BTreeSet::new();
    let mut stack = vec![root.to_string()];

    while let Some(current) = stack.pop() {
        if !result.insert(current.clone()) {
            continue;
        }
        if let Some(children) = capability_children.get(&current) {
            for child in children.iter().rev() {
                stack.push(child.clone());
            }
        }
    }

    result
}

fn count_leaf_requirements<'a, I>(
    requirement_ids: I,
    verified_leaf_ids: &HashSet<String>,
    unverified_leaf_ids: &HashSet<String>,
) -> usize
where
    I: Iterator<Item = &'a String>,
{
    requirement_ids
        .filter(|id| verified_leaf_ids.contains(*id) || unverified_leaf_ids.contains(*id))
        .count()
}
