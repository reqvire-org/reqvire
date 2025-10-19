---
description: Lint and clean up the Reqvire model by fixing issues and identifying items needing review
---

# Lint Model

Lint the Reqvire model to fix quality issues and identify items needing manual review.

## Steps

1. **Apply auto-fixes immediately:**
   ```bash
   reqvire lint --fix
   ```

   This automatically fixes:
   - Syntax and formatting issues
   - Redundant verify relations (verification verifying both leaf and parent)

2. **Check for manual review items:**
   ```bash
   reqvire lint --json > /tmp/lint.json
   jq '.needs_review' /tmp/lint.json
   ```

3. **For manual review items:**

   Read affected specifications:
   ```bash
   reqvire summary --filter-id="<element-id>"
   ```

   Provide recommendations:
   - Show the potentially redundant relation
   - Explain why it may be redundant
   - Ask user if they want to remove it

4. **Validate after changes:**
   ```bash
   reqvire validate
   ```

## Lint Categories

### Auto-Fixable (always safe to apply)

- **Syntax**: Formatting, indentation, structure
- **Redundant verify**: Verification verifies both leaf and parent requirement

### Needs Review (requires judgment)

- **Hierarchical relations**: derivedFrom relations that may be redundant

## Best Practices

- Run `reqvire lint --fix` after adding features
- Review manual items carefully before removing
- Validate model after manual changes
- Use after `/add-feature` or `/add-verification` to clean up

## Notes

- Auto-fixes are always safe - just apply them
- Manual review items need human judgment and context
- Run lint regularly to maintain model quality
