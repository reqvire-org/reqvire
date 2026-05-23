# Elements

### Attachment Hierarchical Independence Constraint

Rules for preventing attachments within the same requirement hierarchy.

**For refinement elements:**
A refinement element can only be attached to a requirement if that requirement has NO hierarchical relationship with the requirement that owns the refinement via `refinedBy`:
- Cannot attach to the requirement that has `refinedBy` to this refinement
- Cannot attach to any parent (ancestor) of that requirement via derivedFrom chain
- Cannot attach to any child (descendant) of that requirement via derive chain

**Upstream attachment propagation:**
If an attachment (refinement identifier) is already attached to an ancestor requirement in the derivation hierarchy, descendants cannot attach the same target:
- Attachments propagate downstream through the derivedFrom chain
- Re-attaching at a descendant level is redundant
- Only the highest-level requirement in a hierarchy branch should attach

Only requirements in a separate branch of the hierarchy (no derivedFrom chain connecting them to the owner or existing attacher) may attach the refinement.

**Rationale**: Attachments enable cross-submodel traceability while maintaining stakeholder separation. Attachments within the same hierarchy are redundant since traceability already flows through the refinedBy relationship and propagates to derived requirements.

#### Metadata
  * type: constraint
---

### Attachment Subgraph Direction Constraint

Rules for keeping cross-subgraph attachment contracts one-directional.

#### Details
For cross-subgraph refinement attachments, flow is defined from the attaching element's feature-root hierarchy to the defining owner's feature-root hierarchy.

**One-direction invariant:**
- If subgraph `A` attaches a refinement owned by subgraph `B`, then subgraph `B` must not attach refinements owned by subgraph `A`
- This rule applies at the feature-root hierarchy level, not only to directly involved requirements or features

**Rationale**: Attachment contracts are used to model waterfall-style dependency flow between subgraphs. Allowing reverse attachment flow between the same two subgraphs breaks boundary directionality and undermines attachment contracts as one-way dependency edges.

#### Metadata
  * type: constraint
---

### Attachment Satisfied Refinement Constraint

Rules requiring attachment targets to have compatible ownership before being attachable.

An attachment target is valid only when it matches the attaching element family:
- Feature attachments target ontology elements only.
- Requirements must not attach ontology directly.
- Requirement attachments target requirement-owned refinements only: `semantic-contract`, `constraint`, `behavior`, `specification`, `state`, or `input-output`.
- Requirement-owned refinements must have a `refine` relation to exactly one compatible requirement owner before they can be attached.
- Feature-owned `source` refinements are not cross-subgraph attachment contracts.
- Refinements without a valid `refine` relation cannot be attached anywhere.

Attachment compatibility is defined by the Reqvire relation, feature, requirement, ontology, and semantic-contract model contracts.

**Rationale**: Enforces model hygiene by making feature-level ontology context the single vocabulary inheritance path, while ensuring reusable requirement contracts are owned by exactly one requirement before being referenced elsewhere.

#### Metadata
  * type: constraint
---

### Cross-Section Duplicate Constraint

Rules for detecting duplicate link targets across subsections.

**Cross-section duplicates** (same target in BOTH Relations AND Attachments):
- Applies to identifier targets
- Treated as semantic errors requiring user resolution
- The validate command fails with error
- The format command does not auto-fix (user must decide which section to keep)

**Within-section duplicates** (same entry repeated in Relations OR in Attachments):
- Treated as formatting issues, not validation errors
- The format fix operation removes duplicate entries
- The validate command does not fail for within-section duplicates

#### Metadata
  * type: constraint
---

### Element Type Relation Compatibility Constraint

Validation rules for element type and relation type combinations.

#### Details
Canonical relation compatibility is defined by the Reqvire relation ontology, core element type vocabulary, feature/requirement refinement vocabulary, and verification type vocabulary.

Validation shall enforce those model contracts so that:
- hierarchy relations stay within feature, requirement, or ontology hierarchy families
- `specify`/`specifiedBy` is the requirement-to-feature bridge
- `refine`/`refinedBy` follows subtype-compatible refinement ownership
- `satisfiedBy`/`satisfy` is limited to requirement and evidence-backed verification satisfaction
- `trace` remains trace-only for custom element types

#### Metadata
  * type: constraint
---

### Single Root Hierarchy Ownership Constraint

Rules for ensuring each requirement hierarchy belongs to exactly one feature root.

#### Details
- Feature hierarchy is defined by `derivedFrom`/`derive` between feature elements.
- Requirement hierarchy is defined by `derivedFrom`/`derive` between requirement elements.
- A top-level requirement must connect to its owning feature through `specify`/`specifiedBy`.
- Every requirement hierarchy element shall resolve to exactly one feature root by traversing requirement parents and then the owning feature hierarchy.
- Resolution count `0` is invalid (orphaned hierarchy from feature-root ownership perspective).
- Resolution count `>1` is invalid (ambiguous multi-root ownership).

This rule is a structural model invariant and shall be enforced as validation, not lint.

#### Metadata
  * type: constraint

#### Relations
  * refine: [Single Root Hierarchy Ownership](Validation.md#single-root-hierarchy-ownership)
---
