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
   - Extract implementation scope count (`requirement` elements only; excludes `capability`)

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
- Implementation coverage does not include `capability`; capability coverage rolls up from requirements that specify capabilities
- Use the `reqvire:syseng` skill to create missing verifications
- Run `reqvire coverage` after adding verifications to confirm improvement
