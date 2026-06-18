# Elements

### Parent Requirement

A parent capability.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Child Requirement](#child-requirement)
  * derive: [Target Requirement](#target-requirement)
  * specify: [Test Capability Test Format Full Relations Specifications Requirements Md](#test-capability-test-format-full-relations-specifications-requirements-md)
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

A requirement that is defined by a constraint.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Test Constraint](#test-constraint)
  * derivedFrom: [Parent Requirement](#parent-requirement)
---

### Test Capability Test Format Full Relations Specifications Requirements Md

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [Parent Requirement](#parent-requirement)
---

### Test Constraint

A constraint that defines the target requirement.

#### Metadata
  * type: constraint

#### Relations
  * define: [Target Requirement](#target-requirement)
---
