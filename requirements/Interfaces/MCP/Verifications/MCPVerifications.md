# Elements

### MCP Protocol Standard Conformance Verification

This verification shall prove that Reqvire MCP initialization, capabilities, tool discovery, resources, and tool metadata conform to the supported MCP protocol revision.

#### Details
Expected checks:
- Initialize the server using MCP protocol revision `2025-11-25` and verify the response includes `protocolVersion`, standard MCP `capabilities`, and `serverInfo`.
- Initialize with an unsupported protocol revision and verify standard MCP initialization error handling.
- Verify the server declares MCP `tools` capability when tool calls are available.
- Verify the server declares MCP `resources` capability only when resources are available.
- Verify MVP capability objects do not advertise prompts, logging, completions, or tasks unless those features are implemented.
- Verify `tools.listChanged`, `resources.listChanged`, and `resources.subscribe` are omitted or false in MVP.
- Verify Reqvire-specific fields such as workspace status, model revision, Reqvire tool contract version, and mutation mode are returned through Reqvire tools/resources, not custom top-level MCP capabilities.
- Verify `tools/list` returns concrete tool definitions with valid `inputSchema` and expected annotations.
- Verify read/report tools include read-only annotations.
- Verify mutation tools are absent from `tools/list` by default and present only with `--enable-mutations`.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Protocol Standard Conformance](../Tools.md#mcp-protocol-standard-conformance)
---

### MCP Server Command Verification

This verification shall prove that `reqvire mcp` starts protocol service mode and reports startup diagnostics without exposing itself as an MCP tool.

#### Details
Expected checks:
- Start `reqvire mcp` in a fixture workspace.
- Verify startup metadata includes Reqvire version and supported MCP protocol revision.
- Verify tool discovery does not include `reqvire.mcp`.
- Verify startup validation failures are forwarded from Reqvire diagnostics and prevent the server from accepting protocol requests.
- Verify default startup returns read/report tools only from MCP `tools/list`.
- Verify startup with `--enable-mutations` returns mutation tools from MCP `tools/list`.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Server Command](../Tools.md#mcp-server-command)
---

### MCP Shared Operation Contracts Verification

This verification shall prove that MCP tools use shared Reqvire operation contracts.

#### Details
Expected checks:
- Run representative Reqvire operations through MCP tools.
- Verify result schemas match shared contract definitions.
- Verify transport-only options such as JSON stdout/file output are not exposed as MCP request fields.
- Verify no-argument tools use valid MCP object input schemas.
- Verify stable structured results are returned in `structuredContent` and conform to the declared `outputSchema`.
- Verify unknown tool calls and malformed requests return standard MCP/JSON-RPC protocol errors.
- Verify Reqvire parse, validation, and business-logic failures are forwarded as MCP tool execution errors with structured Reqvire error data where available.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Shared Operation Interfaces](../Tools.md#mcp-shared-operation-interfaces)
---

### MCP HTTP Transport End-to-End Verification

This verification shall prove that the Reqvire MCP HTTP transport preserves MCP tool semantics, local HTTP safety, and serialized workspace mutation behavior.

#### Details
Expected checks:
- Start `reqvire mcp --transport http` and verify the server binds to `127.0.0.1` by default.
- Start `reqvire mcp --transport http --host 127.0.0.1 --port <PORT>` and verify MCP JSON-RPC requests are accepted at fixed endpoint `/mcp`.
- Verify `reqvire mcp --transport stdio` preserves the current newline-delimited stdio behavior.
- Verify HTTP `tools/list`, `resources/list`, and representative `tools/call` responses match stdio tool names, schemas, annotations, mutation gating, and structured result semantics.
- Verify HTTP requests without an `Origin` header are accepted.
- Verify HTTP requests with loopback `Origin` headers are accepted.
- Verify HTTP requests with non-loopback, `null`, file, or malformed `Origin` headers are rejected before tool execution.
- Verify HTTP mutation tools are absent unless the server is started with `--enable-mutations`.
- Verify mutation-capable HTTP mode still requires explicit `--enable-mutations` and does not become enabled by selecting HTTP transport.
- Verify concurrent HTTP mutation requests for the same workspace are serialized so filesystem writes and post-mutation model refresh cannot interleave.
- Verify a read after an HTTP mutation observes the refreshed model state and reports the observed model revision or fingerprint.
- Verify HTTP GET without implemented SSE streaming returns method-not-allowed behavior and does not execute tools.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Transport Selection](../Tools.md#mcp-transport-selection)
  * verify: [MCP Streamable HTTP Transport Safety](../Tools.md#mcp-streamable-http-transport-safety)
  * verify: [MCP Mutation Concurrency Control](../Tools.md#mcp-mutation-concurrency-control)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
---

### MCP Contract Layer Boundary Verification

This verification shall prove that shared Reqvire MCP contracts are protocol-neutral below the MCP adapter.

#### Details
Expected checks:
- Verify shared request/result/error/evidence/diff/version types do not depend on MCP SDK runtime types.
- Verify the MCP adapter maps shared contracts to MCP `tools/list`, `tools/call`, resources, `structuredContent`, text `content`, and MCP error shapes.
- Verify CLI and MCP can reuse shared operation contracts without MCP requirements deriving from CLI command requirements.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Adapter Boundary](../Tools.md#mcp-adapter-boundary)
---

### MCP Server State and Cache Verification

This verification shall prove that MCP cached state is subordinate to Reqvire source files and Reqvire core parsing.

#### Details
Expected checks:
- Verify workspace status reports workspace root, git `HEAD`, dirty state, Reqvire version, MCP protocol revision, Reqvire tool contract version, model fingerprint, and last diagnostics.
- Verify source file, git state, excluded-pattern, Reqvire version, or Reqvire tool contract changes invalidate cached model state.
- Verify controlled MCP mutations refresh MCP internal state from the updated Reqvire core graph.
- Verify external filesystem drift triggers invalidation/reparse before serving stale model data.
- Verify dirty worktree state is reported in metadata and does not block tools when the equivalent Reqvire core operation can run.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Server State and Cache](../Tools.md#mcp-server-state-and-cache)
---

### MCP Tool Exposure Scope Verification

This verification shall prove that MCP exposes only supported Reqvire model operations.

#### Details
Expected checks:
- Verify `tools/list` does not include a generic shell or `reqvire.command` tool.
- Verify `tools/list` does not include hidden/internal `shell` or `sout` commands.
- Verify `tools/list` does not include `reqvire.mcp`, `reqvire.serve`, `reqvire.export`, or `reqvire.validate` in MVP.
- Verify no MCP prompts are advertised in MVP.
- Verify CLI-only transport flags such as `--json` and `--output` are absent from MCP input schemas.
- Verify CLI flags, modes, and sub-options are represented as typed request fields rather than nested tool names.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Tool Exposure Scope](../Tools.md#mcp-tool-exposure-scope)
---

### MCP Workspace Session Tools Verification

This verification shall prove that workspace/session tools return correct read-only metadata.

#### Details
Expected checks:
- `reqvire.workspace_status` reports workspace root, `HEAD`, dirty state, Reqvire version, supported MCP protocol revision, and Reqvire tool contract version.
- `reqvire.tool_contract` reports supported tools and schema versions for the current startup mode.
- `reqvire.model_revision` changes when model source files change.
- Workspace/session tools do not modify the filesystem.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Workspace Session Tools](../Tools.md#mcp-workspace-session-tools)
---

### MCP Structured Payload Contracts Verification

This verification shall prove that MCP structured payloads are consistent with shared Reqvire operation result contracts.

#### Details
Expected checks:
- Verify each MCP `outputSchema` is generated from or explicitly checked against its shared Reqvire operation result contract.
- Verify successful tool calls return `structuredContent` conforming to the declared `outputSchema`.
- Verify structured results identify relevant workspace/model revision and dirty state when model state affects interpretation.
- Verify structured results expose evidence references when the underlying Reqvire operation produces file, element, relation, attachment, report, or diff evidence.
- Verify element/model/mutation/error-shaped results preserve the semantic obligations of the corresponding shared Reqvire result contract without requiring terminal-output parsing.
- Verify removing or renaming stable structured fields requires a Reqvire tool contract version change.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Structured Payload Interfaces](../Tools.md#mcp-structured-payload-interfaces)
---

### MCP Tool Call Contracts Verification

This verification shall prove that every advertised MCP tool has a complete tool definition and call contract.

#### Details
Expected checks:
- For every tool returned by `tools/list`, verify `name`, `description`, `inputSchema`, annotations, and any declared `outputSchema`.
- Verify every `inputSchema` is a JSON object schema and rejects unsupported arguments.
- Verify no-argument tools use an empty object schema.
- Verify every successful tool call that declares `outputSchema` returns `structuredContent` conforming to that schema.
- Verify every successful tool call includes compatible text `content` for clients that do not consume structured content.
- Verify read/report tools declare `readOnlyHint: true`, `destructiveHint: false`, and `openWorldHint: false`.
- Verify mutation tools are absent from default `tools/list`.
- Verify mutation tools appear only when started with `--enable-mutations` and declare non-read-only annotations.
- Verify `reqvire.format` is read-only when `fix` is false and mutation-gated when `fix` is true.
- Verify `reqvire.lint` does not accept mutating `fix` arguments in read/report mode.
- Verify unknown tools, malformed MCP requests, and schema-invalid arguments produce standard MCP/JSON-RPC protocol errors.
- Verify Reqvire parse, validation, and operation failures produce MCP tool execution errors with structured Reqvire error data where available.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Shared Operation Interfaces](../Tools.md#mcp-shared-operation-interfaces)
---

### MCP Model Evidence Tools Verification

This verification shall prove that model evidence tools return authoritative Reqvire model data with revision metadata.

#### Details
Expected checks:
- Search, read element, model, containment, collect, and submodels tools return data matching Reqvire core reports.
- Results include evidence references for relevant files, elements, relations, and attachments.
- Read tools are allowed on dirty worktrees only when the result marks dirty state.
- Read tools do not mutate the filesystem.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Model Evidence Tools](../Tools.md#mcp-model-evidence-tools)
---

### MCP Quality Traceability Tools Verification

This verification shall prove that quality and traceability tools return structured diagnostics and evidence matching Reqvire core reports.

#### Details
Expected checks:
- Lint, coverage, traces, resources, and change-impact tools match shared Reqvire operation contracts.
- Startup validation failures are returned before the MCP server starts.
- Git comparison tools include compared commit and current `HEAD` metadata.
- Diagnostics are structured and machine-actionable.
- Tools do not require clients to parse human-oriented terminal output.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Quality Traceability Tools](../Tools.md#mcp-quality-traceability-tools)
---

### MCP Tool Side Effect Classification Verification

This verification shall prove that MCP tool discovery and tool annotations match the declared side-effect classification.

#### Details
Expected checks:
- Verify every advertised tool has exactly one declared side-effect class.
- Verify default `tools/list` advertises all `read_only` tools and omits all `mutation` tools.
- Verify default `tools/list` advertises `conditional_mutation` tools only with read-only argument schemas.
- Verify mutation-mode `tools/list` advertises mutation tools and mutation-capable schemas for conditional mutation tools.
- Verify read-only tools declare `readOnlyHint: true`, `destructiveHint: false`, and `openWorldHint: false`.
- Verify mutation tools declare `readOnlyHint: false`, `openWorldHint: false`, and the expected conservative `destructiveHint`.
- Verify `reqvire.lint` does not expose mutating fix behavior until a separate mutation contract is specified.
- Verify operation-specific preview requests for mutation-class tools are available only through mutation-class tools in mutation mode, except conditional mutation tools that explicitly expose read-only preview behavior.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Tool Side Effect Classification](../Tools.md#mcp-tool-side-effect-classification)
---

### MCP Mutation Tool Safety Verification

This verification shall prove that mutation tools enforce operation-specific preview behavior, Reqvire core semantics, filesystem persistence, internal graph refresh, and post-mutation diagnostics.

#### Details
Expected checks:
- Mutation tools are absent from MCP `tools/list` when the server was not started with `--enable-mutations`.
- Mutation tools are present in MCP `tools/list` and accept execution requests only when the server was started with `--enable-mutations`.
- Operation-specific preview mutation requests, such as `dry_run: true`, return changed files and diffs or equivalent change descriptions without filesystem changes.
- Non-dry-run requests use Reqvire core mutation logic and flush filesystem changes before reporting success.
- After successful mutation, subsequent MCP reads observe the refreshed internal graph state.
- Post-mutation results include validation summary, refreshed model revision, and affected element/submodel metadata.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Mutation Tool Safety](../Tools.md#mcp-mutation-tool-safety)
---

### MCP Mutation Execution Flow Verification

This verification shall prove that MCP mutation tools follow deterministic Reqvire-core-backed preview and execution behavior.

#### Details
Expected checks:
- Verify CRUD, relation, and asset mutation preview requests use `dry_run` where provided by the shared Reqvire operation contract.
- Verify formatting preview uses `fix: false` and formatting execution uses `fix: true`.
- Verify preview requests execute through Reqvire core and do not modify the filesystem.
- Verify preview results include diffs or equivalent change descriptions, changed files when known, validation risks, and affected scope.
- Verify execution requests execute only when mutation mode is enabled.
- Verify execution requests update the Reqvire core graph and persist filesystem changes before success is reported.
- Verify post-mutation diagnostics run according to the tool contract.
- Verify subsequent MCP reads observe the refreshed core graph state.
- Verify mutation results include changed files, diffs, diagnostics, refreshed model revision, and affected elements/submodels.
- Verify attempts to bypass Reqvire model semantics or perform arbitrary file writes are rejected.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Mutation Execution Flow](../Tools.md#mcp-mutation-execution-flow)
---

### MCP Resource Interface Verification

This verification shall prove that MCP resources expose read-only, revision-tagged views.

#### Details
Expected checks:
- Workspace, model, element, file, and report resources include revision metadata.
- `resources/list` returns stable resource identifiers.
- `resources/read` returns contents for listed resources.
- Resource reads do not mutate the filesystem.
- Resource payloads match authoritative Reqvire core data for the same revision.
- Resource identifiers are stable and safely encoded.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Resource Interface](../Tools.md#mcp-resource-interface)
---

### MCP Contract Versioning Verification

This verification shall prove that MCP startup/tool discovery reports the negotiated MCP protocol revision, Reqvire version, Reqvire tool contract version, schema revision, and Reqvire capability flags.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Compatibility Versioning](../Tools.md#mcp-compatibility-versioning)
---

### MCP Access Control Baseline Verification

This verification shall prove that MCP does not expose arbitrary shell execution, arbitrary filesystem reads, or mutation tools unless mutation capability is enabled.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Access Control Baseline](../Tools.md#mcp-access-control-baseline)
---

### MCP Server End-to-End Verification

This verification shall prove the Reqvire MCP server behavior through the external CLI/stdin/stdout protocol boundary.

#### Details
The e2e test starts `reqvire mcp` in a fixture workspace and verifies MCP initialization, capabilities, tool discovery, resource discovery and reads, structured tool calls, protocol error handling, default mutation-tool omission, mutation-mode tool exposure, dry-run mutation behavior, persisted mutation behavior, post-mutation reads, and startup validation failure handling.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Protocol Standard Conformance](../Tools.md#mcp-protocol-standard-conformance)
  * verify: [MCP Server Command](../Tools.md#mcp-server-command)
  * verify: [MCP Shared Operation Interfaces](../Tools.md#mcp-shared-operation-interfaces)
  * verify: [MCP Tool Exposure Scope](../Tools.md#mcp-tool-exposure-scope)
  * verify: [MCP Workspace Session Tools](../Tools.md#mcp-workspace-session-tools)
  * verify: [MCP Structured Payload Interfaces](../Tools.md#mcp-structured-payload-interfaces)
  * verify: [MCP Model Evidence Tools](../Tools.md#mcp-model-evidence-tools)
  * verify: [MCP Quality Traceability Tools](../Tools.md#mcp-quality-traceability-tools)
  * verify: [MCP Tool Side Effect Classification](../Tools.md#mcp-tool-side-effect-classification)
  * verify: [MCP Mutation Tool Safety](../Tools.md#mcp-mutation-tool-safety)
  * verify: [MCP Mutation Execution Flow](../Tools.md#mcp-mutation-execution-flow)
  * verify: [MCP Resource Interface](../Tools.md#mcp-resource-interface)
  * verify: [MCP Compatibility Versioning](../Tools.md#mcp-compatibility-versioning)
  * verify: [MCP Access Control Baseline](../Tools.md#mcp-access-control-baseline)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
---
