# Elements

### Reqvire Behavior Rule Ontology

The Reqvire behavior rule ontology defines behavior rules, state transitions, and input-output mappings used by requirement refinements.

Behavior rules, state transitions, and input-output mappings are semantic model terms owned by this capability. Requirements state the system obligations that apply those rules.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

reqvire:BehaviorRule a owl:Class .
reqvire:StateTransition a owl:Class .
reqvire:InputOutputMapping a owl:Class .
reqvire:ContractRule a owl:Class .

reqvire:hasRule a owl:ObjectProperty .
reqvire:hasTransition a owl:ObjectProperty .
reqvire:usesInput a owl:ObjectProperty .
reqvire:producesOutput a owl:ObjectProperty .

reqvire:ruleName a owl:DatatypeProperty .
reqvire:ruleCondition a owl:DatatypeProperty .
reqvire:ruleOutcome a owl:DatatypeProperty .
reqvire:sourceBehavior a owl:DatatypeProperty .
reqvire:behaviorPhase a owl:DatatypeProperty .
reqvire:rulePriority a owl:DatatypeProperty .
reqvire:trigger a owl:DatatypeProperty .
reqvire:precondition a owl:DatatypeProperty .
reqvire:postcondition a owl:DatatypeProperty .
reqvire:fromState a owl:DatatypeProperty .
reqvire:toState a owl:DatatypeProperty .

```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---

### Reqvire Formatting Ontology

The Reqvire formatting ontology defines document normalization rule categories and semantic preservation invariants.

Formatting is separate from general model-element operations because its primary concern is preserving model meaning while normalizing Markdown structure. This ontology defines reusable formatting rule categories and invariants.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

reqvire:FormattingRule a owl:Class .
reqvire:FormattingInvariant a owl:Class .

```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---

### Reqvire Linting Ontology

The Reqvire linting ontology defines auditable model quality checks and lint findings.

Linting is separate from validation because validation blocks invalid model states, while linting reports reviewable quality issues and can offer explicit fixes when the repair is mechanically auditable. This ontology defines reusable linting rule and finding categories.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

reqvire:LintingRule a owl:Class .
reqvire:LintFinding a owl:Class .

reqvire:lintRuleName a owl:DatatypeProperty .
reqvire:lintScope a owl:DatatypeProperty .
reqvire:lintCondition a owl:DatatypeProperty .
reqvire:lintFindingKind a owl:DatatypeProperty .
reqvire:lintRepairMode a owl:DatatypeProperty .

reqvire:crossSubmodelHierarchyLintRule a reqvire:LintingRule ;
  reqvire:lintRuleName "cross-submodel-hierarchy" ;
  reqvire:lintScope "capability-rooted subgraphs" ;
  reqvire:lintCondition "A hierarchical relation crosses capability-root boundaries where an attachment or specify relation would preserve ownership more clearly." ;
  reqvire:lintFindingKind "cross-submodel-coupling" ;
  reqvire:lintRepairMode "auditable-user-action" .

reqvire:redundantRelationLintRule a reqvire:LintingRule ;
  reqvire:lintRuleName "redundant-relation" ;
  reqvire:lintScope "relations and attachments" ;
  reqvire:lintCondition "A relation or attachment duplicates an already implied or repeated model edge." ;
  reqvire:lintFindingKind "redundant-relation" ;
  reqvire:lintRepairMode "explicit-fix-or-format-when-lossless" .

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

reqvire:CommandOperation a owl:Class .
reqvire:MutationOperation a owl:Class .
reqvire:ReportOperation a owl:Class .
reqvire:FormatOperation a owl:Class .
reqvire:MergeCompatibilityCategory a owl:Class .
reqvire:OperationFamily a owl:Class .

reqvire:mergeCategoryName a owl:DatatypeProperty .
reqvire:mergeCategoryElementType a owl:DatatypeProperty .
reqvire:mergeCategoryMeaning a owl:DatatypeProperty .
reqvire:mergeRequiresSameCategory a owl:DatatypeProperty .
reqvire:operationFamilyName a owl:DatatypeProperty .
reqvire:operationFamilyMeaning a owl:DatatypeProperty .

