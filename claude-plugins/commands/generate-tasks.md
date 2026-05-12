---
allowed-tools: Read, Bash(npx:*), Bash(git:*)
argument-hint: [base-commit]
description: Generate implementation task plan from requirement changes using change-impact analysis
model: claude-sonnet-4-5
---

# Generate Tasks

Generate implementation task plan from requirement changes on a feature branch.

## Context

- Current branch: !`git rev-parse --abbrev-ref HEAD`
- Base commit: ${1:-!`git merge-base main HEAD 2>/dev/null || git merge-base master HEAD`}

## Steps

1. **Detect base branch:**
   ```bash
   CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

   if git show-ref --verify --quiet refs/heads/main; then
       BASE_BRANCH="main"
   elif git show-ref --verify --quiet refs/heads/master; then
       BASE_BRANCH="master"
   else
       echo "Specify base commit manually"
       exit 1
   fi

   BASE_COMMIT="${1:-$(git merge-base $BASE_BRANCH HEAD)}"
   ```

2. **Run change impact:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" change-impact --git-commit=$BASE_COMMIT --json --output /tmp/impact.json
   ```

3. **Review impact scope and enumerate covered elements** (from JSON `impact_scope[]`):

   The `impact_scope` array shows the per-branch common parent requirements covering all impacted elements. For each scope root, use downstream collect to enumerate all covered children:
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" collect "<scope-root-name>" --direction DOWNSTREAM --json --output /tmp/scope_<name>.json
   ```

   This returns the scope root and all its descendants, giving you the complete list of affected elements under each scope entry.

4. **For each changed requirement:**

   Get full upstream context using collect:
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" collect "<requirement-name>" --json --output /tmp/req_<requirement-id>.json
   ```

   This provides:
   - Complete ancestor chain via derivedFrom relations (upstream)
   - All parent requirements for context
   - Refinement elements (specifications, constraints, behaviors) that refine the requirement
   - Attached design documents
   - Full implementation context

   Also get direct details:
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-id="<requirement-id>" --json
   ```

   Extract:
   - Requirement content
   - verifiedBy relations (tests to run)
   - satisfiedBy relations (code to update)
   - derivedFrom relations (context)
   - governance_metadata values (status, priority, risk, owner)

   Treat `owner` as an accountability/routing label. It may be a person, role, team, department, subsystem group, or task owner.

5. **For each verification:**

   Get test paths:
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-id="<verification-id>" --json
   ```

   Extract satisfiedBy relations (test files).

6. **Generate TodoWrite task plan:**

   **Use collected context** from `/tmp/req_<requirement-id>.json` to create concise summaries:
   - Extract parent requirement purpose (why this exists)
   - Identify key specifications (how to implement)
   - Note important constraints and validation rules
   - Carry effective governance status, priority, risk, and owner routing
   - Summarize in ~2-3 sentences

   **For new requirements:**
   ```
   ☐ Implement "{Requirement Name}" ({REQ-ID})
     Context: [2-3 sentence summary from collected chain]
     Purpose: [Why - from parent requirement]
     Implementation: [Key specs/API endpoints from collected data]
     Governance: status={status}, priority={priority}, risk={risk}, owner={owner-or-unassigned}
     Owner routing: [person/role/team/department/subsystem/task owner from governance owner]

     ☐ Review full requirement context: [link to collect output]
     ☐ Review requirement: [link to blob]
     ☐ Implement functionality per specifications
     ☐ Run tests: {test paths}
     ☐ Add satisfiedBy relation
     ☐ Validate: reqvire validate
   ```

   **For modified requirements:**
   ```
   ☐ Update "{Requirement Name}" ({REQ-ID})
     Context: [What changed - from collected chain]
     Impact: [Affected specs/constraints from collected data]
     Governance: status={status}, priority={priority}, risk={risk}, owner={owner-or-unassigned}
     Owner routing: [person/role/team/department/subsystem/task owner from governance owner]

     ☐ Review full requirement context: [link to collect output]
     ☐ Review changes: [link to blob]
     ☐ Review code: {satisfiedBy paths}
     ☐ Update implementation
     ☐ Run tests: {test paths}
     ☐ Validate: reqvire validate
   ```

7. **Generate git blob links:**
   ```bash
   REPO_URL=$(git remote get-url origin | sed 's/\.git$//' | sed 's/git@github.com:/https:\/\/github.com\//')
   BLOB_URL="${REPO_URL}/blob/${BASE_COMMIT}/${file_path}#${element-anchor}"
   ```

8. **Save collected context for reference:**

   For each requirement, save a formatted summary:
   ```bash
   # Save collected output to /tmp for reference
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" collect "<requirement-name>" > /tmp/req_context_<requirement-id>.md
   ```

   This provides developers with full context documents they can reference during implementation.

9. **Present task plan:**
   - Phase 1: New requirements to implement (with context summaries)
   - Phase 2: Modified requirements to update (with impact analysis)
   - Phase 3: Affected verifications to review
   - Appendix: Links to full context documents in /tmp

## Task Plan Structure

```markdown
# Implementation Task Plan

**Base**: {base_branch}@{base_commit}
**Feature**: {current_branch}

## Summary
- Impact scope: {scope root names from impact_scope[]}
- New requirements: X
- Modified requirements: Y
- Tests to run: Z
- Owners / routing groups: owner-1 (N), owner-2 (M), unassigned (K)
- High/critical priorities: N
- High/critical risks: M

## Tasks
{TodoWrite formatted tasks with context summaries}

## Reference Documents

Full requirement context available in `/tmp/`:
- `/tmp/req_context_<req-id-1>.md` - Full context for {Requirement Name 1}
- `/tmp/req_context_<req-id-2>.md` - Full context for {Requirement Name 2}

Each context document shows:
- Complete requirement chain (derivedFrom)
- Parent requirements and purpose
- Specifications and implementation details
- Attached design documents
- Constraints and validation rules
```

## Notes

- Task plan uses TodoWrite format for tracking
- Links to exact requirement versions via git blob URLs
- Repository-agnostic: no technology assumptions
- Always read full requirements, not just summaries
- Use governance metadata for prioritization, risk surfacing, and owner routing
