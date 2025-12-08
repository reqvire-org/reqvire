# Elements

### Create Element Operation

The system shall provide the capability to create new model elements by accepting a full element definition string in Markdown format, validating the element structure and relations, and inserting it into the target file.

#### Details
When creating a new element, the system shall:
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
- The system shall support override mode to replace existing element with same name following rules defined in Create Element Override Behavior

#### Metadata
  * type: requirement

#### Attachments
  * [File Persistence Behavior](Behaviors.md#file-persistence-behavior)
  * [Target Location Constraint](Constraints.md#target-location-constraint)
  * [Element Ordering Behavior](Behaviors.md#element-ordering-behavior)
  * [Dry-Run Mode Behavior](Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](../Output/Specifications.md#diff-output-format-specification)

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * derivedFrom: [Reserved Subsections Support](../Core/StructureAndParsing.md#reserved-subsections-support)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [diff.rs](../../../core/src/diff.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * satisfiedBy: [utils.rs](../../../core/src/utils.rs)
  * satisfiedBy: [Create Element Override Behavior](Behaviors.md#create-element-override-behavior)
  * satisfiedBy: [Relation Validation Specification](Specifications.md#relation-validation-specification)
  * verifiedBy: [Create Element Test](Verifications/ElementManipulationVerifications.md#create-element-test)
---

### Delete Element Operation

The system shall provide the capability to delete existing model elements while automatically removing or updating all relations that reference the deleted element, and removing empty files when no elements remain.

#### Details
When deleting an element, the system shall:
- Check if any child elements would become orphaned (have no remaining parent hierarchical relations after deletion)
- Reject the operation if any child would become orphaned
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
- All `derivedFrom` relations pointing to the deleted element shall be removed
- All `verifiedBy` relations pointing to the deleted element shall be removed
- All `verify` relations pointing to the deleted element shall be removed
- All `satisfiedBy` relations pointing to the deleted element shall be removed
- Relations from the deleted element are automatically removed with the element

#### Metadata
  * type: requirement

#### Attachments
  * [File Persistence Behavior](Behaviors.md#file-persistence-behavior)
  * [Dry-Run Mode Behavior](Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](../Output/Specifications.md#diff-output-format-specification)

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

#### Metadata
  * type: requirement

#### Attachments
  * [File Persistence Behavior](Behaviors.md#file-persistence-behavior)
  * [Element Ordering Behavior](Behaviors.md#element-ordering-behavior)

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [File Persistence Test](Verifications/ElementManipulationVerifications.md#file-persistence-test)
  * verifiedBy: [Element Ordering Verification](Verifications/FormattingVerifications.md#element-ordering-verification)
---

### Merge Element Operation

The system shall provide the capability to merge multiple source elements into a target element, consolidating content, relations, and attachments while enforcing type compatibility and removing source elements after successful merge.

#### Details
When merging elements, the system shall:
- Accept target element name (must exist in the model)
- Accept one or more source element names (must exist in the model)
- Validate type compatibility following clearly defined rules in Merge Type Compatibility Constraint
- Transform and merge content following clearly defined rules in Merge Content Transformation Behavior
- Preserve target element's metadata (discard source metadata)
- Delete source elements after successful merge
- Update all relations pointing to source elements to point to target
- Remove empty source files when no elements remain
- Provide updates report following Diff Output Format Specification

The system shall reject the operation with a clear error message if:
- The target element does not exist
- Any source element does not exist
- Source and target element types are incompatible per Merge Type Compatibility Constraint
- Merged result would have cross-section duplicates per Merge Content Transformation Behavior

#### Metadata
  * type: requirement

#### Attachments
  * [File Persistence Behavior](Behaviors.md#file-persistence-behavior)
  * [Dry-Run Mode Behavior](Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](../Output/Specifications.md#diff-output-format-specification)

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [diff.rs](../../../core/src/diff.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [Merge Content Transformation Behavior](Behaviors.md#merge-content-transformation-behavior)
  * satisfiedBy: [Merge Type Compatibility Constraint](Constraints.md#merge-type-compatibility-constraint)
  * verifiedBy: [Merge Elements Test](Verifications/ElementManipulationVerifications.md#merge-elements-test)
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
- Provide updates report following Diff Output Format Specification

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

#### Metadata
  * type: requirement

#### Attachments
  * [File Persistence Behavior](Behaviors.md#file-persistence-behavior)
  * [Target Location Constraint](Constraints.md#target-location-constraint)
  * [Element Ordering Behavior](Behaviors.md#element-ordering-behavior)
  * [Dry-Run Mode Behavior](Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](../Output/Specifications.md#diff-output-format-specification)

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
- Provide updates report following Diff Output Format Specification

The system shall reject the operation with a clear error message if:
- The source file does not exist
- The target file already exists (unless --squash flag is provided)
- The source or target paths fail validation

**Squash Mode Behavior:**
When the --squash flag is provided and the target file already exists, the system shall:
- Move all elements from the source file to the target end of file
- Remove the source file after all elements have been successfully moved
- Preserve element ordering from the source file when inserting into target section

#### Metadata
  * type: requirement

#### Attachments
  * [File Persistence Behavior](Behaviors.md#file-persistence-behavior)
  * [Target Location Constraint](Constraints.md#target-location-constraint)
  * [Dry-Run Mode Behavior](Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](../Output/Specifications.md#diff-output-format-specification)

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

#### Metadata
  * type: requirement

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
- Provide updates report following Diff Output Format Specification

The system shall reject the operation with a clear error message if:
- The element does not exist
- The new name conflicts with an existing element

#### Metadata
  * type: requirement

#### Attachments
  * [File Persistence Behavior](Behaviors.md#file-persistence-behavior)
  * [Dry-Run Mode Behavior](Behaviors.md#dry-run-mode-behavior)
  * [Diff Output Format Specification](../Output/Specifications.md#diff-output-format-specification)

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

#### Metadata
  * type: requirement

#### Attachments
  * [Git Repository Scope Specification](../Core/Specifications.md#git-repository-scope-specification)

#### Relations
  * derivedFrom: [Ignore Files Integration](../Core/Configuration.md#ignore-files-integration)
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * derivedFrom: [Git Repository as Project Root](../Core/ModelManagement.md#git-repository-as-project-root)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [utils.rs](../../../core/src/utils.rs)
  * satisfiedBy: [Target Location Constraint](Constraints.md#target-location-constraint)
  * verifiedBy: [Target Location Validation Test](Verifications/ElementManipulationVerifications.md#target-location-validation-test)
---
