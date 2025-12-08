# Elements

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

#### Relations
  * satisfy: [Verification Roll-up Strategy](VerificationTraces.md#verification-roll-up-strategy)
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
