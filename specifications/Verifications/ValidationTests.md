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

  5bf3bc3f9637ec2c["Requirements Files Search and Detection Test"];
  click 5bf3bc3f9637ec2c "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/Verifications/ValidationTests.md#requirements-files-search-and-detection-test";
  class 5bf3bc3f9637ec2c verification;
  33b15634cbc8d029["SystemRequirements/Requirements.md#requirements-files-search-and-detection"];
  class 33b15634cbc8d029 requirement;
  click 33b15634cbc8d029 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#requirements-files-search-and-detection";
  5bf3bc3f9637ec2c -.->|verifies| 33b15634cbc8d029;
  ec11a68aa5b4bdc1["tests/test-excluded-patterns/test.sh"];
  class ec11a68aa5b4bdc1 default;
  click ec11a68aa5b4bdc1 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/tests/test-excluded-patterns/test.sh";
  ec11a68aa5b4bdc1 -->|satisfies| 5bf3bc3f9637ec2c;
  19bbc820b0234421["Excluded File Relation Validation Test"];
  click 19bbc820b0234421 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/Verifications/ValidationTests.md#excluded-file-relation-validation-test";
  class 19bbc820b0234421 verification;
  3871ef72a30780e5["SystemRequirements/Requirements.md#excluded-file-relation-validation"];
  class 3871ef72a30780e5 requirement;
  click 3871ef72a30780e5 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#excluded-file-relation-validation";
  19bbc820b0234421 -.->|verifies| 3871ef72a30780e5;
  ec11a68aa5b4bdc1 -->|satisfies| 19bbc820b0234421;
  ec87ed0e04fcf5c6["JSON Output Format Test"];
  click ec87ed0e04fcf5c6 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/Verifications/ValidationTests.md#json-output-format-test";
  class ec87ed0e04fcf5c6 verification;
  bf6c9ff8abbc637b["SystemRequirements/Requirements.md#json-output-format"];
  class bf6c9ff8abbc637b requirement;
  click bf6c9ff8abbc637b "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#json-output-format";
  ec87ed0e04fcf5c6 -.->|verifies| bf6c9ff8abbc637b;
  36d2b2cb50297425["tests/test-invalid-relations/test.sh"];
  class 36d2b2cb50297425 default;
  click 36d2b2cb50297425 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/tests/test-invalid-relations/test.sh";
  36d2b2cb50297425 -->|satisfies| ec87ed0e04fcf5c6;
  51427eae92058e14["Same-File Fragment Relations Test"];
  click 51427eae92058e14 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/Verifications/ValidationTests.md#same-file-fragment-relations-test";
  class 51427eae92058e14 verification;
  bff4e3e834a9ffcc["SystemRequirements/Requirements.md#Relation Type Validation"];
  class bff4e3e834a9ffcc requirement;
  click bff4e3e834a9ffcc "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#relation-type-validation";
  51427eae92058e14 -.->|verifies| bff4e3e834a9ffcc;
  f24f11691f55af62["SystemRequirements/Requirements.md#Requirements Processing"];
  class f24f11691f55af62 requirement;
  click f24f11691f55af62 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#requirements-processing";
  51427eae92058e14 -.->|verifies| f24f11691f55af62;
  a4c8929dc751b4b1["tests/test-fragment-relations/test.sh"];
  class a4c8929dc751b4b1 default;
  click a4c8929dc751b4b1 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/tests/test-fragment-relations/test.sh";
  a4c8929dc751b4b1 -->|satisfies| 51427eae92058e14;
  94776cf061319810["Unstructured Documents Test"];
  click 94776cf061319810 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/Verifications/ValidationTests.md#unstructured-documents-test";
  class 94776cf061319810 verification;
  3f1abb8fd2d7c9c2["SystemRequirements/Requirements.md#unstructured-documents"];
  class 3f1abb8fd2d7c9c2 requirement;
  click 3f1abb8fd2d7c9c2 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#unstructured-documents";
  94776cf061319810 -.->|verifies| 3f1abb8fd2d7c9c2;
  7cf643f1c1578783["tests/test-valid-relations/test.sh"];
  class 7cf643f1c1578783 default;
  click 7cf643f1c1578783 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/tests/test-valid-relations/test.sh";
  7cf643f1c1578783 -->|satisfies| 94776cf061319810;
  a24c2f4208509008["Invalid Relations Test"];
  click a24c2f4208509008 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/Verifications/ValidationTests.md#invalid-relations-test";
  class a24c2f4208509008 verification;
  d72f6096b9a5dd8e["SystemRequirements/Requirement.md/Detailed Error Handling and Logging"];
  class d72f6096b9a5dd8e requirement;
  click d72f6096b9a5dd8e "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#detailed-error-handling-and-logging";
  a24c2f4208509008 -.->|verifies| d72f6096b9a5dd8e;
  a24c2f4208509008 -.->|verifies| bff4e3e834a9ffcc;
  774d12db509b4a55["SystemRequirements/Requirement.md/Relation Element Type Validator"];
  class 774d12db509b4a55 requirement;
  click 774d12db509b4a55 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#relation-element-type-validator";
  a24c2f4208509008 -.->|verifies| 774d12db509b4a55;
  7ec3cb7f400a2e8d["UserRequirements.md/Validate Markdown Structure"];
  class 7ec3cb7f400a2e8d requirement;
  click 7ec3cb7f400a2e8d "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#validate-markdown-structure";
  a24c2f4208509008 -.->|verifies| 7ec3cb7f400a2e8d;
  f9182ad2999d989c["UserRequirements.md/Validate Internal Consistency"];
  class f9182ad2999d989c requirement;
  click f9182ad2999d989c "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#validate-internal-consistency";
  a24c2f4208509008 -.->|verifies| f9182ad2999d989c;
  ee05a46627b568b7["UserRequirements.md/Validate Cross-Component Dependencies"];
  class ee05a46627b568b7 requirement;
  click ee05a46627b568b7 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#validate-cross-component-dependencies";
  a24c2f4208509008 -.->|verifies| ee05a46627b568b7;
  2d3cfde19fc6bb79["UserRequirements.md/Provide Validation Reports"];
  class 2d3cfde19fc6bb79 requirement;
  click 2d3cfde19fc6bb79 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#provide-validation-reports";
  a24c2f4208509008 -.->|verifies| 2d3cfde19fc6bb79;
  36d2b2cb50297425 -->|satisfies| a24c2f4208509008;
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

