# Elements

### Contract Reference Change Impact Specification

Contract changes propagate review impact to explicitly referencing requirements.

#### Details
Content changes to a referenced contract MUST reach its referencing requirements and their normal downstream requirement, artifact, and verification context. Reports MUST identify the traversed edge as `contract_references` and keep it distinct from `contract_bindings`.

A requirement's outgoing Contract Reference MUST NOT propagate its changes back to the referenced contract or owner. Adding or removing a reference MUST be reported as a change to the referencing requirement. Identifier-only relocation MUST preserve existing relocation semantics without treating unchanged contract content as changed.

Build the reverse lookup from referenced contract identifier to referencing requirements once for each change-impact operation, visiting each current-model element and authored reference once. Reuse that lookup across changed, relocated-and-edited, and added roots and recursive traversal. Keep consumer order deterministic and do not include Contract Bindings as reference edges. Each operation uses its own current model; edits, removals, and relocations must not reuse a prior lookup.

Impact traversal MUST deduplicate assessment of reached elements across converging acyclic paths while preserving reference-edge provenance. A `contract_references` edge to a consumer MUST remain identifiable when the consumer is also reached through the contract owner's requirement hierarchy. Invalidated verification lists MUST contain each reached verification once. Cyclic references MUST fail model validation before change-impact reporting. Implementation coverage MUST remain independent of this review traversal.

#### Metadata
  * type: specification
---

### Impact Scope Computation Specification

Algorithm for computing the minimal set of common parent requirements that cover all impacted elements in a change impact report.

#### Details
**Purpose:**
Provide reviewers with a high-level scope summary by finding the per-branch lowest common ancestors of all impacted requirements.

**Input Collection:**
1. Collect requirement element IDs from `changed` and `added` sections of the report
2. For `removed` elements: find their `derivedFrom` parent in the reference registry. If that parent still exists in the current registry, include it in the input set

**Bottom-up Merge Algorithm:**
1. Start with input set S of element IDs
2. For each element in S, resolve its immediate parent via `derivedFrom` relation in the current registry
3. Group elements by parent
4. For each parent with 2+ children in S: remove those children from S, add the parent
5. For parents with 1 child in S: keep the child as-is (no merging benefit)
6. Repeat steps 2-5 until S is stable (no changes in an iteration)
7. Return S sorted by element_id

**Edge Cases:**
- Elements with no `derivedFrom` parent: remain in S as-is (they are root scope)
- Deleted element whose parent is also deleted: walk up reference hierarchy until a parent existing in current model is found, or exclude
- Circular references: maintain visited set to prevent infinite loops

**Output:**
- Text: `### Impact Scope` section with bulleted list of impacted capability or requirement scope roots with links
- JSON: `"impact_scope"` array of objects with `name` and `element_id` fields

#### Metadata
  * type: specification
---

### Repo Change Impact Baseline Specification

#### Details
**Selection**
- In ordinary Git mode retain the single-eligible-worktree behavior and existing `--git-commit` / `git_commit` default of HEAD. An ordinary multi-worktree workspace without a Repo composition remains ambiguous and MUST be rejected.
- In Repo mode `--git-commit` / `git_commit` selects Central's base commit, defaulting to its accepted HEAD. Resolve it once to a locally available commit; never apply this ref independently to components.
- Read the selected manifest path and every Central-owned include from that base Central tree, even when its files are absent from the current tree. The default Repo selection and explicit `--manifest` location retain their existing ownership rules.
- Add optional change-impact argument `--base-manifest-commit <ref>`, with MCP field `base_manifest_commit`, for a selected manifest owned by Repo's separate manifest Git checkout. It selects a locally available commit in that owning repository. Require it when Central history cannot identify the historical manifest inputs. Reject this argument for Central-owned manifests and ordinary Git mode. Do not silently reuse the current native manifest as a historical base.
- Resolve all base includes and overlays from their identified owning historical trees. If an input is unversioned or its historical version cannot be identified, fail with the input path; never mix historical declarations with live inputs.
- Require historical component revisions to be full commit object IDs. Reject mutable branch/tag selections in a baseline instead of guessing their historical value. Current composition revisions may use the admitted locally resolved commits.
- Support Repo's existing include, remote, revision precedence and ownership semantics. Do not introduce a second manifest format or reinterpret Central's commit as manifest-project history.

