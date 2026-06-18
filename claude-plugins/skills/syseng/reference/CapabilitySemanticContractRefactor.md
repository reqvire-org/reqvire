# Capability and Semantic Contract Refactor

Use this workflow when migrating an existing Reqvire model toward clear capability semantics, ontology elements, contracts, and requirement-owned obligations.

## Goal

Separate three concerns without losing traceability:

- Capabilities own coherent operational/system ability, authored concept-reference context, and direct verification context.
- `ontology` elements own stable model/domain meaning: `X is`, `X has`, `X relates to Y`, allowed semantic structure, and shared OWL/Turtle vocabulary.
- Requirements own implementable obligations: what the system shall do, what can satisfy it, and what verification proves it.
- Reusable `semantic-contract` elements own SHACL shape profiles over ontology terms reached through explicit `use`/`usedBy` relations and constrain requirements through `constrain`/`constrainedBy`.
- Ontology referenced by model elements should define nouns, relationships, allowed semantic categories, and stable model rules.
- Exact commands, fields, URI patterns, workflow steps, outputs, file paths, and reject/write/emit behavior belong in compatible requirement-owned `source`, `specification`, `constraint`, `behavior`, `state`, and `input-output` contracts. Shape-only `semantic-contract` elements capture reusable SHACL profiles through explicit ontology use.

## Capability Modeling Philosophy

A capability represents a coherent operational, product, business, regulatory, or system ability that the system provides or supports. It is the primary semantic bridge between ontology, requirements, and verification.

Capabilities should describe what the system is able to accomplish rather than how the system is implemented. A capability is not a UI screen, deployment artifact, code module, ticket/task, or low-level implementation detail.

A good capability represents one coherent operational/system concern, one meaningful traceability anchor, and one stable semantic concept. It should remain stable, composable, implementation-independent, verifiable, and understandable by both humans and AI systems.

Use child capabilities when concerns differ in verification, ownership, lifecycle, architecture impact, operational semantics, or requirement clusters. Optional subsections such as `#### Stakeholder Need`, `#### Feature`, `#### Operational Context`, `#### Regulatory Driver`, `#### Mission Objective`, `#### Service Context`, `#### AI Context`, and `#### Notes` enrich capability interpretation but do not replace graph structure.

## System Model Construction Pattern

Use this method when building or refactoring a system model, not only when cleaning an existing semantic contract:

1. **Map capability-root subgraphs and ontology context first**
   - Run `reqvire submodels --json`.
   - Run `reqvire search --filter-type="ontology" --short`.
   - Treat each capability root as an independent coherent operational/system ability.
   - Keep ontology and semantic-contract elements under `requirements/Ontologies`; model elements consume ontology terms through concept references, and requirements consume semantic contracts through `constrainedBy`.
   - Treat ontology as a first-class semantic plane: it defines reusable terms and relationships that non-ontology elements reference explicitly.
   - Do not create one universal top capability just to reuse vocabulary. Shared meaning crosses roots through explicit concept references.

2. **Shape the capability hierarchy before moving requirements**
   - Add child capabilities only when they are real independently verifiable capability slices with their own local requirements or verification concerns.
   - Keep requirements under their local capability with `specify`/`specifiedBy`.
   - Parent capabilities may have no direct local requirements when their child capabilities own the concrete obligations.

3. **Shape the ontology hierarchy before writing local profiles**
   - Put nouns, classes, properties, semantic categories, and stable relationship rules in `ontology`.
   - Reuse existing ontology terms when they already describe the concept.
   - Add ontology terms before adding `#### Concept References` or SHACL shapes that depend on those terms.
   - Keep exact command names, fields, URI patterns, workflow steps, output formats, persistence behavior, and reject/write/emit behavior out of ontology.
   - If prose moved out of a requirement still matters to implementation, preserve it as a requirement-owned contract instead of deleting it.

4. **Keep obligations in requirements**
   - Requirements state what the system shall do.
   - Requirements may use `#### Concept References` to bind readable text to ontology terms.
   - `semantic-contract` elements contain `#### Shapes` only, never `#### Ontology`, use ontology through `use`/`usedBy`, and constrain requirements through `constrain`/`constrainedBy`.

