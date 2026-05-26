---
allowed-tools: Read, Bash(npx:*)
description: Find and analyze redundant verify relations in the Reqvire model
model: claude-sonnet-4-5
---

# Find Redundant Verify Relations

Analyze the verification traces to find redundant verify relations in the model.

## Current Status

- Auto-fixable issues: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --json 2>&1 | jq -r '"\(if .auto_fixable then (.auto_fixable | length) else 0 end) (including redundant verifications)"'`

## Instructions

1. Run the lint command to find redundancies:
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --json --output /tmp/lint.json
   ```

2. Parse the JSON to find redundant verify relations:
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

3. Present the results to the user showing:
   - Which verifications have redundant relations
   - Which specific verify relations can be removed
   - Explanation of why they're redundant

4. If no redundancies found, report: "No redundant verify relations found in the model."

5. **Auto-fix option:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --fix
   ```

## Background

A verify relation is redundant when:
- A verification directly verifies both an element and an ancestor already covered through the capability or requirement trace path
- Since verification traces roll up automatically, verifying the most precise element is usually sufficient
- The direct verification of the ancestor adds noise to the model unless the broader capability or parent requirement has distinct verification evidence

Example:
```
Verification "Password Test" verifies:
  - "Password Strength" (leaf requirement)
  - "Password Authentication" (parent of Password Strength)

→ The verify relation to "Password Authentication" is REDUNDANT
```

The system automatically detects this by building trace trees and checking if any ancestor capabilities or requirements are also directly verified.

## Notes

- Use `reqvire lint --fix` to automatically remove redundant relations
- Redundant verify relations are always safe to remove
- Run `reqvire validate` after fixing to confirm model integrity
