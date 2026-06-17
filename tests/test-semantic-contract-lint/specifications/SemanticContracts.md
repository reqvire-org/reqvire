# Elements

### Billing Capability

Billing capability.

#### Metadata
  * type: capability

#### Attachments
  * [Billing Ontology](#billing-ontology)
  * [Customer Ontology](#customer-ontology)
  * [Tax Ontology](#tax-ontology)

#### Relations
  * specifiedBy: [Billing Requirement](#billing-requirement)
---

### Billing Requirement

The system shall support billing payloads.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Billing Capability](#billing-capability)
  * refinedBy: [Billing Shape Contract](#billing-shape-contract)
---

### Billing Ontology

Billing ontology terms.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Ontology
```turtle
@prefix billing: <urn:reqvire:test:billing:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

billing:Invoice a owl:Class .
billing:BillingPayload a owl:Class .
```
---

### Customer Ontology

Customer ontology terms.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Relations
  * derivedFrom: [Billing Ontology](#billing-ontology)

#### Ontology
```turtle
@prefix billing: <urn:reqvire:test:billing:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

billing:customerId a owl:DatatypeProperty .
```
---

### Tax Ontology

Tax ontology terms.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Relations
  * derivedFrom: [Billing Ontology](#billing-ontology)

#### Ontology
```turtle
@prefix tax: <urn:reqvire:test:tax:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

tax:VatRate a owl:DatatypeProperty .
```
---

### Billing Shape Contract

Billing shape references native and attached ontology terms.

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Billing Requirement](#billing-requirement)
  * use: [Tax Ontology](#tax-ontology)

#### Shapes
```turtle
@prefix billing: <urn:reqvire:test:billing:> .
@prefix tax: <urn:reqvire:test:tax:> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

billing:BillingPayloadShape
  a sh:NodeShape ;
  sh:targetClass billing:Invoice ;
  sh:property [
    sh:path billing:customerId ;
    sh:minCount 1 ;
  ] ;
  sh:property [
    sh:path tax:VatRate ;
    sh:minCount 1 ;
  ] .
```
---
