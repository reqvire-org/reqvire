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

  1c7aa5662fc4c31["Element Content Extraction Test"];
  click 1c7aa5662fc4c31 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/ChangeImpactTests.md#element-content-extraction-test";
  class 1c7aa5662fc4c31 verification;
  9967e71248da3061["SystemRequirements/ChangeImpactPropagation.md#change-impact-detection-algorithm"];
  class 9967e71248da3061 requirement;
  click 9967e71248da3061 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/ChangeImpactPropagation.md#change-impact-detection-algorithm";
  1c7aa5662fc4c31 -.->|verifies| 9967e71248da3061;
  c6d19363284e9125["SystemRequirements/Requirements.md#Requirements Processing"];
  class c6d19363284e9125 requirement;
  click c6d19363284e9125 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#requirements-processing";
  1c7aa5662fc4c31 -.->|verifies| c6d19363284e9125;
  890802c6addd8829["tests/test-element-content-extraction/test.sh"];
  class 890802c6addd8829 default;
  click 890802c6addd8829 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/tests/test-element-content-extraction/test.sh";
  1c7aa5662fc4c31 -.->|trace| 890802c6addd8829;
  5bf242d222f9fc08["Change Impact Detection Test"];
  click 5bf242d222f9fc08 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/ChangeImpactTests.md#change-impact-detection-test";
  class 5bf242d222f9fc08 verification;
  5bf242d222f9fc08 -.->|verifies| 9967e71248da3061;
  7cc5e857c8078367["SystemRequirements/ChangeImpactPropagation.md#change-impact-command-line-interface"];
  class 7cc5e857c8078367 requirement;
  click 7cc5e857c8078367 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/ChangeImpactPropagation.md#change-impact-command-line-interface";
  5bf242d222f9fc08 -.->|verifies| 7cc5e857c8078367;
  369b7fc504ca66ba["tests/test-change-impact-detection/test.sh"];
  class 369b7fc504ca66ba default;
  click 369b7fc504ca66ba "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/tests/test-change-impact-detection/test.sh";
  5bf242d222f9fc08 -.->|trace| 369b7fc504ca66ba;
  7156f2b65aa302ca["Traceability Matrix Verification"];
  click 7156f2b65aa302ca "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/ChangeImpactTests.md#traceability-matrix-verification";
  class 7156f2b65aa302ca verification;
  681cda683cd3fa2a["UserRequirements.md/Traceability Matrix"];
  class 681cda683cd3fa2a requirement;
  click 681cda683cd3fa2a "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#traceability-matrix";
  7156f2b65aa302ca -.->|verifies| 681cda683cd3fa2a;
  7156f2b65aa302ca -.->|trace| 369b7fc504ca66ba;
  c9892202d1906367["Structural Change Reports Verification"];
  click c9892202d1906367 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/ChangeImpactTests.md#structural-change-reports-verification";
  class c9892202d1906367 verification;
  4dc854c91f0e4c8d["UserRequirements.md/Tracing Structural Changes"];
  class 4dc854c91f0e4c8d requirement;
  click 4dc854c91f0e4c8d "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#tracing-structural-changes";
  c9892202d1906367 -.->|verifies| 4dc854c91f0e4c8d;
  c9892202d1906367 -.->|trace| 369b7fc504ca66ba;
  386e7d5a9c90fa8["CLI Git Commit Hash Flag Test"];
  click 386e7d5a9c90fa8 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/ChangeImpactTests.md#cli-git-commit-hash-flag-test";
  class 386e7d5a9c90fa8 verification;
  7d76667e6a943402["SystemRequirements/Requirements.md#cli-git-commit-hash-flag"];
  class 7d76667e6a943402 requirement;
  click 7d76667e6a943402 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#cli-git-commit-hash-flag";
  386e7d5a9c90fa8 -.->|verifies| 7d76667e6a943402;
  386e7d5a9c90fa8 -.->|trace| 369b7fc504ca66ba;
  b8372dd5907469d1["Change Impact Relations Test"];
  click b8372dd5907469d1 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/ChangeImpactTests.md#change-impact-relations-test";
  class b8372dd5907469d1 verification;
  b8372dd5907469d1 -.->|verifies| 9967e71248da3061;
  b8372dd5907469d1 -.->|verifies| 7cc5e857c8078367;
  b8372dd5907469d1 -.->|trace| 369b7fc504ca66ba;
  aad08e3aad7ec4b4["Change Impact Analysis Verification"];
  click aad08e3aad7ec4b4 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/ChangeImpactTests.md#change-impact-analysis-verification";
  class aad08e3aad7ec4b4 verification;
  6f4efc192ae34938["UserRequirements.md/Change Impact Analysis"];
  class 6f4efc192ae34938 requirement;
  click 6f4efc192ae34938 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#change-impact-analysis";
  aad08e3aad7ec4b4 -.->|verifies| 6f4efc192ae34938;
  aad08e3aad7ec4b4 -.->|trace| 369b7fc504ca66ba;
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