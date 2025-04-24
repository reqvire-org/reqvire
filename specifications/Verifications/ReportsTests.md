# Report Generation Tests

This document contains verification tests for ReqFlow's report generation capabilities.

## Report Generation Tests
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  ecd45710776293b["Index Generation Test"];
  click ecd45710776293b "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/Verifications/ReportsTests.md#index-generation-test";
  class ecd45710776293b verification;
  8acd24c7c228637b["SystemRequirements/Requirements.md#index-generation"];
  class 8acd24c7c228637b requirement;
  click 8acd24c7c228637b "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#index-generation";
  ecd45710776293b -.->|verifies| 8acd24c7c228637b;
  473af2ecdcda99a5["tests/test-index-generation/test.sh"];
  class 473af2ecdcda99a5 default;
  click 473af2ecdcda99a5 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/tests/test-index-generation/test.sh";
  ecd45710776293b -.->|trace| 473af2ecdcda99a5;
  5ec920981cff81d1["CLI Summary Report Flag Test"];
  click 5ec920981cff81d1 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/Verifications/ReportsTests.md#cli-summary-report-flag-test";
  class 5ec920981cff81d1 verification;
  6f86272134897867["SystemRequirements/Requirements.md#cli-summary-report-flag"];
  class 6f86272134897867 requirement;
  click 6f86272134897867 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#cli-summary-report-flag";
  5ec920981cff81d1 -.->|verifies| 6f86272134897867;
  e0e585775432dd67["tests/test-element-content-extraction/test.sh"];
  class e0e585775432dd67 default;
  click e0e585775432dd67 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/tests/test-element-content-extraction/test.sh";
  5ec920981cff81d1 -.->|trace| e0e585775432dd67;
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
2. Run ReqFlow with --model-summary flag on the test fixtures
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
2. Run ReqFlow with --generate-index flag
3. Verify that README.md is created in the specifications folder
4. Verify that README.md contains links to all specification documents
5. Verify that document structure and summaries are included

#### Relations
  * verify: [SystemRequirements/Requirements.md#index-generation](../SystemRequirements/Requirements.md#index-generation)
  * trace: [tests/test-index-generation/test.sh](../../tests/test-index-generation/test.sh)

---