5. **Preserve boundaries through reused_contract_context**
   - Use hierarchy only inside a capability, requirement, or ontology family.
   - Use concept references for cross-root ontology term reuse.
   - Use requirement reused_contract_context for cross-root reusable requirement-owned contracts.
   - Use `use` for semantic-contract ontology dependencies and `constrain` for semantic-contract requirement dependencies.
   - After changing reused_contract_context or hierarchy, check `submodels --json` for unintended cross-submodel couplings.

6. **Update verification and tests in the same slice**
   - Verifications may verify capabilities or requirements directly.
   - Capability coverage is rollup from specifying requirements and child capabilities.
   - If names, hierarchy, report shape, or fixtures change, update verifications and e2e expected files before finishing.

## When To Use

Use this workflow when:

- A requirement reads like a vocabulary catalog, domain definition, type taxonomy, relation dictionary, or data-shape definition.
- A capability contains implementable system obligations instead of capability scope.
- A semantic contract lacks explicit ontology `use` relations or requirement `constrain`/`constrainedBy` relations where it is intended to govern requirements.
- A capability-root subgraph has no concrete requirements.
- A requirement duplicates ontology facts already present in an ontology element.
- A semantic contract needs local `Ontology`; it should usually become ontology element plus shape-only `semantic-contract`.

Do not use this workflow for simple duplicate merges. Use `ConsolidateRequirements.md` for merge cleanup.

## Decision Rules

Put content in a capability when it answers:

- What coherent operational, product, business, regulatory, or system ability is this?
- Why does this area exist?
- What stakeholder need, feature context, operational context, regulatory driver, mission objective, service context, AI context, source context, or ontology defines its meaning?
- What source context or ontology defines this capability's language?
- Which requirements belong under this capability?
- Which verification evidence directly verifies it when capability-level evidence is appropriate?

Put content in an ontology element when it says:

- `X is a Y`
- `X has property Z`
- `X relates to Y`
- these fields form a valid object
- this term means...
- this ontology class or property is part of shared model language

Put content in a requirement when it says:

- The system shall...
- When/while/if a condition applies, the system shall...
- The implementation shall expose, validate, reject, generate, collect, report, persist, or process something.
- A verification or implementation artifact can prove it.

Put content in a semantic contract when:

- One or more requirement obligations need a closed-world SHACL profile.
- The profile only uses ontology terms reachable through the semantic contract's explicit ontology-use graph.
- The contract contains `#### Shapes` and no `#### Ontology`.

## Audit Commands

Run from the repository root.

```bash
reqvire validate
reqvire submodels
reqvire search --filter-type="capability" --short
reqvire search --filter-type="semantic-contract" --short
reqvire search --filter-type="requirement" --filter-content="(?i)(\\bis a\\b|\\bhas property\\b|\\bvocabulary\\b|\\bontology\\b|\\bsemantic contract\\b|\\bdefines\\b)" --short
reqvire search --filter-type="semantic-contract" --not-have-relations="use" --short
reqvire search --filter-type="semantic-contract" --not-have-relations="constrain" --short
```

Use `collect` before editing a candidate:

```bash
reqvire collect "<capability-or-requirement-name>" --json
reqvire collect "<capability-name>" --direction DOWNSTREAM --json
```

## Refactor Procedure

### 1. Inspect Capability Roots

Run `reqvire submodels`.

For each capability root:

- Confirm it is a real independent capability root.
- Check whether it has requirements through `specifiedBy`/`specify`.
- If it has zero requirements, confirm it is still a meaningful capability because it has child capabilities or direct verification; otherwise add a concrete obligation that specifies it or move pure vocabulary into `requirements/Ontologies` and reference it from consuming elements.
- Confirm cross-root dependencies are reused_contract_context, not hierarchy relations.
- If one root is too broad, split it into child capabilities first and move local requirements to the child capabilities before editing ontology or contracts.

### 2. Classify Candidate Text

For each candidate requirement or contract, split text into:

- capability ability/context
- reusable semantic meaning
- implementable obligation
- requirement-specific validation/profile detail
- verification/evidence detail

