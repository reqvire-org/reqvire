# Workspace management plan

Updated: 2026-10-31. Planning baseline: `21c6c20dde337d289c4e8c7d5aaef14cb81d4281`, branch `refactor/mcp-branch-tools`, PR <https://github.com/reqvire-org/reqvire/pull/88>.

This document records the agreed developer workflow, the original implementation gaps, and the completed implementation/verification record in section 16. It does not replace the system model or claim that the working-branch changes have shipped.

Implementation started with the system-model milestone. The following engineering choices resolve the contract questions below within the requested scope:

- Version a `.reqvire-composition.json` association in the root. It records native selection and actual manifest-owner commit references, never copied XML or its own containing root commit. Root-owned inputs are read from the containing root tree. Preserve native `.repo/manifest.xml` selection and commit manifest sources in their real owners.
- Use explicit `branch.open` for full validated admission when an already registered branch's committed composition changed. Unchanged healthy contexts remain idempotent. Keep `branch.refresh` guarded for artifact-only HEAD refresh. Add no reload/adopt/mode argument.
- Extend coordinated PR creation with a `repository_bases` map keyed by workspace-relative repository owner path. The existing required `base` selects the root target; changed non-root owners need explicit map entries. Return a per-repository PR set and root links. Comments retain explicit targeting.
- Use available PR-base environment or recorded branch review-target metadata to select a local merge-base. When neither is available and no explicit commit was supplied, return `review_baseline_required` guidance instead of guessing main or silently selecting HEAD.
- Advertise the changed management/publication semantics as tool contract version 5. The transport protocol and branch tool names remain unchanged.
- Keep Git subprocesses behind the existing shared helpers for this iteration; reuse batched immutable-object reads. Do not add a Git-library backend migration to the workspace scope.

These choices are implementation decisions recorded for review, rather than claims that the code already supports them. Milestone 1 requires aligned model/verifications and demonstrably failing behavioral checks; milestone 2 requires implementation, skills/website synchronization, and passing validation.

The implementation and its current verification evidence are recorded in section 16. Section 4 preserves the original baseline audit.

Milestone 1 evidence (2026-10-31): typed MCP previews/applies produced the model changes; source validation reports no errors, lint reports no findings, coverage was inspected, and ontology export succeeds. The focused native-composition run against the unchanged Rust implementation ran 15 cases: one passed (prepared composition admission) and 14 failed. Rejected admission loses accepted reads/pending changes; the other failures cover history, coordinated branches, checkpoints, review baseline, discovery, collisions, unversioned overlays, checkpoint failures, and publication. Logs: /tmp/reqvire-workspace-milestone1-final-red.log and /tmp/reqvire-test-logs/run-tElX0e. The PR test was subsequently separated from MCP push so its precondition cannot mask the PR-contract gap; rerun that case before declaring its own final acceptance. Milestone 2 remains open, including skills and website synchronization.

## 1. Settled decisions

1. Repository organization is project-specific. Reqvire must support both a single repository and a declared composition of repositories without dictating that choice.
2. **Reqvire already uses `.repo/manifest.xml` by default.** This is implemented in `selected_manifest_at` in `crates/reqvire-core/src/repo_workspace.rs`, covered by `default_selection_loads_repo_entry_without_a_root_manifest`, and specified in `system-model/ModelStructure/RepoWorkspaces.md`.
3. No duplicate root manifest or routine `--manifest` argument is required. An explicit manifest override remains an optional alternate selection.
4. Resolve Repo's entry point to its actual manifest sources, includes, overlays, and Git owners. The entry may be a wrapper or symlink; its location does not identify the owning Git history.
5. Version manifest sources in their real owning repositories. Keeping `.repo` out of the outer repository's Git tree does not remove the manifest checkout's own Git history.
6. For the user's example, the server/client monorepo is the workspace-root repository. Website and documentation remain separate declared component repositories. The manifest repository may also contain model files; eligible sources remain governed by workspace scope and exclusions. Files under `.repo` are administration and are not automatically additional model sources.
7. Public MCP management uses model branches. Worktrees and private checkout allocation remain implementation details. The same tools apply to single- and multi-repository contexts, with composition discovered automatically.
8. The system-model manager chooses the base and intended change. An explicit branch-creation tool should prepare the root and every declared component on the requested branch name, using the selected base composition.
9. Creating branches alone changes no commit IDs and therefore does not advance pins. Pins remain exact component commits, rather than feature-branch names.
10. Commits, pushes, and PR creation are separate operations. Automatic commits are opt-in; ordinary mutation does not automatically push or create PRs.
11. One coordinated PR-create call must create or reuse linked PRs for changed component repositories and the root. The user explicitly approved this behavior.
12. Change-impact review should use the selected PR's actual baseline. An explicit Git baseline overrides it; there remains one baseline selector, with no separate manifest revision flag.
13. Core change-impact analysis must not implicitly fetch or synchronize repositories. Preparation and publication operations have separate responsibilities.

