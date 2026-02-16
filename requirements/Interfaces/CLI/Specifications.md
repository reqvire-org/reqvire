# Elements

### CLI Add Element Command Refinement Specification

Specification extracted from requirement "CLI Add Element Command".

#### Details
The `add` command shall:
- Accept element definition input from standard input (stdin) by default
- Support `--content <string>` option as alternative to stdin input
- When `--content` is provided, use its value instead of reading stdin
- Accept target file path as required positional argument (resolved relative to Git repository root)
- Support command syntax: `reqvire add <file>` (stdin) or `reqvire add <file> --content "..."`
- Validate element structure before insertion
- Insert element into file following Element Ordering Behavior
- Apply changes immediately by default
- Support `--dry-run` flag to preview changes without applying
- Support `--override` flag to replace existing element with same name
- Output git-style diff showing file changes by default
- Support `--json` flag for structured output format
- Report validation errors if element structure is invalid
- Exit with code 0 on success, non-zero on error

#### Metadata
  * type: specification
---

### CLI Collect Command Refinement Specification

Specification extracted from requirement "CLI Collect Command".

#### Details
Command syntax: `reqvire collect <element-name> [--direction UPSTREAM|DOWNSTREAM] [--json]`

**Arguments:**
- `<element-name>` - Required. Name of the requirement element to collect from.

**Options:**
- `--direction <DIRECTION>` - Traversal direction. Values: `UPSTREAM` (default) or `DOWNSTREAM`. UPSTREAM traverses derivedFrom relations to ancestors; DOWNSTREAM traverses derive relations to descendants.
- `--json` - Output in JSON format instead of text

**Exit codes:**
- 0 on success
- Non-zero on error (element not found, invalid type, invalid direction, etc.)

#### Metadata
  * type: specification
---

### CLI Interface Structure Refinement Specification

Specification extracted from requirement "CLI Interface Structure".

#### Details
The CLI must display all commands and options and command's options flattened in the main help output which must also be a default command:
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
coverage          Generate verification and implementation coverage report
model             Generate model-centric structure diagram
lint              Analyze model quality and detect issues
add               Add new element to model
rm                Remove element from model
mv                Move element to different location
rename            Rename element
merge             Merge multiple elements into target element
mv-file           Move entire specification file
link              Add relation or attachment between elements
unlink            Remove relation or attachment (auto-detects)
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

#### Metadata
  * type: specification
---

### CLI Move Element Command Refinement Specification

Specification extracted from requirement "CLI Move Element Command".

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
- Reject moves into existing `# Documents` files when the move would create multiple elements in that file
- Exit with code 0 on success, non-zero on error

#### Metadata
  * type: specification
---

### CLI Move File Command Refinement Specification

Specification extracted from requirement "CLI Move File Command".

#### Details
The `mv-file` command shall:
- Accept source file path (required, relative to current working directory)
- Accept target file path (required, relative to current working directory)
- Support `--dry-run` flag to preview changes without applying
- Support `--json` flag for structured output
- Exit with code 0 on success, non-zero on error
- Command syntax: `reqvire mv-file <source-file> <target-file>`
- When `--squash` is used, reject if target is an existing `# Documents` file

#### Metadata
  * type: specification
---

### CLI Search Command Refinement Specification

Specification extracted from requirement "CLI Search Command".

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

#### Metadata
  * type: specification
---

### CLI Traces Command Refinement Specification

Specification extracted from requirement "CLI Traces Command".

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
# Generate traces with links relative to docs/reports/ folder
reqvire traces --from-folder=docs/reports > docs/reports/traces.md

# Links in the output will be relative to docs/reports/ (e.g., ../../specifications/file.md)

#### Metadata
  * type: specification
---

### Format Command Refinement Specification

Specification extracted from requirement "Format Command".

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

#### Metadata
  * type: specification
---
