---
allowed-tools: Read, Bash(reqvire:*), Bash(git:*)
argument-hint: [commit-hash]
description: Analyze change impact for modified requirements using git commit history
model: claude-sonnet-4-5
---

# Analyze Change Impact

Analyze how changes to requirements propagate through the model.

## Git Context

- Current branch: !`git branch --show-current`
- Recent commits: !`git log --oneline -5`
- Base branch merge-base: !`git merge-base main HEAD 2>/dev/null || echo "N/A"`

## Comparison Target

${1:+Comparing against: $1}
${1:-Comparing against: HEAD~1 (previous commit)}

## Steps

1. **Get base commit:**

   ${1:+Using provided commit: $1}
   ${1:-Ask user for commit hash or use default:}
   ```bash
   # Compare against HEAD~1 (previous commit)
   BASE_COMMIT="${1:-HEAD~1}"

   # Or compare against specific commit
   BASE_COMMIT="<commit-hash>"

   # Or compare against base branch
   BASE_COMMIT=$(git merge-base main HEAD)
   ```

2. **Run change impact analysis:**
   ```bash
   reqvire change-impact --git-commit=${1:-HEAD~1}
   reqvire change-impact --git-commit=${1:-HEAD~1} --json --output /tmp/impact.json
   ```

3. **Analyze the results:**

   Extract from JSON:
   - `added[]` - New requirements/verifications
   - `changed[]` - Changed requirements/verifications
   - `removed[]` - Removed elements
   - `relocated[]` - Relocated elements (same name, different path)
   - `impact_scope[]` - Per-branch scope roots (common parent requirements covering all impacted elements)
   - `invalidated_verifications[]` - Verifications that need re-review

4. **For each modified requirement:**

   ```bash
   reqvire search --filter-id="<requirement-id>"
   reqvire traces --filter-id="<requirement-id>"
   ```

   Identify:
   - What changed in the requirement
   - Which verifications verify this requirement
   - Which implementations satisfy this requirement
   - Which children derive from this requirement

5. **Present impact findings:**

   **Impact Scope** (high-level affected areas):
   - [Parent Requirement A](file.md#parent-a) - covers changed children
   - [Standalone Req](file.md#standalone) - directly changed

   **Added Elements:**
   - [New Requirement](file.md#new-req) - type: requirement

   **Modified Elements:**
   - [Changed Requirement](file.md#changed) - review needed

   **Invalidated Verifications:**
   - Verifications needing re-review: X

6. **Provide recommendations:**
   - Start with impact scope to understand affected model areas
   - Review and update invalidated verifications
   - Update test criteria if requirements changed
   - Review implementations marked with satisfiedBy
   - Run tests for affected verifications
   - Update child requirements if parent semantics changed

## Change Propagation Rules

- **Parent → Child**: Parent changes propagate to all derived children
- **Requirement → Verification**: Requirement changes invalidate verifications
- **Requirement → Implementation**: May need implementation updates
- **Verification changes**: Generally don't propagate upward

## Notes

- Use for understanding impact before making changes
- Run after making changes to identify affected elements
- Focus on verifications - they need review when requirements change
- Use `/generate-tasks` to create implementation tasks from impact
