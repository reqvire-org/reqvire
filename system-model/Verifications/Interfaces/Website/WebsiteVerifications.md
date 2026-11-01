# Elements

### Public Documentation Website Verification Objective

This objective groups review of public documentation wording, examples, and navigation against the requirement contracts documented by the website.

#### Metadata
  * type: verification-objective
---

### Website Contract Binding Placement Documentation Verification

This inspection verifies that public documentation explains contract binding placement according to implementation responsibility and inherited obligation scope.

#### Details
Expected checks:
- Review requirements-and-capabilities guidance for separate child bindings when implementations fulfill an obligation independently and need separate coverage tracking.
- Confirm that parent binding guidance requires applicability to the entire requirement subtree and intentional assessment of fulfillment through that subtree's implementation roll-up. A common ancestor alone must not be presented as a reason to move bindings upward.
- Review the concrete example for independently implemented child obligations and distinguish when an entire-parent-subtree obligation warrants a parent binding.
- Confirm that descendants inherit a parent's binding and cannot repeat it, while siblings may bind the same contract independently when no ancestor already binds it.
- Review semantic-model, submodel, and implementation-coverage guidance for consistent placement and scope. A child binding must not imply applicability to unrelated siblings or the whole consuming capability-root submodel.
- Confirm that the owning website requirements reference the documented placement constraint and retain satisfaction links to their page sources.
- Build the website and inspect the affected rendered routes for the placement guidance and example.

Review artifacts: [ModelingLanguage.tsx](../../../../website/src/pages/ModelingLanguage.tsx).

#### Metadata
  * type: demonstration-verification

