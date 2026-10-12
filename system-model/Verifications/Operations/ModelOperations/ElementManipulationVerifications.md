# Elements

### Element Mutation and File Operation Verification Objective

This objective groups verification that shared element, relation, asset and file operations preserve model validity.

#### Details
Operation fixtures use typed MCP authoring requests and compare expected outputs and source trees with unified diffs. Cross-repository aggregate workspace cases exercise the transport-neutral core API; selected-worktree MCP ownership has separate protocol coverage.

#### Metadata
  * type: verification-objective

#### Relations
  * derive: [Atomic Relation Relink Test](#atomic-relation-relink-test)
  * derive: [Create Element Diagnostic Verification](#create-element-diagnostic-verification)
  * derive: [Create Element Duplicate Detection Verification](#create-element-duplicate-detection-verification)
  * derive: [Create Element Override Test](#create-element-override-test)
  * derive: [Create Element Test](#create-element-test)
  * derive: [Cross-Repository Workspace CRUD Test](#cross-repository-workspace-crud-test)
  * derive: [Delete Element Test](#delete-element-test)
  * derive: [File Persistence Test](#file-persistence-test)
  * derive: [Merge Elements Test](#merge-elements-test)
  * derive: [Move Element Test](#move-element-test)
  * derive: [Move File Squash Test](#move-file-squash-test)
  * derive: [Move Folder Test](#move-folder-test)
  * derive: [Ontology Boundary-Changing Mutation Test](#ontology-boundary-changing-mutation-test)
  * derive: [Relation Consistency Test](#relation-consistency-test)
  * derive: [Relation Cross-Section Conflict Verification](#relation-cross-section-conflict-verification)
  * derive: [Target Location Validation Test](#target-location-validation-test)
  * derive: [Verification Objective Mutation Test](#verification-objective-mutation-test)
---

### Atomic Relation Relink Test

Verify atomic relation rewiring in the shared core operation.

#### Details
- Resolve the source, old target and replacement target by exact name or canonical identifier against one pre-operation snapshot.
- Rewire one compatible relation or Contract Reference exactly once. Preserve ownership and permitted requirement-subtree boundaries.
- Reject cycles, incompatible endpoint types, missing old edges and multi-root candidates with structured diagnostics and guidance. Preserve source bytes and the accepted model; a later valid operation succeeds.
- Preview returns deterministic diff evidence without writes. Application validates the complete candidate and persists its semantic change atomically.
- Match the existing relink fixtures and unified source diffs. Their evidence covers current core behavior; new canonical persistence and MCP transport acceptance remain separate plans.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-relink-command/test.sh)
  * verify: [Atomic Relation Relink Operation](../../../Operations/ModelOperations/ElementManipulationRequirements.md#atomic-relation-relink-operation)
---

### Canonical Mutation Persistence Verification

Executable verification for canonical writes within model mutation transactions.

#### Details
- Use a small fixture matrix: create/override in a single-element document, relation edit in a multi-element document, move with referencing documents, and safe lint repair. Compare the complete affected source tree against expected fixture bytes with unified diffs.
- Require default formatting preview to propose zero additional changes in every just-written document. Check reserved subsection layout, parent-before-child ordering, sorted relations, readable labels and relative links through those representative fixtures.
- Keep an untouched model document, a binary/text evidence asset and fenced examples as preservation controls. Compare their bytes before and after; default writes must retain the authored inverse-relation subset.
- Exercise invalid candidates and an injected persistence failure. Require unchanged source bytes, accepted model revision and subsequent read results; a valid later mutation succeeds.
- Include semantic-contract candidates with undeclared or outside-context SHACL references, deletion or override that removes a referenced ontology declaration, and unlinking a required ontology `use` relation. Require shared semantic diagnostics with source identity, reference kind, IRI, reachable context and repair guidance; include the removed declaration source when known. Compare unchanged source files and accepted-model read results after each rejection.
- Exercise explicit full-relation formatting as the only control that adds generated inverse declarations. Repeating it is idempotent.
- Interface fixtures compare exact repaired and full-relations output, preserve unrelated bytes, and require default formatting to be a no-op after creation. Transaction tests reject a late invalid candidate and failed Git publication; folder-move rollback restores the source directory tree. Existing typed mutation fixtures retain semantic rejection, move and fenced-payload coverage.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Element Mutation and File Operation Verification Objective](#element-mutation-and-file-operation-verification-objective)
  * satisfiedBy: [mcp_session_tests.rs](../../../../crates/reqvire-cli/src/mcp_session_tests.rs)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-manipulation/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-interface-boundary/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-ontology-aware-mutations/test.sh)
  * satisfiedBy: [fenced_code.py](../../../../tests/test-parsing-functionality/fenced_code.py)
  * satisfiedBy: [test.sh](../../../../tests/test-semantic-contract-crud/test.sh)
  * verify: [Canonical Model Mutation Persistence](../../../Operations/ModelOperations/ElementManipulationRequirements.md#canonical-model-mutation-persistence)
  * verify: [CRUD Semantic Contract Mutation Validation](../../../Operations/ModelOperations/ElementManipulationRequirements.md#crud-semantic-contract-mutation-validation)
---

### Create Element Diagnostic Verification

Test verifies that the core create operation provides contextual error messages with examples when format parsing fails.

#### Details
**Test Cases:**

1. **Missing ### header**: Provide markdown without element header
   - Verify error message includes "Example of correctly formatted element"
   - Verify example shows "### Element Name"

2. **Multiple ### headers**: Provide markdown with multiple element headers
   - Verify error message includes example
   - Verify example includes Metadata subsection

3. **Invalid metadata format**: Provide metadata without bullet point
   - Verify error message includes "Expected format: '  * key: value'"
   - Verify example shows correct metadata format

4. **Invalid relation format**: Provide relation without proper markdown link
   - Verify error message includes "Expected format: '  * relationType: [Text](link)'"
   - Verify example includes Relations subsection

5. **Invalid contract_bindings format**: Provide contract_bindings without proper markdown link
   - Verify error message includes "Expected format: '  * [Text](link)'"
   - Verify example includes Contract Bindings subsection

6. **Example completeness**: Verify example includes all subsections
   - Details subsection with markdown formatting
   - Metadata subsection with type
   - Relations subsection with derivedFrom and satisfiedBy examples
   - Contract Bindings subsection
   - Separator (---)

**Success Criteria:**
- Core or MCP returns structured errors for all invalid inputs
- Error messages include "Example of correctly formatted element"
- Examples show proper structure for all subsections
- Format guidance is specific to the error type

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-add-command-error-messages/test.sh)
  * verify: [Create Element Operation](../../../Operations/ModelOperations/ElementManipulationRequirements.md#create-element-operation)
  * verify: [Markdown Structure Validator](../../../Operations/Validation/ValidationRequirements.md#markdown-structure-validator)
  * verify: [Validation Error Handling](../../../Operations/Validation/ValidationRequirements.md#validation-error-handling)
---

### Create Element Duplicate Detection Verification

Test verifies that the core create operation rejects elements with duplicate entries.

#### Details
Test cases:
1. **Duplicate relations**: Add element with same relation twice -> Error
2. **Duplicate contract_bindings**: Add element with same contract_bindings twice -> Error
3. **Cross-section duplicate**: Add element with same target in Relations and Contract Bindings -> Error

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-duplicate-detection/test.sh)
  * verify: [Create Element Operation](../../../Operations/ModelOperations/ElementManipulationRequirements.md#create-element-operation)
---

### Create Element Override Test

Verify shared create/override behavior against the existing override fixtures.

#### Details
- Override existing content and compare the complete resulting source fixture; require the core result to identify an update and the reloaded model to validate.
- Reject duplicate creation without override and preserve files. Override of a previously absent name acts as creation.
- Reject an override that would orphan children. Require child names and remediation guidance in the shared error while retaining source bytes.
- Override a childless element successfully and retain its incoming references and valid ownership.
- Match core results and unified source diffs. Terminal status wording is renderer evidence, not a required MCP response field; protocol acceptance is planned separately.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-crud-override/test.sh)
  * verify: [Create Element Operation](../../../Operations/ModelOperations/ElementManipulationRequirements.md#create-element-operation)
---

### Create Element Test

The test shall verify that new model elements can be created from a full Markdown definition string after validation, and that invalid element definitions are rejected with appropriate error reporting.

#### Details
**Test Setup:**
- Prepare a test model with existing files
- Prepare valid element definition strings in Markdown format (with ### header, metadata, relations, content)
- Prepare invalid element definition strings (malformed structure, duplicate names, invalid relations)
- Identify target file path

**Test Steps:**
1. Attempt to create an element with a valid definition string
2. Verify the element was parsed correctly from the string
3. Verify the element structure was validated (subsections, relations, format)
4. Verify the element name was checked for global uniqueness
5. Verify the element was inserted into the correct file
6. Verify the element has proper Markdown structure in the file
7. Validate the model after element creation

**Test Steps for Invalid Cases:**
1. Attempt to create an element with an invalid definition string
2. Verify the validation detects the structural errors
3. Verify the operation is rejected before insertion
4. Verify appropriate validation error messages are reported
5. Verify the target file remains unchanged

**Test Steps for Relation Validation:**
1. Create element with valid relations (targets exist in model)
2. Verify relations are normalized to workspace-root-relative format
3. Verify element is created successfully
4. Attempt to create element with non-existent relation target
5. Verify operation is rejected with clear error message
6. Verify error message indicates which relation target was not found
7. Verify target file remains unchanged after rejection

**Test Steps - Override with Orphaned Children Prevention:**
1. Create a parent element with child elements having parent hierarchical relations
2. Attempt to override the parent element with `override_existing: true`
3. Verify the operation is rejected with clear error message
4. Verify the error lists all children that would be orphaned
5. Verify the error provides resolution guidance
6. Create a parent element with children having multiple parent hierarchical relations
7. Override the parent element with `override_existing: true`
8. Verify the operation succeeds (children have other parents)
9. Override an element with no children
10. Verify the operation succeeds

**Test Coverage:**
- Valid element with all subsections (metadata, relations, details)
- Valid element with minimal structure (only ### header and content)
- Invalid element with duplicate name (violates global uniqueness)
- Invalid element with malformed subsections
- Invalid element with invalid relations
- Invalid element with missing ### header
- **Valid element with relations to existing elements**
- **Invalid element with relation to non-existent element**
- **Relations specified as relative paths (../File.md#element)**
- **Relations specified as workspace-root-relative paths (specifications/File.md#element)**
- **Relations specified as same-file references (#element)**
- **External link relations (http://, https://) are allowed**
- **Element ordering after creation (parent before children)**

**Success Criteria:**
- Valid element definitions are accepted and created
- Element is parsed from the Markdown string correctly
- Element structure validation runs before insertion
- Global uniqueness is enforced
- Element inserted following Element Ordering Behavior
- Invalid element definitions are rejected
- Validation errors are reported clearly
- Failed operations do not modify target files
- Model validation passes after successful creation
- **Relation targets are validated to exist in the model**
- **Non-existent relation targets cause rejection**
- **Relation paths are normalized to workspace-root-relative format**
- **External links (http://, https://) bypass validation**
- Override of parent elements with orphaned children is rejected
- Error message clearly lists orphaned children with resolution guidance
- Override succeeds when children have other parent relations

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-crud-manipulation/test.sh)
  * verify: [Create Element Operation](../../../Operations/ModelOperations/ElementManipulationRequirements.md#create-element-operation)
---

### Cross-Repository Workspace CRUD Test

This test verifies that graph-backed CRUD commands work across multiple eligible Git worktrees under one effective workspace root while rejecting create, move, and asset targets in non-Git workspace folders.

#### Details
**Test Setup:**
- Create a temporary effective workspace root that is not itself a Git repository.
- Create `repo-a` and `repo-b` as descendant eligible Git worktrees.
- Create an ordinary `not-a-repo` folder under the same workspace root.
- Store all model and element fixture content in fixture files, not inline shell heredocs.

**Success Criteria - Cross-Repository CRUD:**
1. the shared create operation in apply mode creates a new element in `repo-b` from fixture content and indexes it with a workspace-root-relative identifier.
2. the shared element-move operation in apply mode moves an element from `repo-a` to a target file in `repo-b` and indexes the moved element under `repo-b`.
3. the shared rename operation in apply mode renames an element in `repo-b` and removes the old name from search results.
4. the shared link operation in apply mode adds a relation from a `repo-b` element to a `repo-a` InternalPath target normalized as `repo-a/...`.
5. the shared unlink operation in apply mode removes that cross-repo relation and leaves no satisfiedBy relation on the source element.
6. the shared relink operation in apply mode replaces a `repo-b` InternalPath target with a `repo-a` InternalPath target.
7. the shared merge operation in apply mode merges a source requirement stored in `repo-a` into a target requirement stored in `repo-b`, removes the source element, and preserves merged content on the target.
8. the shared file-move operation in apply mode moves a model file from `repo-a` to `repo-b`, removes the source file, creates the target file, and keeps the model valid.
9. the shared folder-move operation in apply mode moves a model folder subtree from `repo-a` to `repo-b`, removes the source folder, creates the target folder, and keeps the model valid.
10. the shared asset-move operation in apply mode moves an InternalPath asset from `repo-a` to `repo-b` and rewrites affected model references to the new workspace-root-relative path.
11. the shared asset-removal operation in apply mode removes an InternalPath asset in `repo-b` and removes affected model references.
12. the shared remove operation in apply mode removes an element from `repo-a` and leaves no stale indexed element.
13. The workspace validates after the full cross-repo CRUD sequence.

The the shared link operation in apply mode result MUST reload in a fresh CLI process with the exact workspace-root-relative artifact target. Its Markdown path MUST resolve from `repo-b/specifications` to the recorded `repo-a` file without an extra source-folder traversal.

**Success Criteria - Non-Git Workspace Rejection:**
1. the shared create operation in apply mode rejects a target file under `not-a-repo` with an error mentioning eligible Git worktrees.
2. the shared element-move operation in apply mode rejects a target file under `not-a-repo` with an error mentioning eligible Git worktrees.
3. the shared file-move operation in apply mode rejects a target file under `not-a-repo` with an error mentioning eligible Git worktrees.
4. the shared folder-move operation in apply mode rejects a target folder under `not-a-repo` with an error mentioning eligible Git worktrees.
5. the shared asset-move operation in apply mode rejects a target asset path under `not-a-repo` with an error mentioning eligible Git worktrees.
6. Each rejected operation returns a structured error and does not create the requested target under `not-a-repo`.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-multi-workspace-crud/test.sh)
  * verify: [Target Location Validation and Auto-Creation](../../../Operations/ModelOperations/ElementManipulationRequirements.md#target-location-validation-and-auto-creation)
---

### Delete Element Test

The test shall verify that existing model elements can be deleted, all relations referencing the deleted element are automatically removed, and empty files are removed when no elements remain.

#### Details
**Test Setup:**
- Create a test model with multiple elements
- Create an element to be deleted with incoming and outgoing relations
- Document all relations pointing to the element (derivedFrom, verifiedBy, verify, satisfiedBy)
- Create a file with only one element for empty file cleanup testing

**Test Steps:**
1. Delete the target element
2. Verify the element was completely removed from the file
3. Verify all relations pointing to the deleted element were removed from other elements
4. Verify the file structure and formatting remain intact
5. Validate the model after element deletion
6. Check that no dangling relations exist

**Test Steps - Empty File Cleanup:**
1. Delete the only element in a file
2. Verify the element is removed
3. Verify the file is deleted from the filesystem
4. Verify the file deletion is reported in the operation output
5. Delete an element leaving other elements in the file
6. Verify the file is NOT deleted (still contains elements)

**Test Steps - Unrelated Artifact Links During Deletion:**
1. Validate a model containing a nested requirement file with `satisfiedBy` links to workspace-root `Makefile` and `LICENSE` artifacts and a source file with an extension. Create a decoy source file at the same relative path below the model file's folder, so an incorrect source-relative resolution would select another file. Link the requirement to a disposable verification in another file.
2. Run a dry-run deletion of that verification. Verify all model/artifact files remain byte-for-byte unchanged and the proposed diff preserves artifact paths without substituting element fragments.
3. Apply the deletion. Verify the verification and its incoming relations are removed while all unrelated artifact targets and target kinds remain unchanged.
4. Verify artifact bytes, the accepted Git HEAD, and the index remain unchanged. Load the persisted model in a fresh CLI process and require successful validation.

**Test Steps - Orphaned Children Prevention:**
1. Create a parent element with child elements having parent hierarchical relations
2. Attempt to delete the parent element
3. Verify the operation is rejected with clear error message
4. Verify the error lists all children that would be orphaned
5. Verify the error provides resolution guidance
6. Create a child element with multiple parent hierarchical relations
7. Delete one parent element
8. Verify the operation succeeds (child has other parents)
9. Delete the last remaining parent
10. Verify the operation is rejected (child would be orphaned)

**Test Steps - Native Concept Reference Deletion Prevention:**
1. Use the concept-reference fixture containing the native Service Endpoint concept and an API requirement that references it.
2. Attempt to remove that native concept through the shared remove operation.
3. Require rejection identifying the referencing requirement, missing concept name and identifier, and removed declaration source.
4. Compare the complete source file with its pre-operation snapshot using a unified diff; rejection must leave its bytes unchanged.

The CRUD manipulation fixture establishes ordinary removal, artifact preservation and orphan protection. The concept-reference fixture establishes native concept deletion protection. Semantic-contract fixtures and canonical transaction tests provide separate rejection and persistence evidence.

**Success Criteria:**
- Element is completely removed from the source file
- All incoming relations (relations from other elements to the deleted element) are removed
- File structure remains valid
- Remaining artifact relations retain their original file targets in both dry-run diffs and persisted files, including extensionless root artifacts; fresh-process validation succeeds after deletion.
- Model validation passes
- No dangling relations remain in the model
- Files containing only the deleted element are removed
- Files with remaining elements are preserved
- File deletion is reported when it occurs
- Deletion of parent elements with orphaned children is rejected
- Error message clearly lists orphaned children with resolution guidance
- Deletion succeeds when children have other parent relations
- Deletion of a native concept still referenced by a requirement is rejected with source and target identities, removed declaration source, and unchanged source bytes

**Test Coverage:**
- Delete element with `derivedFrom` relations pointing to it
- Delete element with `verifiedBy` relations pointing to it
- Delete element with `verify` relations pointing to it
- Delete element with `satisfiedBy` relations pointing to it
- Delete element with multiple types of incoming relations
- Delete a verification referenced by a file containing unrelated extensionless root artifact links and confirm preview/apply preservation and valid persisted source
- Delete last element in file (triggers file deletion)
- Delete element leaving other elements (file preserved)
- Delete parent element with children (single parent - rejected)
- Delete parent element with children having multiple parents (allowed)
- Error message validation for orphaned children prevention
- Delete a native concept required by Concept References (rejected)

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-concept-references/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-manipulation/test.sh)
  * verify: [Delete Element Operation](../../../Operations/ModelOperations/ElementManipulationRequirements.md#delete-element-operation)
---

### File Persistence Test

The test shall verify that element manipulation operations are persisted to source files in storage, that only modified files are flushed, and that elements are reordered following the Element Ordering Behavior.

#### Details
**Test Setup:**
- Create a test model with multiple files and elements
- Record initial file timestamps and content
- Prepare element manipulation operations that affect specific files
- Create files with unordered elements (children before parents)

**Test Steps:**
1. Perform element manipulation operation (create/delete/move)
2. Verify that modified files are written to disk
3. Verify that file content on disk matches the in-memory model state
4. Verify that only files affected by the operation were rewritten
5. Verify that unmodified files remain unchanged (check timestamps)
6. Read files from disk and parse to confirm changes persisted
7. Test file I/O error handling (simulate write failures)

**Test Steps - Element Ordering:**
1. Add element that is a child of an existing element
2. Verify that after persistence, parent appears before child
3. Add element that is a parent of existing elements
4. Verify that after persistence, new parent appears before its children
5. Move element into a file with existing hierarchy
6. Verify elements are reordered according to Element Ordering Behavior

**Success Criteria:**
- All changes are persisted to disk after manipulation completes
- File content on disk matches in-memory model state exactly
- Only modified files are written (optimization verified)
- Unmodified files have unchanged timestamps
- File format and structure are maintained
- Errors are handled gracefully with appropriate reporting
- Elements are ordered with parents before children (file-local derivedFrom)
- Siblings at same level are sorted alphabetically
- Hierarchy groups are kept together

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-crud-file-persistence/test.sh)
  * verify: [Element Manipulation File Persistence](../../../Operations/ModelOperations/ElementManipulationRequirements.md#element-manipulation-file-persistence)
---

### Merge Elements Test

Test verifies that the core merge operation correctly combines elements.

#### Details
Test cases:
1. **Basic merge**: merge two requirements and compare resulting file with expected fixture.
2. **Source deletion**: verify merged source element no longer appears in search results.
3. **Relation redirection**: verify relations that pointed to source now point to target.
4. **Multi-element merge**: merge multiple sources and compare resulting file with expected fixture.
5. **Type compatibility error**: merging incompatible types fails with type-mismatch message.
6. **Merge into self error**: merging an element into itself fails.
7. **Non-existent source error**: merge with missing source fails.
8. **Dry-run mode**: `dry_run: true` does not modify files.
9. **Relation deduplication**: duplicate merged relation appears once.
10. **Regression scenario**: multi-source merge keeps target element present and model validates.
11. **Document-to-elements rejection**: merging a source from `# Element` into a target in `# Elements` fails with explicit manual-migration guidance.
12. **Single-root rejection**: merge that would create multi-root hierarchy ownership fails with deterministic single-root ownership error output and persists no changes.

The owning merge suite appends identifier selection for the target and each source member, combined selection, mixed-alias duplicate/self rejection, known wrong-type identifiers and atomic late-source rejection.


The linked fixtures establish shared merge behavior. Typed MCP fixture execution and canonical transaction tests provide protocol and persistence evidence.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-document-operation-constraints/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-merge-elements/test.sh)
  * verify: [Merge Element Operation](../../../Operations/ModelOperations/ElementManipulationRequirements.md#merge-element-operation)
---

### Model Mutation Validation Gate Verification

Representative automated verification of complete-candidate validation through MCP create, MCP override dry-run, and MCP override operations. Fixture-backed E2E assertions compare results and source files with expected output using unified diffs.

#### Details

##### Acceptance Criteria
- Graph-backed mutations and supported dry runs use the shared rule contracts on candidate content, identifiers, membership, ownership, and canonical relations.
- Invalid candidates preserve authored source bytes and the accepted graph, including changes to otherwise untouched elements affected by the mutation.
- Core and MCP mutation entry points retain shared structured diagnostics, guidance, and applicable repair examples.

##### Test Criteria
1. Start with a valid model. Submit an MCP create containing a prohibited prose link; require rejection, structured source/target context and repair guidance, and unchanged model files.
2. Submit an MCP override dry-run containing the same violation; require rejection rather than a successful preview and preserve source files.
3. Create an otherwise valid target element that makes an untouched prose destination resolve to a model element. Require complete-candidate rejection and preservation of all source files.
4. Submit an MCP override containing a prohibited prose link. Require shared structured guidance, unchanged source files, and preservation of the accepted model fingerprint and content.
5. After rejected creates/overrides, submit an explicit repair with readable prose and a valid Contract Reference. Require success, match the resulting file against a checked-in expected fixture, and validate the persisted model when reloaded.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Element Mutation and File Operation Verification Objective](#element-mutation-and-file-operation-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-override/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [Element Manipulation Operations](../../../ModelStructure/ModelManagement.md#element-manipulation-operations)
---

### Move Element Test

The test shall verify that existing model elements can be moved to different locations, all relations referencing the moved element are automatically updated, target locations are created if needed, and empty source files are removed when no elements remain.

#### Details
**Test Setup:**
- Create a test model with multiple files
- Create an element to be moved with incoming and outgoing relations
- Document all relations pointing to the element
- Create a file with only one element for empty file cleanup testing
- Prepare non-existent target locations for auto-creation testing

**Test Steps:**
1. Move the element to the target location
2. Verify the element was removed from the source location
3. Verify the element was inserted into the target location
4. Verify all element content, metadata, and outgoing relations are preserved
5. Verify the element identifier was updated to reflect the new location
6. Verify all incoming relations were updated to reference the new identifier
7. Validate the model after element move
8. Check that no dangling relations exist

**Test Steps - Auto-Creation:**
1. Move element to non-existent target file
2. Verify target file is created with proper structure

**Test Steps - Empty Source File Cleanup:**
1. Move the only element from a file to another location
2. Verify the element is moved successfully
3. Verify the source file is deleted from the filesystem
4. Verify the file deletion is reported in the operation output
5. Move an element leaving other elements in the source file
6. Verify the source file is NOT deleted (still contains elements)

**Success Criteria:**
- Element is removed from source location
- Element is inserted into target location
- Element content and metadata are preserved
- Element identifier is updated correctly
- All incoming relations (derivedFrom, verifiedBy, verify, satisfiedBy) are updated to the new identifier
- Outgoing relations (relations from the moved element to other elements) are preserved unchanged
- File structure remains valid in both source and target files
- Model validation passes
- Non-existent target files are created with proper structure
- Empty source files are deleted after move
- Source files with remaining elements are preserved
- File creation and deletion are reported

**Test Coverage:**
- Move element to a different file
- Move element with `derivedFrom` relations pointing to it
- Move element with `verifiedBy` relations pointing to it
- Move element with `verify` relations pointing to it
- Move element with `satisfiedBy` relations pointing to it
- Move element with multiple types of incoming relations
- Move to non-existent target file (auto-create)
- Move last element from file (triggers source file deletion)
- Move element leaving other elements (source file preserved)
- Verify element ordering after move (parent before children)
- Reject move into existing `# Element` target that would create multiple elements

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-crud-empty-file-cleanup/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-manipulation/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-document-operation-constraints/test.sh)
  * verify: [Move Element Operation](../../../Operations/ModelOperations/ElementManipulationRequirements.md#move-element-operation)
---

### Move File Squash Test

Verify shared whole-file squash semantics and typed MCP move-file selection.

#### Details
- With `source_file`, an occupied `target_file` and `squash: true`, merge eligible contained elements into the existing document through the core writer; remove the emptied source file.
- Preserve existing target/page content and fenced payloads. Apply canonical file-local hierarchy ordering rather than placement into a legacy level-2 element group.
- Rewrite every affected identifier and incoming/outgoing relation, dependency, concept and evidence link. Structured results include moved elements, old/new identifier mappings, changed files and diffs.
- With `squash: false`, reject an occupied destination. Reject incompatible single-element destinations, workspace paths and invalid resulting models atomically.
- `dry_run: true` returns reviewable candidate diffs without writes. Enabled application matches the expected source fixture, validates on reload and requires no further default formatting in changed documents.
- Retain single/multiple-element, ontology-boundary, incoming/outgoing-reference, preview and occupied-target fixture coverage. The fixture harness submits typed MCP requests; accepted source bytes and rewritten references are compared with expected files.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-crud-mv-file-squash/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-document-operation-constraints/test.sh)
  * verify: [Move File Operation](../../../Operations/ModelOperations/ElementManipulationRequirements.md#move-file-operation)
---

### Move Folder Test

The test shall verify that the shared folder-move operation recursively relocates a folder subtree and rewrites every affected model reference before persistence.

#### Details
**Acceptance Criteria:**
- The operation builds a complete old-path to new-path move plan for every file under the source folder.
- The operation rejects missing sources, file sources, existing targets, target paths inside the source folder, overwrite collisions, and workspace-scope escapes before changing files.
- The operation rewrites identifiers for every moved model element.
- The operation rewrites incoming relation targets for moved elements.
- The operation rewrites contract_bindings that target moved contract elements.
- The operation rewrites Concept References that target moved concept elements.
- The operation rewrites model-owned InternalPath references that target moved files.
- The operation preserves internal relations between moved elements and leaves them resolvable after the move.
- The operation validates the candidate model before persistence, including contract_bindings compatibility, concept-reference resolution, semantic-contract SHACL reference reachability, and ontology-root validation.
- Dry-run mode reports the full plan and diffs without filesystem changes.
- Non-dry-run mode persists the recursive move atomically enough that a validation failure leaves source files unchanged.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-crud-mv-folder/test.sh)
  * verify: [Move Folder Operation](../../../Operations/ModelOperations/ElementManipulationRequirements.md#move-folder-operation)
---

### Ontology Boundary-Changing Mutation Test

This test verifies ontology-aware mutation rewrites when a CRUD change crosses ontology boundaries or creates a new boundary inside an existing hierarchy.

#### Details

##### Acceptance Criteria
- Converting a child ontology contribution into an explicit boundary rewrites `ontology_base` and `ontology_prefix` consistently in authored Turtle.
- Converting a grandchild contribution into an explicit boundary rewrites only the affected ontology block and preserves the parent boundary declarations.
- When a grandchild creates a new boundary under an already bounded child ontology, the resulting export declares the direct parent import for the child boundary and preserves the child boundary's import to its own parent.
- Converting an explicit boundary back into a contribution removes the boundary declaration and removes now-unnecessary imports.
- Deleting an ontology contribution preserves the remaining ontology boundary and keeps validation green.
- Moving an ontology contribution preserves its inherited ontology prefix/base bindings and keeps validation green after the move.
- Renaming an ontology contribution updates the element heading without corrupting inherited ontology bindings or validation.
- Merging ontology elements consolidates their authored Turtle into one target Ontology block while preserving the target boundary and validation.
- Validation passes after each successful mutation.

##### Test Criteria
1. **Child boundary conversion**
   - Start with a root ontology and a child contribution that inherits the root base.
   - Run the shared create operation in override mode to convert the child into an explicit boundary.
   - Verify the child block now declares its own `ontology_base` and `ontology_prefix`.
   - Verify the child block includes the required `owl:imports` to the root ontology.

2. **Grandchild boundary conversion under a bounded child**
   - Start with a bounded child ontology that imports the root ontology and a grandchild contribution derived from the child.
   - Run the shared create operation in override mode to convert the grandchild into an explicit boundary.
   - Verify the grandchild block declares its own `ontology_base` and `ontology_prefix`.
   - Verify the grandchild block imports the child ontology base.
   - Verify the child boundary still imports the root ontology base.

3. **Boundary revert**
   - Run the shared create operation in override mode to convert the grandchild boundary back into a contribution.
   - Verify the grandchild block no longer declares its own boundary.
   - Verify the grandchild block no longer carries a redundant cross-boundary import.

4. **Ontology delete**
   - Run the shared remove operation in apply mode on an ontology contribution.
   - Verify the deleted ontology block is removed.
   - Verify the remaining ontology boundary still validates.

5. **Ontology rename**
   - Run the shared rename operation in apply mode on an ontology contribution.
   - Verify the heading changes and the inherited ontology binding remains valid.

6. **Ontology move**
   - Run the shared element-move operation in apply mode on an ontology contribution into a new file.
   - Verify the moved file contains the ontology contribution and the source file no longer does.
   - Verify the moved contribution keeps its inherited ontology binding.

7. **Ontology merge**
   - Run the shared merge operation in apply mode on ontology blocks and verify the target element remains a single ontology element with one `#### Ontology` block.
   - Verify all source authored Turtle is rewritten to the target ontology boundary before consolidation.
   - Verify the merged result preserves the target ontology metadata and recalculates inherited prefix bindings, document declarations, `owl:imports`, and reachable SHACL references as needed.
   - Verify source ontology elements are removed after successful merge and the model still validates.

##### Success Criteria
- The mutation keeps ontology export valid after each step.
- Boundary-changing rewrites remain scoped to the affected ontology element.
- Explicit authored prefixes and imports are rewritten consistently with the resolved ontology base.
- The suite covers the ontology-specific mutation cases currently exercised by the test file, including a positive ontology merge path.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-ontology-aware-mutations/test.sh)
  * verify: [CRUD Semantic Contract Mutation Validation](../../../Operations/ModelOperations/ElementManipulationRequirements.md#crud-semantic-contract-mutation-validation)
  * verify: [Ontology and Shapes Collection](../../../Semantics/SemanticModelRequirements.md#ontology-and-shapes-collection)
---

### Relation Consistency Test

The test shall verify that bidirectional relation consistency is maintained when elements are manipulated.

#### Details
<details>
<summary>Test Criteria</summary>

**Test Setup:**
- Create a test model with elements having bidirectional relations
- Set up test cases for derivedFrom/derive, verifiedBy/verify relations

**Test Steps:**
1. Create an element with relations and verify bidirectional consistency
2. Delete an element and verify both forward and backward relations are removed
3. Move an element and verify both forward and backward relations are updated
4. Validate model consistency after each manipulation operation

**Success Criteria:**
- After element creation: bidirectional relations are properly established
- After element deletion: both forward and backward relations are removed
- After element move: both forward and backward relations are updated
- Model validation passes after each operation
- No dangling or inconsistent relations exist

**Test Coverage:**
- Verify derivedFrom/derive relation consistency
- Verify verifiedBy/verify relation consistency
- Test consistency after create operations
- Test consistency after delete operations
- Test consistency after move operations

</details>

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-crud-relation-consistency/test.sh)
  * verify: [Relation Consistency Maintenance](../../../Operations/ModelOperations/ElementManipulationRequirements.md#relation-consistency-maintenance)
---

### Relation Cross-Section Conflict Verification

Test verifies that core link operations detect cross-section conflicts.

#### Details
Test cases:
1. Element has contract_bindings to B -> the shared link operation in apply mode -> Error (already in Contract Bindings)
2. Element has relation to B -> the shared link operation in apply mode -> Error (already in Relations)

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-duplicate-detection/test.sh)
  * verify: [Cross-Section Duplicate Validation](../../../Operations/Validation/ValidationRequirements.md#cross-section-duplicate-validation)
---

### Target Location Validation Test

The test shall verify that target file path validation and auto-creation work correctly, enforcing safety constraints for gitignore, reqvireignore, and path depth limits.

#### Details
**Test Setup:**
- Create `.gitignore` with exclusion patterns (e.g., `**/build/**`, `temp-*.md`)
- Create `.reqvireignore` with exclusion patterns (e.g., `**/draft-*.md`)
- Prepare valid target paths within depth limits
- Prepare invalid target paths (ignored paths, excessive depth)

**Test Steps - Path Validation:**
1. Attempt to create element in path excluded by `.gitignore`
2. Verify operation is rejected with appropriate error message
3. Attempt to create element in path excluded by `.reqvireignore`
4. Verify operation is rejected with appropriate error message
5. Attempt to create element in path nested more than 10 subdirectories deep
6. Verify operation is rejected with depth limit error
7. Attempt to create element in valid path
8. Verify operation succeeds

**Test Steps - Auto-Creation:**
1. Create element in non-existent file with valid path
2. Verify file is created with proper structure (`# Elements` header)
3. Verify existing file content is preserved when adding elements

**Success Criteria:**
- Paths excluded by `.gitignore` are rejected
- Paths excluded by `.reqvireignore` are rejected
- Paths exceeding 10 subdirectory depth are rejected
- Error messages clearly indicate which constraint was violated
- Valid paths are accepted
- Non-existent files are created with proper structure
- Created files follow Reqvire markdown conventions

**Test Coverage:**
- Gitignore pattern exclusion
- Reqvireignore pattern exclusion
- Path depth limit (exactly 10, 11, 15 subdirectories)
- Auto-create file
- Valid path variations

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-crud-target-location-validation/test.sh)
  * verify: [Target Location Validation and Auto-Creation](../../../Operations/ModelOperations/ElementManipulationRequirements.md#target-location-validation-and-auto-creation)
---

### Verification Objective Mutation Test

The test shall verify that verification-objective elements participate in verification-family hierarchy mutations while remaining separate from concrete verification evidence and verification semantics.

#### Details
Test cases:
1. **Objective hierarchy link**: the shared link operation in apply mode permits `verification-objective` to derive from another verification-family element and persists the relation.
2. **Objective hierarchy unlink**: the shared unlink operation in apply mode removes the objective hierarchy relation and keeps the model valid.
3. **Concrete verification relink**: the shared relink operation in apply mode can move a concrete verification from one verification objective parent to another.
4. **Objective move**: the shared element-move operation in apply mode moves a verification objective to another file and updates incoming concrete verification `derivedFrom` relations.
5. **Objective merge**: the shared merge operation in apply mode permits objective-to-objective merge and removes the merged source element.
6. **Mixed merge rejection**: the shared merge operation in apply mode rejects verification-objective to concrete-verification merge as a type mismatch.
7. **Verification relation rejection**: the shared link operation in apply mode rejects `verify` and `verifiedBy` relations involving verification-objective as the verification endpoint.
8. **Evidence relation rejection**: the shared link operation in apply mode rejects `satisfiedBy` evidence relations from verification-objective to internal file targets.
9. **Dry-run move**: the shared element-move operation in preview mode reports the operation without mutating source or target files.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-verification-objective-mutations/test.sh)
  * verify: [Element Type Relation Compatibility](../../../ModelStructure/ModelManagement.md#element-type-relation-compatibility)
  * verify: [Relation Management Operations](../../../ModelStructure/ModelManagement.md#relation-management-operations)
  * verify: [Atomic Relation Relink Operation](../../../Operations/ModelOperations/ElementManipulationRequirements.md#atomic-relation-relink-operation)
  * verify: [Merge Element Operation](../../../Operations/ModelOperations/ElementManipulationRequirements.md#merge-element-operation)
  * verify: [Move Element Operation](../../../Operations/ModelOperations/ElementManipulationRequirements.md#move-element-operation)
---
