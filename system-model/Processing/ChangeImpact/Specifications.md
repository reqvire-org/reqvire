# Elements

### Impact Scope Computation Specification

Algorithm for computing the minimal set of common parent requirements that cover all impacted elements in a change impact report.

#### Details
**Purpose:**
Provide reviewers with a high-level scope summary by finding the per-branch lowest common ancestors of all impacted requirements.

**Input Collection:**
1. Collect requirement element IDs from `changed` and `added` sections of the report
2. For `removed` elements: find their `derivedFrom` parent in the reference registry. If that parent still exists in the current registry, include it in the input set

**Bottom-up Merge Algorithm:**
1. Start with input set S of element IDs
2. For each element in S, resolve its immediate parent via `derivedFrom` relation in the current registry
3. Group elements by parent
4. For each parent with 2+ children in S: remove those children from S, add the parent
5. For parents with 1 child in S: keep the child as-is (no merging benefit)
6. Repeat steps 2-5 until S is stable (no changes in an iteration)
7. Return S sorted by element_id

**Edge Cases:**
- Elements with no `derivedFrom` parent: remain in S as-is (they are root scope)
- Deleted element whose parent is also deleted: walk up reference hierarchy until a parent existing in current model is found, or exclude
- Circular references: maintain visited set to prevent infinite loops

**Output:**
- Text: `### Impact Scope` section with bulleted list of impacted capability or requirement scope roots with links
- JSON: `"impact_scope"` array of objects with `name` and `element_id` fields

#### Metadata
  * type: specification
---

### Contract Reference Change Impact Specification

Contract changes propagate review impact to explicitly referencing requirements.

#### Details
Content changes to a referenced contract MUST reach its referencing requirements and their normal downstream requirement, artifact, and verification context. Reports MUST identify the traversed edge as `contract_references` and keep it distinct from `contract_bindings`.

A requirement's outgoing Contract Reference MUST NOT propagate its changes back to the referenced contract or owner. Adding or removing a reference MUST be reported as a change to the referencing requirement. Identifier-only relocation MUST preserve existing relocation semantics without treating unchanged contract content as changed.

Impact traversal MUST deduplicate assessment of reached elements across converging acyclic paths while preserving reference-edge provenance. A `contract_references` edge to a consumer MUST remain identifiable when the consumer is also reached through the contract owner's requirement hierarchy. Invalidated verification lists MUST contain each reached verification once. Cyclic references MUST fail model validation before change-impact reporting. Implementation coverage MUST remain independent of this review traversal.

#### Metadata
  * type: specification
---
