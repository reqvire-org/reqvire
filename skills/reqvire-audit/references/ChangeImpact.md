# Analyze Change Impact

Analyze how changes to requirements propagate through the model.

In mutation-enabled MCP, comparisons use the captured accepted current model and HEAD even when external drift or recovery blocks writes. Historical objects must be available locally. Read the live recovery diagnostics separately from accepted comparison provenance; a comparison does not import external current files or resolve a failed checkpoint.

## Steps

1. **Check workspace and Git context:**
   ```bash
   git rev-parse --show-toplevel
   git branch --show-current
   git log --oneline -5
   ```

   Run from the intended effective workspace root, or pass it with `--workspace`. The workspace must contain at least one eligible Git worktree. Reqvire ignores non-Git folders under the workspace.

2. **Choose the review baseline:**

   Use available CI or confirmed MCP PR context when it identifies the intended review. If that context is missing, ask for the intended baseline. A previous commit is appropriate only when the requested review is specifically one commit.

   ```bash
   # Explicit choice supplied for this review:
   BASE_COMMIT="<commit-hash>"

   # Or derive the merge-base with the actual review target:
   # Set PR_BASE_REF to the actual PR target; do not assume main.
   BASE_COMMIT=$(git merge-base HEAD "$PR_BASE_REF")
   ```

3. **Run change impact analysis:**
   ```bash
   # Use available PR context:
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" change-impact --json --output /tmp/impact.json

   # When an explicit override was selected above:
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" change-impact --git-commit="$BASE_COMMIT"
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" change-impact --git-commit="$BASE_COMMIT" --json --output /tmp/impact.json
   ```

   Ordinary Git mode requires one eligible worktree. The explicit override selects its base; omission uses available CI or confirmed MCP PR context. Without that context, ask for a baseline instead of selecting main or HEAD silently. Explicit HEAD reviews pending changes. A plain aggregate with multiple independent worktrees remains ambiguous.

   **Repo composition:** Run at the workspace root with initialized Repo metadata and declared projects prepared locally. `.repo/manifest.xml` is the default. One root baseline supplies its model and composition association, which selects the actual manifest-owner commits and their full component pins. Do not apply the root ref independently in components or require another manifest selector:

   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" change-impact \
     --git-commit="$BASE_COMMIT" --json --output /tmp/impact.json
   ```

   XML/includes/overlays remain versioned in their actual owners. The root `.reqvire-composition.json` contains revision references, without copied XML or a circular root commit. Independent owner history is supported. Unidentified associations or unversioned required inputs report `repo_impact_manifest_history_unavailable`; mutable project pins report `repo_impact_base_unlocked`, and unavailable project objects report `repo_impact_base_unavailable`. Removed owners/projects may use retained local object storage. Never drop unavailable projects, guess another revision or use live XML for a partial comparison. Analysis never fetches, syncs or changes active checkouts.

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
