# Elements

### CLI Workspace Management Verification Objective

Verify native workspace selection and explicit CLI status, add, remove, sync, sync --latest, freeze and Repo migration. The CLI scope is workspace preparation and management; existing MCP branch/commit/publication behavior remains in its own verification scope. Expected acceptance outcomes remain PASS.

#### Metadata
  * type: verification-objective
---

### CLI Native Workspace Selection Verification

Run explicit --workspace and bounded ancestor discovery in a subdirectory workspace with a configurable parent. Both expose the same workspace paths and pins. Running inside a component selects that single repository. No manifest yields empty projects without creating TOML; an unsupported native version fails without fallback or source changes.

#### Details
Executable cases: cli/selection, cli/single, cli/invalid-manifest, plus configured-path assertions in cli/add. Model graph/source membership equivalence remains covered by the owning scope regressions with native fixtures.

Executable entry: tests/run_tests.sh test-workspace-management; the focused Bash cases live in tests/test-workspace-management/cases.sh.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [CLI Workspace Management Verification Objective](#cli-workspace-management-verification-objective)
  * satisfiedBy: [run_tests.sh](../../../../tests/run_tests.sh)
  * verify: [CLI Workspace Manifest Selection](../../../Interfaces/CLI/WorkspaceSelection.md#cli-workspace-manifest-selection)
---

### CLI Repo Workspace Migration Verification

Run workspace migrate --from repo through the default Repo entry, preview first, then apply. Native TOML appears at the effective root, selected URLs/paths/ref/pin are preserved, XML stays unchanged and normal sync succeeds without a root commit. Reject a conflicting native file without replacing it.

Executable entry: tests/run_tests.sh test-workspace-management; the focused Bash cases live in tests/test-workspace-management/cases.sh.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [CLI Workspace Management Verification Objective](#cli-workspace-management-verification-objective)
  * satisfiedBy: [run_tests.sh](../../../../tests/run_tests.sh)
  * verify: [CLI Repo Workspace Migration Command](../../../Interfaces/CLI/Workspaces.md#cli-repo-workspace-migration-command)
---

### CLI Workspace Freeze Verification

Commit external component work, then run workspace freeze --dry-run and workspace freeze. Preview exposes old/new pins without writes; apply changes only the exact commit, preserving ref/comments and both HEADs. A repeated freeze is a no-op. Dirty component work must fail with a dirty/uncommitted diagnostic and unchanged manifest/HEAD.

#### Details
Executable cases: cli/freeze and cli/dirty-freeze. Shared crate tests must additionally cover stale candidates, conflicting ownership and foreign repository identity.

Executable entry: tests/run_tests.sh test-workspace-management; the focused Bash cases live in tests/test-workspace-management/cases.sh.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [CLI Workspace Management Verification Objective](#cli-workspace-management-verification-objective)
  * satisfiedBy: [run_tests.sh](../../../../tests/run_tests.sh)
  * verify: [CLI Workspace Freeze Command](../../../Interfaces/CLI/Workspaces.md#cli-workspace-freeze-command)
---

### CLI Workspace Registration Verification

Run workspace add URL --path nested/docs --ref refs/heads/main with local Git remotes. The configured parent determines the checkout path; status records both the configured ref and exact resolved commit. Preserve ordinary code in an existing root repositories directory and create no root commit.

#### Details
Executable case: cli/add. Shared registration validation retains its separate path/identity/failure acceptance obligations.

Executable entry: tests/run_tests.sh test-workspace-management; the focused Bash cases live in tests/test-workspace-management/cases.sh.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [CLI Workspace Management Verification Objective](#cli-workspace-management-verification-objective)
  * satisfiedBy: [run_tests.sh](../../../../tests/run_tests.sh)
  * verify: [CLI Workspace Registration Command](../../../Interfaces/CLI/Workspaces.md#cli-workspace-registration-command)
---

### CLI Workspace Removal Verification

Run workspace remove website. Its declaration disappears, leaving a valid empty native composition. The checkout's code and committed HEAD remain present and unchanged; no root commit is created.

Executable entry: tests/run_tests.sh test-workspace-management; the focused Bash cases live in tests/test-workspace-management/cases.sh.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [CLI Workspace Management Verification Objective](#cli-workspace-management-verification-objective)
  * satisfiedBy: [run_tests.sh](../../../../tests/run_tests.sh)
  * verify: [CLI Workspace Removal Command](../../../Interfaces/CLI/Workspaces.md#cli-workspace-removal-command)
---

### CLI Workspace Status Verification

Run workspace status --json on a declared composition and a single repository. Report the configured parent, workspace-relative project path, tracking ref, selected pin and observed HEAD. Preserve source bytes and root HEAD; a single repository has no synthesized manifest or projects.

#### Details
Executable cases: cli/status and cli/single. Additional text rendering and dirty/missing/ownership diagnostic checks remain part of implementation coverage.

Executable entry: tests/run_tests.sh test-workspace-management; the focused Bash cases live in tests/test-workspace-management/cases.sh.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [CLI Workspace Management Verification Objective](#cli-workspace-management-verification-objective)
  * satisfiedBy: [run_tests.sh](../../../../tests/run_tests.sh)
  * verify: [CLI Workspace Status Command](../../../Interfaces/CLI/Workspaces.md#cli-workspace-status-command)
---

### CLI Workspace Synchronization Verification

Run workspace sync without a model in the owner and with missing checkouts. Reproduce recorded pin A even when the configured remote ref already selects B, preserving manifest bytes and root HEAD. Run workspace sync --latest to select B and advance only commit while retaining ref and unrelated TOML. A ref-only initial declaration must resolve and record its first pin.

#### Details
Executable cases: cli/sync, cli/latest and cli/initial-ref. No separate workspace update command is needed. Dirty/conflicting destinations and retained partial outcomes remain shared implementation/regression obligations.

Executable entry: tests/run_tests.sh test-workspace-management; the focused Bash cases live in tests/test-workspace-management/cases.sh. Assertions are acceptance criteria and remain red until the native workspace runtime is implemented.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [CLI Workspace Management Verification Objective](#cli-workspace-management-verification-objective)
  * satisfiedBy: [run_tests.sh](../../../../tests/run_tests.sh)
  * verify: [CLI Workspace Synchronization Command](../../../Interfaces/CLI/Workspaces.md#cli-workspace-synchronization-command)
---
