# Elements

### Parent Requirement

A parent feature.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Child Requirement](#child-requirement)
  * derive: [Target Requirement](#target-requirement)
  * specify: [Test Feature Test Format Full Relations Specifications Requirements Md](#test-feature-test-format-full-relations-specifications-requirements-md)
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

### Test Feature Test Format Full Relations Specifications Requirements Md

Test feature root for migrated requirement fixtures.

#### Metadata
  * type: feature

#### Relations
  * specifiedBy: [Parent Requirement](#parent-requirement)
---
