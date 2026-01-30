# Elements

### Parent Requirement

A parent user requirement.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [Child Requirement](#child-requirement)
  * derive: [Target Requirement](#target-requirement)
---

### Child Requirement

A child requirement that derives from parent.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Requirement](#parent-requirement)
  * verifiedBy: [Test Verification](Verifications.md#test-verification)
---

### Target Requirement

A requirement that is refined by a constraint.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Requirement](#parent-requirement)
  * refinedBy: [Test Constraint](#test-constraint)
---

### Test Constraint

A constraint that refines the target requirement.

#### Metadata
  * type: constraint

#### Relations
  * refine: [Target Requirement](#target-requirement)
---
