# Elements

### Target User Requirement

A user requirement to be used as derivedFrom target.

#### Metadata
  * type: user-requirement

---

### Target System Requirement

A system requirement to be used as derivedFrom target.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target User Requirement](#target-user-requirement)

---

### Target Test Verification

A test verification element.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Target User Requirement](#target-user-requirement)

---

### Test Verification with DerivedFrom

INVALID: Test verification cannot use derivedFrom.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Target User Requirement](#target-user-requirement)
  * derivedFrom: [Target User Requirement](#target-user-requirement)

---

### Analysis Verification with DerivedFrom

INVALID: Analysis verification cannot use derivedFrom.

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Target User Requirement](#target-user-requirement)
  * derivedFrom: [Target User Requirement](#target-user-requirement)

---

### Inspection Verification with DerivedFrom

INVALID: Inspection verification cannot use derivedFrom.

#### Metadata
  * type: inspection-verification

#### Relations
  * verify: [Target User Requirement](#target-user-requirement)
  * derivedFrom: [Target User Requirement](#target-user-requirement)

---

### Demonstration Verification with DerivedFrom

INVALID: Demonstration verification cannot use derivedFrom.

#### Metadata
  * type: demonstration-verification

#### Relations
  * verify: [Target User Requirement](#target-user-requirement)
  * derivedFrom: [Target User Requirement](#target-user-requirement)

---

### Requirement Deriving from Verification

INVALID: Requirement cannot derive from verification element.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target Test Verification](#target-test-verification)

---

### Other Element with DerivedFrom

INVALID: Other type can only use trace relations.

#### Metadata
  * type: other

#### Relations
  * derivedFrom: [Target User Requirement](#target-user-requirement)

---
