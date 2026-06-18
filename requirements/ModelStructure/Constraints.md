# Elements

### Reused Contract Context Hierarchical Independence Constraint

Rules for preventing reused_contract_context within the same requirement hierarchy.

**For contract elements:**
A contract element can only be reused to a requirement if that requirement has NO hierarchical relationship with the requirement that owns the contract via `definedBy`:
- Cannot reuse to the requirement that has `definedBy` to this contract
- Cannot reuse to any parent (ancestor) of that requirement via derivedFrom chain
- Cannot reuse to any child (descendant) of that requirement via derive chain

**Upstream reused_contract_context propagation:**
If an reused_contract_context (contract identifier) is already reused to an ancestor requirement in the derivation hierarchy, descendants cannot reuse the same target:
- Reused Contract Context propagate downstream through the derivedFrom chain
- Re-reusesContract at a descendant level is redundant
- Only the highest-level requirement in a hierarchy branch should reuse

Only requirements in a separate branch of the hierarchy (no derivedFrom chain connecting them to the owner or existing reuser) may reuse the contract.

**Rationale**: Reused Contract Context enable cross-submodel traceability while maintaining stakeholder separation. Reused Contract Context within the same hierarchy are redundant since traceability already flows through the definedBy relationship and propagates to derived requirements.

#### Metadata
  * type: constraint
---

### Reused Contract Context Satisfied Contract Constraint

Rules requiring reused_contract_context targets to have compatible ownership before being reusable.

An reused_contract_context target is valid only when it matches the reusesContract element family:
- Ontology elements are not reusable; ontology vocabulary bindings use `#### Concept References` or semantic-contract `use` relations.
- Requirement reused_contract_context target reusable requirement-owned non-semantic-contract elements only: `source`, `constraint`, `behavior`, `specification`, `state`, or `input-output`.
- Only requirement elements may author Reused Contract Context subsections.
- Verification elements must express evidence through `satisfiedBy` and verified targets through `verify`; they must not author reused_contract_context.
- Requirement-owned non-semantic-contract elements must have a `define` relation to exactly one compatible requirement owner before they can be reused.
- Capabilities must not own contracts; invalid capability contract ownership edges are not cross-subgraph reused_contract_context contracts.
- Contract elements without a valid `define` relation cannot be reused anywhere.

Reused Contract Context compatibility is defined by the Reqvire relation, requirement, contract, and semantic-contract model contracts.

**Rationale**: Enforces model hygiene by keeping ontology vocabulary in concept references and semantic-contract `use` relations, while ensuring reusable requirement contracts are owned by exactly one requirement before being referenced elsewhere.

#### Metadata
  * type: constraint
---

### Reused Contract Context Subgraph Direction Constraint

Rules for keeping cross-subgraph reused_contract_context contracts one-directional.

#### Details
For cross-subgraph contract reused_contract_context, flow is defined from the reusesContract element's capability-root hierarchy to the defining owner's capability-root hierarchy.

**One-direction invariant:**
- If subgraph `A` reuses a contract owned by subgraph `B`, then subgraph `B` must not reuse contracts owned by subgraph `A`
- This rule applies at the capability-root hierarchy level, not only to directly involved requirements or capabilities

**Rationale**: Reused Contract Context contracts are used to model waterfall-style dependency flow between subgraphs. Allowing reverse reused_contract_context flow between the same two subgraphs breaks boundary directionality and undermines reused_contract_context contracts as one-way dependency edges.

#### Metadata
  * type: constraint
---

### Element Type Relation Compatibility Constraint

Validation rules for element type and relation type combinations.

#### Details
Canonical relation compatibility is defined by the Reqvire relation ontology, core element type vocabulary, capability/requirement contract vocabulary, and verification type vocabulary.

Validation shall enforce those model contracts so that:
- hierarchy relations stay within capability, requirement, or ontology hierarchy families
- `specify`/`specifiedBy` is the requirement-to-capability bridge
- `define`/`definedBy` is limited to requirement-owned subordinate details and contracts
- `satisfiedBy`/`satisfy` is limited to requirement and evidence-backed verification satisfaction
- custom `other` and `other-TYPENAME` extension elements cannot author canonical semantic relations

#### Metadata
  * type: constraint
---
