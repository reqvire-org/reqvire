# Elements

### Create Element Operation

The system shall provide the capability to create new model elements by accepting a full element definition string in Markdown format, validating the element structure and relations, and inserting it into the target file following Element Ordering Behavior.

#### Details
When creating a new element, the system shall:
- Accept a string containing the full element definition in Markdown format (including ### header, metadata, relations, and content)
- Accept target location: file path
- Validate the target location using path validation rules
- Create target file if it does not exist (subject to validation constraints)
- Parse the element definition string to extract element structure, preserving all subsections:
  - Metadata (element type and custom properties)
  - Relations (derivedFrom, verifiedBy, satisfiedBy, verify)
  - Details (refinement details and nested content)
  - Attachments (links to Refinement elements and files)
- Validate the element structure (proper subsections, valid relations, correct format)
- Verify the element name is globally unique in the model
- Generate a unique element identifier based on file path and element name
- **Validate and normalize all relations in the element:**
  - Parse relation targets from the markdown (may be relative paths or repo-relative paths)
  - Normalize relation targets to be relative to the git repository root
  - Validate that each relation target element exists in the model
  - Reject the operation if any relation target does not exist
  - Provide clear error messages indicating which relation target was not found
- If validation passes, insert the element into the target file following Element Ordering Behavior
- If validation fails, reject the operation and report validation errors
- Maintain file structure and formatting after insertion

**Relation Validation Rules:**
- Relation targets may be specified as:
  - Relative paths from the target file location (e.g., `../UserReqs.md#requirement`)
  - Paths relative to git repository root (e.g., `specifications/UserReqs.md#requirement`)
  - Same-file references (e.g., `#other-requirement`)
- All relation targets must be normalized to git repository root relative format before insertion
- All relation targets must reference existing elements in the model
- External links (http://, https://, etc.) are allowed and not validated

#### Attachments
  * [File Persistence Behavior](Refinements.md#file-persistence-behavior)
  * [Target Location Constraint](Refinements.md#target-location-constraint)
  * [Element Ordering Behavior](Refinements.md#element-ordering-behavior)

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * derivedFrom: [Reserved Subsections Support](../Core/StructureAndParsing.md#reserved-subsections-support)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [diff.rs](../../../core/src/diff.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * satisfiedBy: [utils.rs](../../../core/src/utils.rs)
  * verifiedBy: [Create Element Test](Verifications/ElementManipulationVerifications.md#create-element-test)
---

### Delete Element Operation

The system shall provide the capability to delete existing model elements while automatically removing or updating all relations that reference the deleted element, and removing empty files when no elements remain.

#### Details
When deleting an element, the system shall:
- Remove the element and all its content from the source file
- Identify all relations pointing to the deleted element (incoming relations)
- Remove all relations that reference the deleted element from other elements
- Identify all relations from the deleted element (outgoing relations)
- Remove the complete element section including separators
- Maintain file structure and formatting after deletion
- Provide a report of all relations that were affected by the deletion

**Empty File Cleanup:**
- After deleting the element, check if the source file contains any remaining elements
- If no elements remain and all sections are empty (only page content, headers, or whitespace), remove the file from the filesystem
- If the file is removed, report the file deletion in the operation output

**Relation Handling:**
- All `derivedFrom` relations pointing to the deleted element shall be removed
- All `verifiedBy` relations pointing to the deleted element shall be removed
- All `verify` relations pointing to the deleted element shall be removed
- All `satisfiedBy` relations pointing to the deleted element shall be removed
- Relations from the deleted element are automatically removed with the element

#### Attachments
  * [File Persistence Behavior](Refinements.md#file-persistence-behavior)

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [diff.rs](../../../core/src/diff.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [Delete Element Test](Verifications/ElementManipulationVerifications.md#delete-element-test)
---

### Element Manipulation File Persistence

The system shall persist all element manipulation operations to the source files in storage, synchronizing changes from the in-memory model to the file system and reordering elements following the Element Ordering Behavior.

#### Attachments
  * [File Persistence Behavior](Refinements.md#file-persistence-behavior)
  * [Element Ordering Behavior](Refinements.md#element-ordering-behavior)

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [File Persistence Test](Verifications/ElementManipulationVerifications.md#file-persistence-test)
  * verifiedBy: [Element Ordering Verification](Verifications/FormattingVerifications.md#element-ordering-verification)
---

### Move Element Operation

The system shall provide the capability to move existing model elements to different file locations while automatically updating all relations that reference the moved element, creating target files if needed, and removing empty source files when no elements remain.

#### Details
When moving an element, the system shall:
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
- Provide a report of all relations that were updated

**Empty Source File Cleanup:**
- After moving the element, check if the source file contains any remaining elements
- If no elements remain (only page content, headers, or whitespace), remove the source file from the filesystem
- If the file is removed, report the file deletion in the operation output

**Relation Update Requirements:**
- All relations (both forward and backward) pointing to the moved element shall be updated to the new identifier
- Relations within the moved element (outgoing relations) shall be preserved unchanged

**Identifier Update:**
- The element's identifier changes from `<old-file>#<element-name>` to `<new-file>#<element-name>`
- All references to the old identifier shall be updated to the new identifier

#### Attachments
  * [File Persistence Behavior](Refinements.md#file-persistence-behavior)
  * [Target Location Constraint](Refinements.md#target-location-constraint)
  * [Element Ordering Behavior](Refinements.md#element-ordering-behavior)

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [diff.rs](../../../core/src/diff.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [Move Element Test](Verifications/ElementManipulationVerifications.md#move-element-test)
---

### Move File Operation

The system shall provide the capability to move entire specification files with all their elements to a new location in the repository while updating all relation references throughout the model.

#### Details
When moving a file, the system shall:
- Accept source file path (relative to git repository root)
- Accept target file path (relative to git repository root)
- Accept optional squashing flag
- Validate both source and target paths
- Move the physical file from source to target location
- Update all element identifiers within the file to reflect the new file path
- Update all relation references (both forward and backward) throughout the model that point to any element in the moved file
- Preserve all file content, structure, and formatting

The system shall reject the operation with a clear error message if:
- The source file does not exist
- The target file already exists (unless --squash flag is provided)
- The source or target paths fail validation

**Squash Mode Behavior:**
When the --squash flag is provided and the target file already exists, the system shall:
- Move all elements from the source file to the target end of file
- Remove the source file after all elements have been successfully moved
- Preserve element ordering from the source file when inserting into target section

#### Attachments
  * [File Persistence Behavior](Refinements.md#file-persistence-behavior)
  * [Target Location Constraint](Refinements.md#target-location-constraint)

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * verifiedBy: [Move File Squash Test](Verifications/ElementManipulationVerifications.md#move-file-squash-test)
---

### Relation Consistency Maintenance

The system shall maintain bidirectional relation consistency when elements are manipulated, ensuring that forward and backward relations remain synchronized.

#### Details
When manipulating elements, the system shall ensure:
- If element A derives from element B, then B must have a derive relation to A
- If element A is verified by verification V, then V must have a verify relation to A
- When an element is deleted, both forward and backward relations are removed
- When an element is moved, both forward and backward relations are updated
- After any manipulation operation, the model remains in a valid state with no dangling relations

**Validation:**
- The system shall validate relation consistency after each manipulation operation
- The system shall report any inconsistencies detected during manipulation
- The system shall prevent operations that would leave the model in an inconsistent state

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [Relation Consistency Test](Verifications/ElementManipulationVerifications.md#relation-consistency-test)
---

### Rename Element Operation

The system shall provide the capability to rename existing model elements by changing their heading text while updating all relation references and the registry.

#### Details
When renaming an element, the system shall:
- Accept the current element name and the new element name
- Validate that the current element exists in the model registry
- Validate that the new name is globally unique in the model registry
- Update the element's heading text in the markdown file
- Update all relation references (both forward and backward) to use the new element identifier
- Update the element identifier in the registry

The system shall reject the operation with a clear error message if:
- The element does not exist
- The new name conflicts with an existing element

#### Attachments
  * [File Persistence Behavior](Refinements.md#file-persistence-behavior)

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [diff.rs](../../../core/src/diff.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
---

### Target Location Validation and Auto-Creation

The system shall validate target file paths for element manipulation operations and automatically create files when they do not exist, subject to path safety constraints.

#### Details
The system shall define target location validation constraints.

#### Relations
  * derivedFrom: [Ignore Files Integration](../Core/Configuration.md#ignore-files-integration)
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * derivedFrom: [Git Repository as Project Root](../Core/ModelManagement.md#git-repository-as-project-root)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [utils.rs](../../../core/src/utils.rs)
  * satisfiedBy: [Target Location Constraint](Refinements.md#target-location-constraint)
  * verifiedBy: [Target Location Validation Test](Verifications/ElementManipulationVerifications.md#target-location-validation-test)
---
