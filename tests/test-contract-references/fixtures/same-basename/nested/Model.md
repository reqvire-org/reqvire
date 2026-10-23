# Elements

### Collision Binding Consumer

The system SHALL implement the shared contract.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Collision Specification](../Model.md#collision-specification)

#### Relations
  * specify: [Collision Consumers](#collision-consumers)
---

### Collision Consumers

Consume the shared contract.

#### Metadata
  * type: capability
---

### Collision Derived Requirement

The system SHALL preserve its parent identity.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Collision Owner](../Model.md#collision-owner)
---

### Collision Reference Consumer

The system SHALL document the shared contract.

#### Metadata
  * type: requirement

#### Contract References
  * [Collision Specification](../Model.md#collision-specification)

#### Relations
  * specify: [Collision Consumers](#collision-consumers)
---
