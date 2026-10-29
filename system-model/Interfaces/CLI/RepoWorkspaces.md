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

Provide optional global --manifest <file> to override the Repo manifest location for inspection commands, MCP and embedded Explorer serving. Apply --workspace first and normalize the chosen path relative to that workspace. Without the flag, use the initialized client's .repo/manifest.xml. Resolve Repo-managed includes from its manifest checkout, including standalone manifests and local overlays; do not reinterpret an include name as a Central-root filename or require a duplicate root manifest. An explicit path selects the same Repo manifest format at another location, including a file inside .repo/manifests/. The flag changes location, not Git ownership or versioning. Report a missing or invalid selection with its path before command execution. A manifest-looking filename alone does not change ordinary Git workspace mode.

Delegate composition, source eligibility and repository ownership to the shared source-scope contract. Server consumers receive the selected location without additional project selectors. Inspection does not silently initialize, sync, fetch or switch checkouts.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Repo Manifest Selection](#cli-repo-manifest-selection)
---
