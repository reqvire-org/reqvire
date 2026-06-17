# Elements

### Capability Element Relation Compatibility Test

This test verifies capability hierarchy, requirement-to-capability specification, and rejection of unsupported stakeholder requirement typing.

#### Details
Test cases:
1. Valid top-level requirement with `specify` to a capability validates successfully.
2. Valid capability root with `specifiedBy` to a top-level requirement validates successfully.
3. Valid child requirement with `derivedFrom` to another requirement inherits the same owning capability.
4. Capability `derivedFrom` capability validates successfully.
5. Requirement `derivedFrom` capability fails.
6. Capability `specifiedBy` capability fails.
7. Capability `satisfiedBy` fails, while capability `verifiedBy` is valid.
8. Unsupported stakeholder requirement type metadata fails as an invalid type.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-capability-model/test.sh)
  * verify: [Capability Model Structure](../../../ModelStructure/ModelManagement.md#capability-model-structure)
  * verify: [Element Type Relation Compatibility](../../../ModelStructure/ModelManagement.md#element-type-relation-compatibility)
---

### Cross-Section Duplicate Validation Test

Test verifies that validation detects and reports cross-section duplicates.

#### Details
Test cases:
1. Element with same target in both Relations (as satisfiedBy) and Attachments
2. Run `reqvire validate`
3. Verify validation fails with error mentioning "cross-section duplicate" or similar
4. Verify error identifies the element and the duplicate target

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-duplicate-detection/test.sh)
  * verify: [Cross-Section Duplicate Validation](../../../Operations/Validation/ValidationRequirements.md#cross-section-duplicate-validation)
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
- System shall support all standard element types: capability, requirement, ontology, semantic-contract, verification, test-verification, analysis-verification, inspection-verification, demonstration-verification, formal-proof-verification, source, state, input-output, constraint, behavior, specification, other

##### Test Criteria
1. **Default type assignment verification:**
   - Create test elements in various directories without type metadata
   - Run reqvire summary --json to extract element types
   - Verify all elements without type metadata have type 'requirement'
   - Test elements in root specifications folder
   - Test elements in nested subdirectories
   - Test elements in various file locations

2. **Explicit type metadata verification:**
   - Create elements with explicit type metadata (capability, verification, semantic-contract, etc.)
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
  * satisfiedBy: [test.sh](../../../../tests/test-default-type-assignment/test.sh)
  * verify: [Default Requirement Type Assignment](../../../ModelStructure/ModelManagement.md#default-requirement-type-assignment)
---

### Element Type Relation Compatibility Test

This test verifies that the system correctly validates relation types based on element type constraints defined in the Element Type Relation Compatibility matrix.

#### Details

##### Acceptance Criteria
**derivedFrom/derive Validation:**
- System shall allow `derivedFrom` relations between elements in the same hierarchy family only (`capability` to `capability`, `requirement` to `requirement`, or `ontology` to `ontology`)
- System shall reject `derivedFrom` relations where source is a verification element
- System shall reject `derivedFrom` relations where target is a verification element
- System shall reject `derivedFrom` relations where source is `other` type
- System shall provide clear error message indicating element type incompatibility

**specifiedBy/specify Validation:**
- System shall allow `specifiedBy` from `capability` to `requirement`
- System shall allow `specify` from `requirement` to `capability`
- System shall reject cross-family hierarchy through `derive`/`derivedFrom`, including `capability` to `requirement`, `capability` to `ontology`, and `requirement` to `ontology`
- System shall reject `capability` to `capability` specification through `specifiedBy`

**satisfiedBy/satisfy Validation:**
- System shall allow `satisfiedBy` relations from `requirement` elements to implementation files
- System shall reject `satisfiedBy` relations from `capability` elements
- System shall allow `satisfiedBy` relations from `test-verification` to test implementation files
- System shall allow `satisfiedBy` relations from `formal-proof-verification` to proof evidence files
- System shall reject `satisfiedBy` relations from `analysis-verification`, `inspection-verification`, `demonstration-verification` elements
- System shall provide clear error message for invalid element types using satisfiedBy

**verifiedBy/verify Validation:**
- System shall allow `verifiedBy` relations from `capability` and `requirement` elements to any verification type
- System shall allow `verify` relations from any verification type to `capability` or `requirement` elements
- System shall reject `verifiedBy` relations from non-capability and non-requirement elements
- System shall reject `verifiedBy` relations from verification elements
- System shall reject `verify` relations to non-capability and non-requirement elements

**Refinement Type Validation:**
- System shall allow `refine` relations on `constraint` type elements pointing to requirements
- System shall allow `refine` relations on `behavior` type elements pointing to requirements
- System shall allow `refine` relations on `specification` type elements pointing to requirements
- System shall allow `refine` relations on `state` and `input-output` elements pointing to requirements
- System shall allow `refine` relations on `source` elements pointing to capabilities
- System shall allow `constrain` relations on `semantic-contract` elements pointing to requirements
- System shall reject all other relation types on refinement elements (derivedFrom, verifiedBy, trace, satisfiedBy)
- System shall provide clear error message indicating non-semantic-contract refinement types can only have refine relations

**trace Relation Validation:**
- System shall allow `trace` relations for any non-refinement element type
- System shall allow `trace` relations to any target type

##### Test Criteria
1. **derivedFrom type constraint tests:**
   - Create requirement with `derivedFrom` to another requirement - PASS
   - Create verification with `derivedFrom` to requirement - FAIL with type error
   - Create requirement with `derivedFrom` to verification - FAIL with type error
   - Verify error message includes element types and constraint explanation

2. **satisfiedBy type constraint tests:**
   - Create requirement with `satisfiedBy` to implementation file - PASS
   - Create capability with `satisfiedBy` to implementation file - FAIL with type error
   - Create test-verification with `satisfiedBy` to test file - PASS
   - Create formal-proof-verification with `satisfiedBy` to proof report file - PASS
   - Create analysis-verification with `satisfiedBy` to file - FAIL with type error
   - Verify error message indicates `capability` is not allowed for satisfiedBy

3. **Refinement type relation tests:**
   - Create constraint element with `refine` relation to requirement - PASS
   - Create behavior element with `refine` relation to requirement - PASS
   - Create specification element with `refine` relation to requirement - PASS
   - Create constraint element with `trace` relation - FAIL with error
   - Create behavior element with `derivedFrom` relation - FAIL with error
   - Create specification element with `satisfiedBy` relation - FAIL with error
   - Verify error messages indicate refinement types can only have refine relations
   - Create constraint element with Attachments subsection - FAIL with error
   - Verify error message indicates refinement types cannot have attachments

4. **trace relation permissiveness tests:**
   - Create requirement with `trace` to verification - PASS
   - Create verification with `trace` to requirement - PASS
   - Create verification with `trace` to other verification - PASS
   - Verify trace relations do not trigger type compatibility errors

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-element-type-relation-compatibility/test.sh)
  * verify: [Element Type Relation Compatibility](../../../ModelStructure/ModelManagement.md#element-type-relation-compatibility)
  * verify: [Relation Element Type Validator](../../../Operations/Validation/ValidationRequirements.md#relation-element-type-validator)
---

### File Exclusion Test

This test verifies that Reqvire correctly reads and applies exclusion patterns from the repository root .gitignore file and .reqvireignore file.

#### Details

##### Acceptance Criteria
- System shall read exclusion patterns from root .gitignore file
- System shall read exclusion patterns from root .reqvireignore file
- System shall combine patterns from .gitignore and .reqvireignore
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

3. **Combined exclusion patterns:**
   - Add patterns to .gitignore (e.g., "**/build/**")
   - Add different patterns to .reqvireignore (e.g., "**/DRAFT*.md")
   - Create files matching all pattern sets
   - Verify files matching patterns from ANY source are excluded
   - Verify only non-matching files are processed

4. **Missing .gitignore handling:**
   - Run reqvire in environment without .gitignore file but with .reqvireignore
   - Verify command succeeds without errors
   - Verify only .reqvireignore exclusions are applied

5. **Missing .reqvireignore handling:**
   - Run reqvire in environment without .reqvireignore file but with .gitignore
   - Verify command succeeds without errors
   - Verify only .gitignore exclusions are applied

6. **Both files missing handling:**
   - Run reqvire in environment without .gitignore or .reqvireignore files
   - Verify command succeeds without errors
   - Verify no ignore-file exclusions are applied

7. **Nested files ignored:**
   - Create nested .gitignore in subdirectory with different patterns
   - Create nested .reqvireignore in subdirectory with different patterns
   - Verify patterns from nested files are NOT applied
   - Verify only root file patterns are used

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-gitignore-integration/test.sh)
  * verify: [Ignore Files Integration](../../../ModelStructure/Configuration.md#ignore-files-integration)
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
  * satisfiedBy: [test.sh](../../../../tests/test-invalid-relations/test.sh)
  * verify: [Structure and Addressing in Markdown Documents](../../../ModelStructure/StructureAndParsing.md#structure-and-addressing-in-markdown-documents)
  * verify: [Markdown Structure Validator](../../../Operations/Validation/ValidationRequirements.md#markdown-structure-validator)
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
- System should detect and report non-evidence-backed verification elements with satisfiedBy relations (only test-verification and formal-proof-verification may use satisfiedBy, trace is always allowed)
- System should detect and report if a requirement is missing required capability or parent requirement ownership
- System should detect and report if there is circular dependency in requirements
- Pass 2 validation should only execute when Pass 1 completes without errors

**General Requirements:**
- System should allow requirement elements with satisfiedBy relations pointing to existing implementation files
- System should allow test-verification elements with satisfiedBy relations pointing to existing test scripts
- System should allow formal-proof-verification elements with satisfiedBy relations pointing to existing proof evidence artifacts
- System should report clear error messages with details about the invalid format
- Two separate test scenarios should validate Pass 1 and Pass 2 errors independently

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-invalid-relations/test.sh)
  * verify: [Validate Command](../../../Interfaces/CLI/Commands.md#validate-command)
  * verify: [Cross-Component Dependency Validator](../../../Operations/Validation/ValidationRequirements.md#cross-component-dependency-validator)
  * verify: [Internal Consistency Validator](../../../Operations/Validation/ValidationRequirements.md#internal-consistency-validator)
  * verify: [Markdown Structure Validator](../../../Operations/Validation/ValidationRequirements.md#markdown-structure-validator)
  * verify: [Relation Element Type Validator](../../../Operations/Validation/ValidationRequirements.md#relation-element-type-validator)
  * verify: [Relation Type Validation](../../../Operations/Validation/ValidationRequirements.md#relation-type-validation)
  * verify: [Validation Error Handling](../../../Operations/Validation/ValidationRequirements.md#validation-error-handling)
  * verify: [Trace Relation Non-Directional Behavior](../../../Reports/ModelReports/DiagramGeneration.md#trace-relation-non-directional-behavior)
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
  * satisfiedBy: [test.sh](../../../../tests/test-excluded-patterns/test.sh)
  * verify: [Structured Markdown Files Search and Detection](../../../ModelStructure/Configuration.md#structured-markdown-files-search-and-detection)
  * verify: [GraphRegistry as Primary Registry](../../../Operations/Validation/ValidationRequirements.md#graphregistry-as-primary-registry)
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
  * satisfiedBy: [test.sh](../../../../tests/test-fragment-relations/test.sh)
  * verify: [Requirements Processing](../../../ModelStructure/Configuration.md#requirements-processing)
  * verify: [Relation Type Validation](../../../Operations/Validation/ValidationRequirements.md#relation-type-validation)
---

### Semantic Contract Ontology Declaration Validation Test

This test verifies global ontology term declaration validation across ontology elements.

#### Details
Test cases:
1. Multiple ontology elements declaring the same ontology term IRI fail validation.
2. A semantic ontology term declared with conflicting roles fails validation.
3. Duplicate declaration validation applies to ontology term IRIs and does not validate derived ontology element IRIs.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-semantic-contract-sanity/test.sh)
  * verify: [Ontology and Semantic Contract Model](../../../ModelStructure/ModelManagement.md#ontology-and-semantic-contract-model)
---

### Semantic Contract Relation Validation Test

This test verifies that ontology and semantic-contract relation rules are enforced and that non-semantic refinement types are not mixed.

#### Details
Test cases:
1. `source` refining `requirement` validates successfully.
2. `ontology` validates as an independent ontology element when semantic sections are well formed.
3. `semantic-contract` constraining `requirement` validates as a shape contract when it contains Shapes, uses ontology, and has no Ontology section.
4. `semantic-contract` using `refinedBy`/`refine` fails validation.
5. `source` refining `capability` fails.
6. `constraint`, `behavior`, `specification`, `state`, or `input-output` refining `capability` fails.
7. Capability attachment to `ontology` validates.
8. Requirement `constrainedBy` to `semantic-contract` validates.
9. Requirement attachment to `ontology` fails because ontology use for semantic contracts is explicit through `use`.
10. Capability attachment to `semantic-contract` fails.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-capability-refinements/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-semantic-contract-sanity/test.sh)
  * verify: [Ontology and Semantic Contract Model](../../../ModelStructure/ModelManagement.md#ontology-and-semantic-contract-model)
---

### Semantic Contract SHACL Sanity Validation Test

This test verifies lightweight RDF-based SHACL sanity validation for semantic-contract Shapes.

#### Details
Test cases:
1. A semantic contract with declared ontology classes/properties and supported SHACL constraints validates successfully.
2. A `sh:NodeShape` whose `sh:targetClass` is not declared by any ontology element fails validation.
3. A referenced property shape without exactly one `sh:path` fails validation.
4. A referenced property shape whose `sh:path` is not declared by any ontology element fails validation.
5. A referenced property shape whose `sh:class` is not declared by any ontology element fails validation.
6. A SHACL reference to a term declared by ontology used by the semantic contract validates successfully.
7. A SHACL reference to a term declared by an ancestor of a used ontology validates successfully.
8. A SHACL reference from a semantic contract to a term declared outside the reachable use context fails validation.
9. An outside-context semantic reference validation error includes the referencing semantic-contract identifier, reference kind, referenced IRI, declaring ontology identifier, and guidance to add a `use` relation.
10. A property shape with `sh:maxCount` lower than `sh:minCount` fails validation.
11. A property shape with malformed `sh:in` RDF list structure fails validation.
12. A missing semantic declaration validation error includes the referencing semantic-contract identifier, reference kind, referenced IRI, and fix guidance.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-semantic-contract-sanity/test.sh)
  * verify: [Ontology and Semantic Contract Model](../../../ModelStructure/ModelManagement.md#ontology-and-semantic-contract-model)
---

### Semantic Contract Section Validation Test

This test verifies built-in semantic-contract section validation.

#### Details
Test cases:
1. `ontology` with exactly one `Ontology` fenced Turtle block validates successfully.
2. `ontology` with `Shapes` fails validation; `semantic-contract` with `Shapes` validates when it uses ontology through `use`/`usedBy`.
3. `ontology` missing `Ontology` fails validation.
4. `semantic-contract` with exactly one `Shapes` fenced Turtle block and no `Ontology` validates successfully when its required ontology-use context is present.
5. `semantic-contract` with `Ontology` fails validation.
6. `semantic-contract` missing `Shapes` fails validation.
7. Duplicate `Ontology` or duplicate `Shapes` sections fail validation.
8. Invalid Turtle in `Ontology` fails validation.
9. Invalid Turtle in `Shapes` fails validation.
10. A top parent ontology element without non-empty `ontology_base` metadata fails validation.
11. A top parent ontology element without non-empty `ontology_prefix` metadata fails validation.
12. An ontology Turtle block that declares the inherited `ontology_prefix` to a namespace other than `<ontology_base>#` fails validation.
13. An ontology Turtle block that uses the inherited `ontology_prefix` without explicitly declaring it fails validation.
14. `reqvire ontologies` emits one generated `owl:Ontology` document declaration at the resolved `ontology_base` for a connected ontology hierarchy and lists same-base child ontology elements as contributors instead of declaring a separate child ontology document.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-capability-refinements/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-ontology-single-root/test.sh)
  * verify: [Ontology and Semantic Contract Model](../../../ModelStructure/ModelManagement.md#ontology-and-semantic-contract-model)
---

### Single Element Refinement Validation Test

This test verifies that `# Element` files are parsed as single-element model files and that `refinedBy` targets must resolve to refinement elements (including those defined in `# Element` files).

#### Details

##### Acceptance Criteria
- System shall parse `# Element` files as model files with one element.
- The single element shall use metadata type from `## Metadata`.
- Content under `## <Actual Element Name>` shall allow arbitrary markdown headers, and the heading text shall define the element name.
- `refinedBy` targets shall be identifier links that resolve to refinement elements.
- `refinedBy` plain file-path targets shall be rejected.
- `refinedBy` identifier targets into `# Element` files shall satisfy existing relation type compatibility rules based on target element type.

##### Test Criteria
1. Create valid `# Element` refinement file with:
   - `## Metadata` type `specification`
   - `## Relations` containing `refine` relation
   - `## <Actual Element Name>` body containing nested markdown headers
2. Create requirement with `refinedBy` pointing to the single-element identifier (`file.md#fragment`); run `reqvire validate`; assert success.
3. Change single-element metadata type to `requirement`; run `reqvire validate`; assert failure with incompatible type message.
4. Point `refinedBy` to plain file path (no fragment), including a `# Element` file path; run `reqvire validate`; assert failure.
5. Create `# Element` file where `## <Actual Element Name>` body contains multiple markdown headers (including `###`); run `reqvire validate`; assert success.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-document-refinement-format/test.sh)
  * verify: [Specification File Identification](../../../ModelStructure/StructureAndParsing.md#specification-file-identification)
  * verify: [Relation Element Type Validator](../../../Operations/Validation/ValidationRequirements.md#relation-element-type-validator)
---

### Single Root Hierarchy Ownership Validation Test

This test verifies that each requirement hierarchy element resolves to exactly one owning capability root.

#### Details

##### Acceptance Criteria
- Validation passes when all requirement hierarchy elements resolve to exactly one owning capability root.
- Validation fails when a hierarchy element resolves to more than one owning capability root.
- Validation fails when a hierarchy element resolves to zero owning capability roots.
- Validation error identifies the violating element and the resolved root set/count.

##### Test Criteria
1. **Valid single-root hierarchy:**
   - Create a hierarchy with one capability root, a specified top-level requirement, and descendants.
   - Run `reqvire validate`.
   - Assert success exit code.

2. **Invalid multi-root hierarchy ownership:**
   - Create one descendant requirement with ancestry or `specify` relations that resolve to two different capability roots.
   - Run `reqvire validate`.
   - Assert non-zero exit code.
   - Assert error contains "must resolve to exactly one owning capability".

3. **Invalid zero-root hierarchy ownership:**
   - Create a descendant chain containing only `requirement` elements and no owning capability.
   - Run `reqvire validate`.
   - Assert non-zero exit code.
   - Assert error contains "must resolve to exactly one owning capability".

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-single-root-hierarchy-validation/test.sh)
  * verify: [Single Root Hierarchy Ownership](../../../Operations/Validation/ValidationRequirements.md#single-root-hierarchy-ownership)
---

### Subdirectory Processing Verification

This test verifies that the system correctly processes only files within the current directory when run from a subfolder of a git repository and generates missing relation target errors for references to parent directories.

#### Details

##### Acceptance Criteria
- System shall process only files within the current directory when run from a subfolder
- System shall handle identifier normalization correctly within subdirectory context
- System shall generate missing relation target errors for references to elements or files outside the current subdirectory scope
- System shall work with model, serve, format, traces, and CRUD commands (validation is automatic)
- System shall ignore files outside the current directory scope
- System shall provide meaningful missing relation target error messages for parent directory references
- CRUD commands (add, rm, mv, mv-file) shall resolve paths relative to current working directory

##### Test Criteria
- Commands run from subdirectory process only files within that subdirectory
- Files outside the current directory are not included in processing or output
- Identifier normalization works correctly for paths within subdirectory
- References to parent directories generate missing relation target errors with clear error messages
- Missing relation target errors specifically identify the unreachable parent directory reference
- All major commands (model, serve, format, traces) work from subdirectories with automatic validation
- CRUD commands (add, rm, mv, mv-file) resolve file paths relative to current working directory
- CRUD mv command successfully moves elements within subdirectory scope
- CRUD mv-file command successfully moves entire files within subdirectory scope
- Commands exit with validation error code when parent directory references cannot be resolved
- Error messages clearly explain the missing relation target due to parent directory reference

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-subdirectory-functionality/test.sh)
  * verify: [CLI Move Element Command](../../../Interfaces/CLI/Commands.md#cli-move-element-command)
  * verify: [CLI Move File Command](../../../Interfaces/CLI/Commands.md#cli-move-file-command)
---

### Type Validation Errors Test

Test verifies that type validation errors include helpful type lists.

#### Details
1. Test invalid element type in --filter-type:
   - Use invalid type like "invalid-type"
   - Verify error includes list of valid element types
   - Verify error includes "other-TYPENAME" pattern hint

2. Test invalid relation type in metadata:
   - Create element with invalid relation type like "invalidRelation"
   - Verify error includes list of valid relation types

3. Test custom type pattern acceptance:
   - Verify "other-interface" is accepted as valid type
   - Verify "other-" alone is rejected
   - Verify "other" alone is accepted

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-type-validation-errors/test.sh)
  * verify: [Type Validation Error Requirement](../../../Operations/Validation/ValidationRequirements.md#type-validation-error-requirement)
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
  * satisfiedBy: [test.sh](../../../../tests/test-valid-relations/test.sh)
  * verify: [Excluded File Relation Validation](../../../Operations/Validation/ValidationRequirements.md#excluded-file-relation-validation)
---

