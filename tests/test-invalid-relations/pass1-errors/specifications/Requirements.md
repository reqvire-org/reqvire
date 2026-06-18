# Elements


### Test Capability Test Invalid Relations Pass1 Errors Specifications Requirements Md

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

This document contains requirements with Pass 1 validation errors (parsing/format issues).


### Valid Requirement

This is a valid requirement.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-invalid-relations-pass1-errors-specifications-requirements-md)
  * satisfiedBy: [ValidImplementation.txt](ValidImplementation.txt)

---

### Valid Requirement

Duplicate element name - should trigger duplicate element error.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-invalid-relations-pass1-errors-specifications-requirements-md)
  * derivedFrom: [Valid Requirement](#valid-requirement)

---

### Requirement with Invalid Metadata Format

This requirement has invalid metadata format.

#### Metadata
This is not ok - invalid format.

#### Relations
  * derivedFrom: [Valid Requirement](#valid-requirement)

---

### Requirement with Invalid Relation Format

This requirement has invalid relations format.

#### Relations
  * derivedFrom: [Valid Requirement](#valid-requirement)
This is not ok - invalid format.

---

### Requirement with Invalid Relation Type

This requirement has an invalid relation type with a typo.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-invalid-relations-pass1-errors-specifications-requirements-md)
  * derivedFrom: [Valid Requirement](#valid-requirement)
  * satisfiedBBy: [ValidImplementation.txt](ValidImplementation.txt)

---

### Requirement with Duplicate Subsection

This requirement has duplicate subsection.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-invalid-relations-pass1-errors-specifications-requirements-md)
  * derivedFrom: [Valid Requirement](#valid-requirement)

#### Relations
  * satisfiedBy: [ValidImplementation.txt](ValidImplementation.txt)

---

### Requirement with Invalid Header Structure

This requirement has a level 5 header before reserved subsections.

##### This is invalid - level 5 header before Metadata

Level 5+ headers can only appear inside Details subsection.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-invalid-relations-pass1-errors-specifications-requirements-md)
  * derivedFrom: [Valid Requirement](#valid-requirement)

---
