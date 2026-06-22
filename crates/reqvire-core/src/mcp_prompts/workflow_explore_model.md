# Reqvire Model Exploration

Use this prompt for regular Reqvire questions where text, structure, and reports are enough and SPARQL is not required.

Workflow:
- Start with `reqvire.workspace_status` to understand workspace root, model validity, dirty state, and tool contract version.
- Use `reqvire.search` to find candidate elements by name, type, content, governance metadata, relations, or reused contract context.
- Use `reqvire.read_element` for authoritative details on a specific element.
- Use `reqvire.model`, `reqvire.containment`, `reqvire.collect`, or `reqvire.submodels` to understand structure and dependency scope.
- Use `reqvire.concept_schemes.list`, `reqvire.concepts.list`, or `reqvire.concepts.get` when the user asks about standalone Thesaurus structure, concept schemes, or generated native SKOS concepts.
- Use `search`/`model`/`collect` with `concept-scheme` and `concept` when the user asks how authored concept elements sit in the broader Reqvire model.
- Use semantic tools only when the user asks an ontology, SHACL, concept/thesaurus, prefix, vocabulary, or SPARQL question.

Answer discipline:
- Ground answers in element names, identifiers, and file references returned by tools.
- Distinguish capability, requirement, contract, ontology, concept-scheme, concept, semantic-contract, and verification element types.
- Mention validation or dirty-state caveats when workspace status reports them.
