# Elements

### Item Ontology

#### Metadata
  * type: ontology
  * ontology_base: https://example.org/items
  * ontology_prefix: item

#### Ontology
```turtle
@prefix item: <https://example.org/items#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
<https://example.org/items> a owl:Ontology .
item:Item a owl:Class .
item:name a owl:DatatypeProperty ; rdfs:domain item:Item .
```
---

### Item Lookup

Lists items for downstream consumers.

#### Query
```sparql
PREFIX item: <https://example.org/items#>
SELECT ?item WHERE { ?item a item:Item ; item:name ?name }
```

#### Metadata
  * type: semantic-query

#### Relations
  * use: [Item Ontology](#item-ontology)
---
