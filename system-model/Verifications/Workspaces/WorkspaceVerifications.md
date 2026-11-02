# Elements

### Workspace Management Verification Objective

Verify native workspace discovery, configurable repository layout, shared service ownership, registration/removal, pinned and latest synchronization, exact pin freeze and Repo XML migration. Preserve existing branch, checkpoint, baseline and publication assertions during the format/crate refactor. Run the native workspace acceptance, retained composition regressions and shared crate tests against the same obligations.

#### Metadata
  * type: verification-objective
---

### Native Workspace Definition Verification

Verify workspace selection and native composition resolution. The Bash acceptance cases exercise configurable destinations, nested effective roots, Git-boundary discovery, absent/invalid manifests and code-only owners. Rust parser/path tests and the retained composition regressions cover the structural source and branch cases.

#### Details
- Resolve root and subdirectory manifests through the common workspace API; stop ancestor discovery at a Git repository boundary. An absent manifest selects one repository without creating a manifest; an invalid selected manifest fails instead of falling back.
- Exercise configurable repositories_dir values, nested component paths and an owning repository that already uses repositories for ordinary code. Assert the exact resolved checkout paths and preserve that unrelated code.
- Require exact commit pins, correct repository identities, tracked baseline sources and one owning branch. Reject unsupported versions, missing fields, duplicate/overlapping destinations, traversal, symlink escapes, administration paths and foreign checkouts with path-specific diagnostics.
- Verify both a model-bearing owner and an owner containing only manifest/configuration. Native resolution requires neither Repo initialization nor Python.
- Require all native declarations in one TOML manifest. Reject native include/fragment/overlay declarations explicitly, including when accompanied by otherwise valid repository entries; never accept a partial composition by ignoring unsupported configuration.
- Keep existing fixture outcome assertions; change only format-specific setup/inputs and add assertions for the new service contract.

