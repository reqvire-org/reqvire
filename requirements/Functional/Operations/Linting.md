# Elements

### Model Linting

The system shall provide model linting capabilities to analyze model quality and detect issues in requirements relations.

#### Details
When linting the model, the system shall identify issues with model relations that may need attention.

When categorizing lint issues, the system shall distinguish between issues that can be automatically fixed and those requiring human judgment.

When reporting lint findings, the system shall provide clear categorization to help users understand what actions are needed.

When running the lint command, the system shall support both reporting mode and automatic fixing mode.

When filtering lint results, the system shall allow focusing on specific categories of issues.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Cross-Submodel Hierarchical Relation Detection](#cross-submodel-hierarchical-relation-detection)
  * derive: [Lint Auto-fix Capability](#lint-auto-fix-capability)
  * derive: [Multi-Branch Convergence Detection](#multi-branch-convergence-detection)
  * derive: [Redundant Hierarchical Relations Detection and Auto-Removal](#redundant-hierarchical-relations-detection-and-auto-removal)
  * derive: [Redundant Verify Relations Detection](#redundant-verify-relations-detection)
  * refinedBy: [Lint Output Specification](Specifications.md#lint-output-specification)
  * specify: [Linting Model Quality](../../Features/BehaviorValidationOperations.md#linting-model-quality)
---

### Cross-Submodel Hierarchical Relation Detection

Cross-submodel ownership validation for hierarchical relations during linting.

#### Details
When a user-created hierarchical relation target belongs to a different hierarchical root than its source, linting shall surface this as a cross-submodel violation that must be reviewed as an ownership-boundary issue.
The system shall use this detection during lint analysis to identify and report boundary crossings as manual-review findings.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Linting](#model-linting)
  * refinedBy: [Cross-Submodel Hierarchical Relation Detection Specification](Specifications.md#cross-submodel-hierarchical-relation-detection-specification)
  * satisfiedBy: [lint.rs](../../../core/src/lint.rs)
  * verifiedBy: [Lint Command Verification](Verifications/LintingVerifications.md#lint-command-verification)
---

### Lint Auto-fix Capability

The system shall provide automatic fixing capability for auto-fixable lint issues, applying changes directly to model files when the `--fix` flag is used.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Linting](#model-linting)
  * refinedBy: [Lint Auto-fix Capability Refinement Specification](Specifications.md#lint-auto-fix-capability-refinement-specification)
  * satisfiedBy: [lint.rs](../../../core/src/lint.rs)
  * verifiedBy: [Lint Command Verification](Verifications/LintingVerifications.md#lint-command-verification)
---

### Multi-Branch Convergence Detection

The system shall detect when an element reaches a common ancestor through multiple distinct branch paths without a direct relation, reporting these cases as needing manual review to determine if both branches are semantically necessary or if one represents a modeling error.

#### Metadata
  * type: requirement

#### Attachments
  * [Verification Trace Tree Construction](../Processing/Specifications.md#verification-trace-tree-construction)

#### Relations
  * derivedFrom: [Model Linting](#model-linting)
  * refinedBy: [Multi-Branch Convergence Detection Specification](Specifications.md#multi-branch-convergence-detection-specification)
  * satisfiedBy: [lint.rs](../../../core/src/lint.rs)
  * verifiedBy: [Lint Command Verification](Verifications/LintingVerifications.md#lint-command-verification)
---

### Redundant Hierarchical Relations Detection and Auto-Removal

The system shall detect and auto-remove redundant derivedFrom relations where an element has direct derivedFrom relations to both a requirement and its ancestor in the requirement hierarchy.

#### Metadata
  * type: requirement

#### Attachments
  * [Verification Trace Tree Construction](../Processing/Specifications.md#verification-trace-tree-construction)

#### Relations
  * derivedFrom: [Model Linting](#model-linting)
  * refinedBy: [Redundant Hierarchical Relations Specification](Specifications.md#redundant-hierarchical-relations-specification)
  * satisfiedBy: [lint.rs](../../../core/src/lint.rs)
  * verifiedBy: [Lint Command Verification](Verifications/LintingVerifications.md#lint-command-verification)
---

### Redundant Verify Relations Detection

The system shall detect redundant verify relations where a verification directly verifies both a child requirement and its ancestor, leveraging the existing verification trace tree logic from the Verification Trace Builder.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Verification Trace Tree Construction](../Processing/Specifications.md#verification-trace-tree-construction)

#### Relations
  * derivedFrom: [Model Linting](#model-linting)
  * refinedBy: [Redundant Verify Relations Detection Refinement Specification](Specifications.md#redundant-verify-relations-detection-refinement-specification)
  * satisfiedBy: [lint.rs](../../../core/src/lint.rs)
  * satisfiedBy: [trace_tree_builder.rs](../../../core/src/trace_tree_builder.rs)
  * verifiedBy: [Lint Command Verification](Verifications/LintingVerifications.md#lint-command-verification)
---
