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

  47e4d671c1["Unsupported Relation Type Test"];
  click 47e4d671c1 "ValidationTests.md#unsupported-relation-type-test";
  class 47e4d671c1 verification;
  5870488e00["SystemRequirements/Requirement.md/Relation Type Validation"];
  class 5870488e00 requirement;
  click 5870488e00 "../SystemRequirements/Requirements.md#relation-type-validation";
  47e4d671c1 -->|verifies| 5870488e00;
  cea8390c33["Invalid Relation Types Test"];
  click cea8390c33 "ValidationTests.md#invalid-relation-types-test";
  class cea8390c33 verification;
  bdfd9d65e4["SystemRequirements/Requirement.md/Detailed Error Handling and Logging"];
  class bdfd9d65e4 requirement;
  click bdfd9d65e4 "../SystemRequirements/Requirements.md#detailed-error-handling-and-logging";
  cea8390c33 -->|verifies| bdfd9d65e4;
  376aa21c53["tests/e2e-validation/test_invalid_relations.sh"];
  class 376aa21c53 default;
  click 376aa21c53 "../../tests/e2e-validation/test_invalid_relations.sh";
  cea8390c33 -->|traces| 376aa21c53;
  5ee74702ae["Missing Relation Target Test"];
  click 5ee74702ae "ValidationTests.md#missing-relation-target-test";
  class 5ee74702ae verification;
  5ee74702ae -->|verifies| bdfd9d65e4;
  62736884ad["tests/e2e-validation/test_missing_targets.sh"];
  class 62736884ad default;
  click 62736884ad "../../tests/e2e-validation/test_missing_targets.sh";
  5ee74702ae -->|traces| 62736884ad;
  d0187b8703["Duplicate Relations Test"];
  click d0187b8703 "ValidationTests.md#duplicate-relations-test";
  class d0187b8703 verification;
  d0187b8703 -->|verifies| bdfd9d65e4;
  d0187b8703 -->|traces| 376aa21c53;
  b598b28754["Invalid Relation Format Test"];
  click b598b28754 "ValidationTests.md#invalid-relation-format-test";
  class b598b28754 verification;
  b598b28754 -->|verifies| bdfd9d65e4;
  b598b28754 -->|traces| 376aa21c53;