The following were suggestions, not agreed decisions: an extra `branch.open` reload flag, a separate adopt tool, and a refresh mode. Do not add them to the API by assuming they were approved.

## 2. Terms and responsibilities

| Term | Meaning |
| --- | --- |
| Workspace root | The effective identifier/path root, with its owning Git repository; server/client in the example. |
| Component | A repository declared as a Repo project at a workspace-relative checkout path. |
| Model branch | A named authoring context representing the root branch and selected repository composition. |
| Manifest entry | Repo's default `.repo/manifest.xml`, or an explicitly selected alternate entry. |
| Manifest owner | The Git repository/history that versions a selected source manifest or include. This is determined from ownership, not path spelling. |
| Pin | An exact component commit recorded by the selected manifest composition. |
| Composition checkpoint | Committed evidence sufficient to recover the root model, selected manifest versions, and component commits together. |
| PR baseline | The merge-base for the selected change and its actual review target; distinct from a branch-creation base. |

The manager supplies repository layout, initialized Repo metadata, available objects, branch/base choices, and publication targets. Before authoring admission, a manually prepared composition must have matching checkouts and pins. MCP can provide explicit tools to perform that preparation; the server process need not be stopped while doing it.

MCP prepares isolated branch contexts, validates complete candidate models, routes writes to their owners, checkpoints accepted changes, and performs explicitly requested coordinated publication. Ordinary authoring never silently creates branches or imports external model changes.

## 3. Example layout and setup boundaries

```text
server-client/                  workspace-root Git checkout
  .git/
  server/
  client/
  system-model/
  .repo/
    manifest.xml                default Repo entry point
    manifests/                  Repo's manifest checkout
      ...                       selected source XML and includes
  repositories/
    website/                    separate Git repository
    user-docs/                  separate Git repository
```

The outer repository can ignore:

```gitignore
/.repo/
/repositories/
```

Each component still tracks its own files. The manifest checkout tracks its XML in its own Git repository. This layout does not require an additional XML copy at the workspace root. A project may choose to keep source manifests elsewhere, but that is not the default workflow or a requirement imposed by Reqvire.

Repo's configured manifest remote supplies the selected manifest repository. XML remote declarations and project names supply the component fetch locations; project paths supply local checkout locations. Reqvire consumes that initialized selection rather than inventing repository URLs.

One-time setup publishes the source manifest, initializes Repo using that source, synchronizes components, and records a pinned composition. `repo manifest -r` exports checked-out commit IDs; it does not commit the result. Initial declaration publication and subsequent pin publication serve different purposes and are not two repeated steps for every feature.

The precise copyable setup example must follow the chosen manifest-owner arrangement. Do not publish another example that silently moves the native manifest into the root or leaves required pins unversioned.

## 4. Planning baseline and original gaps

This section is the pre-implementation audit at the planning baseline above. It records the gaps demonstrated by milestone 1; it does not describe the current working-tree implementation. Completion evidence is recorded separately after the acceptance checks.

