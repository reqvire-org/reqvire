# Analyze Coverage

Analyze verification and implementation coverage to identify gaps.

## Steps

1. **Check current coverage state:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '.summary'
   ```

2. **Generate coverage report:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json --output /tmp/coverage.json
   ```

3. **Analyze coverage statistics:**
   - Extract total leaf requirements count (verification scope)
   - Calculate verification percentage
   - Identify unverified requirements count
   - Extract implementation terminal counts and recursive requirement completeness separately; capabilities receive roll-up rather than direct satisfaction
   - Compare `direct_satisfied`, requirement roll-up, contract-consumer roll-up, and combined roll-up sources

4. **Identify unverified leaf requirements:**

   From coverage JSON:
   ```bash
   jq '.unverified_leaf_requirements' /tmp/coverage.json
   ```

   Focus on leaf requirements (requirements without derived children).

5. **Check if parent requirements need verification:**

   For each unverified requirement:
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" traces --filter-name="<requirement-name>"
   ```

   Determine:
   - Is this a leaf requirement? (needs verification)
   - Is this a parent requirement? (should inherit from children)

6. **Present findings:**

   **Coverage Summary:**
   - Total requirements: X
   - Verified requirements: Y
   - Coverage percentage: Z%

   **Unverified Leaf Requirements:**
   - [Requirement Name](file.md#requirement-name) - needs verification
   - [Another Requirement](file.md#another) - needs verification

   **Parent Requirements (OK - coverage rolls up):**
   - [Parent Requirement](file.md#parent) - covered by children

7. **Provide recommendations:**
   - List leaf requirements needing verifications
   - List implementation-uncovered `requirement` elements for `satisfiedBy` planning
   - Suggest using the `reqvire:syseng` skill's [AddVerification](../../syseng/reference/AddVerification.md) workflow for each
   - Explain which parents are OK (inherit from children)

## Notes

- Focus on leaf requirements for verification
- Parent requirements inherit coverage from children
- A terminal requirement has neither requirement children nor required binding consumers and needs direct `satisfiedBy` evidence. A parent or contract owner requires all immediate children and binding consumers to be covered recursively; direct parent evidence does not override gaps.
- Contract References add no fulfillment dependency, terminal unit, blocker, or implementation evidence. A reporting or documentation consumer must not make the producer implemented.
- Inspect binding meaning before treating consumers as implementation contributors. Convert context-only dependencies to references while retaining change-impact visibility; do not add artifact links just to preserve coverage figures.
- Capabilities receive verification and implementation coverage through requirement/capability roll-up. Contracts themselves are not coverage units.
- Use the `reqvire:syseng` skill to create missing verifications
- Run `reqvire coverage` after adding verifications to confirm improvement
