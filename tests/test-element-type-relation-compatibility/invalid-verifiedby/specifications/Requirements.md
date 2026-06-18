# Elements


### Test Capability Test Element Type Relation Compatibility Invalid Verifiedby Specifications

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

### Target Capability

A capability for testing.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-element-type-relation-compatibility-invalid-verifiedby-specifications)
---

### Target Test Verification

A test verification element.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Verification Objective Using Verify](#verification-objective-using-verify)
  * verify: [Target Capability](#target-capability)

---

### Verification with VerifiedBy

INVALID: Verification elements cannot use verifiedBy (they use verify instead).

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Verification Objective Using Verify](#verification-objective-using-verify)
  * verify: [Target Capability](#target-capability)
  * verifiedBy: [Target Test Verification](#target-test-verification)

---

### Requirement VerifiedBy Requirement

INVALID: verifiedBy must point to verification element, not requirement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target Capability](#target-capability)
  * verifiedBy: [Target Capability](#target-capability)

---

### Other Element with VerifiedBy

INVALID: Other type cannot author canonical semantic relations.

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
  * derivedFrom: [Target Capability](#target-capability)
  * verify: [Target Capability](#target-capability)

---

### Verification Objective Using Verify

INVALID: Verification objectives organize concrete verification work and cannot directly verify requirements.

#### Metadata
  * type: verification-objective

#### Relations
  * verify: [Target Capability](#target-capability)

---

### Requirement VerifiedBy Verification Objective

INVALID: verifiedBy must point to a concrete verification, not a verification objective.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target Capability](#target-capability)
  * verifiedBy: [Verification Objective Using Verify](#verification-objective-using-verify)

---
