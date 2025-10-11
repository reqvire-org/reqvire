# Find Redundant Verify Relations

Analyze the verification traces to find redundant verify relations in the model.

## Instructions

1. Run the traces command to generate JSON report:
   ```bash
   cargo run -- traces --json > /tmp/traces.json
   ```

2. Parse the JSON to find all verifications with redundant relations:
   ```bash
   jq -r '
   .. |
   select(.redundant_relations? and (.redundant_relations | length) > 0) |
   "## Verification: \(.name)\n" +
   "**File**: \(.file)\n" +
   "**Identifier**: `\(.identifier)`\n\n" +
   "**Redundant Relations** (can be removed):\n" +
   (.redundant_relations[] | "- `\(.)`\n") +
   "\n**Reason**: These requirements are ancestors in the trace tree and are already covered by verifying their children.\n\n" +
   "---\n"
   ' /tmp/traces.json
   ```

3. Present the results to the user showing:
   - Which verifications have redundant relations
   - Which specific verify relations can be removed
   - Explanation of why they're redundant

4. If no redundancies found, report: "No redundant verify relations found in the model."

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
