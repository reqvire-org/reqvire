# Change Impact Tests

This document verifies the requirements for Reqvire's change impact tracing functionality.

## Change Impact Tracing Verification
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  98d7b16583855bb7["Change Impact Detection Test"];
  click 98d7b16583855bb7 "specifications/Verifications/ChangeImpactTests.md#change-impact-detection-test#change-impact-detection-test";
  class 98d7b16583855bb7 verification;
  c8d1020a3844532d["SystemRequirements/ChangeImpactPropagation.md#change-impact-detection-algorithm"];
  class c8d1020a3844532d requirement;
  click c8d1020a3844532d "specifications/SystemRequirements/ChangeImpactPropagation.md#change-impact-detection-algorithm#change-impact-detection-algorithm";
  98d7b16583855bb7 -.->|verifies| c8d1020a3844532d;
  37a75398bd174177["SystemRequirements/ChangeImpactPropagation.md#change-impact-command-line-interface"];
  class 37a75398bd174177 requirement;
  click 37a75398bd174177 "specifications/SystemRequirements/ChangeImpactPropagation.md#change-impact-command-line-interface#change-impact-command-line-interface";
  98d7b16583855bb7 -.->|verifies| 37a75398bd174177;
  e95e3160edec8f1b["tests/test-change-impact-detection/test.sh"];
  class e95e3160edec8f1b default;
  click e95e3160edec8f1b "tests/test-change-impact-detection/test.sh";
  98d7b16583855bb7 -.->|trace| e95e3160edec8f1b;
  11c3285e74c8d60b["Traceability Matrix Verification"];
  click 11c3285e74c8d60b "specifications/Verifications/ChangeImpactTests.md#traceability-matrix-verification#traceability-matrix-verification";
  class 11c3285e74c8d60b verification;
  4e30ea0930dc9c26["UserRequirements.md/Traceability Matrix"];
  class 4e30ea0930dc9c26 requirement;
  click 4e30ea0930dc9c26 "specifications/UserRequirements.md#traceability-matrix#traceability-matrix";
  11c3285e74c8d60b -.->|verifies| 4e30ea0930dc9c26;
  11c3285e74c8d60b -.->|trace| e95e3160edec8f1b;
  b0e8adb6596a35e7["Structural Change Reports Verification"];
  click b0e8adb6596a35e7 "specifications/Verifications/ChangeImpactTests.md#structural-change-reports-verification#structural-change-reports-verification";
  class b0e8adb6596a35e7 verification;
  d7b7b13a5b8d96e1["UserRequirements.md/Tracing Structural Changes"];
  class d7b7b13a5b8d96e1 requirement;
  click d7b7b13a5b8d96e1 "specifications/UserRequirements.md#tracing-structural-changes#tracing-structural-changes";
  b0e8adb6596a35e7 -.->|verifies| d7b7b13a5b8d96e1;
  b0e8adb6596a35e7 -.->|trace| e95e3160edec8f1b;
  893e340d64ec8044["CLI Git Commit Hash Flag Test"];
  click 893e340d64ec8044 "specifications/Verifications/ChangeImpactTests.md#cli-git-commit-hash-flag-test#cli-git-commit-hash-flag-test";
  class 893e340d64ec8044 verification;
  6c40e66699ba40dd["SystemRequirements/Requirements.md#cli-git-commit-hash-flag"];
  class 6c40e66699ba40dd requirement;
  click 6c40e66699ba40dd "specifications/SystemRequirements/Requirements.md#cli-git-commit-hash-flag#cli-git-commit-hash-flag";
  893e340d64ec8044 -.->|verifies| 6c40e66699ba40dd;
  893e340d64ec8044 -.->|trace| e95e3160edec8f1b;
  a4090fe7e30eeae4["Element Content Extraction Test"];
  click a4090fe7e30eeae4 "specifications/Verifications/ChangeImpactTests.md#element-content-extraction-test#element-content-extraction-test";
  class a4090fe7e30eeae4 verification;
  a4090fe7e30eeae4 -.->|verifies| c8d1020a3844532d;
  bed8d0948b3e5ccd["SystemRequirements/Requirements.md#Requirements Processing"];
  class bed8d0948b3e5ccd requirement;
  click bed8d0948b3e5ccd "specifications/SystemRequirements/Requirements.md#requirements-processing#requirements-processing";
  a4090fe7e30eeae4 -.->|verifies| bed8d0948b3e5ccd;
  c287b52ceb52aa04["tests/test-element-content-extraction/test.sh"];
  class c287b52ceb52aa04 default;
  click c287b52ceb52aa04 "tests/test-element-content-extraction/test.sh";
  a4090fe7e30eeae4 -.->|trace| c287b52ceb52aa04;
  b72d56e7e360fe6c["Change Impact Analysis Verification"];
  click b72d56e7e360fe6c "specifications/Verifications/ChangeImpactTests.md#change-impact-analysis-verification#change-impact-analysis-verification";
  class b72d56e7e360fe6c verification;
  9933cac5853a8584["UserRequirements.md/Change Impact Analysis"];
  class 9933cac5853a8584 requirement;
  click 9933cac5853a8584 "specifications/UserRequirements.md#change-impact-analysis#change-impact-analysis";
  b72d56e7e360fe6c -.->|verifies| 9933cac5853a8584;
  b72d56e7e360fe6c -.->|trace| e95e3160edec8f1b;
  cec02a5e3f71bed1["Change Impact Relations Test"];
  click cec02a5e3f71bed1 "specifications/Verifications/ChangeImpactTests.md#change-impact-relations-test#change-impact-relations-test";
  class cec02a5e3f71bed1 verification;
  cec02a5e3f71bed1 -.->|verifies| c8d1020a3844532d;
  cec02a5e3f71bed1 -.->|verifies| 37a75398bd174177;
  cec02a5e3f71bed1 -.->|trace| e95e3160edec8f1b;
