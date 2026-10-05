# Reqvire Model Structure Refactor

Use this prompt when the user wants to reorganize a Reqvire model without changing system intent.

Contract dependencies include Contract Bindings for shared implementation obligations and Contract References for content dependencies. Both propagate change impact; only binding consumers contribute to the contract owner's implementation fulfillment.

Workflow:
- Start with `reqvire.workspace_status`, validation evidence, `reqvire.lint`, `reqvire.submodels`, `reqvire.containment`, and focused `reqvire.search` for the requested scope.
- Use `reqvire.collect` before editing a candidate capability, requirement, contract, ontology, or verification branch.
- Classify the problem: duplicated requirements, embedded specifications, misplaced files, missing ownership relations, cross-submodel leakage, normative contract language, or ontology/semantic-contract boundary confusion.
- Produce a move/link/rewrite plan before mutation. Confirm high-risk boundary decisions with the user before bulk moves, mass unlinking, or submodel boundary rewrites.

Refactor rules:
- Preserve system behavior and requirement intent. Refactoring changes structure, ownership, containment, or wording boundaries; it must not silently change obligations.
- Capabilities own coherent system ability. Requirements own implementable obligations. Requirement-owned contracts own detailed specs, constraints, behavior, states, sources, and input/output.
- Extract exact technical details from requirements into compatible requirement-owned contracts when that improves traceability.
- Use `definedBy` / `define` for contract ownership. Retain contract bindings on requirements responsible for implementing shared obligations; replace context-only bindings with Contract References using `referenceContract` through `reqvire.link` or the reserved Markdown section.
- A requirement cannot contain both Contract Bindings and Contract References, even for different targets. Allocate distinct input-consumption and shared-output responsibilities to meaningful parent and child requirements when each needs a different dependency kind.
- Keep hierarchy inside compatible families and intended submodel boundaries. Choose cross-submodel contract bindings for shared obligations, contract references for content dependencies, concept references for terminology, and semantic-contract links for SHACL constraints.
- Preserve dependency visibility when replacing a cross-subgraph relation. Contract dependencies must remain acyclic through contract owners, both dependency kinds, and requirement ancestry.
- After replacing cross-boundary dependencies, check that `reqvire.collect` still shows consumer context and change-impact still reports binding and reference consumers. Check implementation coverage again: converting a binding to a reference can reveal an unimplemented owner requirement.
- Put reusable structural meaning in ontology, curated terminology in native concepts, and SHACL closed-world profiles in semantic contracts.
- Semantic contracts contain `#### Shapes`, use ontology through `use` / `usedBy`, constrain requirements through `constrain` / `constrainedBy`, and do not contain `#### Ontology`.
- Keep specification, constraint, and behavior language mechanism-focused. Move requirement-intent `shall` statements back to the owning requirement or rephrase contract text without changing meaning.
- Do not leave deprecated placeholders, duplicated normative language, or no-op cleanup artifacts.

Containment rules:
- Folders make ownership and review boundaries obvious; graph relations define model meaning.
- Capability folders should contain local capability, requirement, contract, and architecture content.
- `Ontologies/` owns ontology and semantic-contract content; `Thesaurus/` owns native concept schemes and concepts; `Verifications/` owns verification elements.

Answer discipline:
- Report the current-state findings, proposed refactor slices, dependency preservation strategy, and validation risks.
- For each proposed edit, state which relation or containment invariant it improves.
- Call out any decisions that require human boundary confirmation before bulk moves, mass unlinking, or ownership rewrites.
- Close each slice with validation evidence, `reqvire.lint`, `reqvire.submodels`, `reqvire.containment`, `reqvire.coverage`, and focused tests when behavior-facing artifacts changed.
