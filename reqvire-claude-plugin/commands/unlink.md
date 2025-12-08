---
allowed-tools: Read, Bash(reqvire:*)
argument-hint: <source> <target>
description: Remove a relation or attachment (auto-detects type)
model: sonnet
---

# Unlink Elements

Remove an existing relation or attachment between elements. The command auto-detects whether the target is a relation or attachment.

## Current Model Context

- Total elements: !`reqvire search --json | jq -r '.global_counters.total_elements'`

## User Request

${1:+Source element: $1}
${2:+Target: $2}
${1:-The user will provide source element and target.}

## Steps

1. **Understand the context:**
   - Identify the source element (by name)
   - Identify the target (element name or file path)
   - The command will auto-detect if it's a relation or attachment

2. **Preview the unlink operation:**
   ```bash
   reqvire unlink "<source-element>" "<target>" --dry-run
   ```

   This shows:
   - Which file will be modified
   - The relation/attachment that will be removed
   - Git-style diff for the affected file

3. **Apply the unlink:**
   ```bash
   reqvire unlink "<source-element>" "<target>"
   ```

   The unlink command automatically:
   - Searches relations first, then attachments
   - Removes the relation/attachment from the source element
   - Cleans up empty sections automatically
   - Maintains model consistency

4. **Verify the changes:**
   ```bash
   reqvire validate
   ```

## Auto-Detection Behavior

The unlink command auto-detects the type:
1. **First**: Searches for a relation from source to target element
2. **Then**: If no relation found, searches for an attachment matching the target
3. Only one relation per source-target pair is allowed, so no ambiguity

## Important Notes

- **Auto-detection**: No need to specify relation type - the command finds it automatically
- **Explicit relations only**: Only removes user-created relations (not auto-generated inverse relations)
- **Element names**: Use the exact element name as it appears in the heading
- **Cleanup**: Empty Relations/Attachments sections are removed automatically
- **Validation**: Consider model validity after unlinking (orphaned elements may cause validation errors)

## Unlink Options

- `<source>`: Name of source element (required)
- `<target>`: Element name or file path (required)
- `--dry-run`: Preview changes without applying

## Error Cases

The unlink operation will fail with a clear error if:
- The source element does not exist
- No relation or attachment found from source to target

## Examples

**Remove a relation (auto-detected):**
```bash
reqvire unlink "Feature Requirement" "User Story"
```

**Remove an attachment file:**
```bash
reqvire unlink "System Requirement" "docs/SLA.pdf"
```

**Remove an attached element:**
```bash
reqvire unlink "System Requirement" "Performance Constraint"
```

**Preview before unlinking:**
```bash
reqvire unlink "Feature X" "Feature Y" --dry-run
```

## When to Use unlink

Use unlink when:
- Removing incorrect traceability links
- Refactoring requirement hierarchies
- Disconnecting deprecated verifications
- Cleaning up obsolete trace relations
- Detaching documents or files
- Removing attached refinement elements

## Related Commands

- **Link elements**: `reqvire link <source> <relation-type-or-attaching> <target>`
- **Search relations**: `reqvire search --have-relations="derivedFrom"`
- **Search attachments**: `reqvire search --has-attachments`
