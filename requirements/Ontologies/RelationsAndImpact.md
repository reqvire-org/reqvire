# Elements

### Change Impact Analysis Shape

Defines SHACL constraints for change-impact analysis records, impact edges, and semantic dependency resolution.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
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

reqvire:ChangeImpactPathShape
  a sh:NodeShape ;
  sh:targetClass reqvire:ChangeImpactPath ;
  sh:property [
    sh:path reqvire:impactEdge ;
    sh:class reqvire:ChangeImpactEdge ;
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
    sh:in ("native" "reused" "not-found" "found-outside-context") ;
  ] .

reqvire:ImpactReviewShape
  a sh:NodeShape ;
  sh:targetClass reqvire:ImpactReview ;
  sh:property [
    sh:path reqvire:reviewedByVerification ;
    sh:class reqvire:Verification ;
  ] ;
  sh:property [
    sh:path reqvire:impactScope ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:impactSeverity ;
    sh:datatype xsd:string ;
  ] .

reqvire:ChangePropagationRuleShape
  a sh:NodeShape ;
  sh:targetClass reqvire:ChangePropagationRule ;
  sh:property [
    sh:path reqvire:changeRuleName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in (
      "parent-to-child-impact"
      "capability-to-specified-requirement-impact"
      "requirement-to-implementation-impact"
      "requirement-to-verification-impact"
      "owner-to-contract-impact"
      "ontology-to-semantic-contract-impact"
      "semantic-contract-to-requirement-impact"
      "semantic-contract-ontology-use-dependency"
      "requirement-to-semantic-contract-review"
      "reused-contract-context-content-impact"
      "semantic-reference-reachability"
      "relocation-without-content-change"
    ) ;
  ] ;
  sh:property [
    sh:path reqvire:changedThing ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:propagationTarget ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:propagationMode ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:impactRelation ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:impactDirection ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("upstream" "downstream" "bidirectional" "none") ;
  ] ;
  sh:property [
    sh:path reqvire:impactReason ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] .

reqvire:ChangeKindShape
  a sh:NodeShape ;
  sh:targetClass reqvire:ChangeKind ;
  sh:property [
    sh:path reqvire:changeKindName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("content-change" "addition" "removal" "relocation" "reused-contract-context-content-change") ;
  ] ;
  sh:property [
    sh:path reqvire:changeKindMeaning ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] .

