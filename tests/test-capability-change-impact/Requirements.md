# Elements


### Verification Objective

Verification objective for concrete verification fixtures in this test model.

#### Metadata
  * type: verification-objective
---
### Product Capability

Product capability.

#### Metadata
  * type: capability

#### Concept References
  * [Product Payload](#product-payload)
  * [Shared Property](#shared-property)

#### Relations
  * specifiedBy: [Payload Requirement](#payload-requirement)
---

### Product Ontology

Product semantic vocabulary.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Ontology
```turtle
@prefix concept: <urn:reqvire:test:concept#> .
@prefix testonto: <https://example.test/ontology#> .
@prefix product: <urn:reqvire:test:product:> .
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
product:ProductPayload a owl:Class ;
  reqvire:mapsToConcept concept:ProductPayload ;
  product:state product:Initial .
```
---

### Product Concepts

Native product concept scheme.

Concept scheme for product payload terms.

#### Metadata
  * type: concept-scheme
  * concept_base: urn:reqvire:test:concept
  * concept_prefix: concept
---
### Product Payload

Native product payload concept.

A product payload concept.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Product Concepts](#product-concepts)
---
### Shared Property

Native shared property concept.

A shared property concept.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Product Payload](#product-payload)
  * broader: [Product Payload](#product-payload)
---
### Shared Ontology

Shared semantic vocabulary.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Relations
  * derivedFrom: [Product Ontology](#product-ontology)

#### Ontology
```turtle
@prefix concept: <urn:reqvire:test:concept#> .
@prefix testonto: <https://example.test/ontology#> .
@prefix shared: <urn:reqvire:test:shared:> .
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
shared:SharedProperty a owl:DatatypeProperty ;
  reqvire:mapsToConcept concept:SharedProperty ;
  shared:state shared:Initial .
```
---

### Contract Only Ontology

Semantic vocabulary used only through the semantic contract relation.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Relations
  * derivedFrom: [Shared Ontology](#shared-ontology)

#### Ontology
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix contract: <urn:reqvire:test:contract:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
contract:ContractProperty a owl:DatatypeProperty ;
  contract:state contract:Initial .
```
---

### Payload Requirement

The system shall produce payloads conforming to the product semantic contract.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Product Capability](#product-capability)
  * constrainedBy: [Payload Shape Contract](#payload-shape-contract)
  * verifiedBy: [Payload Verification](#payload-verification)
  * satisfiedBy: [payload_impl.txt](payload_impl.txt)
---

### Payload Shape Contract

Payload shape contract.

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Payload Requirement](#payload-requirement)
  * use: [Shared Ontology](#shared-ontology)
  * use: [Contract Only Ontology](#contract-only-ontology)

#### Shapes
```turtle
@prefix product: <urn:reqvire:test:product:> .
@prefix shared: <urn:reqvire:test:shared:> .
@prefix contract: <urn:reqvire:test:contract:> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

product:ProductPayloadShape
  a sh:NodeShape ;
  sh:targetClass product:ProductPayload ;
  sh:property [
    sh:path shared:SharedProperty ;
    sh:minCount 1 ;
  ] ;
  sh:property [
    sh:path contract:ContractProperty ;
    sh:minCount 1 ;
  ] .
```
---

### Payload Verification

Payload verification.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: [Payload Requirement](#payload-requirement)
  * satisfiedBy: [payload_test.txt](payload_test.txt)
---
