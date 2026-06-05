# Elements

### Reqvire Verification Ontology Shape Profile

Defines SHACL constraints for verification elements and the capabilities or requirements they verify.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:VerificationShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Verification ;
  sh:property [
    sh:path reqvire:verify ;
    sh:minCount 1 ;
    sh:or (
      [ sh:class reqvire:Capability ]
      [ sh:class reqvire:Requirement ]
    ) ;
  ] ;
  sh:property [
    sh:path reqvire:verifiedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:derivedFrom ;
    sh:maxCount 0 ;
  ] .

reqvire:EvidenceBackedVerificationShape
  a sh:NodeShape ;
  sh:targetClass reqvire:TestVerification, reqvire:FormalProofVerification ;
  sh:property [
    sh:path reqvire:satisfiedBy ;
    sh:minCount 1 ;
    sh:class reqvire:Artifact ;
  ] ;
  sh:property [
    sh:path reqvire:satisfy ;
    sh:maxCount 0 ;
  ] .

reqvire:NonEvidenceBackedVerificationShape
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

reqvire:VerificationTypeShape
  a sh:NodeShape ;
  sh:targetClass reqvire:VerificationType ;
  sh:property [
    sh:path reqvire:verificationTypeName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("verification" "test-verification" "formal-proof-verification" "analysis-verification" "inspection-verification" "demonstration-verification") ;
  ] ;
  sh:property [
    sh:path reqvire:verificationEvidenceBacked ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:boolean ;
  ] ;
  sh:property [
    sh:path reqvire:verificationTypeDescription ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Verification Element Semantic Contract](../Functional/Processing/VerificationTraces.md#verification-element-semantic-contract)
---

### Reqvire Verification Rollup Ontology Shape Profile

Defines SHACL constraints for verification rollup and coverage state records.

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
    sh:path reqvire:coverageReason ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:coverageState ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("verified" "unverified" "blocked") ;
  ] .

reqvire:VerificationRollupRuleShape
  a sh:NodeShape ;
  sh:targetClass reqvire:VerificationRollupRule ;
  sh:property [
    sh:path reqvire:rollupRuleName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in (
      "leaf-requirement-verification"
      "parent-requirement-rollup"
      "capability-coverage-rollup"
      "direct-capability-verification"
      "evidence-backed-verification-satisfaction"
      "non-evidence-backed-verification-satisfaction"
    ) ;
  ] ;
  sh:property [
    sh:path reqvire:rollupCondition ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:rollupOutcome ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:evidenceBacked ;
    sh:maxCount 1 ;
    sh:datatype xsd:boolean ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Verification Rollup Semantic Contract](../Functional/Processing/VerificationTraces.md#verification-rollup-semantic-contract)
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
