# Elements

### API Capability

API capability.

#### Metadata
  * type: capability

#### Attachments
  * [API Ontology](#api-ontology)

#### Relations
  * specifiedBy: [API Endpoint Requirement](#api-endpoint-requirement)
---

### API Ontology

API vocabulary.

#### Metadata
  * type: ontology

#### Ontology
```turtle
@prefix api: <urn:reqvire:test:api:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

api:ServiceEndpoint a owl:Class .
```
---

### API Endpoint Requirement

The system shall expose service endpoint contracts.

#### Metadata
  * type: requirement

#### Relations
  * specify: [API Capability](#api-capability)
  * refinedBy: [API Endpoint Shape Contract](#api-endpoint-shape-contract)
---

### API Endpoint Shape Contract

API endpoint shape contract.

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [API Endpoint Requirement](#api-endpoint-requirement)

#### Shapes
```turtle
@prefix api: <urn:reqvire:test:api:> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

api:ServiceEndpointShape
  a sh:NodeShape ;
  sh:targetClass api:ServiceEndpoint .
```
---
