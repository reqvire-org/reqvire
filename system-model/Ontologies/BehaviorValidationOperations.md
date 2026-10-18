# Elements

### Behavior Rule Structure Shape

Defines SHACL constraints for behavior rules, state transitions, and behavior contracts.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:BehaviorRuleShape
  a sh:NodeShape ;
  sh:targetClass reqvire:BehaviorRule ;
  sh:property [
    sh:path reqvire:ruleName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:ruleCondition ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:ruleOutcome ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:sourceBehavior ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:behaviorPhase ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:rulePriority ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:precondition ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:postcondition ;
    sh:datatype xsd:string ;
  ] .

reqvire:StateTransitionShape
  a sh:NodeShape ;
  sh:targetClass reqvire:StateTransition ;
  sh:property [
    sh:path reqvire:fromState ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:toState ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:trigger ;
    sh:datatype xsd:string ;
  ] .

reqvire:BehaviorShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Behavior ;
  sh:property [
    sh:path reqvire:hasRule ;
    sh:class reqvire:BehaviorRule ;
  ] ;
  sh:property [
    sh:path reqvire:hasTransition ;
    sh:class reqvire:StateTransition ;
  ] .

reqvire:InputOutputMappingShape
  a sh:NodeShape ;
  sh:targetClass reqvire:InputOutputMapping ;
  sh:property [
    sh:path reqvire:usesInput ;
    sh:class reqvire:Element ;
  ] ;
  sh:property [
    sh:path reqvire:producesOutput ;
    sh:class reqvire:Element ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Validate Cross-Component Dependencies](../Operations/Validation/ValidationRequirements.md#validate-cross-component-dependencies)
  * use: [Reqvire Behavior Rule Ontology](#reqvire-behavior-rule-ontology)
---

### Lint Rule Metadata Shape

Defines SHACL constraints for linting rule metadata and repair-mode semantics.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:LintingRuleShape
  a sh:NodeShape ;
  sh:targetClass reqvire:LintingRule ;
  sh:property [
    sh:path reqvire:lintRuleName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:lintScope ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:lintCondition ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:lintFindingKind ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:lintRepairMode ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Model Linting](../Operations/Linting/LintingRequirements.md#model-linting)
  * use: [Reqvire Linting Ontology](#reqvire-linting-ontology)
---

### Reqvire Behavior Rule Ontology

The Reqvire behavior rule ontology defines behavior rules, state transitions, and input-output mappings used by requirement contracts.

Behavior rules, state transitions, and input-output mappings are semantic model terms owned by this capability. Requirements state the system obligations that apply those rules.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:BehaviorRule a owl:Class ;
  rdfs:subClassOf reqvire:ContractRule ;
  rdfs:comment "Contract rule that describes behavior-specific conditions and outcomes." .
reqvire:StateTransition a owl:Class ;
  rdfs:comment "Modeled transition between named states in a behavior contract." .
reqvire:InputOutputMapping a owl:Class ;
  rdfs:comment "Mapping from modeled input elements to modeled output elements." .
reqvire:ContractRule a owl:Class ;
  rdfs:comment "Reusable rule concept carrying canonical rule tokens, conditions, and outcomes." .

reqvire:hasRule a owl:ObjectProperty ;
  rdfs:domain reqvire:Behavior ;
  rdfs:range reqvire:BehaviorRule ;
  rdfs:comment "Associates a behavior contract with a behavior rule." .
reqvire:hasTransition a owl:ObjectProperty ;
  rdfs:domain reqvire:Behavior ;
  rdfs:range reqvire:StateTransition ;
  rdfs:comment "Associates a behavior contract with a state transition." .
reqvire:usesInput a owl:ObjectProperty ;
  rdfs:domain reqvire:InputOutputMapping ;
  rdfs:range reqvire:Element ;
  rdfs:comment "Identifies a model element consumed as input by an input-output mapping." .
reqvire:producesOutput a owl:ObjectProperty ;
  rdfs:domain reqvire:InputOutputMapping ;
  rdfs:range reqvire:Element ;
  rdfs:comment "Identifies a model element produced as output by an input-output mapping." .

reqvire:ruleName a owl:DatatypeProperty ;
  rdfs:domain reqvire:ContractRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical rule token used for lookup, validation, or query contracts." .
reqvire:ruleCondition a owl:DatatypeProperty ;
  rdfs:domain reqvire:ContractRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Textual condition under which a contract rule applies." .
reqvire:ruleOutcome a owl:DatatypeProperty ;
  rdfs:domain reqvire:ContractRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Textual outcome expected when a contract rule condition is met." .
reqvire:sourceBehavior a owl:DatatypeProperty ;
  rdfs:domain reqvire:BehaviorRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Identifier or label for the behavior source described by a behavior rule." .
reqvire:behaviorPhase a owl:DatatypeProperty ;
  rdfs:domain reqvire:BehaviorRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Named lifecycle or execution phase for a behavior rule." .
reqvire:rulePriority a owl:DatatypeProperty ;
  rdfs:domain reqvire:BehaviorRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Priority token used to order or classify behavior rules." .
reqvire:trigger a owl:DatatypeProperty ;
  rdfs:domain reqvire:StateTransition ;
  rdfs:range xsd:string ;
  rdfs:comment "Trigger text or token that initiates a state transition." .
reqvire:precondition a owl:DatatypeProperty ;
  rdfs:domain reqvire:ContractRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Condition that must hold before applying a contract rule." .
reqvire:postcondition a owl:DatatypeProperty ;
  rdfs:domain reqvire:ContractRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Condition expected to hold after applying a contract rule." .
reqvire:fromState a owl:DatatypeProperty ;
  rdfs:domain reqvire:StateTransition ;
  rdfs:range xsd:string ;
  rdfs:comment "Source state token for a state transition." .
reqvire:toState a owl:DatatypeProperty ;
  rdfs:domain reqvire:StateTransition ;
  rdfs:range xsd:string ;
  rdfs:comment "Target state token for a state transition." .

```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Requirement Ontology](CapabilityRequirementModel.md#reqvire-requirement-ontology)
---

### Reqvire Linting Ontology

The Reqvire linting ontology defines auditable model quality checks and lint findings.

Linting is separate from validation because validation blocks invalid model states, while linting records reviewable quality issues and can offer explicit fixes when the repair is mechanically auditable. This ontology defines reusable linting rule and finding categories.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:LintingRule a owl:Class ;
  rdfs:subClassOf reqvire:ContractRule ;
  rdfs:comment "Auditable model-quality rule that records reviewable findings." .
reqvire:LintFinding a owl:Class ;
  rdfs:comment "Reported model-quality finding produced by a linting rule." .

reqvire:lintRuleName a owl:DatatypeProperty ;
  rdfs:domain reqvire:LintingRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical lint rule token used by lint diagnostics, validation profiles, and queries." .
reqvire:lintScope a owl:DatatypeProperty ;
  rdfs:domain reqvire:LintingRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Model scope over which a linting rule evaluates." .
reqvire:lintCondition a owl:DatatypeProperty ;
  rdfs:domain reqvire:LintingRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Condition that causes a linting rule to record a finding." .
reqvire:lintFindingKind a owl:DatatypeProperty ;
  rdfs:domain reqvire:LintingRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical finding-kind token emitted by a linting rule." .
reqvire:lintRepairMode a owl:DatatypeProperty ;
  rdfs:domain reqvire:LintingRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Repair-mode token describing whether and how a lint finding can be corrected." .

reqvire:crossSubmodelHierarchyLintRule a reqvire:LintingRule ;
  reqvire:lintRuleName "cross-submodel-hierarchy" ;
  reqvire:lintScope "capability-rooted subgraphs" ;
  reqvire:lintCondition "A hierarchical relation crosses capability-root boundaries where a specify relation, concept reference, semantic-contract relation, or requirement-owned Contract Binding or Contract Reference would preserve ownership more clearly, according to implementation responsibility or review dependency." ;
  reqvire:lintFindingKind "cross-submodel-coupling" ;
  reqvire:lintRepairMode "auditable-user-action" .

reqvire:redundantRelationLintRule a reqvire:LintingRule ;
  reqvire:lintRuleName "redundant-relation" ;
  reqvire:lintScope "hierarchical and verification relations" ;
  reqvire:lintCondition "A direct relation is redundant under its hierarchical or verification reachability policy." ;
  reqvire:lintFindingKind "redundant-relation" ;
  reqvire:lintRepairMode "mutation-enabled-safe-lint-repair" .

```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Behavior Rule Ontology](#reqvire-behavior-rule-ontology)
---

### Reqvire Model Mutation Ontology

Defines model mutations, identity-preserving definition replacement, deletion, and the relationships retained across a replacement.

The vocabulary distinguishes complete-definition replacement from an edit confined to one content region, identifies that region's owning definition, and distinguishes opening text from a named subsection.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix concept: <https://www.reqvire.org/concepts#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

reqvire:ModelMutation a owl:Class ;
  reqvire:mapsToConcept concept:ModelMutation ;
  rdfs:comment "Change to the authored definitions or structure of an accepted system model." .
reqvire:AtomicModelMutation a owl:Class ;
  rdfs:subClassOf reqvire:ModelMutation ;
  reqvire:mapsToConcept concept:AtomicModelMutation ;
  rdfs:comment "Model mutation with one indivisible accepted-state transition." .
reqvire:ElementOverride a owl:Class ;
  rdfs:subClassOf reqvire:AtomicModelMutation ;
  reqvire:mapsToConcept concept:ElementOverride ;
  owl:disjointWith reqvire:ElementDeletion ;
  rdfs:comment "Replacement of an existing element's authored definition that retains its identity and externally authored incoming relationships." .
reqvire:ElementDeletion a owl:Class ;
  rdfs:subClassOf reqvire:AtomicModelMutation ;
  reqvire:mapsToConcept concept:ElementDeletion ;
  rdfs:comment "Removal of an element from the resulting accepted model." .
reqvire:AuthoredElementDefinition a owl:Class ;
  reqvire:mapsToConcept concept:AuthoredElementDefinition ;
  rdfs:comment "Author-supplied content, metadata, and declarations for one element, excluding generated inverses and declarations authored on other elements." .

reqvire:FocusedElementEdit a owl:Class ;
  rdfs:subClassOf reqvire:AtomicModelMutation ;
  reqvire:mapsToConcept concept:FocusedElementEdit ;
  rdfs:comment "Mutation confined to selected content of one element while retaining identity and declarations outside the selection." .
reqvire:EditTarget a owl:Class ;
  reqvire:mapsToConcept concept:EditTarget ;
  rdfs:comment "Bounded region selected within an authored element definition." .
reqvire:ElementText a owl:Class ;
  rdfs:subClassOf reqvire:EditTarget ;
  reqvire:mapsToConcept concept:ElementText ;
  rdfs:comment "Opening human-readable element content preceding its first subsection boundary." .
reqvire:ElementSubsection a owl:Class ;
  rdfs:subClassOf reqvire:EditTarget ;
  reqvire:mapsToConcept concept:ElementSubsection ;
  rdfs:comment "Named region of an authored element definition with type-dependent content semantics." .
reqvire:editTarget a owl:ObjectProperty ;
  rdfs:domain reqvire:FocusedElementEdit ;
  rdfs:range reqvire:EditTarget ;
  rdfs:comment "Region to which the intended authored-content change is confined." .
reqvire:targetDefinition a owl:ObjectProperty ;
  rdfs:domain reqvire:EditTarget ;
  rdfs:range reqvire:AuthoredElementDefinition ;
  rdfs:comment "Authored element definition containing the selected region." .

reqvire:mutationSubject a owl:ObjectProperty ;
  rdfs:domain reqvire:ModelMutation ;
  rdfs:range reqvire:Element ;
  rdfs:comment "Element whose authored definition or presence is the subject of a model mutation." .
reqvire:replacementDefinition a owl:ObjectProperty ;
  rdfs:domain reqvire:ElementOverride ;
  rdfs:range reqvire:AuthoredElementDefinition ;
  rdfs:comment "Complete authored definition supplied for an element override." .
reqvire:preservedElementIdentity a owl:ObjectProperty ;
  rdfs:domain reqvire:ElementOverride ;
  rdfs:range reqvire:ElementIdentity ;
  rdfs:comment "Element identity retained across the replacement of an authored definition." .
reqvire:retainedIncomingRelation a owl:ObjectProperty ;
  rdfs:domain reqvire:ElementOverride ;
  rdfs:range reqvire:ModelRelation ;
  reqvire:mapsToConcept concept:IncomingRelation ;
  rdfs:comment "Externally authored model relation retained with the overridden element as its target." .
reqvire:retainedDependencyConsumer a owl:ObjectProperty ;
  rdfs:domain reqvire:ElementOverride ;
  rdfs:range reqvire:Element ;
  rdfs:comment "Element whose externally authored contract or terminology dependency continues to target the overridden element." .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Relation Ontology](RelationsAndImpact.md#reqvire-relation-ontology)
---

### Reqvire Validation Ontology

The Reqvire validation ontology defines validation issues, validation rules, lint issues, and mutation safety gates.

Validation is the canonical guardrail layer for the model. This ontology defines the validation rule vocabulary and rule definitions used by validation requirements.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:ValidationRule a owl:Class ;
  rdfs:subClassOf reqvire:ContractRule ;
  rdfs:comment "Validation guardrail rule that can block invalid model states." .
reqvire:ValidationIssue a owl:Class ;
  rdfs:comment "Diagnostic issue recorded by structural or semantic validation." .
reqvire:LintIssue a owl:Class ;
  rdfs:subClassOf reqvire:ValidationIssue ;
  rdfs:comment "Validation issue that represents a lint finding rather than a hard invalid state." .
reqvire:MutationSafetyGate a owl:Class ;
  rdfs:comment "Validation gate that determines whether a mutation may persist changes." .
reqvire:ValidationIssueKind a owl:Class ;
  rdfs:comment "Controlled validation issue kind identified by a canonical diagnostic token." .

reqvire:validationRuleName a owl:DatatypeProperty ;
  rdfs:domain reqvire:ValidationRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical validation rule token used by diagnostics and queries." .
reqvire:validationScope a owl:DatatypeProperty ;
  rdfs:domain reqvire:ValidationRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Model scope over which a validation rule applies." .
reqvire:validationSeverity a owl:DatatypeProperty ;
  rdfs:domain reqvire:ValidationRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Severity token emitted for issues from a validation rule." .
reqvire:validationCondition a owl:DatatypeProperty ;
  rdfs:domain reqvire:ValidationRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Condition that causes a validation rule to record an issue." .
reqvire:validationOutcome a owl:DatatypeProperty ;
  rdfs:domain reqvire:ValidationRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Expected validation outcome when the rule condition is met." .
reqvire:validationRepair a owl:DatatypeProperty ;
  rdfs:domain reqvire:ValidationRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Repair guidance associated with a validation rule." .
reqvire:validationIssueKindName a owl:DatatypeProperty ;
  rdfs:domain reqvire:ValidationIssueKind ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical validation issue-kind token used by parsers, diagnostics, and queries." .
reqvire:lintIssueKind a owl:DatatypeProperty ;
  rdfs:domain reqvire:LintIssue ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical lint issue-kind token carried by a lint issue." .
reqvire:blocksPersistence a owl:DatatypeProperty ;
  rdfs:domain reqvire:MutationSafetyGate ;
  rdfs:range xsd:boolean ;
  rdfs:comment "Indicates whether the safety gate blocks persistence of a mutation." .

reqvire:semanticReferenceNotFoundIssueKind a reqvire:ValidationIssueKind ;
  reqvire:validationIssueKindName "semantic-reference-not-found" ;
  rdfs:comment "A semantic-contract SHACL reference points to an IRI that no Reqvire ontology element declares." .
reqvire:semanticReferenceOutsideContextIssueKind a reqvire:ValidationIssueKind ;
  reqvire:validationIssueKindName "semantic-reference-found-outside-context" ;
  rdfs:comment "A semantic-contract SHACL reference points to an IRI declared by an ontology element that is outside the contract's explicit ontology-use context." .
reqvire:semanticDuplicateDeclarationIssueKind a reqvire:ValidationIssueKind ;
  reqvire:validationIssueKindName "semantic-duplicate-declaration" ;
  rdfs:comment "The same ontology term IRI is declared by multiple ontology elements." .
reqvire:semanticConflictingDeclarationIssueKind a reqvire:ValidationIssueKind ;
  reqvire:validationIssueKindName "semantic-conflicting-declaration" ;
  rdfs:comment "The same ontology term IRI is declared with incompatible semantic roles across ontology elements." .

reqvire:globalElementNameUniquenessRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "global-element-name-uniqueness" ;
  reqvire:validationScope "model" ;
  reqvire:validationSeverity "error" ;
  reqvire:validationCondition "Two parsed elements have the same element name." ;
  reqvire:validationOutcome "Validation fails because element names are stable global identity keys." ;
  reqvire:validationRepair "Rename one element and update incoming references." .

reqvire:singleRootHierarchyOwnershipRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "single-root-hierarchy-ownership" ;
  reqvire:validationScope "capability-requirement-graph" ;
  reqvire:validationSeverity "error" ;
  reqvire:validationCondition "A requirement hierarchy resolves to zero or more than one owning capability root." ;
  reqvire:validationOutcome "Validation fails because each requirement hierarchy must belong to exactly one capability root." ;
  reqvire:validationRepair "Add or repair specify/derivedFrom relations so the hierarchy resolves to one capability root." .

reqvire:relationTypeCompatibilityRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "relation-type-compatibility" ;
  reqvire:validationScope "relations" ;
  reqvire:validationSeverity "error" ;
  reqvire:validationCondition "A relation source, relation type, or target violates relation compatibility rules." ;
  reqvire:validationOutcome "Validation fails before the graph is used." ;
  reqvire:validationRepair "Use a compatible relation type or change element types/ownership." .

reqvire:nativeConceptTaxonomySchemeBoundaryRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "native-concept-taxonomy-scheme-boundary" ;
  reqvire:validationScope "native-concept-taxonomy-relations" ;
  reqvire:validationSeverity "error" ;
  reqvire:validationCondition "A native concept broader or narrower relation targets a concept that resolves to a different concept-scheme root." ;
  reqvire:validationOutcome "Validation fails because SKOS broader/narrower taxonomy is scheme-local in Reqvire native concept authoring." ;
  reqvire:validationRepair "Keep broader/narrower inside one concept scheme, or use related, exactMatch, or closeMatch for cross-scheme concept alignment." .

reqvire:contractBindingsHierarchyIndependenceRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "contract-bindings-hierarchy-independence" ;
  reqvire:validationScope "contract_bindings" ;
  reqvire:validationSeverity "error" ;
  reqvire:validationCondition "An element reuses a contract already available through owner hierarchy or ancestor contract_bindings propagation." ;
  reqvire:validationOutcome "Validation fails because the contract_bindings is redundant or hides the intended dependency boundary." ;
  reqvire:validationRepair "Remove redundant contract_bindings and place bindings at requirements responsible for implementing the obligation. Bind a parent only when the obligation applies to its entire requirement subtree." .

reqvire:contractBindingsSubgraphDirectionRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "contract-bindings-subgraph-direction" ;
  reqvire:validationScope "capability-root-subgraphs" ;
  reqvire:validationSeverity "error" ;
  reqvire:validationCondition "Two requirement subgraphs reuse contracts to each other in both directions." ;
  reqvire:validationOutcome "Validation fails because cross-subgraph contract bindings must be one-directional." ;
  reqvire:validationRepair "Keep one dependency direction and move shared contracts into a common reused source if needed." .

reqvire:crossSectionDuplicateRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "cross-section-duplicate" ;
  reqvire:validationScope "element-subsections" ;
  reqvire:validationSeverity "error" ;
  reqvire:validationCondition "The same target appears in both Relations and Contract Bindings for one element." ;
  reqvire:validationOutcome "Validation fails because the model cannot infer which semantic channel is intended." ;
  reqvire:validationRepair "Remove either the relation entry or the contract_bindings entry." .

reqvire:semanticReferenceReachabilityValidationRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "semantic-reference-reachability-validation" ;
  reqvire:validationScope "semantic-contract-use-ontology-context" ;
  reqvire:validationSeverity "error" ;
  reqvire:validationCondition "A SHACL reference points to an ontology term that is not declared or is declared outside the semantic contract's explicit use graph." ;
  reqvire:validationOutcome "Validation fails because semantic dependencies must remain visible to change impact." ;
  reqvire:validationRepair "Declare the term in ontology used by the semantic contract, add a use relation to the declaring ontology or an ontology descendant whose hierarchy reaches it, or remove/update the reference." .

reqvire:lintManualReviewRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "lint-manual-review" ;
  reqvire:validationScope "lint" ;
  reqvire:validationSeverity "warning" ;
  reqvire:validationCondition "A model quality issue is auditable but not safely auto-fixable." ;
  reqvire:validationOutcome "Report as manual review issue and do not auto-modify the model." ;
  reqvire:validationRepair "User reviews context and applies an explicit model change." .

reqvire:crossSubmodelHierarchicalLintRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "lint-cross-submodel-hierarchical-relation" ;
  reqvire:validationScope "lint" ;
  reqvire:validationSeverity "warning" ;
  reqvire:validationCondition "A user-authored hierarchical relation crosses capability-root submodel ownership boundaries." ;
  reqvire:validationOutcome "Report as manual review because ownership boundary intent is ambiguous." ;
  reqvire:validationRepair "Remove the cross-boundary hierarchy edge and declare the intended compatible dependency: a Contract Binding for a shared implementation obligation, a Contract Reference for change-impact review, a Concept Reference for native terminology, or semantic-contract use/constrain relations. Keep contract dependencies on responsible requirements; retain hierarchy only for genuine ownership within one capability root." .

reqvire:redundantHierarchicalLintRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "lint-redundant-hierarchical-relation" ;
  reqvire:validationScope "lint" ;
  reqvire:validationSeverity "warning" ;
  reqvire:validationCondition "An element has direct hierarchical relations to both an ancestor and descendant path already reachable through hierarchy." ;
  reqvire:validationOutcome "Report as auto-fixable when the redundant relation can be removed without changing reachability." ;
  reqvire:validationRepair "Remove the redundant direct derivedFrom/derive relation." .

reqvire:redundantVerifyLintRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "lint-redundant-verify-relation" ;
  reqvire:validationScope "lint" ;
  reqvire:validationSeverity "warning" ;
  reqvire:validationCondition "A direct verification target is redundant under the verification relation redundancy policy." ;
  reqvire:validationOutcome "Report eligible redundant direct ancestor targets as auto-fixable. Coverage roll-up alone does not make a separately scoped parent verification redundant." ;
  reqvire:validationRepair "Remove only direct verify edges classified as redundant by the shared reachability policy; review independent parent integration intent separately." .

reqvire:multiBranchConvergenceLintRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "lint-multi-branch-convergence" ;
  reqvire:validationScope "lint" ;
  reqvire:validationSeverity "warning" ;
  reqvire:validationCondition "An element reaches a common ancestor through multiple distinct branch paths." ;
  reqvire:validationOutcome "Report as manual review because both paths may be meaningful or one may be a modeling error." ;
  reqvire:validationRepair "Retain both paths only when they represent distinct valid semantics." .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Behavior Rule Ontology](#reqvire-behavior-rule-ontology)
---

### Validation Rule Diagnostic Shape

Defines SHACL constraints for validation rule metadata and validation outcomes.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:ValidationRuleShape
  a sh:NodeShape ;
  sh:targetClass reqvire:ValidationRule ;
  sh:property [
    sh:path reqvire:validationRuleName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:validationScope ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:validationSeverity ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("error" "warning" "info") ;
  ] ;
  sh:property [
    sh:path reqvire:validationCondition ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:validationOutcome ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:validationRepair ;
    sh:datatype xsd:string ;
  ] .

reqvire:ValidationIssueKindShape
  a sh:NodeShape ;
  sh:targetClass reqvire:ValidationIssueKind ;
  sh:property [
    sh:path reqvire:validationIssueKindName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] .

reqvire:LintIssueShape
  a sh:NodeShape ;
  sh:targetClass reqvire:LintIssue ;
  sh:property [
    sh:path reqvire:lintIssueKind ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] .

reqvire:MutationSafetyGateShape
  a sh:NodeShape ;
  sh:targetClass reqvire:MutationSafetyGate ;
  sh:property [
    sh:path reqvire:blocksPersistence ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:boolean ;
  ] .

```

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Validate Cross-Component Dependencies](../Operations/Validation/ValidationRequirements.md#validate-cross-component-dependencies)
  * use: [Reqvire Validation Ontology](#reqvire-validation-ontology)
---
