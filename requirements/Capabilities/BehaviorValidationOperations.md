# Elements

### Formatting Model Documents

As a **System Engineer**, I want Reqvire to format model documents deterministically, so that model files stay readable, reviewable, and stable in diffs without changing model meaning.

#### Details
Formatting model documents is the capability for normalizing Reqvire Markdown structure, relation ordering, duplicate removal, relative links, and formatting diff output without changing model meaning.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: high
  * risk: medium
  * status: approved

#### Attachments
  * [Reqvire Core Element Ontology](../Ontologies/Core.md#reqvire-core-element-ontology)
  * [Reqvire Formatting Ontology](../Ontologies/BehaviorValidationOperations.md#reqvire-formatting-ontology)

#### Relations
  * specifiedBy: [Format Consistency Enforcement](../Functional/Operations/Formatting.md#format-consistency-enforcement)
  * specifiedBy: [Formatting Output](../Functional/Operations/Formatting.md#formatting-output)
  * specifiedBy: [Full Relations Insertion](../Functional/Operations/Formatting.md#full-relations-insertion)
  * specifiedBy: [Replace Absolute Links with Relative Links](../Functional/Operations/Formatting.md#replace-absolute-links-with-relative-links)
---

### Linting Model Quality

As a **System Engineer**, I want Reqvire to lint model quality issues that are suspicious but not always invalid, so that I can review and repair weak structure before it becomes misleading traceability.

#### Details
Linting model quality is the capability for auditable model quality checks that warn about suspicious structure, redundant relations, cross-submodel couplings, semantic reference context, and repairable quality issues.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Attachments
  * [Reqvire Core Element Ontology](../Ontologies/Core.md#reqvire-core-element-ontology)
  * [Reqvire Validation Ontology](../Ontologies/BehaviorValidationOperations.md#reqvire-validation-ontology)
  * [Reqvire Linting Ontology](../Ontologies/BehaviorValidationOperations.md#reqvire-linting-ontology)

#### Relations
  * specifiedBy: [Model Linting](../Functional/Operations/Linting.md#model-linting)
---

### Operating on Model Elements

As a **System Engineer**, I want to create, modify, move, delete, link, unlink, and merge model elements through Reqvire operations, so that I can evolve the model safely while preserving traceability and semantic consistency.

#### Details
Operating on model elements is the capability for user and programmatic operations that create, modify, move, delete, link, unlink, merge, and otherwise maintain model elements.

Operation requirements define concrete command inputs, validation gates, dry-run behavior, persistence behavior, and the model invariants each operation must preserve.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: high
  * risk: high
  * status: approved

#### Attachments
  * [Reqvire Core Element Ontology](../Ontologies/Core.md#reqvire-core-element-ontology)
  * [Reqvire Relation Ontology](../Ontologies/RelationsAndImpact.md#reqvire-relation-ontology)
  * [Reqvire Validation Ontology](../Ontologies/BehaviorValidationOperations.md#reqvire-validation-ontology)
  * [Reqvire Operation Ontology](../Ontologies/BehaviorValidationOperations.md#reqvire-operation-ontology)

#### Relations
  * specifiedBy: [Default Requirement Type Assignment](../Functional/Core/ModelManagement.md#default-requirement-type-assignment)
  * specifiedBy: [Efficient Processing](../Functional/Core/ModelManagement.md#efficient-processing)
  * specifiedBy: [Element Manipulation Operations](../Functional/Core/ModelManagement.md#element-manipulation-operations)
  * specifiedBy: [Requirement Governance Metadata](../Functional/Core/ModelManagement.md#requirement-governance-metadata)
  * specifiedBy: [Template-Based Model Bootstrapping](../Functional/Core/ModelManagement.md#template-based-model-bootstrapping)
---

### Reqvire Behavior Rule Ontology Shape Profile

Defines SHACL constraints for behavior rules, state transitions, and behavior refinements.

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
  * refine: [Behavior Rule Semantic Contract](../Functional/Core/Validation.md#behavior-rule-semantic-contract)
---

### Reqvire Linting Ontology Shape Profile

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
  * refine: [Lint Rule Semantic Contract](../Functional/Operations/Linting.md#lint-rule-semantic-contract)
---

### Reqvire Validation Ontology Shape Profile

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
  * refine: [Validation Rule Semantic Contract](../Functional/Core/Validation.md#validation-rule-semantic-contract)
---

### Validating Structures

As a **System Engineer**, I want Reqvire to validate model structure before reports, mutations, and automation rely on it, so that broken relations, invalid contracts, and unsafe model states are caught with actionable diagnostics.

#### Details
Validating structures is the capability for structural model validation, semantic contract validation, lint classifications, and mutation safety gates.

Validation requirements define when model state is acceptable, when a mutation must be blocked before persistence, and how diagnostics guide the user to repair the model.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: high
  * risk: high
  * status: approved

#### Attachments
  * [Reqvire Core Element Ontology](../Ontologies/Core.md#reqvire-core-element-ontology)
  * [Reqvire Relation Ontology](../Ontologies/RelationsAndImpact.md#reqvire-relation-ontology)
  * [Reqvire Requirement Ontology](../Ontologies/CapabilityRequirementModel.md#reqvire-requirement-ontology)
  * [Reqvire Semantic Contract Ontology](../Ontologies/CapabilityRequirementModel.md#reqvire-semantic-contract-ontology)
  * [Reqvire Validation Ontology](../Ontologies/BehaviorValidationOperations.md#reqvire-validation-ontology)
  * [Reqvire Behavior Rule Ontology](../Ontologies/BehaviorValidationOperations.md#reqvire-behavior-rule-ontology)

#### Relations
  * specifiedBy: [Validate Cross-Component Dependencies](../Functional/Core/Validation.md#validate-cross-component-dependencies)
  * specifiedBy: [Validate Filesystem Structure](../Functional/Core/Validation.md#validate-filesystem-structure)
  * specifiedBy: [Validate Internal Consistency](../Functional/Core/Validation.md#validate-internal-consistency)
  * specifiedBy: [Validate Markdown Structure](../Functional/Core/Validation.md#validate-markdown-structure)
  * specifiedBy: [Validate Relation Types](../Functional/Core/Validation.md#validate-relation-types)
---
