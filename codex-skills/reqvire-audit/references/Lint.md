# Lint and Model Quality

Lint the Reqvire model to fix quality issues and identify items needing manual review. Also covers finding and handling redundant verify relations.

## Steps

1. **Check current lint status:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --json 2>&1 | jq -r '{auto_fixable: (.auto_fixable | length), needs_review: (.needs_manual_review | length)}'
   ```

2. **Apply auto-fixes immediately:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --fix
   ```

   This automatically fixes:
   - Syntax and formatting issues
   - Redundant verify relations (verification verifying both leaf and parent)
   - Safe redundant hierarchical relations (single-chain derivedFrom paths)

3. **Check for manual review items:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --json --output /tmp/lint.json
   jq '.needs_manual_review' /tmp/lint.json
   ```

4. **For manual review items:**

   Read affected specifications:
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-id="<element-id>"
   ```

   Provide recommendations:
   - Show the potentially redundant relation
   - Explain why it may be redundant
   - Ask user if they want to remove it

5. **Validate after changes:**
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

## Finding Redundant Verify Relations

To specifically inspect redundant verify relations before auto-fixing:

1. Run lint and save JSON:
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --json --output /tmp/lint.json
   ```

2. Parse redundant verify relations:
   ```bash
   jq -r '
   .auto_fixable[] |
   select(.type == "redundant_verify_relations") |
   "## Verification: \(.verification.name)\n" +
   "**File**: \(.verification.file)\n" +
   "**Identifier**: `\(.verification.identifier)`\n\n" +
   "**Redundant VERIFY Relations** (will be auto-removed with lint --fix):\n" +
   (.redundant_relations[] | "  * verify: \(.target)\n") +
   "\n**Reason**: \(.rationale)\n\n" +
   "---\n"
   ' /tmp/lint.json || echo "No redundant verify relations found."
   ```

3. Present findings showing:
   - Which verifications have redundant relations
   - Which specific verify relations can be removed
   - Explanation of why they're redundant

4. If no redundancies found, report: "No redundant verify relations found in the model."

### Background: When Is a Verify Relation Redundant?

A verify relation is redundant when:
- A verification directly verifies both a requirement and an ancestor requirement already covered through the requirement trace path
- Since verification traces roll up automatically, verifying the most precise element is usually sufficient
- The direct verification of the ancestor adds noise to the model unless the parent requirement has distinct verification evidence

Example:
```
Verification "Password Test" verifies:
  - "Password Strength" (leaf requirement)
  - "Password Authentication" (parent of Password Strength)

→ The verify relation to "Password Authentication" is REDUNDANT
```

## Best Practices

- Run `reqvire lint --fix` after adding capabilities or verifications to clean up
- Review manual items carefully before removing
- Validate model after manual changes
- Auto-fixes are always safe — just apply them
- Manual review items need human judgment and context
- Run lint regularly to maintain model quality
