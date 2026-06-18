---
allowed-tools: Read, Bash(npx:*), Bash(jq:*), Bash(rg:*), SlashCommand
argument-hint: [model-root-or-focus]
description: Plan and execute a capability-centered Reqvire containment refactor without changing model intent
model: claude-sonnet-4-5
---

# Refactor Containment Structure

Plan and, only after the plan is clear, execute a Reqvire containment refactor. This command reorganizes folders and files around capability, ontology, and verification planes while preserving graph semantics.

Use this command when:

- capability-rooted content is scattered across unrelated folders,
- ontology content is mixed into capability or architecture files,
- verification files are hard to find or mixed with requirement files,
- a model needs a clearer `Capabilities/`, `Ontologies/`, and `Verifications/` structure,
- the user asks for model organization or containment refactoring, not just containment inspection.

For read-only inspection, use `/reqvire:containment`.

## Current Model Context

- Validation status: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate 2>&1 | head -1`
- Total elements: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json 2>/dev/null | jq -r '.global_counters.total_elements // "N/A"'`
- Capability roots: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" submodels --json 2>/dev/null | jq -r '.summary.total_submodels // "N/A"'`
- Files: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json 2>/dev/null | jq -r '.global_counters.total_files // "N/A"'`

## User Request

${1:+Focus: $1}
${1:-The user will provide the model root, target area, or refactor goal.}

## Canonical Reference

Use the syseng skill reference:

`claude-plugins/skills/syseng/reference/ContainmentStructureRefactor.md`

Default target shape, relative to the chosen Reqvire model root:

```text
./
  Capabilities/
  Ontologies/
  Verifications/
```

Treat this as guidance, not a forced schema. `./` is the model root selected for the project, such as `requirements/` or another folder relative to the workspace/repo root. Preserve stable project naming when it is valid and understandable.

## Procedure

### 1. Inspect physical containment

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" containment --json
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json
```

Summarize:

- current model root,
- top-level folders,
- files with many elements,
- files with one or two elements,
- mixed-type files,
- obvious naming drift.

### 2. Inspect graph ownership

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" submodels --json
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="capability" --short
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="ontology" --short
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="test-verification,formal-proof-verification,analysis-verification,inspection-verification,demonstration-verification" --short
```

Classify content:

- `capability` roots and child capability subgraphs,
- `ontology` elements and shared semantic vocabulary,
- requirements that specify capabilities,
- contracts owned by requirement elements,
- verification elements and evidence links.

### 3. Produce a plan before editing

Return a concrete plan grouped by operation type:

- **Move files** with `mv-file`.
- **Move elements** with `mv`.
- **Rewrite relations** with `link`, `unlink`, or `relink`.
- **Add reused_contract_context** where cross-subgraph context must be preserved.
- **Leave in place** when current containment is valid and moving would only create churn.

Flag high-risk changes:

- cross-submodel hierarchy changes,
- ontology extraction,
- files with many incoming links,
- verification moves,
- any broad capability split.

Do not run bulk move operations until the user confirms the plan when risk is non-trivial.

### 4. Execute in slices

Use dry runs first:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-file "<source-file>" "<target-file>" --dry-run
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv "<element-name>" "<target-file>" --dry-run
```

Apply one capability or ontology slice at a time, then validate.

### 5. Validate and report

After each slice:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" submodels
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" containment
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage
```

Final report should include:

- files moved,
- elements moved,
- relations or reused_contract_context changed,
- intentional exceptions to the recommended structure,
- validation/lint/coverage status.

## Guardrails

- Do not change requirements intent during containment refactors.
- Do not use folder paths as a substitute for `specify`, `define`, `verify`, or reused_contract_context.
- Do not force example area names like `Product/` or `Platform/` if the project already has clearer stable names.
- Do not move ontology into capability folders.
- Do not move shared architecture content blindly; extract reusable meaning to ontology or reusable contracts when needed.
- Do not leave removed or deprecated aliases behind. Use git history for migration history.
