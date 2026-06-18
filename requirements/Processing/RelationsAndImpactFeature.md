# Elements

### Trace Changes in System Model

As a **System Engineer**, I want Reqvire to trace model changes through requirements, contracts, semantic contracts, reused_contract_context, verifications, and implementation evidence, so that I can identify what must be reviewed after a change.

#### Details
Trace changes in system model is the capability for impact propagation, auditable dependency paths, and review routing after model changes.

Change impact uses native Reqvire relations, requirement-owned contract reused_contract_context, semantic-contract `use`/`constrain` relations, and explicit concept references. Semantic references are model facts, not capability ontology reused_contract_context.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: high
  * risk: high
  * status: approved

#### Relations
  * specifiedBy: [Tracing Structural Changes](../Reports/ModelReports/ReportingRequirements.md#tracing-structural-changes)
---
