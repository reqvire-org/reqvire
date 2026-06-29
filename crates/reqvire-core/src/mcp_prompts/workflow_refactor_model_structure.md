# Reqvire Model Structure Refactor

Use this prompt when the user wants to reorganize a Reqvire model without changing system intent.

Workflow:
- Start with `reqvire.workspace_status`, validation evidence, `reqvire.lint`, `reqvire.submodels`, `reqvire.containment`, and focused `reqvire.search` for the requested scope.
- Use `reqvire.collect` before editing a candidate capability, requirement, contract, ontology, or verification branch.
- Classify the problem: duplicated requirements, embedded specifications, misplaced files, missing ownership relations, cross-submodel leakage, normative contract language, or ontology/semantic-contract boundary confusion.
- Produce a move/link/rewrite plan before mutation. Confirm high-risk boundary decisions with the user before bulk moves, mass unlinking, or submodel boundary rewrites.

Refactor rules:
- Preserve system behavior and requirement intent. Refactoring changes structure, ownership, containment, or wording boundaries; it must not silently change obligations.
- Capabilities own coherent system ability. Requirements own implementable obligations. Requirement-owned contracts own detailed specs, constraints, behavior, states, sources, and input/output.
- Extract exact technical details from requirements into compatible requirement-owned contracts when that improves traceability.
- Use `definedBy` / `define` for contract ownership and contract bindings only for explicit cross-subgraph reuse of compatible requirement-owned contracts.
- Keep hierarchy inside compatible families and intended submodel boundaries. Cross-submodel requirement hierarchy should become explicit contract bindings, concept references, or semantic-contract links where appropriate.
- Do not remove a cross-subgraph relation unless dependency visibility is preserved with an explicit replacement such as a contract binding, concept reference, semantic-contract relation, or local requirement-owned contract.
- After replacing cross-boundary dependencies, check that `reqvire.collect` still shows the context needed by the consumer and change-impact still reports bound-contract consumers.
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
