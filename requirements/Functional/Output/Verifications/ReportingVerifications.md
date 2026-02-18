# Elements

### CLI Collect Command Test

This test verifies that the collect command aggregates content from a requirement chain with proper source citations.

#### Details

##### Acceptance Criteria
- System shall provide CLI command `collect` that aggregates content from requirement chains
- Command shall accept requirement element name as positional argument
- Command shall support `--json` flag for JSON output format
- Command shall support `--direction` flag with values UPSTREAM (default) and DOWNSTREAM
- When direction is UPSTREAM, command shall traverse derivedFrom relations in reverse direction (child to parents)
- When direction is DOWNSTREAM, command shall traverse derive relations in forward direction (parent to children)
- Command shall collect element content and attachment contents
- Command shall output with source citations
- Command shall reject non-requirement element types with error

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
   - .md file attachments: content read and included
   - other file types: included as markdown link
   - element identifier attachments: referenced element content included

5. **Error Handling - Element Not Found**
   Command: `reqvire collect non-existent-element`
   - exits non-zero
   - error message indicates element not found

6. **Error Handling - Non-Requirement Type**
   Command: `reqvire collect <verification-name>`
   - exits non-zero
   - error message indicates element must be requirement type

7. **Output Ordering**
   - Flat list structure
   - Ancestors first (depth 0 = root)
   - Same-depth elements sorted alphabetically by name

8. **Downstream Text Output**
   Command: `reqvire collect <root-requirement-name> --direction DOWNSTREAM`
   - exits code **0**
   - output contains content from starting requirement (depth 0)
   - output contains content from child requirements
   - each content block followed by source citation
   - starting element appears first, children appear after

9. **Downstream JSON Output**
   Command: `reqvire collect <root-requirement-name> --direction DOWNSTREAM --json`
   - exits code **0**
   - output parses as valid JSON
   - JSON contains `direction` field with value `downstream`
   - JSON items ordered: starting element at depth 0, children at depth 1, etc.
   - each item has correct depth reflecting distance from starting element

10. **Downstream Descendant Chain Collection**
    - Starting from root requirement
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

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-json-file-output/test.sh)
  * verify: [CLI JSON File Output Option](../../../Interfaces/CLI/Commands.md#cli-json-file-output-option)
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
   - Test with multiple element types (requirement, verification, user-requirement)

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
  * verify: [Containment View Report](../Reporting.md#containment-view-report)
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
---

### Containment View Mermaid Diagram Test

This test verifies that the system generates valid Mermaid flowchart diagrams with correct syntax, nested subgraphs, element styling, and clickable links.

#### Details

##### Test Criteria
1. **Mermaid syntax validation:**
   - Verify output starts with `flowchart TD`
   - Verify all subgraphs use correct syntax: `subgraph ID ["Label"]`
   - Verify subgraphs are properly closed with `end`
   - Test diagram can be rendered by Mermaid parser
   - Validate with mermaid-cli or online editor

2. **Subgraph structure:**
   - Verify folders use subgraph with 📁 prefix
   - Verify files use subgraph with 📄 prefix
   - Verify subgraphs are properly nested
   - Test nested structure: folder → subfolder → file
   - Verify `direction TB` is set for nested subgraphs

3. **Element nodes:**
   - Verify nodes use 16-character hash IDs
   - Verify node labels show element names
   - Verify hash IDs are unique across diagram
   - Test hash ID generation is deterministic
   - Verify nodes are placed within file subgraphs

4. **Styling:**
   - Verify `class` directives for element types
   - Test `userRequirement` class has correct colors
   - Test `systemRequirement` class has correct colors
   - Test `verification` class has correct colors
   - Test `default` class for other types
   - Verify CSS class definitions are included

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
---

### Containment View Text Output Test

This test verifies that the system generates correctly formatted human-readable text output for the containment view with proper indentation and element metadata display.

#### Details

##### Test Criteria
1. **Hierarchical indentation:**
   - Verify indentation uses 2 spaces per level
   - Test nested structure: root (0), folder (2), subfolder (4), file (6), element (8)
   - Verify consistent indentation across all levels

2. **Visual markers:**
   - Verify folders display with `📁 Folder: <name>`
   - Verify files display with `📄 File: <path>`
   - Verify elements display with `[<type>] <name>`
   - Test all element types have correct bracket notation

3. **Element type display:**
   - Test `[requirement]` for system requirements
   - Test `[user-requirement]` for user requirements
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
- Visual markers (📁, 📄) are displayed correctly
- Element types are shown in brackets
- All hierarchy levels are represented
- Output is human-readable and well-formatted

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-containment-view/test.sh)
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
- Standard types (requirement, user-requirement, verification types) SHALL NOT be counted as custom
- File-type elements SHALL NOT be counted as custom types

##### Test Criteria
1. **Custom Types in Text Output**
   Command: `reqvire summary` (on test data with custom types)
   - exits code **0**
   - output contains `📋 Element Types:` section
   - custom types appear after standard types
   - format: `Custom (type-name): count`
   - custom types are sorted alphabetically
   - standard types (System Requirements, User Requirements, Verifications) appear first

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
   - Verify requirements with type `user-requirement` are NOT in custom_element_types
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

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-search-all-features/test.sh)
---

