# Elements

### CLI interface

The system shall provide command line interface (CLI) to faciliate model management.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [CLI Interface Structure](CLI/Commands.md#cli-interface-structure)
  * derivedFrom: [System Model Interfaces](../UserStories.md#system-model-interfaces)
---

### Web Interface

The system SHALL provide a web-based interface to browse the System model documentation, including all generated artifacts such as diagrams, reports, and verification traces.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [HTML Export](WebInterface/Features.md#html-export)
  * derive: [Serve Command](WebInterface/Features.md#serve-command)
  * derivedFrom: [System Model Interfaces](../UserStories.md#system-model-interfaces)
  * refinedBy: [Web Interface Refinement Specification](WebInterface/Specifications.md#web-interface-refinement-specification)
---

### MCP Interface

The system shall provide a Model Context Protocol interface for external tools, IDE integrations, CI adapters, and AI agents to access Reqvire model capabilities through typed tool interfaces.

#### Details
- The MCP interface shall adapt Reqvire core behavior to MCP clients.
- The MCP interface shall avoid becoming a second model engine.
- The MCP interface shall avoid becoming a generic shell gateway.
- The MCP interface shall avoid becoming a client-specific abstraction.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [MCP Protocol Standard Conformance](MCP/Tools.md#mcp-protocol-standard-conformance)
  * derive: [MCP Server Command](MCP/Tools.md#mcp-server-command)
  * derive: [MCP Streamable HTTP Transport](MCP/Tools.md#mcp-streamable-http-transport)
  * derive: [MCP Streamable HTTP Transport Safety](MCP/Tools.md#mcp-streamable-http-transport-safety)
  * derive: [MCP Mutation Concurrency Control](MCP/Tools.md#mcp-mutation-concurrency-control)
  * derive: [MCP Adapter Boundary](MCP/Tools.md#mcp-adapter-boundary)
  * derive: [MCP Shared Operation Interfaces](MCP/Tools.md#mcp-shared-operation-interfaces)
  * derive: [MCP Server State and Cache](MCP/Tools.md#mcp-server-state-and-cache)
  * derive: [MCP Tool Exposure Scope](MCP/Tools.md#mcp-tool-exposure-scope)
  * derive: [MCP Workspace Session Tools](MCP/Tools.md#mcp-workspace-session-tools)
  * derive: [MCP Structured Payload Interfaces](MCP/Tools.md#mcp-structured-payload-interfaces)
  * derive: [MCP Model Evidence Tools](MCP/Tools.md#mcp-model-evidence-tools)
  * derive: [MCP Quality Traceability Tools](MCP/Tools.md#mcp-quality-traceability-tools)
  * derive: [MCP Tool Side Effect Classification](MCP/Tools.md#mcp-tool-side-effect-classification)
  * derive: [MCP Mutation Tool Safety](MCP/Tools.md#mcp-mutation-tool-safety)
  * derive: [MCP Mutation Execution Flow](MCP/Tools.md#mcp-mutation-execution-flow)
  * derive: [MCP Resource Interface](MCP/Tools.md#mcp-resource-interface)
  * derive: [MCP Compatibility Versioning](MCP/Tools.md#mcp-compatibility-versioning)
  * derive: [MCP Access Control Baseline](MCP/Tools.md#mcp-access-control-baseline)
  * derivedFrom: [System Model Interfaces](../UserStories.md#system-model-interfaces)
  * refinedBy: [MCP Interface Boundary Specification](MCP/Specifications.md#mcp-interface-boundary-specification)
---
