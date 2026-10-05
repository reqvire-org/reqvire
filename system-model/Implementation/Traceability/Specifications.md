# Elements

### Requirement Implementation Coverage Logic Specification

Technical specification for requirement implementation coverage classification logic.

#### Details
Contract References MUST participate only in change-impact dependency traversal; they MUST NOT contribute to implementation coverage, terminal classification, or fulfillment-cycle validation.

Definitions used by this specification MUST follow the contracts bound by its owning requirement:
- Element type categories follow the Supported Element Types Specification.
- Satisfaction, ownership, and hierarchy relation meanings and directions follow the Relation Semantics Specification.
- Requirement and capability hierarchy, capability ownership, and the prohibition of direct capability satisfaction follow the Capability Model Structure Specification.
- Reusable contract types and their compatible single requirement owner follow the Contract Bindings Satisfied Contract Constraint.
- Binding applicability, placement, downstream inheritance, and redundant ancestor/descendant binding restrictions follow the Contract Bindings Hierarchical Independence Constraint.

Canonical implementation rollup semantics and assessment vocabulary are defined by the Reqvire Implementation Rollup Ontology, used by the Implementation Coverage Rollup Shape that constrains this specification's owning requirement.

**Required contributions**
- Evaluation MUST use the requirement fulfillment graph defined by the Relation Semantics Specification bound by its owning requirement. That graph combines immediate requirement children with explicit consumers of owned contracts and MUST pass cycle validation before coverage is reported.
- Binding requirements MUST be evaluated recursively. A binding at a parent covers its obligation through that parent's complete rollup; inherited applicability MUST NOT add duplicate contributions for its descendants.
- Contributions MUST be deduplicated by requirement identifier, including consumers binding several owned contracts.
- A requirement is terminal only when both contribution sets are empty. Merely owning an unused contract MUST NOT change terminal status.

**Coverage evaluation**
- A terminal requirement MUST be classified as covered if and only if it has at least one direct `satisfiedBy` artifact.
- A nonterminal requirement MUST be classified as covered if and only if every child requirement and every required contract consumer is covered recursively.
- Direct evidence on a nonterminal requirement MUST remain available, but MUST NOT override an uncovered contribution. Covered contributions collectively fulfill the owner without requiring direct owner evidence.
- Evaluation MUST retain contributing requirement identifiers, supporting artifact identifiers, and uncovered requirement identifiers explaining outstanding obligations.
- Capabilities MUST receive implementation coverage through specifying requirements and child capability rollup, without direct satisfaction. An empty capability MUST NOT supply implementation evidence.
- Specifications and other owned contracts MUST NOT count as implementation evidence or coverage units. Their bindings identify required contributions between requirements.

**Coverage sources**
- **Directly satisfied**: covered terminal requirement with direct artifact evidence.
- **Requirement rollup**: covered nonterminal requirement with child contributions only.
- **Contract consumer rollup**: covered nonterminal requirement with contract-consumer contributions only.
- **Combined rollup**: covered requirement with both child and contract-consumer contributions.
- **Uncovered**: terminal requirement lacking direct evidence or nonterminal requirement with at least one uncovered contribution.

**Aggregation and scope**
- Implementation coverage percentages MUST use distinct covered terminal requirements divided by distinct terminal requirements in the selected subject set. Nonterminal status and evidence MUST remain available separately from those percentage units.
- Terminal status, classifications, and supporting evidence MUST be determined using the complete validated model. Scope selection MUST NOT remove external contributions or turn a nonterminal requirement into a terminal requirement.
- External contributors and their artifacts MUST remain available as evidence without entering the selected scope's subject counts. Output formatting and empty-denominator presentation remain defined by reporting contracts.

#### Concept References
  * [Implementation Coverage](../../Thesaurus/Thesaurus.md#implementation-coverage)
  * [Terminal Requirement](../../Thesaurus/Thesaurus.md#terminal-requirement)

#### Metadata
  * type: specification

#### Relations
  * define: [Implementation Coverage Evaluation](ImplementationTraceabilityRequirements.md#implementation-coverage-evaluation)
---
