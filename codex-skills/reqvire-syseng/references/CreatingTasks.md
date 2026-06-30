# Creating Implementation Tasks from Capability-Scoped Changes

This workflow bridges the gap between Reqvire's semantic engineering graph and implementation by analyzing what changed and generating comprehensive task lists.

**Key principle**: Every implementation task maintains traceability from capability meaning to requirement obligations, implementation, and verification evidence.

**For common commands** (change-impact, collect, search, validate), see [SKILL.md Command Reference](../SKILL.md#command-reference). For validation workflow, see [SKILL.md Validation & Quality Checklist](../SKILL.md#validation--quality-checklist).

## When to Use This Workflow

- Generating implementation plans from capability-scoped requirement changes
- Understanding what capabilities, requirements, contracts, or verifications changed on a capability branch
- Creating task breakdowns for developers
- Planning implementation work with full traceability
- Analyzing change impact before implementing

## Core Workflow

### Step 1: Analyze Change Impact

Understand how changes to requirements propagate through the model:

```bash
# Run change impact analysis comparing to base branch
BASE_COMMIT=$(git merge-base main HEAD)
reqvire change-impact --git-commit=$BASE_COMMIT

# Save to JSON for programmatic analysis
reqvire change-impact --git-commit=$BASE_COMMIT --json --output /tmp/impact.json

# Compare against specific commit
reqvire change-impact --git-commit=<commit-hash>

# Compare against previous commit
reqvire change-impact --git-commit=HEAD~1
```

`--git-commit` materializes the base snapshot from the current eligible Git worktree inside the effective workspace. It is not a multi-repository commit selector; paths and identifiers remain workspace-root-relative.

The change-impact command identifies:
- `added[]` - New capabilities/requirements/contracts/verifications
- `changed[]` - Changed capabilities/requirements/contracts/verifications
- `removed[]` - Removed elements
- `relocated[]` - Relocated elements (same name, different path)
- `impact_scope[]` - Per-branch scope roots: common parent capabilities or requirements covering all impacted elements. Use this for a high-level summary of affected model areas
- `invalidated_verifications[]` - Verifications that need re-review

**Change Propagation Rules:**
- **Parent → Child**: Parent capability or requirement changes propagate to derived children
- **Capability → Requirement**: Capability changes may require review of specifying requirements and requirement verification coverage
- **Requirement → Verification**: Requirement changes invalidate verifications
- **Requirement → Implementation**: May need implementation updates
- **Verification changes**: Generally don't propagate upward

### Step 1.5: Enumerate Covered Elements from Impact Scope

For each entry in `impact_scope[]`, use downstream collect to find all covered children:

```bash
# Get all descendants under each scope root
reqvire collect "<scope-root-name>" --direction DOWNSTREAM --json --output /tmp/scope_<name>.json
```

This ensures no elements are missed — `impact_scope` entries are common parents that may cover multiple added/changed children.

### Step 2: Gather Full Capability and Requirement Context

For each changed capability or requirement (from `added[]`, `changed[]`, or enumerated via downstream collect), gather upstream context:

```bash
# Get full ancestor chain with contract_bindings (upstream - default)
reqvire collect "<element-name>" --json --output /tmp/context_<element-id>.json

# Also save human-readable format for reference
reqvire collect "<element-name>" > /tmp/trace_context_<element-id>.md

# Get all descendants under a capability or requirement (downstream)
reqvire collect "<element-name>" --direction DOWNSTREAM --json --output /tmp/context_<element-id>_tree.json

# Get direct element details
reqvire search --filter-id="<element-id>" --json

# Governance-focused views for routing and prioritization
reqvire search --filter-owner="<owner-regex>" --json
reqvire search --filter-priority="high,critical" --json
reqvire search --filter-risk="high,critical" --json
```

**Why use `reqvire collect` for task generation:**
- **Upstream (default)**: Gathers capability and requirement trace context — the "why" context
- **Downstream**: Enumerates all children via `derive` — find everything under a scope root
- Includes all contracts, specifications, and design documents
- Captures constraints and validation rules
- Provides full implementation context in one command
- Saves to `/tmp` for developer reference during implementation

**What collect provides:**
- **Upstream**: All ancestor requirement content (parent chain to root)
- **Downstream**: All descendant requirement content (children to leaves)
- Reused markdown files (read as content)
- Reused contract elements (specifications, constraints, behaviors)
- Source citations for traceability

**Governance metadata in task planning:**
- Carry effective `status`, `priority`, `risk`, and `owner` from search JSON into every task plan
- `owner` is an accountability/routing label and may be a person, role, team, department, subsystem group, or task owner
- Use owner to route work; do not treat it as proof of personal ownership unless the value names a person
- If a governance value is inherited or defaulted, mention that source when it matters for assignment or risk review
- Do not write inherited/default governance values into Markdown unless the user explicitly decides to author them

### Step 3: Identify Verification and Test Paths

For each requirement, identify what needs to be tested:

```bash
# Get verifications for a requirement
reqvire traces --filter-id="<requirement-id>" --json

# Get test file paths from verification
reqvire search --filter-id="<verification-id>" --json
```

Extract:
- `verifiedBy` relations → which verifications to review
- `satisfiedBy` relations on verifications → which test files to run
- `satisfiedBy` relations on requirements → which code to update
- `governance_metadata` values → which status, priority, risk, and owner routing apply

### Step 4: Generate Task Plan

Create a TodoWrite-formatted task plan with full traceability.

**For new requirements:**

```markdown
☐ Implement "{Requirement Name}" ({REQ-ID})
  Context: [2-3 sentence summary from collected chain]
  Purpose: [Why - from parent requirement]
  Implementation: [Key specs/API endpoints from collected data]
  Governance: status={status}, priority={priority}, risk={risk}, owner={owner-or-unassigned}
  Owner routing: [person/role/team/department/subsystem/task owner from governance owner]
  ⚠️ IMPORTANT: Read full requirement - this is only a summary!

  ☐ Review full capability and requirement context: /tmp/trace_context_<element-id>.md
  ☐ Review requirement: [link to git blob]
  ☐ Implement functionality per specifications
  ☐ Run tests: {test paths from verifiedBy → satisfiedBy}
  ☐ Add satisfiedBy relation: reqvire link "{REQ-ID}" "satisfiedBy" "path/to/implementation"
  ☐ Validate model: reqvire validate
```

**For modified requirements:**

```markdown
☐ Update "{Requirement Name}" ({REQ-ID})
  Context: [What changed - from collected chain]
  Impact: [Affected specs/constraints from collected data]
  Governance: status={status}, priority={priority}, risk={risk}, owner={owner-or-unassigned}
  Owner routing: [person/role/team/department/subsystem/task owner from governance owner]
  ⚠️ IMPORTANT: Read full requirement - this is only a summary!

  ☐ Review full capability and requirement context: /tmp/trace_context_<element-id>.md
  ☐ Review changes: [link to git blob]
  ☐ Review code: {satisfiedBy paths from requirement}
  ☐ Update implementation
  ☐ Run tests: {test paths from verifiedBy → satisfiedBy}
  ☐ Validate model: reqvire validate
```

### Step 5: Generate Git Blob Links

Create stable links to exact requirement versions:

```bash
# Get repository URL
REPO_URL=$(git remote get-url origin | sed 's/\.git$//' | sed 's/git@github.com:/https:\/\/github.com\//')

# Generate blob link
BASE_COMMIT=$(git merge-base main HEAD)
BLOB_URL="${REPO_URL}/blob/${BASE_COMMIT}/${file_path}#${element-anchor}"
```

These links ensure developers review the exact requirement version from the base commit.

## Task Plan Structure

A complete task plan follows this format:

```markdown
# Implementation Task Plan

**Base**: {base_branch}@{base_commit}
**Capability**: {current_branch}

## Summary
- New requirements: X
- Modified requirements: Y
- Tests to run: Z
- Owners / routing groups: owner-1 (N), owner-2 (M), unassigned (K)
- High/critical priorities: N
- High/critical risks: M

## Phase 1: New Requirements

☐ Implement "{New Requirement 1}" (REQ-ID-1)
  [Task details as shown above]

☐ Implement "{New Requirement 2}" (REQ-ID-2)
  [Task details as shown above]

## Phase 2: Modified Requirements

☐ Update "{Modified Requirement 1}" (REQ-ID-3)
  [Task details as shown above]

## Phase 3: Affected Verifications

☐ Review verifications for "{Requirement Name}"
  ☐ Run test: {test-path-1}
  ☐ Run test: {test-path-2}
  ☐ Update verification if needed

## Reference Documents

Full trace context available in `/tmp/`:
- `/tmp/trace_context_element-id-1.md` - Full context for {Element Name 1}
- `/tmp/trace_context_element-id-2.md` - Full context for {Element Name 2}

Each context document shows:
- Complete capability and requirement trace chain
- Capability purpose and parent requirement obligations
- Contracts, specifications, and implementation details
- Reused design documents
- Constraints and validation rules
```

## Model Exploration Commands

**CRITICAL: Use reqvire commands to understand the semantic engineering graph - DO NOT read specification files directly!**

When analyzing capability-scoped changes for task generation:

| To Understand This | Use This Command |
|--------------------|------------------|
| What capabilities, requirements, contracts, or verifications changed | `reqvire change-impact --git-commit=<hash> --json` |
| **Full capability and requirement context (ancestors)** | `reqvire collect "<name>" --json --output /tmp/context_<id>.json` |
| Element direct content | `reqvire search --filter-id="<id>" --json` |
| Owner routing | `reqvire search --filter-owner="<owner-regex>" --json` |
| High-priority work | `reqvire search --filter-priority="high,critical" --json` |
| High-risk work | `reqvire search --filter-risk="high,critical" --json` |
| Elements by lifecycle status | `reqvire search --filter-status="draft,review,approved" --json` |
| What verifies a capability or requirement | `reqvire traces --filter-id="<id>" --json` |
| Which tests to run | Extract `satisfiedBy` from verification via `reqvire search` |
| Implementation status | Check `satisfiedBy` relations on specifying requirements |
| Trace hierarchy (up) | `reqvire collect "<name>"` shows upstream capability and requirement context |
| Trace hierarchy (down) | `reqvire collect "<name>" --direction DOWNSTREAM` shows all descendants |

**Why use commands instead of reading files:**
- Automatic relation following
- Structured JSON output for parsing
- Already validated and parsed
- Includes computed fields (verification status, etc.)
- Much more efficient than manual file reading

## Task Plan Principles

- **Traceability First**: Every task maintains requirement → implementation → test links
- **Repository-Agnostic**: No assumptions about codebase unless specified in requirements
- **Explicit Tasks**: One requirement = One top-level task with all sub-steps
- **Test-Driven**: Always include tests in implementation workflow
- **Governance-Aware**: Prioritize high/critical work, surface high/critical risk, and route by effective owner
- **Read Requirements**: Summaries are context only - full requirements are mandatory reading
- **Track Progress**: TodoWrite format enables real-time progress tracking
- **Use Commands**: Always query model via reqvire commands, not file reading

## Best Practices

- **Always read requirements**: Summaries are NOT sufficient for implementation
- **Run tests**: Verify implementation before marking tasks complete
- **Maintain traceability**: Always add/update satisfiedBy relations after implementation
- **One requirement = One task**: Don't combine multiple requirements
- **Explicit tests**: List every test file that needs to run
- **Explicit ownership**: Include effective owner/routing group; call out unassigned ownership when assignment matters
- **Repository-agnostic**: Don't assume technology stack unless in requirements
- **Link to source**: Every changed capability or requirement needs a blob link
- **Track progress**: Use TodoWrite checkboxes throughout implementation
- **Save context**: Keep `/tmp/trace_context_*.md` files for developer reference

## Complete Example

```bash
# 1. Detect base branch and get base commit
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
BASE_BRANCH="main"
BASE_COMMIT=$(git merge-base $BASE_BRANCH HEAD)

# 2. Run change impact analysis
reqvire change-impact --git-commit=$BASE_COMMIT --json --output /tmp/impact.json

# 3. For each changed capability or requirement, gather context
reqvire collect "Authentication Capability" --json --output /tmp/context_auth_capability.json
reqvire collect "Authentication Capability" > /tmp/trace_context_auth_capability.md

# 4. Get element details
reqvire search --filter-id="system-model/Auth.md#authentication-capability" --json

# 5. Get verification and test paths
reqvire traces --filter-id="system-model/Auth.md#authentication-capability" --json
reqvire search --filter-id="system-model/Verifications/AuthTests.md#auth-test" --json

# 6. Generate git blob URL
REPO_URL=$(git remote get-url origin | sed 's/\.git$//' | sed 's/git@github.com:/https:\/\/github.com\//')
BLOB_URL="${REPO_URL}/blob/${BASE_COMMIT}/system-model/Auth.md#authentication-capability"

# 7. Create task plan (manual or automated)
```

**Resulting task:**

```markdown
☐ Implement requirements under "Authentication Capability" (system-model/Auth.md#authentication-capability)
  Context: The capability enables secure authentication for protected
  resources. Its specifying requirements define JWT token handling, refresh
  behavior, password hashing, and session constraints.
  Purpose: Enable secure user authentication for protected resources
  Implementation: JWT token generation, password bcrypt hashing, session management
  ⚠️ IMPORTANT: Read full capability and requirement context - this is only a summary!

  ☐ Review full capability and requirement context: /tmp/trace_context_auth_capability.md
  ☐ Review capability and specifying requirements: https://github.com/org/repo/blob/abc123/system-model/Auth.md#authentication-capability
  ☐ Implement JWT token generation per specification
  ☐ Implement password hashing using bcrypt
  ☐ Implement session management with timeout constraints
  ☐ Run tests: tests/auth/test_authentication.rs
  ☐ Run tests: tests/auth/test_session.rs
  ☐ Add satisfiedBy relation to the relevant requirement: reqvire link "JWT Authentication Requirement" "satisfiedBy" "src/auth/jwt.rs"
  ☐ Validate model: reqvire validate
```

## Integration with Other References

After creating the task plan:
- **For implementation**: Follow the task plan with TodoWrite tracking
- **For requirement clarification**: See [Explore](explore.md) for model exploration
- **For adding new requirements**: See [Add Capability](AddCapability.md)
- **For refactoring requirements**: See [Consolidate Requirements](ConsolidateRequirements.md)
- **For verification**: Run tests and validate model after each implementation

## Using Slash Commands

The syseng skill provides slash commands that automate this workflow:

- `/reqvire:analyze-impact [commit-hash]` - Analyze capability and requirement changes and their impact
- `/reqvire:generate-tasks [base-commit]` - Generate complete task plan from changes

These commands follow the workflow described above and automatically:
1. Detect base branch
2. Run change-impact analysis
3. Collect capability and requirement context
4. Generate TodoWrite task plan
5. Save reference documents to `/tmp`
6. Create git blob links

**When to use slash commands vs manual workflow:**
- **Use slash commands**: Quick task generation for capability branches
- **Use manual workflow**: Custom analysis, learning the process, non-standard cases
