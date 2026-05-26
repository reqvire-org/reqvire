# Elements


### Test Capability Test Invalid Relations Pass2 Errors Specifications Requirements Md

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

This document contains requirements with Pass 2 validation errors (relation validation issues).


### Valid Capability

This is a valid capability that serves as a parent.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-invalid-relations-pass2-errors-specifications-requirements-md)
  * trace: [ValidImplementation.txt](ValidImplementation.txt)

---

### System Requirement with Missing Parent Relation

This system requirement is missing parent relation.

#### Metadata
  * type: requirement

---

### Requirement with Missing Target

This requirement has a relation to a non-existent element.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Valid Capability](#valid-capability)
  * satisfiedBy: [NonExistentElement.md#missing-element](NonExistentElement.md#missing-element)

---

### Requirement with Incompatible Element

This requirement has incompatible element in relation - satisfiedBy cannot point to other requirement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Valid Capability](#valid-capability)
  * satisfiedBy: [Valid Capability](#valid-capability)

---

### Requirement with Circular Dependencies A

This requirement has circular dependency.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Valid Capability](#valid-capability)
  * derivedFrom: [Requirement with Circular Dependencies C](#requirement-with-circular-dependencies-c)

---

### Requirement with Circular Dependencies B

This requirement has circular dependency.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Valid Capability](#valid-capability)
  * derivedFrom: [Requirement with Circular Dependencies A](#requirement-with-circular-dependencies-a)

---

### Requirement with Circular Dependencies C

This requirement has circular dependency.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Valid Capability](#valid-capability)
  * derivedFrom: [Requirement with Circular Dependencies B](#requirement-with-circular-dependencies-b)

---

### Verification with Missing SatisfiedBy Target

This verification has a satisfiedBy relation pointing to a non-existing file.

#### Metadata
  * type: verification

#### Relations
  * verify: [Valid Capability](#valid-capability)
  * satisfiedBy: [non-existing-test-script.sh](non-existing-test-script.sh)

---

### Requirement with Missing VerifiedBy Target

This requirement has a verifiedBy relation pointing to a non-existing verification element.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Valid Capability](#valid-capability)
  * verifiedBy: [NonExistentVerification.md#missing-verification](NonExistentVerification.md#missing-verification)

---

### Valid Verification with Correct SatisfiedBy

This verification correctly uses satisfiedBy pointing to an existing test script.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Valid Capability](#valid-capability)
  * satisfiedBy: [test.sh](test.sh)

---

### Requirement SatisfiedBy Requirement Invalid

This requirement incorrectly uses satisfiedBy pointing to another requirement (should fail).

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Valid Capability](#valid-capability)
  * satisfiedBy: [Valid Capability](#valid-capability)

---

### Verification SatisfiedBy Verification Invalid

This verification incorrectly uses satisfiedBy pointing to another verification (should fail).

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Valid Capability](#valid-capability)
  * satisfiedBy: [Valid Verification with Correct SatisfiedBy](#valid-verification-with-correct-satisfiedby)

---

### Requirement with Missing SatisfiedBy File

This requirement has a satisfiedBy relation pointing to a non-existing file.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Valid Capability](#valid-capability)
  * satisfiedBy: [non-existing-implementation.py](non-existing-implementation.py)

---

### Analysis Verification with Invalid SatisfiedBy

This analysis verification incorrectly has a satisfiedBy relation (should fail - only test-verification may have satisfiedBy).

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Valid Capability](#valid-capability)
  * satisfiedBy: [analysis-document.md](analysis-document.md)

---

### Inspection Verification with Invalid SatisfiedBy

This inspection verification incorrectly has a satisfiedBy relation (should fail - only test-verification may have satisfiedBy).

#### Metadata
  * type: inspection-verification

#### Relations
  * verify: [Valid Capability](#valid-capability)
  * satisfiedBy: [inspection-checklist.md](inspection-checklist.md)

---

### Demonstration Verification with Invalid SatisfiedBy

This demonstration verification incorrectly has a satisfiedBy relation (should fail - only test-verification may have satisfiedBy).

#### Metadata
  * type: demonstration-verification

#### Relations
  * verify: [Valid Capability](#valid-capability)
  * satisfiedBy: [demo-script.sh](demo-script.sh)

---

### System Requirement with SatisfiedBy but No DerivedFrom

This system requirement has satisfiedBy relation but is missing the required hierarchical parent (derivedFrom).

#### Metadata
  * type: requirement

#### Relations
  * satisfiedBy: [ValidImplementation.txt](ValidImplementation.txt)

---

### System Requirement with VerifiedBy but No DerivedFrom

This system requirement has verifiedBy relation but is missing the required hierarchical parent (derivedFrom).

#### Metadata
  * type: requirement

#### Relations
  * verifiedBy: [Valid Verification with Correct SatisfiedBy](#valid-verification-with-correct-satisfiedby)

---
