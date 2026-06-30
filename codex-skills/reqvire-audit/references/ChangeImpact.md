# Analyze Change Impact

Analyze how changes to requirements propagate through the model.

## Steps

1. **Check workspace and Git context:**
   ```bash
   git rev-parse --show-toplevel
   git branch --show-current
   git log --oneline -5
   git merge-base main HEAD 2>/dev/null || echo "N/A"
   ```

   Run from the intended effective workspace root, or pass it with `--workspace`. The workspace must contain at least one eligible Git worktree. Reqvire ignores non-Git folders under the workspace.

2. **Get base commit:**

   ```bash
   # Compare against HEAD~1 (previous commit)
   BASE_COMMIT="${1:-HEAD~1}"

   # Or compare against specific commit
   BASE_COMMIT="<commit-hash>"

   # Or compare against base branch
   BASE_COMMIT=$(git merge-base main HEAD)
   ```

3. **Run change impact analysis:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" change-impact --git-commit=${BASE_COMMIT}
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" change-impact --git-commit=${BASE_COMMIT} --json --output /tmp/impact.json
   ```

   `--git-commit` materializes the base snapshot from the current eligible Git worktree. It is not a multi-repository commit selector; reported paths and identifiers remain relative to the effective workspace root.

4. **Analyze the results:**

   Extract from JSON:
   - `added[]` - New requirements/verifications
   - `changed[]` - Changed requirements/verifications
   - `removed[]` - Removed elements
   - `relocated[]` - Relocated elements (same name, different path)
   - `impact_scope[]` - Per-branch scope roots (common parent requirements covering all impacted elements)
   - `invalidated_verifications[]` - Verifications that need re-review

5. **For each modified requirement:**

   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-id="<requirement-id>"
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" traces --filter-id="<requirement-id>"
   ```

   Identify:
   - What changed in the requirement
   - Which verifications verify this requirement
   - Which implementations satisfy this requirement
   - Which children derive from this requirement

6. **Present impact findings:**

   **Impact Scope** (high-level affected areas):
   - [Parent Requirement A](file.md#parent-a) - covers changed children
   - [Standalone Req](file.md#standalone) - directly changed

   **Added Elements:**
   - [New Requirement](file.md#new-req) - type: requirement

   **Modified Elements:**
   - [Changed Requirement](file.md#changed) - review needed

   **Invalidated Verifications:**
   - Verifications needing re-review: X

7. **Provide recommendations:**
   - Start with impact scope to understand affected model areas
   - Review and update invalidated verifications
   - Update test criteria if requirements changed
   - Review implementations marked with satisfiedBy
   - Run tests for affected verifications
   - Update child requirements if parent semantics changed
   - Use `reqvire:syseng` skill to generate implementation tasks from changes

## Change Propagation Rules

- **Parent → Child**: Parent changes propagate to all derived children
- **Requirement → Verification**: Requirement changes invalidate verifications
- **Requirement → Implementation**: May need implementation updates
- **Verification changes**: Generally don't propagate upward

## Notes

- Use for understanding impact before making changes
- Run after making changes to identify affected elements
- Focus on verifications — they need review when requirements change
- Use the `reqvire:syseng` skill's [CreatingTasks](../../syseng/reference/CreatingTasks.md) workflow to generate implementation tasks from impact
