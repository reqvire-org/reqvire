# Elements

### Reqvire Change Impact Ontology

The Reqvire change impact ontology defines impact analysis concepts and propagation rules.

Change impact is based on auditable graph paths. Native relations and explicit attachments define reachable context. This ontology defines propagation rule categories and impact semantics.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

reqvire:ChangeImpactAnalysis a owl:Class .
reqvire:ChangeImpactPath a owl:Class .
reqvire:ChangeImpactEdge a owl:Class .
reqvire:SemanticDependency a owl:Class .
reqvire:ImpactReview a owl:Class .
reqvire:ChangePropagationRule a owl:Class .
reqvire:ChangeKind a owl:Class .
reqvire:ImpactClassification a owl:Class .

reqvire:changedElement a owl:ObjectProperty .
reqvire:impactedElement a owl:ObjectProperty .
reqvire:impactPath a owl:ObjectProperty .
reqvire:impactEdge a owl:ObjectProperty .
reqvire:semanticDependency a owl:ObjectProperty .
reqvire:reviewedByVerification a owl:ObjectProperty .

reqvire:impactReason a owl:DatatypeProperty .
reqvire:impactRelation a owl:DatatypeProperty .
reqvire:impactDirection a owl:DatatypeProperty .
reqvire:impactScope a owl:DatatypeProperty .
reqvire:impactSeverity a owl:DatatypeProperty .
reqvire:requiresReview a owl:DatatypeProperty .
reqvire:dependencyResolution a owl:DatatypeProperty .
reqvire:changeRuleName a owl:DatatypeProperty .
reqvire:changedThing a owl:DatatypeProperty .
reqvire:propagationTarget a owl:DatatypeProperty .
reqvire:propagationMode a owl:DatatypeProperty .
reqvire:changeKindName a owl:DatatypeProperty .
reqvire:changeKindMeaning a owl:DatatypeProperty .
reqvire:impactClassificationName a owl:DatatypeProperty .
reqvire:impactClassificationMeaning a owl:DatatypeProperty .

reqvire:contentChangeKind a reqvire:ChangeKind ;
  reqvire:changeKindName "content-change" ;
  reqvire:changeKindMeaning "Same stable element identity exists across compared model states but content hash changed." .
reqvire:additionChangeKind a reqvire:ChangeKind ;
  reqvire:changeKindName "addition" ;
  reqvire:changeKindMeaning "Stable element identity exists only in the newer model state." .
reqvire:removalChangeKind a reqvire:ChangeKind ;
  reqvire:changeKindName "removal" ;
  reqvire:changeKindMeaning "Stable element identity exists only in the older model state." .
reqvire:relocationChangeKind a reqvire:ChangeKind ;
  reqvire:changeKindName "relocation" ;
  reqvire:changeKindMeaning "Stable element identity exists in both model states but location context changed." .
reqvire:attachmentContentChangeKind a reqvire:ChangeKind ;
  reqvire:changeKindName "attachment-content-change" ;
  reqvire:changeKindMeaning "Attached resource or attached element content changed independently of the attaching element text." .

reqvire:directImpactClassification a reqvire:ImpactClassification ;
  reqvire:impactClassificationName "direct" ;
  reqvire:impactClassificationMeaning "Impact caused by a change to the element itself." .
reqvire:indirectImpactClassification a reqvire:ImpactClassification ;
  reqvire:impactClassificationName "indirect" ;
  reqvire:impactClassificationMeaning "Impact propagated through relation, attachment, hierarchy, or semantic dependency context." .
reqvire:potentialImpactClassification a reqvire:ImpactClassification ;
  reqvire:impactClassificationName "potential" ;
  reqvire:impactClassificationMeaning "Impact that may require review based on semantic analysis or indirect dependency evidence." .

reqvire:parentToChildImpactRule a reqvire:ChangePropagationRule ;
  reqvire:changeRuleName "parent-to-child-impact" ;
  reqvire:changedThing "capability-requirement-or-ontology-parent" ;
  reqvire:impactRelation "derive" ;
  reqvire:impactDirection "downstream" ;
  reqvire:propagationTarget "child-capability-requirement-or-ontology" ;
  reqvire:propagationMode "review-required" ;
  reqvire:impactReason "Parent changes can change the meaning or scope of derived child elements." .

