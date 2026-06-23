# Elements

### Parent Capability

This is a parent capability for testing requirement-owned contract elements.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [Parent Requirement](#parent-requirement)
---

### Parent Requirement

This is a parent requirement for testing Contract elements.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Parent Capability](#parent-capability)
  * definedBy: [Test Constraint Element](#test-constraint-element)
  * definedBy: [Test Behavior Element](#test-behavior-element)
  * definedBy: [Test Specification Element](#test-specification-element)
  * definedBy: [Test State Element](#test-state-element)
  * definedBy: [Test Input Output Element](#test-input-output-element)
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

### Test State Element

This element documents state details for testing purposes.

#### Metadata
  * type: state

#### Details
This is a state contract that describes lifecycle states and transitions.

---

### Test Input Output Element

This element documents input/output details for testing purposes.

#### Metadata
  * type: input-output

#### Details
This is an input-output contract that describes payload structure and data contracts.

---

### Separate Branch Capability

A separate capability hierarchy for testing contract_bindings.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [Separate Branch Requirement](#separate-branch-requirement)
---

### Separate Branch Requirement

A separate requirement hierarchy for testing contract_bindings.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Separate Branch Capability](#separate-branch-capability)
---

### Requirement With Contract ContractBindingEntry

This requirement has a Contract element bound to it.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Test Constraint Element](#test-constraint-element)

#### Relations
  * derivedFrom: [Separate Branch Requirement](#separate-branch-requirement)
---
