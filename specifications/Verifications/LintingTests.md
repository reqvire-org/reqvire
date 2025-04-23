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

  4cd0fe733f182d5c["Excess Whitespace Detection and Correction"];
  click 4cd0fe733f182d5c "LintingTests.md#excess-whitespace-detection-and-correction";
  class 4cd0fe733f182d5c verification;
  3f3d3f9ccb15029a["SystemRequirements/Requirements.md/Excess Whitespace Linting Implementation"];
  class 3f3d3f9ccb15029a requirement;
  click 3f3d3f9ccb15029a "../SystemRequirements/Requirements.md#excess-whitespace-linting-implementation";
  4cd0fe733f182d5c -.->|verifies| 3f3d3f9ccb15029a;
  8f2198f681f63fea["SystemRequirements/Requirements.md/Dry Run Mode"];
  class 8f2198f681f63fea requirement;
  click 8f2198f681f63fea "../SystemRequirements/Requirements.md#dry-run-mode";
  4cd0fe733f182d5c -.->|verifies| 8f2198f681f63fea;
  3de539f546654b01["tests/test-lint-expected/test.sh"];
  class 3de539f546654b01 default;
  click 3de539f546654b01 "../../tests/test-lint-expected/test.sh";
  4cd0fe733f182d5c -.->|trace| 3de539f546654b01;
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
  * verify: [SystemRequirements/Requirements.md/Dry Run Mode](../SystemRequirements/Requirements.md#dry-run-mode)
  * trace: [tests/test-lint-expected/test.sh](../../tests/test-lint-expected/test.sh)

---

## External Folders Tests
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  97e9fdec98b5f3e5["External Folders Support Verification"];
  click 97e9fdec98b5f3e5 "LintingTests.md#external-folders-support-verification";
  class 97e9fdec98b5f3e5 verification;
  d38ab4ad139183d3["SystemRequirements/Requirements.md#External Folders Support"];
  class d38ab4ad139183d3 requirement;
  click d38ab4ad139183d3 "../SystemRequirements/Requirements.md#external-folders-support";
  97e9fdec98b5f3e5 -.->|verifies| d38ab4ad139183d3;
  99bed90a0d96a1d2["SystemRequirements/Requirements.md#Requirements Processing"];
  class 99bed90a0d96a1d2 requirement;
  click 99bed90a0d96a1d2 "../SystemRequirements/Requirements.md#requirements-processing";
  97e9fdec98b5f3e5 -.->|verifies| 99bed90a0d96a1d2;
  a38888f27160e7fa["tests/test-external-folders/test.sh"];
  class a38888f27160e7fa default;
  click a38888f27160e7fa "../../tests/test-external-folders/test.sh";
  97e9fdec98b5f3e5 -.->|trace| a38888f27160e7fa;
```

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
  * verify: [SystemRequirements/Requirements.md#Requirements Processing](../SystemRequirements/Requirements.md#requirements-processing)
  * trace: [tests/test-external-folders/test.sh](../../tests/test-external-folders/test.sh)

---

## Excluded Patterns Tests
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  excluded_patterns_verification["Excluded Patterns Verification"];
  click excluded_patterns_verification "LintingTests.md#excluded-patterns-verification";
  class excluded_patterns_verification verification;
  excluded_patterns_requirement["SystemRequirements/Requirements.md#Configurable Filename Exclusion Patterns"];
  class excluded_patterns_requirement requirement;
  click excluded_patterns_requirement "../SystemRequirements/Requirements.md#configurable-filename-exclusion-patterns";
  excluded_patterns_verification -.->|verifies| excluded_patterns_requirement;
  excluded_patterns_test["tests/test-excluded-patterns/test.sh"];
  class excluded_patterns_test default;
  click excluded_patterns_test "../../tests/test-excluded-patterns/test.sh";
  excluded_patterns_verification -.->|trace| excluded_patterns_test;
```

---

### Excluded Patterns Verification

This test verifies that the system correctly handles excluded filename patterns in configuration.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- Files matching excluded_filename_patterns should not be processed for validation
- Files matching excluded_filename_patterns should only be tracked for relation targets
- Only relations TO excluded files should be valid, not relations FROM excluded files

##### Test Criteria
- Command should not validate elements within files matching excluded patterns
- Elements in excluded files should not be in registry for direct access
- Only the file itself should be in the registry for relation validation

##### Test Procedure
1. Create configuration with excluded_filename_patterns set
2. Create test fixtures with files matching and not matching excluded patterns
3. Run ReqFlow validation on the test fixtures
4. Verify that excluded files are not processed for validation
5. Verify that relations to excluded files are still considered valid

#### Relations
  * verify: [SystemRequirements/Requirements.md#Configurable Filename Exclusion Patterns](../SystemRequirements/Requirements.md#configurable-filename-exclusion-patterns)
  * verify: [SystemRequirements/Requirements.md#File Pattern Exclusion for Linting](../SystemRequirements/Requirements.md#file-pattern-exclusion-for-linting)
  * trace: [tests/test-excluded-patterns/test.sh](../../tests/test-excluded-patterns/test.sh)

---

### Excluded Linting Verification

This test verifies that the system's linting functionality correctly respects excluded filename patterns.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- Files matching excluded_filename_patterns should not be linted
- ReqFlow should skip linting checks on excluded files

##### Test Criteria
- Command should not lint files matching excluded patterns
- Linting output should not include issues from excluded files

##### Test Procedure
1. Create configuration with excluded_filename_patterns set
2. Create test fixtures with files matching and not matching excluded patterns
3. Run ReqFlow linting on the test fixtures
4. Verify that excluded files are not included in linting output
5. Verify that no linting errors are reported for excluded files

#### Relations
  * verify: [SystemRequirements/Requirements.md#Configurable Filename Exclusion Patterns](../SystemRequirements/Requirements.md#configurable-filename-exclusion-patterns)
  * verify: [SystemRequirements/Requirements.md#File Pattern Exclusion for Linting](../SystemRequirements/Requirements.md#file-pattern-exclusion-for-linting)
  * trace: [tests/test-excluded-linting/test.sh](../../tests/test-excluded-linting/test.sh)