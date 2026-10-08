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

Contract for building upward trace graphs from verification elements to owning capability roots.

#### Details
**Purpose:**
Build a normalized graph showing how verifications trace upward through the requirement hierarchy and owning capability context. Shared ancestors retain every incoming relation without repeated subtree expansion.

**Algorithm Steps:**

1. **Start from verification element**
 - Input: verification element with `verify` relations

2. **Get directly verified requirements**
 - Follow `verify` relations to get target requirements
 - Deduplicate resolved requirement identifiers and mark these as directly verified, including when they are also reached through another directly verified requirement

3. **Traverse upward through specify and derivedFrom**
 - For each requirement, follow `derivedFrom` relations to parent requirements
 - Follow requirement `specify` relations to owning capabilities
 - Continue through capability `derivedFrom` until reaching capability roots

4. **Build graph structure**
 - Preserve all paths (a requirement may be reached through multiple children)
 - Merge common ancestors into single nodes with multiple incoming edges
 - Track which nodes are directly verified vs. transitively traced
 - Expand each reachable element once per verification with an iterative traversal; retain each distinct labelled edge even when its target was already visited
 - Traversal work and stored graph size shall be proportional to reachable elements and their inspected relations, before deterministic output sorting. Shared paths shall not cause subtree copies or recursive stack growth
 - Cycles shall terminate without losing their closing edge; unresolved targets and unrelated relation families shall not introduce dangling graph edges. Normal model validation remains responsible for rejecting invalid models

5. **Mark directly verified nodes**
 - Nodes with direct `verify` relations from the verification
 - Distinguished from nodes reached only through parent traversal

**Report contract:**
- Each verification exposes `trace_graph.nodes` with unique `id`, `name`, `type`, and `is_directly_verified` records, and `trace_graph.edges` with unique `source`, `relation_type`, and `target` records. Edges represent upward `derivedFrom` and requirement-to-capability `specify`; verification edges are represented by `directly_verified_requirements`.
- Nodes sort by identifier; edges sort by source, relation type, then target. Direct requirement identifiers sort uniquely. File grouping and verification source order remain stable.
- `directly_verified_count` counts unique directly verified requirements; `total_requirements_in_tree` counts unique reachable requirements, excluding capability context. This existing count field retains its meaning.
- Concrete verifications without resolved direct requirements are omitted; verification objectives do not produce trace records.
- CLI, MCP, and Explorer Project Store use this same graph projection. Explorer consumers shall render from nodes and edges without reconstructing expanded path trees. The Project Store schema version changes with this representation.

**Virtual Verification Pattern:**
For hierarchical relation analysis (not verification-specific), create a virtual verification element connected to all leaf requirements. Apply the same algorithm to detect redundant hierarchical relations.

**Usage:**
- Trace Report Generator: visualize verification-to-root paths
- Redundant Verify Detection: find ancestors that are both directly and transitively verified
- Redundant Hierarchical Detection: find derivedFrom relations with alternate paths

#### Metadata
  * type: specification
---
