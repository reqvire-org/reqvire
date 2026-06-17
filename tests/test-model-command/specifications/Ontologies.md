# Elements

### Model Command Ontology

Reqvire model command vocabulary used by the model command fixture.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Ontology

```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix reqviretest: <urn:reqvire:test:model-command:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
reqviretest:ModelCommandView a owl:Class .
```
---
