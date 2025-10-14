# CLI

## CLI Interface
### CLI Interface Structure

The CLI interface shall implement the clear `[OPTIONS] <COMMAND> [COMMAND OPTIONS]` structure.

#### Details
The CLI must display all commands and options and command's options flattened in the main help output which must also be a default commnad:
```
Reqvire requirements & treacibility management tool

Usage: reqvire [OPTIONS] <COMMAND> [COMMAND OPTIONS]

Commands:
  format             Format and normalize requirements files. By default, shows preview without applying changes
  validate           Validate model
  help               Print this message or the help of the given subcommand(s)

Options:
  -h, --help               Print help
  -V, --version            Print version

FORMAT OPTIONS:
      --fix      Apply formatting changes to files
      --json     Output results in JSON format
```

#### Relations
  * derivedFrom: [CLI interface](../../UserRequirements.md#cli-interface)
  * satisfiedBy: [../../../cli/src/cli.rs](../../../cli/src/cli.rs)
---

### Format Command

The system shall provide a formatting function, activated by the (format command), which shall execute the formatting process upon user request.

#### Details
`format` command shall:
  - Default to dry-run mode (show suggested changes without applying them)
  - Require --fix flag to actually apply formatting changes to files
  - Display a diff-style summary of changes that would be or have been made
  - Support --json flag for structured output of formatting results
  - Show git diff style output with line numbers and colors for both preview and actual formatting

Additional behavior:
  - By default (no --fix flag), preview changes without applying them
  - --fix flag applies the formatting changes to files
  - --json flag outputs formatting results in JSON format including file changes and diff information
  - When formatting is applied, show a summary of changed files with diff details

#### Relations
  * derivedFrom: [Model Formatting](../../UserRequirements.md#model-formatting)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [../../../cli/src/cli.rs](../../../cli/src/cli.rs)
---

### Validate Command

The system shall provide a validation command that executes model validation and reports any issues found.

#### Details
`validate` command shall:
  - Execute two-pass validation strategy:
    * **Pass 1: Element Collection and Local Validation**
      - Parse all markdown files
      - Extract elements with metadata
      - Perform local validation (element uniqueness, identifier format, metadata syntax)
      - Report errors if found
    * **Pass 2: Graph Construction and Relation Validation**
      - Build GraphRegistry from collected elements
      - Validate all relations (target existence, type compatibility)
      - Perform cross-component validation
      - Report errors if found
  - Print all validation issues found in the model
  - Output a success message "No validation issues found" when the model is valid
  - Support --json flag to output validation results in JSON format

#### Relations
  * derivedFrom: [Provide Validation Reports](../../UserRequirements.md#provide-validation-reports)
  * derivedFrom: [Enhanced Validation Error Reporting](../../UserRequirements.md#enhanced-validation-error-reporting)
  * derivedFrom: [Two-Pass Validation Strategy](../ValidationAndReporting/Validation.md#two-pass-validation-strategy)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [../../../cli/src/cli.rs](../../../cli/src/cli.rs)
---

### Subdirectory Processing Option

The system shall automatically detect when it is run from a subdirectory of a git repository and process only files within that subdirectory.

#### Details
The subdirectory auto-detection is designed to limit the scope of processing to the current working directory when it is a subdirectory of the git root.

When run from the git root, the system processes all files. When run from a subdirectory, it automatically limits scope to that subdirectory:
```
cd specifications/Verifications
reqvire model-summary  # Only processes files in Verifications directory (with automatic validation)
```

The system shall validate references when processing from a subdirectory and generate validation errors for any references to elements or files outside the current subdirectory scope. This includes:

1. **Parent Directory Reference Validation**: Any relation that references an element or file outside the current subdirectory scope shall be reported as a missing relation target error
2. **Scope Boundary Enforcement**: References using relative paths (e.g., `../ParentFile.md#element`) that escape the subdirectory shall result in missing relation target errors when the referenced elements cannot be found
3. **Absolute Path Validation**: Absolute paths that point outside the subdirectory scope shall generate missing relation target errors
4. **Error Reporting**: Missing relation target errors shall clearly identify the unreachable reference due to subdirectory scope limitations

This validation ensures that subdirectory processing maintains logical boundaries and prevents architectural inconsistencies by reporting parent directory references as missing targets.

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [../../../cli/src/cli.rs](../../../cli/src/cli.rs)
---

### Detailed Error Handling and Logging

The system shall implement detailed error handling and logging throughout the application to facilitate troubleshooting and provide meaningful feedback.

#### Relations
  * derivedFrom: [Enhanced Validation Error Reporting](../../UserRequirements.md#enhanced-validation-error-reporting)
  * satisfiedBy: [../../../core/src/error.rs](../../../core/src/error.rs)
---

### CLI Summary Report Command

The system shall provide a model summary report function, activated by the `summary` root command, which shall generate model summary report with ability to be passed several filters.

#### Details
Model summary CLI command:
- `summary`:  Output model registry and summary to stdout, also supports json and cypher output.

All filters can be combined with the command:
- `summary`:  Output model registry and summary, also supports json and cypher output.
  - By file path: `summary --filter-file="src/**/*Reqs.md"`
  - By name: `summary --filter-name=".*safety.*"`
  - By section: `summary --filter-section="System*"`
  - By type: `summary --filter-type="system-requirement"` (exact match)
  - By content: `summary --filter-content="MUST"`
  - Not verified: `summary --filter-is-not-verified`
  - Not satisfied: `summary --filter-is-not-satisfied`

Must support `--json` and `--cypher` flags to output either json formatted string or valid Cypher queries that when executed in graph database produce valid graph of a system model.

#### Relations
  * derivedFrom: [Model Summary Report Generator](../ValidationAndReporting/Reports.md#model-summary-report-generator)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [../../../cli/src/cli.rs](../../../cli/src/cli.rs)
---

### Handle Invalid Regex Filter Patterns

When the user invokes Reqvire with the `summary` command and invalid regular expression to regex based filters are provided the system shall return an error showing the faulty pattern and exit without producing a summary.

#### Relations
  * satisfiedBy: [../../../cli/src/cli.rs](../../../cli/src/cli.rs)
  * derivedFrom: [CLI Summary Report Command](#cli-summary-report-command)
  * verifiedBy: [../../Verifications/ReportsTests.md#model-summary-tests](../../Verifications/ReportsTests.md#model-summary-tests)
---

### Display Name-Regex Option in Help

When the user requests help (`--help` or `-h`), the system shall list summary filter flags under the SUMMARY OPTIONS heading, including descriptions for all filter options.

#### Relations
  * satisfiedBy: [../../../cli/src/cli.rs](../../../cli/src/cli.rs)
  * derivedFrom: [CLI Summary Report Command](#cli-summary-report-command)
  * verifiedBy: [../../Verifications/ReportsTests.md#model-summary-tests](../../Verifications/ReportsTests.md#model-summary-tests)
---

### CLI Sections Summary Command

The system shall provide a command-line interface root command `section-summary` that generates sections summary reports with filtering capabilities.

#### Details
Sections summary CLI command:
- `section-summary`: Output file paths, section names, section order indices, and section content without individual elements
- Support `--json` flag for JSON output format
- Support filtering flags that can be combined:
  - By file path (glob): `--filter-file="src/**/*Reqs.md"`
  - By section name (glob): `--filter-section="System*"`
  - By section content (regex): `--filter-content="MUST"`
- Default to human-readable text output when JSON flag is not present
- Exit with status code 0 on success
- Exit with non-zero status code on errors

Command output shall be written to stdout for easy redirection to files.

#### Relations
  * derivedFrom: [Sections Summary Report Generator](../ValidationAndReporting/Reports.md#sections-summary-report-generator)
  * satisfiedBy: [../../../cli/src/cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [../../../core/src/sections_summary.rs](../../../core/src/sections_summary.rs)
  * verifiedBy: [../../Verifications/ReportsTests.md#sections-summary-tests](../../Verifications/ReportsTests.md#sections-summary-tests)
---

### CLI Model Diagram Command

System shall provide CLI command to generate model diagrams with optional filtering and output format selection.

#### Details
- Command shall be named `model`
- Shall support `--root-id=<identifier>` flag for filtering from specific element
- Shall support `--json` flag for JSON output format
- Default output shall be markdown with embedded Mermaid diagram
- Shall integrate with existing model diagram generation functionality

#### Relations
  * derivedFrom: [Model Visualization and Exploration](../../UserRequirements.md#model-visualization-and-exploration)
  * satisfiedBy: [../../../core/src/diagrams.rs](../../../core/src/diagrams.rs)
  * satisfiedBy: [../../../cli/src/cli.rs](../../../cli/src/cli.rs)
---

### CLI Matrix Command

The system shall implement a `matrix` subcommand under the main `verifications` command that generates traceability matrices showing requirements and their verification status.

#### Details
The command shall:
- Be invoked as `reqvire matrix`
- Generate output in Markdown format by default with hierarchical requirements
- Support `--json` flag for structured JSON output (see [CLI Matrix JSON Flag](../ModelManagement/TraceabilityMatrix.md#cli-matrix-json-flag))
- Support `--svg` flag for SVG matrix output (see [CLI Matrix SVG Flag](../ModelManagement/TraceabilityMatrix.md#cli-matrix-svg-flag))
- The `--json` and `--svg` flags shall be mutually exclusive
- Show requirements as rows and verification elements as columns
- Display verification status for each requirement using [Verification Roll-up Strategy](../ModelManagement/TraceabilityMatrix.md#verification-roll-up-strategy)
- Include hierarchical indentation for derived requirements
- Show verification relationships with checkmarks where applicable
- Exit with status code 0 on success
- Exit with non-zero status code on errors

Command output shall be written to stdout for easy redirection to files.

#### Relations
  * derivedFrom: [Verification Roll-up Strategy](../ModelManagement/TraceabilityMatrix.md#verification-roll-up-strategy)
  * satisfiedBy: [../../../cli/src/cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [../../../core/src/matrix_generator.rs](../../../core/src/matrix_generator.rs)
  * verifiedBy: [../../Verifications/TreacibilityMatrix.md#traceability-matrix-generation-test](../../Verifications/TreacibilityMatrix.md#traceability-matrix-generation-test)
---

### CLI Traces Command

The system shall implement a `traces` subcommand under the main `verifications` command that generates and outputs upward trace trees for verification elements, showing the complete requirement hierarchy from verifications to root requirements.

#### Details
The command shall:
- Be invoked as `reqvire traces`
- Generate output in Markdown format with embedded Mermaid diagrams by default
- Support `--json` flag for structured JSON output without diagrams
- Show verification elements as roots with arrows following relation semantics
- Include clickable links on all nodes (verifications and requirements) in Mermaid diagrams
- Highlight directly verified requirements using CSS classes in diagrams
- Traverse all upward parent relations to reach root requirements
- Merge multiple verification paths into a single tree per verification
- Display a list of redundant relations under each verification's diagram in markdown output, showing requirements that are ancestors of the most derived (leaf) requirement(s) in the trace tree, with markdown links to the redundant requirements
- Include a `redundant_relations` key in JSON output for each verification, containing an array of redundant requirement identifiers (empty array if none)
- Exit with status code 0 on success
- Exit with non-zero status code on errors

The Mermaid diagrams generated for verification traces shall include clickable links on diagram nodes that navigate to the referenced element using relative paths (the `traces` command always uses relative paths and does not support the `--links-with-blobs` flag).

Command output shall be written to stdout for easy redirection to files.

#### Relations
  * derivedFrom: [Verification Trace Builder](../ModelManagement/TraceabilityMatrix.md#verification-trace-builder)
  * satisfiedBy: [../../../cli/src/cli.rs](../../../cli/src/cli.rs)
---

### CLI Traces Filter Options

The system shall support filtering verification traces by verification ID, name pattern, and verification type to allow users to generate traces for specific subsets of verifications.

#### Details
The following filter options shall be supported:
- `--filter-id=<id>`: Generate trace for a specific verification element by its full identifier
- `--filter-name=<regex>`: Filter verifications by name using regular expression matching
- `--filter-type=<type>`: Filter by verification type (test-verification, analysis-verification, inspection-verification, demonstration-verification)

Filters shall be combinable, and when multiple filters are specified, only verifications matching ALL filter criteria shall be included in the output.

#### Relations
  * derivedFrom: [CLI Traces Command](#cli-traces-command)
---

### CLI Traces From-Folder Option

The system shall support a `--from-folder` option for the `traces` command that specifies the relative path from where Reqvire runs to the folder where generated output files will be saved, enabling generation of relative links in Mermaid diagrams and other outputs that are portable when the output is saved in different locations.

#### Details
The `--from-folder` option shall:
- Accept a relative path string as parameter (e.g., `--from-folder=docs/verification-reports`)
- Default to empty/current directory when not specified (maintaining existing behavior)
- Support special case `/` to indicate the reqvire root (git root), keeping identifiers as git-root-relative paths
- Adjust all clickable links in Mermaid diagrams to be relative to the specified folder path
- Adjust all file path references in output to be relative to the specified folder path
- Work with both Markdown output (with Mermaid diagrams) and JSON output
- Ensure generated links work correctly when the output file is saved in the specified folder
- Use the standard path resolution logic to calculate relative paths from the from-folder to git root

**Example usage:**
```bash
# Generate traces with links relative to docs/reports/ folder
reqvire traces --from-folder=docs/reports > docs/reports/traces.md

# The generated links will be relative to docs/reports/, so they work when traces.md is opened
```

**Link generation behavior:**
- When `diagrams_with_blobs` is true and Git info is available:
  - Links point to GitHub blob URLs (external, absolute)
  - `--from-folder` has no effect (external links are already absolute)
- When `diagrams_with_blobs` is false or Git info not available:
  - Links are element identifiers (relative paths from git root)
  - `--from-folder` adjusts these to be relative to the specified folder
  - Example: Identifier `specifications/file.md#element` becomes `../../specifications/file.md#element` when from-folder is `docs/reports`

#### Relations
  * derivedFrom: [CLI Traces Command](#cli-traces-command)
---

### CLI Coverage Command

The system shall implement a `coverage` subcommand under the main `verifications` command that generates verification coverage reports focusing on leaf requirements, test-verification satisfaction status, and orphaned verifications.

#### Details
The command shall:
- Be invoked as `reqvire coverage`
- Support `--json` flag for JSON output format
- Default to human-readable text output when JSON flag is not present
- Generate reports focusing on leaf requirements (requirements without forward relations to other requirements)
- Show the percentage and details of verified and unverified leaf requirements
- Include breakdowns by file, section, and verification type
- Show satisfaction status of test-verification elements (those with satisfiedBy relations)
- Show orphaned verifications (verification elements without any verify relations to requirements)
- Follow [Verification Roll-up Strategy](../ModelManagement/TraceabilityMatrix.md#verification-roll-up-strategy)
- Test-verification elements require satisfiedBy relations to be considered satisfied
- Analysis, inspection, and demonstration verification elements are considered satisfied by default
- Exit with status code 0 on success
- Exit with non-zero status code on errors

Command output shall be written to stdout for easy redirection to files.

#### Relations
  * derivedFrom: [Verification Coverage Report Generator](../ValidationAndReporting/Reports.md#verification-coverage-report-generator)
  * derivedFrom: [Verification Roll-up Strategy](../ModelManagement/TraceabilityMatrix.md#verification-roll-up-strategy)
  * satisfiedBy: [../../../cli/src/cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [../../Verifications/ReportsTests.md#verification-coverage-report-test](../../Verifications/ReportsTests.md#verification-coverage-report-test)
---

### CLI Lint Command

The system shall implement a `lint` command that analyzes model quality and detects issues in requirements relations, providing categorized output that distinguishes between auto-fixable issues and those requiring manual review.

#### Details
The command shall:
- Be invoked as `reqvire lint`
- Default to dry-run mode (report issues without applying fixes)
- Support `--fixable` flag to show only auto-fixable issues
- Support `--auditable` flag to show only issues requiring manual review
- Support `--fix` flag to automatically apply fixes for auto-fixable issues
- Support `--json` flag for structured JSON output
- Default to showing ALL issues when no filter flags are provided
- Categorize output into two sections:
  * **Auto-fixable Issues**: Issues that can be automatically corrected by the system
  * **Needs Manual Review**: Issues that require human judgment to resolve
- Exit with status code 0 when no issues are found or when fixes are successfully applied
- Exit with non-zero status code on errors

Command output shall be written to stdout for easy redirection to files.

#### Relations
  * derivedFrom: [Model Linting](../../UserRequirements.md#model-linting)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
---

### CLI Change Impact Report Command

The system shall provide a change and impact report function, activated by the (change-impact command), which shall generate change impact report

#### Details
Must support `--json` option flag to output json formated string.

#### Relations
  * derivedFrom: [Structural Change Analyzer](../ModelManagement/ChangeImpact.md#structural-change-analyzer)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [../../../cli/src/cli.rs](../../../cli/src/cli.rs)
---

### CLI Git Commit Hash Flag

The system shall provide a git commit hash flag  (--git_commit command option flag), to be used with ** CLI Change Impact Report Flag**.

#### Relations
  * derivedFrom: [CLI Change Impact Report Command](#cli-change-impact-report-command)
  * satisfiedBy: [../../../cli/src/cli.rs](../../../cli/src/cli.rs)
---

### CLI Generate Diagrams Flag

The system shall provide a diagrams generation function, activated by the (generate-diagrams command), which shall generate interactive mermaid diagrams.

#### Relations
  * derivedFrom: [Diagram Generation](../ModelManagement/DiagramGeneration.md#diagram-generation)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [../../../cli/src/cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [../../Verifications/DiagramsTests.md#diagram-generation-test](../../Verifications/DiagramsTests.md#diagram-generation-test)
---

### CLI Remove Diagrams Flag

The system shall provide a diagram removal function, activated by the remove-diagrams command, which shall remove all generated mermaid diagrams from the model.

#### Relations
  * derivedFrom: [Diagram Removal](../ModelManagement/DiagramGeneration.md#diagram-removal)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [../../../cli/src/cli.rs](../../../cli/src/cli.rs)
  * verifiedBy: [../../Verifications/DiagramsTests.md#remove-generated-diagrams-verification](../../Verifications/DiagramsTests.md#remove-generated-diagrams-verification)
---
