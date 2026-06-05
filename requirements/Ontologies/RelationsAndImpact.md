# Elements

### Reqvire Change Impact Ontology

The Reqvire change impact ontology defines impact analysis concepts and propagation rules.

Change impact is based on auditable graph paths. Native relations and explicit attachments define reachable context. This ontology defines propagation rule categories and impact semantics.

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
  rdfs:comment "Auditable path that explains how impact propagates through model relations, attachments, or semantic dependencies." .
reqvire:ChangeImpactEdge a owl:Class ;
  rdfs:label "Change impact edge" ;
  rdfs:comment "Single propagation step in a change-impact path." .
reqvire:SemanticDependency a owl:Class ;
  rdfs:label "Semantic dependency" ;
  rdfs:comment "Resolved dependency from a semantic contract to ontology terms reachable through native context or attachment context." .
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
  rdfs:comment "Element whose identity, content, location, or attached content changed." .
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
  rdfs:comment "Resolution token describing whether a semantic dependency was reachable, attached, missing, or outside context." .
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
reqvire:attachmentContentChangeKind a reqvire:ChangeKind ;
  rdfs:label "Attachment content change" ;
  rdfs:comment "Attached resource or attached element content changed independently of the attaching element text." ;
  reqvire:changeKindName "attachment-content-change" ;
  reqvire:changeKindMeaning "Attached resource or attached element content changed independently of the attaching element text." .

reqvire:directImpactClassification a reqvire:ImpactClassification ;
  rdfs:label "Direct impact" ;
  rdfs:comment "Impact caused by a change to the element itself." ;
  reqvire:impactClassificationName "direct" ;
  reqvire:impactClassificationMeaning "Impact caused by a change to the element itself." .
reqvire:indirectImpactClassification a reqvire:ImpactClassification ;
  rdfs:label "Indirect impact" ;
  rdfs:comment "Impact propagated through relation, attachment, hierarchy, or semantic dependency context." ;
  reqvire:impactClassificationName "indirect" ;
  reqvire:impactClassificationMeaning "Impact propagated through relation, attachment, hierarchy, or semantic dependency context." .
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
  reqvire:impactReason "Capability scope and attached ontology context changes can affect requirements that specify the capability." .

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

reqvire:ownerToRefinementImpactRule a reqvire:ChangePropagationRule ;
  rdfs:label "Owner to refinement impact" ;
  reqvire:changeRuleName "owner-to-refinement-impact" ;
  reqvire:changedThing "capability-or-requirement-owner" ;
  reqvire:impactRelation "refinedBy" ;
  reqvire:impactDirection "downstream" ;
  reqvire:propagationTarget "owned-refinement" ;
  reqvire:propagationMode "review-required" ;
  reqvire:impactReason "Owner changes can change the scope or meaning of owned refinement contracts." .

reqvire:attachmentContentImpactRule a reqvire:ChangePropagationRule ;
  rdfs:label "Attachment content impact" ;
  reqvire:changeRuleName "attachment-content-impact" ;
  reqvire:changedThing "attached-refinement-content" ;
  reqvire:impactRelation "attachment" ;
  reqvire:impactDirection "bidirectional" ;
  reqvire:propagationTarget "attaching-element-and-owner-context" ;
  reqvire:propagationMode "review-required" ;
  reqvire:impactReason "Attachment content changes affect explicit cross-subgraph contract consumers." .

reqvire:semanticReferenceReachabilityRule a reqvire:ChangePropagationRule ;
  rdfs:label "Semantic reference reachability" ;
  reqvire:changeRuleName "semantic-reference-reachability" ;
  reqvire:changedThing "semantic-contract-reference" ;
  reqvire:impactRelation "capability-hierarchy-or-attachment" ;
  reqvire:impactDirection "downstream" ;
  reqvire:propagationTarget "referencing-semantic-contract" ;
  reqvire:propagationMode "validation-error-when-unreachable" ;
  reqvire:impactReason "Semantic references must resolve through native capability-root context or explicit attachment so change impact remains auditable." .

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

Relation contracts are separated from element-family contracts because relation behavior is reused across capability, requirement, refinement, verification, artifact, and attachment flows.

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
reqvire:AttachmentCompatibilityRule a owl:Class ;
  rdfs:subClassOf reqvire:RelationConstraint ;
  rdfs:label "Attachment compatibility rule" ;
  rdfs:comment "Constraint defining valid attachment source, target, and owner compatibility." .
