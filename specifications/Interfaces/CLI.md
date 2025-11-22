# Requirements

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
  * derivedFrom: [CLI interface](Interfaces.md#cli-interface)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
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
  * derivedFrom: [Model Formatting](../System/Operations/Formatting.md#model-formatting)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
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
  * derivedFrom: [Provide Validation Reports](../System/Output/Reporting.md#provide-validation-reports)
  * derivedFrom: [Enhanced Validation Error Reporting](../System/Core/Validation.md#enhanced-validation-error-reporting)
  * derivedFrom: [Two-Pass Validation Strategy](../System/Core/Validation.md#two-pass-validation-strategy)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
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
  * derivedFrom: [Git Repository as Project Root](../System/Core/ModelManagement.md#git-repository-as-project-root)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
---

### Attachment Commands

The system shall provide CLI commands for attachment management: attach, detach, mv-attachment, and rm-attachment.

#### Details
<details>
<summary>View Full Specification</summary>

## Attach Command

Syntax: `reqvire attach <attachment-path> <element-name> [--dry-run]`

Behavior:
- Create Attachments subsection if doesn't exist
- Add link to subsection with format `* [path](path)`
- Skip if already attached (idempotent)
- Support many-to-many (same file to multiple elements)
- Mark element file as modified
- Support dry-run mode for preview

## Detach Command

Syntax: `reqvire detach <element-name> <attachment-path> [--dry-run]`

Behavior:
- Remove link from Attachments subsection
- Remove subsection if no attachments remain
- Trigger change impact on element (CRITICAL)
- Mark element file as modified
- Support dry-run mode for preview

## Move Attachment Command

Syntax: `reqvire mv-attachment <old-path> <new-path> [--dry-run]`

Behavior:
- Update ALL references across all elements
- Update both link text and href (text = path)
- Report affected elements
- Mark all affected element files as modified
- Support dry-run mode for preview

## Remove Attachment Command

Syntax: `reqvire rm-attachment <attachment-path> [--dry-run]`

Behavior:
- Delete physical file from filesystem
- Detach from ALL elements
- Remove empty Attachments subsections
- Report affected elements
- Mark all affected element files as modified
- Support dry-run mode for preview

</details>

#### Relations
  * derivedFrom: [Reserved Subsections Support](../System/Core/StructureAndParsing.md#reserved-subsections-support)
---

### Detailed Error Handling and Logging

The system shall implement detailed error handling and logging throughout the application to facilitate troubleshooting and provide meaningful feedback.

#### Relations
  * derivedFrom: [Enhanced Validation Error Reporting](../System/Core/Validation.md#enhanced-validation-error-reporting)
  * satisfiedBy: [error.rs](../../core/src/error.rs)
---

### CLI Search Command

The system shall provide a unified search function, activated by the `search` root command, which shall search and report on model elements with comprehensive filtering capabilities.

#### Details
Search command features:
- `search`: Search model elements and output results to stdout
- Support `--json` flag for structured JSON output
- Support `--short` flag for abbreviated output (both text and JSON)
- Support comprehensive filter options (all combinable):
  - By file path glob: `--filter-file="src/**/*Reqs.md"`
  - By element name regex: `--filter-name=".*safety.*"`
  - By element type: `--filter-type="system-requirement"` (exact match)
  - By element content regex: `--filter-content="MUST"`
  - By page content regex: `--filter-page-content="architecture"`
  - By having relations: `--have-relations=verifiedBy,satisfiedBy` (comma-separated, must have ALL)
  - By not having relations: `--not-have-relations=verifiedBy` (comma-separated, must NOT have ALL)
  - By having attachments: `--has-attachments` (filter elements with Attachments subsection)
  - By attachment path pattern: `--filter-attachment <glob>` (supports glob patterns like `*.pdf`, `docs/*`)

Short mode behavior:
- Text output: Display abbreviated one-line format per element
- JSON output: Omit fields: `content`, `page_content`, `verified_relations_count`, `satisfied_relations_count`, `element_count`, `total_elements`, `global_counters`

Error handling:
- Invalid regex patterns shall return clear error message showing the faulty pattern and exit
- Invalid relation type names shall return error with list of valid relation types

Default output:
- Human-readable text format when neither `--json` nor `--short` is specified
- Full detail mode showing all element metadata and relations

#### Relations
  * derivedFrom: [Search Fine Grained Filtering](../System/Output/Reporting.md#search-fine-grained-filtering)
  * derivedFrom: [Reserved Subsections Support](../System/Core/StructureAndParsing.md#reserved-subsections-support)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [Search Command Tests](../System/Output/Verifications/ReportingVerifications.md#search-command-tests)
---

### CLI Model Diagram Command

System shall provide CLI command to generate model diagrams with optional filtering and output format selection.

#### Details
- Command shall be named `model`
- Shall support `--from=<name>` flag for filtering from specific element by name
- Element names are globally unique, allowing lookup by name alone
- Shall support `--json` flag for JSON output format
- Default output shall be markdown with embedded Mermaid diagram
- Shall integrate with existing model diagram generation functionality

#### Relations
  * derivedFrom: [Model Visualization and Exploration](../System/Output/DiagramGeneration.md#model-visualization-and-exploration)
  * satisfiedBy: [diagrams.rs](../../core/src/diagrams.rs)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
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
- Exit with status code 0 on success
- Exit with non-zero status code on errors

The Mermaid diagrams generated for verification traces shall include clickable links on diagram nodes that navigate to the referenced element using relative paths (the `traces` command always uses relative paths and does not support the `--links-with-blobs` flag).

Command output shall be written to stdout for easy redirection to files.

**Filter Options:**
The system shall support filtering verification traces by verification ID, name pattern, and verification type to allow users to generate traces for specific subsets of verifications.

The following filter options shall be supported:
- `--filter-id=<id>`: Generate trace for a specific verification element by its full identifier
- `--filter-name=<regex>`: Filter verifications by name using regular expression matching
- `--filter-type=<type>`: Filter by verification type (test-verification, analysis-verification, inspection-verification, demonstration-verification)

Filters shall be combinable, and when multiple filters are specified, only verifications matching ALL filter criteria shall be included in the output.

**From-Folder Option:**
The system shall support a `--from-folder` option for the `traces` command that specifies the relative path from where Reqvire runs to the folder where generated output files will be saved, enabling generation of relative links in Mermaid diagrams and other outputs that are portable when the output is saved in different locations.

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

# Links in the output will be relative to docs/reports/ (e.g., ../../specifications/file.md)
```

#### Relations
  * derivedFrom: [Verification Trace Builder](../System/Processing/VerificationTraces.md#verification-trace-builder)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
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
- Follow [Verification Roll-up Strategy](../ModelManagement/VerificationTraces.md#verification-roll-up-strategy)
- Test-verification elements require satisfiedBy relations to be considered satisfied
- Analysis, inspection, and demonstration verification elements are considered satisfied by default
- Exit with status code 0 on success
- Exit with non-zero status code on errors

Command output shall be written to stdout for easy redirection to files.

#### Relations
  * derivedFrom: [Verification Coverage Report Generator](../System/Output/Reporting.md#verification-coverage-report-generator)
  * derivedFrom: [Verification Roll-up Strategy](../System/Processing/VerificationTraces.md#verification-roll-up-strategy)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [Verification Coverage Report Test](../System/Output/Verifications/ReportingVerifications.md#verification-coverage-report-test)
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
  * derivedFrom: [Model Linting](../System/Operations/Linting.md#model-linting)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
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

#### Relations
  * derivedFrom: [Structural Change Analyzer](../System/Processing/ChangeImpact.md#structural-change-analyzer)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
---

### CLI Generate Diagrams Flag

The system shall provide a diagrams generation function, activated by the (generate-diagrams command), which shall generate interactive mermaid diagrams.

#### Relations
  * derivedFrom: [Diagram Generation](../System/Output/DiagramGeneration.md#diagram-generation)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [Diagram Generation Test](../System/Output/Verifications/DiagramVerifications.md#diagram-generation-test)
---

### CLI Remove Diagrams Flag

The system shall provide a diagram removal function, activated by the remove-diagrams command, which shall remove all generated mermaid diagrams from the model.

#### Relations
  * derivedFrom: [Diagram Removal](../System/Output/DiagramGeneration.md#diagram-removal)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [Remove Generated Diagrams Verification](../System/Output/Verifications/DiagramVerifications.md#remove-generated-diagrams-verification)
---

### CLI Add Element Command

The system shall provide an `add` command to create new model elements by accepting element definition in Markdown format from stdin or as an inline string argument, validating the structure, and inserting it at the specified location.

#### Details
The `add` command shall:
- Accept element definition input from:
  - Standard input (stdin) for piped or redirected content
  - Last positional argument as inline string when not reading from stdin
- Accept target location arguments (resolved relative to Git repository root):
  - Target file path (required, provided as argument or `--to-file` flag)
  - Index within file (optional, provided as argument or `--index` flag, 0-based, defaults to end of file)
- Support command syntax:
  - `reqvire add <file> [<index>]` - reads element from stdin
  - `reqvire add <file> [<index>] <element-markdown>` - element as last argument
  - `reqvire add --to-file=<file> --index=<n> < element.md` - with named flags
- Validate element structure before insertion
- Apply changes immediately by default
- Support `--dry-run` flag to preview changes without applying
- Output git-style diff showing file changes by default
- Support `--json` flag for structured output format
- Report validation errors if element structure is invalid
- Exit with code 0 on success, non-zero on error

#### Relations
  * derivedFrom: [Create Element Operation](../System/Operations/ElementManipulation.md#create-element-operation)
  * derivedFrom: [Subdirectory Processing Option](#subdirectory-processing-option)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
---

### CLI Remove Element Command

The system shall provide an `rm` command to delete existing model elements and automatically remove all relations referencing the deleted element.

#### Details
The `rm` command shall:
- Accept element name as required positional argument
- Support command syntax: `reqvire rm <element-name>`
- Delete the specified element from its file
- Remove all incoming relations from other elements
- Apply changes immediately by default
- Support `--dry-run` flag to preview changes without applying
- Output git-style diff showing file changes by default
- Support `--json` flag for structured output with affected relations
- Report error if element does not exist
- Exit with code 0 on success, non-zero on error

#### Relations
  * derivedFrom: [Delete Element Operation](../System/Operations/ElementManipulation.md#delete-element-operation)
  * derivedFrom: [Subdirectory Processing Option](#subdirectory-processing-option)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
---

### CLI Move Element Command

The system shall provide a `mv` command to move existing model elements to different file locations while automatically updating all relations that reference the moved element.

#### Details
The `mv` command shall:
- Accept element name (required, provided as argument)
- Accept target location arguments (resolved relative to Git repository root):
  - Target file path (required, provided as argument or `--to-file` flag)
  - Index within target file (optional, provided as argument or `--index` flag, 0-based, defaults to end of file)
- Support command syntax:
  - `reqvire mv <element-name> <file> [<index>]` - positional arguments
  - `reqvire mv <element-name> --to-file=<file> --index=<n>` - with named flags
- Move element to target location at specified index
- Update all incoming relations system-wide with new identifier
- Preserve element content, metadata, and outgoing relations
- Apply changes immediately by default
- Support `--dry-run` flag to preview changes without applying
- Output git-style diff showing all affected files by default
- Support `--json` flag for structured output with relation updates and identifier change
- Report identifier change (old → new)
- Report error if element does not exist or target location is invalid
- Exit with code 0 on success, non-zero on error

#### Relations
  * derivedFrom: [Move Element Operation](../System/Operations/ElementManipulation.md#move-element-operation)
  * derivedFrom: [Subdirectory Processing Option](#subdirectory-processing-option)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
---

### CLI Move File Command

The system shall provide a `mv-file` command to move entire specification files with all their elements to a new location.

#### Details
The `mv-file` command shall:
- Accept source file path (required, relative to current working directory)
- Accept target file path (required, relative to current working directory)
- Support `--dry-run` flag to preview changes without applying
- Support `--json` flag for structured output
- Exit with code 0 on success, non-zero on error
- Command syntax: `reqvire mv-file <source-file> <target-file>`

#### Relations
  * derivedFrom: [Move File Operation](../System/Operations/ElementManipulation.md#move-file-operation)
  * derivedFrom: [Subdirectory Processing Option](#subdirectory-processing-option)
---

### CLI Rename Element Command

The system shall provide a `rename` command to rename existing model elements while automatically updating all relations that reference the renamed element.

#### Details
The `rename` command shall:
- Accept current element name (required)
- Accept new element name (required)
- Update all incoming relations system-wide with new identifier
- Apply changes immediately by default
- Support `--dry-run` flag to preview changes without applying
- Output git-style diff showing all affected files by default
- Support `--json` flag for structured output with relation updates and identifier change
- Report identifier change (old → new)
- Report error if element does not exist or new name conflicts with existing element
- Exit with code 0 on success, non-zero on error

#### Relations
  * derivedFrom: [Rename Element Operation](../System/Operations/ElementManipulation.md#rename-element-operation)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
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

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * derivedFrom: [Containment View Report Generation](../System/Output/Reporting.md#containment-view-report-generation)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * satisfiedBy: [containment.rs](../../core/src/containment.rs)
  * satisfiedBy: [diagrams.rs](../../core/src/diagrams.rs)
---