# Elements

This file contains an element with a relation to source elements.

### Related Element

This element has a relation to an element in the source file.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Source Element One](Source.md#source-element-one)
---

### Related Ontology

This ontology element has a relation to an ontology element in the source file.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/related-ontology
  * ontology_prefix: relatedont

#### Relations
  * derivedFrom: [Source Ontology](Source.md#source-ontology)

#### Ontology
```turtle
@prefix relatedont: <https://example.test/related-ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/related-ontology> a owl:Ontology ;
  owl:imports <https://example.test/source-ontology> .
relatedont:RelatedThing a owl:Class .
```
