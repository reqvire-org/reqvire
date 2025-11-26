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

### Verification with VerifiedBy

INVALID: Verification elements cannot use verifiedBy (they use verify instead).

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Target User Requirement](#target-user-requirement)
  * verifiedBy: [Target Test Verification](#target-test-verification)

---

### Requirement VerifiedBy Requirement

INVALID: verifiedBy must point to verification element, not requirement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target User Requirement](#target-user-requirement)
  * verifiedBy: [Target User Requirement](#target-user-requirement)

---

### Other Element with VerifiedBy

INVALID: Other type can only use trace relations.

#### Metadata
  * type: other

#### Relations
  * verifiedBy: [Target Test Verification](#target-test-verification)

---

### Requirement Using Verify

INVALID: Requirements should use verifiedBy, not verify.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target User Requirement](#target-user-requirement)
  * verify: [Target User Requirement](#target-user-requirement)

---
