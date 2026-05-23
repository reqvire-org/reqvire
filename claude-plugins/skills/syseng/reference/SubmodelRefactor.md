# Attachment-Boundary Submodel Refactor

Use this reference when splitting the model into independent submodels with attachment-based cross-boundary contracts.

**For common commands**, see [SKILL.md Command Cheatsheet](../SKILL.md#command-cheatsheet).

## Do It When

- The model must be split into several independent submodels
- Cross-submodel links must be attachments only (no direct cross-submodel relations)
- `collect` must provide all external specs needed by a consuming submodel
- `change-impact` must detect propagation through attached contracts

## Submodel Boundary Principle

- Reqvire models are structured as independent hierarchical submodels, each with clear ownership, lifecycle, and stakeholder responsibility
- Hierarchical relations are used only for internal decomposition within a submodel
- Cross-submodel dependencies are expressed through explicit attachment contracts, not hierarchical coupling
- This preserves boundary clarity and keeps `collect`, change-impact, and coverage outputs deterministic
- A broad feature root may own child features, but requirements should specify the local child feature when that child is the real capability slice
- Do not collapse unrelated work under one feature root just to share ontology; attach the shared ontology from the consuming feature instead

## Mandatory Human Boundary Check

Before applying refactor operations, confirm with the user:

- Submodel ownership map (who owns which folders/elements)
- Which cross-submodel dependencies are allowed as attachments
- Which relation types are forbidden across submodels (`derive`, `derivedFrom`, `refinedBy`, `verifiedBy`)
- Where shared contracts live (ontology elements for vocabulary, requirement-owned semantic contracts for shape profiles, or other requirement-owned refinement elements)

Do not run bulk unlink/move operations before this confirmation.

## Refactor Rule

When a relation crosses intended submodel boundaries, either:

1. Move/reparent to restore hierarchical ownership
2. Replace cross-boundary hierarchy links with attachment-based refinement contracts

When one feature root is too broad, first split it into real child features, then move requirements to specify the child feature that owns their local capability. Keep the parent feature as a capability grouping only when its children still form one coherent root submodel.

## Refactor Procedure (Recursive)

1. Start from each feature root and inspect its first-level feature and requirement children
2. For each first-level child, inspect all direct children and relation edges
3. Continue recursively for each descendant branch until leaf requirements
4. At each level, enforce:
   - hierarchical relations remain internal to that branch/submodel
   - cross-branch dependencies are attachment contracts
5. Re-run validation and submodel analysis after each boundary slice before continuing

## Internal Sub-Boundaries

A submodel may contain internal sub-boundaries (nested domains) with separate ownership and lifecycle. Cross-internal-boundary dependencies should be modeled as explicit attachment contracts when they represent contractual dependency, not hierarchical ownership.

## Workflow

1. **Audit cross-submodel relations and hotspots**
   - `reqvire search --short --json` — group by source/target folders
   - `reqvire lint --json` — prioritize `needs_manual_review` entries with `type: cross_submodel_hierarchical_relation`

2. **Define submodel boundaries**
   - Keep derivation/refinement/verification relations inside each submodel
   - Define allowed cross-boundary artifacts as attachments

3. **Migrate links**
   - For each cross-submodel relation, either move element into owning submodel or replace with attachment
   - Ensure each receiving feature submodel attaches required external ontology, and each receiving requirement attaches required reusable specifications or semantic contracts
   - Preserve dependency visibility: if a requirement relied on a moved concept, add concept references, a local refinement, or an attachment so `collect` still explains the dependency

4. **Validate semantic completeness**
   - `reqvire collect "<feature-or-requirement>" --json` must include required feature-attached ontology, specs, and semantic contracts
   - `reqvire change-impact --git-commit="<base>"` must report impacts when attached contracts change
   - Repeat `reqvire lint --json` — target: fewer or no `cross_submodel_hierarchical_relation` findings

5. **Run quality checks**
   - `reqvire validate && reqvire lint && reqvire coverage`

## Circle-Back Checkpoint (Human Confirmation)

Before applying refactor edits, explicitly confirm:

- Submodel ownership map (who owns which folders/elements)
- Which cross-submodel dependencies are allowed as attachments
- Which relation types are forbidden across submodels
- Whether shared contracts live as refinement elements (and which requirement owns each one)

Do not proceed with bulk unlink/move operations until this is confirmed.

## Correct vs Incorrect Patterns

**Correct** (attachment boundary):
- `Submodel A` requirement keeps internal `derive/refinedBy/verifiedBy` only within `Submodel A`
- `Submodel A` requirement attaches `Submodel B` contract/spec:
  - `reqvire link "A Requirement" attaching "requirements/Contracts/B/InterfaceSpec.md#api-contract"`
- `collect` for `A Requirement` includes the attached external contract content

**Incorrect** (cross-submodel relation leakage):
- `Submodel A` requirement directly uses `derivedFrom` to `Submodel B` requirement
- `Submodel A` requirement uses `refinedBy` to `Submodel B` specification
- This breaks independence and creates hidden coupling

## Report Expectations

**`collect` after refactor:**
- `reqvire collect "<A Requirement>" --json` should include local ancestry + attached external contracts
- Enough content to implement/review without cross-submodel relations

**`change-impact` after refactor:**
- If an attached contract changes, `reqvire change-impact --git-commit="<base>"` should list impacted consumers
- If consumers are missing, attachment boundary coverage is incomplete

## How Not To Do It

- Do not remove cross-submodel relations without replacing them by required attachments
- Do not assume attachment coverage is complete without checking `collect` output
- Do not rely on inferred boundaries — always confirm with the human user first
- Do not run mass refactors in one pass — refactor by boundary slice and validate each slice
