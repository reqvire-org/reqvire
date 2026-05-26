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
  * refine: [Forward-Only Relation Traversal](Reporting.md#forward-only-relation-traversal)
---

### Implementation Coverage Behavior

Coverage behavior for requirement implementation reporting.

#### Details
Implementation coverage source vocabulary is defined by the Reqvire report ontology. Implementation coverage classification shall follow:

- **Directly satisfied**: requirement has one or more `satisfiedBy` relations.
- **Refinement-contract via attachment**: requirement owns refinement elements via `refinedBy`, and at least one owned refinement is attached by a requirement that is directly satisfied.
- **Refinement-contract via child**: requirement owns refinement elements via `refinedBy`, and at least one derived descendant requirement has `satisfiedBy`.
- **Uncovered**: requirement has no coverage evidence from the above sources.

Rules:
- Scope includes only `requirement` elements. Capability elements are excluded from direct implementation coverage and receive implementation coverage through capability roll-up.
- Refinement-contract attachment propagation uses only refinement element identifiers as contracts.
- Generic derivation roll-up is not used for implementation coverage.
- Coverage source and evidence identifiers shall be reported in text and JSON outputs.

#### Metadata
  * type: behavior
---

### Mermaid Diagram Interaction Behavior

Interactive behavior for Mermaid diagrams in HTML export.

#### Details
**Click Behavior:**
- Nodes are clickable links to element definition
- Links use relative paths from output location
- External links open in new tab

**Zoom/Pan:**
- Mouse wheel for zoom
- Click and drag for pan
- Reset button to restore initial view

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

### Short Mode Behavior

Behavior when `--short` flag is provided to CLI commands.

#### Details
Short mode reduces output verbosity for quick scanning:

**Text Output (--short without --json):**
- One line per element: `[type] identifier - name`
- Omit detailed content, relations, and metadata
- Suitable for piping to other tools

**JSON Output (--short with --json):**
- Omit verbose fields: `content`, `page_content`, `attachments`
- Omit computed fields: `element_count`, `total_elements`, `global_counters`
- Retain: `identifier`, `name`, `type`, `file_path`
- Retain: `relations` (for traceability)

**Rationale:**
- Reduces output size for large models
- Faster parsing by downstream tools
- Maintains essential traceability information

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
