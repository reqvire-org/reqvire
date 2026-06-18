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
  rdfs:comment "Canonical rule token used for lookup, reporting, validation, or query contracts." .
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

Linting is separate from validation because validation blocks invalid model states, while linting reports reviewable quality issues and can offer explicit fixes when the repair is mechanically auditable. This ontology defines reusable linting rule and finding categories.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:LintingRule a owl:Class ;
  rdfs:subClassOf reqvire:ContractRule ;
  rdfs:comment "Auditable model-quality rule that reports reviewable findings." .
reqvire:LintFinding a owl:Class ;
  rdfs:comment "Reported model-quality finding produced by a linting rule." .

reqvire:lintRuleName a owl:DatatypeProperty ;
  rdfs:domain reqvire:LintingRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical lint rule token used by lint output, validation profiles, and queries." .
reqvire:lintScope a owl:DatatypeProperty ;
  rdfs:domain reqvire:LintingRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Model scope over which a linting rule evaluates." .
reqvire:lintCondition a owl:DatatypeProperty ;
  rdfs:domain reqvire:LintingRule ;
  rdfs:range xsd:string ;
  rdfs:comment "Condition that causes a linting rule to report a finding." .
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
  reqvire:lintCondition "A hierarchical relation crosses capability-root boundaries where a specify relation, concept reference, semantic-contract relation, or requirement-owned contract reused_contract_context would preserve ownership more clearly." ;
  reqvire:lintFindingKind "cross-submodel-coupling" ;
  reqvire:lintRepairMode "auditable-user-action" .

reqvire:redundantRelationLintRule a reqvire:LintingRule ;
  reqvire:lintRuleName "redundant-relation" ;
  reqvire:lintScope "relations and reused_contract_context" ;
  reqvire:lintCondition "A relation or reused_contract_context duplicates an already implied or repeated model edge." ;
  reqvire:lintFindingKind "redundant-relation" ;
  reqvire:lintRepairMode "explicit-fix-or-format-when-lossless" .

```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Behavior Rule Ontology](#reqvire-behavior-rule-ontology)
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
  rdfs:comment "Diagnostic issue reported by structural or semantic validation." .
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
  rdfs:comment "Canonical validation rule token used by diagnostics, output, and queries." .
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
  rdfs:comment "Condition that causes a validation rule to report an issue." .
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
  rdfs:comment "Canonical validation issue-kind token used by parser output, diagnostics, and queries." .
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

reqvire:reusedContractContextHierarchyIndependenceRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "reused-contract-context-hierarchy-independence" ;
  reqvire:validationScope "reused_contract_context" ;
  reqvire:validationSeverity "error" ;
  reqvire:validationCondition "An element reuses a contract already available through owner hierarchy or ancestor reused_contract_context propagation." ;
  reqvire:validationOutcome "Validation fails because the reused_contract_context is redundant or hides the intended dependency boundary." ;
  reqvire:validationRepair "Remove the redundant reused_contract_context or reuse the contract at the highest valid boundary." .

reqvire:reusedContractContextSubgraphDirectionRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "reused-contract-context-subgraph-direction" ;
  reqvire:validationScope "capability-root-subgraphs" ;
  reqvire:validationSeverity "error" ;
  reqvire:validationCondition "Two requirement subgraphs reuse contracts to each other in both directions." ;
  reqvire:validationOutcome "Validation fails because cross-subgraph reused_contract_context contracts must be one-directional." ;
  reqvire:validationRepair "Keep one dependency direction and move shared contracts into a common reused source if needed." .

reqvire:crossSectionDuplicateRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "cross-section-duplicate" ;
  reqvire:validationScope "element-subsections" ;
  reqvire:validationSeverity "error" ;
  reqvire:validationCondition "The same target appears in both Relations and Reused Contract Context for one element." ;
  reqvire:validationOutcome "Validation fails because the model cannot infer which semantic channel is intended." ;
  reqvire:validationRepair "Remove either the relation entry or the reused_contract_context entry." .

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
  reqvire:validationRepair "Replace with reused_contract_context/trace or remodel hierarchy under one capability root." .

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
  reqvire:validationCondition "A verification directly verifies both an element and an ancestor covered by that element's verification path." ;
  reqvire:validationOutcome "Report as manual review or auto-fixable depending on whether removing the ancestor verify relation preserves intended evidence scope." ;
  reqvire:validationRepair "Keep verification at the most precise capability or requirement scope, or document why broader verification is intended." .

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

### Reqvire Formatting Ontology

The Reqvire formatting ontology defines document normalization rule categories and semantic preservation invariants.

Formatting is separate from general model-element operations because its primary concern is preserving model meaning while normalizing Markdown structure. This ontology defines reusable formatting rule categories and invariants.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

reqvire:FormattingRule a owl:Class ;
  rdfs:comment "Rule category for deterministic Markdown normalization behavior." .
reqvire:FormattingInvariant a owl:Class ;
  rdfs:comment "Semantic preservation invariant that formatting must maintain." .

```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---

### Reqvire Operation Ontology

The Reqvire operation ontology defines operation categories and merge compatibility vocabulary.

This ontology defines reusable operation categories used by operation requirements.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:CommandOperation a owl:Class ;
  rdfs:comment "Operation exposed through a command or equivalent programmatic action." .
