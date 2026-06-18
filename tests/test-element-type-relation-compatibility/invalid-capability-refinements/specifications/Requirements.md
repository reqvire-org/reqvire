# Elements

### Test Capability Invalid Capability Refinements

Capability root for invalid capability-owned refinement cases.

#### Metadata
  * type: capability

#### Relations
  * definedBy: [Capability Source](#capability-source)
  * definedBy: [Capability Constraint](#capability-constraint)
  * definedBy: [Capability Behavior](#capability-behavior)
  * definedBy: [Capability Specification](#capability-specification)
  * definedBy: [Capability State](#capability-state)
  * definedBy: [Capability Input Output](#capability-input-output)
  * definedBy: [Capability Semantic Contract](#capability-semantic-contract)
---

### Capability Source

INVALID: capability-owned source is not allowed.

#### Metadata
  * type: source

#### Relations
  * define: [Test Capability Invalid Capability Refinements](#test-capability-invalid-capability-refinements)
---

### Capability Constraint

INVALID: capability-owned constraint is not allowed.

#### Metadata
  * type: constraint

#### Relations
  * define: [Test Capability Invalid Capability Refinements](#test-capability-invalid-capability-refinements)
---

### Capability Behavior

INVALID: capability-owned behavior is not allowed.

#### Metadata
  * type: behavior

#### Relations
  * define: [Test Capability Invalid Capability Refinements](#test-capability-invalid-capability-refinements)
---

### Capability Specification

INVALID: capability-owned specification is not allowed.

#### Metadata
  * type: specification

#### Relations
  * define: [Test Capability Invalid Capability Refinements](#test-capability-invalid-capability-refinements)
---

### Capability State

INVALID: capability-owned state is not allowed.

#### Metadata
  * type: state

#### Relations
  * define: [Test Capability Invalid Capability Refinements](#test-capability-invalid-capability-refinements)
---

### Capability Input Output

INVALID: capability-owned input-output is not allowed.

#### Metadata
  * type: input-output

#### Relations
  * define: [Test Capability Invalid Capability Refinements](#test-capability-invalid-capability-refinements)
---

### Capability Semantic Contract

INVALID: capability-owned semantic-contract is not allowed.

#### Metadata
  * type: semantic-contract

#### Relations
  * define: [Test Capability Invalid Capability Refinements](#test-capability-invalid-capability-refinements)

#### Shapes
```turtle
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <urn:reqvire:test:capability-refinement:> .

ex:CapabilityShape a sh:NodeShape .
```
---