reqvire:capabilityToRequirementImpactRule a reqvire:ChangePropagationRule ;
  reqvire:changeRuleName "capability-to-specified-requirement-impact" ;
  reqvire:changedThing "capability" ;
  reqvire:impactRelation "specifiedBy" ;
  reqvire:impactDirection "downstream" ;
  reqvire:propagationTarget "requirement" ;
  reqvire:propagationMode "review-required" ;
  reqvire:impactReason "Capability scope and attached ontology context changes can affect requirements that specify the capability." .

reqvire:requirementToImplementationImpactRule a reqvire:ChangePropagationRule ;
  reqvire:changeRuleName "requirement-to-implementation-impact" ;
  reqvire:changedThing "requirement" ;
  reqvire:impactRelation "satisfiedBy" ;
  reqvire:impactDirection "downstream" ;
  reqvire:propagationTarget "implementation-or-evidence-artifact" ;
  reqvire:propagationMode "review-required" ;
  reqvire:impactReason "Requirement changes may invalidate implementation satisfaction evidence." .

reqvire:requirementToVerificationImpactRule a reqvire:ChangePropagationRule ;
  reqvire:changeRuleName "requirement-to-verification-impact" ;
  reqvire:changedThing "requirement" ;
  reqvire:impactRelation "verifiedBy" ;
  reqvire:impactDirection "downstream" ;
  reqvire:propagationTarget "verification" ;
  reqvire:propagationMode "revalidation-required" ;
  reqvire:impactReason "Requirement changes may invalidate verification scope or evidence." .

reqvire:ownerToRefinementImpactRule a reqvire:ChangePropagationRule ;
  reqvire:changeRuleName "owner-to-refinement-impact" ;
  reqvire:changedThing "capability-or-requirement-owner" ;
  reqvire:impactRelation "refinedBy" ;
  reqvire:impactDirection "downstream" ;
  reqvire:propagationTarget "owned-refinement" ;
  reqvire:propagationMode "review-required" ;
  reqvire:impactReason "Owner changes can change the scope or meaning of owned refinement contracts." .

reqvire:attachmentContentImpactRule a reqvire:ChangePropagationRule ;
  reqvire:changeRuleName "attachment-content-impact" ;
  reqvire:changedThing "attached-refinement-content" ;
  reqvire:impactRelation "attachment" ;
  reqvire:impactDirection "bidirectional" ;
  reqvire:propagationTarget "attaching-element-and-owner-context" ;
  reqvire:propagationMode "review-required" ;
  reqvire:impactReason "Attachment content changes affect explicit cross-subgraph contract consumers." .

reqvire:semanticReferenceReachabilityRule a reqvire:ChangePropagationRule ;
  reqvire:changeRuleName "semantic-reference-reachability" ;
  reqvire:changedThing "semantic-contract-reference" ;
  reqvire:impactRelation "capability-hierarchy-or-attachment" ;
  reqvire:impactDirection "downstream" ;
  reqvire:propagationTarget "referencing-semantic-contract" ;
  reqvire:propagationMode "validation-error-when-unreachable" ;
  reqvire:impactReason "Semantic references must resolve through native capability-root context or explicit attachment so change impact remains auditable." .

reqvire:relocationNoPropagationRule a reqvire:ChangePropagationRule ;
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
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---

### Reqvire Relation Ontology

The Reqvire relation ontology defines Reqvire relation vocabulary.

Relation contracts are separated from element-family contracts because relation behavior is reused across capability, requirement, refinement, verification, artifact, and attachment flows.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

reqvire:RelationRule a owl:Class .
reqvire:RelationConstraint a owl:Class .
reqvire:TraversalRule a owl:Class .
reqvire:AttachmentCompatibilityRule a owl:Class .
reqvire:RelationUsageCategory a owl:Class .
reqvire:RelationSemanticCategory a owl:Class .

