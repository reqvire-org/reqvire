# Elements


### Test Feature Test Format Full Relations Specifications Requirements Md

Test feature root for migrated requirement fixtures.

#### Metadata
  * type: feature
---

### Parent Requirement

A parent feature.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Feature](#test-feature-test-format-full-relations-specifications-requirements-md)
---

### Child Requirement

A child requirement that derives from parent.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Requirement](#parent-requirement)
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

---
