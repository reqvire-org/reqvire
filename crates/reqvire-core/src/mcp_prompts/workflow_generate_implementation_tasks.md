# Reqvire Implementation Task Generation

Use this prompt when the user wants implementation tasks from capability-scoped system-model changes.

Workflow:
- Start with `reqvire.workspace_status` and state the comparison base. If a base argument is not supplied, identify or ask for the intended git base before claiming change coverage.
- Run `reqvire.change_impact` and separate `added[]`, `changed[]`, `removed[]`, `relocated[]`, `impact_scope[]`, and `invalidated_verifications[]`.
- Treat `impact_scope[]` as affected-area roots, then use downstream collection from each scope root so child capabilities, requirements, contracts, and verifications are not skipped.
- For each changed or impacted capability and requirement, gather upstream context with `reqvire.collect`; include contract bindings, owned contracts, concept references, semantic contracts, verification links, and implementation/evidence links.
- Prefer Reqvire command evidence over raw Markdown scanning for task generation. Use raw file reads only to inspect exact content after the graph commands identify the relevant element or artifact.
- Use `reqvire.search` to inspect effective governance metadata: `status`, `priority`, `risk`, and `owner`. Preserve whether values are explicit, inherited, defaulted, or unassigned when this affects routing.
- Use `reqvire.traces` and coverage evidence to identify affected verifications, evidence-backed `satisfiedBy` test/proof links, and implementation `satisfiedBy` links on requirements.

Task generation rules:
- Generate tasks for implementable requirement obligations, not for capabilities alone. Capabilities provide context and grouping; requirement tasks carry implementation and verification traceability.
- Separate tasks for new requirements, modified requirements, impacted reusable contracts, invalidated verifications, and implementation/evidence updates.
- Include owner routing, priority, risk, lifecycle status, source capability, requirement name, and relevant contract or semantic-contract context in each task.
- If a changed contract is reused through contract bindings, generate consumer review tasks for each impacted bound requirement.
- If a requirement changed, include tasks to review implementation `satisfiedBy` paths and verification `satisfiedBy` evidence paths.
- Do not write inherited/default governance metadata back into model files unless the user explicitly requests that modeling change.
- Include a clear warning in task summaries when the summary is not sufficient for implementation and the assignee must read the full collected requirement context.
- Organize plans into explicit phases when applicable: new requirements, modified requirements, affected reusable contracts, affected verifications, and final validation/evidence updates.

Answer discipline:
- Provide a task plan, not a vague summary. Each task should identify the model element, why it changed, what implementation or verification work is needed, and which Reqvire validation evidence should close it.
- Group work by capability or owner when that improves execution.
- Call out high or critical priority/risk first.
- Include counts for new requirements, modified requirements, affected tests/evidence, owner routing groups, high/critical priorities, and high/critical risks when the data is available.
- Include a final evidence checklist: validation evidence, `reqvire.lint`, `reqvire.coverage`, affected traces, and relevant tests.
