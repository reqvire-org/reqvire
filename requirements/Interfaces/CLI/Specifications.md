# Elements

### CLI Add Element Command Refinement Specification

#### Details
The `add` command is expected to:
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

#### Details
Command syntax: `reqvire collect <element-name> [--direction UPSTREAM|DOWNSTREAM] [--json]`

**Arguments:**
- `<element-name>` - Required. Name of the capability or requirement element to collect from.

**Options:**
- `--direction <DIRECTION>` - Traversal direction. Values: `UPSTREAM` (default) or `DOWNSTREAM`. Requirement UPSTREAM traverses requirement parents and crosses to the owning capability through `specify`; requirement DOWNSTREAM follows child requirements. Capability UPSTREAM follows capability parents only; capability DOWNSTREAM follows child capabilities and requirements through `specifiedBy`.
- `--json` - Output in JSON format instead of text

**Exit codes:**
- 0 on success
- Non-zero on error (element not found, invalid type, invalid direction, etc.)

#### Metadata
 * type: specification
---

### CLI Coverage Command Refinement Specification

#### Details
Coverage command behavior:
- Be invoked as `reqvire coverage`
- Support `--json` flag for JSON output format
- Default to human-readable text output when JSON flag is not present
- Generate reports focusing on leaf requirements (requirements without forward relations to other requirements)
- Show the percentage and details of verified and unverified leaf requirements
- Include breakdowns by file, section, and verification type
- Show satisfaction status of test-verification elements (those with `satisfiedBy` relations)
- Show orphaned verifications (verification elements without any `verify` relations to capabilities or requirements)
- Include requirement implementation coverage summary for `requirement` elements only
- Include capability coverage roll-up from requirements connected through `specifiedBy` / `specify`
- Classify covered requirements using the implementation coverage source vocabulary defined by the Reqvire report ontology
- Show implementation-uncovered requirements with identifiers and names
- Emit all coverage percentages with at most 2 decimal places in text and JSON output
- Uses [Verification Roll-up Strategy](../../Functional/Processing/VerificationTraces.md#verification-roll-up-strategy)
- Treat test-verification elements as satisfied only when they have `satisfiedBy` relations
- Treat analysis, inspection, and demonstration verification elements as satisfied by default
- Exit with status code 0 on success
- Exit with non-zero status code on errors

Command output is written to stdout for easy redirection to files.

#### Metadata
 * type: specification

#### Relations
 * refine: [CLI Coverage Command](Commands.md#cli-coverage-command)
---

### CLI Ontologies Command Refinement Specification

#### Details
The `ontologies` command shall collect ontology `#### Ontology` content and semantic-contract `#### Shapes` content for downstream tooling:
- Command syntax: `reqvire ontologies [--jsonld] [--full] [--output <FILE>]`
- Default output format: RDF/Turtle (`.ttl`)
- `--jsonld`: emit JSON-LD instead of Turtle
- `--full`: include generated RDF triples for Reqvire model elements, relations, attachments, concept references, ontology declarations, and semantic-contract shape references
- `--output <FILE>`: write the selected format to the requested file instead of stdout
- The command shall use the graph-registry semantic index used by validation.
- The default mode shall preserve the current artifact-only export of authored ontology and SHACL blocks.
- The full mode shall append an in-memory RDF projection of the Reqvire graph registry context without requiring a persistent RDF store.

#### Metadata
 * type: specification

#### Relations
 * refine: [CLI Ontologies Command](Commands.md#cli-ontologies-command)
---

### CLI Interface Structure Refinement Specification

#### Details
The CLI must display all commands and options and command's options flattened in the main help output which must also be a default command:
Reqvire requirements & traceability management tool

Usage: reqvire [OPTIONS] <COMMAND> [COMMAND OPTIONS]

Commands:
export Export model to browsable HTML documentation
serve Serve model as browsable HTML documentation via HTTP server
format Format and normalize requirements files
validate Validate model
search Search and filter model elements
change-impact Analyze change impact and provide report
traces Generate verification traces
coverage Generate verification and implementation coverage report
model Generate model-centric structure diagram
lint Analyze model quality and detect issues
add Add new element to model
rm Remove element from model
mv Move element to different location
rename Rename element
merge Merge multiple elements into target element
mv-file Move entire specification file
link Add relation or attachment between elements
unlink Remove relation or attachment (auto-detects)
mv-asset Move/rename asset file and update references
rm-asset Remove asset file and remove references
containment Generate containment view
resources Generate resources report
ontologies Collect ontology elements and semantic-contract SHACL shapes
help Print help for commands

Ontologies options:
--jsonld Output JSON-LD format instead of RDF/Turtle (.ttl)
--full Include Reqvire model context triples in the semantic export
--output <FILE> Save output to file

Options:
-h, --help Print help
-V, --version Print version

<COMMAND OPTIONS>:
Each command has its own options displayed in a flattened section
(e.g., FORMAT OPTIONS, VALIDATE OPTIONS, etc.)

#### Metadata
 * type: specification

#### Relations
 * refine: [CLI Interface Structure](Commands.md#cli-interface-structure)
---

### CLI Size Estimate JSON Option Specification

The CLI `--with-size-estimates` option is expected to be an opt-in JSON evidence option.

#### Details
- Supported report commands may expose `--with-size-estimates`.
- The option passes `with_size_estimates = true` into model loading for that command invocation.
- The option is accepted only when `--json` is also present.
- Using `--with-size-estimates` without `--json` fails before report execution with an actionable diagnostic.
- The initial supported command is `model --json --with-size-estimates`.
- Additional JSON evidence commands may opt in later after their output contracts are specified.

#### Metadata
 * type: specification

#### Relations
 * refine: [CLI Size Estimate JSON Option](Commands.md#cli-size-estimate-json-option)
---

### Explicit Workspace Selection Specification

The CLI is expected to resolve and enter an explicitly selected workspace before executing Reqvire operations.

#### Details
Workspace selection rules:
- The global option is `--workspace <DIR>`.
- The selected workspace path is resolved and canonicalized before command execution.
- The selected workspace must exist and be a directory.
- The process current directory is changed to the selected workspace before loading `.gitignore`, `.reqvireignore`, scanning Markdown files, resolving git root, computing reports, or executing mutations.
- If the selected workspace is a git repository subdirectory, existing Reqvire subtree-scoped scanning behavior is preserved.
- If `--workspace` is not provided, existing current-working-directory behavior is preserved.
- Workspace selection is process startup configuration and is not exposed as an MCP tool argument.
- MCP workspace/session responses report the effective workspace after selection.
- Invalid workspace paths fail before command execution with a clear error.

#### Metadata
 * type: specification

#### Relations
 * refine: [Explicit Workspace Selection](Commands.md#explicit-workspace-selection)
---

### Diff Output Format Specification

Git-style diff format for change previews.

#### Details
**Used by commands:** format, lint, add, rm, mv, rename, mv-file, change-impact

**Format:**
```diff
--- a/<file_path>
+++ b/<file_path>
@@ -<old_start>,<old_count> +<new_start>,<new_count> @@
-<removed line>
+<added line>
 <context line>
```

**Colors:**
- Red: Removed lines (-)
- Green: Added lines (+)
- Cyan: Hunk headers (@@)
- White: Context lines

**Context:**
- Show 3 lines before and after changes
- Collapse large unchanged sections

#### Metadata
 * type: specification

#### Relations
 * refine: [CLI Diff Output](Commands.md#cli-diff-output)
---

### Error Message Format Specification

Structure for error and warning messages.

#### Details
**Format:**
```
<file_path>:<line_number>: <level>: <message>
 <context_line>
 ^--- <pointer to issue>
```

**Fields:**
- `file_path`: Git-root-relative path
- `line_number`: 1-based line number
- `level`: error | warning | info
- `message`: Concise description
- `context_line`: Source line (optional)
- `suggestion`: How to fix (optional)

**Grouping:**
- Group errors by file
- Sort by line number within file

#### Metadata
 * type: specification

#### Relations
 * refine: [Detailed Error Handling and Logging](Commands.md#detailed-error-handling-and-logging)
---

### CLI JSON File Output Option Refinement Specification

#### Details
`--output` option behavior:
- Be available on every command that has a `--json` flag.
- Require `--json` to also be set; report an error when `--output` is used without `--json`.
- Write JSON content to the specified file path.
- Create the file if it does not exist, and overwrite it if it does.
- Print a confirmation message to stdout: `✅ Output saved to <filepath>`.
- Exit with code 0 on success and non-zero on file write error.

#### Metadata
 * type: specification

#### Relations
 * refine: [CLI JSON File Output Option](Commands.md#cli-json-file-output-option)
---

### CLI Lint Command Refinement Specification

#### Details
Lint command behavior:
- Be invoked as `reqvire lint`.
- Default to dry-run mode (report issues without applying fixes).
- Support `--fixable` to show only auto-fixable issues.
- Support `--auditable` to show only issues requiring manual review.
- Support `--fix` to apply auto-fixable changes.
- Support `--json` for structured output.
- Default to showing all issues when no filter flags are provided.
- Categorize output into:
 - Auto-fixable issues.
 - Needs manual review issues.
- Exit with code 0 when no issues are found or when fixes are successfully applied.
- Exit with non-zero status code on errors.

Command output is written to stdout for easy redirection to files.

#### Metadata
 * type: specification

#### Relations
 * refine: [CLI Lint Command](Commands.md#cli-lint-command)
---

### CLI Merge Element Command Refinement Specification

#### Details
Merge command behavior:
- Accept target element name as the first required positional argument.
- Accept one or more source element names as subsequent required arguments.
- Support command syntax: `reqvire merge <target> <source1> [source2...]`.
- Reject merge when a source is in `# Documents` format and target is in `# Elements` format; report that manual migration is required.
- Apply changes immediately by default.
- Support `--dry-run` to preview changes without applying.
- Output git-style diff showing all affected files by default.
- Support `--json` for structured output.
- Exit with code 0 on success and non-zero on error.

#### Metadata
 * type: specification

#### Relations
 * refine: [CLI Merge Element Command](Commands.md#cli-merge-element-command)
---

### CLI Model Diagram Command Refinement Specification

#### Details
Model command behavior:
- Be named `model`.
- Support `--from=<name>` for filtering from a specific element by name.
- Use globally unique element names for name-based lookup.
- Support `--json` for JSON output format.
- Support `--mmd` for pure Mermaid flowchart output without Markdown wrapper text or fenced code blocks.
- Support `--reverse` for leaf-to-root traversal.
- Support `--filter-type=<types>` with comma-separated element types to filter starting points.
- Default to Markdown output with embedded Mermaid diagram.
- When neither `--from` nor `--filter-type` is provided in forward mode, use ontology roots and capability roots as default starting elements.
- Integrate with existing model diagram generation functionality.

#### Metadata
 * type: specification

#### Relations
 * refine: [CLI Model Diagram Command](Commands.md#cli-model-diagram-command)
---

### CLI Move Asset Command Refinement Specification

#### Details
`mv-asset` command behavior:
- Accept old file path as required positional argument.
- Accept new file path as required positional argument.
- Find all elements referencing the file as InternalPath.
- Update all InternalPath references:
 - In `Attachments` subsection (update both display text and href).
 - In relations (`satisfiedBy`, `satisfy`, `trace` targets).
- Physically move/rename the file on the filesystem.
- Apply changes immediately by default.
- Support `--dry-run` to preview changes without applying.
- Output git-style diff showing all affected files.
- Support `--json` for structured output.
- Support `--output <FILE>` when `--json` is present.
- Report all affected elements and relation updates.
- Exit with code 0 on success and non-zero on error.

#### Metadata
 * type: specification

#### Relations
 * refine: [CLI Move Asset Command](Commands.md#cli-move-asset-command)
---

### CLI Move Element Command Refinement Specification

#### Details
The `mv` command is expected to:
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

#### Details
The `mv-file` command is expected to:
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

### CLI Relink Command Refinement Specification

#### Details
`relink` command behavior:
- Accept source element name as required positional argument.
- Accept relation type as required positional argument.
- Accept old target and new target as required positional arguments.
- Support command syntax: `reqvire relink <source> <relation-type> <old-target> <new-target>`.
- Invoke the functional atomic relation relink operation.
- Validate candidate model state before persistence.
- Support `--dry-run` preview and `--json` output.
- Support `--output <FILE>` when `--json` is present.
- Reject unresolved source/target references, missing source relation, and post-relink validation failures with non-zero exit status.

#### Metadata
 * type: specification

#### Relations
 * refine: [CLI Relink Command](Commands.md#cli-relink-command)
---

### CLI Remove Asset Command Refinement Specification

#### Details
`rm-asset` command behavior:
- Accept file path as required positional argument.
- Find all elements referencing the file as InternalPath.
- Remove all InternalPath references:
 - From `Attachments` subsection (remove link entry and remove subsection if empty).
 - From relations (remove entire relation line for `satisfiedBy`, `satisfy`, `trace`).
- Delete the physical file from the filesystem.
- Apply changes immediately by default.
- Support `--dry-run` to preview changes without applying.
- Output git-style diff showing all affected files.
- Support `--json` for structured output.
- Support `--output <FILE>` when `--json` is present.
- Report all affected elements and removed relations.
- Exit with code 0 on success and non-zero on error.

#### Metadata
 * type: specification

#### Relations
 * refine: [CLI Remove Asset Command](Commands.md#cli-remove-asset-command)
---

### CLI Remove Element Command Refinement Specification

#### Details
`rm` command behavior:
- Accept element name as required positional argument.
- Support command syntax: `reqvire rm <element-name>`.
- Delete the specified element from its file.
- Remove all incoming relations from other elements.
- Apply changes immediately by default.
- Support `--dry-run` to preview changes without applying.
- Output git-style diff showing file changes by default.
- Support `--json` for structured output with affected relations.
- Report error when the element does not exist.
- Exit with code 0 on success and non-zero on error.

#### Metadata
 * type: specification

#### Relations
 * refine: [CLI Remove Element Command](Commands.md#cli-remove-element-command)
---

### CLI Rename Element Command Refinement Specification

#### Details
`rename` command behavior:
- Accept current element name (required).
- Accept new element name (required).
- Update all incoming relations system-wide with the new identifier.
- Apply changes immediately by default.
- Support `--dry-run` to preview changes without applying.
- Output git-style diff showing all affected files by default.
- Support `--json` for structured output with relation updates and identifier change.
- Report identifier change (`old -> new`).
- Report error if the element does not exist or the new name conflicts with an existing element.
- Exit with code 0 on success and non-zero on error.

#### Metadata
 * type: specification

#### Relations
 * refine: [CLI Rename Element Command](Commands.md#cli-rename-element-command)
---

### CLI Resources Command Refinement Specification

#### Details
Resources command behavior:
- Be invoked as `reqvire resources`.
- Support `--json` for JSON output format.
- Default to human-readable text output when `--json` is not present.
- Generate two sections: Relations and Attachments.
- Show files from InternalPath relation targets (`satisfiedBy`, `trace`, and related path-based targets).
- Show referenced refinement identifiers from attachment targets.
- List relation file entries and attachment identifier entries alphabetically.
- For each entry, show referencing elements with links.
- Sort references by relation type (for relations section), then by element identifier.
- Exit with status code 0 on success and non-zero on errors.

Command output is written to stdout for easy redirection to files.

#### Metadata
 * type: specification

#### Relations
 * refine: [CLI Resources Command](Commands.md#cli-resources-command)
---

### CLI Search Command Refinement Specification

#### Details
Search command capabilities:
- `search`: Search model elements and output results to stdout
- Support `--json` flag for structured JSON output
- Support `--short` flag for abbreviated output (both text and JSON)
- Support comprehensive filter options (all combinable):
- By file path glob: `--filter-file="src/**/*Reqs.md"`
- By element name regex: `--filter-name=".*safety.*"`
- By element type: `--filter-type="requirement"` (exact match)
- By effective governance status: `--filter-status=approved`
- By effective governance priority: `--filter-priority=high,critical`
- By effective governance risk: `--filter-risk=high,critical`
- By effective governance owner regex: `--filter-owner="Platform.*"`
- By element content regex: `--filter-content="MUST"`
- By page content regex: `--filter-page-content="architecture"`
- By having relations: `--have-relations=verifiedBy,satisfiedBy` (comma-separated, must have ALL)
- By not having relations: `--not-have-relations=verifiedBy` (comma-separated, must NOT have ALL)
- By having attachments: `--has-attachments` (filter elements with Attachments subsection)
- By attachment identifier pattern: `--filter-attachment <glob>` (supports glob patterns over identifier strings)

Short mode behavior:
- Text output: Display abbreviated one-line format per element
- JSON output: Omit fields: `content`, `page_content`, `verified_relations_count`, `satisfied_relations_count`, `element_count`, `total_elements`, `global_counters`

Error handling:
- Invalid regex patterns is expected to return clear error message showing the faulty pattern and exit
- Invalid relation type names is expected to return error with list of valid relation types
- Invalid governance metadata filter values are expected to return clear error messages with the accepted values

Default output:
- Human-readable text format when neither `--json` nor `--short` is specified
- Full detail mode showing all element metadata and relations
- Full JSON output includes effective governance metadata for governance-bearing elements (`capability` and `requirement`)

#### Metadata
 * type: specification
---

### CLI Submodels Command Refinement Specification

#### Details
Submodels command behavior:
- Be invoked as `reqvire submodels`.
- Support `--from <NAME>` to scope report to one capability or requirement subtree by name.
- Support `--json` for JSON output format.
- Default to human-readable text output when `--json` is not present.
- Report independent capability-rooted submodels using capability hierarchy, `specifiedBy`, and requirement hierarchy.
- Report cross-submodel requirement couplings using explicit requirement-to-requirement relations.
- In `--from` mode for a capability, report the selected capability as the scoped capability submodel and count requirements in the selected capability subtree.
- In `--from` mode for a requirement, treat the selected requirement as a scope boundary and exclude it from reported `submodels` entries.
- Provide deterministic ordering for submodels and couplings.
- Include summary totals for submodels, requirements, and cross-submodel couplings.
- Return a clear error when `--from <NAME>` does not match any capability or requirement scope source.
- Exit with status code 0 on success and non-zero on errors.

Command output is written to stdout for easy redirection to files.

#### Metadata
 * type: specification

#### Relations
 * refine: [CLI Submodels Command](Commands.md#cli-submodels-command)
---

### CLI Traces Command Refinement Specification

#### Details
The command is expected to:
- Be invoked as `reqvire traces`
- Generate output in Markdown format with embedded Mermaid diagrams by default
- Support `--json` flag for structured JSON output without diagrams
- Show verification elements as roots with arrows following relation semantics
- Include clickable links on all nodes (verifications and requirements) in Mermaid diagrams
- Highlight directly verified capabilities or requirements using CSS classes in diagrams
- Traverse all upward parent relations to reach capability-rooted requirements
- Merge multiple verification paths into a single tree per verification
- Exit with status code 0 on success
- Exit with non-zero status code on errors

The Mermaid diagrams generated for verification traces is expected to include clickable links on diagram nodes that navigate to the referenced element using relative paths (the `traces` command always uses relative paths and does not support the `--links-with-blobs` flag).

Command output is expected to be written to stdout for easy redirection to files.

**Filter Options:**
The system is expected to support filtering verification traces by verification ID, name pattern, and verification type to allow users to generate traces for specific subsets of verifications.

The following filter options is expected to be supported:
- `--filter-id=<id>`: Generate trace for a specific verification element by its full identifier
- `--filter-name=<regex>`: Filter verifications by name using regular expression matching
- `--filter-type=<type>`: Filter by verification type (test-verification, analysis-verification, inspection-verification, demonstration-verification, formal-proof-verification)

Filters is expected to be combinable, and when multiple filters are specified, only verifications matching ALL filter criteria is expected to be included in the output.

**From-Folder Option:**
The system is expected to support a `--from-folder` option for the `traces` command that specifies the relative path from where Reqvire runs to the folder where generated output files will be saved, enabling generation of relative links in Mermaid diagrams and other outputs that are portable when the output is saved in different locations.

The `--from-folder` option is expected to:
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

### Detailed Error Handling and Logging Refinement Specification

#### Details
CLI error handling and logging behavior:
- Returns contextual error messages that help users identify command failure causes.
- Preserves actionable feedback format so remediation steps are visible near errors.
- Uses shared validation/error reporting behavior for consistent message quality across commands.
- Emits non-zero exit codes for command failures.

#### Metadata
 * type: specification

#### Relations
 * refine: [Detailed Error Handling and Logging](Commands.md#detailed-error-handling-and-logging)
---

### Format Command Refinement Specification

#### Details
`format` command is expected to:
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

### Mutating Command Hierarchy Safety Refinement Specification

#### Details
For mutating commands (`add`, `rm`, `mv`, `rename`, `merge`, `mv-file`, `link`, `unlink`, `relink`):
- Commands is expected to validate candidate state before persistence.
- Operations that would violate hierarchy invariants is expected to fail with non-zero status.
- Failed operations is expected to not partially persist model changes.
- Error output is expected to include clear hierarchy constraint context.

#### Metadata
 * type: specification

#### Relations
 * refine: [CLI Relink Command](Commands.md#cli-relink-command)
---

### Relation Commands Refinement Specification

#### Details
Relation command behavior:

The `link` command:
- Accepts syntax: `reqvire link <source> <relation-type-or-attaching> <target>`.
- Uses existing element name for source.
- Accepts relation type from `derivedFrom`, `derive`, `satisfiedBy`, `satisfy`, `verifiedBy`, `verify`, `trace`, or `attaching`.
- Supports relation targets as element name, internal file path, or external URL (`http`/`https`).
- Supports attachment targets as refinement element identifiers.
- Rejects `attaching` targets that are not refinement element identifiers.
- Rejects duplicate relation/attachment pairs with clear error.
- Supports `--dry-run` preview.
- Supports `--json` for structured output.
- Supports `--output <FILE>` when `--json` is present.

The `unlink` command:
- Accepts syntax: `reqvire unlink <source> <target>`.
- Auto-detects target in relations first, then attachments.
- Enforces single relation per source-target pair.
- Uses existing element name for source.
- Accepts element name, element identifier, or file path as target.
- Supports `--dry-run` preview.
- Supports `--json` for structured output.
- Supports `--output <FILE>` when `--json` is present.

Hierarchy ownership behavior:
- Hierarchical `link`/`unlink` edits that would produce invalid hierarchy state is expected to fail.
- Error messages is expected to guide users to atomic relink operation for hierarchy boundary rewiring.

#### Metadata
 * type: specification

#### Relations
 * refine: [Relation Commands](Commands.md#relation-commands)
---

### Validate Command Refinement Specification

#### Details
Validate command behavior:
- Executes the two-pass validation strategy.
- Pass 1 performs parsing, element collection, and local validation.
- Pass 2 builds graph state and validates relations and cross-component consistency.
- Prints all validation issues found in the model.
- Prints `No validation issues found` when model validation succeeds.
- Supports `--json` output for structured validation results.

#### Metadata
 * type: specification

#### Relations
 * refine: [Validate Command](Commands.md#validate-command)
---

### Verification Traces Element Navigation Refinement Specification

#### Details
Verification traces element navigation behavior:
- Verification element names displayed as headers are rendered as hyperlinks.
- Hyperlinks point to the verification source file with fragment identifier.
- Link format is `[Verification Name](file_path#element-fragment)`.
- Navigation allows direct jump from traces report to verification definition.

#### Metadata
 * type: specification

#### Relations
 * refine: [Verification Traces Element Navigation](Commands.md#verification-traces-element-navigation)
---
