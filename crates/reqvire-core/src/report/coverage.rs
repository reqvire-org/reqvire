use super::formatting::format_identifier_markdown_link;
use crate::element;
use crate::error::ReqvireError;
use crate::graph_registry::{GraphRegistry, RequirementDependencies};
use crate::relation;
use rustc_hash::{FxHashMap, FxHashSet};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Serialize, Clone)]
pub struct CoverageReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<CoverageScope>,
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

#[derive(Serialize, Clone, Default)]
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
    total_terminal_requirements: usize,
    covered_terminal_requirements: usize,
    uncovered_terminal_requirements: usize,
    implementation_coverage_percentage: f64,
    coverage_sources: CoverageSourceCounts,
}

#[derive(Serialize, Clone, Default)]
struct VerificationTypeCounts {
    test: usize,
    formal_proof: usize,
    analysis: usize,
    inspection: usize,
    demonstration: usize,
}

#[derive(Serialize, Clone, Default)]
struct CoverageSourceCounts {
    direct_satisfied: usize,
    requirement_rollup: usize,
    contract_consumer_rollup: usize,
    combined_rollup: usize,
}

impl VerificationTypeCounts {
    const fn record(&mut self, kind: &element::VerificationType) {
        match kind {
            element::VerificationType::Default | element::VerificationType::Test => self.test += 1,
            element::VerificationType::FormalProof => self.formal_proof += 1,
            element::VerificationType::Analysis => self.analysis += 1,
            element::VerificationType::Inspection => self.inspection += 1,
            element::VerificationType::Demonstration => self.demonstration += 1,
        }
    }
}

impl CoverageSourceCounts {
    fn record(&mut self, source: &str) {
        match source {
            "direct_satisfied" => self.direct_satisfied += 1,
            "requirement_rollup" => self.requirement_rollup += 1,
            "contract_consumer_rollup" => self.contract_consumer_rollup += 1,
            "combined_rollup" => self.combined_rollup += 1,
            _ => {}
        }
    }
}

#[derive(Serialize, Clone)]
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
    local_terminal_requirements: usize,
    local_covered_terminal_requirements: usize,
    aggregate_terminal_requirements: usize,
    aggregate_covered_terminal_requirements: usize,
    implementation_covered: bool,
    implementation_coverage_percentage: f64,
    mark: String,
}

#[derive(Serialize, Clone)]
struct RequirementsByFile {
    files: FxHashMap<String, Vec<RequirementDetails>>,
}
#[derive(Serialize, Clone)]
struct VerificationsByFile {
    files: FxHashMap<String, Vec<VerificationDetails>>,
}
#[derive(Serialize, Clone)]
struct CoveredRequirementsByFile {
    files: FxHashMap<String, Vec<ImplementationRequirementDetails>>,
}
#[derive(Serialize, Clone)]
struct UncoveredRequirementsByFile {
    files: FxHashMap<String, Vec<ImplementationRequirementDetails>>,
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
struct ImplementationRequirementDetails {
    identifier: String,
    name: String,
    coverage_source: String,
    is_terminal: bool,
    aggregate_leaf_requirements: usize,
    aggregate_verified_leaf_requirements: usize,
    aggregate_terminal_requirements: usize,
    aggregate_covered_terminal_requirements: usize,
    direct_evidence: Vec<String>,
    evidence: Vec<String>,
    contributing_requirements: Vec<String>,
    blocking_requirements: Vec<String>,
}

#[derive(Serialize, Clone)]
struct CoverageScope {
    kind: &'static str,
    capability_identifier: String,
    capability_name: String,
    capability_ids: BTreeSet<String>,
    requirement_ids: BTreeSet<String>,
    verification_ids: BTreeSet<String>,
    orphaned_verifications_scope: &'static str,
}

/// Compact generated-store entry. Evidence stays in the shared whole-model records.
#[derive(Serialize)]
pub struct CoverageScopeSummary {
    scope: CoverageScope,
    summary: CoverageSummary,
}

/// Operation-local indexes borrow the already classified records. In particular,
/// compact scope generation never copies evidence or projects detailed reports.
struct ScopeSummaryIndex<'a> {
    registry: &'a GraphRegistry,
    verified_leaf_ids: FxHashSet<&'a str>,
    unverified_leaf_ids: FxHashSet<&'a str>,
    covered: FxHashMap<&'a str, &'a ImplementationRequirementDetails>,
    uncovered: FxHashMap<&'a str, &'a ImplementationRequirementDetails>,
    satisfied_test_ids: FxHashSet<&'a str>,
    unsatisfied_test_ids: FxHashSet<&'a str>,
    verifications_by_requirement: FxHashMap<&'a str, Vec<&'a element::Element>>,
}

