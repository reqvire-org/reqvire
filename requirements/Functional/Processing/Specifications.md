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
- Text: `### Impact Scope` section with bulleted list of scope root requirements with links
- JSON: `"impact_scope"` array of objects with `name` and `element_id` fields

#### Metadata
  * type: specification
---

### Verification Roll-up Specification

Rules for determining verification status of parent requirements based on child verification.

#### Details
**Roll-up Rules:**
- When a requirement has children (through derivedFrom relations), it is considered verified if ALL of its child requirements are verified, regardless of whether the parent has direct verifiedBy relations
- When a requirement has no children (leaf requirement), it is considered verified if it has direct verifiedBy relations
- A parent with any unverified child shall be marked as unverified (❌), even if the parent itself has direct verification
- Verification status rolls up from leaf requirements through the entire parent chain to root requirements

**Applicability:**
This strategy applies to all verification matrices, coverage reports, and trace outputs.

#### Metadata
  * type: specification
---

### Verification Trace Tree Construction

Algorithm for building upward trace trees from verification elements to root requirements.

#### Details
**Purpose:**
Build a tree structure showing how verifications trace upward through the requirement hierarchy to root requirements. Used for trace reports, redundancy detection, and coverage analysis.

**Algorithm Steps:**

1. **Start from verification element**
   - Input: verification element with `verify` relations

2. **Get directly verified requirements**
   - Follow `verify` relations to get target requirements
   - Mark these as "directly verified" in the tree

3. **Traverse upward through derivedFrom**
   - For each requirement, follow `derivedFrom` relations to parent requirements
   - Continue recursively until reaching root requirements (no parents)

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
