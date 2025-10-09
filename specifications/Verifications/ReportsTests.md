# Report Generation Tests

This document contains verification tests for Reqvire's report generation capabilities.

## Report Generation Tests

### Index Generation Test

This test verifies that the system correctly generates an index document with links and summaries to all specification documents.

#### Details

##### Acceptance Criteria
- System shall generate README.md in the specifications folder
- README.md shall contain links to all specification documents
- README.md shall be properly structured with sections
- README.md shall include brief summaries of each document

##### Test Criteria
- Command with --generate-index flag runs successfully
- README.md file is created in the specifications folder
- README.md contains links to all specification documents
- README.md structure follows expected format

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Index Generation](../SystemRequirements/Requirements.md#index-generation)
  * satisfiedBy: [test.sh](../../tests/test-index-generation/test.sh)
---

### Model Summary Tests

This test verifies that the system provides a CLI flag and functionality for generating summary reports of model structure and relationships including varius filters.

#### Details

##### Acceptance Criteria
- Running `reqvire --model-summary --json` produces a valid, pretty-printed JSON summary.
- Running `reqvire --model-summary` (no `--json`) prints a human-readable markdown text summary beginning with `--- MBSE Model summary ---`.
- Both JSON and text summaries include exactly five elements with the identifiers:
  - `Requirement-with-Valid-Standard-Relations`
  - `Requirement-with-Valid-Markdown-Relations`
  - `Requirement-with-DesignSpecifications-Reference`
  - `Requirement-with-Many-Subsections`
  - `Verification-of-Standard-Relations`
- When any filter flags (`--filter-file`, `--filter-section`, `--filter-type`, `--filter-name-regex`, `--filter-content`, `--filter-is-verified`, `--filter-is-satisfied`) are supplied with `--model-summary` (and optionally `--json`), only elements matching **all** specified filters appear in both outputs.
- Supplying multiple filters in combination yields the intersection of their individual results.
- Running any filter flag **without** `--model-summary` fails with a non-zero exit code and a help message indicating the dependency.
- Supplying an invalid regex to `--filter-name-regex` or `--filter-content` fails with a non-zero exit code and displays a `ReqvireError::InvalidRegex` message.
- Model summary report must include all relations for each element, showing both explicit relations and their opposite relations (e.g., if a requirement has `verifiedBy`, the verification element should show `verify`; if a verification has `verify`, the requirement should show `verifiedBy`).
- **Enhanced Content Display**: Model summary must display page content (frontmatter before first section) and section content (content between section headers and first element).
- **Count Information**: Model summary must show counts for files, pages, sections, and elements in both text and JSON formats.
- **Content Formatting**: Page content and section content must use consistent newline formatting (`\n`) matching element content display format.
- **Global Statistics**: Summary must include comprehensive global counters including total files, sections, elements, and verification/satisfaction statistics.

##### Test Criteria
1. **Base JSON summary**  
   Command: `reqvire --model-summary --json`  
   - exits code **0**  
   - output parses under `jq`  
   - `.model_summary.global_counters.total_elements == 5`  
   - `.model_summary.files` contains key `"Requirements.md"`  
   - `.model_summary.files["Requirements.md"]["Requirements"]` contains exactly the five identifiers above  

2. **Base text summary**  
   Command: `reqvire --model-summary`  
   - exits code **0**  
   - first line is `--- MBSE Model summary ---`  
   - exactly five lines matching `🔹 Element: <identifier>` for the five identifiers above  
   - each element block includes `- Name:`, `- Section:`, `- File:`, `- Type:`, and `- Content:`  

3. **Individual filters**  
   For each flag in turn, run both JSON and text modes:  
   - `--filter-file="Requirements.md"`  
   - `--filter-section="Requirements"`  
   - `--filter-type="user-requirement"`  
   - `--filter-name-regex="^Requirement with Valid Standard"`  
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
   Command: `reqvire --model-summary --json --filter-name-regex="***"`  
   - exits non-zero  
   - stderr contains `Invalid regex`  

6. **Filter without model-summary**
   Command: `reqvire --filter-file="*.md"`
   - exits non-zero
   - stderr indicates `requires --model-summary`

7. **Relations coverage**
   Command: `reqvire --model-summary --json`
   - For any requirement with `verifiedBy` relations, verify that the target verification elements show corresponding `verify` relations pointing back to the requirement
   - For any verification with `verify` relations, verify that the target requirement elements show corresponding `verifiedBy` relations pointing back to the verification
   - Same pattern applies to other relation pairs: `satisfiedBy`/`satisfy`, `containedBy`/`contain`, `derivedFrom`/`derive`, `refine`/`refinedBy`
   - Both JSON and text outputs must show complete bidirectional relationship information

8. **Enhanced content and counts verification**
   Command: `reqvire --model-summary --json`
   - JSON output must include `page_content` field for files that have frontmatter content
   - JSON output must include `section_content` field for sections that have content
   - JSON output must include count fields: `total_files`, `total_sections`, `total_elements` in global counters
   - JSON output must include per-file counts: `total_sections`, `total_elements` in file summaries
   - JSON output must include per-section counts: `element_count` in section summaries

9. **Enhanced text output verification**
   Command: `reqvire --model-summary`
   - Text output must show file counts in format: `📂 File: path (sections: N, elements: N)`
   - Text output must show section counts in format: `📖 Section: name (elements: N)`
   - Text output must display page content with `📄 Page content: "content with \n"` format when present
   - Text output must display section content with `📝 Section content: "content with \n"` format when present
   - Global summary must include counts for total files, sections, and elements

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Model Structure and Summaries](../UserRequirements.md#model-structure-and-summaries)
  * verify: [CLI Summary Report Command](../SystemRequirements/Requirements.md#cli-summary-report-command)
  * satisfiedBy: [test.sh](../../tests/test-model-summary-reports/test.sh)
---

### Verification Coverage Report Test

This test verifies that the system correctly generates verification coverage reports focusing on leaf requirements and showing the percentage and details of satisfied and unsatisfied test-verification elements.

#### Details

##### Acceptance Criteria
- System shall provide a CLI command `coverage-report` that generates coverage reports focusing on leaf requirements
- Command shall support `--json` flag for JSON output format
- Coverage report shall include summary section with total counts and percentages for leaf requirements
- Coverage report shall show breakdown by verification type (test, analysis, inspection, demonstration)
- Coverage report shall list verified leaf requirements grouped by file and section
- Coverage report shall list unverified leaf requirements with details
- Coverage report shall list satisfied test-verification elements (those with satisfiedBy relations)
- Coverage report shall list unsatisfied test-verification elements (those without satisfiedBy relations)
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
  * satisfiedBy: [test.sh](../../tests/test-coverage-report/test.sh)
---

### Sections Summary Tests

This test verifies that the system provides sections-summary command functionality for generating focused reports of file and section structure without individual elements.

#### Details

##### Acceptance Criteria
- Running `reqvire sections-summary --json` produces a valid, pretty-printed JSON summary with sections information.
- Running `reqvire sections-summary` (no `--json`) prints a human-readable text summary showing files and sections.
- Both JSON and text outputs include file paths, section names, section order indices, and section content.
- Section order indices preserve the original document structure and enable reconstruction.
- Individual elements (requirements, verifications) are excluded from the output.
- When filter flags are supplied, only sections matching **all** specified filters appear in both outputs.
- Command supports filtering by file path (glob), section name (glob), and section content (regex).
- JSON output includes section order information for document structure reconstruction.

##### Test Criteria
1. **Base JSON sections summary**
   Command: `reqvire sections-summary --json`
   - exits code **0**
   - output parses under `jq`
   - `.files` contains file path keys
   - Each file contains sections with `name`, `content`, and `section_order` fields
   - No individual elements are included in the output

2. **Base text sections summary**
   Command: `reqvire sections-summary`
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
  * verify: [CLI Sections Summary Command](../SystemRequirements/Requirements.md#cli-sections-summary-command)
  * verify: [Sections Summary Report Generator](../SystemRequirements/Requirements.md#sections-summary-report-generator)
  * satisfiedBy: [test.sh](../../tests/test-sections-summary/test.sh)
---

### Verification Traces Filter Options Test

This test verifies that the verification-traces command filter options work correctly when generating upward trace trees from verification elements to root requirements.

#### Details

##### Acceptance Criteria
- System shall provide CLI command `verification-traces` that generates upward trace trees from verifications
- Command shall output to stdout in Markdown format with embedded Mermaid diagrams by default
- Command shall support `--json` flag for structured JSON output without diagrams
- Mermaid diagrams shall show verification element as root with arrows following relation semantics
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
  * verify: [Verification Traces Filter Options](../SystemRequirements/Requirements.md#verification-traces-filter-options)
  * satisfiedBy: [test.sh](../../tests/test-verification-traces/test.sh)
---
