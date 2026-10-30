# Elements

### Change Impact Analysis Verification Objective

This objective groups verification that change impact analysis detects changed model content, relation impact, smart filtering, contract_bindings context, semantic contract context, and structural summaries.

#### Metadata
  * type: verification-objective

#### Relations
  * derive: [Capability Ontology and Semantic Contract Change Impact Test](#capability-ontology-and-semantic-contract-change-impact-test)
  * derive: [Change Impact Analysis Verification](#change-impact-analysis-verification)
  * derive: [Change Impact Detection Test](#change-impact-detection-test)
  * derive: [Change Impact Relations Test](#change-impact-relations-test)
  * derive: [Change Impact Smart Filtering Test](#change-impact-smart-filtering-test)
  * derive: [Element Content Extraction Test](#element-content-extraction-test)
  * derive: [Impact Scope Summary Test](#impact-scope-summary-test)
  * derive: [Structural Change Reports Verification](#structural-change-reports-verification)
---

### Capability Ontology and Semantic Contract Change Impact Test

This test verifies that capability, specified requirement, ontology, and semantic-contract changes propagate through the model.

#### Details

##### Acceptance Criteria
- Capability content changes propagate to requirements through `specifiedBy`.
- Requirement content changes continue to propagate to verifications through `verifiedBy`.
- Requirement content changes flag semantic-contract consistency review through `constrainedBy`.
- Semantic-contract content changes propagate to constrained requirements through `constrain`.
- Semantic-contract content changes do not propagate backward to ontology vocabulary through `use`.
- Ontology content changes propagate to semantic contracts through `usedBy`, then to constrained requirements and downstream verifications.

##### Test Criteria
- Exercise the four fixture changes independently: capability content, its specified requirement content, the constraining semantic contract's minimum cardinality, and contract-only ontology content.
- Run `reqvire change-impact --json`; assert exit code 0 and valid JSON.
- Assert the capability impact tree contains the specified requirement.
- Assert the requirement impact tree contains the semantic contract and verification.
- Assert the semantic-contract impact tree contains the constrained requirement and verification, but not the used ontology.
- Assert the contract-only ontology impact tree contains the semantic contract, constrained requirement, and verification.
- These fixtures establish the listed capability, requirement, semantic-contract and ontology impact paths. They do not assert native concept changes or `reqvire:mapsToConcept` bridge changes.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-capability-change-impact/test.sh)
  * verify: [Change Impact Detection](../../../Processing/ChangeImpact/ChangeImpactRequirements.md#change-impact-detection)
  * verify: [Requirements Change Propagation](../../../Processing/ChangeImpact/ChangeImpactRequirements.md#requirements-change-propagation)
---

### Change Impact Analysis Verification

This test verifies that the system generates change impact reports when requested.

#### Details

##### Acceptance Criteria
- System should generate change impact reports in Markdown format
- System should support JSON output for change impact reports
- Reports should include an overview of model changes and their impact

##### Test Criteria
- Command exits with success (0) return code
- Reports contain expected impact information
- Both Markdown and JSON formats are properly supported

##### Test Procedure
TODO: write test procedure

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-change-impact-detection/test.sh)
  * verify: [CLI Change Impact Report Command](../../../Interfaces/CLI/Commands.md#cli-change-impact-report-command)
---

### Change Impact Detection Test

This test verifies that the system correctly implements change impact detection, including proper default handling of the git commit parameter and smart filtering.

#### Details

##### Acceptance Criteria
- System correctly detects changes between different versions of requirements
- System correctly identifies element relocations across source files while preserving the name-derived Element ID and reporting changed location-based identifiers
- Relocated elements without content changes do not trigger impact propagation
- Relocated elements WITH content or relation changes appear in BOTH "Relocated" AND "Changed" sections
- Relocated elements appear in a separate "Relocated" section in the report
- System properly constructs a change impact report based on relationships between elements
- Relations are compared semantically by element name, not by identifier (prevents false positives when children relocate)
- Relocated parent with new relation to relocated+changed child is detected correctly
- Default base snapshot is the current eligible worktree HEAD when --git-commit is not provided and Git metadata is available
- System provides output in both human-readable text and JSON formats
- Smart filtering removes redundant elements that appear in other elements' relations

##### Test Criteria
- Command exits with success (0) return code
- Change impact report shows expected elements
- Change impact report shows correct relationships between elements
- Changed elements referenced in other changed elements' relations are filtered out (e.g., "Power Saving" filtered when referenced by "Power Saving Mode")
- Relocated elements are reported with old location → new location format
- Pure relocations (same content, different location) do NOT appear in "Removed" + "Added" sections
- Pure relocations do NOT appear in impact propagation tree
- Relocated+changed elements appear in BOTH Relocated AND Changed sections
- Parent element with added relation to relocated+changed child shows in Changed with impact tree
- Relations to relocated elements don't cause false change detection in parent elements
- Summary statistics include count of relocated elements
- Element IDs remain stable when elements are relocated between files
- Output format matches requested format (text or JSON)
- Both explicit and implicit base snapshot parameters work properly
- Source paths, relation targets, contract_bindings targets, evidence paths, and relocation paths remain workspace-root-relative in output
- JSON output is valid and contains all necessary information
- GitHub-style blob URLs are included in the output

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-change-impact-detection/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-change-impact-element-relocation/test.sh)
  * verify: [CLI Change Impact Report Command](../../../Interfaces/CLI/Commands.md#cli-change-impact-report-command)
  * verify: [Requirements Change Propagation](../../../Processing/ChangeImpact/ChangeImpactRequirements.md#requirements-change-propagation)
---

### Change Impact Relations Test

This test verifies that the system correctly handles different relation types when calculating change impact.

#### Details

##### Acceptance Criteria
- System correctly propagates changes through different relation types
- System respects the IMPACT_PROPAGATION_RELATIONS list when determining impact flow
- System does not propagate impact through containment (file location) changes
- Element relocations without content changes do not trigger impact propagation
- System handles complex chains of relations properly

##### Test Criteria
- Command exits with success (0) return code
- Change impact report shows expected propagation through derivedFrom/derive relations
- Change impact report shows expected propagation through satisfiedBy/satisfy relations
- Change impact report shows expected propagation through verifiedBy/verify relations
- Relocations without content changes do NOT propagate through relations
- When an element moves files without content change, related elements are not marked as impacted
- System correctly handles circular dependencies in relation chains

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-change-impact-detection/test.sh)
  * verify: [CLI Change Impact Report Command](../../../Interfaces/CLI/Commands.md#cli-change-impact-report-command)
  * verify: [Requirements Change Propagation](../../../Processing/ChangeImpact/ChangeImpactRequirements.md#requirements-change-propagation)
---

### Change Impact Smart Filtering Test

This test verifies that the smart filtering correctly handles new elements in change impact reports, filtering child elements while showing parent elements.

#### Details

##### Acceptance Criteria
- New parent elements appear in the "New Elements" section
- New child elements (with parent relationships to other new elements) are filtered out
- Filtered child elements are shown in parent's relations with "(new)" marker
- Verification elements that are not children remain in the report

##### Test Criteria
- When adding a parent and child requirement together, only parent appears in "New Elements"
- When adding a requirement and its verification, both appear (verification is not a child)
- Child elements are visible in the parent's change impact tree with appropriate markers

##### Test Procedure
1. Create test repository with existing requirements
2. Add new parent requirement with derive relation to new child requirement
3. Add new child requirement with derivedFrom relation to parent
4. Run change impact detection
5. Verify only parent appears in "New Elements" section
6. Verify child appears in parent's relations with "(new)" marker

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-change-impact-smart-filtering/test.sh)
  * verify: [Requirements Change Propagation](../../../Processing/ChangeImpact/ChangeImpactRequirements.md#requirements-change-propagation)
---

### Cross-Repository Impact Report Verification

Verifies that repository boundaries preserve the complete model comparison and its auditable review paths.

#### Details

##### Acceptance Criteria
- A changed component contract reaches the referencing requirement in a separate documentation repository, its inspection verification and page source. Reference-edge provenance remains distinct and the verification is reported once.
- Text and JSON reports identify the relevant Central, manifest and component commits. Project and manifest-input provenance is sorted and workspace-relative.
- Source links use the owning repository and correct side's commit; component paths are never rendered as Central blob links. Unsupported web remotes retain workspace-relative links.
- Added and removed projects produce the corresponding element additions and removals.
- A project path move without content changes yields stable-name relocations without false additions, removals or content changes.
- CLI, standalone MCP and embedded MCP yield equivalent shared report data.
- CLI and MCP expose one baseline revision selector. MCP discovery advertises `git_commit` and omits the removed separate manifest selector; no component selector is introduced.

##### Test Criteria
Run the Bash Repo workspace suite and compare its PASS inventory, normalized provenance and relocation JSON with the committed golden files. Use jq for structured report assertions and diff for deterministic comparisons. Repeated comparisons of unchanged snapshots MUST be deterministic. Runtime vocabulary and shape exports MUST retain the workspace-snapshot and repository-selection declarations.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Change Impact Analysis Verification Objective](#change-impact-analysis-verification-objective)
  * satisfiedBy: [repo_impact.rs](../../../../crates/reqvire-core/src/repo_impact.rs)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-repo-workspaces/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-runtime-ontology-artifact/test.sh)
  * verify: [CLI Change Impact Report Command](../../../Interfaces/CLI/Commands.md#cli-change-impact-report-command)
  * verify: [MCP Quality Traceability Tools](../../../Interfaces/MCP/Tools.md#mcp-quality-traceability-tools)
  * verify: [Cross-Repository Change Impact Reporting](../../../Processing/ChangeImpact/ChangeImpactRequirements.md#cross-repository-change-impact-reporting)
---

### Element Content Extraction Test

This test verifies that the system correctly extracts element content for change impact detection.

#### Details

##### Acceptance Criteria
- System should properly extract requirement body for change impact detection
- Requirement body should include normalized main text and content from the Details subsection
- System should handle requirements with various combinations of subsections

##### Test Criteria
- Command exits with success (0) return code
- Output shows expected content for each element
- Content extraction correctly handles different subsection ordering
- Content extraction properly handles HTML details tags

##### Test Procedure
1. Create test fixtures with requirements containing various combinations of subsections
2. Run Reqvire model summary on the test fixtures
3. Verify that extracted content matches expected content for each element
4. Verify that content from Details subsection is properly included

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-element-content-extraction/test.sh)
  * verify: [Requirements Processing](../../../ModelStructure/Configuration.md#requirements-processing)
  * verify: [Requirements Change Propagation](../../../Processing/ChangeImpact/ChangeImpactRequirements.md#requirements-change-propagation)
---

### Impact Scope Summary Test

This test verifies that the change impact report correctly computes and displays the impact scope summary showing common parent requirements.

#### Details

##### Acceptance Criteria
- After changing sibling requirements under Branch A, impact scope includes `Branch A`.
- After deleting `Leaf B1`, impact scope includes `Branch B`.
- After changing standalone requirement, impact scope includes `Standalone Req`.
- Text output matches expected report fixture.
- In text output, the `Impact Scope` section appears after `Changed Elements`.
- JSON output includes `impact_scope` with exactly 3 entries and expected names.

##### Test Criteria
- Modify `Leaf A1`, `Leaf A2`, and `Standalone Req`, and delete `Leaf B1` in test fixture model.
- Run `reqvire change-impact`; assert exit code 0 and compare sanitized text output to `expected/change-impact-report.txt`.
- Assert text output section order places `### Impact Scope` after `### Changed Elements`.
- Run `reqvire change-impact --json`; assert valid JSON and presence of `.impact_scope`.
- Assert `.impact_scope | length == 3`.
- Assert `.impact_scope[].name` equals:
  - `Branch A`
  - `Branch B`
  - `Standalone Req`

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-change-impact-scope/test.sh)
  * verify: [Change Impact Detection](../../../Processing/ChangeImpact/ChangeImpactRequirements.md#change-impact-detection)
---

### Repo Change Impact Baseline Verification

Verifies complete historical composition reconstruction and read isolation using real local Repo clients.

#### Details

##### Acceptance Criteria
- The sole selected workspace-root repository commit supplies its model, manifest and includes. Historical component commits come from that manifest. Current files do not substitute for deleted historical includes.
- Independent native manifest history is rejected with the affected path and guidance to select a manifest tracked in the workspace-root repository. CLI, standalone MCP and embedded MCP preserve source state and accepted context on that error. A tracked workspace-root manifest remains usable when unrelated native manifest history exists.
- Missing historical component objects and mutable baseline revisions produce project-specific errors without a partial report.
- Base-only projects remain discoverable from retained local objects after their live checkout is removed.
- Ordinary ambiguous multi-worktree comparisons retain their existing rejection.
- Successful and failed reads preserve repository HEADs, indexes, refs, working bytes, manifest inputs and source-selection Repo administration; operational trace logs are outside the source-state comparison.
- Standalone and embedded MCP preserve the accepted model revision, pending changes and write availability. Accepted pending additions remain in current comparison inputs even after external working-file edits.

##### Test Criteria
Run the Repo workspace suite through its Bash entry point. All impact outcomes in the golden file MUST be PASS. Supported single-selector comparisons must succeed; independent-history and removed-selector rejection checks must fail explicitly with the expected diagnostics, rather than skip comparisons or construct partial baselines. Validate each fixture independently before comparison. Compare repository state before and after impact requests, including failure paths. Inspect retained logs and response JSON when a check fails.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Change Impact Analysis Verification Objective](#change-impact-analysis-verification-objective)
  * satisfiedBy: [mutation_io.rs](../../../../crates/reqvire-core/src/mutation_io.rs)
  * satisfiedBy: [repo_impact.rs](../../../../crates/reqvire-core/src/repo_impact.rs)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-repo-workspaces/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-workspace-flag/test.sh)
  * verify: [Manifest-Based Change Impact Baselines](../../../Processing/ChangeImpact/ChangeImpactRequirements.md#manifest-based-change-impact-baselines)
---

### Structural Change Reports Verification

This test verifies that the system analyzes and reports on structural changes in the System model.

#### Details

##### Acceptance Criteria
- System should analyze structural changes in the System model
- System should identify affected components through relationship traversal
- System should generate reports of impacted elements and structures

##### Test Criteria
- Command exits with success (0) return code
- Change reports correctly identify affected elements
- Relationship traversal properly determines impact propagation

##### Test Procedure
TODO: write test procedure

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-change-impact-detection/test.sh)
  * verify: [Tracing Structural Changes](../../../Reports/ModelReports/ReportingRequirements.md#tracing-structural-changes)
---
