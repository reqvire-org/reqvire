# Test Requirements

This file contains test requirements for validating the traceability matrix functionality.

## System Requirements

### Root Requirement Alpha

This is a top-level requirement without parent relations.

#### Details

This requirement should appear as a root in the traceability matrix.

#### Relations
  * verifiedBy: [Verifications.md/Verification Alpha](Verifications.md#verification-alpha)
  * verifiedBy: [Verifications.md/Verification Beta](Verifications.md#verification-beta)

---

### Root Requirement Beta

This is another top-level requirement without parent relations.

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

#### Details

This requirement should appear with ❌ in the verification status column.

---