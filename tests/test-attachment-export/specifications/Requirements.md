# Elements


### Test Feature Test Attachment Export Specifications Requirements Md

Test feature root for migrated requirement fixtures.

#### Metadata
  * type: feature
---

### Root Requirement

The system shall provide core functionality.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Feature](#test-feature-test-attachment-export-specifications-requirements-md)
---

### System Feature Requirement

The system shall implement the feature as specified in the attached design document.

#### Attachments
  * [Design Spec Contract](../docs/DesignSpec.md#design-spec-contract)

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
  * verifiedBy: [Feature Test](#feature-test)
---

### Design Spec Owner

Owner requirement for the design specification contract.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Feature](#test-feature-test-attachment-export-specifications-requirements-md)
  * refinedBy: [Design Spec Contract](../docs/DesignSpec.md#design-spec-contract)
---

### Feature Test

Verify that the feature works correctly.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [System Feature Requirement](#system-feature-requirement)
---
