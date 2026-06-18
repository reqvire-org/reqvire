# Elements


### Test Capability Test Format Full Relations Specifications Requirements Md

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

### Parent Requirement

A parent capability.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-format-full-relations-specifications-requirements-md)
---

### Child Requirement

A child requirement that derives from parent.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Requirement](#parent-requirement)
---

### Target Requirement

A requirement that is defined by a constraint.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Requirement](#parent-requirement)
  * definedBy: [Test Constraint](#test-constraint)
---

### Test Constraint

A constraint that defines the target requirement.

#### Metadata
  * type: constraint

---