reqvire:mutationOperationFamily a reqvire:OperationFamily ;
  reqvire:operationFamilyName "mutation" ;
  reqvire:operationFamilyMeaning "Operation family that can persist model or asset changes after validation gates pass." .
reqvire:reportOperationFamily a reqvire:OperationFamily ;
  reqvire:operationFamilyName "report" ;
  reqvire:operationFamilyMeaning "Operation family that reads model state and emits structured or human-readable reports without mutating source files." .
reqvire:validationOperationFamily a reqvire:OperationFamily ;
  reqvire:operationFamilyName "validation" ;
  reqvire:operationFamilyMeaning "Operation family that checks whether model state satisfies structural and semantic guardrails." .
reqvire:formatOperationFamily a reqvire:OperationFamily ;
  reqvire:operationFamilyName "formatting" ;
  reqvire:operationFamilyMeaning "Operation family that normalizes Markdown representation while preserving semantic model meaning." .
reqvire:relationMaintenanceOperationFamily a reqvire:OperationFamily ;
  reqvire:operationFamilyName "relation-maintenance" ;
  reqvire:operationFamilyMeaning "Operation family that links, unlinks, relinks, or rewires relation and attachment edges while preserving graph validity." .

reqvire:capabilityMergeCategory a reqvire:MergeCompatibilityCategory ;
  reqvire:mergeCategoryName "capability" ;
  reqvire:mergeCategoryElementType "capability" ;
  reqvire:mergeCategoryMeaning "Capability elements merge only with capability elements." ;
  reqvire:mergeRequiresSameCategory true .
reqvire:requirementMergeCategory a reqvire:MergeCompatibilityCategory ;
  reqvire:mergeCategoryName "requirement" ;
  reqvire:mergeCategoryElementType "requirement" ;
  reqvire:mergeCategoryMeaning "Requirement elements merge only with requirement elements." ;
  reqvire:mergeRequiresSameCategory true .
reqvire:verificationMergeCategory a reqvire:MergeCompatibilityCategory ;
  reqvire:mergeCategoryName "verification" ;
  reqvire:mergeCategoryElementType "verification", "test-verification", "analysis-verification", "inspection-verification", "demonstration-verification", "formal-proof-verification" ;
  reqvire:mergeCategoryMeaning "Verification elements merge only within the verification element family." ;
  reqvire:mergeRequiresSameCategory true .
reqvire:capabilityRefinementMergeCategory a reqvire:MergeCompatibilityCategory ;
  reqvire:mergeCategoryName "capability-refinement" ;
  reqvire:mergeCategoryElementType "source", "semantic-contract", "constraint", "behavior", "specification", "state", "input-output" ;
  reqvire:mergeCategoryMeaning "Capability-owned refinements merge only with compatible capability-owned refinements." ;
  reqvire:mergeRequiresSameCategory true .
reqvire:requirementRefinementMergeCategory a reqvire:MergeCompatibilityCategory ;
  reqvire:mergeCategoryName "requirement-refinement" ;
  reqvire:mergeCategoryElementType "constraint", "behavior", "specification", "state", "input-output" ;
  reqvire:mergeCategoryMeaning "Requirement-owned refinements merge only with requirement-owned refinements." ;
  reqvire:mergeRequiresSameCategory true .
reqvire:semanticContractMergeCategory a reqvire:MergeCompatibilityCategory ;
  reqvire:mergeCategoryName "semantic-contract" ;
  reqvire:mergeCategoryElementType "semantic-contract" ;
  reqvire:mergeCategoryMeaning "Semantic-contract merge compatibility applies to requirement-owned SHACL profile refinements." ;
  reqvire:mergeRequiresSameCategory true .

