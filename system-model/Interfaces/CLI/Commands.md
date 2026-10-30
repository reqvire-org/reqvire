# Elements

### CLI Interface Structure

The CLI interface shall implement the clear `[OPTIONS] <COMMAND> [COMMAND OPTIONS]` structure.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Contract References
  * [Model Interface Operation Boundary Specification](../Specifications.md#model-interface-operation-boundary-specification)

#### Relations
  * definedBy: [CLI Interface Structure Contract Specification](Specifications.md#cli-interface-structure-contract-specification)
  * derive: [CLI Change Impact Report Command](#cli-change-impact-report-command)
  * derive: [CLI Collect Command](#cli-collect-command)
  * derive: [CLI Containment Command](#cli-containment-command)
  * derive: [CLI Coverage Command](#cli-coverage-command)
  * derive: [CLI Lint Command](#cli-lint-command)
  * derive: [CLI Migrate Command](#cli-migrate-command)
  * derive: [CLI Model Command](#cli-model-command)
  * derive: [CLI Ontologies Command](#cli-ontologies-command)
  * derive: [CLI Resources Command](#cli-resources-command)
  * derive: [CLI Search Command](#cli-search-command)
  * derive: [CLI Size Estimate JSON Option](#cli-size-estimate-json-option)
  * derive: [CLI Submodels Command](#cli-submodels-command)
  * derive: [CLI Traces Command](#cli-traces-command)
  * derive: [Explicit Workspace Selection](#explicit-workspace-selection)
  * derive: [Validate Command](#validate-command)
  * derivedFrom: [CLI interface](../InterfacesRequirements.md#cli-interface)
---

### CLI Change Impact Report Command

The system SHALL provide a command-line interface that compares a base workspace snapshot with the current workspace and emits the resulting change-impact report in the requested supported format.

#### Details
Command invocation, base-commit selection, default comparison scope and text/JSON output behavior SHALL follow the owned command contract. External automation may invoke that supported command; this requirement does not define additional selectors, depth controls, API endpoints or webhook services.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Workspace Scope Specification](../../ModelStructure/Specifications.md#workspace-scope-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Text Output Formatting](../../Reports/ModelReports/Specifications.md#text-output-formatting)
  * [Repo Change Impact Baseline Specification](../../Processing/ChangeImpact/Specifications.md#repo-change-impact-baseline-specification)
  * [Repo Change Impact Reporting Specification](../../Processing/ChangeImpact/Specifications.md#repo-change-impact-reporting-specification)

#### Relations
  * definedBy: [Change Propagation Behavior](Behaviors.md#change-propagation-behavior)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * verifiedBy: [CLI Git Commit Hash Flag Test](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-git-commit-hash-flag-test)
  * verifiedBy: [CLI Help Structure Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Change Impact Analysis Verification](../../Verifications/Processing/ChangeImpact/ChangeImpactVerifications.md#change-impact-analysis-verification)
  * verifiedBy: [Change Impact Detection Test](../../Verifications/Processing/ChangeImpact/ChangeImpactVerifications.md#change-impact-detection-test)
  * verifiedBy: [Change Impact Relations Test](../../Verifications/Processing/ChangeImpact/ChangeImpactVerifications.md#change-impact-relations-test)
---

### CLI Collect Command

The system shall provide a `collect` command that performs content collection as specified in the reused specifications.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Contract References
  * [Collect Content Specification](../../Reports/ModelReports/Specifications.md#collect-content-specification)
  * [Collect Output Format Specification](../../Reports/ModelReports/Specifications.md#collect-output-format-specification)

#### Relations
  * definedBy: [CLI Collect Command Contract Specification](Specifications.md#cli-collect-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * verifiedBy: [CLI Collect Command Test](../../Verifications/Reports/ModelReports/ReportingVerifications.md#cli-collect-command-test)
---

### CLI Collect Element Selection

When a collect command selects an existing root element, the system SHALL resolve an exact name or canonical identifier through the shared element-selection contract before collection.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Existing Element Selection Specification](../../ModelStructure/ElementSelection.md#existing-element-selection-specification)

#### Relations
  * derivedFrom: [CLI Collect Command](#cli-collect-command)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * satisfiedBy: [collect.rs](../../../crates/reqvire-core/src/report/collect.rs)
  * verifiedBy: [Existing Element Selection Verification](../../Verifications/ModelStructure/ElementSelectionVerifications.md#existing-element-selection-verification)
---

### CLI Containment Command

The system shall provide a `containment` command that emits JSON containment hierarchy data showing the folder/file/element hierarchy.

#### Details
The `containment` command shall:
- Output structured JSON by default
- Support `--short` flag to show only root elements (those without hierarchical parents in same file)
- Default behavior (without --short): show ALL elements in each file
- Exit with code 0 on success, non-zero on error
- Command syntax: `reqvire containment [--short]`

#### Metadata
  * type: requirement

#### Contract References
  * [ContainmentView](../WebExplorer/ContainmentView.md#containmentview)
  * [Model Browser and Graph Specification](../WebExplorer/Specifications.md#model-browser-and-graph-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [Short Mode Behavior](Behaviors.md#short-mode-behavior)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * satisfiedBy: [containment.rs](../../../crates/reqvire-core/src/containment.rs)
---

### CLI Coverage Command

The system shall provide a `coverage` command that generates both verification coverage and requirement implementation coverage reports.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Verification Type Selection Guidelines](../../ModelStructure/Specifications.md#verification-type-selection-guidelines)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Implementation Coverage Output Structure Specification](../../Reports/ModelReports/Specifications.md#implementation-coverage-output-structure-specification)
  * [Text Output Formatting](../../Reports/ModelReports/Specifications.md#text-output-formatting)

#### Relations
  * definedBy: [CLI Coverage Command Contract Specification](Specifications.md#cli-coverage-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * verifiedBy: [CLI Help Structure Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Verification Coverage Report Test](../../Verifications/Reports/ModelReports/ReportingVerifications.md#verification-coverage-report-test)
---

### CLI Coverage Scope Selection

When a user selects a capability for a coverage command, the system SHALL return the shared coverage report for that capability subtree.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Existing Element Selection Specification](../../ModelStructure/ElementSelection.md#existing-element-selection-specification)
  * [Coverage Scope Specification](../../Reports/ModelReports/Specifications.md#coverage-scope-specification)

#### Relations
  * derivedFrom: [CLI Coverage Command](#cli-coverage-command)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
---

### CLI JSON File Output Option

The system SHALL provide an `--output <FILE>` option on CLI JSON report commands, allowing report data to be written to a file instead of stdout.

#### Details
Implementation details shall follow the associated contract specifications.
Native artifact exporters retain their own serialization and file-publication contracts, including managed query export, semantic/concept export, and static Explorer export.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [CLI JSON File Output Option Contract Specification](Specifications.md#cli-json-file-output-option-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * verifiedBy: [CLI JSON File Output Test](../../Verifications/Reports/ModelReports/ReportingVerifications.md#cli-json-file-output-test)
---

### CLI Lint Command

When command-line lint inspection is requested, the system SHALL report categorized model-quality findings without modifying model sources and SHALL reject repair arguments with MCP guidance.

#### Metadata
  * type: requirement

#### Contract References
  * [Model Interface Operation Boundary Specification](../Specifications.md#model-interface-operation-boundary-specification)
  * [Cross-Submodel Hierarchical Relation Detection Specification](../../Operations/Linting/Specifications.md#cross-submodel-hierarchical-relation-detection-specification)
  * [Lint Output Specification](../../Operations/Linting/Specifications.md#lint-output-specification)
  * [Multi-Branch Convergence Detection Specification](../../Operations/Linting/Specifications.md#multi-branch-convergence-detection-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Text Output Formatting](../../Reports/ModelReports/Specifications.md#text-output-formatting)

#### Relations
  * definedBy: [CLI Lint Command Contract Specification](Specifications.md#cli-lint-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
---

### CLI Migrate Command

The system shall provide a `migrate` command that previews or applies deterministic source migrations for known breaking model-contract changes.

#### Details
Implementation details shall follow the associated contract specifications. The command shall default to dry-run preview mode and require `--fix` to write changes.

#### Metadata
  * type: requirement

#### Contract References
  * [Diff Output Format Specification](../Specifications.md#diff-output-format-specification)
  * [Model Interface Operation Boundary Specification](../Specifications.md#model-interface-operation-boundary-specification)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [Supported Element Types Specification](../../ModelStructure/Specifications.md#supported-element-types-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [CLI Migrate Command Contract Specification](Specifications.md#cli-migrate-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * satisfiedBy: [mod.rs](../../../crates/reqvire-core/src/migrations/mod.rs)
  * verifiedBy: [CLI Help Structure Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [CLI Migrate Command Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-migrate-command-verification)
---

### CLI Model Command

System shall provide a CLI command to generate structured model JSON with optional traversal filtering.

#### Details
Implementation details shall follow the associated contract specifications. The command shall support default model-root traversal, filtered traversal, reverse traversal, type filtering, and canonical JSON output.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Existing Element Selection Specification](../../ModelStructure/ElementSelection.md#existing-element-selection-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Model JSON Output Format Contract Specification](../../Reports/ModelReports/Specifications.md#model-json-output-format-contract-specification)
  * [Reverse Relation Traversal Behavior](../../Reports/ModelReports/Behaviors.md#reverse-relation-traversal-behavior)
  * [Start Element Type Filter Behavior](../../Reports/ModelReports/Behaviors.md#start-element-type-filter-behavior)
  * [Type Validation Error Behavior](../../Operations/Validation/Behaviors.md#type-validation-error-behavior)

#### Relations
  * definedBy: [CLI Model Command Contract Specification](Specifications.md#cli-model-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/report/model.rs)
  * verifiedBy: [CLI Help Structure Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Model Command Verification](../../Verifications/Reports/ModelReports/ReportingVerifications.md#model-command-verification)
---

### CLI Ontologies Command

The system shall provide semantic export CLI commands for selectable Reqvire semantic RDF layers and standalone native concept-scheme workflows.

#### Details
Detailed layer names, default layer behavior, Turtle/JSON-LD serialization, namespace filtering, concept command behavior, external-source boundary, output-file handling, and semantic-index reuse shall follow the associated command specification and semantic export contracts.

#### Metadata
  * type: requirement

#### Contract References
  * [Semantic Contract Structure Specification](../../ModelStructure/Specifications.md#semantic-contract-structure-specification)
  * [Ontology Collection Output Specification](../../Reports/ModelReports/Specifications.md#ontology-collection-output-specification)

#### Relations
  * definedBy: [CLI Ontologies Command Contract Specification](Specifications.md#cli-ontologies-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * satisfiedBy: [index.rs](../../../crates/reqvire-core/src/semantic_contract/index.rs)
  * verifiedBy: [CLI Help Structure Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [CLI Ontologies Command Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
---

### CLI Resources Command

The system SHALL provide a `resources` command that generates the local artifact inventory according to the shared resources report contract.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [CLI Resources Command Contract Specification](Specifications.md#cli-resources-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * verifiedBy: [Resources Report Verification](../../Verifications/Reports/ModelReports/ReportingVerifications.md#resources-report-verification)
---

### CLI Search Command

The system shall provide a unified search function, activated by the `search` root command, which shall search and report on model elements with comprehensive filtering capabilities.

#### Details
WHEN a caller requests contract-reference filtering, the system SHALL apply the shared search presence and target-filter semantics.

Implementation details shall follow the associated contract specifications. Search JSON shall expose parsed semantic ADT fields for ontology elements and semantic-contract elements when full results are requested.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [SearchFiltering](../../Reports/ModelReports/SearchFiltering.md#searchfiltering)
  * [Requirement Governance Metadata Specification](../../ModelStructure/Specifications.md#requirement-governance-metadata-specification)
  * [Supported Element Types Specification](../../ModelStructure/Specifications.md#supported-element-types-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Short Mode Behavior](Behaviors.md#short-mode-behavior)
  * [Text Output Formatting](../../Reports/ModelReports/Specifications.md#text-output-formatting)
  * [Type Validation Error Behavior](../../Operations/Validation/Behaviors.md#type-validation-error-behavior)

#### Relations
  * definedBy: [CLI Search Command Contract Specification](Specifications.md#cli-search-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * verifiedBy: [CLI Help Structure Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Contract Bindings Search Filters Verification](../../Verifications/Operations/ModelOperations/ContractBindingVerifications.md#contract-bindings-search-filters-verification)
  * verifiedBy: [Search Command Tests](../../Verifications/Reports/ModelReports/ReportingVerifications.md#search-command-tests)
---

### CLI Size Estimate JSON Option

The CLI shall provide an opt-in `--with-size-estimates` option for supported JSON report commands.

#### Details
Detailed model-build, eligible-command, JSON evidence, and output-format selection rules shall follow the associated specification.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [CLI Size Estimate JSON Option Specification](Specifications.md#cli-size-estimate-json-option-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * verifiedBy: [CLI Size Estimate JSON Option Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-size-estimate-json-option-verification)
---

### CLI Submodels Command

The system shall provide a `submodels` command that reports independent capability-rooted hierarchies and cross-submodel requirement couplings.

#### Details
The command shall support:
- `--from <NAME>` to select one capability or requirement subtree by exact name or canonical identifier under the bound element-selection contract
- `--json` and `--output <FILE>` for machine-readable output
- In `--from` mode, selected capability scopes are listed as the scoped capability submodel; selected requirement scopes are treated as boundaries and are not listed as submodel entries

Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Existing Element Selection Specification](../../ModelStructure/ElementSelection.md#existing-element-selection-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Requirement Submodels Report Specification](../../Reports/ModelReports/Specifications.md#requirement-submodels-report-specification)

#### Relations
  * definedBy: [CLI Submodels Command Contract Specification](Specifications.md#cli-submodels-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * verifiedBy: [CLI Help Structure Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Submodels Report Verification](../../Verifications/Reports/ModelReports/ReportingVerifications.md#submodels-report-verification)
---

### CLI Traces Command

When verification traces are requested through the CLI, the system SHALL expose the shared upward trace report with its requirement hierarchy and owning capability context.

#### Details
The system SHALL obtain trace results and filter diagnostics from the shared trace operation.

#### Metadata
  * type: requirement

#### Contract References
  * [Type Validation Error Behavior](../../Operations/Validation/Behaviors.md#type-validation-error-behavior)
  * [Verification Trace Tree Construction](../../Verification/Traceability/Specifications.md#verification-trace-tree-construction)

#### Relations
  * definedBy: [CLI Traces Command Contract Specification](Specifications.md#cli-traces-command-contract-specification)
  * derive: [CLI Traces Result Presentation](#cli-traces-result-presentation)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * verifiedBy: [CLI Help Structure Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Verification Traces Filter Options Test](../../Verifications/Reports/ModelReports/ReportingVerifications.md#verification-traces-filter-options-test)
---

### CLI Traces Result Presentation

When the shared trace operation returns a report, the system SHALL serialize it as JSON to the selected output destination.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [CLI Traces Result Presentation Specification](Specifications.md#cli-traces-result-presentation-specification)
  * derivedFrom: [CLI Traces Command](#cli-traces-command)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
---

### Detailed Error Handling and Logging

When a supported command-line request fails, the system SHALL return contextual diagnostics and actionable guidance with a nonzero exit status while keeping logging separate from structured result output.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Validation Error Reporting Behavior](../../Operations/Validation/Behaviors.md#validation-error-reporting-behavior)

#### Relations
  * definedBy: [Detailed Error Handling and Logging Contract Specification](Specifications.md#detailed-error-handling-and-logging-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [main.rs](../../../crates/reqvire-cli/src/main.rs)
  * satisfiedBy: [error.rs](../../../crates/reqvire-core/src/error.rs)
---

### Explicit Workspace Selection

The system shall allow command invocations to select the Reqvire workspace explicitly without requiring the caller process to start inside the workspace directory.

#### Details
Detailed global option, startup ordering, normal/MCP command behavior, fallback behavior, and invalid-workspace handling shall follow the associated specification.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Workspace Scope Specification](../../ModelStructure/Specifications.md#workspace-scope-specification)

#### Relations
  * definedBy: [Explicit Workspace Selection Specification](Specifications.md#explicit-workspace-selection-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * verifiedBy: [Explicit Workspace Selection Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#explicit-workspace-selection-verification)
---

### Validate Command

The system shall provide a validation command that executes model validation and reports any issues found.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Two-Pass Validation Behavior](../../Operations/Validation/Behaviors.md#two-pass-validation-behavior)
  * [Validation Error Reporting Behavior](../../Operations/Validation/Behaviors.md#validation-error-reporting-behavior)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Error Message Format Specification](Specifications.md#error-message-format-specification)
  * [Contract Bindings Hierarchical Independence Constraint](../../ModelStructure/Constraints.md#contract-bindings-hierarchical-independence-constraint)
  * [Contract Bindings Satisfied Contract Constraint](../../ModelStructure/Constraints.md#contract-bindings-satisfied-contract-constraint)

#### Relations
  * definedBy: [Validate Command Contract Specification](Specifications.md#validate-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * verifiedBy: [CLI Help Structure Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Invalid Relations Test](../../Verifications/Operations/Validation/ValidationVerifications.md#invalid-relations-test)
---

### npm CLI Distribution

When Reqvire is invoked through its npm package, the system SHALL launch the packaged native CLI for the supported platform and safely reuse its extraction cache across concurrent invocations.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [npm CLI Launch Specification](Specifications.md#npm-cli-launch-specification)
  * derivedFrom: [CLI interface](../InterfacesRequirements.md#cli-interface)
  * satisfiedBy: [reqvire.js](../../../npm/reqvire/bin/reqvire.js)
---
