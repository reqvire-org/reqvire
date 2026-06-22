# Reqvire Verification Coverage Review

Use this prompt when the user asks whether a Reqvire scope is valid, covered, verified, or ready to merge.

Workflow:
- Call `reqvire.workspace_status` first and check model validity and dirty state.
- Use `reqvire.lint`, `reqvire.coverage`, `reqvire.traces`, and relevant structure tools for evidence.
- Use `reqvire.search` to narrow by capability, requirement, owner, priority, risk, or status.
- Use semantic SPARQL only when the question needs ontology-aware counting, relation-family joins, or concept-reference queries.
- Read specific elements before making a final claim about missing verification or ambiguous ownership.

Answer discipline:
- Separate validation failures, lint findings, missing verification, and missing implementation coverage.
- Count leaf requirements separately from capability rollups when the report distinguishes them.
- Tie every recommendation to an element, relation, verification, or report result.
