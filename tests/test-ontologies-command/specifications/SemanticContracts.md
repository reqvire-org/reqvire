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
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Ontology
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

<https://example.test/ontology> a owl:Ontology ;
  owl:imports <https://example.test/imported> ;
  owl:imports <https://example.test/imported> ;
  rdfs:label "Test ontology" .

# Classes
testonto:ServiceEndpoint a owl:Class .
testonto:Request a owl:Class .
testonto:Response a owl:Class .
testonto:ServiceEndpointAlias a owl:Class ;
  owl:equivalentClass testonto:ServiceEndpoint .
testonto:VerifiedEndpoint a owl:Class ;
  rdfs:subClassOf [
    a owl:Restriction ;
    owl:onProperty testonto:produces ;
    owl:someValuesFrom testonto:Response
  ] .

# Individuals
testonto:ProductionEndpoint a owl:NamedIndividual ;
  owl:sameAs testonto:PrimaryEndpoint .
testonto:PrimaryEndpoint a owl:NamedIndividual .
testonto:SecondaryEndpoint a testonto:ServiceEndpoint ;
  testonto:identifier "secondary" .

# Object properties
testonto:accepts a owl:ObjectProperty, owl:FunctionalProperty ;
  rdfs:domain testonto:ServiceEndpoint ;
  rdfs:range testonto:Request .

testonto:produces a owl:ObjectProperty ;
  rdfs:domain testonto:ServiceEndpoint ;
  rdfs:range testonto:Response .

testonto:exposes a owl:ObjectProperty, owl:TransitiveProperty ;
  rdfs:domain testonto:ServiceEndpoint ;
  rdfs:range [ a owl:Class ; owl:unionOf ( testonto:ServiceEndpoint testonto:ServiceEndpointAlias ) ] ;
  owl:inverseOf testonto:isExposedBy ;
  owl:propertyChainAxiom ( testonto:accepts testonto:produces ) .

testonto:isExposedBy a owl:ObjectProperty, owl:InverseFunctionalProperty ;
  rdfs:domain testonto:ServiceEndpoint ;
  rdfs:range testonto:ServiceEndpoint .

testonto:relatedEndpoint a owl:ObjectProperty, owl:SymmetricProperty, owl:ReflexiveProperty ;
  rdfs:domain testonto:ServiceEndpoint ;
  rdfs:range testonto:ServiceEndpoint .

# Datatype properties
testonto:identifier a owl:DatatypeProperty ;
  rdfs:domain testonto:ServiceEndpoint ;
  rdfs:range xsd:string .

testonto:endpointName a owl:DatatypeProperty ;
  rdfs:domain testonto:ServiceEndpoint ;
  rdfs:range xsd:string ;
  owl:equivalentProperty testonto:identifier .
```
---

### API Endpoint Requirement

The system shall expose service endpoint contracts.

#### Metadata
  * type: requirement

#### Relations
  * specify: [API Capability](#api-capability)
  * constrainedBy: [API Endpoint Shape Contract](#api-endpoint-shape-contract)
---

### API Endpoint Shape Contract

API endpoint shape contract.

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [API Endpoint Requirement](#api-endpoint-requirement)
  * use: [API Ontology](#api-ontology)

#### Shapes
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

testonto:ServiceEndpointShape
  a sh:NodeShape ;
  sh:targetClass testonto:ServiceEndpoint ;
  sh:property [
    sh:path testonto:identifier ;
    sh:datatype xsd:string ;
    sh:minCount 1 ;
  ] .
```
---
