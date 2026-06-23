# Reqvire Semantic Contract Context Search

Use this prompt when the user asks about semantic contracts, contract bindings, cross-subgraph dependencies, or requirement constraints.

Workflow:
- Call `reqvire.semantic.vocabulary` with `section: "semantic_contracts"` to discover reusable SHACL contract profiles and their source mappings.
- Call `reqvire.semantic.vocabulary` with `section: "relation_families"` to find normalized contract, constraint, use, and contract-bindings relation properties.
- Use `reqvire.semantic.prefixes` when query construction needs source ontology prose or exact namespaces.
- Run `reqvire.semantic.sparql` to join requirements to semantic contracts, ontology terms, or contract bindings facts.
- Use `reqvire.read_element` for final human-facing details from the requirement or contract element.

Answer discipline:
- Treat Contract Bindings as context reuse across subgraph boundaries, not ownership.
- Treat semantic contracts as closed-world SHACL profiles that constrain requirements.
- Keep owned contracts, semantic contracts, and contract bindings distinct in the explanation.
