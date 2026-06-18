# Elements

### CLI Interface Verification Objective

This objective groups verification that the Reqvire command-line interface exposes stable commands, options, workspace selection, ontology output, and navigation behavior.

#### Metadata
  * type: verification-objective

#### Relations
  * derive: [CLI Git Commit Hash Flag Test](#cli-git-commit-hash-flag-test)
  * derive: [CLI Help Structure Verification](#cli-help-structure-verification)
  * derive: [CLI Ontologies Command Verification](#cli-ontologies-command-verification)
  * derive: [CLI Size Estimate JSON Option Verification](#cli-size-estimate-json-option-verification)
  * derive: [Explicit Workspace Selection Verification](#explicit-workspace-selection-verification)
  * derive: [Verification Traces Element Navigation Test](#verification-traces-element-navigation-test)
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
   - All current CLI commands are listed: serve, mcp, format, migrate, validate, search, change-impact, traces, coverage, model, lint, add, rm, mv, rename, merge, mv-file, link, unlink, relink, mv-asset, rm-asset, containment, resources, ontologies, submodels, collect

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

### CLI Ontologies Command Verification

This verification shall prove that the ontologies command collects ontology `Ontology` content and semantic-contract `Shapes` content.

#### Details
Expected checks:
- Run `reqvire ontologies` and verify Turtle output contains generated ontology document declarations, ontology term declarations, and SHACL shape references without generated ontology projection facts.
- Verify generated ontology document declarations use the resolved `ontology_base` as the `owl:Ontology` IRI and list same-base ontology elements as contributors instead of emitting one document per element.
- Run `reqvire ontologies --jsonld` and verify the output is valid JSON-LD without generated ontology projection facts.
- Run `reqvire ontologies --include-external` and verify local external ontology source triples are included while default `reqvire ontologies` keeps those source triples out.
- Run `reqvire ontologies --full` and verify Turtle output contains Reqvire model context triples linking the capability, ontology, requirement, and semantic-contract elements, plus generated ontology projection graph, projection, construct, symbol, source/provenance, member, and subject/object/predicate facts for direct-authored constructs.
- Run `reqvire ontologies --full --jsonld` and verify JSON-LD output contains Reqvire model context triples and generated ontology projection facts.
- Run `reqvire ontologies --full --include-external` through fixture coverage when external source materialization is enabled together with full semantic export.
- Verify default Turtle output contains generated ontology document declarations and collected ontology content without generated ontology projection facts.
- Verify the Ontologies Explorer uses the same collected ontology content and semantic projection model when served.
- Verify the Reqvire authored ontology source tree does not contain stale `owl:deprecated true` presentation-only vocabulary declarations after ontology refactoring.
- Verify representative reserved vocabulary IRIs from the RDF, RDFS, OWL, and XSD registry survive ontology serialization and do not require local External Ontology source declarations. The full executable fixed-list coverage is owned by the reserved-vocabulary implementation tests.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-ontologies-command/test.sh)
  * verify: [CLI Ontologies Command](../../../Interfaces/CLI/Commands.md#cli-ontologies-command)
  * verify: [Local External Ontology Sources](../../../Reports/ModelReports/ReportingRequirements.md#local-external-ontology-sources)
  * verify: [OWL Reserved Vocabulary Recognition](../../../Reports/ModelReports/ReportingRequirements.md#owl-reserved-vocabulary-recognition)
  * verify: [Ontology and Shapes Collection](../../../Reports/ModelReports/ReportingRequirements.md#ontology-and-shapes-collection)
  * verify: [Ontology Projection Subgraph Materialization](../../../Reports/ModelReports/ReportingRequirements.md#ontology-projection-subgraph-materialization)
---

### CLI Size Estimate JSON Option Verification

This verification shall prove that the CLI size-estimate option is JSON-only and enables element size estimates for supported commands.

#### Details
Expected checks:
- Run `reqvire model --json --with-size-estimates` and verify element payloads include `size_estimate`.
- Run `reqvire model --with-size-estimates` without `--json` and verify the command fails with a clear diagnostic.
- Run `reqvire model --json` without `--with-size-estimates` and verify element payloads omit `size_estimate`.
- Verify non-JSON model output is unchanged when the option is absent.

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

### Verification Traces Element Navigation Test

Test verifies that verification element names in the traces report are clickable links.

#### Test Steps
1. Run `reqvire traces` command to generate traces report
2. Verify output contains verification headers as markdown links
3. Verify links follow format `[Verification Name](file_path#fragment)`

#### Expected Results
- Verification names are rendered as markdown links
- Links point to source file with verification fragment

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-verification-traces/test.sh)
  * verify: [Verification Traces Element Navigation](../../../Interfaces/CLI/Commands.md#verification-traces-element-navigation)
---