reqvire:derive a owl:ObjectProperty .
reqvire:derivedFrom a owl:ObjectProperty .
reqvire:specify a owl:ObjectProperty .
reqvire:specifiedBy a owl:ObjectProperty .
reqvire:refine a owl:ObjectProperty .
reqvire:refinedBy a owl:ObjectProperty .
reqvire:verify a owl:ObjectProperty .
reqvire:verifiedBy a owl:ObjectProperty .
reqvire:satisfy a owl:ObjectProperty .
reqvire:satisfiedBy a owl:ObjectProperty .
reqvire:trace a owl:ObjectProperty .
reqvire:attach a owl:ObjectProperty .
reqvire:attaches a owl:ObjectProperty .

reqvire:inverseRelation a owl:ObjectProperty .
reqvire:relationConstraint a owl:ObjectProperty .
reqvire:relationName a owl:DatatypeProperty .
reqvire:allowedSourceType a owl:DatatypeProperty .
reqvire:allowedTargetType a owl:DatatypeProperty .
reqvire:relationDirection a owl:DatatypeProperty .
reqvire:traversalDirection a owl:DatatypeProperty .
reqvire:createsOwnership a owl:DatatypeProperty .
reqvire:propagatesChangeImpact a owl:DatatypeProperty .
reqvire:relationRuleDescription a owl:DatatypeProperty .
reqvire:attachmentSourceType a owl:DatatypeProperty .
reqvire:attachmentTargetType a owl:DatatypeProperty .
reqvire:attachmentOwnerType a owl:DatatypeProperty .
reqvire:attachmentRuleDescription a owl:DatatypeProperty .
reqvire:usageCategoryName a owl:DatatypeProperty .
reqvire:usageCategoryMeaning a owl:DatatypeProperty .
reqvire:usageCategoryRelationName a owl:DatatypeProperty .
reqvire:semanticCategoryName a owl:DatatypeProperty .
reqvire:semanticCategoryMeaning a owl:DatatypeProperty .
reqvire:semanticCategoryRelationName a owl:DatatypeProperty .

reqvire:diagramRenderingRelationUsageCategory a reqvire:RelationUsageCategory ;
  reqvire:usageCategoryName "diagram-rendering-forward" ;
  reqvire:usageCategoryMeaning "Forward relation names rendered in diagrams and root-to-leaf model views to avoid duplicate inverse arrows." ;
  reqvire:usageCategoryRelationName "derive", "specifiedBy", "satisfiedBy", "refinedBy", "verifiedBy", "trace" .
reqvire:reverseTraversalRelationUsageCategory a reqvire:RelationUsageCategory ;
  reqvire:usageCategoryName "reverse-traversal" ;
  reqvire:usageCategoryMeaning "Inverse relation names used for leaf-to-root traversal and upward traceability." ;
  reqvire:usageCategoryRelationName "derivedFrom", "specify", "satisfy", "refine", "verify" .
reqvire:changePropagationRelationUsageCategory a reqvire:RelationUsageCategory ;
  reqvire:usageCategoryName "change-propagation" ;
  reqvire:usageCategoryMeaning "Relation names through which changed upstream model meaning propagates to dependent downstream elements." ;
  reqvire:usageCategoryRelationName "derive", "specifiedBy", "satisfiedBy", "refinedBy", "verifiedBy", "attachment" .
reqvire:verificationRollupRelationUsageCategory a reqvire:RelationUsageCategory ;
  reqvire:usageCategoryName "verification-rollup" ;
  reqvire:usageCategoryMeaning "Relation names used to roll verification state from requirement leaves toward ancestors." ;
  reqvire:usageCategoryRelationName "derivedFrom", "verify", "verifiedBy" .

reqvire:hierarchyRelationCategory a reqvire:RelationSemanticCategory ;
  reqvire:semanticCategoryName "hierarchy" ;
  reqvire:semanticCategoryMeaning "Same-family capability, requirement, or ontology derivation hierarchy." ;
  reqvire:semanticCategoryRelationName "derive", "derivedFrom" .
reqvire:capabilitySpecificationRelationCategory a reqvire:RelationSemanticCategory ;
  reqvire:semanticCategoryName "capability-specification" ;
  reqvire:semanticCategoryMeaning "Bridge between capability elements and requirements that specify them." ;
  reqvire:semanticCategoryRelationName "specify", "specifiedBy" .
