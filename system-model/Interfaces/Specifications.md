# Elements

### Diff Output Format Specification

Shared change evidence for operation previews and applied results.

#### Details
- Identify workspace-relative changed files and deterministic before/after line hunks, including additions, removals and readable context. Retain the existing shared diff result structure.
- MCP tools carry this evidence in structured results for authoring, formatting and lint repair. CLI migration and retained change-impact reports may render the same evidence through their supported output contracts.
- Terminal color and presentation are renderer concerns; MCP does not require ANSI colors or CLI output-path flags.
- Preview evidence must describe the same validated operation candidate as application while leaving files and accepted state unchanged.

#### Metadata
  * type: specification

#### Relations
  * define: [Shared Operation Diff Evidence](InterfacesRequirements.md#shared-operation-diff-evidence)
---

### MCP Interface Boundary Specification

The MCP interface is expected to be a protocol adapter over Reqvire core and shared tool contracts.

#### Details
Boundary rules:
- Reqvire markdown files remain the durable source of truth.
- Reqvire core remains authoritative for parsing, validation, reporting, formatting, and mutation semantics.
- MCP handlers call shared Reqvire tool contracts instead of reimplementing model behavior.
- MCP does not expose arbitrary shell command execution.
- MCP retains only core-built model snapshots under the read-only freshness or mutation ownership contract of each worktree context. Accepted model changes are persisted; external edits are not silently adopted by mutation-enabled contexts.
- Typed worktree, commit, and same-repository GitHub publication operations orchestrate repository services around those contexts without reimplementing model semantics or exposing arbitrary Git/gh execution.
- MCP responses include standard MCP protocol metadata where required and Reqvire contract/workspace/model revision metadata inside Reqvire tool or resource results.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Interface](InterfacesRequirements.md#mcp-interface)
---

### Model Interface Operation Boundary Specification

The supported operation surface separates inspection, authored-model changes, and source upgrades.

#### Details
Command-line operations:
- Validation: `validate`, `concepts validate`.
- Quality inspection: `lint`, `coverage`. `lint` accepts report filters and output controls but never `--fix`.
- Discovery and context: `search`, `collect`, `submodels`.
- Structure and reports: `model`, `traces`, `containment`, `resources`, `change-impact`.
- Artifact generation: static Explorer `export`, `semantic export`, `concepts export`, and the existing `ontologies` compatibility alias.
- Managed-query inspection: `semantic query list`, `validate`, `export`, and `check`.
- Interface startup: `mcp`, `serve`; retain workspace selection, help, version, supported filtering and output controls.
- Source upgrade: `migrate` previews known deterministic migrations; only `migrate --fix` applies those rewrites. Migration is available before a model passes current validation and is not a generic authoring escape hatch.

Unsupported command-line operations:
- Remove `add`, `rm`, `mv`, `rename`, `merge`, `mv-file`, `mv-folder`, `link`, `unlink`, `relink`, `mv-asset`, `rm-asset`, and `format`, including their preview and dry-run forms.
- Reject `lint --fix`. Do not retain hidden aliases, compatibility wrappers, or CLI-to-MCP authoring proxies for removed operations.
- Reject removed commands and options during argument dispatch before model loading or source writes, with guidance naming the supported MCP operation and its mutation enablement where appropriate.
- Report/export destinations are generated artifacts; retaining output options does not authorize rewriting authored model sources as a model operation.

Protocol operations:
- Existing read, query, report, semantic export, workspace, and publication contracts remain available under their established permissions.
- Typed element creation and replacement, removal, relocation, renaming, merging, file/folder moves, relation and contract-dependency changes, and asset moves/removal are MCP-only authoring operations. Their previews remain in their MCP tool contracts.
- `reqvire.format` is MCP-only for preview and application. Preview is available without mutation enablement; `fix: true` requires it.
- `reqvire.lint` reports findings by default. Its new `fix: true` mode requires mutation enablement; reporting remains available without it.
- MCP authored-model mutations persist canonical model formatting automatically through the shared core writer.
- Do not expose an MCP migration tool or a pre-validation migration service mode. Standalone and embedded MCP endpoints become available only after normal startup model validation succeeds. Migration of an incompatible model precedes MCP startup through CLI.

This is an intentional breaking operation-surface change. The core remains authoritative for model semantics; removing a CLI adapter does not remove shared core operations, weaken validation, or prohibit ordinary Git and file-editor workflows outside Reqvire's supported authoring interface.

#### Metadata
  * type: specification

#### Relations
  * define: [Model Interface Operation Boundary](InterfacesRequirements.md#model-interface-operation-boundary)
---

### Shared Core Operation Layer Specification

Reqvire core owns operation request/result APIs independently of external transport availability.

#### Details
- Shared operations own parsing, selection, validation, report construction, mutation preview and application, canonical persistence, diagnostics, and cache invalidation for their operation scope.
- CLI adapters expose only supported inspections, artifact generation, server launch, and deterministic migrations. Removing authoring and formatting commands does not remove their core operations.
- MCP adapters invoke shared core authoring and maintenance operations through typed tools and enforce startup permissions, ownership, accepted-state publication, and protocol wrapping.
- Read operations supported by both adapters preserve the same structured semantic results; CLI adds argument/output rendering and MCP adds typed schemas and envelopes.
- Core authoring and maintenance APIs are not CLI command handlers. MCP must not shell out to removed CLI commands or introduce a second model engine.
- Existing-element selection remains shared. Semantic read/query helpers continue to use the native semantic index and tool contracts.
- Cross-interface tests compare retained read operations. Authoring, formatting, and lint-fix tests exercise core and MCP paths; migrations exercise CLI.

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
