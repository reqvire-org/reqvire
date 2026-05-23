# Elements


### Test Feature Test Element Type Relation Compatibility Valid Cases Specifications

Test feature root for migrated requirement fixtures.

#### Metadata
  * type: feature
---

### Feature Parent

A top-level feature.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Feature](#test-feature-test-element-type-relation-compatibility-valid-cases-specifications)
---

### Feature Child

User requirement deriving from another feature.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Feature](#test-feature-test-element-type-relation-compatibility-valid-cases-specifications)
  * derivedFrom: [Feature Parent](#feature-parent)

---

### System Requirement from User Req

System requirement deriving from feature.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Feature Parent](#feature-parent)

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
  * derivedFrom: [Feature Parent](#feature-parent)
  * satisfiedBy: [impl.rs](impl.rs)

---

### Test Verification with SatisfiedBy

Test verification can use satisfiedBy.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Feature Parent](#feature-parent)
  * satisfiedBy: [test.sh](test.sh)

---

### Requirement with Test Verification

Requirement verified by test verification.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Feature Parent](#feature-parent)
  * verifiedBy: [Test Verification with SatisfiedBy](#test-verification-with-satisfiedby)

---

### Requirement with Analysis Verification

Requirement verified by analysis verification.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Feature Parent](#feature-parent)
  * verifiedBy: [Analysis Verification](#analysis-verification)

---

### Analysis Verification

Analysis verification verifying a requirement.

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Feature Parent](#feature-parent)

---

### Inspection Verification

Inspection verification verifying a requirement.

#### Metadata
  * type: inspection-verification

#### Relations
  * verify: [Feature Parent](#feature-parent)

---

### Demonstration Verification

Demonstration verification verifying a requirement.

#### Metadata
  * type: demonstration-verification

#### Relations
  * verify: [Feature Parent](#feature-parent)

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

### Requirement Refined By Behavior

Requirement that asks for behavior definition, refined by a behavior element.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Feature Parent](#feature-parent)
  * refinedBy: [Behavior Element](#behavior-element)

---

### Requirement Refined By Specification

Requirement that asks for specification, refined by a specification element.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Feature Parent](#feature-parent)
  * refinedBy: [Specification Element](#specification-element)

---

### Requirement Refined By Constraint

Requirement that asks for constraint definition, refined by a constraint element.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Feature Parent](#feature-parent)
  * refinedBy: [Constraint Element](#constraint-element)

---