#### Relations
  * verify: [SystemRequirements/Requirement.md/Detailed Error Handling and Logging](../SystemRequirements/Requirements.md#detailed-error-handling-and-logging)
  * verify: [SystemRequirements/Requirement.md/Relation Type Validation](../SystemRequirements/Requirements.md#relation-type-validation)  
  * verify: [SystemRequirements/Requirement.md/Relation Element Type Validator](../SystemRequirements/Requirements.md#relation-element-type-validator)  
  * verify: [UserRequirements.md/Validate Markdown Structure](../UserRequirements.md#validate-markdown-structure)  
  * verify: [UserRequirements.md/Validate Internal Consistency](../UserRequirements.md#validate-internal-consistency)
  * verify: [UserRequirements.md/Validate Cross-Component Dependencies](../UserRequirements.md#validate-cross-component-dependencies)      
  * verify: [UserRequirements.md/Provide Validation Reports](../UserRequirements.md#provide-validation-reports)  
  * satisfiedBy: [tests/test-invalid-relations/test.sh](../../tests/test-invalid-relations/test.sh)

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


#### Relations
  * verify: [SystemRequirements/Requirements.md#Relation Type Validation](../SystemRequirements/Requirements.md#relation-type-validation)
  * verify: [SystemRequirements/Requirements.md#Requirements Processing](../SystemRequirements/Requirements.md#requirements-processing)
  * satisfiedBy: [tests/test-fragment-relations/test.sh](../../tests/test-fragment-relations/test.sh)

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


#### Relations
  * verify: [SystemRequirements/Requirements.md#json-output-format](../SystemRequirements/Requirements.md#json-output-format)
  * satisfiedBy: [tests/test-invalid-relations/test.sh](../../tests/test-invalid-relations/test.sh)

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


#### Relations
  * verify: [SystemRequirements/Requirements.md#requirements-files-search-and-detection](../SystemRequirements/Requirements.md#requirements-files-search-and-detection)
  * satisfiedBy: [tests/test-excluded-patterns/test.sh](../../tests/test-excluded-patterns/test.sh)

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


#### Relations
  * verify: [SystemRequirements/Requirements.md#unstructured-documents](../SystemRequirements/Requirements.md#unstructured-documents)
  * satisfiedBy: [tests/test-valid-relations/test.sh](../../tests/test-valid-relations/test.sh)

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

#### Relations
  * verify: [SystemRequirements/Requirements.md#excluded-file-relation-validation](../SystemRequirements/Requirements.md#excluded-file-relation-validation)
  * satisfiedBy: [tests/test-excluded-patterns/test.sh](../../tests/test-excluded-patterns/test.sh)

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