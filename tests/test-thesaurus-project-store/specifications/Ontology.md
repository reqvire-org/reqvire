# Elements

### Store Test Ontology

Structural API vocabulary used to verify ontology-to-concept mapping evidence in the Project Store Thesaurus projection.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: ex

#### Ontology
```turtle
@prefix concept: <https://example.test/thesaurus#> .
@prefix ex: <https://example.test/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

<https://example.test/ontology> a owl:Ontology .

ex:ServiceEndpoint
  a owl:Class ;
  rdfs:label "Service Endpoint Ontology Class" ;
  reqvire:mapsToConcept concept:ServiceEndpoint .
```
