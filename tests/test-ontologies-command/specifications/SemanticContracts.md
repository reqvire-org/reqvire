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
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

# Classes
api:ServiceEndpoint a owl:Class .
api:Request a owl:Class .
api:Response a owl:Class .
api:ServiceEndpointAlias a owl:Class ;
  owl:equivalentClass api:ServiceEndpoint .

# Individuals
api:ProductionEndpoint a owl:NamedIndividual ;
  owl:sameAs api:PrimaryEndpoint .
api:PrimaryEndpoint a owl:NamedIndividual .
api:SecondaryEndpoint a api:ServiceEndpoint ;
  api:identifier "secondary" .

# Object properties
api:accepts a owl:ObjectProperty, owl:FunctionalProperty ;
  rdfs:domain api:ServiceEndpoint ;
  rdfs:range api:Request .

api:produces a owl:ObjectProperty ;
  rdfs:domain api:ServiceEndpoint ;
  rdfs:range api:Response .

api:exposes a owl:ObjectProperty, owl:TransitiveProperty ;
  rdfs:domain api:ServiceEndpoint ;
  rdfs:range [ a owl:Class ; owl:unionOf ( api:ServiceEndpoint api:ServiceEndpointAlias ) ] ;
  owl:inverseOf api:isExposedBy ;
  owl:propertyChainAxiom ( api:accepts api:produces ) .

api:isExposedBy a owl:ObjectProperty, owl:InverseFunctionalProperty ;
  rdfs:domain api:ServiceEndpoint ;
  rdfs:range api:ServiceEndpoint .

api:relatedEndpoint a owl:ObjectProperty, owl:SymmetricProperty, owl:ReflexiveProperty ;
  rdfs:domain api:ServiceEndpoint ;
  rdfs:range api:ServiceEndpoint .

# Datatype properties
api:identifier a owl:DatatypeProperty ;
  rdfs:domain api:ServiceEndpoint ;
  rdfs:range xsd:string .

api:endpointName a owl:DatatypeProperty ;
  rdfs:domain api:ServiceEndpoint ;
  rdfs:range xsd:string ;
  owl:equivalentProperty api:identifier .
```
---

### API Endpoint Requirement

The system shall expose service endpoint contracts.

#### Metadata
  * type: requirement

#### Relations
  * specify: [API Capability](#api-capability)
  * refinedBy: [API Endpoint Shape Contract](#api-endpoint-shape-contract)
  * refinedBy: [API Projection Query Sentinel](#api-projection-query-sentinel)
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
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

api:ServiceEndpointShape
  a sh:NodeShape ;
  sh:targetClass api:ServiceEndpoint ;
  sh:property [
    sh:path api:identifier ;
    sh:datatype xsd:string ;
    sh:minCount 1 ;
  ] .
```
---

### API Projection Query Sentinel

Semantic query sentinel for ontology export exclusion checks.

#### Metadata
  * type: semantic-query-contract

#### Relations
  * refine: [API Endpoint Requirement](#api-endpoint-requirement)

#### Query
```sparql
SELECT ?marker
WHERE {
  BIND("REQVIRE_ONTOLOGY_EXPORT_RAW_QUERY_SENTINEL" AS ?marker)
}
```
---
