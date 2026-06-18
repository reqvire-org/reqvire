# Elements

### Create Element Override Behavior

When the create element operation is invoked with override mode:
1. The system shall extract the element name from the markdown input (### Element Name pattern)
2. If an element with that name exists in the model:
   - The system shall check if any child elements would become orphaned (have no remaining parent hierarchical relations after deletion)
   - If any children would be orphaned, the system shall reject the operation
   - The system shall provide clear error message listing orphaned children with resolution guidance
   - If no children would be orphaned, the system shall remove the existing element first
3. The system shall then add the new element content to the target file
4. The operation shall be reported as "Update" rather than "Add"

#### Metadata
  * type: behavior
---

### Merge Content Transformation Behavior

Content transformation rules for the merge elements operation.

#### Details
**Content Merging:**
1. For each source element in order:
   - If source has main content (before any #### subsection): append to target's `#### Details`
   - If source has `#### Details`: create `#### Merged Details (Source Name)` with that content
2. Preserve target's original content structure
3. Merged content appears after target's existing Details section (or creates one)
4. For ontology elements:
   - Rewrite each source authored Turtle block to the target's resolved ontology base, prefix, and term namespace before consolidation
   - Fold all rewritten source ontology content into the target's single `#### Ontology` block
   - Preserve the target ontology metadata and keep the merged result normalized to one ontology block per element
   - Recompute or deduplicate inherited prefix bindings, document declarations, `owl:imports`, and SHACL-reachable ontology references after merge

**Relation Deduplication:**
- Relations are deduplicated by (relation_type, target) pair
- First occurrence is kept (target's relations take precedence)
- Different relation types to same target are NOT duplicates

**Reused Contract Context Deduplication:**
- Reused Contract Context are deduplicated by target identifier/path
- First occurrence is kept (target's reused_contract_context take precedence)

**Cross-Section Duplicate Check:**
- Before merge completes, validate no target appears in both Relations AND Reused Contract Context
- If cross-section duplicate detected, abort merge with error listing duplicates
- User must resolve by removing one of the duplicates before retrying

**Relation Redirection:**
- Find all elements with relations pointing to source elements
- Update those relations to point to target element's identifier
- This includes both forward and backward relations

#### Metadata
  * type: behavior
---
