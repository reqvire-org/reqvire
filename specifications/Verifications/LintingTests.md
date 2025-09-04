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

  37dbb69e504fec9d["Excess Whitespace Detection and Correction"];
  class 37dbb69e504fec9d verification;
  click 37dbb69e504fec9d "LintingTests.md#excess-whitespace-detection-and-correction";
  9f473f6e0b993cac["SystemRequirements/Requirements.md/Excess Whitespace Linting Implementation"];
  class 9f473f6e0b993cac requirement;
  click 9f473f6e0b993cac "../SystemRequirements/Requirements.md#excess-whitespace-linting-implementation";
  37dbb69e504fec9d -.->|verifies| 9f473f6e0b993cac;
  deaec107945edbed["SystemRequirements/Requirements.md/Lint Command"];
  class deaec107945edbed requirement;
  click deaec107945edbed "../SystemRequirements/Requirements.md#lint-command";
  37dbb69e504fec9d -.->|verifies| deaec107945edbed;
  a606311f33e42901["tests/test-lint-expected/test.sh"];
  class a606311f33e42901 default;
  click a606311f33e42901 "../../tests/test-lint-expected/test.sh";
  a606311f33e42901 -->|satisfies| 37dbb69e504fec9d;
  706dca24b7cc1eb8["CLI Lint Flag Test"];
  class 706dca24b7cc1eb8 verification;
  click 706dca24b7cc1eb8 "LintingTests.md#cli-lint-flag-test";
  706dca24b7cc1eb8 -.->|verifies| deaec107945edbed;
  a606311f33e42901["tests/test-lint-expected/test.sh"];
  class a606311f33e42901 default;
  click a606311f33e42901 "../../tests/test-lint-expected/test.sh";
  a606311f33e42901 -->|satisfies| 706dca24b7cc1eb8;
  6481e4ca8c4d6920["Model Linting Verification"];
  class 6481e4ca8c4d6920 verification;
  click 6481e4ca8c4d6920 "LintingTests.md#model-linting-verification";
  7305c1d6f7f1e2b2["UserRequirements.md/Model Linting"];
  class 7305c1d6f7f1e2b2 requirement;
  click 7305c1d6f7f1e2b2 "../UserRequirements.md#model-linting";
  6481e4ca8c4d6920 -.->|verifies| 7305c1d6f7f1e2b2;
  a606311f33e42901["tests/test-lint-expected/test.sh"];
  class a606311f33e42901 default;
  click a606311f33e42901 "../../tests/test-lint-expected/test.sh";
  a606311f33e42901 -->|satisfies| 6481e4ca8c4d6920;
  b8bfbd5ccf026b31["Format Consistency Verification"];
  class b8bfbd5ccf026b31 verification;
  click b8bfbd5ccf026b31 "LintingTests.md#format-consistency-verification";
  974ccf933675ef44["UserRequirements.md/Format Consistency Enforcement"];
  class 974ccf933675ef44 requirement;
  click 974ccf933675ef44 "../UserRequirements.md#format-consistency-enforcement";
  b8bfbd5ccf026b31 -.->|verifies| 974ccf933675ef44;
  a606311f33e42901["tests/test-lint-expected/test.sh"];
  class a606311f33e42901 default;
  click a606311f33e42901 "../../tests/test-lint-expected/test.sh";
  a606311f33e42901 -->|satisfies| b8bfbd5ccf026b31;
  a8037b2df81f02be["Linting Command Verification"];
  class a8037b2df81f02be verification;
  click a8037b2df81f02be "LintingTests.md#linting-command-verification";
  a51179cda67cf9f2["UserRequirements.md/Linting Command"];
  class a51179cda67cf9f2 requirement;
  click a51179cda67cf9f2 "../UserRequirements.md#linting-command";
  a8037b2df81f02be -.->|verifies| a51179cda67cf9f2;
  a606311f33e42901["tests/test-lint-expected/test.sh"];
  class a606311f33e42901 default;
  click a606311f33e42901 "../../tests/test-lint-expected/test.sh";
  a606311f33e42901 -->|satisfies| a8037b2df81f02be;
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

