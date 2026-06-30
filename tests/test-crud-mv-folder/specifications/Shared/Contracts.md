# Elements

### Shared Capability

Capability that owns the reusable shared requirement.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [Shared Requirement](#shared-requirement)
---

### Shared Requirement

The system shall define a reusable payload contract.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Shared Capability](#shared-capability)
  * definedBy: [Shared Payload Specification](#shared-payload-specification)
---

### Shared Payload Specification

Reusable payload details consumed from another file.

#### Metadata
  * type: specification

#### Relations
  * define: [Shared Requirement](#shared-requirement)
---
