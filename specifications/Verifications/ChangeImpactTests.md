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

  9102eb42eb7742cb["Change Impact Detection Test"];
  click 9102eb42eb7742cb "ChangeImpactTests.md#change-impact-detection-test";
  class 9102eb42eb7742cb verification;
  d17b0775f448b19b["SystemRequirements/ChangeImpactPropagation.md#change-impact-detection-algorithm"];
  class d17b0775f448b19b requirement;
  click d17b0775f448b19b "../SystemRequirements/ChangeImpactPropagation.md#change-impact-detection-algorithm";
  9102eb42eb7742cb -.->|verifies| d17b0775f448b19b;
  6e747b7f03732873["SystemRequirements/ChangeImpactPropagation.md#change-impact-command-line-interface"];
  class 6e747b7f03732873 requirement;
  click 6e747b7f03732873 "../SystemRequirements/ChangeImpactPropagation.md#change-impact-command-line-interface";
  9102eb42eb7742cb -.->|verifies| 6e747b7f03732873;
  524acc7470b3e5ca["tests/test-change-impact-detection/test.sh"];
  class 524acc7470b3e5ca default;
  click 524acc7470b3e5ca "../../tests/test-change-impact-detection/test.sh";
  9102eb42eb7742cb -.->|trace| 524acc7470b3e5ca;
  401764f61b3932e8["Change Impact Relations Test"];
  click 401764f61b3932e8 "ChangeImpactTests.md#change-impact-relations-test";
  class 401764f61b3932e8 verification;
  401764f61b3932e8 -.->|verifies| d17b0775f448b19b;
  401764f61b3932e8 -.->|verifies| 6e747b7f03732873;
  401764f61b3932e8 -.->|trace| 524acc7470b3e5ca;
  
  element_extraction_test["Element Content Extraction Test"];
  click element_extraction_test "ChangeImpactTests.md#element-content-extraction-test";
  class element_extraction_test verification;
  element_extraction_test -.->|verifies| d17b0775f448b19b;
  req_processing["SystemRequirements/Requirements.md#Requirements Processing"];
  class req_processing requirement;
  click req_processing "../SystemRequirements/Requirements.md#requirements-processing";
  element_extraction_test -.->|verifies| req_processing;
  extraction_test_script["tests/test-element-content-extraction/test.sh"];
  class extraction_test_script default;
  click extraction_test_script "../../tests/test-element-content-extraction/test.sh";
  element_extraction_test -.->|trace| extraction_test_script;
  
  change_impact_verification["Change Impact Analysis Verification"];
  click change_impact_verification "ChangeImpactTests.md#change-impact-analysis-verification";
  class change_impact_verification verification;
  change_impact_req["UserRequirements.md/Change Impact Analysis"];
  class change_impact_req requirement;
  click change_impact_req "../UserRequirements.md#change-impact-analysis";
  change_impact_verification -.->|verifies| change_impact_req;
  change_impact_verification -.->|trace| 524acc7470b3e5ca;
  
  traceability_matrix_verification["Traceability Matrix Verification"];
  click traceability_matrix_verification "ChangeImpactTests.md#traceability-matrix-verification";
  class traceability_matrix_verification verification;
  traceability_matrix_req["UserRequirements.md/Traceability Matrix"];
  class traceability_matrix_req requirement;
  click traceability_matrix_req "../UserRequirements.md#traceability-matrix";
  traceability_matrix_verification -.->|verifies| traceability_matrix_req;
  traceability_matrix_verification -.->|trace| 524acc7470b3e5ca;
  
  structural_change_verification["Structural Change Reports Verification"];
  click structural_change_verification "ChangeImpactTests.md#structural-change-reports-verification";
  class structural_change_verification verification;
  structural_change_req["UserRequirements.md/Tracing Structural Changes"];
  class structural_change_req requirement;
  click structural_change_req "../UserRequirements.md#tracing-structural-changes";
  structural_change_verification -.->|verifies| structural_change_req;
  structural_change_verification -.->|trace| 524acc7470b3e5ca;
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