#### Relations
  * verify: [SystemRequirements/Requirements.md/Excess Whitespace Linting Implementation](../SystemRequirements/Requirements.md#excess-whitespace-linting-implementation)
  * verify: [SystemRequirements/Requirements.md/Lint Command](../SystemRequirements/Requirements.md#lint-command)
  * satisfiedBy: [tests/test-lint-expected/test.sh](../../tests/test-lint-expected/test.sh)

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

#### Relations
  * verify: [UserRequirements.md/Linting Command](../UserRequirements.md#linting-command)
  * satisfiedBy: [tests/test-lint-expected/test.sh](../../tests/test-lint-expected/test.sh)

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

#### Relations
  * verify: [UserRequirements.md/Format Consistency Enforcement](../UserRequirements.md#format-consistency-enforcement)
  * satisfiedBy: [tests/test-lint-expected/test.sh](../../tests/test-lint-expected/test.sh)

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

#### Relations
  * verify: [UserRequirements.md/Model Linting](../UserRequirements.md#model-linting)
  * satisfiedBy: [tests/test-lint-expected/test.sh](../../tests/test-lint-expected/test.sh)

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

#### Relations
  * verify: [SystemRequirements/Requirements.md#lint-command](../SystemRequirements/Requirements.md#lint-command)
  * satisfiedBy: [tests/test-lint-expected/test.sh](../../tests/test-lint-expected/test.sh)

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
  click 5fac8f1060a40eb9 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/Verifications/LintingTests.md#external-folders-support-verification";
  class 5fac8f1060a40eb9 verification;
  66080aef4185b07d["SystemRequirements/Requirements.md#External Folders Support"];
  class 66080aef4185b07d requirement;
  click 66080aef4185b07d "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/SystemRequirements/Requirements.md#external-folders-support";
  5fac8f1060a40eb9 -.->|verifies| 66080aef4185b07d;
  f24f11691f55af62["SystemRequirements/Requirements.md#Requirements Processing"];
  class f24f11691f55af62 requirement;
  click f24f11691f55af62 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/SystemRequirements/Requirements.md#requirements-processing";
  5fac8f1060a40eb9 -.->|verifies| f24f11691f55af62;
  aaaee4f2b1759d4d["tests/test-external-folders/test.sh"];
  class aaaee4f2b1759d4d default;
  click aaaee4f2b1759d4d "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/tests/test-external-folders/test.sh";
  aaaee4f2b1759d4d -->|satisfies| 5fac8f1060a40eb9;
```

---

## Excluded Patterns Tests
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  71b5a1d5e278aa8e["Excluded Linting Verification"];
  class 71b5a1d5e278aa8e verification;
  click 71b5a1d5e278aa8e "LintingTests.md#excluded-linting-verification";
  be83c2991e9535c7["SystemRequirements/Requirements.md#ignoring-unstructured-documents"];
  class be83c2991e9535c7 requirement;
  click be83c2991e9535c7 "../SystemRequirements/Requirements.md#ignoring-unstructured-documents";
  71b5a1d5e278aa8e -.->|verifies| be83c2991e9535c7;
  bef37c31db69b66a["SystemRequirements/Requirements.md#File Pattern Exclusion for Linting"];
  class bef37c31db69b66a requirement;
  click bef37c31db69b66a "../SystemRequirements/Requirements.md#file-pattern-exclusion-for-linting";
  71b5a1d5e278aa8e -.->|verifies| bef37c31db69b66a;
  de8bc63ebb2a09e7["tests/test-excluded-linting/test.sh"];
  class de8bc63ebb2a09e7 default;
  click de8bc63ebb2a09e7 "../../tests/test-excluded-linting/test.sh";
  de8bc63ebb2a09e7 -->|satisfies| 71b5a1d5e278aa8e;
  590a4cc1c558992f["Excluded Patterns Verification"];
  class 590a4cc1c558992f verification;
  click 590a4cc1c558992f "LintingTests.md#excluded-patterns-verification";
  590a4cc1c558992f -.->|verifies| be83c2991e9535c7;
  590a4cc1c558992f -.->|verifies| bef37c31db69b66a;
  a29e69e90fa71f39["tests/test-excluded-patterns/test.sh"];
  class a29e69e90fa71f39 default;
  click a29e69e90fa71f39 "../../tests/test-excluded-patterns/test.sh";
  a29e69e90fa71f39 -->|satisfies| 590a4cc1c558992f;
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

#### Relations
  * verify: [SystemRequirements/Requirements.md#ignoring-unstructured-documents](../SystemRequirements/Requirements.md#ignoring-unstructured-documents)
  * verify: [SystemRequirements/Requirements.md#File Pattern Exclusion for Linting](../SystemRequirements/Requirements.md#file-pattern-exclusion-for-linting)
  * satisfiedBy: [tests/test-excluded-patterns/test.sh](../../tests/test-excluded-patterns/test.sh)

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

#### Relations
  * verify: [SystemRequirements/Requirements.md#ignoring-unstructured-documents](../SystemRequirements/Requirements.md#ignoring-unstructured-documents)
  * verify: [SystemRequirements/Requirements.md#File Pattern Exclusion for Linting](../SystemRequirements/Requirements.md#file-pattern-exclusion-for-linting)
  * satisfiedBy: [tests/test-excluded-linting/test.sh](../../tests/test-excluded-linting/test.sh)