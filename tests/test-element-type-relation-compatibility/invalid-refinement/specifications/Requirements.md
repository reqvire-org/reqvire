# Elements


### Test Capability Test Element Type Relation Compatibility Invalid Refinement Specifications

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

### Target Capability

A capability for testing.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-element-type-relation-compatibility-invalid-refinement-specifications)
---

### Semantic Contract Ontology

Ontology vocabulary used by the invalid semantic-contract refinement fixture.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/invalid-semantic-contract
  * ontology_prefix: invalidsc

#### Ontology
```turtle
@prefix invalidsc: <https://example.test/invalid-semantic-contract#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/invalid-semantic-contract> a owl:Ontology .
invalidsc:Payload a owl:Class .
invalidsc:payloadId a owl:DatatypeProperty .
```

---

### Semantic Contract with Refine

INVALID: Semantic contract must use constrain/constrainedBy instead of define/definedBy.

#### Metadata
  * type: semantic-contract

#### Relations
  * define: [Target Capability](#target-capability)
  * use: [Semantic Contract Ontology](#semantic-contract-ontology)

#### Shapes
```turtle
@prefix invalidsc: <https://example.test/invalid-semantic-contract#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

invalidsc:PayloadShape
  a sh:NodeShape ;
  sh:targetClass invalidsc:Payload ;
  sh:property [
    sh:path invalidsc:payloadId ;
    sh:datatype xsd:string ;
  ] .
```

---

### Requirement Refined By Semantic Contract

INVALID: Requirement must use constrainedBy instead of definedBy for semantic contracts.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target Capability](#target-capability)
  * definedBy: [Semantic Contract with Constraint Relation](#semantic-contract-with-constraint-relation)

---

### Semantic Contract with Constraint Relation

Semantic contract with valid semantic-contract relations, used as an invalid definedBy target.

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Requirement Refined By Semantic Contract](#requirement-refined-by-semantic-contract)
  * use: [Semantic Contract Ontology](#semantic-contract-ontology)

#### Shapes
```turtle
@prefix invalidsc: <https://example.test/invalid-semantic-contract#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

invalidsc:PayloadConstraintShape
  a sh:NodeShape ;
  sh:targetClass invalidsc:Payload ;
  sh:property [
    sh:path invalidsc:payloadId ;
    sh:datatype xsd:string ;
  ] .
```

---

### Constraint with DerivedFrom

INVALID: Constraint (refinement type) cannot have Relations subsection.

#### Metadata
  * type: constraint

#### Relations
  * derivedFrom: [Target Capability](#target-capability)

---

### Constraint with Trace

INVALID: Constraint (refinement type) cannot have Relations subsection.

#### Metadata
  * type: constraint

#### Relations
  * trace: [Target Capability](#target-capability)

---

### Behavior with DerivedFrom

INVALID: Behavior (refinement type) cannot have Relations subsection.

#### Metadata
  * type: behavior

#### Relations
  * derivedFrom: [Target Capability](#target-capability)

---

### Behavior with Trace

INVALID: Behavior (refinement type) cannot have Relations subsection.

#### Metadata
  * type: behavior

#### Relations
  * trace: [Target Capability](#target-capability)

---

### Specification with DerivedFrom

INVALID: Specification (refinement type) cannot have Relations subsection.

#### Metadata
  * type: specification

#### Relations
  * derivedFrom: [Target Capability](#target-capability)

---

### Specification with Trace

INVALID: Specification (refinement type) cannot have Relations subsection.

#### Metadata
  * type: specification

#### Relations
  * trace: [Target Capability](#target-capability)

---