impl<'a> ScopeSummaryIndex<'a> {
    fn new(report: &'a CoverageReport, registry: &'a GraphRegistry) -> Self {
        let mut verifications_by_requirement: FxHashMap<_, Vec<_>> = FxHashMap::default();
        for node in registry.nodes.values() {
            let verification = &node.element;
            if !matches!(
                verification.element_type,
                element::ElementType::Verification(_)
            ) {
                continue;
            }
            for relation in &verification.relations {
                if relation.relation_type.name == "verify" {
                    if let relation::LinkType::Identifier(target) = &relation.target.link {
                        verifications_by_requirement
                            .entry(target.as_str())
                            .or_default()
                            .push(verification);
                    }
                }
            }
        }
        Self {
            registry,
            verified_leaf_ids: report
                .verified_leaf_requirements
                .files
                .values()
                .flatten()
                .map(|record| record.identifier.as_str())
                .collect(),
            unverified_leaf_ids: report
                .unverified_leaf_requirements
                .files
                .values()
                .flatten()
                .map(|record| record.identifier.as_str())
                .collect(),
            covered: report
                .covered_requirements
                .files
                .values()
                .flatten()
                .map(|record| (record.identifier.as_str(), record))
                .collect(),
            uncovered: report
                .uncovered_requirements
                .files
                .values()
                .flatten()
                .map(|record| (record.identifier.as_str(), record))
                .collect(),
            satisfied_test_ids: report
                .satisfied_test_verifications
                .files
                .values()
                .flatten()
                .map(|record| record.identifier.as_str())
                .collect(),
            unsatisfied_test_ids: report
                .unsatisfied_test_verifications
                .files
                .values()
                .flatten()
                .map(|record| record.identifier.as_str())
                .collect(),
            verifications_by_requirement,
        }
    }

    fn project(&self, capability: &element::Element) -> CoverageScopeSummary {
        let (capability_ids, requirement_ids) =
            super::submodels::capability_subtree_members(self.registry, &capability.identifier);
        let mut verifications = BTreeMap::new();
        let mut summary = CoverageSummary {
            total_requirements_in_scope: requirement_ids.len(),
            ..CoverageSummary::default()
        };
        for id in &requirement_ids {
            summary.verified_leaf_requirements +=
                usize::from(self.verified_leaf_ids.contains(id.as_str()));
            summary.unverified_leaf_requirements +=
                usize::from(self.unverified_leaf_ids.contains(id.as_str()));
            if let Some(record) = self.covered.get(id.as_str()) {
                summary.covered_requirements += 1;
                summary.covered_terminal_requirements += usize::from(record.is_terminal);
                summary.coverage_sources.record(&record.coverage_source);
            }
            if let Some(record) = self.uncovered.get(id.as_str()) {
                summary.uncovered_requirements += 1;
                summary.uncovered_terminal_requirements += usize::from(record.is_terminal);
            }
            for verification in self
                .verifications_by_requirement
                .get(id.as_str())
                .into_iter()
                .flatten()
            {
                verifications.insert(verification.identifier.as_str(), *verification);
            }
        }
        for (id, verification) in &verifications {
            summary.satisfied_test_verifications +=
                usize::from(self.satisfied_test_ids.contains(id));
            summary.unsatisfied_test_verifications +=
                usize::from(self.unsatisfied_test_ids.contains(id));
            if let element::ElementType::Verification(kind) = &verification.element_type {
                summary.verification_types.record(kind);
            }
        }
        summary.total_verifications = verifications.len();
        summary.total_leaf_requirements =
            summary.verified_leaf_requirements + summary.unverified_leaf_requirements;
        summary.leaf_requirements_coverage_percentage = percentage(
            summary.verified_leaf_requirements,
            summary.total_leaf_requirements,
        );
        summary.total_test_verifications =
            summary.satisfied_test_verifications + summary.unsatisfied_test_verifications;
        summary.test_verifications_satisfaction_percentage = percentage(
            summary.satisfied_test_verifications,
            summary.total_test_verifications,
        );
        summary.total_terminal_requirements =
            summary.covered_terminal_requirements + summary.uncovered_terminal_requirements;
        summary.implementation_coverage_percentage = percentage(
            summary.covered_terminal_requirements,
            summary.total_terminal_requirements,
        );
        CoverageScopeSummary {
            scope: CoverageScope {
                kind: "capability",
                capability_identifier: capability.identifier.clone(),
                capability_name: capability.name.clone(),
                capability_ids,
                requirement_ids,
                verification_ids: verifications.keys().map(|id| (*id).to_string()).collect(),
                orphaned_verifications_scope: "whole_model_only",
            },
            summary,
        }
    }
}

