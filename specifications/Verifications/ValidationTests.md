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

  fbf9362574b057dd["Invalid Relations Test"];
  click fbf9362574b057dd "ValidationTests.md#invalid-relations-test";
  class fbf9362574b057dd verification;
  bdfd9d65e46117e7["SystemRequirements/Requirement.md/Detailed Error Handling and Logging"];
  class bdfd9d65e46117e7 requirement;
  click bdfd9d65e46117e7 "../SystemRequirements/Requirements.md#detailed-error-handling-and-logging";
  fbf9362574b057dd -.->|verifies| bdfd9d65e46117e7;
  5870488e00ee4f36["SystemRequirements/Requirement.md/Relation Type Validation"];
  class 5870488e00ee4f36 requirement;
  click 5870488e00ee4f36 "../SystemRequirements/Requirements.md#relation-type-validation";
  fbf9362574b057dd -.->|verifies| 5870488e00ee4f36;
  212d1317cd2b25fc["SystemRequirements/Requirement.md/Relation Element Type Validator"];
  class 212d1317cd2b25fc requirement;
  click 212d1317cd2b25fc "../SystemRequirements/Requirements.md#relation-element-type-validator";
  fbf9362574b057dd -.->|verifies| 212d1317cd2b25fc;
  c86fd6ece7a8668a["tests/test-invalid-relations/test.sh"];
  class c86fd6ece7a8668a default;
  click c86fd6ece7a8668a "../../tests/test-invalid-relations/test.sh";
  fbf9362574b057dd -.->|trace| c86fd6ece7a8668a;
  
  fragment_relations_test["Same-File Fragment Relations Test"];
  click fragment_relations_test "ValidationTests.md#same-file-fragment-relations-test";
  class fragment_relations_test verification;
  frag_req_validation["SystemRequirements/Requirements.md#Relation Type Validation"];
  class frag_req_validation requirement;
  click frag_req_validation "../SystemRequirements/Requirements.md#relation-type-validation";
  fragment_relations_test -.->|verifies| frag_req_validation;
  frag_req_processing["SystemRequirements/Requirements.md#Requirements Processing"];
  class frag_req_processing requirement;
  click frag_req_processing "../SystemRequirements/Requirements.md#requirements-processing";
  fragment_relations_test -.->|verifies| frag_req_processing;
  frag_test_script["tests/test-fragment-relations/test.sh"];
  class frag_test_script default;
  click frag_test_script "../../tests/test-fragment-relations/test.sh";
  fragment_relations_test -.->|trace| frag_test_script;
  
  markdown_structure_verification["Markdown Structure Verification"];
  click markdown_structure_verification "ValidationTests.md#markdown-structure-verification";
  class markdown_structure_verification verification;
  markdown_structure_req["UserRequirements.md/Validate Markdown Structure"];
  class markdown_structure_req requirement;
  click markdown_structure_req "../UserRequirements.md#validate-markdown-structure";
  markdown_structure_verification -.->|verifies| markdown_structure_req;
  markdown_structure_verification -.->|trace| c86fd6ece7a8668a;
  
  filesystem_structure_verification["Filesystem Structure Verification"];
  click filesystem_structure_verification "ValidationTests.md#filesystem-structure-verification";
  class filesystem_structure_verification verification;
  filesystem_structure_req["UserRequirements.md/Validate Filesystem Structure"];
  class filesystem_structure_req requirement;
  click filesystem_structure_req "../UserRequirements.md#validate-filesystem-structure";
  filesystem_structure_verification -.->|verifies| filesystem_structure_req;
  filesystem_test_script["tests/test-external-folders/test.sh"];
  class filesystem_test_script default;
  click filesystem_test_script "../../tests/test-external-folders/test.sh";
  filesystem_structure_verification -.->|trace| filesystem_test_script;
  
  internal_consistency_verification["Internal Consistency Verification"];
  click internal_consistency_verification "ValidationTests.md#internal-consistency-verification";
  class internal_consistency_verification verification;
  internal_consistency_req["UserRequirements.md/Validate Internal Consistency"];
  class internal_consistency_req requirement;
  click internal_consistency_req "../UserRequirements.md#validate-internal-consistency";
  internal_consistency_verification -.->|verifies| internal_consistency_req;
  internal_consistency_verification -.->|trace| c86fd6ece7a8668a;
  
  cross_component_verification["Cross-Component Dependencies Verification"];
  click cross_component_verification "ValidationTests.md#cross-component-dependencies-verification";
  class cross_component_verification verification;
  cross_component_req["UserRequirements.md/Validate Cross-Component Dependencies"];
  class cross_component_req requirement;
  click cross_component_req "../UserRequirements.md#validate-cross-component-dependencies";
  cross_component_verification -.->|verifies| cross_component_req;
  cross_component_verification -.->|trace| c86fd6ece7a8668a;
