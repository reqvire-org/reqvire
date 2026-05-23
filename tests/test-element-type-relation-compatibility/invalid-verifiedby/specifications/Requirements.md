# Elements


### Test Feature Test Element Type Relation Compatibility Invalid Verifiedby Specifications

Test feature root for migrated requirement fixtures.

#### Metadata
  * type: feature
---

### Target Feature

A feature for testing.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Feature](#test-feature-test-element-type-relation-compatibility-invalid-verifiedby-specifications)
---

### Target Test Verification

A test verification element.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Target Feature](#target-feature)

---

### Verification with VerifiedBy

INVALID: Verification elements cannot use verifiedBy (they use verify instead).

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Target Feature](#target-feature)
  * verifiedBy: [Target Test Verification](#target-test-verification)

---

### Requirement VerifiedBy Requirement

INVALID: verifiedBy must point to verification element, not requirement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target Feature](#target-feature)
  * verifiedBy: [Target Feature](#target-feature)

---

### Other Element with VerifiedBy

INVALID: Other type can only use trace relations.

#### Metadata
  * type: other-other

#### Relations
  * verifiedBy: [Target Test Verification](#target-test-verification)

---

### Requirement Using Verify

INVALID: Requirements should use verifiedBy, not verify.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target Feature](#target-feature)
  * verify: [Target Feature](#target-feature)

---
