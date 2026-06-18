# Elements

### Atomic Relation Relink Workflow Specification

Detailed workflow for atomically relinking an existing relation target.

#### Details
When relinking a relation, the system is expected to:
- Resolve source element and both targets (`old-target`, `new-target`) in the current model.
- Verify that the source currently has the specified relation to `old-target`.
- Build an in-memory transaction plan for relation rewiring.
- Apply candidate rewiring in memory first, without persisting files.
- Validate the candidate model after rewiring, including all existing validation rules.
- Persist changes only when candidate validation succeeds.
- Roll back and report errors when candidate validation fails.

For hierarchical relinks (`derivedFrom`/`derive`):
- Support subgraph boundary relinking semantics where the source can represent the root of a moved hierarchy boundary.
- Reject relinks that introduce circular dependency, missing parent relation, or single-root hierarchy ownership violations.

Output behavior:
- Support dry-run diff preview without persistence.
- Report affected elements/files deterministically.
- Return non-zero status on any failed validation or unresolved target.

#### Metadata
  * type: specification

#### Relations
  * define: [Atomic Relation Relink Operation](ElementManipulationRequirements.md#atomic-relation-relink-operation)
---

### CRUD Semantic Contract Validation Specification

Candidate semantic-contract validation for graph-backed CRUD mutations.

#### Details
Before any graph-backed CRUD mutation persists model files, the system is expected to validate the mutated in-memory model for dangling semantic-contract references.

This pre-persistence validation applies to graph-backed mutation commands:
- `add`
- `add --override`
- `rm`
- `mv`
- `rename`
- `merge`
- `mv-file`
- `link`
- `unlink`
- `relink`

When a mutation would leave a `Shapes` reference to an IRI that is not declared by any ontology `Ontology`, the command is expected to fail before flushing changes. The error is expected to include:
- the referencing semantic-contract identifier
- the reference kind, such as `sh:path`, `sh:targetClass`, or `sh:class`
- the missing IRI
- fix guidance to update/remove the SHACL reference or restore the declaration

When the mutation removed the ontology element that previously declared the missing IRI, the error is expected to include the removed declaration source identifier.

The command is expected not to persist source-file changes when this validation fails.

When a mutation would leave a `Shapes` reference to an IRI that is declared by an ontology element outside the semantic contract's explicit ontology-use graph, the command is expected to fail before flushing changes. The error is expected to include:
- the referencing semantic-contract identifier
- the reference kind, such as `sh:path`, `sh:targetClass`, or `sh:class`
- the referenced IRI
- the declaring ontology identifier
- guidance to add a `use` relation to the declaring ontology, move the declaration into the semantic contract's explicit ontology-use graph, or update/remove the reference

#### Metadata
  * type: specification

#### Relations
  * define: [CRUD Semantic Contract Mutation Validation](ElementManipulationRequirements.md#crud-semantic-contract-mutation-validation)
---

### Create Element Workflow Specification

Detailed workflow for creating new model elements.

#### Details
When creating a new element, the system is expected to:
- Accept a string containing the full element definition in Markdown format
- Accept target location: file path
- Validate the target location using path validation rules
- Create target file if it does not exist (subject to validation constraints)
- Parse and validate the element definition string
- Verify the element name is globally unique in the model
- Validate and normalize all relations following clearly defined specifications
- Insert the element into the target file following Element Ordering Behavior
- Reject the operation and report validation errors if validation fails
- Provide updates report following Diff Output Format Specification
- The system is expected to support override mode to replace existing element with same name following rules defined in Create Element Override Behavior
- When override targets an ontology element, the operation is expected to support ontology rebasing by updating `ontology_base` and `ontology_prefix` and rewriting dependent ontology boundaries, inherited prefix declarations, imports, and reachable SHACL references atomically before persistence

#### Metadata
  * type: specification
---

### Delete Element Workflow Specification

Detailed workflow for deleting existing model elements.

#### Details
When deleting an element, the system is expected to:
- Check if any child elements would become orphaned (have no remaining parent hierarchical relations after deletion)
- Reject the operation if any child would become orphaned
- Validate the candidate model before persistence and reject deletion when removing the element would leave semantic-contract SHACL references to ontology terms that are no longer declared anywhere
- Report semantic deletion blockers with the referencing semantic-contract identifier, reference kind, missing IRI, and the deleted element/semantic contract that removed the declaration when known
- Provide clear error message listing orphaned children with resolution guidance
- Allow deletion if children have other parent hierarchical relations
- Remove the element and all its content from the source file
- Identify all relations pointing to the deleted element (incoming relations)
- Remove all relations that reference the deleted element from other elements
- Identify all relations from the deleted element (outgoing relations)
- Remove the complete element section including separators
- Maintain file structure and formatting after deletion
- Provide updates report following Diff Output Format Specification

**Empty File Cleanup:**
- After deleting the element, check if the source file contains any remaining elements
- If no elements remain and all sections are empty (only page content, headers, or whitespace), remove the file from the filesystem
- If the file is removed, report the file deletion in the operation output

**Relation Handling:**
- All `derivedFrom` relations pointing to the deleted element is expected to be removed
- All `verifiedBy` relations pointing to the deleted element is expected to be removed
- All `verify` relations pointing to the deleted element is expected to be removed
- All `satisfiedBy` relations pointing to the deleted element is expected to be removed
- Relations from the deleted element are automatically removed with the element

#### Metadata
  * type: specification
---

### Element Manipulation File Persistence Contract Specification

#### Details
The system is expected to persist all element manipulation operations to the source files in storage, synchronizing changes from the in-memory model to the file system and reordering elements following the Element Ordering Behavior.

#### Metadata
  * type: specification
---

### Merge Element Workflow Specification

Detailed workflow for merging multiple source elements into a target element.

#### Details
When merging elements, the system is expected to:
- Accept target element name (must exist in the model)
- Accept one or more source element names (must exist in the model)
- Validate type compatibility following clearly defined rules in Merge Type Compatibility Constraint
- Transform and merge content following clearly defined rules in Merge Content Transformation Behavior
- Preserve target element's metadata (discard source metadata)
- Delete source elements after successful merge
- Update all relations pointing to source elements to point to target
- Remove empty source files when no elements remain
- Provide updates report following Diff Output Format Specification

The system is expected to reject the operation with a clear error message if:
- The target element does not exist
- Any source element does not exist
- Source and target element types are incompatible per Merge Type Compatibility Constraint
- Merged result would have cross-section duplicates per Merge Content Transformation Behavior

For `# Element` targets:
- The merged result is expected to remain serialized as `# Element` with a single implicit element.
- Merged content is expected to remain inside the `## <Actual Element Name>` body of that element.

For ontology targets:
- The merged result is expected to keep a single `#### Ontology` fenced Turtle block in the target element.
- All source ontology Turtle is expected to be rewritten to the target ontology base, prefix, and term namespace before consolidation.
- The rewritten source ontology content is expected to be folded into the target's single ontology block.
- The merge is expected to recalculate inherited prefix bindings, document declarations, `owl:imports`, and reachable SHACL references before persistence.

For single-element-to-elements merge direction:
- If any source element is in a `# Element` file and the target element is in a `# Elements` file, the merge is expected to be rejected with a clear error.
- The error is expected to state that this conversion must be done manually to avoid breaking `# Elements` parsing rules.

#### Metadata
  * type: specification
---

### Move Element Workflow Specification

Detailed workflow for moving existing model elements to different file locations.

#### Details
When moving an element, the system is expected to:
- Validate the target location using path validation rules
- Create target file if it does not exist (subject to validation constraints)
- Remove the element from the source file
- Insert the element into the target file following Element Ordering Behavior
- Preserve all element content, metadata, and relations
- Update the element's identifier to reflect the new location
- Identify all relations pointing to the moved element (incoming relations)
- Update all relations that reference the moved element with the new identifier
- Maintain file structure and formatting in both source and target files
- Ensure the element name is globally unique in the model
- Provide updates report following Diff Output Format Specification

Document format rule:
- If the target file is an existing `# Element` file, moving an element from a different file into it is expected to be rejected with a clear error, because `# Element` files contain exactly one element.

**Empty Source File Cleanup:**
- After moving the element, check if the source file contains any remaining elements
- If no elements remain (only page content, headers, or whitespace), remove the source file from the filesystem
- If the file is removed, report the file deletion in the operation output

**Relation Update Requirements:**
- All relations (both forward and backward) pointing to the moved element is expected to be updated to the new identifier
- Relations within the moved element (outgoing relations) is expected to be preserved unchanged

**Identifier Update:**
- The element's identifier changes from `<old-file>#<element-name>` to `<new-file>#<element-name>`
- All references to the old identifier is expected to be updated to the new identifier

#### Metadata
  * type: specification
---

### Move File Operation Contract Specification

#### Details
When moving a file, the system is expected to:
- Accept source file path (relative to git repository root)
- Accept target file path (relative to git repository root)
- Accept optional squashing flag
- Validate both source and target paths
- Move the physical file from source to target location
- Update all element identifiers within the file to reflect the new file path
- Update all relation references (both forward and backward) throughout the model that point to any element in the moved file
- Preserve all file content, structure, and formatting
- Provide updates report following Diff Output Format Specification

The system is expected to reject the operation with a clear error message if:
- The source file does not exist
- The target file already exists (unless --squash flag is provided)
- The source or target paths fail validation
- `--squash` is used with a target that is an existing `# Element` file

**Squash Mode Behavior:**
When the --squash flag is provided and the target file already exists, the system is expected to:
- Move all elements from the source file to the target end of file
- Remove the source file after all elements have been successfully moved
- Preserve element ordering from the source file when inserting into target section
- Reject squash if target is `# Element` format (single-element file)

#### Metadata
  * type: specification
---

### Orphaned Children Error Message Specification

The error message for orphaned children prevention is expected to include:
- Statement that deletion cannot proceed due to orphaned children
- Element name being deleted
- Count of child elements that would be orphaned
- List of child element names that would be orphaned
- Resolution guidance: "Delete the child elements first, or update the child elements to link to a different parent element"

#### Metadata
  * type: specification
---

### Relation Consistency Maintenance Contract Specification

#### Details
Relation consistency maintenance behavior during element manipulation:
- Maintains opposite relation consistency for supported relation pairs.
- If element `A` has `derivedFrom` to `B`, ensures inverse `derive` exists from `B` to `A`.
- If element `A` is `verifiedBy` verification `V`, ensures inverse `verify` exists from `V` to `A`.
- On delete operations, removes both incoming and outgoing relation references for deleted elements.
- On move/rename operations, updates relation targets for both forward and backward references.
- Preserves valid graph state after each manipulation step without dangling relation targets.

Validation behavior:
- Runs relation-consistency checks after manipulation operations.
- Reports detected inconsistencies with actionable context.
- Blocks manipulations that would leave the model in inconsistent relation state.

#### Metadata
  * type: specification

#### Relations
  * define: [Relation Consistency Maintenance](ElementManipulationRequirements.md#relation-consistency-maintenance)
---

### Relation Validation Specification

Rules for validating and normalizing relation targets during element creation and manipulation.

#### Details
**Target Format Support:**
- Relative paths from the target file location (e.g., `../UserReqs.md#requirement`)
- Paths relative to git repository root (e.g., `specifications/UserReqs.md#requirement`)
- Same-file references (e.g., `#other-requirement`)

**Normalization Rules:**
- All relation targets must be normalized to git repository root relative format before insertion
- All relation targets must reference existing elements in the model
- External links (http://, https://, etc.) are allowed and not validated

**Validation Behavior:**
- Parse relation targets from the markdown
- Normalize relation targets to be relative to the git repository root
- Validate that each relation target element exists in the model
- Reject the operation if any relation target does not exist
- Provide clear error messages indicating which relation target was not found

#### Metadata
  * type: specification
---

### Rename Element Operation Contract Specification

#### Details
When renaming an element, the system is expected to:
- Accept the current element name and the new element name
- Validate that the current element exists in the model registry
- Validate that the new name is globally unique in the model registry
- Update the element's heading text in the markdown file
- Update all relation references (both forward and backward) to use the new element identifier
- Update the element identifier in the registry
- Provide updates report following Diff Output Format Specification

The system is expected to reject the operation with a clear error message if:
- The element does not exist
- The new name conflicts with an existing element

#### Metadata
  * type: specification
---

