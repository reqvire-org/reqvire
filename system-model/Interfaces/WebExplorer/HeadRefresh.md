# Elements

### Explorer Worktree HEAD Refresh

WHEN a user selects an admitted worktree in HEAD mode or requests Refresh, the system SHALL use the shared checked HEAD-refresh operation for an observed branch-tip change and display the resulting normally loaded publication with matching context and commit provenance.

IF refresh is blocked, the system SHALL retain the displayed accepted model and commit label, expose the blocking diagnostic and disabled-write state, and preserve selection and the MCP connection.

#### Concept References
  * [HEAD Mode](../../Thesaurus/Thesaurus.md#head-mode)
  * [HEAD Refresh](../../Thesaurus/Thesaurus.md#head-refresh)
  * [Accepted HEAD](../../Thesaurus/Thesaurus.md#accepted-head)
  * [Observed HEAD](../../Thesaurus/Thesaurus.md#observed-head)

#### Metadata
  * type: requirement

#### Contract Bindings
  * [MCP Branch Refresh Specification](../MCP/HeadRefresh.md#mcp-branch-refresh-specification)

#### Relations
  * definedBy: [Explorer Worktree HEAD Refresh Specification](#explorer-worktree-head-refresh-specification)
  * satisfiedBy: [ExplorerWorkspace.tsx](../../../explorer/design-system/patterns/shell/ExplorerWorkspace.tsx)
  * satisfiedBy: [headRefresh.ts](../../../explorer/src/store/headRefresh.ts)
  * satisfiedBy: [useLiveStore.ts](../../../explorer/src/store/useLiveStore.ts)
  * specify: [Explorer Worktree Browsing](../InterfacesFeature.md#explorer-worktree-browsing)
---

### Explorer Worktree HEAD Refresh Specification

Defines HEAD-default branch selection, explicit refresh and accepted/observed commit presentation in served Explorer.

#### Details
- Keep branch/worktree selection as the default HEAD mode, using exact local branch names in the picker, browser URLs, HTTP/MCP requests and tab-local selection. Opaque IDs remain private runtime/cache identity. A commit ID is provenance, not a new worktree identity. Do not add historical commit selection, a global active branch or a separate MCP switching tool.
- First admission resolves the current local branch HEAD and fully validates its model under existing mode-specific rules. Plain/read-only demand loads retain current dirty-tree and source-cache semantics; HEAD mode does not newly impose writable admission on them. Static exports remain fixed exported snapshots.
- In mutation-enabled serving, selecting an already admitted context compares its accepted and observed HEAD. If they differ, execute `reqvire.branch.refresh` with that exact branch before adopting a new publication; on an unchanged healthy context, reuse the accepted model without a graph rebuild. Accepted pending MCP edits with unchanged HEAD remain usable on reselection and do not trigger a new clean-start requirement.
- Expose an accessible `Refresh HEAD` button in mutation-enabled served Explorer. It calls the same MCP tool with the displayed context's exact `branch` and persisted execution (`dry_run: false`). Resolve that branch from the displayed accepted Project Store, not another tab or an observed checkout change. Explorer HTTP routes and selection URLs also use branch names; private cache/runtime identity remains isolated. Selection and button actions consume the shared backend policy; the browser does not implement a separate classifier or Git-diff endpoint.
- Poll/inventory observations can indicate external HEAD staleness without adopting a tip or rebuilding models. The observed commit is not displayed as accepted model provenance. No continuous background adoption is required.
- A successful refresh uses normal validated loading and the existing complete-publication adoption path while preserving worker/context identity, route, current view and valid navigation, with matching Git metadata, resource/source bytes, manifest/chunks and publication revision. Artifact-only changes retain the semantic model revision; a permitted current-exclusion change can change membership and revision through normal validation. Clear the relevant stale diagnostic only after complete success.
- A protected model change or failed eligibility/loading check retains the last accepted view and commit provenance. Runtime-generation/publication failures use the existing runtime diagnostic and retained-view behavior. Show an actionable blocked/stale diagnostic with affected paths and the observed commit, indicate that ordinary writes are disabled for an external HEAD mismatch, and keep status and accepted-snapshot reads available. Do not reopen/recreate the worker silently, disconnect MCP clients or label old data with the new commit. Protected model changes require an explicit full validation/admission workflow.
- Serialize backend adoption through the selected context while leaving other contexts independent. Existing loading/error presentation, context-scoped caches and cancellation apply: late completions cannot replace another selection or dismiss its newer operation. A blocked refresh must not change the selected branch or another tab's state.
- Use existing snapshot/publication infrastructure. Ordinary manifest/chunk reads remain reads; inventory and inactive contexts do not initiate adoption. Model-semantic revision and Explorer publication revision retain their separate meanings.

#### Metadata
  * type: specification

#### Relations
  * define: [Explorer Worktree HEAD Refresh](#explorer-worktree-head-refresh)
---
