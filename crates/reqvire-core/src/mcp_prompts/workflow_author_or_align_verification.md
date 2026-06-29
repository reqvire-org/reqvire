# Reqvire Verification Authoring and Alignment

Use this prompt when the user wants to add verification coverage, align verification criteria with tests, or review verification evidence.

Workflow:
- Start with `reqvire.workspace_status`, then gather coverage with `reqvire.coverage` and trace context with `reqvire.traces`.
- If the user names a capability, find requirements that specify the capability. Do not make the capability the direct `verify` target.
- For each candidate requirement, use traces to decide whether it is a leaf requirement that needs direct verification or a parent whose coverage should roll up from children.
- Read the full requirement chain and owning capability context with `reqvire.collect` before writing or changing verification criteria.
- Inspect existing verification elements, evidence-backed `satisfiedBy` links, test files, expected fixtures, and related lint findings before changing criteria.

Authoring rules:
- Create or reuse a `verification-objective` as the planning parent. A `verification-objective` organizes scope only and must not use `verify`, `verifiedBy`, or `satisfiedBy`.
- Every concrete verification must derive from a verification objective.
- Concrete verification types include `test-verification`, `formal-proof-verification`, `analysis-verification`, `inspection-verification`, and `demonstration-verification`.
- Link concrete verifications to requirements with `verify` / `verifiedBy`.
- Only evidence-backed concrete verification types, especially `test-verification` and `formal-proof-verification`, may use `satisfiedBy` evidence links.
- Prefer verifying the most precise leaf requirements and let parent requirements and capabilities receive rollup coverage.
- Avoid redundant verify relations where a verification targets both a leaf and an ancestor already covered through the trace path.

Alignment rules:
- Verification criteria must match actual assertions, proofs, inspections, analyses, or demonstrations.
- Remove or rewrite unverifiable claims instead of leaving vague criteria.
- If a model criterion is important but untested, add or request the missing positive/negative assertion in the existing relevant test suite.
- Use deterministic expected output fixtures and diff checks when report shape or command output is being verified.

Answer discipline:
- State the verification scope, target requirements, objective parent, concrete verification type, and evidence links.
- Separate missing coverage, stale criteria, redundant verification links, and test assertion gaps.
- Close with validation evidence to run or request: `reqvire.lint`, `reqvire.coverage`, `reqvire.traces`, and affected tests.