| Area | Implemented now | Remaining work |
| --- | --- | --- |
| Discovery | Native `.repo/manifest.xml`, includes/overlays, project ownership, cross-repository model relations. | Preserve this behavior through history, context preparation, and checkpointing. |
| Public tools | Contract version 4 exposes `reqvire.branch.list/create/open/close/refresh`; old worktree tool names were removed. | Implement composition semantics behind the branch abstraction. |
| Branch creation | Creates root contexts and prepares components with available local objects. | Select native manifest from the requested base; create named component branches instead of detached authoring checkouts. |
| Native commits | Component commits can advance pin bytes in selected native inputs. | Commit those changes in the actual manifest owner and finish a reproducible composition checkpoint. |
| Historical impact | Compares composed models when all relevant manifest inputs are root-tracked. | Remove the requirement for root-owned manifest inputs; reconstruct native historical selections using their actual owners. |
| Refresh/admission | Guarded HEAD refresh protects model/composition inputs. The new real native fixture also confirms explicit branch.open admits a prepared committed composition. | Verify invalid/pending rejection preserves the previous accepted view; clarify the existing operation rather than assume a new tool is needed. |
| Publication | Dependency commits can be pushed before the root; GitHub PR creation is root-only. | Stable component branch publication and coordinated linked PRs, including manifest-owner publication where needed. |
| Review baseline | Shared baseline selector currently defaults to `HEAD`. | Capture the actual PR baseline for review; preserve explicit `HEAD` for pending-change comparisons. |

Code anchors:

- `crates/reqvire-core/src/repo_workspace.rs`: native selection and input ownership.
- `crates/reqvire-core/src/repo_impact.rs`: `historical_model` currently rejects manifest inputs outside the root Git history and recommends a root-tracked override.
- `crates/reqvire-cli/src/mcp_repo_worktrees.rs`: native preparation copies source selections, and component preparation uses detached checkout.
- `crates/reqvire-cli/src/mcp_repo.rs`: native pins are saved without entering the owning manifest repository's Git checkpoint.
- `crates/reqvire-cli/src/mcp_head_refresh.rs`: composition drift is rejected by the existing guarded refresh policy.
- `crates/reqvire-cli/src/mcp_worktrees.rs` and `mcp_github.rs`: push ordering and root-only PR operations.

The native E2E checkpoint currently proves updated on-disk pins and no root XML copy. It does not prove a committed native manifest checkpoint or successful historical replay. Passing existing tests therefore does not establish the target lifecycle.

## 5. Prepare a working composition

The same call expresses the user's intent for either repository organization:

```json
{
  "branch": "feature/new-api",
  "base_ref": "release/1.0"
}
```

Target behavior for `reqvire.branch.create`:

1. Resolve the explicit base in the selected root repository and capture its composition checkpoint. Do not substitute the startup branch or copy that branch's current native manifest.
2. Prepare the new root branch privately from that commit. Existing user checkouts remain in place.
3. Load the manifest version belonging to the selected base, using its recorded ownership/history and native selection.
4. Resolve every component's own pinned commit. Do not apply `release/1.0` independently as a branch name in each component.
5. Prepare `feature/new-api` in every declared component at that component's pinned commit. Each is a separate Git branch with the same name.
6. Prepare isolated Repo metadata and any necessary manifest-owner branch without changing the source composition.
7. Validate the complete graph and acquire ownership before admitting an editable context.

The initial pins remain unchanged. Untouched repositories receive no new commit or PR simply because a branch exists.

To branch from an admitted working context's committed tip:

```json
{
  "branch": "feature/new-api",
  "base_ref": "HEAD",
  "from_branch": "my-working-branch"
}
```

`from_branch` identifies the source context; `branch` names the destination. Explicit bases remain explicit. Context-relative selection may omit the source only when one context makes it unambiguous. Pending accepted changes must be committed or resolved first; no uncommitted source bytes are copied.

Reject missing objects, invalid pins, branch collisions, dirty or foreign checkouts, conflicting ownership, and ambiguous selections with affected repository/path details. Preparation does not fetch, reset, stash, or switch user checkouts. Failure cleanup may remove only newly created unchanged assets; preserve existing work and report residue.

Detached commits remain suitable for read-only historical reconstruction. They are not the intended named authoring branches for this workflow.

## 6. Author and checkpoint changes

The manager or developer authors requirements through MCP and edits implementation, documentation, and tests on the prepared branches. MCP previews and validates complete model candidates before persisting accepted changes to their owning repositories.

Commit policy remains uniform:

- `--enable-mutations` enables authoring; automatic commits remain off by default.
- Without automatic commits, accepted edits remain pending until `reqvire.git.commit`.
- With `--enable-commits`, a successful nonempty mutation invokes the same checkpoint transaction.
- No-op edits and unchanged repositories create no commits.
- Accepted pending bytes must not be reported as a committed composition.

For a composition checkpoint:

