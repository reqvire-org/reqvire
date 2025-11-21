# Validation

## Requirements

### Two-Pass Validation Strategy

The system shall implement a two-pass validation strategy that separates element collection from relation validation, enabling complete error reporting while maintaining existing error behavior.

#### Details
The validation process shall be split into two distinct passes:

**Pass 1: Element Collection and Local Validation**
- Parse all markdown files
- Extract elements with metadata
- Apply automatic semantic normalization during parsing:
  - Convert non-link identifiers to proper markdown links with display text
  - Normalize absolute paths to relative paths for portable references
- Perform local validation (element uniqueness, identifier format, metadata syntax)
- Store elements in ElementRegistry
- Defer relation validation to Pass 2
- If errors are found, report them and exit the process

**Pass 2: Graph Construction and Relation Validation**
- Build GraphRegistry from ElementRegistry
- Validate all relations (target existence, type compatibility)
- Generate missing opposite relations
- Perform cross-component validation
- If errors are found, report them and exit the process

Both passes maintain the existing behavior where validation errors cause process termination with appropriate error reporting.

#### Relations
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * derivedFrom: [Requirements Processing](Configuration.md#requirements-processing)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
---

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
- System should detect and report duplicate element names across different files (global uniqueness violation)
- System should report both file locations where duplicate element names occur
- Error message should clearly indicate element names must be globally unique
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
  * verify: [Validate Command](../Interfaces/CLI.md#validate-command)
  * verify: [Validation Error Handling](#validation-error-handling)
  * verify: [Internal Consistency Validator](#internal-consistency-validator)
  * verify: [Relation Type Validation](#relation-type-validation)
  * verify: [Relation Element Type Validator](#relation-element-type-validator)
  * verify: [Identifiers and Relations](ModelManagement.md#identifiers-and-relations)
  * verify: [Trace Relation Non-Directional Behavior](DiagramGeneration.md#trace-relation-non-directional-behavior)
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
  * verify: [Relation Type Validation](#relation-type-validation)
  * verify: [Requirements Processing](Configuration.md#requirements-processing)
  * satisfiedBy: [test.sh](../../tests/test-fragment-relations/test.sh)
---

### Markdown Structure Validator

The system shall implement a markdown structure validator that enforces Reqvire's requirements for header levels, element structure, relation formatting, and other markdown-specific syntax rules, reporting violations with line numbers and suggested fixes.

#### Relations
  * derivedFrom: [Validate Markdown Structure](#validate-markdown-structure)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)
  * verifiedBy: [Invalid Relations Test](#invalid-relations-test)
---

### Enhanced Validation Error Reporting

The system shall provide comprehensive validation messages that include file paths and line numbers when available, to help users quickly locate and fix model integrity and structure issues in their MBSE specifications.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](../UserStories.md#validating-structures)
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
  * verify: [Structured Markdown Files Search and Detection](Configuration.md#structured-markdown-files-search-and-detection)
  * satisfiedBy: [test.sh](../../tests/test-excluded-patterns/test.sh)
---

### Integrated Validation

The system shall automatically perform validation when any command requires the parsed model, eliminating the need for a separate validate command.

#### Details
Commands shall be categorized into two groups:

**Commands requiring validated model:**
- model-summary: Needs complete element and relation data
- change-impact: Requires valid relations for impact analysis
- traces: Needs validated relationships for traceability
- generate-index: Requires complete element registry
- generate-diagrams: Needs valid relations for visualization
- remove-diagrams: Operates on validated markdown structure
- coverage-report: Requires complete verification data

**Commands operating on raw files:**
- html: Converts markdown to HTML without parsing elements
- format: Fixes markdown formatting without validation
- shell: Interactive mode with optional validation

Commands in the first group shall automatically run the two-pass validation and exit if any errors are found. Commands in the second group shall skip validation to allow operation on potentially invalid documents.

#### Relations
  * derivedFrom: [Provide Validation Reports](Reporting.md#provide-validation-reports)
  * derivedFrom: [CLI Interface Structure](../Interfaces/CLI.md#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
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
  * verify: [Ignore Files Integration](Configuration.md#ignore-files-integration)
  * verify: [Reserved Repository Files Exclusion](Configuration.md#reserved-repository-files-exclusion)
  * satisfiedBy: [test.sh](../../tests/test-gitignore-integration/test.sh)
---

### Validate Markdown Structure

The system shall validate the Markdown structure of system model to ensure compliance with formatting standards.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](../UserStories.md#validating-structures)
  * derivedFrom: [Align with Industry Standards](../UserStories.md#align-with-industry-standards)
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
  * verify: [Excluded File Relation Validation](#excluded-file-relation-validation)
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
  * verify: [Markdown Structure Validator](#markdown-structure-validator)
  * verify: [Structure and Addressing in Markdown Documents](ModelManagement.md#structure-and-addressing-in-markdown-documents)
  * satisfiedBy: [test.sh](../../tests/test-invalid-relations/test.sh)
---

### Internal Consistency Validator

The system shall implement a consistency validator that verifies logical coherence within the model, including checking for circular dependencies, orphaned elements, inconsistent relationship patterns, and element name uniqueness, with detailed error reporting.

#### Details
The consistency validator shall verify:
- **Global Element Name Uniqueness**: Element names are globally unique across all files in the model
- **Duplicate Detection**: Detect and report when multiple elements in different files share the same name
- **Location Reporting**: Report both file locations where duplicate element names occur
- **Clear Error Messages**: Error messages clearly indicate that element names must be globally unique
- **Circular Dependencies**: Detect and report circular dependency chains in requirements
- **Orphaned Elements**: Identify elements without proper traceability connections
- **Inconsistent Patterns**: Detect relationship patterns that violate model constraints

Rationale: Element names serve as stable IDs for element identity, independent of file location. Global uniqueness is essential for proper element identification and change tracking across the model.

#### Relations
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * derivedFrom: [Element Identity Model](ModelManagement.md#element-identity-model)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)
  * verifiedBy: [Invalid Relations Test](#invalid-relations-test)
---

### Validate Filesystem Structure

The system shall validate the organization of files and folders in the repository to ensure consistency with the MBSE methodology.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](../UserStories.md#validating-structures)
---

### Subdirectory Processing Verification

This test verifies that the system correctly processes only files within the current directory when run from a subfolder of a git repository and generates missing relation target errors for references to parent directories.

#### Details

##### Acceptance Criteria
- System shall process only files within the current directory when run from a subfolder
- System shall handle identifier normalization correctly within subdirectory context
- System shall generate missing relation target errors for references to elements or files outside the current subdirectory scope
- System shall work with model, export, format, traces, and CRUD commands (validation is automatic)
- System shall ignore files outside the current directory scope
- System shall provide meaningful missing relation target error messages for parent directory references
- CRUD commands (add, rm, mv, mv-file) shall resolve paths relative to current working directory

##### Test Criteria
- Commands run from subdirectory process only files within that subdirectory
- Files outside the current directory are not included in processing or output
- Identifier normalization works correctly for paths within subdirectory
- References to parent directories generate missing relation target errors with clear error messages
- Missing relation target errors specifically identify the unreachable parent directory reference
- All major commands (model, export, format, traces) work from subdirectories with automatic validation
- CRUD commands (add, rm, mv, mv-file) resolve file paths relative to current working directory
- CRUD mv command successfully moves elements within subdirectory scope
- CRUD mv-file command successfully moves entire files within subdirectory scope
- Commands exit with validation error code when parent directory references cannot be resolved
- Error messages clearly explain the missing relation target due to parent directory reference

#### Metadata
  * type: test-verification

#### Relations
  * verify: [CLI Move Element Command](../Interfaces/CLI.md#cli-move-element-command)
  * verify: [CLI Move File Command](../Interfaces/CLI.md#cli-move-file-command)
  * satisfiedBy: [test.sh](../../tests/test-subdirectory-functionality/test.sh)
---

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
  * verify: [Default Requirement Type Assignment](ModelManagement.md#default-requirement-type-assignment)
  * satisfiedBy: [test.sh](../../tests/test-default-type-assignment/test.sh)
---

### Cross-Component Dependency Validator

The system shall implement a specialized validator that analyzes dependencies across different model components, ensuring proper alignment between architectural layers, requirement levels, and verification elements.

#### Relations
  * derivedFrom: [Validate Cross-Component Dependencies](#validate-cross-component-dependencies)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)
  * verifiedBy: [Invalid Relations Test](#invalid-relations-test)
---

### Validation Error Handling

The system shall maintain consistent error handling across both validation passes, collecting all errors within each pass before reporting.

#### Details
Error handling shall follow these principles:

1. **Complete pass execution**: Each pass runs to completion, collecting all errors found
2. **Aggregated reporting**: All errors from a pass are reported together
3. **Early termination**: Process exits after reporting errors from either pass
4. **Existing error format**: Error messages maintain the current format and structure
5. **Exit codes**: Non-zero exit codes indicate validation failures

This ensures users see all relevant errors at once rather than fixing issues one at a time.

#### Relations
  * derivedFrom: [Detailed Error Handling and Logging](../Interfaces/CLI.md#detailed-error-handling-and-logging)
  * derivedFrom: [Validation Report Generator](Reporting.md#validation-report-generator)
  * satisfiedBy: [error.rs](../../core/src/error.rs)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
---

### Relation Element Type Validator

The system shall implement validation that verifies relation endpoints have appropriate element types based on the relation type.

#### Details
- For `verifiedBy`/`verify` relations, validate that one endpoint is a requirement element and the other is a verification element
- For `satisfiedBy`/`satisfy` relations, validate that one endpoint is a requirement or test-verification element and the other is an implementation element
- For verification elements with `satisfiedBy` relations, validate that only test-verification elements may use satisfiedBy (other verification types should not have satisfiedBy relations)
- `trace` relations are always allowed for any verification type
- Relations should only connect elements of appropriate types based on the RelationTypesRegistry definition
- Warnings should be issued when relation endpoints have incompatible element types

#### Relations
  * derivedFrom: [Validate Relation Types](#validate-relation-types)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)
  * verifiedBy: [Invalid Relations Test](#invalid-relations-test)
---

### GraphRegistry as Primary Registry

The system shall enhance GraphRegistry to serve as the primary structure for relation operations and validation during Pass 2.

#### Details
The GraphRegistry shall be responsible for:

1. **Graph construction**: Building adjacency lists from ElementRegistry
2. **Relation validation**: Checking target existence and type compatibility
3. **Opposite generation**: Creating missing bidirectional relations
4. **Cycle detection**: Identifying circular dependencies
5. **Orphan detection**: Finding isolated elements
6. **Impact analysis**: Supporting change propagation queries

The GraphRegistry shall be constructed from the ElementRegistry after Pass 1 completes successfully.

#### Relations
  * derivedFrom: [Requirements Processing](Configuration.md#requirements-processing)
  * satisfiedBy: [graph_registry.rs](../../core/src/graph_registry.rs)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
  * verifiedBy: [Requirements Files Search and Detection Test](#requirements-files-search-and-detection-test)
---

### Validate Internal Consistency

The system shall check the internal consistency of the system model, ensuring that relationships and elements align correctly.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](../UserStories.md#validating-structures)
  * derivedFrom: [Align with Industry Standards](../UserStories.md#align-with-industry-standards)
---

### Validate Cross-Component Dependencies

The system shall validate dependencies across different components of the MBSE model to identify mismatches or gaps.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](../UserStories.md#validating-structures)
  * derivedFrom: [Align with Industry Standards](../UserStories.md#align-with-industry-standards)
---

### Relation Type Validation

The system shall validate relation types against a defined vocabulary and provide clear error messages for unsupported relation types, including suggestions for the correct relation types.

#### Relations
  * derivedFrom: [Enhanced Validation Error Reporting](#enhanced-validation-error-reporting)
  * satisfiedBy: [relation.rs](../../core/src/relation.rs)
---

### Excluded File Relation Validation

The system shall properly validate relations targeting files matching excluded filename patterns, enabling references to excluded files while still respecting their exclusion from processing and formatting operations.

#### Details
The validation process for excluded files:
1. Files matching excluded patterns are registered in the element registry for relation validation only
2. Internal elements within excluded files are not processed or validated

#### Relations
  * derivedFrom: [File Pattern Exclusion for Format](Formatting.md#file-pattern-exclusion-for-format)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)
---

### Validate Relation Types

The system shall validate relation types and allow only supported types.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](../UserStories.md#validating-structures)
  * derivedFrom: [Align with Industry Standards](../UserStories.md#align-with-industry-standards)
  * derivedFrom: [Relation Types and behaviors](ModelManagement.md#relation-types-and-behaviors)
---
