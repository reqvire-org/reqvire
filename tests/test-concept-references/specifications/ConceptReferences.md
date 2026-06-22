# Elements

### API Ontology

API ontology terms with an invalid concept bridge.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Ontology
```turtle
@prefix concept: <https://example.test/concepts#> .
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix testonto: <https://example.test/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
testonto:ServiceEndpoint a owl:Class ;
  reqvire:mapsToConcept concept:MissingNativeConcept .
```
---
