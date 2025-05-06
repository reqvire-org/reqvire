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

  ecd5cbbaddffb824["Model Summary Tests"];
  click ecd5cbbaddffb824 "https://github.com/Reqvire/reqvire/blob/b8c43d2689b933838bf1838d8a4cdf3393c9014f/specifications/Verifications/ReportsTests.md#model-summary-tests";
  class ecd5cbbaddffb824 verification;
  ed42c8b28f021de0["tests/test-model-summary-reports/test.sh"];
  class ed42c8b28f021de0 default;
  click ed42c8b28f021de0 "https://github.com/Reqvire/reqvire/blob/b8c43d2689b933838bf1838d8a4cdf3393c9014f/tests/test-model-summary-reports/test.sh";
  ecd5cbbaddffb824 -.->|trace| ed42c8b28f021de0;
  62274e8cf8493254["Index Generation Test"];
  click 62274e8cf8493254 "https://github.com/Reqvire/reqvire/blob/b8c43d2689b933838bf1838d8a4cdf3393c9014f/specifications/Verifications/ReportsTests.md#index-generation-test";
  class 62274e8cf8493254 verification;
  3b4bfa0725509a0e["SystemRequirements/Requirements.md#index-generation"];
  class 3b4bfa0725509a0e requirement;
  click 3b4bfa0725509a0e "https://github.com/Reqvire/reqvire/blob/b8c43d2689b933838bf1838d8a4cdf3393c9014f/specifications/SystemRequirements/Requirements.md#index-generation";
  62274e8cf8493254 -.->|verifies| 3b4bfa0725509a0e;
  9c89c7cfe5f93c50["tests/test-index-generation/test.sh"];
  class 9c89c7cfe5f93c50 default;
  click 9c89c7cfe5f93c50 "https://github.com/Reqvire/reqvire/blob/b8c43d2689b933838bf1838d8a4cdf3393c9014f/tests/test-index-generation/test.sh";
  62274e8cf8493254 -.->|trace| 9c89c7cfe5f93c50;
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

##### Test Procedure
1. Create test fixtures with multiple specification documents in various folders
2. Run Reqvire with --generate-index flag
3. Verify that README.md is created in the specifications folder
4. Verify that README.md contains links to all specification documents
5. Verify that document structure and summaries are included

#### Relations
  * verify: [SystemRequirements/Requirements.md#index-generation](../SystemRequirements/Requirements.md#index-generation)
  * trace: [tests/test-index-generation/test.sh](../../tests/test-index-generation/test.sh)

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
  * trace: [tests/test-model-summary-reports/test.sh](../../tests/test-model-summary-reports/test.sh)

---