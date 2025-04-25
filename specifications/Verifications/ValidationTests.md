# Validation Tests

This document verifies the requirements for Reqvire's validation functionality.

## Relation Validation Tests
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  3c58f74cf3262214["JSON Output Format Test"];
  click 3c58f74cf3262214 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/ValidationTests.md#json-output-format-test";
  class 3c58f74cf3262214 verification;
  34955d64b2f2498a["SystemRequirements/Requirements.md#json-output-format"];
  class 34955d64b2f2498a requirement;
  click 34955d64b2f2498a "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#json-output-format";
  3c58f74cf3262214 -.->|verifies| 34955d64b2f2498a;
  dcd79d89b13530ad["tests/test-invalid-relations/test.sh"];
  class dcd79d89b13530ad default;
  click dcd79d89b13530ad "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/tests/test-invalid-relations/test.sh";
  3c58f74cf3262214 -.->|trace| dcd79d89b13530ad;
  38657421ae86147b["Same-File Fragment Relations Test"];
  click 38657421ae86147b "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/ValidationTests.md#same-file-fragment-relations-test";
  class 38657421ae86147b verification;
  5c3a18f061fe0b18["SystemRequirements/Requirements.md#Relation Type Validation"];
  class 5c3a18f061fe0b18 requirement;
  click 5c3a18f061fe0b18 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#relation-type-validation";
  38657421ae86147b -.->|verifies| 5c3a18f061fe0b18;
  c6d19363284e9125["SystemRequirements/Requirements.md#Requirements Processing"];
  class c6d19363284e9125 requirement;
  click c6d19363284e9125 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#requirements-processing";
  38657421ae86147b -.->|verifies| c6d19363284e9125;
  a2e0677ef3715c69["tests/test-fragment-relations/test.sh"];
  class a2e0677ef3715c69 default;
  click a2e0677ef3715c69 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/tests/test-fragment-relations/test.sh";
  38657421ae86147b -.->|trace| a2e0677ef3715c69;
  61d27174e5644f8a["Unstructured Documents Test"];
  click 61d27174e5644f8a "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/ValidationTests.md#unstructured-documents-test";
  class 61d27174e5644f8a verification;
  d6a1fbde09883ed0["SystemRequirements/Requirements.md#unstructured-documents"];
  class d6a1fbde09883ed0 requirement;
  click d6a1fbde09883ed0 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#unstructured-documents";
  61d27174e5644f8a -.->|verifies| d6a1fbde09883ed0;
  ee8eb987af1999ea["tests/test-valid-relations/test.sh"];
  class ee8eb987af1999ea default;
  click ee8eb987af1999ea "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/tests/test-valid-relations/test.sh";
  61d27174e5644f8a -.->|trace| ee8eb987af1999ea;
  e7ae85330d65ed2f["Excluded File Relation Validation Test"];
  click e7ae85330d65ed2f "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/ValidationTests.md#excluded-file-relation-validation-test";
  class e7ae85330d65ed2f verification;
  44b70517dea81a26["SystemRequirements/Requirements.md#excluded-file-relation-validation"];
  class 44b70517dea81a26 requirement;
  click 44b70517dea81a26 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#excluded-file-relation-validation";
  e7ae85330d65ed2f -.->|verifies| 44b70517dea81a26;
  85cb273f062c3eec["tests/test-excluded-patterns/test.sh"];
  class 85cb273f062c3eec default;
  click 85cb273f062c3eec "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/tests/test-excluded-patterns/test.sh";
  e7ae85330d65ed2f -.->|trace| 85cb273f062c3eec;
  cf6ed9d8f0fe7cb6["Requirements Files Search and Detection Test"];
  click cf6ed9d8f0fe7cb6 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/ValidationTests.md#requirements-files-search-and-detection-test";
  class cf6ed9d8f0fe7cb6 verification;
  734ff30870a0bde0["SystemRequirements/Requirements.md#requirements-files-search-and-detection"];
  class 734ff30870a0bde0 requirement;
  click 734ff30870a0bde0 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#requirements-files-search-and-detection";
  cf6ed9d8f0fe7cb6 -.->|verifies| 734ff30870a0bde0;
  cf6ed9d8f0fe7cb6 -.->|trace| 85cb273f062c3eec;
  7e0adaeae461083c["Invalid Relations Test"];
  click 7e0adaeae461083c "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/ValidationTests.md#invalid-relations-test";
  class 7e0adaeae461083c verification;
  d0ab992050899c65["SystemRequirements/Requirement.md/Detailed Error Handling and Logging"];
  class d0ab992050899c65 requirement;
  click d0ab992050899c65 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#detailed-error-handling-and-logging";
  7e0adaeae461083c -.->|verifies| d0ab992050899c65;
  7e0adaeae461083c -.->|verifies| 5c3a18f061fe0b18;
  27a77338e4f9304c["SystemRequirements/Requirement.md/Relation Element Type Validator"];
  class 27a77338e4f9304c requirement;
  click 27a77338e4f9304c "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#relation-element-type-validator";
  7e0adaeae461083c -.->|verifies| 27a77338e4f9304c;
  b6ee889a6a1ac979["UserRequirements.md/Validate Markdown Structure"];
  class b6ee889a6a1ac979 requirement;
  click b6ee889a6a1ac979 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#validate-markdown-structure";
  7e0adaeae461083c -.->|verifies| b6ee889a6a1ac979;
  944bd4459db32d65["UserRequirements.md/Validate Internal Consistency"];
  class 944bd4459db32d65 requirement;
  click 944bd4459db32d65 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#validate-internal-consistency";
  7e0adaeae461083c -.->|verifies| 944bd4459db32d65;
  a830c3c9ac9cf1a9["UserRequirements.md/Validate Cross-Component Dependencies"];
  class a830c3c9ac9cf1a9 requirement;
  click a830c3c9ac9cf1a9 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#validate-cross-component-dependencies";
  7e0adaeae461083c -.->|verifies| a830c3c9ac9cf1a9;
  e1f6859d4a4ea65b["UserRequirements.md/Provide Validation Reports"];
  class e1f6859d4a4ea65b requirement;
  click e1f6859d4a4ea65b "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#provide-validation-reports";
  7e0adaeae461083c -.->|verifies| e1f6859d4a4ea65b;
  7e0adaeae461083c -.->|trace| dcd79d89b13530ad;
```

---

### Invalid Relations Test

The verification test checks that Reqvire correctly identifies and reports invalid relations of different kinds and provide validation report with expected details.

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
2. Run Reqvire validation on the test fixture
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

This test verifies that Reqvire correctly handles and validates relations to fragments within the same file.

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
2. Run Reqvire validation on the test fixtures
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
2. Run Reqvire with --validate --json flag on the test fixtures
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
2. Configure Reqvire with specific pattern rules
3. Run Reqvire on the test fixtures
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
2. Run Reqvire validation on the test fixtures
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
2. Configure Reqvire with specific exclusion patterns
3. Run Reqvire validation on the test fixtures
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