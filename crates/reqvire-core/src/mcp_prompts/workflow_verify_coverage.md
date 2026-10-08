# Reqvire Verification Coverage Review

Use this prompt when the user asks whether a Reqvire scope is valid, covered, verified, or ready to merge.

Contract dependencies include Contract Bindings for shared implementation obligations and Contract References for content dependencies. Both propagate change impact; only binding consumers contribute to the contract owner's implementation fulfillment.

Workflow:
- Call `reqvire.workspace_status` first and check model validity and dirty state.
- Use `reqvire.lint`, `reqvire.coverage`, `reqvire.traces`, and relevant structure tools for evidence.
- Use `reqvire.search` to narrow by capability, requirement, owner, priority, risk, or status.
- Use semantic SPARQL only when the question needs ontology-aware counting, relation-family joins, or concept-reference queries.
- Read specific elements before making a final claim about missing verification or ambiguous ownership.

Answer discipline:
- Separate validation failures, lint findings, missing verification, and missing implementation coverage.
- Count verification leaves separately from implementation terminal requirements and capability rollups. Terminal requirements have neither child requirements nor required binding consumers and need direct `satisfiedBy` evidence.
- A requirement with children or required binding consumers is implementation-covered only when all of those requirements are covered recursively. Direct parent evidence remains visible but cannot override an outstanding obligation. Contract References add no implementation evidence or blockers and do not change terminal classification.
- Evaluate coverage against the full model before selecting scope subjects, so external binding consumers retain their evidence without entering scoped subject counts.
- Tie every recommendation to an element, relation, verification, or report result.