```

---

### Change Impact Detection Test

This test verifies that the system correctly implements change impact detection, including proper default handling of the git commit parameter.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System correctly detects changes between different versions of requirements
- System properly constructs a change impact report based on relationships between elements
- Default git commit is HEAD when --git-commit parameter is not provided
- System provides output in both human-readable text and JSON formats

##### Test Criteria
- Command exits with success (0) return code
- Change impact report shows expected elements
- Change impact report shows correct relationships between elements
- Output format matches requested format (text or JSON)
- Both explicit and implicit git commit parameters work properly
- JSON output is valid and contains all necessary information
- GitHub-style blob URLs are included in the output

#### Relations
  * verify: [SystemRequirements/ChangeImpactPropagation.md#change-impact-detection-algorithm](../SystemRequirements/ChangeImpactPropagation.md#change-impact-detection-algorithm)
  * verify: [SystemRequirements/ChangeImpactPropagation.md#change-impact-command-line-interface](../SystemRequirements/ChangeImpactPropagation.md#change-impact-command-line-interface)
  * trace: [tests/test-change-impact-detection/test.sh](../../tests/test-change-impact-detection/test.sh)

---

### Change Impact Relations Test

This test verifies that the system correctly handles different relation types when calculating change impact.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System correctly propagates changes through different relation types
- System respects the directionality of relations when determining impact
- System handles complex chains of relations properly

##### Test Criteria
- Command exits with success (0) return code
- Change impact report shows expected propagation through derivedFrom/derive relations
- Change impact report shows expected propagation through satisfiedBy/satisfy relations
- Change impact report shows expected propagation through verifiedBy/verify relations
- System correctly handles circular dependencies in relation chains

#### Relations
  * verify: [SystemRequirements/ChangeImpactPropagation.md#change-impact-detection-algorithm](../SystemRequirements/ChangeImpactPropagation.md#change-impact-detection-algorithm)
  * verify: [SystemRequirements/ChangeImpactPropagation.md#change-impact-command-line-interface](../SystemRequirements/ChangeImpactPropagation.md#change-impact-command-line-interface) 
  * trace: [tests/test-change-impact-detection/test.sh](../../tests/test-change-impact-detection/test.sh)

---

### CLI Git Commit Hash Flag Test

This test verifies that the system properly handles the git commit hash flag for change impact analysis.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System shall support --git-commit flag for change impact analysis
- System shall use specified commit hash as base for comparison
- System shall default to HEAD when flag is not specified
- System shall handle relative commit references (HEAD~1, etc.)

##### Test Criteria
- Command with explicit --git-commit flag runs successfully
- Command without flag defaults to HEAD commit
- Relative commit references are correctly resolved
- Invalid commit references are reported appropriately
- Change impact analysis correctly uses specified commit as baseline

##### Test Procedure
1. Create test fixtures with git repository containing multiple commits
2. Run Reqvire with --change-impact --git-commit=HEAD~1
3. Verify that the specified commit is used as baseline
4. Run Reqvire with --change-impact (no git-commit flag)
5. Verify that HEAD is used as default baseline
6. Run with invalid commit reference and verify appropriate error

#### Relations
  * verify: [SystemRequirements/Requirements.md#cli-git-commit-hash-flag](../SystemRequirements/Requirements.md#cli-git-commit-hash-flag)
  * trace: [tests/test-change-impact-detection/test.sh](../../tests/test-change-impact-detection/test.sh)

---

### Element Content Extraction Test

This test verifies that the system correctly extracts element content for change impact detection.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should properly extract requirement body for change impact detection
- Requirement body should include normalized main text and content from the Details subsection
- System should handle requirements with various combinations of subsections

##### Test Criteria
- Command exits with success (0) return code
- Output shows expected content for each element
- Content extraction correctly handles different subsection ordering
- Content extraction properly handles HTML details tags

##### Test Procedure
1. Create test fixtures with requirements containing various combinations of subsections
2. Run Reqvire model summary on the test fixtures
3. Verify that extracted content matches expected content for each element
4. Verify that content from Details subsection is properly included

#### Relations
  * verify: [SystemRequirements/ChangeImpactPropagation.md#change-impact-detection-algorithm](../SystemRequirements/ChangeImpactPropagation.md#change-impact-detection-algorithm)
  * verify: [SystemRequirements/Requirements.md#Requirements Processing](../SystemRequirements/Requirements.md#requirements-processing)
  * trace: [tests/test-element-content-extraction/test.sh](../../tests/test-element-content-extraction/test.sh)

---

### Change Impact Analysis Verification

This test verifies that the system generates change impact reports when requested.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should generate change impact reports in Markdown format
- System should support JSON output for change impact reports
- Reports should include an overview of model changes and their impact

##### Test Criteria
- Command exits with success (0) return code
- Reports contain expected impact information
- Both Markdown and JSON formats are properly supported

##### Test Procedure
TODO: write test procedure

#### Relations
  * verify: [UserRequirements.md/Change Impact Analysis](../UserRequirements.md#change-impact-analysis)
  * trace: [tests/test-change-impact-detection/test.sh](../../tests/test-change-impact-detection/test.sh)

---

### Traceability Matrix Verification

This test verifies that the system generates traceability matrices in Markdown format.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should generate traceability matrices in Markdown format
- Matrices should support different views (Requirements-to-Verification, etc.)
- Matrices should efficiently represent relationships between elements

##### Test Criteria
- Command exits with success (0) return code
- Generated matrices contain expected relationship information
- Different matrix views are properly supported

##### Test Procedure
TODO: write test procedure

#### Relations
  * verify: [UserRequirements.md/Traceability Matrix](../UserRequirements.md#traceability-matrix)
  * trace: [tests/test-change-impact-detection/test.sh](../../tests/test-change-impact-detection/test.sh)

---

### Structural Change Reports Verification

This test verifies that the system analyzes and reports on structural changes in the MBSE model.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should analyze structural changes in the MBSE model
- System should identify affected components through relationship traversal
- System should generate reports of impacted elements and structures

##### Test Criteria
- Command exits with success (0) return code
- Change reports correctly identify affected elements
- Relationship traversal properly determines impact propagation

##### Test Procedure
TODO: write test procedure

#### Relations
  * verify: [UserRequirements.md/Tracing Structural Changes](../UserRequirements.md#tracing-structural-changes)
  * trace: [tests/test-change-impact-detection/test.sh](../../tests/test-change-impact-detection/test.sh)

---