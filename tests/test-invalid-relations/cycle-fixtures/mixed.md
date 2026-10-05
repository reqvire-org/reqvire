# Elements

### Mixed Root

Fixture capability for dependency validation.

#### Metadata
  * type: capability
---

### Parent Requirement

The system SHALL provide the parent requirement function.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Consumer Contract](#consumer-contract)

#### Relations
  * specify: [Mixed Root](#mixed-root)
  * derive: [Child Requirement](#child-requirement)
  * satisfiedBy: [Implementation](implementation.txt)
---

### Child Requirement

The system SHALL provide the child requirement function.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Child Contract](#child-contract)
  * satisfiedBy: [Implementation](implementation.txt)
---

### Consumer Requirement

The system SHALL provide the consumer requirement function.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Child Contract](#child-contract)

#### Relations
  * specify: [Mixed Root](#mixed-root)
  * definedBy: [Consumer Contract](#consumer-contract)
  * satisfiedBy: [Implementation](implementation.txt)
---

### Child Contract

Fixture specification for dependency validation.

#### Metadata
  * type: specification
---

### Consumer Contract

Fixture specification for dependency validation.

#### Metadata
  * type: specification
---
