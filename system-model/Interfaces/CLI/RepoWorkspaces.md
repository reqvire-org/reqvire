# Elements

### CLI Repo Manifest Selection

When a caller explicitly selects a Repo manifest or starts at an initialized Repo workspace, the CLI SHALL pass that selection through the shared manifest-defined workspace scope before executing the selected command, while retaining existing single-repository command behavior.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Manifest-Defined Workspace Scope Specification](../../ModelStructure/RepoWorkspaces.md#manifest-defined-workspace-scope-specification)

#### Relations
  * derivedFrom: [Explicit Workspace Selection](Commands.md#explicit-workspace-selection)
---

### CLI Repo Manifest Selection Specification

Provide optional global --manifest <workspace-relative-file> selection for inspection commands, standalone MCP and embedded Explorer serving. Apply --workspace first, then resolve --manifest relative to the selected workspace. Report a missing/invalid explicit manifest before command execution with the selected path. An initialized Repo client at that root selects Repo mode without requiring the flag. A manifest-looking filename alone does not change an ordinary Git workspace's mode.

Delegate source resolution and eligibility to the bound source-scope contract. Server consumers receive the selected manifest/mode without adding project arguments to individual model tools. Command help advertises the option; the existing command names and ordinary Git invocation retain their semantics. Inspection does not silently initialize, sync, fetch or switch project checkouts; writable-context preparation remains an explicit worktree operation under its existing network policy.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Repo Manifest Selection](#cli-repo-manifest-selection)
---
