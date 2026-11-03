# Elements

### Native Workspace Manifest Specification

#### Details
The native source is reqvire-workspace.toml, with version = 1, required repositories_dir and repository declarations. The repositories array may be empty during setup. Each entry has path and url, a configured ref when updates are supported, and commit for its exact recorded selection. A ref-only initial declaration may be prepared by explicit sync; model admission and reproducible baseline access require full commit IDs. An entry with neither a commit nor a ref is invalid.

ref is the configured source for explicit latest synchronization, not the current working branch or an implicit PR target. Preserve it when committing or freezing feature work. Default sync uses commit when present; latest synchronization resolves ref and advances commit. A commit-only entry is valid for pinned sync but latest synchronization must report the missing configured ref rather than guess main.

repositories_dir is an explicitly configured relative checkout parent, including . for the workspace root. Resolve destinations as manifest_directory / repositories_dir / repository.path. Require contained, unique, non-overlapping component paths, matching identities and no collisions with root-owned files, traversal, Git administration or symlink escapes. Preserve portable canonical-path behavior.

Use one native manifest containing all repository declarations. Native TOML includes, fragments and overlays are unsupported and must be rejected explicitly. An optional project name preserves an existing stable display identity; URL and checkout path still determine repository identity and ownership.

#### Metadata
  * type: specification

