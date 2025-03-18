# Test Requirements with Invalid Relations

This document contains requirements with intentional relation errors for testing validation.

## Requirements

### Valid Requirement

This is a valid requirement with proper relation types.

#### Relations
* refine: [UserRequirements.md/Valid User Requirement](UserRequirements.md#valid-user-requirement)
* satisfiedBy: [ValidImplementation.md](ValidImplementation.md)

---

### Requirement with Invalid Relation Type

This requirement has an invalid relation type with a typo.

#### Relations
* refine: [UserRequirements.md/Valid User Requirement](UserRequirements.md#valid-user-requirement)
* satisfieddBy: [ValidImplementation.md](ValidImplementation.md)

---


### Requirement with Duplicate Relations

This requirement has duplicate relations.

#### Relations
* refine: [UserRequirements.md/Valid User Requirement](UserRequirements.md#valid-user-requirement)
* satisfiedBy: [SameImplementation.md](SameImplementation.md)
* satisfiedBy: [SameImplementation.md](SameImplementation.md)

---

### Requirement with Missing Target

This requirement has a relation to a non-existent element.

#### Relations
* satisfiedBy: [NonExistentElement.md/Missing Element](NonExistentElement.md#missing-element)
