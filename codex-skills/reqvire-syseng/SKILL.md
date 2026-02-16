---
name: reqvire-syseng
description: MBSE and requirements engineering workflow for Reqvire. Use when exploring a Reqvire model, adding or refactoring requirements/specifications/verifications, running impact/coverage analysis, and generating implementation tasks with full traceability.
---

# Reqvire System Engineering Skill

Use this skill for any requirements, specifications, verifications, or system-model work in Reqvire.

## When To Use

- Exploring and understanding existing Reqvire models
- Adding features in MBSE order
- Refactoring or consolidating requirements/specifications
- Analyzing change impact and verification coverage
- Generating implementation tasks from model changes

## Core Rules

1. Work from repository root.
2. Prefer `reqvire` CLI commands over manual markdown edits.
3. MBSE-first sequence is mandatory:
   - Requirements
   - Refinements (specifications/constraints/behaviors)
   - Verifications
   - Code implementation links (`satisfiedBy`)
4. Validate after meaningful changes:
   - `reqvire validate`
   - `reqvire lint`
   - `reqvire coverage`
5. Use `reqvire collect` when implementing or reviewing requirements with trace dependencies.

## Quick Start: Common Workflows

1. Check tool availability:
   - `reqvire --version`
2. Explore current model:
   - `reqvire search --short --json | jq '.summary'`
   - `reqvire model`
3. Apply change using reqvire commands (`add`, `link`, `unlink`, `mv`, `rename`, `rm`).
4. Run validation and coverage.
5. Summarize what changed and what still needs action.

## Task Pattern: Attachment-Boundary Submodel Refactor

### Do It When

- The model should be split into independent submodels.
- Cross-submodel traceability must happen through attachments only.
- `collect` output should carry all external specification context without cross-submodel relations.
- `change-impact` should reflect dependency propagation through attached artifacts/specifications.

### Goal

Refactor relations so each submodel is internally connected, while any dependency on another submodel is represented as an attachment contract (file or refinement element), not a direct relation.

### Workflow

0. Confirm boundaries with the human user (mandatory):
   - Propose submodel list and allowed cross-boundary attachments.
   - Ask user to confirm ownership boundaries before changing relations.
   - Record final boundary contract in the change summary.
1. Identify cross-submodel relations:
   - `reqvire search --short --json`
   - Group by source/target folders and relation types.
2. Define submodel boundaries:
   - Keep derivation/refinement/verification relations inside each submodel.
   - Define allowed cross-boundary artifacts as attachments.
3. Migrate links:
   - For each cross-submodel relation, either move element into owning submodel or replace relation with attachment.
   - Ensure each receiving submodel attaches all required external specifications/constraints/behaviors.
4. Validate semantic completeness:
   - `reqvire collect "<requirement>" --json` must include required attached specs for implementation/review.
   - `reqvire change-impact --git-commit="<base>"` must report impacts when attached contracts change.
5. Run quality checks:
   - `reqvire validate`
   - `reqvire lint`
   - `reqvire coverage`

### Circle-Back Checkpoint (Human Confirmation)

Before applying refactor edits, explicitly confirm:

- Submodel ownership map (who owns which folders/elements).
- Which cross-submodel dependencies are allowed as attachments.
- Which relation types are forbidden across submodels (`derive`, `refinedBy`, `verifiedBy`, etc.).
- Whether shared contracts live as files, refinement elements, or both.

Do not proceed with bulk unlink/move operations until this is confirmed.

### Correct vs Incorrect Patterns

Correct (attachment boundary):

- `Submodel A` requirement keeps internal `derive/refinedBy/verifiedBy` only within `Submodel A`.
- `Submodel A` requirement attaches `Submodel B` contract/spec:
  - `reqvire link "A Requirement" attaching "requirements/Contracts/B/InterfaceSpec.md#api-contract"`
- `collect` for `A Requirement` includes the attached external contract content.

Incorrect (cross-submodel relation leakage):

- `Submodel A` requirement directly uses:
  - `derivedFrom` to `Submodel B` requirement
  - `refinedBy` to `Submodel B` specification
  - `verifiedBy` to `Submodel B` verification
- This breaks independence and creates hidden coupling that attachment boundaries are meant to prevent.

### Example Report Expectations

`collect` expectation (after refactor):

- Running `reqvire collect "<A Requirement>" --json` should include:
  - local ancestry from `Submodel A`
  - attached external contracts/specifications from `Submodel B`
  - enough content to implement/review `A Requirement` without cross-submodel relations

`change-impact` expectation (after refactor):

- If an attached contract changes (content, move, rename), then
  `reqvire change-impact --git-commit="<base>"` should list impacted elements in consuming submodels.
- If impact report does not include known consumers, attachment boundary coverage is incomplete.

