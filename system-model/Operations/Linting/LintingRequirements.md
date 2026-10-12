# Elements

### Model Linting

The system SHALL analyze model quality and report categorized relation findings.

#### Details
The system SHALL distinguish safe auto-fixable findings from findings requiring human judgment and support category filtering.

When a permitted repair operation is requested, the system SHALL apply only eligible safe findings through shared model operations.

#### Metadata
  * type: requirement

#### Contract References
  * [Model Interface Operation Boundary Specification](../../Interfaces/Specifications.md#model-interface-operation-boundary-specification)

#### Relations
  * constrainedBy: [Lint Rule Metadata Shape](../../Ontologies/BehaviorValidationOperations.md#lint-rule-metadata-shape)
  * definedBy: [Lint Output Specification](Specifications.md#lint-output-specification)
  * derive: [Cross-Submodel Hierarchical Relation Detection](#cross-submodel-hierarchical-relation-detection)
  * derive: [Lint Auto-fix Capability](#lint-auto-fix-capability)
  * derive: [Multi-Branch Convergence Detection](#multi-branch-convergence-detection)
  * derive: [Redundant Hierarchical Relations Detection and Auto-Removal](#redundant-hierarchical-relations-detection-and-auto-removal)
  * derive: [Redundant Verify Relations Detection](#redundant-verify-relations-detection)
  * derive: [Rust Workspace Lint Policy](#rust-workspace-lint-policy)
  * specify: [Linting Model Quality](../BehaviorValidationOperationsFeature.md#linting-model-quality)
---

### Cross-Submodel Hierarchical Relation Detection

Cross-submodel ownership validation for hierarchical relations during linting.

#### Details
When a user-created hierarchical relation target belongs to a different hierarchical root than its source, linting shall surface this as a cross-submodel violation that must be reviewed as an ownership-boundary issue.
The system shall use this detection during lint analysis to identify and report boundary crossings as manual-review findings.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Cross-Submodel Hierarchical Relation Detection Specification](Specifications.md#cross-submodel-hierarchical-relation-detection-specification)
  * derivedFrom: [Model Linting](#model-linting)
  * satisfiedBy: [lint.rs](../../../crates/reqvire-core/src/lint.rs)
  * verifiedBy: [Lint Command Verification](../../Verifications/Operations/Linting/LintingVerifications.md#lint-command-verification)
---

### Lint Auto-fix Capability

When a permitted lint repair is requested, the system SHALL apply only safe auto-fixable changes to the complete validated model candidate and preserve manual-review findings.

#### Metadata
  * type: requirement

#### Contract References
  * [Operation Command Contract Specification](../../ModelStructure/Specifications.md#operation-command-contract-specification)
  * [Canonical Model Mutation Persistence Specification](../ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)

#### Relations
  * definedBy: [Lint Auto-fix Capability Contract Specification](Specifications.md#lint-auto-fix-capability-contract-specification)
  * derivedFrom: [Model Linting](#model-linting)
  * verifiedBy: [Lint Command Verification](../../Verifications/Operations/Linting/LintingVerifications.md#lint-command-verification)
---

### Multi-Branch Convergence Detection

The system shall detect when an element reaches a common ancestor through multiple distinct branch paths without a direct relation, reporting these cases as needing manual review to determine if both branches are semantically necessary or if one represents a modeling error.

#### Metadata
  * type: requirement

#### Contract References
  * [Verification Trace Tree Construction](../../Verification/Traceability/Specifications.md#verification-trace-tree-construction)

#### Relations
  * definedBy: [Multi-Branch Convergence Detection Specification](Specifications.md#multi-branch-convergence-detection-specification)
  * derivedFrom: [Model Linting](#model-linting)
  * satisfiedBy: [lint.rs](../../../crates/reqvire-core/src/lint.rs)
  * verifiedBy: [Lint Command Verification](../../Verifications/Operations/Linting/LintingVerifications.md#lint-command-verification)
---

### Redundant Hierarchical Relations Detection and Auto-Removal

The system shall detect and auto-remove redundant derivedFrom relations where an element has direct derivedFrom relations to both a requirement and its ancestor in the requirement hierarchy.

#### Metadata
  * type: requirement

#### Contract References
  * [Verification Trace Tree Construction](../../Verification/Traceability/Specifications.md#verification-trace-tree-construction)

#### Relations
  * definedBy: [Redundant Hierarchical Relations Specification](Specifications.md#redundant-hierarchical-relations-specification)
  * derivedFrom: [Model Linting](#model-linting)
  * satisfiedBy: [lint.rs](../../../crates/reqvire-core/src/lint.rs)
  * verifiedBy: [Lint Command Verification](../../Verifications/Operations/Linting/LintingVerifications.md#lint-command-verification)
---

### Redundant Verify Relations Detection

The system shall detect redundant direct verify relations.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Contract References
  * [Verification Trace Tree Construction](../../Verification/Traceability/Specifications.md#verification-trace-tree-construction)

#### Relations
  * definedBy: [Redundant Verify Relations Detection Contract Specification](Specifications.md#redundant-verify-relations-detection-contract-specification)
  * derivedFrom: [Model Linting](#model-linting)
  * satisfiedBy: [lint.rs](../../../crates/reqvire-core/src/lint.rs)
  * satisfiedBy: [trace_tree_builder.rs](../../../crates/reqvire-core/src/trace_tree_builder.rs)
  * verifiedBy: [Lint Command Verification](../../Verifications/Operations/Linting/LintingVerifications.md#lint-command-verification)
---

### Rust Workspace Lint Policy

The system shall define and run a Rust workspace lint policy for implementation quality checks.

#### Details
- The workspace shall define shared clippy lint levels in the root Cargo manifest.
- The workspace shall provide a repeatable clippy command target.
- The lint policy shall warn on targeted risks identified by code review, including broad clippy checks, `clippy::nursery`, `unwrap_used`, and reference-counted clone mistakes.
- Pedantic linting may remain staged separately when enabling it would create broad low-value churn.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Rust Workspace Lint Policy Specification](Specifications.md#rust-workspace-lint-policy-specification)
  * derivedFrom: [Model Linting](#model-linting)
  * satisfiedBy: [Cargo.toml](../../../Cargo.toml)
  * satisfiedBy: [Makefile](../../../Makefile)
---
