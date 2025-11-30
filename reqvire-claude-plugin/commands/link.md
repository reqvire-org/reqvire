---
allowed-tools: Read, Bash(reqvire:*)
argument-hint: <source> <relation-type> <target>
description: Link two elements with a relation
model: claude-sonnet-4-5-20250929
---

# Link Elements

Create a relation between two existing model elements.

## Current Model Context

- Total elements: !`reqvire search --json | jq -r '.global_counters.total_elements'`

## User Request

${1:+Source element: $1}
${2:+Relation type: $2}
${3:+Target element: $3}
${1:-The user will provide source element, relation type, and target element.}

## Steps

1. **Understand the context:**
   - Identify the source element (by name)
   - Identify the target element (by name)
   - Determine the relation type
   - Verify both elements exist in the model

2. **Preview the link operation:**
   ```bash
   reqvire link "<source-element>" "<relation-type>" "<target-element>" --dry-run
   ```

   This shows:
   - Which file will be modified
   - The relation that will be added
   - Git-style diff for the affected file

3. **Apply the link:**
   ```bash
   reqvire link "<source-element>" "<relation-type>" "<target-element>"
   ```

   The link command automatically:
   - Adds the relation to the source element's Relations section
   - Calculates correct relative path for cross-file links
   - Creates the Relations section if it doesn't exist
   - Maintains idempotency (no duplicate relations)

4. **Verify the changes:**
   ```bash
   reqvire validate
   ```

## Supported Relation Types

| Relation Type | Description | Usage |
|---------------|-------------|-------|
| `derivedFrom` | Source derives from target | Child to parent requirement |
| `derive` | Source has derived target | Parent to child requirement |
| `verifiedBy` | Source is verified by target | Requirement to verification |
| `verify` | Source verifies target | Verification to requirement |
| `satisfiedBy` | Source is satisfied by target | Requirement to implementation |
| `satisfy` | Source satisfies target | Implementation to requirement |
| `trace` | General traceability link | Any traceability relationship |

## Important Notes

- **Idempotent**: Running the same link command twice has no effect (no duplicates)
- **Element names**: Use the exact element name as it appears in the heading
- **Cross-file links**: Relative paths are calculated automatically
- **Inverse relations**: Reqvire auto-generates inverse relations (e.g., derive from derivedFrom)

## Link Options

- `<source>`: Name of source element (required)
- `<relation-type>`: Type of relation (required)
- `<target>`: Name of target element (required)
- `--dry-run`: Preview changes without applying
- `--json`: Output results in JSON format

## Error Cases

The link operation will fail with a clear error if:
- The source element does not exist
- The target element does not exist
- The relation type is invalid

## Examples

**Link requirement to parent:**
```bash
reqvire link "Feature Requirement" "derivedFrom" "User Story"
```

**Link parent to child:**
```bash
reqvire link "System Requirement" "derive" "Feature Requirement"
```

**Link requirement to verification:**
```bash
reqvire link "Authentication Requirement" "verifiedBy" "Auth Test Case"
```

**Preview before linking:**
```bash
reqvire link "Feature X" "trace" "Feature Y" --dry-run
```

## When to Use link

Use link when:
- Establishing traceability between requirements
- Connecting requirements to verifications
- Creating derivation hierarchies
- Adding trace relations for impact analysis

## Related Commands

- **Unlink elements**: `reqvire unlink <source> <relation-type> <target>`
- **Attach files**: `reqvire attach <path> <element>`
- **Search relations**: `reqvire search --have-relations="derivedFrom"`
