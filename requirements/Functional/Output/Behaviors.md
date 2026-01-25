# Elements

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
3. Build tree structure from leaves upward toward root requirements
4. When a specific starting element is provided, start from that element and traverse upward
5. Report "Reverse" direction indicator in output metadata

#### Metadata
  * type: behavior

#### Relations
  * satisfy: [Reverse Relation Traversal](Reporting.md#reverse-relation-traversal)
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

#### Relations
  * satisfy: [CLI Containment Command](../../Interfaces/CLI/Commands.md#cli-containment-command)
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

#### Relations
  * satisfy: [Start Element Type Filtering](Reporting.md#start-element-type-filtering)
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

#### Relations
  * satisfy: [Verification Coverage Report](Reporting.md#verification-coverage-report)
---
