# Elements

### MCP Worktree HEAD Refresh

WHEN a client requests HEAD refresh for an owned worktree context, the system SHALL check the observed local branch tip against the accepted model's protected inputs and, if eligible, reload that committed tip through the normal validated model-loading path.

#### Details
The operation retains context identity and ownership, supports external implementation commits and same-branch amendments, and restores ordinary mutation availability only after a complete successful adoption. It creates no commit and remains usable with automatic commits disabled. Protected model changes require full validation through the normal admission workflow.

#### Concept References
  * [HEAD Refresh](../../Thesaurus/Thesaurus.md#head-refresh)
  * [Accepted HEAD](../../Thesaurus/Thesaurus.md#accepted-head)
  * [Observed HEAD](../../Thesaurus/Thesaurus.md#observed-head)
  * [Protected Model Input](../../Thesaurus/Thesaurus.md#protected-model-input)

#### Metadata
  * type: requirement

#### Contract References
  * [MCP Mutation Concurrency Control Specification](Specifications.md#mcp-mutation-concurrency-control-specification)
  * [MCP Worktree Context Isolation Specification](Specifications.md#mcp-worktree-context-isolation-specification)
  * [Local External Ontology Source Specification](../../Semantics/SemanticModelSpecifications.md#local-external-ontology-source-specification)

#### Relations
  * derivedFrom: [MCP Worktree Model Sessions](Tools.md#mcp-worktree-model-sessions)
  * definedBy: [MCP Worktree HEAD Refresh Specification](#mcp-worktree-head-refresh-specification)
  * definedBy: [MCP HEAD Refresh Input Protection Specification](#mcp-head-refresh-input-protection-specification)
  * satisfiedBy: [mcp_head_refresh.rs](../../../crates/reqvire-cli/src/mcp_head_refresh.rs)
  * satisfiedBy: [mcp_worktrees.rs](../../../crates/reqvire-cli/src/mcp_worktrees.rs)
  * satisfiedBy: [git_commands.rs](../../../crates/reqvire-core/src/git_commands.rs)
  * satisfiedBy: [model_inputs.rs](../../../crates/reqvire-core/src/model_inputs.rs)
---

### MCP Worktree HEAD Refresh Specification

Defines the typed control operation, eligibility boundary and guarded model reload for adopting an external branch HEAD.

#### Details
Interface:
- Expose `reqvire.worktree.refresh_head` on standalone HTTP, stdio and embedded MCP with mutations enabled. It is a context-state mutation, not a read-only tool, and does not require `--enable-commits`.
- Accept `worktree_id` under the existing context-selection rules and optional `dry_run: boolean`, default `false`. There is no arbitrary path, branch-changing, force or validation-bypass argument. No new shell CLI subcommand is required.
- Every apply performs the complete eligibility check. `dry_run: true` returns a check result without changing accepted HEAD, captured files, pending paths, semantic state or Explorer publication. A successful preview does not authorize a later changed candidate.
- Return bounded structured fields `outcome`, `can_refresh`, `dry_run`, `worktree_id`, `branch`, `previous_head`, `observed_head`, `accepted_head`, `model_revision`, changed-file count and `blockers`; include `publication_revision` when serving. Outcomes are `refreshed`, `unchanged`, `checked` and `blocked`. HEAD fields are commit object IDs, not model revisions.
- A policy rejection is a tool-execution result with `outcome: blocked`, `can_refresh: false`, the unchanged accepted HEAD and blocker entries containing `reason`, affected `path` when applicable, and a next action. It MUST NOT be mistaken for an unavailable-tool or invalid-schema protocol error. Blocker reason tokens are `protected_model_source`, `semantic_source_changed`, `semantic_resolution_changed`, `missing_relation_target`, `pending_changes`, `dirty_worktree`, `branch_changed`, `git_operation_in_progress`, `recovery_required`. Bound large lists with explicit truncation/continuation information; do not return model bodies or a complete diff to report eligibility.

Eligibility and execution:
- Retain the same live owned worker, branch, worktree root and opaque `worktree_id`. Route this control operation despite an external HEAD mismatch, without bypassing ownership or transaction/commit recovery. Other unavailable-context errors retain the shared routing contract.
- Require no accepted pending model changes, then require a clean index/worktree including nonignored untracked files. Reject another branch, detached HEAD, an in-progress merge/rebase/cherry-pick/revert and unresolved recovery. Pending accepted changes take the `pending_changes` diagnostic even if their persisted files also make the tree dirty.
- Compare the accepted HEAD tree directly with the observed HEAD tree, using literal paths and NUL-delimited change records. Check additions, modifications, deletions and mode/type changes; treat a rename as deletion plus addition. Do not compare against a merge base or audit intermediate reverted changes. Same-branch amendments and other non-fast-forward endpoint changes are eligible when all checks pass.
- Run the guard once per trigger against the captured accepted/observed commit IDs using the existing context control gate. Commits are immutable. A later invocation resolves HEAD again; no HEAD-refresh-specific prepare/adopt protocol or repeated Git race checks are required.
- After the guard passes, use the same validated snapshot-loading function as normal startup. Reload captured files, parsed graph, pages, semantic store, input observations and committed baseline; retain the owned context and its ID. Recompute the semantic model revision. It remains equal for artifact-only changes with unchanged effective model membership; a permitted exclusion-policy change may change model membership and its revision. Keep pending paths empty.
- In serving mode, use the existing worker runtime-generation and parent publication path after loading, as normal startup does. A guard or loading failure retains the accepted model. Runtime-generation/publication failures follow the existing runtime-diagnostic contract; do not introduce a separate distributed publication transaction for HEAD refresh.
- On success retain worker/context identity and ownership, adopt accepted HEAD, and enable ordinary writes under the existing permission/recovery rules. Update Explorer publication identity when Git metadata or displayed artifact content changes. Rejected guards retain the accepted snapshot and its original provenance; status and retained model reads remain available while external HEAD staleness blocks ordinary writes.
- Unchanged HEAD and relevant observations return `unchanged` without model rebuild or worker restart. Inventory and polling observe staleness but do not adopt it. No operation in this flow fetches, pulls, creates a commit, writes working files or index entries, updates Git refs, switches branches, stashes or resets changes. Observational Git calls preserve the index, including optional-lock policy.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Worktree HEAD Refresh](#mcp-worktree-head-refresh)
---

### MCP HEAD Refresh Input Protection Specification

Defines how protected model inputs and mutable referenced artifacts are distinguished, including inputs absent from the accepted element registry.

#### Details
Accepted dependency manifest:
- Record recognised model-document paths, including empty model documents; actual consumed semantic-source paths/content identity; source-resolution observations, including absent candidates; local relation-target existence/eligibility; and the declaring model document for semantic/existence dependencies. Exclusion files are selection inputs, not automatic blocking inputs. Refresh the manifest after every accepted model mutation that can change those dependencies.
- Reuse the actual scanner, model-file detector, source resolver and validation observations. Classify observations by role. Neither all tracked `SnapshotFiles` entries nor all content reads made while scanning Markdown form the protected set: those collections also contain ordinary artifacts.
- A file may have multiple roles. Protected source/resolution roles take precedence over a `satisfiedBy`, evidence or display role. No RDF extension, directory name or resource link alone establishes a semantic-input role.

Protected content and additions:
- Reject every endpoint diff touching a recognised eligible model document, even a formatting-only edit, removal, rename or mode change. This covers all element types and model documents without registered elements. Do not use semantic equivalence as a lightweight exception, including an external commit of accepted but uncommitted model edits.
- Inspect both eligible old and new versions of changed `.md` candidates using the normal `detect_model_file_type` logic under the freshly loaded current scope/exclusion policy, using the same shared exclusion matcher as normal startup. The first H1 must be exactly `# Elements` or `# Element`. A newly added document, an empty recognised document and conversion of ordinary documentation to such a document all block refresh, even without incoming references. Removing the header or moving/deleting an old source cannot evade protection.
- Ordinary Markdown whose first H1 remains unsupported is mutable; model examples later in that document do not change its classification. An excluded model-looking document is mutable unless another consumed semantic-input role protects it. Addition/removal/change of root `.gitignore` or `.reqvireignore` is allowed. Read current rules before classifying the Git endpoint diff and use those same rules for the normal reload and subsequent MCP operations. A changed document excluded by current rules is not protected solely by its old membership; a newly included changed model document is protected. A policy-only commit may include/exclude otherwise unchanged documents through normal validated loading.
- Determine dependency roles from declaring model files that are eligible under the current rules. If every declaring owner is excluded, its old semantic/existence dependency does not independently block refresh; any remaining eligible owner retains protection. Reject changes to local sources consumed by eligible External Ontology declarations. New declarations already require a changed/new protected model document. An unreferenced new Turtle/RDF/JSON-LD file is mutable and is not automatically imported.
- Recheck source-resolution observations, not only final resolved paths. In particular, adding a repository-root-relative source that shadows an existing document-relative fallback blocks refresh, even when the new bytes are equal. A previously absent candidate therefore remains a protected resolution dependency.
- Git endpoint comparison is insufficient for ignored, untracked or out-of-worktree local semantic sources already consumed under existing loader rules. Verify their accepted content and resolution observations directly and block drift without broadening eligibility or fetching network sources.

Mutable artifacts and references:
- Allow ordinary implementation files, test scripts/evidence, non-model documentation, build/CI/application configuration, images/attachments and generated output when they have no protected role. This includes content changes to `satisfiedBy` targets.
- Preserve local reference-target existence and workspace eligibility using the current target-validation semantics. Removal/rename of a still-declared target blocks with its declared path; mutable target content is not frozen. Referenced resource source text may change Explorer publication identity without changing the semantic model revision.

#### Metadata
  * type: specification

#### Relations
  * define: [MCP Worktree HEAD Refresh](#mcp-worktree-head-refresh)