**Complete historical scope**
- Materialize Central and every base-declared project's model, semantic dependencies and eligible local artifacts at its own commit, under that composition's workspace-relative paths and ignore rules.
- Discover base projects independently of the current project inventory. Added, removed, renamed-path and changed-identity projects must not be silently omitted. A removed project's objects may come from retained local Repo object storage even when no live checkout remains.
- Require repository identity, path containment, non-overlap and locally available objects for each historical selection. Fail before reporting on missing commits, unavailable removed-project objects, invalid manifests, identity conflicts or path escapes, with the affected manifest/project/path/revision.
- Compare two complete workspace graphs. Preserve existing model validation, including the existing historical compatibility policy; never construct a partial baseline by dropping unresolved projects or relations.

**Read isolation**
- Reconstruction MUST use captured inputs or isolated temporary materialization; it MUST NOT checkout/reset active repositories, change HEADs, indexes, working bytes, source-selection Repo administration, manifest pins or accepted MCP state. Operational trace logs are outside source-state preservation.
- Never fetch, sync or contact remotes. Temporary resources must be cleaned after success and failure.
- CLI current scope retains its normal valid working-tree behavior. MCP uses only the selected context's accepted current model, captured manifest/project provenance and accepted pending changes. External file/manifest changes cannot be imported by an impact request.
- A base reconstruction failure returns a read error while preserving the accepted model, revision, pending changes and write availability.



**Diagnostics**
Missing native manifest history reports `base_manifest_commit_required` with the selected manifest-project path. An unavailable historical project commit reports `repo_impact_base_unavailable`; a mutable historical component revision reports `repo_impact_base_unlocked`. Both project errors identify the project name, workspace-relative path and requested revision. Repo-generated selection wrappers locate the native manifest inputs; their bytes are not substituted for the versioned declarations or required to have a Git history.

#### Metadata
  * type: specification

#### Relations
  * define: [Manifest-Based Change Impact Baselines](ChangeImpactRequirements.md#manifest-based-change-impact-baselines)
---

### Repo Change Impact Reporting Specification

#### Details
- Apply the existing added, changed, removed, relocated, impact-scope, invalidated-verification and smart-filtering rules to the complete base and current graphs. Repository boundaries MUST NOT interrupt hierarchy, contract-binding, contract-reference, ontology, semantic-contract, concept, verification or local-artifact impact paths.
- A contract changed in a server repository must reach a referencing documentation requirement in another repository, its verification and its eligible page source. Keep contract-reference and binding edge provenance distinct; references do not add implementation obligations.
- A project path move with unchanged elements uses existing stable-name relocation semantics. It MUST NOT create false content changes or add/remove pairs merely because workspace-relative identifiers changed. Added and removed project elements use their respective complete snapshots.
- Preserve existing report fields. Repo-mode JSON adds `comparison` with `workspace_mode: "repo"` and `base` / `current` records. Each record contains `central_commit`, sorted `manifest_inputs` records with workspace-relative `path` and owning `commit`, and sorted `projects` records with `name`, workspace-relative `path`, repository identity `repository` and resolved `commit`. Sort manifest inputs by path and projects by path, name and repository identity.
- Current provenance identifies the accepted/current source commits even when model inputs include permitted pending changes. It MUST NOT imply those pending bytes are committed; existing context/pending-state evidence remains authoritative.
- Text reports include an equivalent composition provenance summary. All model identifiers, evidence paths and relocation paths remain workspace-relative.
- Resolve source links using the relevant side's owning repository, repository-relative source path and commit. Removed source links use base provenance; changed/added/impacted source links use current provenance. Do not prefix component paths with Central's GitHub identity or commit. When a repository URL cannot be mapped to a supported web URL, retain a workspace-relative link without inventing a remote URL.
- CLI and MCP use the shared baseline and comparison semantics. MCP discovery exposes optional `base_manifest_commit`; it does not add a component selector or reinterpret the context's Central branch. Embedded MCP uses the same accepted-snapshot semantics.
- The structural comparison vocabulary describes snapshots and repository revision selections; it does not require runtime session/provenance RDF instances to be exported.

#### Metadata
  * type: specification

#### Relations
  * define: [Cross-Repository Change Impact Reporting](ChangeImpactRequirements.md#cross-repository-change-impact-reporting)
---
