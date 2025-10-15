# Report Generation Tests

This document contains verification tests for Reqvire's report generation capabilities.

## Report Generation Tests

### Model Summary Tests

This test verifies that the system provides a CLI flag and functionality for generating summary reports of model structure and relationships including varius filters.

#### Details

##### Acceptance Criteria
- Running `reqvire summary --json` produces a valid, pretty-printed JSON summary.
- Running `reqvire summary` (no `--json`) prints a human-readable markdown text summary beginning with `--- MBSE Model summary ---`.
- Both JSON and text summaries include exactly five elements with the identifiers:
  - `Requirement-with-Valid-Standard-Relations`
  - `Requirement-with-Valid-Markdown-Relations`
  - `Requirement-with-DesignSpecifications-Reference`
  - `Requirement-with-Many-Subsections`
  - `Verification-of-Standard-Relations`
- When any filter flags (`--filter-file`, `--filter-section`, `--filter-type`, `--filter-name`, `--filter-content`, `--filter-is-verified`, `--filter-is-satisfied`) are supplied with `summary` (and optionally `--json`), only elements matching **all** specified filters appear in both outputs.
- Supplying multiple filters in combination yields the intersection of their individual results.
- Supplying an invalid regex to `--filter-name` or `--filter-content` fails with a non-zero exit code and displays a `ReqvireError::InvalidRegex` message.
- Model summary report must include all relations for each element, showing both explicit relations and their opposite relations (e.g., if a requirement has `verifiedBy`, the verification element should show `verify`; if a verification has `verify`, the requirement should show `verifiedBy`).
- **Enhanced Content Display**: Model summary must display page content (frontmatter before first section) and section content (content between section headers and first element).
- **Count Information**: Model summary must show counts for files, pages, sections, and elements in both text and JSON formats.
- **Content Formatting**: Page content and section content must use consistent newline formatting (`\n`) matching element content display format.
- **Global Statistics**: Summary must include comprehensive global counters including total files, sections, elements, and verification/satisfaction statistics.

##### Test Criteria
1. **Base JSON summary**
   Command: `reqvire summary --json`
   - exits code **0**
   - output parses under `jq`
   - `.model_summary.global_counters.total_elements == 5`
   - `.model_summary.files` contains key `"Requirements.md"`
   - `.model_summary.files["Requirements.md"]["Requirements"]` contains exactly the five identifiers above

2. **Base text summary**
   Command: `reqvire summary`
   - exits code **0**
   - first line is `--- MBSE Model summary ---`
   - exactly five lines matching `🔹 Element: <identifier>` for the five identifiers above
   - each element block includes `- Name:`, `- Section:`, `- File:`, `- Type:`, and `- Content:`

3. **Individual filters**
   For each flag in turn, run both JSON and text modes:
   - `--filter-file="Requirements.md"`
   - `--filter-section="Requirements"`
   - `--filter-type="user-requirement"`
   - `--filter-name="^Requirement with Valid Standard"`
   - `--filter-content="subsection"`
   - `--filter-is-verified`
   - `--filter-is-satisfied`
   Assert for each:
   - exit code **0**
   - total elements < 5 (unless the filter matches all)
   - only the expected subset of identifiers appears

4. **Filter combinations**
   Combine two filters (e.g. `--filter-type=user-requirement` + `--filter-is-satisfied`) and verify both outputs contain exactly those identifiers passing both filters.

5. **Invalid regex**
   Command: `reqvire summary --json --filter-name="***"`
   - exits non-zero
   - stderr contains `Invalid regex`

6. **Relations coverage**
   Command: `reqvire summary --json`
   - For any requirement with `verifiedBy` relations, verify that the target verification elements show corresponding `verify` relations pointing back to the requirement
   - For any verification with `verify` relations, verify that the target requirement elements show corresponding `verifiedBy` relations pointing back to the verification
   - Same pattern applies to other relation pairs: `satisfiedBy`/`satisfy`, `derivedFrom`/`derive`
   - Both JSON and text outputs must show complete bidirectional relationship information

7. **Enhanced content and counts verification**
   Command: `reqvire summary --json`
   - JSON output must include `page_content` field for files that have frontmatter content
   - JSON output must include `section_content` field for sections that have content
   - JSON output must include count fields: `total_files`, `total_sections`, `total_elements` in global counters
   - JSON output must include per-file counts: `total_sections`, `total_elements` in file summaries
   - JSON output must include per-section counts: `element_count` in section summaries

