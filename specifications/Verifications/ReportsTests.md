# Report Generation Tests

This document contains verification tests for Reqvire's report generation capabilities.

## Report Generation Tests

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
- **Enhanced Content Display**: Search must display page content (frontmatter before first section) and section content (content between section headers and first element) when not in short mode
- **Count Information**: Search must show counts for files, pages, sections, and elements in full mode (omitted in short mode)
- **Short Mode Behavior**: Short mode omits: `content`, `section_content`, `page_content`, `verified_relations_count`, `satisfied_relations_count`, `element_count`, `total_sections`, `total_elements`, `global_counters`

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
   - each element block includes identifier, name, section, file, type, content
   - relations displayed for each element

3. **Short mode text output**
   Command: `reqvire search --short`
   - exits code **0**
   - one line per element: `[type] identifier - name`
   - no content, section content, or page content displayed
   - no count information displayed

4. **Short mode JSON output**
   Command: `reqvire search --short --json`
   - exits code **0**
   - output parses as valid JSON
   - element objects do NOT contain: `content`, `verified_relations_count`, `satisfied_relations_count`
   - section objects do NOT contain: `section_content`, `element_count`
   - file objects do NOT contain: `page_content`, `total_sections`, `total_elements`
   - top level does NOT contain: `global_counters`

5. **Individual filters**
   For each flag in turn, run both JSON and text modes:
   - `--filter-file="**/*Reqs.md"` (glob)
   - `--filter-name=".*safety.*"` (regex)
   - `--filter-section="System*"` (glob)
   - `--filter-type="user-requirement"` (exact)
   - `--filter-content="MUST"` (regex)
   - `--filter-section-content="implement.*"` (regex)
   - `--filter-page-content="architecture"` (regex)
   - `--have-relations=verifiedBy` (comma-separated)
   - `--not-have-relations=verifiedBy` (comma-separated)
   Assert for each:
   - exit code **0**
   - only elements matching the filter appear in output

6. **Filter combinations**
   Combine multiple filters and verify outputs contain exactly those elements passing ALL filters:
   - `--filter-type=user-requirement --have-relations=verifiedBy,satisfiedBy`
   - `--filter-section="System*" --filter-name=".*GPS.*"`
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

11. **Section content filter**
    Command: `reqvire search --filter-section-content="MUST.*implement"`
    - exits code **0**
    - only elements whose parent section content matches regex appear
    - elements in sections without matching content are excluded

12. **Page content filter**
    Command: `reqvire search --filter-page-content="architecture"`
    - exits code **0**
    - only elements whose parent file page content matches regex appear
    - elements in files without matching page content are excluded

13. **Relations coverage**
    Command: `reqvire search --json`
    - Both JSON and text outputs must show complete relationship information
    - All relation types and targets included

14. **Enhanced content and counts verification (full mode)**
    Command: `reqvire search --json`
    - JSON output must include `page_content` field for files that have frontmatter content
    - JSON output must include `section_content` field for sections that have content
    - JSON output must include count fields in global counters
    - JSON output must include per-file and per-section counts

15. **Short mode field omission verification**
    Command: `reqvire search --short --json`
    - Verify all specified fields are omitted from JSON structure
    - Verify no null/empty placeholders for omitted fields (fields completely absent)

#### Metadata
  * type: test-verification

#### Relations
  * verify: [CLI Search Command](../ReqvireTool/UserInterface/CLI.md#cli-search-command)
  * verify: [Search Fine Grained Filtering](../ReqvireTool/ValidationAndReporting/Reports.md#search-fine-grained-filtering)
  * satisfiedBy: [test.sh](../../tests/test-search-all-features/test.sh)
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
  * verify: [CLI Coverage Command](../ReqvireTool/UserInterface/CLI.md#cli-coverage-command)
  * satisfiedBy: [test.sh](../../tests/test-coverage-report/test.sh)
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
  * verify: [CLI Traces Filter Options](../ReqvireTool/UserInterface/CLI.md#cli-traces-filter-options)
  * satisfiedBy: [test.sh](../../tests/test-verification-traces/test.sh)
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
  * verify: [CLI Traces From-Folder Option](../ReqvireTool/UserInterface/CLI.md#cli-traces-from-folder-option)
  * satisfiedBy: [test.sh](../../tests/test-verification-traces/test.sh)
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
   - Each element has: `identifier`, `name`, `element_type`, `file_path`, `section`, `section_index`, `relations`
   - Each relation has: `relation_type`, target (element/file/external)
   - Element targets are nested recursively with same structure
   - File targets have: `path`, `type: "file"`
   - External targets have: `url`, `type: "external"`
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
  * verify: [Forward-Only Relation Traversal](../ReqvireTool/ValidationAndReporting/Reports.md#forward-only-relation-traversal)
  * verify: [Model Diagram Output Formats](../ReqvireTool/ValidationAndReporting/Reports.md#model-diagram-output-formats)
  * satisfiedBy: [test.sh](../../tests/test-model-command/test.sh)
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
  * verify: [Custom Element Type Tracking](../ReqvireTool/ValidationAndReporting/Reports.md#custom-element-type-tracking)
  * satisfiedBy: [test.sh](../../tests/test-search-all-features/test.sh)
---
