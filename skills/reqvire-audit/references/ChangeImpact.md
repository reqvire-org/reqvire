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
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" change-impact --git-commit="$BASE_COMMIT"
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" change-impact --git-commit="$BASE_COMMIT" --json --output /tmp/impact.json
   ```

   Ordinary Git mode requires one eligible worktree; `--git-commit` selects its base, defaulting to HEAD. A plain aggregate with multiple independent worktrees remains ambiguous.

   **Repo composition:** Run at the workspace root, with an initialized Repo workspace and declared projects prepared locally. `--git-commit` is the sole baseline selector: read the root model, selected manifest and includes from that repository commit, then load each component at the full commit ID pinned by that historical manifest. Do not apply the root ref independently in components. Use global `--manifest` to select a tracked manifest:

   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" --manifest default.xml change-impact \
     --git-commit="$BASE_COMMIT" --json --output /tmp/impact.json
   ```

   Every manifest input must have identifiable history in the selected workspace-root repository. Independent native manifest history reports `repo_impact_manifest_history_unavailable` with the input path and guidance to select a manifest tracked in that repository. Mutable baseline project pins report `repo_impact_base_unlocked`; missing project objects report `repo_impact_base_unavailable`. Removed projects may use retained local Repo object storage. Do not drop unavailable projects, guess a second revision or reuse live XML to produce a partial comparison. Reqvire never fetches, syncs or changes active checkouts during analysis.

   **Mutation-enabled MCP:** Use `reqvire.change_impact` with the same selected `branch` and optional `git_commit`. The current side is the accepted model, captured composition and accepted pending edits, including embedded MCP. External changes are not imported by this read. A baseline error preserves the accepted revision, pending edits and write availability.

4. **Analyze the results:**

   Extract from JSON:
   - `added[]` - New requirements/verifications
   - `changed[]` - Changed requirements/verifications
   - `removed[]` - Removed elements
   - `relocated[]` - Relocated elements (same name, different path)
   - `impact_scope[]` - Per-branch scope roots (common parent requirements covering all impacted elements)
   - `invalidated_verifications[]` - Verifications that need re-review

   Repo reports also include `comparison.workspace_mode: "repo"` and `comparison.base` / `comparison.current`. Each side records `central_commit`, sorted `manifest_inputs` with `path` / owning `commit`, and `projects` with `name`, workspace-relative `path`, repository identity `repository` and resolved `commit`. Inspect each owning repository's diff using these selections, including added, removed and moved projects. Commit provenance does not imply pending bytes are committed; inspect context/pending-state evidence separately. Source links use the relevant side's owning repository and commit, with workspace-relative fallbacks for unsupported remotes.

   The top-level `added` and `changed` arrays are smart-filtered report roots, not a complete change inventory: changes already reached through another root remain inside its `change_impact_tree`. Traverse those trees and compare the Git diff and current element inventory before concluding that no elements were added or changed. Native ontology and concept changes can produce a broad review set. An invalidated verification needs evidence review; it is not an assertion that its test has failed.

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
   - Use `reqvire-syseng` skill to generate implementation tasks from changes

## Change Propagation Rules

- **Contract → Binding or reference consumer**: Both dependency kinds propagate content changes for review; only bindings affect implementation fulfillment. Inspect both structured edge kinds in the impact report.
- **Cross-repository documentation**: Include website/docs repositories as declared Repo projects. Documentation requirements use Contract References to product contracts, concrete verifications and eligible `satisfiedBy` page sources. Contract changes can reach all three across repository boundaries. A remote URL alone does not make an external model or history available.
- **Parent → Child**: Parent changes propagate to all derived children
- **Requirement → Verification**: Requirement changes invalidate verifications
- **Requirement → Implementation**: May need implementation updates
- **Verification changes**: Generally don't propagate upward

## Notes

- Use for understanding impact before making changes
- Run after making changes to identify affected elements
- Focus on verifications — they need review when requirements change
- Use the `reqvire-syseng` skill's [CreatingTasks](../../reqvire-syseng/references/CreatingTasks.md) workflow to generate implementation tasks from impact
