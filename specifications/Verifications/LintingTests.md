# Linting Tests

This document verifies the requirements for ReqFlow's linting functionality.

## Linting Functionality Tests
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  3bbd3815f37ee9ae["Linting Command Verification"];
  click 3bbd3815f37ee9ae "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/Verifications/LintingTests.md#linting-command-verification";
  class 3bbd3815f37ee9ae verification;
  887a7d36caacab2b["UserRequirements.md/Linting Command"];
  class 887a7d36caacab2b requirement;
  click 887a7d36caacab2b "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#linting-command";
  3bbd3815f37ee9ae -.->|verifies| 887a7d36caacab2b;
  3de539f546654b01["tests/test-lint-expected/test.sh"];
  class 3de539f546654b01 default;
  click 3de539f546654b01 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/tests/test-lint-expected/test.sh";
  3bbd3815f37ee9ae -.->|trace| 3de539f546654b01;
  746b65fa854bc876["CLI Lint Flag Test"];
  click 746b65fa854bc876 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/Verifications/LintingTests.md#cli-lint-flag-test";
  class 746b65fa854bc876 verification;
  fffbb227966be4f2["SystemRequirements/Requirements.md#cli-lint-flag"];
  class fffbb227966be4f2 requirement;
  click fffbb227966be4f2 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#cli-lint-flag";
  746b65fa854bc876 -.->|verifies| fffbb227966be4f2;
  746b65fa854bc876 -.->|trace| 3de539f546654b01;
  d941947fb524f8c7["Model Linting Verification"];
  click d941947fb524f8c7 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/Verifications/LintingTests.md#model-linting-verification";
  class d941947fb524f8c7 verification;
  103ddb8dc3242215["UserRequirements.md/Model Linting"];
  class 103ddb8dc3242215 requirement;
  click 103ddb8dc3242215 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#model-linting";
  d941947fb524f8c7 -.->|verifies| 103ddb8dc3242215;
  d941947fb524f8c7 -.->|trace| 3de539f546654b01;
  4cd0fe733f182d5c["Excess Whitespace Detection and Correction"];
  click 4cd0fe733f182d5c "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/Verifications/LintingTests.md#excess-whitespace-detection-and-correction";
  class 4cd0fe733f182d5c verification;
  3f3d3f9ccb15029a["SystemRequirements/Requirements.md/Excess Whitespace Linting Implementation"];
  class 3f3d3f9ccb15029a requirement;
  click 3f3d3f9ccb15029a "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#excess-whitespace-linting-implementation";
  4cd0fe733f182d5c -.->|verifies| 3f3d3f9ccb15029a;
  8f2198f681f63fea["SystemRequirements/Requirements.md/Dry Run Mode"];
  class 8f2198f681f63fea requirement;
  click 8f2198f681f63fea "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#dry-run-mode";
  4cd0fe733f182d5c -.->|verifies| 8f2198f681f63fea;
  4cd0fe733f182d5c -.->|trace| 3de539f546654b01;
  80dbe2ea823992bc["Format Consistency Verification"];
  click 80dbe2ea823992bc "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/Verifications/LintingTests.md#format-consistency-verification";
  class 80dbe2ea823992bc verification;
  81758bdb22a3329d["UserRequirements.md/Format Consistency Enforcement"];
  class 81758bdb22a3329d requirement;
  click 81758bdb22a3329d "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#format-consistency-enforcement";
  80dbe2ea823992bc -.->|verifies| 81758bdb22a3329d;
  80dbe2ea823992bc -.->|trace| 3de539f546654b01;
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

### Linting Command Verification

This test verifies that the system properly implements the linting command functionality with both automatic fix and preview options.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should provide a linting command that can automatically apply fixes
- System should offer a preview option to show changes without applying them
- Linting command should be activated via the --lint flag
- Dry run mode should be activated via the --dry-run flag

##### Test Criteria
- Command exits with success (0) return code
- Linting fixes are properly applied in normal mode
- Changes are only displayed but not applied in dry-run mode
- The command provides clear output about what changes would be or have been made

##### Test Procedure
TODO: write test procedure

