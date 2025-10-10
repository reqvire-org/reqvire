# Validation Tests

This document verifies the requirements for Reqvire's validation functionality.

## Relation Validation Tests

### Invalid Relations Test

This verification test checks that Reqvire correctly identifies and reports invalid relations using the two-pass validation architecture, separating parsing errors (Pass 1) from relation validation errors (Pass 2). The test also verifies the validate command functionality.

#### Details
- Pass 1 test: Command exits with non-zero error code and outputs expected parsing/format validation errors
- Pass 2 test: Command exits with non-zero error code and outputs expected relation validation errors
- Error output contains specific error messages for each type of validation error in the appropriate pass
- Validate command test: Verify the validate command executes validation and reports issues correctly

##### Acceptance Criteria
**Validate Command Requirements:**
- System should provide a validate command that executes model validation
- Validate command should output "No validation issues found" when the model is valid
- Validate command should report all validation errors when the model has issues
- Validate command should support --json flag for JSON formatted output
- Validate command should not modify any files during validation
- Validate command should use the same validation logic as other commands that load the model

**Pass 1 Validation Errors (Element Collection and Local Validation):**
- System should detect and report duplicate elements in the same document
- System should detect and report invalid metadata subsection format
- System should detect and report invalid relation format syntax
- System should detect and report invalid relation types (typos, etc.)
- System should detect and report duplicate subsections within elements
- Pass 1 errors should prevent Pass 2 from executing

**Pass 2 Validation Errors (Graph Construction and Relation Validation):**
- System should detect and report relations to non-existent targets
- System should detect and report requirement elements with satisfiedBy relations pointing to non-existing local files
- System should detect and report verification elements with satisfiedBy relations pointing to non-existing local files
- System should detect and report requirement elements with verifiedBy relations pointing to non-existing verification elements
- System should detect and report requirement elements with satisfiedBy relations pointing to other requirement elements (incompatible types)
- System should detect and report verification elements with satisfiedBy relations pointing to other verification elements (incompatible types)
- System should detect and report non-test-verification elements with satisfiedBy relations (only test-verification may use satisfiedBy, trace is always allowed)
- System should detect and report if system requirement is missing parent relation
- System should detect and report if there is circular dependency in requirements
- Pass 2 validation should only execute when Pass 1 completes without errors

