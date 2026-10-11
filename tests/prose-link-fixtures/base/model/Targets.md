# Elements

### Authoring Capability

Model authoring capability.

#### Metadata
  * type: capability
---

### Contract Owner

The system SHALL provide the shared delivery contract.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Authoring Capability](#authoring-capability)
---

### Target Scheme

Delivery terminology.

#### Metadata
  * type: concept-scheme
  * concept_base: https://example.test/prose-concepts
  * concept_prefix: term
---

### Target Concept

Delivery terminology definition.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Target Scheme](#target-scheme)
---

### Target Specification

Delivery contract details.

#### Metadata
  * type: specification

#### Relations
  * define: [Contract Owner](#contract-owner)
---
