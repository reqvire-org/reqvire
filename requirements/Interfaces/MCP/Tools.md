# Elements

### MCP Protocol Standard Conformance

The system shall implement the Model Context Protocol using standard MCP lifecycle, capability, tool, and resource messages.

#### Details
- The MCP interface shall negotiate protocol compatibility through MCP lifecycle initialization.
- The MCP interface shall expose a declared MCP protocol revision.
- The MCP interface shall advertise only standard MCP capability objects at the protocol boundary.
- The MCP interface shall expose Reqvire-specific workspace, model, version, and mutation-mode state through Reqvire MCP tools or resources.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [MCP Interface](../Interfaces.md#mcp-interface)
  * refinedBy: [MCP Protocol Standard Conformance Specification](Specifications.md#mcp-protocol-standard-conformance-specification)
  * verifiedBy: [MCP Protocol Standard Conformance Verification](Verifications/MCPVerifications.md#mcp-protocol-standard-conformance-verification)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
---

### MCP Server Command

The system shall provide `reqvire mcp` as the command that starts the Reqvire MCP server.

#### Details
- The MCP server command shall start protocol service mode for the current Reqvire workspace.
- The MCP server command shall keep server startup behavior outside the MCP tool surface.
- The MCP server command shall expose read/report tools by default.
- The MCP server command shall expose mutation tools only when mutation capability is explicitly enabled at startup.
- The MCP server command shall support opt-in element size estimates when explicitly enabled at startup.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [MCP Interface](../Interfaces.md#mcp-interface)
  * refinedBy: [MCP Server Command Specification](Specifications.md#mcp-server-command-specification)
  * refinedBy: [MCP Size Estimate Startup Specification](Specifications.md#mcp-size-estimate-startup-specification)
  * verifiedBy: [MCP Server Command Verification](Verifications/MCPVerifications.md#mcp-server-command-verification)
  * verifiedBy: [MCP Size Estimate Startup Verification](Verifications/MCPVerifications.md#mcp-size-estimate-startup-verification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [main.rs](../../../cli/src/main.rs)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * satisfiedBy: [tool_interface.rs](../../../core/src/tool_interface.rs)
---

### MCP Streamable HTTP Transport

The system shall provide MCP service through RMCP Streamable HTTP transport.

#### Details
- The MCP server shall use Streamable HTTP as the only supported MCP transport.
- The MCP server shall start an RMCP-backed HTTP endpoint by default.
- The MCP server shall not expose newline-delimited stdio JSON-RPC compatibility mode.
- MCP transport mechanics shall not change Reqvire tool semantics.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [MCP Interface](../Interfaces.md#mcp-interface)
  * refinedBy: [MCP Streamable HTTP Transport Specification](Specifications.md#mcp-streamable-http-transport-specification)
  * verifiedBy: [MCP HTTP Transport End-to-End Verification](Verifications/MCPVerifications.md#mcp-http-transport-end-to-end-verification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
---

### MCP Streamable HTTP Transport Safety

The system shall implement Streamable HTTP transport with local-safe defaults and MCP-compliant HTTP behavior when HTTP transport is enabled.

#### Details
- The MCP Streamable HTTP transport shall provide MCP-compliant HTTP request handling.
- The MCP Streamable HTTP transport shall use local-safe defaults.
- The MCP Streamable HTTP transport shall protect local workspaces from browser-origin and non-local exposure risks.
- The MCP Streamable HTTP transport shall require explicit mutation enablement before exposing mutation tools.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [MCP Interface](../Interfaces.md#mcp-interface)
  * refinedBy: [MCP Streamable HTTP Transport Safety Specification](Specifications.md#mcp-streamable-http-transport-safety-specification)
  * verifiedBy: [MCP HTTP Transport End-to-End Verification](Verifications/MCPVerifications.md#mcp-http-transport-end-to-end-verification)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
---

### MCP Mutation Concurrency Control

The system shall serialize mutation execution per workspace when the MCP transport can receive concurrent client requests.

#### Details
- The MCP server shall prevent concurrent mutation requests from interleaving writes for the same workspace.
- The MCP server shall preserve Reqvire filesystem persistence guarantees under multi-request transports.
- The MCP server shall make observed model revision visible to clients for read responses.
- The MCP server shall keep post-mutation model state synchronized before serving dependent reads.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [MCP Interface](../Interfaces.md#mcp-interface)
  * refinedBy: [MCP Mutation Concurrency Control Specification](Specifications.md#mcp-mutation-concurrency-control-specification)
  * verifiedBy: [MCP HTTP Transport End-to-End Verification](Verifications/MCPVerifications.md#mcp-http-transport-end-to-end-verification)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
---

### MCP Shared Operation Interfaces

The system shall expose MCP tools through shared typed request and result interfaces for matching Reqvire operations.

#### Details
- The MCP interface shall expose Reqvire operations through typed request and result interfaces.
- MCP tools shall reuse Reqvire core operation semantics.
- MCP protocol concerns shall remain at the adapter boundary.
- MCP tools shall provide machine-readable discovery metadata for clients.
- Shared Reqvire tool interfaces shall be exposed by the Reqvire library so in-process applications can discover and call the same operations without using MCP transport.

#### Metadata
  * type: requirement

#### Attachments
  * [JSON Output Structure](../../Functional/Output/Specifications.md#json-output-structure)

#### Relations
  * derivedFrom: [MCP Interface](../Interfaces.md#mcp-interface)
  * refinedBy: [MCP Shared Operation Contracts Specification](Specifications.md#mcp-shared-operation-contracts-specification)
  * refinedBy: [MCP Tool Call Contracts Specification](Specifications.md#mcp-tool-call-contracts-specification)
  * verifiedBy: [MCP Shared Operation Contracts Verification](Verifications/MCPVerifications.md#mcp-shared-operation-contracts-verification)
  * verifiedBy: [MCP Tool Call Contracts Verification](Verifications/MCPVerifications.md#mcp-tool-call-contracts-verification)
  * satisfiedBy: [tool_interface.rs](../../../core/src/tool_interface.rs)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
---

### MCP Adapter Boundary

The system shall keep Reqvire MCP tool interfaces protocol-neutral below the MCP adapter.

#### Details
- The MCP adapter boundary shall remain independent of MCP transport implementation details.
- Shared MCP-facing Reqvire structures shall depend on Reqvire model concepts and core operations.
- The MCP adapter shall map shared Reqvire structures into MCP protocol messages.
- The MCP adapter shall call the shared Reqvire tool registry instead of owning a separate tool implementation.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [MCP Interface](../Interfaces.md#mcp-interface)
  * refinedBy: [MCP Contract Layer Boundary Specification](Specifications.md#mcp-contract-layer-boundary-specification)
  * verifiedBy: [MCP Contract Layer Boundary Verification](Verifications/MCPVerifications.md#mcp-contract-layer-boundary-verification)
  * satisfiedBy: [tool_interface.rs](../../../core/src/tool_interface.rs)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
---

### MCP Server State and Cache

The system shall keep MCP server cached model state subordinate to Reqvire source files and Reqvire core parsing.

#### Details
- The MCP server shall keep Reqvire source files as the durable source of truth.
- The MCP server shall keep cached model state subordinate to Reqvire core parsing.
- The MCP server shall report enough revision state for clients to reason about cache freshness.
- The MCP server shall refresh stale model state before returning authoritative model evidence.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [MCP Interface](../Interfaces.md#mcp-interface)
  * refinedBy: [MCP Server State and Cache Specification](Specifications.md#mcp-server-state-and-cache-specification)
  * verifiedBy: [MCP Server State and Cache Verification](Verifications/MCPVerifications.md#mcp-server-state-and-cache-verification)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
---

### MCP Tool Exposure Scope

The system shall expose only supported Reqvire model operations as MCP tools.

#### Details
- The MCP server shall expose only stable Reqvire model operations as tools.
- The MCP server shall not expose arbitrary command execution.
- The MCP server shall not expose internal or server-management commands as tools.
- The MCP server shall treat successful validation as a startup prerequisite instead of a model tool.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [MCP Interface](../Interfaces.md#mcp-interface)
  * refinedBy: [MCP Tool Exposure Scope Specification](Specifications.md#mcp-tool-exposure-scope-specification)
  * verifiedBy: [MCP Tool Exposure Scope Verification](Verifications/MCPVerifications.md#mcp-tool-exposure-scope-verification)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
---

### MCP Workspace Session Tools

The system shall expose MCP-only workspace/session tools for workspace status, tool interface discovery, and model revision metadata.

#### Details
- The MCP interface shall provide workspace/session tools that have no direct CLI command equivalent.
- Workspace/session tools shall report workspace identity and revision state.
- Workspace/session tools shall report Reqvire and MCP compatibility state.
- Workspace/session tools shall not mutate the model or filesystem.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [MCP Interface](../Interfaces.md#mcp-interface)
  * refinedBy: [MCP Workspace Session Tools Specification](Specifications.md#mcp-workspace-session-tools-specification)
  * verifiedBy: [MCP Workspace Session Tools Verification](Verifications/MCPVerifications.md#mcp-workspace-session-tools-verification)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
---

### MCP Structured Payload Interfaces

The system shall provide MCP structured payload interfaces derived from shared Reqvire operation results.

#### Details
- The MCP interface shall provide structured machine-readable tool results.
- MCP structured results shall remain consistent with authoritative Reqvire operation results.
- MCP structured results shall expose stable model evidence semantics.
- MCP clients shall not need to parse human terminal output to recover authoritative model evidence.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [MCP Interface](../Interfaces.md#mcp-interface)
  * refinedBy: [MCP Structured Payload Contracts Specification](Specifications.md#mcp-structured-payload-contracts-specification)
  * verifiedBy: [MCP Structured Payload Contracts Verification](Verifications/MCPVerifications.md#mcp-structured-payload-contracts-verification)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
---

### MCP Model Evidence Tools

The system shall expose MCP read tools that return model evidence needed by external tools and AI agents.

#### Details
- The MCP interface shall expose read tools for authoritative Reqvire model evidence.
- Model evidence tools shall support element lookup, model structure, containment, collection, submodel analysis, and ontology/SHACL semantic collection.
- Model evidence tools shall expose the canonical capability/requirement/ontology model, including `ontology` elements, `#### Concept References`, and capability-owned or requirement-owned `semantic-contract` shape profiles where the underlying Reqvire operation returns them.
- Model evidence tools shall include revision metadata when model state affects interpretation.
- Model evidence tools shall not mutate the model or filesystem.

#### Metadata
  * type: requirement

#### Attachments
  * [Requirement Governance Metadata Specification](../../Functional/Core/Specifications.md#requirement-governance-metadata-specification)
  * [Flexible Search Type Filtering Refinement Specification](../../Functional/Output/Specifications.md#flexible-search-type-filtering-refinement-specification)
  * [Containment View Report Refinement Specification](../../Functional/Output/Specifications.md#containment-view-report-refinement-specification)
  * [Collect Content Specification](../../Functional/Output/Specifications.md#collect-content-specification)
  * [Requirement Submodels Report Specification](../../Functional/Output/Specifications.md#requirement-submodels-report-specification)
  * [Ontology Collection Output Specification](../../Functional/Output/Specifications.md#ontology-collection-output-specification)

#### Relations
  * derivedFrom: [MCP Interface](../Interfaces.md#mcp-interface)
  * refinedBy: [MCP Model Evidence Tools Specification](Specifications.md#mcp-model-evidence-tools-specification)
  * verifiedBy: [MCP Model Evidence Tools Verification](Verifications/MCPVerifications.md#mcp-model-evidence-tools-verification)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
---

### MCP Quality Traceability Tools

The system shall expose MCP read tools for linting, coverage, verification traces, resources, and change impact after startup validation has passed.

#### Details
- The MCP interface shall expose read tools for Reqvire quality and traceability evidence.
- Quality and traceability tools shall provide machine-readable diagnostics.
- Quality and traceability tools shall use validated model state after MCP startup validation succeeds.
- Quality and traceability tools shall not require clients to execute shell commands.

#### Metadata
  * type: requirement

#### Attachments
  * [Lint Output Specification](../../Functional/Operations/Specifications.md#lint-output-specification)
  * [Requirement Implementation Coverage Logic Specification](../../Functional/Output/Specifications.md#requirement-implementation-coverage-logic-specification)
  * [Verification Trace Tree Construction](../../Functional/Processing/Specifications.md#verification-trace-tree-construction)
  * [Resources Report Format Specification](../../Functional/Output/Specifications.md#resources-report-format-specification)
  * [Impact Scope Computation Specification](../../Functional/Processing/Specifications.md#impact-scope-computation-specification)

#### Relations
  * derivedFrom: [MCP Interface](../Interfaces.md#mcp-interface)
  * refinedBy: [MCP Quality Traceability Tools Specification](Specifications.md#mcp-quality-traceability-tools-specification)
  * verifiedBy: [MCP Quality Traceability Tools Verification](Verifications/MCPVerifications.md#mcp-quality-traceability-tools-verification)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
---

### MCP Tool Side Effect Classification

The system shall classify every MCP tool by side-effect behavior so clients and tests can distinguish read-only tools, conditionally mutating tools, and mutation tools.

#### Details
- The MCP interface shall classify each tool by side-effect behavior.
- MCP tool discovery shall distinguish read-only tools from mutation-capable tools.
- MCP tool discovery shall omit mutation-class tools unless mutation capability is enabled.
- MCP tool metadata shall allow clients to reason about mutation risk before calling a tool.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [MCP Interface](../Interfaces.md#mcp-interface)
  * refinedBy: [MCP Tool Side Effect Classification Specification](Specifications.md#mcp-tool-side-effect-classification-specification)
  * verifiedBy: [MCP Tool Side Effect Classification Verification](Verifications/MCPVerifications.md#mcp-tool-side-effect-classification-verification)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
---

### MCP Mutation Tool Safety

The system shall expose mutation tools only through typed Reqvire core operations that preserve operation-specific preview behavior, validation, persistence guarantees, and post-mutation diagnostics.

#### Details
- The MCP interface shall expose mutation tools only after explicit mutation enablement.
- MCP mutation tools shall use Reqvire core mutation logic.
- MCP mutation tools shall preserve Reqvire semantic model validation, including ontology attachment compatibility, semantic-contract SHACL reference reachability, concept-reference resolution, and single ontology-root validation.
- MCP mutation tools shall preserve Reqvire filesystem persistence behavior.
- MCP mutation results shall report changed model evidence.
- MCP mutation execution shall refresh MCP-visible model state after successful mutation.

#### Metadata
  * type: requirement

#### Attachments
  * [Dry-Run Mode Behavior](../../Functional/Operations/Behaviors.md#dry-run-mode-behavior)
  * [File Persistence Behavior](../../Functional/Operations/Behaviors.md#file-persistence-behavior)
  * [Create Element Workflow Specification](../../Functional/Operations/Specifications.md#create-element-workflow-specification)
  * [Delete Element Workflow Specification](../../Functional/Operations/Specifications.md#delete-element-workflow-specification)
  * [Move Element Workflow Specification](../../Functional/Operations/Specifications.md#move-element-workflow-specification)
  * [Rename Element Operation Refinement Specification](../../Functional/Operations/Specifications.md#rename-element-operation-refinement-specification)
  * [Merge Element Workflow Specification](../../Functional/Operations/Specifications.md#merge-element-workflow-specification)
  * [Move File Operation Refinement Specification](../../Functional/Operations/Specifications.md#move-file-operation-refinement-specification)
  * [Relation Operations Specification](../../Functional/Operations/Specifications.md#relation-operations-specification)
  * [Atomic Relation Relink Workflow Specification](../../Functional/Operations/Specifications.md#atomic-relation-relink-workflow-specification)
  * [Relation Consistency Maintenance Refinement Specification](../../Functional/Operations/Specifications.md#relation-consistency-maintenance-refinement-specification)

#### Relations
  * derivedFrom: [MCP Interface](../Interfaces.md#mcp-interface)
  * refinedBy: [MCP Mutation Tool Safety Specification](Specifications.md#mcp-mutation-tool-safety-specification)
  * verifiedBy: [MCP Mutation Tool Safety Verification](Verifications/MCPVerifications.md#mcp-mutation-tool-safety-verification)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [format.rs](../../../core/src/format.rs)
---

### MCP Mutation Execution Flow

The system shall execute MCP mutations through deterministic operation-specific preview and execution behavior backed by Reqvire core.

#### Details
- MCP mutation execution shall support operation-specific preview and execution controls.
- MCP mutation execution shall use Reqvire core for preview and execution semantics.
- MCP mutation execution shall report changed files, diffs, diagnostics, and affected scope when available.
- MCP mutation execution shall synchronize MCP-visible model state after successful execution.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [MCP Interface](../Interfaces.md#mcp-interface)
  * refinedBy: [MCP Mutation Execution Flow Specification](Specifications.md#mcp-mutation-execution-flow-specification)
  * verifiedBy: [MCP Mutation Execution Flow Verification](Verifications/MCPVerifications.md#mcp-mutation-execution-flow-verification)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [format.rs](../../../core/src/format.rs)
---

### MCP Resource Interface

The system shall expose MCP resources only as read-only, revision-tagged views of workspace, model, element, file, and report state.

#### Details
- The MCP interface shall expose read-only resources for model browsing.
- MCP resources shall represent authoritative Reqvire model or report state.
- MCP resources shall not mutate the filesystem.
- MCP resources shall not become a second model source of truth.

#### Metadata
  * type: requirement

#### Attachments
  * [Resources Report Format Specification](../../Functional/Output/Specifications.md#resources-report-format-specification)
  * [Containment View Report Refinement Specification](../../Functional/Output/Specifications.md#containment-view-report-refinement-specification)
  * [Requirement Submodels Report Specification](../../Functional/Output/Specifications.md#requirement-submodels-report-specification)

#### Relations
  * derivedFrom: [MCP Interface](../Interfaces.md#mcp-interface)
  * refinedBy: [MCP Resource Interface Specification](Specifications.md#mcp-resource-interface-specification)
  * verifiedBy: [MCP Resource Interface Verification](Verifications/MCPVerifications.md#mcp-resource-interface-verification)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
---

### MCP Compatibility Versioning

The system shall expose MCP protocol compatibility and Reqvire tool interface compatibility distinctly from the Reqvire binary version.

#### Details
- The MCP interface shall expose MCP protocol revision separately from Reqvire binary version.
- The MCP interface shall expose Reqvire tool interface version separately from Reqvire binary version.
- MCP clients shall be able to detect protocol and tool compatibility during startup or status checks.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [MCP Interface](../Interfaces.md#mcp-interface)
  * refinedBy: [MCP Contract Versioning Specification](Specifications.md#mcp-contract-versioning-specification)
  * verifiedBy: [MCP Contract Versioning Verification](Verifications/MCPVerifications.md#mcp-contract-versioning-verification)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
---

### MCP Access Control Baseline

The system shall provide safe local-first MCP defaults that avoid arbitrary shell execution, arbitrary filesystem reads, and unguarded mutations.

#### Details
- The MCP interface shall use safe local-first defaults.
- The MCP interface shall not expose arbitrary shell execution.
- The MCP interface shall not expose arbitrary filesystem reads.
- The MCP interface shall limit file evidence to Reqvire model evidence.
- The MCP interface shall not fetch external URLs by default.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [MCP Interface](../Interfaces.md#mcp-interface)
  * refinedBy: [MCP Access Control Baseline Specification](Specifications.md#mcp-access-control-baseline-specification)
  * verifiedBy: [MCP Access Control Baseline Verification](Verifications/MCPVerifications.md#mcp-access-control-baseline-verification)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
---
