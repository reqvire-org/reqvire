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

#### Relations
  * specifiedBy: [Format Consistency Enforcement](Formatting/FormattingRequirements.md#format-consistency-enforcement)
  * specifiedBy: [Formatting Output](Formatting/FormattingRequirements.md#formatting-output)
  * specifiedBy: [Full Relations Insertion](Formatting/FormattingRequirements.md#full-relations-insertion)
  * specifiedBy: [Replace Absolute Links with Relative Links](Formatting/FormattingRequirements.md#replace-absolute-links-with-relative-links)
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

#### Relations
  * specifiedBy: [Model Linting](Linting/LintingRequirements.md#model-linting)
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

#### Relations
  * specifiedBy: [Default Requirement Type Assignment](../ModelStructure/ModelManagement.md#default-requirement-type-assignment)
  * specifiedBy: [Efficient Processing](../ModelStructure/ModelManagement.md#efficient-processing)
  * specifiedBy: [Element Manipulation Operations](../ModelStructure/ModelManagement.md#element-manipulation-operations)
  * specifiedBy: [Requirement Governance Metadata](../ModelStructure/ModelManagement.md#requirement-governance-metadata)
  * specifiedBy: [Template-Based Model Bootstrapping](../ModelStructure/ModelManagement.md#template-based-model-bootstrapping)
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

#### Relations
  * specifiedBy: [Validate Cross-Component Dependencies](Validation/ValidationRequirements.md#validate-cross-component-dependencies)
  * specifiedBy: [Validate Filesystem Structure](Validation/ValidationRequirements.md#validate-filesystem-structure)
  * specifiedBy: [Validate Internal Consistency](Validation/ValidationRequirements.md#validate-internal-consistency)
  * specifiedBy: [Validate Markdown Structure](Validation/ValidationRequirements.md#validate-markdown-structure)
  * specifiedBy: [Validate Relation Types](Validation/ValidationRequirements.md#validate-relation-types)
---
