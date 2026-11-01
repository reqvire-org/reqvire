# Elements

### Manifest-Defined Workspace Scope

When a workspace is selected through a Repo manifest, the system SHALL resolve the manifest-owning workspace-root repository and its pinned component repositories as one workspace-relative model scope, preserving authoritative source ownership and selected revisions whether the root contains model elements or only the versioned manifest and configuration.

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

### Reproducible Workspace Composition

WHEN a manifest-defined composition is checkpointed or reconstructed, the system SHALL identify the authoritative source manifest and any root model content from the same workspace-root branch revision, with component content selected by that manifest's exact pins and Repo administration remaining internal derived state.

#### Metadata
  * type: requirement
  * owner: syseng
  * priority: high
  * risk: high
  * status: approved

#### Relations
  * derivedFrom: [Manifest-Defined Workspace Scope](#manifest-defined-workspace-scope)
  * satisfiedBy: [mcp_repo.rs](../../crates/reqvire-cli/src/mcp_repo.rs)
  * satisfiedBy: [git_commands.rs](../../crates/reqvire-core/src/git_commands.rs)
  * satisfiedBy: [repo_impact.rs](../../crates/reqvire-core/src/repo_impact.rs)
  * satisfiedBy: [repo_workspace.rs](../../crates/reqvire-core/src/repo_workspace.rs)
---

### Manifest-Defined Workspace Scope Specification

The manifest-owning Git repository and selected branch are the workspace authority. That owning checkout is the effective workspace and identifier root. It may contain model elements, or only the tracked source manifest and workspace configuration while the complete system model lives in pinned component repositories. Do not require a root model document, synthetic model element, or duplicated component model.

Repo's .repo/manifest.xml remains the default discovery entry. Resolve its selected source filename and repository identity to the tracked source manifest on the selected root branch, for example manifests/default.xml. The cached files under .repo/manifests are Repo administration, not another authored participant. Native discovery does not require a Reqvire --manifest override. An optional alternate file selection preserves the same authoritative ownership and must not be silently replaced by a cache.

Require Git, Python 3, and initialized local Google Repo metadata. The caller explicitly initializes and synchronizes the client before admission. Keep .repo administration and component checkout directories ignored by the root repository; track the authoritative source manifest and its includes as ordinary files in the root Git history. Establish source ownership using repository identity, selected source path and tracked membership; a matching basename or cached bytes alone is not proof. Reject missing, foreign, ambiguous or unversioned authoritative inputs with their source path. Do not invent a separately versioned manifest owner, branch or PR from Repo's internal checkout.

Read root-owned selecting XML and includes from the selected branch's sources, even when Repo's cached checkout has an older revision or another branch's pin bytes. Use Repo's existing include, extend-project, remote, alias and revision-precedence semantics without importing cached models. Required local overlays must have an unambiguous versioned source in the authoritative root history; an unresolved or unversioned overlay cannot supply a reproducible authoring checkpoint or comparison baseline. Reject submanifests.

Resolve project revision names to concrete locally available commits. Require unique, non-overlapping contained project paths, matching repository identity, and checkouts at their selected revisions. Reject unavailable objects, foreign checkouts, reserved paths, traversal and symlink escapes with the affected input/project/path. A selected-manifest failure must not fall back to generic multi-worktree discovery.

The root and declared projects contribute to one graph with workspace-relative identifiers and globally consistent cross-repository relations. Workspace .reqvireignore rules are global; each repository's Git ignores apply inside its ownership boundary. Root ignore rules must not hide declared component sources. Undeclared repositories, Repo metadata, cached manifest-repository model files and copied dependency/build models are excluded. CLI, MCP and Explorer share this scope, including a root with zero model elements.

Within each relation-validation pass resolve this source inventory at most once and reuse its successes or failures for all file-reference checks. New passes observe the current selection under the interface's normal admission policy; accepted MCP reads retain their captured canonical source and component provenance. Source discovery never creates branches or commits and never fetches or syncs.

#### Metadata
  * type: specification

#### Relations
  * define: [Manifest-Defined Workspace Scope](#manifest-defined-workspace-scope)
---

### Workspace Composition Checkpoint Specification

#### Details
- The source manifest and its versioned includes belong to the manifest-owning root repository and selected branch. .repo/manifest.xml discovers that source; the cached checkout, wrappers and config under .repo are implementation state rather than another authoring repository.
- The root may contain no model elements. Its committed source manifest and exact component pins are sufficient to define the complete product model held in declared repositories.
- One root commit supplies its optional root model content and authoritative source XML, including all versioned selecting inputs. Each declared component is selected by its own full commit ID. Do not record the root commit in its own tree or treat a cached copy's HEAD as another manifest revision.
- Do not require, create or manually advance .reqvire-composition.json as an association between the root and its own manifest. Existing association metadata must not override the selected branch's authoritative source files or turn a cached checkout into a separate owner. Missing canonical sources fail with source-specific preparation guidance.
- Commit changed component authoring branches first. Advance their pins in the tracked source manifest, preserving include composition and unchanged component pins, then commit those pin changes together with accepted root changes in one root commit. Unchanged components receive no commit.
- An explicit initial commit may lock an admitted mutable component selection even without model edits. A fully pinned unchanged source composition is a no-op; a root-only self-association is not a reason to create a commit.
- Accepted root source paths, indexes and expected HEAD follow the existing root commit transaction and reconciliation contract. A later root failure retains confirmed component commits and pending root changes. Retry reuses confirmed components; unknown component outcomes require operator repair, while recorded root attempts use existing root reconciliation.
- Branch creation uses the requested root base's tracked source manifest and prepares the requested named branch in each declared component at that base's own pin. Keep Repo administration derived from that composition without creating reqvire/manifests/<branch>, another manifest ref, or a manifest PR.
- External work commits component changes first, then updated source pins and related root changes on the same root feature branch. Clean validated branch.open admits that composition without a reload flag, manual self-association or commit inside Repo administration.
- Publish required component branches before the root branch. One root PR contains the root model/configuration and pin changes, linked to PRs for changed components. Repo administration has no repository_bases entry, publication outcome or PR of its own.
- Versioned root source selection remains authoritative through restart, reopen and comparison. Missing inputs/objects, conflicting identity, dirty authoring inputs and partial checkpoints fail without fetching, syncing, resetting external work or accepting cache bytes as committed source.

#### Metadata
  * type: specification

#### Relations
  * define: [Reproducible Workspace Composition](#reproducible-workspace-composition)
---