```






























### Unsupported Relation Type Test

This verification test checks that ReqFlow correctly identifies and reports relation types that are not part of the supported vocabulary (e.g., "satisfieddBy" instead of "satisfiedBy").

#### Metadata
  * type: verification

#### Acceptance Criteria
- System should detect and report relation types that are not in the supported vocabulary
- System should provide clear error messages suggesting the closest valid relation type

#### Test Criteria
- Command exits with error (non-zero) return code
- Error output contains specific error messages about unsupported relation types
- Error message suggests possible correct relation types

#### Test Procedure
1. Create a test fixture with requirements containing unsupported relation types (e.g., "satisfieddBy", "basedFrom")
2. Run ReqFlow validation on the test fixture
3. Verify that the validation reports an error for the unsupported relation types
4. Verify that error messages suggest the correct relation types (e.g., "satisfiedBy", "derivedFrom")

#### Implementation
- Test will be implemented in `/tests/e2e-validation/test_unsupported_relations.sh`

#### Relations
  * verify: [SystemRequirements/Requirement.md/Relation Type Validation](../SystemRequirements/Requirements.md#relation-type-validation)

---

### Invalid Relation Types Test

The verification test checks that ReqFlow correctly identifies and reports invalid relation types such as typos (e.g., "satisfieddBy" instead of "satisfiedBy").

#### Metadata
* type: verification

#### Acceptance Criteria
- System should detect and report invalid relation types (typos, etc.)
- System should report clear error messages with details about the invalid format

#### Test Criteria
- Command exits with error (non-zero) return code
- Error output contains specific error messages about the invalid relation types

#### Test Procedure
1. Create a test fixture in `/tests/fixtures/test-invalid-relations/` with requirements containing invalid relation types
2. Run ReqFlow validation on the test fixture
3. Verify that the validation reports an error for the invalid relation types
4. Verify that error messages contain details about the specific typos found

#### Relations
  * verify: [SystemRequirements/Requirement.md/Detailed Error Handling and Logging](../SystemRequirements/Requirements.md#detailed-error-handling-and-logging)
  * trace: [tests/e2e-validation/test_invalid_relations.sh](../../tests/e2e-validation/test_invalid_relations.sh)

---

### Invalid Relation Format Test

The verification test checks that ReqFlow correctly identifies and reports relation types with invalid formats (e.g., containing non-alphanumeric characters).

#### Metadata
  * type: verification

#### Acceptance Criteria
- System should detect and report relation types with non-alphanumeric characters
- System should report clear error messages about allowed relation type format

#### Test Criteria
- Command exits with error (non-zero) return code
- Error output contains specific error messages about the invalid characters in relation types

#### Test Procedure
1. Create a test fixture in `/tests/fixtures/test-invalid-relations/` with requirements containing relations with non-alphanumeric characters
2. Run ReqFlow validation on the test fixture
3. Verify that the validation reports an error for the invalid format
4. Verify that error messages indicate what characters are not allowed

#### Relations
  * verify: [SystemRequirements/Requirement.md/Detailed Error Handling and Logging](../SystemRequirements/Requirements.md#detailed-error-handling-and-logging)
  * trace: [tests/e2e-validation/test_invalid_relations.sh](../../tests/e2e-validation/test_invalid_relations.sh)

---

### Duplicate Relations Test

The verification test checks that ReqFlow correctly identifies and reports duplicate relations (same type and target) within an element.

#### Metadata
* type: verification

#### Acceptance Criteria
- System should detect and report duplicate relations within the same element
- System should clearly identify which relations are duplicated and where they occur

#### Test Criteria
- Command exits with error (non-zero) return code
- Error output contains specific error messages identifying duplicate relations
- Error messages include element name and relation indices

#### Test Procedure
1. Create a test fixture in `/tests/fixtures/test-invalid-relations/` with requirements containing duplicate relations
2. Run ReqFlow validation on the test fixture
3. Verify that the validation reports an error for the duplicates
4. Verify that error messages identify which relations are duplicated

#### Relations
  * verify: [SystemRequirements/Requirement.md/Detailed Error Handling and Logging](../SystemRequirements/Requirements.md#detailed-error-handling-and-logging)
  * trace: [tests/e2e-validation/test_invalid_relations.sh](../../tests/e2e-validation/test_invalid_relations.sh)

---

### Missing Relation Target Test

The verification test checks that ReqFlow correctly identifies and reports relations with targets that do not exist in the model, while properly validating relations to existing targets.

#### Metadata
  * type: verification

#### Acceptance Criteria
- System should detect and report relations to non-existent targets
- System should provide clear error messages identifying the missing targets
- System should properly validate relations in both standard and markdown link formats
- System should correctly validate valid relations to existing targets
- System should extract target URLs from markdown links for validation

#### Test Criteria
- Command exits with error (non-zero) return code when there are invalid targets
- Command exits with success (zero) return code when all targets are valid
- Error output contains specific error messages about the missing relation targets
- Error messages include both source element and target information
- No errors reported for valid relations to existing targets

#### Test Procedure
1. Create a test fixture in `/tests/fixtures/test-missing-targets/` with requirements containing:
   - Relations to non-existent elements in standard format 
   - Relations to non-existent targets in markdown link format
   - Relations to existing elements in standard format
   - Relations to existing elements in markdown link format
2. Run ReqFlow validation on the test fixture
3. Verify that the validation reports errors only for the missing targets
4. Verify that error messages clearly identify which targets are missing
5. Create a fixture with only valid relations and verify validation passes

#### Implementation
- Test is implemented in `/tests/e2e-validation/test_missing_targets.sh`
- Test for valid targets is implemented in `/tests/e2e-validation/test_valid_relations.sh`

#### Relations
  * verify: [SystemRequirements/Requirement.md/Detailed Error Handling and Logging](../SystemRequirements/Requirements.md#detailed-error-handling-and-logging)
  * trace: [tests/e2e-validation/test_missing_targets.sh](../../tests/e2e-validation/test_missing_targets.sh)