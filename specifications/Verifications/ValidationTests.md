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

  9a53e9c00918fd02["JSON Output Format Test"];
  class 9a53e9c00918fd02 verification;
  click 9a53e9c00918fd02 "ValidationTests.md#json-output-format-test";
  ad6f7a2d41d80a38["../UserRequirements.md#model-structure-and-summaries"];
  class ad6f7a2d41d80a38 requirement;
  click ad6f7a2d41d80a38 "../UserRequirements.md#model-structure-and-summaries";
  9a53e9c00918fd02 -.->|verifies| ad6f7a2d41d80a38;
  8b8c9b79dcbe085e["tests/test-invalid-relations/test.sh"];
  class 8b8c9b79dcbe085e default;
  click 8b8c9b79dcbe085e "../../tests/test-invalid-relations/test.sh";
  8b8c9b79dcbe085e -->|satisfies| 9a53e9c00918fd02;
  184bc01c18f5506f["Requirements Files Search and Detection Test"];
  class 184bc01c18f5506f verification;
  click 184bc01c18f5506f "ValidationTests.md#requirements-files-search-and-detection-test";
  66582f9b6bdde6c4["SystemRequirements/Requirements.md#structured-markdown-files-search-and-detection"];
  class 66582f9b6bdde6c4 requirement;
  click 66582f9b6bdde6c4 "../SystemRequirements/Requirements.md#structured-markdown-files-search-and-detection";
  184bc01c18f5506f -.->|verifies| 66582f9b6bdde6c4;
  a29e69e90fa71f39["tests/test-excluded-patterns/test.sh"];
  class a29e69e90fa71f39 default;
  click a29e69e90fa71f39 "../../tests/test-excluded-patterns/test.sh";
  a29e69e90fa71f39 -->|satisfies| 184bc01c18f5506f;
  526fb26c223ad188["Unstructured Documents Test"];
  class 526fb26c223ad188 verification;
  click 526fb26c223ad188 "ValidationTests.md#unstructured-documents-test";
  929c6c204cb3fedb["SystemRequirements/Requirements.md#Excluded File Relation Validation"];
  class 929c6c204cb3fedb requirement;
  click 929c6c204cb3fedb "../SystemRequirements/Requirements.md#excluded-file-relation-validation";
  526fb26c223ad188 -.->|verifies| 929c6c204cb3fedb;
  b91bd280b4e9971["tests/test-valid-relations/test.sh"];
  class b91bd280b4e9971 default;
  click b91bd280b4e9971 "../../tests/test-valid-relations/test.sh";
  b91bd280b4e9971 -->|satisfies| 526fb26c223ad188;
  e37dc7f46d75d46["Invalid Relations Test"];
  class e37dc7f46d75d46 verification;
  click e37dc7f46d75d46 "ValidationTests.md#invalid-relations-test";
  4ecd49d71920c1fc["SystemRequirements/Requirement.md/Detailed Error Handling and Logging"];
  class 4ecd49d71920c1fc requirement;
  click 4ecd49d71920c1fc "../SystemRequirements/Requirements.md#detailed-error-handling-and-logging";
  e37dc7f46d75d46 -.->|verifies| 4ecd49d71920c1fc;
  db64a3e25646a37f["SystemRequirements/Requirement.md/Relation Type Validation"];
  class db64a3e25646a37f requirement;
  click db64a3e25646a37f "../SystemRequirements/Requirements.md#relation-type-validation";
  e37dc7f46d75d46 -.->|verifies| db64a3e25646a37f;
  dc7a9bb1bbebc57f["SystemRequirements/Requirement.md/Relation Element Type Validator"];
  class dc7a9bb1bbebc57f requirement;
  click dc7a9bb1bbebc57f "../SystemRequirements/Requirements.md#relation-element-type-validator";
  e37dc7f46d75d46 -.->|verifies| dc7a9bb1bbebc57f;
  586b073cd97908da["UserRequirements.md/Validate Markdown Structure"];
  class 586b073cd97908da requirement;
  click 586b073cd97908da "../UserRequirements.md#validate-markdown-structure";
  e37dc7f46d75d46 -.->|verifies| 586b073cd97908da;
  c50887ce89be280a["UserRequirements.md/Validate Internal Consistency"];
  class c50887ce89be280a requirement;
  click c50887ce89be280a "../UserRequirements.md#validate-internal-consistency";
  e37dc7f46d75d46 -.->|verifies| c50887ce89be280a;
  3bd9d29239564eeb["UserRequirements.md/Validate Cross-Component Dependencies"];
  class 3bd9d29239564eeb requirement;
  click 3bd9d29239564eeb "../UserRequirements.md#validate-cross-component-dependencies";
  e37dc7f46d75d46 -.->|verifies| 3bd9d29239564eeb;
  ed31b6bed1cde2f8["UserRequirements.md/Provide Validation Reports"];
  class ed31b6bed1cde2f8 requirement;
  click ed31b6bed1cde2f8 "../UserRequirements.md#provide-validation-reports";
  e37dc7f46d75d46 -.->|verifies| ed31b6bed1cde2f8;
  8b8c9b79dcbe085e["tests/test-invalid-relations/test.sh"];
  class 8b8c9b79dcbe085e default;
  click 8b8c9b79dcbe085e "../../tests/test-invalid-relations/test.sh";
  8b8c9b79dcbe085e -->|satisfies| e37dc7f46d75d46;
  6ca2ff1567644e78["Same-File Fragment Relations Test"];
  class 6ca2ff1567644e78 verification;
  click 6ca2ff1567644e78 "ValidationTests.md#same-file-fragment-relations-test";
  6ca2ff1567644e78 -.->|verifies| db64a3e25646a37f;
  bed8d0948b3e5ccd["SystemRequirements/Requirements.md#Requirements Processing"];
  class bed8d0948b3e5ccd requirement;
  click bed8d0948b3e5ccd "../SystemRequirements/Requirements.md#requirements-processing";
  6ca2ff1567644e78 -.->|verifies| bed8d0948b3e5ccd;
  789c690d63402d63["tests/test-fragment-relations/test.sh"];
  class 789c690d63402d63 default;
  click 789c690d63402d63 "../../tests/test-fragment-relations/test.sh";
  789c690d63402d63 -->|satisfies| 6ca2ff1567644e78;
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
- System should detect and report requirement elements with satisfiedBy relations pointing to non-existing local files
- System should detect and report verification elements with satisfiedBy relations pointing to non-existing local files
- System should detect and report requirement elements with verifiedBy relations pointing to non-existing verification elements
- System should allow requirement elements with satisfiedBy relations pointing to existing implementation files
- System should allow verification elements with satisfiedBy relations pointing to existing test scripts
- System should detect and report requirement elements with satisfiedBy relations pointing to other requirement elements (incompatible types)
- System should detect and report verification elements with satisfiedBy relations pointing to other verification elements (incompatible types)
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

  3c0a05c29bf52a57["Subdirectory Processing Verification"];
  class 3c0a05c29bf52a57 verification;
  click 3c0a05c29bf52a57 "ValidationTests.md#subdirectory-processing-verification";
  7bdf935ec6d8effe["SystemRequirements/Requirements.md/Subdirectory Processing Option"];
  class 7bdf935ec6d8effe requirement;
  click 7bdf935ec6d8effe "../SystemRequirements/Requirements.md#subdirectory-processing-option";
  3c0a05c29bf52a57 -.->|verifies| 7bdf935ec6d8effe;
  a565a00c1df16a79["tests/test-subdirectory-functionality/test.sh"];
  class a565a00c1df16a79 default;
  click a565a00c1df16a79 "../../tests/test-subdirectory-functionality/test.sh";
  a565a00c1df16a79 -->|satisfies| 3c0a05c29bf52a57;
```

---

### Subdirectory Processing Verification

This test verifies that the system correctly processes only files within the current directory when run from a subfolder of a git repository.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System shall process only files within the current directory when run from a subfolder
- System shall handle identifier normalization correctly within subdirectory context  
- System shall validate cross-references correctly even when they point outside current directory
- System shall work with validate, model-summary, html, lint, and traces commands
- System shall ignore files outside the current directory scope
- System shall provide meaningful error messages for invalid cross-references

##### Test Criteria
- Commands run from subdirectory process only files within that subdirectory
- Files outside the current directory are not included in processing or output
- Identifier normalization works correctly for paths within subdirectory
- Cross-references to files outside current directory are handled gracefully
- All major commands (validate, model-summary, html, lint, traces) work from subdirectories
- Commands exit with success (0) return code when subdirectory processing works correctly
- Error messages are clear when identifier normalization fails for cross-references

#### Relations
  * verify: [SystemRequirements/Requirements.md/Subdirectory Processing Option](../SystemRequirements/Requirements.md#subdirectory-processing-option)
  * satisfiedBy: [tests/test-subdirectory-functionality/test.sh](../../tests/test-subdirectory-functionality/test.sh)

---