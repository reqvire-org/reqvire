# Test Requirements with Invalid Relations

This document contains requirements with intentional relation errors for testing validation.

## Requirements

### Valid Requirement

This is a valid requirement with proper relation types.

#### Relations
* refine: [UserRequirements.md/Valid User Requirement](UserRequirements.html#valid-user-requirement)
* satisfiedBy: [ValidImplementation.md](ValidImplementation.html)

---

### Requirement with Invalid Relation Type

This requirement has an invalid relation type with a typo.

#### Relations
* refine: [UserRequirements.md/Valid User Requirement](UserRequirements.html#valid-user-requirement)
* satisfieddBy: [InvalidImplementation.md](InvalidImplementation.html)

---

### Requirement with Non-Alphanumeric Relation Type

This requirement has a relation type with invalid characters.

#### Relations
* refine: [UserRequirements.md/Valid User Requirement](UserRequirements.html#valid-user-requirement)
* depends-on: [AnotherRequirement.md](AnotherRequirement.html)

---

### Requirement with Duplicate Relations

This requirement has duplicate relations.

#### Relations
* refine: [UserRequirements.md/Valid User Requirement](UserRequirements.html#valid-user-requirement)
* satisfiedBy: [SameImplementation.md](SameImplementation.html)
* satisfiedBy: [SameImplementation.md](SameImplementation.html)

---

### Requirement with Missing Target

This requirement has a relation to a non-existent element.

#### Relations
* refine: [UserRequirements.md/Valid User Requirement](UserRequirements.html#valid-user-requirement)
* satisfiedBy: [NonExistentElement.md/Missing Element](NonExistentElement.html#missing-element)