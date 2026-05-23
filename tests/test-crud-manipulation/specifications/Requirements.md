# Elements


### Test Feature Test Crud Manipulation Specifications Requirements Md

Test feature root for migrated requirement fixtures.

#### Metadata
  * type: feature
---

### Parent Feature

This is a parent requirement.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Feature](#test-feature-test-crud-manipulation-specifications-requirements-md)
---

### Feature A

This is a test requirement for feature A.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Feature](#parent-feature)
---

### Feature B

This is a test requirement for feature B.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Feature A](#feature-a)
---

### Feature C

This is a test requirement for feature C with relations.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Feature A](#feature-a)
  * verifiedBy: [Test for Feature C](Verifications/Tests.md#test-for-feature-c)
---

### Complex chars, element/name example

This feature has special characters in its name including commas and slashes.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Feature A](#feature-a)
---

### Separate Requirement Branch

A separate requirement hierarchy for testing attachments.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Feature](#test-feature-test-crud-manipulation-specifications-requirements-md)
---