```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---

### Reqvire Validation Ontology

The Reqvire validation ontology defines validation issues, validation rules, lint issues, and mutation safety gates.

Validation is the canonical guardrail layer for the model. This ontology defines the validation rule vocabulary and rule definitions used by validation requirements.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

reqvire:ValidationRule a owl:Class .
reqvire:ValidationIssue a owl:Class .
reqvire:LintIssue a owl:Class .
reqvire:MutationSafetyGate a owl:Class .
reqvire:ValidationIssueKind a owl:Class .

reqvire:validationRuleName a owl:DatatypeProperty .
reqvire:validationScope a owl:DatatypeProperty .
reqvire:validationSeverity a owl:DatatypeProperty .
reqvire:validationCondition a owl:DatatypeProperty .
reqvire:validationOutcome a owl:DatatypeProperty .
reqvire:validationRepair a owl:DatatypeProperty .
reqvire:validationIssueKindName a owl:DatatypeProperty .
reqvire:validationIssueKindMeaning a owl:DatatypeProperty .
reqvire:lintIssueKind a owl:DatatypeProperty .
reqvire:blocksPersistence a owl:DatatypeProperty .

reqvire:semanticReferenceNotFoundIssueKind a reqvire:ValidationIssueKind ;
  reqvire:validationIssueKindName "semantic-reference-not-found" ;
  reqvire:validationIssueKindMeaning "A semantic-contract SHACL reference points to an IRI that no Reqvire ontology element declares." .
reqvire:semanticReferenceOutsideContextIssueKind a reqvire:ValidationIssueKind ;
  reqvire:validationIssueKindName "semantic-reference-found-outside-context" ;
  reqvire:validationIssueKindMeaning "A semantic-contract SHACL reference points to an IRI declared by an ontology element that is outside the owning requirement's reachable capability ontology context." .
reqvire:semanticDuplicateDeclarationIssueKind a reqvire:ValidationIssueKind ;
  reqvire:validationIssueKindName "semantic-duplicate-declaration" ;
  reqvire:validationIssueKindMeaning "The same ontology term IRI is declared by multiple ontology elements." .
reqvire:semanticConflictingDeclarationIssueKind a reqvire:ValidationIssueKind ;
  reqvire:validationIssueKindName "semantic-conflicting-declaration" ;
  reqvire:validationIssueKindMeaning "The same ontology term IRI is declared with incompatible semantic roles across ontology elements." .

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

reqvire:attachmentHierarchyIndependenceRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "attachment-hierarchy-independence" ;
  reqvire:validationScope "attachments" ;
  reqvire:validationSeverity "error" ;
  reqvire:validationCondition "An element attaches a refinement already available through owner hierarchy or ancestor attachment propagation." ;
  reqvire:validationOutcome "Validation fails because the attachment is redundant or hides the intended dependency boundary." ;
  reqvire:validationRepair "Remove the redundant attachment or attach the contract at the highest valid boundary." .

reqvire:attachmentSubgraphDirectionRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "attachment-subgraph-direction" ;
  reqvire:validationScope "capability-root-subgraphs" ;
  reqvire:validationSeverity "error" ;
  reqvire:validationCondition "Two capability-root subgraphs attach refinements to each other in both directions." ;
  reqvire:validationOutcome "Validation fails because cross-subgraph attachment contracts must be one-directional." ;
  reqvire:validationRepair "Keep one dependency direction and move shared contracts into a common attached source if needed." .

reqvire:crossSectionDuplicateRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "cross-section-duplicate" ;
  reqvire:validationScope "element-subsections" ;
  reqvire:validationSeverity "error" ;
  reqvire:validationCondition "The same target appears in both Relations and Attachments for one element." ;
  reqvire:validationOutcome "Validation fails because the model cannot infer which semantic channel is intended." ;
  reqvire:validationRepair "Remove either the relation entry or the attachment entry." .

reqvire:semanticReferenceReachabilityValidationRule a reqvire:ValidationRule ;
  reqvire:validationRuleName "semantic-reference-reachability-validation" ;
  reqvire:validationScope "reachable-ontology-context" ;
  reqvire:validationSeverity "error" ;
  reqvire:validationCondition "A SHACL reference points to an ontology term that is not declared or is declared outside reachable capability-root context." ;
  reqvire:validationOutcome "Validation fails because semantic dependencies must remain visible to change impact." ;
  reqvire:validationRepair "Declare the term in reachable ontology context, attach the declaring ontology to the owning or consuming capability, or remove/update the reference." .

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
  reqvire:validationRepair "Replace with attachment/trace or remodel hierarchy under one capability root." .

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
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---