Executable entry: tests/run_tests.sh test-workspace-management; the focused Bash cases live in tests/test-workspace-management/cases.sh. Run the shared crate tests with cargo test -p reqvire-workspace and retain the Bash suite's PASS outcomes.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Workspace Management Verification Objective](#workspace-management-verification-objective)
  * satisfiedBy: [run_tests.sh](../../../tests/run_tests.sh)
  * verify: [Native Workspace Manifest Resolution](../../Workspaces/Requirements.md#native-workspace-manifest-resolution)
  * verify: [Workspace Discovery](../../Workspaces/Requirements.md#workspace-discovery)
---

### Repo Workspace Migration Verification

Convert selected Repo XML to the native root TOML and synchronize through the ordinary workspace operation.

#### Details
- Use local XML remote/default/project declarations with a full component pin and upstream tracking ref. CLI discovers the default Repo entry; MCP can select an explicit source in an admitted context.
- Preview preserves the absence of native TOML and existing component HEAD. Apply writes TOML at the effective root, preserves XML and checkout paths, records the selected pin/ref, and creates no root commit. Subsequent native sync reproduces that pin.
- A conflicting native target must fail without overwriting it or changing the component HEAD.
- Shared crate XML tests cover include/default normalization, source/cache authority, unsupported input, repeated equivalent conversion and checkouts whose .git file points to independent backing storage. Migration is conversion plus shared sync; no separate backing-store conversion or cleanup workflow is required.

Executable entry: tests/run_tests.sh test-workspace-management; the focused Bash cases live in tests/test-workspace-management/cases.sh. Run the shared crate tests with cargo test -p reqvire-workspace and retain the Bash suite's PASS outcomes.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Workspace Management Verification Objective](#workspace-management-verification-objective)
  * satisfiedBy: [run_tests.sh](../../../tests/run_tests.sh)
  * verify: [Repo Workspace Migration](../../Workspaces/Requirements.md#repo-workspace-migration)
---

### Workspace Baseline Access Verification

Reconstruct complete source selections from a chosen root commit and its native pins using local objects. Cover an owner with no model, added/removed components, path moves, changed identities and unavailable removed-project objects. Assert each component uses its own pin and the baseline never substitutes current manifest bytes or root refs. Successful and failed access preserve source bytes, configuration, HEADs, refs and indexes and never contact remotes. Exercise the same API with one repository and no manifest.

#### Details
Run tests/run_tests.sh test-mcp-repo-workspaces for complete graphs, root/component provenance, removed and relocated projects, unavailable pins, legacy XML included sources, accepted pending snapshots and CLI/MCP parity. Run test-workspace-management's migration-impact case for an XML baseline compared with a native current composition. Shared crate tests assert byte-preserving offline tree reconstruction and repository-scoped symlinks.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Workspace Management Verification Objective](#workspace-management-verification-objective)
  * satisfiedBy: [run_tests.sh](../../../tests/run_tests.sh)
  * satisfiedBy: [check_change_impact.sh](../../../tests/test-mcp-repo-workspaces/check_change_impact.sh)
  * verify: [Workspace Baseline Composition Access](../../Workspaces/Requirements.md#workspace-baseline-composition-access)
---

### Workspace Composition Lifecycle Verification

Verify shared composition preparation, checkpoint ownership, recovery, deliberate external admission and coordinated publication using real local Git histories and native TOML fixtures.

#### Details
- Cover model-bearing and manifest-only owners, explicitly configured checkout parents and root-owned exact pins. Preserve existing regression assertions when replacing XML/Repo fixture setup.
- Prepare a requested branch set from a release base whose pins differ from startup main. Each component uses its own selected base pin; existing user checkouts and initial pins remain unchanged. Close/reopen and restart retain that composition.
- Require changed component commits before one root checkpoint containing new pins and accepted root changes. Unchanged components and fully pinned no-op checkpoints remain unchanged. No separate manifest authoring ref, PR or self-association is created.
- Inject root failures and unknown component outcomes. Retain confirmed component commits and pending root work, preserve recovery restrictions and reuse confirmed work without duplicate commits on retry.
- Prepare committed external component changes and corresponding root pins, then validate/admit the complete clean candidate. Rejection retains accepted reads; artifact-only refresh cannot admit a changed composition.
- Publish required component refs before the root and create/reuse one linked component/root PR set with actual targets. A pin-only root still needs its root PR. Preserve completed pushes/PRs on later failure and safe retry.
- Reproduce the published composition in a fresh checkout. Baseline reads preserve source bytes, configuration, HEADs, refs and indexes and never contact remotes. Keep operation failure distinct from a successfully checked negative outcome.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Workspace Management Verification Objective](#workspace-management-verification-objective)
  * satisfiedBy: [check_composition.sh](../../../tests/test-mcp-repo-workspaces/check_composition.sh)
  * satisfiedBy: [test.sh](../../../tests/test-mcp-repo-workspaces/test.sh)
  * verify: [Coordinated Workspace Branches](../../Workspaces/Requirements.md#coordinated-workspace-branches)
  * verify: [Reproducible Workspace Composition](../../Workspaces/Requirements.md#reproducible-workspace-composition)
  * verify: [Workspace Operation Isolation and Recovery](../../Workspaces/Requirements.md#workspace-operation-isolation-and-recovery)
  * verify: [Workspace Publication Orchestration](../../Workspaces/Requirements.md#workspace-publication-orchestration)
---

### Workspace Dependency Removal Verification

Remove the only dependency through both public adapters. Status must expose a valid empty native composition while the existing checkout, committed HEAD, code and root HEAD remain unchanged. Declaration removal must not delete the Git repository or require local object-retention machinery.

Executable entry: tests/run_tests.sh test-workspace-management; the focused Bash cases live in tests/test-workspace-management/cases.sh. Run the shared crate tests with cargo test -p reqvire-workspace and retain the Bash suite's PASS outcomes.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Workspace Management Verification Objective](#workspace-management-verification-objective)
  * satisfiedBy: [run_tests.sh](../../../tests/run_tests.sh)
  * verify: [Workspace Dependency Removal](../../Workspaces/Requirements.md#workspace-dependency-removal)
---

### Workspace Preparation Verification

Use local Git remotes and native manifests to verify registration and synchronization through CLI and MCP. A configured tracking ref remains distinct from the exact commit pin.

#### Details
- The add cases preserve an unrelated repositories directory, prepare a nested checkout under the configured parent and record the resolved commit plus tracking ref without committing the root.
- Default sync reproduces pin A while the configured remote ref points to B. Explicit latest sync selects B, changes only the pin and preserves the tracking ref and unrelated TOML. Ref-only initial CLI setup resolves and writes its first exact pin.
- CLI preparation succeeds without a model in the owner; model-aware MCP uses normal validated startup. Preparation never implicitly publishes.
- Reject dirty, foreign, divergent and conflicting-owner checkouts. Inject a later-project failure after an earlier project was prepared; preserve the original source pins, report confirmed progress and reuse the clean prepared checkouts on retry. MCP candidate validation rejection restores source bytes and checkouts while retaining accepted reads.

Executable entry: tests/run_tests.sh test-workspace-management; the focused Bash cases live in tests/test-workspace-management/cases.sh. Run the shared crate tests with cargo test -p reqvire-workspace and retain the Bash suite's PASS outcomes.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Workspace Management Verification Objective](#workspace-management-verification-objective)
  * satisfiedBy: [run_tests.sh](../../../tests/run_tests.sh)
  * verify: [Workspace Dependency Registration](../../Workspaces/Requirements.md#workspace-dependency-registration)
  * verify: [Workspace Synchronization](../../Workspaces/Requirements.md#workspace-synchronization)
---

### Workspace Service Boundary Inspection

Inspect Cargo dependencies and workspace call sites against the shared service contract. Confirm reqvire-workspace owns discovery, manifests, repository state, locks, pinning and orchestration; CLI, MCP and model code consume its API. Check that the crate depends on neither CLI nor the model engine, validation is handed across an explicit boundary, and one repository uses the same API without a synthetic manifest. Flag duplicate consumer-side Git/workspace coordination rather than accepting a facade over it.

#### Details
This inspection defines the architecture acceptance checks for implementation review.

#### Metadata
  * type: inspection-verification

#### Relations
  * derivedFrom: [Workspace Management Verification Objective](#workspace-management-verification-objective)
  * verify: [Shared Workspace Management Boundary](../../Workspaces/Requirements.md#shared-workspace-management-boundary)
---

### Workspace Status and Freeze Verification

Verify status and local pin capture with the same Bash assertions across CLI and MCP.

#### Details
- Status exposes configured parent, workspace-relative project paths, tracking refs, recorded pins and observed HEADs, preserving manifest bytes and root HEAD.
- After an external component commit, preview contains old/new pin evidence without writes. Apply changes only commit, preserves ref/comments and both repository HEADs, and repeating freeze is a no-op.
- Dirty-component freeze fails with an explicit diagnostic and preserves source bytes and HEAD. Require actual operation discovery before testing rejection so an unavailable operation cannot pass a negative test.
- Exercise shared ownership locks, invalid foreign/project boundaries and offline baseline reads in the crate and retained composition regressions. Nested-root MCP synchronization, checkpoint and branch close/reopen retain Git-root path prefixes without creating another manifest.

Executable entry: tests/run_tests.sh test-workspace-management; the focused Bash cases live in tests/test-workspace-management/cases.sh. Run the shared crate tests with cargo test -p reqvire-workspace and retain the Bash suite's PASS outcomes.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Workspace Management Verification Objective](#workspace-management-verification-objective)
  * satisfiedBy: [run_tests.sh](../../../tests/run_tests.sh)
  * verify: [Workspace Pin Freeze](../../Workspaces/Requirements.md#workspace-pin-freeze)
  * verify: [Workspace Status Observation](../../Workspaces/Requirements.md#workspace-status-observation)
---
