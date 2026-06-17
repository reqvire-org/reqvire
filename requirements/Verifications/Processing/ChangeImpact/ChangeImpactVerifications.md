# Elements

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
- Attached ontology content changes mark the attaching capability as changed and propagate through that capability context to descendant capabilities and specified requirements.

##### Test Criteria
- Modify a capability, its specified requirement, the constraining semantic contract, a contract-only ontology, and an attached ontology.
- Run `reqvire change-impact --json`; assert exit code 0 and valid JSON.
- Assert the capability impact tree contains the specified requirement.
- Assert the requirement impact tree contains the semantic contract and verification.
- Assert the semantic-contract impact tree contains the constrained requirement and verification, but not the used ontology.
- Assert the contract-only ontology impact tree contains the semantic contract, constrained requirement, and verification.
- Assert the attaching capability reports the attached ontology in `changed_attachments`.

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
- System correctly identifies element relocations (same Element ID, different file_path or section)
- Relocated elements without content changes do not trigger impact propagation
- Relocated elements WITH content or relation changes appear in BOTH "Relocated" AND "Changed" sections
- Relocated elements appear in a separate "Relocated" section in the report
- System properly constructs a change impact report based on relationships between elements
- Relations are compared semantically by element name, not by identifier (prevents false positives when children relocate)
- Relocated parent with new relation to relocated+changed child is detected correctly
- Default git commit is HEAD when --git-commit parameter is not provided
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
- Both explicit and implicit git commit parameters work properly
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

