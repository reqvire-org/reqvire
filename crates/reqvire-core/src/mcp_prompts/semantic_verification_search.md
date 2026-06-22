# Reqvire Semantic Verification Search

Use this prompt when the user asks which requirements are verified or unverified, or which capability scope has coverage through verified requirements.

Workflow:
- Call `reqvire.semantic.vocabulary` with `section: "query_patterns"` and `include_examples: true`.
- Use query patterns such as verified requirements as the starting point, then narrow by capability, requirement type, ontology concept reference, or element identifier.
- Use `reqvire.semantic.vocabulary` with `section: "concepts"` when the scope is expressed as a generated native SKOS concept, concept scheme, broader/narrower term, or related concept.
- Call `reqvire.semantic.vocabulary` with `section: "relation_families"` when you need the normalized verification relation property.
- Run `reqvire.semantic.sparql` for semantic counts or joins.
- Cross-check user-facing evidence with `reqvire.coverage`, `reqvire.traces`, or `reqvire.read_element` when the answer needs verification details or file links.

Typical semantic intent:
```sparql
PREFIX reqvire: <https://www.reqvire.org/ontology#>
SELECT ?requirement ?verification WHERE {
  ?requirement reqvire:elementType "requirement" .
  ?requirement reqvire:requirementVerifiedByVerification ?verification .
}
```

Answer discipline:
- Count requirements, not verification elements, unless the user asks for evidence count.
- Be explicit about whether capability rollup was included.
- If the scope is ambiguous, show the exact scope filter used.