1. Commit changed accepted component files on their prepared branches.
2. Advance the selected manifest declarations to the resulting exact component commits, preserving includes and supported overlays.
3. Commit changed manifest sources in their actual Git owners.
4. Complete the root checkpoint and its durable association to the selected manifest revisions when such an association is necessary.
5. Publish committed acceptance only when the required checkpoint is complete.

Do not commit Repo administration into the root or introduce duplicate XML to make this work. If root and manifest checkouts represent the same Git history, account for that identity explicitly rather than blindly producing conflicting commits from two checkouts.

Git cannot atomically commit across repositories. On failure, retain confirmed commits, pending paths, and recovery evidence; report completed, failed, and unknown outcomes per repository. Never reset confirmed work, silently discard pin updates, blindly retry unknown commits, or report the whole operation completed after saving only manifest bytes.

## 7. External work and admission

External implementation or documentation work uses the prepared branches. An active MCP context continues to serve its accepted snapshot until external changes are explicitly admitted.

For a manually prepared new composition:

1. Commit external changes in their owning repositories.
2. Intentionally select the desired component commits and update the manifest pins.
3. Commit manifest sources and complete the composition checkpoint.
4. Request validated admission of that composed branch.

Admission requires a clean committed composition, no accepted pending edits, no unresolved Git/recovery operation, matching checkouts/pins, and successful complete model validation. Failure preserves the previous accepted model and provenance. External divergence can block writes without discarding accepted reads.

The existing artifact-only refresh remains a useful safety boundary for automatic Explorer behavior: background observation must not silently import changed model inputs or composition. Explicit full admission needs a clear contract using the existing branch tools where practical.

No separate single/multi-repository mode is needed. Explicit `branch.open` performs full validated admission; `branch.refresh` retains the guarded artifact-only boundary. There is no reload flag, adopt tool, or mode.

The manager prepares pins and owner commits before admission of external work. Ordinary accepted-only `git.commit` does not silently ingest unrelated external model edits. An explicit commit can initialize composition metadata for the already admitted clean composition.

## 8. Historical impact and review

A historical comparison must recover the earlier root model, exact manifest inputs, and exact component commits as one composition. Native manifests use the same comparison interface as other supported selections.

One selected root baseline identifies the historical manifest inputs through the committed `.reqvire-composition.json` owner-revision association when root history alone is insufficient. This does not require a root XML copy or a second manifest-commit flag.

Review behavior:

- Default PR review compares against the merge-base with that PR's actual target, consistent with the CI review workflow.
- Capture or supply the PR target independently of the branch-creation `base_ref`.
- Explicit `git_commit` / `--git-commit` overrides the review baseline.
- Explicit `HEAD` compares pending changes against the accepted committed composition. After automatic commits it may correctly produce an empty diff; it is not a substitute for the whole-PR baseline.
- Do not assume `origin/main`. If review context is unavailable, the assistant asks for the intended baseline; the noninteractive fallback needs an explicit contract.
- Resolve any necessary PR metadata before the core comparison. Analysis itself uses available local inputs without fetch/sync or remote API calls.

The current side of MCP comparison is the captured accepted composition plus accepted pending changes, not arbitrary current disk content. Historical materialization must not change source checkouts, indexes, refs, pins, or accepted state.

Support added and removed projects, changed pins, relocated checkout paths, historical includes, and all supported versioned selection inputs. Path relocation alone must not become a false semantic content change. Missing historical objects or unversioned required inputs produce specific errors, not a partial report or fallback to a different manifest.

Reports retain existing impact semantics and add deterministic root/manifest/component provenance. Source links use each file's owning repository, path, and revision. Use workspace-root wording in new prose; existing compatibility fields such as `central_commit` need a deliberate compatibility decision before renaming.

## 9. Coordinated push and PR workflow

`reqvire.git.push` remains explicit and should publish the committed composition in dependency order:

1. Verify the accepted checkpoint and intended registered remotes/branches.
2. Push changed component branches and any additional refs needed to make every selected pin remotely reachable.
3. Publish a distinct manifest owner's required branch/commit when necessary.
4. Confirm dependency publication, then push the root checkpoint last.

Use stable feature branches for ongoing review. Existing SHA retention refs alone do not provide a component PR lifecycle. An unchanged component needs no PR, but any pin required by the published root must still be available remotely.

On component failure, do not attempt root publication. Preserve successful earlier pushes and report them; there is no implied rollback. Reject divergence and arbitrary remote URLs rather than force-pushing.

