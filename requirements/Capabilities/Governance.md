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
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("draft" "review" "approved") ;
    sh:message "Capability status metadata must be one of draft, review, or approved." ;
  ] ;
  sh:property [
    sh:path reqvire:priority ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("low" "medium" "high" "critical") ;
    sh:message "Capability priority metadata must be one of low, medium, high, or critical." ;
  ] ;
  sh:property [
    sh:path reqvire:risk ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("low" "medium" "high" "critical") ;
    sh:message "Capability risk metadata must be one of low, medium, high, or critical." ;
  ] ;
  sh:property [
    sh:path reqvire:owner ;
    sh:datatype xsd:string ;
    sh:maxCount 1 ;
    sh:message "Capability owner metadata must be a single string routing label." ;
  ] .

reqvire:RequirementGovernanceShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Requirement ;
  sh:property [
    sh:path reqvire:status ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("draft" "review" "approved") ;
    sh:message "Requirement status metadata must be one of draft, review, or approved." ;
  ] ;
  sh:property [
    sh:path reqvire:priority ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("low" "medium" "high" "critical") ;
    sh:message "Requirement priority metadata must be one of low, medium, high, or critical." ;
  ] ;
  sh:property [
    sh:path reqvire:risk ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("low" "medium" "high" "critical") ;
    sh:message "Requirement risk metadata must be one of low, medium, high, or critical." ;
  ] ;
  sh:property [
    sh:path reqvire:owner ;
    sh:datatype xsd:string ;
    sh:maxCount 1 ;
    sh:message "Requirement owner metadata must be a single string routing label." ;
  ] .

reqvire:NonGovernanceElementShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Refinement, reqvire:Verification, reqvire:Ontology, reqvire:CustomElement ;
  sh:property [
    sh:path reqvire:status ;
    sh:maxCount 0 ;
    sh:message "Governance status metadata may be authored only by capability or requirement elements." ;
  ] ;
  sh:property [
    sh:path reqvire:priority ;
    sh:maxCount 0 ;
    sh:message "Governance priority metadata may be authored only by capability or requirement elements." ;
  ] ;
  sh:property [
    sh:path reqvire:risk ;
    sh:maxCount 0 ;
    sh:message "Governance risk metadata may be authored only by capability or requirement elements." ;
  ] ;
  sh:property [
    sh:path reqvire:owner ;
    sh:maxCount 0 ;
    sh:message "Governance owner metadata may be authored only by capability or requirement elements." ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Governance Metadata Semantic Contract](../Functional/Core/ModelManagement.md#governance-metadata-semantic-contract)
---
