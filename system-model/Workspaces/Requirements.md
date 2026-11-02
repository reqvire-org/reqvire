# Elements

### Coordinated Workspace Branches

When a caller requests a new working composition from an explicit root base, the system SHALL prepare the requested root and component branches from that base's manifest and each component's own pin while preserving existing user checkouts.

#### Concept References
  * [Workspace Composition](../Thesaurus/Workspaces.md#workspace-composition)
  * [Commit Pin](../Thesaurus/Workspaces.md#commit-pin)

#### Metadata
  * type: requirement

#### Relations
  * specify: [Multi-repository Workspace Management and Orchestration](WorkspacesFeature.md#multi-repository-workspace-management-and-orchestration)
---

### Native Workspace Manifest Resolution

When a native workspace manifest is selected, the system SHALL resolve its repository declarations and exact commit pins into one authoritative composition, rejecting invalid ownership, identities or checkout locations.

#### Concept References
  * [Workspace Manifest](../Thesaurus/Workspaces.md#workspace-manifest)
  * [Repositories Directory](../Thesaurus/Workspaces.md#repositories-directory)
  * [Commit Pin](../Thesaurus/Workspaces.md#commit-pin)

#### Metadata
  * type: requirement

#### Relations
  * specify: [Multi-repository Workspace Management and Orchestration](WorkspacesFeature.md#multi-repository-workspace-management-and-orchestration)
---

### Repo Workspace Migration

When a caller requests migration from a Repo workspace, the system SHALL translate the selected XML repository declarations into a native manifest at the effective workspace root and prepare that composition through the shared synchronization operation.

#### Concept References
  * [Workspace Migration](../Thesaurus/Workspaces.md#workspace-migration)
  * [Workspace Manifest](../Thesaurus/Workspaces.md#workspace-manifest)

#### Metadata
  * type: requirement

#### Relations
  * specify: [Multi-repository Workspace Management and Orchestration](WorkspacesFeature.md#multi-repository-workspace-management-and-orchestration)
---

### Reproducible Workspace Composition

When a workspace composition is checkpointed or reconstructed, the system SHALL select its authoritative manifest and optional root sources from the same owning-repository revision, with component content selected by exact recorded commits.

#### Concept References
  * [Workspace Composition](../Thesaurus/Workspaces.md#workspace-composition)
  * [Commit Pin](../Thesaurus/Workspaces.md#commit-pin)

#### Metadata
  * type: requirement
  * owner: syseng
  * priority: high
  * risk: high
  * status: approved

#### Relations
  * specify: [Multi-repository Workspace Management and Orchestration](WorkspacesFeature.md#multi-repository-workspace-management-and-orchestration)
---

### Shared Workspace Management Boundary

The system SHALL centralize workspace identity, repository coordination and composition operations in one reusable workspace-management crate, exposing a common API to its consumers for single-repository and declared multi-repository workspaces.

#### Concept References
  * [Workspace Composition](../Thesaurus/Workspaces.md#workspace-composition)

#### Metadata
  * type: requirement

#### Relations
  * specify: [Multi-repository Workspace Management and Orchestration](WorkspacesFeature.md#multi-repository-workspace-management-and-orchestration)
---

### Workspace Baseline Composition Access

When a consumer selects an owning-repository baseline commit, the system SHALL provide its complete locally reconstructible composition and source provenance without changing live repositories or acquiring remote state.

#### Concept References
  * [Workspace Composition](../Thesaurus/Workspaces.md#workspace-composition)
  * [Workspace Manifest](../Thesaurus/Workspaces.md#workspace-manifest)
  * [Commit Pin](../Thesaurus/Workspaces.md#commit-pin)

#### Metadata
  * type: requirement

#### Relations
  * specify: [Multi-repository Workspace Management and Orchestration](WorkspacesFeature.md#multi-repository-workspace-management-and-orchestration)
---

### Workspace Dependency Registration

When a caller explicitly registers a dependent repository, the system SHALL prepare its declared checkout, resolve the requested revision to an exact commit and propose the corresponding native manifest declaration.

#### Concept References
  * [Workspace Manifest](../Thesaurus/Workspaces.md#workspace-manifest)
  * [Commit Pin](../Thesaurus/Workspaces.md#commit-pin)

#### Metadata
  * type: requirement

#### Relations
  * specify: [Multi-repository Workspace Management and Orchestration](WorkspacesFeature.md#multi-repository-workspace-management-and-orchestration)
---

### Workspace Dependency Removal

When a caller removes a declared dependency, the system SHALL remove its native manifest declaration while preserving the existing checkout, repository history and unrelated source state.

#### Concept References
  * [Workspace Composition](../Thesaurus/Workspaces.md#workspace-composition)

#### Metadata
  * type: requirement

#### Relations
  * specify: [Multi-repository Workspace Management and Orchestration](WorkspacesFeature.md#multi-repository-workspace-management-and-orchestration)
---

### Workspace Discovery

The system SHALL resolve the effective workspace and its owning repository through the shared workspace service, using one containing repository when no workspace manifest is selected.

#### Concept References
  * [Workspace Composition](../Thesaurus/Workspaces.md#workspace-composition)

#### Metadata
  * type: requirement

#### Relations
  * specify: [Multi-repository Workspace Management and Orchestration](WorkspacesFeature.md#multi-repository-workspace-management-and-orchestration)
---

### Workspace Operation Isolation and Recovery

While a workspace operation owns a composition, the system SHALL serialize conflicting state changes, preserve previously accepted state on rejected candidates and retain confirmed or uncertain repository outcomes for safe recovery.

#### Concept References
  * [Workspace Composition](../Thesaurus/Workspaces.md#workspace-composition)

#### Metadata
  * type: requirement

#### Relations
  * specify: [Multi-repository Workspace Management and Orchestration](WorkspacesFeature.md#multi-repository-workspace-management-and-orchestration)
---

### Workspace Pin Freeze

When a caller explicitly requests a composition freeze, the system SHALL preview or record the exact current committed component revisions as native manifest pins while preserving unrelated source and repository state.

#### Concept References
  * [Workspace Freeze](../Thesaurus/Workspaces.md#workspace-freeze)
  * [Commit Pin](../Thesaurus/Workspaces.md#commit-pin)

#### Metadata
  * type: requirement

#### Relations
  * specify: [Multi-repository Workspace Management and Orchestration](WorkspacesFeature.md#multi-repository-workspace-management-and-orchestration)
---

### Workspace Publication Orchestration

When a caller explicitly requests publication of a coordinated change, the system SHALL publish required component branches before the owning branch and create or reuse the linked repository review set while retaining confirmed outcomes on failure.

#### Concept References
  * [Workspace Composition](../Thesaurus/Workspaces.md#workspace-composition)

#### Metadata
  * type: requirement

#### Relations
  * specify: [Multi-repository Workspace Management and Orchestration](WorkspacesFeature.md#multi-repository-workspace-management-and-orchestration)
---

### Workspace Status Observation

When a caller requests workspace status, the system SHALL report the selected composition and observed repository state without modifying that composition or repository state.

#### Concept References
  * [Workspace Composition](../Thesaurus/Workspaces.md#workspace-composition)
  * [Commit Pin](../Thesaurus/Workspaces.md#commit-pin)

#### Metadata
  * type: requirement

#### Relations
  * specify: [Multi-repository Workspace Management and Orchestration](WorkspacesFeature.md#multi-repository-workspace-management-and-orchestration)
---

### Workspace Synchronization

When a caller requests synchronization, the system SHALL prepare the recorded component commits by default or explicitly advance them from configured refs when latest synchronization is requested, preserving unrelated user work.

#### Concept References
  * [Workspace Composition](../Thesaurus/Workspaces.md#workspace-composition)
  * [Commit Pin](../Thesaurus/Workspaces.md#commit-pin)
  * [Tracking Ref](../Thesaurus/Workspaces.md#tracking-ref)

#### Metadata
  * type: requirement

#### Relations
  * specify: [Multi-repository Workspace Management and Orchestration](WorkspacesFeature.md#multi-repository-workspace-management-and-orchestration)
---
