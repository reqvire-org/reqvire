# Elements

### Public Documentation Website Source Context Specification

#### Details
The public documentation website is the documentation surface for `www.reqvire.org`. React page modules, route configuration, layout, sidebar navigation, and static public images are implementation artifacts for website documentation requirements.

Each website page requirement MUST link to its page source files through `satisfiedBy`. A website specification's references to contracts owned by other requirement branches MUST be supported by Contract References on its owning website requirement. This provides the change-impact path from documented contracts to website requirements and concrete page sources.

The public documentation website and the Explorer are separate surfaces. The Explorer is the model-browsing application served by Reqvire.

#### Metadata
  * type: specification

#### Relations
  * define: [Public Documentation Website](WebsiteRequirements.md#public-documentation-website)
---

### Website Assistant Integration Documentation Specification

#### Details
Assistant integration content covers MCP, coding-assistant plugins, Codex skills, prompt workflows, MCP endpoints, protocol behavior, and assistant-facing Reqvire context.

Review triggers include changes to MCP tool schemas, prompt guidance, protocol support, and assistant integration workflows.

The MCP server page MUST show repeatable `--allow-origin` examples for standalone and embedded MCP, explain the loopback defaults and exact scheme/host/port matching, and distinguish a browser origin from the MCP endpoint URL. It MUST explain the separate roles of bind address, browser origin permission, and deployment authentication according to the transport contracts referenced by its owning requirement.

The MCP server page MUST document automatic acceptance of a non-wildcard bind authority and repeatable `--allow-host` for endpoint aliases, including a reverse-proxy example, wildcard listener behavior, optional port matching, and embedded MCP usage. It MUST distinguish the endpoint hostname from a browser application's origin and explain that non-browser MCP clients normally need host configuration rather than CORS permission.

The MCP server page's coverage guidance and tool-call examples MUST conform to the MCP Coverage Scope Selection Specification and Coverage Scope Specification referenced by the owning requirement. Topics include selector syntax, valid selections, default scope, error responses, and the distinction between report membership and external supporting evidence.

The MCP server page MUST explain `has_contract_references` and `filter_contract_references`, normalized target matching, and composition with other search filters according to the MCP Model Evidence Tools Specification and SearchFiltering contract referenced by its owning requirement. Its example MUST distinguish search selection from change-impact propagation and implementation coverage.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Assistant Integration Documentation](WebsiteRequirements.md#website-assistant-integration-documentation)
---

### Website Command and Workflow Documentation Specification

#### Details
Workflow documentation covers CLI commands, report commands, model mutation commands, change-impact analysis, collection, coverage, traces, and advanced workflows using current command names and options.

Workflow examples form part of the source context for CLI and report contract changes.

User-guide coverage examples MUST conform to the CLI Coverage Scope Selection Specification and Coverage Scope Specification referenced by the owning requirement. Examples cover root and nested capability selection, text and JSON reports, the default scope, and invalid selectors. Explorer guidance MUST conform to the referenced Explorer Coverage Scope and Display Specification; topics include scope selection beside the Coverage title, the default ranked hierarchy with parent-before-child rows and coverage-ranked siblings, capability and requirement expansion for nested child rows, terminal requirement metrics and element-detail links, verification coverage labels, labelled Binding consumers links identifying requirements responsible for shared contract obligations, blocker counts beside requirement statuses, and access to implementation evidence through requirement element details, dashboard and sidebar counts, and access to orphan diagnostics. Advanced workflow guidance links to the detailed coverage explanation.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Command and Workflow Documentation](WebsiteRequirements.md#website-command-and-workflow-documentation)
---

### Website Home Messaging Specification

#### Details
Home-page content comprises concise public positioning for Reqvire, an introduction to the semantic engineering framework, major knowledge categories, and links to conceptual and workflow pages.

Homepage wording MUST use current model vocabulary for ontologies, capabilities, requirements, contracts, verification, implementation artifacts, change impact, and assistant context.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Home Messaging](WebsiteRequirements.md#website-home-messaging)
---

### Website Implementation Coverage Documentation Specification

#### Details
Implementation-coverage content covers satisfaction links for requirements and evidence-backed verifications, the distinction between implementation and verification coverage, and report diagnostics for missing implementation or evidence links.

Definitions and examples MUST conform to the implementation and verification coverage contracts referenced by the owning requirement.

Implementation guidance MUST define terminal requirements, recursive all-child and all-consumer fulfillment, combined obligations, and the rule that direct parent evidence cannot override a gap. It MUST distinguish terminal percentage units from all-requirement classifications and describe partial evidence and blockers on uncovered requirements.

Implementation guidance MUST explain that cycles in the combined child and contract-consumer dependency graph are validation errors, including within one capability root, and are rejected before coverage reporting even when direct evidence exists.

Binding-placement guidance MUST follow the Contract Bindings Hierarchical Independence Constraint referenced by the owning requirement. It MUST distinguish independently implemented child obligations from a parent obligation applying to the entire requirement subtree, and explain that a common ancestor alone does not justify moving bindings upward.

Scoped coverage guidance uses the referenced Coverage Scope Specification for membership, classification, and supporting evidence. The example includes a selected requirement whose owned contract is consumed by a directly satisfied requirement in another submodel. Its explanation distinguishes report subjects, external evidence, and coverage source according to the referenced contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Implementation Coverage Documentation](WebsiteRequirements.md#website-implementation-coverage-documentation)
---

### Website Modeling Language Documentation Specification

#### Details
Modeling-language content covers Reqvire Markdown structure, element metadata, relation syntax, reserved subsections, and examples consistent with the parser and model-structure contracts referenced by the owning requirement.

The reserved-subsection overview and examples MUST use `Contract Bindings` and `Contract References` as defined by the referenced ReservedSubsections and Contract Reference Semantics contracts. Examples MUST distinguish shared implementation obligations from content dependencies and explain element-wide section exclusivity and acyclic reference dependencies.

Terminology MUST use contract dependency as the umbrella term for bindings and references, and define a contract reference as a content dependency that propagates change impact without contributing to the contract owner's implementation fulfillment.

Verification descriptions MUST identify requirements as verification targets and explain capability coverage through requirement roll-up according to the referenced Verification Coverage Specification.

Review triggers include changes to structured Markdown parsing, file identification, reserved subsections, relation syntax, and element addressing.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Modeling Language Documentation](WebsiteRequirements.md#website-modeling-language-documentation)
---

### Website Ontology Documentation Specification

#### Details
Ontology content covers ontology elements, concept references, external ontology sources, built-in reserved vocabulary behavior, semantic contracts, validation rules, export modes, and semantic tooling.

Terminology distinguishes authored ontology/SHACL output, full semantic model context, external ontology source inclusion, and Explorer ontology visualization behavior. Descriptions MUST NOT imply hidden Turtle prefix injection or hidden semantic triples.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Ontology Documentation](WebsiteRequirements.md#website-ontology-documentation)
---

### Website Requirements and Contracts Documentation Specification

#### Details
Requirements-and-capabilities content covers capability scope, requirement obligations, requirement-owned contracts, semantic contracts, governance metadata, concept references, and both kinds of contract dependency.

Contract dependency guidance MUST distinguish Contract Bindings, whose consumers contribute to the owner's implementation fulfillment, from Contract References, which identify content dependencies for change-impact review. It MUST show a documentation requirement referencing a contract and explain element-wide section exclusivity and rejection of circular reference dependencies according to the referenced Contract Reference Semantics Specification.

Contract dependency MUST be presented as the umbrella term for both kinds, while the specific authoring subsections retain the names `Contract Bindings` and `Contract References`.

Contract wording MUST follow the relation and semantic-contract specifications referenced by the owning requirement. The terminology distinguishes requirement-owned definitions of source basis, specifications, constraints, behavior, state, interfaces, and input/output semantics from ontology-plane SHACL profiles and their constraint and ontology-use relations.

Binding-placement guidance MUST follow the Contract Bindings Hierarchical Independence Constraint referenced by the owning requirement. It MUST explain placement by implementation responsibility, separate child bindings for independently tracked obligations, parent bindings for whole-subtree obligations, downstream inheritance, and rejection of repeated ancestor/descendant bindings. A concrete example MUST distinguish independent child obligations from an obligation applying to the entire parent subtree.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Requirements and Contracts Documentation](WebsiteRequirements.md#website-requirements-and-contracts-documentation)
---

### Website Semantic Model Documentation Specification

#### Details
Semantic-model content covers Reqvire element families, relation semantics, ownership rules, submodel boundaries, concept references, contract bindings, verification links, and implementation evidence.

The distinction between requirement-owned contracts and semantic contracts, and the relation vocabulary, MUST conform to the contracts referenced by the owning requirement.

Contract binding scope MUST follow the Contract Bindings Hierarchical Independence Constraint referenced by the owning requirement. Descriptions MUST identify the binding requirement and its descendants as the applicability scope, rather than implying that a binding applies to every requirement in the consuming capability-root submodel. Placement guidance MUST distinguish child implementation responsibility from whole-subtree responsibility at a parent.

Submodel guidance uses the referenced Requirement Submodels Report Specification and Coverage Scope Specification. Topics include validated ownership, cross-scope evidence, accepted selector types for each report, and the branch-reporting rules for capability and requirement selections. Examples MUST preserve the distinction between report scope and model ownership.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Semantic Model Documentation](WebsiteRequirements.md#website-semantic-model-documentation)
---

### Website Strategic Positioning Documentation Specification

#### Details
Strategic-positioning content explains the separation of ontologies, capabilities, requirements, contracts, verifications, implementation artifacts, and evidence. Reqvire's positioning is a semantic engineering framework supporting traceability, verification, change-impact workflows, and AI-assisted implementation context.

Review triggers include changes to relation semantics, contract terminology, semantic-contract rules, and AI-context workflows.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Strategic Positioning Documentation](WebsiteRequirements.md#website-strategic-positioning-documentation)
---

### Website Verification Documentation Specification

#### Details
Verification content covers verification objectives, concrete verification types, evidence-backed verification satisfaction, requirement verification links, and capability coverage roll-up.

Terminology and examples MUST conform to the verification coverage, roll-up, and trace contracts referenced by the owning requirement, including the distinction between verification objectives, concrete verifications, and evidence artifacts.

Scoped verification guidance uses the referenced Coverage Scope Specification. Topics include requirement-target membership, capability roll-up, objective exclusion, shared-verification counts within each scope and globally, non-additive submodel totals, and the scope of orphan diagnostics.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Verification Documentation](WebsiteRequirements.md#website-verification-documentation)
---
