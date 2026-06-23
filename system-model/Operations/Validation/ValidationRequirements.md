# Elements

### Excluded File Relation Validation

The system shall properly validate relations targeting files matching excluded filename patterns, enabling references to excluded files while still respecting their exclusion from processing and formatting operations.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Excluded File Relation Validation Contract Specification](Specifications.md#excluded-file-relation-validation-contract-specification)
  * derivedFrom: [File Pattern Exclusion for Format](../Formatting/FormattingRequirements.md#file-pattern-exclusion-for-format)
  * satisfiedBy: [parser.rs](../../../crates/reqvire-core/src/parser.rs)
  * verifiedBy: [Unstructured Documents Test](../../Verifications/Operations/Validation/ValidationVerifications.md#unstructured-documents-test)
---

### Semantic Contract Shape Validation

The system shall validate semantic-contract SHACL shape documents using the ontology context explicitly reachable from each semantic contract.

#### Details
When validating semantic contracts, the system shall parse each `#### Shapes` Turtle block, validate the SHACL document structure that Reqvire depends on, and resolve SHACL vocabulary references against built-in vocabulary, local external ontology sources, and ontology terms reachable through the semantic contract's explicit `use` relations.

This validation shall allow valid SHACL target mechanisms while preserving Reqvire's traceability requirement that model-owned references are backed by the semantic contract's declared ontology-use context.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Semantic Contract Shape Validation Specification](Specifications.md#semantic-contract-shape-validation-specification)
  * derive: [Reqvire SHACL Context Adapter](#reqvire-shacl-context-adapter)
  * derivedFrom: [Ontology and Semantic Contract Model](../../ModelStructure/ModelManagement.md#ontology-and-semantic-contract-model)
  * verifiedBy: [Semantic Contract SHACL Sanity Validation Test](../../Verifications/Operations/Validation/ValidationVerifications.md#semantic-contract-shacl-sanity-validation-test)
---

### Reqvire SHACL Context Adapter

The system shall adapt Reqvire semantic-contract ontology context into the generic SHACL ontology alignment input.

#### Details
The Reqvire adapter shall ask the semantic index for parsed RDF quads from the ontology subset reachable through semantic-contract `use` relations and ontology ancestry, including reachable local external ontology source quads. It shall derive the SHACL domain ontology index from that supplied RDF context, resolve built-in vocabulary policy, invoke the generic SHACL ontology aligner, and map generic parser/alignment diagnostics into Reqvire semantic diagnostics.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [SHACL Ontology Alignment Specification](../../Architecture/OntologyKernelSpecifications.md#shacl-ontology-alignment-specification)
  * [SHACL Structural Parser Registry Specification](../../Architecture/OntologyKernelSpecifications.md#shacl-structural-parser-registry-specification)
  * [Standards Reserved Vocabulary Recognition Specification](../../Architecture/OntologyKernelSpecifications.md#standards-reserved-vocabulary-recognition-specification)

#### Relations
  * definedBy: [Reqvire SHACL Context Adapter Specification](Specifications.md#reqvire-shacl-context-adapter-specification)
  * derivedFrom: [Semantic Contract Shape Validation](#semantic-contract-shape-validation)
  * satisfiedBy: [graph_registry.rs](../../../crates/reqvire-core/src/graph_registry.rs)
  * satisfiedBy: [semantic_contract.rs](../../../crates/reqvire-core/src/semantic_contract.rs)
  * verifiedBy: [Semantic Contract SHACL Sanity Validation Test](../../Verifications/Operations/Validation/ValidationVerifications.md#semantic-contract-shacl-sanity-validation-test)
---

### Validate Cross-Component Dependencies

The system shall validate dependencies across different components of the System model to identify mismatches or gaps.

#### Metadata
  * type: requirement

#### Relations
  * constrainedBy: [Behavior Rule Structure Shape](../../Ontologies/BehaviorValidationOperations.md#behavior-rule-structure-shape)
  * constrainedBy: [Validation Rule Diagnostic Shape](../../Ontologies/BehaviorValidationOperations.md#validation-rule-diagnostic-shape)
  * derive: [Cross-Component Dependency Validator](#cross-component-dependency-validator)
  * specify: [Validating Structures](../BehaviorValidationOperationsFeature.md#validating-structures)
---

### Cross-Component Dependency Validator

The system shall implement a specialized validator that analyzes dependencies across different model components, ensuring proper alignment between architectural layers, requirement levels, and verification elements.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Validate Cross-Component Dependencies](#validate-cross-component-dependencies)
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/model.rs)
  * satisfiedBy: [parser.rs](../../../crates/reqvire-core/src/parser.rs)
  * verifiedBy: [Invalid Relations Test](../../Verifications/Operations/Validation/ValidationVerifications.md#invalid-relations-test)
---

### Validate Filesystem Structure

The system shall validate the organization of files and folders in the repository to ensure consistency with the MBSE methodology.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Validating Structures](../BehaviorValidationOperationsFeature.md#validating-structures)
---

### Validate Internal Consistency

The system shall check the internal consistency of the system model, ensuring that relationships and elements align correctly.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Internal Consistency Validator](#internal-consistency-validator)
  * derive: [Two-Pass Validation Strategy](#two-pass-validation-strategy)
  * specify: [Validating Structures](../BehaviorValidationOperationsFeature.md#validating-structures)
---

### GraphRegistry as Primary Registry

The system shall enhance GraphRegistry to serve as the primary structure for relation operations and validation during Pass 2.

#### Details
The GraphRegistry shall be responsible for:

1. **Graph construction**: Building adjacency lists from ElementRegistry
2. **Relation validation**: Checking target existence and type compatibility
3. **Opposite generation**: Creating missing bidirectional relations
4. **Cycle detection**: Identifying circular dependencies
5. **Orphan detection**: Finding isolated elements
6. **Impact analysis**: Supporting change propagation queries

The GraphRegistry shall be constructed from the ElementRegistry after Pass 1 completes successfully.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Requirements Processing Specification](Specifications.md#requirements-processing-specification)
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * satisfiedBy: [graph_registry.rs](../../../crates/reqvire-core/src/graph_registry.rs)
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/model.rs)
  * verifiedBy: [Requirements Files Search and Detection Test](../../Verifications/Operations/Validation/ValidationVerifications.md#requirements-files-search-and-detection-test)
---

### Integrated Validation

The system shall automatically perform validation when any command requires the parsed model, eliminating the need for a separate validate command.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Two-Pass Validation Behavior](Behaviors.md#two-pass-validation-behavior)

#### Relations
  * definedBy: [Integrated Validation Contract Specification](Specifications.md#integrated-validation-contract-specification)
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
---

### Internal Consistency Validator

The system shall implement a consistency validator that verifies logical coherence within the model, including checking for circular dependencies, orphaned elements, inconsistent relationship patterns, and element name uniqueness, with detailed error reporting.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Internal Consistency Validator Contract Specification](Specifications.md#internal-consistency-validator-contract-specification)
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/model.rs)
  * satisfiedBy: [parser.rs](../../../crates/reqvire-core/src/parser.rs)
  * verifiedBy: [Invalid Relations Test](../../Verifications/Operations/Validation/ValidationVerifications.md#invalid-relations-test)
---

### Cross-Section Duplicate Validation

The system shall detect when the same target appears in both the Relations and Reused Contract Context subsections of an element, treating this as a validation error.

#### Details
A constraint defines the detailed rules for cross-section duplicate detection.

This applies to identifier targets.

Within-section duplicates (same entry repeated within Relations OR within Reused Contract Context) are formatting issues handled by the format operation, not validation errors.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Cross-Section Duplicate Constraint](Constraints.md#cross-section-duplicate-constraint)
  * derivedFrom: [Internal Consistency Validator](#internal-consistency-validator)
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/model.rs)
  * verifiedBy: [Cross-Section Duplicate Validation Test](../../Verifications/Operations/Validation/ValidationVerifications.md#cross-section-duplicate-validation-test)
---

### Relation Type Validation

The system shall validate relation types against a defined vocabulary and provide clear error messages for unsupported relation types, including suggestions for the correct relation types.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * satisfiedBy: [relation.rs](../../../crates/reqvire-core/src/relation.rs)
  * verifiedBy: [Invalid Relations Test](../../Verifications/Operations/Validation/ValidationVerifications.md#invalid-relations-test)
  * verifiedBy: [Same-File Fragment Relations Test](../../Verifications/Operations/Validation/ValidationVerifications.md#same-file-fragment-relations-test)
---

### Reused Contract Context Target Validation

The system shall validate reused_contract_context targets and reject invalid reused_contract_context references during model validation.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Reused Contract Context Target Validation Contract Specification](Specifications.md#reused-contract-context-target-validation-contract-specification)
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/model.rs)
  * satisfiedBy: [parser.rs](../../../crates/reqvire-core/src/parser.rs)
  * verifiedBy: [Reused Contract Context Subsection Parsing Verification](../../Verifications/Operations/ModelOperations/ReusedContractContextVerifications.md#reused-contract-context-subsection-parsing-verification)
  * verifiedBy: [Reused Contract Context Validation Verification](../../Verifications/Operations/ModelOperations/ReusedContractContextVerifications.md#reused-contract-context-validation-verification)
---

### Reused Contract Context Scope Validation

The system shall validate reused_contract_context scope constraints and report validation errors for violations.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Reused Contract Context Hierarchical Independence Constraint](../../ModelStructure/Constraints.md#reused-contract-context-hierarchical-independence-constraint)
  * [Reused Contract Context Subgraph Direction Constraint](../../ModelStructure/Constraints.md#reused-contract-context-subgraph-direction-constraint)
  * [Reused Contract Context Satisfied Contract Constraint](../../ModelStructure/Constraints.md#reused-contract-context-satisfied-contract-constraint)

#### Relations
  * definedBy: [Reused Contract Context Scope Validation Contract Specification](Specifications.md#reused-contract-context-scope-validation-contract-specification)
  * derivedFrom: [Reused Contract Context Target Validation](#reused-contract-context-target-validation)
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/model.rs)
  * verifiedBy: [Reused Contract Context Scope Constraints Test](../../Verifications/Operations/ModelOperations/ReusedContractContextVerifications.md#reused-contract-context-scope-constraints-test)
---

### Single Root Hierarchy Ownership

The system shall enforce that each requirement hierarchy resolves to exactly one owning capability root through `specify`/`specifiedBy` and requirement/capability hierarchy relations.

#### Details
Validation details shall follow the associated hierarchy ownership constraint.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Single Root Hierarchy Ownership Constraint](Constraints.md#single-root-hierarchy-ownership-constraint)
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * satisfiedBy: [graph_registry.rs](../../../crates/reqvire-core/src/graph_registry.rs)
  * verifiedBy: [Single Root Hierarchy Ownership Validation Test](../../Verifications/Operations/Validation/ValidationVerifications.md#single-root-hierarchy-ownership-validation-test)
---

### Two-Pass Validation Strategy

The system shall execute model validation in two phases: element collection and graph validation.

#### Details
The system shall define two-pass validation behavior.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Two-Pass Validation Behavior](Behaviors.md#two-pass-validation-behavior)
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/model.rs)
---

### Type Validation Error Requirement

The system shall display all valid type options when type validation fails.

#### Details
- Invalid element types shall show list of valid element types including custom type pattern
- Invalid relation types shall show list of valid relation types
- Element type list format: "type1, type2, ... For custom types use: other-TYPENAME"
- Relation type list format: "type1, type2, ..." (alphabetically sorted)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Type Validation Error Behavior](Behaviors.md#type-validation-error-behavior)
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * satisfiedBy: [element.rs](../../../crates/reqvire-core/src/element.rs)
  * satisfiedBy: [relation.rs](../../../crates/reqvire-core/src/relation.rs)
  * verifiedBy: [Type Validation Errors Test](../../Verifications/Operations/Validation/ValidationVerifications.md#type-validation-errors-test)
---

### Validation Error Handling

The system shall maintain consistent error handling across both validation passes, collecting all errors within each pass before reporting.

#### Details
Error handling shall follow these principles:

1. **Complete pass execution**: Each pass runs to completion, collecting all errors found
2. **Aggregated reporting**: All errors from a pass are reported together
3. **Early termination**: Process exits after reporting errors from either pass
4. **Existing error format**: Error messages maintain the current format and structure
5. **Exit codes**: Non-zero exit codes indicate validation failures

This ensures users see all relevant errors at once rather than fixing issues one at a time.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Validation Error Reporting Behavior](Behaviors.md#validation-error-reporting-behavior)
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * satisfiedBy: [error.rs](../../../crates/reqvire-core/src/error.rs)
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/model.rs)
  * verifiedBy: [Invalid Relations Test](../../Verifications/Operations/Validation/ValidationVerifications.md#invalid-relations-test)
---

### Validate Markdown Structure

The system shall validate the Markdown structure of system model to ensure compliance with formatting standards.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Markdown Structure Validator](#markdown-structure-validator)
  * specify: [Validating Structures](../BehaviorValidationOperationsFeature.md#validating-structures)
---

### Markdown Structure Validator

The system shall implement a markdown structure validator that enforces Reqvire's requirements for header levels, element structure, relation formatting, and other markdown-specific syntax rules, reporting violations with line numbers and suggested fixes.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Validate Markdown Structure](#validate-markdown-structure)
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/model.rs)
  * satisfiedBy: [parser.rs](../../../crates/reqvire-core/src/parser.rs)
  * verifiedBy: [Invalid Header Structure Test](../../Verifications/Operations/Validation/ValidationVerifications.md#invalid-header-structure-test)
  * verifiedBy: [Invalid Relations Test](../../Verifications/Operations/Validation/ValidationVerifications.md#invalid-relations-test)
---

### Validate Relation Types

The system shall validate relation types and allow only supported types.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Relation Element Type Validator](#relation-element-type-validator)
  * specify: [Validating Structures](../BehaviorValidationOperationsFeature.md#validating-structures)
---

### Relation Element Type Validator

The system shall implement validation that verifies relation endpoints have appropriate element types based on the relation type, following the Element Type Relation Compatibility matrix.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Relation Element Type Validator Contract Specification](Specifications.md#relation-element-type-validator-contract-specification)
  * derivedFrom: [Validate Relation Types](#validate-relation-types)
  * satisfiedBy: [graph_registry.rs](../../../crates/reqvire-core/src/graph_registry.rs)
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/model.rs)
  * satisfiedBy: [parser.rs](../../../crates/reqvire-core/src/parser.rs)
  * verifiedBy: [Element Type Relation Compatibility Test](../../Verifications/Operations/Validation/ValidationVerifications.md#element-type-relation-compatibility-test)
  * verifiedBy: [Invalid Relations Test](../../Verifications/Operations/Validation/ValidationVerifications.md#invalid-relations-test)
  * verifiedBy: [Single Element Contract Validation Test](../../Verifications/Operations/Validation/ValidationVerifications.md#single-element-contract-validation-test)
---