**Agreed:** one `reqvire.github.pr.create` call creates or reuses linked PRs for every changed component and the root. Each repository needs its actual explicit target; component targets need not match the root target or be `main`.

Create/reuse component PRs first and include their references with composition provenance in the root PR. Repeated calls reuse the exact existing PRs, avoid duplicate creation and unsolicited metadata changes, and report per-repository outcomes. A separate manifest owner may also need review/publication; its treatment must be defined with checkpoint ownership.

If squash or rebase merges change component commit IDs, update pins to the final commits, checkpoint again, and rerun validation/impact before merging the root composition. PR creation does not imply merging.

PR comments remain explicitly requested and targeted. Linked PR creation is approved; automatic comment fan-out has not been approved.

## 10. Traceability across documentation and website repositories

Define documentation obligations as requirements. Put Contract References to the server/client contracts on the responsible documentation requirements, and link relevant verification evidence and page sources through supported declarations.

Contract changes can then identify documentation requirements, verifications, and referenced pages needing review across repository boundaries. Each repository retains its own history and PR workflow. A Contract Reference propagates review impact without asserting independent implementation of its referenced contract.

Only declared, loaded sources participate in the combined model. An arbitrary external URL does not import a repository or its historical model. Do not promise semantic requirement propagation for every code-only diff without a modeled dependency that gives it that meaning.

## 11. Public interface and compatibility

The completed rename is:

| Previous tool | Current tool |
| --- | --- |
| `reqvire.worktree.list` | `reqvire.branch.list` |
| `reqvire.worktree.create` | `reqvire.branch.create` |
| `reqvire.worktree.open` | `reqvire.branch.open` |
| `reqvire.worktree.remove` | `reqvire.branch.close` |
| `reqvire.worktree.refresh_head` | `reqvire.branch.refresh` |

Contract version 4 removed the previous tool names without aliases. Existing routing arguments and legacy payload fields were retained by that rename. Internal worktree modules and test-directory names need not be renamed merely for presentation.

Extend existing operations before inventing parallel single/multi tools. Discover the composition from the selected branch. New return data should expose participating repositories, branches, accepted/observed commits, pending checkpoint state, and per-repository outcomes without leaking private checkout IDs as public routing identity.

Current `branch.close` protects the original and externally prepared checkouts, closes only eligible clean server-created contexts, and preserves Git branches/commits. Retain those protections unless a separately specified lifecycle change is needed. Closing a context does not authorize deleting a coordinated branch set.

Any incompatible argument, response, or behavior change requires a deliberate contract-version/migration decision. Do not claim version 4 already specifies the new lifecycle.

## 12. Decision register

The implementation choices at the start of this document resolve the primary representation/API choices below. Remaining work is to specify their edge cases and verify them; this table retains the concerns that motivated those choices.

| Decision | Required boundary / proposed direction |
| --- | --- |
| Historical native manifest association | Preserve native location and actual ownership. Choose durable versioned provenance linking a root checkpoint to selected manifest-owner revisions where history alone is insufficient. Do not invent a required root XML copy or extra baseline selector. |
| Manifest-owner branches | Define preparation/checkpoint/publication when the owner shares root history, uses another branch of that history, or has independent history. Avoid duplicate logical owners and circular self-pins. |
| Explicit re-admission | Reuse explicit branch.open for full validated admission; preserve guarded artifact-only refresh. No mode or extra tool. |
| External pin updates | Manager prepares external commits, exact pins and owner association before admission. Explicit commit may initialize metadata for an already admitted composition; it never imports arbitrary external model edits. |
| PR bases and results | Define per-repository target input, linked PR result shape, retry behavior, and any distinct manifest-owner PR. Root-only `base` is insufficient for arbitrary component targets. |
| Missing PR context | Recommended noninteractive behavior: actionable baseline-required guidance; assistant asks the user. Confirm compatibility with explicit local `HEAD` review. |
| Durable reopen state | Root composition association plus retained local manifest/component Git objects allow reopen without inheriting the startup branch's selection. |
| Local overlays | Unversioned local overlays cannot silently count as a reproducible historical checkpoint. Define supported versioning or reject affected historical/publication operations with the exact input path. |
| Compatibility fields | Decide additions/migration for legacy worktree/central payload names alongside the next public contract revision. |