### HTML Export Containment View Integration Test

This test verifies that the containment view is correctly integrated into HTML export with proper navigation, interactive features, and styling.

#### Details

##### Test Criteria
1. **File generation:**
   - Run `reqvire export` command
   - Verify `containment.html` file is created in output directory
   - Verify file contains valid HTML5
   - Test file size is reasonable (< 1MB for typical models)

2. **Navigation integration:**
   - Verify containment view appears in navigation menu
   - Test link text is "Containment View" or "Model Structure"
   - Verify clicking nav link loads containment page
   - Test navigation persistence across page loads

3. **Mermaid diagram embedding:**
   - Verify page contains embedded Mermaid diagram
   - Verify Mermaid.js library is loaded
   - Test diagram renders correctly in browser
   - Verify clickable links work (navigate to element pages)

4. **Hierarchical tree navigation:**
   - Verify page includes tree view component
   - Test folders can be expanded/collapsed
   - Verify clicking elements navigates to their pages
   - Test tree state persistence

5. **Styling consistency:**
   - Verify page uses same CSS as other export pages
   - Test element type colors match specification
   - Verify layout matches existing pages
   - Test responsive design on different screen sizes

6. **Interactive features:**
   - Test element type filtering controls
   - Verify filter updates both diagram and tree
   - Test search/filter by folder or file name
   - Verify search highlights matches

7. **Integration with model:**
   - Test containment view updates when model changes
   - Verify new elements appear after re-export
   - Test moved elements show in correct location
   - Verify deterministic output for version control

##### Acceptance Criteria
- containment.html file is generated correctly
- Page appears in navigation menu
- Mermaid diagram renders and is interactive
- Tree navigation works correctly
- Styling is consistent with existing pages
- Interactive features (filter, search) function properly
- Integration updates correctly when model changes

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-containment-view/test.sh)
---

### Model Command Verification

Comprehensive test verifying model command generates model-centric nested output in different modes.

#### Details

##### Acceptance Criteria
1. `reqvire model` generates model-centric output showing root requirements with nested relations
2. `reqvire model --from=<name>` generates nested structure starting from specified element
3. `reqvire model --json` generates valid JSON with nested element structure
4. `reqvire model --from=<name> --json` generates filtered JSON from specified starting point
5. Default mode filters to root requirements (no hierarchical parent relations)
6. Relations contain full target element details recursively

##### Test Criteria
1. **Default Model Output (Root Requirements)**
   Command: `reqvire model --json`
   - exits code **0**
   - output parses as valid JSON
   - JSON contains `elements` array with root requirements only at top level
   - JSON contains `metadata` with total_elements, total_relations, filtered_from (null)
   - Only requirements without hierarchical parent relations at top level
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
   - output contains `# Model Structure`
   - output contains metadata (Total Elements, Total Relations)
   - output contains Mermaid diagram blocks showing all nested relations
   - diagrams use hash identifiers for node IDs

