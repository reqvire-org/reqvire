# Elements

### Root User Need

The system shall define a root requirement for hierarchy validation.

#### Metadata
  * type: user-requirement
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
