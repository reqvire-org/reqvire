# Elements

### Coexistence of Structured and Unstructured Documents

The system shall allow structured markdown and unstructured. (eg., markdown, PDFs, DOCX, raw text) documents to coexist within the same System model.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Managing System Models](../../UserStories.md#managing-system-models)
---

### Relation Types and behaviors

The system shall implement relations following clearly defined specifications for types and behaviors.

#### Attachments
  * [RelationTypes.md](DesignDocuments/RelationTypes.md)

#### Relations
  * derivedFrom: [Identifiers and Relations](StructureAndParsing.md#identifiers-and-relations)
  * satisfiedBy: [relation.rs](../../../core/src/relation.rs)
---

### Element Type Relation Compatibility

The system shall enforce element type constraints for relation types, ensuring that only valid combinations of source and target element types are allowed for each relation type.

#### Details
The system shall define element type relation compatibility constraints.

#### Attachments
  * [RelationTypes.md](DesignDocuments/RelationTypes.md)

#### Relations
  * derivedFrom: [Relation Types and behaviors](#relation-types-and-behaviors)
  * derivedFrom: [Supported Element Types](#supported-element-types)
  * satisfiedBy: [Element Type Relation Compatibility Constraint](Refinements.md#element-type-relation-compatibility-constraint)
  * verifiedBy: [Element Type Relation Compatibility Test](Verifications/ValidationVerifications.md#element-type-relation-compatibility-test)
---

### Efficient Processing

The system shall process structured documents and relations to extract model-relevant information efficiently.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Managing System Models](../../UserStories.md#managing-system-models)
  * derivedFrom: [Promote Automation and Efficiency](../../UserStories.md#promote-automation-and-efficiency)
---

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

#### Relations
  * derivedFrom: [Managing System Models](../../UserStories.md#managing-system-models)
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
  * derivedFrom: [Managing System Models](../../UserStories.md#managing-system-models)
---

### Git Repository as Project Root

The system shall treat the **root directory of the Git repository as the project's base** for all file and folder references, streamlining configuration and promoting a self-contained project structure.

#### Details
All paths specified in Reqvire commands will be resolved relative to the current working directory:
- When run from the git repository root: paths are relative to the git root
- When run from a subdirectory: paths are relative to that subdirectory, and processing is limited to files within that subdirectory scope

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Managing System Models](../../UserStories.md#managing-system-models)
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

#### Relations
  * derivedFrom: [Managing System Models](../../UserStories.md#managing-system-models)
---

### Attachment Identifier Updates

When moving or renaming Refinement elements, the system shall update attachment identifiers in all referencing elements, using the same behavior as relation target updates.

#### Details
This requirement ensures consistency between relation updates and attachment identifier updates during CRUD operations:

- When a Refinement element is **moved** to a different file, all attachments referencing that element shall be updated with the new identifier path
- When a Refinement element is **renamed**, all attachments referencing that element shall be updated with the new element name in the identifier
- The update behavior mirrors how relation targets are updated during move/rename operations
- All files containing attachments that reference the affected element shall be modified

#### Attachments
  * [Attachment Identifier CRUD Update Behavior](Behaviors.md#attachment-identifier-crud-update-behavior)

#### Relations
  * derivedFrom: [Element Manipulation Operations](#element-manipulation-operations)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
---

### Verification Type Categories

The system shall support defined verifications categories.

#### Details
The following verification types are supported:

1. **Default Verification Type**
   - `verification` - Verification through testing (equivalent to `test-verification`)

2. **Specific Verification Types**
   - `test-verification` - Explicit verification through testing with documented test procedures
   - `analysis-verification` - Verification through formal analysis of documentation or code
   - `inspection-verification` - Verification through formal inspection or review
   - `demonstration-verification` - Verification through demonstration in a realistic environment

The appropriate verification type should be selected based on the nature of the requirement:
- **Test-verification**: Used when formal test procedures with expected outcomes are required
- **Analysis-verification**: Used when requirements can be verified through analysis of documentation or code
- **Inspection-verification**: Used when requirements can be verified through review of artifacts
- **Demonstration-verification**: Used when requirements can be verified by demonstrating functionality

#### Relations
  * derivedFrom: [Reserved Subsections Support](StructureAndParsing.md#reserved-subsections-support)
---

### Supported Element Types

The system shall support predefined element types for classification and behavior determination.

#### Details
Element types are identified through a reserved "type" metadata property. The following types are supported:

1. **requirement**: System requirement
2. **user-requirement**: User requirement
3. **verification**: For verification tests and validation procedures (equivalent to test-verification)
4. **test-verification**: For verification tests and validation procedures
5. **analysis-verification**: For verification through formal analysis of documentation or code
6. **inspection-verification**: For verification through formal inspection or review
7. **demonstration-verification**: For verification through demonstration in a realistic environment
8. **constraint**: Refinement documenting constraints that limit or bound the system
9. **behavior**: Refinement documenting behavior details and operational specifications
10. **specification**: Refinement documenting detailed specifications and technical descriptions
11. **other**: Custom element types defined by users

**Note:** Refinement types (constraint, behavior, specification) cannot have a Relations subsection. These elements serve as detailed documentation that can be attached to other elements.

**Relation constraints:** Each element type has specific constraints on which relation types it can use. See [Element Type Relation Compatibility](DesignDocuments/RelationTypes.md#element-type-relation-compatibility) for the complete compatibility matrix.

#### Relations
  * derivedFrom: [Reserved Subsections Support](StructureAndParsing.md#reserved-subsections-support)
  * satisfiedBy: [element.rs](../../../core/src/element.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
---

### Refinement Element Structure Constraints

The system shall reject Refinement elements (constraint, behavior, specification) that include a Relations subsection during validation.

#### Details
Refinement elements serve as detailed documentation that can be attached to other elements. They are not allowed to have relations because:
- They represent atomic pieces of information without traceability relationships
- They are referenced through the Attachments subsection of other elements
- Their content contributes to the parent element's documentation

When a Refinement element contains a Relations subsection, the validator shall report an error indicating that relations are not allowed for this element type.

#### Relations
  * derivedFrom: [Supported Element Types](#supported-element-types)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
---
