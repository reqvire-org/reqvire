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
  * verify: [Validate Command](../ReqvireTool/UserInterface/CLI.md#validate-command)
  * verify: [Validation Error Handling](../ReqvireTool/ValidationAndReporting/Validation.md#validation-error-handling)
  * verify: [Relation Type Validation](../ReqvireTool/ValidationAndReporting/Validation.md#relation-type-validation)
  * verify: [Relation Element Type Validator](../ReqvireTool/ValidationAndReporting/Validation.md#relation-element-type-validator)
  * verify: [Identifiers and Relations](../SpecificationsRequirements.md#identifiers-and-relations)
  * verify: [Trace Relation Non-Directional Behavior](../ReqvireTool/ModelManagement/DiagramGeneration.md#trace-relation-non-directional-behavior)
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
  * verify: [Relation Type Validation](../ReqvireTool/ValidationAndReporting/Validation.md#relation-type-validation)
  * verify: [Requirements Processing](../ReqvireTool/Storage/Configuration.md#requirements-processing)
  * satisfiedBy: [test.sh](../../tests/test-fragment-relations/test.sh)
---

### Requirements Files Search and Detection Test

This test verifies that the system correctly searches for and detects structured document files according to specified patterns.

#### Details

##### Acceptance Criteria
- System shall find all structured document files in project structure
- System shall respect excluded file patterns defined in .gitignore and .reqvireignore files
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
  * verify: [Structured Markdown Files Search and Detection](../ReqvireTool/Storage/Configuration.md#structured-markdown-files-search-and-detection)
  * satisfiedBy: [test.sh](../../tests/test-excluded-patterns/test.sh)
---

### File Exclusion Test

This test verifies that Reqvire correctly reads and applies exclusion patterns from the repository root .gitignore file, .reqvireignore file, and reserved filenames.

#### Details

##### Acceptance Criteria
- System shall read exclusion patterns from root .gitignore file
- System shall read exclusion patterns from root .reqvireignore file
- System shall automatically exclude reserved repository documentation files
- System shall combine patterns from .gitignore, .reqvireignore, and reserved filenames
- Files matching patterns from any source shall be excluded from processing
- System shall use ONLY root .gitignore file, not nested .gitignore files
- System shall use ONLY root .reqvireignore file, not nested .reqvireignore files
- System shall correctly process files when .gitignore is absent
- System shall correctly process files when .reqvireignore is absent
- Exclusion shall work across all commands (validate, summary, format, traces, etc.)

##### Test Criteria
1. **Gitignore pattern exclusion:**
   - Create test environment with root .gitignore containing patterns (e.g., "**/build/**", "temp-*.md")
   - Create files matching those patterns in specifications folder
   - Run reqvire summary command
   - Verify files matching .gitignore patterns are NOT processed
   - Verify files NOT matching patterns ARE processed

2. **Reqvireignore pattern exclusion:**
   - Create test environment with root .reqvireignore containing patterns (e.g., "**/draft-*.md", "examples/**")
   - Create files matching those patterns in specifications folder (files that ARE in Git)
   - Run reqvire summary command
   - Verify files matching .reqvireignore patterns are NOT processed
   - Verify files NOT matching patterns ARE processed

3. **Reserved filenames exclusion:**
   - Create reserved documentation files (e.g., README.md, LICENSE.md, CONTRIBUTING.md) with structured markdown content
   - Run reqvire summary command
   - Verify reserved files are NOT processed as structured markdown
   - Verify reserved files can still be referenced in relations
   - Verify files NOT matching reserved patterns ARE processed

4. **Combined exclusion patterns:**
   - Add patterns to .gitignore (e.g., "**/build/**")
   - Add different patterns to .reqvireignore (e.g., "**/DRAFT*.md")
   - Create reserved files (e.g., README.md)
   - Create files matching all pattern sets
   - Verify files matching patterns from ANY source are excluded
   - Verify only non-matching files are processed

5. **Missing .gitignore handling:**
   - Run reqvire in environment without .gitignore file but with .reqvireignore
   - Verify command succeeds without errors
   - Verify only .reqvireignore exclusions and reserved filenames are applied

6. **Missing .reqvireignore handling:**
   - Run reqvire in environment without .reqvireignore file but with .gitignore
   - Verify command succeeds without errors
   - Verify only .gitignore exclusions and reserved filenames are applied

7. **Both files missing handling:**
   - Run reqvire in environment without .gitignore or .reqvireignore files
   - Verify command succeeds without errors
   - Verify only reserved filename exclusions are applied

8. **Nested files ignored:**
   - Create nested .gitignore in subdirectory with different patterns
   - Create nested .reqvireignore in subdirectory with different patterns
   - Verify patterns from nested files are NOT applied
   - Verify only root file patterns are used

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Ignore Files Integration](../ReqvireTool/Storage/Configuration.md#ignore-files-integration)
  * verify: [Reserved Repository Files Exclusion](../ReqvireTool/Storage/Configuration.md#reserved-repository-files-exclusion)
  * satisfiedBy: [test.sh](../../tests/test-gitignore-integration/test.sh)
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
  * verify: [Excluded File Relation Validation](../ReqvireTool/ValidationAndReporting/Validation.md#excluded-file-relation-validation)
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
  * verify: [Markdown Structure Validator](../ReqvireTool/ValidationAndReporting/Validation.md#markdown-structure-validator)
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
- System shall work with model-summary, html, format, and traces commands (validation is automatic)
- System shall ignore files outside the current directory scope
- System shall provide meaningful missing relation target error messages for parent directory references

##### Test Criteria
- Commands run from subdirectory process only files within that subdirectory
- Files outside the current directory are not included in processing or output
- Identifier normalization works correctly for paths within subdirectory
- References to parent directories generate missing relation target errors with clear error messages
- Missing relation target errors specifically identify the unreachable parent directory reference
- All major commands (model-summary, html, format, traces) work from subdirectories with automatic validation
- Commands exit with validation error code when parent directory references cannot be resolved
- Error messages clearly explain the missing relation target due to parent directory reference

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Subdirectory Processing Option](../ReqvireTool/UserInterface/CLI.md#subdirectory-processing-option)
  * satisfiedBy: [test.sh](../../tests/test-subdirectory-functionality/test.sh)
---

## Element Type Assignment Tests

### Default Element Type Assignment Test

This test verifies that the system assigns the default type 'requirement' to all elements without explicit type metadata, regardless of their file location within the repository.

#### Details

##### Acceptance Criteria
**Location-Independent Default Type:**
- System shall assign type 'requirement' to elements without explicit type metadata
- Default type assignment shall be location-independent (same behavior for all directories)
- System shall NOT use file location to determine element type

**Explicit Type Metadata Overrides:**
- System shall allow explicit type specification via Metadata subsection
- System shall respect explicit type metadata when present
- System shall support all standard element types: requirement, user-requirement, verification, test-verification, analysis-verification, inspection-verification, demonstration-verification, other

##### Test Criteria
1. **Default type assignment verification:**
   - Create test elements in various directories without type metadata
   - Run reqvire summary --json to extract element types
   - Verify all elements without type metadata have type 'requirement'
   - Test elements in root specifications folder
   - Test elements in nested subdirectories
   - Test elements in various file locations

2. **Explicit type metadata verification:**
   - Create elements with explicit type metadata (user-requirement, verification, etc.)
   - Run reqvire summary --json
   - Verify elements have the explicitly specified types
   - Verify explicit types override default behavior

3. **Location independence verification:**
   - Create identical elements in different directories
   - None have explicit type metadata
   - Run reqvire summary --json
   - Verify all elements have type 'requirement' regardless of location

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Default Requirement Type Assignment](../UserRequirements.md#default-requirement-type-assignment)
  * satisfiedBy: [test.sh](../../tests/test-default-type-assignment/test.sh)
---
