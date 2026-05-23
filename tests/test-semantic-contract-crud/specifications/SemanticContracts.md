# Elements

### Billing Feature

Billing feature.

#### Metadata
  * type: feature

#### Attachments
  * [Billing Ontology](#billing-ontology)
  * [Tax Ontology](#tax-ontology)

#### Relations
  * specifiedBy: [Billing Requirement](#billing-requirement)
---

### Billing Ontology

Billing ontology terms.

#### Metadata
  * type: ontology

#### Ontology
```turtle
@prefix billing: <urn:reqvire:test:billing:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

billing:BillingPayload a owl:Class .
```
---

### Billing Requirement

The system shall validate billing payload semantic structure.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Billing Feature](#billing-feature)
  * refinedBy: [Billing Shape Contract](#billing-shape-contract)
---

### Billing Shape Contract

Billing shape references a term declared by another ontology.

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Billing Requirement](#billing-requirement)

#### Shapes
```turtle
@prefix billing: <urn:reqvire:test:billing:> .
@prefix tax: <urn:reqvire:test:tax:> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

billing:BillingPayloadShape
  a sh:NodeShape ;
  sh:targetClass billing:BillingPayload ;
  sh:property [
    sh:path tax:VatRate ;
    sh:minCount 1 ;
  ] .
```
---

### Tax Ontology

Tax ontology terms.

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Billing Ontology](#billing-ontology)

#### Ontology
```turtle
@prefix tax: <urn:reqvire:test:tax:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

tax:VatRate a owl:DatatypeProperty .
```
---

### Customer Ontology

Customer ontology terms.

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Billing Ontology](#billing-ontology)

#### Ontology
```turtle
@prefix customer: <urn:reqvire:test:customer:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

customer:customerId a owl:DatatypeProperty .
```
---
