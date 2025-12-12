# Creating Implementation Tasks from Requirements

This workflow bridges the gap between requirements and implementation by analyzing what changed and generating comprehensive task lists.

**Key principle**: Every implementation task maintains traceability to requirements, implementations, and tests.

**For common commands** (change-impact, collect, search, validate), see [SKILL.md Command Reference](../SKILL.md#command-reference). For validation workflow, see [SKILL.md Validation & Quality Checklist](../SKILL.md#validation--quality-checklist).

## When to Use This Workflow

- Generating implementation plans from requirement changes
- Understanding what requirements changed on a feature branch
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
reqvire change-impact --git-commit=$BASE_COMMIT --json > /tmp/impact.json

# Compare against specific commit
reqvire change-impact --git-commit=<commit-hash>

# Compare against previous commit
reqvire change-impact --git-commit=HEAD~1
```

The change-impact command identifies:
- `added_elements[]` - New requirements/verifications
- `modified_elements[]` - Changed requirements/verifications
- `affected_elements[]` - Elements impacted by changes (via derivedFrom, verifiedBy)

**Change Propagation Rules:**
- **Parent → Child**: Parent changes propagate to all derived children
- **Requirement → Verification**: Requirement changes invalidate verifications
- **Requirement → Implementation**: May need implementation updates
- **Verification changes**: Generally don't propagate upward

### Step 2: Gather Full Requirement Context

For each changed requirement, collect complete context:

```bash
# Get full requirement chain with ancestors and attachments
reqvire collect "<requirement-name>" --json > /tmp/req_<requirement-id>.json

# Also save human-readable format for reference
reqvire collect "<requirement-name>" > /tmp/req_context_<requirement-id>.md

# Get direct requirement details
reqvire search --filter-id="<requirement-id>" --json
```

**Why use `reqvire collect` for task generation:**
- Gathers complete requirement chain via `derivedFrom` relations
- Shows parent requirements (the "why" context)
- Includes all specifications and design documents
- Captures constraints and validation rules
- Provides full implementation context in one command
- Saves to `/tmp` for developer reference during implementation

**What collect provides:**
- All ancestor requirement content
- Attached markdown files (read as content)
- Attached refinement elements (specifications, constraints, behaviors)
- Source citations for traceability

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

### Step 4: Generate Task Plan

Create a TodoWrite-formatted task plan with full traceability.

**For new requirements:**

```markdown
☐ Implement "{Requirement Name}" ({REQ-ID})
  Context: [2-3 sentence summary from collected chain]
  Purpose: [Why - from parent requirement]
  Implementation: [Key specs/API endpoints from collected data]
  ⚠️ IMPORTANT: Read full requirement - this is only a summary!

  ☐ Review full requirement context: /tmp/req_context_<req-id>.md
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
  ⚠️ IMPORTANT: Read full requirement - this is only a summary!

  ☐ Review full requirement context: /tmp/req_context_<req-id>.md
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
**Feature**: {current_branch}

## Summary
- New requirements: X
- Modified requirements: Y
- Tests to run: Z

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

Full requirement context available in `/tmp/`:
- `/tmp/req_context_req-id-1.md` - Full context for {Requirement Name 1}
- `/tmp/req_context_req-id-2.md` - Full context for {Requirement Name 2}

Each context document shows:
- Complete requirement chain (derivedFrom)
- Parent requirements and purpose
- Specifications and implementation details
- Attached design documents
- Constraints and validation rules
```

## Model Exploration Commands

**CRITICAL: Use reqvire commands to understand requirements - DO NOT read specification files directly!**

When analyzing requirements for task generation:

| To Understand This | Use This Command |
|--------------------|------------------|
| What requirements changed | `reqvire change-impact --git-commit=<hash> --json` |
| **Full requirement context** | `reqvire collect "<name>" --json > /tmp/req_<id>.json` |
| Requirement direct content | `reqvire search --filter-id="<id>" --json` |
| What verifies a requirement | `reqvire traces --filter-id="<id>" --json` |
| Which tests to run | Extract `satisfiedBy` from verification via `reqvire search` |
| Implementation status | Check `satisfiedBy` relations in requirement |
| Requirement hierarchy | `reqvire collect "<name>"` shows complete derivedFrom chain |

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
- **Read Requirements**: Summaries are context only - full requirements are mandatory reading
- **Track Progress**: TodoWrite format enables real-time progress tracking
- **Use Commands**: Always query model via reqvire commands, not file reading

## Best Practices

- **Always read requirements**: Summaries are NOT sufficient for implementation
- **Run tests**: Verify implementation before marking tasks complete
- **Maintain traceability**: Always add/update satisfiedBy relations after implementation
- **One requirement = One task**: Don't combine multiple requirements
- **Explicit tests**: List every test file that needs to run
- **Repository-agnostic**: Don't assume technology stack unless in requirements
- **Link to source**: Every requirement needs a blob link
- **Track progress**: Use TodoWrite checkboxes throughout implementation
- **Save context**: Keep `/tmp/req_context_*.md` files for developer reference

## Complete Example

```bash
# 1. Detect base branch and get base commit
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
BASE_BRANCH="main"
BASE_COMMIT=$(git merge-base $BASE_BRANCH HEAD)

# 2. Run change impact analysis
reqvire change-impact --git-commit=$BASE_COMMIT --json > /tmp/impact.json

# 3. For each changed requirement, gather context
reqvire collect "Authentication Feature" --json > /tmp/req_auth_feature.json
reqvire collect "Authentication Feature" > /tmp/req_context_auth_feature.md

# 4. Get requirement details
reqvire search --filter-id="requirements/Auth.md#authentication-feature" --json

# 5. Get verification and test paths
reqvire traces --filter-id="requirements/Auth.md#authentication-feature" --json
reqvire search --filter-id="requirements/Verifications/AuthTests.md#auth-test" --json

# 6. Generate git blob URL
REPO_URL=$(git remote get-url origin | sed 's/\.git$//' | sed 's/git@github.com:/https:\/\/github.com\//')
BLOB_URL="${REPO_URL}/blob/${BASE_COMMIT}/requirements/Auth.md#authentication-feature"

# 7. Create task plan (manual or automated)
```

**Resulting task:**

```markdown
☐ Implement "Authentication Feature" (requirements/Auth.md#authentication-feature)
  Context: System shall authenticate users using JWT tokens with refresh
  capabilities. Derived from "User Security" requirement to protect sensitive
  user data. Includes password hashing specification and session constraints.
  Purpose: Enable secure user authentication for protected resources
  Implementation: JWT token generation, password bcrypt hashing, session management
  ⚠️ IMPORTANT: Read full requirement - this is only a summary!

  ☐ Review full requirement context: /tmp/req_context_auth_feature.md
  ☐ Review requirement: https://github.com/org/repo/blob/abc123/requirements/Auth.md#authentication-feature
  ☐ Implement JWT token generation per specification
  ☐ Implement password hashing using bcrypt
  ☐ Implement session management with timeout constraints
  ☐ Run tests: tests/auth/test_authentication.rs
  ☐ Run tests: tests/auth/test_session.rs
  ☐ Add satisfiedBy relation: reqvire link "Authentication Feature" "satisfiedBy" "src/auth/jwt.rs"
  ☐ Validate model: reqvire validate
```

## Integration with Other References

After creating the task plan:
- **For implementation**: Follow the task plan with TodoWrite tracking
- **For requirement clarification**: See [Explore](explore.md) for model exploration
- **For adding new requirements**: See [Add Feature](AddFeature.md)
- **For refactoring requirements**: See [Consolidate Requirements](ConsolidateRequirements.md)
- **For verification**: Run tests and validate model after each implementation

## Using Slash Commands

The syseng skill provides slash commands that automate this workflow:

- `/reqvire:analyze-impact [commit-hash]` - Analyze requirement changes and their impact
- `/reqvire:generate-tasks [base-commit]` - Generate complete task plan from changes

These commands follow the workflow described above and automatically:
1. Detect base branch
2. Run change-impact analysis
3. Collect requirement context
4. Generate TodoWrite task plan
5. Save reference documents to `/tmp`
6. Create git blob links

**When to use slash commands vs manual workflow:**
- **Use slash commands**: Quick task generation for feature branches
- **Use manual workflow**: Custom analysis, learning the process, non-standard cases
