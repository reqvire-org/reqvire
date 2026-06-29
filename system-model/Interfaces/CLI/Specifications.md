# Elements

### CLI Add Element Command Contract Specification

#### Details
The `add` command accepts element content from stdin or `--content`, validates the target location, applies create-element workflow semantics, and returns the shared mutation result shape.

Command-specific rules:
- It must invoke the shared create-element workflow for parsing, relation validation, ordering, persistence, dry-run diff output, and JSON output.
- Override mode must reuse Create Element Override Behavior, including ontology-aware rebasing when the replaced element is an ontology element.
- It must not implement a separate relation or ontology mutation path outside the shared model-operation contracts.

#### Metadata
  * type: specification
---

### CLI Collect Command Contract Specification

#### Details
The `collect` command exposes the report collection contracts as a CLI operation.

Command-specific rules:
- It must accept a start capability, requirement, or ontology context supported by the collect traversal specification.
- It must delegate traversal, contract_bindings inclusion, source citation, and output payload shape to Collect Content Specification and Collect Output Format Specification.
- It must not define command-local traversal rules.

#### Metadata
  * type: specification
---

### CLI Coverage Command Contract Specification

#### Details
The `coverage` command exposes verification coverage and requirement implementation coverage reports.

Command-specific rules:
- It must select the shared coverage report engines rather than computing coverage in the CLI layer.
- It must preserve the shared JSON output contract when machine-readable output is requested.
- It must keep verification type selection, implementation coverage evidence classification, and text formatting in the reused coverage contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Coverage Command](Commands.md#cli-coverage-command)
---

### CLI Interface Structure Contract Specification

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
link Add relation or contract_bindings between elements
unlink Remove relation or contract_bindings (auto-detects)
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
--layer model Include Reqvire model facts and ontology projection facts in the semantic export
--layer external-used Include the used external ontology subset
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

### CLI JSON File Output Option Contract Specification

#### Details
`--output` option behavior:
- Be available on commands that emit JSON output.
- For commands with a selectable `--json` mode, require `--json` to also be set and report an error when `--output` is used without JSON output selection.
- For JSON-only commands such as `model`, `containment`, `resources`, and `traces`, accept `--output` directly because JSON is the only output mode.
- Write JSON content to the specified file path.
- Create the file if it does not exist, and overwrite it if it does.
- Print a confirmation message to stdout: `✅ Output saved to <filepath>`.
- Exit with code 0 on success and non-zero on file write error.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI JSON File Output Option](Commands.md#cli-json-file-output-option)
---

### CLI Lint Command Contract Specification

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

### CLI Merge Element Command Contract Specification

#### Details
The `merge` command exposes the shared merge-element workflow through the CLI.

Command-specific rules:
- It must accept one target element and one or more source elements.
- It must delegate type compatibility, content transformation, relation rewrites, source deletion, empty-file cleanup, and ontology merge behavior to the merge content, compatibility, and workflow contracts.
- It must preserve JSON output behavior through the shared JSON output contract.
- It must not implement command-local merge semantics outside the graph-backed model-operation contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Merge Element Command](Commands.md#cli-merge-element-command)
---

### CLI Migrate Command Contract Specification

#### Details
Migrate command behavior:
- Be invoked as `reqvire migrate`.
- Default to dry-run mode and report a diff preview without writing files.
- Support `--fix` to apply deterministic migrations to source files.
- Support `--json` for structured migration summary and diff output.
- Support `--output <FILE>` only with `--json`.
- Parse the model in lenient mode so known migration-triggering validation errors can be repaired.
- When launched from a repository subdirectory, apply source rewrites to the model files identified by git-root-relative registry paths without duplicating the subdirectory prefix in the write location.
- Implement the `v0.15-documents-to-element-header` migration by rewriting legacy single-element file headers from `# Documents` to `# Element`.
- Implement the `v0.16-verification-objective` migration by creating one shared holder `verification-objective` element in the repository-root `VerificationObjectiveMigration.md` file and adding holder-owned `derive` relations from that objective to standalone concrete verification elements.
- Implement the `v1.2-concept-reference-links` migration by rewriting legacy `#### Concept References` entries from `* Label: IRI` syntax to `* [Label](concept-element-link)` syntax only when the IRI resolves to exactly one generated native concept element.
- Do not keep runtime parser fallback for legacy concept-reference syntax; migration is the compatibility path.
- Include concept-reference migration counts and diffs in dry-run, applied, and JSON migration output.
- Preserve concrete verification `verify` and evidence relations; the shared holder objective is an explicit migration placeholder that users can later rename, split, merge, or regroup.
- Exit with code 0 when no migration is needed, when a dry-run preview is produced, or when fixes are applied successfully.
- Exit with non-zero status code on unsupported or unsafe migration errors.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Migrate Command](Commands.md#cli-migrate-command)
---

### CLI Model Command Contract Specification

#### Details
The `model` command behavior is governed by the reused model JSON output contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Model Command](Commands.md#cli-model-command)
---

### CLI Move Asset Command Contract Specification

#### Details
The `mv-asset` command moves or renames internal-path assets and updates model references through shared mutation contracts.

Command-specific rules:
- It must resolve asset paths relative to the selected workspace/git root.
- It must update all InternalPath references that point to the moved asset.
- It must support dry-run diff, JSON mutation output, and file persistence behavior through shared contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Move Asset Command](Commands.md#cli-move-asset-command)
---

### CLI Move Element Command Contract Specification

#### Details
The `mv` command behavior is governed by the reused move workflow and target-location contracts.

#### Metadata
  * type: specification
---

### CLI Move File Command Contract Specification

#### Details
The `mv-file` command exposes the shared file-move operation through the CLI.

Command-specific rules:
- It must accept source and target paths relative to the selected workspace/git root.
- It must support dry-run diff and JSON mutation output through shared output contracts.
- It must delegate relation reference updates, squash behavior, target validation, and `# Element` rejection rules to Move File Operation Contract Specification and target-location constraints.

#### Metadata
  * type: specification
---

### CLI Ontologies Command Contract Specification

#### Details
The semantic export command family is governed by the reused ontology collection and semantic export contracts. Canonical users should use `semantic export` and select the narrowest required layer with repeatable `--layer` flags.

Command-specific rules:
- `semantic export --layer ontologies` emits generated ontology document declarations plus authored OWL/RDF ontology vocabulary.
- `semantic export --layer shapes` emits semantic-contract SHACL shapes.
- `semantic export --layer concepts` emits SKOS concept scheme/thesaurus triples.
- `semantic export --layer model` emits Reqvire model facts, relation-family projection facts, ontology term declarations, semantic-contract shape references, and generated ontology projection facts.
- `semantic export --layer external-used` emits only the used external ontology subset.
- `semantic export --layer prefixes` emits generated `reqvire:TurtlePrefixDeclaration` projection facts.
- Omitting `--layer` exports all public semantic layers.
- Turtle output follows the shared prefixed Turtle serializer contract; JSON-LD output remains a separate RDF serialization mode selected by `--jsonld` when supported by the selected layer.
- `--namespace-base <IRI>` applies only to clean authored exports and must be rejected with the `model` layer.
- Authored `reqvire:mapsToConcept` bridge triples are part of the ontology layer, not a separate mapping layer.
- Canonical native thesaurus workflows may use `concepts export` and `concepts validate` for standalone concept-scheme work.
- `concepts export` emits generated SKOS concept scheme/thesaurus triples, with optional `--include-mappings` for valid `reqvire:mapsToConcept` bridge triples.
- `concepts validate` validates standalone concept schemes, concept references, and `reqvire:mapsToConcept` bridge targets through the normal model validation path.
- No CLI flag or mode may emit complete third-party ontology source dumps.
- `--output <FILE>` writes the selected format to a file.
- The command must reuse the semantic index built from the graph registry instead of reparsing Turtle separately from validation.
- The legacy `ontologies` command remains a compatibility alias for combined graph export.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Ontologies Command](Commands.md#cli-ontologies-command)
---

### CLI Relink Command Contract Specification

#### Details
The `relink` command exposes atomic relation target replacement through the CLI.

Command-specific rules:
- It must accept a source element, relation name, old target, and new target.
- It must delegate candidate rewiring, hierarchical boundary semantics, validation-before-persist, rollback, dry-run diff, and JSON output to the atomic relink workflow and validity contracts.
- It must not implement command-local relation rewriting outside the shared graph-backed mutation path.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Relink Command](Commands.md#cli-relink-command)
---

### CLI Remove Asset Command Contract Specification

#### Details
The `rm-asset` command removes internal-path assets and removes model references through shared mutation contracts.

Command-specific rules:
- It must resolve the asset path relative to the selected workspace/git root.
- It must remove all InternalPath references that point to the removed asset.
- It must support dry-run diff, JSON mutation output, and file persistence behavior through shared contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Remove Asset Command](Commands.md#cli-remove-asset-command)
---

### CLI Remove Element Command Contract Specification

#### Details
The `rm` command exposes the shared delete-element workflow.

Command-specific rules:
- It must delete existing model elements through the shared workflow, including orphan-child prevention, semantic-contract mutation validation, relation cleanup, empty-file cleanup, dry-run diff, and JSON mutation output.
- It must not implement command-local deletion semantics outside the graph-backed model-operation contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Remove Element Command](Commands.md#cli-remove-element-command)
---

### CLI Rename Element Command Contract Specification

#### Details
The `rename` command behavior is governed by the reused rename workflow and relation-update contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Rename Element Command](Commands.md#cli-rename-element-command)
---

### CLI Resources Command Contract Specification

#### Details
The `resources` command behavior is governed by the reused relation and contract_bindings inventory contracts.
The command emits JSON by default and does not expose a separate output-format flag.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Resources Command](Commands.md#cli-resources-command)
---

### CLI Search Command Contract Specification

#### Details
The `search` command exposes model search, filtering, and evidence serialization.

Command-specific rules:
- It must delegate file, element, type, governance, relation, contract_bindings, short/full, and content filtering to the report search contracts.
- Full JSON results must expose parsed semantic ADT fields for ontology and semantic-contract elements when present.
- It must not define a separate CLI-only search schema outside the shared JSON output and search-filtering contracts.

#### Metadata
  * type: specification
---

### CLI Size Estimate JSON Option Specification

The CLI `--with-size-estimates` option is expected to be an opt-in JSON evidence option.

#### Details
The `--with-size-estimates` option behavior is governed by the reused report-evidence contracts.

Option rules:
- The option enables model building with element size estimates for the command invocation.
- The option is valid only for commands that emit JSON model evidence.
- JSON-only commands such as `model` may accept the option directly because JSON is the canonical output.
- Commands with human-readable modes must require JSON output selection before exposing size-estimate fields.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Size Estimate JSON Option](Commands.md#cli-size-estimate-json-option)
---

### CLI Submodels Command Contract Specification

#### Details
The `submodels` command behavior is governed by the reused submodel analysis and output contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Submodels Command](Commands.md#cli-submodels-command)
---

### CLI Traces Command Contract Specification

#### Details
The `traces` command behavior is governed by the reused verification trace and link-format contracts.

#### Metadata
  * type: specification
---

### Detailed Error Handling and Logging Contract Specification

#### Details
CLI error handling and logging behavior:
- Returns contextual error messages that help users identify command failure causes.
- Preserves actionable feedback format so remediation steps are visible near errors.
- Uses shared validation/error reporting behavior for consistent message quality across commands.
- Configures logging without mutating the process environment at runtime.
- Defaults log filtering to `error` when `RUST_LOG` is unset so structured stdout output is not polluted by logs.
- Honors `RUST_LOG` when users explicitly request more verbose diagnostics.
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
The `--workspace` behavior is governed by the reused workspace-selection and startup contracts.

Option rules:
- The CLI provides a global workspace selection option.
- Workspace selection applies before model parsing, ignore-pattern loading, git root discovery, reporting, and mutation execution.
- Workspace selection preserves existing current-directory behavior when the option is omitted.
- Workspace selection applies consistently to normal CLI commands and MCP server startup.
- Invalid workspace directories are rejected before command execution.

#### Metadata
  * type: specification

#### Relations
  * define: [Explicit Workspace Selection](Commands.md#explicit-workspace-selection)
---

### Format Command Contract Specification

#### Details
The `format` command behavior is governed by the reused formatting behavior and output contracts.

#### Metadata
  * type: specification
---

### Mutating Command Hierarchy Safety Contract Specification

#### Details
Mutating command hierarchy safety is governed by the reused validation and atomicity contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Relink Command](Commands.md#cli-relink-command)
---

### Relation Commands Contract Specification

#### Details
Relation command behavior is governed by the reused relation, contract_bindings, and atomicity contracts.

Command-specific rules:
- `reqvire link <element-name> <relation> <target>` adds an authored relation unless the relation keyword is the contract binding keyword.
- `reqvire link <element-name> bindContract <target>` adds a Contract Bindings entry to a reusable requirement-owned contract target and creates the subsection when needed.
- `reqvire unlink <element-name> <target>` auto-detects whether the target is an authored relation target or a Contract Bindings target and removes the matching entry.
- Contract Bindings removal removes the subsection when no entries remain.
- The commands must preserve dry-run preview, JSON mutation output, file persistence, relation validation, contract_bindings scope validation, idempotency, and atomic failure behavior from the reused contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [Relation Commands](Commands.md#relation-commands)
---

### Validate Command Contract Specification

#### Details
The `validate` command exposes the shared model validation strategy as an explicit CLI operation.

Command-specific rules:
- It must run the same two-pass validation and semantic-contract checks used by model-dependent commands.
- It must report validation diagnostics through the shared validation error reporting and JSON output contracts.
- It must not mutate the model or filesystem.
- It must exit non-zero when validation fails.

#### Metadata
  * type: specification

#### Relations
  * define: [Validate Command](Commands.md#validate-command)
---
