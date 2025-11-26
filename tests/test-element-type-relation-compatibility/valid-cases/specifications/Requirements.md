# Elements

### User Requirement Parent

A top-level user requirement.

#### Metadata
  * type: user-requirement

---

### User Requirement Child

User requirement deriving from another user requirement.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [User Requirement Parent](#user-requirement-parent)

---

### System Requirement from User Req

System requirement deriving from user requirement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [User Requirement Parent](#user-requirement-parent)

---

### System Requirement from System Req

System requirement deriving from another system requirement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [System Requirement from User Req](#system-requirement-from-user-req)

---

### Requirement with SatisfiedBy

Requirement satisfied by implementation file.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [User Requirement Parent](#user-requirement-parent)
  * satisfiedBy: [impl.rs](impl.rs)

---

### User Requirement with SatisfiedBy

User requirement satisfied by implementation file.

#### Metadata
  * type: user-requirement

#### Relations
  * satisfiedBy: [impl.rs](impl.rs)

---

### Test Verification with SatisfiedBy

Test verification can use satisfiedBy.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [User Requirement Parent](#user-requirement-parent)
  * satisfiedBy: [test.sh](test.sh)

---

### Requirement with Test Verification

Requirement verified by test verification.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [User Requirement Parent](#user-requirement-parent)
  * verifiedBy: [Test Verification with SatisfiedBy](#test-verification-with-satisfiedby)

---

### Requirement with Analysis Verification

Requirement verified by analysis verification.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [User Requirement Parent](#user-requirement-parent)
  * verifiedBy: [Analysis Verification](#analysis-verification)

---

### Analysis Verification

Analysis verification verifying a requirement.

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [User Requirement Parent](#user-requirement-parent)

---

### Inspection Verification

Inspection verification verifying a requirement.

#### Metadata
  * type: inspection-verification

#### Relations
  * verify: [User Requirement Parent](#user-requirement-parent)

---

### Demonstration Verification

Demonstration verification verifying a requirement.

#### Metadata
  * type: demonstration-verification

#### Relations
  * verify: [User Requirement Parent](#user-requirement-parent)

---

### Behavior Element

A behavior element describing system behavior.

#### Metadata
  * type: behavior

---

### Specification Element

A specification element describing detailed specifications.

#### Metadata
  * type: specification

---

### Constraint Element

A constraint element describing system constraints.

#### Metadata
  * type: constraint

---

### Requirement Satisfied By Behavior

Requirement that asks for behavior definition, satisfied by a behavior element.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [User Requirement Parent](#user-requirement-parent)
  * satisfiedBy: [Behavior Element](#behavior-element)

---

### Requirement Satisfied By Specification

Requirement that asks for specification, satisfied by a specification element.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [User Requirement Parent](#user-requirement-parent)
  * satisfiedBy: [Specification Element](#specification-element)

---

### Requirement Satisfied By Constraint

Requirement that asks for constraint definition, satisfied by a constraint element.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [User Requirement Parent](#user-requirement-parent)
  * satisfiedBy: [Constraint Element](#constraint-element)

---
