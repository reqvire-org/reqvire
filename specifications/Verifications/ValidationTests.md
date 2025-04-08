# Validation Tests

This document verifies the requirements for ReqFlow's validation functionality.

## Relation Validation Tests
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  fbf9362574["Invalid Relations Test"];
  click fbf9362574 "ValidationTests.md#invalid-relations-test";
  class fbf9362574 verification;
  bdfd9d65e4["SystemRequirements/Requirement.md/Detailed Error Handling and Logging"];
  class bdfd9d65e4 requirement;
  click bdfd9d65e4 "../SystemRequirements/Requirements.md#detailed-error-handling-and-logging";
  fbf9362574 -->|verifies| bdfd9d65e4;
  5870488e00["SystemRequirements/Requirement.md/Relation Type Validation"];
  class 5870488e00 requirement;
  click 5870488e00 "../SystemRequirements/Requirements.md#relation-type-validation";
  fbf9362574 -->|verifies| 5870488e00;
  212d1317cd["SystemRequirements/Requirement.md/Relation Element Type Validator"];
  class 212d1317cd requirement;
  click 212d1317cd "../SystemRequirements/Requirements.md#relation-element-type-validator";
  fbf9362574 -->|verifies| 212d1317cd;
  c86fd6ece7["tests/test-invalid-relations/test.sh"];
  class c86fd6ece7 default;
  click c86fd6ece7 "../../tests/test-invalid-relations/test.sh";
  fbf9362574 -->|traces| c86fd6ece7;
```

---

### Invalid Relations Test

The verification test checks that ReqFlow correctly identifies and reports invalid relations of different kinds.

#### Metadata
  * type: verification

#### Acceptance Criteria
- System should detect and report invalid relation types (typos, etc.)
- System should detect and report relations to non-existent targets
- System should detect and report if system requirement is missing parent relation
- System should detect and report if there is circular dependency in requirements
- System should detect and report if relation type has incompactible element
- System should detect and report invalid metadata subsection format
- System should detect and report duplicate relations in Relations subsection
- System should detect and report duplicate elements
- System should detect and report duplicate subsections
- System should report clear error messages with details about the invalid format

#### Test Criteria
- Command exits with 0 error code but outputs expected validation errors
- Error output contains specific error messages for each type of invalid relation

#### Test Procedure
1. Create a test fixture in `/tests/fixtures/test-invalid-relations/` with requirements containing invalid relation types
2. Run ReqFlow validation on the test fixture
3. Verify that the validation reports an error for the invalid relation types
4. Verify that error messages contain details about the specific typos found

#### Relations
  * verify: [SystemRequirements/Requirement.md/Detailed Error Handling and Logging](../SystemRequirements/Requirements.md#detailed-error-handling-and-logging)
  * verify: [SystemRequirements/Requirement.md/Relation Type Validation](../SystemRequirements/Requirements.md#relation-type-validation)  
  * verify: [SystemRequirements/Requirement.md/Relation Element Type Validator](../SystemRequirements/Requirements.md#relation-element-type-validator)  
  * trace: [tests/test-invalid-relations/test.sh](../../tests/test-invalid-relations/test.sh)

---