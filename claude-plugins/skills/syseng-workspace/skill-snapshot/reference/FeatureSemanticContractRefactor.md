# Feature and Semantic Contract Refactor

Use this workflow when migrating an existing Reqvire model toward ontology elements and requirement-owned obligations.

## Goal

Separate three concerns without losing traceability:

- Features own capability scope, source context, and reusable ontology vocabulary.
- `ontology` elements own stable model/domain meaning: `X is`, `X has`, `X relates to Y`, allowed semantic structure, and shared OWL/Turtle vocabulary.
- Requirements own implementable obligations: what the system shall do, what can satisfy it, and what verification proves it.
- Requirement-owned `semantic-contract` refinements own obligation-specific SHACL shape profiles over ontology terms reachable from the requirement's owning feature context.
- Ontology attached by features should define nouns, relationships, allowed semantic categories, and stable model rules.
- Exact commands, fields, URI patterns, workflow steps, outputs, file paths, and reject/write/emit behavior belong in requirement-owned `specification`, `behavior`, `state`, `input-output`, or shape-only `semantic-contract` refinements.

## System Model Construction Pattern

Use this method when building or refactoring a system model, not only when cleaning an existing semantic contract:

1. **Map feature-root subgraphs and ontology context first**
   - Run `reqvire submodels --json`.
   - Run `reqvire search --filter-type="ontology" --short`.
   - Treat each feature root as an independent product/capability/stakeholder/regulatory area.
   - Keep ontology elements under `requirements/Ontologies`; feature files consume ontology through attachments.
   - Treat ontology as a first-class semantic plane: it defines reusable terms and relationships that features explicitly attach so requirements inherit them through feature context.
   - Do not create one universal top feature just to reuse vocabulary. Shared meaning crosses roots through attachments.

2. **Shape the feature hierarchy before moving requirements**
   - Add subfeatures only when they are real capability slices with their own local requirements.
   - Keep requirements under their local feature with `specify`/`specifiedBy`.
   - Parent features may have no direct local requirements when their child features own the concrete obligations.

3. **Shape the ontology hierarchy before writing local profiles**
   - Put nouns, classes, properties, semantic categories, and stable relationship rules in `ontology`.
   - Reuse existing ontology terms when they already describe the concept.
   - Add ontology terms before adding `#### Concept References` or requirement-owned SHACL shapes that depend on those terms.
   - Keep exact command names, fields, URI patterns, workflow steps, output formats, persistence behavior, and reject/write/emit behavior out of ontology.
   - If prose moved out of a requirement still matters to implementation, preserve it as a requirement-owned refinement instead of deleting it.

4. **Keep obligations in requirements**
   - Requirements state what the system shall do.
   - Requirements may use `#### Concept References` to bind readable text to ontology terms.
   - Requirement-owned `semantic-contract` elements contain `#### Shapes` only and profile reachable ontology terms for one obligation.

5. **Preserve boundaries through attachments**
   - Use hierarchy only inside a feature, requirement, or ontology family.
   - Use feature attachments for cross-root ontology reuse.
   - Use requirement attachments for cross-root reusable requirement-owned contracts.
   - After changing attachments or hierarchy, check `submodels --json` for unintended cross-submodel couplings.

6. **Update verification and tests in the same slice**
   - Verifications verify requirements, not features directly.
   - Feature coverage is rollup from specifying requirements and child features.
   - If names, hierarchy, report shape, or fixtures change, update verifications and e2e expected files before finishing.

## When To Use

Use this workflow when:

- A requirement reads like a vocabulary catalog, domain definition, type taxonomy, relation dictionary, or data-shape definition.
- A feature contains implementable system obligations instead of capability scope.
- A semantic contract is attached or referenced without a clear owning feature or requirement.
- A feature-root subgraph has no concrete requirements.
- A requirement duplicates ontology facts already present in an ontology element.
- A requirement-owned semantic contract needs local `Ontology`; it should usually become ontology element plus requirement-owned `Shapes`.

Do not use this workflow for simple duplicate merges. Use `ConsolidateRequirements.md` for merge cleanup.

## Decision Rules

Put content in a feature when it answers:

- What product capability, stakeholder area, regulatory area, external obligation, or domain slice is this?
- Why does this area exist?
- What source context or ontology defines this capability's language?
- Which requirements belong under this capability?

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

Put content in a requirement-owned semantic contract when:

- One obligation needs a closed-world SHACL profile.
- The profile only uses ontology terms reachable through the owning requirement's feature context.
- The contract contains `#### Shapes` and no `#### Ontology`.

