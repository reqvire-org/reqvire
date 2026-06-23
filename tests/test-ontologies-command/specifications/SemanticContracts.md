# Elements

### API Capability

API capability.

#### Metadata
  * type: capability

#### Concept References
  * [Service Endpoint](#service-endpoint)
  * [Native Traceability](#native-traceability)

#### Relations
  * specifiedBy: [API Endpoint Requirement](#api-endpoint-requirement)
---

### API Ontology

API vocabulary.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Relations
  * derivedFrom: [Conceptual Layer Ontology](#conceptual-layer-ontology)

#### External Ontology
  * prefix: ext
  * namespace: https://example.test/external#
  * resource: https://example.test/external
  * source: references/external.ttl
  * format: turtle

#### External Ontology
  * prefix: jsonext
  * namespace: https://example.test/jsonld-external#
  * resource: https://example.test/jsonld-external
  * source: references/external.jsonld
  * format: jsonld

#### External Ontology
  * prefix: rdfext
  * namespace: https://example.test/rdf-external#
  * resource: https://example.test/rdf-external
  * source: references/external.rdf
  * format: rdf

#### Ontology
```turtle
@prefix conceptual: <https://example.test/ontology/conceptual#> .
@prefix concept: <https://example.test/concepts#> .
@prefix testonto: <https://example.test/ontology#> .
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix skos: <http://www.w3.org/2004/02/skos/core#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

<https://example.test/ontology> a owl:Ontology ;
  owl:imports <https://example.test/ontology/conceptual> ;
  owl:imports <https://example.test/imported> ;
  owl:imports <https://example.test/imported> ;
  rdfs:label "Test ontology" .

# Classes
testonto:ServiceEndpoint a owl:Class ;
  reqvire:mapsToConcept concept:ServiceEndpoint .
testonto:Request a owl:Class .
testonto:Response a owl:Class .
testonto:ServiceEndpointAlias a owl:Class ;
  owl:equivalentClass testonto:ServiceEndpoint .
testonto:VerifiedEndpoint a owl:Class ;
  rdfs:label "Verified endpoint" ;
  rdfs:comment "Endpoint class that exercises standard reserved annotation vocabulary in the test fixture." ;
  rdfs:subClassOf owl:Thing,
    [
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

testonto:isActive a owl:DatatypeProperty ;
  rdfs:domain testonto:ServiceEndpoint ;
  rdfs:range xsd:boolean .

testonto:retryCount a owl:DatatypeProperty ;
  rdfs:domain testonto:ServiceEndpoint ;
  rdfs:range xsd:integer .

testonto:documentationUri a owl:DatatypeProperty ;
  rdfs:domain testonto:ServiceEndpoint ;
  rdfs:range xsd:anyURI .

testonto:literalText a owl:DatatypeProperty ;
  rdfs:domain testonto:ServiceEndpoint ;
  rdfs:range rdf:PlainLiteral .

testonto:anyLiteral a owl:DatatypeProperty ;
  rdfs:domain testonto:ServiceEndpoint ;
  rdfs:range rdfs:Literal .

testonto:realValue a owl:DatatypeProperty ;
  rdfs:domain testonto:ServiceEndpoint ;
  rdfs:range owl:real .

testonto:rationalValue a owl:DatatypeProperty ;
  rdfs:domain testonto:ServiceEndpoint ;
  rdfs:range owl:rational .

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

#### Concept References
  * [Service Endpoint](#service-endpoint)

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
@prefix ext: <https://example.test/external#> .
@prefix jsonext: <https://example.test/jsonld-external#> .
@prefix rdfext: <https://example.test/rdf-external#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xs: <http://www.w3.org/2001/XMLSchema#> .

testonto:ServiceEndpointShape
  a sh:NodeShape ;
  sh:targetClass testonto:ServiceEndpoint ;
  sh:property [
    sh:path testonto:identifier ;
    sh:datatype xs:string ;
    sh:minCount 1 ;
  ] ;
  sh:property [
    sh:path testonto:isActive ;
    sh:datatype xs:boolean ;
  ] ;
  sh:property [
    sh:path testonto:retryCount ;
    sh:datatype xs:integer ;
  ] ;
  sh:property [
    sh:path testonto:documentationUri ;
    sh:datatype xs:anyURI ;
  ] ;
  sh:property [
    sh:path testonto:identifier ;
    sh:datatype ext:ExternalCode ;
  ] ;
  sh:property [
    sh:path ext:externalCode ;
    sh:datatype ext:ExternalCode ;
  ] ;
  sh:property [
    sh:path jsonext:jsonExternalCode ;
    sh:datatype jsonext:JsonExternalCode ;
  ] ;
  sh:property [
    sh:path rdfext:rdfExternalCode ;
    sh:datatype rdfext:RdfExternalCode ;
  ] .
```
---

### Conceptual Layer Ontology

Concept vocabulary that uses the built-in SKOS external ontology source without declaring a local External Ontology section.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology/conceptual
  * ontology_prefix: conceptual

#### Ontology
```turtle
@prefix conceptual: <https://example.test/ontology/conceptual#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix skos: <http://www.w3.org/2004/02/skos/core#> .

<https://example.test/ontology/conceptual> a owl:Ontology ;
  rdfs:label "Conceptual layer ontology" .

conceptual:TraceabilityConstruct a owl:Class ;
  rdfs:label "Traceability construct" .

conceptual:anchorsConcept a owl:ObjectProperty ;
  rdfs:domain conceptual:TraceabilityConstruct ;
  rdfs:range skos:Concept .
```
---

### Native Concepts

Native Markdown concept scheme for Reqvire concept element export tests.

Native concept scheme authored as Reqvire Markdown.

#### Metadata
  * type: concept-scheme
  * concept_base: https://example.test/concepts
  * concept_prefix: concept
---
### Engineering Knowledge

Native top concept for engineering knowledge.

Knowledge used to describe, constrain, verify, and evolve engineered systems.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Native Concepts](#native-concepts)
---
### Native Traceability

Native concept for traceability.

The ability to follow engineering intent from source need through requirements, implementation, verification, and evidence.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Engineering Knowledge](#engineering-knowledge)
  * broader: [Engineering Knowledge](#engineering-knowledge)
  * related: [Verification Evidence](#verification-evidence)
#### Labels
  * altLabel: Trace link analysis

#### Scope Note
Use this concept for engineering artifact traceability, not runtime distributed tracing.

#### Examples
  * A requirement traces to implementation and verification evidence.
---
### Verification Evidence

Native concept for verification evidence.

Evidence that supports a verification result.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Native Concepts](#native-concepts)
  * related: [Native Traceability](#native-traceability)
---
### Service Endpoint

Native concept for service endpoint vocabulary.

An externally visible service interaction point that accepts requests or produces responses.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Native Concepts](#native-concepts)
  * related: [Native Traceability](#native-traceability)
---
