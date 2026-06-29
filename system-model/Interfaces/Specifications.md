# Elements

### MCP Interface Boundary Specification

The MCP interface is expected to be a protocol adapter over Reqvire core and shared tool contracts.

#### Details
Boundary rules:
- Reqvire markdown files remain the durable source of truth.
- Reqvire core remains authoritative for parsing, validation, reporting, formatting, and mutation semantics.
- MCP handlers call shared Reqvire tool contracts instead of reimplementing model behavior.
- MCP does not expose arbitrary shell command execution.
- MCP does not store an independent model copy that can diverge from the filesystem.
- MCP responses include standard MCP protocol metadata where required and Reqvire contract/workspace/model revision metadata inside Reqvire tool or resource results.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Interface](InterfacesRequirements.md#mcp-interface)
---

### Shared Core Operation Layer Specification

Reqvire operation semantics are expected to be owned by core request/result APIs that can be called by more than one external adapter.

#### Details
Contract rules:
- A shared operation owns model-loading decisions where applicable, validation gates, report construction, mutation preview/application behavior where applicable, and result evidence for its operation scope.
- CLI and MCP adapters may add transport-specific parsing, output rendering, protocol metadata, and error wrapping, but must not fork the underlying operation behavior.
- Stable operation outputs shall be represented as structured Rust result types before CLI JSON serialization or MCP `structuredContent` wrapping.
- Shared operations shall cover element lookup, search, model, collect, submodels, resources, coverage, traces, containment, lint, format, and change-impact behavior before broader mutation operations are migrated.
- Semantic read/query helpers remain a separate semantic-tool adapter scope until they are explicitly migrated into the shared operation layer.
- Mutation operation sharing shall include explicit dry-run, diff, persistence, post-mutation validation, and cache invalidation semantics before adapters are converted.
- Adapter tests should cover at least one representative operation through CLI and MCP-facing paths when an operation is migrated.

#### Metadata
  * type: specification

#### Relations
  * define: [Shared Core Operation Layer](InterfacesRequirements.md#shared-core-operation-layer)
---

### Web Interface Contract Specification

#### Details
The browse interface allows users to:
- View rendered specifications and requirements
- Navigate through diagrams and visualizations
- Access verification traces and coverage reports
- Explore the complete model structure through an integrated web interface

This capability enables both human users (via browser) and AI agents (via MCP server) to efficiently explore and understand the System model without manually navigating file structures.

All Explorer runtime data is expected to use deterministic ordering to enable reliable troubleshooting and reproducible builds.

The system is expected to ensure deterministic Explorer runtime data by:
- Sorting elements by identifier before rendering
- Sorting relations by type and target identifier
- Maintaining consistent navigation and page ordering
- Generating stable diagram node and relation ordering

This determinism ensures that:
- Running serve runtime generation multiple times produces stable Project Store data for unchanged inputs
- Runtime differences reflect actual model content changes
- Continuous integration pipelines can validate Explorer data reproducibly

#### Metadata
  * type: specification
---
