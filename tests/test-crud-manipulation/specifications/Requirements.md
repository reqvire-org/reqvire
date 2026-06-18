# Elements


### Test Capability Test Crud Manipulation Specifications Requirements Md

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

### Parent Capability

This is a parent requirement.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-crud-manipulation-specifications-requirements-md)
---

### Capability A

This is a test requirement for capability A.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Capability](#parent-capability)
---

### Capability B

This is a test requirement for capability B.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability A](#capability-a)
---

### Capability C

This is a test requirement for capability C with relations.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability A](#capability-a)
  * verifiedBy: [Test for Capability C](Verifications/Tests.md#test-for-capability-c)
---

### Complex chars, element/name example

This capability has special characters in its name including commas and slashes.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability A](#capability-a)
---

### Separate Requirement Branch

A separate requirement hierarchy for testing reused_contract_context.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-crud-manipulation-specifications-requirements-md)
---