reqvire:RelationUsageCategory a owl:Class ;
  rdfs:label "Relation usage category" ;
  rdfs:comment "Controlled category grouping relation names by operational usage such as rendering, traversal, change propagation, or rollup." .
reqvire:RelationSemanticCategory a owl:Class ;
  rdfs:label "Relation semantic category" ;
  rdfs:comment "Controlled category grouping relation names by their model semantics." .

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
reqvire:refine a owl:ObjectProperty ;
  rdfs:domain reqvire:Refinement ;
  rdfs:range [ a owl:Class ; owl:unionOf (reqvire:Capability reqvire:Requirement) ] ;
  owl:inverseOf reqvire:refinedBy ;
  rdfs:comment "Inverse ownership relation from a refinement to its capability or requirement owner." .
reqvire:refinedBy a owl:ObjectProperty ;
  rdfs:domain [ a owl:Class ; owl:unionOf (reqvire:Capability reqvire:Requirement) ] ;
  rdfs:range reqvire:Refinement ;
  owl:inverseOf reqvire:refine ;
  rdfs:comment "Forward ownership relation from a capability or requirement to an owned refinement." .
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
reqvire:trace a owl:ObjectProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range reqvire:Element ;
  rdfs:comment "Non-directional traceability relation without ownership or propagation semantics." .
reqvire:attach a owl:ObjectProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range reqvire:Element ;
  owl:inverseOf reqvire:attaches ;
  rdfs:comment "Inverse attachment relation from an attached ontology or reusable contract back to its consuming element." .
reqvire:attaches a owl:ObjectProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range reqvire:Element ;
  owl:inverseOf reqvire:attach ;
  rdfs:comment "Forward attachment relation from a capability or requirement to explicit ontology or reusable contract context." .
reqvire:implementedByArtifact a owl:ObjectProperty ;
  rdfs:domain reqvire:Capability ;
  rdfs:range reqvire:Artifact ;
  owl:propertyChainAxiom (reqvire:specifiedBy reqvire:satisfiedBy) ;
  rdfs:comment "Inferred capability-to-artifact trace when a capability is specified by a requirement satisfied by an artifact." .

reqvire:inverseRelation a owl:ObjectProperty ;
  rdfs:domain reqvire:RelationRule ;
  rdfs:range owl:ObjectProperty ;
  rdfs:comment "Object property that is the true inverse of the relation described by a relation rule." .
reqvire:relationConstraint a owl:ObjectProperty ;
  rdfs:domain reqvire:RelationRule ;
  rdfs:range reqvire:RelationConstraint ;
  rdfs:comment "Constraint associated with a relation rule." .
reqvire:relationName a owl:DatatypeProperty ;
  rdfs:domain reqvire:RelationRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical authored relation name token used in Markdown metadata, CLI output, reports, validation, and queries." .
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
reqvire:attachmentSourceType a owl:DatatypeProperty ;
  rdfs:domain reqvire:AttachmentCompatibilityRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical element type token allowed as an attachment source." .
reqvire:attachmentTargetType a owl:DatatypeProperty ;
  rdfs:domain reqvire:AttachmentCompatibilityRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical element type or refinement-family token allowed as an attachment target." .
reqvire:attachmentOwnerType a owl:DatatypeProperty ;
  rdfs:domain reqvire:AttachmentCompatibilityRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical owner element type required for the attachment target." .
reqvire:attachmentRuleDescription a owl:DatatypeProperty ;
  rdfs:domain reqvire:AttachmentCompatibilityRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable semantic description of an attachment compatibility rule." .
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

reqvire:diagramRenderingRelationUsageCategory a reqvire:RelationUsageCategory ;
  rdfs:label "Diagram rendering forward relations" ;
  rdfs:comment "Forward relation names rendered in diagrams and root-to-leaf model views to avoid duplicate inverse arrows." ;
  reqvire:usageCategoryName "diagram-rendering-forward" ;
  reqvire:usageCategoryMeaning "Forward relation names rendered in diagrams and root-to-leaf model views to avoid duplicate inverse arrows." ;
  reqvire:usageCategoryRelationName "derive", "specifiedBy", "satisfiedBy", "refinedBy", "verifiedBy", "trace" .
reqvire:reverseTraversalRelationUsageCategory a reqvire:RelationUsageCategory ;
  rdfs:label "Reverse traversal relations" ;
  rdfs:comment "Inverse relation names used for leaf-to-root traversal and upward traceability." ;
  reqvire:usageCategoryName "reverse-traversal" ;
  reqvire:usageCategoryMeaning "Inverse relation names used for leaf-to-root traversal and upward traceability." ;
  reqvire:usageCategoryRelationName "derivedFrom", "specify", "satisfy", "refine", "verify" .
