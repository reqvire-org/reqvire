# ChangeImpact

## Change Analysis Requirements

### Structural Change Analyzer

The system shall implement a model change analyzer that identifies structural modifications between model versions, determines affected elements through relationship traversal, and categorizes impacts according to change propagation rules.

#### Relations
  * derivedFrom: [Tracing Structural Changes](Reporting.md#tracing-structural-changes)
  * satisfiedBy: [change_impact.rs](../../core/src/change_impact.rs)
---

### Structural Change Reports

The system shall generate detailed reports summarizing the impact of structural changes, including affected relationships and components.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Structural Change Analyzer](#structural-change-analyzer)
---

### Change Impact Command Line Interface

The system shall provide a command-line interface for initiating change impact analysis and controlling output formats.

#### Details
The CLI shall support the following functionality:

1. **Analysis Invocation**:
   - Support analyzing changes between git commits
   - Enable specifying elements to analyze by ID or pattern
   - Allow limiting analysis to specific relation types
   - Support depth limitations for large models

2. **Output Formats**:
   - Generate formatted text reports
   - Produce JSON-structured impact data
   - Create Mermaid diagrams of impact trees
   - Integrate with HTML report generation

3. **Integration Points**:
   - Support integration with CI/CD pipelines
   - Enable calling from external systems via API
   - Support webhook triggers for automated analysis
   - Allow scripting of analysis operations

#### Relations
  * derivedFrom: [CLI Change Impact Report Command](../Interfaces/CLI.md#cli-change-impact-report-command)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
---

## Requirements

### Change Impact Analysis

When requested the system shall generate change impact report, in Markdown format by default and also supporting json output.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Requirements Change Propagation](Reporting.md#requirements-change-propagation)
  * derivedFrom: [Deterministic Output for All Generated Content](Reporting.md#deterministic-output-for-all-generated-content)
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
  * verify: [Change Impact Detection Algorithm](#change-impact-detection-algorithm)
  * verify: [Change Impact Command Line Interface](#change-impact-command-line-interface)
  * satisfiedBy: [test.sh](../../tests/test-change-impact-detection/test.sh)
  * satisfiedBy: [test.sh](../../tests/test-change-impact-element-relocation/test.sh)
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
  * verify: [Change Impact Detection Algorithm](#change-impact-detection-algorithm)
  * verify: [Change Impact Command Line Interface](#change-impact-command-line-interface)
  * satisfiedBy: [test.sh](../../tests/test-change-impact-detection/test.sh)
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
  * verify: [CLI Change Impact Report Command](../Interfaces/CLI.md#cli-change-impact-report-command)
  * satisfiedBy: [test.sh](../../tests/test-change-impact-detection/test.sh)
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
  * verify: [Change Impact Detection Algorithm](#change-impact-detection-algorithm)
  * verify: [Requirements Processing](Configuration.md#requirements-processing)
  * satisfiedBy: [test.sh](../../tests/test-element-content-extraction/test.sh)
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
  * verify: [Change Impact Analysis](#change-impact-analysis)
  * satisfiedBy: [test.sh](../../tests/test-change-impact-detection/test.sh)
---

### Structural Change Reports Verification

This test verifies that the system analyzes and reports on structural changes in the MBSE model.

#### Details

##### Acceptance Criteria
- System should analyze structural changes in the MBSE model
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
  * verify: [Tracing Structural Changes](Reporting.md#tracing-structural-changes)
  * satisfiedBy: [test.sh](../../tests/test-change-impact-detection/test.sh)
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
  * verify: [Change Impact Detection Algorithm](#change-impact-detection-algorithm)
  * satisfiedBy: [test.sh](../../tests/test-change-impact-smart-filtering/test.sh)
---

## Change Impact Detection Components

### Change Impact Detection Algorithm

The system shall implement a requirement change detection algorithm that identifies changes between versions of the model and determines their impact through relationship traversal.

#### Details
The algorithm shall consist of the following steps:

1. **Diff Analysis**:
   - Compare elements between versions using stable Element IDs (not location-based identifiers)
   - Identify changes by type:
     - **Content Changes**: Element ID exists in both versions, content hash differs
     - **Additions**: Element ID exists only in current version
     - **Removals**: Element ID exists only in previous version
     - **Relocations**: Element ID exists in both, but file_path or section differs
   - Generate a ChangeSet representing all detected changes
   - Associate changes with specific elements in the model
   - Note: Pure relocations (no content changes) do not trigger impact propagation

2. **Relocation Detection**:
   - For each element present in both versions (matched by Element ID):
     - Compare file_path field (implicit file containment)
     - Compare section field (implicit section containment)
     - If either differs → classify as relocation
     - Track old location and new location for reporting
   - Relocations without content changes do NOT propagate impact
   - Relocations WITH content changes propagate based on content change only

3. **Impact Determination**:
   - For each changed element, identify all relations from the element
   - Apply relation-specific propagation rules as defined in RelationTypesRegistry.md
   - Consider the relation direction and change impact direction for each relation
   - Build an impact tree representing the propagation of changes

4. **Recursive Traversal**:
   - Perform a depth-first traversal of relationships
   - Create a directed acyclic graph (DAG) of change impact
   - Handle circular dependencies by preventing infinite recursion
   - Track visited nodes to prevent duplicate processing

5. **Impact Classification**:
   - Assign impact severity levels based on relation types
   - Classify changes as:
     - Direct: Changes to the element itself
     - Indirect: Changes propagated from related elements
     - Potential: Changes that might affect an element based on semantic analysis
   - Calculate aggregated impact scores for each affected element

6. **Performance Optimization**:
   - Implement caching of traversal results
   - Use parallel processing for independent branches of the impact tree
   - Apply pruning techniques to limit traversal depth when appropriate
   - Support incremental impact analysis for large models

**Change Impact Visualization:**
The system shall provide visual representations of change impact to help users understand the scope and implications of changes.

The visualization shall include:

1. **Tree View**:
   - Display a hierarchical tree of affected elements
   - Group elements by change type (content changes, additions, removals, relocations)
   - Show relation types between elements
   - Show old and new locations for relocated elements
   - Support collapsing/expanding nodes for better navigation

2. **Color Coding**:
   - Use consistent color scheme for impact types (Direct: Red, Indirect: Yellow, Potential: Blue)
   - Indicate relation types with different line styles
   - Highlight newly introduced or removed relationships

3. **Interactive Elements**:
   - Allow clicking on elements to focus the view

4. **Summary Statistics**:
   - Display counts of affected elements by type
   - Show metrics for impact breadth and depth
   - Calculate change propagation fan-out metrics
   - Generate overall change impact assessment

**Smart Filtering for Change Impact Reports:**
The system shall implement intelligent filtering logic to eliminate redundant information from change impact reports and focus on primary changes and their relationships.

The smart filtering shall implement the following logic:

1. **Primary Change Detection**:
   - Distinguish between primary changes and secondary changes
   - Filter out elements already referenced in relations of other elements
   - Apply filtering to both new-to-new and changed-to-changed element relationships

2. **Comprehensive Filtering Rules**:
   - Eliminate redundant new elements referenced in relations of other elements
   - Eliminate redundant changed elements referenced in relations of other changed elements
   - Show only independent elements not already covered by relationships
   - Mark new elements with "(new)" suffix and changed elements with "⚠️" symbol

3. **Cross-Category Filtering**:
   - Apply filtering across all categories (new, changed, removed)
   - Preserve the most informative context for each filtered element

4. **Hierarchical Organization**:
   - Present changes in order of importance
   - Group related changes together to show impact chains clearly
   - Maintain complete traceability while reducing visual clutter

5. **Benefits**:
   - Reduced clutter by eliminating redundant information
   - Improved focus on primary changes
   - Clear context for elements shown in relevant relationship context
   - Better readability with concise reports

#### Relations
  * derivedFrom: [Requirements Change Propagation](Reporting.md#requirements-change-propagation)
  * derivedFrom: [Element Identity Model](ModelManagement.md#element-identity-model)
  * satisfiedBy: [change_impact.rs](../../core/src/change_impact.rs)
---
