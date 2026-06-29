# Reqvire Model Quality Audit

Use this prompt when the user asks what is wrong with a Reqvire model, wants lint cleanup, needs model health recommendations, or wants redundant verification analysis.

Workflow:
- Start with `reqvire.workspace_status`.
- Run read-only health evidence first: validation evidence, `reqvire.search`, `reqvire.coverage`, `reqvire.lint`, `reqvire.submodels`, and `reqvire.containment` where available.
- Use JSON outputs when possible and keep intermediate analysis tied to concrete element identifiers, files, relations, and tool findings.
- For coverage questions, focus on leaf requirements for verification coverage and `requirement` elements for implementation coverage.
- For lint findings, separate auto-fixable items from items that need human review.
- For redundant verification analysis, inspect whether a verification targets both a precise requirement and an ancestor already covered by rollup.

Audit categories:
- Validation results: structural errors, relation errors, malformed elements, unresolved links, ontology/semantic-contract issues.
- Coverage analysis: unverified leaf requirements, implementation-uncovered requirements, parent requirements that are correctly covered by children, and capability rollup context.
- Model quality: redundant verify relations, redundant hierarchy, missing ownership, cross-submodel hierarchy leakage, orphaned contracts, normative contract language, and containment problems.
- Semantic structure: ontology vs native concept vs requirement vs semantic-contract boundary issues.
- Recommendations: ordered by severity, blast radius, priority/risk/owner metadata, and ease of safe correction.

Cleanup rules:
- Apply or recommend `reqvire.lint --fix` only for safe auto-fixable issues.
- Do not remove manual-review relations without explaining the trace consequence and getting user confirmation where needed.
- Do not hide validation failures behind coverage summaries. Validation errors come first.
- Do not count capabilities as implementation-covered requirements; capability coverage rolls up from requirements.
- Do not suggest direct capability verification; inspect specifying requirements instead.

Answer discipline:
- Findings first, ordered by severity and grounded in element/file references.
- For each finding, include evidence, impact, and a concrete next action.
- Separate "safe auto-fix", "needs model decision", "needs verification", and "needs implementation/evidence" buckets.
- Close with the minimal validation commands required to prove the cleanup.
