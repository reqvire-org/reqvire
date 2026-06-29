# Elements

### Default Requirement Type Assignment

The system shall automatically assign the **default type `requirement`** to all elements if not explicitly specified in their `metadata` subsection.

#### Details
Type assignment behavior shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Element Type Metadata Specification](Specifications.md#element-type-metadata-specification)

#### Relations
  * definedBy: [Default Requirement Type Assignment Contract Specification](Specifications.md#default-requirement-type-assignment-contract-specification)
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
  * satisfiedBy: [element.rs](../../crates/reqvire-core/src/element.rs)
  * satisfiedBy: [registration.rs](../../crates/reqvire-core/src/graph_registry/registration.rs)
  * satisfiedBy: [model.rs](../../crates/reqvire-core/src/model.rs)
  * verifiedBy: [Element Size Estimate Model Build Verification](../Verifications/ModelStructure/ParsingVerifications.md#element-size-estimate-model-build-verification)
---

### In-Memory Model Build Cache

The system shall cache parsed `ModelManager` instances keyed by a fingerprint of the scanned workspace markdown files and the active model build options, so that repeated tool dispatches over an unchanged workspace return a cached model without re-parsing.

#### Details
- The cache key shall combine `ModelBuildOptions` with a sorted map of every scanned `.md` file path to a content fingerprint (`FileFingerprint`) capturing file length and content hash.
- A cache hit returns a clone of the stored model without re-reading or re-validating any source file.
- A cache miss rebuilds the model via `parse_and_validate_with_options`, stores the result, and returns a clone.
- CRUD mutations (add, move, rename, remove, merge, relink, link, unlink, mv-file, mv-asset, rm-asset) shall invalidate the cache, forcing a rebuild on the next load.
- Only the current working tree is cached; git-commit scans bypass the cache and use `parse_and_validate` directly.
- The cache mutex lock shall be released before rebuild I/O to avoid holding it during parsing.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [In-Memory Model Build Cache Specification](Specifications.md#in-memory-model-build-cache-specification)
  * derivedFrom: [Efficient Processing](#efficient-processing)
  * satisfiedBy: [model_cache.rs](../../crates/reqvire-core/src/model_cache.rs)
  * satisfiedBy: [arg_helpers.rs](../../crates/reqvire-core/src/tool_interface/arg_helpers.rs)
  * verifiedBy: [In-Memory Model Build Cache Verification](../Verifications/ModelStructure/ParsingVerifications.md#in-memory-model-build-cache-verification)

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
  * derive: [Contract Bindings Identifier Updates](#contract-bindings-identifier-updates)
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

### Contract Bindings Identifier Updates

When moving or renaming Contract elements, the system shall update contract_bindings identifiers in all referencing elements, using the same behavior as relation target updates.

#### Details
This requirement ensures consistency between relation updates and contract_bindings identifier updates during CRUD operations:

- When a Contract element is **moved** to a different file, all contract_bindings referencing that element shall be updated with the new identifier path
- When a Contract element is **renamed**, all contract_bindings referencing that element shall be updated with the new element name in the identifier
- The update behavior mirrors how relation targets are updated during move/rename operations
- All files containing contract_bindings that reference the affected element shall be modified

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Contract Bindings Identifier CRUD Update Behavior](Behaviors.md#contract-bindings-identifier-crud-update-behavior)
  * derivedFrom: [Element Manipulation Operations](#element-manipulation-operations)
  * satisfiedBy: [crud.rs](../../crates/reqvire-core/src/crud.rs)
  * verifiedBy: [Contract Bindings Identifier CRUD Verification](../Verifications/Operations/ModelOperations/ContractBindingVerifications.md#contract-bindings-identifier-crud-verification)
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
  * definedBy: [Relation Semantics Specification](Specifications.md#relation-semantics-specification)
  * derive: [Element Type Relation Compatibility](#element-type-relation-compatibility)
  * derive: [Relation Management Operations](#relation-management-operations)
  * derivedFrom: [Identifiers and Relations](StructureAndParsing.md#identifiers-and-relations)
  * satisfiedBy: [relation.rs](../../crates/reqvire-core/src/relation.rs)
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
Capability coverage shall remain separate from capability validation. Capability elements are not directly verified or directly satisfied; verification coverage rolls up from requirements that specify each capability.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability Model Structure](#capability-model-structure)
  * verifiedBy: [Capability Coverage Rollup Test](../Verifications/Reports/ModelReports/ReportingVerifications.md#capability-coverage-rollup-test)
---

### Ontology and Semantic Contract Model

The system shall support first-class `ontology` elements for reusable semantic vocabulary and reusable `semantic-contract` elements for SHACL shape profiles that constrain requirements.

#### Details
Detailed ontology metadata, inheritance, OWL document, semantic-contract, concept-reference, contract ownership, validation, indexing, export, and rebasing rules shall follow the associated specifications and constraints.

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

### Contract Element Structure Constraints

The system shall restrict ordinary contract elements (`source`, `constraint`, `behavior`, `specification`, `state`, `input-output`) to only allow `define` relations, and shall restrict `semantic-contract` elements to `constrain` and `use` relations.

#### Details
Contract relation validation shall enforce the subtype-compatible contract vocabulary defined by the Reqvire capability, requirement, ontology, and semantic-contract model contracts.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Supported Element Types Specification](Specifications.md#supported-element-types-specification)

#### Relations
  * constrainedBy: [Contract Ownership Shape](../Ontologies/CapabilityRequirementModel.md#contract-ownership-shape)
  * constrainedBy: [Custom Element Semantic Boundary Shape](../Ontologies/CapabilityRequirementModel.md#custom-element-semantic-boundary-shape)
  * definedBy: [Contract Element Structure Constraints Specification](Specifications.md#contract-element-structure-constraints-specification)
  * derivedFrom: [Relation Types and behaviors](#relation-types-and-behaviors)
  * satisfiedBy: [model.rs](../../crates/reqvire-core/src/model.rs)
  * verifiedBy: [Contract Relations Rejection Test](../Verifications/ModelStructure/ParsingVerifications.md#contract-relations-rejection-test)
---

### Contract Bindings Scope Constraints

The system shall enforce scope constraints on contract-element contract_bindings to ensure proper cross-submodel traceability while preventing redundant or invalid contract_bindings relationships.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Contract Bindings Hierarchical Independence Constraint](Constraints.md#contract-bindings-hierarchical-independence-constraint)
  * definedBy: [Contract Bindings Satisfied Contract Constraint](Constraints.md#contract-bindings-satisfied-contract-constraint)
  * definedBy: [Contract Bindings Subgraph Direction Constraint](Constraints.md#contract-bindings-subgraph-direction-constraint)
  * derivedFrom: [Contract Element Structure Constraints](#contract-element-structure-constraints)
  * verifiedBy: [Contract Bindings Scope Constraints Test](../Verifications/Operations/ModelOperations/ContractBindingVerifications.md#contract-bindings-scope-constraints-test)
---

### Element Type Relation Compatibility

The system shall enforce element type constraints for relation types, ensuring that only valid combinations of source and target element types are allowed for each relation type.

#### Details
Detailed compatibility matrix, source/target type rules, and mutation enforcement rules shall follow the associated constraint and type specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Element Type Relation Compatibility Constraint](Constraints.md#element-type-relation-compatibility-constraint)
  * definedBy: [Supported Element Types Specification](Specifications.md#supported-element-types-specification)
  * derivedFrom: [Relation Types and behaviors](#relation-types-and-behaviors)
  * satisfiedBy: [crud_ops.rs](../../crates/reqvire-core/src/graph_registry/crud_ops.rs)
  * satisfiedBy: [validation.rs](../../crates/reqvire-core/src/graph_registry/validation.rs)
  * satisfiedBy: [relation.rs](../../crates/reqvire-core/src/relation.rs)
  * verifiedBy: [Verification Objective Mutation Test](../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#verification-objective-mutation-test)
  * verifiedBy: [Element Type Relation Compatibility Test](../Verifications/Operations/Validation/ValidationVerifications.md#element-type-relation-compatibility-test)
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

#### Contract Bindings
  * [Contract Bindings Hierarchical Independence Constraint](Constraints.md#contract-bindings-hierarchical-independence-constraint)
  * [Contract Bindings Satisfied Contract Constraint](Constraints.md#contract-bindings-satisfied-contract-constraint)

#### Relations
  * definedBy: [Relation Operations Specification](Specifications.md#relation-operations-specification)
  * derivedFrom: [Relation Types and behaviors](#relation-types-and-behaviors)
  * satisfiedBy: [crud.rs](../../crates/reqvire-core/src/crud.rs)
  * satisfiedBy: [crud_ops.rs](../../crates/reqvire-core/src/graph_registry/crud_ops.rs)
  * verifiedBy: [Link Command Verification](../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#link-command-verification)
  * verifiedBy: [Unlink Command Verification](../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#unlink-command-verification)
  * verifiedBy: [Verification Objective Mutation Test](../Verifications/Operations/ModelOperations/ElementManipulationVerifications.md#verification-objective-mutation-test)
---

### Requirement Governance Metadata

The system shall support governance metadata for requirement elements, including status, priority, risk, and owner, with effective values resolved from authored metadata, hierarchy inheritance, and defaults.

#### Details
Governance metadata behavior shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Requirement Governance Metadata Specification](Specifications.md#requirement-governance-metadata-specification)

#### Relations
  * definedBy: [Requirement Governance Metadata Inheritance Behavior](Behaviors.md#requirement-governance-metadata-inheritance-behavior)
  * satisfiedBy: [element.rs](../../crates/reqvire-core/src/element.rs)
  * satisfiedBy: [hierarchy.rs](../../crates/reqvire-core/src/graph_registry/hierarchy.rs)
  * satisfiedBy: [search.rs](../../crates/reqvire-core/src/search.rs)
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

#### Contract Bindings
  * [Supported Element Types Specification](Specifications.md#supported-element-types-specification)

#### Relations
  * definedBy: [Verification Type Selection Guidelines](Specifications.md#verification-type-selection-guidelines)
---
