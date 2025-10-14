# Test Requirements

This file contains test requirements for validating the traceability matrix functionality.

## System Requirements

### Root Requirement Alpha

This is a top-level requirement without parent relations.

#### Metadata
* type: user-requirement

#### Details

This requirement should appear as a root in the traceability matrix.

#### Relations
  * verifiedBy: [Verifications.md/Verification Alpha](Verifications.md#verification-alpha)
  * verifiedBy: [Verifications.md/Verification Beta](Verifications.md#verification-beta)

---

### Root Requirement Beta

This is another top-level requirement without parent relations.

#### Metadata
* type: user-requirement

#### Details

This requirement should appear as a separate root in the traceability matrix.

#### Relations
  * verifiedBy: [Verifications.md/Verification Gamma](Verifications.md#verification-gamma)

---

### Child Requirement A1

This is a first-level child requirement that has a parent.

#### Details

This requirement should appear indented under Root Requirement Alpha in the matrix.

#### Relations
  * derivedFrom: [Requirements.md/Root Requirement Alpha](Requirements.md#root-requirement-alpha)
  * verifiedBy: [Verifications.md/Verification Alpha](Verifications.md#verification-alpha)

---

### Child Requirement A2

This is another first-level child requirement that has a parent.

#### Details

This requirement should appear indented under Root Requirement Alpha in the matrix.

#### Relations
  * derivedFrom: [Requirements.md/Root Requirement Alpha](Requirements.md#root-requirement-alpha)
  * verifiedBy: [Verifications.md/Verification Delta](Verifications.md#verification-delta)

---

### Grandchild Requirement A1-1

This is a second-level child requirement showing deeper hierarchy.

#### Details

This requirement should appear with double indentation under Child Requirement A1 in the matrix.

#### Relations
  * derivedFrom: [Requirements.md/Child Requirement A1](Requirements.md#child-requirement-a1)
  * verifiedBy: [Verifications.md/Verification Beta](Verifications.md#verification-beta)

---

### Child Requirement B1

This is a child requirement under the second root.

#### Details

This requirement should appear indented under Root Requirement Beta in the matrix.

#### Relations
  * derivedFrom: [Requirements.md/Root Requirement Beta](Requirements.md#root-requirement-beta)
  * verifiedBy: [Verifications.md/Verification Delta](Verifications.md#verification-delta)

---

### Unverified Requirement

This is a requirement without any verification.

#### Metadata
* type: user-requirement

#### Details

This requirement should appear with ❌ in the verification status column.

---

### Parent With Unverified Child

This is a parent requirement that has both verified and unverified children.

#### Metadata
* type: user-requirement

#### Details

This requirement should be marked as unverified (❌) due to roll-up strategy - not all children are verified.

#### Relations
  * verifiedBy: [Verifications.md/Verification Gamma](Verifications.md#verification-gamma)

---

### Verified Child of Mixed Parent

This is a verified child under Parent With Unverified Child.

#### Details

This child has direct verification but parent should still be unverified due to sibling.

#### Relations
  * derivedFrom: [Requirements.md/Parent With Unverified Child](Requirements.md#parent-with-unverified-child)
  * verifiedBy: [Verifications.md/Verification Alpha](Verifications.md#verification-alpha)

---

### Unverified Child of Mixed Parent

This is an unverified child under Parent With Unverified Child.

#### Details

This child has NO verification, causing parent to be unverified via roll-up.

#### Relations
  * derivedFrom: [Requirements.md/Parent With Unverified Child](Requirements.md#parent-with-unverified-child)

---

### Parent Verified Through Children Only

This is a parent requirement with NO direct verification but all children are verified.

#### Metadata
* type: user-requirement

#### Details

This requirement has no direct verifiedBy relations but should be marked as verified (✅) because ALL children are verified via roll-up strategy.

---

### Verified Child of Parent Verified Through Children

This is a verified child under Parent Verified Through Children Only.

#### Details

This child has direct verification, contributing to parent's verified status.

#### Relations
  * derivedFrom: [Requirements.md/Parent Verified Through Children Only](Requirements.md#parent-verified-through-children-only)
  * verifiedBy: [Verifications.md/Verification Delta](Verifications.md#verification-delta)

---

### Complex Root With Mixed Hierarchy

This is a root requirement with NO direct verification, demonstrating roll-up through a complex hierarchy.

#### Metadata
* type: user-requirement

#### Details

This requirement has no direct verifiedBy relations. It has two children:
- One direct child (leaf) that is verified
- One child with its own child (grandchild), where only the grandchild is verified

The root should be verified (✅) because ALL leaf descendants are verified.

---

### Direct Leaf Child of Complex Root

This is a first-level child that is also a leaf (no further children).

#### Details

This is a leaf requirement with direct verification.

#### Relations
  * derivedFrom: [Requirements.md/Complex Root With Mixed Hierarchy](Requirements.md#complex-root-with-mixed-hierarchy)
  * verifiedBy: [Verifications.md/Verification Alpha](Verifications.md#verification-alpha)

---

### Intermediate Parent Child of Complex Root

This is a first-level child that has its own child (not a leaf).

#### Details

This requirement has NO direct verification but has a verified grandchild.

#### Relations
  * derivedFrom: [Requirements.md/Complex Root With Mixed Hierarchy](Requirements.md#complex-root-with-mixed-hierarchy)

---

### Grandchild Leaf Under Complex Root

This is a second-level child (grandchild of the root) and is a leaf.

#### Details

This is a leaf requirement with direct verification, contributing to intermediate parent and root verification.

#### Relations
  * derivedFrom: [Requirements.md/Intermediate Parent Child of Complex Root](Requirements.md#intermediate-parent-child-of-complex-root)
  * verifiedBy: [Verifications.md/Verification Beta](Verifications.md#verification-beta)

---
