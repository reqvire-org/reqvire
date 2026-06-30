# Elements

### CLI interface

The system shall provide command line interface (CLI) to faciliate model management.

#### Metadata
  * type: requirement

#### Relations
  * derive: [CLI Interface Structure](CLI/Commands.md#cli-interface-structure)
  * specify: [Command-Line Interface](InterfacesFeature.md#command-line-interface)
---

### Shared Core Operation Layer

The system shall provide transport-neutral Reqvire core operations that are reused by CLI and MCP adapters.

#### Details
- Core operation semantics shall live below CLI argument parsing and below MCP protocol wrapping.
- CLI command handlers shall parse user input, call shared operations, and render terminal or JSON output.
- MCP handlers shall map typed tool requests to shared operations and wrap results in MCP protocol responses.
- Read/report operations shall be shared before broader mutation operations, because mutation operations also own dry-run, diff, persistence, and cache invalidation behavior.
- Conditional mutation/report operations such as format and lint may be migrated when their preview/report path and mutating path preserve the same command and MCP side-effect controls.
- Shared operation result types shall preserve machine-readable evidence so CLI JSON and MCP structured responses do not diverge.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [MCP Contract Layer Boundary Specification](MCP/Specifications.md#mcp-contract-layer-boundary-specification)

#### Relations
  * definedBy: [Shared Core Operation Layer Specification](Specifications.md#shared-core-operation-layer-specification)
  * derivedFrom: [CLI interface](#cli-interface)
  * satisfiedBy: [cli.rs](../../crates/reqvire-cli/src/cli.rs)
  * satisfiedBy: [mod.rs](../../crates/reqvire-core/src/operations/mod.rs)
  * satisfiedBy: [read_tools.rs](../../crates/reqvire-core/src/tool_interface/read_tools.rs)
  * verifiedBy: [MCP Shared Operation Contracts Verification](../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-shared-operation-contracts-verification)
---

### MCP Interface

The system shall provide a Model Context Protocol interface for external tools, IDE integrations, CI adapters, and AI agents to access Reqvire model capabilities through typed tool interfaces.

#### Details
- The MCP interface shall adapt Reqvire core behavior to MCP clients.
- The MCP interface shall avoid becoming a second model engine.
- The MCP interface shall avoid becoming a generic shell gateway.
- The MCP interface shall avoid becoming a client-specific abstraction.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [MCP Interface Boundary Specification](Specifications.md#mcp-interface-boundary-specification)
  * derive: [MCP Access Control Baseline](MCP/Tools.md#mcp-access-control-baseline)
  * derive: [MCP Adapter Boundary](MCP/Tools.md#mcp-adapter-boundary)
  * derive: [MCP Compatibility Versioning](MCP/Tools.md#mcp-compatibility-versioning)
  * derive: [MCP Model Evidence Tools](MCP/Tools.md#mcp-model-evidence-tools)
  * derive: [MCP Mutation Concurrency Control](MCP/Tools.md#mcp-mutation-concurrency-control)
  * derive: [MCP Mutation Execution Flow](MCP/Tools.md#mcp-mutation-execution-flow)
  * derive: [MCP Mutation Tool Safety](MCP/Tools.md#mcp-mutation-tool-safety)
  * derive: [MCP Protocol Standard Conformance](MCP/Tools.md#mcp-protocol-standard-conformance)
  * derive: [MCP Quality Traceability Tools](MCP/Tools.md#mcp-quality-traceability-tools)
  * derive: [MCP Resource Interface](MCP/Tools.md#mcp-resource-interface)
  * derive: [MCP Server Command](MCP/Tools.md#mcp-server-command)
  * derive: [MCP Server State and Cache](MCP/Tools.md#mcp-server-state-and-cache)
  * derive: [MCP Shared Operation Interfaces](MCP/Tools.md#mcp-shared-operation-interfaces)
  * derive: [MCP Streamable HTTP Transport](MCP/Tools.md#mcp-streamable-http-transport)
  * derive: [MCP Streamable HTTP Transport Safety](MCP/Tools.md#mcp-streamable-http-transport-safety)
  * derive: [MCP Structured Payload Interfaces](MCP/Tools.md#mcp-structured-payload-interfaces)
  * derive: [MCP Tool Exposure Scope](MCP/Tools.md#mcp-tool-exposure-scope)
  * derive: [MCP Tool Side Effect Classification](MCP/Tools.md#mcp-tool-side-effect-classification)
  * derive: [MCP Workspace Session Tools](MCP/Tools.md#mcp-workspace-session-tools)
  * specify: [MCP Tool Interface](InterfacesFeature.md#mcp-tool-interface)
---

### Web Interface

The system SHALL provide a web-based interface to browse the System model, including Explorer-rendered diagrams, reports, and verification traces.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Web Interface Contract Specification](Specifications.md#web-interface-contract-specification)
  * derive: [Serve Command](WebExplorer/Capabilities.md#serve-command)
  * derive: [Served Explorer Browser Interface](WebExplorer/Capabilities.md#served-explorer-browser-interface)
  * specify: [Web Explorer Interface](InterfacesFeature.md#web-explorer-interface)
---
