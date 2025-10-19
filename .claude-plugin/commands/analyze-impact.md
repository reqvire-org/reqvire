---
name: analyze-impact
description: Analyze change impact for modified requirements using git commit history
---

# Analyze Change Impact

Analyze how changes to requirements propagate through the model.

## Steps

1. **Get base commit:**

   Ask user for commit hash or use default:
   ```bash
   # Compare against HEAD~1 (previous commit)
   BASE_COMMIT="HEAD~1"

   # Or compare against specific commit
   BASE_COMMIT="<commit-hash>"

   # Or compare against base branch
   BASE_COMMIT=$(git merge-base main HEAD)
   ```

2. **Run change impact analysis:**
   ```bash
   reqvire change-impact --git-commit=$BASE_COMMIT
   reqvire change-impact --git-commit=$BASE_COMMIT --json > /tmp/impact.json
   ```

3. **Analyze the results:**

   Extract from JSON:
   - `added_elements[]` - New requirements/verifications
   - `modified_elements[]` - Changed requirements/verifications
   - `affected_elements[]` - Elements impacted by changes

4. **For each modified requirement:**

   ```bash
   reqvire summary --filter-id="<requirement-id>"
   reqvire traces --filter-id="<requirement-id>"
   ```

   Identify:
   - What changed in the requirement
   - Which verifications verify this requirement
   - Which implementations satisfy this requirement
   - Which children derive from this requirement

5. **Present impact findings:**

   **Added Elements:**
   - [New Requirement](file.md#new-req) - type: requirement

   **Modified Elements:**
   - [Changed Requirement](file.md#changed) - review needed

   **Affected Elements (propagation):**
   - Verifications needing review: X
   - Implementations needing update: Y
   - Child requirements affected: Z

6. **Provide recommendations:**
   - Review and update affected verifications
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
