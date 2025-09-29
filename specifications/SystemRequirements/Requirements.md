# System Requirements

## Linting

### Excess Whitespace Linting Implementation

The system shall detect and fix excess whitespace after element headers, subsection headers, and relation identifiers to maintain consistent formatting across all requirements documents.

#### Relations
  * derivedFrom: [Format Consistency Enforcement](../UserRequirements.md#format-consistency-enforcement)
  * satisfiedBy: [whitespace.rs](../../core/src/linting/whitespace.rs)
  * verifiedBy: [Format Command Requirements Verification](../Verifications/Misc.md#format-command-requirements-verification)

---

### Incosistent Newlines Linting Implementation

The system shall detect and fix excess or missing newlines before element headers, subsection headers to maintain consistent formatting across all requirements documents.

#### Relations
  * derivedFrom: [Format Consistency Enforcement](../UserRequirements.md#format-consistency-enforcement)
  * satisfiedBy: [newlines.rs](../../core/src/linting/newlines.rs)
  * verifiedBy: [Format Command Requirements Verification](../Verifications/Misc.md#format-command-requirements-verification)

---

### Missing Separators Linting Implementation

The system shall detect consecutive element sections that lack a separator line (---) between them and insert the separator to maintain consistent visual separation in the documentation.

#### Relations
  * derivedFrom: [Format Consistency Enforcement](../UserRequirements.md#format-consistency-enforcement)
  * satisfiedBy: [separators.rs](../../core/src/linting/separators.rs)
  * verifiedBy: [Format Command Requirements Verification](../Verifications/Misc.md#format-command-requirements-verification)

---

### Reserved Subsections Linting Implementation

The system shall identify and fix inconsistent indentation and bullet types in relation lists and other reserved subsections, standardizing to a consistent format across all requirements documents.

#### Relations
  * derivedFrom: [Format Consistency Enforcement](../UserRequirements.md#format-consistency-enforcement)
  * satisfiedBy: [indentation.rs](../../core/src/linting/indentation.rs)
  * verifiedBy: [Format Command Requirements Verification](../Verifications/Misc.md#format-command-requirements-verification)

---

### Git-Style Diff Output for Linting

The system shall display linting change suggestions in a git-style diff format, color-coded when possible, to clearly show what modifications will be or have been made to the documents.

#### Relations
  * derivedFrom: [Formatting Output](../UserRequirements.md#formatting-output)
  * satisfiedBy: [mod.rs](../../core/src/linting/mod.rs)
  * verifiedBy: [Format Command Requirements Verification](../Verifications/Misc.md#format-command-requirements-verification)

---

### Parallel Linting Processing

The system shall implement parallel processing for linting operations when possible, leveraging multi-core capabilities to improve performance on large documentation sets.

#### Relations
  * derivedFrom: [Model Formatting](../UserRequirements.md#model-formatting)
  * satisfiedBy: [mod.rs](../../core/src/linting/mod.rs)
  * verifiedBy: [Format Command Requirements Verification](../Verifications/Misc.md#format-command-requirements-verification)

---

### File Pattern Exclusion for Linting

The system shall respect configured excluded filename patterns when performing linting operations, ensuring that files intentionally excluded from processing do not receive inappropriate linting suggestions.

#### Relations
  * derivedFrom: [Ignoring Unstructured Documents](#ignoring-unstructured-documents)
  * satisfiedBy: [utils.rs](../../core/src/utils.rs)

---

## Configuration

### User Requirement Root Folders Support

The system shall implement configuration parameter that would specify a single folder path, relative to the Git repository root, that is designated as the primary location for user requirements.

#### Details

'paths.user_requirements_root_folder' parameter of type  String defines default folder for the user-requirements.

All elements in markdown files (except those matching exclusion patterns) in root of this folders are considered **user requirements** unless explicitly set as other element type in the metadata.

#### Relations
  * derivedFrom: [Configurable User Requirements Root Folder](../ManagingMbseModelsRequirements.md#configurable-user-requirements-root-folder)
  * satisfiedBy: [config.rs](../../cli/src/config.rs)

---

### Ignoring Unstructured Documents

The system shall support configurable glob patterns to exclude specific files from requirement processing.

#### Details
```reqvire.yaml
paths:

  # Glob patterns to exclude from structured documents processing
  excluded_filename_patterns:
    - "**/Logical*.md"
    - "**/Physical*.md"    
```

#### Relations
  * derivedFrom: [Project Configuration with YAML](../ManagingMbseModelsRequirements.md#project-configuration-with-yaml)
  * derivedFrom: [Coexistence of Structured and Unstructured Documents](../ManagingMbseModelsRequirements.md#coexistence-of-structured-and-unstructured-documents)
  * satisfiedBy: [config.rs](../../cli/src/config.rs)

---

## CLI

### CLI Interface Structure

The CLI interface shall implement the clear `[OPTIONS] <COMMAND> [COMMAND OPTIONS]` structure.

#### Details

The CLI must display all commands and options and command's options flattened in the main help output which must also be a default commnad:
```
Reqvire requirements & treacibility management tool

Usage: reqvire [OPTIONS] <COMMAND> [COMMAND OPTIONS]

Commands:
  format             Format and normalize requirements files (non-blocking) By default, changes will be applied automatically
  validate           Validate model
  help               Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>    Path to a custom configuration file (YAML format) If not provided, the system will look for reqvire.yml, reqvire.yaml, .reqvire.yml, or .reqvire.yaml in the current directory
  -h, --help               Print help
  -V, --version            Print version

LINT OPTIONS:
      --dry-run  When linting, only show suggestions without applying fixes
      --json  Output results in JSON format
```

#### Relations
  * derivedFrom: [CLI interface](../UserRequirements.md#cli-interface)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)

---

### Format Command

The system shall provide a formatting function, activated by the (format command), which shall execute the formatting process upon user request.

#### Details

`format` command shall:
  - Apply formatting changes to files when run without --dry-run flag
  - Display a diff-style summary of all changes made when formatting is applied
  - Provide a dry run mode (--dry-run option flag) that shows the suggested changes without applying them
  - Support --json flag for structured output of formatting results
  - Show git diff style output with line numbers and colors for both dry-run and actual formatting

Additional behavior:
  - --dry-run flag shows changes without applying them
  - --json flag outputs formatting results in JSON format including file changes and diff information
  - When formatting is applied, show a summary of changed files with diff details

#### Relations
  * derivedFrom: [Formatting Command](../UserRequirements.md#formatting-command)
  * containedBy: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)

---

### Validate Command

The system shall provide a validation command that executes model validation and reports any issues found, without performing any formatting or modification operations.

#### Details

`validate` command shall:
  - Execute the same validation logic as performed during the format command
  - Print all validation issues found in the model
  - Output a success message "No validation issues found" when the model is valid
  - Support --json flag to output validation results in JSON format

#### Relations
  * derivedFrom: [Provide Validation Reports](../UserRequirements.md#provide-validation-reports)
  * derivedFrom: [Enhanced Validation Error Reporting](../UserRequirements.md#enhanced-validation-error-reporting)
  * containedBy: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)

---

### Index Generation

The system shall implement an IndexGenerator component activated by the (generate-index command),  that traverses the specifications directory structure and creates a hierarchical SpecificationIndex.md file with links to documents and elements in the repository root.

#### Details

The index generator shall:
1. Traverse all specifications and documents in the model
2. Group elements by file and section
3. Create a hierarchical index with links to documents and elements
4. Generate summary statistics including total files, sections, and elements
5. Write the index as SpecificationIndex.md to the repository root, not the output folder

#### Relations
  * derivedFrom: [Generate Documentation Index](../UserRequirements.md#generate-documentation-index)
  * containedBy: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [index_generator.rs](../../core/src/index_generator.rs)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)

---

### HTML Navigation Enhancement

The system shall enhance the HTML generator to process SpecificationIndex.md as a special file, adding navigation elements and ensuring it serves as the primary entry point.

#### Details

SpecificationIndex.md file must be saved as index.html file when exported to the HTML output directory.

#### Relations
  * derivedFrom: [Documentation Index HTML Integration](../UserRequirements.md#documentation-index-html-integration)
  * satisfiedBy: [html.rs](../../core/src/html.rs)
  * satisfiedBy: [html_export.rs](../../core/src/html_export.rs)
  * verifiedBy: [HTML Export Verification](../Verifications/Misc.md#html-export-verification)

---

### HTML Export

The system shall generate HTML output ( activated by `html` command) for all markdown files, not just requirements documents, to provide consistent representation of the entire model.

#### Details

The system must accept `--output` command option flag for knowing where to export files. The default value must be 'html' folder:
 - folder will be created if not existing
 - .gitignore file must be added into the folder that ignores all files except .gitignore (itself)

#### Relations
  * derivedFrom: [Export HTML specifications](../UserRequirements.md#export-html-specifications)
  * containedBy: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [html_export.rs](../../core/src/html_export.rs)
  * satisfiedBy: [html.rs](../../core/src/html.rs)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)

---

### Export Related System Elements

The system shall ensure that any related system elements are also copied into output folder to ensure consistency of exported model.

#### Relations
  * refine: [HTML Export](#html-export)
  * satisfiedBy: [html_export.rs](../../core/src/html_export.rs)
  * satisfiedBy: [html.rs](../../core/src/html.rs)

---

### Detailed Error Handling and Logging

The system shall implement detailed error handling and logging throughout the application to facilitate troubleshooting and provide meaningful feedback.

#### Relations
  * derivedFrom: [Enhanced Validation Error Reporting](../UserRequirements.md#enhanced-validation-error-reporting)
  * satisfiedBy: [error.rs](../../core/src/error.rs)

---

## Logic

### Requirements Processing

The system shall parse the files in all folders and subfolders from the root of git repository which are not explicitly excluded using the configuration from reqvire.yaml.

#### Relations
  * derivedFrom: [User Requirement Root Folders Support](#user-requirement-root-folders-support)
  * derivedFrom: [Ignoring Unstructured Documents](#ignoring-unstructured-documents)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)

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
  * derivedFrom: [Project Configuration with YAML](../ManagingMbseModelsRequirements.md#project-configuration-with-yaml)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * containedBy: [CLI Interface Structure](#cli-interface-structure)

---

### Structured Markdown Files Search and Detection

The system shall identify all structured markdown documents available for processing in all directories and sub-directories of the git repository root based on predefined rules.

#### Details

Identification Process:
1. **File Selection**: The process scans all files in the the git repository root and all sub folders.
2. **Excluded Patterns Check**: If a file matches any excluded patterns, it is marked as **not a structured document file**.
3. **File Extension Check**: If the file does not have a `.md` extension, it is marked as **not a structured document file**.

#### Relations
  * derivedFrom: [Requirements Processing](#requirements-processing)
  * satisfiedBy: [model.rs](../../core/src/model.rs)

---

### Automated Diagram Generation on PR Merge

The system shall implement a GitHub workflow that automatically generates and commits updated diagrams when pull requests are merged to the main branch.

#### Details

The GitHub workflow shall:
- Be triggered only when a pull request is merged to the main branch (not on PR creation or updates)
- Check out the latest code from the main branch post-merge
- Build the Reqvire tool from source
- Run the diagram generation process using the `--generate-diagrams` flag
- Generate a traceability matrix SVG using the `--traces --svg` flags
- Check if any diagrams or matrix files have been added or modified
- Commit any updated files with a standardized commit message
- Push the updates back to the main branch

This ensures that the Mermaid diagrams in the repository are always up-to-date after changes are merged to the main branch, providing accurate visual representations of the latest model state without requiring manual intervention.

#### Relations
  * derivedFrom: [Automate Diagram Generation](../UserRequirements.md#automate-diagram-generation)
  * derivedFrom: [Automate Pull Request Validations](../UserRequirements.md#automate-pull-request-validations)
  * satisfiedBy: [generate_diagrams.yml](../../.github/workflows/generate_diagrams.yml)

---

### SysML-Compatible Relationship Rendering

The system shall implement a relationship rendering engine that adheres to SysML notation standards, defining specific arrow styles, line types, and visual properties for each relationship type to ensure diagram consistency and standards compliance.

#### Details

The visual representation and direction of relationships in diagrams aligns with the SysML specification. 
Each relationship is represented using SysML standard notation with a specified arrow direction.
derive (Forward):
- Definition: Links a parent element to child elements derived from it.
- Notation: Dashed arrow with an open arrowhead.
- Arrow Direction: Parent → Child (derived)
- Used when: Parent element wants to show its derived children

derivedFrom (Backward):
- Definition: Links a child element to the parent element it is derived from.
- Notation: Dashed arrow with an open arrowhead.
- Arrow Direction: Child → Parent (source)
- Used when: Child element references its source parent

contain (Forward):
- Definition: Links a parent element to the child elements it contains.
- Notation: Composite association with filled diamond.
- Arrow Direction: Container → Contained
- Used when: Parent element shows what it contains

containedBy (Backward):
- Definition: Links a child element to its containing parent element.
- Notation: Composite association with filled diamond.
- Arrow Direction: Contained → Container
- Used when: Child element references its container

refine (Backward):
- Definition: Links a child element to parent elements that it refines.
- Notation: Solid arrow with an open arrowhead.
- Arrow Direction: Refining → Refined (parent)
- Used when: Child element refines a parent element

refinedBy (Forward):
- Definition: Links a parent element to a child element that refines it.
- Notation: Solid arrow with an open arrowhead.
- Arrow Direction: Refined → Refining (child)
- Used when: Parent element shows which child refines it

verify (Backward):
- Definition: Links a verification artifact to the requirement it verifies.
- Notation: Dashed arrow with an open arrowhead.
- Arrow Direction: Verification → Requirement
- Used when: Test/verification references the requirement it validates

verifiedBy (Forward):
- Definition: Links a requirement to verification artifacts.
- Notation: Dashed arrow with an open arrowhead.
- Arrow Direction: Requirement → Verification
- Used when: Requirement shows how it's verified

satisfy (Backward):
- Definition: Links an implementation to the requirement it satisfies.
- Notation: Solid arrow with an open arrowhead.
- Arrow Direction: Implementation → Requirement
- Used when: Implementation references the requirement it satisfies

satisfiedBy (Forward):
- Definition: Links a requirement to elements that satisfy it.
- Notation: Solid arrow with an open arrowhead.
- Arrow Direction: Requirement → Implementation
- Used when: Requirement shows how it's implemented

trace (Neutral):
- Definition: Shows a general traceability relationship without implying hierarchy.
- Notation: Dashed arrow with an open arrowhead.
- Arrow Direction: Tracing → Traced (neutral)
- Used when: Simple traceability connection is needed  
  
**Summary Table**
| Relation        | Stereotype     | Line style            | Arrowhead               | Arrow Direction                   | Hierarchy Direction |
|-----------------|----------------|-----------------------|-------------------------|-----------------------------------|-------------------- |
| **derive**      | «deriveReqt»   | dashed dependency     | open (hollow) arrowhead | Parent → Child (derived)          | Forward             |
| **derivedFrom** | «deriveReqt»   | dashed dependency     | open (hollow) arrowhead | Child → Parent (source)           | Backward            |
| **satisfy**     | «satisfy»      | solid dependency      | open (hollow) arrowhead | Implementation → Requirement      | Backward            |
| **satisfiedBy** | «satisfy»      | solid dependency      | open (hollow) arrowhead | Requirement → Implementation      | Forward             |
| **verify**      | «verify»       | dashed dependency     | open (hollow) arrowhead | Verification → Requirement        | Backward            |
| **verifiedBy**  | «verify»       | dashed dependency     | open (hollow) arrowhead | Requirement → Verification        | Forward             |
| **refine**      | «refine»       | solid dependency      | open (hollow) arrowhead | Refining → Refined (parent)       | Backward            |
| **refinedBy**   | «refine»       | solid dependency      | open (hollow) arrowhead | Refined → Refining (child)        | Forward             |
| **contain**     | «contain»      | composite association | filled (black) diamond  | Container → Contained             | Forward             |
| **containedBy** | «contain»      | composite association | filled (black) diamond  | Contained → Container             | Backward            |
| **trace**       | «trace»        | dashed dependency     | open (hollow) arrowhead | Tracing → Traced (neutral)        | Forward             |

#### Relations
  * refine: [Diagram Generation](#diagram-generation)
  * satisfiedBy: [diagrams.rs](../../core/src/diagrams.rs)

---

### Diagram Relation Filtering

The system shall implement relation filtering in diagram generation to render only forward relations while ensuring complete element hierarchy representation starting from top-level parent elements.

#### Details

When generating diagrams, the system shall apply the following relation filtering rules:

1. **Diagram Relation Filtering**: Only relations specified in the DIAGRAM_RELATIONS list shall be rendered to prevent duplicate arrows representing the same logical relationship
2. **Complete Hierarchy Inclusion**: When any element in a hierarchical chain is included in a section, all parent elements up to the root of the hierarchy shall be automatically included in the diagram
3. **List-Based Rendering**: Relations shall be rendered according to the DIAGRAM_RELATIONS list which defines which relation from each opposite pair should be shown

The filtering ensures that:
- Bidirectional relationships (e.g., `contain`/`containedBy`, `derivedFrom`/`derive`) appear as single arrows using the relation specified in DIAGRAM_RELATIONS
- Hierarchical context is preserved by including parent elements even if they belong to different sections
- Diagram readability is maintained while accurately representing the complete model structure

#### Relations
  * refine: [SysML-Compatible Relationship Rendering](#sysml-compatible-relationship-rendering)
  * derivedFrom: [Relation Types and behaviors](../SpecificationsRequirements.md#relation-types-and-behaviors)
  * satisfiedBy: [diagrams.rs](../../core/src/diagrams.rs)

---

## Change Impact Analisys

### Structural Change Analyzer

The system shall implement a model change analyzer that identifies structural modifications between model versions, determines affected elements through relationship traversal, and categorizes impacts according to change propagation rules.

#### Relations
  * derivedFrom: [Tracing Structural Changes](../UserRequirements.md#tracing-structural-changes)
  * satisfiedBy: [change_impact.rs](../../core/src/change_impact.rs)

---

### CLI Change Impact Report Command

The system shall provide a change and impact report function, activated by the (change-impact command), which shall generate change impact report

#### Details

Must support `--json` option flag to output json formated string.

#### Relations
  * derivedFrom: [Structural Change Analyzer](#structural-change-analyzer)
  * containedBy: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)

---

### CLI Git Commit Hash Flag

The system shall provide a git commit hash flag  (--git_commit command option flag), to be used with ** CLI Change Impact Report Flag**.

#### Relations
  * derivedFrom: [CLI Change Impact Report Command](#cli-change-impact-report-command)
  * containedBy: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)

---

### CLI Traces Command

The system shall provide a traceability matrix generation function, activated by the (traces command), which shall generate a traceability matrix showing the relationships between requirements and verification elements.

#### Details

Must support `--json` and `--svg` command options flags to output either json formated string or svg vector image.

#### Relations
  * derivedFrom: [Traceability Matrix](../UserRequirements.md#traceability-matrix)
  * containedBy: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)

---

### Traceability Matrix Builder Implementation

The system shall implement a traceability matrix builder component that extracts relationship data from the model, processes it according to configured parameters, and generates structured matrix representations showing connections between requirements and other elements.

#### Details

The traceability matrix shall be organized into multiple tables, with one table per root requirement (requirements without parents). This organization improves readability by grouping related requirements together.

Each table shall have the following structure:
- The first column shows the requirement name as a markdown link to its location in the git repository using the current commit hash
- Requirements shall be displayed in a hierarchical structure with parent-child relationships clearly indicated
- Child requirements shall be indented to show their relationship to parent requirements using arrow and underscore characters:
  - Level 1 (direct children): "↳ " (right arrow followed by a space)
  - Level 2 (grandchildren): "__↳ " (two underscores, then arrow and space)
  - Level 3 (great-grandchildren): "____↳ " (four underscores, then arrow and space)
  - Deeper levels: "______↳ " (six underscores, then arrow and space)
- The second column shows the verification status with a green checkmark "✅" if verified by at least one verification element, or "❌" if not verified
- The subsequent columns represent individual verification elements that verify requirements in this group, with each element name displayed as a markdown link to its location in the git repository
- Each row represents a requirement within the group
- Cell intersections show the relationship between requirements and verifications with a green checkmark "✅" where a relationship exists or empty where no relationship exists
- The matrix shall be rendered as a markdown table for human readability
- The JSON format shall be available for machine processing with all identifiers relative to the repository root

The links in the matrix shall use the git repository URL with the current commit hash to ensure that links remain stable even as the repository evolves. The format shall be similar to that used in the change impact report.

#### Relations
  * derivedFrom: [Traceability Matrix](../UserRequirements.md#traceability-matrix)
  * satisfiedBy: [matrix_generator.rs](../../core/src/matrix_generator.rs)

---

### Markdown Matrix Formatter

The system shall implement a markdown formatter for traceability matrices that produces well-structured, readable markdown tables conforming to the Reqvire markdown-first methodology.

#### Relations
  * derivedFrom: [Traceability Matrix](../UserRequirements.md#traceability-matrix)
  * satisfiedBy: [matrix_generator.rs](../../core/src/matrix_generator.rs)

---

### CLI Traces SVG Flag

The system shall provide an SVG output option for traceability matrices, activated by the (--svg command option flag), which shall generate a simplified SVG representation of the matrix that can be viewed directly or embedded in documents.

#### Details

The SVG output of the matrix shall have the following characteristics:
- It shall only be available when the `traces` command is used
- It cannot be used together with the --json command option flag (they are mutually exclusive)
- It shall display full element names instead of truncated names with ellipses
- It shall dynamically adjust column widths based on the maximum element name length to ensure all text is readable
- It shall not include hyperlinks to elements in the git repository
- It shall maintain the same hierarchical structure as the markdown version
- It shall use the same visual indicators for verification status and relationships
- The output shall be in a self-contained SVG format suitable for embedding in other documents

#### Relations
  * derivedFrom: [CLI Traces Command](#cli-traces-command)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)

---

## Validation Capabilities

### Markdown Structure Validator

The system shall implement a markdown structure validator that enforces Reqvire's requirements for header levels, element structure, relation formatting, and other markdown-specific syntax rules, reporting violations with line numbers and suggested fixes.

#### Relations
  * derivedFrom: [Validate Markdown Structure](../UserRequirements.md#validate-markdown-structure)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)
  * verifiedBy: [Invalid Relations Test](../Verifications/ValidationTests.md#invalid-relations-test)

---

### Internal Consistency Validator

The system shall implement a consistency validator that verifies logical coherence within the model, including checking for circular dependencies, orphaned elements, and inconsistent relationship patterns, with detailed error reporting.

#### Relations
  * derivedFrom: [Validate Internal Consistency](../UserRequirements.md#validate-internal-consistency)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)
  * verifiedBy: [Invalid Relations Test](../Verifications/ValidationTests.md#invalid-relations-test)

---

### Cross-Component Dependency Validator

The system shall implement a specialized validator that analyzes dependencies across different model components, ensuring proper alignment between architectural layers, requirement levels, and verification elements.

#### Relations
  * derivedFrom: [Validate Cross-Component Dependencies](../UserRequirements.md#validate-cross-component-dependencies)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)
  * verifiedBy: [Invalid Relations Test](../Verifications/ValidationTests.md#invalid-relations-test)

---

### Relation Element Type Validator

The system shall implement validation that verifies relation endpoints have appropriate element types based on the relation type.

#### Details
- For `verifiedBy`/`verify` relations, validate that one endpoint is a requirement element and the other is a verification element
- For `satisfiedBy`/`satisfy` relations, validate that one endpoint is a requirement or test-verification element and the other is an implementation element
- For verification elements with `satisfiedBy` relations, validate that only test-verification elements may use satisfiedBy (other verification types should not have satisfiedBy relations)
- `trace` relations are always allowed for any verification type
- Relations should only connect elements of appropriate types based on the RelationTypesRegistry definition
- Warnings should be issued when relation endpoints have incompatible element types

#### Relations
  * derivedFrom: [Validate Relation Types](../UserRequirements.md#validate-relation-types)
  * derivedFrom: [Relation Types and behaviors](../SpecificationsRequirements.md#relation-types-and-behaviors)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)
  * verifiedBy: [Invalid Relations Test](../Verifications/ValidationTests.md#invalid-relations-test)

---

### Relation Type Validation

The system shall validate relation types against a defined vocabulary and provide clear error messages for unsupported relation types, including suggestions for the correct relation types.

#### Relations
  * derivedFrom: [Enhanced Validation Error Reporting](../UserRequirements.md#enhanced-validation-error-reporting)
  * satisfiedBy: [relation.rs](../../core/src/relation.rs)

---

### Excluded File Relation Validation

The system shall properly validate relations targeting files matching excluded filename patterns, enabling references to excluded files while still respecting their exclusion from processing and linting operations.

#### Details
The validation process for excluded files:
1. Files matching excluded patterns are registered in the element registry for relation validation only
2. Internal elements within excluded files are not processed or validated

#### Relations
  * derivedFrom: [Ignoring Unstructured Documents](#ignoring-unstructured-documents)
  * containedBy: [File Pattern Exclusion for Linting](#file-pattern-exclusion-for-linting)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)

---

## Reporting Features

### Model Summary Report Generator

The system shall implement a summary report generator that  produces comprehensive summaries of model relationships, including key metrics, element counts by type and counts.

#### Details

The summary report must include:

**File-level Information:**
- File path and name
- Number of sections per file
- Number of elements per file
- Page content (frontmatter content before first section header)

**Section-level Information:**
- Section name and hierarchy
- Number of elements per section
- Section content (content between section header and first element, excluding generated diagrams)

**Element Information:**
- Element identifier, name, type, and section
- Element content
- Verified and satisfied relations counts
- Complete list of relations with targets and types

**Global Counts:**
- Total files, pages, sections, and elements
- Requirements by type (system, user)
- Verifications by type (test, analysis, inspection, demonstration)
- Missing relations (unverified and unsatisfied requirements)

**Output Formats:**
- Human-readable text format with hierarchical display
- JSON format for programmatic processing
- Cypher format for graph database import

The system must support filtering by file path, element name, section, type, content, verification status, and satisfaction status. All filters are applied conjunctively.

#### Relations
  * derivedFrom: [Model Structure and Summaries](../UserRequirements.md#model-structure-and-summaries)
  * satisfiedBy: [reports.rs](../../core/src/reports.rs)

---

### Model Summary Fine Grained Filtering

The system shall implement a fine grained filtering for the  summary report generator following the specifications.

#### Details

<details><summary>View Full Specification</summary>

## Summary

This specification defines the functional requirements for a filtering subsystem used within the `model-summary` reporting feature. The system must allow clients to selectively include or exclude elements from the summary output based on metadata, content, and traceability properties.

The filters shall be composable and applied conjunctively (i.e., all active filters must match for an element to be included). The filtering system must support both human-readable text output and structured machine-readable output (e.g., JSON).

---

## Filtering Scope

Filtering shall operate on the level of individual `Element` objects in the model registry. Each `Element` has the following relevant properties:

- `file_path: String`
- `name: String`
- `section: String`
- `element_type: ElementType`
- `content: String`
- `relations: Vec<Relation>`

---

## Supported Filters

The filtering system **must support the following filters**, which may be active simultaneously.

### 1. File Path Filter (Glob)

**Purpose:** Restrict summary to elements defined in files whose paths match a given glob pattern.

**Input:** A single string pattern using glob syntax (e.g., `"src/**/*Spec.md"`)

**Match Target:** `Element.file_path`

**Behavior:** Case-sensitive glob match. If the glob does not match any file, no elements are included.

---

### 2. Name Filter (Regex)

**Purpose:** Include only elements whose `name` matches a regular expression.

**Input:** A valid Rust-compatible regular expression (e.g., `"autonomous.*"`)

**Match Target:** `Element.name`

**Behavior:** Case-sensitive match by default. The filter is considered invalid if the regex fails to compile.

---

### 3. Section Filter (Glob)

**Purpose:** Include only elements belonging to sections with matching names.

**Input:** A glob pattern string (e.g., `"System Requirements*"`)

**Match Target:** `Element.section`

**Behavior:** Case-sensitive match. Globbing follows standard `globset` semantics.

---

### 4. Type Filter (Exact Match)

**Purpose:** Include only elements of a specific type.

**Input:** One of the following valid string identifiers:

- `"user-requirement"`
- `"system-requirement"`
- `"verification"`
- `"file"`
- Any user-defined type (e.g., `"interface"`, `"design"`)

**Match Target:** `Element.element_type`

**Behavior:** Matching must be exact. Internally, the filter string shall be mapped to an `ElementType` via a deterministic lookup function.

---

### 5. Content Filter (Regex)

**Purpose:** Include only elements whose body content matches a regular expression.

**Input:** A valid regex pattern applied to the element’s `content`.

**Match Target:** `Element.content`

**Behavior:** Case-sensitive regex match. Invalid patterns must cause an immediate user-facing error.

---

### 6. Not Verified Filter (Boolean)

**Purpose:** Include only requirement elements that are not connected via a `verifiedBy` or `verify` relation.

**Input:** Boolean flag

**Match Target:** `Element.relations`

**Behavior:** When enabled, any element with one or more verification-related relations must be excluded.

---

### 7. Not Satisfied Filter (Boolean)

**Purpose:** Include only requirement elements that are not connected via a `satisfiedBy` or `satisfy` relation.

**Input:** Boolean flag

**Match Target:** `Element.relations`

**Behavior:** when enabled, any element with one or more satisfaction-related relations must be excluded.

---

## Filter Composition

All filters are applied **conjunctively**. That is, an element is included in the summary **only if all active filters return `true`** for that element.

---

## Error Handling

- Invalid regular expressions must produce a fatal error with a descriptive message.
- Invalid glob patterns should fail at startup with appropriate feedback.
- Unknown or malformed `type` filters should be rejected with a list of accepted values.

---

## Extension Considerations

The filtering system must be designed to allow future additions, including:

- Filtering by relation type presence (e.g., "has any relation")
- Filtering by linked element types (e.g., "verifiedBy test-verification")
- Inversion (e.g., "not in section X")

---

## Output Behavior

Filtered results must be consistent across all output modes (text, JSON, HTML). The final summary must include only elements passing the full filter set, and global counters should reflect the filtered subset.

---

## Performance Considerations

The filtering system must evaluate filters with minimal passes over element data. Repeated relation scans (e.g., for verification/satisfaction) should be avoided in favor of single-pass accumulation.

---

## Test Cases (Examples)

| Filter Combination | Expected Result |
|--------------------|------------------|
| `type = verification` | Only verification elements |
| `section = "System*"` + `name = ".*GPS.*"` | System section elements with GPS in name |
| `type = system-requirement` + `not_verified = true` | Unverified system requirements only |

---


</details>

#### Relations
  * satisfiedBy: [reports.rs](../../core/src/reports.rs)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * derivedFrom: [Model Summary Report Generator](#model-summary-report-generator)
  * verifiedBy: [Model Summary Tests](../Verifications/ReportsTests.md#model-summary-tests)

---

### CLI Summary Report Command

The system shall provide a model summary report function, activated by the (model-summary command), which shall generate model summary report with ability to be passed several filters.

#### Details

Model summary CLI command:
- `model-summary`:  Output model registry and summary, also supports json and cypher output.

All filters require `model-summary` to be present. They can be combined:
- `model-summary`:  Output model registry and summary, also supports json and cypher output.
  - By file path: ` --model-summary  --filter-file="src/**/*Reqs.md"`
  - By name: ` --model-summary  --filter-name=".*safety.*"`
  - By section: ` --model-summary  --filter-section="System*"`
  - By type: ` --model-summary  --filter-type="system-requirement"` (exact match)
  - By content: ` --model-summary  --filter-content="MUST"`
  - Not verified: ` --model-summary  --filter-is-not-verified`
  - Not satisfied: ` --model-summary  --filter-is-not-satisfied`

Must support `--json` and `--cypher` flags to output either json formated string or valid Cyper queries that when executed in graph database produce valid grapjh of a system model.

#### Relations
  * derivedFrom: [Model Summary Report Generator](#model-summary-report-generator)
  * containedBy: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)

---

### Handle Invalid Regex Filter Patterns

When the user invokes Reqvire with the `model-summary` and where invalid regular expression to regex based filters are provided the system shall return an error showing the faulty pattern and exit without producing a summary.

#### Relations
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * containedBy: [CLI Summary Report Command](#cli-summary-report-command)
  * verifiedBy: [Model Summary Tests](../Verifications/ReportsTests.md#model-summary-tests)

---

### Display Name-Regex Option in Help

When the user requests help (`--help` or `-h`), the system shall list module summary filter flags under the MODEL SUMMARY FILTERS heading, including its description and its dependency on `--model-summary`.

#### Relations
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * derivedFrom: [CLI Summary Report Command](#cli-summary-report-command)
  * verifiedBy: [Model Summary Tests](../Verifications/ReportsTests.md#model-summary-tests)

---

### Validation Report Generator

The system shall implement a validation report generator that compiles and formats validation results from all validators, providing a unified view of model quality with categorized issues, remediation suggestions, and compliance metrics.

#### Relations
  * derivedFrom: [Provide Validation Reports](../UserRequirements.md#provide-validation-reports)
  * satisfiedBy: [model.rs](../../core/src/model.rs)

---

### Verification Coverage Report Generator

The system shall provide a verification coverage report generator that analyzes leaf requirements verification status and test-verification satisfaction status to produce coverage metrics and detailed reports.

#### Details

The coverage report generator must:
- Identify all leaf requirements (requirements without forward relations to other requirements) in the model
- Determine leaf requirement verification status based on presence of verifiedBy relations
- Identify all test-verification elements in the model
- Determine test-verification satisfaction status based on presence of satisfiedBy relations
- Calculate coverage percentages: (verified_leaf_requirements/total_leaf_requirements * 100) and (satisfied_test_verifications/total_test_verifications * 100)
- Group results by file and section for organization
- Support both human-readable text and machine-readable JSON output formats

The report structure shall include:
- Summary section with leaf requirements and test-verification counts and percentages
- Verified leaf requirements section grouped by file and section
- Unverified leaf requirements section with details (flagged for attention)
- Satisfied test-verification elements section grouped by file and section
- Unsatisfied test-verification elements section with details (flagged for attention)
- Analysis, inspection, and demonstration verification elements are considered satisfied by default

#### Relations
  * derivedFrom: [Verification Coverage Report](../UserRequirements.md#verification-coverage-report)
  * derivedFrom: [Model Summary Report Generator](#model-summary-report-generator)
  * verifiedBy: [Verification Coverage Report Test](../Verifications/ReportsTests.md#verification-coverage-report-test)

---

### CLI Coverage Report Command

The system shall provide a command-line interface command `coverage-report` that generates verification coverage reports.

#### Details

The command shall:
- Be invoked as `reqvire coverage-report`
- Support `--json` flag for JSON output format
- Default to human-readable text output when JSON flag is not present
- Exit with status code 0 on success
- Exit with non-zero status code on errors

Command output shall be written to stdout for easy redirection to files.

#### Relations
  * derivedFrom: [Verification Coverage Report Generator](#verification-coverage-report-generator)
  * derivedFrom: [CLI Summary Report Command](#cli-summary-report-command)
  * verifiedBy: [Verification Coverage Report Test](../Verifications/ReportsTests.md#verification-coverage-report-test)

---

## Diagrams

### Diagram Generation

When requested, the system shall automatically generate diagrams and save them to the required locations of the model.

#### Relations
  * derivedFrom: [Interactive Mermaid Diagrams](../UserRequirements.md#interactive-mermaid-diagrams)
  * satisfiedBy: [diagrams.rs](../../core/src/diagrams.rs)

---

### Auto-Generated Diagram Identification

The system shall embed a unique identification marker "REQVIRE-AUTOGENERATED-DIAGRAM" as a comment within all auto-generated mermaid diagrams to distinguish them from user-created diagrams for reliable filtering and removal operations.

#### Details

The marker must be:
- Embedded as a mermaid comment line using the `%% REQVIRE-AUTOGENERATED-DIAGRAM` format
- Present in every auto-generated diagram
- Used by diagram removal functionality to identify which diagrams to remove
- Not present in user-created custom diagrams

This enables the system to:
- Reliably identify auto-generated diagrams regardless of their location in documents
- Preserve user-created diagrams during removal operations
- Support mixed documents containing both auto-generated and custom diagrams

#### Relations
  * refine: [Diagram Generation](#diagram-generation)
  * satisfiedBy: [diagrams.rs](../../core/src/diagrams.rs)
  * satisfiedBy: [utils.rs](../../core/src/utils.rs)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)
  * verifiedBy: [Diagram Generation Test](../Verifications/DiagramsTests.md#diagram-generation-test)
  * verifiedBy: [Remove Generated Diagrams Verification](../Verifications/DiagramsTests.md#remove-generated-diagrams-verification)

---

### CLI Generate Diagrams Flag

The system shall provide a diagrams generation function, activated by the (generate-diagrams command), which shall generate interactive mermaid diagrams.

#### Relations
  * refine: [Diagram Generation](#diagram-generation)
  * containedBy: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [Diagram Generation Test](../Verifications/DiagramsTests.md#diagram-generation-test)

---

### CLI Remove Diagrams Flag

The system shall provide a diagram removal function, activated by the remove-diagrams command, which shall remove all generated mermaid diagrams from the model.

#### Relations
  * refine: [Diagram Removal](#diagram-removal)
  * containedBy: [CLI Interface Structure](#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * verifiedBy: [Remove Generated Diagrams Verification](../Verifications/DiagramsTests.md#remove-generated-diagrams-verification)

---

### Diagram Removal

When requested, the system shall remove all generated diagrams from the model by locating and deleting all mermaid code blocks that were automatically generated.

#### Relations
  * derivedFrom: [Remove Generated Diagrams](../UserRequirements.md#remove-generated-diagrams)
  * satisfiedBy: [diagrams.rs](../../core/src/diagrams.rs)
  * verifiedBy: [Remove Generated Diagrams Verification](../Verifications/DiagramsTests.md#remove-generated-diagrams-verification)

---

### Interactive Mermaid Diagram Node Behavior

The system shall implement interactive click behavior for Mermaid diagram nodes that redirects to the referenced element.

#### Details

Clickable mermaid diagrams links by default must use use relative links to the git repository.

Configuration options must be provided that can change default behavior to use stable github repository links:
  * diagrams click links are not working on Github if not useng stable github repository links
  * from another side that polutes PR diffs thus choise must be given to the user
  * reqvire.yaml config must expose `style.diagrams_with_blobs: bool` for that purpose.
  
When generating diagram node links and when `style.diagrams_with_blobs` is set to `true`, the system shall:
- Use stable git repository links (`{repository-url}/blob/{commit-hash}/{file-path}`) when git repository information is available
- Fallback to relative markdown links when git repository information is not available
- Use the current commit hash to ensure links remain stable even as the repository evolves
- Match the same link format used in traceability matrices and change impact reports
- Preserve interactive behavior across all generated diagrams

#### Relations
  * refine: [Diagram Generation](#diagram-generation)
  * satisfiedBy: [diagrams.rs](../../core/src/diagrams.rs)
  * satisfiedBy: [config.rs](../../cli/src/config.rs)

---

### Trace Relation Non-Directional Behavior

The system shall treat trace relations as non-directional for circular dependency detection while maintaining their traceability purpose, ensuring that trace relations do not participate in cycle detection algorithms.

#### Details

The trace relation behavior shall include:

1. **Circular Dependency Exclusion**:
   - Trace relations shall not be traversed during circular dependency detection
   - The cycle detection algorithm shall skip trace relations to prevent false positive cycles
   - Trace relations exist solely for traceability and documentation purposes

2. **Non-Propagation Behavior**:
   - Changes shall not propagate through trace relations
   - Trace relations shall not be included in change impact analysis
   - Impact trees shall not traverse trace relation connections

3. **Bidirectional Traceability**:
   - Trace relations shall provide bidirectional navigational capability
   - Users can navigate from source to target and target to source
   - Both directions are semantically equivalent for traceability purposes

4. **Validation Behavior**:
   - Trace relations shall be validated for target existence
   - Trace relations shall not require type compatibility validation
   - Trace relations can connect any element type to any other element type

This ensures that trace relations serve their intended purpose of establishing lightweight traceability connections without creating artificial dependency constraints or participating in architectural validation logic.

#### Relations
  * derivedFrom: [Relation Types and behaviors](../SpecificationsRequirements.md#relation-types-and-behaviors)
  * verifiedBy: [Invalid Relations Test](../Verifications/ValidationTests.md#invalid-relations-test)
  * verifiedBy: [Trace Relations No Cycles Verification](../Verifications/TraceRelationTests.md#trace-relations-no-cycles-verification)

---
