# Local Contract Ownership Normalization

Use this reference when normalizing contract ownership so each contract element lives next to the requirement it defines and has exactly one owning requirement.

**For common commands**, see [SKILL.md Command Cheatsheet](../SKILL.md#command-cheatsheet).

## Do It When

- Contract files or elements are physically separated from the requirement they define
- Some reusable contract elements are referenced only as Reused Contract Context and need an explicit owning requirement
- You need one owning requirement per contract element

## Goal

Assign each contract element to a single owning requirement via `definedBy` (identifier target), place it in the same directory as that requirement, and keep other consumers as reused_contract_context when reuse is intentional.

This ownership is contract/document ownership, not governance ownership. Governance `owner` metadata is a separate routing/accountability label on capability and requirement elements and may name a person, role, team, department, subsystem group, or task owner.

## Mandatory Boundary Clarification (Human Checkpoint)

Confirm before bulk edits:

- Scope (entire model or selected submodels)
- Tie-break rule when multiple candidate owners exist
- Exceptions that should remain shared Reused Contract Context without becoming locally owned contracts

## Workflow

1. Enumerate all contract elements (`source`, `specification`, `constraint`, `behavior`, `state`, `input-output`) and their `define` targets.
2. Select a single owner requirement for each document by semantic/derivation fit
3. Convert owner link to `definedBy` using the contract element identifier, not a plain file path.
4. Keep all non-owner references as reused_contract_context
5. Verify no contract has multiple owner requirements and no contract lives outside its owner requirement directory.
6. Run `reqvire validate`, `reqvire lint`, `reqvire coverage --json`

## Report Expectations

- `validate` passes with no relation/type errors
- `collect` on owner requirement includes the contract through contract ownership
- `change-impact` includes consumers when owned design contract changes and is reused downstream

## How Not To Do It

- Do not blindly replace every reused_contract_context with `definedBy`
- Do not assign multiple owners to one design document
- Do not choose owners without checking requirement intent and derivation context