### How Not To Do It

- Do not remove cross-submodel relations without replacing them by required attachments.
- Do not assume attachment coverage is complete without checking `collect` output.
- Do not rely on inferred boundaries; always confirm with the human user first.
- Do not run mass refactors in one pass; refactor by boundary slice and validate each slice.

## Task Pattern: Requirement-to-Refinement Content Extraction

### Do It When

- Requirement elements contain specification, constraint, or behavior details directly in body/`#### Details`.
- The model needs attachment-ready refinement contracts that can be reused by other submodels.
- You need to reduce requirement prose to intent-level EARS statements while preserving technical content.

### Goal

Move technical details out of requirement text into explicit refinement elements (`specification`, `constraint`, `behavior`) owned by the requirement via `refinedBy`, while keeping requirement intent clear and traceable.

### Mandatory Boundary Clarification (Human Checkpoint)

Before bulk extraction, confirm with the user:

- Which requirement families are in scope for extraction.
- Required split policy: what stays in requirement vs what must move to refinements.
- Naming convention for generated refinement elements.
- Whether existing refinement elements should be reused or new ones created.

Do not run bulk rewrites until this is confirmed.

### Workflow

1. Detect candidates where requirement details include technical implementation content.
2. Classify extracted content into `specification`, `constraint`, or `behavior`.
3. Create or reuse refinement elements and link ownership with `refinedBy`.
4. Move extracted content into refinement `#### Details`.
5. Replace requirement details with concise intent text (for example: "Implementation details shall follow associated refinement specifications.").
6. Validate and inspect reports:
   - `reqvire validate`
   - `reqvire lint`
   - `reqvire coverage --json`
   - targeted `reqvire collect "<requirement>" --json`

### Example Report Expectations

Expected after extraction:

- `validate`: no parse/relation/type errors.
- `lint`: no structural regressions introduced by split.
- `coverage`: verification links remain intact (no new orphaning from content-only refactor).
- `collect`: requirement chain includes the new refinement content needed for implementation/review.

### How Not To Do It

- Do not create empty refinement elements with placeholder text only.
- Do not delete technical details from requirements unless they are actually transferred to linked refinements.
- Do not change requirement intent statement semantics while performing extraction.
- Do not move verification intent into refinements; only specification/constraint/behavior details belong there.
- Do not run one-shot global rewrite without validating each requirement slice.

## Task Pattern: Design-Document Ownership Normalization

### Do It When

- Design documents under `DesignDocuments/` are linked only via `Attachments`.
- Refinement ownership is ambiguous after legacy modeling phases.
- You need exactly one owning requirement per design/refinement document.

### Goal

Assign each design/refinement document element to a single owning requirement via `refinedBy` (identifier target), and keep all other consumers as `Attachments`.

### Mandatory Boundary Clarification (Human Checkpoint)

Before bulk normalization, confirm:

- Ownership rule scope (all design docs vs selected submodels).
- Ownership tie-break policy when multiple requirements currently reference the same file.
- Whether any document should remain attachment-only by design.

### Workflow

1. Inventory all `DesignDocuments/*.md` references.
2. For each document, pick one owning requirement based on derivation/semantic proximity.
3. Convert owner link from attachment to `refinedBy` using the document element identifier (`DesignDocuments/File.md#element-fragment`), not a plain file path.
4. Keep non-owner references as attachments.
5. Validate one-owner rule (no second `refinedBy` owner for same file).
6. Run:
   - `reqvire validate`
   - `reqvire lint`
   - `reqvire coverage --json`

### Example Report Expectations

- `validate` passes with no relation/type violations.
- `collect` on owner requirement includes the design document element through refinement ownership.
- `change-impact` includes consumers when owned design contract changes and is attached downstream.

### How Not To Do It

- Do not convert all attachments to `refinedBy`; consumers must remain attachments.
- Do not assign multiple owners to the same design document.
- Do not change ownership without considering derivation hierarchy and requirement intent.

## Command Reference

```bash
# Explore
reqvire search --short --json | jq '.summary'
reqvire model
reqvire collect "Requirement Name" --json

# Validate quality
reqvire validate
reqvire lint
reqvire coverage

# Impact analysis
reqvire change-impact --git-commit="$(git merge-base main HEAD)"
```

## Validation & Quality Checklist

Run these after each meaningful change:

```bash
reqvire validate
reqvire lint
reqvire coverage
```

## References

Load only the reference file needed for the current task:

- `references/explore.md` - model exploration and advanced search patterns
- `references/AddFeature.md` - MBSE feature creation workflow
- `references/ConsolidateRequirements.md` - model refactoring and cleanup
- `references/CreatingTasks.md` - generate implementation tasks from requirement changes
- `references/SpecificationsExtractionLogic.md` - extract technical specs from requirements
