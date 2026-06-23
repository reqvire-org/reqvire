# Elements

### Cross-Section Duplicate Constraint

Rules for detecting duplicate link targets across subsections.

**Cross-section duplicates** (same target in BOTH Relations AND Contract Bindings):
- Applies to identifier targets
- Treated as semantic errors requiring user resolution
- The validate command fails with error
- The format command does not auto-fix (user must decide which section to keep)

**Within-section duplicates** (same entry repeated in Relations OR in Contract Bindings):
- Treated as formatting issues, not validation errors
- The format fix operation removes duplicate entries
- The validate command does not fail for within-section duplicates

#### Metadata
  * type: constraint
---

### Single Root Hierarchy Ownership Constraint

Rules for ensuring each requirement hierarchy belongs to exactly one capability root.

#### Details
- Capability hierarchy is defined by `derivedFrom`/`derive` between capability elements.
- Requirement hierarchy is defined by `derivedFrom`/`derive` between requirement elements.
- A top-level requirement must connect to its owning capability through `specify`/`specifiedBy`.
- Every requirement hierarchy element shall resolve to exactly one capability root by traversing requirement parents and then the owning capability hierarchy.
- Resolution count `0` is invalid (orphaned hierarchy from capability-root ownership perspective).
- Resolution count `>1` is invalid (ambiguous multi-root ownership).

This rule is a structural model invariant and shall be enforced as validation, not lint.

#### Metadata
  * type: constraint

#### Relations
  * define: [Single Root Hierarchy Ownership](ValidationRequirements.md#single-root-hierarchy-ownership)
---