fn retain_records<T>(files: &mut FxHashMap<String, Vec<T>>, keep: impl Fn(&T) -> bool) {
    files.retain(|_, records| {
        records.retain(&keep);
        !records.is_empty()
    });
}

fn percentage(count: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        round_to_two_decimals(count as f64 * 100.0 / total as f64)
    }
}

/// Helper function to format an identifier as a markdown link
/// Splits identifier like "path/file.md#fragment" into proper link format
fn format_identifier_link(identifier: &str) -> String {
    format_identifier_markdown_link(identifier, identifier)
}

fn round_to_two_decimals(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

impl CoverageReport {
    /// Select subjects after whole-model evidence classification. No dependency graph is pruned.
    pub fn with_scope(
        self,
        registry: &GraphRegistry,
        name: Option<&str>,
    ) -> Result<Self, ReqvireError> {
        let Some(name) = name else {
            return Ok(self);
        };
        let identifier = registry
            .find_element_by_name(name)
            .map_err(|error| match error {
                ReqvireError::MissingElement(_) | ReqvireError::ElementNotFound(_) => {
                    ReqvireError::ElementNotFound(format!("Coverage capability '{name}' not found"))
                }
                other => other,
            })?;
        let capability = registry
            .get_element(&identifier)
            .ok_or_else(|| ReqvireError::ElementNotFound(identifier.clone()))?;
        if !matches!(capability.element_type, element::ElementType::Capability) {
            return Err(ReqvireError::InvalidOperation(format!(
                "Coverage scope '{name}' must be a capability"
            )));
        }
        Ok(self.project_capability(registry, capability))
    }

    pub fn scope_index(&self, registry: &GraphRegistry) -> BTreeMap<String, CoverageScopeSummary> {
        let index = ScopeSummaryIndex::new(self, registry);
        self.capability_coverage
            .capabilities
            .iter()
            .filter_map(|row| {
                let capability = registry.get_element(&row.identifier)?;
                Some((row.identifier.clone(), index.project(capability)))
            })
            .collect()
    }

    fn project_capability(
        mut self,
        registry: &GraphRegistry,
        capability: &element::Element,
    ) -> Self {
        let CoverageScopeSummary { scope, summary } =
            ScopeSummaryIndex::new(&self, registry).project(capability);
        retain_records(&mut self.verified_leaf_requirements.files, |item| {
            scope.requirement_ids.contains(&item.identifier)
        });
        retain_records(&mut self.unverified_leaf_requirements.files, |item| {
            scope.requirement_ids.contains(&item.identifier)
        });
        retain_records(&mut self.covered_requirements.files, |item| {
            scope.requirement_ids.contains(&item.identifier)
        });
        retain_records(&mut self.uncovered_requirements.files, |item| {
            scope.requirement_ids.contains(&item.identifier)
        });
        retain_records(&mut self.satisfied_test_verifications.files, |item| {
            scope.verification_ids.contains(&item.identifier)
        });
        retain_records(&mut self.unsatisfied_test_verifications.files, |item| {
            scope.verification_ids.contains(&item.identifier)
        });
        self.orphaned_verifications.files.clear();
        self.capability_coverage
            .capabilities
            .retain(|item| scope.capability_ids.contains(&item.identifier));
        self.summary = summary;
        self.scope = Some(scope);
        self
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string_pretty(&self).expect("failed to serialize JSON")
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

        if let Some(scope) = &self.scope {
            output.push_str(&format!("## Coverage scope: {}\n\nOrphan diagnostics are available in whole-model coverage only.\n\n", scope.capability_name));
        }

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
            "- **Covered Requirements:** {}\n",
            self.summary.covered_requirements
        ));
        output.push_str(&format!(
            "- **Uncovered Requirements:** {}\n",
            self.summary.uncovered_requirements
        ));
        output.push_str(&format!(
            "- **Total Terminal Requirements:** {}\n- **Covered Terminal Requirements:** {} ({:.1}%)\n- **Uncovered Terminal Requirements:** {}\n\n",
            self.summary.total_terminal_requirements,
            self.summary.covered_terminal_requirements,
            self.summary.implementation_coverage_percentage,
            self.summary.uncovered_terminal_requirements,
        ));

        output.push_str("#### Coverage Sources\n\n");
        output.push_str(&format!(
            "- direct_satisfied: {}\n",
            self.summary.coverage_sources.direct_satisfied
        ));
        output.push_str(&format!(
            "- requirement_rollup: {}\n",
            self.summary.coverage_sources.requirement_rollup
        ));
        output.push_str(&format!(
            "- contract_consumer_rollup: {}\n- combined_rollup: {}\n\n",
            self.summary.coverage_sources.contract_consumer_rollup,
            self.summary.coverage_sources.combined_rollup
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
                    format_implementation_evidence(&mut output, &requirement);
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
                        "- ❌ **[{}]({})** (uncovered)\n",
                        requirement.name, requirement.identifier
                    ));
                    format_implementation_evidence(&mut output, &requirement);
                }
            }
        }

        if !self.capability_coverage.capabilities.is_empty() {
            output.push_str("\n## Capability Coverage\n\n");
            for capability in &self.capability_coverage.capabilities {
                output.push_str(&format!(
                    "- **[{}]({})**: {} verification {:.1}% ({}/{} leaf), implementation {:.1}% ({}/{} terminal requirements), implementation {}\n",
                    capability.name,
                    capability.identifier,
                    capability.mark,
                    capability.verification_coverage_percentage,
                    capability.aggregate_verified_leaf_requirements,
                    capability.aggregate_leaf_requirements,
                    capability.implementation_coverage_percentage,
                    capability.aggregate_covered_terminal_requirements,
                    capability.aggregate_terminal_requirements,
                    if capability.aggregate_requirements == 0 { "not applicable" }
                    else if capability.implementation_covered { "covered" } else { "incomplete" }
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

    let mut verified_leaf_files: FxHashMap<String, Vec<RequirementDetails>> = FxHashMap::default();
    let mut unverified_leaf_files: FxHashMap<String, Vec<RequirementDetails>> =
        FxHashMap::default();
    let mut verified_leaf_ids: FxHashSet<String> = FxHashSet::default();
    let mut unverified_leaf_ids: FxHashSet<String> = FxHashSet::default();
    let mut satisfied_test_files: FxHashMap<String, Vec<VerificationDetails>> =
        FxHashMap::default();
    let mut unsatisfied_test_files: FxHashMap<String, Vec<VerificationDetails>> =
        FxHashMap::default();
    let mut orphaned_verifications_files: FxHashMap<String, Vec<VerificationDetails>> =
        FxHashMap::default();
    let mut covered_requirements_files: FxHashMap<String, Vec<ImplementationRequirementDetails>> =
        FxHashMap::default();
    let mut uncovered_requirements_files: FxHashMap<String, Vec<ImplementationRequirementDetails>> =
        FxHashMap::default();

    // First pass: collect all verification counts
    for element in registry.get_all_elements() {
        if let element::ElementType::Verification(verification_type) = &element.element_type {
            total_verifications += 1;

            // Check if this verification has any verify relations
            let has_verify_relation = element
                .relations
                .iter()
                .any(|r| r.relation_type.name == "verify");

            verification_types.record(verification_type);
            match verification_type {
                element::VerificationType::Default
                | element::VerificationType::Test
                | element::VerificationType::FormalProof => {
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
                element::VerificationType::Analysis
                | element::VerificationType::Inspection
                | element::VerificationType::Demonstration => {}
            }

            // Check if this verification is orphaned (no verify relations)
            if !has_verify_relation {
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

    // Assess the complete requirement graph before projecting any report scope.
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

    let fulfillment = registry.requirement_fulfillment_dependencies();
    let children_by_requirement: FxHashMap<String, Vec<String>> = fulfillment
        .iter()
        .map(|(id, dependencies)| (id.clone(), dependencies.children.iter().cloned().collect()))
        .collect();
    let mut direct_satisfaction: FxHashMap<String, Vec<String>> = FxHashMap::default();

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
    }

    let impl_coverage = evaluate_implementation_coverage(&fulfillment, &direct_satisfaction);
    let total_requirements_in_scope = requirements.len();
    let covered_requirements = impl_coverage
        .values()
        .filter(|state| state.is_covered())
        .count();
    let uncovered_requirements = total_requirements_in_scope - covered_requirements;
    let total_terminal_requirements = impl_coverage
        .values()
        .filter(|state| state.is_terminal)
        .count();
    let covered_terminal_requirements = impl_coverage
        .values()
        .filter(|state| state.is_terminal && state.is_covered())
        .count();
    let uncovered_terminal_requirements =
        total_terminal_requirements - covered_terminal_requirements;
    let mut coverage_sources = CoverageSourceCounts::default();

    // Second pass: identify leaf requirements and check their verification
    for element in registry.get_all_elements() {
        // Only process requirement-type elements
        if matches!(element.element_type, element::ElementType::Requirement(_)) {
            // Contract ownership and contract consumers do not change verification leaf status.
            if children_by_requirement
                .get(&element.identifier)
                .is_none_or(Vec::is_empty)
            {
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

    for req in &requirements {
        let state = &impl_coverage[&req.identifier];
        let subtree = collect_subtree_ids(vec![req.identifier.clone()], &children_by_requirement);
        let record = ImplementationRequirementDetails {
            identifier: req.identifier.clone(),
            name: req.name.clone(),
            coverage_source: state.source.to_string(),
            is_terminal: state.is_terminal,
            aggregate_leaf_requirements: count_leaf_requirements(
                subtree.iter(),
                &verified_leaf_ids,
                &unverified_leaf_ids,
            ),
            aggregate_verified_leaf_requirements: subtree
                .iter()
                .filter(|id| verified_leaf_ids.contains(*id))
                .count(),
            aggregate_terminal_requirements: state.terminal_requirements,
            aggregate_covered_terminal_requirements: state.covered_terminal_requirements,
            direct_evidence: direct_satisfaction
                .get(&req.identifier)
                .cloned()
                .unwrap_or_default(),
            evidence: state.evidence.clone(),
            contributing_requirements: state.contributing_requirements.clone(),
            blocking_requirements: state.blocking_requirements.clone(),
        };
        if state.is_covered() {
            coverage_sources.record(state.source);
            covered_requirements_files
                .entry(req.file_path.clone())
                .or_default()
                .push(record);
        } else {
            uncovered_requirements_files
                .entry(req.file_path.clone())
                .or_default()
                .push(record);
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

    let implementation_coverage_percentage =
        percentage(covered_terminal_requirements, total_terminal_requirements);

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
        scope: None,
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
            total_terminal_requirements,
            covered_terminal_requirements,
            uncovered_terminal_requirements,
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
    children_by_requirement: &FxHashMap<String, Vec<String>>,
    verified_leaf_ids: &FxHashSet<String>,
    unverified_leaf_ids: &FxHashSet<String>,
    impl_coverage: &FxHashMap<String, CoverageState>,
) -> Vec<CapabilityCoverageDetails> {
    let mut capability_children: FxHashMap<String, Vec<String>> = FxHashMap::default();
    let mut capability_requirements: FxHashMap<String, Vec<String>> = FxHashMap::default();

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
            .filter(|id| {
                impl_coverage
                    .get(*id)
                    .is_some_and(CoverageState::is_covered)
            })
            .count();
        let aggregate_covered_requirements = aggregate_requirements
            .iter()
            .filter(|id| {
                impl_coverage
                    .get(*id)
                    .is_some_and(CoverageState::is_covered)
            })
            .count();

        let verification_coverage_percentage = if aggregate_leaf_requirements > 0 {
            round_to_two_decimals(
                (aggregate_verified_leaf_requirements as f64 / aggregate_leaf_requirements as f64)
                    * 100.0,
            )
        } else {
            0.0
        };

        let local_terminal_requirements = local_requirements
            .iter()
            .filter(|id| {
                impl_coverage
                    .get(*id)
                    .is_some_and(|state| state.is_terminal)
            })
            .count();
        let local_covered_terminal_requirements = local_requirements
            .iter()
            .filter(|id| {
                impl_coverage
                    .get(*id)
                    .is_some_and(|state| state.is_terminal && state.is_covered())
            })
            .count();
        let aggregate_terminal_requirements = aggregate_requirements
            .iter()
            .filter(|id| {
                impl_coverage
                    .get(*id)
                    .is_some_and(|state| state.is_terminal)
            })
            .count();
        let aggregate_covered_terminal_requirements = aggregate_requirements
            .iter()
            .filter(|id| {
                impl_coverage
                    .get(*id)
                    .is_some_and(|state| state.is_terminal && state.is_covered())
            })
            .count();
        let implementation_coverage_percentage = percentage(
            aggregate_covered_terminal_requirements,
            aggregate_terminal_requirements,
        );
        let implementation_covered = !aggregate_requirements.is_empty()
            && aggregate_covered_requirements == aggregate_requirements.len();

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
            local_terminal_requirements,
            local_covered_terminal_requirements,
            aggregate_terminal_requirements,
            aggregate_covered_terminal_requirements,
            implementation_covered,
            implementation_coverage_percentage,
            mark: mark.to_string(),
        });
    }

    result.sort_by(|a, b| a.identifier.cmp(&b.identifier));
    result
}

#[derive(Clone)]
struct CoverageState {
    terminal_requirements: usize,
    covered_terminal_requirements: usize,
    source: &'static str,
    is_terminal: bool,
    evidence: Vec<String>,
    contributing_requirements: Vec<String>,
    blocking_requirements: Vec<String>,
}

impl CoverageState {
    fn is_covered(&self) -> bool {
        self.source != "uncovered"
    }
}

fn format_implementation_evidence(output: &mut String, record: &ImplementationRequirementDetails) {
    for (label, identifiers) in [
        ("Direct evidence", &record.direct_evidence),
        ("Supporting evidence", &record.evidence),
        (
            "Contributing requirements",
            &record.contributing_requirements,
        ),
        ("Blocking requirements", &record.blocking_requirements),
    ] {
        if !identifiers.is_empty() {
            output.push_str(&format!("  - {label}:\n"));
            for id in identifiers {
                output.push_str(&format!("    - {}\n", format_identifier_link(id)));
            }
        }
    }
}

/// Evaluate once over the full graph. Only implemented terminals seed coverage;
/// each other requirement waits for every child and every explicit contract consumer.
fn evaluate_implementation_coverage(
    fulfillment: &BTreeMap<String, RequirementDependencies>,
    direct_evidence: &FxHashMap<String, Vec<String>>,
) -> FxHashMap<String, CoverageState> {
    let mut dependencies: FxHashMap<String, BTreeSet<String>> = FxHashMap::default();
    let mut dependents: FxHashMap<String, Vec<String>> = FxHashMap::default();
    let mut remaining: FxHashMap<String, usize> = FxHashMap::default();
    let mut covered: FxHashSet<String> = FxHashSet::default();
    let mut ready = VecDeque::new();

    for (id, contributions) in fulfillment {
        let required: BTreeSet<String> = contributions.all().cloned().collect();
        for dependency in &required {
            dependents
                .entry(dependency.clone())
                .or_default()
                .push(id.clone());
        }
        remaining.insert(id.clone(), required.len());
        if required.is_empty()
            && direct_evidence
                .get(id)
                .is_some_and(|items| !items.is_empty())
        {
            covered.insert(id.clone());
            ready.push_back(id.clone());
        }
        dependencies.insert(id.clone(), required);
    }
    while let Some(id) = ready.pop_front() {
        for parent in dependents.get(&id).into_iter().flatten() {
            let count = remaining
                .get_mut(parent)
                .expect("every dependent is a requirement");
            *count -= 1;
            if *count == 0 && covered.insert(parent.clone()) {
                ready.push_back(parent.clone());
            }
        }
    }

    // Trace partial evidence as well as complete coverage. Iterative traversal
    // deduplicates shared paths and remains bounded even for an invalid cyclic graph.
    dependencies
        .iter()
        .map(|(id, required)| {
            let is_terminal = required.is_empty();
            let source = if !covered.contains(id) {
                "uncovered"
            } else if is_terminal {
                "direct_satisfied"
            } else {
                match (
                    !fulfillment[id].children.is_empty(),
                    !fulfillment[id].contract_consumers.is_empty(),
                ) {
                    (true, true) => "combined_rollup",
                    (true, false) => "requirement_rollup",
                    _ => "contract_consumer_rollup",
                }
            };
            let mut terminal_requirements = 0;
            let mut covered_terminal_requirements = 0;
            let mut evidence = BTreeSet::new();
            let mut blockers = BTreeSet::new();
            let mut visited = FxHashSet::default();
            let mut pending = vec![id];
            while let Some(current) = pending.pop() {
                if !visited.insert(current) {
                    continue;
                }
                evidence.extend(direct_evidence.get(current).into_iter().flatten().cloned());
                if current != id && !covered.contains(current) {
                    blockers.insert(current.clone());
                }
                if let Some(next) = dependencies.get(current) {
                    if next.is_empty() {
                        terminal_requirements += 1;
                        covered_terminal_requirements += usize::from(covered.contains(current));
                    }
                    pending.extend(next);
                }
            }
            if !covered.contains(id) && blockers.is_empty() {
                blockers.insert(id.clone());
            }
            (
                id.clone(),
                CoverageState {
                    terminal_requirements,
                    covered_terminal_requirements,
                    source,
                    is_terminal,
                    evidence: evidence.into_iter().collect(),
                    contributing_requirements: required.iter().cloned().collect(),
                    blocking_requirements: blockers.into_iter().collect(),
                },
            )
        })
        .collect()
}

fn collect_requirement_subtree_ids(
    roots: Option<&Vec<String>>,
    children_by_requirement: &FxHashMap<String, Vec<String>>,
) -> BTreeSet<String> {
    let mut stack = roots.cloned().unwrap_or_default();
    stack.sort();
    collect_subtree_ids(stack, children_by_requirement)
}

fn collect_capability_subtree_ids(
    root: &str,
    capability_children: &FxHashMap<String, Vec<String>>,
) -> BTreeSet<String> {
    collect_subtree_ids(vec![root.to_string()], capability_children)
}

fn collect_subtree_ids(
    mut stack: Vec<String>,
    children_by_parent: &FxHashMap<String, Vec<String>>,
) -> BTreeSet<String> {
    let mut result = BTreeSet::new();

    while let Some(current) = stack.pop() {
        if !result.insert(current.clone()) {
            continue;
        }
        if let Some(children) = children_by_parent.get(&current) {
            for child in children.iter().rev() {
                stack.push(child.clone());
            }
        }
    }

    result
}

fn count_leaf_requirements<'a, I>(
    requirement_ids: I,
    verified_leaf_ids: &FxHashSet<String>,
    unverified_leaf_ids: &FxHashSet<String>,
) -> usize
where
    I: Iterator<Item = &'a String>,
{
    requirement_ids
        .filter(|id| verified_leaf_ids.contains(*id) || unverified_leaf_ids.contains(*id))
        .count()
}
