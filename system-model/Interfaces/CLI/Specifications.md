# Elements

### CLI Collect Command Contract Specification

#### Details
The existing start-element argument accepts an exact name or canonical identifier through the shared element-selection contract. Preserve direction and supported root-family restrictions; do not reinterpret them as selectors.

The `collect` command exposes the report collection contracts as a CLI operation.

Command-specific rules:
- It must accept a start capability, requirement, ontology, semantic-query, concept-scheme, or concept context supported by the collect traversal specification.
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

### CLI Coverage Scope Selection Specification

Command-line mapping of the shared coverage scope contract.

#### Details
- `coverage --from <NAME>` selects a capability by exact name or canonical identifier under the Existing Element Selection Specification. Preserve the `--from` argument name and advertise both forms in help.
- Accept both root and nested capabilities. Requirement, contract, verification, unknown and ambiguous selections MUST fail with a clear diagnostic and a nonzero exit status; never silently fall back to the whole model.
- Omission of `--from` preserves whole-model coverage behavior and existing text and JSON report sections.
- The command MUST delegate scope resolution and coverage projection to the shared reporting operation. CLI code only parses the selector and renders the result.
- Text and `--json` output MUST report the same scope, counts, classifications, gap membership, and supporting evidence. Scoped text MUST identify the selected capability and explain that orphan diagnostics are available in whole-model coverage.
- The normal command help MUST document the optional selector and capability-only scope.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Coverage Scope Selection](Commands.md#cli-coverage-scope-selection)
---

### CLI Interface Structure Contract Specification

Supported command-line discovery and option presentation.

#### Details
- Running `reqvire` without arguments displays main help, including usage, commands, global options and flattened options for supported commands.
- The command catalog is the retained inspection, report, export, query-artifact, server launch and migration surface from the interface boundary referenced by the owning requirement. Include `validate`, `lint`, `coverage`, `search`, `collect`, `submodels`, `model`, `traces`, `containment`, `resources`, `change-impact`, `export`, `semantic`, `concepts`, `ontologies`, `migrate`, `mcp`, `serve` and help/version controls.
- Expand supported `semantic query` list/validate/export/check and concept validation/export discovery. Preserve applicable workspace, filter, output and server startup options.
- Do not list CRUD authoring, formatting, their preview forms or lint repair as CLI operations. Reject removed invocations during argument handling with guidance naming MCP and its mutation enablement. The migration apply flag remains available only on `migrate`.
- Command-specific headings refer only to supported commands; help must not retain formatting or authoring option sections.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Interface Structure](Commands.md#cli-interface-structure)
---

### CLI JSON File Output Option Contract Specification

#### Details
This contract governs JSON report file output. Native artifact exporters follow their own serialization and file-publication contracts, including raw SPARQL query export, semantic/concept serialization, and static Explorer export.

JSON report `--output` option behavior:
- Be available on commands that emit JSON reports.
- For report commands with a selectable `--json` mode, require `--json` to also be set and report an error when `--output` is used without JSON output selection.
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

Read-only command-line model hygiene reporting.

#### Details
- Invoke as `reqvire lint`; report all categories by default without writes.
- Preserve `--fixable` and `--auditable` inspection filters, text/JSON output and supported output-path controls.
- Reject `--fix` during argument parsing, before model loading, with guidance to use `reqvire.lint` with `fix: true` on a mutation-enabled MCP server.
- Categorize safe auto-fixable findings and findings requiring manual review through the shared lint report. Auto-fix guidance names MCP repair, never CLI repair.
- Preserve existing report exit/error behavior and stdout/file output contracts. No fixes, formatting, commits or source rewrites occur through this command.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Lint Command](Commands.md#cli-lint-command)
---

### CLI Managed Query Artifacts Specification

The CLI interface MUST expose managed query operations through shared core contracts.

#### Details
Invoke the shared model-load gate referenced by the owning requirement for model-dependent query operations, preserving each command's diagnostic mode and the core validator's per-candidate diagnostics and repair guidance. Artifact publication requires successful shared validation; diagnostic responses do not mark malformed candidates as valid models.
Existing `name` source selectors accept exact native element names or canonical identifiers under the bound element-selection contract. Explicit IRI selectors, namespace filters and artifact paths retain their existing domains and mutual exclusion.

Query discovery MUST return native authored records sorted by generated IRI then name. Namespace filters MUST match used ontology namespaces. Query validation MUST return per-candidate diagnostics and use the common validation gate. Selectors MUST resolve exactly and reject unknown or ambiguous results. Exported content and hashes MUST come from the shared core renderer. Validation and artifact rendering MUST preserve downstream SERVICE, datasets, and extension functions without executing them.
Commands MUST be `semantic query list [--json] [--namespace-base IRI] [--name NAME] [--iri IRI]`, `semantic query validate [--json] [--name NAME|--iri IRI]`, `semantic query export (--name NAME|--iri IRI) [--output FILE|--json]`, and `semantic query check (--name NAME|--iri IRI) --artifact FILE [--json]`. Export MUST write raw SPARQL by default and atomically replace output files only after successful validation. Check MUST preserve the file, return failure on missing or stale content, and distinguish I/O errors.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Managed Query Artifacts](ManagedQueries.md#cli-managed-query-artifacts)
---

### CLI Migrate Command Contract Specification

#### Details
Migrate command behavior:
- Be invoked as `reqvire migrate`.
- Default to dry-run mode and report a diff preview without writing files.
- Support `--fix` to apply deterministic migrations to source files.
- Support `--json` for structured migration summary and diff output.
- Support `--output <FILE>` only with `--json`.
- Retain migration preview and application in CLI as the explicit source-upgrade exception. They must remain usable when known legacy syntax fails normal validation and therefore prevents MCP initialization; no MCP migration endpoint or validation-bypass server is provided. Migration does not expose generic CRUD or formatting.
- Parse the model in lenient mode so known migration-triggering validation errors can be repaired.
- After startup workspace selection has entered the effective workspace root, apply source rewrites to model files identified by workspace-root-relative registry paths without duplicating any caller launch-directory prefix in the write location.
- Implement the `v0.15-documents-to-element-header` migration by rewriting legacy single-element file headers from `# Documents` to `# Element`.
- Implement the `v0.16-verification-objective` migration by creating one shared holder `verification-objective` element in the repository-root `VerificationObjectiveMigration.md` file and adding holder-owned `derive` relations from that objective to standalone concrete verification elements.
- Implement the `v1.2-concept-reference-links` migration by rewriting legacy `#### Concept References` entries from `* Label: IRI` syntax to `* [Label](concept-element-link)` syntax only when the IRI resolves to exactly one generated native concept element.
- Do not keep runtime parser fallback for legacy concept-reference syntax; migration is the compatibility path.
- Include concept-reference migration counts and diffs in dry-run, applied, and JSON migration output.
- Preserve concrete verification `verify` and evidence relations; the shared holder objective is an explicit migration placeholder that users can later rename, split, merge, or regroup.
- After migration, run normal validation before starting MCP. Unsupported invalid content still requires external source repair; migration only performs its known deterministic rewrites.
- Exit with code 0 when no migration is needed, when a dry-run preview is produced, or when fixes are applied successfully.
- Exit with non-zero status code on unsupported or unsafe migration errors.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Migrate Command](Commands.md#cli-migrate-command)
---

### CLI Model Command Contract Specification

#### Details
The optional `from` start selector accepts an exact name or canonical identifier under the bound Existing Element Selection Specification. Omission, reverse traversal and type filters retain their existing meanings.

The `model` command behavior is governed by the reused model JSON output contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Model Command](Commands.md#cli-model-command)
---

### CLI Ontologies Command Contract Specification

#### Details
The semantic export command family is governed by the reused ontology collection and semantic export contracts. Canonical users should use `semantic export` and select the narrowest required layer with repeatable `--layer` flags.

Command-specific rules:
- `semantic export --layer ontologies` emits generated ontology document declarations plus authored OWL/RDF ontology vocabulary.
- `semantic export --layer queries` MUST emit managed query RDF resources selected by used ontology namespace.
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

### CLI Resources Command Contract Specification

#### Details
The `resources` command behavior is governed by the reused local artifact inventory contract, including its retained JSON schema.
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
- It MUST delegate file, element, type, governance, relation, Contract Bindings, Contract References, short/full, and content filtering to the report search contracts.
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
The existing `from` argument accepts an exact capability or requirement name or canonical identifier under the bound element-selection contract, with unchanged selected-boundary semantics.

The `submodels` command behavior is governed by the reused submodel analysis and output contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Submodels Command](Commands.md#cli-submodels-command)
---

### CLI Traces Command Contract Specification

#### Details
The `traces` command MUST obtain its report and filter diagnostics from the shared trace operation according to the referenced trace-construction and type-validation contracts. These dependencies describe consumed core behavior. JSON presentation is allocated to the CLI Traces Result Presentation child requirement.

#### Metadata
  * type: specification
---

### CLI Traces Result Presentation Specification

#### Details
The CLI MUST serialize the returned report according to the bound JSON Output Structure. With no `--output` argument, the JSON MUST be written to stdout. With `--output <FILE>`, the JSON MUST be written to that file. The file's parsed content MUST equal the report emitted on stdout for the same invocation and model.

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
- `file_path`: Workspace-root-relative path
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
- Workspace selection is applied as a startup directory change before Git worktree discovery, model parsing, ignore-pattern loading, Git metadata collection, reporting, and mutation execution.
- Workspace selection preserves existing current-directory behavior when the option is omitted.
- Workspace selection applies consistently to normal CLI commands and MCP server startup.
- Invalid workspace directories are rejected before command execution.

#### Metadata
  * type: specification

#### Relations
  * define: [Explicit Workspace Selection](Commands.md#explicit-workspace-selection)
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

### npm CLI Launch Specification

#### Details
- Select the package's native archive for Linux x64, macOS arm64, or macOS x64; report unsupported platforms and missing archives as failures.
- Reuse the native executable cache by package version and platform/architecture. Concurrent first invocations must extract privately and publish only a complete executable atomically. A losing publisher reuses the complete winner without removing or overwriting another invocation's cache or extraction state.
- Remove only the current invocation's temporary extraction directory on success or extraction failure. Failed extraction must not publish a partial executable, and a later invocation must be able to retry.
- Forward arguments, inherited standard input/output/error, native exit status, and native termination signal. Repeated launches with an existing complete executable must not extract again.

#### Metadata
  * type: specification

#### Relations
  * define: [npm CLI Distribution](Commands.md#npm-cli-distribution)
---