```

---

### Invalid Relations Test

The verification test checks that ReqFlow correctly identifies and reports invalid relations of different kinds.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should detect and report invalid relation types (typos, etc.)
- System should detect and report relations to non-existent targets
- System should detect and report if system requirement is missing parent relation
- System should detect and report if there is circular dependency in requirements
- System should detect and report if relation type has incompatible element
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

### Same-File Fragment Relations Test

This test verifies that ReqFlow correctly handles and validates relations to fragments within the same file.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should correctly validate relations to fragments within the same file
- System should not report errors for valid fragment references
- System should handle both fragment-only references like "#fragment-id" and proper element IDs

##### Test Criteria
- Command exits with success (zero) return code
- No error output about missing relation targets when using #fragment references
- Successful validation message is displayed

##### Test Procedure
1. Create test fixtures with requirements containing fragment-only references
2. Run ReqFlow validation on the test fixtures
3. Verify that validation succeeds with no errors reported
4. Verify that fragments referenced by proper element ID are correctly validated

#### Relations
  * verify: [SystemRequirements/Requirements.md#Relation Type Validation](../SystemRequirements/Requirements.md#relation-type-validation)
  * verify: [SystemRequirements/Requirements.md#Requirements Processing](../SystemRequirements/Requirements.md#requirements-processing)
  * trace: [tests/test-fragment-relations/test.sh](../../tests/test-fragment-relations/test.sh)

---

### Markdown Structure Verification

This test verifies that the system validates the Markdown structure of MBSE documentation.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should validate markdown structure against formatting standards
- System should detect deviations from the expected markdown structure
- System should provide clear reporting of structure validation issues

##### Test Criteria
- Command exits with success (0) return code for valid structures
- Command reports issues for invalid structures
- Validation messages include file paths and line numbers for issues

##### Test Procedure
TODO: write test procedure

#### Relations
  * verify: [UserRequirements.md/Validate Markdown Structure](../UserRequirements.md#validate-markdown-structure)
  * trace: [tests/test-invalid-relations/test.sh](../../tests/test-invalid-relations/test.sh)

---

### Filesystem Structure Verification

This test verifies that the system validates the organization of files and folders in the repository.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should validate filesystem structure against expected organization
- System should verify that required folders exist
- System should verify that files are appropriately placed

##### Test Criteria
- Command exits with success (0) return code for valid structures
- Command reports issues for invalid structures
- Validation includes checks for missing required folders

##### Test Procedure
TODO: write test procedure

#### Relations
  * verify: [UserRequirements.md/Validate Filesystem Structure](../UserRequirements.md#validate-filesystem-structure)
  * trace: [tests/test-external-folders/test.sh](../../tests/test-external-folders/test.sh)

---

### Internal Consistency Verification

This test verifies that the system checks the internal consistency of the MBSE model.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should validate internal consistency of the model
- System should detect circular dependencies
- System should identify orphaned elements
- System should detect inconsistent relationship patterns

##### Test Criteria
- Command exits with success (0) return code for consistent models
- Command reports issues for inconsistent models
- Validation includes detailed reporting of inconsistencies

##### Test Procedure
TODO: write test procedure

#### Relations
  * verify: [UserRequirements.md/Validate Internal Consistency](../UserRequirements.md#validate-internal-consistency)
  * trace: [tests/test-invalid-relations/test.sh](../../tests/test-invalid-relations/test.sh)

---

### Cross-Component Dependencies Verification

This test verifies that the system validates dependencies across different components of the MBSE model.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should validate dependencies across different model components
- System should verify proper alignment between architectural layers
- System should validate alignment between requirement levels and verification elements

##### Test Criteria
- Command exits with success (0) return code for valid dependencies
- Command reports issues for invalid dependencies
- Validation includes detailed reporting of dependency mismatches

##### Test Procedure
TODO: write test procedure

#### Relations
  * verify: [UserRequirements.md/Validate Cross-Component Dependencies](../UserRequirements.md#validate-cross-component-dependencies)
  * trace: [tests/test-invalid-relations/test.sh](../../tests/test-invalid-relations/test.sh)

---

## Validation Report Tests
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  validation_report_verification["Validation Report Verification"];
  click validation_report_verification "ValidationTests.md#validation-report-verification";
  class validation_report_verification verification;
  validation_report_req["UserRequirements.md/Provide Validation Reports"];
  class validation_report_req requirement;
  click validation_report_req "../UserRequirements.md#provide-validation-reports";
  validation_report_verification -.->|verifies| validation_report_req;
  validation_report_verification -.->|trace| c86fd6ece7a8668a;
  
  verification_gap_verification["Verification Gap Analysis Verification"];
  click verification_gap_verification "ValidationTests.md#verification-gap-analysis-verification";
  class verification_gap_verification verification;
  verification_gap_req["UserRequirements.md/Generate Verifications Reports"];
  class verification_gap_req requirement;
  click verification_gap_req "../UserRequirements.md#generate-verifications-reports";
  verification_gap_verification -.->|verifies| verification_gap_req;
  verification_gap_verification -.->|trace| c86fd6ece7a8668a;
```

---

### Validation Report Verification

This test verifies that the system generates detailed validation reports highlighting any inconsistencies or errors.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should generate validation reports of model structure
- Reports should highlight inconsistencies and errors
- Validation details should include clear descriptions of issues

##### Test Criteria
- Command exits with success (0) return code
- Report contains detailed validation information
- Issues are clearly identified with relevant context

##### Test Procedure
TODO: write test procedure

#### Relations
  * verify: [UserRequirements.md/Provide Validation Reports](../UserRequirements.md#provide-validation-reports)
  * trace: [tests/test-invalid-relations/test.sh](../../tests/test-invalid-relations/test.sh)

---

### Verification Gap Analysis Verification

This test verifies that the system produces reports identifying User and Mission requirements that lack verification relationships.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should identify User and Mission requirements without verifiedBy relationships
- System should generate a report of verification gaps
- Report should be formatted to aid in addressing verification gaps

##### Test Criteria
- Command exits with success (0) return code
- Report accurately identifies requirements without verification
- Report includes all required information to address gaps

##### Test Procedure
TODO: write test procedure

#### Relations
  * verify: [UserRequirements.md/Generate Verifications Reports](../UserRequirements.md#generate-verifications-reports)
  * trace: [tests/test-invalid-relations/test.sh](../../tests/test-invalid-relations/test.sh)

---