These are technical/contract decisions within the agreed native Repo workflow. The default manifest location, root/component organization in the example, and coordinated linked PR requirement are not open questions.

## 13. System-model alignment

Use the `reqvire-syseng` workflow: ontology/capability context, requirements and owned contracts, verification planning, failing meaningful tests, implementation, and satisfaction links. Author structured model changes through typed MCP previews and applies.

| Model area | Required alignment |
| --- | --- |
| `system-model/Ontologies/WorktreeSessions.md` and relevant concepts | Model stable branch/composition/repository revision meaning where missing. Keep worktrees as implementation concepts; put commands and transitions in requirement-owned contracts. |
| `system-model/Interfaces/InterfacesFeature.md` | Review the remaining MCP Worktree Model Management capability name/scope against branch and composition management. Keep change publication and traceability responsibilities explicit. |
| `system-model/ModelStructure/RepoWorkspaces.md` | Preserve native default and owner-aware source scope; clarify checkpoint ownership without moving interface transactions into discovery. |
| `system-model/Interfaces/MCP/Tools.md` and `Specifications.md` | Replace source-manifest inheritance, detached authoring defaults, incomplete native checkpointing, and root-only PR assumptions with the agreed branch-set workflow. |
| `system-model/Interfaces/MCP/HeadRefresh.md` | Specify explicit re-admission versus guarded automatic behavior and failure preservation. |
| `system-model/Processing/ChangeImpact/ChangeImpactRequirements.md` and `Specifications.md` | Native owner-aware history, one baseline, PR-review selection, captured current composition, reproducible provenance and isolation. |
| `system-model/ModelStructure/Specifications.md` | Remove the stale separate-baseline-revision statement and align source/history responsibilities. |
| MCP and change-impact verification documents | Replace tests requiring detached/no-matching-branch or native-history rejection with positive target cases; retain legitimate unavailable-history rejection. |

Add explicit branch-preparation, composition-checkpoint, external-admission, coordinated-publication, and historical-review behaviors where current lifecycle detail is buried only in specifications. Maintain verification objectives, leaf verification coverage, and implementation/evidence links; coverage links alone do not prove behavior.

## 14. Acceptance checks

Use Bash, curl, jq, Git, real local Repo clients, local bare remotes, and existing shell MCP helpers. Python is allowed only where those tools cannot reasonably perform the required check. An existing Python harness or HTTP protocol calls are not an exemption; fixtures and assertions that can use Bash/jq must do so. Google Repo's own Python runtime remains a production dependency.

| Scenario | Required evidence |
| --- | --- |
| Native default | No root XML copy or manifest override; default loading, branch preparation, commit/reopen, and history all succeed. |
| Different creation base | Startup branch and `release/1.0` have different manifests/pins; the new branch uses the explicit base composition. |
| Context-relative base | `HEAD` with selected `from_branch` uses that committed composition; ambiguous routing and pending sources reject. |
| Branch preparation | Root and all components use the requested named branch at their own base commits; pins unchanged; source checkouts/indexes/refs unchanged. |
| Ownership and collisions | Existing branches, dirty/foreign checkouts, unavailable objects, overlapping roots, and conflicting owners reject without adopting external work. |
| Checkpoint | Changed components committed, exact native pins versioned in their owners, durable root association established; restart/reopen recovers the same graph. |
| Untouched components | Branch preparation produces no commit or PR for an untouched repository. |
| Partial/unknown commit | Failure injected at each stage preserves confirmed work and reports exact pending/recovery outcomes; no false completion or blind repeat. |
| External admission | Prepared committed composition loads only on explicit validated admission; invalid candidates and pending/recovery state retain prior accepted reads. |
| Guarded refresh | Automatic Explorer behavior cannot silently adopt changed model/composition inputs; allowed artifact refresh retains existing guarantees. |
| Historical cases | Native includes, supported overlays, independent ownership where supported, project add/remove/relocation, missing old objects, and exact revision pins. |
| Comparison isolation | Success and rejection leave source files, indexes, refs, pins, and accepted snapshots unchanged; no fetch/sync/remote API calls. |
| Review baseline | Whole-PR impact remains meaningful after automatic commits; explicit clean `HEAD` can be empty; actual target is used instead of hardcoded main. |
| Coordinated push | Stable component branches and required pins available remotely before root publication; dependency failure prevents root push and retains earlier successes. |
| Linked PR set | One call creates/reuses every required PR with explicit correct bases and root links; retries do not duplicate PRs; partial outcomes remain reviewable. |
| Final component merge | Squash/rebase final SHAs require updated pins/checkpoint and review before root merge. |
| Fresh consumer | A fresh consumer can retrieve the published composition and reproduce the combined model using native Repo selection. |
| Interface parity | CLI/MCP comparisons share core history; HTTP, stdio, and embedded paths retain routing, ownership, and commit-policy behavior. |
| Cross-repository documentation | A server/client contract change identifies the modeled website/docs obligations and their verification/page evidence. |

