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

  af3810a2c1009014["Change Impact Detection Test"];
  click af3810a2c1009014 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/Verifications/ChangeImpactTests.md#change-impact-detection-test";
  class af3810a2c1009014 verification;
  d46f18156c13fc62["SystemRequirements/ChangeImpactPropagation.md#change-impact-detection-algorithm"];
  class d46f18156c13fc62 requirement;
  click d46f18156c13fc62 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/SystemRequirements/ChangeImpactPropagation.md#change-impact-detection-algorithm";
  af3810a2c1009014 -.->|verifies| d46f18156c13fc62;
  df07168dd5bc52f5["SystemRequirements/ChangeImpactPropagation.md#change-impact-command-line-interface"];
  class df07168dd5bc52f5 requirement;
  click df07168dd5bc52f5 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/SystemRequirements/ChangeImpactPropagation.md#change-impact-command-line-interface";
  af3810a2c1009014 -.->|verifies| df07168dd5bc52f5;
  ce6c792ffe08dec0["tests/test-change-impact-detection/test.sh"];
  class ce6c792ffe08dec0 default;
  click ce6c792ffe08dec0 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/tests/test-change-impact-detection/test.sh";
  af3810a2c1009014 -.->|trace| ce6c792ffe08dec0;
  6620d34cd908cfd0["CLI Git Commit Hash Flag Test"];
  click 6620d34cd908cfd0 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/Verifications/ChangeImpactTests.md#cli-git-commit-hash-flag-test";
  class 6620d34cd908cfd0 verification;
  79259d512a5c44a6["SystemRequirements/Requirements.md#cli-git-commit-hash-flag"];
  class 79259d512a5c44a6 requirement;
  click 79259d512a5c44a6 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/SystemRequirements/Requirements.md#cli-git-commit-hash-flag";
  6620d34cd908cfd0 -.->|verifies| 79259d512a5c44a6;
  6620d34cd908cfd0 -.->|trace| ce6c792ffe08dec0;
  ae04218a948be8f3["Change Impact Relations Test"];
  click ae04218a948be8f3 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/Verifications/ChangeImpactTests.md#change-impact-relations-test";
  class ae04218a948be8f3 verification;
  ae04218a948be8f3 -.->|verifies| d46f18156c13fc62;
  ae04218a948be8f3 -.->|verifies| df07168dd5bc52f5;
  ae04218a948be8f3 -.->|trace| ce6c792ffe08dec0;
  ee80444638d18e81["Traceability Matrix Verification"];
  click ee80444638d18e81 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/Verifications/ChangeImpactTests.md#traceability-matrix-verification";
  class ee80444638d18e81 verification;
  c5b8a7944b6943e2["UserRequirements.md/Traceability Matrix"];
  class c5b8a7944b6943e2 requirement;
  click c5b8a7944b6943e2 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#traceability-matrix";
  ee80444638d18e81 -.->|verifies| c5b8a7944b6943e2;
  ee80444638d18e81 -.->|trace| ce6c792ffe08dec0;
  23fb1c6f020c84c6["Element Content Extraction Test"];
  click 23fb1c6f020c84c6 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/Verifications/ChangeImpactTests.md#element-content-extraction-test";
  class 23fb1c6f020c84c6 verification;
  23fb1c6f020c84c6 -.->|verifies| d46f18156c13fc62;
  f24f11691f55af62["SystemRequirements/Requirements.md#Requirements Processing"];
  class f24f11691f55af62 requirement;
  click f24f11691f55af62 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/SystemRequirements/Requirements.md#requirements-processing";
  23fb1c6f020c84c6 -.->|verifies| f24f11691f55af62;
  dfa34153b7e5ad4a["tests/test-element-content-extraction/test.sh"];
  class dfa34153b7e5ad4a default;
  click dfa34153b7e5ad4a "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/tests/test-element-content-extraction/test.sh";
  23fb1c6f020c84c6 -.->|trace| dfa34153b7e5ad4a;
  53743c2b669637b3["Change Impact Analysis Verification"];
  click 53743c2b669637b3 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/Verifications/ChangeImpactTests.md#change-impact-analysis-verification";
  class 53743c2b669637b3 verification;
  c699ef8d6d1f99d1["UserRequirements.md/Change Impact Analysis"];
  class c699ef8d6d1f99d1 requirement;
  click c699ef8d6d1f99d1 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#change-impact-analysis";
  53743c2b669637b3 -.->|verifies| c699ef8d6d1f99d1;
  53743c2b669637b3 -.->|trace| ce6c792ffe08dec0;
  a1f7b75d8eb82de7["Structural Change Reports Verification"];
  click a1f7b75d8eb82de7 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/Verifications/ChangeImpactTests.md#structural-change-reports-verification";
  class a1f7b75d8eb82de7 verification;
  9b9c33c7182d6eeb["UserRequirements.md/Tracing Structural Changes"];
  class 9b9c33c7182d6eeb requirement;
  click 9b9c33c7182d6eeb "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#tracing-structural-changes";
  a1f7b75d8eb82de7 -.->|verifies| 9b9c33c7182d6eeb;
  a1f7b75d8eb82de7 -.->|trace| ce6c792ffe08dec0;
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