# Elements

### Reqvire Verification Ontology

The Reqvire verification ontology defines verification objective and concrete verification element categories and their relationship to capabilities and requirements.

Verification objective elements organize verification intent and planning hierarchy. Concrete verification elements derive from a verification objective and verify capabilities or requirements. Evidence-backed verification types can also be satisfied by evidence artifacts such as test runs, proof reports, generated fixtures, or theorem/model-checking artifacts.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:ConcreteVerification a owl:Class ;
  rdfs:label "Concrete verification" ;
  rdfs:subClassOf reqvire:Verification ;
  owl:disjointWith reqvire:VerificationObjective ;
  rdfs:comment "Executable or reviewable verification element that derives from a verification objective and verifies a capability or requirement." .
reqvire:EvidenceBackedVerification a owl:Class ;
  rdfs:label "Evidence-backed verification" ;
  rdfs:subClassOf reqvire:ConcreteVerification,
    [ a owl:Restriction ;
      owl:onProperty reqvire:satisfiedBy ;
      owl:someValuesFrom reqvire:Artifact ] ;
  owl:disjointWith reqvire:NonEvidenceBackedVerification ;
  rdfs:comment "Verification class whose instances require satisfiedBy evidence for coverage satisfaction." .
reqvire:NonEvidenceBackedVerification a owl:Class ;
  rdfs:label "Non-evidence-backed verification" ;
  rdfs:subClassOf reqvire:ConcreteVerification ;
  rdfs:comment "Verification class whose instances do not require satisfiedBy evidence for coverage satisfaction." .
reqvire:TestVerification a owl:Class ;
  rdfs:subClassOf reqvire:EvidenceBackedVerification ;
  rdfs:label "Test verification" ;
  rdfs:comment "Verification through test procedures, automated tests, fixtures, or observed test evidence." .
reqvire:FormalProofVerification a owl:Class ;
  rdfs:subClassOf reqvire:EvidenceBackedVerification ;
  rdfs:label "Formal proof verification" ;
  rdfs:comment "Verification through formal proof, model checking, theorem proving, generated fixtures, or proof reports." .
reqvire:AnalysisVerification a owl:Class ;
  rdfs:subClassOf reqvire:NonEvidenceBackedVerification ;
  rdfs:label "Analysis verification" ;
  rdfs:comment "Verification through analysis of design, documentation, calculations, simulations, or code." .
reqvire:InspectionVerification a owl:Class ;
  rdfs:subClassOf reqvire:NonEvidenceBackedVerification ;
  rdfs:label "Inspection verification" ;
  rdfs:comment "Verification through review, audit, visual examination, or documentation inspection." .
reqvire:DemonstrationVerification a owl:Class ;
  rdfs:subClassOf reqvire:NonEvidenceBackedVerification ;
  rdfs:label "Demonstration verification" ;
  rdfs:comment "Verification through demonstration of capability in a representative workflow or environment." .
reqvire:VerificationObjective a owl:Class ;
  rdfs:label "Verification objective" ;
  rdfs:subClassOf reqvire:Verification ;
  rdfs:comment "Planning and grouping element for verification objectives. It may participate in verification hierarchy but does not directly verify requirements or carry evidence artifacts." .

reqvire:VerificationType a owl:Class ;
  rdfs:subClassOf reqvire:VerificationElementType ;
  rdfs:label "Verification type" ;
  rdfs:comment "Canonical verification metadata type and evidence semantics." .
reqvire:verificationTypeName a owl:DatatypeProperty ;
  rdfs:domain reqvire:VerificationType ;
  rdfs:range xsd:string ;
  rdfs:comment "Verification metadata type token used in Markdown." .
reqvire:verificationEvidenceBacked a owl:DatatypeProperty ;
  rdfs:domain reqvire:VerificationType ;
  rdfs:range xsd:boolean ;
  rdfs:comment "Whether verification satisfaction requires satisfiedBy evidence." .
reqvire:verificationTypeDescription a owl:DatatypeProperty ;
  rdfs:domain reqvire:VerificationType ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable semantic meaning of a verification element type." .

reqvire:verificationType a reqvire:VerificationType, reqvire:VerificationElementType ;
  rdfs:label "verification" ;
  rdfs:comment "Compatibility alias for test-verification." ;
  reqvire:elementTypeName "verification" ;
  reqvire:verificationTypeName "verification" ;
  reqvire:verificationEvidenceBacked true ;
  reqvire:verificationTypeDescription "Compatibility alias for test-verification." .
reqvire:testVerificationType a reqvire:VerificationType, reqvire:VerificationElementType ;
  rdfs:label "test-verification" ;
  rdfs:comment "Verification through test procedures, automated tests, fixtures, or observed test evidence." ;
  reqvire:elementTypeName "test-verification" ;
  reqvire:verificationTypeName "test-verification" ;
  reqvire:verificationEvidenceBacked true ;
  reqvire:verificationTypeDescription "Verification through test procedures, automated tests, fixtures, or observed test evidence." .
