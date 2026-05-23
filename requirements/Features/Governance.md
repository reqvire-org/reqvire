# Elements

### Reqvire Governance Ontology Shape Profile

SHACL profile split from Reqvire Governance Ontology so ontology vocabulary remains first-class and requirement-owned semantic contracts carry closed-world constraints.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:GovernanceShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Element ;
  sh:property [
    sh:path reqvire:status ;
    sh:datatype xsd:string ;
    sh:in ("draft" "review" "approved") ;
  ] ;
  sh:property [
    sh:path reqvire:priority ;
    sh:datatype xsd:string ;
    sh:in ("low" "medium" "high" "critical") ;
  ] ;
  sh:property [
    sh:path reqvire:risk ;
    sh:datatype xsd:string ;
    sh:in ("low" "medium" "high" "critical") ;
  ] ;
  sh:property [
    sh:path reqvire:owner ;
    sh:datatype xsd:string ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Ontology and Semantic Contract Model](../Functional/Core/ModelManagement.md#ontology-and-semantic-contract-model)
---

