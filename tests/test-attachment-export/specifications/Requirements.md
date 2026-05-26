# Elements


### Test Capability Test Attachment Export Specifications Requirements Md

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

### Root Requirement

The system shall provide core functionality.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-attachment-export-specifications-requirements-md)
---

### System Capability Requirement

The system shall implement the capability as specified in the attached design document.

#### Attachments
  * [Design Spec Contract](../docs/DesignSpec.md#design-spec-contract)

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
  * verifiedBy: [Capability Test](#capability-test)
---

### Design Spec Owner

Owner requirement for the design specification contract.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-attachment-export-specifications-requirements-md)
  * refinedBy: [Design Spec Contract](../docs/DesignSpec.md#design-spec-contract)
---

### Capability Test

Verify that the capability works correctly.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [System Capability Requirement](#system-capability-requirement)
---
