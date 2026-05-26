# Elements

### Reqvire Verification Ontology

The Reqvire verification ontology defines verification element categories and their relationship to capabilities and requirements.

Verification elements verify capabilities or requirements. Evidence-backed verification types can also be satisfied by evidence artifacts such as test runs, proof reports, generated fixtures, or theorem/model-checking artifacts.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

reqvire:TestVerification a owl:Class ; rdfs:subClassOf reqvire:Verification .
reqvire:FormalProofVerification a owl:Class ; rdfs:subClassOf reqvire:Verification .
reqvire:AnalysisVerification a owl:Class ; rdfs:subClassOf reqvire:Verification .
reqvire:InspectionVerification a owl:Class ; rdfs:subClassOf reqvire:Verification .
reqvire:DemonstrationVerification a owl:Class ; rdfs:subClassOf reqvire:Verification .

reqvire:VerificationType a owl:Class ;
  rdfs:comment "Canonical verification metadata type and evidence semantics." .
reqvire:verificationTypeName a owl:DatatypeProperty ;
  rdfs:comment "Verification metadata type token used in Markdown." .
reqvire:verificationEvidenceBacked a owl:DatatypeProperty ;
  rdfs:comment "Whether verification satisfaction requires satisfiedBy evidence." .
reqvire:verificationTypeDescription a owl:DatatypeProperty ;
  rdfs:comment "Stable semantic meaning of a verification element type." .

reqvire:verificationType a reqvire:VerificationType, reqvire:VerificationElementType ;
  reqvire:elementTypeName "verification" ;
  reqvire:verificationTypeName "verification" ;
  reqvire:verificationEvidenceBacked true ;
  reqvire:verificationTypeDescription "Compatibility alias for test-verification." .
reqvire:testVerificationType a reqvire:VerificationType, reqvire:VerificationElementType ;
  reqvire:elementTypeName "test-verification" ;
  reqvire:verificationTypeName "test-verification" ;
  reqvire:verificationEvidenceBacked true ;
  reqvire:verificationTypeDescription "Verification through test procedures, automated tests, fixtures, or observed test evidence." .
reqvire:formalProofVerificationType a reqvire:VerificationType, reqvire:VerificationElementType ;
  reqvire:elementTypeName "formal-proof-verification" ;
  reqvire:verificationTypeName "formal-proof-verification" ;
  reqvire:verificationEvidenceBacked true ;
  reqvire:verificationTypeDescription "Verification through formal proof, model checking, theorem proving, generated fixtures, or proof reports." .
reqvire:analysisVerificationType a reqvire:VerificationType, reqvire:VerificationElementType ;
  reqvire:elementTypeName "analysis-verification" ;
  reqvire:verificationTypeName "analysis-verification" ;
  reqvire:verificationEvidenceBacked false ;
  reqvire:verificationTypeDescription "Verification through analysis of design, documentation, calculations, simulations, or code." .
reqvire:inspectionVerificationType a reqvire:VerificationType, reqvire:VerificationElementType ;
  reqvire:elementTypeName "inspection-verification" ;
  reqvire:verificationTypeName "inspection-verification" ;
  reqvire:verificationEvidenceBacked false ;
  reqvire:verificationTypeDescription "Verification through review, audit, visual examination, or documentation inspection." .
reqvire:demonstrationVerificationType a reqvire:VerificationType, reqvire:VerificationElementType ;
  reqvire:elementTypeName "demonstration-verification" ;
  reqvire:verificationTypeName "demonstration-verification" ;
  reqvire:verificationEvidenceBacked false ;
  reqvire:verificationTypeDescription "Verification through demonstration of capability in a representative workflow or environment." .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---

### Reqvire Verification Rollup Ontology

The Reqvire verification rollup ontology defines requirement verification state and capability coverage.

Rollup is calculated through capability and requirement graph structure. This ontology defines verification rollup rule categories and coverage semantics.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

reqvire:VerificationRollup a owl:Class .
reqvire:CapabilityCoverage a owl:Class .
reqvire:RequirementCoverage a owl:Class .
reqvire:CoverageState a owl:Class .
reqvire:VerificationRollupRule a owl:Class .

reqvire:rollupCapability a owl:ObjectProperty .
reqvire:rollupRequirement a owl:ObjectProperty .
reqvire:coveredByVerification a owl:ObjectProperty .
reqvire:blockedByRequirement a owl:ObjectProperty .
reqvire:coverageState a owl:DatatypeProperty .
reqvire:coverageReason a owl:DatatypeProperty .
reqvire:rollupRuleName a owl:DatatypeProperty .
reqvire:rollupCondition a owl:DatatypeProperty .
reqvire:rollupOutcome a owl:DatatypeProperty .
reqvire:evidenceBacked a owl:DatatypeProperty .

reqvire:leafRequirementVerificationRule a reqvire:VerificationRollupRule ;
  reqvire:rollupRuleName "leaf-requirement-verification" ;
  reqvire:rollupCondition "A requirement has no child requirements through derive." ;
  reqvire:rollupOutcome "The requirement is verified when it has at least one direct verifiedBy relation to a verification element." .

reqvire:parentRequirementRollupRule a reqvire:VerificationRollupRule ;
  reqvire:rollupRuleName "parent-requirement-rollup" ;
  reqvire:rollupCondition "A requirement has one or more child requirements through derive." ;
  reqvire:rollupOutcome "The requirement is verified only when all child requirements are verified; direct parent verification does not override an unverified child." .

reqvire:capabilityCoverageRollupRule a reqvire:VerificationRollupRule ;
  reqvire:rollupRuleName "capability-coverage-rollup" ;
  reqvire:rollupCondition "A capability has requirements through specifiedBy or descendant capabilities through derive." ;
  reqvire:rollupOutcome "The capability coverage state is derived from requirements that specify the capability, child requirements, and child capability coverage." .

reqvire:directCapabilityVerificationRule a reqvire:VerificationRollupRule ;
  reqvire:rollupRuleName "direct-capability-verification" ;
  reqvire:rollupCondition "A capability has a direct verifiedBy relation to a verification element." ;
  reqvire:rollupOutcome "The capability may be directly verified; requirement-derived capability coverage remains a separate rollup state." .

reqvire:evidenceBackedVerificationRule a reqvire:VerificationRollupRule ;
  reqvire:rollupRuleName "evidence-backed-verification-satisfaction" ;
  reqvire:rollupCondition "A verification element is test-verification or formal-proof-verification." ;
  reqvire:rollupOutcome "The verification is satisfied only when it has at least one satisfiedBy evidence artifact." ;
  reqvire:evidenceBacked true .

reqvire:nonEvidenceBackedVerificationRule a reqvire:VerificationRollupRule ;
  reqvire:rollupRuleName "non-evidence-backed-verification-satisfaction" ;
  reqvire:rollupCondition "A verification element is analysis-verification, inspection-verification, or demonstration-verification." ;
  reqvire:rollupOutcome "The verification does not require satisfiedBy evidence for coverage satisfaction." ;
  reqvire:evidenceBacked false .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---