reqvire:MutationOperation a owl:Class ;
  rdfs:subClassOf reqvire:CommandOperation ;
  rdfs:comment "Command operation that can persist model or asset changes." .
reqvire:ReportOperation a owl:Class ;
  rdfs:subClassOf reqvire:CommandOperation ;
  rdfs:comment "Command operation that reads model state and emits output without mutating source files." .
reqvire:FormatOperation a owl:Class ;
  rdfs:subClassOf reqvire:CommandOperation ;
  rdfs:comment "Command operation that normalizes Markdown while preserving semantic model meaning." .
reqvire:MergeCompatibilityCategory a owl:Class ;
  rdfs:comment "Controlled category describing which element types are merge-compatible." .
reqvire:OperationFamily a owl:Class ;
  rdfs:comment "Controlled operation-family value used to classify Reqvire operations." .

reqvire:mergeCategoryElementType a owl:DatatypeProperty ;
  rdfs:domain reqvire:MergeCompatibilityCategory ;
  rdfs:range xsd:string ;
  rdfs:comment "Element type token included in a merge compatibility category." .
reqvire:mergeRequiresSameCategory a owl:DatatypeProperty ;
  rdfs:domain reqvire:MergeCompatibilityCategory ;
  rdfs:range xsd:boolean ;
  rdfs:comment "Indicates whether merge candidates must belong to the same compatibility category." .

reqvire:mutationOperationFamily a reqvire:OperationFamily ;
  rdfs:label "mutation" ;
  rdfs:comment "Operation family that can persist model or asset changes after validation gates pass." .
reqvire:reportOperationFamily a reqvire:OperationFamily ;
  rdfs:label "report" ;
  rdfs:comment "Operation family that reads model state and emits structured or human-readable reports without mutating source files." .
reqvire:validationOperationFamily a reqvire:OperationFamily ;
  rdfs:label "validation" ;
  rdfs:comment "Operation family that checks whether model state satisfies structural and semantic guardrails." .
reqvire:formatOperationFamily a reqvire:OperationFamily ;
  rdfs:label "formatting" ;
  rdfs:comment "Operation family that normalizes Markdown representation while preserving semantic model meaning." .
reqvire:relationMaintenanceOperationFamily a reqvire:OperationFamily ;
  rdfs:label "relation-maintenance" ;
  rdfs:comment "Operation family that links, unlinks, relinks, or rewires relation and reused_contract_context edges while preserving graph validity." .

reqvire:capabilityMergeCategory a reqvire:MergeCompatibilityCategory ;
  rdfs:label "capability" ;
  reqvire:mergeCategoryElementType "capability" ;
  rdfs:comment "Capability elements merge only with capability elements." ;
  reqvire:mergeRequiresSameCategory true .
reqvire:requirementMergeCategory a reqvire:MergeCompatibilityCategory ;
  rdfs:label "requirement" ;
  reqvire:mergeCategoryElementType "requirement" ;
  rdfs:comment "Requirement elements merge only with requirement elements." ;
  reqvire:mergeRequiresSameCategory true .
reqvire:verificationMergeCategory a reqvire:MergeCompatibilityCategory ;
  rdfs:label "verification" ;
  reqvire:mergeCategoryElementType "verification", "test-verification", "analysis-verification", "inspection-verification", "demonstration-verification", "formal-proof-verification" ;
  rdfs:comment "Concrete verification elements merge only within the concrete verification element family." ;
  reqvire:mergeRequiresSameCategory true .
reqvire:verificationObjectiveMergeCategory a reqvire:MergeCompatibilityCategory ;
  rdfs:label "verification-objective" ;
  reqvire:mergeCategoryElementType "verification-objective" ;
  rdfs:comment "Verification objective elements merge only with other verification objectives, not with concrete verification elements." ;
  reqvire:mergeRequiresSameCategory true .
reqvire:contractMergeCategory a reqvire:MergeCompatibilityCategory ;
  rdfs:label "contract" ;
  reqvire:mergeCategoryElementType "source", "constraint", "behavior", "specification", "state", "input-output" ;
  rdfs:comment "Requirement-owned contracts merge only with compatible requirement-owned contracts." ;
  reqvire:mergeRequiresSameCategory true .
reqvire:requirementContractMergeCategory a reqvire:MergeCompatibilityCategory ;
  rdfs:label "requirement-contract" ;
  reqvire:mergeCategoryElementType "constraint", "behavior", "specification", "state", "input-output" ;
  rdfs:comment "Requirement-owned contracts merge only with requirement-owned contracts." ;
  reqvire:mergeRequiresSameCategory true .
reqvire:semanticContractMergeCategory a reqvire:MergeCompatibilityCategory ;
  rdfs:label "semantic-contract" ;
  reqvire:mergeCategoryElementType "semantic-contract" ;
  rdfs:comment "Semantic-contract merge compatibility applies to reusable SHACL profile contracts." ;
  reqvire:mergeRequiresSameCategory true .
reqvire:ontologyMergeCategory a reqvire:MergeCompatibilityCategory ;
  rdfs:label "ontology" ;
  reqvire:mergeCategoryElementType "ontology" ;
  rdfs:comment "Ontology elements merge only with ontology elements and fold authored Turtle into the target ontology block." ;
  reqvire:mergeRequiresSameCategory true .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
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

