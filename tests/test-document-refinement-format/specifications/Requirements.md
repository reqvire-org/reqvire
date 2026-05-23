# Elements


### Test Feature Test Document Refinement Format Specifications Requirements Md

Test feature root for migrated requirement fixtures.

#### Metadata
  * type: feature
---

### Root User Need

The system shall define a root requirement for hierarchy validation.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Feature](#test-feature-test-document-refinement-format-specifications-requirements-md)
---

### Parent Requirement

The system shall provide a valid owner requirement for refinement documents.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root User Need](#root-user-need)
---

### Requirement Using Document Refinement

The system shall allow refinedBy only to refinement element identifiers, including elements defined in #Documents files.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Requirement](#parent-requirement)
  * refinedBy: [ChangePropagation](DesignDocuments/ChangePropagation.md#changepropagation)
---
