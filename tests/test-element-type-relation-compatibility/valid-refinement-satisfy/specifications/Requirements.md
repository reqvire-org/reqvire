# Elements

### Target User Requirement

A user requirement that refinements can refine.

#### Metadata
  * type: user-requirement

#### Relations
  * refinedBy: [Specification with Satisfy](#specification-with-satisfy)

---

### Target System Requirement

A system requirement that refinements can refine.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target User Requirement](#target-user-requirement)
  * refinedBy: [Constraint with Satisfy](#constraint-with-satisfy)
  * refinedBy: [Behavior with Satisfy](#behavior-with-satisfy)

---

### Constraint with Satisfy

VALID: Constraint (refinement type) can refine requirement via refinedBy.

#### Metadata
  * type: constraint

---

### Behavior with Satisfy

VALID: Behavior (refinement type) can refine requirement via refinedBy.

#### Metadata
  * type: behavior

---

### Specification with Satisfy

VALID: Specification (refinement type) can refine requirement via refinedBy.

#### Metadata
  * type: specification

---
