---
description: Find and analyze redundant verify relations in the Reqvire model
---

# Find Redundant Verify Relations

Analyze the verification traces to find redundant verify relations in the model.

## Instructions

1. Run the traces command to generate JSON report:
   ```bash
   cargo run -- traces --json 2>/dev/null > /tmp/traces.json
   ```

2. Parse the JSON to find all verifications with redundant verify relations:
   ```bash
   jq -r '
   .files[] | .sections[] | .verifications[] |
   select(.redundant_relations | length > 0) |
   "## Verification: \(.name)\n" +
   "**File**: \(.file)\n" +
   "**Identifier**: `\(.identifier)`\n" +
   "**Directly Verified**: \(.directly_verified_count) requirements\n\n" +
   "**Redundant VERIFY Relations** (remove these from the verification):\n" +
   (.redundant_relations[] | "  * verify: \(.)\n") +
   "\n**Reason**: These requirements are ancestors of other verified requirements. Since verification automatically rolls up through derivedFrom relations, verifying the leaf requirements is sufficient.\n\n" +
   "---\n"
   ' /tmp/traces.json || echo "No redundant verify relations found in the model."
   ```

3. Present the results to the user showing:
   - Which verifications have redundant relations
   - Which specific verify relations can be removed
   - Explanation of why they're redundant

4. If no redundancies found, report: "No redundant verify relations found in the model."

## Requirements

- **jq** must be installed:
  - Mac: `brew install jq`
  - Linux: `sudo apt-get install jq` or `sudo yum install jq`
  - Check: `jq --version`

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
