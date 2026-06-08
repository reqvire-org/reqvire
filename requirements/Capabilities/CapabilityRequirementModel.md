# Elements

### Reqvire Capability Ontology Shape Profile

Defines SHACL constraints for capability structure, ownership, refinement, verification, and forbidden implementation satisfaction edges.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:CapabilityShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Capability ;
  sh:property [
    sh:path reqvire:derive ;
    sh:class reqvire:Capability ;
  ] ;
  sh:property [
    sh:path reqvire:derivedFrom ;
    sh:class reqvire:Capability ;
  ] ;
  sh:property [
    sh:path reqvire:refinedBy ;
    sh:class reqvire:Refinement ;
  ] ;
  sh:property [
    sh:path reqvire:specifiedBy ;
    sh:class reqvire:Requirement ;
  ] ;
  sh:property [
    sh:path reqvire:verifiedBy ;
    sh:class reqvire:Verification ;
  ] ;
  sh:property [
    sh:path reqvire:satisfiedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:satisfy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:specify ;
    sh:maxCount 0 ;
  ] .

reqvire:CapabilityOwnedRefinementShape
  a sh:NodeShape ;
  sh:targetClass reqvire:CapabilityOwnedRefinement ;
  sh:property [
    sh:path reqvire:refine ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:class reqvire:Capability ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Capability Element Semantic Contract](../Functional/Core/ModelManagement.md#capability-element-semantic-contract)
---

### Reqvire Requirement Ontology Shape Profile

Defines SHACL constraints for requirement ownership, hierarchy, refinements, verification, and capability specification.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:RequirementShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Requirement ;
  sh:or (
    [
      sh:property [
        sh:path reqvire:specify ;
        sh:minCount 1 ;
        sh:class reqvire:Capability ;
      ]
    ]
    [
      sh:property [
        sh:path reqvire:derivedFrom ;
        sh:minCount 1 ;
        sh:class reqvire:Requirement ;
      ]
    ]
  ) ;
  sh:property [
    sh:path reqvire:specify ;
    sh:class reqvire:Capability ;
  ] ;
  sh:property [
    sh:path reqvire:derivedFrom ;
    sh:class reqvire:Requirement ;
  ] ;
  sh:property [
    sh:path reqvire:derive ;
    sh:class reqvire:Requirement ;
  ] ;
  sh:property [
    sh:path reqvire:refinedBy ;
    sh:class reqvire:Refinement ;
  ] ;
  sh:property [
    sh:path reqvire:verifiedBy ;
    sh:class reqvire:Verification ;
  ] ;
  sh:property [
    sh:path reqvire:satisfiedBy ;
    sh:class reqvire:Artifact ;
  ] ;
  sh:property [
    sh:path reqvire:requirementObligationText ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:satisfy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:specifiedBy ;
    sh:maxCount 0 ;
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
  * refine: [Requirement Element Semantic Contract](../Functional/Core/ModelManagement.md#requirement-element-semantic-contract)
---

### Reqvire Semantic Contract Ontology Shape Profile

Defines SHACL constraints for semantic-contract identity, profile kind, shape content, and owning requirement.

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
    sh:datatype xsd:anyURI ;
  ] ;
  sh:property [
    sh:path reqvire:semanticContractKind ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("semantic-contract") ;
  ] ;
  sh:property [
    sh:path reqvire:ontologyText ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:shapesText ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
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
  * refine: [Semantic Contract Element Semantic Contract](../Functional/Core/ModelManagement.md#semantic-contract-element-semantic-contract)
---

### Reqvire Semantic Query Contract Shape Profile

Defines SHACL constraints for semantic-query-contract identity, query content, query language, and owning requirement.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:SemanticQueryContractShape
  a sh:NodeShape ;
  sh:targetClass reqvire:SemanticQueryContract ;
  sh:property [
    sh:path reqvire:queryContractIri ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:anyURI ;
  ] ;
  sh:property [
    sh:path reqvire:queryLanguage ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("sparql") ;
  ] ;
  sh:property [
    sh:path reqvire:queryText ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:ontologyText ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:shapesText ;
    sh:maxCount 0 ;
  ] ;
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
  * refine: [Semantic Query Contract Element Semantic Contract](../Functional/Core/ModelManagement.md#semantic-query-contract-element-semantic-contract)
---

### Reqvire Ontology Element Shape Profile

Defines SHACL constraints for ontology elements as vocabulary-bearing graph nodes.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:OntologyElementShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Ontology ;
  sh:property [
    sh:path reqvire:ontologyText ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:shapesText ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:attaches ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:derive ;
    sh:class reqvire:Ontology ;
  ] ;
  sh:property [
    sh:path reqvire:derivedFrom ;
    sh:class reqvire:Ontology ;
  ] ;
  sh:property [
    sh:path reqvire:refinedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:verifiedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:satisfiedBy ;
    sh:maxCount 0 ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Ontology Element Semantic Contract](../Functional/Core/ModelManagement.md#ontology-element-semantic-contract)
---

### Reqvire Refinement Subtype Shape Profile

Defines SHACL constraints for concrete refinement subtype ownership and traceability.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

reqvire:RefinementShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Refinement ;
  sh:property [
    sh:path reqvire:refine ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:or (
      [ sh:class reqvire:Capability ]
      [ sh:class reqvire:Requirement ]
    ) ;
  ] ;
  sh:property [
    sh:path reqvire:derive ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:derivedFrom ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:specify ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:specifiedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:verify ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:verifiedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:satisfy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:satisfiedBy ;
    sh:maxCount 0 ;
  ] .

reqvire:SourceShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Source ;
  sh:property [
    sh:path reqvire:refine ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:class reqvire:Capability ;
  ] .

reqvire:SpecificationShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Specification ;
  sh:property [
    sh:path reqvire:refine ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:or (
      [ sh:class reqvire:Capability ]
      [ sh:class reqvire:Requirement ]
    ) ;
  ] .

reqvire:ConstraintShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Constraint ;
  sh:property [
    sh:path reqvire:refine ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:or (
      [ sh:class reqvire:Capability ]
      [ sh:class reqvire:Requirement ]
    ) ;
  ] .

reqvire:BehaviorShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Behavior ;
  sh:property [
    sh:path reqvire:refine ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:or (
      [ sh:class reqvire:Capability ]
      [ sh:class reqvire:Requirement ]
    ) ;
  ] .

reqvire:StateShape
  a sh:NodeShape ;
  sh:targetClass reqvire:State ;
  sh:property [
    sh:path reqvire:refine ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:or (
      [ sh:class reqvire:Capability ]
      [ sh:class reqvire:Requirement ]
    ) ;
  ] .

reqvire:InputOutputShape
  a sh:NodeShape ;
  sh:targetClass reqvire:InputOutput ;
  sh:property [
    sh:path reqvire:refine ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:or (
      [ sh:class reqvire:Capability ]
      [ sh:class reqvire:Requirement ]
    ) ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Refinement Subtype Semantic Contract](../Functional/Core/ModelManagement.md#refinement-subtype-semantic-contract)
---

### Reqvire Custom Element Trace Shape Profile

Defines SHACL constraints for custom `other-*` elements as trace-only extension nodes.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:CustomElementShape
  a sh:NodeShape ;
  sh:targetClass reqvire:CustomElement ;
  sh:property [
    sh:path reqvire:elementType ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:pattern "^other-.+" ;
  ] ;
  sh:property [
    sh:path reqvire:derive ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:derivedFrom ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:specify ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:specifiedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:refine ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:refinedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:verify ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:verifiedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:satisfy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:satisfiedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:attaches ;
    sh:maxCount 0 ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Custom Element Trace Semantic Contract](../Functional/Core/ModelManagement.md#custom-element-trace-semantic-contract)
---
