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