---
allowed-tools: Read, Bash(npx:*)
description: Analyze coverage (verification and implementation) and identify gaps
model: claude-sonnet-4-5
---

# Analyze Coverage

Analyze verification and implementation coverage to identify gaps.

## Current Coverage

- Total requirements: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '.summary.total_leaf_requirements'`
- Verified: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '.summary.verified_leaf_requirements'`
- Coverage: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '.summary.leaf_requirements_coverage_percentage'`%
- Unverified: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '.summary.unverified_leaf_requirements'`
- Implementation scope: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '.summary.total_requirements_in_scope'`
- Implementation covered: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '.summary.covered_requirements'`
- Implementation uncovered: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '.summary.uncovered_requirements'`
- Implementation coverage: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '.summary.implementation_coverage_percentage'`%

## Steps

1. **Generate coverage report:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json --output /tmp/coverage.json
   ```

2. **Analyze coverage statistics:**
   - Extract total leaf requirements count (verification scope)
   - Calculate verification percentage
   - Identify unverified requirements count
   - Extract implementation scope count (`requirement` elements only; excludes `user-requirement`)

3. **Identify unverified leaf requirements:**

   From coverage JSON:
   ```bash
   jq '.unverified_leaf_requirements' /tmp/coverage.json
   ```

   Focus on leaf requirements (requirements without derived children).

4. **Check if parent requirements need verification:**

   For each unverified requirement:
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" traces --filter-name="<requirement-name>"
   ```

   Determine:
   - Is this a leaf requirement? (needs verification)
   - Is this a parent requirement? (should inherit from children)

5. **Present findings:**

   **Coverage Summary:**
   - Total requirements: X
   - Verified requirements: Y
   - Coverage percentage: Z%

   **Unverified Leaf Requirements:**
   - [Requirement Name](file.md#requirement-name) - needs verification
   - [Another Requirement](file.md#another) - needs verification

   **Parent Requirements (OK - coverage rolls up):**
   - [Parent Requirement](file.md#parent) - covered by children

6. **Provide recommendations:**
   - List leaf requirements needing verifications
   - List implementation-uncovered `requirement` elements for `satisfiedBy` planning
   - Suggest using `/add-verification` for each
   - Explain which parents are OK (inherit from children)

## Notes

- Focus on leaf requirements for verification
- Parent requirements inherit coverage from children
- Implementation coverage does not include `user-requirement`
- Use `/add-verification` to create missing verifications
- Run `reqvire coverage` after adding verifications to confirm improvement
