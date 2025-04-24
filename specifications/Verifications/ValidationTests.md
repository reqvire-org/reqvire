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

  1b82af45e393cf5a["Same-File Fragment Relations Test"];
  click 1b82af45e393cf5a "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/ValidationTests.md#same-file-fragment-relations-test";
  class 1b82af45e393cf5a verification;
  7390fcf6e2c328f4["SystemRequirements/Requirements.md#Relation Type Validation"];
  class 7390fcf6e2c328f4 requirement;
  click 7390fcf6e2c328f4 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#relation-type-validation";
  1b82af45e393cf5a -.->|verifies| 7390fcf6e2c328f4;
  6aeba4bf990bc9e4["SystemRequirements/Requirements.md#Requirements Processing"];
  class 6aeba4bf990bc9e4 requirement;
  click 6aeba4bf990bc9e4 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#requirements-processing";
  1b82af45e393cf5a -.->|verifies| 6aeba4bf990bc9e4;
  be191d3d32b91f2c["tests/test-fragment-relations/test.sh"];
  class be191d3d32b91f2c default;
  click be191d3d32b91f2c "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/tests/test-fragment-relations/test.sh";
  1b82af45e393cf5a -.->|trace| be191d3d32b91f2c;
  bf27e485475aec65["JSON Output Format Test"];
  click bf27e485475aec65 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/ValidationTests.md#json-output-format-test";
  class bf27e485475aec65 verification;
  1961923a36e51056["SystemRequirements/Requirements.md#json-output-format"];
  class 1961923a36e51056 requirement;
  click 1961923a36e51056 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#json-output-format";
  bf27e485475aec65 -.->|verifies| 1961923a36e51056;
  af25a95139b7ce51["tests/test-invalid-relations/test.sh"];
  class af25a95139b7ce51 default;
  click af25a95139b7ce51 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/tests/test-invalid-relations/test.sh";
  bf27e485475aec65 -.->|trace| af25a95139b7ce51;
  7c22da9123ea1f8c["Excluded File Relation Validation Test"];
  click 7c22da9123ea1f8c "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/ValidationTests.md#excluded-file-relation-validation-test";
  class 7c22da9123ea1f8c verification;
  8e8279e74b2e2559["SystemRequirements/Requirements.md#excluded-file-relation-validation"];
  class 8e8279e74b2e2559 requirement;
  click 8e8279e74b2e2559 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#excluded-file-relation-validation";
  7c22da9123ea1f8c -.->|verifies| 8e8279e74b2e2559;
  12603942d997b0fa["tests/test-excluded-patterns/test.sh"];
  class 12603942d997b0fa default;
  click 12603942d997b0fa "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/tests/test-excluded-patterns/test.sh";
  7c22da9123ea1f8c -.->|trace| 12603942d997b0fa;
  4f3ff0127dd7115b["Unstructured Documents Test"];
  click 4f3ff0127dd7115b "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/ValidationTests.md#unstructured-documents-test";
  class 4f3ff0127dd7115b verification;
  15fa7a7cfb9bc2a3["SystemRequirements/Requirements.md#unstructured-documents"];
  class 15fa7a7cfb9bc2a3 requirement;
  click 15fa7a7cfb9bc2a3 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#unstructured-documents";
  4f3ff0127dd7115b -.->|verifies| 15fa7a7cfb9bc2a3;
  150e2af822f85d6c["tests/test-valid-relations/test.sh"];
  class 150e2af822f85d6c default;
  click 150e2af822f85d6c "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/tests/test-valid-relations/test.sh";
  4f3ff0127dd7115b -.->|trace| 150e2af822f85d6c;
  379602464f079711["Requirements Files Search and Detection Test"];
  click 379602464f079711 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/ValidationTests.md#requirements-files-search-and-detection-test";
  class 379602464f079711 verification;
  fb2eb9df7ab72606["SystemRequirements/Requirements.md#requirements-files-search-and-detection"];
  class fb2eb9df7ab72606 requirement;
  click fb2eb9df7ab72606 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#requirements-files-search-and-detection";
  379602464f079711 -.->|verifies| fb2eb9df7ab72606;
  379602464f079711 -.->|trace| 12603942d997b0fa;
  aecfae8ba4d19a55["Invalid Relations Test"];
  click aecfae8ba4d19a55 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/ValidationTests.md#invalid-relations-test";
  class aecfae8ba4d19a55 verification;
  8cd1d72c54544cd9["SystemRequirements/Requirement.md/Detailed Error Handling and Logging"];
  class 8cd1d72c54544cd9 requirement;
  click 8cd1d72c54544cd9 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#detailed-error-handling-and-logging";
  aecfae8ba4d19a55 -.->|verifies| 8cd1d72c54544cd9;
  aecfae8ba4d19a55 -.->|verifies| 7390fcf6e2c328f4;
  9b1583400f6bf5ee["SystemRequirements/Requirement.md/Relation Element Type Validator"];
  class 9b1583400f6bf5ee requirement;
  click 9b1583400f6bf5ee "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#relation-element-type-validator";
  aecfae8ba4d19a55 -.->|verifies| 9b1583400f6bf5ee;
  cc431fdb7d8cadde["UserRequirements.md/Validate Markdown Structure"];
  class cc431fdb7d8cadde requirement;
  click cc431fdb7d8cadde "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#validate-markdown-structure";
  aecfae8ba4d19a55 -.->|verifies| cc431fdb7d8cadde;
  38ec9e189e6980d7["UserRequirements.md/Validate Internal Consistency"];
  class 38ec9e189e6980d7 requirement;
  click 38ec9e189e6980d7 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#validate-internal-consistency";
  aecfae8ba4d19a55 -.->|verifies| 38ec9e189e6980d7;
  a6c1d4d1f6866aa8["UserRequirements.md/Validate Cross-Component Dependencies"];
  class a6c1d4d1f6866aa8 requirement;
  click a6c1d4d1f6866aa8 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#validate-cross-component-dependencies";
  aecfae8ba4d19a55 -.->|verifies| a6c1d4d1f6866aa8;
  a4b1fa740dda1d5["UserRequirements.md/Provide Validation Reports"];
  class a4b1fa740dda1d5 requirement;
  click a4b1fa740dda1d5 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#provide-validation-reports";
  aecfae8ba4d19a55 -.->|verifies| a4b1fa740dda1d5;
  aecfae8ba4d19a55 -.->|trace| af25a95139b7ce51;
