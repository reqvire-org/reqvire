# Elements

### Governance Metadata Scope Shape

Defines SHACL constraints for governance metadata on capability and requirement elements.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:CapabilityGovernanceShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Capability ;
  sh:property [
    sh:path reqvire:status ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("draft" "review" "approved") ;
    sh:message "Capability status metadata must be one of draft, review, or approved." ;
  ] ;
  sh:property [
    sh:path reqvire:priority ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("low" "medium" "high" "critical") ;
    sh:message "Capability priority metadata must be one of low, medium, high, or critical." ;
  ] ;
  sh:property [
    sh:path reqvire:risk ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("low" "medium" "high" "critical") ;
    sh:message "Capability risk metadata must be one of low, medium, high, or critical." ;
  ] ;
  sh:property [
    sh:path reqvire:owner ;
    sh:datatype xsd:string ;
    sh:maxCount 1 ;
    sh:message "Capability owner metadata must be a single string routing label." ;
  ] .

reqvire:RequirementGovernanceShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Requirement ;
  sh:property [
    sh:path reqvire:status ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("draft" "review" "approved") ;
    sh:message "Requirement status metadata must be one of draft, review, or approved." ;
  ] ;
  sh:property [
    sh:path reqvire:priority ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("low" "medium" "high" "critical") ;
    sh:message "Requirement priority metadata must be one of low, medium, high, or critical." ;
  ] ;
  sh:property [
    sh:path reqvire:risk ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("low" "medium" "high" "critical") ;
    sh:message "Requirement risk metadata must be one of low, medium, high, or critical." ;
  ] ;
  sh:property [
    sh:path reqvire:owner ;
    sh:datatype xsd:string ;
    sh:maxCount 1 ;
    sh:message "Requirement owner metadata must be a single string routing label." ;
  ] .