4. **Nested JSON Structure Validation**
   Command: `reqvire model --json`
   - JSON has keys: `elements`, `metadata`
   - Each element has: `identifier`, `name`, `element_type`, `file_path`, `section`, `section_index`, `relations`, `attachments`
   - Each relation has: `relation_type`, target (element/file/external)
   - Element targets are nested recursively with same structure
   - File targets have: `path`, `type: "file"`
   - External targets have: `url`, `type: "external"`
   - Attachments is an array of file path strings (empty array if no attachments)
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
  * verify: [Forward-Only Relation Traversal](../Reporting.md#forward-only-relation-traversal)
  * verify: [Model Diagram Output Formats](../Reporting.md#model-diagram-output-formats)
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
  * verify: [Comma-Separated Type Filter Parsing](../Reporting.md#comma-separated-type-filter-parsing)
---

### Resources Report Verification

This test verifies that the resources command correctly generates reports showing all files referenced by the model through relations and attachments.

#### Details

##### Acceptance Criteria
**Report Structure:**
- Report shall have two main sections: Relations and Attachments
- Each section lists files alphabetically by path
- Each file shows referencing elements with markdown links

**Relations Section:**
- Includes files from InternalPath relation targets (satisfiedBy, trace, etc.)
- Each reference shows relation type and source element
- References sorted by relation type, then by element identifier

**Attachments Section:**
- Includes files from FilePath attachment targets
- Each reference shows source element
- References sorted by element identifier

**Output Formats:**
- Text output uses markdown formatting with headers and bullet lists
- JSON output provides structured data with relations, attachments, and summary

**HTML Export Integration:**
- Resources view available in HTML export
- Navigation link appears in header alongside other views

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
   - contains `attachments` array with file_path and references
   - contains `summary` with totals

3. **Relations section content**
   - satisfiedBy relations to code files appear in Relations section
   - trace relations to document files appear in Relations section
   - each reference includes relation_type, element_id, element_name

4. **Attachments section content**
   - FilePath attachments appear in Attachments section
   - ElementIdentifier attachments do NOT appear (they reference model elements, not files)
   - each reference includes element_id, element_name

5. **Sorting verification**
   - Files sorted alphabetically by path
   - Within each file, references sorted by relation_type then element_id
   - Consistent ordering across multiple runs

6. **HTML export integration**
   Command: `reqvire export --output=/tmp/test-export`
   - resources.html file generated
   - Navigation header includes Resources link
   - Resources view contains same content as CLI text output

7. **Empty sections handling**
   - If no InternalPath relations exist, Relations section shows appropriate message
   - If no FilePath attachments exist, Attachments section shows appropriate message

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-resources-report/test.sh)
  * verify: [Resources Report](../Reporting.md#resources-report)
  * verify: [CLI Resources Command](../../../Interfaces/CLI/Commands.md#cli-resources-command)
---

### Submodels Report Verification

This test verifies that the `submodels` command reports independent requirement hierarchies and cross-submodel requirement couplings with deterministic output.

#### Details

##### Acceptance Criteria
- System shall provide CLI command `submodels`
- Command shall support `--from <NAME>` to scope report to one requirement subtree
- Command shall support `--json` flag for JSON output format
- Text output shall include:
  - discovered submodels grouped by top root
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
   - output matches expected fixture

2. **JSON output structure**
   Command: `reqvire submodels --json`
   - exits code **0**
   - output parses under `jq`
   - contains `submodels` array with root metadata and requirement counts
   - contains `cross_submodel_couplings` array with source/target and relation type fields
   - contains `summary` with total counts
   - output matches expected fixture

3. **Root filter output**
   Command: `reqvire submodels --from "Root One"`
   - exits code **0**
   - selected scope is not listed as a submodel entry
   - when selected scope has multiple first-level child branches, each branch root is listed as a scoped submodel entry
   - output contains only submodels discovered within selected scope
   - summary `Submodels` count matches filtered-scope submodels only
   - output matches expected filtered fixture

4. **Root filter JSON output**
   Command: `reqvire submodels --from "Root One" --json`
   - exits code **0**
   - output parses under `jq`
   - selected scope does not appear in `submodels` array
   - `submodels` includes one entry per first-level scoped branch root under the selected scope
   - `submodels` contains only filtered-scope submodels
   - output matches expected filtered JSON fixture

5. **Root filter missing root error**
   Command: `reqvire submodels --from "Missing Root"`
   - exits non-zero
   - error message indicates selected scope was not found

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-submodels-command/test.sh)
  * verify: [Requirement Submodels Report](../Reporting.md#requirement-submodels-report)
  * verify: [CLI Submodels Command](../../../Interfaces/CLI/Commands.md#cli-submodels-command)
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

#### Attachments
  * [Reverse Relation Traversal Behavior](../Behaviors.md#reverse-relation-traversal-behavior)

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-model-command/test.sh)
  * verify: [Reverse Relation Traversal](../Reporting.md#reverse-relation-traversal)
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
   - `--filter-type="user-requirement"` (exact)
   - `--filter-type="constraint"` (Refinement type)
   - `--filter-type="behavior"` (Refinement type)
   - `--filter-type="specification"` (Refinement type)
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
    - Attachments is an array of strings (file paths and element identifiers)
    - File path attachments displayed as relative paths (e.g., `"path/to/file.pdf"`)
    - Element identifier attachments displayed as full identifiers (e.g., `"specifications/File.md#refinement-element"`)
    - Elements without attachments have empty array `[]`
    - Attachment paths are relative to git root

17. **Attachments omitted in short mode**
    Command: `reqvire search --short --json`
    - Element objects do NOT contain `attachments` field
    - Field is completely absent (not empty array)

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-search-all-features/test.sh)
  * verify: [Search Report Generator](../Reporting.md#search-report-generator)
  * verify: [CLI Search Command](../../../Interfaces/CLI/Commands.md#cli-search-command)
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

#### Attachments
  * [Start Element Type Filter Behavior](../Behaviors.md#start-element-type-filter-behavior)

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-model-command/test.sh)
  * verify: [Start Element Type Filtering](../Reporting.md#start-element-type-filtering)
---

### TraceFlow View Test

This test verifies that the TraceFlow view page is correctly generated during HTML export with an interactive D3.js Sankey diagram showing verification traceability flow.

#### Details

##### Acceptance Criteria
- System shall generate `traceflow.html` file during HTML export
- TraceFlow page shall contain D3.js Sankey diagram visualization
- Sankey diagram shall show flow from user requirements to system requirements to verifications
- Navigation bar shall include "TraceFlow" link after "Traces"
- Diagram shall support pan/zoom with mouse wheel and buttons
- Diagram shall support touch pinch-zoom for mobile devices
- Clicking nodes shall navigate to element definition pages
- Page shall include title, description, and usage instructions

##### Test Criteria
1. **File Generation**
   Command: `reqvire export --output <dir>`
   - exits code **0**
   - `traceflow.html` file exists in output directory
   - file contains valid HTML5

2. **Navigation Integration**
   - All HTML files contain "TraceFlow" link in navigation bar
   - Link positioned after "Traces" and before "Coverage"
   - Link href is `traceflow.html` (with correct relative prefix)

3. **Sankey Diagram Content**
   - Page contains D3.js Sankey diagram
   - Diagram shows requirement flow (user-req → system-req → verification)
   - Nodes are color-coded by type
   - Links connect related elements

4. **Interactive Features**
   - Pan/zoom buttons (+/-/reset) are present and functional
   - Mouse wheel zoom works
   - Touch pinch-zoom works on mobile
   - Node click navigates to element page

5. **Page Content**
   - Title: "TraceFlow - Verification Traceability"
   - Description text explaining the view
   - Instructions for using the diagram

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-traceflow-view/test.sh)
  * verify: [TraceFlow View Report Generation](../Reporting.md#traceflow-view-report-generation)
---

### Verification Coverage Report Test

This test verifies that the system correctly generates verification coverage reports focusing on leaf requirements, showing the percentage and details of satisfied and unsatisfied test-verification elements, and identifying orphaned verifications.

#### Details

##### Acceptance Criteria
- System shall provide a CLI command `coverage` that generates coverage reports focusing on leaf requirements
- Command shall support `--json` flag for JSON output format
- Coverage report shall include summary section with total counts and percentages for leaf requirements
- Coverage report shall show breakdown by verification type (test, analysis, inspection, demonstration)
- Coverage report shall list verified leaf requirements grouped by file and section
- Coverage report shall list unverified leaf requirements with details
- Coverage report shall list satisfied test-verification elements (those with satisfiedBy relations)
- Coverage report shall list unsatisfied test-verification elements (those without satisfiedBy relations)
- Coverage report shall list orphaned verifications (verifications without any verify relations to requirements)
- Coverage report shall show orphaned verifications count and percentage in summary section
- Non-test-verification elements (analysis, inspection, demonstration) are considered satisfied by default (no satisfiedBy required)
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
   - satisfied test-verification elements are marked with ✅
   - unsatisfied test-verification elements are marked with ❌

2. **JSON Coverage Report**
   Command: `reqvire coverage-report --json`
   - exits code **0**
   - output parses as valid JSON
   - JSON contains `summary` object with leaf requirements counts and percentages
   - JSON contains `verified_leaf_requirements` and `unverified_leaf_requirements` sections
   - JSON contains `satisfied_test_verifications` and `unsatisfied_test_verifications` sections
   - verification details include identifier, name, section, type, and satisfied_by relations (for test-verification only)

3. **Coverage Calculation**
   - Leaf requirements coverage percentage calculated as (verified_leaf_requirements/total_leaf_requirements * 100)
   - Test-verification satisfaction percentage calculated as (satisfied_test_verifications/total_test_verifications * 100)
   - Verification types correctly categorized
   - Test-verification elements without satisfiedBy relations are flagged as unsatisfied
   - Test-verification elements with valid satisfiedBy relations are considered satisfied
   - Analysis, inspection, and demonstration verifications are considered satisfied by default (no satisfiedBy evaluation)

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-coverage-report/test.sh)
  * verify: [CLI Coverage Command](../../../Interfaces/CLI/Commands.md#cli-coverage-command)
---

### Verification Traces Filter Options Test

This test verifies that the verification-traces command filter options work correctly when generating upward trace trees from verification elements to root requirements.

#### Details

##### Acceptance Criteria
- System shall provide CLI command `traces` that generates upward trace trees from verifications
- Command shall output to stdout in Markdown format with embedded Mermaid diagrams by default
- Command shall support `--json` flag for structured JSON output without diagrams
- Mermaid diagrams shall show verification element as root with arrows following relation semantics
- Mermaid diagrams shall include clickable links on all nodes (verifications and requirements)
- Directly verified requirements shall be marked/highlighted in diagrams using CSS classes
- System shall traverse all upward parent relations to reach root requirements
- System shall merge multiple verification paths into single tree per verification
- System shall support `--filter-id=<id>` filter for specific verification element
- System shall support `--filter-name=<regex>` for filtering by verification name pattern
- System shall support `--filter-type=<type>` for filtering by verification type
- Multiple filters shall be combinable using AND logic
- JSON output shall include verification ID, directly verified requirements, and complete trace tree structure

##### Test Criteria
1. **Basic Markdown Output**
   Command: `reqvire verification-traces`
   - exits code **0**
   - output contains `# Verification Traceability Report`
   - output contains Mermaid diagram blocks with `graph BT`
   - diagrams include verification element nodes and requirement nodes
   - diagrams include click handlers for all nodes (format: `click NODE_ID "url"`)
   - directly verified requirements have `:::verified` CSS class in diagram

2. **JSON Output**
   Command: `reqvire verification-traces --json`
   - exits code **0**
   - output parses as valid JSON
   - JSON contains `verifications` array
   - each verification includes `verification_id`, `verification_name`, `verification_type`
   - each verification includes `directly_verified_requirements` array
   - each verification includes `trace_tree` with nested requirement structure

3. **Correct Arrow Directions**
   - Mermaid diagrams use `SYS001 -.->|verify| VER001` format (requirement verifies verification)
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
   - analysis, inspection, demonstration verifications are excluded

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
   - For from-folder `output` and identifier `specifications/UserRequirements.md#element`:
     - Link should be `../specifications/UserRequirements.md#element`
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