## Audit Commands

Run from the repository root.

```bash
reqvire validate
reqvire submodels
reqvire search --filter-type="feature" --short
reqvire search --filter-type="semantic-contract" --short
reqvire search --filter-type="requirement" --filter-content="(?i)(\\bis a\\b|\\bhas property\\b|\\bvocabulary\\b|\\bontology\\b|\\bsemantic contract\\b|\\bdefines\\b)" --short
reqvire search --filter-type="semantic-contract" --not-have-relations="refine" --short
```

Use `collect` before editing a candidate:

```bash
reqvire collect "<feature-or-requirement-name>" --json
reqvire collect "<feature-name>" --direction DOWNSTREAM --json
```

## Refactor Procedure

### 1. Inspect Feature Roots

Run `reqvire submodels`.

For each feature root:

- Confirm it is a real independent capability root.
- Check whether it has requirements through `specifiedBy`/`specify`.
- If it has zero requirements, either add a concrete obligation that specifies it or move pure vocabulary into `requirements/Ontologies` and attach it from consuming features.
- Confirm cross-root dependencies are attachments, not hierarchy relations.
- If one root is too broad, split it into child features first and move local requirements to the child features before editing ontology or refinements.

### 2. Classify Candidate Text

For each candidate requirement or refinement, split text into:

- capability context
- reusable semantic meaning
- implementable obligation
- requirement-specific validation/profile detail
- verification/evidence detail

Do not delete meaning. Move it to the correct element type.

### 3. Move Reusable Meaning Into Ontology

If stable semantic meaning is currently in a requirement:

- Add or update an `ontology` element.
- Place authored ontology elements in `requirements/Ontologies`, grouped by semantic area.
- Put ontology vocabulary in `#### Ontology`.
- Do not put `#### Shapes` in ontology.
- Attach the ontology from the feature that needs that vocabulary.
- Remove duplicated semantic prose from the requirement after preserving the obligation.
- Keep ontology focused on terms and relationships. If a statement names concrete CLI/MCP commands, exact output fields, file paths, report sections, validation messages, or mutation steps, move it to `specification`, `behavior`, `state`, `input-output`, or a requirement-owned shape profile instead.

### 4. Keep Obligations In Requirements

Requirements should remain short and testable.

Good requirement shape:

```markdown
### Access Token Rejection

The system shall reject API requests whose access token does not conform to the access token semantic contract.

#### Metadata
  * type: requirement

#### Relations
  * specify: [API Authentication](../Features.md#api-authentication)
  * refinedBy: [Access Token Request Shape](#access-token-request-shape)
  * verifiedBy: [Access Token Rejection Test](Verifications.md#access-token-rejection-test)
---
```

### 5. Use Requirement-Owned Shapes For Local Profiles

When an obligation needs specific closed-world validation:

- Create a `semantic-contract` refining the requirement.
- Include `#### Shapes`.
- Do not include `#### Ontology`.
- Use only ontology terms declared by reachable ontology context.

### 6. Wire Relations And Attachments

Use:

- `feature specifiedBy requirement` or `requirement specify feature` for ownership.
- feature `Attachments` to ontology elements.
- `requirement refinedBy semantic-contract` for obligation-specific SHACL profiles.
- Feature attachments only for ontology elements from other feature roots.
- Requirement attachments only for requirement-owned semantic contracts or requirement-detail refinements.

Do not use `trace` as a substitute for ownership or dependency.
Do not remove a cross-root dependency unless the consumer now has an explicit attachment that gives `collect` and change impact the same dependency path.

### 7. Update Verifications And Tests

When requirements move or split:

- Update `verifiedBy`/`verify` links to verify requirements, not features.
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

- Every requirement resolves to exactly one owning feature root.
- Feature roots either have specifying requirements or are intentionally ontology-only and attached by consumers.
- Semantic meaning is not duplicated in requirements and ontology elements.
- Requirement-owned semantic contracts contain `Shapes` only and no `Ontology`.
- Semantic references resolve through reachable ontology context and explicit ontology attachments.
- `reqvire validate`, `reqvire lint`, and relevant tests pass.

## Pitfalls

- Do not create one universal feature root for all shared contracts.
- Do not move workflow behavior or implementation commitments into ontology just because the text is structured.
- Do not let requirement-owned SHACL profiles introduce new ontology terms.
- Do not remove cross-subgraph relations without replacing intentional dependencies with attachments.
- Do not leave feature roots with zero requirements unless they are deliberately ontology-only and consumed through attachments.
