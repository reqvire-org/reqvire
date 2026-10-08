# Elements

### Root Capability

Ability to exercise element selection.

#### Metadata
  * type: capability
---

### Nested Capability

Ability to exercise nested selection.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [Root Capability](#root-capability)
---

### Other Capability

An independent capability.

#### Metadata
  * type: capability
---

### Binding Capability

Ability to consume a contract in a third subgraph.

#### Metadata
  * type: capability
---

### Parent Requirement

The system SHALL provide selection evidence.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Nested Capability](#nested-capability)
  * definedBy: [Owned Specification](#owned-specification)
---

### Child Requirement

The system SHALL preserve the selected subject.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Requirement](#parent-requirement)
  * satisfiedBy: [Evidence](evidence.txt)
---

### Sibling Requirement

The system SHALL preserve other subjects.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Requirement](#parent-requirement)
---

### Merge Requirement

The system SHALL provide another merge source.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Requirement](#parent-requirement)
---

### External Owner

The system SHALL own a reusable contract in another requirement tree.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Other Capability](#other-capability)
  * definedBy: [External Specification](#external-specification)
---

### Owned Specification

Selected subjects have canonical identifiers.

#### Metadata
  * type: specification
---

### Binding Consumer

The system SHALL reuse a contract from another requirement tree.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Binding Capability](#binding-capability)

#### Contract Bindings
  * [Owned Specification](#owned-specification)
---

### Reference Consumer

The system SHALL retain a content dependency.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [External Owner](#external-owner)

#### Contract References
  * [Owned Specification](#owned-specification)
---

### External Specification

External contract content can be reused.

#### Metadata
  * type: specification
---

### Selection Objective

Exercise relation endpoints.

#### Metadata
  * type: verification-objective
---

### Selection Check

Check selected children.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Selection Objective](#selection-objective)
  * verify: [Child Requirement](#child-requirement)
  * satisfiedBy: [Evidence](evidence.txt)
---

### Other Check

Check other children.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Selection Objective](#selection-objective)
  * verify: [Sibling Requirement](#sibling-requirement)
  * satisfiedBy: [Evidence](evidence.txt)
---
