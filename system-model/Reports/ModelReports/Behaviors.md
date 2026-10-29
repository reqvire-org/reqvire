# Elements

### Forward-Only Relation Traversal Behavior

When generating forward model output:
1. Follow the canonical forward report categories: `derive`, `specifiedBy`, `satisfiedBy`, `definedBy`, `constrainedBy`, `use`, `narrower`, `related`, `exactMatch`, `closeMatch` and `verifiedBy`.
2. Resolve `--from` through the shared exact element selector and use that element as the traversal start.
3. Without `--from`, select default roots through the shared root rules, including ontology, capability and native concept roots; an explicit type filter selects its contracted start scope.
4. Traverse resolved targets recursively with one visited set per starting element. Authored inverse relations contribute their generated forward edges; inverse tokens themselves are not followed in this mode.
5. Keep `contract_bindings` and `contract_references` as separate target collections on element records rather than recursive relation categories.
6. Report `Forward` direction metadata and preserve deterministic nested output.

#### Metadata
  * type: behavior

#### Relations
  * define: [Forward-Only Relation Traversal](ReportingRequirements.md#forward-only-relation-traversal)
---

### Reverse Relation Traversal Behavior

When reverse model traversal is enabled:
1. Follow the registered inverse categories: `derivedFrom`, `specify`, `satisfy`, `define`, `constrain`, `usedBy`, `broader`, `related`, `exactMatch`, `closeMatch` and `verify`.
2. Without an explicit start, select leaves with no resolved outgoing forward model relations, subject to the start-type filter.
3. With `--from`, use the selected element and follow resolved inverse targets with one visited set for that start.
4. Keep symmetric concept associations and mappings bounded by the same visited-set behavior.
5. Report `Reverse` direction metadata; contract dependency collections remain separately represented.

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
- Child coverage can establish parent coverage without making a separate parent integration verification redundant; lint classifies redundant direct verification edges according to its own reachability policy
- Coverage percentages focus on leaf requirement verification status

#### Metadata
  * type: behavior
---
