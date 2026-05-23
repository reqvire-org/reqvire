# Elements


### Test Feature Test Element Type Relation Compatibility Invalid Derivedfrom Specifications

Test feature root for migrated requirement fixtures.

#### Metadata
  * type: feature
---

### Target Feature

A feature to be used as derivedFrom target.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Feature](#test-feature-test-element-type-relation-compatibility-invalid-derivedfrom-specifications)
---

### Target System Requirement

A system requirement to be used as derivedFrom target.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target Feature](#target-feature)

---

### Target Test Verification

A test verification element.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Target Feature](#target-feature)

---

### Test Verification with DerivedFrom

INVALID: Test verification cannot use derivedFrom.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Target Feature](#target-feature)
  * derivedFrom: [Target Feature](#target-feature)

---

### Analysis Verification with DerivedFrom

INVALID: Analysis verification cannot use derivedFrom.

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Target Feature](#target-feature)
  * derivedFrom: [Target Feature](#target-feature)

---

### Inspection Verification with DerivedFrom

INVALID: Inspection verification cannot use derivedFrom.

#### Metadata
  * type: inspection-verification

#### Relations
  * verify: [Target Feature](#target-feature)
  * derivedFrom: [Target Feature](#target-feature)

---

### Demonstration Verification with DerivedFrom

INVALID: Demonstration verification cannot use derivedFrom.

#### Metadata
  * type: demonstration-verification

#### Relations
  * verify: [Target Feature](#target-feature)
  * derivedFrom: [Target Feature](#target-feature)

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
  * type: other-other

#### Relations
  * derivedFrom: [Target Feature](#target-feature)

---
