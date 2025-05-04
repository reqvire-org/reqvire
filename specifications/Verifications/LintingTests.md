# Linting Tests

This document verifies the requirements for Reqvire's linting functionality.

## Linting Functionality Tests
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  fbe07d524dbc79f7["Linting Command Verification"];
  click fbe07d524dbc79f7 "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/Verifications/LintingTests.md#linting-command-verification";
  class fbe07d524dbc79f7 verification;
  28b0f9fa78937e61["UserRequirements.md/Linting Command"];
  class 28b0f9fa78937e61 requirement;
  click 28b0f9fa78937e61 "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/UserRequirements.md#linting-command";
  fbe07d524dbc79f7 -.->|verifies| 28b0f9fa78937e61;
  569c6cdb3294a8d5["tests/test-lint-expected/test.sh"];
  class 569c6cdb3294a8d5 default;
  click 569c6cdb3294a8d5 "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/tests/test-lint-expected/test.sh";
  fbe07d524dbc79f7 -.->|trace| 569c6cdb3294a8d5;
  4ec0755de791b8c6["CLI Lint Flag Test"];
  click 4ec0755de791b8c6 "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/Verifications/LintingTests.md#cli-lint-flag-test";
  class 4ec0755de791b8c6 verification;
  3f235c1000d5347f["SystemRequirements/Requirements.md#cli-lint-flag"];
  class 3f235c1000d5347f requirement;
  click 3f235c1000d5347f "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/SystemRequirements/Requirements.md#cli-lint-flag";
  4ec0755de791b8c6 -.->|verifies| 3f235c1000d5347f;
  4ec0755de791b8c6 -.->|trace| 569c6cdb3294a8d5;
  716f75498f400b63["Excess Whitespace Detection and Correction"];
  click 716f75498f400b63 "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/Verifications/LintingTests.md#excess-whitespace-detection-and-correction";
  class 716f75498f400b63 verification;
  ab8dfb01e717d34["SystemRequirements/Requirements.md/Excess Whitespace Linting Implementation"];
  class ab8dfb01e717d34 requirement;
  click ab8dfb01e717d34 "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/SystemRequirements/Requirements.md#excess-whitespace-linting-implementation";
  716f75498f400b63 -.->|verifies| ab8dfb01e717d34;
  d21b16b30de7350d["SystemRequirements/Requirements.md/Dry Run Mode"];
  class d21b16b30de7350d requirement;
  click d21b16b30de7350d "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/SystemRequirements/Requirements.md#dry-run-mode";
  716f75498f400b63 -.->|verifies| d21b16b30de7350d;
  716f75498f400b63 -.->|trace| 569c6cdb3294a8d5;
  90b51d9c48bc542d["Model Linting Verification"];
  click 90b51d9c48bc542d "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/Verifications/LintingTests.md#model-linting-verification";
  class 90b51d9c48bc542d verification;
  84c4dc11e82e8638["UserRequirements.md/Model Linting"];
  class 84c4dc11e82e8638 requirement;
  click 84c4dc11e82e8638 "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/UserRequirements.md#model-linting";
  90b51d9c48bc542d -.->|verifies| 84c4dc11e82e8638;
  90b51d9c48bc542d -.->|trace| 569c6cdb3294a8d5;
  f35ede8f8934159a["Format Consistency Verification"];
  click f35ede8f8934159a "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/Verifications/LintingTests.md#format-consistency-verification";
  class f35ede8f8934159a verification;
  1ddbeea0cf8eaad5["UserRequirements.md/Format Consistency Enforcement"];
  class 1ddbeea0cf8eaad5 requirement;
  click 1ddbeea0cf8eaad5 "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/UserRequirements.md#format-consistency-enforcement";
  f35ede8f8934159a -.->|verifies| 1ddbeea0cf8eaad5;
  f35ede8f8934159a -.->|trace| 569c6cdb3294a8d5;
