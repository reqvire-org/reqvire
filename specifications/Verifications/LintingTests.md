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

  80dc4150bd3cbdba["Format Consistency Verification"];
  click 80dc4150bd3cbdba "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/LintingTests.md#format-consistency-verification";
  class 80dc4150bd3cbdba verification;
  b692557f47cee0f7["UserRequirements.md/Format Consistency Enforcement"];
  class b692557f47cee0f7 requirement;
  click b692557f47cee0f7 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#format-consistency-enforcement";
  80dc4150bd3cbdba -.->|verifies| b692557f47cee0f7;
  8e125941dffc5ae9["tests/test-lint-expected/test.sh"];
  class 8e125941dffc5ae9 default;
  click 8e125941dffc5ae9 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/tests/test-lint-expected/test.sh";
  80dc4150bd3cbdba -.->|trace| 8e125941dffc5ae9;
  b38613988d69bc35["Excess Whitespace Detection and Correction"];
  click b38613988d69bc35 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/LintingTests.md#excess-whitespace-detection-and-correction";
  class b38613988d69bc35 verification;
  51b41cf76fdb8ffc["SystemRequirements/Requirements.md/Excess Whitespace Linting Implementation"];
  class 51b41cf76fdb8ffc requirement;
  click 51b41cf76fdb8ffc "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#excess-whitespace-linting-implementation";
  b38613988d69bc35 -.->|verifies| 51b41cf76fdb8ffc;
  bd9d6fae737d418f["SystemRequirements/Requirements.md/Dry Run Mode"];
  class bd9d6fae737d418f requirement;
  click bd9d6fae737d418f "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#dry-run-mode";
  b38613988d69bc35 -.->|verifies| bd9d6fae737d418f;
  b38613988d69bc35 -.->|trace| 8e125941dffc5ae9;
  86f2e734068525a9["Model Linting Verification"];
  click 86f2e734068525a9 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/LintingTests.md#model-linting-verification";
  class 86f2e734068525a9 verification;
  a479ae0b8d8c4fce["UserRequirements.md/Model Linting"];
  class a479ae0b8d8c4fce requirement;
  click a479ae0b8d8c4fce "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#model-linting";
  86f2e734068525a9 -.->|verifies| a479ae0b8d8c4fce;
  86f2e734068525a9 -.->|trace| 8e125941dffc5ae9;
  a1a9644ad79a3c0e["Linting Command Verification"];
  click a1a9644ad79a3c0e "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/LintingTests.md#linting-command-verification";
  class a1a9644ad79a3c0e verification;
  51a11693af2e41fb["UserRequirements.md/Linting Command"];
  class 51a11693af2e41fb requirement;
  click 51a11693af2e41fb "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#linting-command";
  a1a9644ad79a3c0e -.->|verifies| 51a11693af2e41fb;
  a1a9644ad79a3c0e -.->|trace| 8e125941dffc5ae9;
  3ef728b41be431ea["CLI Lint Flag Test"];
  click 3ef728b41be431ea "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/LintingTests.md#cli-lint-flag-test";
  class 3ef728b41be431ea verification;
  32d2c19b923c0ac7["SystemRequirements/Requirements.md#cli-lint-flag"];
  class 32d2c19b923c0ac7 requirement;
  click 32d2c19b923c0ac7 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#cli-lint-flag";
  3ef728b41be431ea -.->|verifies| 32d2c19b923c0ac7;
  3ef728b41be431ea -.->|trace| 8e125941dffc5ae9;
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

  94db2f7249338823["External Folders Support Verification"];
  click 94db2f7249338823 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/LintingTests.md#external-folders-support-verification";
  class 94db2f7249338823 verification;
  854c934a3e442261["SystemRequirements/Requirements.md#External Folders Support"];
  class 854c934a3e442261 requirement;
  click 854c934a3e442261 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#external-folders-support";
  94db2f7249338823 -.->|verifies| 854c934a3e442261;
  6aeba4bf990bc9e4["SystemRequirements/Requirements.md#Requirements Processing"];
  class 6aeba4bf990bc9e4 requirement;
  click 6aeba4bf990bc9e4 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#requirements-processing";
  94db2f7249338823 -.->|verifies| 6aeba4bf990bc9e4;
  cefc0e47d5306f53["tests/test-external-folders/test.sh"];
  class cefc0e47d5306f53 default;
  click cefc0e47d5306f53 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/tests/test-external-folders/test.sh";
  94db2f7249338823 -.->|trace| cefc0e47d5306f53;
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

  a49974e0f5470954["Excluded Patterns Verification"];
  click a49974e0f5470954 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/LintingTests.md#excluded-patterns-verification";
  class a49974e0f5470954 verification;
  348451e9313c44a3["SystemRequirements/Requirements.md#Configurable Filename Exclusion Patterns"];
  class 348451e9313c44a3 requirement;
  click 348451e9313c44a3 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#configurable-filename-exclusion-patterns";
  a49974e0f5470954 -.->|verifies| 348451e9313c44a3;
  c2d5791e32e75081["SystemRequirements/Requirements.md#File Pattern Exclusion for Linting"];
  class c2d5791e32e75081 requirement;
  click c2d5791e32e75081 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#file-pattern-exclusion-for-linting";
  a49974e0f5470954 -.->|verifies| c2d5791e32e75081;
  12603942d997b0fa["tests/test-excluded-patterns/test.sh"];
  class 12603942d997b0fa default;
  click 12603942d997b0fa "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/tests/test-excluded-patterns/test.sh";
  a49974e0f5470954 -.->|trace| 12603942d997b0fa;
  18222f46dc9c9778["Excluded Linting Verification"];
  click 18222f46dc9c9778 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/LintingTests.md#excluded-linting-verification";
  class 18222f46dc9c9778 verification;
  18222f46dc9c9778 -.->|verifies| 348451e9313c44a3;
  18222f46dc9c9778 -.->|verifies| c2d5791e32e75081;
  8f580a92cf160f78["tests/test-excluded-linting/test.sh"];
  class 8f580a92cf160f78 default;
  click 8f580a92cf160f78 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/tests/test-excluded-linting/test.sh";
  18222f46dc9c9778 -.->|trace| 8f580a92cf160f78;
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