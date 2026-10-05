# Elements

### Implementation Coverage Evaluation

The system SHALL evaluate requirement implementation coverage from implementation evidence and requirement relationships according to its implementation coverage logic specification.

#### Details
The system SHALL identify the evidence supporting each implementation coverage classification.

The system SHALL interpret element types, requirement and capability hierarchy, satisfaction relations, contract ownership, and binding inheritance according to its bound model contracts.

The system SHALL evaluate implementation coverage according to the implementation rollup semantics used by its constraining semantic contract.

#### Concept References
  * [Implementation Coverage](../../Thesaurus/Thesaurus.md#implementation-coverage)
  * [Terminal Requirement](../../Thesaurus/Thesaurus.md#terminal-requirement)

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Contract Reference Semantics Specification](../../ModelStructure/Specifications.md#contract-reference-semantics-specification)
  * [Supported Element Types Specification](../../ModelStructure/Specifications.md#supported-element-types-specification)
  * [Relation Semantics Specification](../../ModelStructure/Specifications.md#relation-semantics-specification)
  * [Capability Model Structure Specification](../../ModelStructure/Specifications.md#capability-model-structure-specification)
  * [Contract Bindings Satisfied Contract Constraint](../../ModelStructure/Constraints.md#contract-bindings-satisfied-contract-constraint)
  * [Contract Bindings Hierarchical Independence Constraint](../../ModelStructure/Constraints.md#contract-bindings-hierarchical-independence-constraint)

#### Relations
  * satisfiedBy: [fulfillment.rs](../../../crates/reqvire-core/src/graph_registry/fulfillment.rs)
  * satisfiedBy: [coverage.rs](../../../crates/reqvire-core/src/report/coverage.rs)
  * specify: [Implementation Traceability](../ImplementationFeature.md#implementation-traceability)
---
