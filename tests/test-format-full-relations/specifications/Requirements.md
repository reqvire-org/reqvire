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
