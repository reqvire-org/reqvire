---
allowed-tools: Read, Bash(reqvire:*)
description: Find and analyze redundant verify relations in the Reqvire model
model: claude-sonnet-4-5-20250929
---

# Find Redundant Verify Relations

Analyze the verification traces to find redundant verify relations in the model.

## Current Status

- Lint check: !`reqvire lint --json 2>&1 | jq -r 'if .auto_fixable then (.auto_fixable | length) else 0 end' | xargs -I{} echo "{} auto-fixable issues (including redundant verifications)"`

## Instructions

1. Run the lint command to find redundancies:
   ```bash
   reqvire lint --json > /tmp/lint.json
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
   reqvire lint --fix
   ```

## Background

A verify relation is redundant when:
- A verification directly verifies both a child requirement AND its parent
- Since verification traces roll up automatically, verifying the child is sufficient
- The direct verification of the parent adds noise to the model

Example:
```
Verification "Password Test" verifies:
  - "Password Strength" (leaf requirement)
  - "Password Authentication" (parent of Password Strength)

→ The verify relation to "Password Authentication" is REDUNDANT
```

The system automatically detects this by building trace trees and checking if any ancestor requirements are also directly verified.

## Notes

- Use `reqvire lint --fix` to automatically remove redundant relations
- Redundant verify relations are always safe to remove
- Run `reqvire validate` after fixing to confirm model integrity
