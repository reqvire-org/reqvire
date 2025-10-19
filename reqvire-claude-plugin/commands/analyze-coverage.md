---
allowed-tools: Read, Bash(reqvire:*)
description: Analyze verification coverage and identify unverified requirements
model: claude-sonnet-4-5-20250929
---

# Analyze Verification Coverage

Analyze verification coverage to identify gaps and unverified requirements.

## Current Coverage

- Total requirements: !`reqvire coverage --json | jq -r '.summary.total_leaf_requirements'`
- Verified: !`reqvire coverage --json | jq -r '.summary.verified_leaf_requirements'`
- Coverage: !`reqvire coverage --json | jq -r '.summary.leaf_requirements_coverage_percentage'`%
- Unverified: !`reqvire coverage --json | jq -r '.summary.unverified_leaf_requirements'`

## Steps

1. **Generate coverage report:**
   ```bash
   reqvire coverage
   reqvire coverage --json > /tmp/coverage.json
   ```

2. **Analyze coverage statistics:**
   - Extract total requirements count
   - Calculate verification percentage
   - Identify unverified requirements count

3. **Identify unverified leaf requirements:**

   From coverage JSON:
   ```bash
   jq '.unverified_leaf_requirements' /tmp/coverage.json
   ```

   Focus on leaf requirements (requirements without derived children).

4. **Check if parent requirements need verification:**

   For each unverified requirement:
   ```bash
   reqvire traces --filter-name="<requirement-name>"
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
   - Suggest using `/add-verification` for each
   - Explain which parents are OK (inherit from children)

## Notes

- Focus on leaf requirements for verification
- Parent requirements inherit coverage from children
- Use `/add-verification` to create missing verifications
- Run `reqvire coverage` after adding verifications to confirm improvement
