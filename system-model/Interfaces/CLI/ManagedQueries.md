# Elements

### CLI Managed Query Artifacts

The system SHALL expose managed semantic query discovery, validation, and artifact access through the CLI interface.

#### Details
The interface consumes the core query contracts for selection, validation, namespace filtering, and deterministic rendering.

#### Contract References
  * [Semantic Query Discovery Specification](../../Semantics/SemanticQuerySpecifications.md#semantic-query-discovery-specification)
  * [Semantic Query Context Validation Specification](../../Semantics/SemanticQuerySpecifications.md#semantic-query-context-validation-specification)
  * [Semantic Query Artifact Export Specification](../../Semantics/SemanticQuerySpecifications.md#semantic-query-artifact-export-specification)
  * [Semantic Query Artifact Drift Check Specification](../../Semantics/SemanticQuerySpecifications.md#semantic-query-artifact-drift-check-specification)

#### Metadata
  * type: requirement

#### Relations
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * derivedFrom: [CLI Interface Structure](Commands.md#cli-interface-structure)
  * definedBy: [CLI Managed Query Artifacts Specification](Specifications.md#cli-managed-query-artifacts-specification)
---