reqvire:formalProofVerificationType a reqvire:VerificationType, reqvire:VerificationElementType ;
  rdfs:label "formal-proof-verification" ;
  rdfs:comment "Verification through formal proof, model checking, theorem proving, generated fixtures, or proof reports." ;
  reqvire:elementTypeName "formal-proof-verification" ;
  reqvire:verificationTypeName "formal-proof-verification" ;
  reqvire:verificationEvidenceBacked true ;
  reqvire:verificationTypeDescription "Verification through formal proof, model checking, theorem proving, generated fixtures, or proof reports." .
reqvire:analysisVerificationType a reqvire:VerificationType, reqvire:VerificationElementType ;
  rdfs:label "analysis-verification" ;
  rdfs:comment "Verification through analysis of design, documentation, calculations, simulations, or code." ;
  reqvire:elementTypeName "analysis-verification" ;
  reqvire:verificationTypeName "analysis-verification" ;
  reqvire:verificationEvidenceBacked false ;
  reqvire:verificationTypeDescription "Verification through analysis of design, documentation, calculations, simulations, or code." .
reqvire:inspectionVerificationType a reqvire:VerificationType, reqvire:VerificationElementType ;
  rdfs:label "inspection-verification" ;
  rdfs:comment "Verification through review, audit, visual examination, or documentation inspection." ;
  reqvire:elementTypeName "inspection-verification" ;
  reqvire:verificationTypeName "inspection-verification" ;
  reqvire:verificationEvidenceBacked false ;
  reqvire:verificationTypeDescription "Verification through review, audit, visual examination, or documentation inspection." .
reqvire:demonstrationVerificationType a reqvire:VerificationType, reqvire:VerificationElementType ;
  rdfs:label "demonstration-verification" ;
  rdfs:comment "Verification through demonstration of capability in a representative workflow or environment." ;
  reqvire:elementTypeName "demonstration-verification" ;
  reqvire:verificationTypeName "demonstration-verification" ;
  reqvire:verificationEvidenceBacked false ;
  reqvire:verificationTypeDescription "Verification through demonstration of capability in a representative workflow or environment." .
