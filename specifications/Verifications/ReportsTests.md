# Report Generation Tests

This document contains verification tests for Reqvire's report generation capabilities.

## Report Generation Tests
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  76ae69270700044b["Model Summary Tests"];
  class 76ae69270700044b verification;
  click 76ae69270700044b "ReportsTests.md#model-summary-tests";
  7b75340700b95177["tests/test-model-summary-reports/test.sh"];
  class 7b75340700b95177 default;
  click 7b75340700b95177 "../../tests/test-model-summary-reports/test.sh";
  7b75340700b95177 -->|satisfies| 76ae69270700044b;
  349f5e874cf22d98["Verification Coverage Report Test"];
  class 349f5e874cf22d98 verification;
  click 349f5e874cf22d98 "ReportsTests.md#verification-coverage-report-test";
  7099e5b2f8a08808["tests/test-coverage-report/test.sh"];
  class 7099e5b2f8a08808 default;
  click 7099e5b2f8a08808 "../../tests/test-coverage-report/test.sh";
  7099e5b2f8a08808 -->|satisfies| 349f5e874cf22d98;
  3108f29b131412a3["Index Generation Test"];
  class 3108f29b131412a3 verification;
  click 3108f29b131412a3 "ReportsTests.md#index-generation-test";
  a21995894299effb["SystemRequirements/Requirements.md#index-generation"];
  class a21995894299effb requirement;
  click a21995894299effb "../SystemRequirements/Requirements.md#index-generation";
  3108f29b131412a3 -.->|verifies| a21995894299effb;
  1123809d5f501bf1["tests/test-index-generation/test.sh"];
  class 1123809d5f501bf1 default;
  click 1123809d5f501bf1 "../../tests/test-index-generation/test.sh";
  1123809d5f501bf1 -->|satisfies| 3108f29b131412a3;
```

---

### Index Generation Test

This test verifies that the system correctly generates an index document with links and summaries to all specification documents.

#### Metadata
  * type: verification

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

#### Relations
  * verify: [SystemRequirements/Requirements.md#index-generation](../SystemRequirements/Requirements.md#index-generation)
  * satisfiedBy: [tests/test-index-generation/test.sh](../../tests/test-index-generation/test.sh)

---

### Model Summary Tests

This test verifies that the system provides a CLI flag and functionality for generating summary reports of model structure and relationships including varius filters.

#### Metadata
  * type: verification

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

#### Relations
  * satisfiedBy: [tests/test-model-summary-reports/test.sh](../../tests/test-model-summary-reports/test.sh)

---

### Verification Coverage Report Test

This test verifies that the system correctly generates verification coverage reports showing the percentage and details of satisfied and unsatisfied verifications.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System shall provide a CLI command `coverage-report` that generates coverage reports
- Command shall support `--json` flag for JSON output format
- Coverage report shall include summary section with total counts and percentages
- Coverage report shall show breakdown by verification type (test, analysis, inspection, demonstration)
- Coverage report shall list satisfied verifications grouped by file and section
- Coverage report shall list unsatisfied verifications with details
- JSON output shall be valid and machine-readable
- Text output shall be human-readable with clear formatting

##### Test Criteria

1. **Basic Coverage Report**
   Command: `reqvire coverage-report`
   - exits code **0**
   - output contains `=== Verification Coverage Report ===`
   - output contains `Summary:` section with total counts
   - output contains `Verification Types:` breakdown
   - output contains coverage percentage calculation
   - satisfied verifications are marked with ✅
   - unsatisfied verifications are marked with ❌

2. **JSON Coverage Report**
   Command: `reqvire coverage-report --json`
   - exits code **0**
   - output parses as valid JSON
   - JSON contains `summary` object with required fields
   - JSON contains `satisfied_verifications` and `unsatisfied_verifications` sections
   - verification details include identifier, name, section, type, and satisfied_by relations

3. **Coverage Calculation**
   - Coverage percentage calculated as (satisfied/total * 100)
   - Verification types correctly categorized
   - Satisfied verifications have non-empty `satisfied_by` relations
   - Unsatisfied verifications have empty `satisfied_by` relations

#### Relations
  * satisfiedBy: [tests/test-coverage-report/test.sh](../../tests/test-coverage-report/test.sh)

---