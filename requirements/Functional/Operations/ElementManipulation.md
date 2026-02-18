# Elements

### Atomic Relation Relink Operation

The system shall provide an atomic relation relink operation that rewires an existing relation target to a new target while preserving model validity.

#### Details
For hierarchical relinks (`derivedFrom`/`derive`), the operation shall support subgraph boundary relinking semantics, applying changes as one transaction and validating the resulting model state before persistence.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * refinedBy: [Atomic Relink Validity Constraint](Constraints.md#atomic-relink-validity-constraint)
  * refinedBy: [Atomic Relation Relink Workflow Specification](Specifications.md#atomic-relation-relink-workflow-specification)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [Atomic Relation Relink Test](Verifications/ElementManipulationVerifications.md#atomic-relation-relink-test)
---

### Create Element Operation

The system shall provide the capability to create new model elements by accepting a full element definition string in Markdown format, validating the element structure and relations, and inserting it into the target file.

#### Metadata
  * type: requirement

#### Attachments
  * [Target Location Constraint](Constraints.md#target-location-constraint)
  * [Element Ordering Behavior](Behaviors.md#element-ordering-behavior)
  * [Attachment Hierarchical Independence Constraint](../Core/Constraints.md#attachment-hierarchical-independence-constraint)
  * [Attachment Satisfied Refinement Constraint](../Core/Constraints.md#attachment-satisfied-refinement-constraint)
  * [Element Type Metadata Specification](../Core/Specifications.md#element-type-metadata-specification)

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
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
  * refinedBy: [Element Manipulation File Persistence Refinement Specification](Specifications.md#element-manipulation-file-persistence-refinement-specification)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [File Persistence Test](Verifications/ElementManipulationVerifications.md#file-persistence-test)
  * verifiedBy: [Element Ordering Verification](Verifications/FormattingVerifications.md#element-ordering-verification)
---

### Merge Element Operation

The system shall provide the capability to merge multiple source elements into a target element, consolidating content, relations, and attachments while enforcing type compatibility and removing source elements after successful merge.

#### Details
When the merge target is a `# Documents` model file element, the operation shall preserve the `# Documents` file format and keep the result as a single-element document file.
The operation shall reject merges where a source element is in a `# Documents` file and the target element is in a `# Elements` file. That conversion can violate `# Elements` parsing constraints and shall be performed manually.

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

#### Details
The operation shall reject moves into an existing `# Documents` file when that move would introduce an additional element. `# Documents` files are single-element model files.

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
When `--squash` is requested, the operation shall reject squashing into an existing `# Documents` target file. `# Documents` files are single-element model files and cannot accept squashed multi-element content.

#### Metadata
  * type: requirement

#### Attachments
  * [Target Location Constraint](Constraints.md#target-location-constraint)

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * refinedBy: [Move File Operation Refinement Specification](Specifications.md#move-file-operation-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [diff.rs](../../../core/src/diff.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [Move File Squash Test](Verifications/ElementManipulationVerifications.md#move-file-squash-test)
---

### Relation Consistency Maintenance

The system shall maintain bidirectional relation consistency when elements are manipulated, ensuring that forward and backward relations remain synchronized.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * refinedBy: [Relation Consistency Maintenance Refinement Specification](Specifications.md#relation-consistency-maintenance-refinement-specification)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [Relation Consistency Test](Verifications/ElementManipulationVerifications.md#relation-consistency-test)
---

### Rename Element Operation

The system shall provide the capability to rename existing model elements by changing their heading text while updating all relation references and the registry.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * refinedBy: [Rename Element Operation Refinement Specification](Specifications.md#rename-element-operation-refinement-specification)
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
  * [Ignore Files Specification](../Core/Specifications.md#ignore-files-specification)
  * [Git Repository Scope Specification](../Core/Specifications.md#git-repository-scope-specification)

#### Relations
  * derivedFrom: [Element Manipulation Operations](../Core/ModelManagement.md#element-manipulation-operations)
  * refinedBy: [Target Location Constraint](Constraints.md#target-location-constraint)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [utils.rs](../../../core/src/utils.rs)
  * verifiedBy: [Target Location Validation Test](Verifications/ElementManipulationVerifications.md#target-location-validation-test)
---
