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
