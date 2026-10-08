# Elements

### Owners

Ability to exercise owners.

#### Metadata
  * type: capability

---

### Consumers

Ability to exercise consumers.

#### Metadata
  * type: capability

---

### Delegated Service

Ability to exercise delegated service.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [Owners](#owners)

---

### Empty Capability

Ability to exercise empty capability.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [Owners](#owners)

---

### Terminal Done

The system SHALL fulfill the terminal done obligation.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Owners](#owners)
  * satisfiedBy: [a](../evidence/a.txt)
---

### Terminal Gap

The system SHALL fulfill the terminal gap obligation.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Owners](#owners)
---

### Unused Owner

The system SHALL fulfill the unused owner obligation.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Owners](#owners)
  * definedBy: [Unused Contract](#unused-contract)
---

### Unused Contract

Shared obligation contract.

#### Metadata
  * type: specification
---

### Unused Done

The system SHALL fulfill the unused done obligation.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Owners](#owners)
  * definedBy: [Unused Done Contract](#unused-done-contract)
  * satisfiedBy: [b](../evidence/b.txt)
---

### Unused Done Contract

Shared obligation contract.

#### Metadata
  * type: specification
---

### Tree Root

The system SHALL fulfill the tree root obligation.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Owners](#owners)
---

### Tree Middle

The system SHALL fulfill the tree middle obligation.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Tree Root](#tree-root)
---

### Tree Other

The system SHALL fulfill the tree other obligation.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Tree Root](#tree-root)
---

### Tree A

The system SHALL fulfill the tree a obligation.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Tree Middle](#tree-middle)
  * satisfiedBy: [a](../evidence/a.txt)
---

### Tree Shared

The system SHALL fulfill the tree shared obligation.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Tree Middle](#tree-middle)
  * derivedFrom: [Tree Other](#tree-other)
  * satisfiedBy: [b](../evidence/b.txt)
---

### Parent With Evidence

The system SHALL fulfill the parent with evidence obligation.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Owners](#owners)
  * satisfiedBy: [parent](../evidence/parent.txt)
---

### Missing Child

The system SHALL fulfill the missing child obligation.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent With Evidence](#parent-with-evidence)
---

### Mixed Owner

The system SHALL fulfill the mixed owner obligation.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Owners](#owners)
  * definedBy: [Mixed Contract](#mixed-contract)
  * definedBy: [Duplicate Contract](#duplicate-contract)
---

### Mixed Contract

Shared obligation contract.

#### Metadata
  * type: specification
---

### Duplicate Contract

Shared obligation contract.

#### Metadata
  * type: specification
---

### Partial Consumer

The system SHALL fulfill the partial consumer obligation.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Mixed Contract](#mixed-contract)
  * [Duplicate Contract](#duplicate-contract)

#### Relations
  * specify: [Consumers](#consumers)
  * satisfiedBy: [parent](../evidence/parent.txt)
---

### Consumer Child

The system SHALL fulfill the consumer child obligation.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Partial Consumer](#partial-consumer)
  * satisfiedBy: [a](../evidence/a.txt)
---

### Missing Consumer Child

The system SHALL fulfill the missing consumer child obligation.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Partial Consumer](#partial-consumer)
---

### Other Consumer

The system SHALL fulfill the other consumer obligation.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Mixed Contract](#mixed-contract)

#### Relations
  * specify: [Consumers](#consumers)
  * satisfiedBy: [b](../evidence/b.txt)
---

### Complete Owner

The system SHALL fulfill the complete owner obligation.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Delegated Service](#delegated-service)
  * definedBy: [Complete Contract](#complete-contract)
---

### Complete Contract

Shared obligation contract.

#### Metadata
  * type: specification
---

### Complete Consumer

The system SHALL fulfill the complete consumer obligation.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Complete Contract](#complete-contract)

#### Relations
  * specify: [Consumers](#consumers)
---

### Complete Consumer A

The system SHALL fulfill the complete consumer a obligation.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Complete Consumer](#complete-consumer)
  * satisfiedBy: [a](../evidence/a.txt)
---

### Complete Consumer B

The system SHALL fulfill the complete consumer b obligation.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Complete Consumer](#complete-consumer)
  * satisfiedBy: [b](../evidence/b.txt)
---

### Both Complete

The system SHALL fulfill the both complete obligation.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Owners](#owners)
  * definedBy: [Both Contract](#both-contract)
---

### Both Contract

Shared obligation contract.

#### Metadata
  * type: specification
---

### Both Child

The system SHALL fulfill the both child obligation.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Both Complete](#both-complete)
  * satisfiedBy: [a](../evidence/a.txt)
---

### Both Consumer

The system SHALL fulfill the both consumer obligation.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Both Contract](#both-contract)

#### Relations
  * specify: [Consumers](#consumers)
  * satisfiedBy: [b](../evidence/b.txt)
---

### Both Child Gap

The system SHALL fulfill the both child gap obligation.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Owners](#owners)
  * definedBy: [Child Gap Contract](#child-gap-contract)
  * satisfiedBy: [parent](../evidence/parent.txt)
---

### Child Gap Contract

Shared obligation contract.

#### Metadata
  * type: specification
---

### Both Missing Child

The system SHALL fulfill the both missing child obligation.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Both Child Gap](#both-child-gap)
---

### Good Consumer

The system SHALL fulfill the good consumer obligation.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Child Gap Contract](#child-gap-contract)

#### Relations
  * specify: [Consumers](#consumers)
  * satisfiedBy: [b](../evidence/b.txt)
---

### Both Consumer Gap

The system SHALL fulfill the both consumer gap obligation.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Owners](#owners)
  * definedBy: [Consumer Gap Contract](#consumer-gap-contract)
---

### Consumer Gap Contract

Shared obligation contract.

#### Metadata
  * type: specification
---

### Good Child

The system SHALL fulfill the good child obligation.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Both Consumer Gap](#both-consumer-gap)
  * satisfiedBy: [a](../evidence/a.txt)
---

### Missing Consumer

The system SHALL fulfill the missing consumer obligation.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Consumer Gap Contract](#consumer-gap-contract)

#### Relations
  * specify: [Consumers](#consumers)
---

### Review Objective

Plan verification.

#### Metadata
  * type: verification-objective
---

### Contract Owner Review

Inspect childless contract owners.

#### Metadata
  * type: inspection-verification

#### Relations
  * derivedFrom: [Review Objective](#review-objective)
  * verify: [Unused Owner](#unused-owner)
  * verify: [Complete Owner](#complete-owner)
---
