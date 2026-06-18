# Elements

### Atomic Relation Relink Operation

The system shall provide an atomic relation relink operation that rewires an existing relation target to a new target while preserving model validity.

#### Details
For hierarchical relinks (`derivedFrom`/`derive`), the operation shall support subgraph boundary relinking semantics, applying changes as one transaction and validating the resulting model state before persistence.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Atomic Relink Validity Constraint](Constraints.md#atomic-relink-validity-constraint)
  * definedBy: [Atomic Relation Relink Workflow Specification](Specifications.md#atomic-relation-relink-workflow-specification)
  * derivedFrom: [Element Manipulation Operations](../../ModelStructure/ModelManagement.md#element-manipulation-operations)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [Atomic Relation Relink Test](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#atomic-relation-relink-test)
  * verifiedBy: [Verification Objective Mutation Test](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#verification-objective-mutation-test)
---

### CRUD Semantic Contract Mutation Validation

The system shall validate semantic-contract reference integrity before persisting graph-backed CRUD mutations.

#### Details
When a CRUD mutation changes model elements or relations, the system shall reject the candidate model before persistence if any semantic-contract Shapes reference becomes undeclared by all ontology elements or is declared only outside the contract's explicit ontology-use context.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [CRUD Semantic Contract Validation Specification](Specifications.md#crud-semantic-contract-validation-specification)
  * derivedFrom: [Element Manipulation Operations](../../ModelStructure/ModelManagement.md#element-manipulation-operations)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [Delete Element Test](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#delete-element-test)
---

### Create Element Operation

The system shall provide the capability to create new model elements by accepting a full element definition string in Markdown format, validating the element structure and relations, and inserting it into the target file.

#### Metadata
  * type: requirement

#### Attachments
  * [Target Location Constraint](Constraints.md#target-location-constraint)
  * [Element Ordering Behavior](../Formatting/Behaviors.md#element-ordering-behavior)
  * [Attachment Hierarchical Independence Constraint](../../ModelStructure/Constraints.md#attachment-hierarchical-independence-constraint)
  * [Attachment Satisfied Refinement Constraint](../../ModelStructure/Constraints.md#attachment-satisfied-contract-constraint)
  * [Element Type Metadata Specification](../../ModelStructure/Specifications.md#element-type-metadata-specification)

#### Relations
  * definedBy: [Create Element Override Behavior](Behaviors.md#create-element-override-behavior)
  * definedBy: [Create Element Workflow Specification](Specifications.md#create-element-workflow-specification)
  * definedBy: [Relation Validation Specification](Specifications.md#relation-validation-specification)
  * derivedFrom: [Element Manipulation Operations](../../ModelStructure/ModelManagement.md#element-manipulation-operations)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [diff.rs](../../../core/src/diff.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * satisfiedBy: [utils.rs](../../../core/src/utils.rs)
  * verifiedBy: [Create Element Test](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#create-element-test)
---

### Delete Element Operation

The system shall provide the capability to delete existing model elements while automatically removing or updating all relations that reference the deleted element, and removing empty files when no elements remain.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Delete Element Workflow Specification](Specifications.md#delete-element-workflow-specification)
  * definedBy: [Orphaned Children Error Message Specification](Specifications.md#orphaned-children-error-message-specification)
  * derivedFrom: [Element Manipulation Operations](../../ModelStructure/ModelManagement.md#element-manipulation-operations)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [diff.rs](../../../core/src/diff.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [Delete Element Test](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#delete-element-test)
---

### Element Manipulation File Persistence

The system shall persist all element manipulation operations to the source files in storage, synchronizing changes from the in-memory model to the file system and reordering elements following the Element Ordering Behavior.

#### Metadata
  * type: requirement

#### Attachments
  * [Element Ordering Behavior](../Formatting/Behaviors.md#element-ordering-behavior)

#### Relations
  * definedBy: [Element Manipulation File Persistence Refinement Specification](Specifications.md#element-manipulation-file-persistence-refinement-specification)
  * derivedFrom: [Element Manipulation Operations](../../ModelStructure/ModelManagement.md#element-manipulation-operations)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [File Persistence Test](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#file-persistence-test)
---

### Merge Element Operation

The system shall provide the capability to merge multiple source elements into a target element, consolidating content, relations, and attachments while enforcing type compatibility and removing source elements after successful merge.

#### Details
When the merge target is a `# Element` model file element, the operation shall preserve the `# Element` file format and keep the result as a single-element file.
The operation shall reject merges where a source element is in a `# Element` file and the target element is in a `# Elements` file. That conversion can violate `# Elements` parsing constraints and shall be performed manually.
When the target is an ontology element, the operation shall preserve the target ontology metadata, rewrite each source ontology block to the target ontology boundary, and fold all rewritten authored Turtle into the target's single `#### Ontology` block. Ontology merges shall preserve a valid dependency chain by recalculating any affected inherited prefix bindings, document declarations, `owl:imports`, and reachable SHACL references before persistence.

#### Metadata
  * type: requirement

#### Attachments
  * [Attachment Hierarchical Independence Constraint](../../ModelStructure/Constraints.md#attachment-hierarchical-independence-constraint)
  * [Attachment Satisfied Refinement Constraint](../../ModelStructure/Constraints.md#attachment-satisfied-contract-constraint)

#### Relations
  * definedBy: [Merge Content Transformation Behavior](Behaviors.md#merge-content-transformation-behavior)
  * definedBy: [Merge Type Compatibility Constraint](Constraints.md#merge-type-compatibility-constraint)
  * definedBy: [Merge Element Workflow Specification](Specifications.md#merge-element-workflow-specification)
  * derivedFrom: [Element Manipulation Operations](../../ModelStructure/ModelManagement.md#element-manipulation-operations)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [diff.rs](../../../core/src/diff.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [Merge Elements Test](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#merge-elements-test)
  * verifiedBy: [Verification Objective Mutation Test](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#verification-objective-mutation-test)
---

### Move Element Operation

The system shall provide the capability to move existing model elements to different file locations while automatically updating all relations that reference the moved element, creating target files if needed, and removing empty source files when no elements remain.

#### Details
The operation shall reject moves into an existing `# Element` file when that move would introduce an additional element. `# Element` files are single-element model files.

#### Metadata
  * type: requirement

#### Attachments
  * [Target Location Constraint](Constraints.md#target-location-constraint)
  * [Element Ordering Behavior](../Formatting/Behaviors.md#element-ordering-behavior)

#### Relations
  * definedBy: [Move Element Workflow Specification](Specifications.md#move-element-workflow-specification)
  * derivedFrom: [Element Manipulation Operations](../../ModelStructure/ModelManagement.md#element-manipulation-operations)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [diff.rs](../../../core/src/diff.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [Move Element Test](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#move-element-test)
  * verifiedBy: [Verification Objective Mutation Test](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#verification-objective-mutation-test)
---

### Move File Operation

The system shall provide the capability to move entire specification files with all their elements to a new location in the repository while updating all relation references throughout the model.

#### Details
When `--squash` is requested, the operation shall reject squashing into an existing `# Element` target file. `# Element` files are single-element model files and cannot accept squashed multi-element content.

#### Metadata
  * type: requirement

#### Attachments
  * [Target Location Constraint](Constraints.md#target-location-constraint)

#### Relations
  * definedBy: [Move File Operation Refinement Specification](Specifications.md#move-file-operation-refinement-specification)
  * derivedFrom: [Element Manipulation Operations](../../ModelStructure/ModelManagement.md#element-manipulation-operations)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [diff.rs](../../../core/src/diff.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [Move File Squash Test](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#move-file-squash-test)
---

### Relation Consistency Maintenance

The system shall maintain bidirectional relation consistency when elements are manipulated, ensuring that forward and backward relations remain synchronized.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Relation Consistency Maintenance Refinement Specification](Specifications.md#relation-consistency-maintenance-refinement-specification)
  * derivedFrom: [Element Manipulation Operations](../../ModelStructure/ModelManagement.md#element-manipulation-operations)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [Relation Consistency Test](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#relation-consistency-test)
---

### Rename Element Operation

The system shall provide the capability to rename existing model elements by changing their heading text while updating all relation references and the registry.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Rename Element Operation Refinement Specification](Specifications.md#rename-element-operation-refinement-specification)
  * derivedFrom: [Element Manipulation Operations](../../ModelStructure/ModelManagement.md#element-manipulation-operations)
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
  * [Ignore Files Specification](../../ModelStructure/Specifications.md#ignore-files-specification)
  * [Git Repository Scope Specification](../../ModelStructure/Specifications.md#git-repository-scope-specification)

#### Relations
  * definedBy: [Target Location Constraint](Constraints.md#target-location-constraint)
  * derivedFrom: [Element Manipulation Operations](../../ModelStructure/ModelManagement.md#element-manipulation-operations)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [utils.rs](../../../core/src/utils.rs)
  * verifiedBy: [Target Location Validation Test](../../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#target-location-validation-test)
---
