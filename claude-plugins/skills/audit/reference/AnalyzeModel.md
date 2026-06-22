# Analyze Reqvire Model

Perform comprehensive analysis of the current Reqvire model to identify issues, coverage gaps, and provide improvement recommendations.

## Steps

1. **Check current model state:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate 2>&1 | head -1
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.global_counters'
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '.summary'
   ```

2. **Run validation:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate --json --output /tmp/validation.json
   ```

3. **Generate model search:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --short --json --output /tmp/search.json
   ```

   Use `--short` to get model structure without full content.

4. **Check coverage:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json --output /tmp/coverage.json
   ```

5. **Run lint checks:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --json --output /tmp/lint.json
   ```

6. **Analyze submodel boundaries:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" submodels --json --output /tmp/submodels.json
   ```

7. **Analyze the results:**
   - Review validation errors and warnings
   - Identify unverified requirements from coverage report
   - Check for model quality issues from lint report
   - Review cross-submodel couplings and boundary hotspots
   - Calculate coverage percentages and statistics

8. **Provide recommendations:**
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
- For detailed coverage analysis, see [AnalyzeCoverage](AnalyzeCoverage.md)
- For lint details, see [Lint](Lint.md)
- For change impact analysis, see [ChangeImpact](ChangeImpact.md)
