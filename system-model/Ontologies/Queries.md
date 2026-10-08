# Elements

### external-used-term-seed-query

Select external ontology terms referenced by authored ontology, SHACL, concept-reference, model, or generated semantic projection facts whose IRIs fall under declared external namespaces.

#### Query
```sparql
PREFIX reqvire: <https://www.reqvire.org/ontology#>

SELECT DISTINCT ?term
WHERE {
  ?source a reqvire:ExternalOntologySource ;
    reqvire:externalOntologyNamespace ?namespace .
  {
    ?block reqvire:referencesTerm ?term .
  }
  UNION {
    ?block reqvire:declaresTerm ?term .
  }
  UNION {
    ?projection reqvire:conceptReference ?term .
  }
  UNION {
    ?projection reqvire:constructSubject|reqvire:constructPredicate|reqvire:constructObject|reqvire:constructProperty ?term .
  }
  FILTER(isIRI(?term))
  FILTER(STRSTARTS(STR(?term), STR(?namespace)))
}
```

#### Metadata
  * type: semantic-query

#### Relations
  * use: [Reqvire Semantic Export Ontology](SemanticExport.md#reqvire-semantic-export-ontology)
  * use: [Reqvire Relation Ontology](RelationsAndImpact.md#reqvire-relation-ontology)
---

### external-used-term-direct-description-construct

Construct direct raw-external-graph description triples for seed external ontology terms only.

#### Query
```sparql
PREFIX reqvire: <https://www.reqvire.org/ontology#>

CONSTRUCT {
  ?term ?p ?o .
}
WHERE {
  ?subset a reqvire:UsedExternalOntologySubset ;
    reqvire:externalUsedTerm ?term ;
    reqvire:externalSubsetGraph ?rawExternalGraph .
  GRAPH ?rawExternalGraph {
    ?term ?p ?o .
  }
}
```

#### Metadata
  * type: semantic-query

#### Relations
  * use: [Reqvire Semantic Export Ontology](SemanticExport.md#reqvire-semantic-export-ontology)
  * use: [Reqvire Relation Ontology](RelationsAndImpact.md#reqvire-relation-ontology)
---

### external-used-term-support-closure-construct

Construct one-hop support facts for used external terms across selected RDF, RDFS, and OWL support predicates.

#### Query
```sparql
PREFIX owl: <http://www.w3.org/2002/07/owl#>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX reqvire: <https://www.reqvire.org/ontology#>

CONSTRUCT {
  ?support ?p ?supportObject .
}
WHERE {
  ?subset a reqvire:UsedExternalOntologySubset ;
    reqvire:externalUsedTerm ?term ;
    reqvire:externalSubsetGraph ?rawExternalGraph .
  GRAPH ?rawExternalGraph {
    ?term ?p ?support .
    FILTER(?p IN (rdf:type, rdfs:subClassOf, rdfs:subPropertyOf, rdfs:domain, rdfs:range, owl:equivalentClass, owl:equivalentProperty, owl:inverseOf, owl:onProperty, owl:someValuesFrom, owl:allValuesFrom, owl:hasValue))
    FILTER(isIRI(?support) || isBlank(?support))
    OPTIONAL {
      ?support ?p ?supportObject .
      FILTER(?p IN (rdf:type, rdfs:subClassOf, rdfs:subPropertyOf, rdfs:domain, rdfs:range, owl:equivalentClass, owl:equivalentProperty, owl:inverseOf, owl:onProperty, owl:someValuesFrom, owl:allValuesFrom, owl:hasValue))
    }
  }
}
```

#### Metadata
  * type: semantic-query

#### Relations
  * use: [Reqvire Semantic Export Ontology](SemanticExport.md#reqvire-semantic-export-ontology)
  * use: [Reqvire Relation Ontology](RelationsAndImpact.md#reqvire-relation-ontology)
---

### external-used-term-annotation-construct

Construct label, comment, preferred-label, definition, and description annotation triples for used external terms and support terms.

#### Query
```sparql
PREFIX dcterms: <http://purl.org/dc/terms/>
PREFIX owl: <http://www.w3.org/2002/07/owl#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX reqvire: <https://www.reqvire.org/ontology#>
PREFIX skos: <http://www.w3.org/2004/02/skos/core#>

CONSTRUCT {
  ?describedTerm ?annotationProperty ?annotationValue .
}
WHERE {
  ?subset a reqvire:UsedExternalOntologySubset ;
    reqvire:externalSubsetGraph ?rawExternalGraph .
  {
    ?subset reqvire:externalUsedTerm ?describedTerm .
  }
  UNION {
    ?subset reqvire:externalUsedTerm ?term .
    GRAPH ?rawExternalGraph {
      ?term ?supportProperty ?describedTerm .
      FILTER(?supportProperty IN (rdfs:subClassOf, rdfs:subPropertyOf, rdfs:domain, rdfs:range, owl:equivalentClass, owl:equivalentProperty, owl:inverseOf, owl:onProperty, owl:someValuesFrom, owl:allValuesFrom, owl:hasValue))
    }
  }
  GRAPH ?rawExternalGraph {
    ?describedTerm ?annotationProperty ?annotationValue .
    FILTER(?annotationProperty IN (rdfs:label, rdfs:comment, skos:prefLabel, skos:definition, dcterms:description))
  }
}
```

#### Metadata
  * type: semantic-query

#### Relations
  * use: [Reqvire Semantic Export Ontology](SemanticExport.md#reqvire-semantic-export-ontology)
  * use: [Reqvire Relation Ontology](RelationsAndImpact.md#reqvire-relation-ontology)
---

### relation-family-normalized-projection

Materialize canonical forward and inverse relation-family predicates for every authored Reqvire model relation so semantic search can query relation meaning rather than raw Markdown relation tokens.

#### Query
```sparql
PREFIX reqvire: <https://www.reqvire.org/ontology#>

CONSTRUCT {
  ?canonicalSource ?forwardProperty ?canonicalTarget .
  ?canonicalTarget ?inverseProperty ?canonicalSource .
}
WHERE {
  ?relation a reqvire:ModelRelation ;
    reqvire:relationSource ?source ;
    reqvire:relationTarget ?target ;
    reqvire:relationType ?relationName .

  ?rule a reqvire:RelationRule ;
    reqvire:relationName ?relationName ;
    reqvire:relationDirection ?direction ;
    reqvire:normalizedForwardProperty ?forwardProperty ;
    reqvire:normalizedInverseProperty ?inverseProperty .

  BIND(IF(?direction = "inverse", ?target, ?source) AS ?canonicalSource)
  BIND(IF(?direction = "inverse", ?source, ?target) AS ?canonicalTarget)
}
```

#### Produces
  * family: reqvire:hierarchyRelationFamily
  * family: reqvire:capabilitySpecificationRelationFamily
  * family: reqvire:contractOwnershipRelationFamily
  * family: reqvire:semanticContractConstraintRelationFamily
  * family: reqvire:semanticContractOntologyUseRelationFamily
  * family: reqvire:verificationRelationFamily
  * family: reqvire:satisfactionRelationFamily
  * family: reqvire:contractBindingRelationFamily
  * family: reqvire:contractReferenceRelationFamily

#### Metadata
  * type: semantic-query

#### Relations
  * use: [Reqvire Semantic Export Ontology](SemanticExport.md#reqvire-semantic-export-ontology)
  * use: [Reqvire Relation Ontology](RelationsAndImpact.md#reqvire-relation-ontology)
---

### concept-relation-normalized-projection

Materialize canonical SKOS concept relation facts from authored native concept relations so child broader, parent narrower, symmetric association, and mapping neighborhoods are queryable without client-side inverse inference.

#### Query
```sparql
PREFIX reqvire: <https://www.reqvire.org/ontology#>
PREFIX skos: <http://www.w3.org/2004/02/skos/core#>

CONSTRUCT {
  ?canonicalSource ?forwardProperty ?canonicalTarget .
  ?canonicalTarget ?inverseProperty ?canonicalSource .
}
WHERE {
  ?relation a reqvire:ModelRelation ;
    reqvire:relationSource ?source ;
    reqvire:relationTarget ?target ;
    reqvire:relationType ?relationName .

  VALUES (?relationName ?direction ?forwardProperty ?inverseProperty) {
    ("broader" "forward" skos:broader skos:narrower)
    ("narrower" "inverse" skos:broader skos:narrower)
    ("related" "forward" skos:related skos:related)
    ("exactMatch" "forward" skos:exactMatch skos:exactMatch)
    ("closeMatch" "forward" skos:closeMatch skos:closeMatch)
  }

  BIND(IF(?direction = "inverse", ?target, ?source) AS ?canonicalSource)
  BIND(IF(?direction = "inverse", ?source, ?target) AS ?canonicalTarget)
}
```

#### Produces
  * property: skos:broader
  * property: skos:narrower
  * property: skos:related
  * property: skos:exactMatch
  * property: skos:closeMatch

#### Metadata
  * type: semantic-query

#### Relations
  * use: [Reqvire Semantic Export Ontology](SemanticExport.md#reqvire-semantic-export-ontology)
  * use: [Reqvire Relation Ontology](RelationsAndImpact.md#reqvire-relation-ontology)
---