8. **Enhanced text output verification**
   Command: `reqvire summary`
   - Text output must show file counts in format: `📂 File: path (sections: N, elements: N)`
   - Text output must show section counts in format: `📖 Section: name (elements: N)`
   - Text output must display page content with `📄 Page content: "content with \n"` format when present
   - Text output must display section content with `📝 Section content: "content with \n"` format when present
   - Global summary must include counts for total files, sections, and elements

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Display Name-Regex Option in Help](../ReqvireTool/UserInterface/CLI.md#display-name-regex-option-in-help)
  * verify: [Model Summary Fine Grained Filtering](../ReqvireTool/ValidationAndReporting/Reports.md#model-summary-fine-grained-filtering)
  * verify: [Handle Invalid Regex Filter Patterns](../ReqvireTool/UserInterface/CLI.md#handle-invalid-regex-filter-patterns)
  * satisfiedBy: [test.sh](../../tests/test-model-summary-reports/test.sh)
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

### Sections Summary Tests

This test verifies that the system provides `section-summary` command functionality for generating focused reports of file and section structure without individual elements.

#### Details

##### Acceptance Criteria
- Running `reqvire section-summary --json` produces a valid, pretty-printed JSON summary with sections information.
- Running `reqvire section-summary` (no `--json`) prints a human-readable text summary showing files and sections.
- Both JSON and text outputs include file paths, section names, section order indices, and section content.
- Section order indices preserve the original document structure and enable reconstruction.
- Individual elements (requirements, verifications) are excluded from the output.
- When filter flags are supplied, only sections matching **all** specified filters appear in both outputs.
- Command supports filtering by file path (glob), section name (glob), and section content (regex).
- JSON output includes section order information for document structure reconstruction.

##### Test Criteria
1. **Base JSON sections summary**
   Command: `reqvire section-summary --json`
   - exits code **0**
   - output parses under `jq`
   - `.files` contains file path keys
   - Each file contains sections with `name`, `content`, and `section_order` fields
   - No individual elements are included in the output

2. **Base text sections summary**
   Command: `reqvire section-summary`
   - exits code **0**
   - output shows file paths and section information
   - sections are ordered by their `section_order` index
   - section content is displayed but individual elements are not

3. **Section filtering**
   For each filter flag, run both JSON and text modes:
   - `--filter-file="Requirements.md"` (glob pattern)
   - `--filter-section="Requirements*"` (glob pattern)
   - `--filter-content="system"` (regex pattern)
   - Each filter produces subset of sections matching criteria
   - Filters work independently and in combination

4. **Section order preservation**
   - JSON output includes `section_order` field for each section
   - Text output displays sections in document order
   - Order indices enable reconstruction of original document structure

#### Metadata
  * type: test-verification

#### Relations
  * verify: [CLI Sections Summary Command](../ReqvireTool/UserInterface/CLI.md#cli-sections-summary-command)
  * satisfiedBy: [test.sh](../../tests/test-sections-summary/test.sh)
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

Comprehensive test verifying model command generates correct output in different modes.

#### Details

##### Acceptance Criteria
1. `reqvire model` generates markdown with complete model diagram showing all elements
2. `reqvire model --root-id=<id>` generates diagram with only forward-related elements from root
3. `reqvire model --json` generates valid JSON structure with all model data
4. `reqvire model --root-id=<id> --json` generates filtered JSON with forward-related elements
5. Filtered diagrams include only elements reachable via forward relations
6. JSON output can be parsed and contains expected fields (folders, relations)

##### Test Criteria
1. **Full Model Markdown Output**
   Command: `reqvire model`
   - exits code **0**
   - output contains `# Model Diagram Report`
   - output contains Mermaid diagram block with `graph TD`
   - diagram includes all test elements
   - diagram shows hierarchical structure (folders > files > sections > elements)

2. **Filtered Model Markdown Output**
   Command: `reqvire model --root-id=<test-element-id>`
   - exits code **0**
   - output contains `# Model Diagram Report`
   - output contains Mermaid diagram with filtered elements
   - diagram includes only elements reachable via forward relations from root
   - elements not in forward path are excluded

3. **Full Model JSON Output**
   Command: `reqvire model --json`
   - exits code **0**
   - output parses as valid JSON
   - JSON contains `folders` array
   - JSON contains `relations` array
   - folders contain files, files contain sections, sections contain elements
   - all test elements are present in JSON structure

4. **Filtered Model JSON Output**
   Command: `reqvire model --root-id=<test-element-id> --json`
   - exits code **0**
   - output parses as valid JSON
   - JSON contains only elements reachable via forward relations
   - filtered elements count is less than full model

5. **Forward-Only Traversal Verification**
   - Create test with element A that derives B, and B derives C
   - Running `reqvire model --root-id=<A-id>` includes B and C
   - Create element D that is derived from B (backward relation)
   - Running `reqvire model --root-id=<A-id>` includes B and C but NOT D
   - Confirms only forward relations (derive, satisfiedBy, verifiedBy, trace) are followed

6. **JSON Structure Validation**
   Command: `reqvire model --json`
   - JSON has keys: `folders`, `relations`
   - Each folder has: `name`, `path`, `files`
   - Each file has: `name`, `path`, `sections`
   - Each section has: `name`, `elements`
   - Each element has: `identifier`, `name`, `element_type`
   - Each relation has: `source_id`, `target_id`, `relation_type`, `is_external`

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Forward-Only Relation Traversal](../ReqvireTool/ValidationAndReporting/Reports.md#forward-only-relation-traversal)
  * verify: [Model Diagram Output Formats](../ReqvireTool/ValidationAndReporting/Reports.md#model-diagram-output-formats)
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
  * satisfiedBy: [test.sh](../../tests/test-model-summary-reports/test.sh)
---
