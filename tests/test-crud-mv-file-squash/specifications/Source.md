# Elements

This file contains elements that will be moved to an existing target file using --squash.

### Source Element One

This is the first element that will be moved.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Requirement](Parent.md#parent-requirement)
---

### Source Element Two

This is the second element that will be moved.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Requirement](Parent.md#parent-requirement)
---

### Source Ontology

Ontology element that will be moved with the file squash.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/source-ontology
  * ontology_prefix: sourceont

#### Ontology
```turtle
@prefix sourceont: <https://example.test/source-ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/source-ontology> a owl:Ontology .
sourceont:SourceThing a owl:Class .
```
---
