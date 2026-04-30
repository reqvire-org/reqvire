# Elements

### Root Requirement

This is the root requirement.

#### Metadata
  * type: user-requirement
---

### Alternative Parent Requirement

This is an alternative parent requirement also derived from root.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
---

### Multi-Path Redundant Requirement

This requirement reaches root through MULTIPLE convergent paths (via parent-requirement and alternative-parent-requirement). Since root is reachable through other paths, the direct root relation is redundant and safe to auto-remove.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Alternative Parent Requirement](#alternative-parent-requirement)
  * derivedFrom: [Parent Requirement](#parent-requirement)
---

### Parent Requirement

This is a parent requirement derived from root.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
---

### Cross-Submodel Linked Requirement

This requirement should only be modeled via attachment, not by hierarchical relation.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Requirement](#parent-requirement)
  * derivedFrom: [Standalone Child](#standalone-child)
---

### Leaf Requirement

This is a leaf requirement derived from parent.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Requirement](#parent-requirement)
  * verifiedBy: [Test Verification](Verifications/Tests.md#test-verification)
---

### Single-Chain Redundant Requirement

This requirement has derivedFrom to both parent and root through a SINGLE chain (safe to auto-remove the root relation).

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Requirement](#parent-requirement)
---

### Standalone Submodel Root

This is an independent root for cross-submodel boundary checks.

#### Metadata
  * type: user-requirement
---

### Standalone Child

This requirement is owned by `Standalone Submodel Root`.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Standalone Submodel Root](#standalone-submodel-root)
---
