# Elements

### CLI Workspace Manifest Selection

When the workspace service selects a native composition, the CLI SHALL pass its resolved workspace description to the requested operation while preserving the service's single-repository default.

#### Metadata
  * type: requirement

#### Contract References
  * [Workspace Discovery Specification](../../Workspaces/Specifications.md#workspace-discovery-specification)
  * [Workspace Management API Specification](../../Workspaces/Specifications.md#workspace-management-api-specification)

#### Relations
  * derivedFrom: [Explicit Workspace Selection](Commands.md#explicit-workspace-selection)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * satisfiedBy: [main.rs](../../../crates/reqvire-cli/src/main.rs)
---

### CLI Workspace Manifest Selection Specification

Resolve global --workspace before dispatch and pass the selected directory to the shared workspace service. Native discovery uses reqvire-workspace.toml in that effective directory or the service's bounded ancestor search. The manifest's repositories_dir and component paths determine checkout locations; the CLI does not assume a parent directory name.

Use the same resolved description for inspection, workspace preparation, MCP startup and embedded serving. The native flow needs no routine manifest-location override or Google Repo initialization. Invalid selected sources fail with the service's path-specific diagnostics; do not fall back to another composition.

Discovery and inspection do not initialize repositories, fetch, switch branches or advance pins. Those effects require the explicitly selected workspace-management command.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Workspace Manifest Selection](#cli-workspace-manifest-selection)
---
