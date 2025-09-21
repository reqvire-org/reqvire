# Test Requirements with Invalid Relations

This document contains requirements with intentional relation errors for testing validation.

## Requirements

### Valid Requirement

This is a valid requirement with proper relation types.

#### Relations
  * containedBy: [../UserRequirements.md/Valid User Requirement](../UserRequirements.md#valid-user-requirement)
  * satisfiedBy: [ValidImplementation.txt](ValidImplementation.txt)

---

### Valid Requirement

Duplicated element name. It will report also duplicate subsesction, something to fix later.

#### Relations
  * refine: #Valid Requirement

---

### Requirement with invalid Metadata format

This requirement has invalid metadata format

#### Metadata
This is not ok.

#### Relations
  * derivedFrom: #Valid Requirement
  
---

### Another requirement with invalid Metadata format

This another requirement has invalid metadata format

#### Metadata
  * nottype: something

#### Relations
  * derivedFrom: #Valid Requirement
  
---

### Requirement with invalid Relation format

This requirement has invalid relations format and should 


#### Relations
  * derivedFrom: #Valid Requirement
This is not ok.
  
---

  
### System Requirement with Missing Parent Relation

This system requirement is missing parent relation.

---

### Requirement with Invalid Relation Type

This requirement has an invalid relation type with a typo.

#### Relations
  * containedBy: [../UserRequirements.md/Valid User Requirement](../UserRequirements.md#valid-user-requirement)
  * satisfiedBBy: [ValidImplementation.txt](ValidImplementation.txt)

---


### Requirement with Duplicate Relations

This requirement has duplicate relations.

#### Relations
  * containedBy: [../UserRequirements.md/Valid User Requirement](../UserRequirements.md#valid-user-requirement)
  * satisfiedBy: [ValidImplementation.txt](ValidImplementation.txt)
  * satisfiedBy: [ValidImplementation.txt](ValidImplementation.txt)

---

### Requirement with Missing Target

This requirement has a relation to a non-existent element.

#### Relations
  * containedBy: [../UserRequirements.md/Valid User Requirement](../UserRequirements.md#valid-user-requirement)
  * satisfiedBy: [NonExistentElement.md/Missing Element](NonExistentElement.md#missing-element)
---

### Requirement with Incompactible element

This requirement has incompactible element in relation, satisfiedBy cannot point to other requiremet.

#### Relations
  * containedBy: [../UserRequirements.md/Valid User Requirement](../UserRequirements.md#valid-user-requirement)
  * satisfiedBy: #Requirement with invalid relation type

---

### Requirement with Circular dependencies A

This requirement has circular dependency

#### Relations
  * derivedFrom: #Requirement with Circular dependencies C

---

### Requirement with Circular dependencies B

This requirement has circular dependency

#### Relations
  * derivedFrom: #Requirement with Circular dependencies A

---

### Requirement with Circular dependencies C

This requirement has circular dependency

#### Relations
  * derivedFrom: #Requirement with Circular dependencies B

---

### Requirement with invalid target

This should be validated as invalid target because # is missing.

#### Relations
  * derivedFrom: Valid Requirement
  
---

### Requirement with Duplicate Subsection

This requirement has duplicate subsection.

#### Relations
  * containedBy: [../UserRequirements.md/Valid User Requirement](../UserRequirements.md#valid-user-requirement)

#### Relations
  * containedBy: [../UserRequirements.md/Valid User Requirement](../UserRequirements.md#valid-user-requirement)

---

### Verification with Missing SatisfiedBy Target

This verification has a satisfiedBy relation pointing to a non-existing file.

#### Metadata
  * type: verification

#### Relations
  * verify: [Valid Requirement](#valid-requirement)
  * satisfiedBy: [non-existing-test-script.sh](non-existing-test-script.sh)

---

### Requirement with Missing VerifiedBy Target

This requirement has a verifiedBy relation pointing to a non-existing verification element.

#### Relations
  * containedBy: [../UserRequirements.md/Valid User Requirement](../UserRequirements.md#valid-user-requirement)
  * verifiedBy: [NonExistentVerification.md#missing-verification](NonExistentVerification.md#missing-verification)

---

### Valid Requirement with Correct SatisfiedBy

This requirement correctly uses satisfiedBy pointing to an existing implementation file.

#### Relations
  * containedBy: [../UserRequirements.md/Valid User Requirement](../UserRequirements.md#valid-user-requirement)
  * satisfiedBy: [ValidImplementation.txt](ValidImplementation.txt)

---

### Valid Verification with Correct SatisfiedBy

This verification correctly uses satisfiedBy pointing to an existing test script.

#### Metadata
  * type: verification

#### Relations
  * verify: [Valid Requirement](#valid-requirement)
  * satisfiedBy: [test.sh](test.sh)

---

### Requirement SatisfiedBy Requirement Invalid

This requirement incorrectly uses satisfiedBy pointing to another requirement (should fail).

#### Relations
  * containedBy: [../UserRequirements.md/Valid User Requirement](../UserRequirements.md#valid-user-requirement)
  * satisfiedBy: [Valid Requirement](#valid-requirement)

---

### Verification SatisfiedBy Verification Invalid

This verification incorrectly uses satisfiedBy pointing to another verification (should fail).

#### Metadata
  * type: verification

#### Relations
  * verify: [Valid Requirement](#valid-requirement)
  * satisfiedBy: [Valid Verification with Correct SatisfiedBy](#valid-verification-with-correct-satisfiedby)

---

### Requirement with Missing SatisfiedBy File

This requirement has a satisfiedBy relation pointing to a non-existing file.

#### Relations
  * containedBy: [../UserRequirements.md/Valid User Requirement](../UserRequirements.md#valid-user-requirement)
  * satisfiedBy: [non-existing-implementation.py](non-existing-implementation.py)

---