Define and run the new checks before implementation, retaining the expected failures as evidence of the actual gaps. Run one E2E suite per runner invocation. Do not replace behavioral failures with unavailable-tool assertions or skip native cases.

## 15. Implementation sequence and completion gate

1. Resolve the checkpoint/manifest-owner association and explicit admission contracts; record decisions here.
2. Update ontology/concepts as needed, capability scope, requirements, behaviors, specifications, verification objectives, and concrete verifications through MCP.
3. Add the native-history, native-checkpoint, coordinated-branch, and PR-set tests and demonstrate their expected failures on the current implementation.
4. Implement shared composition provenance/history resolution, then base-aware branch preparation and named component branches.
5. Complete owner-aware commits, durable checkpoints, failure reconciliation, and reopen behavior.
6. Implement intentional external admission and PR-baseline selection without weakening automatic refresh or accepted snapshot isolation.
7. Complete stable coordinated pushes, linked PR creation/reuse, and dependency failure reporting.
8. Update shared skills, README, website/workflow documentation, examples, and tool/version migration guidance to match the final contract.
9. Run relevant Rust tests and E2E suites, model validate/lint/coverage, formatting/clippy, and affected UI/website checks. Broaden testing where the cross-cutting changes require it; report precisely what ran.
10. Review a complete developer flow from native Repo initialization through a cross-repository change, checkpoint, restart, impact review, push, linked PRs, final pins, and fresh-consumer replay before releasing the repaired workflow.

Implementation completion is recorded below. This plan does not authorize automatic publication or changes to unrelated concurrent Explorer work.

## 16. Implementation and verification record

The working branch remains `refactor/mcp-branch-tools`. The system model was updated first using real typed MCP previews/applies. The failing milestone was retained before Rust implementation. Final traceability links were added through the same tools, including the composition reader, checkpoint transaction, coordinated publication implementation, and Bash acceptance checks.

Implemented boundaries:

- `repo_composition.rs` records actual manifest-owner revisions in the root association and reads native historical inputs through the shared batched Git reader. Native includes and versioned overlays retain their owners; missing history never triggers synchronization.
- `mcp_repo_worktrees.rs` prepares named component branches from the requested root base and retains actual local Git objects for reopen. A same-repository manifest owner uses its separate `reqvire/manifests/<branch>` lane.
- `mcp_repo.rs` initializes and completes owner-aware checkpoints, preserves confirmed partial commits, and reports unknown per-owner ref outcomes without blind retries. Competing owners of one logical branch reject before side effects.
- Explicit `branch.open` validates prepared committed composition changes in the live owner, preserving captured reads and accepted state on failure. Recovery-required contexts reject admission; guarded artifact refresh retains its existing boundary.
- `mcp_repo_publication.rs` uses the existing bounded Git/GitHub operations for dependency-first publication, explicit per-owner PR bases, linked create/reuse results, retained partial PR sets, and one explicitly targeted comment. No automatic push, merge, or comment fan-out was added.
- Shared review selection uses actual available PR context or one explicit baseline override. Missing context is an actionable error. Captured change-impact reads retain accepted files, HEAD and branch even when external drift or recovery blocks writes.
- Tool contract version 5, README, website workflow pages and all four canonical/installed Reqvire skills are aligned. Runtime ontology artifacts were regenerated; unrelated blank-node labels were preserved to keep the semantic diff reviewable.

Verification evidence:

