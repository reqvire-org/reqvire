# Elements


### Test Capability Test Resources Report Specifications Requirements Md

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

### Root Capability

Top-level capability used as hierarchy parent for system requirements in this fixture.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-resources-report-specifications-requirements-md)
---

### System Performance

The system shall meet performance requirements.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Capability](#root-capability)
  * satisfiedBy: [impl.rs](../core/src/impl.rs)
---

### Data Integrity

The system shall ensure data integrity across all operations.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Capability](#root-capability)
  * satisfiedBy: [impl.rs](../core/src/impl.rs)
  * satisfiedBy: [design.md](../docs/design.md)
---

### User Interface

The system shall provide a user-friendly interface.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [UI Mockup Spec](#ui-mockup-spec)

#### Relations
  * derivedFrom: [Root Capability](#root-capability)
---

### API Requirements

The system shall provide a RESTful API.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [API Contract Spec](#api-contract-spec)

#### Relations
  * derivedFrom: [Root Capability](#root-capability)
  * satisfiedBy: [api.rs](../core/src/api.rs)
---

### Documentation Contract Owner

Owner requirement for documentation contracts.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-resources-report-specifications-requirements-md)
  * definedBy: [UI Mockup Spec](#ui-mockup-spec)
  * definedBy: [API Contract Spec](#api-contract-spec)
---

### UI Mockup Spec

Contract contract describing UI mockup expectations.

#### Metadata
  * type: specification
---

### API Contract Spec

Contract contract describing API specification constraints.

#### Metadata
  * type: specification
---
