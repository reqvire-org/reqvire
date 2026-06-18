# Elements

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

