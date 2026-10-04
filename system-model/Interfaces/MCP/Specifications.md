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
- Model mutation results include changed files and diff. Repository and publication operations return their typed side effects and recovery context even when no model files change.
- External URLs are returned as references and are not fetched by default. Explicitly enabled GitHub operations access only the pinned repository through their publication contracts.

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
- Context routing MUST be advertised by an explicit `worktree_contexts` capability and schema revision. Existing single-context calls may omit `worktree_id`; clients must select it before using multiple contexts. GitHub requested/available and automatic-commit settings MUST be separately discoverable.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Compatibility Versioning](Tools.md#mcp-compatibility-versioning)
---

### MCP Coverage Scope Selection Specification

MCP request mapping of the shared coverage scope contract.

#### Details
- `reqvire.coverage` accepts an optional string argument `from` selecting a capability by its exact model element name.
- Omitting `from` preserves whole-model behavior. Unknown names and names of non-capability elements produce a structured tool error; never silently return whole-model coverage for an invalid explicit selection.
- The tool MUST invoke the shared coverage operation with the selector and current validated snapshot. It MUST not compute MCP-specific membership, coverage classifications, or aggregates.
- For the same model snapshot and selector, the structured report payload MUST match other consumers of the shared operation, including scope metadata, evidence identifiers, and whole-model-only orphan semantics. Protocol envelope and revision metadata remain governed by existing MCP contracts.
- Tool discovery MUST advertise the optional argument. Results need only contain the requested scope, without embedding every available scope.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Coverage Scope Selection](Tools.md#mcp-coverage-scope-selection)
---

### MCP Model Evidence Tools Specification

The MCP interface is expected to expose read-only model evidence tools grounded in Reqvire core reports and lookup behavior.

#### Details
Model evidence tool behavior is inherited from reused Reqvire search, model, containment, collect, submodel, and ontology collection contracts. MCP adds typed request/result schemas, workspace/model revision metadata, and evidence references describing which elements, files, relations, contract_bindings, ontology blocks, and shape blocks were included.

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
- `has_contract_bindings`: optional boolean requiring at least one contract_bindings.
- `filter_contract_bindings`: optional contract_bindings target glob.
- `has_contract_references`: optional boolean requiring at least one Contract Reference.
- `filter_contract_references`: optional glob matching a normalized Contract Reference target identifier. Invalid globs MUST produce a structured error; reference filters MUST combine conjunctively with other search filters.

Governance metadata filters apply to effective governance metadata values and exclude non-governance-bearing elements when active. Successful `reqvire.search` structured results include effective governance metadata for capability and requirement element evidence.

Semantic model evidence rules:
- `filter_type` accepts all canonical element type tokens supported by Reqvire core, including `capability`, `requirement`, `ontology`, `semantic-contract`, `source`, `specification`, `constraint`, `behavior`, `state`, `input-output`, and verification types.
- `reqvire.search --filter-type=ontology` through MCP returns ontology elements with parsed ontology ADT content when full results are requested.
- `reqvire.search --filter-type=semantic-contract` through MCP returns reusable shape contracts with parsed semantic-contract ADT content, constrained requirements, and ontology-use relations when full results are requested.
- `reqvire.read_element` returns `concept_references` for non-ontology, non-semantic-contract elements that author `#### Concept References`.
- `reqvire.collect` includes authored concept references for capability/requirement collection and semantic-contract ontology-use context for semantic-contract evidence where the underlying Reqvire operation returns it.
- `reqvire.model` and `reqvire.submodels` preserve capability roots, requirement ownership through `specify`/`specifiedBy`, ontology hierarchy through `derive`/`derivedFrom`, and concept-reference facts needed for semantic dependency traceability.
- `reqvire.semantic.export` exposes the canonical layer-composed semantic RDF export. It accepts optional `format`, optional repeatable-equivalent `layers` array with `ontologies`, `shapes`, `concepts`, `model`, `external-used`, `prefixes`, and `queries`, and optional `namespace_base`. Omitted or empty `layers` exports all public layers.
- `reqvire.semantic.ontologies` exposes authored OWL/RDF ontology vocabulary only. It accepts optional `format` with values `turtle` or `jsonld`; omitted format defaults to `turtle`. Authored `reqvire:mapsToConcept` bridge triples remain in this ontology layer.
- `reqvire.semantic.shapes` exposes semantic-contract SHACL shapes only. It accepts optional `format` with values `turtle` or `jsonld`; omitted format defaults to `turtle`.
- `reqvire.semantic.concepts` exposes SKOS concept scheme/thesaurus triples only. It accepts optional `format` with values `turtle` or `jsonld`; omitted format defaults to `turtle`. It does not include authored ontology bridge triples.
- `reqvire.concept_schemes.list` exposes standalone native concept schemes and their `concept_base`/`concept_prefix` namespace context.
- `reqvire.concepts.list` exposes standalone native SKOS concepts generated from Reqvire `concept` elements, with optional filtering by text or scheme IRI.
- `reqvire.concepts.get` reads one generated native concept or concept scheme by generated IRI, source element identifier, or source element name.
- `reqvire.concept_mappings.list` lists validated `reqvire:mapsToConcept` bridge triples from structural ontology terms to generated native SKOS concepts; target validation is enforced by canonical model startup and validation.
- `reqvire.semantic.model` exposes generated Reqvire model facts for elements, relations, concept references, semantic term context, and ontology projection facts. It accepts optional `format`.
- `reqvire.semantic.graph` exposes the combined public semantic export. It accepts optional `format` and is equivalent to `reqvire.semantic.export` with omitted layers.
- Semantic export responses return selected serialized content, effective semantic layer, effective external materialization state where relevant, semantic index summary, collected block metadata, graph layer metadata, diagnostics, generated ontology document declarations, visible ontology term declarations, and SHACL shape references.
- `reqvire.semantic.prefixes` returns ontology element-defined prefixes, namespaces, source provenance, source prose content, and a reusable SPARQL prefix block. It accepts optional `include_external`; omitted or false returns authored ontology prefixes only, while true also returns local external ontology source prefixes marked as external and used-subset materialization metadata.
- `reqvire.semantic.vocabulary` returns compact paged semantic vocabulary with prefixes included in every response for SPARQL query construction. It accepts optional `include_external`; omitted or false returns authored vocabulary only, while true also returns used external subset vocabulary terms marked as external with external source metadata.
- MCP semantic tools that return Turtle use the same prefixed Turtle semantic export contract as CLI output, including deterministic `@prefix` declarations, safe compact prefixed names, preservation of authored `owl:Ontology` and `owl:imports` facts, and no Turtle prefix behavior for JSON-LD responses.
- `reqvire.semantic.sparql` executes SPARQL against the semantic store used by the combined semantic graph. It accepts optional `include_external`; omitted or false queries the authored semantic store only, while true queries a store that includes only the used external subset.
- Export tool responses expose `graph_layers` metadata with layer roles `ontologies`, `shapes`, `concepts`, `model`, `external-used`, `prefixes`, `queries`, and `raw-external-source`; query-helper responses expose store graph roles `default`, `authored-ontology`, `authored-model`, `generated`, `external-used-subset`, and `raw-external-source`. Raw external source graphs remain hidden.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Model Evidence Tools](Tools.md#mcp-model-evidence-tools)
---

### MCP Mutation Concurrency Control Specification

The MCP server is expected to preserve Reqvire filesystem mutation guarantees under multi-request transports.

#### Details
Concurrency rules:
- Each mutation-enabled worktree context MUST claim exclusive ownership of its branch and worktree before accepting context-bound requests. Ownership is coordinated between Reqvire MCP processes sharing the repository and released when the owning process exits.
- Context admission, including startup, MUST require exactly one eligible Git worktree at that context's workspace root, a named branch with an existing commit, a clean index and working tree including non-ignored untracked files, configured Git author identity, and a valid model. Server startup MUST NOT create or switch branches, stash changes, or create worktrees; explicit worktree tools and browser branch selection may resolve and admit requested contexts after startup. Mutation-enabled Explorer selection MUST reject dirty targets before loading their models under these same admission rules; it cannot use a read-only fallback to bypass them.
- Ownership covers standalone MCP and the embedded mutation-enabled MCP endpoint equally. Read-only servers retain their existing dirty-worktree and source-refresh behaviour.
- Automatic Git commits default to false. Only the startup flag `--enable-commits`, together with `--enable-mutations`, enables them; enabling mutations alone MUST NOT enable commits. The commit setting remains fixed for the server session and MUST NOT change ownership, validation, serialization, or authoritative snapshot behavior.
- The accepted validated model and its captured source inputs are authoritative for that context. External model-file edits are not imported; affected managed files may be overwritten by subsequent mutations. Unrelated files and staged changes MUST NOT be included in a mutation commit.
- An unexpected branch or HEAD change MUST stop mutation execution. Ownership coordinates cooperating MCP writers; it does not prevent arbitrary external Git commands or file edits.
- Requests sharing a context MUST serialize candidate preparation, validation, persistence, local Git commit publication, and model publication. Reads MUST observe one accepted snapshot and MUST NOT publish partially persisted changes.

Mutation critical section:
- Check the owned branch and expected HEAD.
- Execute the shared core operation against the accepted model and captured files, preparing changes without modifying the working tree.
- Validate the complete candidate model before persistence, including formatting, relation, asset and file/folder operations.
- Persist only prepared changes. With automatic commits disabled, model mutation tools leave Git HEAD and index contents unchanged and leave accepted file changes uncommitted. Later mutations in the same session continue from the accepted persisted snapshot even though its own writes made the worktree dirty.
- With commits enabled, create one local Git commit for a successful non-empty mutation, parented by the last accepted HEAD. Git ref publication MUST compare the expected old HEAD.
- Preserve executable file modes when moving assets. Reject a newly occupied destination that was absent from the accepted snapshot rather than overwriting an unrelated external file.
- Publish the new graph/page/semantic snapshot after persistence succeeds and, when commits are enabled, after the commit succeeds. Include `commit` in the mutation result only when that request creates a commit; omit it with commits disabled and for previews, rejections, and no-ops.
- Dry runs, rejected candidates, and no-op executions MUST NOT create commits or advance the accepted snapshot.
- On persistence or Git publication failure, restore the operation's affected paths and retain the previous accepted model. If recovery cannot complete, disable further writes and report recovery failure explicitly.
- Recovery MUST NOT restore files into an externally changed branch or HEAD; report recovery failure and disable further writes in that case.
- Git commits use Reqvire's prepared tree and Git author/committer identity; user index contents and hook-modified trees MUST NOT be substituted for the validated candidate.
- Model mutations perform no automatic push, branch creation, worktree creation, merge, or history rewrite. Explicit worktree/commit/publication tools have their own contracts; explicit commit success advances only the selected context's expected HEAD.
- The clean-start requirement applies in both commit modes. After stopping a session that left uncommitted changes, users must commit or otherwise resolve those changes before restarting mutation-enabled MCP; restart MUST NOT silently commit or discard them.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Mutation Concurrency Control](Tools.md#mcp-mutation-concurrency-control)
---

### MCP Mutation Execution Flow Specification

MCP model mutation tools are expected to follow deterministic Reqvire-core-backed preview and execution behavior.

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
- Server prepares and validates the Reqvire core candidate, persists its affected files, commits it only when `--enable-commits` was supplied at startup, and publishes the accepted snapshot under the MCP Mutation Concurrency Control Specification.
- Server runs formatting/validation diagnostics according to the tool contract.
- Server retains the complete accepted persisted model for subsequent requests without filesystem freshness scans in mutation mode, regardless of the commit setting.
- Server computes affected elements/submodels for client cache invalidation.
- Server returns mutation result with changed files, diffs, diagnostics, refreshed model revision, and affected scope.

Mutation flow constraints:
- Mutation requests that bypass Reqvire model semantics are rejected.
- Arbitrary file writes are not exposed as model mutation tools.
- Single-root ownership, relation type compatibility, contract bindings, and file persistence guarantees are inherited from Reqvire core operation contracts.
- Operation-specific preview requests for mutation-class tools are available only when mutation tools are advertised, except for conditional mutation tools such as `reqvire.format` where the read-only preview form may be advertised by default.
- Post-write success handling requires a successful tool result and a persisted execution request. An MCP result with `isError: true`, a JSON-RPC error, or a preview response does not trigger a successful-mutation Explorer refresh.
- A successful persisted change triggers refresh of that context's Explorer runtime, when served, in either commit mode; the presence of a `commit` result field MUST NOT be the refresh condition. A refresh failure remains distinguishable from rejection before persistence; preserve the existing runtime failure diagnostic and last valid published snapshot.

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
- Controlled mutations run the same semantic model validation gates as Reqvire core before persistence. This includes ontology element structure, single connected ontology root, contract_bindings compatibility, semantic-contract `Shapes` reference reachability, and `Concept References` resolution.
- Folder mutation tools inherit the `mv-folder` contract and report moved folders, moved files, moved elements, changed referencing files, validation status, and affected scope in preview and execution results.
- Durable writes flush modified files to the filesystem with the same guarantees as reused file persistence behavior.
- The MCP server keeps its internal graph synchronized from the updated core graph after each successful mutation before serving subsequent model reads.
- A complete updated core model may be adopted after a controlled mutation only when its graph, pages, semantic state, and persisted-input identity agree under the bound cache contract. If that complete state is unavailable, invalidate and rebuild before dependent model reads; avoiding reparse is a later optimization, not grounds to serve stale derived state.
- MCP mutation results add protocol metadata, refreshed model revision metadata, and affected scope metadata.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Mutation Tool Safety](Tools.md#mcp-mutation-tool-safety)
---

### MCP Prompt Guidance Specification

The MCP interface is expected to expose build-time prompt templates for regular Reqvire usage and semantic query workflows.

#### Details
- Model-bearing prompts accept and retain `worktree_id` according to context routing. Guidance MUST keep that selector on subsequent tool calls; selecting a branch in Explorer does not select an MCP context.
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
- `reqvire.semantic.contract_context_search` guides semantic-contract and contract dependency search.
- `reqvire.semantic.author_ontology_contract` guides ontology and semantic-contract authoring with semantic vocabulary evidence.
- `reqvire.workflow.explore_model` guides regular read-only Reqvire model exploration.
- `reqvire.workflow.plan_change` guides model and implementation change planning.
- `reqvire.workflow.generate_implementation_tasks` guides traceable task generation from capability-scoped model changes.
- `reqvire.workflow.author_capability_requirement` guides capability, requirement, contract, concept-reference, and traceability authoring.
- `reqvire.workflow.author_or_align_verification` guides verification objective, concrete verification, evidence, and test-criteria alignment.
- `reqvire.workflow.refactor_model_structure` guides intent-preserving model structure refactors, contract extraction, containment, and submodel boundaries.
- `reqvire.workflow.audit_change_impact` guides system-model change-impact audits and identifies impacted elements that need model updates.
- `reqvire.workflow.author_concepts` guides native concept-scheme and concept authoring for SKOS thesauri.
- `reqvire.workflow.model_quality_audit` guides validation, lint, coverage, containment, redundant verification, and model-health audits.
- `reqvire.workflow.verify_coverage` guides validation, lint, coverage, and verification trace review.

Prompt content rules:
- Contract dependency guidance MUST follow the Contract Reference Semantics Specification referenced by the owning requirement. It MUST identify Contract Bindings as shared implementation obligations and Contract References as content dependencies, with both propagating change impact and only binding consumers contributing to owner fulfillment.
- Exploration, authoring, refactoring, task-generation, change-impact, semantic contract-context, model-quality, and coverage prompts MUST inspect both dependency kinds. Authoring and refactoring guidance MUST explain `referenceContract`, section exclusivity, acyclic dependencies, and placement by implementation responsibility. Coverage guidance MUST distinguish implementation-terminal requirements from verification leaves and exclude references from fulfillment.
- Semantic prompts direct clients to discover prefixes and vocabulary before writing SPARQL.
- Semantic prompts reference `reqvire.semantic.vocabulary`, `reqvire.semantic.prefixes`, and `reqvire.semantic.sparql`.
- Semantic prompts state that `include_external` exposes only the used external subset and is not a way to browse or dump raw full external ontology dependencies.
- Ontology/semantic-contract authoring prompts require layer decisions between native concepts, ontology, requirements, requirement-owned contracts, and semantic contracts before edits are proposed.
- Ontology/semantic-contract authoring prompts require ontology boundary checks for `ontology_base`, `ontology_prefix`, explicit Turtle prefixes, `use`/`usedBy`, `constrain`/`constrainedBy`, and SHACL-vs-OWL ownership.
- Regular workflow prompts reference non-semantic tools such as workspace status, search, read element, model, collect, lint, coverage, and traces.
- Implementation-task prompts require change-impact buckets, downstream collection from `impact_scope[]`, governance metadata (`status`, `priority`, `risk`, `owner`), requirement implementation links, verification evidence links, and binding and reference consumers to be included in task planning. They require Reqvire command evidence to be preferred over raw Markdown scanning and require task plans to separate new requirements, modified requirements, affected reusable contracts, affected verifications, and final validation/evidence updates.
- Capability/requirement authoring prompts distinguish capabilities from requirements, require EARS-style implementable obligations for requirements, preserve `specify`/`specifiedBy`, `definedBy`/`define`, concept references, semantic contracts, and verification expectations. They require submodel inspection before capability-root selection and forbid adding governance metadata unless the user, source material, or existing parent context explicitly calls for authored values.
- Verification authoring prompts require verification-objective parents, concrete verification types, `verify`/`verifiedBy` requirement targets, evidence-backed `satisfiedBy` rules, leaf-requirement rollup, and alignment between verification criteria and actual tests or evidence.
- Model-structure refactor prompts require intent preservation, contract extraction, containment checks, submodel boundary review, responsibility-based binding or reference replacements for cross-boundary reuse, and validation in slices. They require cross-subgraph dependency visibility to be preserved through explicit replacements such as contract bindings, contract references, concept references, semantic-contract relations, or local requirement-owned contracts.
- Change-impact audit prompts reference change-impact analysis and require direct changes, propagated impacts, invalidated verifications, and no-update decisions to be reported separately.
- Change-impact audit prompts instruct clients to state the comparison base; analyze the structured `added[]`, `changed[]`, `removed[]`, `relocated[]`, `impact_scope[]`, and `invalidated_verifications[]` buckets; and treat `impact_scope[]` as the high-level affected-area summary.
- Change-impact audit prompts instruct clients to collect downstream from each impact-scope root so descendants are not skipped.
- Change-impact audit prompts include change-propagation rules for parent-child hierarchy, capability-to-requirement review, requirement-to-verification invalidation, satisfiedBy evidence review, verification-only changes, and binding and reference consumers.
- Change-impact audit prompts include review of impacted documentation or assistant-guidance artifacts referencing changed specifications.
- Concept-authoring prompts require native `concept-scheme`/`concept` authoring, unique concept namespaces, SKOS taxonomy/mapping rules, concept-reference consumers, concept naming precedence, generated SKOS identity guardrails, and concept-vs-ontology decisions.
- Ontology/semantic-contract authoring prompts forbid governance metadata and implementation satisfaction claims on ontology elements.
- Model-quality audit prompts require findings to be separated into validation, coverage, lint/model-quality, containment/submodel, semantic-structure, safe auto-fix, and manual-review categories.
- Prompt content warns clients not to rebuild semantic stores or infer prefixes from raw Turtle when MCP vocabulary/prefix tools are available.
- Prompt content distinguishes capability, requirement, contract, ontology, semantic-contract, verification, and both contract dependency kinds where relevant.

Safety behavior:
- Prompt listing and retrieval do not parse arbitrary files, execute shell commands, fetch remote URLs, or mutate workspace state.
- Prompt retrieval may append client-supplied prompt arguments as context but shall not treat them as executable instructions.

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
- The `tools` capability is declared as a standard MCP tools capability object. Because Reqvire tool availability is fixed for a server process after startup flags and optional GitHub availability checks have completed, `tools.listChanged` is omitted or false in MVP.
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
Quality and traceability tool behavior is inherited from reused Reqvire lint, coverage, traces, resources, and change-impact contracts. These tools return structured diagnostics and evidence after server startup validation has passed. Validation is not exposed as an MCP tool because validation is the prerequisite for starting the MCP server, matching normal Reqvire command execution behavior. Tools that compare against source-control commits require an eligible Git worktree and include the compared commit and current eligible worktree `HEAD` in result metadata.

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

Context-qualified URIs carry `?worktree_id=<encoded context ID>`; discovery and templates identify the selected context. Unqualified reads follow the same single-context-only default as tools. Resource subscriptions remain unsupported until separately specified; context routing does not advertise new subscription capability.

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
- Optional `include_external` boolean defaults to false.
- The tool reads prefix declarations from parsed ontology element metadata and the already-built semantic model index.

Result behavior:
- Result payloads include `prefixes`, `sparql_prefix_block`, `conflicts`, `summary`, semantic `diagnostics`, and `model_fingerprint`.
- `model_fingerprint` follows the shared Model Revision Hash Specification and agrees with `reqvire.model_revision` for the same parsed snapshot.
- Each prefix entry includes `prefix`, `namespace`, `ontology_base`, `term_namespace`, `ontology_document_iri`, source element provenance, and contributors.
- Authored prefix entries are marked `external: false`; imported external ontology prefix entries are returned only when `include_external` is true, are marked `external: true`, and identify `external_materialization: "used_subset"`.
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
- Optional `full` boolean defaults to `true`. When true, the queried graph includes authored ontology and SHACL RDF, authored model facts, semantic-export relation-family projection facts, and generated ontology projection facts. When false, the queried graph includes authored ontology and SHACL RDF only.
- Optional `include_external` boolean defaults to false. When true, the selected graph also includes only the used external ontology subset derived from parsed local external dependency files.

Execution behavior:
- The validated Reqvire model owns captured RDF inputs and initializes the selected in-memory Oxigraph store on first query use under the In-Memory Model Build Cache Specification.
- Subsequent calls reuse that snapshot's selected store. Initialization uses its captured RDF inputs and propagates preparation errors through the existing tool error contract.
- The tool executes the query with Oxigraph SPARQL evaluation.
- The tool does not persist an RDF store and does not write generated triples back to Markdown source.
- The tool does not expose SPARQL Update, arbitrary shell execution, arbitrary filesystem reads, or remote URL fetching.

Result behavior:
- SELECT results return ordered `variables`, `bindings`, `row_count`, and RDF term metadata for each bound value.
- ASK results return a boolean.
- CONSTRUCT and DESCRIBE results return graph triples with RDF term metadata and `triple_count`.
- Result payloads include `format: "sparql"`, the effective `full` value, the effective `include_external` value, semantic index `summary`, semantic `diagnostics`, and `model_fingerprint`.
- SELECT, ASK, and graph results use the shared Model Revision Hash Specification. The fingerprint identifies the parsed snapshot and is independent of query text and result shape.
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
- Optional `ontology_document` filters vocabulary items to terms defined by an exact OWL ontology document IRI.
- Optional `ontology_base` is an alias for `ontology_document` because the resolved Reqvire `ontology_base` is the generated OWL ontology document IRI.
- Optional `include_source` defaults to true and controls source provenance where supported.
- Optional `include_examples` defaults to false and controls whether query pattern entries include SPARQL examples.
- Optional `include_external` defaults to false and controls whether used external ontology subset vocabulary appears in prefixes and item sections.

Result behavior:
- Every response includes `prefixes` and `sparql_prefix_block`.
- `section: "all"` returns section counts, section cursors, summary, prefixes, diagnostics, and model fingerprint instead of dumping every vocabulary item.
- Item section responses return `items`, `paging`, prefixes, diagnostics, and model fingerprint.
- Both summary and item-section `model_fingerprint` values follow the shared Model Revision Hash Specification and agree with `reqvire.model_revision` for the same parsed snapshot.
- `relation_families` items include family name, IRI/CURIE, meaning, normalized forward property, normalized inverse property, raw relation rules, and transitive flag.
- `classes` and `properties` items include IRI/CURIE, role, external marker, label/comment where available, source when requested, and domain/range when available.
- Authored `classes` and `properties` items include `ontology_document` when Reqvire can resolve the owning OWL document.
- Imported external vocabulary items are omitted by default; when included, only used external subset items are returned, marked `external: true`, carry `ontology_document` from the declared external ontology source resource or namespace fallback, and use external source metadata.
- `semantic_contracts` items include shape source and referenced SHACL target/path/class IRIs.

Execution behavior:
- The tool reads from the parsed semantic model index and ontology document declarations.
- The tool does not rebuild, reload, or mutate the semantic RDF store.
- The tool does not write generated vocabulary data back to Markdown source.

#### Concept References
  * [Relation Family](../../Thesaurus/Thesaurus.md#relation-family)

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
- `reqvire mcp --enable-mutations` claims the current clean branch and worktree as the original context under the MCP Mutation Concurrency Control Specification, validates the model, and advertises mutation tools, with automatic commits disabled.
- `reqvire mcp --enable-mutations --enable-commits` additionally enables automatic local commits. `--enable-commits` is a boolean startup flag, defaults to false when absent, and requires `--enable-mutations`. Invalid combinations MUST fail argument validation before model loading or opening a listener.
- `reqvire mcp --enable-mutations --enable-github` enables startup checks for optional publishing tools. `--github-remote <NAME>` selects the configured remote and defaults to `origin`. GitHub enablement requires mutations; the remote override requires GitHub enablement. Checks and degraded local operation follow MCP GitHub Tool Availability Specification.
- Root and MCP command help MUST describe `--enable-commits` as opt-in automatic commits requiring mutation mode.
- `reqvire mcp --allow-origin <ORIGIN>` MUST accept repeatable additional browser origins according to the MCP Streamable HTTP Transport Safety Specification bound by its owning requirement. Invalid values MUST fail argument validation before model loading or opening a listener.
- `reqvire mcp --allow-host <HOST[:PORT]>` MUST accept repeatable endpoint hostnames according to the same bound transport safety contract. Invalid values MUST fail argument validation before model loading or opening a listener.
- `reqvire mcp` is not exposed back through MCP as a tool.
- The server resolves the workspace root using the Workspace Scope Specification shared with Reqvire core commands.
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

Read-only MCP caches parsed model state as a performance optimization. Mutation-enabled MCP retains the authoritative accepted model separately for each owned worktree context.

#### Details
Per-context server state includes:
- Context ID, workspace root, branch ownership, expected HEAD, commit policy, and pending accepted changes.
- Eligible Git worktree roots and their current `HEAD` values when available.
- Dirty/clean worktree status for eligible Git worktrees when available.
- Reqvire binary version.
- Supported MCP protocol revision.
- Reqvire tool contract version.
- Parsed model cache identity and dependency observations under the bound In-Memory Model Build Cache Specification.
- Active exclusion configuration and its matching policy.
- Last parse and validation diagnostics.

Read-only cache rules (mutation mode instead follows the ownership and accepted-snapshot contract above):
- Eligible Git-worktree Reqvire markdown files remain the durable source of truth.
- Reqvire core parsing remains authoritative for model semantics.
- Model-loading tools and resources use the bound core cache construction identity, dependency freshness, build coordination, and publication contract; MCP does not maintain a second parsed-model cache.
- Changes to applicable root ignore files are observed before the next model read and update the actual matcher and selected inventory without restarting MCP. Unchanged effective rules remain stable across regex use and worker threads.
- Local external ontology bytes and other construction/validation dependencies participate in freshness under the core contract. Semantic prefixes, vocabulary, exports, and SPARQL use derived state from the same completed model as the graph. Missing or invalid current inputs follow the owning strict/lenient operation's error behavior instead of silently returning older semantic data.
- Public model revisions follow the bound Model Revision Hash Specification. They do not replace the parsed-model cache key: source bytes, build options, excluded patterns, and source-control metadata retain their existing invalidation responsibilities. Migrating model revisions does not migrate the existing file-content hash algorithm.
- Cached state is invalidated when relevant source/dependency observations, eligible Git worktree metadata state, effective exclusions, Reqvire version, or Reqvire tool contract version changes. Source changes can require refresh while the public parsed-element revision remains unchanged.
- Mutation-enabled sessions reuse their accepted model and captured inputs. Successful persistence, including a successful commit when enabled, replaces that snapshot and its derived semantic state together; failed operations retain the previous snapshot. A missing `commit` field does not prevent adoption in the default commit-disabled mode.
- Each response is constructed from one completed model; any exposed model fingerprint describes that response's parsed elements. The source-cache generation is internal and is not inferred from the public model fingerprint.
- Core cache identity and public SHA-256 revision encoding retain their existing contracts. Context routing adds explicit context identity; clients MUST NOT use a model fingerprint alone as a cross-context cache key.
- Read-only mode reports dirty state without blocking compatible read operations. Mutation-enabled startup requires a clean worktree.

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
- Endpoint host validation MUST retain loopback hostnames and add an explicitly configured non-wildcard bind hostname or IP address at the actual listening port. IPv6 bind addresses MUST be accepted in bare or bracketed form and formatted with brackets in displayed endpoint URLs.
- Repeatable `--allow-host <HOST[:PORT]>` options MUST add accepted endpoint authorities for direct access or a reverse proxy. Values MUST be concrete DNS names, IPv4 addresses, or bracketed IPv6 addresses with an optional valid port, without a scheme, credentials, path, query, fragment, wildcard, or unspecified IP address. Hostname matching MUST be case-insensitive and exact; an omitted port permits that named host on any port, while an explicit port limits it to that port.
- Wildcard bind addresses `0.0.0.0` and `::` MUST select listening interfaces without disabling host validation. Non-loopback request hosts used with a wildcard listener MUST be configured through `--allow-host`.
- RMCP MUST enforce the effective allowed hosts on MCP requests before execution. Unlisted hosts MUST receive HTTP 403. Proxy forwarding headers MUST NOT implicitly authorize an unlisted request host; the deployment MUST preserve an allowed Host authority or explicitly rewrite it to an allowed backend authority.
- A request Host without an explicit port MUST use the HTTP default port 80, or 443 when the request URI explicitly uses HTTPS, for matching a port-restricted host entry. Proxy forwarding headers MUST NOT determine this default.
- Host permissions and browser-origin permissions MUST remain independent. Allowing an endpoint hostname MUST NOT authorize a browser origin, and allowing a browser origin MUST NOT authorize an endpoint hostname. These permissions MUST NOT enable mutation tools or replace deployment authentication.
- Requests without an `Origin` header are allowed so non-browser MCP clients can connect.
- HTTP and HTTPS origins on `localhost`, `127.0.0.1`, and `[::1]` MUST remain permitted on any port by default.
- Repeatable `--allow-origin <ORIGIN>` options MUST add browser origins to those defaults. Each value MUST be an HTTP or HTTPS origin containing a host and optional port, without credentials, path, query, fragment, wildcard, or opaque `null` origin. Invalid values MUST be rejected at startup.
- Configured origins MUST match by normalized scheme, hostname, and effective port. Omitted HTTP and HTTPS ports mean 80 and 443 respectively; specifying a host MUST NOT grant access to its other ports or subdomains.
- Requests without `Origin` MUST retain normal MCP behavior. Requests with an unpermitted, malformed, opaque, or multiple-valued Origin header MUST receive HTTP 403 before MCP execution, including preflight requests.
- Allowed browser requests MUST receive `Access-Control-Allow-Origin` equal to their request origin and origin-dependent `Vary` headers, including MCP protocol error responses.
- CORS preflight requests for permitted origins MUST succeed for POST with `Content-Type`, `Authorization`, `Mcp-Protocol-Version`, `Mcp-Session-Id`, and `Last-Event-ID` request headers. Responses MUST expose `Mcp-Session-Id` and `Mcp-Protocol-Version` to browser clients. The stateless endpoint's existing GET and DELETE behavior MUST remain unchanged.
- The shared MCP origin policy MUST govern request admission and CORS responses on both standalone and embedded endpoints. CORS handling MUST apply only to `/mcp`, preserving Explorer route behavior.
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
- HTTP transport startup options MUST include repeatable additional browser origins.
- HTTP transport startup options MUST include repeatable accepted endpoint hostnames.
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
- Existing `model_fingerprint` and workspace `model.fingerprint` values use the bound Model Revision Hash Specification through shared core computation. Changing from 16 to 64 hexadecimal characters preserves field names and response shapes; schemas and examples accept the new format. This migration does not add per-element fingerprint fields or redefine Explorer manifest revisions.
- Results indicate dirty/clean workspace state when that affects interpretation.
- Results expose evidence references to the files, elements, relations, contract_bindings, reports, or diffs used to produce the result when those concepts are relevant.
- Element-shaped results expose stable element identity, element type, source location, and requested relation/contract_bindings/content views when those concepts are relevant.
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
- `workspace`: effective workspace root plus eligible Git worktree revision and dirty-state metadata when available.
- `reqvire_version`: Reqvire binary version.
- `mcp_protocol_revision`: negotiated MCP protocol revision.
- `reqvire_tool_contract_version`: Reqvire MCP tool contract version.
- `model_revision`: model fingerprint or revision identifier.
- `evidence`: files, elements, relations, contract_bindings, or reports used to produce the result.
- `warnings`: non-fatal diagnostics.

Context-bound results additionally identify `worktree_id`. Server-wide tool contract and worktree inventory results identify the server/repository scope.

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
- `reqvire.semantic.shapes`
- `reqvire.semantic.concepts`
- `reqvire.semantic.model`
- `reqvire.semantic.graph`
- `reqvire.semantic.export`
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
- `reqvire.move_folder`
- `reqvire.link`
- `reqvire.unlink`
- `reqvire.relink`
- `reqvire.move_asset`
- `reqvire.remove_asset`

Worktree management tools:
- `reqvire.worktree.list` (read-only inventory)
- `reqvire.worktree.create`
- `reqvire.worktree.open`
- `reqvire.worktree.remove`

Commit and optional publication tools:
- `reqvire.git.commit` (local, mutation mode)
- `reqvire.git.push` (GitHub enabled and available)
- `reqvire.github.pr.create` (GitHub enabled and available)
- `reqvire.github.pr.comment` (GitHub enabled and available)

Explorer worktree selection is a browser action, not an MCP tool.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Shared Operation Interfaces](Tools.md#mcp-shared-operation-interfaces)
---

### MCP Tool Exposure Scope Specification

The MCP server is expected to expose only specified Reqvire model and repository workflow operations as MCP tools.

#### Details
Exposure rules:
- Do not expose a generic shell, `reqvire.command`, generic Git/gh passthrough, arbitrary GitHub API, or credential-management tool.
- Do not expose hidden/internal commands such as `shell` or `sout`.
- Do not expose `reqvire mcp` as an MCP tool because it starts the server.
- Do not expose `reqvire serve` as an MCP tool because it starts an HTTP Explorer server.
- Do not expose `reqvire validate` as an MCP tool because successful validation is a server startup prerequisite.
- Expose Reqvire workflow prompts through standard MCP prompt methods rather than as MCP tools.
- CLI flags, modes, and sub-options become typed request fields on one stable MCP operation instead of nested MCP tool names.
- Worktree, commit, push, PR creation, and PR commenting are distinct typed operations, not CLI aliases. Their availability follows local mutation and GitHub startup gating.
- CLI-only transport flags such as `--json` and `--output` are never MCP tool arguments.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Tool Exposure Scope](Tools.md#mcp-tool-exposure-scope)
---

### MCP Tool Side Effect Classification Specification

Every MCP tool is expected to declare its side-effect class and availability.

#### Details
Canonical MCP side-effect classes are defined by the MCP tool contract requirements and their operation specifications.

Classification rules:
- `read_only` tools are advertised in default `tools/list`.
- `read_only` tools use MCP annotations `readOnlyHint: true`, `destructiveHint: false`, and `openWorldHint: false`.
- `conditional_mutation` tools are advertised in default `tools/list` only when their default/allowed default-mode arguments are read-only.
- `conditional_mutation` tools reject or omit mutating arguments unless mutation mode is enabled.
- `mutation` tools are omitted from default `tools/list`.
- Local `mutation` tools require mutation mode and use MCP annotations `readOnlyHint: false`, `openWorldHint: false`, and conservative `destructiveHint`. Publication tools additionally require successful GitHub enablement and use `openWorldHint: true`; annotation hints do not replace runtime gates.

Read-only tools:
- `reqvire.worktree.list`
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
- `reqvire.semantic.shapes`
- `reqvire.semantic.concepts`
- `reqvire.semantic.model`
- `reqvire.semantic.graph`
- `reqvire.semantic.export`
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

Local mutation tools:
- `reqvire.worktree.create`
- `reqvire.worktree.open`
- `reqvire.worktree.remove`
- `reqvire.git.commit`
- `reqvire.add_element`
- `reqvire.remove_element`
- `reqvire.move_element`
- `reqvire.rename_element`
- `reqvire.merge_elements`
- `reqvire.move_file`
- `reqvire.move_folder`
- `reqvire.link`
- `reqvire.unlink`
- `reqvire.relink`
- `reqvire.move_asset`
- `reqvire.remove_asset`

Optional publication mutation tools:
- `reqvire.git.push`
- `reqvire.github.pr.create`
- `reqvire.github.pr.comment`

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Tool Side Effect Classification](Tools.md#mcp-tool-side-effect-classification)
---

### MCP Workspace Session Tools Specification

The MCP interface is expected to expose workspace/session tools that have no direct CLI command equivalent.

#### Details
Required workspace/session tools:
- `reqvire.workspace_status`: reports effective workspace root, eligible Git worktree roots, eligible Git `HEAD` values, dirty state when available, Reqvire version, supported MCP protocol revision, Reqvire tool contract version, and last diagnostics summary.
- `reqvire.tool_contract`: reports supported tool names, request schemas, result schemas, versions, and Reqvire capability flags for the current startup mode.
- `reqvire.model_revision`: reports the parsed-element fingerprint defined by the bound Model Revision Hash Specification, source file metadata, excluded-pattern metadata, and cache freshness. Cache freshness is determined separately from the parsed-element fingerprint.
- `reqvire.model_revision.model_fingerprint`, `reqvire.workspace_status.model.fingerprint`, and existing semantic-tool `model_fingerprint` fields use the same shared model revision computation for the same snapshot.
- Preserve existing field names and output shapes while migrating digest values from 16 to 64 lowercase hexadecimal characters. Clients MUST discard cached revisions when upgrading to canonical encoding `reqvire.model-revision.v2`, including v1 SHA-256 revisions; v2 includes the distinct Contract References collection. There is no old-to-new value mapping. The bound Model Revision Hash Specification defines the newly covered authored metadata and exclusions.
- The revision identifies the defined parsed-element projection. Page frontmatter, external referenced-file contents, Git state, and other excluded inputs may change without changing it; source cache freshness continues to follow the MCP Server State and Cache Specification.
- This migration adds no per-element hash field, command, or argument. Explorer's manifest-derived revision keeps its existing wire-byte contract.

Workspace status and model revision are context-bound. Status MUST distinguish accepted pending changes from observed external dirty state and include commit policy and GitHub availability/diagnostics. Tool contract discovery is server-wide.

These tools are read-only and must not mutate the model.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Workspace Session Tools](Tools.md#mcp-workspace-session-tools)
---

### Serve Command Embedded MCP Endpoint Specification

The Explorer serve command is expected to optionally mount the same Reqvire MCP Streamable HTTP service on the Explorer HTTP listener.

#### Details
Embedded MCP behavior:
- `reqvire serve` starts the Explorer HTTP server only and does not expose MCP by default.
- `reqvire serve --enable-mcp` mounts the Reqvire MCP Streamable HTTP service at `/mcp` on the same host and port as the Explorer server.
- `reqvire serve --enable-mcp --enable-mutations` enables MCP mutation tools for the embedded `/mcp` endpoint.
- `--enable-mutations` requires `--enable-mcp`; mutation tools are not advertised or executable for embedded MCP unless both capabilities are present.
- Embedded MCP automatic commits default to false. `reqvire serve --enable-mcp --enable-mutations --enable-commits` enables them with the same semantics as standalone MCP. `--enable-commits` requires `--enable-mutations`, which requires `--enable-mcp`; invalid combinations MUST fail argument validation before model loading or opening a listener. Root and serve command help MUST document this opt-in flag.
- `reqvire serve --enable-mcp --allow-origin <ORIGIN>` MUST configure the embedded endpoint using the MCP Streamable HTTP Transport Safety Specification bound by its owning requirement. `--allow-origin` MUST require `--enable-mcp` and support the same repeated values and validation as standalone MCP startup.
- `reqvire serve --enable-mcp --allow-host <HOST[:PORT]>` MUST configure embedded MCP endpoint host validation through the same bound contract, including automatic permission for a non-wildcard bind host. `--allow-host` MUST require `--enable-mcp`.
- `reqvire serve --enable-mcp --enable-mutations --enable-github` enables the same optional publishing group and startup checks as standalone MCP. `--github-remote` has the same default and prerequisite. Help MUST distinguish these flags from `--enable-commits`.
- The embedded `/mcp` endpoint reuses the same MCP adapter, shared Reqvire tool registry, RMCP Streamable HTTP transport configuration, allowed-origin policy, stateless JSON response mode, and mutation serialization behavior as `reqvire mcp`.
- The embedded MCP endpoint registers the current serve workspace as its original context. Additional contexts retain their own workspace roots and exclusion configurations. Browser worktree selection does not change MCP request routing.
- Successful embedded MCP mutations immediately rebuild and atomically publish the affected context's served Explorer runtime snapshot, complete manifest, immutable chunks, and revision through the post-write hook. Explorer live refresh adopts that snapshot without restarting the server. Browser manifest polls and chunk requests read the published in-memory snapshot without scanning or rebuilding the model or taking the MCP workspace write gate under the bound Served Explorer Runtime Freshness and Explorer Live Store Refresh contracts.
- Explorer runtime data rebuilds and embedded MCP mutation execution share that context's workspace lock so runtime data generation does not read partially written model files.
- Runtime data responses for `assets/project-store.js` and `ontologies.ttl` use no-store cache control so clients do not reuse stale generated datastores after mutation.
- The Explorer SPA fallback must not handle `/mcp` requests when embedded MCP is enabled.
- Embedded MCP startup is a server startup concern; it must not expose `reqvire.serve` or `reqvire.mcp` as MCP tools.
- Non-local exposure of an embedded `/mcp` endpoint requires an explicit deployment-layer authentication and authorization decision; enabling embedded MCP only starts the in-process endpoint.

#### Metadata
  * type: specification

#### Relations
  * define: [Serve Command Embedded MCP Endpoint](../WebExplorer/Capabilities.md#serve-command-embedded-mcp-endpoint)
---

### MCP Managed Query Artifacts Specification

The MCP interface MUST expose managed query operations through shared core contracts.

#### Details
Query discovery MUST return native authored records sorted by generated IRI then name. Namespace filters MUST match used ontology namespaces. Query validation MUST return per-candidate diagnostics and use the common validation gate. Selectors MUST resolve exactly and reject unknown or ambiguous results. Exported content and hashes MUST come from the shared core renderer. Validation and artifact rendering MUST preserve downstream SERVICE, datasets, and extension functions without executing them.
Tools MUST be `reqvire.semantic.queries` with optional `iri`, `name`, `namespace_base`, and `include_content`, and `reqvire.semantic.queries.validate` with optional `iri` or `name`. `include_content` MUST add content and SHA-256. `reqvire.semantic.export` MUST support the queries layer, including it for omitted or empty layer selection.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Managed Query Artifacts](ManagedQueries.md#mcp-managed-query-artifacts)
---

### MCP Worktree Context Isolation Specification

The shared context boundary separates model identity, accepted state, and operation effects between worktrees. MCP workers and the Explorer runtime implement this boundary at their respective interfaces.

#### Details
- A context identifies one admitted Git worktree model using `worktree_id`, canonical workspace root, branch, and lifecycle state. Its accepted model revision, expected HEAD, ownership, commit policy, pending accepted changes, and diagnostics remain associated with that context.
- Consumers MUST resolve explicit context selectors before accessing model state. Unknown, removed, stopped, or unavailable contexts MUST fail without substituting another context. Omitted-selector behavior belongs to the consuming interface contract; there is no server-global active branch.
- Model-bound responses MUST identify their selected context and captured revision. A response MUST represent one completed snapshot; equal relative element identifiers or parsed-model fingerprints do not establish equal context identity.
- Model caches, derived semantic state, navigation/runtime data, pending changes, and publication cursors MUST retain their context boundary. Operation effects and publication MUST update only their intended context. Request cancellation, late responses, and failures MUST NOT redirect results to another context.
- Each model operation retains the ordinary workspace eligibility boundary, Reqvire core semantics, and branch ownership guarantees. Context management does not grant cross-worktree filesystem access to model or asset operations.
- Context mutations MUST serialize against that context's accepted state, while independent contexts may progress separately. Context failure MUST be reported explicitly and MUST NOT corrupt or replace another context's accepted model.
- MCP protocol routing and worker lifecycle are implemented by the child worker-session requirement. Explorer consumers bound to this contract implement the corresponding runtime boundary; the shared owner is fulfilled only when both its worker child and required binding consumers are implemented.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Worktree Context Isolation](Tools.md#mcp-worktree-context-isolation)
---

### MCP Worktree Creation Specification

Contract for mcp worktree creation.

#### Details
- `reqvire.worktree.create` MUST require `branch` and `base_ref`; an optional `from_worktree_id` scopes resolution of a context-relative base such as `HEAD`. With multiple contexts, context-relative resolution without that ID MUST fail. Named branches such as `main` and remote-tracking refs such as `origin/main` are explicit bases in the same repository.
- Resolve the base to one commit before creation and return `base_commit`, branch, root, `worktree_id`, and accepted revision. A remote-tracking ref means the locally recorded commit; creation MUST NOT fetch, push, merge, or promise remote freshness.
- The new branch name MUST be a valid unused Git branch name. Unknown/non-commit bases, a dirty or unavailable selected source context, duplicate branches, detached targets, foreign repositories, and conflicting ownership MUST fail before publishing a context. Pending accepted changes and external changes are not copied to the new branch; the client commits or otherwise resolves them first.
- Allocate the destination under server-managed storage, outside every context's eligible model inventory; callers MUST NOT supply arbitrary filesystem destinations. Canonical paths and symlinks MUST NOT permit overlap or scanning another context's model files.
- Create the branch and worktree without checking out another branch in any existing worktree. Start its worker, acquire ownership, and validate its model under the ordinary startup rules before making the context routable.
- On failure, release newly acquired ownership and compensate only branch/worktree assets created by this request when still unchanged and safe to remove. Never delete pre-existing work or externally modified partial assets; report residual paths/refs for recovery. An invalid or incomplete context MUST NOT be returned as usable.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Worktree Creation](Tools.md#mcp-worktree-creation)
---

### MCP Worktree Opening and Inventory Specification

Contract for mcp worktree opening and inventory.

#### Details
- `reqvire.worktree.list` MUST report worktrees in the startup common Git repository, branch/HEAD, ownership, managed/original status, lifecycle state, and `worktree_id` only for registered contexts. Inventory does not acquire ownership, import external edits into accepted models, or open workers. Unregistered worktrees report unknown ownership (`owned: null`); admission establishes ownership under the existing exclusive-lock checks.
- `reqvire.worktree.open` MUST require an existing local `branch`. Return an already healthy owned context idempotently; otherwise reuse its existing eligible worktree, or create a managed worktree for that existing branch if none exists. This includes worktrees previously prepared for Explorer branch browsing. This operation MUST NOT create a new branch or force a branch into two checkouts.
- Admission requires a named committed branch, clean index/worktree including non-ignored untracked files, configured Git identity, valid model, no conflicting owner, and no overlapping context roots. These rules apply equally to pre-existing and newly created worktrees, whether requested through MCP opening or mutation-enabled Explorer selection. Read-only serving does not acquire mutation ownership or enable mutation tools. Fail without stash/reset/checkout when those conditions do not hold. An existing unregistered dirty worktree cannot be silently adopted or loaded through a read-only fallback in mutation-enabled serving.
- Preserve pre-existing worktrees on failed admission. Worktrees created by this request follow the creation rollback rules. External/pre-existing worktrees may be opened but MUST remain distinguishable from disposable server-created worktrees.
- A server restart does not implicitly reclaim all worktrees or invent new branches. It registers its clean startup context; explicit open can reclaim an existing clean branch after ownership is released. Preserve pending changes on shutdown; dirty contexts require user resolution before reopening.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Worktree Opening and Inventory](Tools.md#mcp-worktree-opening-and-inventory)
---

### MCP Managed Worktree Removal Specification

Contract for mcp managed worktree removal.

#### Details
- `reqvire.worktree.remove` MUST require `worktree_id` and is available only with mutations enabled. The original startup worktree, external/pre-existing worktrees, unknown IDs, and worktrees owned by another server MUST be rejected.
- Serialize removal with context operations, stop accepting new calls, drain in-flight work, and recheck expected HEAD and clean index/files/non-ignored untracked state. Pending accepted changes or external dirtiness MUST block removal. No force option is exposed.
- Remove the managed worktree through Git and release ownership after worker shutdown and cleanup; preserve its branch and every commit. Subsequent calls to its ID MUST fail. Never delete another context's paths or alter its model.
- On failed cleanup, report whether the context is stopped and which path remains; do not claim successful removal or silently resume writes. Explicit reopening must revalidate and reclaim ownership.
- Ordinary server shutdown only stops workers and releases ownership; it MUST NOT remove worktrees or their uncommitted changes.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Managed Worktree Removal](Tools.md#mcp-managed-worktree-removal)
---

### MCP Accepted Change Commit Specification

Contract for mcp accepted change commit.

#### Details
- `reqvire.git.commit` MUST require a non-empty `message` and use context routing. It is available with `--enable-mutations`, independent of gh availability or `--enable-github`, in both automatic-commit modes.
- Under the context mutation gate, verify ownership and expected HEAD, validate the complete accepted candidate with Reqvire core, and construct the commit from the accepted baseline plus accumulated accepted changes. Never use unrelated staged files, current external file edits, or hook-modified trees as the accepted candidate.
- Preserve unrelated index entries and working files. Reconcile index entries for accepted paths only against the accepted commit; external edits in those working files remain distinguishable as dirty state, not accepted model input. Commit scope includes accepted asset moves/removals and executable modes under existing mutation guarantees.
- Publish the ref using the expected old HEAD. Only success advances expected HEAD and clears the committed pending-change set. Return old/new HEAD, changed files, context, and model revision. A commit alone does not change parsed model content or trigger a model/Explorer rebuild.
- With no net accepted changes, return an explicit no-op and no commit. After an automatic mutation commit, manual commit therefore normally returns no-op. Model mutations without `--enable-commits` still leave HEAD unchanged until explicit commit.
- A failed validation, commit creation, or ref update MUST preserve pending accepted changes, the previous accepted model, and unrelated index/worktree state. Unexpected HEAD disables writes under the ownership contract; recovery MUST NOT reset an externally changed ref. Commit messages are data, never shell source.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Accepted Change Commit](Tools.md#mcp-accepted-change-commit)
---

### MCP GitHub Tool Availability Specification

Contract for mcp github tool availability.

#### Details
- Standalone `mcp` and embedded `serve --enable-mcp` MUST support `--enable-github`, default false, requiring `--enable-mutations`. `--enable-commits` remains independent and default false. Add optional `--github-remote <NAME>` defaulting to `origin`, requiring `--enable-github`; it names a configured remote, not an arbitrary URL. Invalid flag combinations fail before model loading or listening.
- Before advertising publication tools, resolve the selected remote to one repository/host and run bounded, noninteractive checks for gh installation/version, active host authentication, and repository access. Use `gh auth status --active --hostname <host>` exit status for authentication; JSON output alone is not proof of authentication. Repository lookup may obtain canonical identity and `viewerPermission`.
- Pin the verified host/repository and configured Git push destination for the session. Missing gh, failed authentication, missing/ambiguous remote, inaccessible repository, malformed output, or timeout MUST disable the entire publication tool group (`reqvire.git.push`, `reqvire.github.pr.create`, `reqvire.github.pr.comment`) for that session. The server MUST still start if its ordinary local prerequisites pass; commit and worktree tools remain available.
- Disabled tools MUST be absent from `tools/list` and rejected on direct `tools/call`. Report enablement, availability, selected remote/repository, and a sanitized reason in startup diagnostics, server contract status, and workspace status. Availability after failed startup checks requires restart; it does not silently change during a session.
- Startup checks MUST NOT push, create branches/PRs/comments, or request credentials interactively. `viewerPermission` is diagnostic information, not a write guarantee or a substitute for Git transport credentials. Do not remove all tools solely on that coarse permission value.
- Each actual operation remains subject to current authentication, Git credentials, scopes, repository/branch rules, and permission enforcement. Return a structured permission/authentication error from a denied operation without falsely claiming that startup guaranteed access.
- Published help, tool schemas, and capability flags MUST distinguish mutations, automatic commits, worktree contexts, GitHub requested, and GitHub available. Local tools are `read_only` or `mutation` with `openWorldHint: false`; publication tools are `mutation`, `readOnlyHint: false`, `openWorldHint: true`, with conservative destructive/idempotency annotations.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP GitHub Tool Availability](Tools.md#mcp-github-tool-availability)
---

### MCP Publication Scope and Recovery Specification

Contract for mcp publication scope and recovery.

#### Details
- GitHub publication is limited to one same-repository workflow in the startup common Git repository. The configured fetch/push URLs must resolve to the pinned GitHub host and repository; SSH/HTTPS representations of the same identity are allowed. Reject forks, different repositories, multiple push destinations, changed remote identity/configuration, and caller-supplied repository URLs. Local worktree/commit tools do not require a GitHub remote.
- Git/gh commands MUST use typed operation arguments and fixed executables without a shell, passthrough arguments, login/token-management tools, arbitrary API requests, or generic command execution. Multiline messages and bodies are passed as data. Bound execution time and disable interactive prompts. Diagnostics MUST omit credentials and tokens.
- Before each operation, verify selected ownership, branch, and expected HEAD under its context gate. Publication MUST NOT import external disk edits or rewrite local history. Push/PR creation require no pending accepted changes and a clean selected worktree/index; PR comments do not require committing unrelated pending model work.
- Return a structured operation result including context, branch, repository/remote where relevant, attempted commit where relevant, outcome (`completed`, `no_op`, `failed`, or `unknown`), diagnostics, and PR/comment identifiers and URLs when known. A tool failure MUST use the existing MCP error contract, retaining this recovery context.
- Network, permission, validation, subprocess, or remote failures MUST leave accepted models, local commits, and pending changes intact. Report non-fast-forward and branch-rule rejections without automatic rebase, merge, force push, or reset. Explicit commit success is not undone when a later push or PR operation fails.
- On timeout/disconnection after a side effect may have occurred, reconcile remote state using read-only queries where possible. Otherwise report `unknown`; never claim nothing happened or blindly retry a comment/PR creation. Report the target and available evidence so the client can resolve uncertainty.
- Worktree switching, committing, pushing, PR creation, and commenting remain separate explicit calls. No merge, branch deletion, force push, or fork operation is exposed by this capability.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Publication Scope and Recovery](Tools.md#mcp-publication-scope-and-recovery)
---

### MCP Branch Push Specification

Contract for mcp branch push.

#### Details
- `reqvire.git.push` uses `worktree_id` and an optional `remote` name that MUST match the startup-pinned remote; omission selects that remote. Source and destination refs are derived from the owned branch, not arbitrary caller refspecs.
- Verify the selected context is clean with no pending accepted changes and HEAD equals its last accepted commit. Push that exact commit to `refs/heads/<owned-branch>` with no force, mirror, tag, deletion, or alternate destination options.
- Return attempted commit, remote branch, and confirmed remote state. Set upstream tracking only for that owned branch after successful publication. An already matching remote branch is a successful no-op; divergence is a structured rejection requiring client resolution.
- Serialize same-context mutation/commit with the push so a moving local HEAD cannot change the submitted commit. Other worktree contexts remain independent. Push MUST NOT create a PR or comment.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Branch Push](Tools.md#mcp-branch-push)
---

### MCP Pull Request Creation Specification

Contract for mcp pull request creation.

#### Details
- `reqvire.github.pr.create` MUST require `base`, non-empty `title`, and `body`; `draft` is an optional boolean defaulting to false. The head is the selected owned branch. No implicit default to `main` or another base is allowed.
- Require the head branch to be pushed at the accepted local HEAD and require the base to exist in the same remote repository, differ from the head, and yield a valid proposed comparison. A base may be another feature/PR branch, supporting stacked PRs without cross-repository heads.
- Use explicit repository, base, and head with gh. Missing publication MUST fail with a request to push first, never trigger gh's implicit push, fork, branch creation, or commit behavior.
- Before creation, look up an existing open PR for the exact repository/head/base. Return its number and URL as `no_op` rather than duplicating it; do not silently edit its title/body/draft state. Multiple ambiguous matches or an existing open head PR against a different base produce a conflict identifying the known PR.
- Return repository, head/base, submitted commit, number, URL, and draft state for a confirmed result. Follow publication recovery rules if creation succeeds remotely but the response is lost.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Pull Request Creation](Tools.md#mcp-pull-request-creation)
---

### MCP Pull Request Commenting Specification

Contract for mcp pull request commenting.

#### Details
- `reqvire.github.pr.comment` MUST require a positive `pr_number` and non-empty `body`, with ordinary context routing. Resolve the PR by number within the pinned repository; do not accept arbitrary PR URLs or repository selectors.
- Commenting may target a review PR in that repository even when its head differs from the selected context branch. Return the actual target PR and originating context so the caller can distinguish them. It does not push, create a PR, commit pending work, or edit/delete an existing comment.
- Preserve multiline body content exactly. Return PR number/URL and comment ID/URL only after confirmation. Permission and missing-PR failures must be explicit.
- A caller-issued repeat is a new comment request, not a guaranteed idempotent operation. On an ambiguous result, report/reconcile under the publication recovery contract; never automatically resend merely because the first response was lost.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Pull Request Commenting](Tools.md#mcp-pull-request-commenting)
---

### MCP Worktree Worker Sessions Specification

The MCP worker adapter implements the shared worktree context boundary for protocol requests and model operations.

#### Details
- Each context MUST expose an opaque `worktree_id`, canonical workspace root, branch, expected HEAD, model revision, ownership/lifecycle state, automatic-commit setting, and accepted-uncommitted-change status. IDs identify contexts within the server session; they are not model element identifiers and are not assumed stable after restart.
- Workspace-bound tools, including model reads, semantic queries, mutations, status, commits, and publication, MUST accept `worktree_id`. Omission is allowed only when exactly one context is registered. With multiple contexts, omission MUST fail with the available IDs; unknown, removed, or unavailable IDs MUST fail without fallback. Client selection MUST NOT mutate server-global active-branch state.
- `reqvire.tool_contract` and `reqvire.worktree.list` are server-scoped. Worktree creation/opening use explicit repository branch/base selectors. Every context-bound response and error MUST identify the selected context when resolution succeeded; successful model evidence also identifies its captured model revision.
- Resources and model-bearing prompts MUST route identically. Context-qualified resource URIs MUST carry `worktree_id` in a query parameter; legacy unqualified URIs work only with a single context. Prompt arguments and resource templates MUST advertise the selector. Resource caches and subscriptions MUST distinguish context identity even when relative identifiers or model fingerprints coincide.
- Each context MUST use an isolated worker process with a fixed workspace root and reuse Reqvire core parsing, validation, reports, mutations, and semantic stores. The manager MUST NOT switch process-global working directories or construct a parallel model engine. Existing Workspace Scope semantics apply inside each worker, not to a union of managed worktrees.
- Every mutation-enabled worker MUST satisfy the existing clean-start and branch/worktree ownership contract. The startup worktree is registered as the original context. Commit policy is inherited from startup flags and fixed for all workers in that server session.
- Accepted model/captured inputs, expected HEAD, pending accepted changes, mutation gate, exclusions, diagnostics, and lazy semantic stores MUST be isolated per context. Interleaved requests may run across contexts; mutations, explicit commits, and publication of one context MUST serialize against its own accepted state. Brief repository-administration locking MUST protect shared branch/worktree metadata without replacing per-context ownership.
- Worker exit or unrecoverable mutation failure MUST mark only its context unavailable and prevent new writes there. Reads MUST NOT silently substitute another context or an older worker snapshot. Opening it again applies clean-start validation and ownership checks.
- In embedded mode, each available context has a separate Explorer runtime. Explorer selects a context per browser tab; mutations refresh only their own context's runtime. Browser selection MUST NOT alter any MCP request selector or global active branch. No MCP Explorer-switching tool is exposed.
- Read-only MCP retains its current workspace loading/freshness rules; these tools do not automatically open new writable contexts or change its treatment of external edits.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Worktree Worker Sessions](Tools.md#mcp-worktree-worker-sessions)
---
