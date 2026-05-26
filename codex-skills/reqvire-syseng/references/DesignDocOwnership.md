# Design-Document Ownership Normalization

Use this reference when normalizing design document ownership so each document element has exactly one owning requirement.

**For common commands**, see [SKILL.md Command Cheatsheet](../SKILL.md#command-cheatsheet).

## Do It When

- `DesignDocuments/*.md` files are referenced via attachments but lack explicit owner requirement
- The model still contains legacy attachment-only refinement contracts
- You need one owning requirement per design/refinement document

## Goal

Assign each design/refinement document element to a single owning requirement via `refinedBy` (identifier target), while other requirements consume it through attachments.

This ownership is refinement/document ownership, not governance ownership. Governance `owner` metadata is a separate routing/accountability label on capability and requirement elements and may name a person, role, team, department, subsystem group, or task owner.

## Mandatory Boundary Clarification (Human Checkpoint)

Confirm before bulk edits:

- Scope (entire model or selected submodels)
- Tie-break rule when multiple candidate owners exist
- Exceptions that should stay attachment-only

## Workflow

1. Enumerate all references to `DesignDocuments/*.md`
2. Select a single owner requirement for each document by semantic/derivation fit
3. Convert owner link to `refinedBy` using document element identifier (`DesignDocuments/File.md#element-fragment`), not a plain file path
4. Keep all non-owner references as attachments
5. Verify no design document has multiple owner requirements
6. Run `reqvire validate`, `reqvire lint`, `reqvire coverage --json`

## Report Expectations

- `validate` passes with no relation/type errors
- `collect` on owner requirement includes the design document through refinement ownership
- `change-impact` includes consumers when owned design contract changes and is attached downstream

## How Not To Do It

- Do not blindly replace every attachment with `refinedBy`
- Do not assign multiple owners to one design document
- Do not choose owners without checking requirement intent and derivation context
