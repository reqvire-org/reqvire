# Linting Tests

This document verifies the requirements for ReqFlow's linting functionality.

## Whitespace Linting Tests
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  4cd0fe733f["Excess Whitespace Detection and Correction"];
  click 4cd0fe733f "LintingTests.md#excess-whitespace-detection-and-correction";
  class 4cd0fe733f verification;
  3f3d3f9ccb["SystemRequirements/Requirements.md/Excess Whitespace Linting Implementation"];
  class 3f3d3f9ccb default;
  click 3f3d3f9ccb "../SystemRequirements/Requirements.md#excess-whitespace-linting-implementation";
  4cd0fe733f -->|verifies| 3f3d3f9ccb;
  8f2198f681["SystemRequirements/Requirements.md/Dry Run Mode Implementation"];
  class 8f2198f681 default;
  click 8f2198f681 "../SystemRequirements/Requirements.md#dry-run-mode";
  4cd0fe733f -->|verifies| 8f2198f681;
  e349eadbaa["tests/e2e-linting/test_whitespace_linting.sh"];
  class e349eadbaa default;
  click e349eadbaa "../../tests/e2e-linting/test_whitespace_linting.sh";
  4cd0fe733f -->|traces| e349eadbaa;
```

---

### Excess Whitespace Detection and Correction

The verification test checks that ReqFlow correctly identifies and fixes excess whitespace after headers.

#### Metadata
  * type: verification

#### Acceptance Criteria
- System should detect excess whitespace after headers
- System should fix excess whitespace in linting mode
- The output should show before/after changes

#### Test Criteria
- Command exits with success (0) return code
- Output shows whitespace being fixed
- Output should contain diff-style formatting

#### Test Procedure
1. Create a test fixture in `/tests/fixtures/test-lint-headers/` with excess whitespace after headers
2. Run ReqFlow linting in dry-run mode to check detection
3. Verify that excess whitespace is reported
4. Run ReqFlow linting in normal mode to apply fixes
5. Verify that the whitespace has been corrected in the output files

#### Relations
  * verify: [SystemRequirements/Requirements.md/Excess Whitespace Linting Implementation](../SystemRequirements/Requirements.md#excess-whitespace-linting-implementation)
  * verify: [SystemRequirements/Requirements.md/Dry Run Mode Implementation](../SystemRequirements/Requirements.md#dry-run-mode)
  * trace: [tests/e2e-linting/test_whitespace_linting.sh](../../tests/e2e-linting/test_whitespace_linting.sh)

---

## External Folders Tests

---

### External Folders Support Verification

The verification test checks that ReqFlow correctly processes requirements in external folders.

#### Metadata
  * type: verification

#### Acceptance Criteria
- System should process requirements in external folders
- System should treat external requirements as system requirements
- User Requirements in external folders should cause validation errors

#### Test Criteria
- Validation succeeds with proper external folder setup
- Validation fails when external folder contains user requirements
- Diagram generation includes external folder requirements

#### Test Procedure
1. Set up test fixtures in `/tests/fixtures/test-external-folders/` with external folders configuration
2. Run ReqFlow with invalid setup (user requirements in external folder)
3. Verify that validation reports the appropriate error
4. Run ReqFlow with valid setup (only system requirements in external folder)
5. Verify that validation succeeds and requirements are properly processed

#### Relations
  * verify: [SystemRequirements/Requirements.md#External Folders Support](../SystemRequirements/Requirements.md#external-folders-support)
  * verify: [SystemRequirements/Requirements.md#Directory Structure Processing](../SystemRequirements/Requirements.md#directory-structure-processing)
  * trace: [tests/e2e-external-folders/test_external_folders.sh](../../tests/e2e-external-folders/test_external_folders.sh)
