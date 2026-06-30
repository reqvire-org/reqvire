# Elements

### CLI Interface Verification Objective

This objective groups verification that the Reqvire command-line interface exposes stable commands, options, workspace selection, and ontology output behavior.

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

This test verifies that the system properly handles the git commit hash flag for change impact analysis.

#### Details

##### Acceptance Criteria
- System shall support --git-commit flag for change impact analysis
- System shall use specified commit hash as base for comparison
- System shall default to HEAD when flag is not specified
- System shall handle relative commit references (HEAD~1, etc.)

##### Test Criteria
- Command with explicit --git-commit flag runs successfully
- Command without flag defaults to HEAD commit
- Relative commit references are correctly resolved
- Invalid commit references are reported appropriately
- Change impact analysis correctly uses specified commit as baseline

##### Test Procedure
1. Create test fixtures with git repository containing multiple commits
2. Run Reqvire with --change-impact --git-commit=HEAD~1
3. Verify that the specified commit is used as baseline
4. Run Reqvire with --change-impact (no git-commit flag)
5. Verify that HEAD is used as default baseline
6. Run with invalid commit reference and verify appropriate error

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-change-impact-detection/test.sh)
  * verify: [CLI Change Impact Report Command](../../../Interfaces/CLI/Commands.md#cli-change-impact-report-command)
---

### CLI Help Structure Verification

This test verifies that the CLI help output displays all commands and their options flattened in the main help, and that all CLI command requirements are covered.

#### Details

##### Acceptance Criteria
- Running `reqvire` (without arguments) displays the main help output as the default command
- Main help output shows Usage line: `reqvire [OPTIONS] <COMMAND> [COMMAND OPTIONS]`
- Main help lists all available commands under "Commands:" section
- Main help lists all global options under "Options:" section
- Command-specific options are displayed under dedicated headings (e.g., "SUMMARY OPTIONS:", "FORMAT OPTIONS:")
- All command options are flattened and visible in the main help without needing to drill down
- Help output includes nested subcommands expanded (e.g., "traces", "coverage")
- Each command's help text is descriptive and clear

##### Test Criteria
1. **Main help output structure**
   Command: `reqvire`
   - exits code **0**
   - output contains "Usage: reqvire [OPTIONS] <COMMAND> [COMMAND OPTIONS]"
   - output contains "Commands:" section
   - output contains "Options:" section with `-h, --help`, `-V, --version`

2. **Command listing completeness**
   - All current CLI commands are listed: serve, mcp, format, migrate, validate, search, change-impact, traces, coverage, model, lint, add, rm, mv, rename, merge, mv-file, mv-folder, link, unlink, relink, mv-asset, rm-asset, containment, resources, ontologies, submodels, collect

3. **Options flattening - all command-specific option sections present**
   - SERVE OPTIONS section visible
   - MCP OPTIONS section visible
   - FORMAT OPTIONS section visible
   - MIGRATE OPTIONS section visible
   - CHANGE IMPACT OPTIONS section visible
   - TRACES OPTIONS section visible
   - COVERAGE OPTIONS section visible
   - MODEL OPTIONS section visible
   - ONTOLOGIES OPTIONS section visible

4. **Help text quality**
   - Each command has descriptive help text
   - Options have clear descriptions

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
  * verify: [Format Command](../../../Interfaces/CLI/Commands.md#format-command)
  * verify: [Validate Command](../../../Interfaces/CLI/Commands.md#validate-command)
  * verify: [Served Explorer Browser Interface](../../../Interfaces/WebExplorer/Capabilities.md#served-explorer-browser-interface)
---

### CLI Migrate Command Verification

This verification shall prove that the migrate command previews and applies deterministic source migrations without writing outside the intended model file locations.

#### Details
Expected checks:
- Run `reqvire validate` on a fixture with legacy contract relations and verify migration candidates are reported.
- Run `reqvire migrate` and verify the dry-run preview reports legacy contract relation rewrites without changing source files.
- Run `reqvire migrate --fix` and verify legacy `refinedBy` and `refine` relations are rewritten to `definedBy` and `define`.
- Run `reqvire migrate --fix` from a repository subdirectory and verify changed files are written to the git-root-relative source path rather than a duplicated subdirectory path.
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

### Explicit Workspace Selection Verification

This verification shall prove that Reqvire commands can be launched outside a project while operating on an explicitly selected workspace.

#### Details
Expected checks:
- Run a normal CLI report command from outside the target repository using `--workspace <DIR>` and verify it reads the selected workspace model.
- Run model mutation commands from outside the target repository using `--workspace <DIR>` and verify changed files are written inside the selected workspace.
- Run file move mutation from outside the target repository using `--workspace <DIR>` and verify the moved file and updated model references remain inside the selected workspace.
- Run change-impact from outside the target repository using `--workspace <DIR>` and verify git comparison is computed against the selected workspace repository.
- Run MCP stdio startup from outside the target repository using `--workspace <DIR>` and verify workspace status reports the selected workspace.
- Verify workspace selection is applied before ignore-pattern loading and model validation.
- Verify invalid workspace paths fail before command execution.
- Verify omitting `--workspace` preserves current working directory behavior.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-workspace-flag/test.sh)
  * verify: [Explicit Workspace Selection](../../../Interfaces/CLI/Commands.md#explicit-workspace-selection)
---
