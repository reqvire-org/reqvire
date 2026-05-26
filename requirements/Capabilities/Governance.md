# Elements

### Reqvire Governance Ontology Shape Profile

Defines SHACL constraints for governance metadata on capability and requirement elements.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:CapabilityGovernanceShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Capability ;
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

reqvire:RequirementGovernanceShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Requirement ;
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

reqvire:NonGovernanceElementShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Refinement, reqvire:Verification, reqvire:Ontology, reqvire:CustomElement ;
  sh:property [
    sh:path reqvire:status ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:priority ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:risk ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:owner ;
    sh:maxCount 0 ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Governance Metadata Semantic Contract](../Functional/Core/ModelManagement.md#governance-metadata-semantic-contract)
---