reqvire:NonGovernanceElementShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Refinement, reqvire:Verification, reqvire:Ontology, reqvire:CustomElement ;
  sh:property [
    sh:path reqvire:status ;
    sh:maxCount 0 ;
    sh:message "Governance status metadata may be authored only by capability or requirement elements." ;
  ] ;
  sh:property [
    sh:path reqvire:priority ;
    sh:maxCount 0 ;
    sh:message "Governance priority metadata may be authored only by capability or requirement elements." ;
  ] ;
  sh:property [
    sh:path reqvire:risk ;
    sh:maxCount 0 ;
    sh:message "Governance risk metadata may be authored only by capability or requirement elements." ;
  ] ;
  sh:property [
    sh:path reqvire:owner ;
    sh:maxCount 0 ;
    sh:message "Governance owner metadata may be authored only by capability or requirement elements." ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Ontology and Semantic Contract Model](../ModelStructure/ModelManagement.md#ontology-and-semantic-contract-model)
  * use: [Reqvire Semantic Contract Ontology](CapabilityRequirementModel.md#reqvire-semantic-contract-ontology)
  * use: [Reqvire Governance Ontology](#reqvire-governance-ontology)
---

### Reqvire Governance Ontology

The Reqvire governance ontology defines vocabulary for lifecycle, priority, risk, and owner metadata used for planning and routing.

Governance metadata is part of the Reqvire semantic model because it changes planning, ownership routing, and effective context even when it does not change implementation behavior. This ontology defines governance authoring categories, inheritance semantics, effective-value semantics, and persistence semantics.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:GovernedElement a owl:Class ;
  rdfs:subClassOf reqvire:Element ;
  rdfs:comment "Element type that may author explicit governance metadata." .
reqvire:Capability rdfs:subClassOf reqvire:GovernedElement .
reqvire:Requirement rdfs:subClassOf reqvire:GovernedElement .
reqvire:GovernanceRule a owl:Class ;
  rdfs:subClassOf reqvire:ContractRule ;
  rdfs:comment "Contract rule describing governance authoring, inheritance, effective value, or persistence semantics." .
reqvire:GovernanceMetadata a owl:Class ;
  rdfs:comment "Governance metadata concept covering status, priority, risk, and owner fields." .
reqvire:GovernanceValue a owl:Class ;
  rdfs:comment "Controlled vocabulary value for a governance metadata field." .
reqvire:StatusValue a owl:Class ;
  rdfs:subClassOf reqvire:GovernanceValue ;
  rdfs:comment "Controlled status metadata value." .
reqvire:PriorityValue a owl:Class ;
  rdfs:subClassOf reqvire:GovernanceValue ;
  rdfs:comment "Controlled priority metadata value." .
reqvire:RiskValue a owl:Class ;
  rdfs:subClassOf reqvire:GovernanceValue ;
  rdfs:comment "Controlled risk metadata value." .

reqvire:status a owl:DatatypeProperty ;
  rdfs:domain reqvire:GovernedElement ;
  rdfs:range xsd:string ;
  rdfs:comment "Lifecycle readiness metadata token authored by capability and requirement elements." .
reqvire:priority a owl:DatatypeProperty ;
  rdfs:domain reqvire:GovernedElement ;
  rdfs:range xsd:string ;
  rdfs:comment "Planning importance metadata token authored by capability and requirement elements." .
reqvire:risk a owl:DatatypeProperty ;
  rdfs:domain reqvire:GovernedElement ;
  rdfs:range xsd:string ;
  rdfs:comment "Requirement realization risk metadata token authored by capability and requirement elements." .
reqvire:owner a owl:DatatypeProperty ;
  rdfs:domain reqvire:GovernedElement ;
  rdfs:range xsd:string ;
  rdfs:comment "Free-form accountability or routing label authored by capability and requirement elements." .
reqvire:governanceDefaultValue a owl:DatatypeProperty ;
  rdfs:domain reqvire:GovernanceValue ;
  rdfs:range xsd:boolean ;
  rdfs:comment "Indicates whether a governance value is the default for its value family." .
reqvire:governanceValueName a owl:DatatypeProperty ;
  rdfs:domain reqvire:GovernanceValue ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical governance metadata value token used by Markdown metadata, filters, output, and queries." .
reqvire:governanceRuleName a owl:DatatypeProperty ;
  rdfs:domain reqvire:GovernanceRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical governance rule token used for rule lookup, reporting, and queries." .
reqvire:governanceAppliesTo a owl:DatatypeProperty ;
  rdfs:domain reqvire:GovernanceRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Token list describing the model element families to which a governance rule applies." .
reqvire:governanceSourceOrder a owl:DatatypeProperty ;
  rdfs:domain reqvire:GovernanceRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Ordered token list for effective governance value resolution." .
reqvire:governancePersistence a owl:DatatypeProperty ;
  rdfs:domain reqvire:GovernanceRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Persistence-mode token for governance metadata during formatting or mutation." .

reqvire:draftStatus a reqvire:StatusValue ;
  reqvire:governanceValueName "draft" ;
  rdfs:comment "The element is being authored or revised and is not ready for formal review." ;
  reqvire:governanceDefaultValue false .
reqvire:reviewStatus a reqvire:StatusValue ;
  reqvire:governanceValueName "review" ;
  rdfs:comment "The element is ready for, or currently under, stakeholder or engineering review." ;
  reqvire:governanceDefaultValue false .
reqvire:approvedStatus a reqvire:StatusValue ;
  reqvire:governanceValueName "approved" ;
  rdfs:comment "The element definition has completed review and is accepted as authoritative for downstream work." ;
  reqvire:governanceDefaultValue true .

reqvire:lowPriority a reqvire:PriorityValue ;
  reqvire:governanceValueName "low" ;
  rdfs:comment "Useful or desirable, but deferrable without major mission, stakeholder, or integration impact." ;
  reqvire:governanceDefaultValue false .
reqvire:mediumPriority a reqvire:PriorityValue ;
  reqvire:governanceValueName "medium" ;
  rdfs:comment "Normal planning importance; expected to be delivered unless schedule, cost, or scope tradeoffs require adjustment." ;
  reqvire:governanceDefaultValue true .
reqvire:highPriority a reqvire:PriorityValue ;
  reqvire:governanceValueName "high" ;
  rdfs:comment "Important to mission, stakeholder value, integration, or compliance and should be protected during tradeoffs." ;
  reqvire:governanceDefaultValue false .
reqvire:criticalPriority a reqvire:PriorityValue ;
  reqvire:governanceValueName "critical" ;
  rdfs:comment "Essential; failure to satisfy creates unacceptable mission, safety, compliance, contractual, or release impact." ;
  reqvire:governanceDefaultValue false .

reqvire:lowRisk a reqvire:RiskValue ;
  reqvire:governanceValueName "low" ;
  rdfs:comment "Requirement realization is well understood, stable, feasible, and straightforward to verify." ;
  reqvire:governanceDefaultValue true .
reqvire:mediumRisk a reqvire:RiskValue ;
  reqvire:governanceValueName "medium" ;
  rdfs:comment "Requirement realization has manageable uncertainty, moderate implementation or verification complexity, or limited downstream coupling." ;
  reqvire:governanceDefaultValue false .
reqvire:highRisk a reqvire:RiskValue ;
  reqvire:governanceValueName "high" ;
  rdfs:comment "Requirement realization has significant technical uncertainty, volatility, verification difficulty, integration exposure, or likely downstream rework." ;
  reqvire:governanceDefaultValue false .
reqvire:criticalRisk a reqvire:RiskValue ;
  reqvire:governanceValueName "critical" ;
  rdfs:comment "Requirement realization has severe uncertainty or exposure where failure, change, or non-compliance may materially affect mission, safety, compliance, cost, or schedule." ;
  reqvire:governanceDefaultValue false .

reqvire:governanceAuthoringRule a reqvire:GovernanceRule ;
  reqvire:governanceRuleName "governance-authoring" ;
  reqvire:governanceAppliesTo "capability, requirement" ;
  reqvire:ruleCondition "An element declares status, priority, risk, or owner metadata." ;
  reqvire:ruleOutcome "Only capability and requirement elements may author governance metadata; refinements and verifications obtain context from their owner or linked requirement." .

reqvire:governanceEffectiveValueRule a reqvire:GovernanceRule ;
  reqvire:governanceRuleName "governance-effective-value" ;
  reqvire:governanceAppliesTo "capability, requirement, refinement, verification" ;
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
  * derivedFrom: [Reqvire Behavior Rule Ontology](BehaviorValidationOperations.md#reqvire-behavior-rule-ontology)
---

