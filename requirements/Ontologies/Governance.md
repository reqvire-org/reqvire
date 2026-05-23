# Elements

# Governance

### Reqvire Governance Ontology

The Reqvire governance ontology defines vocabulary for lifecycle, priority, risk, and owner metadata used for planning and routing.

Governance metadata is part of the Reqvire semantic model because it changes planning, ownership routing, and effective context even when it does not change implementation behavior. This ontology defines governance authoring categories, inheritance semantics, effective-value semantics, and persistence semantics.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

reqvire:GovernanceRule a owl:Class .
reqvire:GovernanceMetadata a owl:Class .
reqvire:GovernanceValue a owl:Class .
reqvire:StatusValue a owl:Class ;
  rdfs:subClassOf reqvire:GovernanceValue .
reqvire:PriorityValue a owl:Class ;
  rdfs:subClassOf reqvire:GovernanceValue .
reqvire:RiskValue a owl:Class ;
  rdfs:subClassOf reqvire:GovernanceValue .

reqvire:status a owl:DatatypeProperty .
reqvire:priority a owl:DatatypeProperty .
reqvire:risk a owl:DatatypeProperty .
reqvire:owner a owl:DatatypeProperty .
reqvire:governanceValueName a owl:DatatypeProperty .
reqvire:governanceValueMeaning a owl:DatatypeProperty .
reqvire:governanceDefaultValue a owl:DatatypeProperty .
reqvire:governanceRuleName a owl:DatatypeProperty .
reqvire:governanceAppliesTo a owl:DatatypeProperty .
reqvire:governanceSourceOrder a owl:DatatypeProperty .
reqvire:governancePersistence a owl:DatatypeProperty .

reqvire:draftStatus a reqvire:StatusValue ;
  reqvire:governanceValueName "draft" ;
  reqvire:governanceValueMeaning "The element is being authored or revised and is not ready for formal review." ;
  reqvire:governanceDefaultValue false .
reqvire:reviewStatus a reqvire:StatusValue ;
  reqvire:governanceValueName "review" ;
  reqvire:governanceValueMeaning "The element is ready for, or currently under, stakeholder or engineering review." ;
  reqvire:governanceDefaultValue false .
reqvire:approvedStatus a reqvire:StatusValue ;
  reqvire:governanceValueName "approved" ;
  reqvire:governanceValueMeaning "The element definition has completed review and is accepted as authoritative for downstream work." ;
  reqvire:governanceDefaultValue true .

reqvire:lowPriority a reqvire:PriorityValue ;
  reqvire:governanceValueName "low" ;
  reqvire:governanceValueMeaning "Useful or desirable, but deferrable without major mission, stakeholder, or integration impact." ;
  reqvire:governanceDefaultValue false .
reqvire:mediumPriority a reqvire:PriorityValue ;
  reqvire:governanceValueName "medium" ;
  reqvire:governanceValueMeaning "Normal planning importance; expected to be delivered unless schedule, cost, or scope tradeoffs require adjustment." ;
  reqvire:governanceDefaultValue true .
reqvire:highPriority a reqvire:PriorityValue ;
  reqvire:governanceValueName "high" ;
  reqvire:governanceValueMeaning "Important to mission, stakeholder value, integration, or compliance and should be protected during tradeoffs." ;
  reqvire:governanceDefaultValue false .
reqvire:criticalPriority a reqvire:PriorityValue ;
  reqvire:governanceValueName "critical" ;
  reqvire:governanceValueMeaning "Essential; failure to satisfy creates unacceptable mission, safety, compliance, contractual, or release impact." ;
  reqvire:governanceDefaultValue false .

reqvire:lowRisk a reqvire:RiskValue ;
  reqvire:governanceValueName "low" ;
  reqvire:governanceValueMeaning "Requirement realization is well understood, stable, feasible, and straightforward to verify." ;
  reqvire:governanceDefaultValue true .
reqvire:mediumRisk a reqvire:RiskValue ;
  reqvire:governanceValueName "medium" ;
  reqvire:governanceValueMeaning "Requirement realization has manageable uncertainty, moderate implementation or verification complexity, or limited downstream coupling." ;
  reqvire:governanceDefaultValue false .
reqvire:highRisk a reqvire:RiskValue ;
  reqvire:governanceValueName "high" ;
  reqvire:governanceValueMeaning "Requirement realization has significant technical uncertainty, volatility, verification difficulty, integration exposure, or likely downstream rework." ;
  reqvire:governanceDefaultValue false .
reqvire:criticalRisk a reqvire:RiskValue ;
  reqvire:governanceValueName "critical" ;
  reqvire:governanceValueMeaning "Requirement realization has severe uncertainty or exposure where failure, change, or non-compliance may materially affect mission, safety, compliance, cost, or schedule." ;
  reqvire:governanceDefaultValue false .

reqvire:governanceAuthoringRule a reqvire:GovernanceRule ;
  reqvire:governanceRuleName "governance-authoring" ;
  reqvire:governanceAppliesTo "feature, requirement" ;
  reqvire:ruleCondition "An element declares status, priority, risk, or owner metadata." ;
  reqvire:ruleOutcome "Only feature and requirement elements may author governance metadata; refinements and verifications obtain context from their owner or linked requirement." .

reqvire:governanceEffectiveValueRule a reqvire:GovernanceRule ;
  reqvire:governanceRuleName "governance-effective-value" ;
  reqvire:governanceAppliesTo "feature, requirement, refinement, verification" ;
  reqvire:governanceSourceOrder "explicit, inherited, default" ;
  reqvire:ruleCondition "A consumer requests effective governance metadata." ;
  reqvire:ruleOutcome "Return value and source for status, priority, risk, and owner; inherited values include source_identifier; defaults omit source_identifier." .

reqvire:governancePersistenceRule a reqvire:GovernanceRule ;
  reqvire:governanceRuleName "governance-persistence" ;
  reqvire:governancePersistence "explicit-only" ;
  reqvire:ruleCondition "Formatting or mutation rewrites a model file." ;
  reqvire:ruleOutcome "Persist only explicit governance metadata authored or changed by the user; never materialize inherited or default governance metadata into Markdown." .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---

