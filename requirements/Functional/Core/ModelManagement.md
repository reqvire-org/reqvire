# Elements

### Default Requirement Type Assignment

The system shall automatically assign the **default type `requirement`** to all elements if not explicitly specified in their `metadata` subsection.

#### Details
<details>
<summary>Type Assignment Rules</summary>

When an element does not have a `#### Metadata` subsection with a `type` property, the system assigns the default type `requirement`.

**This behavior is location-independent:** All elements default to type `requirement` regardless of their folder location within the Git repository.

**To use other element types**, users must explicitly specify the type in the element's Metadata subsection:
```markdown
#### Metadata
  * type: user-requirement
```

**Supported element types:**
- `requirement` (default)
- `user-requirement`
- `verification` / `test-verification`
- `analysis-verification`
- `inspection-verification`
- `demonstration-verification`
- `constraint` (refinement type)
- `behavior` (refinement type)
- `specification` (refinement type)
- `other`

</details>

#### Metadata
  * type: user-requirement

#### Attachments
  * [Element Type Metadata Specification](Specifications.md#element-type-metadata-specification)

#### Relations
  * derivedFrom: [Operating on Model Elements](../../UserStories.md#operating-on-model-elements)
  * verifiedBy: [Element Subsection Parsing Test](Verifications/ParsingVerifications.md#element-subsection-parsing-test)
  * verifiedBy: [Default Element Type Assignment Test](Verifications/ValidationVerifications.md#default-element-type-assignment-test)
---

### Efficient Processing

The system shall process structured documents and relations to extract model-relevant information efficiently.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Operating on Model Elements](../../UserStories.md#operating-on-model-elements)
  * derivedFrom: [Promote Automation and Efficiency](../../UserStories.md#promote-automation-and-efficiency)
---

### Element Manipulation Operations

The system shall provide programmatic manipulation of model elements through operations including, but not limited to, creating new elements, deleting existing elements, moving elements between locations, and renaming elements while maintaining model integrity and traceability.

#### Details
All manipulation operations shall:
- Maintain model integrity and consistency
- Update or remove affected relations automatically
- Preserve traceability where appropriate

#### Metadata
  * type: user-requirement

#### Attachments
  * [Diff Output Format Specification](../Output/Specifications.md#diff-output-format-specification)

#### Relations
  * derive: [Attachment Identifier Updates](#attachment-identifier-updates)
  * derive: [Create Element Operation](../Operations/ElementManipulation.md#create-element-operation)
  * derive: [Delete Element Operation](../Operations/ElementManipulation.md#delete-element-operation)
  * derive: [Element Manipulation File Persistence](../Operations/ElementManipulation.md#element-manipulation-file-persistence)
  * derive: [Move Element Operation](../Operations/ElementManipulation.md#move-element-operation)
  * derive: [Move File Operation](../Operations/ElementManipulation.md#move-file-operation)
  * derive: [Relation Consistency Maintenance](../Operations/ElementManipulation.md#relation-consistency-maintenance)
  * derive: [Rename Element Operation](../Operations/ElementManipulation.md#rename-element-operation)
  * derive: [Target Location Validation and Auto-Creation](../Operations/ElementManipulation.md#target-location-validation-and-auto-creation)
  * derivedFrom: [Operating on Model Elements](../../UserStories.md#operating-on-model-elements)
  * refinedBy: [Dry-Run Mode Behavior](../Operations/Behaviors.md#dry-run-mode-behavior)
  * refinedBy: [File Persistence Behavior](../Operations/Behaviors.md#file-persistence-behavior)
---

### Attachment Identifier Updates

When moving or renaming Refinement elements, the system shall update attachment identifiers in all referencing elements, using the same behavior as relation target updates.

#### Details
This requirement ensures consistency between relation updates and attachment identifier updates during CRUD operations:

- When a Refinement element is **moved** to a different file, all attachments referencing that element shall be updated with the new identifier path
- When a Refinement element is **renamed**, all attachments referencing that element shall be updated with the new element name in the identifier
- The update behavior mirrors how relation targets are updated during move/rename operations
- All files containing attachments that reference the affected element shall be modified

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Element Manipulation Operations](#element-manipulation-operations)
  * refinedBy: [Attachment Identifier CRUD Update Behavior](Behaviors.md#attachment-identifier-crud-update-behavior)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * verifiedBy: [Attachment Identifier CRUD Verification](Verifications/AttachmentsVerifications.md#attachment-identifier-crud-verification)
---

### Git Repository as Project Root

The system shall use the Git repository root as the project base for path resolution and scope management.

#### Details
- The system shall treat the root directory of the Git repository as the project's base for all file and folder references
- The system shall implement path resolution following clearly defined specifications
- When run from git root, the system shall process all files with paths resolved relative to git root
- When run from a subdirectory, the system shall automatically detect the subdirectory context and limit processing scope
- When run from a subdirectory, the system shall validate that all references stay within scope boundaries following clearly defined specifications

#### Metadata
  * type: user-requirement

#### Attachments
  * [Containment Specification](../../Refinements.md#containment-specification)

#### Relations
  * derive: [Target Location Validation and Auto-Creation](../Operations/ElementManipulation.md#target-location-validation-and-auto-creation)
  * derivedFrom: [Defining Model Structure](../../UserStories.md#defining-model-structure)
  * refinedBy: [Subdirectory Auto-Detection Behavior](Behaviors.md#subdirectory-auto-detection-behavior)
  * refinedBy: [Git Repository Scope Specification](Specifications.md#git-repository-scope-specification)
  * satisfiedBy: [git_commands.rs](../../../core/src/git_commands.rs)
  * verifiedBy: [Subdirectory Processing Verification](Verifications/ValidationVerifications.md#subdirectory-processing-verification)
---

### Relation Types and behaviors

The system shall implement relations following clearly defined specifications for types and behaviors.

#### Metadata
  * type: requirement

#### Attachments
  * [RelationTypes.md](DesignDocuments/RelationTypes.md)
  * [Relation Semantics Specification](../../Refinements.md#relation-semantics-specification)

#### Relations
  * derive: [Element Type Relation Compatibility](#element-type-relation-compatibility)
  * derive: [Relation Management Operations](#relation-management-operations)
  * derive: [Validate Relation Types](Validation.md#validate-relation-types)
  * derive: [Trace Relation Non-Directional Behavior](../Output/DiagramGeneration.md#trace-relation-non-directional-behavior)
  * derivedFrom: [Identifiers and Relations](StructureAndParsing.md#identifiers-and-relations)
  * satisfiedBy: [relation.rs](../../../core/src/relation.rs)
  * verifiedBy: [Element Subsection Parsing Test](Verifications/ParsingVerifications.md#element-subsection-parsing-test)
---

### Element Type Relation Compatibility

The system shall enforce element type constraints for relation types, ensuring that only valid combinations of source and target element types are allowed for each relation type.

#### Details
The system shall define element type relation compatibility constraints.

#### Metadata
  * type: requirement

#### Attachments
  * [Supported Element Types Specification](../../Refinements.md#supported-element-types-specification)

#### Relations
  * derive: [Relation Element Type Validator](Validation.md#relation-element-type-validator)
  * derivedFrom: [Relation Types and behaviors](#relation-types-and-behaviors)
  * refinedBy: [Element Type Relation Compatibility Constraint](Constraints.md#element-type-relation-compatibility-constraint)
  * verifiedBy: [Element Type Relation Compatibility Test](Verifications/ValidationVerifications.md#element-type-relation-compatibility-test)
---

### Refinement Element Structure Constraints

The system shall restrict Refinement elements (constraint, behavior, specification) to only allow `refine` relations.

#### Details
Refinement elements serve as detailed documentation that augments requirements and drives implementation. Their relation usage is restricted because:
- They represent atomic pieces of information focused on documenting requirements
- They are primarily referenced through the Attachments subsection of other elements
- Their `refine` relation links back to the requirement they refine, establishing ownership
- Each refinement can only be owned by one requirement (uniqueness constraint)

When a Refinement element contains relations other than `refine`, the validator shall report an error indicating that only `refine` relations are allowed for refinement types.

#### Metadata
  * type: requirement

#### Attachments
  * [Supported Element Types Specification](../../Refinements.md#supported-element-types-specification)

#### Relations
  * derivedFrom: [Relation Types and behaviors](#relation-types-and-behaviors)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * verifiedBy: [Refinement Relations Rejection Test](Verifications/ParsingVerifications.md#refinement-relations-rejection-test)
---

### Attachment Scope Constraints

The system shall enforce scope constraints on attachments (both refinement elements and file assets) to ensure proper cross-submodel traceability while preventing redundant or invalid attachment relationships.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Refinement Element Structure Constraints](#refinement-element-structure-constraints)
  * refinedBy: [Attachment Hierarchical Independence Constraint](Constraints.md#attachment-hierarchical-independence-constraint)
  * refinedBy: [Attachment Satisfied Refinement Constraint](Constraints.md#attachment-satisfied-refinement-constraint)
  * verifiedBy: [Attachment Scope Constraints Test](Verifications/AttachmentsVerifications.md#attachment-scope-constraints-test)
---

### Relation Management Operations

The system shall provide programmatic manipulation of element relations through link and unlink operations while maintaining model integrity.

#### Details
When linking, the system shall:
- Add a relation entry to the element's Relations subsection
- Create the Relations subsection if it does not exist
- Validate the relation type and element type compatibility
- Skip if relation already exists (idempotent operation)

When unlinking, the system shall:
- Remove the relation entry from the element's Relations subsection
- Remove the Relations subsection if no relations remain

#### Metadata
  * type: requirement

#### Attachments
  * [Attachment Hierarchical Independence Constraint](Constraints.md#attachment-hierarchical-independence-constraint)
  * [Attachment Satisfied Refinement Constraint](Constraints.md#attachment-satisfied-refinement-constraint)

#### Relations
  * derivedFrom: [Relation Types and behaviors](#relation-types-and-behaviors)
  * refinedBy: [Relation Operations Specification](../Operations/Specifications.md#relation-operations-specification)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [Link Command Verification](../Operations/Verifications/ElementManipulationVerifications.md#link-command-verification)
  * verifiedBy: [Unlink Command Verification](../Operations/Verifications/ElementManipulationVerifications.md#unlink-command-verification)
---

### Template-Based Model Bootstrapping

The system shall enable systems engineers to quickly bootstrap new System models from predefined templates stored in Git repositories, accelerating project initialization and promoting best-practice model structures.

#### Details
Template Bootstrapping Capabilities

Users can initialize new models using the CLI with templates from Git repositories:
- Discover available templates within a specified repository
- Select and apply templates interactively
- Bootstrap model structure with predefined files, folders, and requirements

Templates are consumed from Git repositories only, with support for repositories containing multiple templates alongside other content.

**Example usage:**
```bash
reqvire init --template <github-repo-url>
```

The system discovers all available templates in the repository and allows the user to select which template to apply.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Operating on Model Elements](../../UserStories.md#operating-on-model-elements)
---

### Verification Type Categories

The system shall support defined verification categories following clearly defined specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Supported Element Types Specification](../../Refinements.md#supported-element-types-specification)

#### Relations
  * refinedBy: [Verification Type Selection Guidelines](Specifications.md#verification-type-selection-guidelines)
---
