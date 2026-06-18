---
allowed-tools: Read, Bash(npx:*), Bash(rg:*), Bash(jq:*)
description: Refactor capabilities, ontology, semantic contracts, and requirements into canonical Reqvire ownership boundaries
model: claude-sonnet-4-5
---

# Capability, Ontology, and Semantic Contract Refactor

Refactor the Reqvire model so capability scope, ontology meaning, semantic-contract shape profiles, and requirement obligations are separated without losing traceability.

Use this command when:

- Requirements contain vocabulary, ontology, taxonomy, or domain-definition prose.
- Capability roots are missing concrete specifying requirements.
- Ontology definitions and semantic contracts are not clearly separated.
- Semantic contracts define ontology instead of shape profiles.
- A model slice needs to move from prose definitions to ontology elements.

Do not use this command for simple duplicate merges. Use `/reqvire:consolidate` for merge cleanup.

## Model Context

- Validation status: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate 2>&1 | head -1`
- Submodels: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" submodels --json 2>/dev/null | jq -r '"\(.summary.total_submodels // "N/A") capability roots, \(.summary.total_requirements // "N/A") requirements, \(.summary.total_cross_submodel_couplings // "N/A") cross-submodel couplings"'`
- Semantic contracts: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="semantic-contract" --short 2>/dev/null | wc -l | tr -d ' '`

## Canonical Split

Use the syseng skill reference:

`claude-plugins/skills/syseng/reference/CapabilitySemanticContractRefactor.md`

Apply these boundaries:

- Capability: coherent operational/system ability, stakeholder/regulatory/source scope, ownership, optional concept-reference context, optional semantic-enrichment context, and direct verification context.
- `ontology`: reusable ontology/domain meaning and shared semantic structures.
- Requirement: implementable, testable system obligation.
- `semantic-contract`: reusable SHACL `Shapes` profile over explicitly used ontology context; constrains requirements through `constrain`/`constrainedBy`.

Ontology referenced by model elements should live under `system-model/Ontologies` and define nouns, relationships, allowed semantic categories, and stable model rules. Exact commands, fields, URI patterns, workflow steps, outputs, file paths, and reject/write/emit behavior belong in compatible requirement-owned `source`, `specification`, `constraint`, `behavior`, `state`, and `input-output` contracts. Shape-only `semantic-contract` elements are reusable checks with explicit ontology `use` and requirement `constrain` relations.

## Procedure

### 1. Inspect current capability roots and ontology context

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" submodels
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="ontology" --short
```

For each touched capability root:

- Confirm it is a real independent capability root.
- Confirm it has specifying requirements, child capabilities, or intentional direct verification. Move pure vocabulary into ontology.
- Confirm cross-root contract dependencies are explicit reused_contract_context, not hierarchy relations.
- If a root is too broad, split it into meaningful child capabilities first and move requirements to specify their local child capability.
- Do not make one universal capability root just to reuse ontology. Shared ontology terms cross roots through explicit concept references.
- Confirm reusable terms are in ontology elements, not hidden in requirement prose or semantic contracts.
- Confirm model elements author concept references for the ontology terms their prose depends on; semantic contracts use ontology through `use`.

### 2. Find refactor candidates

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="requirement" --filter-content="(?i)(\\bis a\\b|\\bhas property\\b|\\bvocabulary\\b|\\bontology\\b|\\bsemantic contract\\b|\\bdefines\\b)" --short
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="semantic-contract" --short
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="semantic-contract" --not-have-relations="use" --short
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="semantic-contract" --not-have-relations="constrain" --short
```

Before editing a candidate, collect context:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" collect "<capability-or-requirement>" --json
```

### 3. Classify and move content

For each candidate, classify each sentence:

- Capability/source context goes to capability text or requirement-owned source contracts.
- Stable semantic meaning goes to `ontology`.
- System obligation stays in requirement.
- Local SHACL profile goes to a reusable `semantic-contract` with explicit `use` and `constrain` relations.
- Verification criteria stays in verification elements.
- Exact commands, fields, URI patterns, workflow steps, output formats, file paths, persistence behavior, and reject/write/emit behavior stay in compatible requirement-owned contracts, not ontology.

Do not delete content until it has a new owner.

### 4. Wire relations

Use:

- `requirement specify capability`
- `capability specifiedBy requirement`
- `#### Concept References` from non-ontology elements to ontology terms
- `semantic-contract use ontology` for vocabulary context
- `semantic-contract constrain requirement` or `requirement constrainedBy semantic-contract` for shape profile application
- Reused Contract Context for reusable requirement-owned requirement-owned contracts

Use semantic relation families or concept references instead of a generic relation to preserve ownership or dependency meaning.
Do not remove a cross-root dependency unless an explicit concept reference, semantic-contract relation, or requirement reused_contract_context preserves the dependency for `collect` and change impact.

### 5. Update verification and tests

If the refactor changes requirements or output structure:

- Update `verifiedBy`/`verify` links.
- Update e2e fixtures and expected output files.
- Add or update verification text so the model explains the expected test behavior.
- Run focused e2e tests for touched behavior.

### 6. Validate in slices

After each slice:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" submodels
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage
```

Run full e2e before finishing when requirements, verifications, or expected report output changed.

## Completion Checklist

- Requirements answer what the system shall do.
- Capabilities answer what coherent operational/system ability owns the slice.
- Ontology elements define shared ontology/domain meaning.
- Semantic contracts contain `Shapes` only and no `Ontology`.
- Verifications may verify capabilities or requirements directly.
- Submodel counts and coverage changes are intentional and reflected in tests.
- `validate`, `lint`, focused e2e, and full e2e pass.
