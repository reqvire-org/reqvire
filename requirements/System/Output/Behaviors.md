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
