---
allowed-tools: Read, Bash(reqvire:*)
description: Analyze the current Reqvire model structure, identify issues, coverage gaps, and provide improvement recommendations
model: claude-sonnet-4-5-20250929
---

# Analyze Reqvire Model

Perform comprehensive analysis of the current Reqvire model.

## Current Model State

- Validation status: !`reqvire validate 2>&1 | head -1`
- Total elements: !`reqvire search --json | jq -r '.global_counters.total_elements'`
- Verification coverage: !`reqvire coverage --json | jq -r '.summary.leaf_requirements_coverage_percentage'`%
- Test satisfaction: !`reqvire coverage --json | jq -r '.summary.test_verifications_satisfaction_percentage'`%

## Steps

1. **Run validation:**
   ```bash
   reqvire validate --json > /tmp/validation.json
   ```

2. **Generate model search:**
   ```bash
   reqvire search --short --json > /tmp/search.json
   ```

   Use `--short` to get model structure without full content.

3. **Check coverage:**
   ```bash
   reqvire coverage --json > /tmp/coverage.json
   ```

4. **Run lint checks:**
   ```bash
   reqvire lint --json > /tmp/lint.json
   ```

5. **Analyze the results:**
   - Review validation errors and warnings
   - Identify unverified requirements from coverage report
   - Check for model quality issues from lint report
   - Calculate coverage percentages and statistics

6. **Provide recommendations:**
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
