---
allowed-tools: Read, Bash(reqvire:*)
argument-hint: <source> <relation-type> <target>
description: Remove a relation between two elements
model: claude-sonnet-4-5-20250929
---

# Unlink Elements

Remove an existing relation between two model elements.

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
   - Determine the relation type to remove
   - Verify the relation exists

2. **Preview the unlink operation:**
   ```bash
   reqvire unlink "<source-element>" "<relation-type>" "<target-element>" --dry-run
   ```

   This shows:
   - Which file will be modified
   - The relation that will be removed
   - Git-style diff for the affected file

3. **Apply the unlink:**
   ```bash
   reqvire unlink "<source-element>" "<relation-type>" "<target-element>"
   ```

   The unlink command automatically:
   - Removes the relation from the source element's Relations section
   - Cleans up the Relations section if it becomes empty
   - Maintains model consistency

4. **Verify the changes:**
   ```bash
   reqvire validate
   ```

## Supported Relation Types

| Relation Type | Description |
|---------------|-------------|
| `derivedFrom` | Source derives from target |
| `derive` | Source has derived target |
| `verifiedBy` | Source is verified by target |
| `verify` | Source verifies target |
| `satisfiedBy` | Source is satisfied by target |
| `satisfy` | Source satisfies target |
| `trace` | General traceability link |

## Important Notes

- **Explicit relations only**: Only removes user-created relations (not auto-generated inverse relations)
- **Element names**: Use the exact element name as it appears in the heading
- **Cleanup**: Empty Relations sections are removed automatically
- **Validation**: Consider model validity after unlinking (orphaned elements may cause validation errors)

## Unlink Options

- `<source>`: Name of source element (required)
- `<relation-type>`: Type of relation (required)
- `<target>`: Name of target element (required)
- `--dry-run`: Preview changes without applying
- `--json`: Output results in JSON format

## Error Cases

The unlink operation will fail with a clear error if:
- The source element does not exist
- The target element does not exist
- The specified relation does not exist
- The relation type is invalid

## Examples

**Remove derivation link:**
```bash
reqvire unlink "Feature Requirement" "derivedFrom" "User Story"
```

**Remove verification link:**
```bash
reqvire unlink "Requirement" "verifiedBy" "Test Case"
```

**Preview before unlinking:**
```bash
reqvire unlink "Feature X" "trace" "Feature Y" --dry-run
```

## When to Use unlink

Use unlink when:
- Removing incorrect traceability links
- Refactoring requirement hierarchies
- Disconnecting deprecated verifications
- Cleaning up obsolete trace relations

## Related Commands

- **Link elements**: `reqvire link <source> <relation-type> <target>`
- **Detach files**: `reqvire detach <element> <path>`
- **Search relations**: `reqvire search --have-relations="derivedFrom"`
