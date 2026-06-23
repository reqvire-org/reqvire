# Elements


### Verification Objective

Verification objective for concrete verification fixtures in this test model.

#### Metadata
  * type: verification-objective
---
### Product Capability

The top product capability for collect traversal.

#### Metadata
  * type: capability

#### Relations
  * derive: [Child Capability](#child-capability)
---

### Child Capability

The child product capability that specifies the root requirement.

#### Metadata
  * type: capability

#### Concept References
  * [Collect Contract](#collect-contract)

#### Relations
  * derivedFrom: [Product Capability](#product-capability)
  * specifiedBy: [Root Requirement](#root-requirement)

---

### Collect Ontology

Ontology content for collect traversal.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Relations
  * derive: [Collect Child Ontology](#collect-child-ontology)

#### Ontology
```turtle
@prefix concept: <urn:reqvire:test:concept#> .
@prefix testonto: <https://example.test/ontology#> .
@prefix ex: <urn:reqvire:test:collect:> .
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
ex:CollectContract a owl:Class ;
  reqvire:mapsToConcept concept:CollectContract .
```
---

### Collect Concepts

Native collect concept scheme.

Concept scheme for collect traversal.

#### Metadata
  * type: concept-scheme
  * concept_base: urn:reqvire:test:concept
  * concept_prefix: concept
---
### Collect Contract

Native collect contract concept.

Concept used to ground collect contract context.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Collect Concepts](#collect-concepts)
---
### Collect Child Ontology

Child ontology content for ontology-start collect traversal.

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Collect Ontology](#collect-ontology)

#### Ontology
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix ex: <urn:reqvire:test:collect:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:CollectChildTerm a owl:Class .
```
---

### Collect Shape Contract

Semantic contract content for ontology-start collect traversal.

#### Metadata
  * type: semantic-contract

#### Relations
  * use: [Collect Child Ontology](#collect-child-ontology)

#### Shapes
```turtle
@prefix ex: <urn:reqvire:test:collect:> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

ex:CollectContractShape
  a sh:NodeShape ;
  sh:targetClass ex:CollectChildTerm .
```
---

### Root Requirement

The root requirement for testing content collection.

#### Details
This is the top-level requirement that has no derivedFrom relations.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Child Capability](#child-capability)
---

### Mid-Level Requirement

The mid-level requirement derives from the root.

#### Details
This requirement sits in the middle of the hierarchy.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
---

### Leaf Requirement

The leaf requirement at the bottom of the hierarchy.

#### Details
This is the leaf requirement that derives from the mid-level.

#### Reused Contract Context
  * [Design Doc Specification](DesignDoc.md#design-doc-specification)

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Mid-Level Requirement](#mid-level-requirement)
---

### Design Owner Requirement

Owner requirement for design contract specification.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Design Capability](#design-capability)
  * definedBy: [Design Doc Specification](DesignDoc.md#design-doc-specification)
---

### Design Capability

Capability that owns reusable design documentation.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [Design Owner Requirement](#design-owner-requirement)
---

### Test Verification

A verification element to test error handling.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: [Leaf Requirement](#leaf-requirement)
---
