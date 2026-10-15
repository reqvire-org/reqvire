# Elements

### MCP Protocol and Tool Verification Objective

This objective groups verification that Reqvire MCP servers, tools, resources, payload contracts, access controls, and mutation boundaries conform to the supported protocol behavior.

#### Metadata
  * type: verification-objective

#### Relations
  * derive: [Embedded MCP Serve Endpoint Verification](#embedded-mcp-serve-endpoint-verification)
  * derive: [MCP Access Control Baseline Verification](#mcp-access-control-baseline-verification)
  * derive: [MCP Contract Layer Boundary Verification](#mcp-contract-layer-boundary-verification)
  * derive: [MCP Contract Versioning Verification](#mcp-contract-versioning-verification)
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
  * derive: [MCP Shared Operation Contracts Verification](#mcp-shared-operation-contracts-verification)
  * derive: [MCP Size Estimate Startup Verification](#mcp-size-estimate-startup-verification)
  * derive: [MCP Structured Payload Contracts Verification](#mcp-structured-payload-contracts-verification)
  * derive: [MCP Tool Contract and Side Effect Classification Verification](#mcp-tool-contract-and-side-effect-classification-verification)
  * derive: [MCP Tool Exposure Scope Verification](#mcp-tool-exposure-scope-verification)
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
- Execute an embedded MCP mutation and verify that the published runtime seed contains updated model data. An already-open compiled Explorer adopts the manifest and missing chunks under the owning Explorer Automatic Store Refresh Verification.
- Verify unchanged conditional manifest requests retain the materialized runtime snapshot under the owning Served Explorer Runtime Freshness Verification.
- Verify read-only embedded MCP advertises no periodic live refresh while serving selected branches through worktree contexts; mutation tools remain unavailable whether the worktree was reused or created for browsing.
- Verify `assets/project-store.js` and `ontologies.ttl` responses include no-store cache control.
- Verify `--enable-mutations` is rejected unless `--enable-mcp` is also provided for `reqvire serve`.
- Verify `/mcp` is handled by RMCP transport and is not served by the Explorer SPA fallback.
- Verify repeated `--allow-origin` values configure embedded MCP with the same origin matching and CORS preflight behavior as standalone MCP. Verify the option requires `--enable-mcp` and Explorer routes retain their own response headers.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-http-access/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-serve-command/test.sh)
  * verify: [Serve Command Embedded MCP Endpoint](../../../Interfaces/WebExplorer/Capabilities.md#serve-command-embedded-mcp-endpoint)
---

### MCP Access Control Baseline Verification

This verification shall prove that MCP does not expose arbitrary shell execution, arbitrary filesystem reads, or mutation tools unless mutation capability is enabled.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-http-access/test.sh)
  * verify: [MCP Access Control Baseline](../../../Interfaces/MCP/Tools.md#mcp-access-control-baseline)
---

### MCP Add Element Tool Verification

Acceptance review for the typed MCP `reqvire.add_element` adapter.

#### Details
- Submit typed `file` and `content` fields containing valid Markdown; require exactly one created element and insertion using core file-local hierarchy ordering.
- Exercise `override_existing: true`, preserving incoming references and validating retained ownership/type dependencies. Reject malformed content, duplicate names without override and invalid complete candidates atomically.
- Create and override one representative list-fenced payload; literal headings and declarations remain content, while real metadata and relations following the closing fence retain their meaning and code bytes are preserved.
- Use existing-file and new-file fixtures, plus a single-element destination that rejects a second sibling. Read back the accepted element and its canonical identifier.
- Require mutation enablement for the tool, including `dry_run: true`. Preview returns the validated candidate diff without writes; `dry_run: false` persists only accepted changes and subsequent reads expose the new model/revision.
- Compare a minimal representative fixture matrix and complete source tree against expected structured output and byte diffs. Require canonical formatting in affected model documents and preservation of untouched documents, fenced examples and unrelated assets.
- Check rejection guidance and unchanged source/accepted state. Reject CLI-only output controls; MCP returns results to its client.

- Operation fixtures submit typed requests through real MCP servers and compare expected output/source bytes with unified diffs. Protocol discovery and permissions are exercised by the server suite; selector fixtures exercise exact-name and identifier controls.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [element_selection_checks.py](../../../../tests/element_selection_checks.py)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-manipulation/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-override/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * satisfiedBy: [fenced_code.py](../../../../tests/test-parsing-functionality/fenced_code.py)
  * verify: [MCP Add Element Tool](../../../Interfaces/MCP/Tools.md#mcp-add-element-tool)
---

### MCP Cache and Runtime Coherence Verification

Verify that long-lived MCP tools apply the core cache freshness/publication contract and that embedded Explorer refresh follows completed successful writes.

#### Details

##### Acceptance Criteria
- In one running read-only MCP process, changing applicable root exclusions changes subsequent search/read results without restart. Editing only a used external ontology updates vocabulary and SPARQL results; their public parsed-element revision may remain unchanged. Invalid current inputs produce the applicable tool error or lenient diagnostics instead of stale success, and repaired inputs recover.
- Run equivalent model-read scenarios through standalone MCP and the embedded MCP endpoint so both adapters apply the same core cache contract.
- Hold five read-only HTTP requests at a controlled dispatch barrier and confirm they overlap on blocking threads while a ping completes on a single-thread asynchronous runtime. Verify response IDs and complete tool/resource payloads for status, search, element lookup, coverage, lint, traces and semantic queries against serial results.
- Fill read-only admission and execution limits; confirm excess requests receive a retryable capacity error before dispatch, queued cancellation frees admission, and cancelling a started read retains its execution permit and shared workspace guard until completion. Verify failed and panicking handlers release reservations. An exclusive writer must wait for active readers and take precedence over later readers; existing partial-persistence and post-write freshness checks remain applicable.
- Run five-client cold and warm reads through the real HTTP endpoints in the existing cache suite, checking complete responses, request correlation, one coordinated cold build, source-change visibility and invalid-input recovery. Record single-client and five-client timings separately from build counts; timing alone does not establish overlap.
- Coordinate a read/rebuild with a real persisted MCP write. Reads that already captured an allowed pre-write snapshot remain internally consistent; dependent reads after successful mutation completion observe the post-write graph and semantic state. Tools that cannot expose the required concurrent-read context wait for the write gate.
- In a real mutation-enabled worker under each commit policy, hold concurrent accepted reads behind controlled barriers, complete an independent read and mutation while they remain held, then confirm the delayed responses retain their original revision/HEAD/pending paths and later reads use the new snapshot. Verify final physical/recovery diagnostics without misclassifying the internal commit as an external checkout change.
- Verify out-of-order pipe replies keep request IDs and payloads paired; older responses cannot roll back published status or runtime. Confirm bounded read admission does not starve control operations, external source edits stay excluded, and dry-run/rejected mutations do not publish. Stop/remove a worker with reads in flight and confirm all callers fail without hanging, retaining ownership or substituting another context.
- Through the production in-process HTTP router and real workers, hold an accepted read across a completed mutation in both commit modes. Assert its captured revision/HEAD/pending paths, newer published runtime, JSON-RPC response IDs, overload code and retryable data. Cancelling a dispatched caller retains its read slot until completion; the separate control budget permits previews and worktree removal while reads are held.
- Move HEAD externally while an accepted read is held, then explicitly reopen the unavailable context. Confirm old callers fail with their original context identity, retained browser/read handles cannot retain ownership, and new admission uses the clean current HEAD without replacing the old caller's result with a new-context response.
- After external checkout changes, resource and prompt admission errors retain context diagnostics in protocol errors rather than returning malformed successful resource/prompt payloads.
- A superseded pre-write build cannot replace the model used for post-write MCP evidence or Explorer runtime generation. Generated store and ontology artifacts agree with the accepted post-write model under the existing runtime projection.
- Construct mutation adapter regressions through the production worktree manager and actual workers under both commit policies. Successful persisted mutations rebuild and publish the selected runtime once. Preview requests, no-op executions, JSON-RPC errors, and tool results with `isError: true` perform no runtime build, change no published assets/revision, and do not clear a previous runtime diagnostic.
- A runtime-generation or parent payload-publication failure after a successful persisted mutation preserves the last valid Explorer snapshot internally and reports runtime unavailability, while accepted model reads and their revision/HEAD/pending paths remain available. Context diagnostics survive HTTP resource/tool metadata serialization. A subsequent valid worker runtime publication clears the diagnostic without changing identical runtime content's revision.
- Read-only aggregate workspace adapters retain model reads and workspace-relative child paths, omit mutation tools, reject mutation calls, and preserve child source/index bytes without writable ownership.
- Ordinary manifest/chunk requests keep reading the published snapshot without triggering source scans or builds. External edits become visible to read-only MCP model reads under the core contract; mutation-enabled reads retain the accepted snapshot; this change does not add external-edit polling to Explorer's publication lifecycle.
- Tool names, request arguments, structured-result field names, SHA-256 model revision encoding, and Explorer manifest/chunk/ETag contracts remain compatible with the existing interface specifications.
- Count worker Git observations separately from model/runtime construction. Confirm unchanged read-only loads retain entry and exit checkout checks while using the exit observation for response metadata, failed/invalid loads do not publish a runtime, and restoring valid sources permits a fresh load. A branch change during an accepted mutation-mode read must be visible in its final physical status without changing the accepted model identity.

##### Required Evidence
- Extend the existing cache/MCP/serve suites with real server requests and committed expected results. Observe cache builds through internal test instrumentation or logs, and count the actual worker runtime-construction entry point per request phase; unchanged assets alone cannot prove that an unnecessary rebuild did not run.
- Use controlled barriers for read/write races and a mutation rejected by core validation that returns an MCP tool error inside a successful JSON-RPC response.
- Compare MCP graph/query results and generated Explorer artifacts from the accepted model while preserving existing last-valid-snapshot and browser refresh assertions. Keep the existing browser wire-hash verification unchanged.

##### Evidence Scope
The HTTP suite exercises read-only source freshness separately from authoritative mutation-mode runtime publication. Adapter tests use the production worktree manager and actual workers for ownership and publication without requiring a listening socket. Isolated mutation-session tests own detailed persistence/rollback and commit-reconciliation fault injection, using the same worker RPC dispatcher. Executing the HTTP suite remains necessary to establish transport-visible outcomes.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [mcp_cache_tests.rs](../../../../crates/reqvire-cli/src/mcp_cache_tests.rs)
  * satisfiedBy: [mcp_session_tests.rs](../../../../crates/reqvire-cli/src/mcp_session_tests.rs)
  * satisfiedBy: [model_cache_tests.rs](../../../../crates/reqvire-core/src/model_cache_tests.rs)
  * satisfiedBy: [test.sh](../../../../tests/test-cache-integration/test.sh)
  * satisfiedBy: [check_git_observations.py](../../../../tests/test-mcp-ownership/check_git_observations.py)
  * satisfiedBy: [check_parallel_reads.py](../../../../tests/test-mcp-ownership/check_parallel_reads.py)
  * verify: [MCP Mutation Concurrency Control](../../../Interfaces/MCP/Tools.md#mcp-mutation-concurrency-control)
  * verify: [MCP Mutation Execution Flow](../../../Interfaces/MCP/Tools.md#mcp-mutation-execution-flow)
  * verify: [MCP Server State and Cache](../../../Interfaces/MCP/Tools.md#mcp-server-state-and-cache)
  * verify: [Served Explorer Runtime Freshness](../../../Interfaces/WebExplorer/Capabilities.md#served-explorer-runtime-freshness)
---

### MCP Contract Binding Removal Verification

Acceptance review for the typed MCP `reqvire.unlink` adapter.

#### Details
- Remove a selected binding with typed `source` and `target`; name and identifier controls select the same dependency.
- Preserve ordinary relations and other bindings; remove the subsection when its final entry is removed.
- Reject missing endpoints or invalid resulting candidates atomically; later reads, collection and implementation coverage no longer report the removed binding.
- Require mutation enablement for the tool, including `dry_run: true`. Preview returns the validated candidate diff without writes; `dry_run: false` persists only accepted changes and subsequent reads expose the new model/revision.
- Compare a minimal representative fixture matrix and complete source tree against expected structured output and byte diffs. Require canonical formatting in affected model documents and preservation of untouched documents, fenced examples and unrelated assets.
- Check rejection guidance and unchanged source/accepted state. Reject CLI-only output controls; MCP returns results to its client.

- Operation fixtures submit typed requests through real MCP servers and compare expected output/source bytes with unified diffs. Protocol discovery and permissions are exercised by the server suite; selector fixtures exercise exact-name and identifier controls.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [element_selection_checks.py](../../../../tests/element_selection_checks.py)
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Contract Binding Tools](../../../Interfaces/MCP/Tools.md#mcp-contract-binding-tools)
---

### MCP Contract Binding Tool Verification

Acceptance review for the typed MCP `reqvire.link` adapter.

#### Details
- Use a requirement source and compatible owned contract target with `relation_type: "bindContract"`; exact names and canonical identifiers agree.
- Create Contract Bindings when absent and add the canonical source-relative contract link. Reject file/URL targets, invalid contract owners, independent-scope violations and binding fulfillment cycles.
- Reject a consumer already authoring Contract References and duplicate bindings according to the core contract. Preserve review/implementation dependency distinctions.
- Require mutation enablement for the tool, including `dry_run: true`. Preview returns the validated candidate diff without writes; `dry_run: false` persists only accepted changes and subsequent reads expose the new model/revision.
- Compare a minimal representative fixture matrix and complete source tree against expected structured output and byte diffs. Require canonical formatting in affected model documents and preservation of untouched documents, fenced examples and unrelated assets.
- Check rejection guidance and unchanged source/accepted state. Reject CLI-only output controls; MCP returns results to its client.

- Operation fixtures submit typed requests through real MCP servers and compare expected output/source bytes with unified diffs. Protocol discovery and permissions are exercised by the server suite; selector fixtures exercise exact-name and identifier controls.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [element_selection_checks.py](../../../../tests/element_selection_checks.py)
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-contract-bindings-constraints/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Contract Binding Tools](../../../Interfaces/MCP/Tools.md#mcp-contract-binding-tools)
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
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Compatibility Versioning](../../../Interfaces/MCP/Tools.md#mcp-compatibility-versioning)
---

### MCP Formatting Tool Verification

Executable verification for the conditional MCP formatting interface.

#### Details
- Use a small valid fixture containing both document forms, an unsorted authored relation subset, a concept reference, fenced content and an evidence asset. Match structured results and complete expected source trees with unified diffs.
- In default mode, omitted `fix` and `fix: false` return preview evidence without source, commit or accepted revision changes. Discovery permits only false and advertises read-only annotations; an explicit true call is rejected before side effects.
- In mutation-enabled mode, `fix: true` applies default formatting in the selected context. Require canonical affected files, preserved body/document forms and evidence bytes, coherent immediate reads and unchanged unrelated context state. Repeating application is a no-op.
- Match representative bare and list-fenced payloads byte-for-byte through preview and application. Non-reserved subsection headers and content remain within their owning element in both document forms.
- With `with_full_relations: true`, preview and application include only registered compatible inverses; default formatting preserves the authored relation subset. Require idempotence and matched source diffs.
- Reject schema errors and invalid candidates without source or accepted-state changes. Preserve shared diagnostics and guidance; require the ordinary ownership, concurrency, persistence and optional-commit rules for writes.

- The interface fixture compares read-only schemas and enabled application, matches explicit inverse output to the preview diff, and requires a second full-relations application to be a no-op. Existing formatting fixtures use typed MCP preview/application calls.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [element_selection_checks.py](../../../../tests/element_selection_checks.py)
  * satisfiedBy: [test.sh](../../../../tests/test-advanced-format/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-concept-elements/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-format-full-relations/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-interface-boundary/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Formatting Tool](../../../Interfaces/MCP/Tools.md#mcp-formatting-tool)
---

### MCP HTTP Transport End-to-End Verification

This verification shall prove that the Reqvire MCP HTTP transport preserves MCP tool semantics, local HTTP safety, and serialized workspace mutation behavior.

#### Details
Expected checks:
- Start `reqvire mcp` and verify the server binds to `127.0.0.1` by default.
- Start `reqvire mcp --host 127.0.0.1 --port <PORT>` and verify standard MCP streamable HTTP requests are accepted at fixed endpoint `/mcp`.
- Initialize default HTTP startup and explicit `--transport http` startup; compare tool, resource, and prompt discovery for equivalent behavior.
- Verify HTTP `tools/list`, `resources/list`, and representative `tools/call` responses expose the expected tool names, schemas, annotations, mutation gating, and structured result semantics.
- Compare the complete default and mutation-enabled HTTP tool catalog fixtures against the shared registry in unit tests. Both catalogs include the read-only managed query discovery and validation tools.
- Verify HTTP requests without an `Origin` header are accepted.
- Verify HTTP requests with loopback `Origin` headers are accepted.
- Verify unlisted non-loopback, `null`, file, malformed, and multiple-valued Origin headers receive HTTP 403 before tool execution.
- Configure two origins through repeated `--allow-origin` arguments on standalone and embedded MCP. Verify both origins can initialize and call tools, loopback and no-Origin clients still work, and a different scheme, port, subdomain, or unlisted origin is rejected.
- Verify allowed origins receive matching CORS headers on successful requests and protocol errors. Verify OPTIONS preflight permits POST and the specified MCP/authentication headers, exposes the MCP response headers, and varies by origin. Verify invalid origins are rejected on OPTIONS, POST, GET, and DELETE.
- Verify default HTTP/HTTPS ports match equivalent explicitly specified ports, while nondefault ports remain distinct.
- Verify wildcard, opaque, non-HTTP(S), credential-bearing, path, query, fragment, malformed, and invalid-port CLI values fail before startup. Verify `serve --allow-origin` requires `--enable-mcp` and both help surfaces describe repetition.
- Verify configuring embedded MCP CORS preserves Explorer routes and does not grant those routes cross-origin access.
- Bind to a non-wildcard hostname or IP and verify its authority at the listening port is accepted automatically. Verify wildcard binds preserve host validation and accept explicitly configured public hostnames, IP authorities, and multiple aliases.
- Verify a bare Host matches an explicitly permitted HTTP port 80, with HTTPS URI authorities using port 443, while proxy forwarding headers cannot change that interpretation.
- Verify bare and bracketed IPv6 bind forms produce the same listener hostname and correctly bracketed endpoint URL.
- Verify configured hosts initialize MCP and discover tools through standalone and embedded endpoints. Check case-insensitive exact hostname matching, explicit port restrictions, omitted-port behavior, and rejection of different hosts, subdomains, and ports before mutation execution.
- Verify `Forwarded` and `X-Forwarded-Host` headers do not authorize an otherwise rejected Host header. Verify allowing a host does not allow its browser origin, and allowing an origin does not allow its host as an endpoint authority.
- Verify malformed, wildcard, URL-shaped, credential-bearing, path-bearing, query-bearing, fragment-bearing, invalid-port, and unspecified-IP host arguments fail before startup. Verify both help surfaces describe repetition and embedded `--allow-host` requires `--enable-mcp`.
- Verify HTTP mutation tools are absent unless the server is started with `--enable-mutations`.
- Verify mutation-capable HTTP mode still requires explicit `--enable-mutations` and does not become enabled by selecting HTTP transport.
- Verify concurrent HTTP mutation requests for the same workspace are serialized so filesystem writes and post-mutation model refresh cannot interleave.
- Verify a read after an HTTP mutation observes the refreshed model state and reports the observed model revision or fingerprint.
- Verify HTTP transport behavior is provided by RMCP rather than a Reqvire-owned HTTP JSON-RPC parser.

Cache rebuild races and post-write model/runtime coherence are additionally covered by MCP Cache and Runtime Coherence Verification. Existing transport assertions alone do not establish those guarantees.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [mcp_http.rs](../../../../crates/reqvire-cli/src/mcp_http.rs)
  * satisfiedBy: [mod.rs](../../../../crates/reqvire-core/src/tool_interface/mod.rs)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-http-access/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Mutation Concurrency Control](../../../Interfaces/MCP/Tools.md#mcp-mutation-concurrency-control)
  * verify: [MCP Server Command](../../../Interfaces/MCP/Tools.md#mcp-server-command)
  * verify: [MCP Streamable HTTP Transport](../../../Interfaces/MCP/Tools.md#mcp-streamable-http-transport)
  * verify: [MCP Streamable HTTP Transport Safety](../../../Interfaces/MCP/Tools.md#mcp-streamable-http-transport-safety)
---

### MCP Link Tool Verification

Acceptance review for the typed MCP `reqvire.link` adapter.

#### Details
- Use typed `source`, `relation_type` and `target`. Element-valued endpoints accept exact names or canonical identifiers; reject ambiguous selections rather than preferring a file path.
- Create a missing Relations subsection, persist the requested authored relation and retain its inverse only in memory unless already authored. Reject duplicates, incompatible relation types, unknown endpoints and invalid hierarchy/cycles.
- Use `bindContract` or `referenceContract` to author the correct dependency subsection; preserve owner/type/scope constraints and reject mixed dependency sections. Supported resource/evidence URLs and extensionless artifact targets retain their target domains.
- Preview/apply an extensionless root evidence link in a nested document and a cross-repository artifact link; require canonical source-relative destinations, unchanged unrelated resource links and valid reload.
- Require mutation enablement for the tool, including `dry_run: true`. Preview returns the validated candidate diff without writes; `dry_run: false` persists only accepted changes and subsequent reads expose the new model/revision.
- Compare a minimal representative fixture matrix and complete source tree against expected structured output and byte diffs. Require canonical formatting in affected model documents and preservation of untouched documents, fenced examples and unrelated assets.
- Check rejection guidance and unchanged source/accepted state. Reject CLI-only output controls; MCP returns results to its client.

- Operation fixtures submit typed requests through real MCP servers and compare expected output/source bytes with unified diffs. Protocol discovery and permissions are exercised by the server suite; selector fixtures exercise exact-name and identifier controls.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [element_selection_checks.py](../../../../tests/element_selection_checks.py)
  * satisfiedBy: [test.sh](../../../../tests/test-contract-references/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-link-unlink/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Relation Management Tools](../../../Interfaces/MCP/Tools.md#mcp-relation-management-tools)
  * verify: [Relation Management Operations](../../../ModelStructure/ModelManagement.md#relation-management-operations)
---

### MCP Lint Fix Verification

Executable verification for the new repair mode on the existing MCP lint tool.

#### Details
- Compare default and mutation-enabled `tools/list` schemas and side-effect metadata. Default calls without `fix` or with `fix: false` inspect; `fix: true` is rejected before writes when mutations are disabled.
- Use one fixture with a safe redundant relation and a manual-review relation. Inspection preserves bytes and revision; enabled repair removes only the safe relation, matches expected output and source diffs, and reports changed files and applied fixes. A second repair is a no-op.
- Require immediate subsequent model reads to expose the accepted repair and default formatting preview to report no further changes in affected files. Untouched documents remain identical; general mutation fixtures cover fenced content and evidence preservation.
- Reject an invalid resulting candidate or injected persistence failure atomically, preserving files and accepted revision. Verify normal mutation ownership/concurrency gates and selected-context routing apply to repair.
- Preserve report filtering and ordinary diagnostics. No migration MCP tool appears; startup still rejects an invalid model.

- The repair fixture matches the single safe-edge removal and exact applied diff, retains the manual-review finding, and checks a second repair is a no-op. Transaction tests inject invalid candidates and failed Git publication under both commit policies.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [mcp_session_tests.rs](../../../../crates/reqvire-cli/src/mcp_session_tests.rs)
  * satisfiedBy: [element_selection_checks.py](../../../../tests/element_selection_checks.py)
  * satisfiedBy: [test.sh](../../../../tests/test-interface-boundary/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-lint-command/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Lint Fix Tool](../../../Interfaces/MCP/Tools.md#mcp-lint-fix-tool)
---

### MCP Merge Elements Tool Verification

Executable verification for typed MCP merge requests over the shared operation.

#### Details
- Use existing merge fixture inputs with typed `target`, `sources` and `dry_run` fields. Exact names and canonical identifiers select the same target and each source member; compare structured results and expected source trees with unified diffs.
- Preview on a mutation-enabled server preserves files and accepted revision. Application retains the target, merges compatible content, deduplicates converging dependencies and references, removes sources and canonically persists affected documents.
- Reject missing or wrong-type members, mixed-name/identifier duplicate or self selections and a late invalid source atomically. Preserve all source bytes and accepted read results; a later valid request succeeds. Disabled mutation discovery omits the tool and direct calls cannot write.
- Include a rejected merge whose resulting binding violates contract ownership or hierarchy scope; preserve the shared diagnostic and unchanged source tree.

- Operation fixtures submit typed requests through real MCP servers and compare expected output/source bytes with unified diffs. Protocol discovery and permissions are exercised by the server suite; selector fixtures exercise exact-name and identifier controls.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [element_selection_checks.py](../../../../tests/element_selection_checks.py)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-merge-bugfix/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-merge-elements/test.sh)
  * verify: [MCP Merge Elements Tool](../../../Interfaces/MCP/Tools.md#mcp-merge-elements-tool)
---

### MCP Model Evidence Tools Verification

This verification shall prove that model evidence tools return authoritative Reqvire model data with revision metadata.

#### Details
Expected checks:
- Search, read element, model, containment, collect, and submodels tools return data matching Reqvire core reports.
- Search supports `filter_type=ontology` and returns parsed ontology ADT content.
- Verify `tools/list` advertises `reqvire.semantic.ontologies` as a read-only semantic model evidence tool.
- Verify `tools/list` advertises `reqvire.semantic.shapes`, `reqvire.semantic.concepts`, and `reqvire.semantic.graph` as read-only semantic export tools.
- Verify `tools/list` advertises `format` selection on `reqvire.semantic.ontologies`, `reqvire.semantic.shapes`, and `reqvire.semantic.concepts`, without an `include_external` argument on those layer-specific tools.
- Verify `tools/list` advertises the `include_external` argument on `reqvire.semantic.prefixes`, `reqvire.semantic.vocabulary`, and `reqvire.semantic.sparql`.
- Verify `reqvire.semantic.ontologies` returns authored OWL/RDF ontology vocabulary, including authored `reqvire:mapsToConcept` bridges when present, and excludes SHACL shapes.
- Verify `reqvire.semantic.shapes` returns SHACL shape content and excludes authored ontology classes.
- Verify `reqvire.semantic.concepts` returns SKOS concept content and excludes authored `reqvire:mapsToConcept` bridge triples.
- Verify `reqvire.concept_mappings.list` returns the validated structural-to-native concept mapping inventory.
- Verify `reqvire.semantic.ontologies` returns generated `rdfs:isDefinedBy` links from authored named ontology resources to the generated ontology document IRI.
- Verify `reqvire.semantic.ontologies` excludes local External Ontology dependency triples and external term declarations by default.
- Verify `reqvire.semantic.export` with `layers: ["ontologies", "external-used"]` includes only the used external subset from parsed Turtle/TTL, RDF/XML, JSON-LD, and built-in external sources.
- Verify `reqvire.semantic.export` with the `external-used` layer does not generate `rdfs:isDefinedBy` ownership links for imported external ontology terms.
- Verify `reqvire.semantic.graph` is equivalent to `reqvire.semantic.export` with omitted layers and returns Reqvire model facts plus generated ontology projection facts alongside authored semantic content.
- Verify semantic prefix and vocabulary tools exclude imported external ontology vocabulary by default, include only used external subset vocabulary when `include_external` is true, and mark included imported entries as external with source metadata and used-subset materialization metadata.
- Verify `reqvire.semantic.sparql` queries the authored semantic store by default and can query only the used external subset when `include_external` is true.
- Verify unused external ontology dependency terms remain unavailable through MCP semantic ontology, vocabulary, and SPARQL outputs.
- Read element returns `concept_references` for elements that author `#### Concept References`.
- Collect returns authored concept references for capability and requirement elements and semantic-contract ontology-use context where the underlying operation returns semantic-contract evidence.
- Results include evidence references for relevant files, elements, relations, and contract_bindings.
- Read tools are allowed on dirty worktrees only when the result marks dirty state.
- Read tools do not mutate the filesystem.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Model Evidence Tools](../../../Interfaces/MCP/Tools.md#mcp-model-evidence-tools)
  * verify: [Ontology Term Definition Link Materialization](../../../Semantics/SemanticModelRequirements.md#ontology-term-definition-link-materialization)
---

### MCP Move Asset Tool Verification

Acceptance review for the typed MCP `reqvire.move_asset` adapter.

#### Details
- Use `old_path` and `new_path` within the admitted workspace to move or rename an evidence asset.
- Preserve asset bytes and supported executable modes while updating all affected model evidence links to the relocated workspace target.
- Reject missing sources, occupied destinations and invalid candidate references atomically. Return moved paths, changed referencing files and source diffs; normalization applies only to affected model documents.
- Require mutation enablement for the tool, including `dry_run: true`. Preview returns the validated candidate diff without writes; `dry_run: false` persists only accepted changes and subsequent reads expose the new model/revision.
- Compare a minimal representative fixture matrix and complete source tree against expected structured output and byte diffs. Require canonical formatting in affected model documents and preservation of untouched documents, fenced examples and unrelated assets.
- Check rejection guidance and unchanged source/accepted state. Reject CLI-only output controls; MCP returns results to its client.

- Operation fixtures submit typed requests through real MCP servers and compare expected output/source bytes with unified diffs. Protocol discovery and permissions are exercised by the server suite; selector fixtures exercise exact-name and identifier controls.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [element_selection_checks.py](../../../../tests/element_selection_checks.py)
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Move Asset Tool](../../../Interfaces/MCP/Tools.md#mcp-move-asset-tool)
---

### MCP Move Element Tool Verification

Acceptance review for the typed MCP `reqvire.move_element` adapter.

#### Details
- Use `element_name` and destination `file` to move an element into an existing or new eligible model document; exact-name and identifier controls agree.
- Require source removal, destination insertion, canonical identifier changes, correctly rebased outgoing links and rewritten incoming relations, contract dependencies and concept references.
- Preserve source/destination single-element constraints, file-local hierarchy ordering and ontology boundary rules. Reject invalid destinations and semantic candidates atomically.
- Resolve paths within the selected worktree's effective workspace independently of the caller's launch directory. Move one custom-type fixture and preserve its complete `other-` metadata token.
- Require mutation enablement for the tool, including `dry_run: true`. Preview returns the validated candidate diff without writes; `dry_run: false` persists only accepted changes and subsequent reads expose the new model/revision.
- Compare a minimal representative fixture matrix and complete source tree against expected structured output and byte diffs. Require canonical formatting in affected model documents and preservation of untouched documents, fenced examples and unrelated assets.
- Check rejection guidance and unchanged source/accepted state. Reject CLI-only output controls; MCP returns results to its client.

- Operation fixtures submit typed requests through real MCP servers and compare expected output/source bytes with unified diffs. Protocol discovery and permissions are exercised by the server suite; selector fixtures exercise exact-name and identifier controls.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [element_selection_checks.py](../../../../tests/element_selection_checks.py)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-manipulation/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mv-cross-file-relation-integrity/test.sh)
  * verify: [MCP Move Element Tool](../../../Interfaces/MCP/Tools.md#mcp-move-element-tool)
---

### MCP Move File Tool Verification

Acceptance review for the typed MCP `reqvire.move_file` adapter.

#### Details
- Use `source_file` and `target_file` to relocate all contained elements while preserving ordinary page content and fenced payloads.
- Require updated element identifiers, incoming relations/dependencies/concept references, and rebased links from moved elements, including relative evidence paths.
- With `squash: false`, reject occupied destinations; with `squash: true`, merge into an eligible destination through the core compatibility/validation rules. Remove the emptied source and reject invalid paths or resulting models atomically.
- Resolve source and destination paths within the selected worktree's effective workspace independently of the caller's launch directory; preserve scope restrictions and reject unreachable parent-directory targets.
- Require mutation enablement for the tool, including `dry_run: true`. Preview returns the validated candidate diff without writes; `dry_run: false` persists only accepted changes and subsequent reads expose the new model/revision.
- Compare a minimal representative fixture matrix and complete source tree against expected structured output and byte diffs. Require canonical formatting in affected model documents and preservation of untouched documents, fenced examples and unrelated assets.
- Check rejection guidance and unchanged source/accepted state. Reject CLI-only output controls; MCP returns results to its client.

- Operation fixtures submit typed requests through real MCP servers and compare expected output/source bytes with unified diffs. Protocol discovery and permissions are exercised by the server suite; selector fixtures exercise exact-name and identifier controls.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [element_selection_checks.py](../../../../tests/element_selection_checks.py)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-mv-file-squash/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-document-operation-constraints/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Move File Tool](../../../Interfaces/MCP/Tools.md#mcp-move-file-tool)
---

### MCP Move Folder Tool Verification

Acceptance review for the typed MCP `reqvire.move_folder` adapter.

#### Details
- Use `source_folder` and `target_folder` to move a complete eligible subtree, preserving nested files and evidence bytes/modes through core semantics.
- Require moved folder/file paths, element identifier mappings, changed referencing files, validation status and affected scope in preview and execution results.
- Rewrite references within and outside the moved subtree, including contract dependencies, concepts and assets. Reject collisions and non-Git or out-of-workspace destinations without partial moves.
- Require mutation enablement for the tool, including `dry_run: true`. Preview returns the validated candidate diff without writes; `dry_run: false` persists only accepted changes and subsequent reads expose the new model/revision.
- Compare a minimal representative fixture matrix and complete source tree against expected structured output and byte diffs. Require canonical formatting in affected model documents and preservation of untouched documents, fenced examples and unrelated assets.
- Check rejection guidance and unchanged source/accepted state. Reject CLI-only output controls; MCP returns results to its client.

- Operation fixtures submit typed requests through real MCP servers and compare expected output/source bytes with unified diffs. Protocol discovery and permissions are exercised by the server suite; selector fixtures exercise exact-name and identifier controls.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [mcp_session_tests.rs](../../../../crates/reqvire-cli/src/mcp_session_tests.rs)
  * satisfiedBy: [element_selection_checks.py](../../../../tests/element_selection_checks.py)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-mv-folder/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Move Folder Tool](../../../Interfaces/MCP/Tools.md#mcp-move-folder-tool)
---

### MCP Mutation Execution Flow Verification

This verification shall prove that MCP mutation tools follow deterministic Reqvire-core-backed preview and execution behavior.

#### Details
Expected checks:
- Verify CRUD, relation, and asset mutation preview requests use `dry_run` where provided by the shared Reqvire operation contract.
- Verify lint defaults to inspection, enabled lint repair uses `fix: true`, and formatting preview uses `fix: false` and formatting execution uses `fix: true`.
- Verify preview requests execute through Reqvire core and do not modify the filesystem.
- Verify preview results include diffs or equivalent change descriptions, changed files when known, validation risks, and affected scope.
- Verify execution requests execute only when mutation mode is enabled.
- Verify execution requests validate prepared file changes, canonically serialize affected model documents and persist the validated candidate, create one local commit only with `--enable-commits`, and publish the accepted Reqvire core graph before success is reported.
- Verify post-mutation diagnostics run according to the tool contract.
- Verify just-written documents need no further default formatting; compare expected fixture bytes. These canonical-write and lint-repair checks are new acceptance criteria awaiting implementation.
- Verify subsequent MCP reads observe the refreshed core graph state.
- Verify mutation results include changed files, diffs, diagnostics, refreshed model revision, and affected elements/submodels.
- Verify attempts to bypass Reqvire model semantics or perform arbitrary file writes are rejected.

- Verify mutation startup rejects staged, unstaged, and non-ignored untracked files, detached/unborn HEAD, invalid models, and a second owner; restart after release resumes the same branch.
- Exercise branch and worktree locks independently: keep one held, reject another owner with the exact path, acquire operation and original OS error, and retain the lock file identity/content. Restart with the same accessible files after orderly worker shutdown and forced termination; a worker surviving its parent must keep excluding a contender until that worker exits. Confirm unchanged model bytes, HEAD and index in both commit modes.
- Retain a duplicate Unix descriptor to reproduce subprocess-startup inheritance for branch, worktree and administration locks. Orderly owner release must allow immediate reacquisition while that duplicate remains open; closing the duplicate afterward must not release the new owner's lock. Run worktree reopen/remove lifecycle checks with the default parallel Rust suite.
- Reproduce denied lock-file opens and inject denied/unsupported acquisition and other OS failures. Assert operation, path, underlying error and actionable category; non-contention failures must not claim an active owner. Failed admission must release any earlier acquired lock. Do not unlink or chmod a lock to recover. Distinguish Unix fault injection/process checks from real Windows/Docker bind-mount validation.
- Without `--enable-commits`, verify successful mutations change the expected files, leave HEAD and index contents unchanged, omit the `commit` result field, and permit successive writes and reads from the accepted snapshot even though the worktree is now dirty.
- With `--enable-commits`, verify successful mutations create exactly one commit with only their prepared paths and return its identifier. In both modes, rejected, dry-run and no-op requests leave files, HEAD, index, and published runtime state unchanged and omit `commit`.
- Verify commit-disabled shutdown leaves changes uncommitted; mutation-enabled restart rejects that dirty worktree until the user commits or otherwise resolves it, then resumes the same branch without creating an extra commit.
- Verify external model edits do not enter session reads or commits, unrelated staged changes are preserved, and unexpected HEAD/branch changes reject writes.
- Verify complete candidate validation covers asset operations and formatting before persistence, and commit/persistence failures preserve accepted files and model state.
- Verify file/folder moves update subsequent reads, asset moves preserve executable modes, and newly occupied external destination files are not overwritten.
- In both commit modes, inject permission-setting denial while content writes remain possible. Existing content edits with matching executable state must retain their native permissions without a permission call, including non-default modes. Exercise new files and executable/non-executable asset moves; a required denied or ineffectual permission change must reject publication with its path and operation, preserve the accepted revision, and leave the session usable when rollback fully succeeds.
- Force a later Git publication failure and confirm restored bytes and native permissions, unchanged HEAD/index/accepted revision, and no unnecessary permission calls during restoration. Separately force a genuinely required restoration failure and confirm writes remain disabled. Verify unattempted paths are not restored and no failed candidate becomes visible.
- With `core.filemode=false`, simulate filesystem execute bits differing from tracked Git modes. Content edits and asset moves must preserve the logical index mode and avoid filesystem chmod, with the correct tree mode after explicit or automatic commit. Include real-worker checks for non-default native permissions and moves; retain a Windows host-clone/container-bind-mount reproduction procedure and distinguish local fault injection from platform validation.
- Verify successive reads reuse the accepted snapshot without filesystem cache rebuilding; semantic exports and Explorer refresh adopt successfully persisted candidates in both commit modes. In commit-enabled mode, adoption must also wait for a successful commit.
- Check shared identity of accepted model and file-map entries across read scopes. Verify request-local overlays provide read-after-write, delete/rename/directory visibility and executable-mode preservation without changing the base or importing external edits; dropping failed/preview scopes discards preparation. Capture dependencies only during explicit startup capture.
- In both commit modes, retain earlier read handles through successful and failed writes and confirm their content remains immutable. For commit-disabled sessions, exercise successive edits, deletion/recreation, moves and byte/mode reversions to the committed baseline; pending paths match only accepted differences. Failed and preview requests retain pending state, while successful explicit/automatic commits clear it and preserve unrelated staged files.
- Instrument real worker Git subprocesses for repeated accepted reads, unchanged read-only loads, and runtime metadata. Confirm reduced duplicate observations without cross-request reuse. Exercise branch/HEAD changes between requests and during loading/preparation, fresh staged/unstaged/untracked dirty state, detached/unborn metadata, invalid reload recovery, and Git observation failures. Preserve the existing commit/publication race and rollback checks.
- Exercise the production worker through its private pipes in both commit modes: repeated reads, preview, accepted add, duplicate rejection, external edit exclusion, removal back to the original file set, and a no-op explicit commit. These process checks establish worker behavior without claiming HTTP transport coverage.
- In recovery-required state, send resource reads through the production HTTP router and inspect the serialized JSON-RPC response: context identity, accepted revision, and recovery diagnostics in `_meta` must match internal dispatch. Cover both commit policies; retain the standalone/embedded HTTP E2E assertions.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [mcp_cache_tests.rs](../../../../crates/reqvire-cli/src/mcp_cache_tests.rs)
  * satisfiedBy: [mcp_session.rs](../../../../crates/reqvire-cli/src/mcp_session.rs)
  * satisfiedBy: [mcp_session_tests.rs](../../../../crates/reqvire-cli/src/mcp_session_tests.rs)
  * satisfiedBy: [mcp_worktrees.rs](../../../../crates/reqvire-cli/src/mcp_worktrees.rs)
  * satisfiedBy: [mutation_io.rs](../../../../crates/reqvire-core/src/mutation_io.rs)
  * satisfiedBy: [check_git_observations.py](../../../../tests/test-mcp-ownership/check_git_observations.py)
  * satisfiedBy: [check_locks.py](../../../../tests/test-mcp-ownership/check_locks.py)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-ownership/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Mutation Concurrency Control](../../../Interfaces/MCP/Tools.md#mcp-mutation-concurrency-control)
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
- Non-dry-run mutation requests that would break requirement contract_bindings compatibility, semantic-contract SHACL reference reachability, concept-reference resolution, or single ontology-root validation are rejected before persistence.
- Folder mutation requests verify the `reqvire.move_folder` MCP tool is absent unless mutations are enabled, then verify dry-run and execution responses include moved folder paths, moved file paths, moved element mappings, changed referencing files, validation status, and affected scope.
- After successful mutation, subsequent MCP reads observe the refreshed internal graph state.
- Post-mutation results include validation summary, refreshed model revision, and affected element/submodel metadata.

The MCP ownership suite appends standalone/embedded identifier selection under both commit policies, preview equality, ambiguity/contradictory selector rejection, unchanged files/modes/index/HEAD/accepted revision/runtime, preserved pending writes and later usable operations.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-ownership/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Mutation Tool Safety](../../../Interfaces/MCP/Tools.md#mcp-mutation-tool-safety)
---

### MCP Prompt Guidance Verification

This verification shall prove that MCP prompt templates are discoverable, retrievable, build-time static, and useful for regular Reqvire and semantic query workflows.

#### Details
Expected checks:
- Verify initialization advertises a standard MCP prompts capability.
- Verify `prompts/list` returns regular workflow prompts, semantic query prompts, authoring prompts, refactor prompts, task-generation prompts, and audit prompts.
- Verify `prompts/get` for `reqvire.semantic.query` returns text that references semantic vocabulary, prefix, SPARQL tools, ontology-document vocabulary filtering, and states that `include_external` exposes only the used external subset.
- Verify `prompts/get` for a regular workflow prompt returns text that references standard Reqvire model exploration tools.
- Verify `prompts/get` for `reqvire.workflow.audit_change_impact` returns text that references change-impact analysis, direct changes, propagated impacts, structured impact buckets, impact-scope review, invalidated verifications, binding and reference consumers, documentation artifacts, and system-model update decisions.
- Verify `prompts/get` for the new authoring and audit workflows returns text that references implementation task generation, governance metadata, Reqvire command evidence, EARS requirements, submodel inspection, verification objectives, evidence-backed satisfiedBy links, model refactor boundaries, dependency-preserving cross-subgraph replacements, ontology/semantic-contract layer decisions, ontology governance/satisfaction guardrails, native concept authoring, concept naming/identity guardrails, and model-quality audit categories.
- Retrieve exploration, authoring, refactoring, task-generation, change-impact, semantic contract-context, model-quality, and coverage prompts. Each distinguishes Contract Bindings as shared implementation obligations from Contract References as content dependencies; both propagate change impact and only bindings contribute to the owner's implementation fulfillment.
- Authoring and refactoring prompts identify `referenceContract`, element-wide section exclusivity, responsibility-based placement, and acyclic dependencies. Task and impact prompts include reference consumers in review work. Semantic context prompts include both normalized dependency predicates. Coverage prompts exclude references from fulfillment and distinguish terminal requirements from verification leaves.
- Verify unknown prompt names return a protocol error.
- Verify prompt retrieval does not mutate model source files.

Review authoring/refactor/quality prompt fixtures against the supported boundary: typed MCP authoring, correct preview/application gates, safe lint repair, canonical writes and CLI-only migrations before normal startup. Fail on removed CLI authoring instructions or an invented migration endpoint. Prompt payload implementations remain to be updated for this new contract.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [mcp_prompts.rs](../../../../crates/reqvire-core/src/mcp_prompts.rs)
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
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Protocol Standard Conformance](../../../Interfaces/MCP/Tools.md#mcp-protocol-standard-conformance)
---

### MCP Quality Traceability Tools Verification

This verification shall prove that quality and traceability tools return structured diagnostics and evidence matching Reqvire core reports.

#### Details
Expected checks:
- Lint, coverage, traces, resources, ontologies, and change-impact tools match shared Reqvire operation contracts.
- Ontologies tool returns collected ontology `Ontology` blocks and semantic-contract `Shapes` blocks in Turtle by default.
- Turtle responses from semantic MCP tools include one deterministic top-level `@prefix` declaration block, use compact prefixed names where safe, preserve multiple authored `owl:Ontology` document subjects and `owl:imports` facts, and parse as RDF/Turtle.
- Ontologies tool supports JSON-LD output through a typed MCP argument.
- JSON-LD responses remain JSON-LD RDF serializations and do not emit Turtle `@prefix` syntax.
- Semantic graph tool supports `full: true` and returns Reqvire model context triples and ontology projection facts alongside authored semantic content.
- Startup validation failures are returned before the MCP server starts.
- Git comparison tools require an eligible Git worktree and include compared commit metadata plus current eligible worktree `HEAD` metadata.
- Diagnostics are structured and machine-actionable.
- Tools do not require clients to parse human-oriented terminal output.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Quality Traceability Tools](../../../Interfaces/MCP/Tools.md#mcp-quality-traceability-tools)
---

### MCP Relink Tool Verification

Executable verification for typed MCP relation and contract-reference rewiring.

#### Details
- Use the existing relink fixture inputs with typed `source`, `relation_type`, `from_target`, `to_target` and `dry_run` fields. Match structured results and expected source trees with unified diffs; exact names and canonical identifiers select the same subjects.
- Mutation-enabled preview preserves source bytes and accepted revision. Application replaces the old endpoint exactly once, validates the complete candidate, canonically persists affected documents and exposes the accepted result to subsequent reads in the selected context.
- Reject a missing old edge, incompatible target, hierarchy or dependency cycle, multi-root candidate and a disabled direct call before persistence. Preserve files and accepted read results; a later valid request succeeds.

- Operation fixtures submit typed requests through real MCP servers and compare expected output/source bytes with unified diffs. Protocol discovery and permissions are exercised by the server suite; selector fixtures exercise exact-name and identifier controls.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [element_selection_checks.py](../../../../tests/element_selection_checks.py)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-relink-command/test.sh)
  * verify: [MCP Relink Tool](../../../Interfaces/MCP/Tools.md#mcp-relink-tool)
---

### MCP Remove Asset Tool Verification

Acceptance review for the typed MCP `reqvire.remove_asset` adapter.

#### Details
- Use `file_path` to remove an eligible evidence asset and clean incoming model references through the shared core operation.
- Report the asset deletion and changed referencing documents; preserve unrelated evidence files, model links and content.
- Reject unresolved paths and invalid candidate models without source deletion or accepted-state changes.
- Require mutation enablement for the tool, including `dry_run: true`. Preview returns the validated candidate diff without writes; `dry_run: false` persists only accepted changes and subsequent reads expose the new model/revision.
- Compare a minimal representative fixture matrix and complete source tree against expected structured output and byte diffs. Require canonical formatting in affected model documents and preservation of untouched documents, fenced examples and unrelated assets.
- Check rejection guidance and unchanged source/accepted state. Reject CLI-only output controls; MCP returns results to its client.

- Operation fixtures submit typed requests through real MCP servers and compare expected output/source bytes with unified diffs. Protocol discovery and permissions are exercised by the server suite; selector fixtures exercise exact-name and identifier controls.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [element_selection_checks.py](../../../../tests/element_selection_checks.py)
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Remove Asset Tool](../../../Interfaces/MCP/Tools.md#mcp-remove-asset-tool)
---

### MCP Remove Element Tool Verification

Acceptance review for the typed MCP `reqvire.remove_element` adapter.

#### Details
- Select the subject with `element_name` by exact name and canonical identifier; both forms produce identical removals.
- Remove incoming relations and dependencies, preserve unrelated evidence target kinds and paths, and remove an emptied eligible model file according to core deletion rules.
- Reject missing subjects, orphaning deletions and invalid remaining semantic-contract context with clear diagnostics and unchanged accepted state.
- Require mutation enablement for the tool, including `dry_run: true`. Preview returns the validated candidate diff without writes; `dry_run: false` persists only accepted changes and subsequent reads expose the new model/revision.
- Compare a minimal representative fixture matrix and complete source tree against expected structured output and byte diffs. Require canonical formatting in affected model documents and preservation of untouched documents, fenced examples and unrelated assets.
- Check rejection guidance and unchanged source/accepted state. Reject CLI-only output controls; MCP returns results to its client.

- Operation fixtures submit typed requests through real MCP servers and compare expected output/source bytes with unified diffs. Protocol discovery and permissions are exercised by the server suite; selector fixtures exercise exact-name and identifier controls.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [element_selection_checks.py](../../../../tests/element_selection_checks.py)
  * satisfiedBy: [test.sh](../../../../tests/test-concept-references/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-manipulation/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Remove Element Tool](../../../Interfaces/MCP/Tools.md#mcp-remove-element-tool)
---

### MCP Rename Element Tool Verification

Acceptance review for the typed MCP `reqvire.rename_element` adapter.

#### Details
- Use the existing-subject selector `element_name` and literal `new_name`; exact name and canonical identifier select the same subject.
- Require renamed heading/fragment and updated incoming labels and identifiers, including contract and terminology references, without interpreting the replacement name as a selector.
- Reject missing/ambiguous subjects, duplicate replacement names and invalid candidates. A valid rename after rejection succeeds without leaked candidate state.
- Require mutation enablement for the tool, including `dry_run: true`. Preview returns the validated candidate diff without writes; `dry_run: false` persists only accepted changes and subsequent reads expose the new model/revision.
- Compare a minimal representative fixture matrix and complete source tree against expected structured output and byte diffs. Require canonical formatting in affected model documents and preservation of untouched documents, fenced examples and unrelated assets.
- Check rejection guidance and unchanged source/accepted state. Reject CLI-only output controls; MCP returns results to its client.

- Operation fixtures submit typed requests through real MCP servers and compare expected output/source bytes with unified diffs. Protocol discovery and permissions are exercised by the server suite; selector fixtures exercise exact-name and identifier controls.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [element_selection_checks.py](../../../../tests/element_selection_checks.py)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-manipulation/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Rename Element Tool](../../../Interfaces/MCP/Tools.md#mcp-rename-element-tool)
  * verify: [Rename Element Operation](../../../Operations/ModelOperations/ElementManipulationRequirements.md#rename-element-operation)
---

### MCP Repository Workflow Verification Objective

This objective groups verification of isolated worktree model sessions, accepted-change commits, and optional same-repository GitHub publication.

#### Details
These are planned executable acceptance checks. Add test evidence only after implementing the fixtures and observing their results. Use disposable local Git repositories and controlled gh executables; do not publish to a live repository during regular tests.

#### Metadata
  * type: verification-objective

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
---

### MCP Accepted Change Commit Verification

Verify the successful and rejected paths of mcp accepted change commit.

#### Details
- Accumulate two accepted mutations with automatic commits disabled, explicitly commit them, and assert exact contents, parent HEAD, message, modes, pending-state clearance, and success of a subsequent mutation.
- Stage an unrelated file and edit a managed file externally after acceptance; assert neither external version enters the commit, unrelated index entries survive, and accepted model/revision stays consistent. Include accepted moves, removals, asset modes, and changes that cancel to a net no-op.
- Check auto-commit mode, empty-message rejection, repeated no-op commits, missing identity, invalid candidates, commit/ref failure, unexpected HEAD, and separate worktree commits. Assert failure retains pending changes and no operation publishes another context's files.
- Stall an explicit-commit reference-transaction hook after the ref changes and add an unrelated external file that prevents verified automatic reconciliation. Require a bounded unresolved-outcome failure, recovery-required state with audited reads available, retained accepted HEAD/model/pending paths and previous index, released index lock, and no implicit retry or ref reset.
- Confirm the fixture's published parent and exact binary asset bytes, stop the old session, reconcile affected index entries from that commit, and restart. Require the existing published HEAD, cleared recovery/pending state, writable new context, and exact captured asset bytes without another commit.
- Require the explicit-commit failure diagnostic to identify the accepted HEAD and exact attempted commit that became the branch tip.
- Verify local commit remains discoverable/executable without gh and without GitHub enablement; commit-only success must not trigger Explorer content refresh.

- Reproduce automatic and explicit publication stalls through the actual worker's 30-second deadline. When the exact intended commit was published, require automatic verified reconciliation and success with `reconciled: true`, accepted published HEAD, cleared recovery/pending state, readable candidate content, writable state, and exactly one ref-hook effect. Automatic mutation refreshes only its selected Explorer runtime; explicit commit preserves accepted model/revision and runtime identity. A subsequent commit is no-op.
- Stall both policies before ref publication and confirm the branch remains at accepted HEAD. Require an operation error, unchanged HEAD/index, released index lock, retained accepted model/revision, and writable state. Automatic mutation restores its physical source and pending state; explicit commit preserves its accepted pending source and paths uncommitted. No retry repeats the hook effect.
- In both policies, verify automatic reconciliation of exact binary bytes, unusual literal paths, deletion scope, old captured reads, and preservation of pre-existing unrelated staging and group-only execute permissions. Separately prevent automatic reconciliation with conflicting external evidence, then repair it and exercise `reqvire.git.reconcile`: default preview retains accepted HEAD/revision/index/runtime and disabled writes; explicit apply accepts only the recorded commit and restores writes without another ref side effect. Reject wrong commit, branch/tip changes, physical edits, symlinks, owner-execute mode changes, new untracked sources, merge state, affected or unrelated external index edits, and an occupied index lock. Preserve rejected content/index, accepted state, and externally held lock; repeated apply is rejected.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Repository Workflow Verification Objective](#mcp-repository-workflow-verification-objective)
  * satisfiedBy: [mcp_cache_tests.rs](../../../../crates/reqvire-cli/src/mcp_cache_tests.rs)
  * satisfiedBy: [mcp_session_tests.rs](../../../../crates/reqvire-cli/src/mcp_session_tests.rs)
  * satisfiedBy: [mcp_worktrees.rs](../../../../crates/reqvire-cli/src/mcp_worktrees.rs)
  * verify: [MCP Accepted Change Commit](../../../Interfaces/MCP/Tools.md#mcp-accepted-change-commit)
  * verify: [MCP Commit Outcome Reconciliation](../../../Interfaces/MCP/Tools.md#mcp-commit-outcome-reconciliation)
---

### MCP Branch Push Verification

Verify the successful and rejected paths of mcp branch push.

#### Details
- Push a newly committed branch to a local bare fixture remote; verify exact SHA, same branch name, upstream tracking, and no changes to other refs. Repeated publication of the same tip is a no-op.
- Reject accepted pending changes, external dirtiness, unexpected HEAD, wrong remote, non-fast-forward divergence, and denied transport/branch permissions without force or loss of local commits.
- Coordinate push with same-context mutation and simultaneous activity in another context; verify the exact intended commit is published and no implicit PR/comment is attempted.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Repository Workflow Verification Objective](#mcp-repository-workflow-verification-objective)
  * satisfiedBy: [mcp_github.rs](../../../../crates/reqvire-cli/src/mcp_github.rs)
  * verify: [MCP Branch Push](../../../Interfaces/MCP/Tools.md#mcp-branch-push)
---

### MCP GitHub Tool Availability Verification

Verify the successful and rejected paths of mcp github tool availability.

#### Details
- Test the standalone and embedded flag matrix: default, mutations only, optional commits, GitHub with/without mutations, and remote override with/without GitHub. Invalid combinations fail before listener startup.
- Supply a controlled gh executable for missing binary, bad version, authentication failure, inaccessible/wrong repository, malformed JSON, nonzero exit, and timeout. Verify MCP remains locally usable, all three publication tools are omitted, direct calls are rejected, and reasons contain no credentials.
- Give successful subprocess fixtures the production deadlines, including a delayed successful startup. Isolate the shortened deadline to an intentionally stalled authentication check and confirm that check was reached and reports a timeout, rather than accepting an unrelated startup failure as timeout coverage.
- Confirm startup uses active host authentication status rather than trusting JSON exit behavior, issues no publication commands, and retains fixed availability until restart.
- Exercise low `viewerPermission`, later revoked credentials, denied pushes, and denied comments; prove individual call results report real authorization failure while local model operations remain available. Check input/output schemas and side-effect annotations.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Repository Workflow Verification Objective](#mcp-repository-workflow-verification-objective)
  * satisfiedBy: [mcp_github.rs](../../../../crates/reqvire-cli/src/mcp_github.rs)
  * satisfiedBy: [mcp_worktrees.rs](../../../../crates/reqvire-cli/src/mcp_worktrees.rs)
  * verify: [MCP GitHub Tool Availability](../../../Interfaces/MCP/Tools.md#mcp-github-tool-availability)
---

### MCP Managed Worktree Removal Verification

Verify the successful and rejected paths of mcp managed worktree removal.

#### Details
- Remove a clean managed context with both commit policies and verify its branch/commits remain. Confirm original and other contexts still answer unchanged, while calls to the removed ID fail.
- Reject original/external worktrees, foreign ownership, invalid IDs, accepted uncommitted changes, external modified/staged/untracked files, and changed HEAD without deletion.
- Hold an operation during removal and assert orderly draining and rejection of new calls; inject Git removal failure and verify explicit stopped/residual status. Shutdown must preserve every worktree and pending file.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Repository Workflow Verification Objective](#mcp-repository-workflow-verification-objective)
  * satisfiedBy: [mcp_worktrees.rs](../../../../crates/reqvire-cli/src/mcp_worktrees.rs)
  * verify: [MCP Managed Worktree Removal](../../../Interfaces/MCP/Tools.md#mcp-managed-worktree-removal)
---

### MCP Publication Scope and Recovery Verification

Verify the successful and rejected paths of mcp publication scope and recovery.

#### Details
- Use temporary Git repositories and a recording gh double to test same-repository SSH/HTTPS identity normalization, foreign/fork targets, multiple push URLs, changed remotes, and invalid caller selectors. Assert rejection before any publication command.
- Exercise quotes, newlines, shell metacharacters, and leading hyphens in messages/bodies; assert exact argument/data preservation and no shell evaluation. Verify prompt suppression, timeouts, and sanitized errors.
- Exercise concurrent large stdout/stderr with binary stdin, exact local Git blob bytes above the publication output limit, literal unusual paths, and alternate-index isolation. Require incomplete stdin to fail an otherwise successful command while preserving nonzero stderr diagnostics, and empty stdin to close. Test each output stream's cap, exact-boundary output, prompt settings and configured SSH transport, spawn failure versus interrupted execution, descendants holding pipes with a live or exited parent, and a side effect completed before timeout without erasure or automatic retry. Run process-family termination assertions on the native platform under test; cross-compilation alone is not native cleanup confirmation.
- Inject authentication, branch-rule, non-fast-forward, subprocess, network, and ambiguous-response failures. Assert accepted snapshots, local commits, pending changes, and unrelated contexts survive unchanged.
- Test completed/failed/unknown reconciliation, including timeout after a remote side effect. Verify no blind retry, forced push, merge, reset, implicit commit, or fork.
- Run regular automated checks only against local fixture remotes and controlled gh behavior. Real GitHub publication is an explicitly authorized integration exercise, never a normal test side effect.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Repository Workflow Verification Objective](#mcp-repository-workflow-verification-objective)
  * satisfiedBy: [mcp_github.rs](../../../../crates/reqvire-cli/src/mcp_github.rs)
  * satisfiedBy: [mcp_process.rs](../../../../crates/reqvire-cli/src/mcp_process.rs)
  * verify: [MCP Publication Scope and Recovery](../../../Interfaces/MCP/Tools.md#mcp-publication-scope-and-recovery)
---

### MCP Pull Request Commenting Verification

Verify the successful and rejected paths of mcp pull request commenting.

#### Details
- Comment on a same-repository PR with the context head and with a different head, including a context with pending accepted changes. Verify exact multiline content, originating context, target PR, and comment URL/ID; HEAD and files remain unchanged.
- Reject invalid/missing PR numbers, blank bodies, foreign repository arguments, permission failures, and disabled direct calls without remote side effects.
- Lose the response after posting and verify reconciliation or an explicit unknown outcome without duplicate auto-posts. Distinguish an intentional second caller request from an automatic retry.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Repository Workflow Verification Objective](#mcp-repository-workflow-verification-objective)
  * satisfiedBy: [mcp_github.rs](../../../../crates/reqvire-cli/src/mcp_github.rs)
  * verify: [MCP Pull Request Commenting](../../../Interfaces/MCP/Tools.md#mcp-pull-request-commenting)
---

### MCP Pull Request Creation Verification

Verify the successful and rejected paths of mcp pull request creation.

#### Details
- Create ordinary and draft PRs using explicit main and feature-branch bases; verify recorded gh arguments, pushed accepted SHA, same repository, and returned number/URL. Exercise stacked PR creation.
- Reject missing/unknown/same-as-head bases, unpushed or stale remote head, no valid comparison, dirty/pending context, and foreign heads before creation. Assert no implicit push/fork/commit.
- Return an existing exact open PR without duplicates or metadata edits; surface conflicting base/multiple matches. Simulate a response lost after creation and confirm read-only reconciliation rather than blind retries.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Repository Workflow Verification Objective](#mcp-repository-workflow-verification-objective)
  * satisfiedBy: [mcp_github.rs](../../../../crates/reqvire-cli/src/mcp_github.rs)
  * verify: [MCP Pull Request Creation](../../../Interfaces/MCP/Tools.md#mcp-pull-request-creation)
---

### MCP Worktree Context Isolation Verification

Verify that MCP worker sessions implement the shared context boundary on successful and rejected paths.

#### Details
- Start with two worktrees containing identical relative paths and element names but different model content. Interleave reads, CRUD, coverage, collect, semantic prefixes/exports/SPARQL, resources, and model-bearing prompts; assert the selected content, revisions, Git HEAD, and accepted pending changes never cross contexts.
- Reject omitted selectors with multiple registered contexts and unknown/removed/unavailable IDs; allow omission for one context. Assert server-scoped discovery/listing remains callable and no request changes a global selection.
- Inspect serialized HTTP responses for omitted, unknown and removed context selectors on schema-valid read and serialized tool calls: assert `result.isError`, structured tool identity and selector diagnostics, with no JSON-RPC error or substituted context. Assert invalid selector types still use `-32602`; resource/prompt failures remain protocol errors. Repeat both commit modes, retain explicit-context success and single-context omission, and verify rejection leaves models and Git unchanged.
- Verify independent worker processes, fixed roots, exclusion rules, semantic initialization, and mutation gates. Hold one worker mutation and read/write the other; then prove reads within the held context cannot observe partial persistence.
- Test ownership contention across MCP processes, external HEAD changes, invalid/dirty startup, worker crash, reopening, and both commit modes. Failure in one worker must leave the other usable.
- Force persistence plus rollback failure in both commit modes, including captured external semantic inputs. Assert supported model/status/resource/semantic reads retain exactly the accepted model/revision and expose recovery-required state, disabled writes and the original diagnostic. Change disk files and Git availability afterward; neither may replace the accepted snapshot or produce a false clean-state observation.
- Stall an automatic-commit reference-transaction hook after the ref changes and add an unrelated external file that prevents verified automatic reconciliation. Require an unresolved-outcome diagnostic, disabled writes, the unchanged accepted HEAD/model/revision and index, released index lock, and exact accepted source content through a captured read despite changed physical files. Reject subsequent commit without repeating the ref side effect. A published candidate without conflicting evidence must instead reconcile automatically and return success under the commit verification.
- In the isolated fixture, confirm the published commit's parent and exact intended source bytes, stop the old session, repair only the affected index entries, and restart from a clean validated worktree. Require the same published HEAD, writable new context, and exact reconciled source content; do not make another commit.
- Require the automatic-commit failure diagnostic to identify the accepted HEAD and exact attempted commit that became the branch tip.
- Reject further mutations (including previews and format), explicit commits, worktree removal, live change-impact analysis, and push/PR/comment publication before their side effects. Keep discovery callable. Confirm another context remains usable, reopening the live context does not clear recovery, and worker death makes even accepted reads unavailable. Cover adapter failure injection plus production worker/process and in-process HTTP routing; do not describe socket-restricted checks as external HTTP passes.
- Exercise standalone and embedded endpoints. Each mutation refreshes only its context's Explorer runtime, and browser context selection does not redirect MCP calls. Verify resource URI/template and prompt selector schemas, isolation of cache keys, and unchanged read-only refresh behavior; do not advertise unsupported subscriptions.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Repository Workflow Verification Objective](#mcp-repository-workflow-verification-objective)
  * satisfiedBy: [mcp_cache_tests.rs](../../../../crates/reqvire-cli/src/mcp_cache_tests.rs)
  * satisfiedBy: [mcp_session_tests.rs](../../../../crates/reqvire-cli/src/mcp_session_tests.rs)
  * satisfiedBy: [mcp_worktrees.rs](../../../../crates/reqvire-cli/src/mcp_worktrees.rs)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-ownership/test.sh)
  * verify: [MCP Worktree Worker Sessions](../../../Interfaces/MCP/Tools.md#mcp-worktree-worker-sessions)
---

### MCP Worktree Creation Verification

Verify the successful and rejected paths of mcp worktree creation.

#### Details
- Create two branches from a committed owned branch, local `main`, and a locally known remote-tracking ref. Assert exact base SHAs, separate roots/workers, unchanged source checkouts, and no Git network calls.
- Reject unknown/non-commit bases, missing or ambiguous context-relative selection, invalid/duplicate branch names, dirty source state in either commit mode, foreign repositories, path overlap/symlink escape, and ownership conflicts with no source changes.
- Reproduce a valid source whose selected base has an invalid model. Inject worker/startup/worktree creation failure and verify no usable context is exposed; cleanup affects only new unchanged assets, with explicit residual-state reporting if cleanup is unsafe.
- Verify uncommitted accepted and external edits are never silently copied, committed, stashed, or discarded.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Repository Workflow Verification Objective](#mcp-repository-workflow-verification-objective)
  * satisfiedBy: [mcp_worktrees.rs](../../../../crates/reqvire-cli/src/mcp_worktrees.rs)
  * verify: [MCP Worktree Creation](../../../Interfaces/MCP/Tools.md#mcp-worktree-creation)
---

### MCP Worktree Opening and Inventory Verification

Verify the successful and rejected paths of mcp worktree opening and inventory.

#### Details
- List original, managed, external, unregistered, and unavailable worktrees without side effects. Open an existing clean branch with and without an existing worktree; repeated opening returns the same healthy context and does not create duplicate checkouts.
- Hold the repository-administration lock while reopening a healthy registered branch. Verify it returns the identical context without admission, while an unopened branch is rejected without changing registrations or checkouts; opening the new branch succeeds after lock release.
- Repeat administration failure with an unopenable lock path; assert no published context or model/HEAD/index change and reuse after repair. Use shared lock-acquisition fault injection for denied/unsupported OS failures across branch, worktree and administration paths, checking that they preserve the file and report the correct diagnostic category. Failed creation cleanup must preserve both the initiating error and any lock error explaining retained paths.
- Reject unknown/detached branches, dirty index/files/untracked files, invalid models, missing Git identity, overlapping roots, and another process's ownership. Verify pre-existing worktrees survive failures unchanged.
- Stop/restart with committed work and with pending accepted changes; confirm no implicit branch nesting or loss of files, clean reopening, and rejection of dirty reopening. Inventory must distinguish unavailable contexts from healthy sessions.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Repository Workflow Verification Objective](#mcp-repository-workflow-verification-objective)
  * satisfiedBy: [mcp_session.rs](../../../../crates/reqvire-cli/src/mcp_session.rs)
  * satisfiedBy: [mcp_worktrees.rs](../../../../crates/reqvire-cli/src/mcp_worktrees.rs)
  * verify: [MCP Worktree Opening and Inventory](../../../Interfaces/MCP/Tools.md#mcp-worktree-opening-and-inventory)
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
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Resource Interface](../../../Interfaces/MCP/Tools.md#mcp-resource-interface)
---

### MCP Scoped Coverage Verification

This verification checks coverage scope discovery, request mapping, and shared result semantics through MCP.

#### Details
Expected checks:
- Confirm discovery exposes optional string argument `from` on `reqvire.coverage`.
- Compare a request without `from` against the existing whole-model response contract.
- Request root, nested, and empty capability scopes and compare the report payload with CLI JSON for the same selector and validated model snapshot, excluding protocol envelope and transport-specific metadata.
- Include cross-root contract evidence and a shared verification in the fixture; assert preserved evidence, deduplicated counts, selected membership, and the whole-model-only orphan marker.
- Unknown and non-capability names must produce structured errors without a whole-model result.
- After a model revision changes scoped membership or supporting evidence, repeat the request and assert that the result reflects the new validated snapshot under the existing freshness contract.

The MCP ownership suite appends name/identifier scoped report equality and known wrong-type identifier diagnostics through standalone/embedded servers under both commit policies.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-ownership/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-scoped-coverage/test.sh)
  * verify: [MCP Coverage Scope Selection](../../../Interfaces/MCP/Tools.md#mcp-coverage-scope-selection)
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
- Verify `model_fingerprint` agrees with `reqvire.model_revision` for the same snapshot and its schema advertises the shared 64-character SHA-256 format.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-model-revision-hashing/test.sh)
  * verify: [MCP Semantic Prefix Registry Tools](../../../Interfaces/MCP/Tools.md#mcp-semantic-prefix-registry-tools)
---

### MCP Semantic Query Tools Verification

This verification shall prove that MCP SPARQL queries execute over Reqvire semantic RDF evidence without mutating workspace state.

#### Details
Expected checks:
- Verify `tools/list` advertises `reqvire.semantic.sparql` as a read-only tool.
- Verify `reqvire.semantic.sparql` executes a SELECT query over authored ontology and SHACL RDF.
- Verify `reqvire.semantic.sparql` uses full semantic graph context by default, including authored model facts and generated helper/projection facts.
- Verify local external ontology dependency triples are outside the default queried graph and only used external subset triples become queryable when `include_external` is true.
- Verify the full semantic graph materializes relation-family normalized predicates equivalent to the relation-family CONSTRUCT query specification.
- Verify SELECT results include ordered variables, bindings, RDF term metadata, row count, semantic index summary, diagnostics, and model fingerprint.
- Verify SELECT, ASK, and CONSTRUCT responses retain the same 64-character `model_fingerprint` for the same parsed snapshot, matching `reqvire.model_revision` and the advertised schema.
- Verify invalid SPARQL returns an MCP tool error rather than mutating files.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-model-revision-hashing/test.sh)
  * verify: [MCP Semantic Query Tools](../../../Interfaces/MCP/Tools.md#mcp-semantic-query-tools)
---

### MCP Semantic Vocabulary Tools Verification

This verification shall prove that MCP vocabulary discovery exposes compact paged semantic vocabulary with prefixes for SPARQL query construction.

#### Details
Expected checks:
- Verify `tools/list` advertises `reqvire.semantic.vocabulary` as a read-only tool.
- Verify `reqvire.semantic.vocabulary` with `section: "all"` returns section counts, prefixes, a SPARQL prefix block, diagnostics, and model fingerprint.
- Verify summary and item-section responses agree with `reqvire.model_revision` for the same snapshot and their fingerprint schema advertises the shared 64-character SHA-256 format.
- Verify imported external vocabulary is omitted by default and only used external subset vocabulary is included with `external: true` plus external source metadata when `include_external` is true.
- Verify authored vocabulary items expose `ontology_document` and can be filtered by exact `ontology_document` or `ontology_base`.
- Verify used external subset vocabulary items expose `ontology_document` from the declared external source resource or namespace fallback and can be filtered by exact document only when `include_external` is true.
- Verify `section: "relation_families"` returns relation family entries with normalized forward and inverse properties.
- Verify paging returns `next_cursor` when a section has more items than the requested limit.
- Verify query patterns include SPARQL examples when `include_examples` is true.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-model-revision-hashing/test.sh)
  * verify: [MCP Semantic Vocabulary Tools](../../../Interfaces/MCP/Tools.md#mcp-semantic-vocabulary-tools)
---

### MCP Server Command Verification

This verification shall prove that `reqvire mcp` starts protocol service mode and reports startup diagnostics without exposing itself as an MCP tool.

#### Details
Expected checks:
- Start `reqvire mcp` in a fixture workspace.
- Verify root and MCP help describe `--transport`, both `http` and `stdio`, the HTTP default, and HTTP-only listener/access options.
- Reject unknown transports and each explicit HTTP-only option in stdio mode before loading an invalid model; diagnostics identify the offending transport or option.
- Verify explicit absolute workspace selection from an unrelated launch directory isolates two project processes and their model results.
- Verify startup metadata includes Reqvire version and supported MCP protocol revision.
- Verify tool discovery does not include `reqvire.mcp`.
- Verify startup validation failures are forwarded from Reqvire diagnostics and prevent the server from accepting protocol requests.
- Verify default startup returns read/report tools only from MCP `tools/list`.
- Verify startup with `--enable-mutations` returns mutation tools from MCP `tools/list`.
- Verify root, `mcp --help`, and `serve --help` advertise `--enable-commits`, its default-disabled behavior, and mutation-mode prerequisite.
- Verify `mcp --enable-commits` and `serve --enable-mcp --enable-commits` reject the missing `--enable-mutations` before model loading/listening; `serve --enable-mutations --enable-commits` rejects missing `--enable-mcp`.
- Verify valid standalone and embedded combinations accept `--enable-commits` and reach the ordinary clean-worktree startup checks.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-cli-help-structure/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-ownership/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Server Command](../../../Interfaces/MCP/Tools.md#mcp-server-command)
  * verify: [Serve Command Embedded MCP Endpoint](../../../Interfaces/WebExplorer/Capabilities.md#serve-command-embedded-mcp-endpoint)
---

### MCP Server End-to-End Verification

This verification shall prove the Reqvire MCP server behavior through the external RMCP Streamable HTTP and stdio protocol boundaries.

#### Details
The e2e test starts `reqvire mcp` in a fixture workspace and verifies MCP initialization, capabilities, tool discovery, resource discovery and reads, structured tool calls including ontology semantic collection, protocol error handling, explicit transport selection and stdio parity, default mutation-tool omission, mutation-mode tool exposure including non-read-only annotations on mutation tools such as `reqvire.add_element`, dry-run mutation behavior including read-only `reqvire.format` preview advertising `fix` enum `[false]` in default mode, persisted mutation behavior, post-mutation reads, and startup validation failure handling.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
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

### MCP Server State and Cache Verification

This verification shall prove read-only MCP source freshness and mutation-enabled MCP reuse of its accepted persisted snapshot.

#### Details
Read-only expected checks:
- Verify workspace status reports workspace root, source-control `HEAD` and dirty state when available, Reqvire version, MCP protocol revision, Reqvire tool contract version, model fingerprint, and last diagnostics.
- Verify source file, available source-control state, excluded-pattern, Reqvire version, or Reqvire tool contract changes invalidate cached model state.
- Verify markdown content changes invalidate cached model state even when filesystem modification time is preserved.
- Verify changes to source-file inputs excluded from model revision encoding can invalidate/rebuild cached state while leaving the public model revision unchanged; the SHA-256 model revision must not replace the existing source cache key.
- Change page-only content while preserving file length and modification time; verify the next MCP search exposes the updated page content while `model_fingerprint` remains unchanged.
- Verify external filesystem drift triggers invalidation/reparse before serving stale model data.
- Verify dirty worktree state is reported in metadata when available and does not block tools when the equivalent Reqvire core operation can run.

Mutation-enabled expected checks:
- Verify accepted persisted mutations replace the graph, pages, and semantic snapshot together in both commit modes, while external edits do not replace it.
- Verify repeated reads reuse the accepted snapshot without filesystem cache rebuilding.

##### Evidence Scope
The linked hashing suite proves the unchanged-public-revision page-content freshness case. Model Cache Input Freshness Verification owns actual cache reuse and complete dependency invalidation; MCP Cache and Runtime Coherence Verification owns live configuration/dependency visibility across MCP tools and the embedded runtime boundary. Their passing regression assertions establish those guarantees independently of equal model revisions.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [mcp_cache_tests.rs](../../../../crates/reqvire-cli/src/mcp_cache_tests.rs)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-ownership/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-model-revision-hashing/test.sh)
  * verify: [MCP Server State and Cache](../../../Interfaces/MCP/Tools.md#mcp-server-state-and-cache)
---

### MCP Shared Operation Contracts Verification

This verification shall prove that MCP tools use shared Reqvire operation contracts.

#### Details
Expected checks:
- Run representative Reqvire operations through MCP tools.
- Run representative Reqvire operations through an external Rust fixture using the public Reqvire tool registry API.
- Verify source-level delegation for migrated read/report operations: MCP read tools call the shared operations layer instead of directly reimplementing search, model, collect, submodels, resources, coverage, traces, containment, lint, format, or change-impact behavior.
- Verify retained matching CLI report command handlers call the shared operations layer for migrated operations while retaining CLI-only parsing and rendering responsibilities.
- Verify result schemas match shared contract definitions.
- Verify transport-only options such as JSON stdout/file output are not exposed as MCP request fields.
- Verify no-argument tools use valid MCP object input schemas.
- Verify `reqvire.search` inputSchema advertises governance metadata filter fields `filter_status`, `filter_priority`, `filter_risk`, and `filter_owner`, and rejects unsupported governance metadata filter values with accepted-value diagnostics.
- Verify stable structured results are returned in `structuredContent` and conform to the declared `outputSchema`.
- Verify unknown tool calls, malformed requests, and schema-invalid arguments return standard MCP/JSON-RPC protocol errors.
- Verify Reqvire parse, validation, and business-logic failures are forwarded as MCP tool execution errors with structured Reqvire error data where available.

The MCP ownership suite appends report/collection/read/native-concept/managed-query name-or-identifier assertions, context and snapshot isolation, type checks and selector discovery. Schema-valid selection errors must retain structured tool errors.

The supported operation catalog must retain CLI inspections/exports/migration while routing authoring, formatting application and safe lint repair through MCP. Compare shared results for retained reads; authoring checks compare MCP with core instead of invoking removed CLI mutation handlers. These boundary checks await the implementation stage.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-ownership/test.sh)
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
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Server Command](../../../Interfaces/MCP/Tools.md#mcp-server-command)
---

### MCP Stdio Process Lifecycle Verification

This verification proves that client-managed stdio shutdown preserves model work and releases runtime ownership.

#### Details
Expected checks:
- Verify invalid model and missing-workspace startup exit nonzero, report diagnostics only on stderr, and emit no successful initialization or non-protocol stdout.
- After initialization, close stdin and require successful process exit within ten seconds. Verify stderr may contain diagnostics and every emitted stdout line is valid JSON-RPC.
- Initialize mutation-enabled stdio, close stdin, and reacquire its clean context in a fresh mutation-enabled process.
- Repeat ownership reacquisition after SIGINT and SIGTERM, allowing platform-specific signal exit status while requiring bounded shutdown.
- Create a managed worktree, apply an uncommitted fixture mutation, and shut down through EOF. Verify the worktree, branch, file contents, HEAD, and index remain intact.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Stdio Process Lifecycle](../../../Interfaces/MCP/Tools.md#mcp-stdio-process-lifecycle)
---

### MCP Stdio Transport End-to-End Verification

This verification exercises native and npm-launched stdio processes against the corresponding HTTP protocol surface in isolated project fixtures.

#### Details
Expected checks:
- Complete initialize and initialized notification handshakes; parse every stdout line as JSON-RPC and correlate responses by request ID.
- Compare default HTTP, explicit HTTP, and stdio tool/resource/template/prompt discovery. Compare complete catalogs, representative structured search results, resource metadata, and a retrieved prompt while allowing only process-specific metadata differences.
- Confirm default stdio omits mutation tools and rejects a direct mutation call without changing source files. Confirm unknown methods and invalid tool arguments return protocol errors.
- Run two stdio projects concurrently from an unrelated launch directory using absolute workspace paths; confirm each search returns only its own fixture requirement. Hold the default HTTP port occupied while these processes initialize to establish that stdio creates no HTTP listener.
- Enable mutations and compare the complete tool catalog with mutation-enabled HTTP. Preview an added requirement without changing bytes, then apply it and compare the persisted document with a committed expected fixture. Read the added element and confirm HEAD and the index remain unchanged without commit opt-in.
- Confirm size estimates are opt-in for stdio and compare their discovery/result behavior with HTTP.
- Reject dirty and already-owned mutation contexts with stderr diagnostics and no successful initialization.
- Verify root and MCP help advertise both transport choices and HTTP-only options. Reject unknown transport values and each explicit HTTP-only option, including default host and port values, before loading an invalid model.
- Launch the real npm wrapper around the captured native binary; verify initialization, a search call, JSON-only stdout, successful EOF shutdown, and propagation of native startup failure status.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Stdio Transport](../../../Interfaces/MCP/Tools.md#mcp-stdio-transport)
---

### MCP Structured Payload Contracts Verification

This verification shall prove that MCP structured payloads are consistent with shared Reqvire operation result contracts.

#### Details
Expected checks:
- Verify each MCP `outputSchema` is generated from or explicitly checked against its shared Reqvire operation result contract.
- Verify successful tool calls return `structuredContent` conforming to the declared `outputSchema`.
- Verify model revision, workspace status, semantic prefixes, vocabulary, and SPARQL keep their existing fingerprint field names and advertise and return the shared 64-character SHA-256 format.
- Verify structured results identify relevant workspace/model revision and dirty state when model state affects interpretation.
- Verify structured results expose evidence references when the underlying Reqvire operation produces file, element, relation, contract_bindings, report, or diff evidence.
- Verify element-shaped results preserve semantic model ADT fields when present, including `ontology`, `semantic_contract`, and `concept_references`.
- Verify element/model/mutation/error-shaped results preserve the semantic obligations of the corresponding shared Reqvire result contract without requiring terminal-output parsing.
- Verify removing or renaming stable structured fields requires a Reqvire tool contract version change.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-model-revision-hashing/test.sh)
  * verify: [MCP Structured Payload Interfaces](../../../Interfaces/MCP/Tools.md#mcp-structured-payload-interfaces)
---

### MCP Tool Contract and Side Effect Classification Verification

This verification shall prove that every advertised MCP tool has a complete tool definition and call contract and that tool discovery and tool annotations match the declared side-effect classification.

#### Details
Expected checks:
- For every tool returned by `tools/list`, verify `name`, `description`, `inputSchema`, annotations, and any declared `outputSchema`.
- Verify every `inputSchema` is a JSON object schema and rejects unsupported arguments.
- Verify every successful tool call includes compatible text `content` for clients that do not consume structured content.
- Verify every advertised tool has exactly one declared side-effect class.
- Verify default `tools/list` advertises all `read_only` tools and omits all `mutation` tools.
- Verify default `tools/list` advertises `conditional_mutation` tools only with read-only argument schemas.
- Verify mutation-mode `tools/list` advertises eligible local mutation tools and mutation-capable schemas for conditional mutation tools; publication tools additionally require enabled and available GitHub integration.
- Verify read-only tools declare `readOnlyHint: true`, `destructiveHint: false`, and `openWorldHint: false`.
- Verify local mutation tools declare `readOnlyHint: false`, `openWorldHint: false`, and the expected conservative `destructiveHint`; optional publication tools declare `openWorldHint: true`.
- Verify `reqvire.lint` and `reqvire.format` are conditional mutation tools: default schemas allow only `fix: false`; enabled schemas accept true, annotations match the permitted startup behavior, and disabled repair/application requests are rejected before side effects.
- Verify operation-specific preview requests for mutation-class tools are available only through mutation-class tools in mutation mode, except conditional mutation tools that explicitly expose read-only preview behavior.
- Verify Reqvire parse, validation, and operation failures produce MCP tool execution errors with structured Reqvire error data where available.
- Count catalog construction and wrapper validation during repeated calls after initialization: immutable definitions are reused, and each call enters wrapper validation once. Preserve independent worker validation and startup GitHub availability gating.
- Compare advertised definitions across read-only and mutation modes, automatic commits enabled/disabled, and GitHub unavailable/available. Reject unknown tools/fields, missing required fields, invalid types, enum values and array members, and invalid context selectors with unchanged error envelopes before any worker mutation.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [mcp.rs](../../../../crates/reqvire-cli/src/mcp.rs)
  * satisfiedBy: [mcp_worktrees.rs](../../../../crates/reqvire-cli/src/mcp_worktrees.rs)
  * satisfiedBy: [definitions.rs](../../../../crates/reqvire-core/src/tool_interface/definitions.rs)
  * satisfiedBy: [test.sh](../../../../tests/test-interface-boundary/test.sh)
  * verify: [MCP Shared Operation Interfaces](../../../Interfaces/MCP/Tools.md#mcp-shared-operation-interfaces)
  * verify: [MCP Tool Side Effect Classification](../../../Interfaces/MCP/Tools.md#mcp-tool-side-effect-classification)
---

### MCP Tool Exposure Scope Verification

This verification shall prove that MCP exposes only specified Reqvire model and typed repository workflow operations.

#### Details
Expected checks:
- Verify `tools/list` does not include a generic shell or `reqvire.command` tool.
- Verify `tools/list` does not include hidden/internal `shell` or `sout` commands.
- Verify `tools/list` does not include `reqvire.mcp`, `reqvire.serve`, or `reqvire.validate` in MVP.
- Verify prompt templates are advertised through `prompts/list`, not `tools/list`.
- Verify CLI-only transport flags such as `--json` and `--output` are absent from MCP input schemas.
- Verify CLI flags, modes, and sub-options are represented as typed request fields rather than nested tool names.

Require no migration MCP tool or pre-validation maintenance endpoint. An invalid model must fail startup before protocol requests are accepted; supported CLI migration preview/application can repair known legacy fixtures before normal validation and MCP restart.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [mcp.rs](../../../../crates/reqvire-cli/src/mcp.rs)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Tool Exposure Scope](../../../Interfaces/MCP/Tools.md#mcp-tool-exposure-scope)
---

### MCP Unlink Tool Verification

Acceptance review for the typed MCP `reqvire.unlink` adapter.

#### Details
- Use typed `source` and `target`; exact-name and canonical-identifier selections agree. Detect a matching relation, Contract Binding or Contract Reference through core rules.
- Remove the requested authored declarations and mirrored authored inverse entries where present; clean an emptied reserved subsection. Keep other relations and evidence links unchanged.
- When generated inverses exist only in memory, do not fabricate new authored declarations. Reject unresolved or invalid requests atomically and retain dependency/ownership consistency.
- Require mutation enablement for the tool, including `dry_run: true`. Preview returns the validated candidate diff without writes; `dry_run: false` persists only accepted changes and subsequent reads expose the new model/revision.
- Compare a minimal representative fixture matrix and complete source tree against expected structured output and byte diffs. Require canonical formatting in affected model documents and preservation of untouched documents, fenced examples and unrelated assets.
- Check rejection guidance and unchanged source/accepted state. Reject CLI-only output controls; MCP returns results to its client.

- Operation fixtures submit typed requests through real MCP servers and compare expected output/source bytes with unified diffs. Protocol discovery and permissions are exercised by the server suite; selector fixtures exercise exact-name and identifier controls.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [element_selection_checks.py](../../../../tests/element_selection_checks.py)
  * satisfiedBy: [test.sh](../../../../tests/test-contract-references/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-link-unlink/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * verify: [MCP Relation Management Tools](../../../Interfaces/MCP/Tools.md#mcp-relation-management-tools)
  * verify: [Relation Management Operations](../../../ModelStructure/ModelManagement.md#relation-management-operations)
---

### MCP Workspace Session Tools Verification

This verification shall prove that workspace/session tools return correct read-only metadata.

#### Details
Expected checks:
- `reqvire.workspace_status` reports workspace root, eligible worktree `HEAD` and dirty state when available, Reqvire version, supported MCP protocol revision, and Reqvire tool contract version.
- `reqvire.tool_contract` reports supported tools and schema versions for the current startup mode.
- `reqvire.model_revision` changes when included parsed-element fields change, including governance and ontology/concept namespace metadata, additions, removals, moves, and renames.
- Existing model-fingerprint fields in workspace status, model revision, semantic prefixes, semantic vocabulary, and SPARQL responses contain the same 64-character lowercase hexadecimal revision for the same snapshot.
- Reordering unordered relations, bindings, or metadata preserves the revision. Copying identical canonical inputs to another absolute workspace directory preserves it.
- Changes confined to excluded inputs such as page frontmatter, Git state, or referenced external-file bytes need not change the model revision. Applicable source-cache invalidation still occurs under its own contract.
- Field names and response shapes remain unchanged, with no per-element fingerprint additions. Schemas/examples reflect the digest migration, and the commit body and pull request description document the encoding version and compatibility guidance.
- Workspace/session tools do not modify the filesystem.

The linked MCP server suite covers workspace and tool-contract metadata. The model-revision hashing suite exercises the shared revision through real HTTP calls, fixed canonical fixtures, metadata and ordering mutations, and output-schema assertions.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](#mcp-protocol-and-tool-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-mcp-server/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-model-revision-hashing/test.sh)
  * verify: [MCP Workspace Session Tools](../../../Interfaces/MCP/Tools.md#mcp-workspace-session-tools)
---
