# Elements

### Attachment Commands

The system shall provide attachment management through the unified link/unlink commands using the 'attaching' keyword.

#### Details
Attachment management behavior:

**Attach (via link):**
- Syntax: `reqvire link <element-name> attaching <target> [--dry-run] [--json] [--output <FILE>]`
- Target: Refinement element identifier
- Create Attachments subsection if doesn't exist
- Add link to subsection with format `* [display-name](file.md#refinement-id)`
- Skip if already attached (idempotent)
- Support many-to-many (same attachment to multiple elements)
- Support dry-run mode for preview
- Support structured JSON output and JSON file output

**Detach (via unlink):**
- Syntax: `reqvire unlink <element-name> <target> [--dry-run] [--json] [--output <FILE>]`
- Auto-detects whether target is relation or attachment
- Remove link from Attachments subsection
- Remove subsection if no attachments remain
- Trigger change impact on element
- Support dry-run mode for preview
- Support structured JSON output and JSON file output

#### Metadata
  * type: requirement

#### Attachments
  * [File Persistence Behavior](../../Functional/Operations/Behaviors.md#file-persistence-behavior)
  * [Dry-Run Mode Behavior](../../Functional/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)
  * [Attachment Hierarchical Independence Constraint](../../Functional/Core/Constraints.md#attachment-hierarchical-independence-constraint)
  * [Attachment Satisfied Refinement Constraint](../../Functional/Core/Constraints.md#attachment-satisfied-refinement-constraint)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [Attachment Input Auto-Detection Behavior](../../Functional/Core/Behaviors.md#attachment-input-auto-detection-behavior)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * verifiedBy: [Attach Command Verification](../../Functional/Core/Verifications/AttachmentsVerifications.md#attach-command-verification)
  * verifiedBy: [Detach Command Verification](../../Functional/Core/Verifications/AttachmentsVerifications.md#detach-command-verification)
---

### CLI Interface Structure

The CLI interface shall implement the clear `[OPTIONS] <COMMAND> [COMMAND OPTIONS]` structure.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Attachment Commands](#attachment-commands)
  * derive: [CLI Diff Output](#cli-diff-output)
  * derive: [Explicit Workspace Selection](#explicit-workspace-selection)
  * derive: [CLI Add Element Command](#cli-add-element-command)
  * derive: [CLI Change Impact Report Command](#cli-change-impact-report-command)
  * derive: [CLI Collect Command](#cli-collect-command)
  * derive: [CLI Containment Command](#cli-containment-command)
  * derive: [CLI Coverage Command](#cli-coverage-command)
  * derive: [CLI Size Estimate JSON Option](#cli-size-estimate-json-option)
  * derive: [CLI Lint Command](#cli-lint-command)
  * derive: [CLI Merge Element Command](#cli-merge-element-command)
  * derive: [CLI Model Diagram Command](#cli-model-diagram-command)
  * derive: [CLI Move Asset Command](#cli-move-asset-command)
  * derive: [CLI Move Element Command](#cli-move-element-command)
  * derive: [CLI Move File Command](#cli-move-file-command)
  * derive: [CLI Relink Command](#cli-relink-command)
  * derive: [CLI Remove Asset Command](#cli-remove-asset-command)
  * derive: [CLI Remove Element Command](#cli-remove-element-command)
  * derive: [CLI Rename Element Command](#cli-rename-element-command)
  * derive: [CLI Resources Command](#cli-resources-command)
  * derive: [CLI Ontologies Command](#cli-ontologies-command)
  * derive: [CLI Search Command](#cli-search-command)
  * derive: [CLI Submodels Command](#cli-submodels-command)
  * derive: [CLI Traces Command](#cli-traces-command)
  * derive: [Format Command](#format-command)
  * derive: [Validate Command](#validate-command)
  * derivedFrom: [CLI interface](../Interfaces.md#cli-interface)
  * refinedBy: [CLI Interface Structure Refinement Specification](Specifications.md#cli-interface-structure-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
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

#### Attachments
  * [Git Repository Scope Specification](../../Functional/Core/Specifications.md#git-repository-scope-specification)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [Explicit Workspace Selection Specification](Specifications.md#explicit-workspace-selection-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [Explicit Workspace Selection Verification](Verifications/CLIVerifications.md#explicit-workspace-selection-verification)
---

### CLI Diff Output

The CLI shall provide a standardized diff-style output contract for commands that preview or report file modifications.

#### Details
The diff output contract shall define a shared presentation format for command results that show line-level file changes.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * satisfiedBy: [diff.rs](../../../core/src/diff.rs)
---

### CLI Add Element Command

The system shall provide an `add` command to create new model elements by accepting element definition in Markdown format from stdin or the `--content` argument, validating the structure, and inserting it into the target file.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Git Repository Scope Specification](../../Functional/Core/Specifications.md#git-repository-scope-specification)
  * [File Persistence Behavior](../../Functional/Operations/Behaviors.md#file-persistence-behavior)
  * [Target Location Constraint](../../Functional/Operations/Constraints.md#target-location-constraint)
  * [Dry-Run Mode Behavior](../../Functional/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Create Element Override Behavior](../../Functional/Operations/Behaviors.md#create-element-override-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)
  * [Create Element Workflow Specification](../../Functional/Operations/Specifications.md#create-element-workflow-specification)
  * [Attachment Hierarchical Independence Constraint](../../Functional/Core/Constraints.md#attachment-hierarchical-independence-constraint)
  * [Attachment Satisfied Refinement Constraint](../../Functional/Core/Constraints.md#attachment-satisfied-refinement-constraint)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI Add Element Command Refinement Specification](Specifications.md#cli-add-element-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [CLI Add Element Test](../../Functional/Operations/Verifications/ElementManipulationVerifications.md#cli-add-element-test)
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
- Integrate with HTML report generation

**Integration Support**:
- Support integration with CI/CD pipelines
- Enable calling from external systems via API
- Support webhook triggers for automated analysis
- Allow scripting of analysis operations

#### Metadata
  * type: requirement

#### Attachments
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)
  * [Text Output Formatting](../../Functional/Output/Specifications.md#text-output-formatting)
  * [Mermaid Diagram Style Specification](../../Functional/Output/Specifications.md#mermaid-diagram-style-specification)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [Change Propagation Behavior](../../Functional/Processing/Behaviors.md#change-propagation-behavior)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [Change Impact Analysis Verification](../../Functional/Processing/Verifications/ChangeImpactVerifications.md#change-impact-analysis-verification)
  * verifiedBy: [Change Impact Detection Test](../../Functional/Processing/Verifications/ChangeImpactVerifications.md#change-impact-detection-test)
  * verifiedBy: [Change Impact Relations Test](../../Functional/Processing/Verifications/ChangeImpactVerifications.md#change-impact-relations-test)
  * verifiedBy: [CLI Git Commit Hash Flag Test](Verifications/CLIVerifications.md#cli-git-commit-hash-flag-test)
  * verifiedBy: [CLI Help Structure Verification](Verifications/CLIVerifications.md#cli-help-structure-verification)
---

### CLI Collect Command

The system shall provide a `collect` command that performs content collection as specified in the attached specifications.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Collect Content Specification](../../Functional/Output/Specifications.md#collect-content-specification)
  * [Collect Output Format Specification](../../Functional/Output/Specifications.md#collect-output-format-specification)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI Collect Command Refinement Specification](Specifications.md#cli-collect-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [CLI Collect Command Test](../../Functional/Output/Verifications/ReportingVerifications.md#cli-collect-command-test)
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

#### Attachments
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)
  * [Mermaid Diagram Style Specification](../../Functional/Output/Specifications.md#mermaid-diagram-style-specification)
  * [ContainmentView.md](../../Functional/Output/DesignDocuments/ContainmentView.md#containmentview)
  * [D3.js Containment Tree Specification](../WebInterface/Specifications.md#d3js-containment-tree-specification)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [Short Mode Behavior](../../Functional/Output/Behaviors.md#short-mode-behavior)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [containment.rs](../../../core/src/containment.rs)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
---

### CLI Coverage Command

The system shall provide a `coverage` command that generates both verification coverage and requirement implementation coverage reports.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Verification Type Selection Guidelines](../../Functional/Core/Specifications.md#verification-type-selection-guidelines)
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)
  * [Implementation Coverage Output Structure Specification](../../Functional/Output/Specifications.md#implementation-coverage-output-structure-specification)
  * [Text Output Formatting](../../Functional/Output/Specifications.md#text-output-formatting)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI Coverage Command Refinement Specification](Specifications.md#cli-coverage-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [Verification Coverage Report Test](../../Functional/Output/Verifications/ReportingVerifications.md#verification-coverage-report-test)
  * verifiedBy: [CLI Help Structure Verification](Verifications/CLIVerifications.md#cli-help-structure-verification)
---

### CLI JSON File Output Option

The system shall provide an `--output <FILE>` option on all commands that support `--json`, allowing JSON output to be written to a file instead of stdout.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI JSON File Output Option Refinement Specification](Specifications.md#cli-json-file-output-option-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [CLI JSON File Output Test](../../Functional/Output/Verifications/ReportingVerifications.md#cli-json-file-output-test)
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
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI Size Estimate JSON Option Specification](Specifications.md#cli-size-estimate-json-option-specification)
  * verifiedBy: [CLI Size Estimate JSON Option Verification](Verifications/CLIVerifications.md#cli-size-estimate-json-option-verification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
---

### CLI Lint Command

The system shall implement a `lint` command that analyzes model quality and detects issues in requirements relations, providing categorized output that distinguishes between auto-fixable issues and those requiring manual review.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Dry-Run Mode Behavior](../../Functional/Operations/Behaviors.md#dry-run-mode-behavior)
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)
  * [Lint Output Specification](../../Functional/Operations/Specifications.md#lint-output-specification)
  * [Cross-Submodel Hierarchical Relation Detection Specification](../../Functional/Operations/Specifications.md#cross-submodel-hierarchical-relation-detection-specification)
  * [Text Output Formatting](../../Functional/Output/Specifications.md#text-output-formatting)
  * [Multi-Branch Convergence Detection Specification](../../Functional/Operations/Specifications.md#multi-branch-convergence-detection-specification)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI Lint Command Refinement Specification](Specifications.md#cli-lint-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
---

### CLI Merge Element Command

The system shall provide a `merge` command to combine multiple elements into a target element.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Merge Content Transformation Behavior](../../Functional/Operations/Behaviors.md#merge-content-transformation-behavior)
  * [Merge Type Compatibility Constraint](../../Functional/Operations/Constraints.md#merge-type-compatibility-constraint)
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)
  * [Merge Element Workflow Specification](../../Functional/Operations/Specifications.md#merge-element-workflow-specification)
  * [Attachment Hierarchical Independence Constraint](../../Functional/Core/Constraints.md#attachment-hierarchical-independence-constraint)
  * [Attachment Satisfied Refinement Constraint](../../Functional/Core/Constraints.md#attachment-satisfied-refinement-constraint)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI Merge Element Command Refinement Specification](Specifications.md#cli-merge-element-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
---

### CLI Model Diagram Command

System shall provide CLI command to generate model diagrams with optional filtering and output format selection.

#### Details
Implementation details shall follow the associated refinement specifications. The command shall support default model-root traversal, filtered traversal, Markdown output, pure Mermaid output, and JSON output.

#### Metadata
  * type: requirement

#### Attachments
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)
  * [Mermaid Diagram Style Specification](../../Functional/Output/Specifications.md#mermaid-diagram-style-specification)
  * [Model Diagram Output Formats Refinement Specification](../../Functional/Output/Specifications.md#model-diagram-output-formats-refinement-specification)
  * [Reverse Relation Traversal Behavior](../../Functional/Output/Behaviors.md#reverse-relation-traversal-behavior)
  * [Start Element Type Filter Behavior](../../Functional/Output/Behaviors.md#start-element-type-filter-behavior)
  * [Type Validation Error Behavior](../../Functional/Core/Behaviors.md#type-validation-error-behavior)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI Model Diagram Command Refinement Specification](Specifications.md#cli-model-diagram-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * verifiedBy: [Model Command Verification](../../Functional/Output/Verifications/ReportingVerifications.md#model-command-verification)
  * verifiedBy: [CLI Help Structure Verification](Verifications/CLIVerifications.md#cli-help-structure-verification)
---

### CLI Move Asset Command

The system shall provide a `mv-asset` command to move or rename InternalPath files and automatically update all references across the model.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [File Persistence Behavior](../../Functional/Operations/Behaviors.md#file-persistence-behavior)
  * [Dry-Run Mode Behavior](../../Functional/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI Move Asset Command Refinement Specification](Specifications.md#cli-move-asset-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [Move Asset Command Verification](../../Functional/Core/Verifications/AttachmentsVerifications.md#move-asset-command-verification)
---

### CLI Move Element Command

The system shall provide a `mv` command to move existing model elements to different file locations while automatically updating all relations that reference the moved element.

#### Details
The command shall reject moving an element into an existing `# Documents` file when that move would create more than one element in the target document file.

#### Metadata
  * type: requirement

#### Attachments
  * [Git Repository Scope Specification](../../Functional/Core/Specifications.md#git-repository-scope-specification)
  * [File Persistence Behavior](../../Functional/Operations/Behaviors.md#file-persistence-behavior)
  * [Target Location Constraint](../../Functional/Operations/Constraints.md#target-location-constraint)
  * [Dry-Run Mode Behavior](../../Functional/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)
  * [Move Element Workflow Specification](../../Functional/Operations/Specifications.md#move-element-workflow-specification)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI Move Element Command Refinement Specification](Specifications.md#cli-move-element-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [Subdirectory Processing Verification](../../Functional/Core/Verifications/ValidationVerifications.md#subdirectory-processing-verification)
  * verifiedBy: [CLI Move Element Test](../../Functional/Operations/Verifications/ElementManipulationVerifications.md#cli-move-element-test)
---

### CLI Move File Command

The system shall provide a `mv-file` command to move entire specification files with all their elements to a new location.

#### Details
The command shall reject `mv-file --squash` when the target file is an existing `# Documents` file.

#### Metadata
  * type: requirement

#### Attachments
  * [File Persistence Behavior](../../Functional/Operations/Behaviors.md#file-persistence-behavior)
  * [Target Location Constraint](../../Functional/Operations/Constraints.md#target-location-constraint)
  * [Dry-Run Mode Behavior](../../Functional/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI Move File Command Refinement Specification](Specifications.md#cli-move-file-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [Subdirectory Processing Verification](../../Functional/Core/Verifications/ValidationVerifications.md#subdirectory-processing-verification)
  * verifiedBy: [CLI Move File Test](../../Functional/Operations/Verifications/ElementManipulationVerifications.md#cli-move-file-test)
---

### CLI Relink Command

The system shall provide a `relink` command that exposes the atomic relation relink operation.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Atomic Relation Relink Workflow Specification](../../Functional/Operations/Specifications.md#atomic-relation-relink-workflow-specification)
  * [Atomic Relink Validity Constraint](../../Functional/Operations/Constraints.md#atomic-relink-validity-constraint)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI Relink Command Refinement Specification](Specifications.md#cli-relink-command-refinement-specification)
  * refinedBy: [Mutating Command Hierarchy Safety Refinement Specification](Specifications.md#mutating-command-hierarchy-safety-refinement-specification)
  * verifiedBy: [Atomic Relation Relink Test](../../Functional/Operations/Verifications/ElementManipulationVerifications.md#atomic-relation-relink-test)
---

### CLI Remove Asset Command

The system shall provide an `rm-asset` command to remove InternalPath files and automatically remove all references from the model.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [File Persistence Behavior](../../Functional/Operations/Behaviors.md#file-persistence-behavior)
  * [Dry-Run Mode Behavior](../../Functional/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI Remove Asset Command Refinement Specification](Specifications.md#cli-remove-asset-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [Remove Asset Command Verification](../../Functional/Core/Verifications/AttachmentsVerifications.md#remove-asset-command-verification)
---

### CLI Remove Element Command

The system shall provide an `rm` command to delete existing model elements and automatically remove all relations referencing the deleted element.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [File Persistence Behavior](../../Functional/Operations/Behaviors.md#file-persistence-behavior)
  * [Dry-Run Mode Behavior](../../Functional/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)
  * [Delete Element Workflow Specification](../../Functional/Operations/Specifications.md#delete-element-workflow-specification)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI Remove Element Command Refinement Specification](Specifications.md#cli-remove-element-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [CLI Remove Element Test](../../Functional/Operations/Verifications/ElementManipulationVerifications.md#cli-remove-element-test)
---

### CLI Rename Element Command

The system shall provide a `rename` command to rename existing model elements while automatically updating all relations that reference the renamed element.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [File Persistence Behavior](../../Functional/Operations/Behaviors.md#file-persistence-behavior)
  * [Dry-Run Mode Behavior](../../Functional/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI Rename Element Command Refinement Specification](Specifications.md#cli-rename-element-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [CLI Rename Element Test](../../Functional/Operations/Verifications/ElementManipulationVerifications.md#cli-rename-element-test)
---

### CLI Resources Command

The system shall provide a `resources` command that generates a report showing all files referenced by the model through relations and attachments.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI Resources Command Refinement Specification](Specifications.md#cli-resources-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [Resources Report Verification](../../Functional/Output/Verifications/ReportingVerifications.md#resources-report-verification)
---

### CLI Ontologies Command

The system shall provide an `ontologies` command that collects all ontology `#### Ontology` and semantic-contract `#### Shapes` RDF blocks from the graph registry, with an optional full semantic model projection.

#### Details
The command shall:
- Emit RDF/Turtle by default.
- Support `--jsonld` to emit JSON-LD instead of Turtle.
- Support `--full` to include RDF triples for Reqvire model elements, relations, attachments, concept references, ontology declarations, and semantic-contract shape references.
- Support `--output <FILE>` to write the selected format to a file.
- Reuse the semantic index built from the graph registry instead of reparsing Turtle separately from validation.

#### Metadata
  * type: requirement

#### Attachments
  * [Semantic Contract Structure Specification](../../Functional/Core/Specifications.md#semantic-contract-structure-specification)
  * [Ontology Collection Output Specification](../../Functional/Output/Specifications.md#ontology-collection-output-specification)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI Ontologies Command Refinement Specification](Specifications.md#cli-ontologies-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [semantic_contract.rs](../../../core/src/semantic_contract.rs)
  * verifiedBy: [CLI Ontologies Command Verification](Verifications/CLIVerifications.md#cli-ontologies-command-verification)
  * verifiedBy: [CLI Help Structure Verification](Verifications/CLIVerifications.md#cli-help-structure-verification)
---

### CLI Search Command

The system shall provide a unified search function, activated by the `search` root command, which shall search and report on model elements with comprehensive filtering capabilities.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Requirement Governance Metadata Specification](../../Functional/Core/Specifications.md#requirement-governance-metadata-specification)
  * [Supported Element Types Specification](../../Refinements.md#supported-element-types-specification)
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)
  * [Short Mode Behavior](../../Functional/Output/Behaviors.md#short-mode-behavior)
  * [Text Output Formatting](../../Functional/Output/Specifications.md#text-output-formatting)
  * [Type Validation Error Behavior](../../Functional/Core/Behaviors.md#type-validation-error-behavior)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI Search Command Refinement Specification](Specifications.md#cli-search-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [Attachment Search Filters Verification](../../Functional/Core/Verifications/AttachmentsVerifications.md#attachment-search-filters-verification)
  * verifiedBy: [Search Command Tests](../../Functional/Output/Verifications/ReportingVerifications.md#search-command-tests)
  * verifiedBy: [CLI Help Structure Verification](Verifications/CLIVerifications.md#cli-help-structure-verification)
---

### CLI Submodels Command

The system shall provide a `submodels` command that reports independent capability-rooted hierarchies and cross-submodel requirement couplings.

#### Details
The command shall support:
- `--from <NAME>` to scope report output to one capability or requirement subtree by name
- `--json` and `--output <FILE>` for machine-readable output
- In `--from` mode, selected capability scopes are listed as the scoped capability submodel; selected requirement scopes are treated as boundaries and are not listed as submodel entries

Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)
  * [Requirement Submodels Report Specification](../../Functional/Output/Specifications.md#requirement-submodels-report-specification)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI Submodels Command Refinement Specification](Specifications.md#cli-submodels-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [Submodels Report Verification](../../Functional/Output/Verifications/ReportingVerifications.md#submodels-report-verification)
  * verifiedBy: [CLI Help Structure Verification](Verifications/CLIVerifications.md#cli-help-structure-verification)
---

### CLI Traces Command

The system shall provide a `traces` command that generates and outputs upward trace trees for verification elements, showing the complete requirement hierarchy and owning capability context.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)
  * [Verification Trace Tree Construction](../../Functional/Processing/Specifications.md#verification-trace-tree-construction)
  * [Mermaid Diagram Style Specification](../../Functional/Output/Specifications.md#mermaid-diagram-style-specification)
  * [Type Validation Error Behavior](../../Functional/Core/Behaviors.md#type-validation-error-behavior)

#### Relations
  * derive: [Verification Traces Element Navigation](#verification-traces-element-navigation)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [CLI Traces Command Refinement Specification](Specifications.md#cli-traces-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [Verification Traces Filter Options Test](../../Functional/Output/Verifications/ReportingVerifications.md#verification-traces-filter-options-test)
  * verifiedBy: [Verification Traces From-Folder Test](../../Functional/Output/Verifications/ReportingVerifications.md#verification-traces-from-folder-test)
  * verifiedBy: [CLI Help Structure Verification](Verifications/CLIVerifications.md#cli-help-structure-verification)
---

### Verification Traces Element Navigation

The system shall make verification element names in the traces report clickable links that navigate to the element's definition in its source file.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [CLI Traces Command](#cli-traces-command)
  * refinedBy: [Verification Traces Element Navigation Refinement Specification](Specifications.md#verification-traces-element-navigation-refinement-specification)
  * satisfiedBy: [verification_trace.rs](../../../core/src/verification_trace.rs)
  * verifiedBy: [Verification Traces Element Navigation Test](Verifications/CLIVerifications.md#verification-traces-element-navigation-test)
---

### Detailed Error Handling and Logging

The system shall implement detailed error handling and logging throughout the application to facilitate troubleshooting and provide meaningful feedback.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Validation Error Reporting Behavior](../../Functional/Core/Behaviors.md#validation-error-reporting-behavior)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [Detailed Error Handling and Logging Refinement Specification](Specifications.md#detailed-error-handling-and-logging-refinement-specification)
  * satisfiedBy: [error.rs](../../../core/src/error.rs)
---

### Format Command

The system shall provide a formatting function, activated by the (format command), which shall execute the formatting process upon user request.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Dry-Run Mode Behavior](../../Functional/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [Format Command Refinement Specification](Specifications.md#format-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [Element Ordering Verification](../../Functional/Operations/Verifications/FormattingVerifications.md#element-ordering-verification)
  * verifiedBy: [Format Command Requirements Verification](../../Functional/Operations/Verifications/FormattingVerifications.md#format-command-requirements-verification)
  * verifiedBy: [Full Relations Insertion Verification](../../Functional/Operations/Verifications/FormattingVerifications.md#full-relations-insertion-verification)
  * verifiedBy: [CLI Help Structure Verification](Verifications/CLIVerifications.md#cli-help-structure-verification)
---

### Relation Commands

The system shall provide unified CLI commands for relation and attachment management: link and unlink.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Relation Operations Specification](../../Functional/Operations/Specifications.md#relation-operations-specification)
  * [RelationTypes.md](../../Functional/Core/DesignDocuments/RelationTypes.md#relationtypes)
  * [Dry-Run Mode Behavior](../../Functional/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [Relation Commands Refinement Specification](Specifications.md#relation-commands-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [Link Command Verification](../../Functional/Operations/Verifications/ElementManipulationVerifications.md#link-command-verification)
  * verifiedBy: [Unlink Command Verification](../../Functional/Operations/Verifications/ElementManipulationVerifications.md#unlink-command-verification)
---

### Validate Command

The system shall provide a validation command that executes model validation and reports any issues found.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Two-Pass Validation Behavior](../../Functional/Core/Behaviors.md#two-pass-validation-behavior)
  * [Validation Error Reporting Behavior](../../Functional/Core/Behaviors.md#validation-error-reporting-behavior)
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)
  * [Error Message Format Specification](Specifications.md#error-message-format-specification)
  * [Attachment Hierarchical Independence Constraint](../../Functional/Core/Constraints.md#attachment-hierarchical-independence-constraint)
  * [Attachment Satisfied Refinement Constraint](../../Functional/Core/Constraints.md#attachment-satisfied-refinement-constraint)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * refinedBy: [Validate Command Refinement Specification](Specifications.md#validate-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [Invalid Relations Test](../../Functional/Core/Verifications/ValidationVerifications.md#invalid-relations-test)
  * verifiedBy: [CLI Help Structure Verification](Verifications/CLIVerifications.md#cli-help-structure-verification)
---
