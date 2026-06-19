# Elements

### MCP Protocol and Tool Verification Objective

This objective groups verification that Reqvire MCP servers, tools, resources, payload contracts, access controls, and mutation boundaries conform to the supported protocol behavior.

#### Metadata
  * type: verification-objective

#### Relations
  * derive: [MCP Access Control Baseline Verification](#mcp-access-control-baseline-verification)
  * derive: [MCP Contract Layer Boundary Verification](#mcp-contract-layer-boundary-verification)
  * derive: [MCP Contract Versioning Verification](#mcp-contract-versioning-verification)
  * derive: [Embedded MCP Serve Endpoint Verification](#embedded-mcp-serve-endpoint-verification)
  * derive: [MCP HTTP Transport End-to-End Verification](#mcp-http-transport-end-to-end-verification)
  * derive: [MCP Model Evidence Tools Verification](#mcp-model-evidence-tools-verification)
  * derive: [MCP Mutation Execution Flow Verification](#mcp-mutation-execution-flow-verification)
  * derive: [MCP Mutation Tool Safety Verification](#mcp-mutation-tool-safety-verification)
  * derive: [MCP Prompt Guidance Verification](#mcp-prompt-guidance-verification)
  * derive: [MCP Protocol Standard Conformance Verification](#mcp-protocol-standard-conformance-verification)
  * derive: [MCP Quality Traceability Tools Verification](#mcp-quality-traceability-tools-verification)
  * derive: [MCP Resource Interface Verification](#mcp-resource-interface-verification)
  * derive: [MCP Semantic Prefix Registry Tools Verification](#mcp-semantic-prefix-registry-tools-verification)
  * derive: [MCP Semantic Query Tools Verification](#mcp-semantic-query-tools-verification)
  * derive: [MCP Semantic Vocabulary Tools Verification](#mcp-semantic-vocabulary-tools-verification)
  * derive: [MCP Server Command Verification](#mcp-server-command-verification)
  * derive: [MCP Server End-to-End Verification](#mcp-server-end-to-end-verification)
  * derive: [MCP Server State and Cache Verification](#mcp-server-state-and-cache-verification)
  * derive: [MCP Shared Operation Contracts Verification](#mcp-shared-operation-contracts-verification)
  * derive: [MCP Size Estimate Startup Verification](#mcp-size-estimate-startup-verification)
  * derive: [MCP Structured Payload Contracts Verification](#mcp-structured-payload-contracts-verification)
  * derive: [MCP Tool Call Contracts Verification](#mcp-tool-call-contracts-verification)
  * derive: [MCP Tool Exposure Scope Verification](#mcp-tool-exposure-scope-verification)
  * derive: [MCP Tool Side Effect Classification Verification](#mcp-tool-side-effect-classification-verification)
  * derive: [MCP Workspace Session Tools Verification](#mcp-workspace-session-tools-verification)
---

### MCP Access Control Baseline Verification

This verification shall prove that MCP does not expose arbitrary shell execution, arbitrary filesystem reads, or mutation tools unless mutation capability is enabled.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Access Control Baseline](../../../Interfaces/MCP/Tools.md#mcp-access-control-baseline)
---

### MCP Contract Layer Boundary Verification

This verification shall prove that shared Reqvire MCP contracts are protocol-neutral below the MCP adapter.

#### Details
Expected checks:
- Verify shared request/result/error/evidence/diff/version types do not depend on MCP SDK runtime types.
- Verify an in-process Rust application can discover and call Reqvire tools through the public Reqvire library without starting MCP HTTP transport.
- Verify the MCP adapter maps shared contracts to MCP `tools/list`, `tools/call`, resources, `structuredContent`, text `content`, and MCP error shapes.
- Verify CLI and MCP can reuse shared operation contracts without MCP requirements deriving from CLI command requirements.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Adapter Boundary](../../../Interfaces/MCP/Tools.md#mcp-adapter-boundary)
---

### MCP Contract Versioning Verification

This verification shall prove that MCP startup/tool discovery reports the negotiated MCP protocol revision, Reqvire version, Reqvire tool contract version, schema revision, and Reqvire capability flags.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Compatibility Versioning](../../../Interfaces/MCP/Tools.md#mcp-compatibility-versioning)
---

### MCP HTTP Transport End-to-End Verification

This verification shall prove that the Reqvire MCP HTTP transport preserves MCP tool semantics, local HTTP safety, and serialized workspace mutation behavior.

#### Details
Expected checks:
- Start `reqvire mcp` and verify the server binds to `127.0.0.1` by default.
- Start `reqvire mcp --host 127.0.0.1 --port <PORT>` and verify standard MCP streamable HTTP requests are accepted at fixed endpoint `/mcp`.
- Verify `reqvire mcp --transport stdio` is rejected because stdio compatibility mode is not supported.
- Verify HTTP `tools/list`, `resources/list`, and representative `tools/call` responses expose the expected tool names, schemas, annotations, mutation gating, and structured result semantics.
- Verify HTTP requests without an `Origin` header are accepted.
- Verify HTTP requests with loopback `Origin` headers are accepted.
- Verify HTTP requests with non-loopback, `null`, file, or malformed `Origin` headers are rejected before tool execution.
- Verify HTTP mutation tools are absent unless the server is started with `--enable-mutations`.
- Verify mutation-capable HTTP mode still requires explicit `--enable-mutations` and does not become enabled by selecting HTTP transport.
- Verify concurrent HTTP mutation requests for the same workspace are serialized so filesystem writes and post-mutation model refresh cannot interleave.
- Verify a read after an HTTP mutation observes the refreshed model state and reports the observed model revision or fingerprint.
- Verify HTTP transport behavior is provided by RMCP rather than a Reqvire-owned HTTP JSON-RPC parser.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Mutation Concurrency Control](../../../Interfaces/MCP/Tools.md#mcp-mutation-concurrency-control)
  * verify: [MCP Streamable HTTP Transport](../../../Interfaces/MCP/Tools.md#mcp-streamable-http-transport)
  * verify: [MCP Streamable HTTP Transport Safety](../../../Interfaces/MCP/Tools.md#mcp-streamable-http-transport-safety)
---

### MCP Model Evidence Tools Verification

This verification shall prove that model evidence tools return authoritative Reqvire model data with revision metadata.

#### Details
Expected checks:
- Search, read element, model, containment, collect, and submodels tools return data matching Reqvire core reports.
- Search supports `filter_type=ontology` and returns parsed ontology ADT content.
- Verify `tools/list` advertises `reqvire.semantic.ontologies` as a read-only semantic model evidence tool.
- Verify `tools/list` advertises the `include_external` argument on `reqvire.semantic.ontologies`.
- Verify `tools/list` advertises the `include_external` argument on `reqvire.semantic.prefixes`, `reqvire.semantic.vocabulary`, and `reqvire.semantic.sparql`.
- Verify `reqvire.semantic.ontologies` returns both RDF ontology and SHACL shape content by default.
- Verify `reqvire.semantic.ontologies` filters to RDF-only or SHACL-only content when the `content` argument is set.
- Verify `reqvire.semantic.ontologies` excludes local External Ontology dependency triples and external term declarations by default and includes only the used external subset from parsed Turtle/TTL, RDF/XML, and JSON-LD sources when `include_external` is true.
- Verify semantic prefix and vocabulary tools exclude imported external ontology vocabulary by default, include only used external subset vocabulary when `include_external` is true, and mark included imported entries as external with source metadata and used-subset materialization metadata.
- Verify `reqvire.semantic.sparql` queries the authored semantic store by default and can query only the used external subset when `include_external` is true.
- Verify unused external ontology dependency terms remain unavailable through MCP semantic ontology, vocabulary, and SPARQL outputs.
- Read element returns `concept_references` for elements that author `#### Concept References`.
- Collect returns authored concept references for capability and requirement elements and semantic-contract ontology-use context where the underlying operation returns semantic-contract evidence.
- Results include evidence references for relevant files, elements, relations, and reused_contract_context.
- Read tools are allowed on dirty worktrees only when the result marks dirty state.
- Read tools do not mutate the filesystem.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Model Evidence Tools](../../../Interfaces/MCP/Tools.md#mcp-model-evidence-tools)
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
  * verify: [MCP Mutation Execution Flow](../../../Interfaces/MCP/Tools.md#mcp-mutation-execution-flow)
---

### MCP Mutation Tool Safety Verification

This verification shall prove that mutation tools enforce operation-specific preview behavior, Reqvire core semantics, filesystem persistence, internal graph refresh, and post-mutation diagnostics.

#### Details
Expected checks:
- Mutation tools are absent from MCP `tools/list` when the server was not started with `--enable-mutations`.
- Mutation tools are present in MCP `tools/list` and accept execution requests only when the server was started with `--enable-mutations`.
- Operation-specific preview mutation requests, such as `dry_run: true`, return changed files and diffs or equivalent change descriptions without filesystem changes.
- Non-dry-run requests use Reqvire core mutation logic and flush filesystem changes before reporting success.
- Non-dry-run mutation requests that would break requirement reused_contract_context compatibility, semantic-contract SHACL reference reachability, concept-reference resolution, or single ontology-root validation are rejected before persistence.
- After successful mutation, subsequent MCP reads observe the refreshed internal graph state.
- Post-mutation results include validation summary, refreshed model revision, and affected element/submodel metadata.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Mutation Tool Safety](../../../Interfaces/MCP/Tools.md#mcp-mutation-tool-safety)
---

### MCP Prompt Guidance Verification

This verification shall prove that MCP prompt templates are discoverable, retrievable, build-time static, and useful for regular Reqvire and semantic query workflows.

#### Details
Expected checks:
- Verify initialization advertises a standard MCP prompts capability.
- Verify `prompts/list` returns regular workflow prompts and semantic query prompts.
- Verify `prompts/get` for `reqvire.semantic.query` returns text that references semantic vocabulary, prefix, and SPARQL tools and states that `include_external` exposes only the used external subset.
- Verify `prompts/get` for a regular workflow prompt returns text that references standard Reqvire model exploration tools.
- Verify unknown prompt names return a protocol error.
- Verify prompt retrieval does not mutate model source files.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Prompt Guidance](../../../Interfaces/MCP/Tools.md#mcp-prompt-guidance)
---

### MCP Protocol Standard Conformance Verification

This verification shall prove that Reqvire MCP initialization, capabilities, tool discovery, resources, and tool metadata conform to the supported MCP protocol revision.

#### Details
Expected checks:
- Initialize the server using MCP protocol revision `2025-11-25` and verify the response includes `protocolVersion`, standard MCP `capabilities`, and `serverInfo`.
- Send a request with an unsupported `Mcp-Protocol-Version` HTTP header and verify RMCP rejects it before tool execution.
- Verify the server declares MCP `tools` capability when tool calls are available.
- Verify the server declares MCP `resources` capability only when resources are available.
- Verify implemented capability objects advertise tools, resources, and prompts, and do not advertise logging, completions, or tasks unless those capabilities are implemented.
- Verify `tools.listChanged`, `resources.listChanged`, and `resources.subscribe` are omitted or false in MVP.
- Verify Reqvire-specific fields such as workspace status, model revision, Reqvire tool contract version, and mutation mode are returned through Reqvire tools/resources, not custom top-level MCP capabilities.
- Verify `tools/list` returns concrete tool definitions with valid `inputSchema` and expected annotations.
- Verify read/report tools include read-only annotations.
- Verify mutation tools are absent from `tools/list` by default and present only with `--enable-mutations`.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Protocol Standard Conformance](../../../Interfaces/MCP/Tools.md#mcp-protocol-standard-conformance)
---

### MCP Quality Traceability Tools Verification

This verification shall prove that quality and traceability tools return structured diagnostics and evidence matching Reqvire core reports.

#### Details
Expected checks:
- Lint, coverage, traces, resources, ontologies, and change-impact tools match shared Reqvire operation contracts.
- Ontologies tool returns collected ontology `Ontology` blocks and semantic-contract `Shapes` blocks in Turtle by default.
- Ontologies tool supports JSON-LD output through a typed MCP argument.
- Ontologies tool supports `full: true` and returns Reqvire model context triples and ontology projection facts alongside ontology and SHACL content.
- Startup validation failures are returned before the MCP server starts.
- Git comparison tools include compared commit and current `HEAD` metadata.
- Diagnostics are structured and machine-actionable.
- Tools do not require clients to parse human-oriented terminal output.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Quality Traceability Tools](../../../Interfaces/MCP/Tools.md#mcp-quality-traceability-tools)
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
  * verify: [MCP Resource Interface](../../../Interfaces/MCP/Tools.md#mcp-resource-interface)
---

### MCP Semantic Prefix Registry Tools Verification

This verification shall prove that MCP prefix discovery exposes ontology element-defined prefixes with source prose content and without mutating workspace state.

#### Details
Expected checks:
- Verify `tools/list` advertises `reqvire.semantic.prefixes` as a read-only tool.
- Verify `reqvire.semantic.prefixes` returns the ontology element-defined `testonto` prefix and namespace from the parsed semantic model index.
- Verify imported external ontology prefixes are omitted by default and included with `external: true` plus external source metadata when `include_external` is true.
- Verify prefix source context includes element identifier, name, file path, line number, and ontology element prose content.
- Verify source content excludes authored Turtle prefix blocks.
- Verify the response includes a SPARQL prefix block suitable for query construction.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Semantic Prefix Registry Tools](../../../Interfaces/MCP/Tools.md#mcp-semantic-prefix-registry-tools)
---

### MCP Semantic Query Tools Verification

This verification shall prove that MCP SPARQL queries execute over Reqvire semantic RDF evidence without mutating workspace state.

#### Details
Expected checks:
- Verify `tools/list` advertises `reqvire.semantic.sparql` as a read-only tool.
- Verify `reqvire.semantic.sparql` executes a SELECT query over authored ontology and SHACL RDF.
- Verify `reqvire.semantic.sparql` uses full semantic graph context by default, including generated model-context triples.
- Verify local external ontology dependency triples are outside the default queried graph and only used external subset triples become queryable when `include_external` is true.
- Verify the full semantic graph materializes relation-family normalized predicates equivalent to the relation-family CONSTRUCT query specification.
- Verify SELECT results include ordered variables, bindings, RDF term metadata, row count, semantic index summary, diagnostics, and model fingerprint.
- Verify invalid SPARQL returns an MCP tool error rather than mutating files.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Semantic Query Tools](../../../Interfaces/MCP/Tools.md#mcp-semantic-query-tools)
---

### MCP Semantic Vocabulary Tools Verification

This verification shall prove that MCP vocabulary discovery exposes compact paged semantic vocabulary with prefixes for SPARQL query construction.

#### Details
Expected checks:
- Verify `tools/list` advertises `reqvire.semantic.vocabulary` as a read-only tool.
- Verify `reqvire.semantic.vocabulary` with `section: "all"` returns section counts, prefixes, a SPARQL prefix block, diagnostics, and model fingerprint.
- Verify imported external vocabulary is omitted by default and only used external subset vocabulary is included with `external: true` plus external source metadata when `include_external` is true.
- Verify `section: "relation_families"` returns relation family entries with normalized forward and inverse properties.
- Verify paging returns `next_cursor` when a section has more items than the requested limit.
- Verify query patterns include SPARQL examples when `include_examples` is true.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Semantic Vocabulary Tools](../../../Interfaces/MCP/Tools.md#mcp-semantic-vocabulary-tools)
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
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Server Command](../../../Interfaces/MCP/Tools.md#mcp-server-command)
---

### MCP Server End-to-End Verification

This verification shall prove the Reqvire MCP server behavior through the external RMCP Streamable HTTP protocol boundary.

#### Details
The e2e test starts `reqvire mcp` in a fixture workspace and verifies MCP initialization, capabilities, tool discovery, resource discovery and reads, structured tool calls including ontology semantic collection, protocol error handling, stdio transport rejection, default mutation-tool omission, mutation-mode tool exposure, dry-run mutation behavior, persisted mutation behavior, post-mutation reads, and startup validation failure handling.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Access Control Baseline](../../../Interfaces/MCP/Tools.md#mcp-access-control-baseline)
  * verify: [MCP Compatibility Versioning](../../../Interfaces/MCP/Tools.md#mcp-compatibility-versioning)
  * verify: [MCP Model Evidence Tools](../../../Interfaces/MCP/Tools.md#mcp-model-evidence-tools)
  * verify: [MCP Mutation Execution Flow](../../../Interfaces/MCP/Tools.md#mcp-mutation-execution-flow)
  * verify: [MCP Mutation Tool Safety](../../../Interfaces/MCP/Tools.md#mcp-mutation-tool-safety)
  * verify: [MCP Protocol Standard Conformance](../../../Interfaces/MCP/Tools.md#mcp-protocol-standard-conformance)
  * verify: [MCP Quality Traceability Tools](../../../Interfaces/MCP/Tools.md#mcp-quality-traceability-tools)
  * verify: [MCP Resource Interface](../../../Interfaces/MCP/Tools.md#mcp-resource-interface)
  * verify: [MCP Server Command](../../../Interfaces/MCP/Tools.md#mcp-server-command)
  * verify: [MCP Shared Operation Interfaces](../../../Interfaces/MCP/Tools.md#mcp-shared-operation-interfaces)
  * verify: [MCP Structured Payload Interfaces](../../../Interfaces/MCP/Tools.md#mcp-structured-payload-interfaces)
  * verify: [MCP Tool Exposure Scope](../../../Interfaces/MCP/Tools.md#mcp-tool-exposure-scope)
  * verify: [MCP Tool Side Effect Classification](../../../Interfaces/MCP/Tools.md#mcp-tool-side-effect-classification)
  * verify: [MCP Workspace Session Tools](../../../Interfaces/MCP/Tools.md#mcp-workspace-session-tools)
---

### Embedded MCP Serve Endpoint Verification

This verification shall prove that `reqvire serve --enable-mcp` exposes the Reqvire MCP Streamable HTTP endpoint on the same listener as the Explorer without enabling mutations by default.

#### Details
Expected checks:
- Start `reqvire serve --enable-mcp --host 127.0.0.1 --port <PORT>` in a fixture workspace.
- Verify the Explorer root URL still returns the SPA shell.
- Verify standard MCP Streamable HTTP requests are accepted at `http://127.0.0.1:<PORT>/mcp`.
- Verify MCP `tools/list` does not include mutation tools when only `--enable-mcp` is present.
- Start `reqvire serve --enable-mcp --enable-mutations --host 127.0.0.1 --port <PORT>` and verify MCP `tools/list` includes mutation tools.
- Execute an embedded MCP mutation and verify the mutation refreshes the materialized Explorer runtime store, so a subsequent `assets/project-store.js` request contains the updated model datastore after browser/client reload.
- Verify ordinary `assets/project-store.js` requests do not regenerate the runtime store from disk without an embedded MCP write mutation or an explicit serve-owned refresh path.
- Verify `assets/project-store.js` and `ontologies.ttl` responses include no-store cache control.
- Verify `--enable-mutations` is rejected unless `--enable-mcp` is also provided for `reqvire serve`.
- Verify `/mcp` is handled by RMCP transport and is not served by the Explorer SPA fallback.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Serve Command Embedded MCP Endpoint](../../../Interfaces/WebExplorer/Capabilities.md#serve-command-embedded-mcp-endpoint)
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
  * verify: [MCP Server State and Cache](../../../Interfaces/MCP/Tools.md#mcp-server-state-and-cache)
---

### MCP Shared Operation Contracts Verification

This verification shall prove that MCP tools use shared Reqvire operation contracts.

#### Details
Expected checks:
- Run representative Reqvire operations through MCP tools.
- Run representative Reqvire operations through an external Rust fixture using the public Reqvire tool registry API.
- Verify result schemas match shared contract definitions.
- Verify transport-only options such as JSON stdout/file output are not exposed as MCP request fields.
- Verify no-argument tools use valid MCP object input schemas.
- Verify stable structured results are returned in `structuredContent` and conform to the declared `outputSchema`.
- Verify unknown tool calls and malformed requests return standard MCP/JSON-RPC protocol errors.
- Verify Reqvire parse, validation, and business-logic failures are forwarded as MCP tool execution errors with structured Reqvire error data where available.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Shared Operation Interfaces](../../../Interfaces/MCP/Tools.md#mcp-shared-operation-interfaces)
---

### MCP Size Estimate Startup Verification

This verification shall prove that MCP size estimates are controlled by server startup configuration.

#### Details
Expected checks:
- Start `reqvire mcp` without `--with-size-estimates` and verify model element tool responses omit `size_estimate`.
- Start `reqvire mcp --with-size-estimates` and verify model element tool responses include `size_estimate`.
- Verify `reqvire.read_element` includes element `size_estimate` when enabled.
- Verify `reqvire.model` includes element `size_estimate` for top-level and nested relation elements when enabled.
- Verify workspace status or tool contract reports size-estimate enabled state.
- Verify `--with-size-estimates` is a startup option and is not accepted as a per-tool MCP argument.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Server Command](../../../Interfaces/MCP/Tools.md#mcp-server-command)
---

### MCP Structured Payload Contracts Verification

This verification shall prove that MCP structured payloads are consistent with shared Reqvire operation result contracts.

#### Details
Expected checks:
- Verify each MCP `outputSchema` is generated from or explicitly checked against its shared Reqvire operation result contract.
- Verify successful tool calls return `structuredContent` conforming to the declared `outputSchema`.
- Verify structured results identify relevant workspace/model revision and dirty state when model state affects interpretation.
- Verify structured results expose evidence references when the underlying Reqvire operation produces file, element, relation, reused_contract_context, report, or diff evidence.
- Verify element-shaped results preserve semantic model ADT fields when present, including `ontology`, `semantic_contract`, and `concept_references`.
- Verify element/model/mutation/error-shaped results preserve the semantic obligations of the corresponding shared Reqvire result contract without requiring terminal-output parsing.
- Verify removing or renaming stable structured fields requires a Reqvire tool contract version change.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Structured Payload Interfaces](../../../Interfaces/MCP/Tools.md#mcp-structured-payload-interfaces)
---

### MCP Tool Call Contracts Verification

This verification shall prove that every advertised MCP tool has a complete tool definition and call contract.

#### Details
Expected checks:
- For every tool returned by `tools/list`, verify `name`, `description`, `inputSchema`, annotations, and any declared `outputSchema`.
- Verify every `inputSchema` is a JSON object schema and rejects unsupported arguments.
- Verify `reqvire.search` inputSchema advertises governance metadata filter fields `filter_status`, `filter_priority`, `filter_risk`, and `filter_owner`.
- Verify `reqvire.search` rejects unsupported governance metadata filter values with accepted-value diagnostics.
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
  * verify: [MCP Shared Operation Interfaces](../../../Interfaces/MCP/Tools.md#mcp-shared-operation-interfaces)
---

### MCP Tool Exposure Scope Verification

This verification shall prove that MCP exposes only supported Reqvire model operations.

#### Details
Expected checks:
- Verify `tools/list` does not include a generic shell or `reqvire.command` tool.
- Verify `tools/list` does not include hidden/internal `shell` or `sout` commands.
- Verify `tools/list` does not include `reqvire.mcp`, `reqvire.serve`, or `reqvire.validate` in MVP.
- Verify prompt templates are advertised through `prompts/list`, not `tools/list`.
- Verify CLI-only transport flags such as `--json` and `--output` are absent from MCP input schemas.
- Verify CLI flags, modes, and sub-options are represented as typed request fields rather than nested tool names.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [MCP Tool Exposure Scope](../../../Interfaces/MCP/Tools.md#mcp-tool-exposure-scope)
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
  * verify: [MCP Tool Side Effect Classification](../../../Interfaces/MCP/Tools.md#mcp-tool-side-effect-classification)
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
  * verify: [MCP Workspace Session Tools](../../../Interfaces/MCP/Tools.md#mcp-workspace-session-tools)
---
