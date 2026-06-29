# Reqvire Change Impact Audit

Use this prompt when the user asks whether changed Reqvire system-model elements require updates to impacted elements that have not changed yet.

Workflow:
- Call `reqvire.workspace_status` first and record model validity, dirty state, current branch, and HEAD.
- Establish and state the comparison base before judging impact. Prefer the user-provided base; otherwise use the repository's merge-base with `main` when available, or explain the fallback.
- Call `reqvire.change_impact` with that base and analyze the structured buckets: `added[]`, `changed[]`, `removed[]`, `relocated[]`, `impact_scope[]`, and `invalidated_verifications[]`.
- Treat `impact_scope[]` as the high-level affected-area summary. For each scope root, use downstream collection to enumerate covered children so no impacted descendant is skipped.
- Separate direct added, changed, removed, and relocated elements from propagated impacts, changed contract bindings, and invalidated verifications.
- Use `reqvire.search` and `reqvire.read_element` to inspect every changed or impacted capability, requirement, specification, contract, ontology, semantic-contract, and verification element before deciding whether it needs an update.
- Use `reqvire.collect` upstream and downstream around changed requirements and capabilities to inspect parents, children, owning capabilities, requirement-owned contracts, semantic contracts, and verification evidence.
- Use `reqvire.traces`, `reqvire.coverage`, and `reqvire.lint` when the question asks whether the model is still coherent, verified, or merge-ready.
- Use `reqvire.semantic.vocabulary` and `reqvire.semantic.sparql` only when ontology terms, semantic contracts, concept references, or relation-family joins are part of the impact.

Change propagation rules:
- Parent capability or requirement changes propagate to derived children.
- Capability changes may require review of specifying requirements and requirement verification coverage.
- Requirement changes invalidate verifications and may require updates to satisfiedBy implementation or evidence links.
- Requirement-owned contract changes may affect consumers through contract bindings even when the consuming requirement text did not change.
- Verification-only changes generally do not propagate upward unless they change verification scope, evidence expectations, or coverage claims.

System-model audit checks:
- Check whether changed parent requirements alter child requirement obligations.
- Check whether changed requirements invalidate verification criteria, verification evidence, or satisfiedBy evidence.
- Check whether changed capabilities require updates to specified requirements, subcapabilities, concept references, or verification rollups.
- Check whether changed specifications, constraints, behaviors, states, input-output contracts, or sources still match their owning requirement.
- Check whether changed ontology or semantic-contract elements require updates to constrained requirements, concept references, semantic export expectations, or MCP semantic query guidance.
- Check whether impacted documentation or assistant-guidance artifacts bound to changed specifications still enumerate current commands, prompts, workflows, and protocol behavior.
- Focus recommendations on authored system-model content. Do not recommend code implementation work unless implementation evidence or satisfiedBy links are directly affected.

Answer discipline:
- Start with impact scope before detailed element decisions.
- List direct changes separately from propagated impacts.
- Include added, changed, removed, relocated, and invalidated verification counts when available.
- For each impacted element, say `update needed`, `review only`, or `no update needed`, with the reason.
- Name exact elements and file anchors whenever available.
- Call out invalidated verifications and any tests or evidence that must be rerun.
- If all impacted model elements are already aligned, say that explicitly and list the evidence used.
