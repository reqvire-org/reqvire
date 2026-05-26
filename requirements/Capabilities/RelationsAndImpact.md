# Elements

### Reqvire Change Impact Ontology Shape Profile

SHACL profile split from Reqvire Change Impact Ontology so ontology vocabulary remains first-class and semantic contracts carry closed-world constraints.

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
  * refine: [Tracing Structural Changes](../Functional/Output/Reporting.md#tracing-structural-changes)
---

### Reqvire Relation Ontology Shape Profile

SHACL profile split from Reqvire Relation Ontology so ontology vocabulary remains first-class and semantic contracts carry closed-world constraints.

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
    sh:path reqvire:specifiedBy ;
    sh:class reqvire:Requirement ;
  ] .

reqvire:RequirementRelationShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Requirement ;
  sh:property [
    sh:path reqvire:derive ;
    sh:class reqvire:Requirement ;
  ] ;
  sh:property [
    sh:path reqvire:specify ;
    sh:class reqvire:Capability ;
  ] ;
  sh:property [
    sh:path reqvire:verifiedBy ;
    sh:class reqvire:Verification ;
  ] .

reqvire:VerificationRelationShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Verification ;
  sh:property [
    sh:path reqvire:verify ;
    sh:class reqvire:Requirement ;
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
  * refine: [Ontology and Semantic Contract Model](../Functional/Core/ModelManagement.md#ontology-and-semantic-contract-model)
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
  * [Reqvire Semantic Contract Ontology](../Ontologies/CapabilityRequirementModel.md#reqvire-semantic-contract-ontology)
  * [Reqvire Change Impact Ontology](../Ontologies/RelationsAndImpact.md#reqvire-change-impact-ontology)
  * [Reqvire Relation Ontology](../Ontologies/RelationsAndImpact.md#reqvire-relation-ontology)

#### Relations
  * specifiedBy: [Tracing Structural Changes](../Functional/Output/Reporting.md#tracing-structural-changes)
---
