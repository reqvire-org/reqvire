# Elements

### Create Element Operation

The system shall provide the capability to create new model elements by accepting a full element definition string in Markdown format, validating the element structure and relations, and inserting it into the target file.

#### Metadata
  * type: requirement

#### Attachments
  * [Target Location Constraint](Constraints.md#target-location-constraint)
  * [Element Ordering Behavior](Behaviors.md#element-ordering-behavior)
  * [Attachment Hierarchical Independence Constraint](../Core/Constraints.md#attachment-hierarchical-independence-constraint)
  * [Attachment Satisfied Refinement Constraint](../Core/Constraints.md#attachment-satisfied-refinement-constraint)

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * derivedFrom: [Reserved Subsections Support](../Core/StructureAndParsing.md#reserved-subsections-support)
  * refinedBy: [Create Element Override Behavior](Behaviors.md#create-element-override-behavior)
  * refinedBy: [Create Element Workflow Specification](Specifications.md#create-element-workflow-specification)
  * refinedBy: [Relation Validation Specification](Specifications.md#relation-validation-specification)
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

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * refinedBy: [Delete Element Workflow Specification](Specifications.md#delete-element-workflow-specification)
  * refinedBy: [Orphaned Children Error Message Specification](Specifications.md#orphaned-children-error-message-specification)
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

#### Metadata
  * type: requirement

#### Attachments
  * [Attachment Hierarchical Independence Constraint](../Core/Constraints.md#attachment-hierarchical-independence-constraint)
  * [Attachment Satisfied Refinement Constraint](../Core/Constraints.md#attachment-satisfied-refinement-constraint)

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * refinedBy: [Merge Content Transformation Behavior](Behaviors.md#merge-content-transformation-behavior)
  * refinedBy: [Merge Type Compatibility Constraint](Constraints.md#merge-type-compatibility-constraint)
  * refinedBy: [Merge Element Workflow Specification](Specifications.md#merge-element-workflow-specification)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [diff.rs](../../../core/src/diff.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [Merge Elements Test](Verifications/ElementManipulationVerifications.md#merge-elements-test)
---

### Move Element Operation

The system shall provide the capability to move existing model elements to different file locations while automatically updating all relations that reference the moved element, creating target files if needed, and removing empty source files when no elements remain.

#### Metadata
  * type: requirement

#### Attachments
  * [Target Location Constraint](Constraints.md#target-location-constraint)
  * [Element Ordering Behavior](Behaviors.md#element-ordering-behavior)

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * refinedBy: [Move Element Workflow Specification](Specifications.md#move-element-workflow-specification)
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
  * [Target Location Constraint](Constraints.md#target-location-constraint)

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

#### Relations
  * derivedFrom: [Ignore Files Integration](../Core/Configuration.md#ignore-files-integration)
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * derivedFrom: [Git Repository as Project Root](../Core/ModelManagement.md#git-repository-as-project-root)
  * refinedBy: [Target Location Constraint](Constraints.md#target-location-constraint)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [utils.rs](../../../core/src/utils.rs)
  * verifiedBy: [Target Location Validation Test](Verifications/ElementManipulationVerifications.md#target-location-validation-test)
---
