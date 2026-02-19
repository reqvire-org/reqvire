# Elements

### Root User Requirement

Top-level user requirement used as hierarchy parent for system requirements in this fixture.

#### Metadata
  * type: user-requirement
---

### System Performance

The system shall meet performance requirements.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root User Requirement](#root-user-requirement)
  * satisfiedBy: [impl.rs](../core/src/impl.rs)
---

### Data Integrity

The system shall ensure data integrity across all operations.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root User Requirement](#root-user-requirement)
  * satisfiedBy: [impl.rs](../core/src/impl.rs)
  * trace: [design.md](../docs/design.md)
---

### User Interface

The system shall provide a user-friendly interface.

#### Metadata
  * type: requirement

#### Attachments
  * [UI Mockup Spec](#ui-mockup-spec)

#### Relations
  * derivedFrom: [Root User Requirement](#root-user-requirement)
---

### API Requirements

The system shall provide a RESTful API.

#### Metadata
  * type: requirement

#### Attachments
  * [API Contract Spec](#api-contract-spec)

#### Relations
  * derivedFrom: [Root User Requirement](#root-user-requirement)
  * satisfiedBy: [api.rs](../core/src/api.rs)
---

### Documentation Contract Owner

Owner requirement for documentation refinement contracts.

#### Metadata
  * type: user-requirement

#### Relations
  * refinedBy: [UI Mockup Spec](#ui-mockup-spec)
  * refinedBy: [API Contract Spec](#api-contract-spec)
---

### UI Mockup Spec

Refinement contract describing UI mockup expectations.

#### Metadata
  * type: specification
---

### API Contract Spec

Refinement contract describing API specification constraints.

#### Metadata
  * type: specification
---
