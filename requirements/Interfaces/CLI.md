# Elements

### Attachment Commands

The system shall provide CLI commands for attachment management: attach and detach.

#### Details
The attachment commands manage entries in the Attachments subsection of elements. Attachments can be either InternalPath files (PDFs, images, scripts) or Identifier references to refinement elements (behaviors, constraints, specifications).

**Attach Command:**
- Syntax: `reqvire attach <attachment-path> <element-name> [--dry-run]`
- Create Attachments subsection if doesn't exist
- Add link to subsection with format `* [display-name](path)`
- Skip if already attached (idempotent)
- Support many-to-many (same attachment to multiple elements)
- Support dry-run mode for preview

**Detach Command:**
- Syntax: `reqvire detach <element-name> <attachment-path> [--dry-run]`
- Remove link from Attachments subsection
- Remove subsection if no attachments remain
- Trigger change impact on element
- Support dry-run mode for preview

#### Attachments
  * [File Persistence Behavior](../System/Operations/Behaviors.md#file-persistence-behavior)
  * [Dry-Run Mode Behavior](../System/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](../System/Output/Specifications.md#diff-output-format-specification)

#### Relations
  * derivedFrom: [Reserved Subsections Support](../System/Core/StructureAndParsing.md#reserved-subsections-support)
  * verifiedBy: [Attach Command Verification](../System/Core/Verifications/AttachmentsVerifications.md#attach-command-verification)
  * verifiedBy: [Detach Command Verification](../System/Core/Verifications/AttachmentsVerifications.md#detach-command-verification)
---

### CLI Interface Structure

The CLI interface shall implement the clear `[OPTIONS] <COMMAND> [COMMAND OPTIONS]` structure.

#### Details
The CLI must display all commands and options and command's options flattened in the main help output which must also be a default command:
```
Reqvire requirements & traceability management tool

Usage: reqvire [OPTIONS] <COMMAND> [COMMAND OPTIONS]

Commands:
  export            Export model to browsable HTML documentation
  serve             Serve model as browsable HTML documentation via HTTP server
  format            Format and normalize requirements files
  validate          Validate model
  search            Search and filter model elements
  change-impact     Analyze change impact and provide report
  traces            Generate verification traces
  coverage          Generate verification coverage report
  model             Generate model-centric structure diagram
  lint              Analyze model quality and detect issues
  add               Add new element to model
  rm                Remove element from model
  mv                Move element to different location
  rename            Rename element
  mv-file           Move entire specification file
  attach            Attach document or Refinement element
  detach            Detach document or Refinement element
  link              Add relation between elements
  unlink            Remove relation between elements
  mv-asset          Move/rename asset file and update references
  rm-asset          Remove asset file and remove references
  containment       Generate containment view
  resources         Generate resources report
  help              Print help for commands

Options:
  -h, --help               Print help
  -V, --version            Print version

<COMMAND OPTIONS>:
  Each command has its own options displayed in a flattened section
  (e.g., FORMAT OPTIONS, VALIDATE OPTIONS, etc.)
```

#### Relations
  * derive: [CLI Add Element Command](#cli-add-element-command)
  * derive: [CLI Change Impact Report Command](#cli-change-impact-report-command)
  * derive: [CLI Containment Command](#cli-containment-command)
  * derive: [CLI Coverage Command](#cli-coverage-command)
  * derive: [CLI Lint Command](#cli-lint-command)
  * derive: [CLI Model Diagram Command](#cli-model-diagram-command)
  * derive: [CLI Move Asset Command](#cli-move-asset-command)
  * derive: [CLI Move Element Command](#cli-move-element-command)
  * derive: [CLI Move File Command](#cli-move-file-command)
  * derive: [CLI Remove Asset Command](#cli-remove-asset-command)
  * derive: [CLI Remove Element Command](#cli-remove-element-command)
  * derive: [CLI Rename Element Command](#cli-rename-element-command)
  * derive: [CLI Resources Command](#cli-resources-command)
  * derive: [CLI Search Command](#cli-search-command)
  * derive: [CLI Traces Command](#cli-traces-command)
  * derive: [Format Command](#format-command)
  * derive: [Validate Command](#validate-command)
  * derive: [Integrated Validation](../System/Core/Validation.md#integrated-validation)
  * derivedFrom: [CLI interface](Interfaces.md#cli-interface)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
---

### CLI Add Element Command

The system shall provide an `add` command to create new model elements by accepting element definition in Markdown format from stdin, validating the structure, and inserting it into the target file.

#### Details
The `add` command shall:
- Accept element definition input from standard input (stdin)
- Accept target file path as required positional argument (resolved relative to Git repository root)
- Support command syntax: `reqvire add <file>`
- Validate element structure before insertion
- Insert element into file following Element Ordering Behavior
- Apply changes immediately by default
- Support `--dry-run` flag to preview changes without applying
- Output git-style diff showing file changes by default
- Support `--json` flag for structured output format
- Report validation errors if element structure is invalid
- Exit with code 0 on success, non-zero on error

#### Attachments
  * [Git Repository Scope Specification](../System/Core/Specifications.md#git-repository-scope-specification)
  * [File Persistence Behavior](../System/Operations/Behaviors.md#file-persistence-behavior)
  * [Target Location Constraint](../System/Operations/Constraints.md#target-location-constraint)
  * [Dry-Run Mode Behavior](../System/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](../System/Output/Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../System/Output/Specifications.md#json-output-structure)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [CLI Add Element Test](../System/Operations/Verifications/ElementManipulationVerifications.md#cli-add-element-test)
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

#### Attachments
  * [JSON Output Structure](../System/Output/Specifications.md#json-output-structure)
  * [Text Output Formatting](../System/Output/Specifications.md#text-output-formatting)
  * [Change Propagation Behavior](../System/Processing/Behaviors.md#change-propagation-behavior)
  * [Mermaid Diagram Style Specification](../System/Output/Specifications.md#mermaid-diagram-style-specification)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [CLI Git Commit Hash Flag Test](Verifications/CLIVerifications.md#cli-git-commit-hash-flag-test)
  * verifiedBy: [CLI Help Structure Verification](Verifications/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Change Impact Analysis Verification](../System/Processing/Verifications/ChangeImpactVerifications.md#change-impact-analysis-verification)
  * verifiedBy: [Change Impact Detection Test](../System/Processing/Verifications/ChangeImpactVerifications.md#change-impact-detection-test)
  * verifiedBy: [Change Impact Relations Test](../System/Processing/Verifications/ChangeImpactVerifications.md#change-impact-relations-test)
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

#### Attachments
  * [JSON Output Structure](../System/Output/Specifications.md#json-output-structure)
  * [Short Mode Behavior](../System/Output/Behaviors.md#short-mode-behavior)
  * [Mermaid Diagram Style Specification](../System/Output/Specifications.md#mermaid-diagram-style-specification)
  * [ContainmentView.md](../System/Output/DesignDocuments/ContainmentView.md)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * satisfiedBy: [containment.rs](../../core/src/containment.rs)
  * satisfiedBy: [diagrams.rs](../../core/src/diagrams.rs)
---

### CLI Coverage Command

The system shall provide a `coverage` command that generates verification coverage reports focusing on leaf requirements, test-verification satisfaction status, and orphaned verifications.

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
- Follow [Verification Roll-up Strategy](../System/Processing/VerificationTraces.md#verification-roll-up-strategy)
- Test-verification elements require satisfiedBy relations to be considered satisfied
- Analysis, inspection, and demonstration verification elements are considered satisfied by default
- Exit with status code 0 on success
- Exit with non-zero status code on errors

Command output shall be written to stdout for easy redirection to files.

#### Attachments
  * [Verification Type Categories Specification](../System/Core/Specifications.md#verification-type-categories-specification)
  * [JSON Output Structure](../System/Output/Specifications.md#json-output-structure)
  * [Text Output Formatting](../System/Output/Specifications.md#text-output-formatting)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [CLI Help Structure Verification](Verifications/CLIVerifications.md#cli-help-structure-verification)
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

#### Attachments
  * [JSON Output Structure](../System/Output/Specifications.md#json-output-structure)
  * [Text Output Formatting](../System/Output/Specifications.md#text-output-formatting)
  * [Dry-Run Mode Behavior](../System/Operations/Behaviors.md#dry-run-mode-behavior)

#### Relations
  * derive: [Lint Auto-fix Capability](../System/Operations/Linting.md#lint-auto-fix-capability)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
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

#### Attachments
  * [JSON Output Structure](../System/Output/Specifications.md#json-output-structure)
  * [Mermaid Diagram Style Specification](../System/Output/Specifications.md#mermaid-diagram-style-specification)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * satisfiedBy: [diagrams.rs](../../core/src/diagrams.rs)
---

### CLI Move Asset Command

The system shall provide a `mv-asset` command to move or rename InternalPath files and automatically update all references across the model.

#### Details
The `mv-asset` command shall:
- Accept old file path as required positional argument
- Accept new file path as required positional argument
- Find all elements referencing the file as InternalPath
- Update all InternalPath references:
  - In Attachments subsection (update both display text and href)
  - In Relations (satisfiedBy, satisfy, trace targets)
- Physically move/rename the file on filesystem
- Apply changes immediately by default
- Support `--dry-run` flag to preview changes without applying
- Output git-style diff showing all affected files
- Report all affected elements and relation updates
- Exit with code 0 on success, non-zero on error

#### Attachments
  * [File Persistence Behavior](../System/Operations/Behaviors.md#file-persistence-behavior)
  * [Dry-Run Mode Behavior](../System/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](../System/Output/Specifications.md#diff-output-format-specification)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [Move Asset Command Verification](../System/Core/Verifications/AttachmentsVerifications.md#move-asset-command-verification)
---

### CLI Move Element Command

The system shall provide a `mv` command to move existing model elements to different file locations while automatically updating all relations that reference the moved element.

#### Details
The `mv` command shall:
- Accept element name as required positional argument
- Accept target file path as required positional argument (resolved relative to Git repository root)
- Support command syntax: `reqvire mv <element-name> <file>`
- Move element to target file following Element Ordering Behavior
- Update all incoming relations system-wide with new identifier
- Preserve element content, metadata, and outgoing relations
- Apply changes immediately by default
- Support `--dry-run` flag to preview changes without applying
- Output git-style diff showing all affected files by default
- Support `--json` flag for structured output with relation updates and identifier change
- Report identifier change (old → new)
- Report error if element does not exist or target location is invalid
- Exit with code 0 on success, non-zero on error

#### Attachments
  * [Git Repository Scope Specification](../System/Core/Specifications.md#git-repository-scope-specification)
  * [File Persistence Behavior](../System/Operations/Behaviors.md#file-persistence-behavior)
  * [Target Location Constraint](../System/Operations/Constraints.md#target-location-constraint)
  * [Dry-Run Mode Behavior](../System/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](../System/Output/Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../System/Output/Specifications.md#json-output-structure)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [Subdirectory Processing Verification](../System/Core/Verifications/ValidationVerifications.md#subdirectory-processing-verification)
  * verifiedBy: [CLI Move Element Test](../System/Operations/Verifications/ElementManipulationVerifications.md#cli-move-element-test)
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

#### Attachments
  * [File Persistence Behavior](../System/Operations/Behaviors.md#file-persistence-behavior)
  * [Target Location Constraint](../System/Operations/Constraints.md#target-location-constraint)
  * [Dry-Run Mode Behavior](../System/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](../System/Output/Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../System/Output/Specifications.md#json-output-structure)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [Subdirectory Processing Verification](../System/Core/Verifications/ValidationVerifications.md#subdirectory-processing-verification)
  * verifiedBy: [CLI Move File Test](../System/Operations/Verifications/ElementManipulationVerifications.md#cli-move-file-test)
---

### CLI Remove Asset Command

The system shall provide an `rm-asset` command to remove InternalPath files and automatically remove all references from the model.

#### Details
The `rm-asset` command shall:
- Accept file path as required positional argument
- Find all elements referencing the file as InternalPath
- Remove all InternalPath references:
  - From Attachments subsection (remove link entry, remove subsection if empty)
  - From Relations (remove entire relation line for satisfiedBy, satisfy, trace)
- Delete physical file from filesystem
- Apply changes immediately by default
- Support `--dry-run` flag to preview changes without applying
- Output git-style diff showing all affected files
- Report all affected elements and removed relations
- Exit with code 0 on success, non-zero on error

#### Attachments
  * [File Persistence Behavior](../System/Operations/Behaviors.md#file-persistence-behavior)
  * [Dry-Run Mode Behavior](../System/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](../System/Output/Specifications.md#diff-output-format-specification)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [Remove Asset Command Verification](../System/Core/Verifications/AttachmentsVerifications.md#remove-asset-command-verification)
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

#### Attachments
  * [File Persistence Behavior](../System/Operations/Behaviors.md#file-persistence-behavior)
  * [Dry-Run Mode Behavior](../System/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](../System/Output/Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../System/Output/Specifications.md#json-output-structure)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [CLI Remove Element Test](../System/Operations/Verifications/ElementManipulationVerifications.md#cli-remove-element-test)
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

#### Attachments
  * [File Persistence Behavior](../System/Operations/Behaviors.md#file-persistence-behavior)
  * [Dry-Run Mode Behavior](../System/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](../System/Output/Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../System/Output/Specifications.md#json-output-structure)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [CLI Rename Element Test](../System/Operations/Verifications/ElementManipulationVerifications.md#cli-rename-element-test)
---

### CLI Resources Command

The system shall provide a `resources` command that generates a report showing all files referenced by the model through relations and attachments.

#### Details
The command shall:
- Be invoked as `reqvire resources`
- Support `--json` flag for JSON output format
- Default to human-readable text output when JSON flag is not present
- Generate two sections: Relations and Attachments
- Show files from InternalPath relation targets (satisfiedBy, trace, etc.)
- Show files from FilePath attachment targets
- List files alphabetically by path
- For each file, show referencing elements with links
- Sort references by relation type (for relations section), then by element identifier
- Exit with status code 0 on success
- Exit with non-zero status code on errors

Command output shall be written to stdout for easy redirection to files.

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * derivedFrom: [Resources Report](../System/Output/Reporting.md#resources-report)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [Resources Report Verification](../System/Output/Verifications/ReportingVerifications.md#resources-report-verification)
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

#### Attachments
  * [Supported Element Types Specification](../System/Core/Specifications.md#supported-element-types-specification)
  * [JSON Output Structure](../System/Output/Specifications.md#json-output-structure)
  * [Short Mode Behavior](../System/Output/Behaviors.md#short-mode-behavior)
  * [Text Output Formatting](../System/Output/Specifications.md#text-output-formatting)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [CLI Help Structure Verification](Verifications/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Attachment Search Filters Verification](../System/Core/Verifications/AttachmentsVerifications.md#attachment-search-filters-verification)
  * verifiedBy: [Search Command Tests](../System/Output/Verifications/ReportingVerifications.md#search-command-tests)
---

### CLI Traces Command

The system shall provide a `traces` command that generates and outputs upward trace trees for verification elements, showing the complete requirement hierarchy from verifications to root requirements.

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

#### Attachments
  * [JSON Output Structure](../System/Output/Specifications.md#json-output-structure)
  * [Verification Trace Tree Construction](../System/Processing/Specifications.md#verification-trace-tree-construction)
  * [Mermaid Diagram Style Specification](../System/Output/Specifications.md#mermaid-diagram-style-specification)

#### Relations
  * derive: [Verification Traces Element Navigation](#verification-traces-element-navigation)
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [CLI Help Structure Verification](Verifications/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Verification Traces Filter Options Test](../System/Output/Verifications/ReportingVerifications.md#verification-traces-filter-options-test)
  * verifiedBy: [Verification Traces From-Folder Test](../System/Output/Verifications/ReportingVerifications.md#verification-traces-from-folder-test)
---

### Verification Traces Element Navigation

The system shall make verification element names in the traces report clickable links that navigate to the element's definition in its source file.

#### Details
- Verification element names displayed as headers shall be hyperlinks
- Links shall point to the verification's source file with fragment identifier
- Format: `[Verification Name](file_path#element-fragment)`
- Enables direct navigation from traces report to verification definition

#### Relations
  * derivedFrom: [CLI Traces Command](#cli-traces-command)
  * satisfiedBy: [verification_trace.rs](../../core/src/verification_trace.rs)
  * verifiedBy: [Verification Traces Element Navigation Test](Verifications/CLIVerifications.md#verification-traces-element-navigation-test)
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
  - Support --with-full-relations flag to include all registered relations (user-created and auto-generated)

Additional behavior:
  - By default (no --fix flag), preview changes without applying them
  - --fix flag applies the formatting changes to files
  - --json flag outputs formatting results in JSON format including file changes and diff information
  - When formatting is applied, show a summary of changed files with diff details
  - --with-full-relations flag inserts all relations from the model registry into elements, including auto-generated inverse relations (e.g., satisfiedBy from satisfy, derive from derivedFrom)

#### Attachments
  * [Dry-Run Mode Behavior](../System/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](../System/Output/Specifications.md#diff-output-format-specification)
  * [JSON Output Structure](../System/Output/Specifications.md#json-output-structure)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [CLI Help Structure Verification](Verifications/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Element Ordering Verification](../System/Operations/Verifications/FormattingVerifications.md#element-ordering-verification)
  * verifiedBy: [Format Command Requirements Verification](../System/Operations/Verifications/FormattingVerifications.md#format-command-requirements-verification)
  * verifiedBy: [Full Relations Insertion Verification](../System/Operations/Verifications/FormattingVerifications.md#full-relations-insertion-verification)
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

#### Attachments
  * [Two-Pass Validation Behavior](../System/Core/Behaviors.md#two-pass-validation-behavior)
  * [Validation Error Reporting Behavior](../System/Core/Behaviors.md#validation-error-reporting-behavior)
  * [JSON Output Structure](../System/Output/Specifications.md#json-output-structure)
  * [Error Message Format Specification](../System/Output/Specifications.md#error-message-format-specification)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [CLI Help Structure Verification](Verifications/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Invalid Relations Test](../System/Core/Verifications/ValidationVerifications.md#invalid-relations-test)
---

### Detailed Error Handling and Logging

The system shall implement detailed error handling and logging throughout the application to facilitate troubleshooting and provide meaningful feedback.

#### Relations
  * derive: [Validation Error Handling](../System/Core/Validation.md#validation-error-handling)
  * derivedFrom: [Enhanced Validation Error Reporting](../System/Core/Validation.md#enhanced-validation-error-reporting)
  * satisfiedBy: [error.rs](../../core/src/error.rs)
---

### Relation Commands

The system shall provide CLI commands for relation management: link and unlink.

#### Details
The `link` command shall:
- Accept syntax: `reqvire link <source> <relation-type> <target-element>`
- Source: existing element name or internal file path
- Target: existing element name
- Support `--dry-run` flag for preview

The `unlink` command shall:
- Accept syntax: `reqvire unlink <source> <relation-type> <target-element>`
- Source: existing element name or internal file path
- Target: existing element name
- Support `--dry-run` flag for preview

#### Attachments
  * [Relation Operations Specification](../System/Operations/Specifications.md#relation-operations-specification)
  * [RelationTypes.md](../System/Core/DesignDocuments/RelationTypes.md)
  * [Dry-Run Mode Behavior](../System/Operations/Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](../System/Output/Specifications.md#diff-output-format-specification)

#### Relations
  * derivedFrom: [CLI Interface Structure](#cli-interface-structure)
---
