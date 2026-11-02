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

### Workspace Change Impact Baseline Specification

#### Details
**Baseline selection**
Use one explicit --git-commit / git_commit override when supplied. Otherwise use the actual available PR target and its local merge-base. If no review context exists, return review_baseline_required and require the intended baseline or explicit HEAD; never silently choose main. Resolve the selected root ref once, independently of component refs. Ordinary single-repository scope keeps its existing comparison behavior.

**Shared composition access**
Request the complete baseline repository selections and native manifest provenance from the workspace service. Construct model sources, semantic dependencies, eligible artifacts and ignore rules under that returned inventory at each repository's own exact commit. Do not parse another manifest, use live source bytes in place of the selected root tree or reimplement object-store/checkout discovery.

The owner may contain no model elements. Added, removed, relocated and changed-identity components participate from their respective complete snapshots. Missing source history, unsupported input, mutable pins, foreign identities, invalid paths and unavailable commits fail with the affected source/repository/path; do not drop unresolved components or introduce a second baseline selector.

**Read isolation**
Graph construction and analysis use captured sources or isolated temporary materialization supplied through the shared boundary. Never fetch, sync, switch active checkouts, change refs/indexes/source bytes/pins or adopt external edits. Preserve accepted current model, revision, pending changes and write availability after failed comparisons.

MCP comparisons use only the selected context's accepted model and captured current provenance, including accepted pending edits. Explicit HEAD compares that pending state to the captured committed composition and may be empty after automatic commits. Baseline selection metadata may come from CI or an already confirmed PR context; the analysis operation itself does not contact remote services.

Preserve existing diagnostic roles, including repo_impact_manifest_history_unavailable, repo_impact_base_unavailable and repo_impact_base_unlocked, with native-source preparation guidance. The representation change does not rename public errors or weaken existing historical model validation.

#### Metadata
  * type: specification

#### Relations
  * define: [Manifest-Based Change Impact Baselines](ChangeImpactRequirements.md#manifest-based-change-impact-baselines)
---

### Workspace Change Impact Reporting Specification

#### Details
- Apply the existing added, changed, removed, relocated, impact-scope, invalidated-verification and smart-filtering rules to the complete base and current graphs. Repository boundaries MUST NOT interrupt hierarchy, contract-binding, contract-reference, ontology, semantic-contract, concept, verification or local-artifact impact paths.
- A contract changed in a server repository must reach a referencing documentation requirement in another repository, its verification and its eligible page source. Keep contract-reference and binding edge provenance distinct; references do not add implementation obligations.
- A project path move with unchanged elements uses existing stable-name relocation semantics. It MUST NOT create false content changes or add/remove pairs merely because workspace-relative identifiers changed. Added and removed project elements use their respective complete snapshots.
- Preserve existing report fields. Manifest-defined comparison JSON retains `comparison` with `workspace_mode: "repo"` and `base` / `current` records. Each record contains `central_commit`, sorted `manifest_inputs` records with workspace-relative `path` and owning `commit`, and sorted `projects` records with `name`, workspace-relative `path`, repository identity `repository` and resolved `commit`. Sort manifest inputs by path and projects by path, name and repository identity.
- Current provenance identifies the accepted/current source commits even when model inputs include permitted pending changes. It MUST NOT imply those pending bytes are committed; existing context/pending-state evidence remains authoritative.
- Text reports include an equivalent composition provenance summary. All model identifiers, evidence paths and relocation paths remain workspace-relative.
- Resolve source links using the relevant side's owning repository, repository-relative source path and commit. Removed source links use base provenance; changed/added/impacted source links use current provenance. Do not prefix component paths with root's GitHub identity or commit. When a repository URL cannot be mapped to a supported web URL, retain a workspace-relative link without inventing a remote URL.
- CLI and MCP use the shared single-selector baseline and comparison semantics. MCP discovery exposes optional `git_commit` only for baseline selection; it does not add another repository revision selector or reinterpret the selected context branch. Embedded MCP uses the same accepted-snapshot semantics.
- The structural comparison vocabulary describes snapshots and repository revision selections; it does not require runtime session/provenance RDF instances to be exported.

Root-owned source manifest inputs use canonical tracked source paths and the corresponding root commit in base/current provenance. Derived cache paths and checkout HEADs must not masquerade as separate authored sources or produce incorrect blob links. Reports remain complete when every model element belongs to pinned component repositories and the root contributes only source manifest/configuration files.

The existing workspace_mode value repo and legacy-named provenance fields remain wire-compatible labels for this comparison. They do not require Google Repo or XML input. The workspace service supplies the native manifest path, owning source path and component selections; this report consumes that provenance without implementing workspace management.

#### Metadata
  * type: specification

#### Relations
  * define: [Cross-Repository Change Impact Reporting](ChangeImpactRequirements.md#cross-repository-change-impact-reporting)
---
