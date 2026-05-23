---
allowed-tools: Read, Bash(npx:*), Bash(rg:*), Bash(jq:*)
description: Refactor features, ontology, semantic contracts, and requirements into canonical Reqvire ownership boundaries
model: claude-sonnet-4-5
---

# Feature, Ontology, and Semantic Contract Refactor

Refactor the Reqvire model so feature scope, ontology meaning, semantic-contract shape profiles, and requirement obligations are separated without losing traceability.

Use this command when:

- Requirements contain vocabulary, ontology, taxonomy, or domain-definition prose.
- Feature roots are missing concrete specifying requirements.
- Ontology definitions and requirement-owned semantic contracts are not clearly separated.
- Requirement-owned semantic contracts define ontology instead of shape profiles.
- A model slice needs to move from prose definitions to ontology elements.

Do not use this command for simple duplicate merges. Use `/reqvire:consolidate` for merge cleanup.

## Model Context

- Validation status: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate 2>&1 | head -1`
- Submodels: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" submodels --json 2>/dev/null | jq -r '"\(.summary.total_submodels // "N/A") feature roots, \(.summary.total_requirements // "N/A") requirements, \(.summary.total_cross_submodel_couplings // "N/A") cross-submodel couplings"'`
- Semantic contracts: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="semantic-contract" --short 2>/dev/null | wc -l | tr -d ' '`

## Canonical Split

Use the syseng skill reference:

`claude-plugins/skills/syseng/reference/FeatureSemanticContractRefactor.md`

Apply these boundaries:

- Feature: capability, product area, stakeholder/regulatory/source scope, ownership, and ontology context.
- `ontology`: reusable ontology/domain meaning and shared semantic structures.
- Requirement: implementable, testable system obligation.
- Requirement-owned `semantic-contract`: obligation-specific SHACL `Shapes` profile over reachable ontology context.

Ontology attached by features should live under `requirements/Ontologies` and define nouns, relationships, allowed semantic categories, and stable model rules. Exact commands, fields, URI patterns, workflow steps, outputs, file paths, and reject/write/emit behavior belong in requirement-owned `specification`, `behavior`, `state`, `input-output`, or shape-only `semantic-contract` refinements.

## Procedure

### 1. Inspect current feature roots and ontology context

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" submodels
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="ontology" --short
```

For each touched feature root:

- Confirm it is a real independent capability root.
- Confirm it has specifying requirements or is intentionally ontology-only and attached by consumers.
- Confirm cross-root dependencies are explicit attachments, not hierarchy relations.
- If a root is too broad, split it into meaningful child features first and move requirements to specify their local child feature.
- Do not make one universal feature root just to reuse ontology. Shared ontology crosses roots through attachments.
- Confirm reusable terms are in ontology elements, not hidden in requirement prose or requirement-owned semantic contracts.
- Confirm features attach the ontology their requirements need; requirements inherit ontology through the owning feature path.

### 2. Find refactor candidates

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="requirement" --filter-content="(?i)(\\bis a\\b|\\bhas property\\b|\\bvocabulary\\b|\\bontology\\b|\\bsemantic contract\\b|\\bdefines\\b)" --short
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="semantic-contract" --short
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="semantic-contract" --not-have-relations="refine" --short
```

Before editing a candidate, collect context:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" collect "<feature-or-requirement>" --json
```

### 3. Classify and move content

For each candidate, classify each sentence:

- Capability/source context goes to feature or feature-owned `source`.
- Stable semantic meaning goes to `ontology`.
- System obligation stays in requirement.
- Local SHACL profile goes to requirement-owned `semantic-contract`.
- Verification criteria stays in verification elements.
- Exact commands, fields, URI patterns, workflow steps, output formats, file paths, persistence behavior, and reject/write/emit behavior stay in requirement-owned refinements, not ontology.

Do not delete content until it has a new owner.

### 4. Wire relations

Use:

- `requirement specify feature`
- `feature specifiedBy requirement`
- feature `Attachments` to ontology elements
- `requirement refinedBy semantic-contract` for shape profiles
- Attachments for intentional cross-feature ontology dependencies and reusable requirement-owned contracts

Do not use `trace` to replace ownership or dependency.
Do not remove a cross-root dependency unless an explicit attachment preserves the dependency for `collect` and change impact.

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
- Features answer what capability/domain/source context owns the slice.
- Ontology elements define shared ontology/domain meaning.
- Requirement-owned semantic contracts contain `Shapes` only and no `Ontology`.
- Verifications verify requirements, not features directly.
- Submodel counts and coverage changes are intentional and reflected in tests.
- `validate`, `lint`, focused e2e, and full e2e pass.
