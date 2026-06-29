# Reqvire Concept Authoring

Use this prompt when the user wants to create or revise native Reqvire concept schemes, concepts, thesaurus taxonomy, mappings, or concept references.

Workflow:
- Start with `reqvire.workspace_status`, then inspect existing concept schemes and concepts with `reqvire.concept_schemes.list`, `reqvire.concepts.list`, `reqvire.concepts.get`, `reqvire.semantic.vocabulary`, and focused `reqvire.search`.
- Decide whether the requested content is curated terminology or formal structural meaning. Use native concepts for terminology; use ontology only for formal classes, properties, individuals, axioms, and SHACL targets.
- Inspect dependent concept references, ontology `reqvire:mapsToConcept` bridges, Explorer/MCP concept outputs, and tests that assert generated SKOS IRIs before changing concept identity.
- When names collide, keep the pure terminology name on the concept and make non-concept elements carry their role, such as capability ability wording, requirement obligation wording, contract role wording, verification validation wording, or ontology structural-holder wording.

Concept authoring rules:
- Author curated terminology as native `concept-scheme` and `concept` Markdown elements, not as Turtle-authored `skos:Concept` resources inside ontology.
- Put shared concept schemes under `system-model/Thesaurus` or the project's vocabulary folder.
- Each `concept-scheme` owns `concept_base` and `concept_prefix` directly.
- A `concept` main body defines the human/domain meaning before reserved subsections.
- Do not add `concept_id`, `concept_kind`, `pref_label`, or language metadata; Reqvire derives generated SKOS identity from the element and concept-scheme context.
- Concept definitions explain terminology for people and tools. They must not state system obligations, implementation behavior, lifecycle state rules, or formal schema constraints.
- Use `derivedFrom` for concept-scheme context or concept grouping context.
- Use `broader` / `narrower` only for SKOS thesaurus taxonomy where the child is a narrower term or kind of the parent.
- Use `related` for associated concepts that are not a valid browse-tree parent/child pair.
- Use `exactMatch` and `closeMatch` only for intentional concept-to-concept mappings across vocabularies.
- Do not add `#### Details`, `#### Ontology`, `#### Shapes`, `#### Definition`, or `#### Top Concepts` to native concept elements.
- Do not add language metadata unless the project has an explicit language policy.

Concept-reference rules:
- Use `#### Concept References` on non-ontology, non-semantic-contract elements when prose should bind readable terms to native concept elements.
- Concept references must target native `concept` elements through Markdown links, not IRIs or CURIEs.
- Semantic contracts must not author concept references; they depend on ontology through `use`.
- Ontology-to-concept bridges belong in ontology Turtle with `reqvire:mapsToConcept` when useful.

Quality checks:
- Every concept has a nearest concept-scheme context.
- Every concept scheme has a unique generated namespace.
- Top concepts are meaningful; avoid generic catch-all roots.
- Do not copy every OWL `rdfs:subClassOf` edge into SKOS `broader` without checking thesaurus meaning.
- Do not maintain separate top-concept lists in Markdown; Reqvire derives top concepts from direct scheme children without `broader`.

Answer discipline:
- State the concept scheme, candidate concepts, taxonomy/mapping choices, concept-reference consumers, and generated SKOS identity impact.
- Close with validation evidence, concept-listing checks, semantic concept export checks, and affected tests.