#### Relations
  * define: [Native Workspace Manifest Resolution](Requirements.md#native-workspace-manifest-resolution)
---

### Repo Workspace Migration Specification

Migration resolves Repo's active entry independently of legacy baseline reconstruction. The default .repo/manifest.xml wrapper selects XML under .repo/manifests/; includes resolve within that manifest checkout. Support Repo's symlink entry within the workspace and an explicit root-owned XML source. A root file with the same name must not shadow the active Repo selection; importing that different source requires an explicit source selection. Preserve the source XML, selector and Git backing metadata. Reject missing or escaping selected sources and include cycles without substituting a different manifest.

This migration rule does not relax baseline provenance. Comparisons against older compositions continue to read versioned root-owned XML from the selected baseline tree and never substitute a live Repo manifest checkout.

#### Details
Read the selected Repo XML manifest and translate its effective repository declarations to reqvire-workspace.toml at the effective workspace root, then invoke the normal workspace synchronization implementation. Reuse shared parsing, identity and synchronization services; do not create a second checkout engine.

Preserve repository URLs, resolved workspace-relative checkout locations, configured tracking refs and exact selected revisions. For revision declarations that are branch/tag names, retain the tracking ref and resolve the initial exact commit through the shared preparation path. Do not guess an unrelated main branch for a commit-only declaration. Preserve include/default/remote declarations by resolving them into the equivalent native inventory; unsupported or ambiguous inputs fail explicitly rather than silently omitting repositories.

Write an explicit repositories_dir. Use . when preserving existing paths requires the workspace root as the checkout parent; per-repository paths remain nonempty and contained. Conversion does not move repositories to a hardcoded directory. Existing matching Git checkouts are reused by normal sync, including their ordinary Git-directory links. Migration performs no special Git backing-store conversion and does not delete Repo metadata or source XML.

Preview reports the generated native manifest and synchronization plan without writing source files or switching checkouts. Apply writes the native manifest then runs the shared sync operation. A conflicting existing native manifest is not overwritten; equivalent repeated conversion is a no-op apart from normal requested synchronization. Preserve source XML and unrelated user files. Report source paths and confirmed preparation outcomes if synchronization later fails; do not claim that failed synchronization completed migration.

This operation converts workspace configuration, not model element syntax. It does not create commits, publish branches or open PRs. Normal native loading uses the resulting TOML manifest. Legacy-format compatibility for older baselines remains a shared workspace-reader concern, separate from this conversion flow.

#### Metadata
  * type: specification

#### Relations
  * define: [Repo Workspace Migration](Requirements.md#repo-workspace-migration)
---

### Workspace Baseline Composition Specification

#### Details
Read the selected owning repository tree's native manifest and exact pins, together with its optional root sources. Resolve component sources from each recorded repository and commit at that composition's own workspace-relative paths. The owner may contain no model sources. Never resolve a component baseline by applying the root's Git ref independently to it.

Support the migration PR itself: if the selected baseline predates TOML, resolve its versioned root-owned Repo XML and includes through the workspace crate's legacy reader. Obtain selecting paths from the baseline tree, never substitute current XML or require a second Git baseline flag. The current composition remains native TOML. This baseline reader does not restore implicit live XML loading or require Repo/Python.

Build the baseline inventory independently of the current inventory so additions, removals, identity changes and path relocations are represented. Reuse retained local objects for removed/moved components. Missing source history, unsupported formats, invalid paths, foreign identity, mutable pins or unavailable commits fail with their source/repository/path rather than producing a partial baseline.

Return root revision, tracked manifest path, exact component commits, identities and ownership mappings through the common service API. Consumers own graph construction, comparison and report rendering. Review-baseline selection is a consumer concern; this contract supplies the composition for the selected commit.

Use captured inputs or isolated temporary materialization. Preserve source bytes, refs, HEADs, indexes, configuration, pins and accepted state. Never fetch, synchronize or consult live manifest bytes in place of the selected historical tree.

#### Metadata
  * type: specification

#### Relations
  * define: [Workspace Baseline Composition Access](Requirements.md#workspace-baseline-composition-access)
---

### Workspace Branch Coordination Specification

#### Details
Resolve the explicitly selected root base once. Its tracked native manifest and configured checkout locations select the component inventory and each exact base pin. Do not substitute startup branch sources, current checkout pins or one common Git ref for component commits.

Create the requested named working branch in the owner and each participating component at its own selected commit, using private worktrees where needed to preserve user checkouts. Matching names identify a coordinated branch set; each branch still belongs to its own repository. Report conflicting existing branches instead of resetting them.

Creation alone does not advance pins or create commits. Single-repository operation has the same branch lifecycle with no component participants. Preserve branch refs/commits when closing disposable clean contexts, and retain repository objects required to reopen/reconstruct compositions. Locks and recovery outcomes follow the shared operation boundary.

#### Metadata
  * type: specification

#### Relations
  * define: [Coordinated Workspace Branches](Requirements.md#coordinated-workspace-branches)
---

### Workspace Composition Checkpoint Specification

#### Details
The owning repository and selected branch track the native manifest and any root model/configuration changes. One root revision supplies that manifest and its exact component pins, including when all model sources are in components. Do not write a self-pin, separate manifest-owner revision, authoring branch, PR or root-to-manifest association.

Commit changed component authoring branches first. Compute their confirmed full commit IDs, update the corresponding TOML pins in the single native manifest through the workspace service, then commit those pin changes with accepted root changes in one root checkpoint. Preserve unrelated declarations and unchanged pins; unchanged components receive no commit. A fully pinned unchanged composition is a no-op.

Guard expected repository identities, branch tips, source bytes, indexes and accepted pending changes before applying. The caller supplies the accepted change set and validates candidates through the service boundary. Do not include unrelated staged/disk edits or silently adopt external commits.

A later root failure retains confirmed component commits and pending root pin changes. Retry reuses confirmed work; unknown component outcomes require operator repair, while recorded root attempts retain their existing verified reconciliation behavior. Cross-repository operations must not be reported as an atomic transaction.

The root may contain only the manifest and configuration. Commits remain local until an explicit publication request. Source selection and checkpoint reconstruction do not fetch or synchronize repositories.

#### Metadata
  * type: specification

#### Relations
  * define: [Reproducible Workspace Composition](Requirements.md#reproducible-workspace-composition)
---

### Workspace Dependency Registration Specification

#### Details
Require an explicit repository URL, component path relative to the configured repositories_dir and source ref. Validate identity and destinations through the shared workspace API. Resolve the requested ref and record both its configured tracking selection and the exact initial commit; never lose ref after freezing or checkpointing feature work.

Clone or reuse a matching checkout through shared synchronization/preparation. Initial setup explicitly writes a version 1 native manifest with repositories_dir before add. Registration requires that existing configuration and never invents a default checkout parent; a separate init command or creation option is unnecessary. Return the manifest diff and preparation outcomes. Preserve unrelated source, user work and ignore content. Suggest only the declared checkout's Git-root-relative ignore entry. Do not commit, push or create a PR.

#### Metadata
  * type: specification

#### Relations
  * define: [Workspace Dependency Registration](Requirements.md#workspace-dependency-registration)
---

### Workspace Dependency Removal Specification

#### Details
Select one existing declaration by its manifest-relative component path and remove that declaration through the shared workspace API. Preserve repositories_dir, other declarations, tracking refs, pins and unrelated comments/configuration. Removing the final declaration leaves a valid empty native composition.

Removal unregisters the dependency; it does not delete the checkout or its Git history, commit changes, push or create a PR. Existing specific ignore entries remain intact. There is no mandatory local object-retention or backing-store conversion step. A missing declaration reports its path without changing the source. Normal shared ownership and candidate-validation guards apply.

#### Metadata
  * type: specification

#### Relations
  * define: [Workspace Dependency Removal](Requirements.md#workspace-dependency-removal)
---

### Workspace Discovery Specification

#### Details
An explicitly supplied workspace directory selects the effective scope. Otherwise search from the invocation directory for the nearest reqvire-workspace.toml in its ancestor chain, stopping at the containing Git repository boundary. Do not cross into a containing composition from a dependent repository implicitly.

The manifest directory is the effective workspace root. Its containing Git repository and selected branch own the composition, even when that directory is below the Git root. Without a manifest, retain the selected directory's normal single-repository scope and report the containing repository through the same workspace API. Do not synthesize a manifest or infer a composition from arbitrary nested repositories. If no eligible owning repository exists, report the selected path.

A present invalid or unreadable manifest fails with its path; discovery must not downgrade it to an ordinary repository. Discovery observes local state only and does not clone, fetch, switch branches, write pins or create commits.

#### Metadata
  * type: specification

#### Relations
  * define: [Workspace Discovery](Requirements.md#workspace-discovery)
---

### Workspace Management API Specification

#### Details
The reqvire-workspace crate is the sole owner of discovery, manifest resolution, repository identities, status, add/remove, sync with pinned/latest policy, freeze, Repo-to-native conversion, branch coordination, checkpointing, baseline access, locks/recovery and publication ordering. It exposes transport-neutral descriptions, plans/diffs and per-repository outcomes, including the single-repository default without a synthetic manifest.

CLI and protocol adapters parse arguments, enforce permissions and render results. They do not maintain separate parsers, pin rewriters, Git-directory converters, synchronization engines or repository journals. The crate may reuse Git/GitHub tools or libraries behind its API, starting with existing shared Git execution and batched object readers.

The crate depends on neither CLI nor the model engine. Model parsing/validation and graph/report operations are consumers. Integrate candidate validation and admission through an explicit hook so model-aware consumers preserve accepted views on rejection. Ordinary workspace preparation does not require model content.

CLI exposes preparation and management only: status, add, remove, sync, freeze and migration. Branch lifecycle, coordinated commit/push and PR APIs remain available to existing MCP consumers; this work adds no CLI branch, commit, push or PR wrappers. Keep existing public MCP names and comparison payloads unless a separate contract change explicitly requires otherwise.

#### Metadata
  * type: specification

#### Relations
  * define: [Shared Workspace Management Boundary](Requirements.md#shared-workspace-management-boundary)
---

### Workspace Operation Isolation Specification

#### Details
Use one ownership/locking implementation across all callers, including external freeze and repository preparation. Check expected root branch/HEAD, manifest inputs, component identities, pins and operation state before writes and again when applying a previewed plan. Do not admit external drift implicitly.

Keep accepted composition provenance and any candidate separate. A model-validation consumer must validate the complete candidate before admission; rejection preserves accepted reads. Recovery restrictions must not turn a partial candidate into accepted source state.

Cross-repository Git operations are not atomic. Record per-repository outcomes and retain confirmed commits/checkouts after later failure. Reuse confirmed work on retry. An unknown component ref outcome blocks further writes/publication until operator repair; recorded root attempts use the existing verified reconciliation boundary.

Bound process execution and preserve cancellation/unknown-outcome distinctions. Do not delete user work, reset conflicting refs or weaken ownership merely to make a retry succeed.

#### Metadata
  * type: specification

#### Relations
  * define: [Workspace Operation Isolation and Recovery](Requirements.md#workspace-operation-isolation-and-recovery)
---

### Workspace Pin Freeze Specification

#### Details
Resolve current committed component HEADs locally and verify identities and ownership. Dirty component work blocks apply because it is not represented by a commit pin; root changes may remain pending for a later root commit.

Preview returns deterministic old/new pin evidence without writes. Apply rechecks source, HEADs and locks, then changes commit values only in the single native manifest. Expose its deterministic old/new pin diff. Preserve configured refs, checkout layout, unrelated declarations/comments and unchanged pins. An unchanged composition is a no-op.

Freeze does not fetch, switch branches, commit edits or publish. A standalone caller cannot bypass a live owner's lock. An owning model-aware caller may explicitly prepare committed external state through candidate validation and admission, provided no accepted pending edits or unresolved recovery are overwritten. Rejection preserves accepted reads and original source state.

#### Metadata
  * type: specification

#### Relations
  * define: [Workspace Pin Freeze](Requirements.md#workspace-pin-freeze)
---

### Workspace Publication Coordination Specification

#### Details
Require explicit publication authorization and the actual target for the owner and each changed component. Publish the pinned component commits before publishing the root pins that reference them. A coordinated PR request creates/reuses changed-component PRs and one owning-repository PR, linking the set without creating administrative publication participants.

Use repository identity, selected branch, base and confirmed operation outcomes to route and retry publication. Do not fan out comments or other communication unless explicitly requested. No-op components receive no manufactured commit or PR.

Retain confirmed pushes/PRs when later operations fail, report the incomplete set and reuse prior outcomes on retry. After squash/rebase changes a final component commit, update and review the corresponding root pin before the root merges. Publication does not silently advance source pins or adopt external edits.

#### Metadata
  * type: specification

#### Relations
  * define: [Workspace Publication Orchestration](Requirements.md#workspace-publication-orchestration)
---

### Workspace Status Specification

#### Details
Report the effective root, owning repository/branch, native selection, configured checkout parent and declared components with their configured refs, recorded commits, observed HEADs, branch names, dirty/missing state and diagnostics. Keep accepted selection and live observations distinguishable.

The CLI JSON projection includes workspace_root, manifest, repositories_dir and projects. Each project exposes its workspace-relative path, ref when configured, revision/commit for the selected exact pin, and head for the observed commit when available, together with existing identity/status data. MCP preserves existing workspace_status and context fields, embedding the same project facts in context.projects and adding ref/observed-head information without renaming existing fields.

A single repository reports an empty project inventory through the same service. Status works without a model and performs no fetch, branch/pin change, commit, publication or implicit admission.

#### Metadata
  * type: specification

#### Relations
  * define: [Workspace Status Observation](Requirements.md#workspace-status-observation)
---

### Workspace Synchronization Specification

#### Details
Prepare the declared composition through one synchronization operation. By default use each recorded commit; clone missing repositories and acquire required missing objects without advancing existing pins. For an initial ref-only declaration, resolve its configured ref and record the first exact commit. With latest explicitly enabled, resolve configured refs, prepare the resulting commits and write their exact pins while preserving refs and unrelated manifest content. Pin writes target the single tracked native manifest. Require refs for every selected latest update before applying source changes.

Use existing Git operations and shared repository identity/path checks. Reuse matching checkouts. For a named owning branch, prepare matching named component working branches at their recorded selections through the shared branch machinery; reuse matching existing branches, allow only safe fast-forward movement and report conflicting branch tips rather than resetting them. For a detached CLI/CI owner, prepare detached component checkouts and do not manufacture an authoring branch. MCP retains its named-owner admission requirement. Preserve local work. Do not reset dirty or diverged user branches or implement an automatic merge/rebase engine. Report ordinary Git blockers and confirmed per-repository outcomes for safe retry.

Preparation works for ordinary code-only workspaces before a valid model exists. Consumers with an accepted model validate a candidate through the shared admission hook before replacing that view. Default/latest sync never creates commits, pushes, opens PRs or silently deletes removed checkouts. Normal source discovery and change impact analysis do not synchronize implicitly.

#### Metadata
  * type: specification

#### Relations
  * define: [Workspace Synchronization](Requirements.md#workspace-synchronization)
---