reqvire:changePropagationRelationUsageCategory a reqvire:RelationUsageCategory ;
  rdfs:label "Change propagation relations" ;
  rdfs:comment "Relation names through which changed upstream model meaning propagates to dependent downstream elements." ;
  reqvire:usageCategoryName "change-propagation" ;
  reqvire:usageCategoryMeaning "Relation names through which changed upstream model meaning propagates to dependent downstream elements." ;
  reqvire:usageCategoryRelationName "derive", "specifiedBy", "satisfiedBy", "refinedBy", "verifiedBy", "attachment" .
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
reqvire:refinementRelationCategory a reqvire:RelationSemanticCategory ;
  rdfs:label "Refinement ownership relations" ;
  rdfs:comment "Ownership of capability-owned and requirement-owned refinements." ;
  reqvire:semanticCategoryName "refinement-ownership" ;
  reqvire:semanticCategoryMeaning "Ownership of capability-owned and requirement-owned refinements." ;
  reqvire:semanticCategoryRelationName "refine", "refinedBy" .
reqvire:verificationRelationCategory a reqvire:RelationSemanticCategory ;
  rdfs:label "Verification relations" ;
  rdfs:comment "Links between capabilities or requirements and verification elements that verify them." ;
  reqvire:semanticCategoryName "verification" ;
  reqvire:semanticCategoryMeaning "Links between capabilities or requirements and verification elements that verify them." ;
  reqvire:semanticCategoryRelationName "verify", "verifiedBy" .
reqvire:traceabilityRelationCategory a reqvire:RelationSemanticCategory ;
  rdfs:label "Traceability relation" ;
  rdfs:comment "Lightweight documentation/discovery relation without ownership or change propagation." ;
  reqvire:semanticCategoryName "traceability" ;
  reqvire:semanticCategoryMeaning "Lightweight documentation/discovery relation without ownership or change propagation." ;
  reqvire:semanticCategoryRelationName "trace" .
reqvire:attachmentRelationCategory a reqvire:RelationSemanticCategory ;
  rdfs:label "Attachment dependency relation" ;
  rdfs:comment "Explicit dependency from a capability to ontology context or from a requirement to a reusable requirement-owned contract." ;
  reqvire:semanticCategoryName "attachment-dependency" ;
  reqvire:semanticCategoryMeaning "Explicit dependency from a capability to ontology context or from a requirement to a reusable requirement-owned contract." ;
  reqvire:semanticCategoryRelationName "attachment" .

reqvire:deriveRelationRule a reqvire:RelationRule ;
  rdfs:label "derive" ;
  reqvire:relationName "derive" ;
  reqvire:inverseRelation reqvire:derivedFrom ;
  reqvire:allowedSourceType "capability", "requirement", "ontology" ;
  reqvire:allowedTargetType "same-hierarchy-family" ;
  reqvire:relationDirection "forward" ;
  reqvire:createsOwnership true ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Parent capability, requirement, or ontology derives child elements only within the same hierarchy family." .

reqvire:derivedFromRelationRule a reqvire:RelationRule ;
  rdfs:label "derivedFrom" ;
  reqvire:relationName "derivedFrom" ;
  reqvire:inverseRelation reqvire:derive ;
  reqvire:allowedSourceType "capability", "requirement", "ontology" ;
  reqvire:allowedTargetType "same-hierarchy-family" ;
  reqvire:relationDirection "inverse" ;
  reqvire:createsOwnership true ;
  reqvire:propagatesChangeImpact false ;
  reqvire:relationRuleDescription "Child capability, requirement, or ontology points to its parent in the same hierarchy family." .

reqvire:specifiedByRelationRule a reqvire:RelationRule ;
  rdfs:label "specifiedBy" ;
  reqvire:relationName "specifiedBy" ;
  reqvire:inverseRelation reqvire:specify ;
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
  reqvire:allowedSourceType "requirement" ;
  reqvire:allowedTargetType "capability" ;
  reqvire:relationDirection "inverse" ;
  reqvire:createsOwnership true ;
  reqvire:propagatesChangeImpact false ;
  reqvire:relationRuleDescription "Requirement points to the capability it specifies." .

reqvire:refinedByRelationRule a reqvire:RelationRule ;
  rdfs:label "refinedBy" ;
  reqvire:relationName "refinedBy" ;
  reqvire:inverseRelation reqvire:refine ;
  reqvire:allowedSourceType "capability", "requirement" ;
  reqvire:allowedTargetType "subtype-compatible-refinement" ;
  reqvire:relationDirection "forward" ;
  reqvire:createsOwnership true ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Capability or requirement owns a subtype-compatible refinement element." .

