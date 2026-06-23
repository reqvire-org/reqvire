# Specification Language Cleanup

Use this workflow when auditing specification/constraint/behavior language that uses normative phrasing.

## Do It When

- A `specification` or `constraint` element contains user-facing `shall` statements that belong in a parent requirement.
- A requirement intent is mixed with implementation mechanism details.
- You are finishing a model-refactoring pass and need consistent language boundaries before updating tests or docs.

## Goal

Keep requirements (and their `#### Details`) as the place for intent and intent-level constraints.
Keep contracts focused on mechanism, constraints, and behaviors in non-normative language where possible.
Ensure every cleanup decision is tied to a parent or sibling requirement through ownership/contract_bindings traceability.

## Procedure

### 1) Build a candidate list

Use quick scans to find explicit normative phrasing and scope candidates:

```bash
reqvire search --filter-content="(?i)(\bshall\b|\bmust\b|\bmust not\b)" --filter-type=specification --short
reqvire search --filter-content="(?i)(\bshall\b|\bmust\b|\bmust not\b)" --filter-type=constraint --short
reqvire search --filter-content="(?i)(\bshall\b|\bmust\b|\bmust not\b)" --filter-type=behavior --short
```

### 2) Decide action per element

For each candidate contract:

- `move`: normative requirement-intent sentence belongs to the owning requirement, move it to that requirement `#### Details`.
- `rephrase`: keep in contract but rewrite without `shall`-style statements.

Before each batch, confirm expected ownership boundaries and keep one-shot changes small.

### 3) Apply and verify ownership links

- If you move ownership text out of a contract, ensure the parent requirement still has clear traceability via `definedBy`/`Contract Bindings`.
- Do not leave duplicated normative language after move.
- Keep contracts useful even after cleanup (no placeholder-only content).

### 4) Validate behavior boundaries

Run:

```bash
reqvire validate
reqvire lint
reqvire coverage --json
```

Expected clean state:

- requirements retain clear EARS intent and traceability
- contracts are mechanism-focused and non-normative
- no placeholder/no-op cleanup artifacts remain

## Example "good" result

- Parent requirement carries intent in body/`#### Details`.
- Linked contract explains interfaces, constraints, and implementation details without `shall`.
- `reqvire collect` for the parent still includes needed contract context through normal traceability paths.

## How Not To Do It

- Do not bulk rewrite whole files without checking parent ownership.
- Do not move normative detail to a different requirement just to clear search matches.
- Do not delete technical content without migrating equivalent context to a related element.
