---
allowed-tools: Read, Bash(npx:*)
description: Analyze the current Reqvire model structure, identify issues, coverage gaps, and provide improvement recommendations
model: claude-sonnet-4-5
---

# Analyze Reqvire Model

Perform comprehensive analysis of the current Reqvire model.

## Current Model State

- Validation status: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate 2>&1 | head -1`
- Total elements: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.global_counters.total_elements'`
- Verification coverage: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '.summary.leaf_requirements_coverage_percentage'`%
- Test satisfaction: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '.summary.test_verifications_satisfaction_percentage'`%

## Steps

1. **Run validation:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate --json --output /tmp/validation.json
   ```

2. **Generate model search:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --short --json --output /tmp/search.json
   ```

   Use `--short` to get model structure without full content.

3. **Check coverage:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json --output /tmp/coverage.json
   ```

4. **Run lint checks:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --json --output /tmp/lint.json
   ```

5. **Analyze submodel boundaries:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" submodels --json --output /tmp/submodels.json
   ```

6. **Analyze the results:**
   - Review validation errors and warnings
   - Identify unverified requirements from coverage report
   - Check for model quality issues from lint report
   - Review cross-submodel couplings and boundary hotspots
   - Calculate coverage percentages and statistics

7. **Provide recommendations:**
   - List specific issues found with file locations
   - Suggest improvements prioritized by impact
   - Recommend commands to fix issues (e.g., `reqvire lint --fix`)
   - Identify requirements needing verifications

## Output Format

Present findings in clear sections:
- **Validation Results**: Errors and warnings
- **Coverage Analysis**: Verification coverage statistics
- **Model Quality**: Lint findings (auto-fixable vs needs review)
- **Recommendations**: Prioritized action items

## Notes

- Use `/tmp` for JSON outputs
- Focus on actionable recommendations
