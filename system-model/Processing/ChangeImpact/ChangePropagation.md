# Element

## Metadata
  * type: specification

## Relations
  * define: [Requirements Change Propagation](ChangeImpactRequirements.md#requirements-change-propagation)

## ChangePropagation

# Change Propagation Specification

## Comparison and Change Classification

Compare a base workspace snapshot with the current validated snapshot. Match elements by stable name-derived IDs, retain workspace-relative identifiers, and distinguish content changes, additions, removals and relocations.

- A content change alters the semantic content hash of an element or a depended-on contract.
- A relocation changes a file path or normalized identifier without changing the modeled meaning; report its addressing updates separately from semantic impact.
- Contract Bindings and Contract References remain distinct dependency facts. Their membership and target content contribute to change-impact analysis without treating references as implementation obligations.
- Local files and external URLs are resource or evidence targets of compatible relations. They are not Contract Binding or Contract Reference targets.

## Propagation

Use canonical relations, both contract dependency kinds, authored concept references and semantic-contract ontology-use/requirement-constraint context to construct auditable impact paths.

1. Identify modified elements and changed dependency facts in the compared snapshots.
2. Follow the impact-propagating directions declared by the relation and change-impact vocabularies. Preserve each path's trigger and source context.
3. Propagate changed contract content to its binding and reference consumers, then follow those consumers' downstream context. A Contract Reference triggers review without contributing implementation evidence, blockers or fulfillment.
4. Identify affected requirements, owned definitions, implementation/evidence artifacts and concrete verifications. Requirements reached by more than one acyclic path are assessed once while their dependency provenance remains available.
5. Mark affected verification intent for review and re-execution when the verified obligation changes. Changes to verification-only content do not invent implementation changes.
6. Compute per-branch requirement common ancestors for impact scope. Preserve capability-root ownership rather than using a contract dependency as a hierarchy edge.

Reject invalid model dependency cycles before graph-dependent reports. Valid converging paths terminate deterministically and retain stable output ordering.

## Contract Dependency Example

The following excerpts illustrate independently owned contract consumption. A complete model supplies the owning capabilities, verification objectives and any referenced artifacts.

```markdown
### Service Level Requirement

The system SHALL satisfy the service availability obligation.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Service Assurance](Capabilities.md#service-assurance)

---

### Service Level Specification

Definition of the availability metric, target and measurement interval.

#### Metadata
  * type: specification

#### Relations
  * define: [Service Level Requirement](#service-level-requirement)

---

### Service Implementation Requirement

The system SHALL implement the shared availability obligation.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Service Level Specification](#service-level-specification)

#### Relations
  * specify: [Service Delivery](Capabilities.md#service-delivery)

---

### Service Documentation Requirement

The system SHALL document the availability obligation.

#### Metadata
  * type: requirement

#### Contract References
  * [Service Level Specification](#service-level-specification)

#### Relations
  * specify: [Service Documentation](Capabilities.md#service-documentation)
```

Changing the availability target requires review of both consumers. Implementing the documentation requirement does not fulfill the service-level owner. Moving the specification while preserving its content rewrites addressing and remains a relocation rather than a semantic content change.

## Reporting and Filtering

Return the existing structured added, changed, removed, relocated, impact-scope and invalidated-verification buckets. Each affected node retains canonical identity, change kind, propagation trigger and linked review context.

Smart filtering suppresses repeated presentation of the same primary change while preserving complete paths and review evidence. Group changes by their requirement context, order equivalent results deterministically, and retain cross-category provenance. Text and diagram rendering consume the evaluated report; MCP returns structured evidence without requiring terminal-text parsing.

Generated visualization labels identify real relation or dependency semantics, including `derivedFrom`, `specify`, `definedBy`, `verifiedBy`, `satisfiedBy`, `contract_bindings` and `contract_references` where applicable. A report trace is a computed path, not an authored `trace` relation.
