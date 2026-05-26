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

### Verification Roll-up Specification

Rules for determining verification status of parent requirements based on child verification.

#### Details
Canonical verification roll-up rules, evidence-backed verification semantics, and capability coverage states are defined by the Reqvire verification rollup ontology.

Implementation behavior:
- Coverage and trace outputs shall compute requirement verification state from the requirement hierarchy and direct `verifiedBy`/`verify` evidence.
- Parent requirement state shall be computed from child requirement state according to the roll-up contract.
- Capability coverage state shall be computed from requirements that specify the capability, child requirement roll-up, and descendant capability coverage.

**Applicability:**
This strategy applies to all verification matrices, coverage reports, and trace outputs.

#### Metadata
 * type: specification
---

### Verification Trace Tree Construction

Algorithm for building upward trace trees from verification elements to owning capability roots.

#### Details
**Purpose:**
Build a tree structure showing how verifications trace upward through the requirement hierarchy and owning capability context. Used for trace reports, redundancy detection, and coverage analysis.

**Algorithm Steps:**

1. **Start from verification element**
 - Input: verification element with `verify` relations

2. **Get directly verified elements**
 - Follow `verify` relations to get target capabilities or requirements
 - Mark these as "directly verified" in the tree

3. **Traverse upward through specify and derivedFrom**
 - For each directly verified capability, follow capability `derivedFrom` until reaching a capability root
 - For each requirement, follow `derivedFrom` relations to parent requirements
 - Follow `specify` to the owning capability when the requirement root is reached
 - Continue recursively through capability `derivedFrom` until reaching a capability root

4. **Build tree structure**
 - Preserve all paths (a requirement may be reached through multiple children)
 - Merge common ancestors into single nodes with multiple incoming edges
 - Track which nodes are directly verified vs. transitively traced

5. **Mark directly verified nodes**
 - Nodes with direct `verify` relations from the verification
 - Distinguished from nodes reached only through parent traversal

**Virtual Verification Pattern:**
For hierarchical relation analysis (not verification-specific), create a virtual verification element connected to all leaf requirements. Apply the same algorithm to detect redundant hierarchical relations.

**Usage:**
- Trace Report Generator: visualize verification-to-root paths
- Redundant Verify Detection: find ancestors that are both directly and transitively verified
- Redundant Hierarchical Detection: find derivedFrom relations with alternate paths

#### Metadata
 * type: specification
---
