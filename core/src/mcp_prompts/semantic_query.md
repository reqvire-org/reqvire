# Reqvire Semantic Query

Use this prompt when the user asks a model-meaning question that is better answered from ontology-aware RDF than from text search alone.

Workflow:
- Call `reqvire.semantic.vocabulary` first with `section: "all"` or a narrower section to discover prefixes, classes, properties, relation families, query patterns, and diagnostics.
- Use the returned `sparql_prefix_block` verbatim when writing SPARQL.
- If a namespace or prefix is unclear, call `reqvire.semantic.prefixes` and inspect source element content.
- Prefer normalized semantic properties and relation-family query patterns over hard-coded markdown relation names.
- Run the query with `reqvire.semantic.sparql`.
- Use `include_external: true` only when the question needs imported ontology terms; it exposes the used external subset, not raw full external ontology dependencies.
- When `include_external: true` is needed, query the materialized used external vocabulary terms and their supporting labels, domains, ranges, and source metadata. Do not enumerate unused terms from the raw dependency files.
- Explain which semantic terms, properties, and graph facts the answer depends on.

Useful starting calls:
```json
{ "name": "reqvire.semantic.vocabulary", "arguments": { "section": "query_patterns", "include_examples": true } }
{ "name": "reqvire.semantic.vocabulary", "arguments": { "section": "relation_families", "limit": 50 } }
{ "name": "reqvire.semantic.sparql", "arguments": { "query": "PREFIX reqvire: <https://www.reqvire.org/ontology#>\\nSELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 20" } }
```

Answer discipline:
- Do not rebuild the semantic store.
- Do not infer prefixes from Turtle text when the prefix registry can provide them.
- If the query returns no rows, inspect vocabulary diagnostics and relation families before concluding the model lacks the fact.
