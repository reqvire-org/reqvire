# Elements

### CLI Repo Manifest Selection

When a caller explicitly selects a Repo manifest or starts at an initialized Repo workspace, the CLI SHALL pass that selection through the shared manifest-defined workspace scope before executing the selected command, while retaining existing single-repository command behavior.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Manifest-Defined Workspace Scope Specification](../../ModelStructure/RepoWorkspaces.md#manifest-defined-workspace-scope-specification)

#### Relations
  * derivedFrom: [Explicit Workspace Selection](Commands.md#explicit-workspace-selection)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * satisfiedBy: [main.rs](../../../crates/reqvire-cli/src/main.rs)
---

### CLI Repo Manifest Selection Specification

Provide optional global --manifest <file> to override the Repo manifest location for inspection commands, MCP and embedded Explorer serving. Apply --workspace first and normalize the chosen path relative to that workspace. Without the flag, use the initialized client's .repo/manifest.xml. The selected entry identifies the tracked source XML and versioned includes owned by the selected root branch under the shared source-scope contract. Preserve Repo's include and selection semantics at that authoritative source boundary. The optional flag changes selection location, not source ownership; selecting a cached path does not make Repo administration another authoring repository. Do not require a duplicate XML copy or routine Reqvire override for native discovery. Missing or invalid authoritative sources report their path before command execution. A manifest-looking filename alone does not change ordinary Git workspace mode.

Delegate composition, source eligibility and repository ownership to the shared source-scope contract. Server consumers receive the selected location without additional project selectors. Inspection does not silently initialize, sync, fetch or switch checkouts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Repo Manifest Selection](#cli-repo-manifest-selection)
---
