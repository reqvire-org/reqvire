# Elements

### Interface Root

Fixture capability for dependency validation.

#### Metadata
  * type: capability
---

### Serving Capability

Fixture capability for dependency validation.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [Interface Root](#interface-root)
---

### Endpoint Capability

Fixture capability for dependency validation.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [Interface Root](#interface-root)
---

### Serve Requirement

The system SHALL provide the serve requirement function.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Endpoint Contract](#endpoint-contract)

#### Relations
  * specify: [Serving Capability](#serving-capability)
  * definedBy: [Serve Contract](#serve-contract)
  * satisfiedBy: [Implementation](implementation.txt)
---

### Embedded Endpoint

The system SHALL provide the embedded endpoint function.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Serve Contract](#serve-contract)

#### Relations
  * specify: [Endpoint Capability](#endpoint-capability)
  * definedBy: [Endpoint Contract](#endpoint-contract)
  * satisfiedBy: [Implementation](implementation.txt)
---

### Serve Contract

Fixture specification for dependency validation.

#### Metadata
  * type: specification
---

### Endpoint Contract

Fixture specification for dependency validation.

#### Metadata
  * type: specification
---