#### Relations
  * verify: [UserRequirements.md/Linting Command](../UserRequirements.md#linting-command)
  * trace: [tests/test-lint-expected/test.sh](../../tests/test-lint-expected/test.sh)

---

### Format Consistency Verification

This test verifies that the system properly enforces consistent formatting in requirements documents.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should detect and fix excess whitespace after element names and relation identifiers
- System should normalize to exactly two newlines before subsections
- System should automatically insert separator lines between elements if not present
- System should ensure consistent indentation in relation lists

##### Test Criteria
- Command exits with success (0) return code
- Formatting issues are detected and fixed according to the standards
- The fixed documents maintain proper structure and content

##### Test Procedure
TODO: write test procedure

#### Relations
  * verify: [UserRequirements.md/Format Consistency Enforcement](../UserRequirements.md#format-consistency-enforcement)
  * trace: [tests/test-lint-expected/test.sh](../../tests/test-lint-expected/test.sh)

---

### Model Linting Verification

This test verifies that the system provides linting capabilities to identify and fix stylistic and non-critical issues in MBSE models.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should identify stylistic and formatting issues in MBSE models
- System should fix non-critical issues without affecting functional integrity
- System should provide clear reporting of the issues it identifies

##### Test Criteria
- Command exits with success (0) return code
- Linting identifies all expected issue types across test files
- Fixes are appropriately applied without breaking model structure

##### Test Procedure
TODO: write test procedure

#### Relations
  * verify: [UserRequirements.md/Model Linting](../UserRequirements.md#model-linting)
  * trace: [tests/test-lint-expected/test.sh](../../tests/test-lint-expected/test.sh)

---

### CLI Lint Flag Test

This test verifies that the system provides a linting function activated by the --lint flag.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System shall provide a --lint flag in CLI
- Flag shall activate the linting process
- Linting shall execute upon user request
- Process shall apply fixes by default

##### Test Criteria
- Command with --lint flag runs successfully
- Linting process is executed on target files
- Linting issues are identified and fixed
- Changes are applied to files

##### Test Procedure
1. Create test fixtures with known linting issues (excess whitespace, inconsistent newlines, etc.)
2. Run ReqFlow with --lint flag on the test fixtures
3. Verify that linting process is executed and files are modified
4. Compare output files with expected fixed versions
5. Run with --lint --dry-run and verify that issues are identified but files aren't changed

#### Relations
  * verify: [SystemRequirements/Requirements.md#cli-lint-flag](../SystemRequirements/Requirements.md#cli-lint-flag)
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
  click 97e9fdec98b5f3e5 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/Verifications/LintingTests.md#external-folders-support-verification";
  class 97e9fdec98b5f3e5 verification;
  d38ab4ad139183d3["SystemRequirements/Requirements.md#External Folders Support"];
  class d38ab4ad139183d3 requirement;
  click d38ab4ad139183d3 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#external-folders-support";
  97e9fdec98b5f3e5 -.->|verifies| d38ab4ad139183d3;
  99bed90a0d96a1d2["SystemRequirements/Requirements.md#Requirements Processing"];
  class 99bed90a0d96a1d2 requirement;
  click 99bed90a0d96a1d2 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#requirements-processing";
  97e9fdec98b5f3e5 -.->|verifies| 99bed90a0d96a1d2;
  a38888f27160e7fa["tests/test-external-folders/test.sh"];
  class a38888f27160e7fa default;
  click a38888f27160e7fa "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/tests/test-external-folders/test.sh";
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

  be641c88c635747f["Excluded Patterns Verification"];
  click be641c88c635747f "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/Verifications/LintingTests.md#excluded-patterns-verification";
  class be641c88c635747f verification;
  cc8128cae305b29d["SystemRequirements/Requirements.md#Configurable Filename Exclusion Patterns"];
  class cc8128cae305b29d requirement;
  click cc8128cae305b29d "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#configurable-filename-exclusion-patterns";
  be641c88c635747f -.->|verifies| cc8128cae305b29d;
  45770e9b319d3819["SystemRequirements/Requirements.md#File Pattern Exclusion for Linting"];
  class 45770e9b319d3819 requirement;
  click 45770e9b319d3819 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#file-pattern-exclusion-for-linting";
  be641c88c635747f -.->|verifies| 45770e9b319d3819;
  79ca606f26bdd145["tests/test-excluded-patterns/test.sh"];
  class 79ca606f26bdd145 default;
  click 79ca606f26bdd145 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/tests/test-excluded-patterns/test.sh";
  be641c88c635747f -.->|trace| 79ca606f26bdd145;
  4c4775ba9ffce94["Excluded Linting Verification"];
  click 4c4775ba9ffce94 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/Verifications/LintingTests.md#excluded-linting-verification";
  class 4c4775ba9ffce94 verification;
  4c4775ba9ffce94 -.->|verifies| cc8128cae305b29d;
  4c4775ba9ffce94 -.->|verifies| 45770e9b319d3819;
  b28a9ed0b063b190["tests/test-excluded-linting/test.sh"];
  class b28a9ed0b063b190 default;
  click b28a9ed0b063b190 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/tests/test-excluded-linting/test.sh";
  4c4775ba9ffce94 -.->|trace| b28a9ed0b063b190;
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