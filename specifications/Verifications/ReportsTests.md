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

  f6e9175718fdded9["Index Generation Test"];
  click f6e9175718fdded9 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/ReportsTests.md#index-generation-test";
  class f6e9175718fdded9 verification;
  62ab248e5b824192["SystemRequirements/Requirements.md#index-generation"];
  class 62ab248e5b824192 requirement;
  click 62ab248e5b824192 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#index-generation";
  f6e9175718fdded9 -.->|verifies| 62ab248e5b824192;
  96abed834a531e73["tests/test-index-generation/test.sh"];
  class 96abed834a531e73 default;
  click 96abed834a531e73 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/tests/test-index-generation/test.sh";
  f6e9175718fdded9 -.->|trace| 96abed834a531e73;
  9896fb72b6f1aa54["CLI Summary Report Flag Test"];
  click 9896fb72b6f1aa54 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/ReportsTests.md#cli-summary-report-flag-test";
  class 9896fb72b6f1aa54 verification;
  9cd307abe52b0aee["SystemRequirements/Requirements.md#cli-summary-report-flag"];
  class 9cd307abe52b0aee requirement;
  click 9cd307abe52b0aee "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#cli-summary-report-flag";
  9896fb72b6f1aa54 -.->|verifies| 9cd307abe52b0aee;
  4de1c5f9aeb25f86["tests/test-element-content-extraction/test.sh"];
  class 4de1c5f9aeb25f86 default;
  click 4de1c5f9aeb25f86 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/tests/test-element-content-extraction/test.sh";
  9896fb72b6f1aa54 -.->|trace| 4de1c5f9aeb25f86;
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