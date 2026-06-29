# Reqvire Ontology and Semantic Contract Authoring

Use this prompt when the user wants to create, extend, or refactor ontology elements or semantic-contract SHACL profiles.

Workflow:
- Start with `reqvire.workspace_status`, then inspect existing ontology and semantic-contract content with `reqvire.search`, `reqvire.semantic.prefixes`, `reqvire.semantic.vocabulary`, and `reqvire.semantic.export`.
- Establish the system-of-interest scope and the competency questions the ontology must help answer.
- Decide whether the requested meaning belongs in native concepts, ontology, requirements, requirement-owned contracts, or semantic contracts before authoring terms.
- For ontology changes, inspect the owning ontology document, inherited `ontology_base`, `ontology_prefix`, external ontology declarations, and dependent semantic contracts.
- For semantic-contract changes, inspect the ontology terms reachable through explicit `use` relations and the requirements reached through `constrain` / `constrainedBy`.

Ontology authoring rules:
- Use `ontology` elements for reusable structural meaning: OWL classes, properties, individuals, domain/range, hierarchy, axioms, and stable relationship semantics.
- Author exactly one `#### Ontology` Turtle block per ontology element.
- The top ontology boundary owns non-empty `ontology_base` and `ontology_prefix`; authored Turtle must declare the inherited term namespace explicitly.
- Link ontology hierarchy with `derivedFrom` / `derive` only between ontology elements.
- Do not manually author generated `rdfs:isDefinedBy` facts for every local term.
- Use external ontology sections only for local external vocabularies that are not authored by the Reqvire model. Do not add external sections for built-in RDF, RDFS, OWL, XSD, SHACL, or SKOS vocabularies.
- Use `reqvire:mapsToConcept` only as an optional structural ontology-to-native-concept bridge.
- Do not put governance metadata on ontology elements.
- Do not claim implementation satisfaction from ontology elements; implementation and evidence links belong on requirements and evidence-backed verifications.
- Prefer deterministic IRIs and stable CURIE prefixes. Avoid random UUIDs unless the domain truly requires non-semantic identifiers.
- Add domain/range only when the subject and object/literal scope is stable and not misleadingly broad.

Semantic-contract rules:
- Use `semantic-contract` elements for reusable closed-world SHACL shape profiles.
- Semantic contracts contain `#### Shapes` and must not contain `#### Ontology`.
- Semantic contracts use ontology with `use` / `usedBy` and constrain requirements with `constrain` / `constrainedBy`.
- SHACL shapes consume ontology terms; they do not declare new OWL classes or properties.
- Put cardinality, pattern, datatype, enumeration, and data-quality validation in SHACL. Put stable reusable domain meaning in OWL/RDFS.

Layer decision rules:
- If the work is labels, synonyms, definitions, editorial taxonomy, stakeholder vocabulary, or search/navigation language, use native concepts instead of ontology.
- If the work is implementable system behavior, exact commands, payload fields, output format, persistence, validation messages, or workflow steps, use requirements or requirement-owned contracts.
- If a requirement obligation needs a machine-checkable profile, keep the obligation in the requirement and link it to a semantic contract.

Answer discipline:
- State the layer decision before proposing edits.
- List required prefixes, ontology document boundaries, shape dependencies, constrained requirements, external ontology dependencies, and validation commands.
- Close with validation evidence, semantic export checks for `ontologies` and `shapes`, and affected tests or fixtures.
