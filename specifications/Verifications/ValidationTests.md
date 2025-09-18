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

  e37dc7f46d75d46["Invalid Relations Test"];
  class e37dc7f46d75d46 verification;
  click e37dc7f46d75d46 "ValidationTests.md#invalid-relations-test";
  8b8c9b79dcbe085e["tests/test-invalid-relations/test.sh"];
  class 8b8c9b79dcbe085e default;
  click 8b8c9b79dcbe085e "../../tests/test-invalid-relations/test.sh";
  e37dc7f46d75d46 -->|satisfiedBy| 8b8c9b79dcbe085e;
  9a53e9c00918fd02["JSON Output Format Test"];
  class 9a53e9c00918fd02 verification;
  click 9a53e9c00918fd02 "ValidationTests.md#json-output-format-test";
  8b8c9b79dcbe085e["tests/test-invalid-relations/test.sh"];
  class 8b8c9b79dcbe085e default;
  click 8b8c9b79dcbe085e "../../tests/test-invalid-relations/test.sh";
  9a53e9c00918fd02 -->|satisfiedBy| 8b8c9b79dcbe085e;
  526fb26c223ad188["Unstructured Documents Test"];
  class 526fb26c223ad188 verification;
  click 526fb26c223ad188 "ValidationTests.md#unstructured-documents-test";
  b91bd280b4e9971["tests/test-valid-relations/test.sh"];
  class b91bd280b4e9971 default;
  click b91bd280b4e9971 "../../tests/test-valid-relations/test.sh";
  526fb26c223ad188 -->|satisfiedBy| b91bd280b4e9971;
  6ca2ff1567644e78["Same-File Fragment Relations Test"];
  class 6ca2ff1567644e78 verification;
  click 6ca2ff1567644e78 "ValidationTests.md#same-file-fragment-relations-test";
  789c690d63402d63["tests/test-fragment-relations/test.sh"];
  class 789c690d63402d63 default;
  click 789c690d63402d63 "../../tests/test-fragment-relations/test.sh";
  6ca2ff1567644e78 -->|satisfiedBy| 789c690d63402d63;
  184bc01c18f5506f["Requirements Files Search and Detection Test"];
  class 184bc01c18f5506f verification;
  click 184bc01c18f5506f "ValidationTests.md#requirements-files-search-and-detection-test";
  a29e69e90fa71f39["tests/test-excluded-patterns/test.sh"];
  class a29e69e90fa71f39 default;
  click a29e69e90fa71f39 "../../tests/test-excluded-patterns/test.sh";
  184bc01c18f5506f -->|satisfiedBy| a29e69e90fa71f39;
  586b073cd97908da["Validate Markdown Structure"];
  class 586b073cd97908da requirement;
  click 586b073cd97908da "../UserRequirements.md#validate-markdown-structure";
  9d7ad0f9a306af77["Markdown Structure Validator"];
  class 9d7ad0f9a306af77 requirement;
  click 9d7ad0f9a306af77 "../SystemRequirements/Requirements.md#markdown-structure-validator";
  586b073cd97908da -.->|deriveReqT| 9d7ad0f9a306af77;
  586b073cd97908da -.->|verifiedBy| e37dc7f46d75d46;
  ad6f7a2d41d80a38["Model Structure and Summaries"];
  class ad6f7a2d41d80a38 requirement;
  click ad6f7a2d41d80a38 "../UserRequirements.md#model-structure-and-summaries";
  b882613af131f35f["Model Summary Report Generator"];
  class b882613af131f35f requirement;
  click b882613af131f35f "../SystemRequirements/Requirements.md#model-summary-report-generator";
  ad6f7a2d41d80a38 -.->|deriveReqT| b882613af131f35f;
  ad6f7a2d41d80a38 -.->|verifiedBy| 9a53e9c00918fd02;
  bed8d0948b3e5ccd["Requirements Processing"];
  class bed8d0948b3e5ccd requirement;
  click bed8d0948b3e5ccd "../SystemRequirements/Requirements.md#requirements-processing";
  d50a859650933e55["model.rs"];
  class d50a859650933e55 default;
  click d50a859650933e55 "../../core/src/model.rs";
  bed8d0948b3e5ccd -->|satisfiedBy| d50a859650933e55;
  f22d93285fcd7664["parser.rs"];
  class f22d93285fcd7664 default;
  click f22d93285fcd7664 "../../core/src/parser.rs";
  bed8d0948b3e5ccd -->|satisfiedBy| f22d93285fcd7664;
  a4090fe7e30eeae4["Element Content Extraction Test"];
  class a4090fe7e30eeae4 verification;
  click a4090fe7e30eeae4 "ChangeImpactTests.md#element-content-extraction-test";
  bed8d0948b3e5ccd -.->|verifiedBy| a4090fe7e30eeae4;
  66582f9b6bdde6c4["Structured Markdown Files Search and Detection"];
  class 66582f9b6bdde6c4 requirement;
  click 66582f9b6bdde6c4 "../SystemRequirements/Requirements.md#structured-markdown-files-search-and-detection";
  bed8d0948b3e5ccd -.->|deriveReqT| 66582f9b6bdde6c4;
  bed8d0948b3e5ccd -.->|verifiedBy| 6ca2ff1567644e78;
  ed31b6bed1cde2f8["Provide Validation Reports"];
  class ed31b6bed1cde2f8 requirement;
  click ed31b6bed1cde2f8 "../UserRequirements.md#provide-validation-reports";
  ed31b6bed1cde2f8 -.->|verifiedBy| e37dc7f46d75d46;
  d667d94124e3bab7["Validation Report Generator"];
  class d667d94124e3bab7 requirement;
  click d667d94124e3bab7 "../SystemRequirements/Requirements.md#validation-report-generator";
  ed31b6bed1cde2f8 -.->|deriveReqT| d667d94124e3bab7;
  d50a859650933e55["model.rs"];
  class d50a859650933e55 default;
  click d50a859650933e55 "../../core/src/model.rs";
  66582f9b6bdde6c4 -->|satisfiedBy| d50a859650933e55;
  66582f9b6bdde6c4 -.->|verifiedBy| 184bc01c18f5506f;
  dc7a9bb1bbebc57f["Relation Element Type Validator"];
  class dc7a9bb1bbebc57f requirement;
  click dc7a9bb1bbebc57f "../SystemRequirements/Requirements.md#relation-element-type-validator";
  d50a859650933e55["model.rs"];
  class d50a859650933e55 default;
  click d50a859650933e55 "../../core/src/model.rs";
  dc7a9bb1bbebc57f -->|satisfiedBy| d50a859650933e55;
  f22d93285fcd7664["parser.rs"];
  class f22d93285fcd7664 default;
  click f22d93285fcd7664 "../../core/src/parser.rs";
  dc7a9bb1bbebc57f -->|satisfiedBy| f22d93285fcd7664;
  dc7a9bb1bbebc57f -.->|verifiedBy| e37dc7f46d75d46;
  3bd9d29239564eeb["Validate Cross-Component Dependencies"];
  class 3bd9d29239564eeb requirement;
  click 3bd9d29239564eeb "../UserRequirements.md#validate-cross-component-dependencies";
  3bd9d29239564eeb -.->|verifiedBy| e37dc7f46d75d46;
  80aa3982504aea7b["Cross-Component Dependency Validator"];
  class 80aa3982504aea7b requirement;
  click 80aa3982504aea7b "../SystemRequirements/Requirements.md#cross-component-dependency-validator";
  3bd9d29239564eeb -.->|deriveReqT| 80aa3982504aea7b;
  4ecd49d71920c1fc["Detailed Error Handling and Logging"];
  class 4ecd49d71920c1fc requirement;
  click 4ecd49d71920c1fc "../SystemRequirements/Requirements.md#detailed-error-handling-and-logging";
  a581221890d15c0c["src/error.rs"];
  class a581221890d15c0c default;
  click a581221890d15c0c "../../core/src/error.rs";
  4ecd49d71920c1fc -->|satisfiedBy| a581221890d15c0c;
  4ecd49d71920c1fc -.->|verifiedBy| e37dc7f46d75d46;
  db64a3e25646a37f["Relation Type Validation"];
  class db64a3e25646a37f requirement;
  click db64a3e25646a37f "../SystemRequirements/Requirements.md#relation-type-validation";
  9450d4313f47ef36["src/relation.rs"];
  class 9450d4313f47ef36 default;
  click 9450d4313f47ef36 "../../core/src/relation.rs";
  db64a3e25646a37f -->|satisfiedBy| 9450d4313f47ef36;
  db64a3e25646a37f -.->|verifiedBy| e37dc7f46d75d46;
  db64a3e25646a37f -.->|verifiedBy| 6ca2ff1567644e78;
  929c6c204cb3fedb["Excluded File Relation Validation"];
  class 929c6c204cb3fedb requirement;
  click 929c6c204cb3fedb "../SystemRequirements/Requirements.md#excluded-file-relation-validation";
  f22d93285fcd7664["src/parser.rs"];
  class f22d93285fcd7664 default;
  click f22d93285fcd7664 "../../core/src/parser.rs";
  929c6c204cb3fedb -->|satisfiedBy| f22d93285fcd7664;
  929c6c204cb3fedb -.->|verifiedBy| 526fb26c223ad188;
  c50887ce89be280a["Validate Internal Consistency"];
  class c50887ce89be280a requirement;
  click c50887ce89be280a "../UserRequirements.md#validate-internal-consistency";
  c50887ce89be280a -.->|verifiedBy| e37dc7f46d75d46;
  bcf308e253d2c6e7["Internal Consistency Validator"];
  class bcf308e253d2c6e7 requirement;
  click bcf308e253d2c6e7 "../SystemRequirements/Requirements.md#internal-consistency-validator";
  c50887ce89be280a -.->|deriveReqT| bcf308e253d2c6e7;
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
  a565a00c1df16a79["tests/test-subdirectory-functionality/test.sh"];
  class a565a00c1df16a79 default;
  click a565a00c1df16a79 "../../tests/test-subdirectory-functionality/test.sh";
  3c0a05c29bf52a57 -->|satisfiedBy| a565a00c1df16a79;
  7bdf935ec6d8effe["Subdirectory Processing Option"];
  class 7bdf935ec6d8effe requirement;
  click 7bdf935ec6d8effe "../SystemRequirements/Requirements.md#subdirectory-processing-option";
  80defdd4cbc7ee18["cli.rs"];
  class 80defdd4cbc7ee18 default;
  click 80defdd4cbc7ee18 "../../cli/src/cli.rs";
  7bdf935ec6d8effe -->|satisfiedBy| 80defdd4cbc7ee18;
  7bdf935ec6d8effe -.->|verifiedBy| 3c0a05c29bf52a57;
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