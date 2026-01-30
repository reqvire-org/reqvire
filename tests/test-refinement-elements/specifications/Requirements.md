# Elements

### Parent Requirement

This is a parent requirement for testing Refinement elements.

#### Metadata
  * type: user-requirement

#### Relations
  * refinedBy: [Test Constraint Element](#test-constraint-element)
  * refinedBy: [Test Behavior Element](#test-behavior-element)
  * refinedBy: [Test Specification Element](#test-specification-element)
---

### Test Constraint Element

This element documents a constraint for testing purposes.

#### Metadata
  * type: constraint

#### Details
This is a constraint that limits system behavior. It specifies boundaries and limitations.

---

### Test Behavior Element

This element documents behavior details for testing purposes.

#### Metadata
  * type: behavior

#### Details
This is a behavior specification that describes how the system operates under certain conditions.

---

### Test Specification Element

This element documents a specification for testing purposes.

#### Metadata
  * type: specification

#### Details
This is a detailed specification document containing technical requirements and parameters.

---

### Separate Branch Requirement

A separate requirement hierarchy for testing attachments.

#### Metadata
  * type: user-requirement
---

### Requirement With Refinement Attachment

This requirement has a Refinement element attached to it.

#### Metadata
  * type: requirement

#### Attachments
  * [Test Constraint Element](#test-constraint-element)

#### Relations
  * derivedFrom: [Separate Branch Requirement](#separate-branch-requirement)
---
