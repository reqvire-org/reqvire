# Reqvire Capability and Requirement Authoring

Use this prompt when the user wants to add or revise capabilities, requirements, requirement-owned contracts, or their traceability.

Contract dependencies include Contract Bindings for shared implementation obligations and Contract References for content dependencies. Both propagate change impact; only binding consumers contribute to the contract owner's implementation fulfillment.

Workflow:
- Start with `reqvire.workspace_status`, then inspect current capability roots and independent submodels with `reqvire.submodels`, `reqvire.search`, and model structure with `reqvire.model` or `reqvire.collect` for the requested scope.
- Decide whether the requested content belongs to an existing capability root, a child capability, a new independent capability root, the ontology plane, the thesaurus plane, or a requirement-owned contract.
- Before authoring a requirement, read the owning capability context and any parent requirement chain. Inspect contract bindings and contract references when the requirement depends on external contracts.
- Inspect available ontology, semantic-contract, and concept context when the requested content uses domain vocabulary, SHACL profiles, or `#### Concept References`.

Authoring rules:
- A `capability` describes a stable system ability, operational concern, product/business/regulatory scope, or stakeholder value. It is not a weaker requirement, UI screen, code module, deployment artifact, ticket, or implementation detail.
- A `requirement` states an implementable obligation using EARS wording with SHALL, such as "The system SHALL...", "When...", "While...", "If...", or "Where...". Put additional obligations in Details as separate EARS statements.
- Do not create implementation work without requirements, and do not create a requirement cluster without a clear capability context.
- Add child capabilities only for meaningful independently traceable or verifiable slices, not just to share vocabulary.
- Requirements specify capabilities with `specify` / `specifiedBy`. Requirement hierarchy uses `derivedFrom` only between requirements.
- Capabilities are not directly verified or directly satisfied; coverage rolls up from requirements that specify them.
- Requirement-owned contracts use `definedBy` / `define` for `source`, `specification`, `constraint`, `behavior`, `state`, and `input-output` elements.
- Place contract bindings on requirements responsible for implementing shared obligations. Use Contract References for consuming existing behavior or contract content, including documentation and service adapters; author them with `referenceContract` through `reqvire.link` or in the reserved Markdown section.
- A requirement cannot contain both Contract Bindings and Contract References, even for different targets. When it has distinct consuming and shared-output responsibilities, allocate the output obligation to a meaningful child requirement with its own binding.
- Contract dependencies must remain acyclic through contract owners, both dependency kinds, and requirement ancestry. Validate the complete graph after authoring or changing links.
- Use native `concept-scheme` and `concept` elements for curated terminology and `#### Concept References` from non-ontology, non-semantic-contract model elements.
- Use `ontology` for reusable structural meaning and `semantic-contract` for SHACL profiles that `use` ontology and `constrain` requirements.
- Keep exact commands, fields, URI patterns, payloads, report sections, file paths, and workflow behavior in requirement-owned contracts instead of capability prose or ontology.
- Preserve explicit governance metadata on capabilities and requirements, but do not add `status`, `priority`, `risk`, or `owner` unless the user, source material, or existing parent context explicitly calls for authored values.

Verification expectations:
- Identify whether new or changed requirements are leaf requirements that need direct verification.
- Plan or author a concrete verification only for requirements, not capabilities.
- If verification is missing, recommend `reqvire.workflow.author_or_align_verification` or include a concrete verification follow-up.

Answer discipline:
- Show the proposed graph shape: capability, requirements, contracts, semantic contracts, concepts, ontology, and verifications.
- Explain why each new or edited element has the chosen type.
- List required relations and validation checks before edits are considered complete.