reqvire:refineRelationRule a reqvire:RelationRule ;
  rdfs:label "refine" ;
  reqvire:relationName "refine" ;
  reqvire:inverseRelation reqvire:refinedBy ;
  reqvire:allowedSourceType "refinement" ;
  reqvire:allowedTargetType "capability-or-requirement-owner" ;
  reqvire:relationDirection "inverse" ;
  reqvire:createsOwnership true ;
  reqvire:propagatesChangeImpact false ;
  reqvire:relationRuleDescription "Refinement element points to its single valid owner." .

reqvire:verifiedByRelationRule a reqvire:RelationRule ;
  rdfs:label "verifiedBy" ;
  reqvire:relationName "verifiedBy" ;
  reqvire:inverseRelation reqvire:verify ;
  reqvire:allowedSourceType "capability", "requirement" ;
  reqvire:allowedTargetType "verification" ;
  reqvire:relationDirection "forward" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Capability or requirement points to verification evidence that verifies it." .

reqvire:verifyRelationRule a reqvire:RelationRule ;
  rdfs:label "verify" ;
  reqvire:relationName "verify" ;
  reqvire:inverseRelation reqvire:verifiedBy ;
  reqvire:allowedSourceType "verification" ;
  reqvire:allowedTargetType "capability", "requirement" ;
  reqvire:relationDirection "inverse" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact false ;
  reqvire:relationRuleDescription "Verification element points to the capability or requirement it verifies." .

reqvire:satisfiedByRelationRule a reqvire:RelationRule ;
  rdfs:label "satisfiedBy" ;
  reqvire:relationName "satisfiedBy" ;
  reqvire:inverseRelation reqvire:satisfy ;
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
  reqvire:allowedSourceType "internal-path" ;
  reqvire:allowedTargetType "requirement", "test-verification", "formal-proof-verification" ;
  reqvire:relationDirection "inverse" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact false ;
  reqvire:relationRuleDescription "Implementation or evidence artifact points back to the requirement or evidence-backed verification it satisfies." .

reqvire:traceRelationRule a reqvire:RelationRule ;
  rdfs:label "trace" ;
  reqvire:relationName "trace" ;
  reqvire:allowedSourceType "any-non-refinement" ;
  reqvire:allowedTargetType "any" ;
  reqvire:relationDirection "non-directional" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact false ;
  reqvire:relationRuleDescription "Trace is a lightweight documentation relation without ownership or change propagation semantics." .

reqvire:attachmentRelationRule a reqvire:RelationRule ;
  rdfs:label "attachment" ;
  reqvire:relationName "attachment" ;
  reqvire:allowedSourceType "capability", "requirement" ;
  reqvire:allowedTargetType "capability-attached-ontology-or-requirement-owned-refinement" ;
  reqvire:relationDirection "forward" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Attachment references capability-owned ontology context or a compatible requirement-owned refinement contract across explicit subgraph boundaries." .

reqvire:capabilityAttachmentCompatibilityRule a reqvire:AttachmentCompatibilityRule ;
  rdfs:label "Capability attachment compatibility" ;
  reqvire:attachmentSourceType "capability" ;
  reqvire:attachmentTargetType "ontology" ;
  reqvire:attachmentOwnerType "capability" ;
  reqvire:attachmentRuleDescription "Capability attachments reference ontology elements from explicit capability-root dependency contexts." .

reqvire:requirementAttachmentCompatibilityRule a reqvire:AttachmentCompatibilityRule ;
  rdfs:label "Requirement attachment compatibility" ;
  reqvire:attachmentSourceType "requirement" ;
  reqvire:attachmentTargetType "semantic-contract", "semantic-query-contract", "constraint", "behavior", "specification", "state", "input-output" ;
  reqvire:attachmentOwnerType "requirement" ;
  reqvire:attachmentRuleDescription "Requirement attachments reference requirement-owned refinements from explicit dependency contexts." .

reqvire:ownedRefinementAttachmentRule a reqvire:AttachmentCompatibilityRule ;
  rdfs:label "Owned refinement attachment compatibility" ;
  reqvire:attachmentSourceType "requirement" ;
  reqvire:attachmentTargetType "refinement" ;
  reqvire:attachmentRuleDescription "A requirement attachment target must be a refinement already owned by exactly one compatible requirement through refine/refinedBy." .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---
