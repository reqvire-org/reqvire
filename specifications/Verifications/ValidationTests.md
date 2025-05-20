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
  click 5bf3bc3f9637ec2c "ValidationTests.md#requirements-files-search-and-detection-test";
  class 5bf3bc3f9637ec2c verification;
  480cb73e8bcb0786["SystemRequirements/Requirements.md#structured-markdown-files-search-and-detection"];
  class 480cb73e8bcb0786 requirement;
  click 480cb73e8bcb0786 "../SystemRequirements/Requirements.md#structured-markdown-files-search-and-detection";
  5bf3bc3f9637ec2c -.->|verifies| 480cb73e8bcb0786;
  ec11a68aa5b4bdc1["tests/test-excluded-patterns/test.sh"];
  class ec11a68aa5b4bdc1 default;
  click ec11a68aa5b4bdc1 "../../tests/test-excluded-patterns/test.sh";
  ec11a68aa5b4bdc1 -->|satisfies| 5bf3bc3f9637ec2c;
  94776cf061319810["Unstructured Documents Test"];
  click 94776cf061319810 "ValidationTests.md#unstructured-documents-test";
  class 94776cf061319810 verification;
  3871ef72a30780e5["SystemRequirements/Requirements.md#Excluded File Relation Validation"];
  class 3871ef72a30780e5 requirement;
  click 3871ef72a30780e5 "../SystemRequirements/Requirements.md#excluded-file-relation-validation";
  94776cf061319810 -.->|verifies| 3871ef72a30780e5;
  7cf643f1c1578783["tests/test-valid-relations/test.sh"];
  class 7cf643f1c1578783 default;
  click 7cf643f1c1578783 "../../tests/test-valid-relations/test.sh";
  7cf643f1c1578783 -->|satisfies| 94776cf061319810;
  a24c2f4208509008["Invalid Relations Test"];
  click a24c2f4208509008 "ValidationTests.md#invalid-relations-test";
  class a24c2f4208509008 verification;
  d72f6096b9a5dd8e["SystemRequirements/Requirement.md/Detailed Error Handling and Logging"];
  class d72f6096b9a5dd8e requirement;
  click d72f6096b9a5dd8e "../SystemRequirements/Requirements.md#detailed-error-handling-and-logging";
  a24c2f4208509008 -.->|verifies| d72f6096b9a5dd8e;
  bff4e3e834a9ffcc["SystemRequirements/Requirement.md/Relation Type Validation"];
  class bff4e3e834a9ffcc requirement;
  click bff4e3e834a9ffcc "../SystemRequirements/Requirements.md#relation-type-validation";
  a24c2f4208509008 -.->|verifies| bff4e3e834a9ffcc;
  774d12db509b4a55["SystemRequirements/Requirement.md/Relation Element Type Validator"];
  class 774d12db509b4a55 requirement;
  click 774d12db509b4a55 "../SystemRequirements/Requirements.md#relation-element-type-validator";
  a24c2f4208509008 -.->|verifies| 774d12db509b4a55;
  7ec3cb7f400a2e8d["UserRequirements.md/Validate Markdown Structure"];
  class 7ec3cb7f400a2e8d requirement;
  click 7ec3cb7f400a2e8d "../UserRequirements.md#validate-markdown-structure";
  a24c2f4208509008 -.->|verifies| 7ec3cb7f400a2e8d;
  f9182ad2999d989c["UserRequirements.md/Validate Internal Consistency"];
  class f9182ad2999d989c requirement;
  click f9182ad2999d989c "../UserRequirements.md#validate-internal-consistency";
  a24c2f4208509008 -.->|verifies| f9182ad2999d989c;
  ee05a46627b568b7["UserRequirements.md/Validate Cross-Component Dependencies"];
  class ee05a46627b568b7 requirement;
  click ee05a46627b568b7 "../UserRequirements.md#validate-cross-component-dependencies";
  a24c2f4208509008 -.->|verifies| ee05a46627b568b7;
  2d3cfde19fc6bb79["UserRequirements.md/Provide Validation Reports"];
  class 2d3cfde19fc6bb79 requirement;
  click 2d3cfde19fc6bb79 "../UserRequirements.md#provide-validation-reports";
  a24c2f4208509008 -.->|verifies| 2d3cfde19fc6bb79;
  36d2b2cb50297425["tests/test-invalid-relations/test.sh"];
  class 36d2b2cb50297425 default;
  click 36d2b2cb50297425 "../../tests/test-invalid-relations/test.sh";
  36d2b2cb50297425 -->|satisfies| a24c2f4208509008;
  ec87ed0e04fcf5c6["JSON Output Format Test"];
  click ec87ed0e04fcf5c6 "ValidationTests.md#json-output-format-test";
  class ec87ed0e04fcf5c6 verification;
  40de7485b25294["../UserRequirements.md#model-structure-and-summaries"];
  class 40de7485b25294 requirement;
  click 40de7485b25294 "../UserRequirements.md#model-structure-and-summaries";
  ec87ed0e04fcf5c6 -.->|verifies| 40de7485b25294;
  36d2b2cb50297425 -->|satisfies| ec87ed0e04fcf5c6;
  51427eae92058e14["Same-File Fragment Relations Test"];
  click 51427eae92058e14 "ValidationTests.md#same-file-fragment-relations-test";
  class 51427eae92058e14 verification;
  51427eae92058e14 -.->|verifies| bff4e3e834a9ffcc;
  f24f11691f55af62["SystemRequirements/Requirements.md#Requirements Processing"];
  class f24f11691f55af62 requirement;
  click f24f11691f55af62 "../SystemRequirements/Requirements.md#requirements-processing";
  51427eae92058e14 -.->|verifies| f24f11691f55af62;
  a4c8929dc751b4b1["tests/test-fragment-relations/test.sh"];
  class a4c8929dc751b4b1 default;
  click a4c8929dc751b4b1 "../../tests/test-fragment-relations/test.sh";
  a4c8929dc751b4b1 -->|satisfies| 51427eae92058e14;
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
  * verify: [../UserRequirements.md#model-structure-and-summaries](../UserRequirements.md#model-structure-and-summaries)
  * satisfiedBy: [tests/test-invalid-relations/test.sh](../../tests/test-invalid-relations/test.sh)

---

### Requirements Files Search and Detection Test

This test verifies that the system correctly searches for and detects structured document files according to specified patterns and configurations.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System shall find all structured document files in project structure based on configuration
- System shall respect excluded file patterns defined in configuration
- System shall handle nested directory structures correctly
- System shall correctly identify and categorize different file types


##### Test Criteria
- All expected requirements files are identified
- Files matching exclusion patterns are skipped
- Nested directories are correctly traversed
- Non-markdown files are handled appropriately

#### Relations
  * verify: [SystemRequirements/Requirements.md#structured-markdown-files-search-and-detection](../SystemRequirements/Requirements.md#structured-markdown-files-search-and-detection)
  * satisfiedBy: [tests/test-excluded-patterns/test.sh](../../tests/test-excluded-patterns/test.sh)

---

### Unstructured Documents Test

This test verifies that the system correctly validates relations to excluded files.

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
  * verify: [SystemRequirements/Requirements.md#Excluded File Relation Validation](../SystemRequirements/Requirements.md#excluded-file-relation-validation)
  * satisfiedBy: [tests/test-valid-relations/test.sh](../../tests/test-valid-relations/test.sh)

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