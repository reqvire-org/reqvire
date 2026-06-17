# Elements

### Test Capability Invalid Capability Refinements

Capability root for invalid capability-owned refinement cases.

#### Metadata
  * type: capability

#### Relations
  * refinedBy: [Capability Source](#capability-source)
  * refinedBy: [Capability Constraint](#capability-constraint)
  * refinedBy: [Capability Behavior](#capability-behavior)
  * refinedBy: [Capability Specification](#capability-specification)
  * refinedBy: [Capability State](#capability-state)
  * refinedBy: [Capability Input Output](#capability-input-output)
  * refinedBy: [Capability Semantic Contract](#capability-semantic-contract)
---

### Capability Source

INVALID: capability-owned source is not allowed.

#### Metadata
  * type: source

#### Relations
  * refine: [Test Capability Invalid Capability Refinements](#test-capability-invalid-capability-refinements)
---

### Capability Constraint

INVALID: capability-owned constraint is not allowed.

#### Metadata
  * type: constraint

#### Relations
  * refine: [Test Capability Invalid Capability Refinements](#test-capability-invalid-capability-refinements)
---

### Capability Behavior

INVALID: capability-owned behavior is not allowed.

#### Metadata
  * type: behavior

#### Relations
  * refine: [Test Capability Invalid Capability Refinements](#test-capability-invalid-capability-refinements)
---

### Capability Specification

INVALID: capability-owned specification is not allowed.

#### Metadata
  * type: specification

#### Relations
  * refine: [Test Capability Invalid Capability Refinements](#test-capability-invalid-capability-refinements)
---

### Capability State

INVALID: capability-owned state is not allowed.

#### Metadata
  * type: state

#### Relations
  * refine: [Test Capability Invalid Capability Refinements](#test-capability-invalid-capability-refinements)
---

### Capability Input Output

INVALID: capability-owned input-output is not allowed.

#### Metadata
  * type: input-output

#### Relations
  * refine: [Test Capability Invalid Capability Refinements](#test-capability-invalid-capability-refinements)
---

### Capability Semantic Contract

INVALID: capability-owned semantic-contract is not allowed.

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Test Capability Invalid Capability Refinements](#test-capability-invalid-capability-refinements)

#### Shapes
```turtle
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <urn:reqvire:test:capability-refinement:> .

ex:CapabilityShape a sh:NodeShape .
```
---
