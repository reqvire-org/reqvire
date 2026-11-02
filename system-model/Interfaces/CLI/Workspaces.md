# Elements

### CLI Repo Workspace Migration Specification

#### Details
Expose workspace migrate --from repo with optional --source FILE for an explicitly selected XML source; otherwise use the Repo client's selected manifest entry. --dry-run previews conversion and the shared sync plan without writes. Apply writes reqvire-workspace.toml at the effective root and invokes the same sync engine used by workspace sync. Report conversion/sync failures explicitly and preserve the original XML. This workspace-configuration migration is separate from the existing model-syntax migrate command and creates no commit or publication.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Repo Workspace Migration Command](#cli-repo-workspace-migration-command)
---

### CLI Workspace Command Dispatch Specification

#### Details
Expose workspace status, add, remove, sync, freeze and migrate as workspace preparation/management commands. Dispatch through the workspace crate before requiring model construction. There are no new CLI branch, commit, push or PR wrappers. Ordinary Git and gh remain available for those developer workflows.

Apply workspace selection before dispatch, render text and JSON results and report failures with nonzero exit status and shared per-repository outcomes. sync defaults to recorded pins; --latest selects configured refs and advances pins through the same operation. Do not expose a separate workspace update command or model CRUD through this surface.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Workspace Commands](#cli-workspace-commands)
---

### CLI Workspace Commands

When an explicit workspace-management command is invoked, the CLI SHALL dispatch it through the shared workspace API before requiring a complete system model, while preserving the supported model-authoring interface boundary.

#### Metadata
  * type: requirement

#### Contract References
  * [Workspace Management API Specification](../../Workspaces/Specifications.md#workspace-management-api-specification)

#### Relations
  * derivedFrom: [CLI Interface Structure](Commands.md#cli-interface-structure)
---

### CLI Repo Workspace Migration Command

When a caller requests Repo workspace migration through the CLI, the system SHALL convert the XML manifest to the native root manifest and invoke shared synchronization without requiring a valid model.

#### Metadata
  * type: requirement

#### Contract References
  * [Repo Workspace Migration Specification](../../Workspaces/Specifications.md#repo-workspace-migration-specification)

#### Relations
  * derivedFrom: [CLI Workspace Commands](#cli-workspace-commands)
---

### CLI Workspace Freeze Command

When a caller requests workspace freeze through the CLI, the system SHALL preview or apply the shared local pin-capture operation and display its exact source diff and blockers.

#### Metadata
  * type: requirement

#### Contract References
  * [Workspace Pin Freeze Specification](../../Workspaces/Specifications.md#workspace-pin-freeze-specification)

#### Relations
  * derivedFrom: [CLI Workspace Commands](#cli-workspace-commands)
---

### CLI Workspace Registration Command

When a caller registers a workspace dependency through the CLI, the system SHALL present the shared registration result and exact manifest pin without creating commits or publishing changes.

#### Metadata
  * type: requirement

#### Contract References
  * [Workspace Dependency Registration Specification](../../Workspaces/Specifications.md#workspace-dependency-registration-specification)

#### Relations
  * derivedFrom: [CLI Workspace Commands](#cli-workspace-commands)
---

### CLI Workspace Removal Command

When a caller removes a workspace dependency through the CLI, the system SHALL unregister its declaration through the shared service and preserve the existing checkout.

#### Metadata
  * type: requirement

#### Contract References
  * [Workspace Dependency Removal Specification](../../Workspaces/Specifications.md#workspace-dependency-removal-specification)

#### Relations
  * derivedFrom: [CLI Workspace Commands](#cli-workspace-commands)
---

### CLI Workspace Status Command

When a caller requests workspace status through the CLI, the system SHALL render the shared local composition and repository observations without altering the workspace.

#### Metadata
  * type: requirement

#### Contract References
  * [Workspace Status Specification](../../Workspaces/Specifications.md#workspace-status-specification)

#### Relations
  * derivedFrom: [CLI Workspace Commands](#cli-workspace-commands)
---

### CLI Workspace Synchronization Command

When a caller requests workspace synchronization through the CLI, the system SHALL prepare the recorded composition through the shared service and report confirmed or blocked repository outcomes.

#### Metadata
  * type: requirement

#### Contract References
  * [Workspace Synchronization Specification](../../Workspaces/Specifications.md#workspace-synchronization-specification)

#### Relations
  * derivedFrom: [CLI Workspace Commands](#cli-workspace-commands)
---

### CLI Workspace Freeze Specification

#### Details
Expose workspace freeze --dry-run for preview and workspace freeze for apply through the shared local pin-capture API. Record committed component HEADs as commit values while preserving configured refs. Root changes may remain pending for a later ordinary Git commit.

Report dirty components, identity errors, stale previews and conflicting ownership without bypassing them. Show exact old/new pin evidence in text/JSON. Do not fetch, switch branches, create commits or publish.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Workspace Freeze Command](#cli-workspace-freeze-command)
---

### CLI Workspace Registration Specification

#### Details
Accept workspace add URL --path PATH --ref REF. PATH is relative to the explicitly configured repositories_dir. Preserve the supplied tracking ref and its resolved exact commit in the new declaration, then use shared preparation/synchronization for the checkout. Never assume a checkout parent named repositories.

Return the declaration/preparation result as text or JSON and suggest only the specific checkout ignore entry. Do not commit or publish changes or overwrite unrelated files.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Workspace Registration Command](#cli-workspace-registration-command)
---

### CLI Workspace Removal Specification

#### Details
Expose workspace remove PATH, where PATH selects an existing manifest-relative dependency path. Invoke the shared removal operation and report the declaration diff in text or JSON. Preserve the checkout, Git state and unrelated source/ignore content; no automatic commit, push or deletion is implied.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Workspace Removal Command](#cli-workspace-removal-command)
---

### CLI Workspace Status Specification

#### Details
Expose workspace status with text and JSON representations of the shared observation. Include configured checkout locations, recorded pins versus current HEADs, missing/dirty/conflicting repositories and active ownership or recovery diagnostics. A single repository uses the same service contract without synthetic dependency declarations.

Do not fetch, repair, create branches, rewrite pins, commit or publish. Observation does not bypass another session's lock or admit its external drift.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Workspace Status Command](#cli-workspace-status-command)
---

### CLI Workspace Synchronization Specification

#### Details
Expose workspace sync and workspace sync --latest through the same shared operation. Default sync prepares existing exact pins; an initial ref-only configuration resolves and writes its first pins. --latest follows configured refs and advances commit values while preserving refs and unrelated source.

Clone or reuse checkouts and acquire missing objects only within this explicit operation. Preparation works for ordinary code-only workspaces before model startup. Use normal Git checks for dirty or conflicting work, preserve confirmed results on partial failure and expose text/JSON diagnostics. Do not create commits or publication operations.

#### Metadata
  * type: specification

#### Relations
  * define: [CLI Workspace Synchronization Command](#cli-workspace-synchronization-command)
---
