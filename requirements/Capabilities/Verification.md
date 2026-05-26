# Elements

### Reqvire Verification Ontology Shape Profile

SHACL profile split from Reqvire Verification Ontology so ontology vocabulary remains first-class and semantic contracts carry closed-world constraints.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

reqvire:VerificationShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Verification ;
  sh:property [
    sh:path reqvire:verify ;
    sh:minCount 1 ;
    sh:class reqvire:Requirement ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Verification Upward Traceability](../Functional/Processing/VerificationTraces.md#verification-upward-traceability)
---

### Reqvire Verification Rollup Ontology Shape Profile

SHACL profile split from Reqvire Verification Rollup Ontology so ontology vocabulary remains first-class and semantic contracts carry closed-world constraints.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:CapabilityCoverageShape
  a sh:NodeShape ;
  sh:targetClass reqvire:CapabilityCoverage ;
  sh:property [
    sh:path reqvire:rollupCapability ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:class reqvire:Capability ;
  ] ;
  sh:property [
    sh:path reqvire:coverageState ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("verified" "partially-verified" "unverified" "blocked") ;
  ] ;
  sh:property [
    sh:path reqvire:blockedByRequirement ;
    sh:class reqvire:Requirement ;
  ] .

reqvire:RequirementCoverageShape
  a sh:NodeShape ;
  sh:targetClass reqvire:RequirementCoverage ;
  sh:property [
    sh:path reqvire:rollupRequirement ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:class reqvire:Requirement ;
  ] ;
  sh:property [
    sh:path reqvire:coveredByVerification ;
    sh:class reqvire:Verification ;
  ] ;
  sh:property [
    sh:path reqvire:coverageState ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("verified" "unverified" "blocked") ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Verification Upward Traceability](../Functional/Processing/VerificationTraces.md#verification-upward-traceability)
---

### Verification Traceability

As a **V&V Engineer**, I want Reqvire to trace verification evidence through capabilities, requirements, and capability roots, so that I can see which abilities and obligations are verified, which are blocked, and which capabilities still have coverage gaps.

#### Details
Verification traceability is the capability for verification elements, verification evidence, direct capability verification, and requirement verification rollup.

Verification elements verify capabilities or requirements. Capabilities may be directly verified; capability coverage status also rolls up from requirements that specify the capability.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: high
  * risk: high
  * status: approved

#### Attachments
  * [Reqvire Core Element Ontology](../Ontologies/Core.md#reqvire-core-element-ontology)
  * [Reqvire Relation Ontology](../Ontologies/RelationsAndImpact.md#reqvire-relation-ontology)
  * [Reqvire Verification Ontology](../Ontologies/Verification.md#reqvire-verification-ontology)
  * [Reqvire Verification Rollup Ontology](../Ontologies/Verification.md#reqvire-verification-rollup-ontology)

#### Relations
  * specifiedBy: [Verification Upward Traceability](../Functional/Processing/VerificationTraces.md#verification-upward-traceability)
---
