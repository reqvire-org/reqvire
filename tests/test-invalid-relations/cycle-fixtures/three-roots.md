# Elements

### Root A

Fixture capability for dependency validation.

#### Metadata
  * type: capability
---

### Root B

Fixture capability for dependency validation.

#### Metadata
  * type: capability
---

### Root C

Fixture capability for dependency validation.

#### Metadata
  * type: capability
---

### Requirement A

The system SHALL provide the requirement a function.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Contract C](#contract-c)

#### Relations
  * specify: [Root A](#root-a)
  * definedBy: [Contract A](#contract-a)
  * satisfiedBy: [Implementation](implementation.txt)
---

### Requirement B

The system SHALL provide the requirement b function.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Contract A](#contract-a)

#### Relations
  * specify: [Root B](#root-b)
  * definedBy: [Contract B](#contract-b)
  * satisfiedBy: [Implementation](implementation.txt)
---

### Requirement C

The system SHALL provide the requirement c function.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Contract B](#contract-b)

#### Relations
  * specify: [Root C](#root-c)
  * definedBy: [Contract C](#contract-c)
  * satisfiedBy: [Implementation](implementation.txt)
---

### Contract A

Fixture specification for dependency validation.

#### Metadata
  * type: specification
---

### Contract B

Fixture specification for dependency validation.

#### Metadata
  * type: specification
---

### Contract C

Fixture specification for dependency validation.

#### Metadata
  * type: specification
---
