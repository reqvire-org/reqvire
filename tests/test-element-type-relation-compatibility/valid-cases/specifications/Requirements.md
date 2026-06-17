# Elements


### Test Capability Test Element Type Relation Compatibility Valid Cases Specifications

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability

#### Relations
  * verifiedBy: [Capability Level Verification](#capability-level-verification)
---

### Capability Parent

A top-level capability.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-element-type-relation-compatibility-valid-cases-specifications)
---

### Capability Child

User requirement deriving from another capability.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-element-type-relation-compatibility-valid-cases-specifications)
  * derivedFrom: [Capability Parent](#capability-parent)

---

### System Requirement from User Req

System requirement deriving from capability.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability Parent](#capability-parent)

---

### System Requirement from System Req

System requirement deriving from another system requirement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [System Requirement from User Req](#system-requirement-from-user-req)

---

### Requirement with SatisfiedBy

Requirement satisfied by implementation file.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability Parent](#capability-parent)
  * satisfiedBy: [impl.rs](impl.rs)

---

### Test Verification with SatisfiedBy

Test verification can use satisfiedBy.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Capability Parent](#capability-parent)
  * satisfiedBy: [test.sh](test.sh)

---

### Requirement with Test Verification

Requirement verified by test verification.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability Parent](#capability-parent)
  * verifiedBy: [Test Verification with SatisfiedBy](#test-verification-with-satisfiedby)

---

### Requirement with Analysis Verification

Requirement verified by analysis verification.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability Parent](#capability-parent)
  * verifiedBy: [Analysis Verification](#analysis-verification)

---

### Analysis Verification

Analysis verification verifying a requirement.

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Capability Parent](#capability-parent)

---

### Inspection Verification

Inspection verification verifying a requirement.

#### Metadata
  * type: inspection-verification

#### Relations
  * verify: [Capability Parent](#capability-parent)

---

### Demonstration Verification

Demonstration verification verifying a requirement.

#### Metadata
  * type: demonstration-verification

#### Relations
  * verify: [Capability Parent](#capability-parent)

---

### Capability Level Verification

Analysis verification directly verifying a capability.

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Test Capability](#test-capability-test-element-type-relation-compatibility-valid-cases-specifications)

---

### Behavior Element

A behavior element describing system behavior.

#### Metadata
  * type: behavior

---

### Specification Element

A specification element describing detailed specifications.

#### Metadata
  * type: specification

---

### Constraint Element

A constraint element describing system constraints.

#### Metadata
  * type: constraint

---

### Source Element

A source element describing requirement-owned source material.

#### Metadata
  * type: source

---

### Semantic Contract Ontology

Ontology vocabulary used by the semantic contract relation fixture.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/semantic-contract
  * ontology_prefix: screl

#### Ontology
```turtle
@prefix screl: <https://example.test/semantic-contract#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/semantic-contract> a owl:Ontology .
screl:Payload a owl:Class .
screl:payloadId a owl:DatatypeProperty .
```

---

### Requirement Constrained By Semantic Contract

Requirement constrained by a reusable semantic contract.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability Parent](#capability-parent)
  * constrainedBy: [Reusable Semantic Contract](#reusable-semantic-contract)

---

### Reusable Semantic Contract

Reusable semantic contract that constrains a requirement and uses ontology vocabulary.

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Requirement Constrained By Semantic Contract](#requirement-constrained-by-semantic-contract)
  * use: [Semantic Contract Ontology](#semantic-contract-ontology)

#### Shapes
```turtle
@prefix screl: <https://example.test/semantic-contract#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

screl:PayloadShape
  a sh:NodeShape ;
  sh:targetClass screl:Payload ;
  sh:property [
    sh:path screl:payloadId ;
    sh:datatype xsd:string ;
  ] .
```

---

### Requirement Refined By Behavior

Requirement that asks for behavior definition, refined by a behavior element.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability Parent](#capability-parent)
  * refinedBy: [Behavior Element](#behavior-element)

---

### Requirement Refined By Specification

Requirement that asks for specification, refined by a specification element.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability Parent](#capability-parent)
  * refinedBy: [Specification Element](#specification-element)

---

### Requirement Refined By Constraint

Requirement that asks for constraint definition, refined by a constraint element.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability Parent](#capability-parent)
  * refinedBy: [Constraint Element](#constraint-element)

---

### Requirement Refined By Source

Requirement that traces source material through a requirement-owned source refinement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Capability Parent](#capability-parent)
  * refinedBy: [Source Element](#source-element)

---