#### Relations
  * derivedFrom: [Public Documentation Website Verification Objective](#public-documentation-website-verification-objective)
  * verify: [Website Implementation Coverage Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-implementation-coverage-documentation)
  * verify: [Website Requirements and Contracts Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-requirements-and-contracts-documentation)
  * verify: [Website Semantic Model Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-semantic-model-documentation)
---

### Website Contract Reference Documentation Verification

This inspection verifies that public documentation distinguishes contract implementation obligations from content dependencies and follows the referenced modeling and interface contracts.

#### Details
Expected checks:
- Review the modeling-language overview and examples for the canonical Contract Bindings and Contract References subsection names, requirement-only sources, supported owned contract targets, element-wide section exclusivity, and acyclic reference dependencies.
- Confirm that contract dependency is defined as the umbrella term for bindings and references, and that references propagate change impact without contributing to the contract owner's implementation fulfillment.
- Review requirements-and-contracts, semantic-model, and workflow guidance for the distinction between implementation obligations and change-impact dependencies. Reference-only consumers do not contribute implementation coverage or evidence to the contract owner.
- Confirm that verification descriptions identify requirements as targets and capabilities as receiving coverage through roll-up.
- Review the MCP server page for has_contract_references and filter_contract_references, normalized target matching, filter composition, and a concrete search example consistent with the referenced MCP model-evidence and search-filtering contracts.
- Confirm that the owning documentation requirements reference the contracts they describe, so contract changes reach documentation through change-impact analysis.
- Build and inspect the affected rendered routes and check that examples agree with validated model behavior before publication.

Review artifacts: [ModelingLanguage.tsx](../../../../website/src/pages/ModelingLanguage.tsx).

#### Metadata
  * type: demonstration-verification

#### Relations
  * derivedFrom: [Public Documentation Website Verification Objective](#public-documentation-website-verification-objective)
  * verify: [Website Assistant Integration Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-assistant-integration-documentation)
  * verify: [Website Command and Workflow Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-command-and-workflow-documentation)
  * verify: [Website Modeling Language Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-modeling-language-documentation)
  * verify: [Website Requirements and Contracts Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-requirements-and-contracts-documentation)
  * verify: [Website Semantic Model Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-semantic-model-documentation)
---

### Website Explorer Trace Flow Documentation Verification

This inspection verifies the Serve Explorer user guide against the native trace rendering contract.

#### Details
- Review the rendered guide for verification/file selection, native requirement ancestry and owning capability context, direction and branch controls, element-detail/source navigation, and availability in served and exported Explorer.
- Review shareable view selections, copied/reloaded links, browser history, explicit-link precedence, unavailable-selection fallback and the worktree spinner/success/error-dismissal flow against the shared selection and worktree contracts.
- Compare the wording with the Explorer Verification Trace Rendering Specification referenced by the owning website requirement and the compiled production trace integration checks.
- Build the website and inspect the rendered user-guide route before publication.

Review artifact: [UserGuide.tsx](../../../../website/src/pages/UserGuide.tsx).

#### Metadata
  * type: inspection-verification

#### Relations
  * derivedFrom: [Public Documentation Website Verification Objective](#public-documentation-website-verification-objective)
  * verify: [Website Command and Workflow Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-command-and-workflow-documentation)
---

### Website MCP HTTP Configuration Documentation Verification

This inspection verifies that endpoint-host and browser-origin setup examples match MCP startup and transport behavior.

#### Details
- Review standalone and embedded command examples for repeatable `--allow-origin`, the embedded `--enable-mcp` prerequisite, and valid HTTP(S) origins without paths.
- Confirm that the page explains additive loopback defaults, exact scheme/host/port matching, and rejection of unlisted origins.
- Confirm that the page distinguishes browser origin configuration from bind address, endpoint URL, and authentication.
- Review direct-host and reverse-proxy examples for automatic bind-host acceptance, explicit allowed hostnames with optional ports, wildcard listener behavior, and equivalent embedded MCP options.
- Build the website and check the examples against command help and the transport verification results.
- Review mutation examples for default-disabled automatic commits, explicit `--enable-commits`, standalone/embedded prerequisites, unchanged ownership, and resolution of uncommitted changes before a clean restart.
- Review stdio startup and client configuration across MCP Server, Coding Assistants, User Guide and Integrations: HTTP default, explicit `--transport stdio`, absolute workspace selection, shared permissions, rejected HTTP-only flags, protocol-only stdout and preservation of pending work on shutdown. Compare with the passing native/npm transport and lifecycle acceptance cases.
- Review full-definition override guidance on MCP Server and User Guide: retained incoming children and consumers, replacement of authored metadata/outgoing declarations, preview/application, cross-file rewrites, invalid-candidate preservation and actual deletion's orphan protection. Compare with the override acceptance matrix.

Review artifacts: [McpServer.tsx](../../../../website/src/pages/McpServer.tsx).

#### Metadata
  * type: demonstration-verification

#### Relations
  * derivedFrom: [Public Documentation Website Verification Objective](#public-documentation-website-verification-objective)
  * verify: [Website Assistant Integration Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-assistant-integration-documentation)
---

### Website MCP Worktree and Publication Documentation Verification

This review verifies the documented worktree and publication workflow against implemented tool contracts and Explorer behavior before public release.

#### Details
- Review standalone and embedded startup examples, flag prerequisites/defaults, configured remote selection, failed gh checks, and operation-time permission errors against command help and discovery fixtures.
- Follow examples from explicit base to worktree creation/opening, context-scoped model reads/mutations, manual commit, push, explicit-base PR creation, and comment. Check same-repository and no-fork rules, no implicit effects, and recovery from uncertain outcomes. Check the documented distinction between accepted snapshot reads, blocked writes/publication, unavailable local file downloads, and repair/restart after mutation recovery failure.
- Review branch-selected tool/prompt/resource examples, `from_branch` for relative creation bases, version-5 composition-contract guidance, optional single-context selection and required multi-context selection. Explain explicit admission, shared concurrency protections for clients on one branch, percent-encoded resource branch names, rejected obsolete MCP ID inputs, and private runtime/cache identity and public branch-selected responses, picker labels, browser URLs and HTTP routes. Compare examples with executable acceptance cases; source/build checks alone are not runtime evidence.
- Verify documentation presents one branch picker, metadata-only inventory, and on-demand loading with worktree reuse or creation handled internally. Explain read-only dirty-content support and existing cache freshness, mutation-enabled rejection of dirty targets before first load, normal ownership admission, accepted-snapshot reuse on reselection including accepted uncommitted writes, and worktree preservation on shutdown. The examples must not imply eager loading of all worktrees, cache resets on switching, or a read-only bypass in mutation mode. Clean restart, accepted-only commits, and managed-worktree removal retain their existing rules. Two tabs independently select branches without changing another client's routing or requiring an MCP switching tool.
- Build and inspect the MCP documentation route only after implementation. Do not mark this planned review completed by adding source-file links as executable evidence.
- Follow HEAD refresh apply and check-only examples against real discovery. Review accepted/observed provenance, same-branch amendments, protected new documents/semantic resolution, mutable evidence, clean/pending restrictions, default HEAD selection, shared Refresh control and labelled retained-model rejection. Confirm ignore-file edits are allowed, current rules govern eligibility and declaring owners, successful refresh adopts those rules for later tools, preview/rejection retain accepted rules, and policy-only membership changes can change the model revision. Compare the public examples with the passing owning acceptance checks.
- Check restart guidance distinguishes reusable leftover lock files from live ownership, permission denial and unsupported locking, without advising lock deletion or automatic permission changes.

- Follow the native server/client-plus-website/docs example without a root manifest copy or manifest override. Confirm source XML is versioned in its actual owner, root JSON references the correct owner commits, and ignored administration remains outside root tracking.
- Follow coordinated branch creation from a differing release base, explicit/automatic owner-aware checkpoints, restart, external validated admission/rejection, actual PR baseline selection, stable pushes and linked PR creation with explicit per-owner bases. Compare examples with passing behavioral checks and inspect the built MCP/User Guide routes. Source text and production builds alone do not prove the workflow.

- Inspect external checkpoint examples against schema version 1: source inventory remains complete, changed owner commits are recorded, root commit has no circular self-pin, and branch.open follows clean committed preparation. Confirm detached preparation is distinguished from named authoring, preview is followed by an actual apply, and root live reconciliation is not promised for unknown component/manifest refs.

Verify both model-bearing and manifest-only central repository examples use the same root-owned source manifest branch and PR. Source XML remains tracked outside ignored Repo administration; examples do not create a separate manifest lane, cached-manifest commit/PR or manual root self-association. External-work instructions update tracked source pins before explicit admission.

#### Metadata
  * type: demonstration-verification

#### Relations
  * derivedFrom: [Public Documentation Website Verification Objective](#public-documentation-website-verification-objective)
  * verify: [Website Assistant Integration Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-assistant-integration-documentation)
---

### Website Managed Query Documentation Verification

Inspect the native query authoring and CLI/MCP artifact examples against their shared core contracts.

#### Details
Check Query and optional Produces syntax, explicit ontology use and prefixes, generated identity/form, namespace selection, exact-byte export/hash checks, and downstream runtime behavior. Build the website and compare documented commands with CLI help and managed query verification results.

#### Metadata
  * type: demonstration-verification

#### Relations
  * derivedFrom: [Public Documentation Website Verification Objective](#public-documentation-website-verification-objective)
  * verify: [Website Ontology Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-ontology-documentation)
---

### Website Scoped Coverage Documentation Verification

This inspection verifies that public documentation describes scoped coverage consistently across command workflows, assistant integration, implementation coverage, verification, and submodel guidance.

#### Details
Expected checks:
- Review rendered user-guide and advanced-workflow pages for whole-model defaults, root and nested capability selection, `coverage --from <NAME>` examples in text and JSON modes, and links to the detailed coverage guidance.
- Check the documented CLI examples against implemented command help and representative model output. A requirement name accepted by `submodels --from` must not be presented as valid for capability-only coverage scope.
- Review the MCP server page for the optional `from` tool argument, a concrete `reqvire.coverage` request example, invalid-selector behavior, whole-model default, and shared CLI/MCP result semantics.
- Review implementation-coverage guidance for a cross-submodel contract-consumption example. The selected requirement retains its whole-model source and evidence while the external consumer stays outside scoped membership and counts; the text must not claim direct satisfaction or verification from that reuse alone.
- Check terminal versus verification-leaf definitions, recursive all-child and all-consumer rules, combined obligations, and the inability of direct parent evidence to override a gap. Check terminal percentage denominators, the zero-terminal scoped example, current source tokens, and partial evidence and blocker fields against representative report output.
- Confirm that coverage guidance identifies combined hierarchy and contract-consumer cycles as validation errors before reporting, including same-root cycles with direct evidence.
- Review verification guidance for requirement-only targets, capability roll-up, objective exclusion, shared-verification deduplication within each scope and globally, non-additive submodel verification totals, and whole-model-only orphan diagnostics.
- Review Explorer guidance for one complete whole-model or capability-scoped dashboard, detailed scoped issues below the drill-down, no mode selector or View issues action, the selectable Whole Model tree root, scope-only URLs and migration of retired mode links, left capability navigation with alphabetical roots and siblings, keyboard selection and independent disclosure, scoped page titles, ranked hierarchical capability rows without a display switch, capability and requirement expansion for nested child rows, terminal requirement metrics and element-detail links, labelled coverage bars without redundant state text, labelled Binding consumers links identifying requirements responsible for shared contract obligations, blocker counts beside requirement metrics, and access to implementation evidence through requirement element details, coherent dashboard/sidebar counts, and the action that opens whole-model orphan diagnostics.
- Review semantic-model and submodel guidance for validated ownership, evidence crossing scope without transferring membership, and the distinction between capability-scoped coverage and the submodels command's capability/requirement scope rules.
- Review the rendered Coverage user guide for scope sharing through the browser URL, Back/Forward restoration, explicit URL precedence, separate branch preferences, and explained fallback when the selected capability becomes unavailable. Verify the Whole Model title and selectable scope root, capabilities nested below it, the persistent header branch picker, and visible compact type/source breakdowns below metrics. Verification types use one bar plus five exact labelled counts including zeros, without a Report details disclosure.
- Build and inspect the affected public documentation routes, confirm navigation links resolve, and ensure examples and descriptions agree with the implemented feature before publication.

Review artifacts: [Advanced.tsx](../../../../website/src/pages/Advanced.tsx), [UserGuide.tsx](../../../../website/src/pages/UserGuide.tsx).

#### Metadata
  * type: demonstration-verification

#### Relations
  * derivedFrom: [Public Documentation Website Verification Objective](#public-documentation-website-verification-objective)
  * verify: [Website Assistant Integration Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-assistant-integration-documentation)
  * verify: [Website Command and Workflow Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-command-and-workflow-documentation)
  * verify: [Website Implementation Coverage Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-implementation-coverage-documentation)
  * verify: [Website Semantic Model Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-semantic-model-documentation)
  * verify: [Website Verification Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-verification-documentation)
---
