# Elements


### Test Capability Test Element Type Relation Compatibility Invalid Satisfiedby Specifications

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

### Target Capability

A capability for testing.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-element-type-relation-compatibility-invalid-satisfiedby-specifications)
---

### Target Test Verification

A test verification element.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Target Capability](#target-capability)

---

### Analysis Verification with SatisfiedBy

INVALID: Analysis verification cannot use satisfiedBy (only test-verification can).

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Target Capability](#target-capability)
  * satisfiedBy: [analysis-doc.txt](analysis-doc.txt)

---

### Inspection Verification with SatisfiedBy

INVALID: Inspection verification cannot use satisfiedBy.

#### Metadata
  * type: inspection-verification

#### Relations
  * verify: [Target Capability](#target-capability)
  * satisfiedBy: [inspection-checklist.txt](inspection-checklist.txt)

---

### Demonstration Verification with SatisfiedBy

INVALID: Demonstration verification cannot use satisfiedBy.

#### Metadata
  * type: demonstration-verification

#### Relations
  * verify: [Target Capability](#target-capability)
  * satisfiedBy: [demo-script.sh](demo-script.sh)

---

### Capability with SatisfiedBy

INVALID: User requirements cannot use satisfiedBy.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-element-type-relation-compatibility-invalid-satisfiedby-specifications)
  * satisfiedBy: [impl.txt](impl.txt)

---

### Requirement SatisfiedBy Requirement

INVALID: satisfiedBy must point to implementation file, not another requirement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target Capability](#target-capability)
  * satisfiedBy: [Target Capability](#target-capability)

---

### Test Verification SatisfiedBy Verification

INVALID: satisfiedBy must point to implementation file, not another verification.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Target Capability](#target-capability)
  * satisfiedBy: [Target Test Verification](#target-test-verification)

---

### Other Element with SatisfiedBy

INVALID: Other type can only use trace relations.

#### Metadata
  * type: other-other

#### Relations
  * satisfiedBy: [impl.txt](impl.txt)

---