reqvire:satisfactionRelationCategory a reqvire:RelationSemanticCategory ;
  reqvire:semanticCategoryName "satisfaction" ;
  reqvire:semanticCategoryMeaning "Links from requirements or evidence-backed verifications to implementation or evidence artifacts." ;
  reqvire:semanticCategoryRelationName "satisfy", "satisfiedBy" .
reqvire:refinementRelationCategory a reqvire:RelationSemanticCategory ;
  reqvire:semanticCategoryName "refinement-ownership" ;
  reqvire:semanticCategoryMeaning "Ownership of capability-owned and requirement-owned refinements." ;
  reqvire:semanticCategoryRelationName "refine", "refinedBy" .
reqvire:verificationRelationCategory a reqvire:RelationSemanticCategory ;
  reqvire:semanticCategoryName "verification" ;
  reqvire:semanticCategoryMeaning "Links between capabilities or requirements and verification elements that verify them." ;
  reqvire:semanticCategoryRelationName "verify", "verifiedBy" .
reqvire:traceabilityRelationCategory a reqvire:RelationSemanticCategory ;
  reqvire:semanticCategoryName "traceability" ;
  reqvire:semanticCategoryMeaning "Lightweight documentation/discovery relation without ownership or change propagation." ;
  reqvire:semanticCategoryRelationName "trace" .
reqvire:attachmentRelationCategory a reqvire:RelationSemanticCategory ;
  reqvire:semanticCategoryName "attachment-dependency" ;
  reqvire:semanticCategoryMeaning "Explicit dependency from a capability to ontology context or from a requirement to a reusable requirement-owned contract." ;
  reqvire:semanticCategoryRelationName "attachment" .

reqvire:deriveRelationRule a reqvire:RelationRule ;
  reqvire:relationName "derive" ;
  reqvire:inverseRelation reqvire:derivedFrom ;
  reqvire:allowedSourceType "capability", "requirement", "ontology" ;
  reqvire:allowedTargetType "same-hierarchy-family" ;
  reqvire:relationDirection "forward" ;
  reqvire:createsOwnership true ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Parent capability, requirement, or ontology derives child elements only within the same hierarchy family." .

reqvire:derivedFromRelationRule a reqvire:RelationRule ;
  reqvire:relationName "derivedFrom" ;
  reqvire:inverseRelation reqvire:derive ;
  reqvire:allowedSourceType "capability", "requirement", "ontology" ;
  reqvire:allowedTargetType "same-hierarchy-family" ;
  reqvire:relationDirection "inverse" ;
  reqvire:createsOwnership true ;
  reqvire:propagatesChangeImpact false ;
  reqvire:relationRuleDescription "Child capability, requirement, or ontology points to its parent in the same hierarchy family." .

reqvire:specifiedByRelationRule a reqvire:RelationRule ;
  reqvire:relationName "specifiedBy" ;
  reqvire:inverseRelation reqvire:specify ;
  reqvire:allowedSourceType "capability" ;
  reqvire:allowedTargetType "requirement" ;
  reqvire:relationDirection "forward" ;
  reqvire:createsOwnership true ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Capability points to a requirement that specifies the capability." .

reqvire:specifyRelationRule a reqvire:RelationRule ;
  reqvire:relationName "specify" ;
  reqvire:inverseRelation reqvire:specifiedBy ;
  reqvire:allowedSourceType "requirement" ;
  reqvire:allowedTargetType "capability" ;
  reqvire:relationDirection "inverse" ;
  reqvire:createsOwnership true ;
  reqvire:propagatesChangeImpact false ;
  reqvire:relationRuleDescription "Requirement points to the capability it specifies." .

reqvire:refinedByRelationRule a reqvire:RelationRule ;
  reqvire:relationName "refinedBy" ;
  reqvire:inverseRelation reqvire:refine ;
  reqvire:allowedSourceType "capability", "requirement" ;
  reqvire:allowedTargetType "subtype-compatible-refinement" ;
  reqvire:relationDirection "forward" ;
  reqvire:createsOwnership true ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Capability or requirement owns a subtype-compatible refinement element." .

