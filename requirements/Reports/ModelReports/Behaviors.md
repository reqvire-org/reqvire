# Elements

### Forward-Only Relation Traversal Behavior

When filtering model output from a starting requirement:
1. Follow only forward relations (`derive`, `satisfiedBy`, `verifiedBy`, `trace`).
2. Start from the specified root element when `--from` is provided.
3. Recursively traverse outgoing relations until leaf elements are reached.
4. Do not traverse backward relations during forward traversal mode.
5. When no `--from` filter is provided, include the complete model.

#### Metadata
  * type: behavior

#### Relations
  * define: [Forward-Only Relation Traversal](ReportingRequirements.md#forward-only-relation-traversal)
---

### Implementation Coverage Behavior

Coverage behavior for requirement implementation reporting.

#### Details
Implementation coverage source vocabulary is defined by the Reqvire report ontology. Implementation coverage classification shall follow:

- **Directly satisfied**: requirement has one or more `satisfiedBy` relations.
- **Contract via attachment**: requirement owns contract elements via `definedBy`, and at least one owned contract is attached by a requirement that is directly satisfied.
- **Contract via child**: requirement owns contract elements via `definedBy`, and at least one derived descendant requirement has `satisfiedBy`.
- **Uncovered**: requirement has no coverage evidence from the above sources.

Rules:
- Scope includes only `requirement` elements. Capability elements are excluded from direct implementation coverage and receive implementation coverage through capability roll-up.
- Refinement-contract attachment propagation uses only refinement element identifiers as contracts.
- Generic derivation roll-up is not used for implementation coverage.
- Coverage source and evidence identifiers shall be reported in text and JSON outputs.

#### Metadata
  * type: behavior
---

### Mermaid Diagram Link Behavior

Link behavior for generated Mermaid diagram source.

#### Details
Generated Mermaid diagrams shall emit click directives for nodes with navigable model targets.

Rules:
- Element nodes include clickable links to the referenced element or source anchor.
- Link targets use relative paths or canonical Explorer routes appropriate to the output context.
- External links remain explicit external targets.

Browser zoom, pan, reset controls, rendered hover behavior, and Mermaid viewport styling are WebInterface presentation concerns.

#### Metadata
  * type: behavior
---

### Reverse Relation Traversal Behavior

When reverse mode is enabled for model traversal:
1. Follow backward relations (derivedFrom, satisfy, verify) instead of forward relations
2. Start from leaf elements (elements with no outgoing forward relations) when no specific element is specified
3. Build tree structure from leaves upward toward model roots
4. When a specific starting element is provided, start from that element and traverse upward
5. Report "Reverse" direction indicator in output metadata

#### Metadata
  * type: behavior
---

### Start Element Type Filter Behavior

When start element type filtering is enabled:
1. Accept a list of element types to filter starting points
2. Use only elements matching specified types as traversal starting points
3. When combined with reverse traversal, filter leaf elements by type
4. When used with forward traversal, filter root elements by type
5. Support all element types as filter values

#### Metadata
  * type: behavior
---

### Verification Coverage Philosophy Behavior

Coverage evaluation philosophy for verification reporting.

#### Details
**Leaf Requirements (MUST be verified):**
- Requirements that don't derive other requirements
- Must have verifiedBy relations to verification artifacts
- Verification gaps are flagged in coverage reports

**Parent/Intermediate Requirements (MAY be verified):**
- Requirements that derive other requirements
- Optional verification since leaf verifications may cover them
- System engineers responsible for ensuring verification scopes are sufficient

**Verification Relationships:**
- One verification may verify multiple leaf requirements (N:1)
- Change impact propagates from parent to leaf requirements and verifications
- AI systems can help create comprehensive verification scopes and prevent overlap

**Roll-up Strategy:**
- Leaf requirement verification provides coverage for parent requirements
- Parent verification is redundant if all derived leaves are verified
- Coverage percentages focus on leaf requirement verification status

#### Metadata
  * type: behavior
---
