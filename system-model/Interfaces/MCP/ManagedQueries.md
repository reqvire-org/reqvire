# Elements

### MCP Managed Query Artifacts

The system SHALL expose managed semantic query discovery, validation, and artifact access through the MCP interface.

#### Details
The interface consumes the core query contracts for selection, validation, namespace filtering, and deterministic rendering.

#### Contract References
  * [Semantic Query Discovery Specification](../../Semantics/SemanticQuerySpecifications.md#semantic-query-discovery-specification)
  * [Semantic Query Context Validation Specification](../../Semantics/SemanticQuerySpecifications.md#semantic-query-context-validation-specification)
  * [Semantic Query Artifact Export Specification](../../Semantics/SemanticQuerySpecifications.md#semantic-query-artifact-export-specification)

#### Metadata
  * type: requirement

#### Relations
  * satisfiedBy: [semantic_tools.rs](../../../crates/reqvire-core/src/tool_interface/semantic_tools.rs)
  * derivedFrom: [MCP Semantic Query Tools](Tools.md#mcp-semantic-query-tools)
  * definedBy: [MCP Managed Query Artifacts Specification](Specifications.md#mcp-managed-query-artifacts-specification)
---
