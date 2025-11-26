# Elements

### Target User Requirement

A user requirement for testing.

#### Metadata
  * type: user-requirement

---

### Target Test Verification

A test verification element.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Target User Requirement](#target-user-requirement)

---

### Analysis Verification with SatisfiedBy

INVALID: Analysis verification cannot use satisfiedBy (only test-verification can).

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Target User Requirement](#target-user-requirement)
  * satisfiedBy: [analysis-doc.txt](analysis-doc.txt)

---

### Inspection Verification with SatisfiedBy

INVALID: Inspection verification cannot use satisfiedBy.

#### Metadata
  * type: inspection-verification

#### Relations
  * verify: [Target User Requirement](#target-user-requirement)
  * satisfiedBy: [inspection-checklist.txt](inspection-checklist.txt)

---

### Demonstration Verification with SatisfiedBy

INVALID: Demonstration verification cannot use satisfiedBy.

#### Metadata
  * type: demonstration-verification

#### Relations
  * verify: [Target User Requirement](#target-user-requirement)
  * satisfiedBy: [demo-script.sh](demo-script.sh)

---

### Requirement SatisfiedBy Requirement

INVALID: satisfiedBy must point to implementation file, not another requirement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target User Requirement](#target-user-requirement)
  * satisfiedBy: [Target User Requirement](#target-user-requirement)

---

### Test Verification SatisfiedBy Verification

INVALID: satisfiedBy must point to implementation file, not another verification.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Target User Requirement](#target-user-requirement)
  * satisfiedBy: [Target Test Verification](#target-test-verification)

---

### Other Element with SatisfiedBy

INVALID: Other type can only use trace relations.

#### Metadata
  * type: other

#### Relations
  * satisfiedBy: [impl.txt](impl.txt)

---
