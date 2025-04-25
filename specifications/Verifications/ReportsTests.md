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

  e6e9798505cdb1ec["Index Generation Test"];
  click e6e9798505cdb1ec "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/ReportsTests.md#index-generation-test";
  class e6e9798505cdb1ec verification;
  ca07c1345af51729["SystemRequirements/Requirements.md#index-generation"];
  class ca07c1345af51729 requirement;
  click ca07c1345af51729 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#index-generation";
  e6e9798505cdb1ec -.->|verifies| ca07c1345af51729;
  9a608ec01ff92139["tests/test-index-generation/test.sh"];
  class 9a608ec01ff92139 default;
  click 9a608ec01ff92139 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/tests/test-index-generation/test.sh";
  e6e9798505cdb1ec -.->|trace| 9a608ec01ff92139;
  f8788f9e745e13b["CLI Summary Report Flag Test"];
  click f8788f9e745e13b "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/ReportsTests.md#cli-summary-report-flag-test";
  class f8788f9e745e13b verification;
  a3d8f2cd99b2d548["SystemRequirements/Requirements.md#cli-summary-report-flag"];
  class a3d8f2cd99b2d548 requirement;
  click a3d8f2cd99b2d548 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#cli-summary-report-flag";
  f8788f9e745e13b -.->|verifies| a3d8f2cd99b2d548;
  890802c6addd8829["tests/test-element-content-extraction/test.sh"];
  class 890802c6addd8829 default;
  click 890802c6addd8829 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/tests/test-element-content-extraction/test.sh";
  f8788f9e745e13b -.->|trace| 890802c6addd8829;
```

---

### CLI Summary Report Flag Test

This test verifies that the system provides a CLI flag for generating summary reports of model structure and relationships.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System shall provide a --model-summary flag
- Flag shall activate the summary report generation
- Report shall include model structure information
- Report shall include relationship counts and types

##### Test Criteria
- Command with --model-summary flag runs successfully
- Summary report is generated with expected sections
- Report includes counts of element types
- Report includes information about relationship types and counts
- Elements are organized by file and section

##### Test Procedure
1. Create test fixtures with various elements and relationship types
2. Run Reqvire with --model-summary flag on the test fixtures
3. Verify that summary report is generated with expected format
4. Confirm that all element types and counts are accurately reported
5. Verify that relationship information is correctly represented

#### Relations
  * verify: [SystemRequirements/Requirements.md#cli-summary-report-flag](../SystemRequirements/Requirements.md#cli-summary-report-flag)
  * trace: [tests/test-element-content-extraction/test.sh](../../tests/test-element-content-extraction/test.sh)

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