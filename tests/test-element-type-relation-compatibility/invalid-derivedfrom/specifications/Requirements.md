# Elements



### Verification Objective

Verification objective for concrete verification fixtures in this test model.

#### Metadata
  * type: verification-objective
---
### Test Capability Test Element Type Relation Compatibility Invalid Derivedfrom Specifications

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

### Target Capability

A capability to be used as derivedFrom target.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-element-type-relation-compatibility-invalid-derivedfrom-specifications)
---

### Target System Requirement

A system requirement to be used as derivedFrom target.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target Capability](#target-capability)

---

### Target Test Verification

A test verification element.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: [Target Capability](#target-capability)

---

### Test Verification with DerivedFrom

INVALID: Test verification cannot use derivedFrom.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: [Target Capability](#target-capability)
  * derivedFrom: [Target Capability](#target-capability)

---

### Analysis Verification with DerivedFrom

INVALID: Analysis verification cannot use derivedFrom.

#### Metadata
  * type: analysis-verification

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: [Target Capability](#target-capability)
  * derivedFrom: [Target Capability](#target-capability)

---

### Inspection Verification with DerivedFrom

INVALID: Inspection verification cannot use derivedFrom.

#### Metadata
  * type: inspection-verification

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: [Target Capability](#target-capability)
  * derivedFrom: [Target Capability](#target-capability)

---

### Demonstration Verification with DerivedFrom

INVALID: Demonstration verification cannot use derivedFrom.

#### Metadata
  * type: demonstration-verification

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: [Target Capability](#target-capability)
  * derivedFrom: [Target Capability](#target-capability)

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
  * derivedFrom: [Target Capability](#target-capability)

---