**General Requirements:**
- System should allow requirement elements with satisfiedBy relations pointing to existing implementation files
- System should allow test-verification elements with satisfiedBy relations pointing to existing test scripts
- System should report clear error messages with details about the invalid format
- Two separate test scenarios should validate Pass 1 and Pass 2 errors independently

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Validate Command](../SystemRequirements/Requirements.md#validate-command)
  * verify: [Two-Pass Validation Strategy](../SystemRequirements/TwoPassValidation.md#two-pass-validation-strategy)
  * verify: [Validation Error Handling](../SystemRequirements/TwoPassValidation.md#validation-error-handling)
  * verify: [Detailed Error Handling and Logging](../SystemRequirements/Requirements.md#detailed-error-handling-and-logging)
  * verify: [Relation Type Validation](../SystemRequirements/Requirements.md#relation-type-validation)
  * verify: [Relation Element Type Validator](../SystemRequirements/Requirements.md#relation-element-type-validator)
  * verify: [Identifiers and Relations](../SpecificationsRequirements.md#identifiers-and-relations)
  * verify: [Trace Relation Non-Directional Behavior](../SystemRequirements/Requirements.md#trace-relation-non-directional-behavior)
  * verify: [Validate Markdown Structure](../UserRequirements.md#validate-markdown-structure)
  * verify: [Validate Internal Consistency](../UserRequirements.md#validate-internal-consistency)
  * verify: [Validate Cross-Component Dependencies](../UserRequirements.md#validate-cross-component-dependencies)
  * verify: [Provide Validation Reports](../UserRequirements.md#provide-validation-reports)
  * satisfiedBy: [test.sh](../../tests/test-invalid-relations/test.sh)
---

### Same-File Fragment Relations Test

This test verifies that Reqvire correctly handles and validates relations to fragments within the same file.

#### Details

##### Acceptance Criteria
- System should correctly validate relations to fragments within the same file
- System should not report errors for valid fragment references
- System should handle both fragment-only references like "#fragment-id" and proper element IDs

##### Test Criteria
- Command exits with success (zero) return code
- No error output about missing relation targets when using #fragment references
- Successful validation message is displayed

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Relation Type Validation](../SystemRequirements/Requirements.md#relation-type-validation)
  * verify: [Requirements Processing](../SystemRequirements/Requirements.md#requirements-processing)
  * satisfiedBy: [test.sh](../../tests/test-fragment-relations/test.sh)
---

### JSON Output Format Test

This test verifies that the system properly implements JSON output formatting for various commands that require it.

#### Details

##### Acceptance Criteria
- System shall support --json flag for various commands
- JSON output shall be properly formatted according to a consistent schema
- JSON structure shall be parsable and machine-readable

##### Test Criteria
- Commands with --json flag produce valid JSON
- JSON output can be parsed without errors
- Output structure matches expected schema

#### Metadata
  * type: test-verification
---

### Requirements Files Search and Detection Test

This test verifies that the system correctly searches for and detects structured document files according to specified patterns and configurations.

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

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Structured Markdown Files Search and Detection](../SystemRequirements/Requirements.md#structured-markdown-files-search-and-detection)
  * satisfiedBy: [test.sh](../../tests/test-excluded-patterns/test.sh)
---

### Unstructured Documents Test

This test verifies that the system correctly validates relations to excluded files.

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

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Excluded File Relation Validation](../SystemRequirements/Requirements.md#excluded-file-relation-validation)
  * satisfiedBy: [test.sh](../../tests/test-valid-relations/test.sh)
---

### Invalid Header Structure Test

This test verifies that Reqvire correctly detects and reports invalid header structures in elements, specifically level 5+ headers appearing outside of Details subsections.

#### Details

##### Acceptance Criteria
- System SHALL detect level 5+ headers (`#####`) appearing before reserved subsections (`#### Metadata`, `#### Relations`, `#### Details`)
- System SHALL allow level 5+ headers only within `#### Details` subsection
- System SHALL provide clear error messages indicating the invalid header structure with file and line number
- Error message SHALL specify that level 5+ headers can only appear inside Details subsection
- Validation SHALL fail when invalid header structure is detected

##### Test Criteria
- Command exits with non-zero error code when invalid header structure is found
- Error output contains specific error message about invalid header level
- Error message includes element name, file path, and line number
- Valid elements with level 5+ headers inside Details subsection pass validation
- Elements with level 5+ headers before reserved subsections fail validation

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Markdown Structure Validator](../SystemRequirements/Requirements.md#markdown-structure-validator)
  * verify: [Validate Markdown Structure](../UserRequirements.md#validate-markdown-structure)
  * verify: [Structure of Markdown Documents](../SpecificationsRequirements.md#structure-of-markdown-documents)
  * satisfiedBy: [test.sh](../../tests/test-invalid-relations/test.sh)
---

## Validation Report Tests

### Subdirectory Processing Verification

This test verifies that the system correctly processes only files within the current directory when run from a subfolder of a git repository and generates missing relation target errors for references to parent directories.

#### Details

##### Acceptance Criteria
- System shall process only files within the current directory when run from a subfolder
- System shall handle identifier normalization correctly within subdirectory context
- System shall generate missing relation target errors for references to elements or files outside the current subdirectory scope
- System shall work with model-summary, html, lint, and traces commands (validation is automatic)
- System shall ignore files outside the current directory scope
- System shall provide meaningful missing relation target error messages for parent directory references

##### Test Criteria
- Commands run from subdirectory process only files within that subdirectory
- Files outside the current directory are not included in processing or output
- Identifier normalization works correctly for paths within subdirectory
- References to parent directories generate missing relation target errors with clear error messages
- Missing relation target errors specifically identify the unreachable parent directory reference
- All major commands (model-summary, html, lint, traces) work from subdirectories with automatic validation
- Commands exit with validation error code when parent directory references cannot be resolved
- Error messages clearly explain the missing relation target due to parent directory reference

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Subdirectory Processing Option](../SystemRequirements/Requirements.md#subdirectory-processing-option)
  * satisfiedBy: [test.sh](../../tests/test-subdirectory-functionality/test.sh)
---

### Refine Relation Chain Validation Test

This test verifies that Reqvire correctly validates refinement relation chains following the one-direction rule, ensuring that elements in a refinement chain do not mix refinement relations with other hierarchical relations.

#### Details

##### Acceptance Criteria
**One-Direction Rule Enforcement:**
- System shall start validation with all elements that have "refinedBy" relations
- System shall follow parent→child direction using "refinedBy" relations
- System shall recursively validate all descendants in the refinement chain
- System shall track visited elements to avoid redundant checking

**Valid Refinement Chains:**
- Pure refinement chains where all descendants use "refinedBy" shall pass validation
- Child elements may have "refine" to their direct parent (automatic opposite)
- Child elements may have non-hierarchical relations (verifiedBy, verify, trace, satisfiedBy, satisfy)
- Multiple "refinedBy" relations from the same parent are allowed

**Invalid Refinement Chains:**
- System shall detect and reject chains where a child has "derive" or "derivedFrom"
- System shall detect and reject chains where a child has "contain" or "containedBy"
- System shall report violations using the MixedHierarchicalRelations error type
- System shall identify which specific elements violate the constraint

**Error Reporting:**
- Error messages shall identify the specific elements that violate the constraint
- Error messages shall explain that refinement chains cannot mix with other hierarchical relations
- Validation shall fail (non-zero exit code) when violations are detected

##### Test Criteria
**Pass Scenarios:**
- Pure refine chain: `Req1 --refinedBy--> Req2 --refinedBy--> Req3` validates successfully
- With non-hierarchical relations: `Req2 --verifiedBy--> Test1` validates successfully
- Multiple children: `Req1 --refinedBy--> Req2` and `Req1 --refinedBy--> Req3` validates successfully

**Fail Scenarios:**
- Mixed with derive: `Req1 --refinedBy--> Req2 --derive--> Req3` fails with MixedHierarchicalRelations error
- Mixed with contain: `Req1 --refinedBy--> Req2 --contain--> Req3` fails with MixedHierarchicalRelations error
- Mixed with derivedFrom: `Req1 --refinedBy--> Req2` where Req2 has `derivedFrom` to Req3 fails
- Mixed with containedBy: `Req1 --refinedBy--> Req2` where Req2 has `containedBy` to Req3 fails
- Command exits with non-zero error code for all fail scenarios
- Error output contains "Mixed hierarchical relations in chain" message
- Error message identifies the specific elements violating the constraint

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Refine Relation Chain Validator](../SystemRequirements/Requirements.md#refine-relation-chain-validator)
  * verify: [Relation Types and behaviors](../SpecificationsRequirements.md#relation-types-and-behaviors)
  * satisfiedBy: [test.sh](../../tests/test-refine-chain-validation/test.sh)
---
