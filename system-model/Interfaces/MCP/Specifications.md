# Elements

### MCP Access Control Baseline Specification

The MCP server is expected to start with safe local-first access behavior.

#### Details
Access rules:
- Prefer local-only HTTP transport first.
- Do not expose arbitrary shell execution.
- Do not expose arbitrary filesystem reads.
- File evidence is limited to files referenced by the Reqvire model.
- Mutation tools are not exposed unless the server is started with `reqvire mcp --enable-mutations`.
- Every mutation result includes changed files and diff.
- External URLs are returned as references and are not fetched by default.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Access Control Baseline](Tools.md#mcp-access-control-baseline)
---

### MCP Contract Layer Boundary Specification

The shared MCP contract layer is expected to remain protocol-neutral.

#### Details
Boundary rules:
- Shared contract types define Reqvire operation requests, results, errors, evidence references, mutation diffs, workspace/model revision metadata, and contract versions.
- Shared contract types depend on Reqvire model concepts and core operation semantics.
- Shared contract types do not depend on MCP SDK runtime types, transport types, or client runtime types.
- The shared Reqvire tool registry is exposed through the Reqvire library for in-process applications that need to discover tool definitions and call tools without MCP transport.
- The MCP adapter maps shared contracts to MCP `tools/list`, `tools/call`, resources, `structuredContent`, text `content`, and MCP error shapes.
- The MCP adapter uses the same shared Reqvire tool registry that an in-process application can use directly.
- Reqvire CLI and MCP may reuse shared operation contracts where they expose the same Reqvire operation, but MCP requirements do not derive from CLI command requirements.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Adapter Boundary](Tools.md#mcp-adapter-boundary)
---

### MCP Contract Versioning Specification

The MCP server is expected to expose the negotiated MCP protocol revision and a Reqvire tool contract version separate from the Reqvire binary version.

#### Details
Contract version payload includes:
- Reqvire binary version.
- MCP protocol revision.
- Reqvire tool contract name.
- Reqvire tool contract version.
- Schema revision.
- Reqvire capability flags such as `read_reports` or `mutations_enabled`.
- Current MCP tool list hash or revision identifier.

Compatibility rules:
- MCP protocol compatibility follows MCP version negotiation.
- Additive fields are allowed inside a Reqvire tool contract version.
- Removing or renaming fields requires a new Reqvire tool contract version.
- Changing mutation semantics requires a new Reqvire tool contract version or explicit Reqvire capability flag.
- Clients verify contract compatibility during startup/status checks.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Compatibility Versioning](Tools.md#mcp-compatibility-versioning)
---

### MCP Model Evidence Tools Specification

The MCP interface is expected to expose read-only model evidence tools grounded in Reqvire core reports and lookup behavior.

#### Details
Model evidence tool behavior is inherited from reused Reqvire search, model, containment, collect, submodel, and ontology collection contracts. MCP adds typed request/result schemas, workspace/model revision metadata, and evidence references describing which elements, files, relations, reused_contract_context, ontology blocks, and shape blocks were included.

`reqvire.search` tool calls are expected to expose typed request fields equivalent to the stable Reqvire search filters:
- `short`: optional boolean controlling abbreviated output.
- `filter_file`: optional file path glob.
- `filter_name`: optional element name regex.
- `filter_type`: optional element type filter string.
- `filter_status`: optional comma-separated requirement governance status filter (`draft`, `review`, `approved`).
- `filter_priority`: optional comma-separated requirement governance priority filter (`low`, `medium`, `high`, `critical`).
- `filter_risk`: optional comma-separated requirement governance risk filter (`low`, `medium`, `high`, `critical`).
- `filter_owner`: optional regex over effective requirement governance owner.
- `filter_content`: optional element content regex.
- `filter_page_content`: optional parent file page content regex.
- `have_relations`: optional comma-separated relation type list requiring all listed relations.
- `not_have_relations`: optional comma-separated relation type list excluding elements that have all listed relations.
- `has_reused_contract_context`: optional boolean requiring at least one reused_contract_context.
- `filter_reused_contract_context`: optional reused_contract_context target glob.

Governance metadata filters apply to effective governance metadata values and exclude non-governance-bearing elements when active. Successful `reqvire.search` structured results include effective governance metadata for capability and requirement element evidence.

Semantic model evidence rules:
- `filter_type` accepts all canonical element type tokens supported by Reqvire core, including `capability`, `requirement`, `ontology`, `semantic-contract`, `source`, `specification`, `constraint`, `behavior`, `state`, `input-output`, and verification types.
- `reqvire.search --filter-type=ontology` through MCP returns ontology elements with parsed ontology ADT content when full results are requested.
- `reqvire.search --filter-type=semantic-contract` through MCP returns reusable shape contracts with parsed semantic-contract ADT content, constrained requirements, and ontology-use relations when full results are requested.
- `reqvire.read_element` returns `concept_references` for non-ontology, non-semantic-contract elements that author `#### Concept References`.
- `reqvire.collect` includes authored concept references for capability/requirement collection and semantic-contract ontology-use context for semantic-contract evidence where the underlying Reqvire operation returns it.
- `reqvire.model` and `reqvire.submodels` preserve capability roots, requirement ownership through `specify`/`specifiedBy`, ontology hierarchy through `derive`/`derivedFrom`, and concept-reference facts needed for semantic dependency traceability.
- `reqvire.semantic.ontologies` exposes the same semantic collection as the CLI `ontologies` command under the semantic MCP namespace.
- `reqvire.semantic.ontologies` accepts optional `format` with values `turtle` or `jsonld`; omitted format defaults to `turtle`.
- `reqvire.semantic.ontologies` accepts optional `content` with values `rdf`, `shacl`, or `both`; omitted content defaults to `both`.
- `reqvire.semantic.ontologies` accepts optional `full` boolean; omitted or false returns generated ontology document declarations plus authored ontology and SHACL artifacts, while true also includes generated Reqvire model context triples and ontology projection facts. Generated ontology document declarations use the resolved `ontology_base` as the `owl:Ontology` IRI and list same-base ontology elements as contributors.
- `reqvire.semantic.ontologies` returns selected serialized semantic content, effective content filter, semantic index summary, collected block metadata, diagnostics, generated ontology document declarations, ontology term declarations, and SHACL shape references.
- `reqvire.semantic.prefixes` returns ontology element-defined prefixes, namespaces, source provenance, source prose content, and a reusable SPARQL prefix block.
- `reqvire.semantic.vocabulary` returns compact paged semantic vocabulary with prefixes included in every response for SPARQL query construction.
- `reqvire.semantic.sparql` executes SPARQL against the same semantic collection used by `reqvire.semantic.ontologies`.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Model Evidence Tools](Tools.md#mcp-model-evidence-tools)
---

### MCP Mutation Concurrency Control Specification

The MCP server is expected to preserve Reqvire filesystem mutation guarantees under multi-request transports.

#### Details
Concurrency rules:
- For transports that can receive concurrent requests, mutation execution is serialized per workspace.
- The per-workspace write gate covers loading or refreshing the current model state, applying the Reqvire core mutation, flushing changed files, running required diagnostics, and refreshing MCP-visible model state.
- Two mutation requests for the same workspace must not concurrently mutate or flush overlapping model state.
- Mutation requests queue deterministically behind the per-workspace write gate.
- Read-only tools may run concurrently with other reads.
- Read-only tools may run concurrently with mutations only if each read result includes the model revision or fingerprint it observed.
- If stronger consistency is required for a tool, that read tool may take the same workspace read/write gate and wait for active mutation completion.
- Mutation results include changed files, diffs or equivalent change descriptions, and refreshed model revision/fingerprint after execution.
- Failed mutations must not leave MCP-visible cached state ahead of the filesystem.

Mutation critical section:
- Acquire the workspace mutation gate.
- Refresh or validate the current model view from Reqvire core.
- Apply the typed Reqvire core operation.
- Flush filesystem changes using the same persistence guarantees as Reqvire CLI/core.
- Run required formatting, validation, or affected-scope diagnostics.
- Refresh MCP-visible model state from the updated Reqvire core graph or reparsed filesystem state.
- Release the workspace mutation gate.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Mutation Concurrency Control](Tools.md#mcp-mutation-concurrency-control)
---

### MCP Mutation Execution Flow Specification

MCP mutation tools are expected to follow deterministic Reqvire-core-backed preview and execution behavior.

#### Details
Mutation control rules:
- MCP does not define one generic dry-run protocol for all mutation tools.
- Each mutation-capable tool exposes the operation-specific preview or execution control from the shared Reqvire operation contract.
- CRUD, relation, and asset mutation tools use `dry_run` where the Reqvire operation contract provides that control.
- Formatting uses `fix`: `fix: false` is the read-only preview behavior and `fix: true` is the mutation behavior.
- Future mutation operations may use different operation-specific controls when inherited from Reqvire core contracts.

Durable mutation flow:
- Client gathers evidence using read/report tools.
- Client prepares a typed mutation request.
- Client may send a preview request using the operation-specific non-mutating control, such as `dry_run: true` or `fix: false`.
- Server executes preview through Reqvire core without filesystem changes.
- Preview result returns diffs or equivalent change description, changed files when known, validation risks, and affected scope.
- Client sends explicit execution request using the operation-specific mutating control, such as `dry_run: false` or `fix: true`, when mutation mode is enabled.
- Server executes through Reqvire core; Reqvire core updates the in-memory graph and persists filesystem changes.
- Server runs formatting/validation diagnostics according to the tool contract.
- Server syncs its MCP internal graph view from the updated Reqvire core graph.
- Server computes affected elements/submodels for client cache invalidation.
- Server returns mutation result with changed files, diffs, diagnostics, refreshed model revision, and affected scope.

Mutation flow constraints:
- Mutation requests that bypass Reqvire model semantics are rejected.
- Arbitrary file writes are not exposed as model mutation tools.
- Single-root ownership, relation type compatibility, reused_contract_context contracts, and file persistence guarantees are inherited from Reqvire core operation contracts.
- Operation-specific preview requests for mutation-class tools are available only when mutation tools are advertised, except for conditional mutation tools such as `reqvire.format` where the read-only preview form may be advertised by default.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Mutation Execution Flow](Tools.md#mcp-mutation-execution-flow)
---

### MCP Mutation Tool Safety Specification

The MCP interface is expected to expose mutation tools only through typed Reqvire operations with explicit safety controls.

#### Details
Mutation exposure and safety rules:
- Mutation tools are omitted from MCP `tools/list` by default.
- Mutation tools are registered and returned by MCP `tools/list` only when the server is started with `reqvire mcp --enable-mutations`.
- Mutation operation semantics are inherited from reused Reqvire functional/operation contracts.
- Controlled mutations update the Reqvire in-memory graph through core mutation logic before filesystem flush.
- Controlled mutations run the same semantic model validation gates as Reqvire core before persistence. This includes ontology element structure, single connected ontology root, reused_contract_context compatibility, semantic-contract `Shapes` reference reachability, and `Concept References` resolution.
- Durable writes flush modified files to the filesystem with the same guarantees as reused file persistence behavior.
- The MCP server keeps its internal graph synchronized from the updated core graph after each successful mutation before serving subsequent model reads.
- The MCP server avoids mandatory full reparse after controlled mutations; full reparse is reserved for external filesystem drift, changed source fingerprints, or operations that require it.
- MCP mutation results add protocol metadata, refreshed model revision metadata, and affected scope metadata.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Mutation Tool Safety](Tools.md#mcp-mutation-tool-safety)
---

### MCP Prompt Guidance Specification

The MCP interface is expected to expose build-time prompt templates for regular Reqvire usage and semantic query workflows.

#### Details
Prompt capability behavior:
- The server advertises standard MCP `prompts` capability during initialization.
- The server implements `prompts/list` and `prompts/get`.
- Prompt definitions include `name`, `title`, `description`, and optional argument definitions.
- Prompt retrieval returns standard MCP prompt messages with text content.
- Prompt templates are compiled into the Rust binary using build-time string inclusion and are not loaded from workspace files.
- Prompt templates are versioned with the Reqvire binary and MCP contract implementation.

Prompt set:
- `reqvire.semantic.query` guides ontology-aware SPARQL query construction.
- `reqvire.semantic.verification_search` guides semantic verification counts and evidence lookup.
- `reqvire.semantic.contract_context_search` guides semantic-contract and reused contract context search.
- `reqvire.workflow.explore_model` guides regular read-only Reqvire model exploration.
- `reqvire.workflow.plan_change` guides model and implementation change planning.
- `reqvire.workflow.verify_coverage` guides validation, lint, coverage, and verification trace review.

Prompt content rules:
- Semantic prompts direct clients to discover prefixes and vocabulary before writing SPARQL.
- Semantic prompts reference `reqvire.semantic.vocabulary`, `reqvire.semantic.prefixes`, and `reqvire.semantic.sparql`.
- Regular workflow prompts reference non-semantic tools such as workspace status, search, read element, model, collect, lint, coverage, and traces.
- Prompt content warns clients not to rebuild semantic stores or infer prefixes from raw Turtle when MCP vocabulary/prefix tools are available.
- Prompt content distinguishes capability, requirement, contract, ontology, semantic-contract, verification, and reused contract context semantics where relevant.

Safety behavior:
- Prompt listing and retrieval do not parse arbitrary files, execute shell commands, fetch remote URLs, or mutate workspace state.
- Prompt retrieval may append client-supplied prompt arguments as context but shall not treat them as executable instructions.

#### Concept References
  * MCP prompt contract: https://www.reqvire.org/ontology#McpPromptContract
  * MCP semantic query prompt contract: https://www.reqvire.org/ontology#McpSemanticQueryPromptContract
  * MCP regular workflow prompt contract: https://www.reqvire.org/ontology#McpWorkflowPromptContract

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Prompt Guidance](Tools.md#mcp-prompt-guidance)
---

### MCP Protocol Standard Conformance Specification

The MCP interface is expected to conform to MCP protocol revision `2025-11-25`.

#### Details
Protocol conformance rules:
- The server implements MCP lifecycle initialization and version negotiation for protocol revision `2025-11-25`.
- The server rejects unsupported MCP protocol revisions using standard MCP initialization error handling.
- The server `initialize` result includes `protocolVersion`, standard `capabilities`, and `serverInfo`.
- The server declares standard MCP server capabilities using MCP capability objects. Implemented server capabilities include `tools`, `resources`, and `prompts`.
- The `tools` capability is declared as a standard MCP tools capability object. Because Reqvire tool availability is fixed for a server process after startup flags are parsed, `tools.listChanged` is omitted or false in MVP.
- The `resources` capability is declared as a standard MCP resources capability object only when resource listing/reading is implemented. Resource `subscribe` and `listChanged` are omitted or false in MVP.
- The `prompts` capability is declared as a standard MCP prompts capability object. Prompt templates are fixed at build time, so `prompts.listChanged` is omitted or false.
- The server does not advertise Reqvire domain capabilities as a custom top-level capability array.
- Concrete callable operations are advertised through MCP `tools/list`.
- Concrete resource views are advertised through MCP `resources/list` and read through MCP `resources/read` when resource capability is enabled.
- Concrete prompt templates are advertised through MCP `prompts/list` and retrieved through MCP `prompts/get` when prompt capability is enabled.
- Reqvire-specific state such as workspace status, dirty state, model revision, Reqvire tool contract version, and mutation mode is returned by Reqvire MCP tools/resources.
- Tool definitions use MCP `inputSchema`, optional `outputSchema`, and tool annotations.
- Read/report tools use `readOnlyHint: true`.
- Mutation tools are omitted from `tools/list` unless mutation mode is enabled; when present, they use `readOnlyHint: false` and conservative destructive annotations where applicable.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Protocol Standard Conformance](Tools.md#mcp-protocol-standard-conformance)
---

### MCP Quality Traceability Tools Specification

The MCP interface is expected to expose read-only quality and traceability tools grounded in Reqvire core reports.

#### Details
Quality and traceability tool behavior is inherited from reused Reqvire lint, coverage, traces, resources, and change-impact contracts. These tools return structured diagnostics and evidence after server startup validation has passed. Validation is not exposed as an MCP tool because validation is the prerequisite for starting the MCP server, matching normal Reqvire command execution behavior. Tools that compare against git commits include the compared commit and current `HEAD` in result metadata.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Quality Traceability Tools](Tools.md#mcp-quality-traceability-tools)
---

### MCP Resource Interface Specification

The MCP interface is expected to expose read-only resources for clients that support resource browsing.

#### Details
Candidate resources:
- `reqvire://workspace/status`
- `reqvire://model/summary`
- `reqvire://model/containment`
- `reqvire://model/submodels`
- `reqvire://element/{encoded_id}`
- `reqvire://file/{encoded_path}`
- `reqvire://reports/coverage`
- `reqvire://reports/lint`
- `reqvire://reports/resources`

Resources include revision metadata and must not mutate model files or cache state in ways that change observable model behavior. Resource identifiers are returned by MCP `resources/list`, parameterized resource views are returned by MCP `resources/templates/list` only if templates are implemented, and resource contents are returned by MCP `resources/read`.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Resource Interface](Tools.md#mcp-resource-interface)
---

### MCP Semantic Prefix Registry Tools Specification

The MCP interface is expected to expose read-only ontology-defined prefix discovery for semantic query construction.

#### Details
Prefix registry request:
- Tool name is `reqvire.semantic.prefixes`.
- The tool has no required arguments.
- The tool reads prefix declarations from parsed ontology element metadata and the already-built semantic model index.

Result behavior:
- Result payloads include `prefixes`, `sparql_prefix_block`, `conflicts`, `summary`, semantic `diagnostics`, and `model_fingerprint`.
- Each prefix entry includes `prefix`, `namespace`, `ontology_base`, `term_namespace`, `ontology_document_iri`, source element provenance, and contributors.
- `source` includes `element_identifier`, `element_name`, `file_path`, `line_number`, and ontology element prose `content`.
- Source `content` excludes authored Turtle and SHACL blocks, so clients receive the model element description rather than embedded RDF source text.
- `sparql_prefix_block` contains namespace declarations formatted for direct inclusion before SPARQL queries.
- Prefix conflicts are reported when the same prefix token resolves to more than one namespace.

Execution behavior:
- The tool does not scrape Turtle prefix declarations to infer Reqvire ontology prefixes.
- The tool does not rebuild, reload, or mutate the semantic RDF store for prefix discovery.
- The tool does not write generated prefix data back to Markdown source.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Semantic Prefix Registry Tools](Tools.md#mcp-semantic-prefix-registry-tools)
---

### MCP Semantic Query Tools Specification

The MCP interface is expected to expose read-only SPARQL query execution over Reqvire semantic RDF evidence.

#### Details
SPARQL tool request:
- Tool name is `reqvire.semantic.sparql`.
- Required `query` string contains a SPARQL 1.1 query.
- Optional `full` boolean defaults to `true`. When true, the queried graph includes authored ontology and SHACL RDF plus generated Reqvire model-context triples, semantic-export relation-family projection facts, and ontology projection facts. When false, the queried graph includes generated ontology document declarations plus authored ontology and SHACL RDF only.

Execution behavior:
- The validated Reqvire model owns an in-memory Oxigraph semantic store built after parsing and graph validation.
- The tool executes against the selected model-owned semantic store without rebuilding or reloading RDF for each query call.
- The tool executes the query with Oxigraph SPARQL evaluation.
- The tool does not persist an RDF store and does not write generated triples back to Markdown source.
- The tool does not expose SPARQL Update, arbitrary shell execution, arbitrary filesystem reads, or remote URL fetching.

Result behavior:
- SELECT results return ordered `variables`, `bindings`, `row_count`, and RDF term metadata for each bound value.
- ASK results return a boolean.
- CONSTRUCT and DESCRIBE results return graph triples with RDF term metadata and `triple_count`.
- Result payloads include `format: "sparql"`, the effective `full` value, semantic index `summary`, semantic `diagnostics`, and `model_fingerprint`.
- Invalid SPARQL or RDF load failures return MCP tool errors without mutating workspace state.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Semantic Query Tools](Tools.md#mcp-semantic-query-tools)
---

### MCP Semantic Relation Family Projection Access Specification

The MCP interface is expected to make relation-family projection facts available as queryable semantic graph content produced by the semantic export model.

#### Details
- `reqvire.semantic.sparql` queries relation-family projection facts only from the selected model-owned semantic store.
- `reqvire.semantic.vocabulary` may expose normalized relation-family properties and query examples from authored ontology vocabulary and semantic export contracts.
- MCP does not own relation-family projection materialization, execute the projection-side construct query, or rebuild relation-family triples per tool call.
- When `full` is false, relation-family projection facts are outside the queried graph.
- MCP does not write generated relation-family projection facts back to Markdown source.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Semantic Relation Family Projection Access](Tools.md#mcp-semantic-relation-family-projection-access)
---

### MCP Semantic Vocabulary Tools Specification

The MCP interface is expected to expose compact paged semantic vocabulary for ontology-aware query construction.

#### Details
Vocabulary tool request:
- Tool name is `reqvire.semantic.vocabulary`.
- Optional `section` defaults to `all` and accepts `all`, `prefixes`, `classes`, `properties`, `relation_families`, `controlled_vocabularies`, `semantic_contracts`, `query_patterns`, `source_map`, or `diagnostics`.
- Optional `limit` defaults to 50 and is capped at 200.
- Optional `cursor` continues a previous section page.
- Optional `filter` performs a text match over compact item content.
- Optional `include_source` defaults to true and controls source provenance where supported.
- Optional `include_examples` defaults to false and controls whether query pattern entries include SPARQL examples.

Result behavior:
- Every response includes `prefixes` and `sparql_prefix_block`.
- `section: "all"` returns section counts, section cursors, summary, prefixes, diagnostics, and model fingerprint instead of dumping every vocabulary item.
- Item section responses return `items`, `paging`, prefixes, diagnostics, and model fingerprint.
- `relation_families` items include family name, IRI/CURIE, meaning, normalized forward property, normalized inverse property, raw relation rules, and transitive flag.
- `classes` and `properties` items include IRI/CURIE, role, label/comment where authored, source when requested, and domain/range when available.
- `semantic_contracts` items include shape source and referenced SHACL target/path/class IRIs.

Execution behavior:
- The tool reads from the parsed semantic model index and ontology document declarations.
- The tool does not rebuild, reload, or mutate the semantic RDF store.
- The tool does not write generated vocabulary data back to Markdown source.

#### Concept References
  * MCP semantic vocabulary contract: https://www.reqvire.org/ontology#McpSemanticVocabularyContract
  * MCP semantic vocabulary tool contract: https://www.reqvire.org/ontology#McpSemanticVocabularyToolContract
  * Relation Family: https://www.reqvire.org/ontology#RelationFamily

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Semantic Vocabulary Tools](Tools.md#mcp-semantic-vocabulary-tools)
---

### MCP Server Command Specification

The `reqvire mcp` command is expected to start the MCP server for the current workspace.

#### Details
Command behavior:
- `reqvire mcp` starts MCP protocol service mode with read/report tools only, and MCP `tools/list` does not include mutation tools.
- `reqvire mcp --enable-mutations` starts MCP protocol service mode with mutation mode enabled, and MCP `tools/list` includes mutation tools.
- `reqvire mcp` is not exposed back through MCP as a tool.
- The server resolves the workspace root using the same workspace assumptions as Reqvire core commands.
- Startup validates the model before the server accepts protocol requests.
- Startup validation failures are forwarded from Reqvire validation diagnostics and prevent the MCP server from starting.
- Startup diagnostics include Reqvire version and supported MCP protocol revision.
- Startup fails with a clear error when the workspace cannot be resolved or parsed enough to report status.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Server Command](Tools.md#mcp-server-command)
---

### MCP Server State and Cache Specification

The MCP server is expected to cache parsed model state only as a performance optimization.

#### Details
Server state includes:
- Workspace root.
- Current git `HEAD`.
- Dirty/clean worktree status.
- Reqvire binary version.
- Supported MCP protocol revision.
- Reqvire tool contract version.
- Parsed model cache source fingerprints.
- Excluded-pattern metadata.
- Last parse and validation diagnostics.

Cache rules:
- Reqvire markdown files remain the durable source of truth.
- Reqvire core parsing remains authoritative for model semantics.
- Cached state is invalidated when source files, git state, excluded patterns, Reqvire version, or Reqvire tool contract version changes.
- Controlled MCP mutations sync MCP internal state from the updated Reqvire core graph after successful core mutation.
- External filesystem drift triggers cache invalidation and reparse before serving stale model data.
- Dirty worktree state is reported in metadata and is not a default execution blocker when the equivalent Reqvire core operation can run.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Server State and Cache](Tools.md#mcp-server-state-and-cache)
---

### MCP Shared Operation Contracts Specification

MCP tools are expected to use shared Reqvire operation contracts.

#### Details
Contract rules:
- MCP tool names map to stable Reqvire operations, not CLI spelling details.
- MCP tool names use only MCP-compatible characters: ASCII letters, digits, underscore, hyphen, and dot.
- Operation parameters become typed request fields.
- `--json` is not an MCP argument because MCP responses are structured.
- File-output transport options are not MCP arguments because MCP clients receive protocol responses.
- MCP tools inherit operation behavior from reused Reqvire functional/output contracts.
- MCP tool definitions include JSON object `inputSchema`; no-argument tools use an empty object schema.
- MCP tool definitions include `outputSchema` for structured results when the result contract is stable.
- MCP tool calls return `structuredContent` for machine-readable results and include a text content copy when needed for client compatibility.
- MCP adds protocol metadata, request/result typing, and evidence references around Reqvire operation contracts.
- In-process library callers can use the shared Reqvire tool registry to receive the same tool definitions and structured operation results before MCP protocol wrapping.
- Unknown tool calls, malformed requests, and invalid arguments use standard MCP/JSON-RPC protocol errors.
- Reqvire validation, parse, and business-logic failures use MCP tool execution errors with structured Reqvire error data where available.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Shared Operation Interfaces](Tools.md#mcp-shared-operation-interfaces)
---

### MCP Size Estimate Startup Specification

The MCP server is expected to expose element size estimates only when explicitly enabled at startup.

#### Details
- `reqvire mcp --with-size-estimates` starts the MCP process with `with_size_estimates = true` for model loading.
- `reqvire mcp` without the flag starts with size estimates disabled.
- MCP tool results that serialize model elements include `size_estimate` when the server was started with size estimates enabled.
- MCP tool results omit `size_estimate` when the server was not started with size estimates enabled.
- MCP workspace status or tool contract output reports whether size estimates are enabled for the process.
- The startup flag is a server-process option and is not exposed as a per-tool MCP request argument.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Server Command](Tools.md#mcp-server-command)
---

### MCP Streamable HTTP Transport Safety Specification

The HTTP transport is expected to follow MCP Streamable HTTP rules and safe local-server defaults.

#### Details
HTTP endpoint rules:
- The server exposes fixed endpoint `/mcp` for HTTP transport.
- The endpoint supports standard RMCP request handling for initialization, tools, resources, accepted-content negotiation, and transport errors.
- The local HTTP profile uses stateless JSON responses for request/response calls while preserving standard MCP message semantics.
- HTTP responses are produced by RMCP and shall not be generated by a Reqvire-owned hand-written HTTP JSON-RPC parser.
- HTTP transport does not introduce additional Reqvire tools, prompts, or resources.

Local safety rules:
- HTTP transport binds to `127.0.0.1` by default.
- Binding to non-localhost addresses requires explicit startup configuration.
- Requests without an `Origin` header are allowed so non-browser MCP clients can connect.
- Requests with a loopback `Origin` header are allowed for local browser-based tools through RMCP allowed-origin configuration.
- Requests with a non-loopback, `null`, file, or malformed `Origin` header are rejected by RMCP allowed-origin validation before executing MCP requests.
- Origin validation protects local HTTP MCP servers from browser-originated cross-site or DNS rebinding requests and does not restrict normal non-browser MCP clients.
- Mutation-capable HTTP servers require explicit `--enable-mutations` and must not be enabled accidentally by selecting HTTP transport.
- Non-local HTTP exposure requires an explicit authentication/authorization decision before it is considered supported.
- HTTP transport must not expose arbitrary filesystem reads, arbitrary shell execution, or server-management operations.

Session and streaming rules:
- If HTTP sessions are implemented, session identifiers are generated by the server and returned through MCP-compliant HTTP headers.
- Clients using server-assigned sessions must include the session identifier on subsequent requests.
- SSE streams, when implemented, must not send unrelated JSON-RPC responses on a stream unless allowed by the MCP transport rules.
- Streaming support is optional; lack of streaming must not change tool result semantics.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Streamable HTTP Transport Safety](Tools.md#mcp-streamable-http-transport-safety)
---

### MCP Streamable HTTP Transport Specification

The MCP server is expected to use RMCP Streamable HTTP as its protocol transport.

#### Details
Transport rules:
- `reqvire mcp` starts RMCP Streamable HTTP transport.
- Stdio transport is not supported and is not accepted as a compatibility mode.
- Transport implementation is a server startup concern and is not exposed as an MCP tool.
- Tool names, input schemas, output schemas, annotations, resources, mutation gating, and Reqvire core behavior are independent from HTTP transport mechanics unless the MCP protocol requires transport-specific metadata.
- Streamable HTTP transport uses the Rust `rmcp` streamable HTTP server transport according to MCP Streamable HTTP rules.
- HTTP transport startup options include host and port.
- HTTP transport defaults to `127.0.0.1` and fixed endpoint `/mcp`.
- HTTP transport is appropriate for long-running local service use, multiple clients, and future streaming/server-to-client notifications.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Streamable HTTP Transport](Tools.md#mcp-streamable-http-transport)
---

### MCP Structured Payload Contracts Specification

MCP tool result schemas are expected to be derived from shared Reqvire operation result contracts.

#### Details
Schema source rules:
- MCP `outputSchema` may be generated from shared Reqvire JSON/result contracts or maintained explicitly beside those contracts.
- The implementation must keep MCP `outputSchema`, MCP `structuredContent`, and the shared Reqvire operation result contract consistent.
- MCP structured payload requirements define semantic obligations, not a frozen field-level implementation layout before the shared contracts exist.
- Human-readable text `content` is compatibility output; clients must be able to use `structuredContent` for machine-readable behavior.

Common semantic obligations:
- Results identify the Reqvire operation/tool that produced them.
- Results identify the relevant workspace/model revision when the operation depends on model state.
- Results indicate dirty/clean workspace state when that affects interpretation.
- Results expose evidence references to the files, elements, relations, reused_contract_context, reports, or diffs used to produce the result when those concepts are relevant.
- Element-shaped results expose stable element identity, element type, source location, and requested relation/reused_contract_context/content views when those concepts are relevant.
- Element-shaped results preserve semantic model ADT fields when present: `ontology`, `semantic_contract`, and `concept_references`.
- Model-shaped results expose enough hierarchy, containment, and submodel boundary information to match the corresponding Reqvire operation contract.
- Mutation-shaped results expose preview/executed state, changed files, diffs or equivalent change descriptions, diagnostics, affected scope, and refreshed revision metadata when available.
- Error-shaped results expose stable Reqvire error semantics and recovery context when available.

Versioning rules:
- Schema changes that remove or rename stable structured fields require a new Reqvire tool contract version.
- Additive structured fields are allowed within a Reqvire tool contract version when clients can ignore unknown fields.
- MCP structured payloads must not require clients to parse human terminal output to recover authoritative model evidence.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Structured Payload Interfaces](Tools.md#mcp-structured-payload-interfaces)
---

### MCP Tool Call Contracts Specification

Each MCP tool is expected to have an explicit MCP tool definition and call result contract.

#### Details
All tools returned by MCP `tools/list` follow this contract:
- `name`: stable MCP-compatible tool name.
- `description`: human-readable operation summary grounded in Reqvire behavior.
- `inputSchema`: JSON object schema for the tool arguments.
- `outputSchema`: JSON object schema when the structured result contract is stable.
- `structuredContent`: machine-readable result matching `outputSchema` when `outputSchema` is provided.
- `content`: text content containing a concise human-readable summary or serialized structured result for client compatibility.
- `annotations`: MCP tool annotations describing side effects.

Common output envelope fields:
- `workspace`: workspace root, git `HEAD`, and dirty state.
- `reqvire_version`: Reqvire binary version.
- `mcp_protocol_revision`: negotiated MCP protocol revision.
- `reqvire_tool_contract_version`: Reqvire MCP tool contract version.
- `model_revision`: model fingerprint or revision identifier.
- `evidence`: files, elements, relations, reused_contract_context, or reports used to produce the result.
- `warnings`: non-fatal diagnostics.

Workspace/session tools:
- `reqvire.workspace_status`
- `reqvire.tool_contract`
- `reqvire.model_revision`

Model evidence tools:
- `reqvire.search`
- `reqvire.read_element`
- `reqvire.model`
- `reqvire.containment`
- `reqvire.collect`
- `reqvire.submodels`
- `reqvire.semantic.ontologies`
- `reqvire.semantic.prefixes`
- `reqvire.semantic.vocabulary`
- `reqvire.semantic.sparql`

Quality and traceability tools:
- `reqvire.lint`
- `reqvire.coverage`
- `reqvire.traces`
- `reqvire.resources`
- `reqvire.change_impact`

Mutation and maintenance tools:
- `reqvire.format`
- `reqvire.add_element`
- `reqvire.remove_element`
- `reqvire.move_element`
- `reqvire.rename_element`
- `reqvire.merge_elements`
- `reqvire.move_file`
- `reqvire.link`
- `reqvire.unlink`
- `reqvire.relink`
- `reqvire.move_asset`
- `reqvire.remove_asset`

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Shared Operation Interfaces](Tools.md#mcp-shared-operation-interfaces)
---

### MCP Tool Exposure Scope Specification

The MCP server is expected to expose only stable Reqvire model operations as MCP tools.

#### Details
Exposure rules:
- Do not expose a generic shell or `reqvire.command` tool.
- Do not expose hidden/internal commands such as `shell` or `sout`.
- Do not expose `reqvire mcp` as an MCP tool because it starts the server.
- Do not expose `reqvire serve` as an MCP tool because it starts an HTTP Explorer server.
- Do not expose `reqvire validate` as an MCP tool because successful validation is a server startup prerequisite.
- Expose Reqvire workflow prompts through standard MCP prompt methods rather than as MCP tools.
- CLI flags, modes, and sub-options become typed request fields on one stable MCP operation instead of nested MCP tool names.
- CLI-only transport flags such as `--json` and `--output` are never MCP tool arguments.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Tool Exposure Scope](Tools.md#mcp-tool-exposure-scope)
---

### MCP Tool Side Effect Classification Specification

Every MCP tool is expected to declare its side-effect class and availability.

#### Details
Canonical MCP side-effect classes are defined by the Reqvire interface ontology.

Classification rules:
- `read_only` tools are advertised in default `tools/list`.
- `read_only` tools use MCP annotations `readOnlyHint: true`, `destructiveHint: false`, and `openWorldHint: false`.
- `conditional_mutation` tools are advertised in default `tools/list` only when their default/allowed default-mode arguments are read-only.
- `conditional_mutation` tools reject or omit mutating arguments unless mutation mode is enabled.
- `mutation` tools are omitted from default `tools/list`.
- `mutation` tools are advertised only in mutation mode and use MCP annotations `readOnlyHint: false`, `openWorldHint: false`, and conservative `destructiveHint`.

Read-only tools:
- `reqvire.workspace_status`
- `reqvire.tool_contract`
- `reqvire.model_revision`
- `reqvire.search`
- `reqvire.read_element`
- `reqvire.model`
- `reqvire.containment`
- `reqvire.collect`
- `reqvire.submodels`
- `reqvire.semantic.ontologies`
- `reqvire.semantic.prefixes`
- `reqvire.semantic.vocabulary`
- `reqvire.semantic.sparql`
- `reqvire.lint`
- `reqvire.coverage`
- `reqvire.traces`
- `reqvire.resources`
- `reqvire.change_impact`

Conditional mutation tools:
- `reqvire.format`

Mutation tools:
- `reqvire.add_element`
- `reqvire.remove_element`
- `reqvire.move_element`
- `reqvire.rename_element`
- `reqvire.merge_elements`
- `reqvire.move_file`
- `reqvire.link`
- `reqvire.unlink`
- `reqvire.relink`
- `reqvire.move_asset`
- `reqvire.remove_asset`

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Tool Side Effect Classification](Tools.md#mcp-tool-side-effect-classification)
---

### MCP Workspace Session Tools Specification

The MCP interface is expected to expose workspace/session tools that have no direct CLI command equivalent.

#### Details
Required workspace/session tools:
- `reqvire.workspace_status`: reports workspace root, git `HEAD`, dirty state, Reqvire version, supported MCP protocol revision, Reqvire tool contract version, and last diagnostics summary.
- `reqvire.tool_contract`: reports supported tool names, request schemas, result schemas, versions, and Reqvire capability flags for the current startup mode.
- `reqvire.model_revision`: reports model fingerprint, source file metadata, excluded-pattern metadata, and cache freshness.

These tools are read-only and must not mutate the model.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Workspace Session Tools](Tools.md#mcp-workspace-session-tools)
---

