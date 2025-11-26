# Elements

### User Requirement A

A user requirement for testing trace relations.

#### Metadata
  * type: user-requirement

---

### User Requirement B

Another user requirement.

#### Metadata
  * type: user-requirement

#### Relations
  * trace: [User Requirement A](#user-requirement-a)

---

### System Requirement A

System requirement with trace to user requirement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [User Requirement A](#user-requirement-a)
  * trace: [User Requirement B](#user-requirement-b)

---

### Test Verification A

Test verification with trace to requirement.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [User Requirement A](#user-requirement-a)
  * trace: [System Requirement A](#system-requirement-a)

---

### Test Verification B

Test verification with trace to another verification.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [User Requirement B](#user-requirement-b)
  * trace: [Test Verification A](#test-verification-a)

---

### Analysis Verification with Trace

Analysis verification can use trace.

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [User Requirement A](#user-requirement-a)
  * trace: [User Requirement B](#user-requirement-b)

---

### Inspection Verification with Trace

Inspection verification can use trace.

#### Metadata
  * type: inspection-verification

#### Relations
  * verify: [User Requirement A](#user-requirement-a)
  * trace: [Test Verification A](#test-verification-a)

---

### Demonstration Verification with Trace

Demonstration verification can use trace.

#### Metadata
  * type: demonstration-verification

#### Relations
  * verify: [User Requirement A](#user-requirement-a)
  * trace: [Analysis Verification with Trace](#analysis-verification-with-trace)

---

### Other Element A

Other type can only use trace relations.

#### Metadata
  * type: other

#### Relations
  * trace: [User Requirement A](#user-requirement-a)

---

### Other Element B

Other type tracing to another other element.

#### Metadata
  * type: other

#### Relations
  * trace: [Other Element A](#other-element-a)

---
