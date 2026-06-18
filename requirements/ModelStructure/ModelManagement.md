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
  * definedBy: [Default Requirement Type Assignment Refinement Specification](Specifications.md#default-requirement-type-assignment-refinement-specification)
  * specify: [Operating on Model Elements](../Operations/BehaviorValidationOperationsFeature.md#operating-on-model-elements)
  * verifiedBy: [Element Subsection Parsing Test](../Verifications/ModelStructure/ParsingVerifications.md#element-subsection-parsing-test)
  * verifiedBy: [Default Element Type Assignment Test](../Verifications/Operations/Validation/ValidationVerifications.md#default-element-type-assignment-test)
---

### Efficient Processing

The system shall process structured documents and relations to extract model-relevant information efficiently.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Opt-In Element Size Estimate Model Build](#opt-in-element-size-estimate-model-build)
  * specify: [Operating on Model Elements](../Operations/BehaviorValidationOperationsFeature.md#operating-on-model-elements)
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
  * definedBy: [Element Size Estimate Model Build Specification](Specifications.md#element-size-estimate-model-build-specification)
  * derivedFrom: [Efficient Processing](#efficient-processing)
  * satisfiedBy: [element.rs](../../core/src/element.rs)
  * satisfiedBy: [graph_registry.rs](../../core/src/graph_registry.rs)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
  * verifiedBy: [Element Size Estimate Model Build Verification](../Verifications/ModelStructure/ParsingVerifications.md#element-size-estimate-model-build-verification)
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
  * definedBy: [Dry-Run Mode Behavior](Behaviors.md#dry-run-mode-behavior)
  * definedBy: [File Persistence Behavior](Behaviors.md#file-persistence-behavior)
  * definedBy: [Operation Command Contract Specification](Specifications.md#operation-command-contract-specification)
  * derive: [Attachment Identifier Updates](#attachment-identifier-updates)
  * derive: [Create Element Operation](../Operations/ModelOperations/ElementManipulationRequirements.md#create-element-operation)
  * derive: [Delete Element Operation](../Operations/ModelOperations/ElementManipulationRequirements.md#delete-element-operation)
  * derive: [Element Manipulation File Persistence](../Operations/ModelOperations/ElementManipulationRequirements.md#element-manipulation-file-persistence)
  * derive: [Move Element Operation](../Operations/ModelOperations/ElementManipulationRequirements.md#move-element-operation)
  * derive: [Move File Operation](../Operations/ModelOperations/ElementManipulationRequirements.md#move-file-operation)
  * derive: [Relation Consistency Maintenance](../Operations/ModelOperations/ElementManipulationRequirements.md#relation-consistency-maintenance)
  * derive: [Rename Element Operation](../Operations/ModelOperations/ElementManipulationRequirements.md#rename-element-operation)
  * derive: [Target Location Validation and Auto-Creation](../Operations/ModelOperations/ElementManipulationRequirements.md#target-location-validation-and-auto-creation)
  * specify: [Operating on Model Elements](../Operations/BehaviorValidationOperationsFeature.md#operating-on-model-elements)
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
  * definedBy: [Attachment Identifier CRUD Update Behavior](Behaviors.md#attachment-identifier-crud-update-behavior)
  * derivedFrom: [Element Manipulation Operations](#element-manipulation-operations)
  * satisfiedBy: [crud.rs](../../core/src/crud.rs)
  * verifiedBy: [Attachment Identifier CRUD Verification](../Verifications/Operations/ModelOperations/AttachmentsVerifications.md#attachment-identifier-crud-verification)
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
  * definedBy: [Subdirectory Auto-Detection Behavior](Behaviors.md#subdirectory-auto-detection-behavior)
  * definedBy: [Containment Specification](Specifications.md#containment-specification)
  * definedBy: [Git Repository Scope Specification](Specifications.md#git-repository-scope-specification)
  * specify: [Defining Model Structure](ModelStructureFeature.md#defining-model-structure)
  * verifiedBy: [Subdirectory Processing Verification](../Verifications/Operations/Validation/ValidationVerifications.md#subdirectory-processing-verification)
---

### Relation Types and behaviors

The system shall implement relations following clearly defined specifications for types and behaviors.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [RelationTypes](RelationTypes.md#relationtypes)
  * definedBy: [Relation Semantics Specification](Specifications.md#relation-semantics-specification)
  * definedBy: [Relation Types and behaviors Refinement Specification](Specifications.md#relation-types-and-behaviors-refinement-specification)
  * derive: [Element Type Relation Compatibility](#element-type-relation-compatibility)
  * derive: [Relation Management Operations](#relation-management-operations)
  * derive: [Trace Relation Non-Directional Behavior](../Reports/ModelReports/DiagramGeneration.md#trace-relation-non-directional-behavior)
  * derivedFrom: [Identifiers and Relations](StructureAndParsing.md#identifiers-and-relations)
  * satisfiedBy: [relation.rs](../../core/src/relation.rs)
  * verifiedBy: [Element Subsection Parsing Test](../Verifications/ModelStructure/ParsingVerifications.md#element-subsection-parsing-test)
---

### Capability Model Structure

The system shall support `capability` elements as product/capability roots that are specified by requirements.

#### Details
Capability model behavior shall follow the relation type and validation specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Capability Model Structure Specification](Specifications.md#capability-model-structure-specification)
  * derivedFrom: [Relation Types and behaviors](#relation-types-and-behaviors)
  * verifiedBy: [Capability Element Relation Compatibility Test](../Verifications/Operations/Validation/ValidationVerifications.md#capability-element-relation-compatibility-test)
---

### Capability Collect Traversal

The system shall collect capability and requirement context using separate capability and requirement hierarchy traversal with the `specifiedBy`/`specify` bridge only where directionally intended.

#### Details
Collect traversal behavior shall follow the associated output specification.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Capability Collect Traversal Specification](Specifications.md#capability-collect-traversal-specification)
  * derivedFrom: [Capability Model Structure](#capability-model-structure)
  * verifiedBy: [Capability Collect Traversal Test](../Verifications/Reports/ModelReports/ReportingVerifications.md#capability-collect-traversal-test)
---

### Capability Coverage Rollup

The system shall report capability verification and implementation coverage by rolling up coverage from requirements that specify each capability.

#### Details
Capability coverage shall remain separate from capability validation. Capability elements may be directly verified but are not directly satisfied.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability Model Structure](#capability-model-structure)
  * verifiedBy: [Capability Coverage Rollup Test](../Verifications/Reports/ModelReports/ReportingVerifications.md#capability-coverage-rollup-test)
---

### Ontology and Semantic Contract Model

The system shall support first-class `ontology` elements for reusable semantic vocabulary and reusable `semantic-contract` elements for SHACL shape profiles that constrain requirements.

#### Details
Ontology elements shall define ontology vocabulary, model concepts, semantic categories, and reusable domain meaning. Authored Reqvire ontology elements shall be kept under the dedicated `requirements/Ontologies` folder rather than nested in capability files. The top parent ontology element in each ontology subgraph shall define non-empty `ontology_base` and `ontology_prefix` metadata; descendant ontology elements inherit both through `derivedFrom`/`derive` hierarchy. Authored OWL ontology blocks define classes, properties, individuals, hierarchy, and axioms in the corresponding hash term namespace. The root ontology block should explicitly declare `<ontology_base> a owl:Ontology` for authored OWL document identity. Reqvire uses the inherited `ontology_prefix` as the canonical CURIE label and `<ontology_base>#` as the canonical namespace; authored Turtle blocks that use the inherited prefix must explicitly declare that prefix to the canonical namespace, and validation fails when the declaration is missing or points to a different namespace. Reqvire emits one generated document-level `owl:Ontology` declaration per resolved `ontology_base`; ontology elements that inherit the same base contribute authored vocabulary to that same document, while `derivedFrom` across different resolved bases becomes `owl:imports`. Authored child ontology blocks do not need to repeat document-level `owl:Ontology` or `owl:imports` declarations. Ontology-controlled vocabulary records shall carry formal semantics through IRI identity, typed class membership, hierarchy, and axioms; standard annotation properties such as `rdfs:label` and `rdfs:comment` may be used for optional presentation metadata, but canonical authored tokens, parser fields, interface enum values, report kinds, controlled-vocabulary payloads, and queryable meanings shall remain declared domain properties. Deprecated presentation-only ontology properties shall not remain in authored Reqvire ontology source after refactoring. Semantic contracts shall define reusable SHACL shape profiles that explicitly use ontology through `use`/`usedBy`, may constrain zero or more requirements through `constrainedBy`/`constrain`, and must not define ontology vocabulary. The model structure capability must include ontology rebasing as part of the owned ontology context, so that changes to `ontology_base` or `ontology_prefix` are handled atomically and the dependent boundary chain is rewritten rather than edited piecemeal.

Non-ontology, non-semantic-contract elements may bind readable prose to ontology vocabulary with `#### Concept References`. Requirement elements may be constrained by semantic contracts through `constrainedBy`/`constrain`. Semantic contracts use ontology vocabulary through `use`/`usedBy`; they are already part of the semantic graph and must not author concept references. Requirements may be defined by source, behavior, state, specification, constraint, and input-output contracts when they need subordinate detail over ontology-backed terms. Capabilities must not own source, constraint, behavior, specification, state, input-output, or semantic-contract elements through `definedBy`/`define`.

#### Metadata
  * type: requirement

#### Relations
  * constrainedBy: [Capability Structure and Relation Shape](../Ontologies/CapabilityRequirementModel.md#capability-structure-and-relation-shape)
  * constrainedBy: [Ontology Element Vocabulary Shape](../Ontologies/CapabilityRequirementModel.md#ontology-element-vocabulary-shape)
  * constrainedBy: [Requirement Ownership and Coverage Shape](../Ontologies/CapabilityRequirementModel.md#requirement-ownership-and-coverage-shape)
  * constrainedBy: [Semantic Contract Structure Shape](../Ontologies/CapabilityRequirementModel.md#semantic-contract-structure-shape)
  * constrainedBy: [Element Identity and Metadata Shape](../Ontologies/Core.md#element-identity-and-metadata-shape)
  * constrainedBy: [Governance Metadata Scope Shape](../Ontologies/Governance.md#governance-metadata-scope-shape)
  * constrainedBy: [Relation Compatibility Shape](../Ontologies/RelationsAndImpact.md#relation-compatibility-shape)
  * definedBy: [Defining Model Structure Source](ModelStructureFeature.md#defining-model-structure-source)
  * definedBy: [Ontology Annotation Convention Specification](Specifications.md#ontology-annotation-convention-specification)
  * definedBy: [Semantic Contract Structure Specification](Specifications.md#semantic-contract-structure-specification)
  * derivedFrom: [Capability Model Structure](#capability-model-structure)
  * verifiedBy: [Semantic Contract Ontology Declaration Validation Test](../Verifications/Operations/Validation/ValidationVerifications.md#semantic-contract-ontology-declaration-validation-test)
  * verifiedBy: [Semantic Contract Relation Validation Test](../Verifications/Operations/Validation/ValidationVerifications.md#semantic-contract-relation-validation-test)
  * verifiedBy: [Semantic Contract Section Validation Test](../Verifications/Operations/Validation/ValidationVerifications.md#semantic-contract-section-validation-test)
  * verifiedBy: [Semantic Contract SHACL Sanity Validation Test](../Verifications/Operations/Validation/ValidationVerifications.md#semantic-contract-shacl-sanity-validation-test)
---

### Element Type Relation Compatibility

The system shall enforce element type constraints for relation types, ensuring that only valid combinations of source and target element types are allowed for each relation type.

#### Details
The system shall define element type relation compatibility constraints.
Mutation commands that create or rewrite relations shall enforce the same compatibility matrix before persisting source files.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Element Type Relation Compatibility Constraint](Constraints.md#element-type-relation-compatibility-constraint)
  * definedBy: [Supported Element Types Specification](Specifications.md#supported-element-types-specification)
  * derivedFrom: [Relation Types and behaviors](#relation-types-and-behaviors)
  * satisfiedBy: [graph_registry.rs](../../core/src/graph_registry.rs)
  * satisfiedBy: [relation.rs](../../core/src/relation.rs)
  * verifiedBy: [Verification Objective Mutation Test](../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#verification-objective-mutation-test)
  * verifiedBy: [Element Type Relation Compatibility Test](../Verifications/Operations/Validation/ValidationVerifications.md#element-type-relation-compatibility-test)
---

### Contract Element Structure Constraints

The system shall restrict ordinary contract elements (`source`, `constraint`, `behavior`, `specification`, `state`, `input-output`) to only allow `define` relations, and shall restrict `semantic-contract` elements to `constrain` and `use` relations.

#### Details
Contract relation validation shall enforce the subtype-compatible contract vocabulary defined by the Reqvire capability, requirement, ontology, and semantic-contract model contracts.

#### Metadata
  * type: requirement

#### Attachments
  * [Supported Element Types Specification](Specifications.md#supported-element-types-specification)

#### Relations
  * constrainedBy: [Custom Element Trace Boundary Shape](../Ontologies/CapabilityRequirementModel.md#custom-element-trace-boundary-shape)
  * constrainedBy: [Refinement Ownership Shape](../Ontologies/CapabilityRequirementModel.md#refinement-ownership-shape)
  * definedBy: [Refinement Element Structure Constraints Refinement Specification](Specifications.md#contract-element-structure-constraints-specification)
  * derivedFrom: [Relation Types and behaviors](#relation-types-and-behaviors)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
  * verifiedBy: [Refinement Relations Rejection Test](../Verifications/ModelStructure/ParsingVerifications.md#refinement-relations-rejection-test)
---

### Attachment Scope Constraints

The system shall enforce scope constraints on contract-element attachments to ensure proper cross-submodel traceability while preventing redundant or invalid attachment relationships.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Attachment Hierarchical Independence Constraint](Constraints.md#attachment-hierarchical-independence-constraint)
  * definedBy: [Attachment Satisfied Refinement Constraint](Constraints.md#attachment-satisfied-contract-constraint)
  * definedBy: [Attachment Subgraph Direction Constraint](Constraints.md#attachment-subgraph-direction-constraint)
  * derivedFrom: [Refinement Element Structure Constraints](#contract-element-structure-constraints)
  * verifiedBy: [Attachment Scope Constraints Test](../Verifications/Operations/ModelOperations/AttachmentsVerifications.md#attachment-scope-constraints-test)
---

### Relation Management Operations

The system shall provide programmatic manipulation of element relations through link and unlink operations while maintaining model integrity.

#### Details
When linking, the system shall:
- Add a relation entry to the element's Relations subsection
- Create the Relations subsection if it does not exist
- Validate the relation type and element type compatibility before persistence
- Skip if relation already exists (idempotent operation)

When unlinking, the system shall:
- Remove the relation entry from the element's Relations subsection
- Remove the Relations subsection if no relations remain

#### Metadata
  * type: requirement

#### Attachments
  * [Attachment Hierarchical Independence Constraint](Constraints.md#attachment-hierarchical-independence-constraint)
  * [Attachment Satisfied Refinement Constraint](Constraints.md#attachment-satisfied-contract-constraint)

#### Relations
  * definedBy: [Relation Operations Specification](Specifications.md#relation-operations-specification)
  * derivedFrom: [Relation Types and behaviors](#relation-types-and-behaviors)
  * satisfiedBy: [crud.rs](../../core/src/crud.rs)
  * satisfiedBy: [graph_registry.rs](../../core/src/graph_registry.rs)
  * verifiedBy: [Link Command Verification](../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#link-command-verification)
  * verifiedBy: [Unlink Command Verification](../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#unlink-command-verification)
  * verifiedBy: [Verification Objective Mutation Test](../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#verification-objective-mutation-test)
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
  * definedBy: [Requirement Governance Metadata Inheritance Behavior](Behaviors.md#requirement-governance-metadata-inheritance-behavior)
  * satisfiedBy: [element.rs](../../core/src/element.rs)
  * satisfiedBy: [graph_registry.rs](../../core/src/graph_registry.rs)
  * satisfiedBy: [search.rs](../../core/src/search.rs)
  * specify: [Operating on Model Elements](../Operations/BehaviorValidationOperationsFeature.md#operating-on-model-elements)
  * verifiedBy: [Requirement Governance Metadata Verification](../Verifications/ModelStructure/ParsingVerifications.md#requirement-governance-metadata-verification)
  * verifiedBy: [Requirement Governance Metadata Formatting Verification](../Verifications/Operations/Formatting/FormattingVerifications.md#requirement-governance-metadata-formatting-verification)
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
  * specify: [Operating on Model Elements](../Operations/BehaviorValidationOperationsFeature.md#operating-on-model-elements)
---

### Verification Type Categories

The system shall support defined verification categories following clearly defined specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Supported Element Types Specification](Specifications.md#supported-element-types-specification)

#### Relations
  * definedBy: [Verification Type Selection Guidelines](Specifications.md#verification-type-selection-guidelines)
---
