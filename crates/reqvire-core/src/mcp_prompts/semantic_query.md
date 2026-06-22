# Reqvire Semantic Query

Use this prompt when the user asks a model-meaning question that is better answered from ontology-aware RDF than from text search alone.

Workflow:
- Call `reqvire.semantic.vocabulary` first with `section: "all"` or a narrower section to discover prefixes, classes, properties, native SKOS concept schemes/concepts, relation families, query patterns, and diagnostics.
- Use `section: "concepts"` when the question is about thesaurus terms, concept schemes, SKOS labels/definitions, broader/narrower/related links, mappings, or generated Markdown-native concept source provenance.
- Use `reqvire.concept_schemes.list`, `reqvire.concepts.list`, or `reqvire.concepts.get` when the question is specifically about standalone native Thesaurus entries and does not require SPARQL.
- Call `reqvire.semantic.concepts` when the user needs the clean generated SKOS concept/thesaurus RDF layer; pass `include_mappings: true` only when structural `reqvire:mapsToConcept` bridge triples are relevant.
- Use the returned `sparql_prefix_block` verbatim when writing SPARQL.
- If a namespace or prefix is unclear, call `reqvire.semantic.prefixes` and inspect source element content.
- Prefer normalized semantic properties and relation-family query patterns over hard-coded markdown relation names.
- When the user asks about one ontology document or external ontology source, pass `ontology_document` or `ontology_base` to `reqvire.semantic.vocabulary` before writing broad SPARQL.
- Run the query with `reqvire.semantic.sparql`.
- Use `include_external: true` only when the question needs imported ontology terms; it materializes and exposes only the used external subset, not raw full external ontology dependencies.
- When `include_external: true` is needed, query the materialized used external vocabulary terms and their supporting labels, domains, ranges, and source metadata. Prefer graph-scoped queries against `urn:reqvire:semantic-graph:external-used-subset` (from the o-kernel subset service) and do not enumerate unused terms from the raw dependency files.
- For generated or inferred model facts, query `urn:reqvire:semantic-graph:authored-model` and `urn:reqvire:semantic-graph:generated`.
- Concept references from model elements target generated native `skos:Concept` resources. Structural OWL/RDFS terms can point back to concepts through `reqvire:mapsToConcept`; do not treat structural ontology terms themselves as SKOS concepts.
- `reqvire.semantic.sparql` responses include graph layer metadata to confirm which graph roles are active for the current options.
- Explain which semantic terms, properties, and graph facts the answer depends on.

Useful starting calls:
```json
{ "name": "reqvire.semantic.vocabulary", "arguments": { "section": "query_patterns", "include_examples": true } }
{ "name": "reqvire.semantic.vocabulary", "arguments": { "section": "relation_families", "limit": 50 } }
{ "name": "reqvire.semantic.vocabulary", "arguments": { "section": "concepts", "limit": 50, "include_source": true } }
{ "name": "reqvire.concepts.list", "arguments": { "filter": "verification" } }
{ "name": "reqvire.semantic.vocabulary", "arguments": { "section": "classes", "ontology_document": "https://example.test/ontology" } }
{ "name": "reqvire.semantic.sparql", "arguments": { "query": "PREFIX reqvire: <https://www.reqvire.org/ontology#>\\nSELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 20" } }
```

Answer discipline:
- Do not rebuild the semantic store.
- Do not infer prefixes from Turtle text when the prefix registry can provide them.
- Do not assume concepts are authored in ontology Turtle; native `concept-scheme` and `concept` Markdown elements generate the SKOS concept layer.
- If the query returns no rows, inspect vocabulary diagnostics and relation families before concluding the model lacks the fact.
