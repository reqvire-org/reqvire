# Elements

### Default Requirement Type Assignment

The system shall automatically assign the **default type `requirement`** to all elements if not explicitly specified in their `metadata` subsection.

#### Details
Type assignment behavior shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Element Type Metadata Specification](Specifications.md#element-type-metadata-specification)

#### Relations
  * refinedBy: [Default Requirement Type Assignment Refinement Specification](Specifications.md#default-requirement-type-assignment-refinement-specification)
  * specify: [Operating on Model Elements](../../Capabilities/BehaviorValidationOperations.md#operating-on-model-elements)
  * verifiedBy: [Element Subsection Parsing Test](Verifications/ParsingVerifications.md#element-subsection-parsing-test)
  * verifiedBy: [Default Element Type Assignment Test](Verifications/ValidationVerifications.md#default-element-type-assignment-test)
---

### Efficient Processing

The system shall process structured documents and relations to extract model-relevant information efficiently.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Opt-In Element Size Estimate Model Build](#opt-in-element-size-estimate-model-build)
  * specify: [Operating on Model Elements](../../Capabilities/BehaviorValidationOperations.md#operating-on-model-elements)
---

### Opt-In Element Size Estimate Model Build

The system shall support an opt-in model build mode that computes canonical size estimates for parsed model elements.

#### Details
- Element size estimates shall be model evidence metadata derived during model build when explicitly enabled.
- Normal model loading shall not compute size estimates by default.
- Size estimates shall not be written to source Markdown files.
- The model build option shall be reusable by CLI JSON commands and MCP server startup.
- Size estimates shall be element-level metadata; report-level aggregate estimates are out of scope for this requirement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Efficient Processing](#efficient-processing)
  * refinedBy: [Element Size Estimate Model Build Specification](Specifications.md#element-size-estimate-model-build-specification)
  * satisfiedBy: [element.rs](../../../core/src/element.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * verifiedBy: [Element Size Estimate Model Build Verification](Verifications/ParsingVerifications.md#element-size-estimate-model-build-verification)
---

### Element Manipulation Operations

The system shall provide programmatic manipulation of model elements through operations including, but not limited to, creating new elements, deleting existing elements, moving elements between locations, and renaming elements while maintaining model integrity and traceability.

#### Details
All manipulation operations shall:
- Maintain model integrity and consistency
- Update or remove affected relations automatically
- Preserve traceability where appropriate

#### Metadata
  * type: requirement

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
  * refinedBy: [Dry-Run Mode Behavior](../Operations/Behaviors.md#dry-run-mode-behavior)
  * refinedBy: [File Persistence Behavior](../Operations/Behaviors.md#file-persistence-behavior)
  * refinedBy: [Operation Command Contract Specification](../Operations/Specifications.md#operation-command-contract-specification)
  * specify: [Operating on Model Elements](../../Capabilities/BehaviorValidationOperations.md#operating-on-model-elements)
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
  * type: requirement

#### Relations
  * refinedBy: [Subdirectory Auto-Detection Behavior](Behaviors.md#subdirectory-auto-detection-behavior)
  * refinedBy: [Git Repository Scope Specification](Specifications.md#git-repository-scope-specification)
  * refinedBy: [Containment Specification](../../Refinements.md#containment-specification)
  * specify: [Defining Model Structure](../../Capabilities.md#defining-model-structure)
  * verifiedBy: [Subdirectory Processing Verification](Verifications/ValidationVerifications.md#subdirectory-processing-verification)
---

### Relation Types and behaviors

The system shall implement relations following clearly defined specifications for types and behaviors.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Element Type Relation Compatibility](#element-type-relation-compatibility)
  * derive: [Relation Management Operations](#relation-management-operations)
  * derive: [Trace Relation Non-Directional Behavior](../Output/DiagramGeneration.md#trace-relation-non-directional-behavior)
  * derivedFrom: [Identifiers and Relations](StructureAndParsing.md#identifiers-and-relations)
  * refinedBy: [RelationTypes](DesignDocuments/RelationTypes.md#relationtypes)
  * refinedBy: [Relation Types and behaviors Refinement Specification](Specifications.md#relation-types-and-behaviors-refinement-specification)
  * refinedBy: [Relation Semantics Specification](../../Refinements.md#relation-semantics-specification)
  * satisfiedBy: [relation.rs](../../../core/src/relation.rs)
  * verifiedBy: [Element Subsection Parsing Test](Verifications/ParsingVerifications.md#element-subsection-parsing-test)
---

### Element Type Relation Compatibility

The system shall enforce element type constraints for relation types, ensuring that only valid combinations of source and target element types are allowed for each relation type.

#### Details
The system shall define element type relation compatibility constraints.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Relation Types and behaviors](#relation-types-and-behaviors)
  * refinedBy: [Element Type Relation Compatibility Constraint](Constraints.md#element-type-relation-compatibility-constraint)
  * refinedBy: [Supported Element Types Specification](../../Refinements.md#supported-element-types-specification)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [relation.rs](../../../core/src/relation.rs)
  * verifiedBy: [Element Type Relation Compatibility Test](Verifications/ValidationVerifications.md#element-type-relation-compatibility-test)
---

### Capability Model Structure

The system shall support `capability` elements as product/capability roots that are specified by requirements.

#### Details
Capability model behavior shall follow the relation type and validation specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Relation Types and behaviors](#relation-types-and-behaviors)
  * refinedBy: [Capability Model Structure Specification](Specifications.md#capability-model-structure-specification)
  * verifiedBy: [Capability Element Relation Compatibility Test](Verifications/ValidationVerifications.md#capability-element-relation-compatibility-test)
---

### Capability Collect Traversal

The system shall collect capability and requirement context using separate capability and requirement hierarchy traversal with the `specifiedBy`/`specify` bridge only where directionally intended.

#### Details
Collect traversal behavior shall follow the associated output specification.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability Model Structure](#capability-model-structure)
  * refinedBy: [Capability Collect Traversal Specification](../Output/Specifications.md#capability-collect-traversal-specification)
  * verifiedBy: [Capability Collect Traversal Test](../Output/Verifications/ReportingVerifications.md#capability-collect-traversal-test)
---

### Capability Coverage Rollup

The system shall report capability verification and implementation coverage by rolling up coverage from requirements that specify each capability.

#### Details
Capability coverage shall remain separate from capability validation. Capability elements may be directly verified but are not directly satisfied.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability Model Structure](#capability-model-structure)
  * verifiedBy: [Capability Coverage Rollup Test](../Output/Verifications/ReportingVerifications.md#capability-coverage-rollup-test)
---

### Ontology and Semantic Contract Model

The system shall support first-class `ontology` elements for reusable semantic vocabulary and requirement-owned `semantic-contract` refinement elements for obligation-specific SHACL shape profiles.

#### Details
Ontology elements shall define ontology vocabulary, model concepts, semantic categories, and reusable domain meaning. Authored Reqvire ontology elements shall be kept under the dedicated `requirements/Ontologies` folder rather than nested in capability files. Requirement-owned semantic contracts shall define SHACL shape profiles for one obligation and must not define ontology vocabulary.

Capability elements attach ontology elements to make vocabulary reachable for the capability, descendant capabilities, and requirements that specify that capability context. Requirement elements must not attach ontology directly; they inherit ontology context through their owning capability path. Capability and requirement elements may refine to semantic contracts, behavior, state, specification, constraint, and input-output refinements when they need closed-world profiles or additional operational/contract detail over reachable ontology terms.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability Model Structure](#capability-model-structure)
  * refinedBy: [Semantic Contract Structure Specification](Specifications.md#semantic-contract-structure-specification)
  * verifiedBy: [Semantic Contract Ontology Declaration Validation Test](Verifications/ValidationVerifications.md#semantic-contract-ontology-declaration-validation-test)
  * verifiedBy: [Semantic Contract Ownership Validation Test](Verifications/ValidationVerifications.md#semantic-contract-ownership-validation-test)
  * verifiedBy: [Semantic Contract Section Validation Test](Verifications/ValidationVerifications.md#semantic-contract-section-validation-test)
  * verifiedBy: [Semantic Contract SHACL Sanity Validation Test](Verifications/ValidationVerifications.md#semantic-contract-shacl-sanity-validation-test)
---

### Refinement Element Structure Constraints

The system shall restrict Refinement elements (`source`, `semantic-contract`, `constraint`, `behavior`, `specification`, `state`, `input-output`) to only allow `refine` relations.

#### Details
Refinement relation validation shall enforce the subtype-compatible refinement vocabulary defined by the Reqvire capability, requirement, ontology, and semantic-contract model contracts.

#### Metadata
  * type: requirement

#### Attachments
  * [Supported Element Types Specification](../../Refinements.md#supported-element-types-specification)

#### Relations
  * derivedFrom: [Relation Types and behaviors](#relation-types-and-behaviors)
  * refinedBy: [Refinement Element Structure Constraints Refinement Specification](Specifications.md#refinement-element-structure-constraints-refinement-specification)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * verifiedBy: [Refinement Relations Rejection Test](Verifications/ParsingVerifications.md#refinement-relations-rejection-test)
---

### Attachment Scope Constraints

The system shall enforce scope constraints on refinement-element attachments to ensure proper cross-submodel traceability while preventing redundant or invalid attachment relationships.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Refinement Element Structure Constraints](#refinement-element-structure-constraints)
  * refinedBy: [Attachment Hierarchical Independence Constraint](Constraints.md#attachment-hierarchical-independence-constraint)
  * refinedBy: [Attachment Satisfied Refinement Constraint](Constraints.md#attachment-satisfied-refinement-constraint)
  * refinedBy: [Attachment Subgraph Direction Constraint](Constraints.md#attachment-subgraph-direction-constraint)
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

### Requirement Governance Metadata

The system shall support governance metadata for requirement elements, including status, priority, risk, and owner, with effective values resolved from authored metadata, hierarchy inheritance, and defaults.

#### Details
Governance metadata behavior shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Requirement Governance Metadata Specification](Specifications.md#requirement-governance-metadata-specification)

#### Relations
  * refinedBy: [Requirement Governance Metadata Inheritance Behavior](Behaviors.md#requirement-governance-metadata-inheritance-behavior)
  * satisfiedBy: [element.rs](../../../core/src/element.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [search.rs](../../../core/src/search.rs)
  * specify: [Operating on Model Elements](../../Capabilities/BehaviorValidationOperations.md#operating-on-model-elements)
  * verifiedBy: [Requirement Governance Metadata Verification](Verifications/ParsingVerifications.md#requirement-governance-metadata-verification)
  * verifiedBy: [Requirement Governance Metadata Formatting Verification](../Operations/Verifications/FormattingVerifications.md#requirement-governance-metadata-formatting-verification)
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
  * type: requirement

#### Relations
  * specify: [Operating on Model Elements](../../Capabilities/BehaviorValidationOperations.md#operating-on-model-elements)
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
