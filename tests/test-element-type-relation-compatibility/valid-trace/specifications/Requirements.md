# Elements


### Test Feature Test Element Type Relation Compatibility Valid Trace Specifications

Test feature root for migrated requirement fixtures.

#### Metadata
  * type: feature
---

### Feature A

A feature for testing trace relations.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Feature](#test-feature-test-element-type-relation-compatibility-valid-trace-specifications)
---

### Feature B

Another feature.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Feature](#test-feature-test-element-type-relation-compatibility-valid-trace-specifications)
  * trace: [Feature A](#feature-a)

---

### System Requirement A

System requirement with trace to feature.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Feature A](#feature-a)
  * trace: [Feature B](#feature-b)

---

### Test Verification A

Test verification with trace to requirement.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Feature A](#feature-a)
  * trace: [System Requirement A](#system-requirement-a)

---

### Test Verification B

Test verification with trace to another verification.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Feature B](#feature-b)
  * trace: [Test Verification A](#test-verification-a)

---

### Analysis Verification with Trace

Analysis verification can use trace.

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Feature A](#feature-a)
  * trace: [Feature B](#feature-b)

---

### Inspection Verification with Trace

Inspection verification can use trace.

#### Metadata
  * type: inspection-verification

#### Relations
  * verify: [Feature A](#feature-a)
  * trace: [Test Verification A](#test-verification-a)

---

### Demonstration Verification with Trace

Demonstration verification can use trace.

#### Metadata
  * type: demonstration-verification

#### Relations
  * verify: [Feature A](#feature-a)
  * trace: [Analysis Verification with Trace](#analysis-verification-with-trace)

---

### Other Element A

Other type can only use trace relations.

#### Metadata
  * type: other-other

#### Relations
  * trace: [Feature A](#feature-a)

---

### Other Element B

Other type tracing to another other element.

#### Metadata
  * type: other-other

#### Relations
  * trace: [Other Element A](#other-element-a)

---
