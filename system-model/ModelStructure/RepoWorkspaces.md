# Elements

### Manifest-Defined Workspace Scope

When a workspace is selected through a Repo manifest, the system SHALL resolve Central and its declared component repositories as one workspace-relative model scope, with each source retaining its owning repository and selected revision. It SHALL reject invalid or unavailable selections before accepting the model and preserve ordinary single-repository scope behavior.

#### Concept References
  * [Repo Workspace](../Thesaurus/Thesaurus.md#repo-workspace)
  * [Repo Manifest](../Thesaurus/Thesaurus.md#repo-manifest)

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Workspace Root Path Authority](ModelManagement.md#workspace-root-path-authority)
  * satisfiedBy: [config.rs](../../crates/reqvire-core/src/config.rs)
  * satisfiedBy: [repo_workspace.rs](../../crates/reqvire-core/src/repo_workspace.rs)
  * satisfiedBy: [workspace.rs](../../crates/reqvire-core/src/workspace.rs)
---

### Manifest-Defined Workspace Scope Specification

The owning Central Git worktree remains the effective workspace and identifier root. Its versioned Repo manifest selects declared project paths and revisions. Repo resolves the effective manifest, including supported composition; source scope includes Central itself and the resolved declared projects. A different manifest checkout or remote manifest branch cannot silently replace the selected Central worktree's versioned selection.

Require Git, Python 3 and initialized local Google Repo metadata at the Central root. The caller explicitly initializes and synchronizes the Repo client before admission. Track the selected manifest and its includes in Central; keep Repo administration and component checkouts out of Central's tracked files. Resolve Central-owned inputs with the initialized Repo implementation. Support includes, extend-project composition, relative fetch URLs, remote aliases and remote-level revision precedence; reject Repo submanifests. A checkout may name its remote differently when it has one unambiguous matching repository identity, including equivalent SSH URL spellings. A conflicting declared remote is rejected.

Resolve project revision names to concrete available local commits and retain the selected manifest inputs and per-project revision provenance with the loaded model. Project files remain Central-relative identifiers. Repository identity is ownership/provenance metadata, not an element namespace. Capture tracked inputs separately from Central and each declared repository so that Central ignoring the component checkout directory does not hide declared project sources. Workspace .reqvireignore exclusions remain global; a repository's Git ignores apply within its own ownership boundary. Exclude Repo metadata, copied dependency/build models and undeclared component repositories from this mode.

Require unique, non-overlapping project checkout paths, matching local Git project identity, available revisions and containment within the effective workspace. Reject unavailable projects/commits, duplicate project paths, path traversal, symlink escapes and foreign checkouts with the affected manifest/project/path. Do not silently fall back to ordinary Git scope after a selected manifest fails. Manifest-defined source discovery is common to CLI, MCP and Explorer; their existing admission and freshness policies remain interface-owned.

The source boundary supplies the declared project inventory, selected revisions and file-to-repository ownership to consumers. Worktree creation, private-checkout lifecycle, accepted-session mutation, Git commit sequencing and Explorer publication are owned by their respective interface requirements, rather than by source discovery.

```mermaid
flowchart TD
  B[Central branch] --> W[Central worktree]
  W --> M[Versioned Repo manifest]
  M --> R[Repo resolves project paths and revisions]
  W --> S[Central sources]
  R --> P[Declared component sources and owners]
  S --> A[One workspace-relative model]
  P --> A
```

Within each relation-validation pass, resolve the effective workspace scope at most once when file targets require it, and reuse that inventory and selected revision provenance for every file-reference check, including a failed scope resolution. Increasing the number of file references must not repeat Repo manifest resolution or project Git verification. Each new pass observes the current workspace selection through the existing admission boundary; accepted MCP snapshots retain their own request-local scope. Preserve per-reference existence, ownership and eligibility diagnostics.

#### Metadata
  * type: specification

#### Relations
  * define: [Manifest-Defined Workspace Scope](#manifest-defined-workspace-scope)
---
