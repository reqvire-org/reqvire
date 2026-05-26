# Elements


### Test Capability Test Element Type Relation Compatibility Valid Cases Specifications

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability

#### Relations
  * refinedBy: [Capability Behavior Element](#capability-behavior-element)
  * verifiedBy: [Capability Level Verification](#capability-level-verification)
---

### Capability Parent

A top-level capability.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-element-type-relation-compatibility-valid-cases-specifications)
---

### Capability Child

User requirement deriving from another capability.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-element-type-relation-compatibility-valid-cases-specifications)
  * derivedFrom: [Capability Parent](#capability-parent)

---

### System Requirement from User Req

System requirement deriving from capability.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability Parent](#capability-parent)

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
  * derivedFrom: [Capability Parent](#capability-parent)
  * satisfiedBy: [impl.rs](impl.rs)

---

### Test Verification with SatisfiedBy

Test verification can use satisfiedBy.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Capability Parent](#capability-parent)
  * satisfiedBy: [test.sh](test.sh)

---

### Requirement with Test Verification

Requirement verified by test verification.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability Parent](#capability-parent)
  * verifiedBy: [Test Verification with SatisfiedBy](#test-verification-with-satisfiedby)

---

### Requirement with Analysis Verification

Requirement verified by analysis verification.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability Parent](#capability-parent)
  * verifiedBy: [Analysis Verification](#analysis-verification)

---

### Analysis Verification

Analysis verification verifying a requirement.

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Capability Parent](#capability-parent)

---

### Inspection Verification

Inspection verification verifying a requirement.

#### Metadata
  * type: inspection-verification

#### Relations
  * verify: [Capability Parent](#capability-parent)

---

### Demonstration Verification

Demonstration verification verifying a requirement.

#### Metadata
  * type: demonstration-verification

#### Relations
  * verify: [Capability Parent](#capability-parent)

---

### Capability Level Verification

Analysis verification directly verifying a capability.

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Test Capability](#test-capability-test-element-type-relation-compatibility-valid-cases-specifications)

---

### Behavior Element

A behavior element describing system behavior.

#### Metadata
  * type: behavior

---

### Capability Behavior Element

A behavior refinement owned by a capability.

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
  * derivedFrom: [Capability Parent](#capability-parent)
  * refinedBy: [Behavior Element](#behavior-element)

---

### Requirement Refined By Specification

Requirement that asks for specification, refined by a specification element.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability Parent](#capability-parent)
  * refinedBy: [Specification Element](#specification-element)

---

### Requirement Refined By Constraint

Requirement that asks for constraint definition, refined by a constraint element.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability Parent](#capability-parent)
  * refinedBy: [Constraint Element](#constraint-element)

---
