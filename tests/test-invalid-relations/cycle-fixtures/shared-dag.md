# Elements

### Shared Root

Fixture capability for dependency validation.

#### Metadata
  * type: capability
---

### Contract Owner

The system SHALL provide the contract owner function.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Shared Root](#shared-root)
  * definedBy: [Shared Contract](#shared-contract)
  * definedBy: [Second Shared Contract](#second-shared-contract)
  * satisfiedBy: [Implementation](implementation.txt)
---

### Consumer Parent

The system SHALL provide the consumer parent function.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Shared Contract](#shared-contract)
  * [Second Shared Contract](#second-shared-contract)

#### Relations
  * specify: [Shared Root](#shared-root)
  * derive: [Left Branch](#left-branch)
  * derive: [Right Branch](#right-branch)
  * satisfiedBy: [Implementation](implementation.txt)
---

### Left Branch

The system SHALL provide the left branch function.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Shared Terminal](#shared-terminal)
  * satisfiedBy: [Implementation](implementation.txt)
---

### Right Branch

The system SHALL provide the right branch function.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Shared Terminal](#shared-terminal)
  * satisfiedBy: [Implementation](implementation.txt)
---

### Shared Terminal

The system SHALL provide the shared terminal function.

#### Metadata
  * type: requirement

#### Relations
  * satisfiedBy: [Implementation](implementation.txt)
---

### Shared Contract

Fixture specification for dependency validation.

#### Metadata
  * type: specification
---

### Second Shared Contract

Fixture specification for dependency validation.

#### Metadata
  * type: specification
---
