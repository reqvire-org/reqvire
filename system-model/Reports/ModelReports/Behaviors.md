# Elements

### Forward-Only Relation Traversal Behavior

When filtering model output from a starting requirement:
1. Follow only forward relations (`derive`, `specifiedBy`, `definedBy`, `constrainedBy`, `usedBy`, `satisfiedBy`, `verifiedBy`, and `contract_bindings`) that are enabled for the selected traversal mode.
2. Start from the specified root element when `--from` is provided.
3. Recursively traverse outgoing relations until leaf elements are reached.
4. Do not traverse backward relations during forward traversal mode.
5. When no `--from` filter is provided, include the complete model.

#### Metadata
  * type: behavior

#### Relations
  * define: [Forward-Only Relation Traversal](ReportingRequirements.md#forward-only-relation-traversal)
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
