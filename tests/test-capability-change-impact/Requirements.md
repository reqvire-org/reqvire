# Elements

### Product Capability

Product capability.

#### Metadata
  * type: capability

#### Attachments
  * [Product Ontology](#product-ontology)
  * [Shared Ontology](#shared-ontology)

#### Relations
  * specifiedBy: [Payload Requirement](#payload-requirement)
---

### Product Ontology

Product semantic vocabulary.

#### Metadata
  * type: ontology

#### Ontology
```turtle
@prefix product: <urn:reqvire:test:product:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

product:ProductPayload a owl:Class ;
  product:state product:Initial .
```
---

### Shared Ontology

Shared semantic vocabulary.

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Product Ontology](#product-ontology)

#### Ontology
```turtle
@prefix shared: <urn:reqvire:test:shared:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

shared:SharedProperty a owl:DatatypeProperty ;
  shared:state shared:Initial .
```
---

### Payload Requirement

The system shall produce payloads conforming to the product semantic contract.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Product Capability](#product-capability)
  * refinedBy: [Payload Shape Contract](#payload-shape-contract)
  * verifiedBy: [Payload Verification](#payload-verification)
  * satisfiedBy: [payload_impl.txt](payload_impl.txt)
---

### Payload Shape Contract

Payload shape contract.

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Payload Requirement](#payload-requirement)

#### Shapes
```turtle
@prefix product: <urn:reqvire:test:product:> .
@prefix shared: <urn:reqvire:test:shared:> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

product:ProductPayloadShape
  a sh:NodeShape ;
  sh:targetClass product:ProductPayload ;
  sh:property [
    sh:path shared:SharedProperty ;
    sh:minCount 1 ;
  ] .
```
---

### Payload Verification

Payload verification.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Payload Requirement](#payload-requirement)
  * satisfiedBy: [payload_test.txt](payload_test.txt)
---
