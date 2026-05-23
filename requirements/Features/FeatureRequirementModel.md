# Elements

### Reqvire Feature Ontology Shape Profile

SHACL profile split from Reqvire Feature Ontology so ontology vocabulary remains first-class and requirement-owned semantic contracts carry closed-world constraints.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

reqvire:FeatureShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Feature ;
  sh:property [
    sh:path reqvire:refinedBy ;
    sh:class reqvire:Refinement ;
  ] ;
  sh:property [
    sh:path reqvire:specifiedBy ;
    sh:class reqvire:Requirement ;
  ] ;
  sh:property [
    sh:path reqvire:derive ;
    sh:class reqvire:Feature ;
  ] .

reqvire:FeatureOwnedRefinementShape
  a sh:NodeShape ;
  sh:targetClass reqvire:FeatureOwnedRefinement ;
  sh:property [
    sh:path reqvire:refine ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:class reqvire:Feature ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Ontology and Semantic Contract Model](../Functional/Core/ModelManagement.md#ontology-and-semantic-contract-model)
---

### Reqvire Requirement Ontology Shape Profile

SHACL profile split from Reqvire Requirement Ontology so ontology vocabulary remains first-class and requirement-owned semantic contracts carry closed-world constraints.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

reqvire:RequirementShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Requirement ;
  sh:property [
    sh:path reqvire:specify ;
    sh:minCount 1 ;
    sh:class reqvire:Feature ;
  ] ;
  sh:property [
    sh:path reqvire:refinedBy ;
    sh:class reqvire:Refinement ;
  ] ;
  sh:property [
    sh:path reqvire:verifiedBy ;
    sh:class reqvire:Verification ;
  ] .

reqvire:RequirementOwnedRefinementShape
  a sh:NodeShape ;
  sh:targetClass reqvire:RequirementOwnedRefinement ;
  sh:property [
    sh:path reqvire:refine ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:class reqvire:Requirement ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Ontology and Semantic Contract Model](../Functional/Core/ModelManagement.md#ontology-and-semantic-contract-model)
---

### Reqvire Semantic Contract Ontology Shape Profile

SHACL profile split from Reqvire Semantic Contract Ontology so ontology vocabulary remains first-class and requirement-owned semantic contracts carry closed-world constraints.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:SemanticContractShape
  a sh:NodeShape ;
  sh:targetClass reqvire:SemanticContract ;
  sh:property [
    sh:path reqvire:semanticContractIri ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:semanticContractKind ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("ontology" "semantic-contract") ;
  ] ;
  sh:property [
    sh:path reqvire:ontologyText ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:shapesText ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:refine ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:class reqvire:Element ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Ontology and Semantic Contract Model](../Functional/Core/ModelManagement.md#ontology-and-semantic-contract-model)
---

