# Elements

### Spec-1

A valid contract that defines User Req A.

#### Details
This contract is owned via definedBy relation, so it can be reused to requirements outside its defining hierarchy.

#### Metadata
  * type: specification
---

### Spec-2

A valid contract that defines Child With Contract.

#### Details
This contract tests the ancestor case - Ancestor Req is a parent of the defining requirement, so it cannot reuse Spec-2.

#### Metadata
  * type: specification
---

### Spec-X

A valid contract owned by User Req X.

#### Details
This contract is used to verify reverse-direction reused_contract_context flow between User Req A and User Req X hierarchies.

#### Metadata
  * type: specification
---

### Orphan-Spec

An orphan contract without any define relations.

#### Details
This contract has no define relations, so it cannot be reused anywhere.

#### Metadata
  * type: specification
---
