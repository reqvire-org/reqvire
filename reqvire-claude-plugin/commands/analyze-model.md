---
description: Analyze the current Reqvire model structure, identify issues, coverage gaps, and provide improvement recommendations
---

# Analyze Reqvire Model

Perform comprehensive analysis of the current Reqvire model.

## Steps

1. **Remove diagrams to save tokens:**
   ```bash
   reqvire remove-diagrams
   ```

2. **Run validation:**
   ```bash
   reqvire validate --json > /tmp/validation.json
   ```

3. **Generate model summary:**
   ```bash
   reqvire summary --json > /tmp/summary.json
   ```

4. **Check coverage:**
   ```bash
   reqvire coverage --json > /tmp/coverage.json
   ```

5. **Run lint checks:**
   ```bash
   reqvire lint --json > /tmp/lint.json
   ```

6. **Analyze the results:**
   - Review validation errors and warnings
   - Identify unverified requirements from coverage report
   - Check for model quality issues from lint report
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

- Always run `remove-diagrams` first to save context tokens
- Use `/tmp` for JSON outputs
- Focus on actionable recommendations
