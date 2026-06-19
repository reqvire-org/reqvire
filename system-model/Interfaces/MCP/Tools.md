# Elements

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
  * definedBy: [MCP Access Control Baseline Specification](Specifications.md#mcp-access-control-baseline-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * verifiedBy: [MCP Access Control Baseline Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-access-control-baseline-verification)
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
  * definedBy: [MCP Contract Layer Boundary Specification](Specifications.md#mcp-contract-layer-boundary-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * satisfiedBy: [tool_interface.rs](../../../core/src/tool_interface.rs)
  * verifiedBy: [MCP Contract Layer Boundary Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-contract-layer-boundary-verification)
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
  * definedBy: [MCP Contract Versioning Specification](Specifications.md#mcp-contract-versioning-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * verifiedBy: [MCP Contract Versioning Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-contract-versioning-verification)
---

### MCP Model Evidence Tools

The system shall expose MCP read tools that return model evidence needed by external tools and AI agents.

#### Details
- The MCP interface shall expose read tools for authoritative Reqvire model evidence.
- Model evidence tools shall support element lookup, model structure, containment, collection, submodel analysis, and ontology/SHACL semantic collection.
- The ontology/SHACL semantic collection tool shall be named under the `reqvire.semantic` namespace and shall filter serialized content to RDF only, SHACL only, or both.
- The ontology/SHACL semantic collection tool shall support optional used external ontology subset materialization through a typed `include_external` argument without changing the default authored-only export.
- Semantic prefix and vocabulary tools shall keep imported external ontology declarations hidden by default and expose only used external subset entries through a typed `include_external` argument with explicit external markers and source metadata.
- Model evidence tools shall support read-only semantic query execution over collected ontology, SHACL, model-context, and ontology projection RDF when requested by a typed MCP operation, and shall support an explicit `include_external` argument for querying the graph that includes the used external subset.
- Model evidence tools shall expose the canonical capability/requirement/ontology model, including `ontology` elements, `#### Concept References`, reusable `semantic-contract` shape profiles, constrained requirements, and ontology-use relations where the underlying Reqvire operation returns them.
- Model evidence tools shall include revision metadata when model state affects interpretation.
- Model evidence tools shall not mutate the model or filesystem.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Requirement Governance Metadata Specification](../../ModelStructure/Specifications.md#requirement-governance-metadata-specification)
  * [Flexible Search Type Filtering Contract Specification](../../Reports/ModelReports/Specifications.md#flexible-search-type-filtering-contract-specification)
  * [Containment View Report Contract Specification](../../Reports/ModelReports/Specifications.md#containment-view-report-contract-specification)
  * [Collect Content Specification](../../Reports/ModelReports/Specifications.md#collect-content-specification)
  * [Requirement Submodels Report Specification](../../Reports/ModelReports/Specifications.md#requirement-submodels-report-specification)
  * [Ontology Collection Output Specification](../../Reports/ModelReports/Specifications.md#ontology-collection-output-specification)
  * [Local External Ontology Source Specification](../../Reports/ModelReports/Specifications.md#local-external-ontology-source-specification)

#### Relations
  * definedBy: [MCP Model Evidence Tools Specification](Specifications.md#mcp-model-evidence-tools-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * verifiedBy: [MCP Model Evidence Tools Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-model-evidence-tools-verification)
---

### MCP Semantic Query Tools

The system shall expose an MCP read tool for SPARQL queries over Reqvire semantic RDF evidence.

#### Details
- The MCP interface shall execute SPARQL queries against Reqvire's collected semantic RDF graph.
- The semantic query tool shall use the existing in-memory semantic export and Oxigraph query engine.
- The semantic query tool shall support authored ontology and SHACL RDF, and shall include generated Reqvire model-context, relation-family projection, and ontology projection facts by default when they are present in the full semantic export.
- The semantic query tool shall return structured results for SELECT, ASK, CONSTRUCT, and DESCRIBE queries.
- The semantic query interface shall provide ontology-defined prefix discovery so clients can construct namespace-correct SPARQL without rebuilding the RDF store.
- The semantic query tool shall not mutate the model or filesystem.
- The semantic query tool shall not expose arbitrary shell execution, arbitrary filesystem reads, or remote URL fetching.

#### Concept References
  * MCP semantic query contract: https://www.reqvire.org/ontology#McpSemanticQueryContract
  * MCP SPARQL query tool contract: https://www.reqvire.org/ontology#McpSparqlQueryToolContract

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Semantic Relation Family Projection Specification](../../Reports/ModelReports/Specifications.md#semantic-relation-family-projection-specification)

#### Relations
  * definedBy: [MCP Semantic Query Tools Specification](Specifications.md#mcp-semantic-query-tools-specification)
  * derivedFrom: [MCP Model Evidence Tools](#mcp-model-evidence-tools)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * satisfiedBy: [tool_interface.rs](../../../core/src/tool_interface.rs)
  * verifiedBy: [MCP Semantic Query Tools Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-semantic-query-tools-verification)
---

### MCP Prompt Guidance

The system shall expose MCP prompts that guide regular Reqvire workflows and semantic query construction.

#### Details
- The MCP interface shall advertise the standard MCP prompts capability.
- The MCP interface shall support `prompts/list` and `prompts/get` for Reqvire-authored prompt templates.
- Prompt templates shall include regular Reqvire model exploration, change planning, and verification coverage review workflows.
- Prompt templates shall include semantic query, semantic verification search, and semantic contract-context search workflows.
- Semantic prompt templates shall direct clients to use `reqvire.semantic.vocabulary`, `reqvire.semantic.prefixes`, and `reqvire.semantic.sparql` for ontology-aware questions, and shall state that `include_external` exposes only the used external subset rather than raw external dependency files.
- Prompt templates shall be imported into Rust at build time and shall not be read from workspace source files at runtime.
- Prompt retrieval shall not mutate the model or filesystem.

#### Concept References
  * MCP prompt contract: https://www.reqvire.org/ontology#McpPromptContract
  * MCP semantic query prompt contract: https://www.reqvire.org/ontology#McpSemanticQueryPromptContract
  * MCP regular workflow prompt contract: https://www.reqvire.org/ontology#McpWorkflowPromptContract

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [MCP Prompt Guidance Specification](Specifications.md#mcp-prompt-guidance-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * derivedFrom: [MCP Semantic Query Tools](#mcp-semantic-query-tools)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * satisfiedBy: [mcp_prompts.rs](../../../core/src/mcp_prompts.rs)
  * verifiedBy: [MCP Prompt Guidance Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-prompt-guidance-verification)
---

### MCP Semantic Prefix Registry Tools

The system shall expose an MCP read tool that lists ontology-defined prefixes and namespaces with source element context.

#### Details
- The MCP interface shall expose prefixes from ontology element metadata, not from ad hoc Turtle prefix scraping.
- The prefix registry tool shall return prefix, namespace, ontology_base, term_namespace, ontology_document_iri, source element provenance, and source element prose content.
- The source content shall describe the ontology element and shall not include the authored Turtle block.
- The prefix registry tool shall return a SPARQL prefix block suitable for client query construction.
- The prefix registry tool shall not mutate the model or filesystem.
- The prefix registry tool shall not rebuild or reload the semantic store to answer prefix discovery.

#### Concept References
  * MCP semantic prefix registry contract: https://www.reqvire.org/ontology#McpSemanticPrefixRegistryContract
  * MCP semantic prefix tool contract: https://www.reqvire.org/ontology#McpSemanticPrefixToolContract

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [MCP Semantic Prefix Registry Tools Specification](Specifications.md#mcp-semantic-prefix-registry-tools-specification)
  * derivedFrom: [MCP Semantic Query Tools](#mcp-semantic-query-tools)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * satisfiedBy: [tool_interface.rs](../../../core/src/tool_interface.rs)
  * verifiedBy: [MCP Semantic Prefix Registry Tools Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-semantic-prefix-registry-tools-verification)
---

### MCP Semantic Relation Family Projection Access

The system shall make semantic-export relation-family projection facts available through MCP semantic query and vocabulary tools.

#### Details
- MCP shall expose normalized relation-family vocabulary and query examples sourced from the ontology/semantic export contract.
- MCP shall query relation-family projection facts from the existing full semantic graph; MCP does not own relation-family materialization.
- MCP shall not rebuild relation-family facts, execute projection-side construct materialization, mutate model source files, or write generated triples back to Markdown.

#### Concept References
  * Relation family construct query: https://www.reqvire.org/ontology#RelationFamilyConstructQuery
  * Model relation: https://www.reqvire.org/ontology#ModelRelation

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [MCP Semantic Relation Family Projection Access Specification](Specifications.md#mcp-semantic-relation-family-projection-access-specification)
  * derivedFrom: [MCP Semantic Query Tools](#mcp-semantic-query-tools)
  * satisfiedBy: [tool_interface.rs](../../../core/src/tool_interface.rs)
  * verifiedBy: [MCP Semantic Query Tools Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-semantic-query-tools-verification)
---

### MCP Semantic Vocabulary Tools

The system shall expose an MCP read tool that pages compact semantic vocabulary for SPARQL query construction.

#### Details
- The MCP interface shall expose semantic vocabulary from the parsed semantic model index.
- The vocabulary tool shall include ontology-defined prefixes and a SPARQL prefix block in every response.
- The vocabulary tool shall support section paging for prefixes, classes, properties, relation families, controlled vocabularies, semantic contracts, query patterns, source map entries, and diagnostics.
- The vocabulary tool shall expose relation families with normalized forward and inverse properties so clients can query semantic relation meaning instead of hard-coding raw relation tokens.
- The vocabulary tool shall not mutate the model or filesystem.
- The vocabulary tool shall not rebuild or reload the semantic store to answer vocabulary discovery.

#### Concept References
  * MCP semantic vocabulary contract: https://www.reqvire.org/ontology#McpSemanticVocabularyContract
  * MCP semantic vocabulary tool contract: https://www.reqvire.org/ontology#McpSemanticVocabularyToolContract
  * Relation Family: https://www.reqvire.org/ontology#RelationFamily

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [MCP Semantic Vocabulary Tools Specification](Specifications.md#mcp-semantic-vocabulary-tools-specification)
  * derivedFrom: [MCP Semantic Query Tools](#mcp-semantic-query-tools)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * satisfiedBy: [tool_interface.rs](../../../core/src/tool_interface.rs)
  * verifiedBy: [MCP Semantic Vocabulary Tools Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-semantic-vocabulary-tools-verification)
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
  * definedBy: [MCP Mutation Concurrency Control Specification](Specifications.md#mcp-mutation-concurrency-control-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * verifiedBy: [MCP HTTP Transport End-to-End Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-http-transport-end-to-end-verification)
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
  * definedBy: [MCP Mutation Execution Flow Specification](Specifications.md#mcp-mutation-execution-flow-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [format.rs](../../../core/src/format.rs)
  * verifiedBy: [MCP Mutation Execution Flow Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-mutation-execution-flow-verification)
---

### MCP Mutation Tool Safety

The system shall expose mutation tools only through typed Reqvire core operations that preserve operation-specific preview behavior, validation, persistence guarantees, and post-mutation diagnostics.

#### Details
- The MCP interface shall expose mutation tools only after explicit mutation enablement.
- MCP mutation tools shall use Reqvire core mutation logic.
- MCP mutation tools shall preserve Reqvire semantic model validation, including reused_contract_context compatibility, semantic-contract SHACL reference reachability, concept-reference resolution, and single ontology-root validation.
- MCP mutation tools shall preserve Reqvire filesystem persistence behavior.
- MCP mutation results shall report changed model evidence.
- MCP mutation execution shall refresh MCP-visible model state after successful mutation.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [File Persistence Behavior](../../ModelStructure/Behaviors.md#file-persistence-behavior)
  * [Create Element Workflow Specification](../../Operations/ModelOperations/Specifications.md#create-element-workflow-specification)
  * [Delete Element Workflow Specification](../../Operations/ModelOperations/Specifications.md#delete-element-workflow-specification)
  * [Move Element Workflow Specification](../../Operations/ModelOperations/Specifications.md#move-element-workflow-specification)
  * [Rename Element Operation Contract Specification](../../Operations/ModelOperations/Specifications.md#rename-element-operation-contract-specification)
  * [Merge Element Workflow Specification](../../Operations/ModelOperations/Specifications.md#merge-element-workflow-specification)
  * [Move File Operation Contract Specification](../../Operations/ModelOperations/Specifications.md#move-file-operation-contract-specification)
  * [Relation Operations Specification](../../ModelStructure/Specifications.md#relation-operations-specification)
  * [Atomic Relation Relink Workflow Specification](../../Operations/ModelOperations/Specifications.md#atomic-relation-relink-workflow-specification)
  * [Relation Consistency Maintenance Contract Specification](../../Operations/ModelOperations/Specifications.md#relation-consistency-maintenance-contract-specification)

#### Relations
  * definedBy: [MCP Mutation Tool Safety Specification](Specifications.md#mcp-mutation-tool-safety-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * satisfiedBy: [crud.rs](../../../core/src/crud.rs)
  * satisfiedBy: [format.rs](../../../core/src/format.rs)
  * verifiedBy: [MCP Mutation Tool Safety Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-mutation-tool-safety-verification)
---

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
  * constrainedBy: [MCP Tool Side-Effect Shape](../../Ontologies/Interfaces.md#mcp-tool-side-effect-shape)
  * definedBy: [MCP Protocol Standard Conformance Specification](Specifications.md#mcp-protocol-standard-conformance-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * verifiedBy: [MCP Protocol Standard Conformance Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-protocol-standard-conformance-verification)
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

#### Reused Contract Context
  * [Lint Output Specification](../../Operations/Linting/Specifications.md#lint-output-specification)
  * [Requirement Implementation Coverage Logic Specification](../../Reports/ModelReports/Specifications.md#requirement-implementation-coverage-logic-specification)
  * [Verification Trace Tree Construction](../../Verification/Traceability/Specifications.md#verification-trace-tree-construction)
  * [Resources Report Format Specification](../../Reports/ModelReports/Specifications.md#resources-report-format-specification)
  * [Impact Scope Computation Specification](../../Processing/ChangeImpact/Specifications.md#impact-scope-computation-specification)

#### Relations
  * definedBy: [MCP Quality Traceability Tools Specification](Specifications.md#mcp-quality-traceability-tools-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * verifiedBy: [MCP Quality Traceability Tools Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-quality-traceability-tools-verification)
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

#### Reused Contract Context
  * [Resources Report Format Specification](../../Reports/ModelReports/Specifications.md#resources-report-format-specification)
  * [Containment View Report Contract Specification](../../Reports/ModelReports/Specifications.md#containment-view-report-contract-specification)
  * [Requirement Submodels Report Specification](../../Reports/ModelReports/Specifications.md#requirement-submodels-report-specification)

#### Relations
  * definedBy: [MCP Resource Interface Specification](Specifications.md#mcp-resource-interface-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * verifiedBy: [MCP Resource Interface Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-resource-interface-verification)
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
  * definedBy: [MCP Server Command Specification](Specifications.md#mcp-server-command-specification)
  * definedBy: [MCP Size Estimate Startup Specification](Specifications.md#mcp-size-estimate-startup-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [main.rs](../../../cli/src/main.rs)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * satisfiedBy: [tool_interface.rs](../../../core/src/tool_interface.rs)
  * verifiedBy: [MCP Server Command Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-server-command-verification)
  * verifiedBy: [MCP Size Estimate Startup Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-size-estimate-startup-verification)
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
  * definedBy: [MCP Server State and Cache Specification](Specifications.md#mcp-server-state-and-cache-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * verifiedBy: [MCP Server State and Cache Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-server-state-and-cache-verification)
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

#### Reused Contract Context
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [MCP Shared Operation Contracts Specification](Specifications.md#mcp-shared-operation-contracts-specification)
  * definedBy: [MCP Tool Call Contracts Specification](Specifications.md#mcp-tool-call-contracts-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * satisfiedBy: [tool_interface.rs](../../../core/src/tool_interface.rs)
  * verifiedBy: [MCP Shared Operation Contracts Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-shared-operation-contracts-verification)
  * verifiedBy: [MCP Tool Call Contracts Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-tool-call-contracts-verification)
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
  * definedBy: [MCP Streamable HTTP Transport Specification](Specifications.md#mcp-streamable-http-transport-specification)
  * derive: [Serve Command Embedded MCP Endpoint](../WebExplorer/Capabilities.md#serve-command-embedded-mcp-endpoint)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * verifiedBy: [MCP HTTP Transport End-to-End Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-http-transport-end-to-end-verification)
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
  * definedBy: [MCP Streamable HTTP Transport Safety Specification](Specifications.md#mcp-streamable-http-transport-safety-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * verifiedBy: [MCP HTTP Transport End-to-End Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-http-transport-end-to-end-verification)
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
  * definedBy: [MCP Structured Payload Contracts Specification](Specifications.md#mcp-structured-payload-contracts-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * verifiedBy: [MCP Structured Payload Contracts Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-structured-payload-contracts-verification)
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
  * definedBy: [MCP Tool Exposure Scope Specification](Specifications.md#mcp-tool-exposure-scope-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * verifiedBy: [MCP Tool Exposure Scope Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-tool-exposure-scope-verification)
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
  * definedBy: [MCP Tool Side Effect Classification Specification](Specifications.md#mcp-tool-side-effect-classification-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * verifiedBy: [MCP Tool Side Effect Classification Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-tool-side-effect-classification-verification)
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
  * definedBy: [MCP Workspace Session Tools Specification](Specifications.md#mcp-workspace-session-tools-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../cli/src/mcp.rs)
  * verifiedBy: [MCP Workspace Session Tools Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-workspace-session-tools-verification)
---
