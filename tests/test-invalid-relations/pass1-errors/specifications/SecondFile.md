# Elements


### Test Capability Test Invalid Relations Pass1 Errors Specifications Secondfile Md

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

This file contains an element with a name that already exists in Requirements.md, testing global uniqueness validation.


### Valid Requirement

This element has the same name as an element in Requirements.md - should trigger global uniqueness error.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-invalid-relations-pass1-errors-specifications-secondfile-md)
  * derivedFrom: Requirements.md#valid-requirement

---

### Another Valid Element

This is a unique element in this file.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-invalid-relations-pass1-errors-specifications-secondfile-md)
---