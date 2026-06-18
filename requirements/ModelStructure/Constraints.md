# Elements

### Attachment Hierarchical Independence Constraint

Rules for preventing attachments within the same requirement hierarchy.

**For contract elements:**
A contract element can only be attached to a requirement if that requirement has NO hierarchical relationship with the requirement that owns the contract via `definedBy`:
- Cannot attach to the requirement that has `definedBy` to this contract
- Cannot attach to any parent (ancestor) of that requirement via derivedFrom chain
- Cannot attach to any child (descendant) of that requirement via derive chain

**Upstream attachment propagation:**
If an attachment (contract identifier) is already attached to an ancestor requirement in the derivation hierarchy, descendants cannot attach the same target:
- Attachments propagate downstream through the derivedFrom chain
- Re-attaching at a descendant level is redundant
- Only the highest-level requirement in a hierarchy branch should attach

Only requirements in a separate branch of the hierarchy (no derivedFrom chain connecting them to the owner or existing attacher) may attach the contract.

**Rationale**: Attachments enable cross-submodel traceability while maintaining stakeholder separation. Attachments within the same hierarchy are redundant since traceability already flows through the definedBy relationship and propagates to derived requirements.

#### Metadata
  * type: constraint
---

### Attachment Satisfied Contract Constraint

Rules requiring attachment targets to have compatible ownership before being attachable.

An attachment target is valid only when it matches the attaching element family:
- Ontology elements are not attachable; ontology vocabulary bindings use `#### Concept References` or semantic-contract `use` relations.
- Requirement attachments target reusable requirement-owned non-semantic-contract elements only: `source`, `constraint`, `behavior`, `specification`, `state`, or `input-output`.
- Only requirement elements may author Attachments subsections.
- Verification elements must express evidence through `satisfiedBy` and verified targets through `verify`; they must not author attachments.
- Requirement-owned non-semantic-contract elements must have a `define` relation to exactly one compatible requirement owner before they can be attached.
- Capabilities must not own contracts; invalid capability contract ownership edges are not cross-subgraph attachment contracts.
- Contract elements without a valid `define` relation cannot be attached anywhere.

Attachment compatibility is defined by the Reqvire relation, requirement, contract, and semantic-contract model contracts.

**Rationale**: Enforces model hygiene by keeping ontology vocabulary in concept references and semantic-contract `use` relations, while ensuring reusable requirement contracts are owned by exactly one requirement before being referenced elsewhere.

#### Metadata
  * type: constraint
---

### Attachment Subgraph Direction Constraint

Rules for keeping cross-subgraph attachment contracts one-directional.

#### Details
For cross-subgraph contract attachments, flow is defined from the attaching element's capability-root hierarchy to the defining owner's capability-root hierarchy.

**One-direction invariant:**
- If subgraph `A` attaches a contract owned by subgraph `B`, then subgraph `B` must not attach contracts owned by subgraph `A`
- This rule applies at the capability-root hierarchy level, not only to directly involved requirements or capabilities

**Rationale**: Attachment contracts are used to model waterfall-style dependency flow between subgraphs. Allowing reverse attachment flow between the same two subgraphs breaks boundary directionality and undermines attachment contracts as one-way dependency edges.

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
- `trace` remains trace-only for custom element types

#### Metadata
  * type: constraint
---
