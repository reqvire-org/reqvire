# Contract Dependency Boundary Submodel Refactor

Use this reference when splitting the model into independent submodels with explicit contract dependencies across their boundaries.

**For common commands**, see [SKILL.md Command Cheatsheet](../SKILL.md#command-cheatsheet).

## Do It When

- The model must be split into several independent submodels
- Cross-submodel links must be contract bindings or references only (no direct cross-submodel relations)
- `collect` must provide all external specs needed by a consuming submodel
- `change-impact` must detect propagation through bound and referenced contracts

## Submodel Boundary Principle

- Reqvire models are structured as independent hierarchical submodels, each with clear ownership, lifecycle, and stakeholder responsibility
- Hierarchical relations are used only for internal decomposition within a submodel
- Cross-submodel dependencies use bindings for implementation obligations and references for content dependencies, not hierarchical coupling
- This preserves boundary clarity and keeps `collect`, change-impact, and coverage outputs deterministic
- A broad capability root may own child capabilities, but requirements should specify the local child capability when that child is the real capability slice
- Do not collapse unrelated work under one capability root just to share ontology; use explicit concept references from the consuming elements instead

## Mandatory Human Boundary Check

Before applying refactor operations, confirm with the user:

- Submodel ownership map (who owns which folders/elements)
- Which cross-submodel dependencies are allowed as contract bindings or references
- Which relation types are forbidden across submodels (`derive`, `derivedFrom`, `definedBy`, `verifiedBy`)
- Where shared contracts live (ontology elements for vocabulary, semantic-contract elements for reusable shape profiles, or compatible requirement-owned contract elements)

Do not run bulk unlink/move operations before this confirmation.

## Refactor Rule

When a relation crosses intended submodel boundaries, either:

1. Move/reparent to restore hierarchical ownership
2. Replace cross-boundary hierarchy links with bindings for implementation obligations or references for content dependencies

When one capability root is too broad, first split it into real child capabilities, then move requirements to specify the child capability that owns their local capability. Keep the parent capability as a capability grouping only when its children still form one coherent root submodel.

## Refactor Procedure (Recursive)

1. Start from each capability root and inspect its first-level capability and requirement children
2. For each first-level child, inspect all direct children and relation edges
3. Continue recursively for each descendant branch until leaf requirements
4. At each level, enforce:
   - hierarchical relations remain internal to that branch/submodel
   - cross-branch dependencies use the appropriate contract dependency kind
5. Re-run validation and submodel analysis after each boundary slice before continuing

## Internal Sub-Boundaries

A submodel may contain internal sub-boundaries (nested domains) with separate ownership and lifecycle. Cross-internal-boundary dependencies use bindings for delegated implementation obligations and references for content dependencies. Neither changes hierarchical ownership.

## Workflow

1. **Audit cross-submodel relations and hotspots**
   - `reqvire search --short --json` — group by source/target folders
   - `reqvire lint --json` — prioritize `needs_manual_review` entries with `type: cross_submodel_hierarchical_relation`

2. **Define submodel boundaries**
   - Keep derivation/contract/verification relations inside each submodel
   - Define allowed cross-boundary artifacts as contract bindings or references

3. **Migrate links**
   - For each cross-submodel relation, either move element into owning submodel or replace with contract bindings or references
   - Ensure each receiving element authors required concept references, each receiving requirement reuses required reusable requirement-owned contracts, and semantic contracts are linked with `constrainedBy`/`constrain`
   - Preserve dependency visibility: use Concept References for moved concepts; for requirement-owned contracts, choose bindings or references according to the consumer's responsibility

4. **Validate semantic completeness**
   - `reqvire collect "<capability-or-requirement>" --json` must include authored concept references, reused/refining specs, and explicitly constraining semantic contracts
   - `reqvire change-impact --git-commit="<base>"` must report impacts when bound or referenced contracts change
   - Repeat `reqvire lint --json` — target: fewer or no `cross_submodel_hierarchical_relation` findings

5. **Run quality checks**
   - `reqvire validate && reqvire lint && reqvire coverage`

## Circle-Back Checkpoint (Human Confirmation)

Before applying refactor edits, explicitly confirm:

- Submodel ownership map (who owns which folders/elements)
- Which cross-submodel dependencies are allowed as contract bindings or references
- Which relation types are forbidden across submodels
- Whether shared contracts live as contract elements (and which requirement owns each one)

Do not proceed with bulk unlink/move operations until this is confirmed.

## Correct vs Incorrect Patterns

**Correct** (explicit contract dependency):
- `Submodel A` requirement keeps internal `derive/definedBy/verifiedBy` only within `Submodel A`
- `Submodel A` requirement implements an obligation from `Submodel B` contract/spec:
  - `reqvire link "A Requirement" bindContract "system-model/Contracts/B/InterfaceSpec.md#api-contract"`
- A documentation or result-consuming requirement uses `referenceContract` instead of `bindContract`.
- `collect` includes both kinds; only binding consumers contribute to the contract owner's implementation roll-up.
- Each requirement authors at most one dependency section. References must remain acyclic through owners, bindings, and requirement ancestry.

**Incorrect** (cross-submodel relation leakage):
- `Submodel A` requirement directly uses `derivedFrom` to `Submodel B` requirement
- `Submodel A` requirement uses `definedBy` to `Submodel B` specification
- This breaks independence and creates hidden coupling

## Report Expectations

**`collect` after refactor:**
- `reqvire collect "<A Requirement>" --json` should include local ancestry + reused external contracts
- Enough content to implement/review without cross-submodel relations

**`change-impact` after refactor:**
- If a bound or referenced contract changes, `reqvire change-impact --git-commit="<base>"` should list impacted consumers
- If consumers are missing, the cross-boundary contract dependencies are incomplete

## How Not To Do It

- Do not remove cross-submodel relations without replacing them by required contract bindings or references
- Check `collect` output to confirm that all required contract dependencies are present
- Do not rely on inferred boundaries — always confirm with the human user first
- Do not run mass refactors in one pass — refactor by boundary slice and validate each slice
