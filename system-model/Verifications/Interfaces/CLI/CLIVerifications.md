# Elements

### CLI Interface Verification Objective

This objective groups verification of retained command-line inspection, reports, exports, query artifacts, workspace controls and server launch, plus deterministic migration before normal validation. It includes the breaking removal of CLI authoring, formatting and lint repair.

#### Metadata
  * type: verification-objective

#### Relations
  * derive: [CLI Git Commit Hash Flag Test](#cli-git-commit-hash-flag-test)
  * derive: [CLI Help Structure Verification](#cli-help-structure-verification)
  * derive: [CLI Migrate Command Verification](#cli-migrate-command-verification)
  * derive: [CLI Ontologies Command Verification](#cli-ontologies-command-verification)
  * derive: [CLI Size Estimate JSON Option Verification](#cli-size-estimate-json-option-verification)
  * derive: [Explicit Workspace Selection Verification](#explicit-workspace-selection-verification)
---

### CLI Git Commit Hash Flag Test

This test verifies that the system properly handles the source-control commit reference flag for change impact analysis.

#### Details

##### Acceptance Criteria
- System shall support --git-commit flag for change impact analysis
- System shall use the specified commit reference to materialize the base workspace snapshot in the current single-worktree comparison mode
- System shall default to the current eligible worktree HEAD when the flag is not specified and Git metadata is available
- System shall handle relative commit references (HEAD~1, etc.) without changing the workspace-root-relative identifier namespace

##### Test Criteria
- Command with explicit --git-commit flag runs successfully
- Command without flag defaults to the current eligible worktree HEAD when Git metadata is available
- Relative commit references are correctly resolved
- Invalid commit references are reported appropriately
- Change impact analysis correctly uses the specified commit as the base snapshot while reporting workspace-root-relative paths

##### Test Procedure
1. Create test fixtures with a workspace containing an eligible Git worktree with multiple commits
2. Run `reqvire change-impact --git-commit=HEAD~1`
3. Verify that the specified commit is used as baseline
4. Run `reqvire change-impact` without a git-commit flag
5. Verify that the current eligible worktree HEAD is used as the default baseline
6. Run with invalid commit reference and verify appropriate error

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-change-impact-detection/test.sh)
  * verify: [CLI Change Impact Report Command](../../../Interfaces/CLI/Commands.md#cli-change-impact-report-command)
---

### CLI Help Structure Verification

Verify main help advertises the supported CLI surface and its applicable options.

#### Details
- Compare fixture-based main help output with the expected retained catalog: validation, lint inspection, coverage, search/collect/submodels, model/traces/containment/resources/change-impact, static and semantic/concept exports, managed query list/validate/export/check, ontologies alias, migrate, server launch and help/version.
- Require no authoring/format command entries or option headings, and no `--fix` in lint options. Migration options retain `--fix`.
- Running without arguments exits successfully and displays usage, global controls and flattened supported command options. Nested semantic/concept commands and filters/output controls remain discoverable.
- The interface-boundary verification covers argument rejection and source preservation. These revised expected outputs are acceptance criteria for the subsequent implementation/test update.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-cli-help-structure/test.sh)
  * verify: [CLI Change Impact Report Command](../../../Interfaces/CLI/Commands.md#cli-change-impact-report-command)
  * verify: [CLI Coverage Command](../../../Interfaces/CLI/Commands.md#cli-coverage-command)
  * verify: [CLI JSON File Output Option](../../../Interfaces/CLI/Commands.md#cli-json-file-output-option)
  * verify: [CLI Ontologies Command](../../../Interfaces/CLI/Commands.md#cli-ontologies-command)
  * verify: [CLI Search Command](../../../Interfaces/CLI/Commands.md#cli-search-command)
  * verify: [CLI Traces Command](../../../Interfaces/CLI/Commands.md#cli-traces-command)
  * verify: [Validate Command](../../../Interfaces/CLI/Commands.md#validate-command)
  * verify: [Served Explorer Browser Interface](../../../Interfaces/WebExplorer/Capabilities.md#served-explorer-browser-interface)
---

### CLI Migrate Command Verification

This verification shall prove that the migrate command previews and applies deterministic source migrations without writing outside the intended model file locations.

#### Details
Expected checks:
- A known legacy fixture fails normal validation and MCP startup. CLI migration preview still succeeds without changing source bytes; application matches the expected migrated fixture. Normal validation and read-only MCP startup succeed after the supported migration. No MCP migration or pre-validation maintenance tool is advertised.
- Compare reports and complete source trees against fixtures with unified diffs. These startup and interface-boundary checks extend the existing migration evidence in the subsequent implementation stage.
- Run `reqvire validate` on a fixture with legacy contract relations and verify migration candidates are reported.
- Run `reqvire migrate` and verify the dry-run preview reports legacy contract relation rewrites without changing source files.
- Run `reqvire migrate --fix` and verify legacy `refinedBy` and `refine` relations are rewritten to `definedBy` and `define`.
- Run `reqvire migrate --fix` from an effective workspace root below a parent repository and verify changed files are written to the workspace-root-relative source path rather than a duplicated caller launch-directory path.
- Run `reqvire validate` after migration and verify the migrated model passes.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-contract-relation-migration/test.sh)
  * verify: [CLI Migrate Command](../../../Interfaces/CLI/Commands.md#cli-migrate-command)
---

### CLI Ontologies Command Verification

This verification shall prove that semantic export commands expose ontology, shapes, concepts, and combined graph layers through stable CLI options and output formats.

#### Details
Expected checks:
- Run `reqvire semantic export --layer ontologies` and verify the command succeeds and emits authored ontology vocabulary without semantic-contract SHACL shapes.
- Verify Turtle exports include one deterministic top-level `@prefix` declaration block, use compact prefixed names for built-in and authored namespaces where safe, preserve multiple authored `owl:Ontology` document subjects and `owl:imports` facts, and remain parseable as RDF/Turtle.
- Run `reqvire semantic export --layer shapes` and verify the command succeeds and emits semantic-contract SHACL shapes without authored ontology classes.
- Run `reqvire semantic export --layer concepts` and verify the command succeeds and emits SKOS concepts without authored ontology bridge triples.
- Run `reqvire semantic export --layer ontologies` and verify authored `reqvire:mapsToConcept` bridge triples are present when ontology terms map to generated native concepts.
- Run `reqvire semantic export --layer model` and verify generated Reqvire element, relation, contract binding, concept reference, semantic term context, and ontology projection facts are present without Turtle prefix projection facts.
- Run `reqvire semantic export --layer prefixes` and verify generated `reqvire:TurtlePrefixDeclaration` projection facts are present.
- Run `reqvire concepts export --include-mappings` and verify the root Concepts command emits the same standalone Thesaurus SKOS layer plus optional valid bridge triples.
- Run `reqvire concepts validate` and verify standalone concept-scheme namespace ownership, concept references, and `reqvire:mapsToConcept` targets validate through the canonical model validation path.
- Run `reqvire semantic export` and verify the default export includes all public layers with model facts, generated ontology projection facts, prefix projection facts, used external subset materialization, and `owl:NamedIndividual` typing for concrete parsed model elements.
- Run `reqvire semantic export --layer external-used` and verify only the used external subset is exposed, not raw full third-party ontology source graphs.
- Run JSON-LD variants and verify emitted JSON-LD is valid for supported layers.
- Verify JSON-LD variants remain JSON-LD RDF serializations and do not emit Turtle `@prefix` syntax.
- Verify the legacy `reqvire ontologies` command remains a compatibility alias for combined semantic graph export.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-ontologies-command/test.sh)
  * verify: [CLI Ontologies Command](../../../Interfaces/CLI/Commands.md#cli-ontologies-command)
---

### CLI Scoped Coverage Verification

This verification checks the coverage scope command interface against the shared reporting contract.

#### Details
Expected checks:
- Run `coverage` without a selector and compare existing whole-model text and JSON output expectations.
- Run `coverage --from <NAME>` and `coverage --from <NAME> --json` for a root, a nested capability, and an empty capability. Compare scoped subjects, classifications, evidence, counts, and gaps with the shared report result.
- Check that selected-capability metadata and the whole-model-only orphan explanation are present in their respective output formats.
- Select unknown, requirement, contract, and verification names and assert a diagnostic plus nonzero exit status without whole-model fallback.
- Confirm help documents the optional capability selector, and that quoted capability names containing spaces resolve correctly.

The appended element-selection assertions require name/identifier report equality, ambiguity and known wrong-type diagnostics, and help advertising both selector forms. URL navigation is a separate browser verification.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [CLI Interface Verification Objective](#cli-interface-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-scoped-coverage/test.sh)
  * verify: [CLI Coverage Scope Selection](../../../Interfaces/CLI/Commands.md#cli-coverage-scope-selection)
---

### CLI Size Estimate JSON Option Verification

This verification shall prove that the CLI size-estimate option is JSON-only and enables element size estimates for supported commands.

#### Details
Expected checks:
- Run `reqvire model --with-size-estimates` and verify element payloads include `size_estimate`.
- Run `reqvire model` without `--with-size-estimates` and verify element payloads omit `size_estimate`.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-model-command/test.sh)
  * verify: [CLI Size Estimate JSON Option](../../../Interfaces/CLI/Commands.md#cli-size-estimate-json-option)
---

### CLI Traces Result Presentation Verification

Verify that trace-report presentation fulfills the CLI output obligation independently of the referenced core algorithm.

#### Details
- Run the traces command without output selection and parse the stdout report as JSON.
- Run the same command with a file destination and compare parsed file content with stdout content from the same model and filters.
- Confirm successful file output and valid JSON use the shared JSON output behavior.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [CLI Interface Verification Objective](#cli-interface-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-json-file-output/test.sh)
  * verify: [CLI Traces Result Presentation](../../../Interfaces/CLI/Commands.md#cli-traces-result-presentation)
---

### Explicit Workspace Selection Verification

This verification shall prove that Reqvire commands can be launched outside a project while operating on an explicitly selected workspace.

#### Details
Expected checks:
- Run a normal CLI report command from outside the target workspace using `--workspace <DIR>` and verify it reads the selected workspace model.
- Run change-impact from outside the target workspace using `--workspace <DIR>` and verify source-control comparison is computed against the selected workspace context.
- Start MCP Streamable HTTP from outside the target workspace using `--workspace <DIR>`, initialize at `/mcp`, and verify `reqvire.workspace_status` reports the selected workspace.
- Verify workspace selection is applied before ignore-pattern loading and model validation.
- Verify invalid workspace paths fail before command execution.
- Verify omitting `--workspace` preserves current working directory behavior.

The existing harness also exercises shared-core element and file moves through the current CLI adapter. Those cases establish selected-workspace path handling, not acceptance of CLI authoring under the new interface boundary or MCP mutation transport. MCP authoring acceptance is covered by the separate mutation and interface-boundary verification plans.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-workspace-flag/test.sh)
  * verify: [Explicit Workspace Selection](../../../Interfaces/CLI/Commands.md#explicit-workspace-selection)
---

### Model Interface Operation Boundary Verification

Verify the supported CLI catalog, rejected authoring surface, MCP permissions, and source-migration exception as one interface acceptance matrix.

#### Details
Acceptance checks:
- Compare root and nested help against the exact retained command catalog, including static and semantic exports, concept tools, managed-query inspections, workspace controls, server launch, and migrations.
- For every removed CLI authoring command and CLI `format`, test invocation and preview/dry-run forms; test `lint --fix`. Require argument rejection with MCP guidance before loading even an invalid fixture, and compare source trees to prove no writes.
- Verify retained CLI reads and artifact generation still operate without an MCP client; artifact output does not introduce a model mutation path.
- Verify migration preview leaves incompatible source fixtures unchanged, migration application performs only supported rewrites, and the upgraded model validates before MCP can start.
- Compare read-only and mutation-enabled MCP catalogs: authoring tools require enablement; formatting preview and lint reporting remain available; applying format or lint fixes requires enablement.
- Verify no MCP migration tool or pre-validation service mode is advertised, and invalid startup never opens a usable protocol endpoint.

Executable fixtures reject the removed CLI surface before workspace loading, compare the retained help catalog, exercise migration preview/application, and check typed MCP permission schemas and source diffs.

- Exercise failures on retained validation/report commands as well as removed arguments: require contextual CLI messages and actionable guidance, nonzero failure exits, and logging that preserves structured stdout under its default and explicitly selected filters.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [CLI Interface Verification Objective](#cli-interface-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-cli-help-structure/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-contract-relation-migration/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-interface-boundary/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [Detailed Error Handling and Logging](../../../Interfaces/CLI/Commands.md#detailed-error-handling-and-logging)
  * verify: [Model Interface Operation Boundary](../../../Interfaces/InterfacesRequirements.md#model-interface-operation-boundary)
---

### npm Concurrent Launch Verification

Verify packaged CLI invocation and atomic extraction under concurrent first use.

#### Details
- Launch sixteen copies of the actual npm launcher against a fixture archive with synchronized extraction. All invocations must complete and preserve their argument/output values; one complete executable remains cached and invocation-owned temporary directories are removed.
- Warm-cache launches skip extraction. Distinct versions remain isolated.
- Fail extraction after writing partial output, assert no executable is published and temporary output is cleaned, then retry successfully. A concurrent failed extractor must not damage a successful publisher.
- Check missing archives, unsupported platforms, native nonzero exit status, termination signals, and arguments containing spaces.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [CLI Interface Verification Objective](#cli-interface-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-npm-launcher/test.sh)
  * verify: [npm CLI Distribution](../../../Interfaces/CLI/Commands.md#npm-cli-distribution)
---
