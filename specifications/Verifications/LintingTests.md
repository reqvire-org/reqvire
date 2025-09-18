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

  6481e4ca8c4d6920["Model Linting Verification"];
  class 6481e4ca8c4d6920 verification;
  click 6481e4ca8c4d6920 "LintingTests.md#model-linting-verification";
  a606311f33e42901["tests/test-lint-expected/test.sh"];
  class a606311f33e42901 default;
  click a606311f33e42901 "../../tests/test-lint-expected/test.sh";
  6481e4ca8c4d6920 -->|satisfiedBy| a606311f33e42901;
  a8037b2df81f02be["Linting Command Verification"];
  class a8037b2df81f02be verification;
  click a8037b2df81f02be "LintingTests.md#linting-command-verification";
  a606311f33e42901["tests/test-lint-expected/test.sh"];
  class a606311f33e42901 default;
  click a606311f33e42901 "../../tests/test-lint-expected/test.sh";
  a8037b2df81f02be -->|satisfiedBy| a606311f33e42901;
  b8bfbd5ccf026b31["Format Consistency Verification"];
  class b8bfbd5ccf026b31 verification;
  click b8bfbd5ccf026b31 "LintingTests.md#format-consistency-verification";
  a606311f33e42901["tests/test-lint-expected/test.sh"];
  class a606311f33e42901 default;
  click a606311f33e42901 "../../tests/test-lint-expected/test.sh";
  b8bfbd5ccf026b31 -->|satisfiedBy| a606311f33e42901;
  706dca24b7cc1eb8["CLI Lint Flag Test"];
  class 706dca24b7cc1eb8 verification;
  click 706dca24b7cc1eb8 "LintingTests.md#cli-lint-flag-test";
  a606311f33e42901["tests/test-lint-expected/test.sh"];
  class a606311f33e42901 default;
  click a606311f33e42901 "../../tests/test-lint-expected/test.sh";
  706dca24b7cc1eb8 -->|satisfiedBy| a606311f33e42901;
  37dbb69e504fec9d["Excess Whitespace Detection and Correction"];
  class 37dbb69e504fec9d verification;
  click 37dbb69e504fec9d "LintingTests.md#excess-whitespace-detection-and-correction";
  a606311f33e42901["tests/test-lint-expected/test.sh"];
  class a606311f33e42901 default;
  click a606311f33e42901 "../../tests/test-lint-expected/test.sh";
  37dbb69e504fec9d -->|satisfiedBy| a606311f33e42901;
  9f473f6e0b993cac["Excess Whitespace Linting Implementation"];
  class 9f473f6e0b993cac requirement;
  click 9f473f6e0b993cac "../SystemRequirements/Requirements.md#excess-whitespace-linting-implementation";
  63a0f5a33e87fcee["linting/whitespace.rs"];
  class 63a0f5a33e87fcee default;
  click 63a0f5a33e87fcee "../../core/src/linting/whitespace.rs";
  9f473f6e0b993cac -->|satisfiedBy| 63a0f5a33e87fcee;
  9f473f6e0b993cac -.->|verifiedBy| 37dbb69e504fec9d;
  a51179cda67cf9f2["Linting Command"];
  class a51179cda67cf9f2 requirement;
  click a51179cda67cf9f2 "../UserRequirements.md#linting-command";
  a51179cda67cf9f2 -.->|verifiedBy| a8037b2df81f02be;
  26fdf88d16b09109["Linting Output"];
  class 26fdf88d16b09109 requirement;
  click 26fdf88d16b09109 "../UserRequirements.md#linting-output";
  a51179cda67cf9f2 -->|refinedBy| 26fdf88d16b09109;
  deaec107945edbed["Lint Command"];
  class deaec107945edbed requirement;
  click deaec107945edbed "../SystemRequirements/Requirements.md#lint-command";
  a51179cda67cf9f2 -.->|deriveReqT| deaec107945edbed;
  7305c1d6f7f1e2b2["Model Linting"];
  class 7305c1d6f7f1e2b2 requirement;
  click 7305c1d6f7f1e2b2 "../UserRequirements.md#model-linting";
  7305c1d6f7f1e2b2 --o|contains| a51179cda67cf9f2;
  7305c1d6f7f1e2b2 -.->|verifiedBy| 6481e4ca8c4d6920;
  974ccf933675ef44["Format Consistency Enforcement"];
  class 974ccf933675ef44 requirement;
  click 974ccf933675ef44 "../UserRequirements.md#format-consistency-enforcement";
  7305c1d6f7f1e2b2 --o|contains| 974ccf933675ef44;
  47401bd64e231632["Parallel Linting Processing"];
  class 47401bd64e231632 requirement;
  click 47401bd64e231632 "../SystemRequirements/Requirements.md#parallel-linting-processing";
  7305c1d6f7f1e2b2 -.->|deriveReqT| 47401bd64e231632;
  8dfe33c28555e80a["Replace Absolute Links with Relative Links"];
  class 8dfe33c28555e80a requirement;
  click 8dfe33c28555e80a "../UserRequirements.md#replace-absolute-links-with-relative-links";
  7305c1d6f7f1e2b2 --o|contains| 8dfe33c28555e80a;
  974ccf933675ef44 -.->|deriveReqT| 9f473f6e0b993cac;
  719bf8b75772947d["Missing Separators Linting Implementation"];
  class 719bf8b75772947d requirement;
  click 719bf8b75772947d "../SystemRequirements/Requirements.md#missing-separators-linting-implementation";
  974ccf933675ef44 -.->|deriveReqT| 719bf8b75772947d;
  974ccf933675ef44 -.->|verifiedBy| b8bfbd5ccf026b31;
  7d44a9de72f2ed11["Incosistent Newlines Linting Implementation"];
  class 7d44a9de72f2ed11 requirement;
  click 7d44a9de72f2ed11 "../SystemRequirements/Requirements.md#incosistent-newlines-linting-implementation";
  974ccf933675ef44 -.->|deriveReqT| 7d44a9de72f2ed11;
  a7bd845c64d1685e["Reserved Subsections Linting Implementation"];
  class a7bd845c64d1685e requirement;
  click a7bd845c64d1685e "../SystemRequirements/Requirements.md#reserved-subsections-linting-implementation";
  974ccf933675ef44 -.->|deriveReqT| a7bd845c64d1685e;
  80defdd4cbc7ee18["cli.rs"];
  class 80defdd4cbc7ee18 default;
  click 80defdd4cbc7ee18 "../../cli/src/cli.rs";
  deaec107945edbed -->|satisfiedBy| 80defdd4cbc7ee18;
  deaec107945edbed -.->|verifiedBy| 706dca24b7cc1eb8;
  deaec107945edbed -.->|verifiedBy| 37dbb69e504fec9d;
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
  de8bc63ebb2a09e7["tests/test-excluded-linting/test.sh"];
  class de8bc63ebb2a09e7 default;
  click de8bc63ebb2a09e7 "../../tests/test-excluded-linting/test.sh";
  71b5a1d5e278aa8e -->|satisfiedBy| de8bc63ebb2a09e7;
  590a4cc1c558992f["Excluded Patterns Verification"];
  class 590a4cc1c558992f verification;
  click 590a4cc1c558992f "LintingTests.md#excluded-patterns-verification";
  a29e69e90fa71f39["tests/test-excluded-patterns/test.sh"];
  class a29e69e90fa71f39 default;
  click a29e69e90fa71f39 "../../tests/test-excluded-patterns/test.sh";
  590a4cc1c558992f -->|satisfiedBy| a29e69e90fa71f39;
  be83c2991e9535c7["Ignoring Unstructured Documents"];
  class be83c2991e9535c7 requirement;
  click be83c2991e9535c7 "../SystemRequirements/Requirements.md#ignoring-unstructured-documents";
  8419dcc77d92b609["config.rs"];
  class 8419dcc77d92b609 default;
  click 8419dcc77d92b609 "../../cli/src/config.rs";
  be83c2991e9535c7 -->|satisfiedBy| 8419dcc77d92b609;
  be83c2991e9535c7 -.->|verifiedBy| 71b5a1d5e278aa8e;
  bed8d0948b3e5ccd["Requirements Processing"];
  class bed8d0948b3e5ccd requirement;
  click bed8d0948b3e5ccd "../SystemRequirements/Requirements.md#requirements-processing";
  be83c2991e9535c7 -.->|deriveReqT| bed8d0948b3e5ccd;
  be83c2991e9535c7 -.->|verifiedBy| 590a4cc1c558992f;
  bef37c31db69b66a["File Pattern Exclusion for Linting"];
  class bef37c31db69b66a requirement;
  click bef37c31db69b66a "../SystemRequirements/Requirements.md#file-pattern-exclusion-for-linting";
  be83c2991e9535c7 -.->|deriveReqT| bef37c31db69b66a;
  929c6c204cb3fedb["Excluded File Relation Validation"];
  class 929c6c204cb3fedb requirement;
  click 929c6c204cb3fedb "../SystemRequirements/Requirements.md#excluded-file-relation-validation";
  be83c2991e9535c7 -.->|deriveReqT| 929c6c204cb3fedb;
  ce2625feec883e55["utils.rs"];
  class ce2625feec883e55 default;
  click ce2625feec883e55 "../../core/src/utils.rs";
  bef37c31db69b66a -->|satisfiedBy| ce2625feec883e55;
  bef37c31db69b66a -.->|verifiedBy| 71b5a1d5e278aa8e;
  bef37c31db69b66a -.->|verifiedBy| 590a4cc1c558992f;
  bef37c31db69b66a --o|contains| 929c6c204cb3fedb;
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