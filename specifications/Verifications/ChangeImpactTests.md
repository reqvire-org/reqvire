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

  401764f61b["Change Impact Relations Test"];
  click 401764f61b "ChangeImpactTests.md#change-impact-relations-test";
  class 401764f61b verification;
  503dda29a8["SystemRequirements/ChangeImpactRequirements.md#change-impact-detection-algorithm"];
  class 503dda29a8 requirement;
  click 503dda29a8 "../SystemRequirements/ChangeImpactRequirements.md#change-impact-detection-algorithm";
  401764f61b -->|verifies| 503dda29a8;
  9d6f79f601["SystemRequirements/ChangeImpactRequirements.md#change-impact-command-line-interface"];
  class 9d6f79f601 requirement;
  click 9d6f79f601 "../SystemRequirements/ChangeImpactRequirements.md#change-impact-command-line-interface";
  401764f61b -->|verifies| 9d6f79f601;
  524acc7470["tests/test-change-impact-detection/test.sh"];
  class 524acc7470 default;
  click 524acc7470 "../../tests/test-change-impact-detection/test.sh";
  401764f61b -->|traces| 524acc7470;
  9102eb42eb["Change Impact Detection Test"];
  click 9102eb42eb "ChangeImpactTests.md#change-impact-detection-test";
  class 9102eb42eb verification;
  9102eb42eb -->|verifies| 503dda29a8;
  9102eb42eb -->|verifies| 9d6f79f601;
  9102eb42eb -->|traces| 524acc7470;
```

---

### Change Impact Detection Test

This test verifies that the system correctly implements change impact detection, including proper default handling of the git commit parameter.

#### Metadata
  * type: verification

#### Acceptance Criteria
- System correctly detects changes between different versions of requirements
- System properly constructs a change impact report based on relationships between elements
- Default git commit is HEAD when --git-commit parameter is not provided

#### Test Criteria
- Command exits with success (0) return code
- Change impact report shows expected elements
- Change impact report shows correct relationships between elements
- Output format matches requested format (text or JSON)
- Both explicit and implicit git commit parameters work properly

#### Relations
  * verify: [SystemRequirements/ChangeImpactRequirements.md#change-impact-detection-algorithm](../SystemRequirements/ChangeImpactRequirements.md#change-impact-detection-algorithm)
  * verify: [SystemRequirements/ChangeImpactRequirements.md#change-impact-command-line-interface](../SystemRequirements/ChangeImpactRequirements.md#change-impact-command-line-interface)
  * trace: [tests/test-change-impact-detection/test.sh](../../tests/test-change-impact-detection/test.sh)

---

### Change Impact Relations Test

This test verifies that the system correctly handles different relation types when calculating change impact.

#### Metadata
  * type: verification

#### Acceptance Criteria
- System correctly propagates changes through different relation types
- System respects the directionality of relations when determining impact
- System handles complex chains of relations properly

#### Test Criteria
- Command exits with success (0) return code
- Change impact report shows expected propagation through derivedFrom/derive relations
- Change impact report shows expected propagation through satisfiedBy/satisfy relations
- Change impact report shows expected propagation through verifiedBy/verify relations
- System correctly handles circular dependencies in relation chains

#### Relations
  * verify: [SystemRequirements/ChangeImpactRequirements.md#change-impact-detection-algorithm](../SystemRequirements/ChangeImpactRequirements.md#change-impact-detection-algorithm)
  * verify: [SystemRequirements/ChangeImpactRequirements.md#change-impact-command-line-interface](../SystemRequirements/ChangeImpactRequirements.md#change-impact-command-line-interface) 
  * trace: [tests/test-change-impact-detection/test.sh](../../tests/test-change-impact-detection/test.sh)