```

---

### Excess Whitespace Detection and Correction

The verification test checks that Reqvire correctly identifies and fixes excess whitespace after headers.

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
2. Run Reqvire linting in dry-run mode to check detection
3. Verify that excess whitespace is reported
4. Run Reqvire linting in normal mode to apply fixes
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
2. Run Reqvire with --lint flag on the test fixtures
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

  5fac8f1060a40eb9["External Folders Support Verification"];
  click 5fac8f1060a40eb9 "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/Verifications/LintingTests.md#external-folders-support-verification";
  class 5fac8f1060a40eb9 verification;
  66080aef4185b07d["SystemRequirements/Requirements.md#External Folders Support"];
  class 66080aef4185b07d requirement;
  click 66080aef4185b07d "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/SystemRequirements/Requirements.md#external-folders-support";
  5fac8f1060a40eb9 -.->|verifies| 66080aef4185b07d;
  f24f11691f55af62["SystemRequirements/Requirements.md#Requirements Processing"];
  class f24f11691f55af62 requirement;
  click f24f11691f55af62 "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/SystemRequirements/Requirements.md#requirements-processing";
  5fac8f1060a40eb9 -.->|verifies| f24f11691f55af62;
  aaaee4f2b1759d4d["tests/test-external-folders/test.sh"];
  class aaaee4f2b1759d4d default;
  click aaaee4f2b1759d4d "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/tests/test-external-folders/test.sh";
  5fac8f1060a40eb9 -.->|trace| aaaee4f2b1759d4d;
```

---

### External Folders Support Verification

The verification test checks that Reqvire correctly processes requirements in external folders.

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
2. Run Reqvire with invalid setup (user requirements in external folder)
3. Verify that validation reports the appropriate error
4. Run Reqvire with valid setup (only system requirements in external folder)
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

  c63c04402c100a4a["Excluded Linting Verification"];
  click c63c04402c100a4a "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/Verifications/LintingTests.md#excluded-linting-verification";
  class c63c04402c100a4a verification;
  4bf74699f40f4a76["SystemRequirements/Requirements.md#Configurable Filename Exclusion Patterns"];
  class 4bf74699f40f4a76 requirement;
  click 4bf74699f40f4a76 "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/SystemRequirements/Requirements.md#configurable-filename-exclusion-patterns";
  c63c04402c100a4a -.->|verifies| 4bf74699f40f4a76;
  48e8a0b4b18111c4["SystemRequirements/Requirements.md#File Pattern Exclusion for Linting"];
  class 48e8a0b4b18111c4 requirement;
  click 48e8a0b4b18111c4 "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/SystemRequirements/Requirements.md#file-pattern-exclusion-for-linting";
  c63c04402c100a4a -.->|verifies| 48e8a0b4b18111c4;
  b2805e4d183be784["tests/test-excluded-linting/test.sh"];
  class b2805e4d183be784 default;
  click b2805e4d183be784 "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/tests/test-excluded-linting/test.sh";
  c63c04402c100a4a -.->|trace| b2805e4d183be784;
  3cea1c7ac2f7a8c5["Excluded Patterns Verification"];
  click 3cea1c7ac2f7a8c5 "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/Verifications/LintingTests.md#excluded-patterns-verification";
  class 3cea1c7ac2f7a8c5 verification;
  3cea1c7ac2f7a8c5 -.->|verifies| 4bf74699f40f4a76;
  3cea1c7ac2f7a8c5 -.->|verifies| 48e8a0b4b18111c4;
  ec11a68aa5b4bdc1["tests/test-excluded-patterns/test.sh"];
  class ec11a68aa5b4bdc1 default;
  click ec11a68aa5b4bdc1 "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/tests/test-excluded-patterns/test.sh";
  3cea1c7ac2f7a8c5 -.->|trace| ec11a68aa5b4bdc1;
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
3. Run Reqvire validation on the test fixtures
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
- Reqvire should skip linting checks on excluded files

##### Test Criteria
- Command should not lint files matching excluded patterns
- Linting output should not include issues from excluded files

##### Test Procedure
1. Create configuration with excluded_filename_patterns set
2. Create test fixtures with files matching and not matching excluded patterns
3. Run Reqvire linting on the test fixtures
4. Verify that excluded files are not included in linting output
5. Verify that no linting errors are reported for excluded files

#### Relations
  * verify: [SystemRequirements/Requirements.md#Configurable Filename Exclusion Patterns](../SystemRequirements/Requirements.md#configurable-filename-exclusion-patterns)
  * verify: [SystemRequirements/Requirements.md#File Pattern Exclusion for Linting](../SystemRequirements/Requirements.md#file-pattern-exclusion-for-linting)
  * trace: [tests/test-excluded-linting/test.sh](../../tests/test-excluded-linting/test.sh)