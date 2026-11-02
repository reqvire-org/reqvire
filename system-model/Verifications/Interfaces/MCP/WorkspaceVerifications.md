# Elements

### MCP Workspace Management Verification Objective

Verify typed workspace preparation and management over real local HTTP MCP transport, sharing the same Git/TOML outcome assertions as CLI while retaining validated startup, mutation enablement and accepted-model boundaries.

#### Metadata
  * type: verification-objective

#### Relations
  * derivedFrom: [MCP Protocol and Tool Verification Objective](MCPVerifications.md#mcp-protocol-and-tool-verification-objective)
---

### MCP Repo Workspace Migration Verification

From a normally validated admitted root context, call reqvire.workspace.migrate with from:repo and a scoped XML source. Preview preserves state; apply writes the root native TOML and uses shared sync. Preserve XML, checkout paths, exact pin/tracking ref and root HEAD; subsequent native sync succeeds. This does not test or authorize model-less MCP startup.

Executable entry: tests/run_tests.sh test-workspace-management; the focused Bash cases live in tests/test-workspace-management/cases.sh. Assertions are acceptance criteria and remain red until the native workspace runtime is implemented.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Workspace Management Verification Objective](#mcp-workspace-management-verification-objective)
  * satisfiedBy: [run_tests.sh](../../../../tests/run_tests.sh)
  * verify: [MCP Repo Workspace Migration](../../../Interfaces/MCP/Tools.md#mcp-repo-workspace-migration)
---

### MCP Workspace Freeze Verification

After externally committed component work, preview and apply reqvire.workspace.freeze. Show old/new pins, preserve source on preview, admit the complete valid changed composition on apply, preserve refs and repository HEADs, and repeat without changes. Dirty components must fail with explicit diagnostics and unchanged state.

Executable entry: tests/run_tests.sh test-workspace-management; the focused Bash cases live in tests/test-workspace-management/cases.sh. Assertions are acceptance criteria and remain red until the native workspace runtime is implemented.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Workspace Management Verification Objective](#mcp-workspace-management-verification-objective)
  * satisfiedBy: [run_tests.sh](../../../../tests/run_tests.sh)
  * verify: [MCP Workspace Pin Freeze](../../../Interfaces/MCP/Tools.md#mcp-workspace-pin-freeze)
---

### MCP Workspace Registration Verification

Discover reqvire.workspace.add in a mutation-enabled server, register a local repository at a configured nested destination, and assert its ref, exact pin, checkout HEAD and absence of an implicit root commit. Preserve unrelated root code.

Executable entry: tests/run_tests.sh test-workspace-management; the focused Bash cases live in tests/test-workspace-management/cases.sh. Assertions are acceptance criteria and remain red until the native workspace runtime is implemented.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Workspace Management Verification Objective](#mcp-workspace-management-verification-objective)
  * satisfiedBy: [run_tests.sh](../../../../tests/run_tests.sh)
  * verify: [MCP Workspace Dependency Registration](../../../Interfaces/MCP/Tools.md#mcp-workspace-dependency-registration)
---

### MCP Workspace Removal Verification

Call reqvire.workspace.remove for the declared path. Accepted status must show the changed empty composition while checkout code/history and root HEAD are preserved.

Executable entry: tests/run_tests.sh test-workspace-management; the focused Bash cases live in tests/test-workspace-management/cases.sh. Assertions are acceptance criteria and remain red until the native workspace runtime is implemented.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Workspace Management Verification Objective](#mcp-workspace-management-verification-objective)
  * satisfiedBy: [run_tests.sh](../../../../tests/run_tests.sh)
  * verify: [MCP Workspace Dependency Removal](../../../Interfaces/MCP/Tools.md#mcp-workspace-dependency-removal)
---

### MCP Workspace Synchronization Verification

Call reqvire.workspace.sync with latest omitted, then in a separate case with latest:true. Default selects pin A despite a newer ref tip B; latest selects B and advances only commit, preserving ref/comments and root HEAD. Accepted status must reflect each selected composition.

Executable entry: tests/run_tests.sh test-workspace-management; the focused Bash cases live in tests/test-workspace-management/cases.sh. Assertions are acceptance criteria and remain red until the native workspace runtime is implemented.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Workspace Management Verification Objective](#mcp-workspace-management-verification-objective)
  * satisfiedBy: [run_tests.sh](../../../../tests/run_tests.sh)
  * verify: [MCP Workspace Synchronization](../../../Interfaces/MCP/Tools.md#mcp-workspace-synchronization)
---

### MCP Workspace Tool Exposure Verification

Mutation-enabled discovery retains reqvire.workspace_status and exposes workspace.add/remove/sync/freeze/migrate, with no workspace.update tool. Read-only discovery retains status and hides those mutation tools. Status must expose native configured locations, selected pins, tracking refs and observed HEADs without mutation.

#### Details
Executable cases: mcp/discovery, mcp/status and mcp/readonly. Existing access-control, ownership and recovery regressions remain required; discovery alone does not demonstrate those protections.

Executable entry: tests/run_tests.sh test-workspace-management; the focused Bash cases live in tests/test-workspace-management/cases.sh. Assertions are acceptance criteria and remain red until the native workspace runtime is implemented.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Workspace Management Verification Objective](#mcp-workspace-management-verification-objective)
  * satisfiedBy: [run_tests.sh](../../../../tests/run_tests.sh)
  * verify: [MCP Workspace Session Tools](../../../Interfaces/MCP/Tools.md#mcp-workspace-session-tools)
---