Do not delete meaning. Move it to the correct element type.

### 3. Move Reusable Meaning Into Ontology

If stable semantic meaning is currently in a requirement:

- Add or update an `ontology` element.
- Place authored ontology and semantic-contract elements in `requirements/Ontologies`, grouped by semantic area.
- Put ontology vocabulary in `#### Ontology`.
- Do not put `#### Shapes` in ontology.
- Add concept references from the model elements that need that vocabulary.
- Remove duplicated semantic prose from the requirement after preserving the obligation.
- Keep ontology focused on terms and relationships. If a statement names concrete CLI/MCP commands, exact output fields, file paths, report sections, validation messages, or mutation steps, move it to `specification`, `behavior`, `state`, `input-output`, or a semantic-contract shape profile instead.

### 4. Keep Obligations In Requirements

Requirements should remain short and testable.

Good requirement shape:

```markdown
### Access Token Rejection

The system shall reject API requests whose access token does not conform to the access token semantic contract.

#### Metadata
  * type: requirement

#### Relations
  * specify: [API Authentication](Feature.md#api-authentication)
  * constrainedBy: [Access Token Request Shape](#access-token-request-shape)
  * verifiedBy: [Access Token Rejection Test](Verifications.md#access-token-rejection-test)
---
```

### 5. Use Shape Contracts For Local Profiles

When an obligation needs specific closed-world validation:

- Create a `semantic-contract` under `requirements/Ontologies` near the ontology it uses, then link it to the requirement with `constrain`/`constrainedBy`.
- Include `#### Shapes`.
- Do not include `#### Ontology`.
- Add `use` relations to the ontology elements that declare the SHACL terms.
- Use only ontology terms declared by the semantic contract's explicit ontology-use graph.

### 6. Wire Relations And Reused Contract Context

Use:

- `capability specifiedBy requirement` or `requirement specify capability` for ownership.
- concept references from model elements to ontology terms.
- `semantic-contract constrain requirement` or `requirement constrainedBy semantic-contract` for SHACL profile application.
- `semantic-contract use ontology` or `ontology usedBy semantic-contract` for ontology vocabulary dependencies.
- Capabilities do not author reused_contract_context; they use concept references for ontology terms.
- Requirement reused_contract_context only for compatible requirement-owned `source`, `constraint`, `behavior`, `specification`, `state`, or `input-output` contracts.

Use semantic relation families or concept references instead of a generic relation to preserve ownership or dependency meaning.
Do not remove a cross-root dependency unless the consumer now has an explicit concept reference, semantic-contract relation, or requirement reused_contract_context that gives `collect` and change impact the same dependency path.

### 7. Update Verifications And Tests

When requirements move or split:

- Use `verifiedBy`/`verify` links when verification intentionally targets capabilities or requirements.
- Add verification coverage for new obligations.
- Update e2e fixtures and expected output when submodel counts, coverage, search output, or ontology export output changes.
- Keep verifications evidence-backed when they require files through `satisfiedBy`.

### 8. Validate In Slices

After each meaningful slice:

```bash
reqvire validate
reqvire lint
reqvire submodels
reqvire coverage
```

Run focused e2e tests for touched behavior, then full e2e before finishing.

## Completion Criteria

- Every requirement resolves to exactly one owning capability root.
- Capability roots have specifying requirements, child capabilities, or intentional direct verification. Pure vocabulary belongs in ontology and is referenced by consuming elements.
- Semantic meaning is not duplicated in requirements and ontology elements.
- Semantic contracts contain `Shapes` only and no `Ontology`.
- Semantic-contract references resolve through explicit ontology `use` context and ontology hierarchy.
- `reqvire validate`, `reqvire lint`, and relevant tests pass.

## Pitfalls

- Do not create one universal capability root for all shared contracts.
- Do not move workflow behavior or implementation commitments into ontology just because the text is structured.
- Do not let SHACL profiles introduce new ontology terms.
- Do not remove cross-subgraph relations without replacing intentional dependencies with reused_contract_context.
- Do not leave capability roots with zero requirements unless they have child capabilities or direct verification; move pure vocabulary to ontology.
