# Elements


### Test Feature Test Element Type Relation Compatibility Invalid Satisfiedby Specifications

Test feature root for migrated requirement fixtures.

#### Metadata
  * type: feature
---

### Target Feature

A feature for testing.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Feature](#test-feature-test-element-type-relation-compatibility-invalid-satisfiedby-specifications)
---

### Target Test Verification

A test verification element.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Target Feature](#target-feature)

---

### Analysis Verification with SatisfiedBy

INVALID: Analysis verification cannot use satisfiedBy (only test-verification can).

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Target Feature](#target-feature)
  * satisfiedBy: [analysis-doc.txt](analysis-doc.txt)

---

### Inspection Verification with SatisfiedBy

INVALID: Inspection verification cannot use satisfiedBy.

#### Metadata
  * type: inspection-verification

#### Relations
  * verify: [Target Feature](#target-feature)
  * satisfiedBy: [inspection-checklist.txt](inspection-checklist.txt)

---

### Demonstration Verification with SatisfiedBy

INVALID: Demonstration verification cannot use satisfiedBy.

#### Metadata
  * type: demonstration-verification

#### Relations
  * verify: [Target Feature](#target-feature)
  * satisfiedBy: [demo-script.sh](demo-script.sh)

---

### Feature with SatisfiedBy

INVALID: User requirements cannot use satisfiedBy.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Feature](#test-feature-test-element-type-relation-compatibility-invalid-satisfiedby-specifications)
  * satisfiedBy: [impl.txt](impl.txt)

---

### Requirement SatisfiedBy Requirement

INVALID: satisfiedBy must point to implementation file, not another requirement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target Feature](#target-feature)
  * satisfiedBy: [Target Feature](#target-feature)

---

### Test Verification SatisfiedBy Verification

INVALID: satisfiedBy must point to implementation file, not another verification.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Target Feature](#target-feature)
  * satisfiedBy: [Target Test Verification](#target-test-verification)

---

### Other Element with SatisfiedBy

INVALID: Other type can only use trace relations.

#### Metadata
  * type: other-other

#### Relations
  * satisfiedBy: [impl.txt](impl.txt)

---
