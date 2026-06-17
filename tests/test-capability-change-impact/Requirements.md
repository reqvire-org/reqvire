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
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Ontology
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix product: <urn:reqvire:test:product:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
product:ProductPayload a owl:Class ;
  product:state product:Initial .
```
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
@prefix testonto: <https://example.test/ontology#> .
@prefix shared: <urn:reqvire:test:shared:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
shared:SharedProperty a owl:DatatypeProperty ;
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
  * verify: [Payload Requirement](#payload-requirement)
  * satisfiedBy: [payload_test.txt](payload_test.txt)
---
