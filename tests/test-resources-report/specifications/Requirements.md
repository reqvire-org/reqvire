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
  * [ui-mockup.png](../docs/ui-mockup.png)

#### Relations
  * derivedFrom: [Root User Requirement](#root-user-requirement)
---

### API Requirements

The system shall provide a RESTful API.

#### Metadata
  * type: requirement

#### Attachments
  * [api-spec.md](../docs/api-spec.md)

#### Relations
  * derivedFrom: [Root User Requirement](#root-user-requirement)
  * satisfiedBy: [api.rs](../core/src/api.rs)
---
