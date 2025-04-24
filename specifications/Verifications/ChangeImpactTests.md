# Change Impact Tests

This document verifies the requirements for ReqFlow's change impact tracing functionality.

## Change Impact Tracing Verification
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  5c3e28e702ebd6b2["Traceability Matrix Verification"];
  click 5c3e28e702ebd6b2 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/ChangeImpactTests.md#traceability-matrix-verification";
  class 5c3e28e702ebd6b2 verification;
  25ad41b0b912092b["UserRequirements.md/Traceability Matrix"];
  class 25ad41b0b912092b requirement;
  click 25ad41b0b912092b "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#traceability-matrix";
  5c3e28e702ebd6b2 -.->|verifies| 25ad41b0b912092b;
  c46d067d93f972bc["tests/test-change-impact-detection/test.sh"];
  class c46d067d93f972bc default;
  click c46d067d93f972bc "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/tests/test-change-impact-detection/test.sh";
  5c3e28e702ebd6b2 -.->|trace| c46d067d93f972bc;
  847954159345f3a4["CLI Git Commit Hash Flag Test"];
  click 847954159345f3a4 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/ChangeImpactTests.md#cli-git-commit-hash-flag-test";
  class 847954159345f3a4 verification;
  c71f1cc579bb4db["SystemRequirements/Requirements.md#cli-git-commit-hash-flag"];
  class c71f1cc579bb4db requirement;
  click c71f1cc579bb4db "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#cli-git-commit-hash-flag";
  847954159345f3a4 -.->|verifies| c71f1cc579bb4db;
  847954159345f3a4 -.->|trace| c46d067d93f972bc;
  d06fce8d063fd3d1["Element Content Extraction Test"];
  click d06fce8d063fd3d1 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/ChangeImpactTests.md#element-content-extraction-test";
  class d06fce8d063fd3d1 verification;
  b6dcdc95b5e8f272["SystemRequirements/ChangeImpactPropagation.md#change-impact-detection-algorithm"];
  class b6dcdc95b5e8f272 requirement;
  click b6dcdc95b5e8f272 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/ChangeImpactPropagation.md#change-impact-detection-algorithm";
  d06fce8d063fd3d1 -.->|verifies| b6dcdc95b5e8f272;
  6aeba4bf990bc9e4["SystemRequirements/Requirements.md#Requirements Processing"];
  class 6aeba4bf990bc9e4 requirement;
  click 6aeba4bf990bc9e4 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#requirements-processing";
  d06fce8d063fd3d1 -.->|verifies| 6aeba4bf990bc9e4;
  4de1c5f9aeb25f86["tests/test-element-content-extraction/test.sh"];
  class 4de1c5f9aeb25f86 default;
  click 4de1c5f9aeb25f86 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/tests/test-element-content-extraction/test.sh";
  d06fce8d063fd3d1 -.->|trace| 4de1c5f9aeb25f86;
  317567e3530952b1["Structural Change Reports Verification"];
  click 317567e3530952b1 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/ChangeImpactTests.md#structural-change-reports-verification";
  class 317567e3530952b1 verification;
  e42698fdbbf344aa["UserRequirements.md/Tracing Structural Changes"];
  class e42698fdbbf344aa requirement;
  click e42698fdbbf344aa "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#tracing-structural-changes";
  317567e3530952b1 -.->|verifies| e42698fdbbf344aa;
  317567e3530952b1 -.->|trace| c46d067d93f972bc;
  fc17fb0d86285ae1["Change Impact Analysis Verification"];
  click fc17fb0d86285ae1 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/ChangeImpactTests.md#change-impact-analysis-verification";
  class fc17fb0d86285ae1 verification;
  52aac80b9a806080["UserRequirements.md/Change Impact Analysis"];
  class 52aac80b9a806080 requirement;
  click 52aac80b9a806080 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#change-impact-analysis";
  fc17fb0d86285ae1 -.->|verifies| 52aac80b9a806080;
  fc17fb0d86285ae1 -.->|trace| c46d067d93f972bc;
  650026bd69216e2e["Change Impact Relations Test"];
  click 650026bd69216e2e "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/ChangeImpactTests.md#change-impact-relations-test";
  class 650026bd69216e2e verification;
  650026bd69216e2e -.->|verifies| b6dcdc95b5e8f272;
  75403d710ba5793e["SystemRequirements/ChangeImpactPropagation.md#change-impact-command-line-interface"];
  class 75403d710ba5793e requirement;
  click 75403d710ba5793e "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/ChangeImpactPropagation.md#change-impact-command-line-interface";
  650026bd69216e2e -.->|verifies| 75403d710ba5793e;
  650026bd69216e2e -.->|trace| c46d067d93f972bc;
  1d07eeb17bd1b96f["Change Impact Detection Test"];
  click 1d07eeb17bd1b96f "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/ChangeImpactTests.md#change-impact-detection-test";
  class 1d07eeb17bd1b96f verification;
  1d07eeb17bd1b96f -.->|verifies| b6dcdc95b5e8f272;
  1d07eeb17bd1b96f -.->|verifies| 75403d710ba5793e;
  1d07eeb17bd1b96f -.->|trace| c46d067d93f972bc;
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
2. Run ReqFlow with --change-impact --git-commit=HEAD~1
3. Verify that the specified commit is used as baseline
4. Run ReqFlow with --change-impact (no git-commit flag)
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
2. Run ReqFlow model summary on the test fixtures
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