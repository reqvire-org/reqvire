# Elements

### Reporting, Search, Coverage, and Model Export Verification Objective

This objective groups verification that Reqvire reporting commands expose collect, search, coverage, traces, containment, resources, submodels, model traversal, JSON, and diagram output correctly.

#### Metadata
  * type: verification-objective

#### Relations
  * derive: [Capability Collect Traversal Test](#capability-collect-traversal-test)
  * derive: [Capability Coverage Rollup Test](#capability-coverage-rollup-test)
  * derive: [CLI Collect Command Test](#cli-collect-command-test)
  * derive: [CLI JSON File Output Test](#cli-json-file-output-test)
  * derive: [Containment Hierarchy Extraction Test](#containment-hierarchy-extraction-test)
  * derive: [Containment View Design Documents Test](#containment-view-design-documents-test)
  * derive: [Containment View JSON Output Test](#containment-view-json-output-test)
  * derive: [Containment View Mermaid Diagram Test](#containment-view-mermaid-diagram-test)
  * derive: [Containment View Text Output Test](#containment-view-text-output-test)
  * derive: [Custom Element Type Tracking Test](#custom-element-type-tracking-test)
  * derive: [JSON Element Size Estimate Output Verification](#json-element-size-estimate-output-verification)
  * derive: [Model Command Verification](#model-command-verification)
  * derive: [Model Containment Report Integration Test](#model-containment-report-integration-test)
  * derive: [Multi-Type Search Filter Test](#multi-type-search-filter-test)
  * derive: [Resources Report Verification](#resources-report-verification)
  * derive: [Reverse Model Traversal Test](#reverse-model-traversal-test)
  * derive: [Search Command Tests](#search-command-tests)
  * derive: [Semantic Contract Search JSON Test](#semantic-contract-search-json-test)
  * derive: [Start Type Filter Test](#start-type-filter-test)
  * derive: [Submodels Report Verification](#submodels-report-verification)
  * derive: [Verification Coverage Report Test](#verification-coverage-report-test)
  * derive: [Verification Traces Filter Options Test](#verification-traces-filter-options-test)
  * derive: [Verification Traces From-Folder Test](#verification-traces-from-folder-test)
---

### CLI Collect Command Test

This test verifies that the collect command aggregates capability, requirement, and ontology context with proper source citations.

#### Details

##### Acceptance Criteria
- System shall provide CLI command `collect` that aggregates content from capability, requirement, and ontology context
- Command shall accept capability, requirement, or ontology element name as positional argument
- Command shall support `--json` flag for JSON output format
- Command shall support `--direction` flag with values UPSTREAM (default) and DOWNSTREAM
- When direction is UPSTREAM, command shall traverse derivedFrom relations in reverse direction (child to parents)
- When direction is DOWNSTREAM, command shall traverse derive relations in forward direction (parent to children)
- Command shall collect element content and attachment contents
- Command shall output with source citations
- Command shall include semantic contracts when collecting downstream from ontology context they use
- Command shall reject element types other than capability, requirement, or ontology with error

##### Test Criteria
1. **Basic Text Output**
   Command: `reqvire collect <requirement-name>`
   - exits code **0**
   - output contains content from starting requirement
   - output contains content from ancestor requirements
   - each content block followed by source citation
   - citation format: `— Source: [Element Name](identifier)`

2. **JSON Output Structure**
   Command: `reqvire collect <requirement-name> --json`
   - exits code **0**
   - output parses as valid JSON
   - JSON contains `starting_element` field
   - JSON contains `items` array with collected content
   - JSON contains `metadata` with counts
   - each item has: name, identifier, file_path, element_type, content, depth, source_type

3. **Ancestor Chain Collection**
   - Starting from leaf requirement
   - Collects content from all derivedFrom ancestors
   - Ancestors ordered by depth (root first, depth 0)
   - Same-level elements sorted alphabetically

4. **Attachment Content Collection**
- element identifier attachments: referenced element content included

5. **Error Handling - Element Not Found**
   Command: `reqvire collect non-existent-element`
   - exits non-zero
   - error message indicates element not found

6. **Error Handling - Non-Requirement Type**
   Command: `reqvire collect <verification-name>`
   - exits non-zero
   - error message indicates element must be a capability or requirement type

7. **Output Ordering**
   - Flat list structure
   - Ancestors first (depth 0 = root)
   - Same-depth elements sorted alphabetically by name

8. **Downstream Text Output**
   Command: `reqvire collect <top-level-requirement-name> --direction DOWNSTREAM`
   - exits code **0**
   - output contains content from starting requirement (depth 0)
   - output contains content from child requirements
   - each content block followed by source citation
   - starting element appears first, children appear after

9. **Downstream JSON Output**
   Command: `reqvire collect <top-level-requirement-name> --direction DOWNSTREAM --json`
   - exits code **0**
   - output parses as valid JSON
   - JSON contains `direction` field with value `downstream`
   - JSON items ordered: starting element at depth 0, children at depth 1, etc.
   - each item has correct depth reflecting distance from starting element

10. **Downstream Descendant Chain Collection**
    - Starting from top-level requirement
    - Collects content from all derive descendants
    - Starting element at depth 0
    - Same-depth elements sorted alphabetically

11. **Default Direction is UPSTREAM**
    Command: `reqvire collect <requirement-name>` (no --direction flag)
    - behavior identical to `--direction UPSTREAM`

12. **Explicit UPSTREAM Direction**
    Command: `reqvire collect <requirement-name> --direction UPSTREAM`
    - behavior identical to omitting --direction

13. **Invalid Direction Error**
    Command: `reqvire collect <requirement-name> --direction INVALID`
    - exits non-zero
    - error message indicates invalid direction

14. **Capability/Requirement Bridge Direction**
    - Requirement UPSTREAM includes owning capability context through `specify`
    - Requirement DOWNSTREAM does not include capability context
    - Capability UPSTREAM includes parent capabilities only
    - Capability DOWNSTREAM includes child capabilities, requirements that specify each capability, and requirement descendants

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-collect-command/test.sh)
  * verify: [CLI Collect Command](../../../Interfaces/CLI/Commands.md#cli-collect-command)
---

### CLI JSON File Output Test

This test verifies that the `--output` flag writes JSON output to a file when used with `--json` across CLI commands.

#### Details

##### Acceptance Criteria
- `--output <FILE>` writes JSON to file when combined with `--json`
- All public commands that expose `--json`, including relation and asset mutation commands, support the same `--output <FILE>` behavior
- Confirmation message printed to stdout: `✅ Output saved to <filepath>`
- File contains valid JSON identical to what `--json` alone would produce on stdout
- `--output` without `--json` produces an error and non-zero exit code
- File is created if it doesn't exist
- File is overwritten if it exists

##### Test Criteria
1. **JSON file output with validate**
   Command: `reqvire validate --json --output output.json`
   - exits code **0**
   - stdout contains `✅ Output saved to output.json`
   - `output.json` exists and contains valid JSON
   - JSON content matches `reqvire validate --json` stdout output

2. **JSON file output with search**
   Command: `reqvire search --json --output search.json`
   - exits code **0**
   - stdout contains `✅ Output saved to search.json`
   - `search.json` exists and contains valid JSON

3. **Error when --output used without --json**
   Command: `reqvire validate --output output.json`
   - exits non-zero
   - stderr contains error message about requiring --json

4. **File overwrite behavior**
   - Create a file with existing content
   - Run command with `--json --output <same-file>`
   - Verify file is overwritten with new JSON content

5. **Mutation command parity**
   - Run dry-run JSON file output for `link`, `unlink`, `relink`, `mv-asset`, and `rm-asset`
   - Verify each command writes valid JSON to the requested output file

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-json-file-output/test.sh)
  * verify: [CLI JSON File Output Option](../../../Interfaces/CLI/Commands.md#cli-json-file-output-option)
---

### Capability Collect Traversal Test

This test verifies collect traversal for `capability`, `requirement`, and the directional `specifiedBy`/`specify` bridge.

#### Details
Test cases:
1. Collecting a requirement UPSTREAM includes its owning capability and capability ancestors.
2. Collecting a requirement DOWNSTREAM includes requirement descendants only.
3. Collecting a capability UPSTREAM includes capability ancestors only.
4. Collecting a capability DOWNSTREAM includes child capabilities, requirements that specify each capability, and requirement descendants.
5. Attached ontology context is included when collecting capability context.
6. Collecting a requirement includes ontology context inherited from the owning capability path without requiring direct requirement ontology attachments.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-collect-command/test.sh)
  * verify: [Capability Collect Traversal](../../../ModelStructure/ModelManagement.md#capability-collect-traversal)
---

### Capability Coverage Rollup Test

This test verifies that capability coverage is reported by rolling up the requirements that specify each capability.

#### Details
Expected checks:
- Capability elements may be directly verified through `verifiedBy`; rollup coverage remains requirement-derived.
- Requirements that specify a capability contribute verification coverage to that capability.
- Child requirements contribute through the requirement `derive` hierarchy.
- Child capabilities contribute through the capability `derive` hierarchy.
- Capability coverage output remains separate from structural validation.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-coverage-report/test.sh)
  * verify: [Capability Coverage Rollup](../../../ModelStructure/ModelManagement.md#capability-coverage-rollup)
---

### Containment Hierarchy Extraction Test

This test verifies that the system correctly extracts the physical containment hierarchy from the model, representing folders, files, and elements in a tree structure while omitting sections.

#### Details

##### Test Criteria
1. **Hierarchy structure extraction:**
   - Create test model with nested folder structure
   - Include multiple levels: root → subfolder → file → elements
   - Verify folders are extracted in correct hierarchy
   - Verify files are placed under correct folders
   - Verify elements are extracted from files

2. **Section omission:**
   - Create test file with H2 sections and H3 elements
   - Verify sections (H2) are NOT included in hierarchy
   - Verify elements (H3) are directly under files
   - Confirm no section nodes in output structure

3. **Element information:**
   - Verify element identifier is extracted correctly
   - Verify element name matches H3 header text
   - Verify element type from Metadata is captured
   - Test with multiple element types (capability, requirement, verification)

4. **Ordering verification:**
   - Verify folders are sorted alphabetically
   - Verify files are sorted alphabetically within folders
   - Verify elements maintain document order
   - Test deterministic output across multiple runs

5. **Data structure validation:**
   - Verify tree structure: Folder → [Subfolders, Files]
   - Verify File → [Elements] relationship
   - Verify all paths are relative to git root
   - Test with empty folders and files without elements

6. **Intermediate folders without files:**
   - Create folder structure: `root/parent/child/file.md` where `parent` has no direct files
   - Verify `parent` folder appears in hierarchy even though it has no files
   - Verify `child` folder appears as subfolder of `parent`
   - Verify deeply nested structures (3+ levels) with intermediate empty folders work correctly
   - Test case: `specifications/System/Core/Verifications/file.md` should show `System` folder even if `System/` has no files directly

##### Acceptance Criteria
- All folders, files, and elements are extracted correctly
- Sections are omitted from the hierarchy
- Element metadata (identifier, name, type) is preserved
- Output follows deterministic ordering
- Tree structure is valid and navigable
- Intermediate folders without direct files are included in hierarchy

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-containment-view/test.sh)
  * verify: [Containment View Report](../../../Reports/ModelReports/ReportingRequirements.md#containment-view-report)
---

### Containment View Design Documents Test

This test verifies that design documents (files in DesignDocuments folders) are correctly included in the containment view output.

#### Test Steps
1. Create a model with DesignDocuments folder containing markdown files
2. Run `reqvire containment` command
3. Verify design documents appear in output grouped by folder
4. Verify design documents are visually distinguished from elements
5. In diagram output, verify design document nodes have click handlers

#### Expected Results
- Design documents are shown under their parent folder
- Design documents display filename
- Design documents are styled differently from specification elements
- Click handlers navigate to document files

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-containment-view/test.sh)
  * verify: [Containment View Report](../../../Reports/ModelReports/ReportingRequirements.md#containment-view-report)
---

### Containment View JSON Output Test

This test verifies that the system generates valid, well-structured JSON output for the containment view with correct schema and deterministic ordering.

#### Details

##### Test Criteria
1. **JSON schema validation:**
   - Verify root object has required keys: `root_folder`, `folders`, `files`, `element_count`
   - Verify folder objects have: `path`, `name`, `subfolders`, `files`
   - Verify file objects have: `path`, `name`, `elements`
   - Verify element objects have: `identifier`, `name`, `type`
   - Test schema with JSON validator

2. **Data accuracy:**
   - Verify `root_folder` matches specifications directory
   - Verify `element_count` matches actual element count
   - Verify all folder paths are correct and relative
   - Verify all file paths are correct and relative
   - Verify all element identifiers match format `path#fragment`

3. **Nested structure:**
   - Verify subfolders are nested under parent folders
   - Verify files are associated with correct folders
   - Verify elements are associated with correct files
   - Test deep nesting (3+ levels)

4. **JSON validity:**
   - Verify output is valid JSON (parse without errors)
   - Verify special characters are properly escaped
   - Verify Unicode characters in element names are handled
   - Test with `jq` tool for validation

5. **Deterministic output:**
   - Verify key ordering is consistent
   - Verify array ordering is deterministic
   - Run command multiple times and compare outputs
   - Verify byte-identical JSON across runs

##### Acceptance Criteria
- JSON output is valid and parseable
- Schema matches specification exactly
- All data is accurate and complete
- Special characters and Unicode are handled correctly
- Output is deterministic

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-containment-view/test.sh)
  * verify: [Containment View Report](../../../Reports/ModelReports/ReportingRequirements.md#containment-view-report)
---

### Containment View Mermaid Diagram Test

This test verifies that the system generates valid Mermaid flowchart diagrams with correct syntax, nested subgraphs, semantic element classes, and clickable links.

#### Details

##### Test Criteria
1. **Mermaid syntax validation:**
   - Verify output starts with `flowchart TD`
   - Verify all subgraphs use correct syntax: `subgraph ID ["Label"]`
   - Verify subgraphs are properly closed with `end`
   - Test diagram can be rendered by Mermaid parser
   - Validate with mermaid-cli or online editor

2. **Subgraph structure:**
   - Verify folders use deterministic subgraph labels
   - Verify files use deterministic subgraph labels
   - Verify subgraphs are properly nested
   - Test nested structure: folder → subfolder → file
   - Verify `direction TB` is set for nested subgraphs

3. **Element nodes:**
   - Verify nodes use 16-character hash IDs
   - Verify node labels show element names
   - Verify hash IDs are unique across diagram
   - Test hash ID generation is deterministic
   - Verify nodes are placed within file subgraphs

4. **Semantic classes:**
   - Verify `class` directives for element types
   - Test capability, requirement, verification, and default node classes are emitted from element type semantics
   - Verify class definitions or semantic class names are included for supported Mermaid consumers
   - Verify generated class names are deterministic and usable by the Explorer Mermaid renderer

5. **Clickable links:**
   - Verify `click` directives for all element nodes
   - Verify links use correct format: `click hashId "path#fragment"`
   - Verify paths are relative to diagram location
   - Test links with special characters in fragments
   - Verify fragment normalization (lowercase, hyphens)

6. **Deterministic output:**
   - Verify node ordering is consistent
   - Verify hash IDs are stable across runs
   - Compare output across multiple executions
   - Test byte-identical output

7. **Element display modes:**
   - Default mode (no flags): verify ALL elements are displayed in each file
   - With `--short` flag: verify only root elements are displayed (those without hierarchical parents in same file)
   - Verify element count changes appropriately between modes
   - Verify description text reflects current display mode

##### Acceptance Criteria
- Mermaid diagram syntax is valid
- Subgraphs correctly represent folder/file hierarchy
- Element nodes use hash IDs and show names
- Styling classes are applied correctly
- Clickable links navigate to correct elements
- Output is deterministic
- Default mode shows all elements
- Short mode shows only root elements

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-containment-view/test.sh)
  * verify: [Containment View Report](../../../Reports/ModelReports/ReportingRequirements.md#containment-view-report)
---

### Containment View Text Output Test

This test verifies that the system generates correctly formatted human-readable text output for the containment view with proper indentation and element metadata display.

#### Details

##### Test Criteria
1. **Hierarchical indentation:**
   - Verify indentation uses 2 spaces per level
   - Test nested structure: root (0), folder (2), subfolder (4), file (6), element (8)
   - Verify consistent indentation across all levels

2. **Container markers:**
   - Verify folders display with a stable folder marker and name
   - Verify files display with a stable file marker and path
   - Verify elements display with `[<type>] <name>`
   - Test all element types have correct bracket notation

3. **Element type display:**
   - Test `[requirement]` for requirements
   - Test `[capability]` for capability elements
   - Test `[verification]` and `[test-verification]` for verifications
   - Test custom element types

4. **Content accuracy:**
   - Verify all folders are displayed
   - Verify all files are displayed with correct paths
   - Verify all elements are displayed with correct names
   - Test empty folders and files are handled correctly

5. **Output format validation:**
   - Verify output is valid UTF-8 text
   - Verify line breaks are consistent
   - Test output matches expected format exactly
   - Compare against reference output

##### Acceptance Criteria
- Text output uses correct indentation (2 spaces per level)
- Container markers are displayed correctly
- Element types are shown in brackets
- All hierarchy levels are represented
- Output is human-readable and well-formatted

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-containment-view/test.sh)
  * verify: [Containment View Report](../../../Reports/ModelReports/ReportingRequirements.md#containment-view-report)
---

### Custom Element Type Tracking Test

This test verifies that the system correctly tracks and displays custom element types in model summary reports, providing accurate counts in both text and JSON output formats.

#### Details

##### Acceptance Criteria
- System SHALL track elements with custom types (types not in the standard categories)
- Custom types SHALL be counted separately from standard element types
- Text output SHALL display custom types in alphabetical order
- JSON output SHALL include `custom_element_types` field in global_counters
- Custom types field SHALL be omitted from JSON when no custom types exist
- Standard types (capability, requirement, verification types, and refinement types) SHALL NOT be counted as custom
- File-type elements SHALL NOT be counted as custom types

##### Test Criteria
1. **Custom Types in Text Output**
   Command: `reqvire summary` (on test data with custom types)
   - exits code **0**
   - output contains `📋 Element Types:` section
   - custom types appear after standard types
   - format: `Custom (type-name): count`
   - custom types are sorted alphabetically
   - standard types (Capabilities, System Requirements, Verifications, and Refinements) appear first

2. **Custom Types in JSON Output**
   Command: `reqvire summary --json` (on test data with custom types)
   - exits code **0**
   - output parses as valid JSON
   - JSON contains `.global_counters.custom_element_types` object
   - custom_element_types is a HashMap with type names as keys and counts as values
   - example: `{"custom_element_types": {"moe": 2, "interface": 1}}`
   - type names are stored and displayed exactly as defined in metadata

3. **No Custom Types Handling**
   Command: `reqvire summary --json` (on test data without custom types)
   - exits code **0**
   - output parses as valid JSON
   - JSON does NOT contain `custom_element_types` field in global_counters
   - field is omitted entirely (not present as empty object)

4. **Standard Types Not Counted as Custom**
   - Verify requirements with type `requirement` are NOT in custom_element_types
   - Verify capabilities with type `capability` are NOT in custom_element_types
   - Verify verifications with type `test-verification` are NOT in custom_element_types
   - Verify verifications with type `analysis-verification` are NOT in custom_element_types
   - Verify verifications with type `inspection-verification` are NOT in custom_element_types
   - Verify verifications with type `demonstration-verification` are NOT in custom_element_types
   - Only elements with non-standard types appear in custom_element_types

5. **Multiple Custom Types**
   Command: `reqvire summary` (on test data with multiple different custom types)
   - exits code **0**
   - each distinct custom type is counted separately
   - counts are accurate for each type
   - alphabetical sorting is maintained
   - example output:
     ```
     Custom (interface): 1
     Custom (moe): 2
     Custom (other): 1
     ```

6. **Alphabetical Sorting**
   - Custom types in text output are sorted alphabetically by type name
   - Example: "interface" appears before "moe", which appears before "other"
   - Sorting is case-sensitive (lowercase before uppercase if mixed)

7. **JSON Structure Validation**
   Command: `reqvire summary --json` (on test data with custom types)
   - `.global_counters.custom_element_types` is an object/map
   - Keys are strings (type names)
   - Values are numbers (counts > 0)
   - No custom_element_types entry with zero count

8. **Custom Type Metadata Write-back**
   Command: `reqvire mv <custom-element-name> <target-file>`
   - exits code **0**
   - moved Markdown preserves the exact custom metadata token, such as `type: other-use-case`
   - moved Markdown does not serialize the internal custom type name without the `other-` prefix

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-custom-element-types/test.sh)
  * verify: [CLI Move Element Command](../../../Interfaces/CLI/Commands.md#cli-move-element-command)
  * verify: [Search Report Generator](../../../Reports/ModelReports/ReportingRequirements.md#search-report-generator)
---

### JSON Element Size Estimate Output Verification

This verification shall prove that JSON model evidence outputs include element size estimates only when explicitly enabled.

#### Details
Expected checks:
- Run model JSON output without size estimates and verify element payloads omit `size_estimate`.
- Run model JSON output with size estimates enabled and verify top-level element payloads include `size_estimate`.
- Verify nested relation element targets include `size_estimate` when size estimates are enabled.
- Verify non-JSON model output remains unchanged and does not render size-estimate fields.
- Verify the size estimate has `content_bytes`, `rendered_context_bytes`, and `estimated_tokens`.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-model-command/test.sh)
  * verify: [JSON Element Size Estimate Exposure](../../../Reports/ModelReports/ReportingRequirements.md#json-element-size-estimate-exposure)
---

### Model Command Verification

Comprehensive test verifying model command generates model-centric nested output in different modes.

#### Details

##### Acceptance Criteria
1. `reqvire model` generates model-centric output showing ontology-root and capability-root structures with nested relations
2. `reqvire model --from=<name>` generates nested structure starting from specified element
3. `reqvire model --json` generates valid JSON with nested element structure
4. `reqvire model --from=<name> --json` generates filtered JSON from specified starting point
5. Default mode filters to top-level model roots according to model traversal rules
6. `reqvire model --mmd` generates pure Mermaid flowchart text without a Markdown wrapper
7. Relations contain full target element details recursively

##### Test Criteria
1. **Default Model Output (Model Roots)**
   Command: `reqvire model --json`
   - exits code **0**
   - output parses as valid JSON
   - JSON contains `elements` array with model roots at top level
   - JSON contains `metadata` with total_elements, total_relations, filtered_from (null)
   - Only elements selected by default root traversal appear at top level
   - Nested relations contain full element details recursively

2. **Filtered Model Output (From Specific Element)**
   Command: `reqvire model --from=<test-element-name> --json`
   - exits code **0**
   - output parses as valid JSON
   - JSON elements array contains specified element at top level
   - metadata.filtered_from contains element name
   - Only forward-related elements appear in nested structure

3. **Markdown Output with Mermaid Diagrams**
   Command: `reqvire model`
   - exits code **0**
   - output contains metadata (Total Elements, Total Relations)
   - output contains Mermaid diagram blocks showing all nested relations
   - diagrams use hash identifiers for node IDs

4. **Pure Mermaid Output**
   Command: `reqvire model --mmd`
   - exits code **0**
   - output begins with Mermaid graph syntax
   - output does not contain Markdown fenced code blocks
   - output includes ontology, capability, requirement, and attachment edges when present in the model context

5. **Nested JSON Structure Validation**
   Command: `reqvire model --json`
   - JSON has keys: `elements`, `metadata`
   - Each element has: `identifier`, `name`, `element_type`, `file_path`, `section`, `section_index`, `relations`, `attachments`
   - Each relation has: `relation_type`, target (element/file/external)
   - Element targets are nested recursively with same structure
   - File targets have: `path`, `type: "file"`
   - External targets have: `url`, `type: "external"`
   - Attachments is an array of refinement element identifier strings (empty array if no attachments)
   - Metadata has: `total_elements`, `total_relations`, `filtered_from`

5. **Forward-Only Traversal Verification**
   - Create test with element A that derives B, and B derives C
   - Running `reqvire model --from=<A-name>` includes B and C nested in relations
   - Create element D that is derived from B (backward relation)
   - Running `reqvire model --from=<A-name>` includes B and C but NOT D
   - Confirms only forward relations (derive, satisfiedBy, verifiedBy, trace) are followed

6. **Cycle Detection Verification**
   - System prevents infinite recursion when cycles exist in forward relations
   - Nested structure handles circular dependencies gracefully
   - Each element appears at most once in traversal

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-model-command/test.sh)
  * verify: [Forward-Only Relation Traversal](../../../Reports/ModelReports/ReportingRequirements.md#forward-only-relation-traversal)
  * verify: [Model Diagram Output Formats](../../../Reports/ModelReports/ReportingRequirements.md#model-diagram-output-formats)
---

### Model Containment Report Integration Test

This test verifies that containment report generation exposes deterministic folder, file, and element hierarchy data for the Explorer model view.

#### Details

##### Test Criteria
1. **Report generation:**
   - Run `reqvire containment` command.
   - Verify containment output includes folders, files, elements, and design-document containers.
   - Verify generated Mermaid containment output is deterministic.

2. **Navigation integration:**
   - Verify containment paths are stable and scoped to the current workspace.
   - Verify file and folder entries preserve their hierarchical parent/child relationships.

3. **Integration with model:**
   - Test containment output updates when the model changes.
   - Verify new elements appear in the correct containing file.
   - Verify deterministic output for version control.

##### Acceptance Criteria
- Containment output is generated correctly.
- Folder, file, and element hierarchy is deterministic.
- Integration updates correctly when model changes.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-containment-view/test.sh)
  * verify: [Containment View Report](../../../Reports/ModelReports/ReportingRequirements.md#containment-view-report)
---

### Multi-Type Search Filter Test

This test verifies that the search command correctly filters by multiple element types using comma-separated values.

#### Details

##### Test Steps
1. Search with single type (baseline - existing behavior)
2. Search with two comma-separated types
3. Search with three or more types
4. Search with custom type in list
5. Search with invalid type in list (expect error)
6. Search with multiple types combined with other filters (name, file, etc.)
7. Verify JSON output with multiple types

##### Expected Results
- Single type queries work as before (backward compatibility)
- Multi-type queries return elements matching ANY specified type
- Invalid types produce clear error messages
- Comma-separated parsing handles whitespace correctly
- Combined filters work correctly
- JSON output is valid

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-multi-type-search/test.sh)
  * verify: [Comma-Separated Type Filter Parsing](../../../Reports/ModelReports/ReportingRequirements.md#comma-separated-type-filter-parsing)
---

### Resources Report Verification

This test verifies that the resources command correctly generates reports showing file-based relations and identifier-based attachments.

#### Details

##### Acceptance Criteria
**Report Structure:**
- Report shall have two main sections: Relations and Attachments
- Relations section lists files alphabetically by path
- Attachments section lists refinement identifiers alphabetically
- Each entry shows referencing elements with markdown links

**Relations Section:**
- Includes files from InternalPath relation targets (satisfiedBy, trace, etc.)
- Each reference shows relation type and source element
- References sorted by relation type, then by element identifier

**Attachments Section:**
- Includes refinement identifiers from attachment targets
- Each reference shows source element
- References sorted by element identifier

**Output Formats:**
- Text output uses markdown formatting with headers and bullet lists
- JSON output provides structured data with relations, attachments, and summary

**Explorer Integration:**
- Resources view is available as a supporting SPA route in the served Explorer
- Supporting report routes are reachable through canonical route links or source/report affordances, not a shared top header

##### Test Criteria
1. **Basic text output**
   Command: `reqvire resources`
   - exits code **0**
   - output contains "## Relations" section header
   - output contains "## Attachments" section header
   - files are listed with ### headers
   - referencing elements shown as bullet points with markdown links

2. **JSON output structure**
   Command: `reqvire resources --json`
   - exits code **0**
   - output parses under `jq`
   - contains `relations` array with file_path and references
   - contains `attachments` array with identifier and references
   - contains `summary` with totals

3. **Relations section content**
   - satisfiedBy relations to code files appear in Relations section
   - trace relations to document files appear in Relations section
   - each reference includes relation_type, element_id, element_name

4. **Attachments section content**
   - refinement identifier attachments appear in Attachments section
   - each reference includes element_id, element_name

5. **Sorting verification**
   - Relation files sorted alphabetically by path
   - Attachment identifiers sorted alphabetically by identifier
   - Within each entry, references sorted by relation_type then element_id
   - Consistent ordering across multiple runs

6. **Empty sections handling**
   - If no InternalPath relations exist, Relations section shows appropriate message
   - If no attachment identifiers exist, Attachments section shows appropriate message

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-resources-report/test.sh)
  * verify: [CLI Resources Command](../../../Interfaces/CLI/Commands.md#cli-resources-command)
  * verify: [Resources Report](../../../Reports/ModelReports/ReportingRequirements.md#resources-report)
---

### Reverse Model Traversal Test

Test verifies reverse traversal output against golden files for both JSON and Markdown modes.

#### Details
Test cases:
1. Run `reqvire model --reverse --json`; assert command success and valid JSON output.
2. Compare JSON output to `expected/expected_reverse_output.json`.
3. Run `reqvire model --reverse`; assert command success.
4. Compare Markdown output to `expected/expected_reverse_output.md`.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-model-command/test.sh)
  * verify: [Reverse Relation Traversal](../../../Reports/ModelReports/ReportingRequirements.md#reverse-relation-traversal)
---

### Search Command Tests

This test verifies that the system provides a unified `search` command functionality for searching and filtering model elements with comprehensive filter options and output modes.

#### Details

##### Acceptance Criteria
- Running `reqvire search --json` produces a valid, pretty-printed JSON search result
- Running `reqvire search` (no `--json`) prints a human-readable text search result
- Running `reqvire search --short` produces abbreviated text output (one-line per element format: `[type] identifier - name`)
- Running `reqvire search --short --json` produces abbreviated JSON output (omits specified fields)
- Both JSON and text outputs include all elements matching filter criteria
- All filter flags work individually and in combination (conjunctive AND logic)
- Supplying an invalid regex to any regex-based filter fails with a non-zero exit code and displays a clear error message
- Search results must include all relations for each element
- Search results must include all attachments for each element (omitted in short mode)
- Search JSON results must include effective governance metadata for governance-bearing elements
- Search JSON and text summaries must include effective governance metadata counters for matched governance-bearing elements
- **Enhanced Content Display**: Search must display page content (frontmatter before first element) when not in short mode
- **Count Information**: Search must show counts for files and elements in full mode (omitted in short mode)
- **Short Mode Behavior**: Short mode omits: `content`, `page_content`, `verified_relations_count`, `satisfied_relations_count`, `element_count`, `total_elements`, `global_counters`, `attachments`

##### Test Criteria
1. **Base JSON search**
   Command: `reqvire search --json`
   - exits code **0**
   - output parses under `jq`
   - `.files` contains file path keys
   - All elements included when no filters specified

2. **Base text search**
   Command: `reqvire search`
   - exits code **0**
   - human-readable output with hierarchical structure
   - each element block includes identifier, name, file, type, content
   - relations displayed for each element

3. **Short mode text output**
   Command: `reqvire search --short`
   - exits code **0**
   - one line per element: `[type] identifier - name`
   - no content or page content displayed
   - no count information displayed

4. **Short mode JSON output**
   Command: `reqvire search --short --json`
   - exits code **0**
   - output parses as valid JSON
   - element objects do NOT contain: `content`, `verified_relations_count`, `satisfied_relations_count`
   - file objects do NOT contain: `page_content`, `total_elements`
   - top level does NOT contain: `global_counters`

5. **Individual filters**
   For each flag in turn, run both JSON and text modes:
   - `--filter-file="**/*Reqs.md"` (glob)
   - `--filter-name=".*safety.*"` (regex)
   - `--filter-type="capability"` (exact)
   - `--filter-type="constraint"` (Refinement type)
   - `--filter-type="behavior"` (Refinement type)
   - `--filter-type="specification"` (Refinement type)
   - `--filter-status=approved` (effective governance status)
   - `--filter-priority=high,critical` (effective governance priority)
   - `--filter-risk=high,critical` (effective governance risk)
   - `--filter-owner="Platform.*"` (effective governance owner regex)
   - `--filter-content="MUST"` (regex)
   - `--filter-page-content="architecture"` (regex)
   - `--have-relations=verifiedBy` (comma-separated)
   - `--not-have-relations=verifiedBy` (comma-separated)
   Assert for each:
   - exit code **0**
   - only elements matching the filter appear in output

6. **Filter combinations**
   Combine multiple filters and verify outputs contain exactly those elements passing ALL filters:
   - `--filter-type=requirement --have-relations=verifiedBy,satisfiedBy`
   - `--filter-file="System*" --filter-name=".*GPS.*"`
   - `--filter-content="MUST" --not-have-relations=verifiedBy`

7. **Invalid regex**
   Command: `reqvire search --json --filter-name="***"`
   - exits non-zero
   - stderr contains error message with faulty pattern

8. **Invalid relation type**
   Command: `reqvire search --have-relations=invalidRelationType`
   - exits non-zero
   - stderr contains error with list of valid relation types

9. **Multiple relations in have-relations**
   Command: `reqvire search --have-relations=verifiedBy,satisfiedBy`
   - exits code **0**
   - only elements that have BOTH verifiedBy AND satisfiedBy relations appear
   - elements with only one of the relations are excluded

10. **Multiple relations in not-have-relations**
    Command: `reqvire search --not-have-relations=verifiedBy,satisfiedBy`
    - exits code **0**
    - only elements that do NOT have ALL specified relations appear
    - if element lacks verifiedBy OR satisfiedBy (or both), it is included

11. **Page content filter**
    Command: `reqvire search --filter-page-content="architecture"`
    - exits code **0**
    - only elements whose parent file page content matches regex appear
    - elements in files without matching page content are excluded

12. **Relations coverage**
    Command: `reqvire search --json`
    - Both JSON and text outputs must show complete relationship information
    - All relation types and targets included

13. **Enhanced content and counts verification (full mode)**
    Command: `reqvire search --json`
    - JSON output must include `page_content` field for files that have frontmatter content
    - JSON output must include count fields in global counters
    - JSON output must include per-file counts

14. **Short mode field omission verification**
    Command: `reqvire search --short --json`
    - Verify all specified fields are omitted from JSON structure
    - Verify no null/empty placeholders for omitted fields (fields completely absent)

16. **Attachments in search output (full mode)**
    Command: `reqvire search --json`
    - JSON output must include `attachments` field for each element
    - Attachments is an array of refinement element identifier strings
    - Element identifier attachments displayed as full identifiers (e.g., `"specifications/File.md#refinement-element"`)
    - Elements without attachments have empty array `[]`
    - Attachment identifiers are normalized to git-root-relative identifier format

17. **Attachments omitted in short mode**
    Command: `reqvire search --short --json`
    - Element objects do NOT contain `attachments` field
    - Field is completely absent (not empty array)

18. **Governance metadata in search output and filters**
    Command: `reqvire search --json`
    - Governance-bearing element objects include effective governance metadata with value and source information
    - Inherited governance metadata entries include `source_identifier` for the requirement ancestor that supplied the value
    - Explicit and default governance metadata entries omit `source_identifier`
    - `--filter-status`, `--filter-priority`, and `--filter-risk` match effective enum values
    - `--filter-owner` matches effective owner values by regex
    - Governance metadata filters exclude non-governance-bearing elements
    - Invalid governance metadata filter values fail with clear accepted-value diagnostics
    - JSON summary includes governance metadata counters for status, priority, risk, and owner
    - Text summary includes equivalent governance metadata counters
    - Unassigned owner is summarized as `unassigned`

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-search-all-capabilities/test.sh)
  * verify: [CLI Search Command](../../../Interfaces/CLI/Commands.md#cli-search-command)
  * verify: [Search Report Generator](../../../Reports/ModelReports/ReportingRequirements.md#search-report-generator)
---

### Semantic Contract Search JSON Test

Test verifies that search JSON exposes semantic-contract ADT content and derived semantic-contract identity.

#### Details
Test cases:
- `search --filter-type=semantic-contract --json` returns semantic-contract elements.
- JSON includes `semantic_contract.iri`.
- JSON includes `ontology.ontology.language`, content, and fenced block line number.
- JSON includes `semantic_contract.shapes.language` and content when Shapes exists.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-semantic-contract-json/test.sh)
  * verify: [CLI Search Command](../../../Interfaces/CLI/Commands.md#cli-search-command)
  * verify: [Search Report Generator](../../../Reports/ModelReports/ReportingRequirements.md#search-report-generator)
---

### Start Type Filter Test

Test verifies model start-element type filtering using JSON/Markdown assertions and expected fixtures.

#### Details
Test cases:
1. Run `reqvire model --filter-type=test-verification --json`; assert success and valid JSON.
2. Assert `metadata.type_filter` contains `test-verification`.
3. Assert all top-level `.elements[].element_type` values are `test-verification`.
4. Run `reqvire model --reverse --filter-type=test-verification --json`; assert success and compare with `expected/expected_reverse_filter_output.json`.
5. Run `reqvire model --reverse --filter-type=test-verification`; assert success and compare with `expected/expected_reverse_filter_output.md`.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-model-command/test.sh)
  * verify: [Start Element Type Filtering](../../../Reports/ModelReports/ReportingRequirements.md#start-element-type-filtering)
---

### Submodels Report Verification

This test verifies that the `submodels` command reports independent capability-rooted submodels and cross-submodel requirement couplings with deterministic output.

#### Details

##### Acceptance Criteria
- System shall provide CLI command `submodels`
- Command shall support `--from <NAME>` to scope report to one capability or requirement subtree
- Command shall support `--json` flag for JSON output format
- Text output shall include:
  - discovered capability-rooted submodels
  - per-submodel requirement counts
  - cross-submodel requirement coupling list
  - summary totals
- JSON output shall include `submodels`, `cross_submodel_couplings`, and `summary`
- Output ordering shall be deterministic across runs

##### Test Criteria
1. **Basic text output**
   Command: `reqvire submodels`
   - exits code **0**
   - output contains `## Submodels`
   - output contains `## Cross-Submodel Couplings`
   - output contains `## Summary`
   - summary block includes `**Submodels:**`, `**Requirements:**`, and `**Cross-Submodel Couplings:**`
   - submodel roots are capability elements, not requirement roots
   - summary counts match fixture totals
   - output matches expected fixture

2. **JSON output structure**
   Command: `reqvire submodels --json`
   - exits code **0**
   - output parses under `jq`
   - contains `submodels` array with root metadata and requirement counts
   - full-report `submodels` entries use capability roots
   - contains `cross_submodel_couplings` array with source/target and relation type fields
   - summary fields match fixture totals:
     - `summary.total_submodels`
     - `summary.total_requirements`
     - `summary.total_cross_submodel_couplings`
   - output matches expected fixture

3. **Capability filter output**
   Command: `reqvire submodels --from "Capability One"`
   - exits code **0**
   - selected capability appears as the scoped capability submodel entry
   - requirement count includes requirements that specify the capability and their descendants
   - output contains only submodel data and couplings relevant to selected capability scope
   - output matches expected filtered fixture

4. **Capability filter JSON output**
   Command: `reqvire submodels --from "Capability One" --json`
   - exits code **0**
   - output parses under `jq`
   - selected capability appears in `submodels` array
   - `summary.total_requirements` and `summary.total_cross_submodel_couplings` are derived from selected capability scope
   - output matches expected filtered JSON fixture

5. **Requirement filter output**
   Command: `reqvire submodels --from "Root One"`
   - exits code **0**
   - selected scope is not listed as a submodel entry
   - when selected scope has multiple first-level child branches, each branch root is listed as a scoped submodel entry
   - output contains only submodels discovered within selected scope
   - summary `Submodels` count matches filtered-scope submodels only
   - summary `Requirements` and `Cross-Submodel Couplings` counts reflect selected scope only
   - output matches expected filtered fixture

6. **Requirement filter JSON output**
   Command: `reqvire submodels --from "Root One" --json`
   - exits code **0**
   - output parses under `jq`
   - selected scope does not appear in `submodels` array
   - `submodels` includes one entry per first-level scoped branch root under the selected scope
   - `submodels` contains only filtered-scope submodels
   - `summary.total_requirements` and `summary.total_cross_submodel_couplings` are derived from selected-scope couplings
   - output matches expected filtered JSON fixture

7. **Root filter missing root error**
   Command: `reqvire submodels --from "Missing Root"`
   - exits non-zero
   - error message indicates selected scope was not found

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-submodels-command/test.sh)
  * verify: [CLI Submodels Command](../../../Interfaces/CLI/Commands.md#cli-submodels-command)
  * verify: [Requirement Submodels Report](../../../Reports/ModelReports/ReportingRequirements.md#requirement-submodels-report)
---

### Verification Coverage Report Test

This test verifies that the system correctly generates verification coverage reports focusing on leaf requirements, showing the percentage and details of satisfied and unsatisfied evidence-backed verification elements, and identifying orphaned verifications.

#### Details

##### Acceptance Criteria
- System shall provide a CLI command `coverage` that generates coverage reports focusing on leaf requirements
- Command shall support `--json` flag for JSON output format
- Coverage report shall include summary section with total counts and percentages for leaf requirements
- Coverage report shall show breakdown by verification type (test, formal-proof, analysis, inspection, demonstration)
- Coverage report shall list verified leaf requirements grouped by file and section
- Coverage report shall list unverified leaf requirements with details
- Coverage report shall list satisfied evidence-backed verifications (`test-verification` and `formal-proof-verification`) with `satisfiedBy` relations
- Coverage report shall list unsatisfied evidence-backed verifications (`test-verification` and `formal-proof-verification`) without `satisfiedBy` relations
- Coverage report shall list orphaned verifications (verifications without any verify relations to capabilities or requirements)
- Coverage report shall show orphaned verifications count and percentage in summary section
- Non-evidence-backed verification elements (analysis, inspection, demonstration) are considered satisfied by default (no satisfiedBy required)
- JSON output shall be valid and machine-readable
- Text output shall be human-readable with clear formatting

##### Test Criteria
1. **Basic Coverage Report**
   Command: `reqvire coverage-report`
   - exits code **0**
   - output contains `=== Verification Coverage Report ===`
   - output contains `Summary:` section with leaf requirements counts and percentages
   - output contains `Verification Types:` breakdown
   - output contains coverage percentage calculation for leaf requirements
   - verified leaf requirements are marked with ✅
   - unverified leaf requirements are marked with ❌
   - satisfied evidence-backed verification elements are marked with ✅
   - unsatisfied evidence-backed verification elements are marked with ❌

2. **JSON Coverage Report**
   Command: `reqvire coverage-report --json`
   - exits code **0**
   - output parses as valid JSON
   - JSON contains `summary` object with leaf requirements counts and percentages
   - JSON contains `verified_leaf_requirements` and `unverified_leaf_requirements` sections
   - JSON contains `satisfied_test_verifications` and `unsatisfied_test_verifications` sections
   - verification details include identifier, name, section, type, and satisfied_by relations for evidence-backed verification types

3. **Coverage Calculation**
   - Leaf requirements coverage percentage calculated as (verified_leaf_requirements/total_leaf_requirements * 100)
   - Evidence-backed verification satisfaction percentage calculated as (satisfied_test_verifications/total_test_verifications * 100), where the existing JSON field includes both test and formal-proof verifications
   - Verification types correctly categorized
   - Test-verification and formal-proof-verification elements without satisfiedBy relations are flagged as unsatisfied
   - Test-verification and formal-proof-verification elements with valid satisfiedBy relations are considered satisfied
   - Analysis, inspection, and demonstration verifications are considered satisfied by default (no satisfiedBy evaluation)

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-coverage-report/test.sh)
  * verify: [CLI Coverage Command](../../../Interfaces/CLI/Commands.md#cli-coverage-command)
---

### Verification Traces Filter Options Test

This test verifies that the verification-traces command filter options work correctly when generating upward trace trees from verification elements to owning capability roots.

#### Details

##### Acceptance Criteria
- System shall provide CLI command `traces` that generates upward trace trees from verifications
- Command shall output to stdout in Markdown format with embedded Mermaid diagrams by default
- Command shall support `--json` flag for structured JSON output without diagrams
- Mermaid diagrams shall show verification element as root with arrows following relation semantics
- Mermaid diagrams shall include clickable links on all nodes (verifications and requirements)
- Directly verified requirements shall be marked/highlighted in diagrams using CSS classes
- System shall traverse all upward parent relations to reach owning capability roots
- System shall merge multiple verification paths into single tree per verification
- System shall support `--filter-id=<id>` filter for specific verification element
- System shall support `--filter-name=<regex>` for filtering by verification name pattern
- System shall support `--filter-type=<type>` for filtering by verification type
- Multiple filters shall be combinable using AND logic
- JSON output shall include verification ID, directly verified capabilities or requirements, and complete trace tree structure

##### Test Criteria
1. **Basic Markdown Output**
   Command: `reqvire verification-traces`
   - exits code **0**
   - output contains `# Verification Traceability Report`
   - output contains Mermaid diagram blocks with `graph BT`
   - diagrams include verification element nodes and requirement nodes
   - diagrams include click handlers for all nodes (format: `click NODE_ID "url"`)
   - directly verified capabilities or requirements have `:::verified` CSS class in diagram

2. **JSON Output**
   Command: `reqvire verification-traces --json`
   - exits code **0**
   - output parses as valid JSON
   - JSON contains `verifications` array
   - each verification includes `verification_id`, `verification_name`, `verification_type`
   - each verification includes `directly_verified_requirements` array
   - each verification includes `trace_tree` with nested requirement structure

3. **Correct Arrow Directions**
   - Mermaid diagrams use `SYS001 -.->|verify| VER001` or `CAP001 -.->|verify| VER001` format (verified element links to verification)
   - Mermaid diagrams use `USER001 -.->|deriveReqT| SYS001` format (parent derives child)
   - Arrow directions match Reqvire relation semantics (TargetToElement, ElementToTarget)

4. **Specific Verification Filter**
   Command: `reqvire verification-traces --filter-id="specifications/Verifications/ValidationTests.md#invalid-relations-test"`
   - exits code **0**
   - output contains only trace for specified verification
   - other verifications are excluded

5. **Name Pattern Filter**
   Command: `reqvire verification-traces --filter-name=".*Coverage.*"`
   - exits code **0**
   - output contains only verifications matching regex pattern
   - non-matching verifications are excluded

6. **Type Filter**
   Command: `reqvire verification-traces --filter-type="test-verification"`
   - exits code **0**
   - output contains only test-verification elements
   - analysis, inspection, demonstration, and formal proof verifications are excluded

7. **Combined Filters**
   Command: `reqvire verification-traces --filter-type="test-verification" --filter-name=".*Test"`
   - exits code **0**
   - output contains only verifications matching ALL filter criteria (AND logic)
   - verifications matching only one filter are excluded

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-verification-traces/test.sh)
  * verify: [CLI Traces Command](../../../Interfaces/CLI/Commands.md#cli-traces-command)
---

### Verification Traces From-Folder Test

This test verifies that the --from-folder option correctly generates relative links in verification traces output when the output file will be saved in a specific folder location.

#### Details

##### Acceptance Criteria
- System shall provide `--from-folder=<path>` option for `traces` command
- Option shall accept a relative path to the folder where output will be saved
- When `diagrams_with_blobs` is false (default), generated Mermaid diagram links shall be relative to the specified folder
- When `diagrams_with_blobs` is true with Git info, links shall remain as GitHub blob URLs (absolute)
- Links shall be correctly calculated so they work when output file is saved in the from-folder location
- Option shall work with both Markdown and JSON output formats
- Option shall work in combination with filter options

##### Test Criteria
1. **Basic From-Folder Option**
   Command: `reqvire traces --from-folder=docs/reports`
   - exits code **0**
   - output contains Mermaid diagrams with click handlers
   - click handler links are relative paths calculated from `docs/reports/` to git root
   - example: if element identifier is `specifications/file.md#element`, link should be `../../specifications/file.md#element`

2. **From-Folder with Current Directory**
   Command: `reqvire traces --from-folder=.`
   - exits code **0**
   - links are relative to current directory (git root)
   - same as omitting --from-folder option

3. **From-Folder with Nested Path**
   Command: `reqvire traces --from-folder=output/verification/traces`
   - exits code **0**
   - links correctly navigate up three levels then to specifications
   - example: `../../specifications/file.md#element` becomes `../../../specifications/file.md#element`

4. **From-Folder with JSON Output**
   Command: `reqvire traces --from-folder=docs/reports --json`
   - exits code **0**
   - JSON output parses correctly
   - JSON element identifiers remain absolute (from git root)
   - from-folder only affects Markdown diagram links, not JSON structure

5. **From-Folder Combined with Filters**
   Command: `reqvire traces --from-folder=docs/reports --filter-type=test-verification`
   - exits code **0**
   - filtering works correctly
   - generated links still relative to `docs/reports/`

6. **From-Folder with Git Blobs Enabled**
   Environment: `diagrams_with_blobs=true` in config
   Command: `reqvire traces --from-folder=docs/reports`
   - exits code **0**
   - links remain as GitHub blob URLs (absolute)
   - from-folder has no effect on external GitHub links

7. **From-Folder Path Calculation Correctness**
   - For from-folder `a/b/c` and identifier `specs/req.md#id`:
     - Link should be `../../../specs/req.md#id`
   - For from-folder `output` and identifier `specifications/Requirements.md#element`:
     - Link should be `../specifications/Requirements.md#element`
   - Path traversal (..) count matches folder depth

8. **From-Folder Special Case for Root**
   Command: `reqvire traces --from-folder=/`
   - exits code **0**
   - identifiers remain as git-root-relative paths (no relative path calculation)
   - links use identifiers as-is (e.g., `specifications/file.md#element`)
   - special case `/` indicates reqvire root (git root)

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-verification-traces/test.sh)
  * verify: [CLI Traces Command](../../../Interfaces/CLI/Commands.md#cli-traces-command)
---
