# Elements

### CLI Add Element Command Refinement Specification

#### Details
The `add` command behavior is governed by the attached create-element workflow, override behavior, and ontology-aware mutation contracts.

#### Metadata
  * type: specification
---

### CLI Collect Command Refinement Specification

#### Details
The `collect` command behavior is governed by the attached collection traversal and output contracts.

#### Metadata
  * type: specification
---

### CLI Coverage Command Refinement Specification

#### Details
The `coverage` command behavior is governed by the attached coverage report contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Coverage Command](Commands.md#cli-coverage-command)
---

### CLI Interface Structure Refinement Specification

#### Details
The CLI must display all commands and options and command's options flattened in the main help output which must also be a default command:
Reqvire requirements & traceability management tool

Usage: reqvire [OPTIONS] <COMMAND> [COMMAND OPTIONS]

Commands:
serve Serve the embedded Explorer UI via HTTP server
export Export the Explorer SPA as a static site to a directory
mcp Start Reqvire MCP server
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
relink Replace an existing relation target with a new target in one operation
mv-asset Move/rename asset file and update references
rm-asset Remove asset file and remove references
containment Generate containment view
resources Generate resources report
ontologies Collect ontology elements and semantic-contract SHACL shapes
submodels Analyze independent capability-rooted submodels and cross-submodel couplings
collect Collect content from capability, requirement, or ontology context
help Print help for commands

Ontologies options:
--jsonld Output JSON-LD format instead of RDF/Turtle (.ttl)
--full Include Reqvire model context triples and ontology projection facts in the semantic export
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
  * define: [CLI Interface Structure](Commands.md#cli-interface-structure)
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
  * define: [CLI JSON File Output Option](Commands.md#cli-json-file-output-option)
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
  * define: [CLI Lint Command](Commands.md#cli-lint-command)
---

### CLI Merge Element Command Refinement Specification

#### Details
The `merge` command behavior is governed by the attached merge content, compatibility, and workflow contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Merge Element Command](Commands.md#cli-merge-element-command)
---

### CLI Migrate Command Refinement Specification

#### Details
Migrate command behavior:
- Be invoked as `reqvire migrate`.
- Default to dry-run mode and report a diff preview without writing files.
- Support `--fix` to apply deterministic migrations to source files.
- Support `--json` for structured migration summary and diff output.
- Support `--output <FILE>` only with `--json`.
- Parse the model in lenient mode so known migration-triggering validation errors can be repaired.
- Implement the `v0.15-documents-to-element-header` migration by rewriting legacy single-element file headers from `# Documents` to `# Element`.
- Implement the `v0.16-verification-objective` migration by creating one shared holder `verification-objective` element in the repository-root `VerificationObjectiveMigration.md` file and adding holder-owned `derive` relations from that objective to standalone concrete verification elements.
- Preserve concrete verification `verify` and evidence relations; the shared holder objective is an explicit migration placeholder that users can later rename, split, merge, or regroup.
- Exit with code 0 when no migration is needed, when a dry-run preview is produced, or when fixes are applied successfully.
- Exit with non-zero status code on unsupported or unsafe migration errors.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Migrate Command](Commands.md#cli-migrate-command)
---

### CLI Model Diagram Command Refinement Specification

#### Details
The `model` command behavior is governed by the attached model-diagram output contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Model Diagram Command](Commands.md#cli-model-diagram-command)
---

### CLI Move Asset Command Refinement Specification

#### Details
The `mv-asset` command behavior is governed by the attached asset-move and reference-update contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Move Asset Command](Commands.md#cli-move-asset-command)
---

### CLI Move Element Command Refinement Specification

#### Details
The `mv` command behavior is governed by the attached move workflow and target-location contracts.

#### Metadata
  * type: specification
---

### CLI Move File Command Refinement Specification

#### Details
The `mv-file` command behavior is governed by the attached file-move workflow and file-format contracts.

#### Metadata
  * type: specification
---

### CLI Ontologies Command Refinement Specification

#### Details
The `ontologies` command behavior is governed by the attached ontology collection and semantic export contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Ontologies Command](Commands.md#cli-ontologies-command)
---

### CLI Relink Command Refinement Specification

#### Details
The `relink` command behavior is governed by the attached relink workflow and atomic validity contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Relink Command](Commands.md#cli-relink-command)
---

### CLI Remove Asset Command Refinement Specification

#### Details
The `rm-asset` command behavior is governed by the attached asset-move/remove and output contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Remove Asset Command](Commands.md#cli-remove-asset-command)
---

### CLI Remove Element Command Refinement Specification

#### Details
The `rm` command behavior is governed by the attached delete workflow and relation-cleanup contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Remove Element Command](Commands.md#cli-remove-element-command)
---

### CLI Rename Element Command Refinement Specification

#### Details
The `rename` command behavior is governed by the attached rename workflow and relation-update contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Rename Element Command](Commands.md#cli-rename-element-command)
---

### CLI Resources Command Refinement Specification

#### Details
The `resources` command behavior is governed by the attached relation and attachment inventory contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Resources Command](Commands.md#cli-resources-command)
---

### CLI Search Command Refinement Specification

#### Details
The `search` command behavior is governed by the attached search/filter/output contracts.

#### Metadata
  * type: specification
---

### CLI Size Estimate JSON Option Specification

The CLI `--with-size-estimates` option is expected to be an opt-in JSON evidence option.

#### Details
The `--with-size-estimates` option behavior is governed by the attached report-evidence contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Size Estimate JSON Option](Commands.md#cli-size-estimate-json-option)
---

### CLI Submodels Command Refinement Specification

#### Details
The `submodels` command behavior is governed by the attached submodel analysis and output contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Submodels Command](Commands.md#cli-submodels-command)
---

### CLI Traces Command Refinement Specification

#### Details
The `traces` command behavior is governed by the attached verification trace and link-format contracts.

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
  * define: [Detailed Error Handling and Logging](Commands.md#detailed-error-handling-and-logging)
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
  * define: [CLI Diff Output](Commands.md#cli-diff-output)
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
  * define: [Detailed Error Handling and Logging](Commands.md#detailed-error-handling-and-logging)
---

### Explicit Workspace Selection Specification

The CLI is expected to resolve and enter an explicitly selected workspace before executing Reqvire operations.

#### Details
The `--workspace` behavior is governed by the attached workspace-selection and startup contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [Explicit Workspace Selection](Commands.md#explicit-workspace-selection)
---

### Format Command Refinement Specification

#### Details
The `format` command behavior is governed by the attached formatting behavior and output contracts.

#### Metadata
  * type: specification
---

### Mutating Command Hierarchy Safety Refinement Specification

#### Details
Mutating command hierarchy safety is governed by the attached validation and atomicity contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Relink Command](Commands.md#cli-relink-command)
---

### Relation Commands Refinement Specification

#### Details
Relation command behavior is governed by the attached relation, attachment, and atomicity contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [Relation Commands](Commands.md#relation-commands)
---

### Validate Command Refinement Specification

#### Details
The `validate` command behavior is governed by the attached validation strategy and output contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [Validate Command](Commands.md#validate-command)
---

### Verification Traces Element Navigation Refinement Specification

#### Details
Verification trace element navigation behavior is governed by the attached verification-trace link-format contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [Verification Traces Element Navigation](Commands.md#verification-traces-element-navigation)
---