- All 383 Rust workspace tests pass; formatting and clippy pass after the final production changes.
- The first full E2E run executed 103 suites (99 passed, 4 failed). It exposed a help fixture mismatch, accepted-snapshot comparison gating, stale generated ontology, and ordinary multi-worktree error ordering. Those findings were corrected. Expanded native checks also exposed misleading successful reopen of recovery-required contexts; it now rejects with the retained reason.
- The expanded native composition checks cover 23 cases, including initial checkpoint/no-op, differing release bases, shared root/manifest history, explicit/automatic commits, external admission, ownership conflicts, definite/unknown checkpoint failures, native included/overlay history, PR reuse/partial retries, targeted comments, and fresh-consumer replay.
- Explorer: all 420 UI tests pass, with type checking, design-system/style checks and production build passing. Website: full type checking and production build pass; both generated documentation routes contain the updated workflow examples.
- Model: validation reports no errors; lint has no findings; ontology export succeeds. Coverage is 211/236 verified leaf requirements with zero orphaned verifications; the 25 uncovered leaf requirements are outside this change's new verification obligation.
- Skill installer acceptance passes, and installed `.agents`/`.codex` packages were synchronized from the canonical sources.
- The sequential workspace harness was converted entirely from Python to Bash/curl/jq, retaining all 15 cases, shared server sessions, real Git/Repo fixtures, golden comparisons and browser checks. Native servers use `--port 0` instead of Python port allocation. All four model evidence links now target the shell harness. Only Google Repo's production Python runtime remains in this suite; the resolver instrumentation is itself a shell wrapper. The test guide now requires Python to be justified by a check that shell tools cannot reasonably perform.

Milestone 2 verification is complete. The final full E2E run executed all 103 suites: 102 passed, with only the workspace suite failing because of two legacy fixture assumptions (a divergent component main branch and equivalent local/file remote spellings). After correcting those fixtures, the complete workspace suite passed. After the subsequent Bash conversion and removal of Python port allocation, that complete suite passed again, including its 23 composition cases. No production code changed after the final full E2E run.

Logs: `/tmp/reqvire-workspace-complete-e2e.log`, `/tmp/reqvire-workspace-final-bash-repo-suite.log`, `/tmp/reqvire-workspace-release-rust.log`, `/tmp/reqvire-workspace-release-clippy.log`, `/tmp/reqvire-workspace-final-explorer.log`, and the final website/model logs under `/tmp/reqvire-workspace-*`.

Commit and publication of the completed working-branch changes remain explicit user operations. Test publication uses local fixtures; no release or merge was performed.

## 17. Follow-up guidance audit (2026-11-01)

The post-implementation audit aligned skills, website, developer documentation and the system model with the verified behavior:

- Removed a remaining hardcoded main baseline and implicit previous-commit choice from skill examples. Component source links now use owner-specific report provenance instead of the root URL/commit.
- Added direct Contract References from the MCP and User Guide website requirements to the composition checkpoint and historical impact contracts, so those contract changes reach the responsible page sources.
- Corrected stale refresh-only admission wording, native saves described as merely local bytes, and verification text expecting live admission to terminate captured reads. Clarified accepted pending edits versus a later multi-owner checkpoint failure.
- Documented the manifest branch lane, the root-only scope of live commit reconciliation, and composition admission for external component commits even when their files are ordinary artifacts. Added the existing manifest/project guard reason to the refresh contract.
- Added native manifest fetch/path/pin examples and a concrete existing-inventory jq update for external work. Verified that owner-revision update through real Repo/Git/MCP external admission in a temporary fixture. Preview examples now include an actual apply request before commit.
- Corrected the test-build instructions: the runner defaults to release, despite the old guide claiming debug. The user's retained failed executable matched the older release build exactly. Rebuilt release and reran the complete workspace suite through the default runner path; it passed, including accepted-pending impact reads and unknown component/manifest recovery rejection.
- Website type checking/production build, skill installer acceptance, synchronized installed skills, model validation/lint, ontology export and documentation-consumer queries pass. Branch composition and change publication retain 100% modeled verification/implementation coverage. The 25 existing unverified leaf requirements remain visible; this audit did not manufacture coverage links.

Follow-up evidence: `/tmp/reqvire-workspace-default-release-verification.log`, `/tmp/reqvire-workspace-guidance-example.log`, `/tmp/reqvire-workspace-guidance-skill-installers.log`, `/tmp/reqvire-workspace-guidance-website-build.log`, and `/tmp/reqvire-workspace-guidance-validate.log`.