reqvire:verificationObjectiveType a reqvire:VerificationElementType ;
  rdfs:label "verification-objective" ;
  rdfs:comment "Verification planning objective. This type organizes concrete verification work and is not a concrete verification method." ;
  reqvire:elementTypeName "verification-objective" ;
  reqvire:defaultElementType false .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Relation Ontology](RelationsAndImpact.md#reqvire-relation-ontology)
---

### Reqvire Verification Rollup Ontology

The Reqvire verification rollup ontology defines requirement verification state and capability coverage.

Rollup is calculated through capability and requirement graph structure. This ontology defines verification rollup rule categories and coverage semantics.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:VerificationRollup a owl:Class ;
  rdfs:label "Verification rollup" ;
  rdfs:comment "Computed verification coverage record for a capability or requirement." .
reqvire:CapabilityCoverage a owl:Class ;
  rdfs:subClassOf reqvire:VerificationRollup ;
  rdfs:label "Capability coverage" ;
  rdfs:comment "Computed verification coverage state for a capability." .
reqvire:RequirementCoverage a owl:Class ;
  rdfs:subClassOf reqvire:VerificationRollup ;
  rdfs:label "Requirement coverage" ;
  rdfs:comment "Computed verification coverage state for a requirement." .
reqvire:CoverageState a owl:Class ;
  rdfs:label "Coverage state" ;
  rdfs:comment "Controlled vocabulary for verification coverage state tokens." .
reqvire:VerificationRollupRule a owl:Class ;
  rdfs:label "Verification rollup rule" ;
  rdfs:comment "Controlled rule describing how requirement and capability verification coverage is computed." .

reqvire:rollupCapability a owl:ObjectProperty ;
  rdfs:domain reqvire:CapabilityCoverage ;
  rdfs:range reqvire:Capability ;
  rdfs:comment "Capability whose coverage state is described by a capability coverage record." .
reqvire:rollupRequirement a owl:ObjectProperty ;
  rdfs:domain reqvire:RequirementCoverage ;
  rdfs:range reqvire:Requirement ;
  rdfs:comment "Requirement whose coverage state is described by a requirement coverage record." .
reqvire:coveredByVerification a owl:ObjectProperty ;
  rdfs:domain reqvire:VerificationRollup ;
  rdfs:range reqvire:ConcreteVerification ;
  rdfs:comment "Verification element contributing evidence or method coverage to the rollup record." .
reqvire:blockedByRequirement a owl:ObjectProperty ;
  rdfs:domain reqvire:VerificationRollup ;
  rdfs:range reqvire:Requirement ;
  rdfs:comment "Requirement that blocks a capability or requirement coverage rollup." .
reqvire:coverageState a owl:DatatypeProperty ;
  rdfs:domain reqvire:VerificationRollup ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical verification coverage state token." .
reqvire:coverageReason a owl:DatatypeProperty ;
  rdfs:domain reqvire:VerificationRollup ;
  rdfs:range xsd:string ;
  rdfs:comment "Human-readable explanation for the computed coverage state." .
reqvire:rollupRuleName a owl:DatatypeProperty ;
  rdfs:domain reqvire:VerificationRollupRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical verification rollup rule token used by reports and semantic validation." .
reqvire:rollupCondition a owl:DatatypeProperty ;
  rdfs:domain reqvire:VerificationRollupRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable condition under which a verification rollup rule applies." .
reqvire:rollupOutcome a owl:DatatypeProperty ;
  rdfs:domain reqvire:VerificationRollupRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable coverage outcome produced by a verification rollup rule." .
reqvire:evidenceBacked a owl:DatatypeProperty ;
  rdfs:domain reqvire:VerificationRollupRule ;
  rdfs:range xsd:boolean ;
  rdfs:comment "Whether the rollup rule applies to evidence-backed verification types." .

reqvire:leafRequirementVerificationRule a reqvire:VerificationRollupRule ;
  rdfs:label "Leaf requirement verification" ;
  reqvire:rollupRuleName "leaf-requirement-verification" ;
  reqvire:rollupCondition "A requirement has no child requirements through derive." ;
  reqvire:rollupOutcome "The requirement is verified when it has at least one direct verifiedBy relation to a verification element." .

reqvire:parentRequirementRollupRule a reqvire:VerificationRollupRule ;
  rdfs:label "Parent requirement rollup" ;
  reqvire:rollupRuleName "parent-requirement-rollup" ;
  reqvire:rollupCondition "A requirement has one or more child requirements through derive." ;
  reqvire:rollupOutcome "The requirement is verified only when all child requirements are verified; direct parent verification does not override an unverified child." .

reqvire:capabilityCoverageRollupRule a reqvire:VerificationRollupRule ;
  rdfs:label "Capability coverage rollup" ;
  reqvire:rollupRuleName "capability-coverage-rollup" ;
  reqvire:rollupCondition "A capability has requirements through specifiedBy or descendant capabilities through derive." ;
  reqvire:rollupOutcome "The capability coverage state is derived from requirements that specify the capability, child requirements, and child capability coverage." .

reqvire:directCapabilityVerificationRule a reqvire:VerificationRollupRule ;
  rdfs:label "Direct capability verification" ;
  reqvire:rollupRuleName "direct-capability-verification" ;
  reqvire:rollupCondition "A capability has a direct verifiedBy relation to a verification element." ;
  reqvire:rollupOutcome "The capability may be directly verified; requirement-derived capability coverage remains a separate rollup state." .

reqvire:evidenceBackedVerificationRule a reqvire:VerificationRollupRule ;
  rdfs:label "Evidence-backed verification satisfaction" ;
  reqvire:rollupRuleName "evidence-backed-verification-satisfaction" ;
  reqvire:rollupCondition "A verification element is test-verification or formal-proof-verification." ;
  reqvire:rollupOutcome "The verification is satisfied only when it has at least one satisfiedBy evidence artifact." ;
  reqvire:evidenceBacked true .

reqvire:nonEvidenceBackedVerificationRule a reqvire:VerificationRollupRule ;
  rdfs:label "Non-evidence-backed verification satisfaction" ;
  reqvire:rollupRuleName "non-evidence-backed-verification-satisfaction" ;
  reqvire:rollupCondition "A verification element is analysis-verification, inspection-verification, or demonstration-verification." ;
  reqvire:rollupOutcome "The verification does not require satisfiedBy evidence for coverage satisfaction." ;
  reqvire:evidenceBacked false .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Verification Ontology](#reqvire-verification-ontology)
---

### Verification Coverage Rollup Shape

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
    sh:class reqvire:ConcreteVerification ;
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
  * constrain: [Verification Upward Traceability](../Verification/Traceability/VerificationTracesRequirements.md#verification-upward-traceability)
  * use: [Reqvire Verification Rollup Ontology](#reqvire-verification-rollup-ontology)
---

### Verification Target and Evidence Shape

Defines SHACL constraints for verification elements and the capabilities or requirements they verify.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:ConcreteVerificationShape
  a sh:NodeShape ;
  sh:targetClass reqvire:TestVerification, reqvire:FormalProofVerification, reqvire:AnalysisVerification, reqvire:InspectionVerification, reqvire:DemonstrationVerification ;
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
    sh:minCount 1 ;
    sh:class reqvire:VerificationObjective ;
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

reqvire:VerificationObjectiveShape
  a sh:NodeShape ;
  sh:targetClass reqvire:VerificationObjective ;
  sh:property [
    sh:path reqvire:verify ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:verifiedBy ;
    sh:maxCount 0 ;
  ] ;
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
  * constrain: [Verification Upward Traceability](../Verification/Traceability/VerificationTracesRequirements.md#verification-upward-traceability)
  * use: [Reqvire Verification Ontology](#reqvire-verification-ontology)
---
