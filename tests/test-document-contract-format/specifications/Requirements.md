# Elements


### Test Capability Test Document Contract Format Specifications Requirements Md

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

### Root User Need

The system shall define a root requirement for hierarchy validation.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-document-contract-format-specifications-requirements-md)
---

### Parent Requirement

The system shall provide a valid owner requirement for single-element contract files.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root User Need](#root-user-need)
---

### Requirement Using Single Element Contract

The system shall allow definedBy only to contract element identifiers, including elements defined in `# Element` files.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Requirement](#parent-requirement)
  * definedBy: [ChangePropagation](DesignDocuments/ChangePropagation.md#changepropagation)
---
