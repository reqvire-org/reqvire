# Elements


### Test Capability Test Element Type Relation Compatibility Valid Trace Specifications

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

### Capability A

A capability for testing trace relations.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-element-type-relation-compatibility-valid-trace-specifications)
---

### Capability B

Another capability.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-element-type-relation-compatibility-valid-trace-specifications)
  * trace: [Capability A](#capability-a)

---

### System Requirement A

System requirement with trace to capability.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability A](#capability-a)
  * trace: [Capability B](#capability-b)

---

### Test Verification A

Test verification with trace to requirement.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Capability A](#capability-a)
  * trace: [System Requirement A](#system-requirement-a)

---

### Test Verification B

Test verification with trace to another verification.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Capability B](#capability-b)
  * trace: [Test Verification A](#test-verification-a)

---

### Analysis Verification with Trace

Analysis verification can use trace.

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Capability A](#capability-a)
  * trace: [Capability B](#capability-b)

---

### Inspection Verification with Trace

Inspection verification can use trace.

#### Metadata
  * type: inspection-verification

#### Relations
  * verify: [Capability A](#capability-a)
  * trace: [Test Verification A](#test-verification-a)

---

### Demonstration Verification with Trace

Demonstration verification can use trace.

#### Metadata
  * type: demonstration-verification

#### Relations
  * verify: [Capability A](#capability-a)
  * trace: [Analysis Verification with Trace](#analysis-verification-with-trace)

---

### Other Element A

Other type can only use trace relations.

#### Metadata
  * type: other-other

#### Relations
  * trace: [Capability A](#capability-a)

---

### Other Element B

Other type tracing to another other element.

#### Metadata
  * type: other-other

#### Relations
  * trace: [Other Element A](#other-element-a)

---