reqvire:refineRelationRule a reqvire:RelationRule ;
  reqvire:relationName "refine" ;
  reqvire:inverseRelation reqvire:refinedBy ;
  reqvire:allowedSourceType "refinement" ;
  reqvire:allowedTargetType "capability-or-requirement-owner" ;
  reqvire:relationDirection "inverse" ;
  reqvire:createsOwnership true ;
  reqvire:propagatesChangeImpact false ;
  reqvire:relationRuleDescription "Refinement element points to its single valid owner." .

reqvire:verifiedByRelationRule a reqvire:RelationRule ;
  reqvire:relationName "verifiedBy" ;
  reqvire:inverseRelation reqvire:verify ;
  reqvire:allowedSourceType "capability", "requirement" ;
  reqvire:allowedTargetType "verification" ;
  reqvire:relationDirection "forward" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Capability or requirement points to verification evidence that verifies it." .

reqvire:verifyRelationRule a reqvire:RelationRule ;
  reqvire:relationName "verify" ;
  reqvire:inverseRelation reqvire:verifiedBy ;
  reqvire:allowedSourceType "verification" ;
  reqvire:allowedTargetType "capability", "requirement" ;
  reqvire:relationDirection "inverse" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact false ;
  reqvire:relationRuleDescription "Verification element points to the capability or requirement it verifies." .

reqvire:satisfiedByRelationRule a reqvire:RelationRule ;
  reqvire:relationName "satisfiedBy" ;
  reqvire:inverseRelation reqvire:satisfy ;
  reqvire:allowedSourceType "requirement", "test-verification", "formal-proof-verification" ;
  reqvire:allowedTargetType "internal-path" ;
  reqvire:relationDirection "forward" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Requirement or evidence-backed verification points to implementation or evidence artifacts." .

reqvire:satisfyRelationRule a reqvire:RelationRule ;
  reqvire:relationName "satisfy" ;
  reqvire:inverseRelation reqvire:satisfiedBy ;
  reqvire:allowedSourceType "internal-path" ;
  reqvire:allowedTargetType "requirement", "test-verification", "formal-proof-verification" ;
  reqvire:relationDirection "inverse" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact false ;
  reqvire:relationRuleDescription "Implementation or evidence artifact points back to the requirement or evidence-backed verification it satisfies." .

reqvire:traceRelationRule a reqvire:RelationRule ;
  reqvire:relationName "trace" ;
  reqvire:allowedSourceType "any-non-refinement" ;
  reqvire:allowedTargetType "any" ;
  reqvire:relationDirection "non-directional" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact false ;
  reqvire:relationRuleDescription "Trace is a lightweight documentation relation without ownership or change propagation semantics." .

reqvire:attachmentRelationRule a reqvire:RelationRule ;
  reqvire:relationName "attachment" ;
  reqvire:allowedSourceType "capability", "requirement" ;
  reqvire:allowedTargetType "capability-attached-ontology-or-requirement-owned-refinement" ;
  reqvire:relationDirection "forward" ;
  reqvire:createsOwnership false ;
  reqvire:propagatesChangeImpact true ;
  reqvire:relationRuleDescription "Attachment references capability-owned ontology context or a compatible requirement-owned refinement contract across explicit subgraph boundaries." .

reqvire:capabilityAttachmentCompatibilityRule a reqvire:AttachmentCompatibilityRule ;
  reqvire:attachmentSourceType "capability" ;
  reqvire:attachmentTargetType "ontology" ;
  reqvire:attachmentOwnerType "capability" ;
  reqvire:attachmentRuleDescription "Capability attachments reference ontology elements from explicit capability-root dependency contexts." .

reqvire:requirementAttachmentCompatibilityRule a reqvire:AttachmentCompatibilityRule ;
  reqvire:attachmentSourceType "requirement" ;
  reqvire:attachmentTargetType "semantic-contract", "constraint", "behavior", "specification", "state", "input-output" ;
  reqvire:attachmentOwnerType "requirement" ;
  reqvire:attachmentRuleDescription "Requirement attachments reference requirement-owned refinements from explicit dependency contexts." .

reqvire:ownedRefinementAttachmentRule a reqvire:AttachmentCompatibilityRule ;
  reqvire:attachmentSourceType "requirement" ;
  reqvire:attachmentTargetType "refinement" ;
  reqvire:attachmentRuleDescription "A requirement attachment target must be a refinement already owned by exactly one compatible requirement through refine/refinedBy." .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---