```

---

### Invalid Relations Test

The verification test checks that ReqFlow correctly identifies and reports invalid relations of different kinds and provide validation report with expected details.

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
  * verify: [UserRequirements.md/Validate Markdown Structure](../UserRequirements.md#validate-markdown-structure)  
  * verify: [UserRequirements.md/Validate Internal Consistency](../UserRequirements.md#validate-internal-consistency)
  * verify: [UserRequirements.md/Validate Cross-Component Dependencies](../UserRequirements.md#validate-cross-component-dependencies)      
  * verify: [UserRequirements.md/Provide Validation Reports](../UserRequirements.md#provide-validation-reports)  
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

### JSON Output Format Test

This test verifies that the system properly implements JSON output formatting for validation and other commands.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System shall support --json flag for validation commands
- JSON output shall be properly formatted according to a consistent schema
- JSON structure shall be parsable and machine-readable
- All relevant validation data shall be included in the output

##### Test Criteria
- Commands with --json flag produce valid JSON
- JSON output can be parsed without errors
- Output structure matches expected schema
- All validation errors are properly represented in JSON format

##### Test Procedure
1. Create test fixtures with known validation issues
2. Run ReqFlow with --validate --json flag on the test fixtures
3. Verify that output is valid JSON by parsing it
4. Confirm all validation errors are represented in the JSON output
5. Verify that JSON structure is consistent across different validation scenarios

#### Relations
  * verify: [SystemRequirements/Requirements.md#json-output-format](../SystemRequirements/Requirements.md#json-output-format)
  * trace: [tests/test-invalid-relations/test.sh](../../tests/test-invalid-relations/test.sh)

---

### Requirements Files Search and Detection Test

This test verifies that the system correctly searches for and detects requirements files according to specified patterns and configurations.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System shall find all requirements files in project structure based on configuration
- System shall respect excluded file patterns defined in configuration
- System shall handle nested directory structures correctly
- System shall correctly identify and categorize different file types
- System shall process both specifications and external folders

##### Test Criteria
- All expected requirements files are identified
- Files matching exclusion patterns are skipped
- Nested directories are correctly traversed
- Both specifications and external folders are processed
- Non-markdown files are handled appropriately

##### Test Procedure
1. Create test fixtures with various directory structures including:
   - Files in different levels of nesting
   - Files matching exclusion patterns
   - Files in both specifications and external folders
2. Configure ReqFlow with specific pattern rules
3. Run ReqFlow on the test fixtures
4. Verify that all expected files are found and processed
5. Verify that excluded files are correctly skipped
6. Verify correct handling of nested directories

#### Relations
  * verify: [SystemRequirements/Requirements.md#requirements-files-search-and-detection](../SystemRequirements/Requirements.md#requirements-files-search-and-detection)
  * trace: [tests/test-excluded-patterns/test.sh](../../tests/test-excluded-patterns/test.sh)

---

### Unstructured Documents Test

This test verifies that the system correctly handles unstructured documents for relation targets.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System shall allow referencing unstructured documents (text files, code files)
- System shall not attempt to parse unstructured documents as requirements
- System shall validate that referenced unstructured documents exist
- System shall not report validation errors for valid references to unstructured documents

##### Test Criteria
- Relations referencing unstructured documents are treated as valid
- No attempt is made to extract elements from unstructured documents
- Validation succeeds when referenced unstructured documents exist
- Validation fails when referenced unstructured documents don't exist

##### Test Procedure
1. Create test fixtures with:
   - Requirements referencing unstructured documents (.txt, .cpp files)
   - Valid references to existing unstructured documents
   - Invalid references to non-existent unstructured documents
2. Run ReqFlow validation on the test fixtures
3. Verify that valid references to unstructured documents are accepted
4. Verify that invalid references to non-existent files are reported
5. Verify that unstructured documents are not parsed for elements

#### Relations
  * verify: [SystemRequirements/Requirements.md#unstructured-documents](../SystemRequirements/Requirements.md#unstructured-documents)
  * trace: [tests/test-valid-relations/test.sh](../../tests/test-valid-relations/test.sh)

---

### Excluded File Relation Validation Test

This test verifies that the system correctly validates relations to excluded files.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System shall validate existence of excluded files referenced in relations
- System shall not parse excluded files for elements
- System shall allow relations TO excluded files but not FROM excluded files
- System shall apply exclusion patterns based on configuration

##### Test Criteria
- Relations to excluded files are treated as valid if files exist
- Relations from excluded files are not processed
- Excluded files are not parsed for elements
- Files matching exclusion patterns are correctly identified

##### Test Procedure
1. Create test fixtures with:
   - Files matching exclusion patterns
   - Requirements referencing excluded files
   - Requirements referenced by excluded files (should be ignored)
2. Configure ReqFlow with specific exclusion patterns
3. Run ReqFlow validation on the test fixtures
4. Verify that relations to excluded files are validated for file existence
5. Verify that excluded files are not parsed for elements
6. Verify that relations from excluded files are not processed

#### Relations
  * verify: [SystemRequirements/Requirements.md#excluded-file-relation-validation](../SystemRequirements/Requirements.md#excluded-file-relation-validation)
  * trace: [tests/test-excluded-patterns/test.sh](../../tests/test-excluded-patterns/test.sh)

---

## Validation Report Tests
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  f972dbd490433f18["Verification Gap Analysis Verification"];
  click f972dbd490433f18 "ValidationTests.md#verification-gap-analysis-verification";
  class f972dbd490433f18 verification;
  d0e9e8d143493413["UserRequirements.md/Generate Verifications Reports"];
  class d0e9e8d143493413 requirement;
  click d0e9e8d143493413 "../UserRequirements.md#generate-verifications-reports";
  f972dbd490433f18 -.->|verifies| d0e9e8d143493413;
  c86fd6ece7a8668a["tests/test-invalid-relations/test.sh"];
  class c86fd6ece7a8668a default;
  click c86fd6ece7a8668a "../../tests/test-invalid-relations/test.sh";
  f972dbd490433f18 -.->|trace| c86fd6ece7a8668a;
  c4332134ea225a0e["Validation Report Verification"];
  click c4332134ea225a0e "ValidationTests.md#validation-report-verification";
  class c4332134ea225a0e verification;
  482c757913204fb8["UserRequirements.md/Provide Validation Reports"];
  class 482c757913204fb8 requirement;
  click 482c757913204fb8 "../UserRequirements.md#provide-validation-reports";
  c4332134ea225a0e -.->|verifies| 482c757913204fb8;
  c4332134ea225a0e -.->|trace| c86fd6ece7a8668a;
```

---