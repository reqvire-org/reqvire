# Elements

### Parent Requirement

A parent user requirement.

#### Metadata
  * type: user-requirement
---

### Child Requirement

A child requirement that derives from parent.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Requirement](#parent-requirement)
---

### Target Requirement

A requirement that will be satisfied by a constraint.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Requirement](#parent-requirement)
---

### Test Constraint

A constraint that satisfies the target requirement.

#### Metadata
  * type: constraint

#### Relations
  * satisfy: [Target Requirement](#target-requirement)
---
