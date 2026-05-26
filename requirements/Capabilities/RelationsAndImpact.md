# Elements

### Reqvire Change Impact Ontology Shape Profile

Defines SHACL constraints for change-impact analysis records, impact edges, and semantic dependency resolution.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:ChangeImpactAnalysisShape
  a sh:NodeShape ;
  sh:targetClass reqvire:ChangeImpactAnalysis ;
  sh:property [
    sh:path reqvire:changedElement ;
    sh:minCount 1 ;
    sh:class reqvire:Element ;
  ] ;
  sh:property [
    sh:path reqvire:impactedElement ;
    sh:class reqvire:Element ;
  ] ;
  sh:property [
    sh:path reqvire:impactPath ;
    sh:class reqvire:ChangeImpactPath ;
  ] ;
  sh:property [
    sh:path reqvire:requiresReview ;
    sh:maxCount 1 ;
    sh:datatype xsd:boolean ;
  ] .

reqvire:ChangeImpactEdgeShape
  a sh:NodeShape ;
  sh:targetClass reqvire:ChangeImpactEdge ;
  sh:property [
    sh:path reqvire:impactRelation ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:impactDirection ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("upstream" "downstream" "bidirectional") ;
  ] ;
  sh:property [
    sh:path reqvire:impactReason ;
    sh:datatype xsd:string ;
  ] .

reqvire:SemanticDependencyShape
  a sh:NodeShape ;
  sh:targetClass reqvire:SemanticDependency ;
  sh:property [
    sh:path reqvire:semanticDependency ;
    sh:class reqvire:SemanticContract ;
  ] ;
  sh:property [
    sh:path reqvire:dependencyResolution ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("native" "attached" "not-found" "found-outside-context") ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Change Impact Semantic Contract](../Functional/Output/Reporting.md#change-impact-semantic-contract)
---

### Reqvire Relation Ontology Shape Profile

Defines SHACL constraints for Reqvire relation usage across capabilities, requirements, verifications, and relation-rule metadata.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:CapabilityRelationShape
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
  ] .

reqvire:RequirementRelationShape
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
    sh:path reqvire:derive ;
    sh:class reqvire:Requirement ;
  ] ;
  sh:property [
    sh:path reqvire:derivedFrom ;
    sh:class reqvire:Requirement ;
  ] ;
  sh:property [
    sh:path reqvire:specify ;
    sh:class reqvire:Capability ;
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
    sh:path reqvire:satisfy ;
    sh:maxCount 0 ;
  ] .

reqvire:VerificationRelationShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Verification ;
  sh:property [
    sh:path reqvire:verify ;
    sh:or (
      [ sh:class reqvire:Capability ]
      [ sh:class reqvire:Requirement ]
    ) ;
  ] ;
  sh:property [
    sh:path reqvire:derivedFrom ;
    sh:maxCount 0 ;
  ] .

reqvire:EvidenceBackedVerificationRelationShape
  a sh:NodeShape ;
  sh:targetClass reqvire:TestVerification, reqvire:FormalProofVerification ;
  sh:property [
    sh:path reqvire:satisfiedBy ;
    sh:class reqvire:Artifact ;
  ] .

reqvire:NonEvidenceBackedVerificationRelationShape
  a sh:NodeShape ;
  sh:targetClass reqvire:AnalysisVerification, reqvire:InspectionVerification, reqvire:DemonstrationVerification ;
  sh:property [
    sh:path reqvire:satisfiedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:satisfy ;
    sh:maxCount 0 ;
  ] .

reqvire:CapabilityAttachmentShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Capability ;
  sh:property [
    sh:path reqvire:attaches ;
    sh:class reqvire:Ontology ;
  ] .

reqvire:RequirementAttachmentShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Requirement ;
  sh:property [
    sh:path reqvire:attaches ;
    sh:or (
      [ sh:class reqvire:SemanticContract ]
      [ sh:class reqvire:Constraint ]
      [ sh:class reqvire:Behavior ]
      [ sh:class reqvire:Specification ]
      [ sh:class reqvire:State ]
      [ sh:class reqvire:InputOutput ]
    ) ;
  ] .

reqvire:NonAttachmentElementShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Refinement, reqvire:Verification, reqvire:Ontology, reqvire:CustomElement ;
  sh:property [
    sh:path reqvire:attaches ;
    sh:maxCount 0 ;
  ] .

reqvire:RelationRuleShape
  a sh:NodeShape ;
  sh:targetClass reqvire:RelationRule ;
  sh:property [
    sh:path reqvire:relationName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:allowedSourceType ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:allowedTargetType ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:relationDirection ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("forward" "inverse" "non-directional") ;
  ] ;
  sh:property [
    sh:path reqvire:propagatesChangeImpact ;
    sh:maxCount 1 ;
    sh:datatype xsd:boolean ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Relation Model Semantic Contract](../Functional/Core/ModelManagement.md#relation-model-semantic-contract)
---

### Trace Changes in System Model

As a **System Engineer**, I want Reqvire to trace model changes through requirements, refinements, semantic contracts, attachments, verifications, and implementation evidence, so that I can identify what must be reviewed after a change.

#### Details
Trace changes in system model is the capability for impact propagation, auditable dependency paths, and review routing after model changes.

Change impact uses native Reqvire relations and explicit attachments. Semantic references are modeled through capability-root hierarchy or attachment context so change impact is not hidden in an unrelated ontology reference.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: high
  * risk: high
  * status: approved

#### Attachments
  * [Reqvire Core Element Ontology](../Ontologies/Core.md#reqvire-core-element-ontology)
  * [Reqvire Requirement Ontology](../Ontologies/CapabilityRequirementModel.md#reqvire-requirement-ontology)
  * [Reqvire Semantic Contract Ontology](../Ontologies/CapabilityRequirementModel.md#reqvire-semantic-contract-ontology)
  * [Reqvire Change Impact Ontology](../Ontologies/RelationsAndImpact.md#reqvire-change-impact-ontology)
  * [Reqvire Relation Ontology](../Ontologies/RelationsAndImpact.md#reqvire-relation-ontology)
  * [Reqvire Verification Ontology](../Ontologies/Verification.md#reqvire-verification-ontology)

#### Relations
  * specifiedBy: [Tracing Structural Changes](../Functional/Output/Reporting.md#tracing-structural-changes)
---
