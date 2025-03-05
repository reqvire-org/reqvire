# Validation Tests

This document verifies the requirements for ReqFlow's validation functionality.

## Relation Validation Tests

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
* verifies: [SystemRequirements/Requirements.md/Relation Type Validation](../SystemRequirements/Requirements.html#relation-type-validation)

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
* verifies: [SystemRequirements/Requirements.md/Detailed Error Handling and Logging](../SystemRequirements/Requirements.html#detailed-error-handling-and-logging)
* trace: [tests/e2e-validation/test_invalid_relations.sh](../../../tests/e2e-validation/test_invalid_relations.sh)

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
* verifies: [SystemRequirements/Requirements.md/Detailed Error Handling and Logging](../SystemRequirements/Requirements.html#detailed-error-handling-and-logging)
* trace: [tests/e2e-validation/test_invalid_relations.sh](../../../tests/e2e-validation/test_invalid_relations.sh)

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
* verifies: [SystemRequirements/Requirements.md/Detailed Error Handling and Logging](../SystemRequirements/Requirements.html#detailed-error-handling-and-logging)
* trace: [tests/e2e-validation/test_invalid_relations.sh](../../../tests/e2e-validation/test_invalid_relations.sh)

---

### Missing Relation Target Test

The verification test checks that ReqFlow correctly identifies and reports relations with targets that do not exist in the model.

#### Metadata
* type: verification

#### Acceptance Criteria
- System should detect and report relations to non-existent targets
- System should provide clear error messages identifying the missing targets

#### Test Criteria
- Command exits with error (non-zero) return code
- Error output contains specific error messages about the missing relation targets
- Error messages include both source element and target information

#### Test Procedure
1. Create a test fixture in `/tests/fixtures/test-invalid-relations/` with requirements containing relations to non-existent elements
2. Run ReqFlow validation on the test fixture
3. Verify that the validation reports errors for the missing targets
4. Verify that error messages clearly identify which targets are missing

#### Relations
* verifies: [SystemRequirements/Requirements.md/Detailed Error Handling and Logging](../SystemRequirements/Requirements.html#detailed-error-handling-and-logging)
* trace: [tests/e2e-validation/test_invalid_relations.sh](../../../tests/e2e-validation/test_invalid_relations.sh)
