# Elements

### Target User Requirement

A user requirement that refinements can satisfy.

#### Metadata
  * type: user-requirement

---

### Target System Requirement

A system requirement that refinements can satisfy.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target User Requirement](#target-user-requirement)

---

### Constraint with Satisfy

VALID: Constraint (refinement type) can have satisfy relation to requirement.

#### Metadata
  * type: constraint

#### Relations
  * satisfy: [Target System Requirement](#target-system-requirement)

---

### Behavior with Satisfy

VALID: Behavior (refinement type) can have satisfy relation to requirement.

#### Metadata
  * type: behavior

#### Relations
  * satisfy: [Target System Requirement](#target-system-requirement)

---

### Specification with Satisfy

VALID: Specification (refinement type) can have satisfy relation to requirement.

#### Metadata
  * type: specification

#### Relations
  * satisfy: [Target User Requirement](#target-user-requirement)

---
