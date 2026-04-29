---
allowed-tools: Read, Bash(npx:*)
description: Lint and clean up the Reqvire model by fixing issues and identifying items needing review
model: claude-sonnet-4-5
---

# Lint Model

Lint the Reqvire model to fix quality issues and identify items needing manual review.

## Current Lint Status

- Auto-fixable: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --json 2>&1 | jq -r '"\(if .auto_fixable then (.auto_fixable | length) else 0 end) issues"'`
- Manual review: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --json 2>&1 | jq -r '"\(if .needs_manual_review then (.needs_manual_review | length) else 0 end) items"'`

## Steps

1. **Apply auto-fixes immediately:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --fix
   ```

   This automatically fixes:
   - Syntax and formatting issues
   - Redundant verify relations (verification verifying both leaf and parent)
   - Safe redundant hierarchical relations (single-chain derivedFrom paths)

2. **Check for manual review items:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --json --output /tmp/lint.json
   jq '.needs_manual_review' /tmp/lint.json
   ```

3. **For manual review items:**

   Read affected specifications:
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-id="<element-id>"
   ```

   Provide recommendations:
   - Show the potentially redundant relation
   - Explain why it may be redundant
   - Ask user if they want to remove it

4. **Validate after changes:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
   ```

## Lint Categories

### Auto-Fixable (always safe to apply)

- **Redundant verify relations**: Verification verifies both leaf and parent requirement
- **Safe redundant hierarchical relations**: Single-chain derivedFrom paths that can be safely removed

### Needs Review (requires judgment)

- **Multi-branch convergence**: Element reaches ancestor through multiple distinct paths
- **Complex hierarchical relations**: Multi-path derivedFrom relations requiring human judgment

## Best Practices

- Run `reqvire lint --fix` after adding features
- Review manual items carefully before removing
- Validate model after manual changes
- Use after `/reqvire:add-feature` or `/reqvire:add-verification` to clean up

## Notes

- Auto-fixes are always safe - just apply them
- Manual review items need human judgment and context
- Run lint regularly to maintain model quality
