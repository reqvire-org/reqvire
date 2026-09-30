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

#### Metadata
  * type: inspection-verification

#### Relations
  * derivedFrom: [Public Documentation Website Verification Objective](#public-documentation-website-verification-objective)
  * verify: [Website Implementation Coverage Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-implementation-coverage-documentation)
  * verify: [Website Requirements and Contracts Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-requirements-and-contracts-documentation)
  * verify: [Website Semantic Model Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-semantic-model-documentation)
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
- Review Explorer guidance for Scope selection beside the Coverage title, the Whole model default, ranked hierarchical capability rows without a display switch, capability and requirement expansion for nested child rows, terminal requirement metrics and element-detail links, verification coverage labels, labelled Binding consumers links identifying requirements responsible for shared contract obligations, blocker counts beside requirement statuses, and access to implementation evidence through requirement element details, coherent dashboard/sidebar counts, and the action that opens whole-model orphan diagnostics.
- Review semantic-model and submodel guidance for validated ownership, evidence crossing scope without transferring membership, and the distinction between capability-scoped coverage and the submodels command's capability/requirement scope rules.
- Build and inspect the affected public documentation routes, confirm navigation links resolve, and ensure examples and descriptions agree with the implemented feature before publication.

#### Metadata
  * type: inspection-verification

#### Relations
  * derivedFrom: [Public Documentation Website Verification Objective](#public-documentation-website-verification-objective)
  * verify: [Website Assistant Integration Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-assistant-integration-documentation)
  * verify: [Website Command and Workflow Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-command-and-workflow-documentation)
  * verify: [Website Implementation Coverage Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-implementation-coverage-documentation)
  * verify: [Website Semantic Model Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-semantic-model-documentation)
  * verify: [Website Verification Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-verification-documentation)
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

#### Metadata
  * type: inspection-verification

#### Relations
  * derivedFrom: [Public Documentation Website Verification Objective](#public-documentation-website-verification-objective)
  * verify: [Website Modeling Language Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-modeling-language-documentation)
  * verify: [Website Requirements and Contracts Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-requirements-and-contracts-documentation)
  * verify: [Website Semantic Model Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-semantic-model-documentation)
  * verify: [Website Command and Workflow Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-command-and-workflow-documentation)
  * verify: [Website Assistant Integration Documentation](../../../Interfaces/Website/WebsiteRequirements.md#website-assistant-integration-documentation)
---