reqvire:ImpactClassificationShape
  a sh:NodeShape ;
  sh:targetClass reqvire:ImpactClassification ;
  sh:property [
    sh:path reqvire:impactClassificationName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("direct" "indirect" "potential") ;
  ] ;
  sh:property [
    sh:path reqvire:impactClassificationMeaning ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Tracing Structural Changes](../Reports/ModelReports/ReportingRequirements.md#tracing-structural-changes)
  * use: [Reqvire Change Impact Ontology](#reqvire-change-impact-ontology)
---

### Relation Compatibility Shape

Defines SHACL constraints for Reqvire relation usage across capabilities, requirements, verifications, and relation-rule metadata.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
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
    sh:minCount 1 ;
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

reqvire:CapabilityReusedContractContextRejectionShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Capability ;
  sh:property [
    sh:path reqvire:reusesContract ;
    sh:maxCount 0 ;
  ] .

reqvire:RequirementReusedContractContextShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Requirement ;
  sh:property [
    sh:path reqvire:reusesContract ;
    sh:or (
      [ sh:class reqvire:Constraint ]
      [ sh:class reqvire:Behavior ]
      [ sh:class reqvire:Specification ]
      [ sh:class reqvire:State ]
      [ sh:class reqvire:InputOutput ]
    ) ;
  ] .

reqvire:NonReusedContractContextElementShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Contract, reqvire:Verification, reqvire:Ontology, reqvire:CustomElement ;
  sh:property [
    sh:path reqvire:reusesContract ;
    sh:maxCount 0 ;
  ] .

reqvire:RelationRuleShape
  a sh:NodeShape ;
  sh:targetClass reqvire:RelationRule ;
  sh:closed true ;
  sh:ignoredProperties ( rdf:type rdfs:label rdfs:comment ) ;
  sh:property [
    sh:path reqvire:relationName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("derive" "derivedFrom" "specifiedBy" "specify" "definedBy" "define" "constrainedBy" "constrain" "use" "usedBy" "verifiedBy" "verify" "satisfiedBy" "satisfy" "reused_contract_context") ;
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
    sh:path reqvire:inverseRelation ;
    sh:maxCount 1 ;
    sh:nodeKind sh:IRI ;
  ] ;
  sh:property [
    sh:path reqvire:relationPattern ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:class reqvire:RelationSemanticPattern ;
  ] ;
  sh:property [
    sh:path reqvire:relationFamily ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:class reqvire:RelationFamily ;
  ] ;
  sh:property [
    sh:path reqvire:normalizedForwardProperty ;
    sh:maxCount 1 ;
    sh:nodeKind sh:IRI ;
  ] ;
  sh:property [
    sh:path reqvire:normalizedInverseProperty ;
    sh:maxCount 1 ;
    sh:nodeKind sh:IRI ;
  ] ;
  sh:property [
    sh:path reqvire:transitiveClosureForwardProperty ;
    sh:maxCount 1 ;
    sh:nodeKind sh:IRI ;
  ] ;
  sh:property [
    sh:path reqvire:transitiveClosureInverseProperty ;
    sh:maxCount 1 ;
    sh:nodeKind sh:IRI ;
  ] ;
  sh:property [
    sh:path reqvire:relationConstraint ;
    sh:class reqvire:RelationConstraint ;
  ] ;
  sh:property [
    sh:path reqvire:propagatesChangeImpact ;
    sh:maxCount 1 ;
    sh:datatype xsd:boolean ;
  ] ;
  sh:property [
    sh:path reqvire:createsOwnership ;
    sh:maxCount 1 ;
    sh:datatype xsd:boolean ;
  ] ;
  sh:property [
    sh:path reqvire:relationRuleDescription ;
    sh:datatype xsd:string ;
  ] .

reqvire:TraversalRuleShape
  a sh:NodeShape ;
  sh:targetClass reqvire:TraversalRule ;
  sh:closed true ;
  sh:ignoredProperties ( rdf:type rdfs:label rdfs:comment ) ;
  sh:property [
    sh:path reqvire:traversalDirection ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("forward" "reverse" "bidirectional") ;
  ] .

reqvire:ReusedContractContextCompatibilityRuleShape
  a sh:NodeShape ;
  sh:targetClass reqvire:ReusedContractContextCompatibilityRule ;
  sh:closed true ;
  sh:ignoredProperties ( rdf:type rdfs:label rdfs:comment ) ;
  sh:property [
    sh:path reqvire:reusedContractContextSourceType ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:reusedContractContextTargetType ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:reusedContractContextOwnerType ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:reusedContractContextRuleDescription ;
    sh:datatype xsd:string ;
  ] .

reqvire:RelationUsageCategoryShape
  a sh:NodeShape ;
  sh:targetClass reqvire:RelationUsageCategory ;
  sh:closed true ;
  sh:ignoredProperties ( rdf:type rdfs:label rdfs:comment ) ;
  sh:property [
    sh:path reqvire:usageCategoryName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("diagram-rendering-forward" "reverse-traversal" "change-propagation" "verification-rollup") ;
  ] ;
  sh:property [
    sh:path reqvire:usageCategoryRelationName ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("derive" "derivedFrom" "specifiedBy" "specify" "definedBy" "define" "constrainedBy" "constrain" "use" "usedBy" "verifiedBy" "verify" "satisfiedBy" "satisfy" "reused_contract_context") ;
  ] ;
  sh:property [
    sh:path reqvire:usageCategoryMeaning ;
    sh:datatype xsd:string ;
  ] .

reqvire:RelationSemanticCategoryShape
  a sh:NodeShape ;
  sh:targetClass reqvire:RelationSemanticCategory ;
  sh:closed true ;
  sh:ignoredProperties ( rdf:type rdfs:label rdfs:comment ) ;
  sh:property [
    sh:path reqvire:semanticCategoryName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("hierarchy" "capability-specification" "satisfaction" "contract-ownership" "semantic-contract-constraint" "semantic-contract-ontology-use" "verification" "cross-subgraph-contract-dependency") ;
  ] ;
  sh:property [
    sh:path reqvire:semanticCategoryRelationName ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("derive" "derivedFrom" "specifiedBy" "specify" "definedBy" "define" "constrainedBy" "constrain" "use" "usedBy" "verifiedBy" "verify" "satisfiedBy" "satisfy" "reused_contract_context") ;
  ] ;
  sh:property [
    sh:path reqvire:semanticCategoryMeaning ;
    sh:datatype xsd:string ;
  ] .

reqvire:RelationSemanticPatternShape
  a sh:NodeShape ;
  sh:targetClass reqvire:RelationSemanticPattern ;
  sh:closed true ;
  sh:ignoredProperties ( rdf:type rdfs:label rdfs:comment ) ;
  sh:property [
    sh:path reqvire:semanticPatternName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("hierarchy" "bridge" "ownership" "constraint" "dependency" "verification" "satisfaction" "cross-subgraph-contract-dependency") ;
  ] .

reqvire:RelationFamilyShape
  a sh:NodeShape ;
  sh:targetClass reqvire:RelationFamily ;
  sh:closed true ;
  sh:ignoredProperties ( rdf:type rdfs:label rdfs:comment ) ;
  sh:property [
    sh:path reqvire:relationFamilyName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("hierarchy" "capability-specification" "contract-ownership" "semantic-contract-constraint" "semantic-contract-ontology-use" "verification" "satisfaction" "cross-subgraph-contract-dependency") ;
  ] ;
  sh:property [
    sh:path reqvire:relationFamilyMeaning ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:relationFamilyForwardProperty ;
    sh:maxCount 1 ;
    sh:nodeKind sh:IRI ;
  ] ;
  sh:property [
    sh:path reqvire:relationFamilyInverseProperty ;
    sh:maxCount 1 ;
    sh:nodeKind sh:IRI ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Ontology and Semantic Contract Model](../ModelStructure/ModelManagement.md#ontology-and-semantic-contract-model)
  * use: [Reqvire Requirement Ontology](CapabilityRequirementModel.md#reqvire-requirement-ontology)
  * use: [Reqvire Semantic Contract Ontology](CapabilityRequirementModel.md#reqvire-semantic-contract-ontology)
  * use: [Reqvire Relation Ontology](#reqvire-relation-ontology)
  * use: [Reqvire Verification Ontology](Verification.md#reqvire-verification-ontology)
---

### Reqvire Change Impact Ontology

The Reqvire change impact ontology defines impact analysis concepts and propagation rules.

Change impact is based on auditable graph paths. Native relations, concept references, semantic-contract use/constrain edges, and explicit requirement-owned contract reused_contract_context define reachable context. This ontology defines propagation rule categories and impact semantics.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:ChangeImpactAnalysis a owl:Class ;
  rdfs:label "Change impact analysis" ;
  rdfs:comment "Analysis record for a changed element, impacted elements, propagation paths, and review status." .
reqvire:ChangeImpactPath a owl:Class ;
  rdfs:label "Change impact path" ;
  rdfs:comment "Auditable path that explains how impact propagates through model relations, reused_contract_context, or semantic dependencies." .
reqvire:ChangeImpactEdge a owl:Class ;
  rdfs:label "Change impact edge" ;
  rdfs:comment "Single propagation step in a change-impact path." .
reqvire:SemanticDependency a owl:Class ;
  rdfs:label "Semantic dependency" ;
  rdfs:comment "Resolved dependency from a semantic contract to ontology terms reachable through explicit ontology-use context." .
reqvire:ImpactReview a owl:Class ;
  rdfs:label "Impact review" ;
  rdfs:comment "Review record for impact scope, severity, and verification evidence used to assess a change." .
reqvire:ChangePropagationRule a owl:Class ;
  rdfs:label "Change propagation rule" ;
  rdfs:comment "Controlled rule describing how a model change propagates through a relation or dependency category." .
reqvire:ChangeKind a owl:Class ;
  rdfs:label "Change kind" ;
  rdfs:comment "Controlled vocabulary for model-diff change kind tokens." .
reqvire:ImpactClassification a owl:Class ;
  rdfs:label "Impact classification" ;
  rdfs:comment "Controlled vocabulary for change-impact classification tokens." .

reqvire:changedElement a owl:ObjectProperty ;
  rdfs:domain reqvire:ChangeImpactAnalysis ;
  rdfs:range reqvire:Element ;
  rdfs:comment "Element whose identity, content, location, or reused content changed." .
reqvire:impactedElement a owl:ObjectProperty ;
  rdfs:domain reqvire:ChangeImpactAnalysis ;
  rdfs:range reqvire:Element ;
  rdfs:comment "Element that requires review because of a direct or propagated change impact." .
reqvire:impactPath a owl:ObjectProperty ;
  rdfs:domain reqvire:ChangeImpactAnalysis ;
  rdfs:range reqvire:ChangeImpactPath ;
  rdfs:comment "Auditable path connecting a changed element to an impacted element." .
reqvire:impactEdge a owl:ObjectProperty ;
  rdfs:domain reqvire:ChangeImpactPath ;
  rdfs:range reqvire:ChangeImpactEdge ;
  rdfs:comment "Propagation edge included in a change-impact path." .
reqvire:semanticDependency a owl:ObjectProperty ;
  rdfs:domain [ a owl:Class ; owl:unionOf (reqvire:ChangeImpactAnalysis reqvire:SemanticDependency) ] ;
  rdfs:range [ a owl:Class ; owl:unionOf (reqvire:SemanticDependency reqvire:SemanticContract) ] ;
  rdfs:comment "Semantic dependency considered during change-impact analysis, or the semantic contract target resolved by a dependency record." .
reqvire:reviewedByVerification a owl:ObjectProperty ;
  rdfs:domain reqvire:ImpactReview ;
  rdfs:range reqvire:Verification ;
  rdfs:comment "Verification evidence or method used to review an impact." .

reqvire:impactReason a owl:DatatypeProperty ;
  rdfs:domain [ a owl:Class ; owl:unionOf (reqvire:ChangeImpactEdge reqvire:ChangePropagationRule) ] ;
  rdfs:range xsd:string ;
  rdfs:comment "Human-readable reason explaining why an edge or rule propagates impact." .
reqvire:impactRelation a owl:DatatypeProperty ;
  rdfs:domain [ a owl:Class ; owl:unionOf (reqvire:ChangeImpactEdge reqvire:ChangePropagationRule) ] ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical relation or dependency token that caused an impact edge or activates a propagation rule." .
reqvire:impactDirection a owl:DatatypeProperty ;
  rdfs:domain [ a owl:Class ; owl:unionOf (reqvire:ChangeImpactEdge reqvire:ChangePropagationRule) ] ;
  rdfs:range xsd:string ;
  rdfs:comment "Propagation direction token for an impact edge or propagation rule." .
reqvire:impactScope a owl:DatatypeProperty ;
  rdfs:domain reqvire:ImpactReview ;
  rdfs:range xsd:string ;
  rdfs:comment "Review scope or boundary considered for an impact review." .
reqvire:impactSeverity a owl:DatatypeProperty ;
  rdfs:domain reqvire:ImpactReview ;
  rdfs:range xsd:string ;
  rdfs:comment "Severity assessment recorded for an impact review." .
reqvire:requiresReview a owl:DatatypeProperty ;
  rdfs:domain reqvire:ChangeImpactAnalysis ;
  rdfs:range xsd:boolean ;
  rdfs:comment "Whether the change-impact analysis result requires human or verification review." .
reqvire:dependencyResolution a owl:DatatypeProperty ;
  rdfs:domain reqvire:SemanticDependency ;
  rdfs:range xsd:string ;
  rdfs:comment "Resolution token describing whether a semantic dependency was reachable, reused, missing, or outside context." .
reqvire:changeRuleName a owl:DatatypeProperty ;
  rdfs:domain reqvire:ChangePropagationRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical change-propagation rule token used by reports and semantic validation." .
reqvire:changedThing a owl:DatatypeProperty ;
  rdfs:domain reqvire:ChangePropagationRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Model thing category whose change activates a propagation rule." .
reqvire:propagationTarget a owl:DatatypeProperty ;
  rdfs:domain reqvire:ChangePropagationRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Model thing category that receives propagated impact." .
reqvire:propagationMode a owl:DatatypeProperty ;
  rdfs:domain reqvire:ChangePropagationRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Review, validation, or reporting mode for a propagation rule." .
reqvire:changeKindName a owl:DatatypeProperty ;
  rdfs:domain reqvire:ChangeKind ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical model-diff change kind token." .
reqvire:changeKindMeaning a owl:DatatypeProperty ;
  rdfs:domain reqvire:ChangeKind ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable semantic meaning of a model-diff change kind." .
reqvire:impactClassificationName a owl:DatatypeProperty ;
  rdfs:domain reqvire:ImpactClassification ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical impact classification token." .
reqvire:impactClassificationMeaning a owl:DatatypeProperty ;
  rdfs:domain reqvire:ImpactClassification ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable semantic meaning of an impact classification." .

reqvire:contentChangeKind a reqvire:ChangeKind ;
  rdfs:label "Content change" ;
  rdfs:comment "Same stable element identity exists across compared model states but content hash changed." ;
  reqvire:changeKindName "content-change" ;
  reqvire:changeKindMeaning "Same stable element identity exists across compared model states but content hash changed." .
reqvire:additionChangeKind a reqvire:ChangeKind ;
  rdfs:label "Addition" ;
  rdfs:comment "Stable element identity exists only in the newer model state." ;
  reqvire:changeKindName "addition" ;
  reqvire:changeKindMeaning "Stable element identity exists only in the newer model state." .
reqvire:removalChangeKind a reqvire:ChangeKind ;
  rdfs:label "Removal" ;
  rdfs:comment "Stable element identity exists only in the older model state." ;
  reqvire:changeKindName "removal" ;
  reqvire:changeKindMeaning "Stable element identity exists only in the older model state." .
reqvire:relocationChangeKind a reqvire:ChangeKind ;
  rdfs:label "Relocation" ;
  rdfs:comment "Stable element identity exists in both model states but location context changed." ;
  reqvire:changeKindName "relocation" ;
  reqvire:changeKindMeaning "Stable element identity exists in both model states but location context changed." .
reqvire:reusedContractContextContentChangeKind a reqvire:ChangeKind ;
  rdfs:label "Reused Contract Context content change" ;
  rdfs:comment "Reused resource or reused element content changed independently of the reusesContract element text." ;
  reqvire:changeKindName "reused-contract-context-content-change" ;
  reqvire:changeKindMeaning "Reused resource or reused element content changed independently of the reusesContract element text." .

reqvire:directImpactClassification a reqvire:ImpactClassification ;
  rdfs:label "Direct impact" ;
  rdfs:comment "Impact caused by a change to the element itself." ;
  reqvire:impactClassificationName "direct" ;
  reqvire:impactClassificationMeaning "Impact caused by a change to the element itself." .
reqvire:indirectImpactClassification a reqvire:ImpactClassification ;
  rdfs:label "Indirect impact" ;
  rdfs:comment "Impact propagated through relation, reused_contract_context, hierarchy, or semantic dependency context." ;
  reqvire:impactClassificationName "indirect" ;
  reqvire:impactClassificationMeaning "Impact propagated through relation, reused_contract_context, hierarchy, or semantic dependency context." .
reqvire:potentialImpactClassification a reqvire:ImpactClassification ;
  rdfs:label "Potential impact" ;
  rdfs:comment "Impact that may require review based on semantic analysis or indirect dependency evidence." ;
  reqvire:impactClassificationName "potential" ;
  reqvire:impactClassificationMeaning "Impact that may require review based on semantic analysis or indirect dependency evidence." .

reqvire:parentToChildImpactRule a reqvire:ChangePropagationRule ;
  rdfs:label "Parent to child impact" ;
  reqvire:changeRuleName "parent-to-child-impact" ;
  reqvire:changedThing "capability-requirement-or-ontology-parent" ;
  reqvire:impactRelation "derive" ;
  reqvire:impactDirection "downstream" ;
  reqvire:propagationTarget "child-capability-requirement-or-ontology" ;
  reqvire:propagationMode "review-required" ;
  reqvire:impactReason "Parent changes can change the meaning or scope of derived child elements." .

reqvire:capabilityToRequirementImpactRule a reqvire:ChangePropagationRule ;
  rdfs:label "Capability to specified requirement impact" ;
  reqvire:changeRuleName "capability-to-specified-requirement-impact" ;
  reqvire:changedThing "capability" ;
  reqvire:impactRelation "specifiedBy" ;
  reqvire:impactDirection "downstream" ;
  reqvire:propagationTarget "requirement" ;
  reqvire:propagationMode "review-required" ;
  reqvire:impactReason "Capability scope and authored concept-reference context changes can affect requirements that specify the capability." .

reqvire:requirementToImplementationImpactRule a reqvire:ChangePropagationRule ;
  rdfs:label "Requirement to implementation impact" ;
  reqvire:changeRuleName "requirement-to-implementation-impact" ;
  reqvire:changedThing "requirement" ;
  reqvire:impactRelation "satisfiedBy" ;
  reqvire:impactDirection "downstream" ;
  reqvire:propagationTarget "implementation-or-evidence-artifact" ;
  reqvire:propagationMode "review-required" ;
  reqvire:impactReason "Requirement changes may invalidate implementation satisfaction evidence." .

reqvire:requirementToVerificationImpactRule a reqvire:ChangePropagationRule ;
  rdfs:label "Requirement to verification impact" ;
  reqvire:changeRuleName "requirement-to-verification-impact" ;
  reqvire:changedThing "requirement" ;
  reqvire:impactRelation "verifiedBy" ;
  reqvire:impactDirection "downstream" ;
  reqvire:propagationTarget "verification" ;
  reqvire:propagationMode "revalidation-required" ;
  reqvire:impactReason "Requirement changes may invalidate verification scope or evidence." .

reqvire:ownerToContractImpactRule a reqvire:ChangePropagationRule ;
  rdfs:label "Owner to contract impact" ;
  reqvire:changeRuleName "owner-to-contract-impact" ;
  reqvire:changedThing "requirement-owner" ;
  reqvire:impactRelation "definedBy" ;
  reqvire:impactDirection "downstream" ;
  reqvire:propagationTarget "owned-contract" ;
  reqvire:propagationMode "review-required" ;
  reqvire:impactReason "Owner changes can change the scope or meaning of owned contracts." .

reqvire:ontologyToSemanticContractImpactRule a reqvire:ChangePropagationRule ;
  rdfs:label "Ontology to semantic contract impact" ;
  reqvire:changeRuleName "ontology-to-semantic-contract-impact" ;
  reqvire:changedThing "ontology" ;
  reqvire:impactRelation "usedBy" ;
  reqvire:impactDirection "downstream" ;
  reqvire:propagationTarget "semantic-contract" ;
  reqvire:propagationMode "review-required" ;
  reqvire:impactReason "Ontology vocabulary changes can invalidate SHACL shape profiles that explicitly use that vocabulary." .

reqvire:semanticContractToRequirementImpactRule a reqvire:ChangePropagationRule ;
  rdfs:label "Semantic contract to requirement impact" ;
  reqvire:changeRuleName "semantic-contract-to-requirement-impact" ;
  reqvire:changedThing "semantic-contract" ;
  reqvire:impactRelation "constrain" ;
  reqvire:impactDirection "downstream" ;
  reqvire:propagationTarget "requirement" ;
  reqvire:propagationMode "review-required" ;
  reqvire:impactReason "Semantic contract changes can alter the constraints imposed on the requirement they constrain." .

reqvire:semanticContractOntologyUseDependencyRule a reqvire:ChangePropagationRule ;
  rdfs:label "Semantic contract ontology use dependency" ;
  reqvire:changeRuleName "semantic-contract-ontology-use-dependency" ;
  reqvire:changedThing "semantic-contract-use-relation" ;
  reqvire:impactRelation "use" ;
  reqvire:impactDirection "dependency" ;
  reqvire:propagationTarget "ontology-context" ;
  reqvire:propagationMode "dependency-record" ;
  reqvire:impactReason "Semantic contracts record ontology vocabulary dependencies through use relations, while ontology content changes propagate back through the inverse usedBy relation." .

reqvire:conceptReferenceDependencyRule a reqvire:ChangePropagationRule ;
  rdfs:label "Concept reference dependency" ;
  reqvire:changeRuleName "concept-reference-dependency" ;
  reqvire:changedThing "concept-reference" ;
  reqvire:impactRelation "conceptReference" ;
  reqvire:impactDirection "dependency" ;
  reqvire:propagationTarget "referenced-ontology-term" ;
  reqvire:propagationMode "dependency-record" ;
  reqvire:impactReason "Non-ontology, non-semantic-contract elements record ontology term dependencies through explicit concept references." .

reqvire:requirementToSemanticContractReviewRule a reqvire:ChangePropagationRule ;
  rdfs:label "Requirement to semantic contract review" ;
  reqvire:changeRuleName "requirement-to-semantic-contract-review" ;
  reqvire:changedThing "requirement" ;
  reqvire:impactRelation "constrainedBy" ;
  reqvire:impactDirection "downstream" ;
  reqvire:propagationTarget "semantic-contract" ;
  reqvire:propagationMode "consistency-review-required" ;
  reqvire:impactReason "Requirement obligation changes can make existing semantic contract constraints inconsistent with the requirement text." .

reqvire:reusedContractContextContentImpactRule a reqvire:ChangePropagationRule ;
  rdfs:label "Reused Contract Context content impact" ;
  reqvire:changeRuleName "reused-contract-context-content-impact" ;
  reqvire:changedThing "reused-contract-content" ;
  reqvire:impactRelation "reused_contract_context" ;
  reqvire:impactDirection "bidirectional" ;
  reqvire:propagationTarget "reusesContract-element-and-owner-context" ;
  reqvire:propagationMode "review-required" ;
  reqvire:impactReason "Reused Contract Context content changes affect explicit cross-subgraph contract consumers." .

reqvire:semanticReferenceReachabilityRule a reqvire:ChangePropagationRule ;
  rdfs:label "Semantic reference reachability" ;
  reqvire:changeRuleName "semantic-reference-reachability" ;
  reqvire:changedThing "semantic-reference" ;
  reqvire:impactRelation "conceptReference-or-use" ;
  reqvire:impactDirection "downstream" ;
  reqvire:propagationTarget "referencing-element-or-semantic-contract" ;
  reqvire:propagationMode "validation-error-when-unreachable" ;
  reqvire:impactReason "Concept references must resolve to declared ontology terms, and semantic-contract SHACL references must resolve through explicit use relations." .

reqvire:relocationNoPropagationRule a reqvire:ChangePropagationRule ;
  rdfs:label "Relocation without content change" ;
  reqvire:changeRuleName "relocation-without-content-change" ;
  reqvire:changedThing "element-or-asset-location" ;
  reqvire:impactRelation "identifier-or-path-update" ;
  reqvire:impactDirection "none" ;
  reqvire:propagationTarget "none" ;
  reqvire:propagationMode "report-only" ;
  reqvire:impactReason "Pure relocation updates identifiers or paths but does not change semantic content." .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Semantic Contract Ontology](CapabilityRequirementModel.md#reqvire-semantic-contract-ontology)
---

### Reqvire Relation Ontology

The Reqvire relation ontology defines Reqvire relation vocabulary.

Relation contracts are separated from element-family contracts because relation behavior is reused across capability, requirement, contract, verification, artifact, and reused_contract_context flows.

#### Relation Semantics Context

A Reqvire relation type defines a semantic connection between model elements, its authored direction, its inverse relation when one exists, whether it creates ownership, and whether changes should propagate through that relation during impact analysis.

Relation families group authored relation tokens and inverse pairs by stable model meaning. Semantic query clients should prefer relation-family normalized predicates over raw relation-token matching so queries can ask for hierarchy, capability specification, contract ownership, semantic-contract constraint, semantic-contract ontology use, verification, satisfaction, and cross-subgraph contract dependency without hard-coding every authored direction.

Only hierarchy relation families have transitive closure semantics. Other relation families are direct relationships unless a separate ontology rule states otherwise.

#### Change Impact Context

Change impact follows relation semantics rather than file layout:

- Hierarchy changes propagate from parent elements to derived child elements inside the same hierarchy family.
- Requirement changes propagate to implementation/evidence artifacts, owned contract artifacts, constraining semantic contracts, and concrete verification elements that verify the requirement.
- Ontology changes propagate through ontology hierarchy and to semantic contracts that use the changed ontology vocabulary.
- Semantic-contract use records are dependency edges; semantic-contract content changes do not propagate back to ontology vocabulary.
- Verification updates generally do not propagate upward, but may be required after capability or requirement changes.

#### Endpoint Compatibility Context

Relation endpoint compatibility is semantic:

- Capability, requirement, ontology, and verification hierarchy families stay separate for `derive` and `derivedFrom`.
- Requirements specify capabilities through `specify`; capabilities point to specifying requirements through `specifiedBy`.
- Requirements own non-semantic contract context through `definedBy`; ordinary contract elements point back to their owner through `define`.
- Semantic contracts constrain requirements through `constrain` and `constrainedBy`, and use ontology vocabulary through `use` and `usedBy`.
- Capabilities and requirements are verified by concrete verification elements through `verifiedBy` and `verify`; verification objectives organize verification hierarchy but are not concrete verification evidence.
- Requirements and evidence-backed verifications are satisfied by implementation or evidence artifacts through `satisfiedBy` and `satisfy`.
- Reused Contract Context expresses a requirement dependency on compatible requirement-owned contract context from another subgraph without transferring ownership.
- Explicit `other` elements cannot author semantic relations; model meaning should be expressed with a specific supported element type or ontology concept references.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:RelationRule a owl:Class ;
  rdfs:label "Relation rule" ;
  rdfs:comment "Controlled rule for an authored Reqvire relation name, allowed endpoint types, direction, ownership, and impact behavior." .
reqvire:RelationConstraint a owl:Class ;
  rdfs:label "Relation constraint" ;
  rdfs:comment "Constraint concept that further qualifies a relation rule." .
reqvire:TraversalRule a owl:Class ;
  rdfs:label "Traversal rule" ;
  rdfs:comment "Rule describing how a relation category may be traversed for model navigation." .
reqvire:ReusedContractContextCompatibilityRule a owl:Class ;
  rdfs:subClassOf reqvire:RelationConstraint ;
  rdfs:label "Reused Contract Context compatibility rule" ;
  rdfs:comment "Constraint defining valid reused_contract_context source, target, and owner compatibility." .
reqvire:RelationUsageCategory a owl:Class ;
  rdfs:label "Relation usage category" ;
  rdfs:comment "Controlled category grouping relation names by operational usage such as rendering, traversal, change propagation, or rollup." .
reqvire:RelationSemanticCategory a owl:Class ;
  rdfs:label "Relation semantic category" ;
  rdfs:comment "Controlled category grouping relation names by their model semantics." .
reqvire:RelationSemanticPattern a owl:Class ;
  rdfs:label "Relation semantic pattern" ;
  rdfs:comment "Controlled semantic pattern that describes whether a relation family is hierarchy, bridge, ownership, constraint, dependency, verification, satisfaction, or cross-subgraph contract dependency." .
reqvire:RelationFamily a owl:Class ;
  rdfs:label "Relation family" ;
  rdfs:comment "Controlled family that groups authored relation names and inverse pairs by stable model meaning for semantic search." .
reqvire:ModelRelation a owl:Class ;
  rdfs:label "Model relation" ;
  rdfs:comment "First-class projection record for one authored Reqvire relation edge, used by construct-query specifications to preserve source, target, and relation-token pairing." .
reqvire:HierarchyRelationPattern a owl:Class ;
  rdfs:subClassOf reqvire:RelationSemanticPattern ;
  rdfs:label "Hierarchy relation pattern" ;
  rdfs:comment "Relation pattern for same-family parent-child hierarchy with optional ancestor-descendant closure." .
reqvire:BridgeRelationPattern a owl:Class ;
  rdfs:subClassOf reqvire:RelationSemanticPattern ;
  rdfs:label "Bridge relation pattern" ;
  rdfs:comment "Relation pattern connecting model families without transitive hierarchy semantics." .
reqvire:OwnershipRelationPattern a owl:Class ;
  rdfs:subClassOf reqvire:RelationSemanticPattern ;
  rdfs:label "Ownership relation pattern" ;
  rdfs:comment "Relation pattern where one element owns another element as local defining context." .
reqvire:ConstraintRelationPattern a owl:Class ;
  rdfs:subClassOf reqvire:RelationSemanticPattern ;
  rdfs:label "Constraint relation pattern" ;
  rdfs:comment "Relation pattern applying semantic-contract constraints to governed requirements." .
reqvire:DependencyRelationPattern a owl:Class ;
  rdfs:subClassOf reqvire:RelationSemanticPattern ;
  rdfs:label "Dependency relation pattern" ;
  rdfs:comment "Relation pattern where one element depends on another context element without ownership." .
reqvire:VerificationRelationPattern a owl:Class ;
  rdfs:subClassOf reqvire:RelationSemanticPattern ;
  rdfs:label "Verification relation pattern" ;
  rdfs:comment "Relation pattern connecting capabilities or requirements to verification elements." .
reqvire:SatisfactionRelationPattern a owl:Class ;
  rdfs:subClassOf reqvire:RelationSemanticPattern ;
  rdfs:label "Satisfaction relation pattern" ;
  rdfs:comment "Relation pattern connecting requirements or evidence-backed verifications to implementation or evidence artifacts." .
reqvire:CrossSubgraphContractDependencyPattern a owl:Class ;
  rdfs:subClassOf reqvire:RelationSemanticPattern ;
  rdfs:label "Cross-subgraph contract dependency pattern" ;
  rdfs:comment "Relation pattern for a requirement using reusable requirement-owned contract context from another subgraph without transferring ownership." .

reqvire:hierarchyRelationPattern a owl:NamedIndividual, reqvire:RelationSemanticPattern, reqvire:HierarchyRelationPattern ;
  reqvire:semanticPatternName "hierarchy" ;
  rdfs:comment "Same-family parent-child hierarchy. Only this pattern has ancestor-descendant closure semantics." .
reqvire:bridgeRelationPattern a owl:NamedIndividual, reqvire:RelationSemanticPattern, reqvire:BridgeRelationPattern ;
  reqvire:semanticPatternName "bridge" ;
  rdfs:comment "Direct bridge between different model families without hierarchy closure." .
reqvire:ownershipRelationPattern a owl:NamedIndividual, reqvire:RelationSemanticPattern, reqvire:OwnershipRelationPattern ;
  reqvire:semanticPatternName "ownership" ;
  rdfs:comment "Direct ownership of local defining context." .
reqvire:constraintRelationPattern a owl:NamedIndividual, reqvire:RelationSemanticPattern, reqvire:ConstraintRelationPattern ;
  reqvire:semanticPatternName "constraint" ;
  rdfs:comment "Direct application of semantic-contract constraints." .
reqvire:dependencyRelationPattern a owl:NamedIndividual, reqvire:RelationSemanticPattern, reqvire:DependencyRelationPattern ;
  reqvire:semanticPatternName "dependency" ;
  rdfs:comment "Direct semantic dependency without ownership." .
reqvire:verificationRelationPattern a owl:NamedIndividual, reqvire:RelationSemanticPattern, reqvire:VerificationRelationPattern ;
  reqvire:semanticPatternName "verification" ;
  rdfs:comment "Direct verification relationship." .
reqvire:satisfactionRelationPattern a owl:NamedIndividual, reqvire:RelationSemanticPattern, reqvire:SatisfactionRelationPattern ;
  reqvire:semanticPatternName "satisfaction" ;
  rdfs:comment "Direct implementation or evidence satisfaction relationship." .
reqvire:crossSubgraphContractDependencyPattern a owl:NamedIndividual, reqvire:RelationSemanticPattern, reqvire:CrossSubgraphContractDependencyPattern ;
  reqvire:semanticPatternName "cross-subgraph-contract-dependency" ;
  rdfs:comment "Direct cross-subgraph dependency from a requirement to reusable requirement-owned contract context declared through the reused_contract_context authoring mechanism." .

reqvire:hierarchyRelationFamily a owl:NamedIndividual, reqvire:RelationFamily ;
  rdfs:label "Hierarchy relation family" ;
  reqvire:relationFamilyName "hierarchy" ;
  reqvire:relationFamilyMeaning "Same-family parent-child derivation with ancestor-descendant closure semantics." ;
  reqvire:relationFamilyForwardProperty reqvire:childElement ;
  reqvire:relationFamilyInverseProperty reqvire:parentElement .
reqvire:capabilitySpecificationRelationFamily a owl:NamedIndividual, reqvire:RelationFamily ;
  rdfs:label "Capability specification relation family" ;
  reqvire:relationFamilyName "capability-specification" ;
  reqvire:relationFamilyMeaning "Bridge between a capability and the requirements that specify it." ;
  reqvire:relationFamilyForwardProperty reqvire:capabilitySpecifiedByRequirement ;
  reqvire:relationFamilyInverseProperty reqvire:requirementSpecifiesCapability .
reqvire:contractOwnershipRelationFamily a owl:NamedIndividual, reqvire:RelationFamily ;
  rdfs:label "Contract ownership relation family" ;
  reqvire:relationFamilyName "contract-ownership" ;
  reqvire:relationFamilyMeaning "Ownership from a requirement to local non-semantic contract context that defines it." ;
  reqvire:relationFamilyForwardProperty reqvire:requirementDefinesContract ;
  reqvire:relationFamilyInverseProperty reqvire:contractDefinedByRequirement .
reqvire:semanticContractConstraintRelationFamily a owl:NamedIndividual, reqvire:RelationFamily ;
  rdfs:label "Semantic contract constraint relation family" ;
  reqvire:relationFamilyName "semantic-contract-constraint" ;
  reqvire:relationFamilyMeaning "Application of semantic-contract SHACL constraints to governed requirements." ;
  reqvire:relationFamilyForwardProperty reqvire:requirementConstrainedBySemanticContract ;
  reqvire:relationFamilyInverseProperty reqvire:semanticContractConstrainsRequirement .
reqvire:semanticContractOntologyUseRelationFamily a owl:NamedIndividual, reqvire:RelationFamily ;
  rdfs:label "Semantic contract ontology use relation family" ;
  reqvire:relationFamilyName "semantic-contract-ontology-use" ;
  reqvire:relationFamilyMeaning "Dependency from a semantic contract to ontology vocabulary used by its SHACL shapes." ;
  reqvire:relationFamilyForwardProperty reqvire:semanticContractUsesOntology ;
  reqvire:relationFamilyInverseProperty reqvire:ontologyUsedBySemanticContract .
reqvire:verificationRelationFamily a owl:NamedIndividual, reqvire:RelationFamily ;
  rdfs:label "Verification relation family" ;
  reqvire:relationFamilyName "verification" ;
  reqvire:relationFamilyMeaning "Verification relationship between capabilities or requirements and concrete verification elements." ;
  reqvire:relationFamilyForwardProperty reqvire:elementVerifiedByVerification ;
  reqvire:relationFamilyInverseProperty reqvire:verificationVerifiesElement .
reqvire:satisfactionRelationFamily a owl:NamedIndividual, reqvire:RelationFamily ;
  rdfs:label "Satisfaction relation family" ;
  reqvire:relationFamilyName "satisfaction" ;
  reqvire:relationFamilyMeaning "Implementation or evidence satisfaction relationship from requirements or evidence-backed verifications to artifacts." ;
  reqvire:relationFamilyForwardProperty reqvire:elementSatisfiedByArtifact ;
  reqvire:relationFamilyInverseProperty reqvire:artifactSatisfiesElement .
reqvire:crossSubgraphContractDependencyRelationFamily a owl:NamedIndividual, reqvire:RelationFamily ;
  rdfs:label "Cross-subgraph contract dependency relation family" ;
  reqvire:relationFamilyName "cross-subgraph-contract-dependency" ;
  reqvire:relationFamilyMeaning "Dependency from a requirement to reusable requirement-owned contract context in another subgraph." ;
  reqvire:relationFamilyForwardProperty reqvire:requirementUsesCrossSubgraphContract ;
  reqvire:relationFamilyInverseProperty reqvire:crossSubgraphContractUsedByRequirement .

reqvire:derive a owl:ObjectProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range reqvire:Element ;
  owl:inverseOf reqvire:derivedFrom ;
  rdfs:comment "Forward hierarchy relation from a capability, requirement, or ontology parent to a same-family child." .
reqvire:derivedFrom a owl:ObjectProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range reqvire:Element ;
  owl:inverseOf reqvire:derive ;
  rdfs:comment "Inverse hierarchy relation from a capability, requirement, or ontology child to a same-family parent." .
reqvire:parentElement a owl:ObjectProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range reqvire:Element ;
  owl:inverseOf reqvire:childElement ;
  rdfs:comment "Normalized semantic hierarchy relation from any derived element to its immediate parent element, materialized from derivedFrom or inverse derive authoring." .
reqvire:childElement a owl:ObjectProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range reqvire:Element ;
  owl:inverseOf reqvire:parentElement ;
  rdfs:comment "Normalized semantic hierarchy relation from any parent element to its immediate derived child element, materialized from derive or inverse derivedFrom authoring." .
reqvire:ancestorElement a owl:ObjectProperty, owl:TransitiveProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range reqvire:Element ;
  owl:inverseOf reqvire:descendantElement ;
  rdfs:comment "Normalized transitive semantic hierarchy relation from an element to any ancestor in the same hierarchy family." .
reqvire:descendantElement a owl:ObjectProperty, owl:TransitiveProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range reqvire:Element ;
  owl:inverseOf reqvire:ancestorElement ;
  rdfs:comment "Normalized transitive semantic hierarchy relation from an element to any descendant in the same hierarchy family." .
reqvire:parentCapability a owl:ObjectProperty ;
  rdfs:subPropertyOf reqvire:parentElement ;
  rdfs:domain reqvire:Capability ;
  rdfs:range reqvire:Capability ;
  owl:inverseOf reqvire:childCapability ;
  rdfs:comment "Normalized immediate parent relation inside capability hierarchy." .
reqvire:childCapability a owl:ObjectProperty ;
  rdfs:subPropertyOf reqvire:childElement ;
  rdfs:domain reqvire:Capability ;
  rdfs:range reqvire:Capability ;
  owl:inverseOf reqvire:parentCapability ;
  rdfs:comment "Normalized immediate child relation inside capability hierarchy." .
reqvire:ancestorCapability a owl:ObjectProperty, owl:TransitiveProperty ;
  rdfs:subPropertyOf reqvire:ancestorElement ;
  rdfs:domain reqvire:Capability ;
  rdfs:range reqvire:Capability ;
  owl:inverseOf reqvire:descendantCapability ;
  rdfs:comment "Normalized transitive ancestor relation inside capability hierarchy." .
reqvire:descendantCapability a owl:ObjectProperty, owl:TransitiveProperty ;
  rdfs:subPropertyOf reqvire:descendantElement ;
  rdfs:domain reqvire:Capability ;
  rdfs:range reqvire:Capability ;
  owl:inverseOf reqvire:ancestorCapability ;
  rdfs:comment "Normalized transitive descendant relation inside capability hierarchy." .
reqvire:parentRequirement a owl:ObjectProperty ;
  rdfs:subPropertyOf reqvire:parentElement ;
  rdfs:domain reqvire:Requirement ;
  rdfs:range reqvire:Requirement ;
  owl:inverseOf reqvire:childRequirement ;
  rdfs:comment "Normalized immediate parent relation inside requirement hierarchy." .
reqvire:childRequirement a owl:ObjectProperty ;
  rdfs:subPropertyOf reqvire:childElement ;
  rdfs:domain reqvire:Requirement ;
  rdfs:range reqvire:Requirement ;
  owl:inverseOf reqvire:parentRequirement ;
  rdfs:comment "Normalized immediate child relation inside requirement hierarchy." .
reqvire:ancestorRequirement a owl:ObjectProperty, owl:TransitiveProperty ;
  rdfs:subPropertyOf reqvire:ancestorElement ;
  rdfs:domain reqvire:Requirement ;
  rdfs:range reqvire:Requirement ;
  owl:inverseOf reqvire:descendantRequirement ;
  rdfs:comment "Normalized transitive ancestor relation inside requirement hierarchy." .
reqvire:descendantRequirement a owl:ObjectProperty, owl:TransitiveProperty ;
  rdfs:subPropertyOf reqvire:descendantElement ;
  rdfs:domain reqvire:Requirement ;
  rdfs:range reqvire:Requirement ;
  owl:inverseOf reqvire:ancestorRequirement ;
  rdfs:comment "Normalized transitive descendant relation inside requirement hierarchy." .
reqvire:parentOntology a owl:ObjectProperty ;
  rdfs:subPropertyOf reqvire:parentElement ;
  rdfs:domain reqvire:Ontology ;
  rdfs:range reqvire:Ontology ;
  owl:inverseOf reqvire:childOntology ;
  rdfs:comment "Normalized immediate parent relation inside ontology hierarchy." .
reqvire:childOntology a owl:ObjectProperty ;
  rdfs:subPropertyOf reqvire:childElement ;
  rdfs:domain reqvire:Ontology ;
  rdfs:range reqvire:Ontology ;
  owl:inverseOf reqvire:parentOntology ;
  rdfs:comment "Normalized immediate child relation inside ontology hierarchy." .
reqvire:ancestorOntology a owl:ObjectProperty, owl:TransitiveProperty ;
  rdfs:subPropertyOf reqvire:ancestorElement ;
  rdfs:domain reqvire:Ontology ;
  rdfs:range reqvire:Ontology ;
  owl:inverseOf reqvire:descendantOntology ;
  rdfs:comment "Normalized transitive ancestor relation inside ontology hierarchy." .
reqvire:descendantOntology a owl:ObjectProperty, owl:TransitiveProperty ;
  rdfs:subPropertyOf reqvire:descendantElement ;
  rdfs:domain reqvire:Ontology ;
  rdfs:range reqvire:Ontology ;
  owl:inverseOf reqvire:ancestorOntology ;
  rdfs:comment "Normalized transitive descendant relation inside ontology hierarchy." .
reqvire:parentVerificationElement a owl:ObjectProperty ;
  rdfs:subPropertyOf reqvire:parentElement ;
  rdfs:domain reqvire:Verification ;
  rdfs:range reqvire:Verification ;
  owl:inverseOf reqvire:childVerificationElement ;
  rdfs:comment "Normalized immediate parent relation inside verification-family hierarchy." .
reqvire:childVerificationElement a owl:ObjectProperty ;
  rdfs:subPropertyOf reqvire:childElement ;
  rdfs:domain reqvire:Verification ;
  rdfs:range reqvire:Verification ;
  owl:inverseOf reqvire:parentVerificationElement ;
  rdfs:comment "Normalized immediate child relation inside verification-family hierarchy." .
reqvire:ancestorVerificationElement a owl:ObjectProperty, owl:TransitiveProperty ;
  rdfs:subPropertyOf reqvire:ancestorElement ;
  rdfs:domain reqvire:Verification ;
  rdfs:range reqvire:Verification ;
  owl:inverseOf reqvire:descendantVerificationElement ;
  rdfs:comment "Normalized transitive ancestor relation inside verification-family hierarchy." .
reqvire:descendantVerificationElement a owl:ObjectProperty, owl:TransitiveProperty ;
  rdfs:subPropertyOf reqvire:descendantElement ;
  rdfs:domain reqvire:Verification ;
  rdfs:range reqvire:Verification ;
  owl:inverseOf reqvire:ancestorVerificationElement ;
  rdfs:comment "Normalized transitive descendant relation inside verification-family hierarchy." .
reqvire:specify a owl:ObjectProperty ;
  rdfs:domain reqvire:Requirement ;
  rdfs:range reqvire:Capability ;
  owl:inverseOf reqvire:specifiedBy ;
  rdfs:comment "Inverse relation from a requirement to the capability it specifies." .
reqvire:specifiedBy a owl:ObjectProperty ;
  rdfs:domain reqvire:Capability ;
  rdfs:range reqvire:Requirement ;
  owl:inverseOf reqvire:specify ;
  rdfs:comment "Forward relation from a capability to a requirement that specifies it." .
reqvire:capabilitySpecifiedByRequirement a owl:ObjectProperty ;
  rdfs:domain reqvire:Capability ;
  rdfs:range reqvire:Requirement ;
  owl:inverseOf reqvire:requirementSpecifiesCapability ;
  rdfs:comment "Normalized direct bridge from a capability to a requirement that specifies it, materialized from specifiedBy or inverse specify authoring." .
reqvire:requirementSpecifiesCapability a owl:ObjectProperty ;
  rdfs:domain reqvire:Requirement ;
  rdfs:range reqvire:Capability ;
  owl:inverseOf reqvire:capabilitySpecifiedByRequirement ;
  rdfs:comment "Normalized direct bridge from a requirement to the capability it specifies, materialized from specify or inverse specifiedBy authoring." .
reqvire:define a owl:ObjectProperty ;
  rdfs:domain reqvire:Contract ;
  rdfs:range reqvire:Requirement ;
  owl:inverseOf reqvire:definedBy ;
  rdfs:comment "Inverse ownership relation from a contract to its requirement owner." .
reqvire:definedBy a owl:ObjectProperty ;
  rdfs:domain reqvire:Requirement ;
  rdfs:range reqvire:Contract ;
  owl:inverseOf reqvire:define ;
  rdfs:comment "Forward ownership relation from a requirement to an owned contract." .
reqvire:requirementDefinesContract a owl:ObjectProperty ;
  rdfs:domain reqvire:Requirement ;
  rdfs:range reqvire:Contract ;
  owl:inverseOf reqvire:contractDefinedByRequirement ;
  rdfs:comment "Normalized direct ownership relation from a requirement to its owned contract context." .
reqvire:contractDefinedByRequirement a owl:ObjectProperty ;
  rdfs:domain reqvire:Contract ;
  rdfs:range reqvire:Requirement ;
  owl:inverseOf reqvire:requirementDefinesContract ;
  rdfs:comment "Normalized direct ownership relation from a contract to its requirement owner." .
reqvire:constrain a owl:ObjectProperty ;
  rdfs:domain reqvire:SemanticContract ;
  rdfs:range reqvire:Requirement ;
  owl:inverseOf reqvire:constrainedBy ;
  rdfs:comment "Relation from a semantic contract to a requirement constrained by that contract." .
reqvire:constrainedBy a owl:ObjectProperty ;
  rdfs:domain reqvire:Requirement ;
  rdfs:range reqvire:SemanticContract ;
  owl:inverseOf reqvire:constrain ;
  rdfs:comment "Relation from a requirement to a semantic contract that constrains it." .
reqvire:requirementConstrainedBySemanticContract a owl:ObjectProperty ;
  rdfs:domain reqvire:Requirement ;
  rdfs:range reqvire:SemanticContract ;
  owl:inverseOf reqvire:semanticContractConstrainsRequirement ;
  rdfs:comment "Normalized direct constraint relation from a requirement to a semantic contract that constrains it." .
reqvire:semanticContractConstrainsRequirement a owl:ObjectProperty ;
  rdfs:domain reqvire:SemanticContract ;
  rdfs:range reqvire:Requirement ;
  owl:inverseOf reqvire:requirementConstrainedBySemanticContract ;
  rdfs:comment "Normalized direct constraint relation from a semantic contract to a constrained requirement." .
reqvire:use a owl:ObjectProperty ;
  rdfs:domain reqvire:SemanticContract ;
  rdfs:range reqvire:Ontology ;
  owl:inverseOf reqvire:usedBy ;
  rdfs:comment "Relation from a semantic contract to ontology vocabulary it uses." .
reqvire:usedBy a owl:ObjectProperty ;
  rdfs:domain reqvire:Ontology ;
  rdfs:range reqvire:SemanticContract ;
  owl:inverseOf reqvire:use ;
  rdfs:comment "Relation from ontology vocabulary to a semantic contract that uses it." .
reqvire:semanticContractUsesOntology a owl:ObjectProperty ;
  rdfs:domain reqvire:SemanticContract ;
  rdfs:range reqvire:Ontology ;
  owl:inverseOf reqvire:ontologyUsedBySemanticContract ;
  rdfs:comment "Normalized direct dependency from a semantic contract to ontology vocabulary it uses." .
reqvire:ontologyUsedBySemanticContract a owl:ObjectProperty ;
  rdfs:domain reqvire:Ontology ;
  rdfs:range reqvire:SemanticContract ;
  owl:inverseOf reqvire:semanticContractUsesOntology ;
  rdfs:comment "Normalized direct dependency from ontology vocabulary to semantic contracts that use it." .
reqvire:verify a owl:ObjectProperty ;
  rdfs:domain reqvire:Verification ;
  rdfs:range [ a owl:Class ; owl:unionOf (reqvire:Capability reqvire:Requirement) ] ;
  owl:inverseOf reqvire:verifiedBy ;
  rdfs:comment "Inverse relation from a verification element to the capability or requirement it verifies." .
reqvire:verifiedBy a owl:ObjectProperty ;
  rdfs:domain [ a owl:Class ; owl:unionOf (reqvire:Capability reqvire:Requirement) ] ;
  rdfs:range reqvire:Verification ;
  owl:inverseOf reqvire:verify ;
  rdfs:comment "Forward relation from a capability or requirement to a verification element that verifies it." .
reqvire:elementVerifiedByVerification a owl:ObjectProperty ;
  rdfs:domain [ a owl:Class ; owl:unionOf (reqvire:Capability reqvire:Requirement) ] ;
  rdfs:range reqvire:Verification ;
  owl:inverseOf reqvire:verificationVerifiesElement ;
  rdfs:comment "Normalized direct verification relation from a capability or requirement to a verification element." .
reqvire:verificationVerifiesElement a owl:ObjectProperty ;
  rdfs:domain reqvire:Verification ;
  rdfs:range [ a owl:Class ; owl:unionOf (reqvire:Capability reqvire:Requirement) ] ;
  owl:inverseOf reqvire:elementVerifiedByVerification ;
  rdfs:comment "Normalized direct verification relation from a verification element to the capability or requirement it verifies." .
reqvire:satisfy a owl:ObjectProperty ;
  rdfs:domain reqvire:Artifact ;
  rdfs:range reqvire:Element ;
  owl:inverseOf reqvire:satisfiedBy ;
  rdfs:comment "Inverse relation from an implementation or evidence artifact to a requirement or evidence-backed verification it satisfies." .
reqvire:satisfiedBy a owl:ObjectProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range reqvire:Artifact ;
  owl:inverseOf reqvire:satisfy ;
  rdfs:comment "Forward relation from a requirement or evidence-backed verification to implementation or evidence artifacts." .
reqvire:elementSatisfiedByArtifact a owl:ObjectProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range reqvire:Artifact ;
  owl:inverseOf reqvire:artifactSatisfiesElement ;
  rdfs:comment "Normalized direct satisfaction relation from a requirement or evidence-backed verification to an implementation or evidence artifact." .
reqvire:artifactSatisfiesElement a owl:ObjectProperty ;
  rdfs:domain reqvire:Artifact ;
  rdfs:range reqvire:Element ;
  owl:inverseOf reqvire:elementSatisfiedByArtifact ;
  rdfs:comment "Normalized direct satisfaction relation from an implementation or evidence artifact to the element it satisfies." .
reqvire:reuse a owl:ObjectProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range reqvire:Element ;
  owl:inverseOf reqvire:reusesContract ;
  rdfs:comment "Inverse reused_contract_context relation from a reusable requirement-owned contract back to its consuming requirement." .
reqvire:reusesContract a owl:ObjectProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range reqvire:Element ;
  owl:inverseOf reqvire:reuse ;
  rdfs:comment "Forward reused_contract_context relation from a requirement to explicit reusable requirement-owned contract context." .
reqvire:requirementUsesCrossSubgraphContract a owl:ObjectProperty ;
  rdfs:domain reqvire:Requirement ;
  rdfs:range reqvire:Contract ;
  owl:inverseOf reqvire:crossSubgraphContractUsedByRequirement ;
  rdfs:comment "Normalized direct relation from a requirement to reusable contract context it uses from another subgraph." .
reqvire:crossSubgraphContractUsedByRequirement a owl:ObjectProperty ;
  rdfs:domain reqvire:Contract ;
  rdfs:range reqvire:Requirement ;
  owl:inverseOf reqvire:requirementUsesCrossSubgraphContract ;
  rdfs:comment "Normalized inverse relation from reusable contract context to requirements in other subgraphs that use it." .
reqvire:implementedByArtifact a owl:ObjectProperty ;
  rdfs:domain reqvire:Capability ;
  rdfs:range reqvire:Artifact ;
  owl:propertyChainAxiom (reqvire:specifiedBy reqvire:satisfiedBy) ;
  rdfs:comment "Inferred capability-to-artifact trace when a capability is specified by a requirement satisfied by an artifact." .

reqvire:inverseRelation a owl:ObjectProperty ;
  rdfs:domain reqvire:RelationRule ;
  rdfs:range owl:ObjectProperty ;
  rdfs:comment "Object property that is the true inverse of the relation described by a relation rule." .
reqvire:relationPattern a owl:ObjectProperty ;
  rdfs:domain reqvire:RelationRule ;
  rdfs:range reqvire:RelationSemanticPattern ;
  rdfs:comment "Semantic pattern for the relation family described by a relation rule." .
reqvire:relationFamily a owl:ObjectProperty ;
  rdfs:domain reqvire:RelationRule ;
  rdfs:range reqvire:RelationFamily ;
  rdfs:comment "Stable semantic family for the authored relation rule, grouping inverse relation tokens and normalized query properties." .
reqvire:relationSource a owl:ObjectProperty ;
  rdfs:domain reqvire:ModelRelation ;
  rdfs:range reqvire:Element ;
  rdfs:comment "Source element for one authored model relation edge." .
reqvire:normalizedForwardProperty a owl:ObjectProperty ;
  rdfs:domain reqvire:RelationRule ;
  rdfs:range owl:ObjectProperty ;
  rdfs:comment "Canonical forward normalized property materialized for query-oriented model graph search." .
reqvire:normalizedInverseProperty a owl:ObjectProperty ;
  rdfs:domain reqvire:RelationRule ;
  rdfs:range owl:ObjectProperty ;
  rdfs:comment "Canonical inverse normalized property materialized for query-oriented model graph search." .
reqvire:transitiveClosureForwardProperty a owl:ObjectProperty ;
  rdfs:domain reqvire:RelationRule ;
  rdfs:range owl:ObjectProperty ;
  rdfs:comment "Canonical forward transitive closure property materialized only for relation patterns with closure semantics." .
reqvire:transitiveClosureInverseProperty a owl:ObjectProperty ;
  rdfs:domain reqvire:RelationRule ;
  rdfs:range owl:ObjectProperty ;
  rdfs:comment "Canonical inverse transitive closure property materialized only for relation patterns with closure semantics." .
reqvire:relationConstraint a owl:ObjectProperty ;
  rdfs:domain reqvire:RelationRule ;
  rdfs:range reqvire:RelationConstraint ;
  rdfs:comment "Constraint associated with a relation rule." .
reqvire:relationName a owl:DatatypeProperty ;
  rdfs:domain reqvire:RelationRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical authored relation name token used in Markdown metadata, CLI output, reports, validation, and queries." .
reqvire:semanticPatternName a owl:DatatypeProperty ;
  rdfs:domain reqvire:RelationSemanticPattern ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical relation semantic pattern token used by semantic search contracts and relation-rule metadata." .
reqvire:allowedSourceType a owl:DatatypeProperty ;
  rdfs:domain reqvire:RelationRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical element type or target-kind token allowed as the relation source." .
reqvire:allowedTargetType a owl:DatatypeProperty ;
  rdfs:domain reqvire:RelationRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical element type, target-kind, or compatibility token allowed as the relation target." .
reqvire:relationDirection a owl:DatatypeProperty ;
  rdfs:domain reqvire:RelationRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical direction token for authored relation presentation and traversal." .
reqvire:traversalDirection a owl:DatatypeProperty ;
  rdfs:domain reqvire:TraversalRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical traversal direction token for a traversal rule." .
reqvire:createsOwnership a owl:DatatypeProperty ;
  rdfs:domain reqvire:RelationRule ;
  rdfs:range xsd:boolean ;
  rdfs:comment "Whether the relation establishes ownership in the Reqvire model graph." .
reqvire:propagatesChangeImpact a owl:DatatypeProperty ;
  rdfs:domain reqvire:RelationRule ;
  rdfs:range xsd:boolean ;
  rdfs:comment "Whether changes propagate through this relation in change-impact analysis." .
reqvire:relationRuleDescription a owl:DatatypeProperty ;
  rdfs:domain reqvire:RelationRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable semantic description of a relation rule." .
reqvire:reusedContractContextSourceType a owl:DatatypeProperty ;
  rdfs:domain reqvire:ReusedContractContextCompatibilityRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical element type token allowed as an reused_contract_context source." .
reqvire:reusedContractContextTargetType a owl:DatatypeProperty ;
  rdfs:domain reqvire:ReusedContractContextCompatibilityRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical element type or contract-family token allowed as an reused_contract_context target." .
reqvire:reusedContractContextOwnerType a owl:DatatypeProperty ;
  rdfs:domain reqvire:ReusedContractContextCompatibilityRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical owner element type required for the reused_contract_context target." .
reqvire:reusedContractContextRuleDescription a owl:DatatypeProperty ;
  rdfs:domain reqvire:ReusedContractContextCompatibilityRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable semantic description of an reused_contract_context compatibility rule." .
reqvire:usageCategoryName a owl:DatatypeProperty ;
  rdfs:domain reqvire:RelationUsageCategory ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical relation usage category token." .
reqvire:usageCategoryMeaning a owl:DatatypeProperty ;
  rdfs:domain reqvire:RelationUsageCategory ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable semantic meaning of a relation usage category." .
reqvire:usageCategoryRelationName a owl:DatatypeProperty ;
  rdfs:domain reqvire:RelationUsageCategory ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical relation name token included in a usage category." .
reqvire:semanticCategoryName a owl:DatatypeProperty ;
  rdfs:domain reqvire:RelationSemanticCategory ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical relation semantic category token." .
reqvire:semanticCategoryMeaning a owl:DatatypeProperty ;
  rdfs:domain reqvire:RelationSemanticCategory ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable semantic meaning of a relation semantic category." .
reqvire:semanticCategoryRelationName a owl:DatatypeProperty ;
  rdfs:domain reqvire:RelationSemanticCategory ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical relation name token included in a semantic category." .
reqvire:relationFamilyName a owl:DatatypeProperty ;
  rdfs:domain reqvire:RelationFamily ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical relation-family token used by semantic search and relation-rule metadata." .
reqvire:relationFamilyMeaning a owl:DatatypeProperty ;
  rdfs:domain reqvire:RelationFamily ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable semantic meaning of a relation family." .
reqvire:relationFamilyForwardProperty a owl:ObjectProperty ;
  rdfs:domain reqvire:RelationFamily ;
  rdfs:range owl:ObjectProperty ;
  rdfs:comment "Normalized forward query property for a relation family." .
reqvire:relationFamilyInverseProperty a owl:ObjectProperty ;
  rdfs:domain reqvire:RelationFamily ;
  rdfs:range owl:ObjectProperty ;
  rdfs:comment "Normalized inverse query property for a relation family." .

reqvire:diagramRenderingRelationUsageCategory a reqvire:RelationUsageCategory ;
  rdfs:label "Diagram rendering forward relations" ;
  rdfs:comment "Forward relation names rendered in diagrams and root-to-leaf model views to avoid duplicate inverse arrows." ;
  reqvire:usageCategoryName "diagram-rendering-forward" ;
  reqvire:usageCategoryMeaning "Forward relation names rendered in diagrams and root-to-leaf model views to avoid duplicate inverse arrows." ;
  reqvire:usageCategoryRelationName "derive", "specifiedBy", "satisfiedBy", "definedBy", "constrainedBy", "use", "verifiedBy" .
reqvire:reverseTraversalRelationUsageCategory a reqvire:RelationUsageCategory ;
  rdfs:label "Reverse traversal relations" ;
  rdfs:comment "Inverse relation names used for leaf-to-root traversal and upward traceability." ;
  reqvire:usageCategoryName "reverse-traversal" ;
  reqvire:usageCategoryMeaning "Inverse relation names used for leaf-to-root traversal and upward traceability." ;
  reqvire:usageCategoryRelationName "derivedFrom", "specify", "satisfy", "define", "constrain", "usedBy", "verify" .
reqvire:changePropagationRelationUsageCategory a reqvire:RelationUsageCategory ;
  rdfs:label "Change propagation relations" ;
  rdfs:comment "Relation names through which changed upstream model meaning propagates to dependent downstream elements." ;
  reqvire:usageCategoryName "change-propagation" ;
  reqvire:usageCategoryMeaning "Relation names through which changed upstream model meaning propagates to dependent downstream elements." ;
  reqvire:usageCategoryRelationName "derive", "specifiedBy", "satisfiedBy", "definedBy", "constrainedBy", "constrain", "use", "usedBy", "verifiedBy", "reused_contract_context" .
reqvire:verificationRollupRelationUsageCategory a reqvire:RelationUsageCategory ;
  rdfs:label "Verification rollup relations" ;
  rdfs:comment "Relation names used to roll verification state from requirement leaves toward ancestors." ;
  reqvire:usageCategoryName "verification-rollup" ;
  reqvire:usageCategoryMeaning "Relation names used to roll verification state from requirement leaves toward ancestors." ;
  reqvire:usageCategoryRelationName "derivedFrom", "verify", "verifiedBy" .

reqvire:hierarchyRelationCategory a reqvire:RelationSemanticCategory ;
  rdfs:label "Hierarchy relations" ;
  rdfs:comment "Same-family capability, requirement, or ontology derivation hierarchy." ;
  reqvire:semanticCategoryName "hierarchy" ;
  reqvire:semanticCategoryMeaning "Same-family capability, requirement, or ontology derivation hierarchy." ;
  reqvire:semanticCategoryRelationName "derive", "derivedFrom" .
reqvire:capabilitySpecificationRelationCategory a reqvire:RelationSemanticCategory ;
  rdfs:label "Capability specification relations" ;
  rdfs:comment "Bridge between capability elements and requirements that specify them." ;
  reqvire:semanticCategoryName "capability-specification" ;
  reqvire:semanticCategoryMeaning "Bridge between capability elements and requirements that specify them." ;
  reqvire:semanticCategoryRelationName "specify", "specifiedBy" .
reqvire:satisfactionRelationCategory a reqvire:RelationSemanticCategory ;
  rdfs:label "Satisfaction relations" ;
  rdfs:comment "Links from requirements or evidence-backed verifications to implementation or evidence artifacts." ;
  reqvire:semanticCategoryName "satisfaction" ;
  reqvire:semanticCategoryMeaning "Links from requirements or evidence-backed verifications to implementation or evidence artifacts." ;
  reqvire:semanticCategoryRelationName "satisfy", "satisfiedBy" .
reqvire:contractRelationCategory a reqvire:RelationSemanticCategory ;
  rdfs:label "Contract ownership relations" ;
  rdfs:comment "Ownership of requirement-owned contracts." ;
  reqvire:semanticCategoryName "contract-ownership" ;
  reqvire:semanticCategoryMeaning "Ownership of requirement-owned contracts." ;
  reqvire:semanticCategoryRelationName "define", "definedBy" .
reqvire:semanticContractConstraintRelationCategory a reqvire:RelationSemanticCategory ;
  rdfs:label "Semantic contract constraint relations" ;
  rdfs:comment "Application of reusable semantic contracts to constrained requirements." ;
  reqvire:semanticCategoryName "semantic-contract-constraint" ;
  reqvire:semanticCategoryMeaning "Application of reusable semantic contracts to constrained requirements." ;
  reqvire:semanticCategoryRelationName "constrain", "constrainedBy" .
reqvire:semanticContractOntologyUseRelationCategory a reqvire:RelationSemanticCategory ;
  rdfs:label "Semantic contract ontology use relations" ;
  rdfs:comment "Explicit ontology vocabulary dependencies used by semantic contracts." ;
  reqvire:semanticCategoryName "semantic-contract-ontology-use" ;
  reqvire:semanticCategoryMeaning "Explicit ontology vocabulary dependencies used by semantic contracts." ;
  reqvire:semanticCategoryRelationName "use", "usedBy" .
reqvire:verificationRelationCategory a reqvire:RelationSemanticCategory ;
  rdfs:label "Verification relations" ;
  rdfs:comment "Links between capabilities or requirements and verification elements that verify them." ;
  reqvire:semanticCategoryName "verification" ;
  reqvire:semanticCategoryMeaning "Links between capabilities or requirements and verification elements that verify them." ;
  reqvire:semanticCategoryRelationName "verify", "verifiedBy" .
reqvire:crossSubgraphContractDependencyRelationCategory a reqvire:RelationSemanticCategory ;
  rdfs:label "Cross-subgraph contract dependency relation" ;
  rdfs:comment "Explicit dependency from a requirement to reusable requirement-owned contract context in another subgraph." ;
  reqvire:semanticCategoryName "cross-subgraph-contract-dependency" ;
  reqvire:semanticCategoryMeaning "Explicit dependency from a requirement to reusable requirement-owned contract context in another subgraph." ;
  reqvire:semanticCategoryRelationName "reused_contract_context" .

reqvire:deriveRelationRule a reqvire:RelationRule ;
  rdfs:label "derive" ;
  reqvire:relationName "derive" ;
  reqvire:inverseRelation reqvire:derivedFrom ;
  reqvire:relationPattern reqvire:hierarchyRelationPattern ;
  reqvire:relationFamily reqvire:hierarchyRelationFamily ;
  reqvire:normalizedForwardProperty reqvire:childElement ;
  reqvire:normalizedInverseProperty reqvire:parentElement ;
  reqvire:transitiveClosureForwardProperty reqvire:descendantElement ;
  reqvire:transitiveClosureInverseProperty reqvire:ancestorElement ;
  reqvire:allowedSourceType "capability", "requirement", "ontology", "verification-family" ;
  reqvire:allowedTargetType "same-hierarchy-family" ;
  reqvire:relationDirection "forward" ;
  reqvire:createsOwnership true ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Parent capability, requirement, ontology, or verification-family element derives child elements only within the same hierarchy family." .

reqvire:derivedFromRelationRule a reqvire:RelationRule ;
  rdfs:label "derivedFrom" ;
  reqvire:relationName "derivedFrom" ;
  reqvire:inverseRelation reqvire:derive ;
  reqvire:relationPattern reqvire:hierarchyRelationPattern ;
  reqvire:relationFamily reqvire:hierarchyRelationFamily ;
  reqvire:normalizedForwardProperty reqvire:childElement ;
  reqvire:normalizedInverseProperty reqvire:parentElement ;
  reqvire:transitiveClosureForwardProperty reqvire:descendantElement ;
  reqvire:transitiveClosureInverseProperty reqvire:ancestorElement ;
  reqvire:allowedSourceType "capability", "requirement", "ontology", "verification-family" ;
  reqvire:allowedTargetType "same-hierarchy-family" ;
  reqvire:relationDirection "inverse" ;
  reqvire:createsOwnership true ;
  reqvire:propagatesChangeImpact false ;
  reqvire:relationRuleDescription "Child capability, requirement, ontology, or verification-family element points to its parent in the same hierarchy family." .

reqvire:specifiedByRelationRule a reqvire:RelationRule ;
  rdfs:label "specifiedBy" ;
  reqvire:relationName "specifiedBy" ;
  reqvire:inverseRelation reqvire:specify ;
  reqvire:relationPattern reqvire:bridgeRelationPattern ;
  reqvire:relationFamily reqvire:capabilitySpecificationRelationFamily ;
  reqvire:normalizedForwardProperty reqvire:capabilitySpecifiedByRequirement ;
  reqvire:normalizedInverseProperty reqvire:requirementSpecifiesCapability ;
  reqvire:allowedSourceType "capability" ;
  reqvire:allowedTargetType "requirement" ;
  reqvire:relationDirection "forward" ;
  reqvire:createsOwnership true ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Capability points to a requirement that specifies the capability." .

reqvire:specifyRelationRule a reqvire:RelationRule ;
  rdfs:label "specify" ;
  reqvire:relationName "specify" ;
  reqvire:inverseRelation reqvire:specifiedBy ;
  reqvire:relationPattern reqvire:bridgeRelationPattern ;
  reqvire:relationFamily reqvire:capabilitySpecificationRelationFamily ;
  reqvire:normalizedForwardProperty reqvire:capabilitySpecifiedByRequirement ;
  reqvire:normalizedInverseProperty reqvire:requirementSpecifiesCapability ;
  reqvire:allowedSourceType "requirement" ;
  reqvire:allowedTargetType "capability" ;
  reqvire:relationDirection "inverse" ;
  reqvire:createsOwnership true ;
  reqvire:propagatesChangeImpact false ;
  reqvire:relationRuleDescription "Requirement points to the capability it specifies." .

reqvire:definedByRelationRule a reqvire:RelationRule ;
  rdfs:label "definedBy" ;
  reqvire:relationName "definedBy" ;
  reqvire:inverseRelation reqvire:define ;
  reqvire:relationPattern reqvire:ownershipRelationPattern ;
  reqvire:relationFamily reqvire:contractOwnershipRelationFamily ;
  reqvire:normalizedForwardProperty reqvire:requirementDefinesContract ;
  reqvire:normalizedInverseProperty reqvire:contractDefinedByRequirement ;
  reqvire:allowedSourceType "requirement" ;
  reqvire:allowedTargetType "subtype-compatible-non-semantic-contract" ;
  reqvire:relationDirection "forward" ;
  reqvire:createsOwnership true ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Requirement owns a subtype-compatible non-semantic contract element." .

reqvire:defineRelationRule a reqvire:RelationRule ;
  rdfs:label "define" ;
  reqvire:relationName "define" ;
  reqvire:inverseRelation reqvire:definedBy ;
  reqvire:relationPattern reqvire:ownershipRelationPattern ;
  reqvire:relationFamily reqvire:contractOwnershipRelationFamily ;
  reqvire:normalizedForwardProperty reqvire:requirementDefinesContract ;
  reqvire:normalizedInverseProperty reqvire:contractDefinedByRequirement ;
  reqvire:allowedSourceType "non-semantic-contract" ;
  reqvire:allowedTargetType "requirement-owner" ;
  reqvire:relationDirection "inverse" ;
  reqvire:createsOwnership true ;
  reqvire:propagatesChangeImpact false ;
  reqvire:relationRuleDescription "Non-semantic-contract element points to its single valid owner." .

reqvire:constrainedByRelationRule a reqvire:RelationRule ;
  rdfs:label "constrainedBy" ;
  reqvire:relationName "constrainedBy" ;
  reqvire:inverseRelation reqvire:constrain ;
  reqvire:relationPattern reqvire:constraintRelationPattern ;
  reqvire:relationFamily reqvire:semanticContractConstraintRelationFamily ;
  reqvire:normalizedForwardProperty reqvire:requirementConstrainedBySemanticContract ;
  reqvire:normalizedInverseProperty reqvire:semanticContractConstrainsRequirement ;
  reqvire:allowedSourceType "requirement" ;
  reqvire:allowedTargetType "semantic-contract" ;
  reqvire:relationDirection "forward" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Requirement points to a semantic contract that constrains it." .

reqvire:constrainRelationRule a reqvire:RelationRule ;
  rdfs:label "constrain" ;
  reqvire:relationName "constrain" ;
  reqvire:inverseRelation reqvire:constrainedBy ;
  reqvire:relationPattern reqvire:constraintRelationPattern ;
  reqvire:relationFamily reqvire:semanticContractConstraintRelationFamily ;
  reqvire:normalizedForwardProperty reqvire:requirementConstrainedBySemanticContract ;
  reqvire:normalizedInverseProperty reqvire:semanticContractConstrainsRequirement ;
  reqvire:allowedSourceType "semantic-contract" ;
  reqvire:allowedTargetType "requirement" ;
  reqvire:relationDirection "inverse" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Semantic contract points to a requirement constrained by the contract." .

reqvire:useRelationRule a reqvire:RelationRule ;
  rdfs:label "use" ;
  reqvire:relationName "use" ;
  reqvire:inverseRelation reqvire:usedBy ;
  reqvire:relationPattern reqvire:dependencyRelationPattern ;
  reqvire:relationFamily reqvire:semanticContractOntologyUseRelationFamily ;
  reqvire:normalizedForwardProperty reqvire:semanticContractUsesOntology ;
  reqvire:normalizedInverseProperty reqvire:ontologyUsedBySemanticContract ;
  reqvire:allowedSourceType "semantic-contract" ;
  reqvire:allowedTargetType "ontology" ;
  reqvire:relationDirection "forward" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Semantic contract points to ontology vocabulary used by its SHACL shapes." .

reqvire:usedByRelationRule a reqvire:RelationRule ;
  rdfs:label "usedBy" ;
  reqvire:relationName "usedBy" ;
  reqvire:inverseRelation reqvire:use ;
  reqvire:relationPattern reqvire:dependencyRelationPattern ;
  reqvire:relationFamily reqvire:semanticContractOntologyUseRelationFamily ;
  reqvire:normalizedForwardProperty reqvire:semanticContractUsesOntology ;
  reqvire:normalizedInverseProperty reqvire:ontologyUsedBySemanticContract ;
  reqvire:allowedSourceType "ontology" ;
  reqvire:allowedTargetType "semantic-contract" ;
  reqvire:relationDirection "inverse" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Ontology points to a semantic contract that uses its vocabulary." .

reqvire:verifiedByRelationRule a reqvire:RelationRule ;
  rdfs:label "verifiedBy" ;
  reqvire:relationName "verifiedBy" ;
  reqvire:inverseRelation reqvire:verify ;
  reqvire:relationPattern reqvire:verificationRelationPattern ;
  reqvire:relationFamily reqvire:verificationRelationFamily ;
  reqvire:normalizedForwardProperty reqvire:elementVerifiedByVerification ;
  reqvire:normalizedInverseProperty reqvire:verificationVerifiesElement ;
  reqvire:allowedSourceType "capability", "requirement" ;
  reqvire:allowedTargetType "concrete-verification" ;
  reqvire:relationDirection "forward" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Capability or requirement points to concrete verification evidence that verifies it. Verification objectives are excluded." .

reqvire:verifyRelationRule a reqvire:RelationRule ;
  rdfs:label "verify" ;
  reqvire:relationName "verify" ;
  reqvire:inverseRelation reqvire:verifiedBy ;
  reqvire:relationPattern reqvire:verificationRelationPattern ;
  reqvire:relationFamily reqvire:verificationRelationFamily ;
  reqvire:normalizedForwardProperty reqvire:elementVerifiedByVerification ;
  reqvire:normalizedInverseProperty reqvire:verificationVerifiesElement ;
  reqvire:allowedSourceType "concrete-verification" ;
  reqvire:allowedTargetType "capability", "requirement" ;
  reqvire:relationDirection "inverse" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact false ;
  reqvire:relationRuleDescription "Concrete verification element points to the capability or requirement it verifies. Verification objectives are excluded." .

reqvire:satisfiedByRelationRule a reqvire:RelationRule ;
  rdfs:label "satisfiedBy" ;
  reqvire:relationName "satisfiedBy" ;
  reqvire:inverseRelation reqvire:satisfy ;
  reqvire:relationPattern reqvire:satisfactionRelationPattern ;
  reqvire:relationFamily reqvire:satisfactionRelationFamily ;
  reqvire:normalizedForwardProperty reqvire:elementSatisfiedByArtifact ;
  reqvire:normalizedInverseProperty reqvire:artifactSatisfiesElement ;
  reqvire:allowedSourceType "requirement", "test-verification", "formal-proof-verification" ;
  reqvire:allowedTargetType "internal-path" ;
  reqvire:relationDirection "forward" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Requirement or evidence-backed verification points to implementation or evidence artifacts." .

reqvire:satisfyRelationRule a reqvire:RelationRule ;
  rdfs:label "satisfy" ;
  reqvire:relationName "satisfy" ;
  reqvire:inverseRelation reqvire:satisfiedBy ;
  reqvire:relationPattern reqvire:satisfactionRelationPattern ;
  reqvire:relationFamily reqvire:satisfactionRelationFamily ;
  reqvire:normalizedForwardProperty reqvire:elementSatisfiedByArtifact ;
  reqvire:normalizedInverseProperty reqvire:artifactSatisfiesElement ;
  reqvire:allowedSourceType "internal-path" ;
  reqvire:allowedTargetType "requirement", "test-verification", "formal-proof-verification" ;
  reqvire:relationDirection "inverse" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact false ;
  reqvire:relationRuleDescription "Implementation or evidence artifact points back to the requirement or evidence-backed verification it satisfies." .

reqvire:reusedContractContextRelationRule a reqvire:RelationRule ;
  rdfs:label "reused_contract_context" ;
  reqvire:relationName "reused_contract_context" ;
  reqvire:relationPattern reqvire:crossSubgraphContractDependencyPattern ;
  reqvire:relationFamily reqvire:crossSubgraphContractDependencyRelationFamily ;
  reqvire:normalizedForwardProperty reqvire:requirementUsesCrossSubgraphContract ;
  reqvire:normalizedInverseProperty reqvire:crossSubgraphContractUsedByRequirement ;
  reqvire:allowedSourceType "requirement" ;
  reqvire:allowedTargetType "requirement-owned-contract" ;
  reqvire:relationDirection "forward" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Reused Contract Context is the authoring mechanism for a requirement using compatible requirement-owned contract context from another subgraph." .

reqvire:requirementReusedContractContextCompatibilityRule a reqvire:ReusedContractContextCompatibilityRule ;
  rdfs:label "Requirement reused_contract_context compatibility" ;
  reqvire:reusedContractContextSourceType "requirement" ;
  reqvire:reusedContractContextTargetType "source", "constraint", "behavior", "specification", "state", "input-output" ;
  reqvire:reusedContractContextOwnerType "requirement" ;
  reqvire:reusedContractContextRuleDescription "Requirement reused_contract_context reference requirement-owned contracts from explicit dependency contexts." .

reqvire:ownedContractReusedContractContextRule a reqvire:ReusedContractContextCompatibilityRule ;
  rdfs:label "Owned contract reused_contract_context compatibility" ;
  reqvire:reusedContractContextSourceType "requirement" ;
  reqvire:reusedContractContextTargetType "contract" ;
  reqvire:reusedContractContextRuleDescription "A requirement reused_contract_context target must be a contract already owned by exactly one compatible requirement through define/definedBy." .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---
