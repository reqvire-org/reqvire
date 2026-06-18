# Elements

### CLI Interface Structure

The CLI interface shall implement the clear `[OPTIONS] <COMMAND> [COMMAND OPTIONS]` structure.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [CLI Interface Structure Contract Specification](Specifications.md#cli-interface-structure-contract-specification)
  * derive: [Reused Contract Context Commands](#reused-contract-context-commands)
  * derive: [CLI Add Element Command](#cli-add-element-command)
  * derive: [CLI Change Impact Report Command](#cli-change-impact-report-command)
  * derive: [CLI Collect Command](#cli-collect-command)
  * derive: [CLI Containment Command](#cli-containment-command)
  * derive: [CLI Coverage Command](#cli-coverage-command)
  * derive: [CLI Diff Output](#cli-diff-output)
  * derive: [CLI Lint Command](#cli-lint-command)
  * derive: [CLI Merge Element Command](#cli-merge-element-command)
  * derive: [CLI Migrate Command](#cli-migrate-command)
  * derive: [CLI Model Diagram Command](#cli-model-diagram-command)
  * derive: [CLI Move Asset Command](#cli-move-asset-command)
  * derive: [CLI Move Element Command](#cli-move-element-command)
  * derive: [CLI Move File Command](#cli-move-file-command)
  * derive: [CLI Ontologies Command](#cli-ontologies-command)
  * derive: [CLI Relink Command](#cli-relink-command)
  * derive: [CLI Remove Asset Command](#cli-remove-asset-command)
  * derive: [CLI Remove Element Command](#cli-remove-element-command)
  * derive: [CLI Rename Element Command](#cli-rename-element-command)
  * derive: [CLI Resources Command](#cli-resources-command)
  * derive: [CLI Search Command](#cli-search-command)
  * derive: [CLI Size Estimate JSON Option](#cli-size-estimate-json-option)
  * derive: [CLI Submodels Command](#cli-submodels-command)
  * derive: [CLI Traces Command](#cli-traces-command)
  * derive: [Explicit Workspace Selection](#explicit-workspace-selection)
  * derive: [Format Command](#format-command)
  * derive: [Validate Command](#validate-command)
  * derivedFrom: [CLI interface](../InterfacesRequirements.md#cli-interface)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
---

### Reused Contract Context Commands

The system shall provide reused_contract_context management through the unified link/unlink commands using the 'reusesContract' keyword.

#### Details
Reused Contract Context management behavior:

**Reuse (via link):**
- Syntax: `reqvire link <element-name> reusesContract <target> [--dry-run] [--json] [--output <FILE>]`
- Target: Contract element identifier
- Create Reused Contract Context subsection if doesn't exist
- Add link to subsection with format `* [display-name](file.md#contract-id)`
- Skip if already reused (idempotent)
- Support many-to-many (same reused_contract_context to multiple elements)
- Support dry-run mode for preview
- Support structured JSON output and JSON file output

**Remove Reused Context (via unlink):**
- Syntax: `reqvire unlink <element-name> <target> [--dry-run] [--json] [--output <FILE>]`
- Auto-detects whether target is relation or reused_contract_context
- Remove link from Reused Contract Context subsection
- Remove subsection if no reused_contract_context remain
- Trigger change impact on element
- Support dry-run mode for preview
- Support structured JSON output and JSON file output

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [File Persistence Behavior](../../ModelStructure/Behaviors.md#file-persistence-behavior)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Reused Contract Context Hierarchical Independence Constraint](../../ModelStructure/Constraints.md#reused-contract-context-hierarchical-independence-constraint)
  * [Reused Contract Context Satisfied Contract Constraint](../../ModelStructure/Constraints.md#reused-contract-context-satisfied-contract-constraint)

#### Relations
  * definedBy: [Reused Contract Context Input Auto-Detection Behavior](Behaviors.md#reused-contract-context-input-auto-detection-behavior)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * verifiedBy: [Reuse Command Verification](../../Verifications/Operations/ModelOperations/ReusedContractContextVerifications.md#reuse-command-verification)
  * verifiedBy: [Remove Reused Context Command Verification](../../Verifications/Operations/ModelOperations/ReusedContractContextVerifications.md#remove reused context-command-verification)
---

### CLI Add Element Command

The system shall provide an `add` command to create new model elements by accepting element definition in Markdown format from stdin or the `--content` argument, validating the structure, and inserting it into the target file.

#### Details
Implementation details shall follow the associated contract specifications. Ontology rebasing under override is governed by the reused create-element workflow and ontology-aware mutation contracts.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Git Repository Scope Specification](../../ModelStructure/Specifications.md#git-repository-scope-specification)
  * [File Persistence Behavior](../../ModelStructure/Behaviors.md#file-persistence-behavior)
  * [Target Location Constraint](../../Operations/ModelOperations/Constraints.md#target-location-constraint)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [Create Element Override Behavior](../../Operations/ModelOperations/Behaviors.md#create-element-override-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Create Element Workflow Specification](../../Operations/ModelOperations/Specifications.md#create-element-workflow-specification)
  * [Reused Contract Context Hierarchical Independence Constraint](../../ModelStructure/Constraints.md#reused-contract-context-hierarchical-independence-constraint)
  * [Reused Contract Context Satisfied Contract Constraint](../../ModelStructure/Constraints.md#reused-contract-context-satisfied-contract-constraint)

#### Relations
  * definedBy: [CLI Add Element Command Contract Specification](Specifications.md#cli-add-element-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [CLI Add Element Test](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#cli-add-element-test)
---

### CLI Change Impact Report Command

The system shall provide a command-line interface for initiating change impact analysis and controlling output formats.

#### Details
Command invocation: `reqvire change-impact [OPTIONS]`

**Analysis Options**:
- `--git-commit <hash>`: Specify git commit hash for comparing changes
- Support analyzing changes between git commits
- Enable specifying elements to analyze by ID or pattern
- Allow limiting analysis to specific relation types
- Support depth limitations for large models

**Output Options**:
- `--json`: Output structured JSON impact data
- Default to formatted text reports
- Support Mermaid diagrams of impact trees
- Integrate with text, JSON, Mermaid, and Explorer-supported reporting workflows

**Integration Support**:
- Support integration with CI/CD pipelines
- Enable calling from external systems via API
- Support webhook triggers for automated analysis
- Allow scripting of analysis operations

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Text Output Formatting](../../Reports/ModelReports/Specifications.md#text-output-formatting)
  * [Explorer Mermaid Diagram Style Specification](../WebExplorer/Specifications.md#explorer-mermaid-diagram-style-specification)

#### Relations
  * definedBy: [Change Propagation Behavior](Behaviors.md#change-propagation-behavior)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
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

#### Reused Contract Context
  * [Collect Content Specification](../../Reports/ModelReports/Specifications.md#collect-content-specification)
  * [Collect Output Format Specification](../../Reports/ModelReports/Specifications.md#collect-output-format-specification)

#### Relations
  * definedBy: [CLI Collect Command Contract Specification](Specifications.md#cli-collect-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [CLI Collect Command Test](../../Verifications/Reports/ModelReports/ReportingVerifications.md#cli-collect-command-test)
---

### CLI Containment Command

The system shall provide a `containment` command to generate markdown output with the containment view diagram showing the folder/file/element hierarchy.

#### Details
The `containment` command shall:
- Output markdown format with header and embedded Mermaid flowchart diagram
- Support `--json` flag for structured JSON output
- Support `--short` flag to show only root elements (those without hierarchical parents in same file)
- Default behavior (without --short): show ALL elements in each file
- Exit with code 0 on success, non-zero on error
- Command syntax: `reqvire containment [--json] [--short]`

The markdown output shall include:
- `# Containment View` header
- Mermaid code block with flowchart diagram (using `graph TD` direction)
- Description indicating whether all elements or only root elements are displayed

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Explorer Mermaid Diagram Style Specification](../WebExplorer/Specifications.md#explorer-mermaid-diagram-style-specification)
  * [ContainmentView](../WebExplorer/ContainmentView.md#containmentview)
  * [Model Browser and Graph Specification](../WebExplorer/Specifications.md#model-browser-and-graph-specification)

#### Relations
  * definedBy: [Short Mode Behavior](Behaviors.md#short-mode-behavior)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [containment.rs](../../../core/src/containment.rs)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
---

### CLI Coverage Command

The system shall provide a `coverage` command that generates both verification coverage and requirement implementation coverage reports.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Verification Type Selection Guidelines](../../ModelStructure/Specifications.md#verification-type-selection-guidelines)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Implementation Coverage Output Structure Specification](../../Reports/ModelReports/Specifications.md#implementation-coverage-output-structure-specification)
  * [Text Output Formatting](../../Reports/ModelReports/Specifications.md#text-output-formatting)

#### Relations
  * definedBy: [CLI Coverage Command Contract Specification](Specifications.md#cli-coverage-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [CLI Help Structure Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Verification Coverage Report Test](../../Verifications/Reports/ModelReports/ReportingVerifications.md#verification-coverage-report-test)
---

### CLI Diff Output

The CLI shall provide a standardized diff-style output contract for commands that preview or report file modifications.

#### Details
The diff output contract shall define a shared presentation format for command results that show line-level file changes.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [diff.rs](../../../core/src/diff.rs)
---

### CLI JSON File Output Option

The system shall provide an `--output <FILE>` option on all commands that support `--json`, allowing JSON output to be written to a file instead of stdout.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [CLI JSON File Output Option Contract Specification](Specifications.md#cli-json-file-output-option-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [CLI JSON File Output Test](../../Verifications/Reports/ModelReports/ReportingVerifications.md#cli-json-file-output-test)
---

### CLI Lint Command

The system shall implement a `lint` command that analyzes model quality and detects issues in requirements relations, providing categorized output that distinguishes between auto-fixable issues and those requiring manual review.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Lint Output Specification](../../Operations/Linting/Specifications.md#lint-output-specification)
  * [Cross-Submodel Hierarchical Relation Detection Specification](../../Operations/Linting/Specifications.md#cross-submodel-hierarchical-relation-detection-specification)
  * [Text Output Formatting](../../Reports/ModelReports/Specifications.md#text-output-formatting)
  * [Multi-Branch Convergence Detection Specification](../../Operations/Linting/Specifications.md#multi-branch-convergence-detection-specification)

#### Relations
  * definedBy: [CLI Lint Command Contract Specification](Specifications.md#cli-lint-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
---

### CLI Merge Element Command

The system shall provide a `merge` command to combine multiple elements into a target element.

#### Details
Implementation details shall follow the associated contract specifications. Ontology merge behavior is governed by the reused merge content, compatibility, and workflow contracts.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Merge Content Transformation Behavior](../../Operations/ModelOperations/Behaviors.md#merge-content-transformation-behavior)
  * [Merge Type Compatibility Constraint](../../Operations/ModelOperations/Constraints.md#merge-type-compatibility-constraint)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Merge Element Workflow Specification](../../Operations/ModelOperations/Specifications.md#merge-element-workflow-specification)
  * [Reused Contract Context Hierarchical Independence Constraint](../../ModelStructure/Constraints.md#reused-contract-context-hierarchical-independence-constraint)
  * [Reused Contract Context Satisfied Contract Constraint](../../ModelStructure/Constraints.md#reused-contract-context-satisfied-contract-constraint)

#### Relations
  * definedBy: [CLI Merge Element Command Contract Specification](Specifications.md#cli-merge-element-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
---

### CLI Migrate Command

The system shall provide a `migrate` command that previews or applies deterministic source migrations for known breaking model-contract changes.

#### Details
Implementation details shall follow the associated contract specifications. The command shall default to dry-run preview mode and require `--fix` to write changes.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Supported Element Types Specification](../../ModelStructure/Specifications.md#supported-element-types-specification)

#### Relations
  * definedBy: [CLI Migrate Command Contract Specification](Specifications.md#cli-migrate-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [mod.rs](../../../core/src/migrations/mod.rs)
  * verifiedBy: [CLI Help Structure Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-help-structure-verification)
---

### CLI Model Diagram Command

System shall provide CLI command to generate model diagrams with optional filtering and output format selection.

#### Details
Implementation details shall follow the associated contract specifications. The command shall support default model-root traversal, filtered traversal, Markdown output, pure Mermaid output, and JSON output.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Explorer Mermaid Diagram Style Specification](../WebExplorer/Specifications.md#explorer-mermaid-diagram-style-specification)
  * [Model Diagram Output Formats Contract Specification](../../Reports/ModelReports/Specifications.md#model-diagram-output-formats-contract-specification)
  * [Reverse Relation Traversal Behavior](../../Reports/ModelReports/Behaviors.md#reverse-relation-traversal-behavior)
  * [Start Element Type Filter Behavior](../../Reports/ModelReports/Behaviors.md#start-element-type-filter-behavior)
  * [Type Validation Error Behavior](../../Operations/Validation/Behaviors.md#type-validation-error-behavior)

#### Relations
  * definedBy: [CLI Model Diagram Command Contract Specification](Specifications.md#cli-model-diagram-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * verifiedBy: [CLI Help Structure Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Model Command Verification](../../Verifications/Reports/ModelReports/ReportingVerifications.md#model-command-verification)
---

### CLI Move Asset Command

The system shall provide a `mv-asset` command to move or rename InternalPath files and automatically update all references across the model.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [File Persistence Behavior](../../ModelStructure/Behaviors.md#file-persistence-behavior)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [CLI Move Asset Command Contract Specification](Specifications.md#cli-move-asset-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [Move Asset Command Verification](../../Verifications/Operations/ModelOperations/ReusedContractContextVerifications.md#move-asset-command-verification)
---

### CLI Move Element Command

The system shall provide a `mv` command to move existing model elements to different file locations while automatically updating all relations that reference the moved element.

#### Details
The command shall reject moving an element into an existing `# Element` file when that move would create more than one element in the target document file.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Git Repository Scope Specification](../../ModelStructure/Specifications.md#git-repository-scope-specification)
  * [File Persistence Behavior](../../ModelStructure/Behaviors.md#file-persistence-behavior)
  * [Target Location Constraint](../../Operations/ModelOperations/Constraints.md#target-location-constraint)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Move Element Workflow Specification](../../Operations/ModelOperations/Specifications.md#move-element-workflow-specification)

#### Relations
  * definedBy: [CLI Move Element Command Contract Specification](Specifications.md#cli-move-element-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [CLI Move Element Test](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#cli-move-element-test)
  * verifiedBy: [Subdirectory Processing Verification](../../Verifications/Operations/Validation/ValidationVerifications.md#subdirectory-processing-verification)
---

### CLI Move File Command

The system shall provide a `mv-file` command to move entire specification files with all their elements to a new location.

#### Details
The command shall reject `mv-file --squash` when the target file is an existing `# Element` file.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [File Persistence Behavior](../../ModelStructure/Behaviors.md#file-persistence-behavior)
  * [Target Location Constraint](../../Operations/ModelOperations/Constraints.md#target-location-constraint)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [CLI Move File Command Contract Specification](Specifications.md#cli-move-file-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [CLI Move File Test](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#cli-move-file-test)
  * verifiedBy: [Subdirectory Processing Verification](../../Verifications/Operations/Validation/ValidationVerifications.md#subdirectory-processing-verification)
---

### CLI Ontologies Command

The system shall provide an `ontologies` command that collects all ontology `#### Ontology` and semantic-contract `#### Shapes` RDF blocks from the graph registry, emits generated ontology document declarations derived from root ontology metadata, supports optional local external ontology source inclusion, and supports an optional full semantic model projection. The command shall emit one `owl:Ontology` declaration per resolved `ontology_base`; ontology elements in the same base contribute vocabulary to that document, while cross-base ontology hierarchy may emit `owl:imports`.

#### Details
The command shall:
- Emit RDF/Turtle by default.
- Support `--jsonld` to emit JSON-LD instead of Turtle.
- Support `--full` to include RDF triples for Reqvire model elements, relations, reused_contract_context, concept references, ontology term declarations, semantic-contract shape references, and generated ontology projection facts.
- Support `--include-external` to include triples loaded from local ontology `#### External Ontology` source files.
- Support `--output <FILE>` to write the selected format to a file.
- Reuse the semantic index built from the graph registry instead of reparsing Turtle separately from validation.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Semantic Contract Structure Specification](../../ModelStructure/Specifications.md#semantic-contract-structure-specification)
  * [Ontology Collection Output Specification](../../Reports/ModelReports/Specifications.md#ontology-collection-output-specification)

#### Relations
  * definedBy: [CLI Ontologies Command Contract Specification](Specifications.md#cli-ontologies-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [semantic_contract.rs](../../../core/src/semantic_contract.rs)
  * verifiedBy: [CLI Help Structure Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [CLI Ontologies Command Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
---

### CLI Relink Command

The system shall provide a `relink` command that exposes the atomic relation relink operation.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Atomic Relation Relink Workflow Specification](../../Operations/ModelOperations/Specifications.md#atomic-relation-relink-workflow-specification)
  * [Atomic Relink Validity Constraint](../../Operations/ModelOperations/Constraints.md#atomic-relink-validity-constraint)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [CLI Relink Command Contract Specification](Specifications.md#cli-relink-command-contract-specification)
  * definedBy: [Mutating Command Hierarchy Safety Contract Specification](Specifications.md#mutating-command-hierarchy-safety-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * verifiedBy: [Atomic Relation Relink Test](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#atomic-relation-relink-test)
---

### CLI Remove Asset Command

The system shall provide an `rm-asset` command to remove InternalPath files and automatically remove all references from the model.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [File Persistence Behavior](../../ModelStructure/Behaviors.md#file-persistence-behavior)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [CLI Remove Asset Command Contract Specification](Specifications.md#cli-remove-asset-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [Remove Asset Command Verification](../../Verifications/Operations/ModelOperations/ReusedContractContextVerifications.md#remove-asset-command-verification)
---

### CLI Remove Element Command

The system shall provide an `rm` command to delete existing model elements and automatically remove all relations referencing the deleted element.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [File Persistence Behavior](../../ModelStructure/Behaviors.md#file-persistence-behavior)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Delete Element Workflow Specification](../../Operations/ModelOperations/Specifications.md#delete-element-workflow-specification)

#### Relations
  * definedBy: [CLI Remove Element Command Contract Specification](Specifications.md#cli-remove-element-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [CLI Remove Element Test](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#cli-remove-element-test)
---

### CLI Rename Element Command

The system shall provide a `rename` command to rename existing model elements while automatically updating all relations that reference the renamed element.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [File Persistence Behavior](../../ModelStructure/Behaviors.md#file-persistence-behavior)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [CLI Rename Element Command Contract Specification](Specifications.md#cli-rename-element-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [CLI Rename Element Test](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#cli-rename-element-test)
---

### CLI Resources Command

The system shall provide a `resources` command that generates a report showing all files referenced by the model through relations and reused_contract_context.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [CLI Resources Command Contract Specification](Specifications.md#cli-resources-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [Resources Report Verification](../../Verifications/Reports/ModelReports/ReportingVerifications.md#resources-report-verification)
---

### CLI Search Command

The system shall provide a unified search function, activated by the `search` root command, which shall search and report on model elements with comprehensive filtering capabilities.

#### Details
Implementation details shall follow the associated contract specifications. Search JSON shall expose parsed semantic ADT fields for ontology elements and semantic-contract elements when full results are requested.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Requirement Governance Metadata Specification](../../ModelStructure/Specifications.md#requirement-governance-metadata-specification)
  * [Supported Element Types Specification](../../ModelStructure/Specifications.md#supported-element-types-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Short Mode Behavior](Behaviors.md#short-mode-behavior)
  * [Text Output Formatting](../../Reports/ModelReports/Specifications.md#text-output-formatting)
  * [Type Validation Error Behavior](../../Operations/Validation/Behaviors.md#type-validation-error-behavior)

#### Relations
  * definedBy: [CLI Search Command Contract Specification](Specifications.md#cli-search-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [CLI Help Structure Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Reused Contract Context Search Filters Verification](../../Verifications/Operations/ModelOperations/ReusedContractContextVerifications.md#reused-contract-context-search-filters-verification)
  * verifiedBy: [Search Command Tests](../../Verifications/Reports/ModelReports/ReportingVerifications.md#search-command-tests)
---

### CLI Size Estimate JSON Option

The CLI shall provide an opt-in `--with-size-estimates` option for supported JSON report commands.

#### Details
- The option shall enable model building with element size estimates for the command invocation.
- The option shall be valid only for commands that emit JSON model evidence.
- The option shall require `--json` when used by CLI report commands.
- If the option is used without `--json`, the CLI shall fail with a clear error.
- The option shall not change non-JSON output behavior.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [CLI Size Estimate JSON Option Specification](Specifications.md#cli-size-estimate-json-option-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [CLI Size Estimate JSON Option Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-size-estimate-json-option-verification)
---

### CLI Submodels Command

The system shall provide a `submodels` command that reports independent capability-rooted hierarchies and cross-submodel requirement couplings.

#### Details
The command shall support:
- `--from <NAME>` to scope report output to one capability or requirement subtree by name
- `--json` and `--output <FILE>` for machine-readable output
- In `--from` mode, selected capability scopes are listed as the scoped capability submodel; selected requirement scopes are treated as boundaries and are not listed as submodel entries

Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Requirement Submodels Report Specification](../../Reports/ModelReports/Specifications.md#requirement-submodels-report-specification)

#### Relations
  * definedBy: [CLI Submodels Command Contract Specification](Specifications.md#cli-submodels-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [CLI Help Structure Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Submodels Report Verification](../../Verifications/Reports/ModelReports/ReportingVerifications.md#submodels-report-verification)
---

### CLI Traces Command

The system shall provide a `traces` command that generates and outputs upward trace trees for verification elements, showing the complete requirement hierarchy and owning capability context.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Verification Trace Tree Construction](../../Verification/Traceability/Specifications.md#verification-trace-tree-construction)
  * [Explorer Mermaid Diagram Style Specification](../WebExplorer/Specifications.md#explorer-mermaid-diagram-style-specification)
  * [Type Validation Error Behavior](../../Operations/Validation/Behaviors.md#type-validation-error-behavior)

#### Relations
  * definedBy: [CLI Traces Command Contract Specification](Specifications.md#cli-traces-command-contract-specification)
  * derive: [Verification Traces Element Navigation](#verification-traces-element-navigation)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [CLI Help Structure Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Verification Traces Filter Options Test](../../Verifications/Reports/ModelReports/ReportingVerifications.md#verification-traces-filter-options-test)
  * verifiedBy: [Verification Traces From-Folder Test](../../Verifications/Reports/ModelReports/ReportingVerifications.md#verification-traces-from-folder-test)
---

### Verification Traces Element Navigation

The system shall make verification element names in the traces report clickable links that navigate to the element's definition in its source file.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Verification Traces Element Navigation Contract Specification](Specifications.md#verification-traces-element-navigation-contract-specification)
  * derivedFrom: [CLI Traces Command](#cli-traces-command)
  * satisfiedBy: [verification_trace.rs](../../../core/src/verification_trace.rs)
  * verifiedBy: [Verification Traces Element Navigation Test](../../Verifications/Interfaces/CLI/CLIVerifications.md#verification-traces-element-navigation-test)
---

### Detailed Error Handling and Logging

The system shall implement detailed error handling and logging throughout the application to facilitate troubleshooting and provide meaningful feedback.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Validation Error Reporting Behavior](../../Operations/Validation/Behaviors.md#validation-error-reporting-behavior)

#### Relations
  * definedBy: [Detailed Error Handling and Logging Contract Specification](Specifications.md#detailed-error-handling-and-logging-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [error.rs](../../../core/src/error.rs)
---

### Explicit Workspace Selection

The system shall allow command invocations to select the Reqvire workspace explicitly without requiring the caller process to start inside the workspace directory.

#### Details
- The CLI shall provide a global workspace selection option.
- Workspace selection shall apply before model parsing, ignore-pattern loading, git root discovery, reporting, and mutation execution.
- Workspace selection shall preserve existing behavior when the option is not provided.
- Workspace selection shall apply consistently to normal CLI commands and MCP server startup.
- Workspace selection shall reject invalid workspace directories before executing a command.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Git Repository Scope Specification](../../ModelStructure/Specifications.md#git-repository-scope-specification)

#### Relations
  * definedBy: [Explicit Workspace Selection Specification](Specifications.md#explicit-workspace-selection-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [Explicit Workspace Selection Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#explicit-workspace-selection-verification)
---

### Format Command

The system shall provide a formatting function, activated by the (format command), which shall execute the formatting process upon user request.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [Format Command Contract Specification](Specifications.md#format-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [CLI Help Structure Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Element Ordering Verification](../../Verifications/Operations/Formatting/FormattingVerifications.md#element-ordering-verification)
  * verifiedBy: [Format Command Requirements Verification](../../Verifications/Operations/Formatting/FormattingVerifications.md#format-command-requirements-verification)
  * verifiedBy: [Full Relations Insertion Verification](../../Verifications/Operations/Formatting/FormattingVerifications.md#full-relations-insertion-verification)
---

### Relation Commands

The system shall provide unified CLI commands for relation and reused_contract_context management: link and unlink.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Relation Operations Specification](../../ModelStructure/Specifications.md#relation-operations-specification)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [Relation Commands Contract Specification](Specifications.md#relation-commands-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [Link Command Verification](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#link-command-verification)
  * verifiedBy: [Unlink Command Verification](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#unlink-command-verification)
---

### Validate Command

The system shall provide a validation command that executes model validation and reports any issues found.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Two-Pass Validation Behavior](../../Operations/Validation/Behaviors.md#two-pass-validation-behavior)
  * [Validation Error Reporting Behavior](../../Operations/Validation/Behaviors.md#validation-error-reporting-behavior)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)
  * [Error Message Format Specification](Specifications.md#error-message-format-specification)
  * [Reused Contract Context Hierarchical Independence Constraint](../../ModelStructure/Constraints.md#reused-contract-context-hierarchical-independence-constraint)
  * [Reused Contract Context Satisfied Contract Constraint](../../ModelStructure/Constraints.md#reused-contract-context-satisfied-contract-constraint)

#### Relations
  * definedBy: [Validate Command Contract Specification](Specifications.md#validate-command-contract-specification)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [CLI Help Structure Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Invalid Relations Test](../../Verifications/Operations/Validation/ValidationVerifications.md#invalid-relations-test)
---
