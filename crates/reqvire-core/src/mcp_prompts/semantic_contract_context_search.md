# Reqvire Semantic Contract Context Search

Use this prompt when the user asks about semantic contracts, contract bindings, contract references, cross-subgraph dependencies, or requirement constraints.

Contract dependencies include Contract Bindings for shared implementation obligations and Contract References for content dependencies. Both propagate change impact; only binding consumers contribute to the contract owner's implementation fulfillment.

Workflow:
- Call `reqvire.semantic.vocabulary` with `section: "semantic_contracts"` to discover reusable SHACL contract profiles and their source mappings.
- Call `reqvire.semantic.vocabulary` with `section: "relation_families"` to find normalized contract, constraint, use, binding, and reference properties. Query `reqvire:bindsContract` for obligations and `reqvire:referencesContract` for content dependencies.
- Use `reqvire.semantic.prefixes` when query construction needs source ontology prose or exact namespaces.
- Run `reqvire.semantic.sparql` to join requirements to semantic contracts, ontology terms, or contract dependency facts. Keep the binding and reference predicates distinct in results.
- Use `reqvire.read_element` for final human-facing details from the requirement or contract element.

Answer discipline:
- Explain which requirement owns the contract and which requirements bind or reference it.
- Treat semantic contracts as closed-world SHACL profiles that constrain requirements.
- Keep owned contracts, semantic contracts, Contract Bindings, and Contract References distinct in the explanation.
