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
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
  * verifiedBy: [MCP Access Control Baseline Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-access-control-baseline-verification)
---

### MCP Adapter Boundary

The system shall keep Reqvire MCP tool interfaces protocol-neutral below the MCP adapter.

#### Details
Detailed protocol-neutral type ownership, adapter mapping, shared registry, SDK/runtime independence, and CLI/MCP derivation rules shall follow the associated specification.

#### Metadata
  * type: requirement

#### Contract References
  * [Model Interface Operation Boundary Specification](../Specifications.md#model-interface-operation-boundary-specification)
  * [Shared Core Operation Layer Specification](../Specifications.md#shared-core-operation-layer-specification)

#### Relations
  * definedBy: [MCP Contract Layer Boundary Specification](Specifications.md#mcp-contract-layer-boundary-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
  * satisfiedBy: [mod.rs](../../../crates/reqvire-core/src/tool_interface/mod.rs)
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
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
  * verifiedBy: [MCP Contract Versioning Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-contract-versioning-verification)
---

### MCP Formatting Tool

The system SHALL expose a typed MCP formatting tool that previews without writes by default and applies deterministic formatting only when mutations are enabled.

#### Metadata
  * type: requirement

#### Contract References
  * [Diff Output Format Specification](../Specifications.md#diff-output-format-specification)
  * [Model Interface Operation Boundary Specification](../Specifications.md#model-interface-operation-boundary-specification)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [Element Ordering Behavior](../../Operations/Formatting/Behaviors.md#element-ordering-behavior)
  * [Format Duplicate Removal Behavior](../../Operations/Formatting/Behaviors.md#format-duplicate-removal-behavior)
  * [Document Structure Specification](../../Operations/Formatting/Specifications.md#document-structure-specification)
  * [Format Consistency Specification](../../Operations/Formatting/Specifications.md#format-consistency-specification)
  * [Full Relations Insertion Contract Specification](../../Operations/Formatting/Specifications.md#full-relations-insertion-contract-specification)
  * [Relation Ordering Specification](../../Operations/Formatting/Specifications.md#relation-ordering-specification)
  * [Canonical Model Mutation Persistence Specification](../../Operations/ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [MCP Formatting Tool Specification](Specifications.md#mcp-formatting-tool-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp_session.rs](../../../crates/reqvire-cli/src/mcp_session.rs)
  * satisfiedBy: [format.rs](../../../crates/reqvire-core/src/format.rs)
  * satisfiedBy: [read_tools.rs](../../../crates/reqvire-core/src/tool_interface/read_tools.rs)
  * specify: [MCP Model Maintenance](../InterfacesFeature.md#mcp-model-maintenance)
---

### MCP Lint Fix Tool

When an MCP client requests lint repair with mutations enabled, the system SHALL apply only safe auto-fixable findings through the shared validated, canonical model mutation transaction.

#### Details
When mutation mode is disabled, the system SHALL retain lint inspection and reject repair requests before side effects.

#### Metadata
  * type: requirement

#### Contract References
  * [Operation Command Contract Specification](../../ModelStructure/Specifications.md#operation-command-contract-specification)
  * [Lint Auto-fix Capability Contract Specification](../../Operations/Linting/Specifications.md#lint-auto-fix-capability-contract-specification)
  * [Lint Output Specification](../../Operations/Linting/Specifications.md#lint-output-specification)
  * [Canonical Model Mutation Persistence Specification](../../Operations/ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)

#### Relations
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp_session.rs](../../../crates/reqvire-cli/src/mcp_session.rs)
  * satisfiedBy: [lint.rs](../../../crates/reqvire-core/src/lint.rs)
  * satisfiedBy: [mod.rs](../../../crates/reqvire-core/src/operations/mod.rs)
  * satisfiedBy: [read_tools.rs](../../../crates/reqvire-core/src/tool_interface/read_tools.rs)
  * specify: [MCP Model Maintenance](../InterfacesFeature.md#mcp-model-maintenance)
---

### MCP Model Change Publication

WHILE mutation mode is enabled, the system SHALL provide explicit publication of accepted model changes to local Git history and, when GitHub integration is enabled and available, to the same remote repository.

#### Details
Automatic commits remain an independent, default-disabled option. Publication does not support forks, arbitrary Git or gh commands, merges, or history rewriting.

#### Concept References
  * [Model Change Publication](../../Thesaurus/Thesaurus.md#model-change-publication)

#### Metadata
  * type: requirement

#### Relations
  * specify: [MCP Change Publication](../InterfacesFeature.md#mcp-change-publication)
---

### MCP Accepted Change Commit

WHEN a client explicitly requests a commit in an owned context, the system SHALL commit only that context's accepted uncommitted model changes and advance its expected HEAD after successful Git publication.

#### Concept References
  * [Model Change Publication](../../Thesaurus/Thesaurus.md#model-change-publication)

#### Metadata
  * type: requirement

#### Contract References
  * [MCP Worktree Context Isolation Specification](Specifications.md#mcp-worktree-context-isolation-specification)

#### Relations
  * definedBy: [MCP Accepted Change Commit Specification](Specifications.md#mcp-accepted-change-commit-specification)
  * derivedFrom: [MCP Model Change Publication](#mcp-model-change-publication)
  * satisfiedBy: [mcp_session.rs](../../../crates/reqvire-cli/src/mcp_session.rs)
---

### MCP Branch Push

WHEN a client requests publication of a committed owned branch, the system SHALL push its accepted HEAD to the identically named branch of the pinned remote without rewriting remote history.

#### Concept References
  * [Model Change Publication](../../Thesaurus/Thesaurus.md#model-change-publication)

#### Metadata
  * type: requirement

#### Contract References
  * [MCP GitHub Tool Availability Specification](Specifications.md#mcp-github-tool-availability-specification)
  * [MCP Publication Scope and Recovery Specification](Specifications.md#mcp-publication-scope-and-recovery-specification)
  * [MCP Worktree Context Isolation Specification](Specifications.md#mcp-worktree-context-isolation-specification)

#### Relations
  * definedBy: [MCP Branch Push Specification](Specifications.md#mcp-branch-push-specification)
  * derivedFrom: [MCP Model Change Publication](#mcp-model-change-publication)
  * satisfiedBy: [mcp_github.rs](../../../crates/reqvire-cli/src/mcp_github.rs)
---

### MCP Commit Outcome Reconciliation

WHEN automatic or explicit local commit publication is interrupted in an owned context, the system SHALL verify the local Git outcome, complete index publication and accept the exact validated published candidate without repeating the ref side effect, or report confirmed non-publication with rollback for an automatic mutation and retained pending edits for an explicit commit.

#### Concept References
  * [Model Change Publication](../../Thesaurus/Thesaurus.md#model-change-publication)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [MCP Commit Outcome Reconciliation Specification](Specifications.md#mcp-commit-outcome-reconciliation-specification)
  * derivedFrom: [MCP Model Change Publication](#mcp-model-change-publication)
  * satisfiedBy: [mcp_session.rs](../../../crates/reqvire-cli/src/mcp_session.rs)
  * satisfiedBy: [mcp_worker.rs](../../../crates/reqvire-cli/src/mcp_worker.rs)
  * satisfiedBy: [mcp_worktrees.rs](../../../crates/reqvire-cli/src/mcp_worktrees.rs)
  * verifiedBy: [MCP Accepted Change Commit Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-accepted-change-commit-verification)
---

### MCP GitHub Tool Availability

WHILE GitHub integration is explicitly enabled, the system SHALL advertise publication tools only after successful noninteractive startup checks and SHALL expose their unavailability without preventing local MCP model work.

#### Concept References
  * [Model Change Publication](../../Thesaurus/Thesaurus.md#model-change-publication)

#### Metadata
  * type: requirement

#### Contract References
  * [MCP Worktree Context Isolation Specification](Specifications.md#mcp-worktree-context-isolation-specification)

#### Relations
  * definedBy: [MCP GitHub Tool Availability Specification](Specifications.md#mcp-github-tool-availability-specification)
  * derivedFrom: [MCP Model Change Publication](#mcp-model-change-publication)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * satisfiedBy: [mcp_github.rs](../../../crates/reqvire-cli/src/mcp_github.rs)
---

### MCP Publication Scope and Recovery

WHEN a repository publication operation executes, the system SHALL constrain it to the selected owned context and pinned repository and SHALL report failure or uncertain outcome without losing accepted local work.

#### Concept References
  * [Model Change Publication](../../Thesaurus/Thesaurus.md#model-change-publication)

#### Metadata
  * type: requirement

#### Contract References
  * [MCP Worktree Context Isolation Specification](Specifications.md#mcp-worktree-context-isolation-specification)

#### Relations
  * definedBy: [MCP Publication Scope and Recovery Specification](Specifications.md#mcp-publication-scope-and-recovery-specification)
  * derivedFrom: [MCP Model Change Publication](#mcp-model-change-publication)
  * satisfiedBy: [mcp_github.rs](../../../crates/reqvire-cli/src/mcp_github.rs)
  * satisfiedBy: [mcp_process.rs](../../../crates/reqvire-cli/src/mcp_process.rs)
  * satisfiedBy: [mcp_worktrees.rs](../../../crates/reqvire-cli/src/mcp_worktrees.rs)
---

### MCP Pull Request Commenting

WHEN a client requests a comment on a pull request in the pinned repository, the system SHALL publish the supplied body to that pull request and return the confirmed comment identity.

#### Concept References
  * [Model Change Publication](../../Thesaurus/Thesaurus.md#model-change-publication)

#### Metadata
  * type: requirement

#### Contract References
  * [MCP GitHub Tool Availability Specification](Specifications.md#mcp-github-tool-availability-specification)
  * [MCP Publication Scope and Recovery Specification](Specifications.md#mcp-publication-scope-and-recovery-specification)
  * [MCP Worktree Context Isolation Specification](Specifications.md#mcp-worktree-context-isolation-specification)

#### Relations
  * definedBy: [MCP Pull Request Commenting Specification](Specifications.md#mcp-pull-request-commenting-specification)
  * derivedFrom: [MCP Model Change Publication](#mcp-model-change-publication)
  * satisfiedBy: [mcp_github.rs](../../../crates/reqvire-cli/src/mcp_github.rs)
---

### MCP Pull Request Creation

WHEN a client requests a pull request for a pushed owned branch with an explicit base, the system SHALL create or identify a matching pull request within the pinned repository.

#### Concept References
  * [Model Change Publication](../../Thesaurus/Thesaurus.md#model-change-publication)

#### Metadata
  * type: requirement

#### Contract References
  * [MCP GitHub Tool Availability Specification](Specifications.md#mcp-github-tool-availability-specification)
  * [MCP Publication Scope and Recovery Specification](Specifications.md#mcp-publication-scope-and-recovery-specification)
  * [MCP Worktree Context Isolation Specification](Specifications.md#mcp-worktree-context-isolation-specification)

#### Relations
  * definedBy: [MCP Pull Request Creation Specification](Specifications.md#mcp-pull-request-creation-specification)
  * derivedFrom: [MCP Model Change Publication](#mcp-model-change-publication)
  * satisfiedBy: [mcp_github.rs](../../../crates/reqvire-cli/src/mcp_github.rs)
---

### MCP Model Evidence Tools

The system shall expose MCP read tools that return model evidence needed by external tools and AI agents.

#### Details
WHEN a caller requests contract-reference filtering, the system SHALL apply the shared search presence and target-filter semantics.

- The MCP interface shall expose read tools for authoritative Reqvire model evidence.
- Model evidence tools shall support element lookup, model structure, containment, collection, submodel analysis, and split semantic export collection.
- Semantic export tools shall be named under the `reqvire.semantic` namespace and shall expose separate read tools for ontology vocabulary, SHACL shapes, SKOS concepts, generated model facts, a combined graph wrapper, and the canonical layer-composed export operation.
- `reqvire.semantic.export` shall support typed `layers` values `ontologies`, `shapes`, `concepts`, `model`, `external-used`, `prefixes`, and `queries`; omitted or empty layers shall export all public layers.
- The `ontologies`, `shapes`, `concepts`, `model`, and `graph` tools shall be stable wrappers over the same export-layer serialization contract.
- Semantic prefix and vocabulary tools shall keep imported external ontology declarations hidden by default and expose only used external subset entries through a typed `include_external` argument with explicit external markers and source metadata generated through the o-kernel subset layer.
- Model evidence tools shall support read-only semantic query execution over collected ontology, SHACL, model, and ontology projection RDF when requested by a typed MCP operation, and shall support an explicit `include_external` argument for querying the graph that includes the used external subset.
- Model evidence tools shall expose the canonical capability/requirement/ontology model, including `ontology` elements, `#### Concept References`, reusable `semantic-contract` shape profiles, constrained requirements, and ontology-use relations where the underlying Reqvire operation returns them.
- Model evidence tools shall include revision metadata when model state affects interpretation.
- Model evidence tools shall not mutate the model or filesystem.

#### Metadata
  * type: requirement

#### Contract References
  * [Requirement Governance Metadata Specification](../../ModelStructure/Specifications.md#requirement-governance-metadata-specification)
  * [SearchFiltering](../../Reports/ModelReports/SearchFiltering.md#searchfiltering)
  * [Collect Content Specification](../../Reports/ModelReports/Specifications.md#collect-content-specification)
  * [Containment View Report Contract Specification](../../Reports/ModelReports/Specifications.md#containment-view-report-contract-specification)
  * [Contract Reference Evidence Projection Specification](../../Reports/ModelReports/Specifications.md#contract-reference-evidence-projection-specification)
  * [Flexible Search Type Filtering Contract Specification](../../Reports/ModelReports/Specifications.md#flexible-search-type-filtering-contract-specification)
  * [Ontology Collection Output Specification](../../Reports/ModelReports/Specifications.md#ontology-collection-output-specification)
  * [Requirement Submodels Report Specification](../../Reports/ModelReports/Specifications.md#requirement-submodels-report-specification)
  * [Local External Ontology Source Specification](../../Semantics/SemanticModelSpecifications.md#local-external-ontology-source-specification)

#### Relations
  * definedBy: [MCP Model Evidence Tools Specification](Specifications.md#mcp-model-evidence-tools-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
  * satisfiedBy: [read_tools.rs](../../../crates/reqvire-core/src/tool_interface/read_tools.rs)
  * verifiedBy: [MCP Model Evidence Tools Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-model-evidence-tools-verification)
---

### MCP Model Evidence Element Selection

When an MCP model-evidence tool selects an existing source element, the system SHALL apply the shared element-selection contract within the requested model context before producing evidence.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Existing Element Selection Specification](../../ModelStructure/ElementSelection.md#existing-element-selection-specification)

#### Relations
  * derivedFrom: [MCP Model Evidence Tools](#mcp-model-evidence-tools)
  * satisfiedBy: [mod.rs](../../../crates/reqvire-core/src/operations/mod.rs)
  * satisfiedBy: [semantic_tools.rs](../../../crates/reqvire-core/src/tool_interface/semantic_tools.rs)
  * verifiedBy: [Existing Element Selection Verification](../../Verifications/ModelStructure/ElementSelectionVerifications.md#existing-element-selection-verification)
---

### MCP Semantic Query Tools

The system shall expose an MCP read tool for SPARQL queries over Reqvire semantic RDF evidence.

#### Details
Detailed engine, graph composition, graph-role metadata, query-form result shape, prefix discovery, error, read-only, and safety rules shall follow the associated specification.

#### Metadata
  * type: requirement

#### Contract References
  * [Model Revision Hash Specification](../../Processing/ContentHashing/Specifications.md#model-revision-hash-specification)
  * [Semantic Relation Family Projection Specification](../../Reports/ModelReports/Specifications.md#semantic-relation-family-projection-specification)

#### Relations
  * definedBy: [MCP Semantic Query Tools Specification](Specifications.md#mcp-semantic-query-tools-specification)
  * derivedFrom: [MCP Model Evidence Tools](#mcp-model-evidence-tools)
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
  * satisfiedBy: [semantic_tools.rs](../../../crates/reqvire-core/src/tool_interface/semantic_tools.rs)
  * verifiedBy: [MCP Semantic Query Tools Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-semantic-query-tools-verification)
---

### MCP Prompt Guidance

The system SHALL expose MCP prompts that guide regular Reqvire workflows and semantic query construction.

#### Details
- The MCP interface SHALL advertise the standard MCP prompts capability.
- The MCP interface SHALL support `prompts/list` and `prompts/get` for Reqvire-authored prompt templates.
- Prompt templates SHALL include regular Reqvire model exploration, change planning, implementation task generation, capability/requirement authoring, verification authoring/alignment, model-structure refactoring, change-impact audit, concept authoring, model-quality audit, and verification coverage review workflows.
- Prompt templates SHALL include semantic query, semantic verification search, semantic contract-context search, and ontology/semantic-contract authoring workflows.
- Semantic prompt templates SHALL direct clients to use `reqvire.semantic.vocabulary`, `reqvire.semantic.prefixes`, and `reqvire.semantic.sparql` for ontology-aware questions, and SHALL state that `include_external` exposes only the used external subset (`reqvire:external-used-subset`) rather than raw external dependency files.
- Prompt templates SHALL be imported into Rust at build time and SHALL not be read from workspace source files at runtime.
- Prompt retrieval SHALL not mutate the model or filesystem.

When guiding contract authoring, refactoring, exploration, implementation planning, impact review, or coverage assessment, the system SHALL distinguish shared implementation obligations from content dependencies according to the referenced contract semantics.

#### Metadata
  * type: requirement

#### Contract References
  * [MCP Formatting Tool Specification](Specifications.md#mcp-formatting-tool-specification)
  * [MCP Lint Fix Tool Specification](Specifications.md#mcp-lint-fix-tool-specification)
  * [Model Interface Operation Boundary Specification](../Specifications.md#model-interface-operation-boundary-specification)
  * [Contract Reference Semantics Specification](../../ModelStructure/Specifications.md#contract-reference-semantics-specification)
  * [Canonical Model Mutation Persistence Specification](../../Operations/ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)

#### Relations
  * definedBy: [MCP Prompt Guidance Specification](Specifications.md#mcp-prompt-guidance-specification)
  * derivedFrom: [MCP Semantic Query Tools](#mcp-semantic-query-tools)
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

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [MCP Semantic Prefix Registry Tools Specification](Specifications.md#mcp-semantic-prefix-registry-tools-specification)
  * derivedFrom: [MCP Semantic Query Tools](#mcp-semantic-query-tools)
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
  * satisfiedBy: [prefixes.rs](../../../crates/reqvire-core/src/semantic_contract/prefixes.rs)
  * satisfiedBy: [semantic_tools.rs](../../../crates/reqvire-core/src/tool_interface/semantic_tools.rs)
  * verifiedBy: [MCP Semantic Prefix Registry Tools Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-semantic-prefix-registry-tools-verification)
---

### MCP Semantic Relation Family Projection Access

The system shall make semantic-export relation-family projection facts available through MCP semantic query and vocabulary tools.

#### Details
- MCP shall expose normalized relation-family vocabulary and query examples sourced from the ontology/semantic export contract.
- MCP shall query relation-family projection facts from the existing semantic query graph; MCP does not own relation-family materialization.
- MCP shall not rebuild relation-family facts, execute projection-side construct materialization, mutate model source files, or write generated triples back to Markdown.

#### Concept References
  * [Relation family construct query](../../Thesaurus/Thesaurus.md#relation-family-construct-query)
  * [Model relation](../../Thesaurus/Thesaurus.md#model-relation)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [MCP Semantic Relation Family Projection Access Specification](Specifications.md#mcp-semantic-relation-family-projection-access-specification)
  * derivedFrom: [MCP Semantic Query Tools](#mcp-semantic-query-tools)
  * satisfiedBy: [semantic_tools.rs](../../../crates/reqvire-core/src/tool_interface/semantic_tools.rs)
  * verifiedBy: [MCP Semantic Query Tools Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-semantic-query-tools-verification)
---

### MCP Semantic Vocabulary Tools

The system shall expose an MCP read tool that pages compact semantic vocabulary for SPARQL query construction.

#### Details
- The MCP interface shall expose semantic vocabulary from the parsed semantic model index.
- The vocabulary tool shall include ontology-defined prefixes and a SPARQL prefix block in every response.
- The vocabulary tool shall support section paging for prefixes, classes, properties, relation families, controlled vocabularies, semantic contracts, query patterns, source map entries, and diagnostics.
- The vocabulary tool shall support ontology-document filtering for authored terms and used external subset terms.
- The vocabulary tool shall expose relation families with normalized forward and inverse properties so clients can query semantic relation meaning instead of hard-coding raw relation tokens.
- The vocabulary tool shall not mutate the model or filesystem.
- The vocabulary tool shall not rebuild or reload the semantic store to answer vocabulary discovery.

#### Concept References
  * [Relation Family](../../Thesaurus/Thesaurus.md#relation-family)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [MCP Semantic Vocabulary Tools Specification](Specifications.md#mcp-semantic-vocabulary-tools-specification)
  * derivedFrom: [MCP Semantic Query Tools](#mcp-semantic-query-tools)
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
  * satisfiedBy: [vocabulary.rs](../../../crates/reqvire-core/src/semantic_contract/vocabulary.rs)
  * satisfiedBy: [semantic_tools.rs](../../../crates/reqvire-core/src/tool_interface/semantic_tools.rs)
  * verifiedBy: [MCP Semantic Vocabulary Tools Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-semantic-vocabulary-tools-verification)
---

### MCP Mutation Concurrency Control

While mutation mode is enabled, the system SHALL exclusively own each admitted context's Git branch and worktree, serialize its model changes, and validate and persist each successful mutation before publishing that context's model state.

#### Details
- The MCP server shall prevent concurrent mutation requests from interleaving writes for the same workspace.
- The MCP server SHALL reject mutation startup for dirty, invalid, detached, uncommitted, or already-owned worktrees.
- If branch, worktree or repository-administration ownership cannot be acquired, the system SHALL reject the operation with an actionable diagnostic that distinguishes contention from filesystem failures, without modifying model files or bypassing ownership.
- The MCP server SHALL create automatic commits only when explicitly enabled at startup; without that opt-in, successful mutations SHALL leave HEAD and the Git index unchanged.
- The MCP server SHALL preserve the previous accepted model after rejected or failed mutations and SHALL reject unexpected branch or HEAD changes.
- The MCP server shall make observed model revision visible to clients for read responses.
- The MCP server shall keep post-mutation model state synchronized before serving dependent reads.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [In-Memory Model Build Cache Specification](../../ModelStructure/Specifications.md#in-memory-model-build-cache-specification)

#### Relations
  * definedBy: [MCP Mutation Concurrency Control Specification](Specifications.md#mcp-mutation-concurrency-control-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
  * satisfiedBy: [mcp_session.rs](../../../crates/reqvire-cli/src/mcp_session.rs)
  * satisfiedBy: [mutation_io.rs](../../../crates/reqvire-core/src/mutation_io.rs)
  * verifiedBy: [MCP HTTP Transport End-to-End Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-http-transport-end-to-end-verification)
---

### MCP Mutation Execution Flow

The system SHALL execute MCP mutations through deterministic operation-specific preview and execution behavior backed by Reqvire core.

#### Details
The system SHALL provide preview, execution, diagnostics, changed-file reporting, affected-scope reporting, synchronization, and mutation safety according to its mutation execution specification.

When a client creates, removes, or relinks a Contract Reference, the system SHALL apply the shared reference mutation contract.

#### Metadata
  * type: requirement

#### Contract References
  * [Model Interface Operation Boundary Specification](../Specifications.md#model-interface-operation-boundary-specification)
  * [Contract Reference Mutation Specification](../../ModelStructure/Specifications.md#contract-reference-mutation-specification)
  * [In-Memory Model Build Cache Specification](../../ModelStructure/Specifications.md#in-memory-model-build-cache-specification)
  * [Canonical Model Mutation Persistence Specification](../../Operations/ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)

#### Relations
  * definedBy: [MCP Mutation Execution Flow Specification](Specifications.md#mcp-mutation-execution-flow-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * verifiedBy: [MCP Mutation Execution Flow Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-mutation-execution-flow-verification)
---

### MCP Mutation Tool Safety

The system SHALL expose model authoring exclusively through typed MCP tools with mutation enablement, complete candidate validation and canonical transactional persistence.

#### Metadata
  * type: requirement

#### Contract References
  * [Model Interface Operation Boundary Specification](../Specifications.md#model-interface-operation-boundary-specification)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [File Persistence Behavior](../../ModelStructure/Behaviors.md#file-persistence-behavior)
  * [Existing Element Selection Specification](../../ModelStructure/ElementSelection.md#existing-element-selection-specification)
  * [In-Memory Model Build Cache Specification](../../ModelStructure/Specifications.md#in-memory-model-build-cache-specification)
  * [Relation Operations Specification](../../ModelStructure/Specifications.md#relation-operations-specification)
  * [Atomic Relation Relink Workflow Specification](../../Operations/ModelOperations/Specifications.md#atomic-relation-relink-workflow-specification)
  * [Canonical Model Mutation Persistence Specification](../../Operations/ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)
  * [Create Element Workflow Specification](../../Operations/ModelOperations/Specifications.md#create-element-workflow-specification)
  * [Delete Element Workflow Specification](../../Operations/ModelOperations/Specifications.md#delete-element-workflow-specification)
  * [Merge Element Workflow Specification](../../Operations/ModelOperations/Specifications.md#merge-element-workflow-specification)
  * [Move Element Workflow Specification](../../Operations/ModelOperations/Specifications.md#move-element-workflow-specification)
  * [Move File Operation Contract Specification](../../Operations/ModelOperations/Specifications.md#move-file-operation-contract-specification)
  * [Move Folder Operation Contract Specification](../../Operations/ModelOperations/Specifications.md#move-folder-operation-contract-specification)
  * [Relation Consistency Maintenance Contract Specification](../../Operations/ModelOperations/Specifications.md#relation-consistency-maintenance-contract-specification)
  * [Rename Element Operation Contract Specification](../../Operations/ModelOperations/Specifications.md#rename-element-operation-contract-specification)

#### Relations
  * definedBy: [MCP Mutation Tool Safety Specification](Specifications.md#mcp-mutation-tool-safety-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * specify: [MCP Model Authoring](../InterfacesFeature.md#mcp-model-authoring)
  * verifiedBy: [MCP Mutation Tool Safety Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-mutation-tool-safety-verification)
---

### MCP Add Element Tool

The system SHALL expose typed MCP tools to create or replace a model element from typed Markdown content under the supported interface permissions.

#### Metadata
  * type: requirement

#### Contract References
  * [Diff Output Format Specification](../Specifications.md#diff-output-format-specification)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [File Persistence Behavior](../../ModelStructure/Behaviors.md#file-persistence-behavior)
  * [Contract Bindings Hierarchical Independence Constraint](../../ModelStructure/Constraints.md#contract-bindings-hierarchical-independence-constraint)
  * [Contract Bindings Satisfied Contract Constraint](../../ModelStructure/Constraints.md#contract-bindings-satisfied-contract-constraint)
  * [Workspace Scope Specification](../../ModelStructure/Specifications.md#workspace-scope-specification)
  * [Create Element Override Behavior](../../Operations/ModelOperations/Behaviors.md#create-element-override-behavior)
  * [Target Location Constraint](../../Operations/ModelOperations/Constraints.md#target-location-constraint)
  * [Canonical Model Mutation Persistence Specification](../../Operations/ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)
  * [Create Element Workflow Specification](../../Operations/ModelOperations/Specifications.md#create-element-workflow-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [MCP Add Element Tool Specification](Specifications.md#mcp-add-element-tool-specification)
  * derivedFrom: [MCP Mutation Tool Safety](#mcp-mutation-tool-safety)
  * satisfiedBy: [mutation_tools.rs](../../../crates/reqvire-core/src/tool_interface/mutation_tools.rs)
  * verifiedBy: [MCP Add Element Tool Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-add-element-tool-verification)
---

### MCP Merge Elements Tool

The system SHALL expose typed MCP tools to merge selected elements into a compatible target under the supported interface permissions.

#### Metadata
  * type: requirement

#### Contract References
  * [Contract Bindings Hierarchical Independence Constraint](../../ModelStructure/Constraints.md#contract-bindings-hierarchical-independence-constraint)
  * [Contract Bindings Satisfied Contract Constraint](../../ModelStructure/Constraints.md#contract-bindings-satisfied-contract-constraint)
  * [Existing Element Selection Specification](../../ModelStructure/ElementSelection.md#existing-element-selection-specification)
  * [Merge Content Transformation Behavior](../../Operations/ModelOperations/Behaviors.md#merge-content-transformation-behavior)
  * [Merge Type Compatibility Constraint](../../Operations/ModelOperations/Constraints.md#merge-type-compatibility-constraint)
  * [Canonical Model Mutation Persistence Specification](../../Operations/ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)
  * [Merge Element Workflow Specification](../../Operations/ModelOperations/Specifications.md#merge-element-workflow-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [MCP Merge Elements Tool Specification](Specifications.md#mcp-merge-elements-tool-specification)
  * derivedFrom: [MCP Mutation Tool Safety](#mcp-mutation-tool-safety)
  * satisfiedBy: [mutation_tools.rs](../../../crates/reqvire-core/src/tool_interface/mutation_tools.rs)
---

### MCP Move Asset Tool

The system SHALL expose typed MCP tools to move or rename an evidence asset and update model references to it under the supported interface permissions.

#### Metadata
  * type: requirement

#### Contract References
  * [Diff Output Format Specification](../Specifications.md#diff-output-format-specification)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [File Persistence Behavior](../../ModelStructure/Behaviors.md#file-persistence-behavior)
  * [Canonical Model Mutation Persistence Specification](../../Operations/ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [MCP Move Asset Tool Specification](Specifications.md#mcp-move-asset-tool-specification)
  * derivedFrom: [MCP Mutation Tool Safety](#mcp-mutation-tool-safety)
  * satisfiedBy: [mutation_tools.rs](../../../crates/reqvire-core/src/tool_interface/mutation_tools.rs)
  * verifiedBy: [MCP Move Asset Tool Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-move-asset-tool-verification)
---

### MCP Move Element Tool

The system SHALL expose typed MCP tools to relocate a selected element and rewrite its affected references under the supported interface permissions.

#### Metadata
  * type: requirement

#### Contract References
  * [Diff Output Format Specification](../Specifications.md#diff-output-format-specification)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [File Persistence Behavior](../../ModelStructure/Behaviors.md#file-persistence-behavior)
  * [Existing Element Selection Specification](../../ModelStructure/ElementSelection.md#existing-element-selection-specification)
  * [Workspace Scope Specification](../../ModelStructure/Specifications.md#workspace-scope-specification)
  * [Target Location Constraint](../../Operations/ModelOperations/Constraints.md#target-location-constraint)
  * [Canonical Model Mutation Persistence Specification](../../Operations/ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)
  * [Move Element Workflow Specification](../../Operations/ModelOperations/Specifications.md#move-element-workflow-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [MCP Move Element Tool Specification](Specifications.md#mcp-move-element-tool-specification)
  * derivedFrom: [MCP Mutation Tool Safety](#mcp-mutation-tool-safety)
  * satisfiedBy: [mutation_tools.rs](../../../crates/reqvire-core/src/tool_interface/mutation_tools.rs)
  * verifiedBy: [MCP Move Element Tool Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-move-element-tool-verification)
---

### MCP Move File Tool

The system SHALL expose typed MCP tools to move a model file and its contained elements under the supported interface permissions.

#### Metadata
  * type: requirement

#### Contract References
  * [Diff Output Format Specification](../Specifications.md#diff-output-format-specification)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [File Persistence Behavior](../../ModelStructure/Behaviors.md#file-persistence-behavior)
  * [Target Location Constraint](../../Operations/ModelOperations/Constraints.md#target-location-constraint)
  * [Canonical Model Mutation Persistence Specification](../../Operations/ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [MCP Move File Tool Specification](Specifications.md#mcp-move-file-tool-specification)
  * derivedFrom: [MCP Mutation Tool Safety](#mcp-mutation-tool-safety)
  * satisfiedBy: [mutation_tools.rs](../../../crates/reqvire-core/src/tool_interface/mutation_tools.rs)
  * verifiedBy: [MCP Move File Tool Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-move-file-tool-verification)
---

### MCP Move Folder Tool

The system SHALL expose a typed MCP mutation tool to move or rename a model folder subtree through the shared folder-move operation.

#### Metadata
  * type: requirement

#### Contract References
  * [Diff Output Format Specification](../Specifications.md#diff-output-format-specification)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [File Persistence Behavior](../../ModelStructure/Behaviors.md#file-persistence-behavior)
  * [Target Location Constraint](../../Operations/ModelOperations/Constraints.md#target-location-constraint)
  * [Canonical Model Mutation Persistence Specification](../../Operations/ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)
  * [Move Folder Operation Contract Specification](../../Operations/ModelOperations/Specifications.md#move-folder-operation-contract-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [MCP Move Folder Tool Specification](Specifications.md#mcp-move-folder-tool-specification)
  * derivedFrom: [MCP Mutation Tool Safety](#mcp-mutation-tool-safety)
  * satisfiedBy: [mutation_tools.rs](../../../crates/reqvire-core/src/tool_interface/mutation_tools.rs)
  * verifiedBy: [MCP Move Folder Tool Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-move-folder-tool-verification)
  * verifiedBy: [MCP Mutation Tool Safety Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-mutation-tool-safety-verification)
---

### MCP Relation Management Tools

The system SHALL expose typed MCP tools to manage typed relations and requirement-owned contract dependencies under the supported interface permissions.

#### Metadata
  * type: requirement

#### Contract References
  * [Diff Output Format Specification](../Specifications.md#diff-output-format-specification)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [Existing Element Selection Specification](../../ModelStructure/ElementSelection.md#existing-element-selection-specification)
  * [Contract Reference Mutation Specification](../../ModelStructure/Specifications.md#contract-reference-mutation-specification)
  * [Relation Operations Specification](../../ModelStructure/Specifications.md#relation-operations-specification)
  * [Canonical Model Mutation Persistence Specification](../../Operations/ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [MCP Relation Management Tools Specification](Specifications.md#mcp-relation-management-tools-specification)
  * derivedFrom: [MCP Mutation Tool Safety](#mcp-mutation-tool-safety)
  * satisfiedBy: [mutation_tools.rs](../../../crates/reqvire-core/src/tool_interface/mutation_tools.rs)
  * verifiedBy: [MCP Link Tool Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-link-tool-verification)
  * verifiedBy: [MCP Unlink Tool Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-unlink-tool-verification)
---

### MCP Contract Binding Tools

The system SHALL expose typed MCP tools to add and remove independent implementation dependencies on compatible requirement-owned contracts under the supported interface permissions.

#### Metadata
  * type: requirement

#### Contract References
  * [File Persistence Behavior](../../ModelStructure/Behaviors.md#file-persistence-behavior)
  * [Contract Bindings Hierarchical Independence Constraint](../../ModelStructure/Constraints.md#contract-bindings-hierarchical-independence-constraint)
  * [Contract Bindings Satisfied Contract Constraint](../../ModelStructure/Constraints.md#contract-bindings-satisfied-contract-constraint)
  * [Canonical Model Mutation Persistence Specification](../../Operations/ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)

#### Relations
  * definedBy: [MCP Contract Binding Tools Behavior](Specifications.md#mcp-contract-binding-tools-behavior)
  * derivedFrom: [MCP Relation Management Tools](#mcp-relation-management-tools)
  * satisfiedBy: [mutation_tools.rs](../../../crates/reqvire-core/src/tool_interface/mutation_tools.rs)
  * verifiedBy: [MCP Contract Binding Removal Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-contract-binding-removal-verification)
  * verifiedBy: [MCP Contract Binding Tool Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-contract-binding-tool-verification)
---

### MCP Relink Tool

The system SHALL expose typed MCP tools to replace a relation or supported contract-reference target atomically under the supported interface permissions.

#### Metadata
  * type: requirement

#### Contract References
  * [Diff Output Format Specification](../Specifications.md#diff-output-format-specification)
  * [Existing Element Selection Specification](../../ModelStructure/ElementSelection.md#existing-element-selection-specification)
  * [Atomic Relink Validity Constraint](../../Operations/ModelOperations/Constraints.md#atomic-relink-validity-constraint)
  * [Atomic Relation Relink Workflow Specification](../../Operations/ModelOperations/Specifications.md#atomic-relation-relink-workflow-specification)
  * [Canonical Model Mutation Persistence Specification](../../Operations/ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [MCP Relink Tool Specification](Specifications.md#mcp-relink-tool-specification)
  * definedBy: [Mutating Command Hierarchy Safety Contract Specification](Specifications.md#mutating-command-hierarchy-safety-contract-specification)
  * derivedFrom: [MCP Relation Management Tools](#mcp-relation-management-tools)
  * satisfiedBy: [mutation_tools.rs](../../../crates/reqvire-core/src/tool_interface/mutation_tools.rs)
---

### MCP Remove Asset Tool

The system SHALL expose typed MCP tools to remove an evidence asset and clean up its model references under the supported interface permissions.

#### Metadata
  * type: requirement

#### Contract References
  * [Diff Output Format Specification](../Specifications.md#diff-output-format-specification)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [File Persistence Behavior](../../ModelStructure/Behaviors.md#file-persistence-behavior)
  * [Canonical Model Mutation Persistence Specification](../../Operations/ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [MCP Remove Asset Tool Specification](Specifications.md#mcp-remove-asset-tool-specification)
  * derivedFrom: [MCP Mutation Tool Safety](#mcp-mutation-tool-safety)
  * satisfiedBy: [mutation_tools.rs](../../../crates/reqvire-core/src/tool_interface/mutation_tools.rs)
  * verifiedBy: [MCP Remove Asset Tool Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-remove-asset-tool-verification)
---

### MCP Remove Element Tool

The system SHALL expose typed MCP tools to remove a selected element while preserving the remaining model under the supported interface permissions.

#### Metadata
  * type: requirement

#### Contract References
  * [Diff Output Format Specification](../Specifications.md#diff-output-format-specification)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [File Persistence Behavior](../../ModelStructure/Behaviors.md#file-persistence-behavior)
  * [Existing Element Selection Specification](../../ModelStructure/ElementSelection.md#existing-element-selection-specification)
  * [Canonical Model Mutation Persistence Specification](../../Operations/ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)
  * [Delete Element Workflow Specification](../../Operations/ModelOperations/Specifications.md#delete-element-workflow-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [MCP Remove Element Tool Specification](Specifications.md#mcp-remove-element-tool-specification)
  * derivedFrom: [MCP Mutation Tool Safety](#mcp-mutation-tool-safety)
  * satisfiedBy: [mutation_tools.rs](../../../crates/reqvire-core/src/tool_interface/mutation_tools.rs)
  * verifiedBy: [MCP Remove Element Tool Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-remove-element-tool-verification)
---

### MCP Rename Element Tool

The system SHALL expose typed MCP tools to rename a selected element and update its identity and references under the supported interface permissions.

#### Metadata
  * type: requirement

#### Contract References
  * [Diff Output Format Specification](../Specifications.md#diff-output-format-specification)
  * [Dry-Run Mode Behavior](../../ModelStructure/Behaviors.md#dry-run-mode-behavior)
  * [File Persistence Behavior](../../ModelStructure/Behaviors.md#file-persistence-behavior)
  * [Existing Element Selection Specification](../../ModelStructure/ElementSelection.md#existing-element-selection-specification)
  * [Canonical Model Mutation Persistence Specification](../../Operations/ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [MCP Rename Element Tool Specification](Specifications.md#mcp-rename-element-tool-specification)
  * derivedFrom: [MCP Mutation Tool Safety](#mcp-mutation-tool-safety)
  * satisfiedBy: [mutation_tools.rs](../../../crates/reqvire-core/src/tool_interface/mutation_tools.rs)
  * verifiedBy: [MCP Rename Element Tool Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-rename-element-tool-verification)
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
  * definedBy: [MCP Protocol Standard Conformance Specification](Specifications.md#mcp-protocol-standard-conformance-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
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

#### Contract References
  * [Requirement Implementation Coverage Logic Specification](../../Implementation/Traceability/Specifications.md#requirement-implementation-coverage-logic-specification)
  * [Lint Output Specification](../../Operations/Linting/Specifications.md#lint-output-specification)
  * [Impact Scope Computation Specification](../../Processing/ChangeImpact/Specifications.md#impact-scope-computation-specification)
  * [Implementation Coverage Output Structure Specification](../../Reports/ModelReports/Specifications.md#implementation-coverage-output-structure-specification)
  * [Resources Report Format Specification](../../Reports/ModelReports/Specifications.md#resources-report-format-specification)
  * [Verification Coverage Specification](../../Reports/ModelReports/Specifications.md#verification-coverage-specification)
  * [Verification Trace Tree Construction](../../Verification/Traceability/Specifications.md#verification-trace-tree-construction)

#### Relations
  * definedBy: [MCP Quality Traceability Tools Specification](Specifications.md#mcp-quality-traceability-tools-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
  * satisfiedBy: [read_tools.rs](../../../crates/reqvire-core/src/tool_interface/read_tools.rs)
  * verifiedBy: [MCP Quality Traceability Tools Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-quality-traceability-tools-verification)
---

### MCP Coverage Scope Selection

When a client selects a capability in a coverage tool request, the system SHALL return the shared coverage report for that capability subtree from the current validated model snapshot.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Existing Element Selection Specification](../../ModelStructure/ElementSelection.md#existing-element-selection-specification)
  * [Coverage Scope Specification](../../Reports/ModelReports/Specifications.md#coverage-scope-specification)

#### Relations
  * derivedFrom: [MCP Quality Traceability Tools](#mcp-quality-traceability-tools)
  * satisfiedBy: [definitions.rs](../../../crates/reqvire-core/src/tool_interface/definitions.rs)
  * satisfiedBy: [dispatch.rs](../../../crates/reqvire-core/src/tool_interface/dispatch.rs)
  * satisfiedBy: [read_tools.rs](../../../crates/reqvire-core/src/tool_interface/read_tools.rs)
---

### MCP Resource Interface

The system shall expose MCP resources only as read-only, revision-tagged views of workspace, model, element, file, and report state.

#### Details
Detailed resource listing, template, read, revision metadata, non-mutating behavior, and source-of-truth rules shall follow the associated specification.

#### Metadata
  * type: requirement

#### Contract References
  * [Containment View Report Contract Specification](../../Reports/ModelReports/Specifications.md#containment-view-report-contract-specification)
  * [Requirement Submodels Report Specification](../../Reports/ModelReports/Specifications.md#requirement-submodels-report-specification)
  * [Resources Report Format Specification](../../Reports/ModelReports/Specifications.md#resources-report-format-specification)

#### Relations
  * definedBy: [MCP Resource Interface Specification](Specifications.md#mcp-resource-interface-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
  * satisfiedBy: [read_tools.rs](../../../crates/reqvire-core/src/tool_interface/read_tools.rs)
  * verifiedBy: [MCP Resource Interface Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-resource-interface-verification)
---

### MCP Server Command

The system shall provide `reqvire mcp` as the command that starts the Reqvire MCP server.

#### Details
- The MCP server command shall start protocol service mode for the current Reqvire workspace.
- WHEN a supported transport is selected, the system SHALL start that transport for the resolved workspace.
- IF transport selection or transport-specific options are incompatible, THEN the system SHALL reject startup before model loading or protocol service.
- The MCP server command shall keep server startup behavior outside the MCP tool surface.
- The MCP server command shall expose read/report tools by default.
- The MCP server command shall expose mutation tools only when mutation capability is explicitly enabled at startup.
- The MCP server command SHALL disable automatic commits by default and SHALL accept an explicit commit opt-in only when mutation capability is enabled.
- WHEN GitHub publication is requested, the system SHALL apply startup availability checks and expose diagnostic state independently of local mutation and commit enablement.
- The MCP server command shall support opt-in element size estimates when explicitly enabled at startup.
- WHILE HTTP transport is selected, WHEN additional browser origins are configured at startup, the system SHALL apply them to the MCP endpoint's origin policy.
- WHILE HTTP transport is selected, WHEN additional endpoint hostnames are configured at startup, the system SHALL apply them to the MCP endpoint's host policy.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Workspace Scope Specification](../../ModelStructure/Specifications.md#workspace-scope-specification)
  * [MCP Streamable HTTP Transport Safety Specification](Specifications.md#mcp-streamable-http-transport-safety-specification)

#### Relations
  * definedBy: [MCP Server Command Specification](Specifications.md#mcp-server-command-specification)
  * definedBy: [MCP Size Estimate Startup Specification](Specifications.md#mcp-size-estimate-startup-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * satisfiedBy: [main.rs](../../../crates/reqvire-cli/src/main.rs)
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
  * satisfiedBy: [mod.rs](../../../crates/reqvire-core/src/tool_interface/mod.rs)
  * verifiedBy: [MCP Server Command Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-server-command-verification)
  * verifiedBy: [MCP Size Estimate Startup Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-size-estimate-startup-verification)
---

### MCP Server State and Cache

The system shall serve MCP model state from Reqvire core parsing, refreshing source inputs in read-only mode and retaining the accepted persisted snapshot in mutation-enabled mode.

#### Details
- The MCP server shall keep accepted model changes durable in Reqvire source files; mutation-enabled sessions shall also commit each non-empty persisted mutation only when automatic commits are explicitly enabled.
- Both modes shall use Reqvire core parsing and validation to construct accepted model state.
- The MCP server shall report enough revision state for clients to reason about cache freshness.
- Read-only MCP shall refresh stale model state before returning authoritative model evidence, including changes to active exclusions and model construction dependencies without requiring a restart.
- Read-only MCP SHALL allow independent requests to execute concurrently within bounded admission and execution limits, without blocking the asynchronous transport or bypassing controlled workspace writes and core cache freshness checks.
- Mutation-enabled MCP shall reuse its accepted model and captured source inputs for reads and candidate preparation without importing external edits or rebuilding the filesystem cache on each request.
- Mutation-enabled MCP SHALL allow bounded concurrent audited snapshot reads, preserve each read's captured model identity through its response, and keep mutation preparation and publication serialized within the selected context.
- Mutation-enabled MCP shall publish complete graph, page, and semantic state only after candidate validation, successful persistence, and a successful commit when enabled under the ownership contract.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Model Revision Hash Specification](../../Processing/ContentHashing/Specifications.md#model-revision-hash-specification)
  * [In-Memory Model Build Cache Specification](../../ModelStructure/Specifications.md#in-memory-model-build-cache-specification)

#### Relations
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
  * satisfiedBy: [mcp_session.rs](../../../crates/reqvire-cli/src/mcp_session.rs)
  * satisfiedBy: [model_cache.rs](../../../crates/reqvire-core/src/model_cache.rs)
  * satisfiedBy: [mutation_io.rs](../../../crates/reqvire-core/src/mutation_io.rs)
  * satisfiedBy: [arg_helpers.rs](../../../crates/reqvire-core/src/tool_interface/arg_helpers.rs)
---

### MCP Shared Operation Interfaces

The system shall expose MCP tools through shared typed request and result interfaces for matching Reqvire operations.

#### Details
Detailed request/result types, shared operation semantics, adapter boundary, discovery metadata, registry, and error-contract rules shall follow the associated specifications.

#### Metadata
  * type: requirement

#### Contract References
  * [Model Interface Operation Boundary Specification](../Specifications.md#model-interface-operation-boundary-specification)
  * [Shared Core Operation Layer Specification](../Specifications.md#shared-core-operation-layer-specification)
  * [Existing Element Selection Specification](../../ModelStructure/ElementSelection.md#existing-element-selection-specification)
  * [JSON Output Structure](../../Reports/ModelReports/Specifications.md#json-output-structure)

#### Relations
  * definedBy: [MCP Shared Operation Contracts Specification](Specifications.md#mcp-shared-operation-contracts-specification)
  * definedBy: [MCP Tool Call Contracts Specification](Specifications.md#mcp-tool-call-contracts-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
  * satisfiedBy: [mod.rs](../../../crates/reqvire-core/src/operations/mod.rs)
  * satisfiedBy: [definitions.rs](../../../crates/reqvire-core/src/tool_interface/definitions.rs)
  * satisfiedBy: [dispatch.rs](../../../crates/reqvire-core/src/tool_interface/dispatch.rs)
  * satisfiedBy: [mod.rs](../../../crates/reqvire-core/src/tool_interface/mod.rs)
  * satisfiedBy: [read_tools.rs](../../../crates/reqvire-core/src/tool_interface/read_tools.rs)
  * verifiedBy: [MCP Shared Operation Contracts Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-shared-operation-contracts-verification)
  * verifiedBy: [MCP Tool Contract and Side Effect Classification Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-tool-contract-and-side-effect-classification-verification)
---

### MCP Stdio Process Lifecycle

WHILE serving a local MCP client over standard input and output, the system SHALL keep protocol output separate from diagnostics and SHALL release owned runtime resources when the client connection ends or the server is terminated.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
---

### MCP Stdio Transport

WHEN a local MCP client selects standard input and output transport, the system SHALL serve the resolved project workspace through the same typed engineering operations and permission boundaries as the HTTP interface.

#### Metadata
  * type: requirement

#### Contract References
  * [MCP Mutation Concurrency Control Specification](Specifications.md#mcp-mutation-concurrency-control-specification)

#### Relations
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
---

### MCP Streamable HTTP Transport

The system shall provide MCP service through RMCP Streamable HTTP transport.

#### Details
- The MCP server shall start an RMCP-backed HTTP endpoint by default.
- MCP transport mechanics shall not change Reqvire tool semantics.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [MCP Streamable HTTP Transport Specification](Specifications.md#mcp-streamable-http-transport-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
  * verifiedBy: [MCP HTTP Transport End-to-End Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-http-transport-end-to-end-verification)
---

### MCP Streamable HTTP Transport Safety

WHEN HTTP transport is enabled, the system SHALL provide MCP-compliant request handling with configurable browser-origin access.

#### Details
- WHEN no additional browser origins are configured, the system SHALL accept loopback origins and clients without an Origin header.
- WHEN an explicitly permitted browser origin sends an MCP request or CORS preflight, the system SHALL provide the cross-origin response headers needed to access the endpoint.
- IF a request contains an invalid or unpermitted Origin header, THEN the system SHALL reject it before MCP execution.
- IF a configured origin is invalid, THEN the system SHALL reject startup with a diagnostic identifying the invalid value.
- The system SHALL use a loopback HTTP listening address by default.
- WHEN an explicit non-wildcard listening address is configured, the system SHALL accept MCP requests addressed to that host and listening port.
- WHEN additional endpoint hostnames are configured, the system SHALL accept MCP requests addressed to those hosts according to the associated specification.
- IF an MCP request is addressed to an unpermitted host, THEN the system SHALL reject it before MCP execution.
- The system SHALL require explicit startup enablement before exposing mutation tools.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [MCP Streamable HTTP Transport Safety Specification](Specifications.md#mcp-streamable-http-transport-safety-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
  * satisfiedBy: [mcp_http.rs](../../../crates/reqvire-cli/src/mcp_http.rs)
  * verifiedBy: [MCP HTTP Transport End-to-End Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-http-transport-end-to-end-verification)
---

### MCP Structured Payload Interfaces

The system shall provide MCP structured payload interfaces derived from shared Reqvire operation results.

#### Details
Detailed schema-source, semantic evidence, mutation/error result, versioning, and terminal-output separation rules shall follow the associated specification.

#### Metadata
  * type: requirement

#### Contract References
  * [Model Revision Hash Specification](../../Processing/ContentHashing/Specifications.md#model-revision-hash-specification)

#### Relations
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
  * verifiedBy: [MCP Structured Payload Contracts Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-structured-payload-contracts-verification)
---

### MCP Tool Exposure Scope

The system SHALL expose only specified Reqvire model operations and typed worktree, commit, and same-repository publication operations as MCP tools.

#### Details
Detailed supported-operation, exclusion, internal/server-management boundary, startup-validation, and MCP contract rules shall follow the associated specification.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [MCP Tool Exposure Scope Specification](Specifications.md#mcp-tool-exposure-scope-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
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

#### Contract References
  * [MCP Formatting Tool Specification](Specifications.md#mcp-formatting-tool-specification)
  * [MCP Lint Fix Tool Specification](Specifications.md#mcp-lint-fix-tool-specification)

#### Relations
  * definedBy: [MCP Tool Side Effect Classification Specification](Specifications.md#mcp-tool-side-effect-classification-specification)
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * verifiedBy: [MCP Tool Contract and Side Effect Classification Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-tool-contract-and-side-effect-classification-verification)
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

#### Contract Bindings
  * [Workspace Scope Specification](../../ModelStructure/Specifications.md#workspace-scope-specification)
  * [Model Revision Hash Specification](../../Processing/ContentHashing/Specifications.md#model-revision-hash-specification)

#### Relations
  * derivedFrom: [MCP Interface](../InterfacesRequirements.md#mcp-interface)
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
---

### MCP Worktree Model Sessions

WHILE mutation mode is enabled, the system SHALL manage independently owned model sessions in worktrees of the startup Git repository through one MCP endpoint.

#### Details
Worktree administration is an explicit repository operation. It does not change the original checkout branch or broaden the workspace boundary of any model operation.

#### Concept References
  * [MCP Worktree Context](../../Thesaurus/Thesaurus.md#mcp-worktree-context)

#### Metadata
  * type: requirement

#### Relations
  * specify: [MCP Worktree Model Management](../InterfacesFeature.md#mcp-worktree-model-management)
---

### MCP Managed Worktree Removal

WHEN a client requests removal of a clean server-created worktree, the system SHALL stop that context and remove only its disposable worktree while retaining the branch and commits.

#### Concept References
  * [MCP Worktree Context](../../Thesaurus/Thesaurus.md#mcp-worktree-context)

#### Metadata
  * type: requirement

#### Contract References
  * [MCP Worktree Context Isolation Specification](Specifications.md#mcp-worktree-context-isolation-specification)

#### Relations
  * definedBy: [MCP Managed Worktree Removal Specification](Specifications.md#mcp-managed-worktree-removal-specification)
  * derivedFrom: [MCP Worktree Model Sessions](#mcp-worktree-model-sessions)
  * satisfiedBy: [mcp_worktrees.rs](../../../crates/reqvire-cli/src/mcp_worktrees.rs)
---

### MCP Worktree Context Isolation

WHEN a request addresses a model worktree context, the system SHALL execute it against only that context and preserve independent ownership, accepted state, and caches for every other context.

#### Details
The child worker-session requirement implements the MCP side of this boundary. Explorer runtime consumers implement the same boundary through Contract Bindings; references alone do not contribute implementation coverage.

#### Concept References
  * [MCP Worktree Context](../../Thesaurus/Thesaurus.md#mcp-worktree-context)

#### Metadata
  * type: requirement

#### Contract Bindings
  * [MCP Mutation Concurrency Control Specification](Specifications.md#mcp-mutation-concurrency-control-specification)
  * [Workspace Scope Specification](../../ModelStructure/Specifications.md#workspace-scope-specification)

#### Relations
  * definedBy: [MCP Worktree Context Isolation Specification](Specifications.md#mcp-worktree-context-isolation-specification)
  * derivedFrom: [MCP Worktree Model Sessions](#mcp-worktree-model-sessions)
---

### MCP Worktree Worker Sessions

WHEN an MCP request targets an admitted worktree context, the system SHALL route it to that context's isolated worker and apply the shared context boundary to model reads, mutations, resources, prompts, and repository operations.

IF mutation recovery fails while the worker retains a valid accepted snapshot, the system SHALL keep supported reads of that snapshot available with recovery diagnostics and SHALL reject further writes and publication until the context is recovered through normal admission or verified reconciliation of its recorded commit outcome.

#### Concept References
  * [MCP Worktree Context](../../Thesaurus/Thesaurus.md#mcp-worktree-context)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [MCP Worktree Worker Sessions Specification](Specifications.md#mcp-worktree-worker-sessions-specification)
  * derivedFrom: [MCP Worktree Context Isolation](#mcp-worktree-context-isolation)
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
  * satisfiedBy: [mcp_worker.rs](../../../crates/reqvire-cli/src/mcp_worker.rs)
  * satisfiedBy: [mcp_worktrees.rs](../../../crates/reqvire-cli/src/mcp_worktrees.rs)
---

### MCP Worktree Creation

WHEN a client requests a new model branch from an explicit base, the system SHALL create and validate an isolated Git worktree before returning an owned context for that branch.

#### Concept References
  * [MCP Worktree Context](../../Thesaurus/Thesaurus.md#mcp-worktree-context)

#### Metadata
  * type: requirement

#### Contract References
  * [MCP Worktree Context Isolation Specification](Specifications.md#mcp-worktree-context-isolation-specification)

#### Relations
  * definedBy: [MCP Worktree Creation Specification](Specifications.md#mcp-worktree-creation-specification)
  * derivedFrom: [MCP Worktree Model Sessions](#mcp-worktree-model-sessions)
  * satisfiedBy: [mcp_worktrees.rs](../../../crates/reqvire-cli/src/mcp_worktrees.rs)
---

### MCP Worktree Opening and Inventory

WHEN a client selects an existing branch, the system SHALL open an eligible worktree context for that branch without switching any existing checkout and SHALL expose context and ownership information through read-only inventory.

#### Concept References
  * [MCP Worktree Context](../../Thesaurus/Thesaurus.md#mcp-worktree-context)

#### Metadata
  * type: requirement

#### Contract References
  * [MCP Worktree Context Isolation Specification](Specifications.md#mcp-worktree-context-isolation-specification)

#### Relations
  * definedBy: [MCP Worktree Opening and Inventory Specification](Specifications.md#mcp-worktree-opening-and-inventory-specification)
  * derivedFrom: [MCP Worktree Model Sessions](#mcp-worktree-model-sessions)
  * satisfiedBy: [mcp_worktrees.rs](../../../crates/reqvire-cli/src/mcp_worktrees.rs)
---
