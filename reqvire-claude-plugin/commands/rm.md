---
allowed-tools: Read, Bash(reqvire:*)
argument-hint: [element-name]
description: Remove an element from the model
model: default
---

# Remove Element

Remove an existing model element from the specifications.

## Current Model Context

- Total elements: !`reqvire search --json | jq -r '.global_counters.total_elements'`

## User Request

${1:+Element name: $1}
${1:-The user will provide element name to remove.}

## Steps

1. **Understand the context:**
   - Identify the element to remove (by name)
   - Verify element exists in the model
   - Check if other elements have relations to this element

2. **Preview the remove operation:**
   ```bash
   reqvire rm "<element-name>" --dry-run
   ```

   This shows:
   - Which file will be modified
   - The element that will be removed
   - Git-style diff showing the deletion

3. **Apply the removal:**
   ```bash
   reqvire rm "<element-name>"
   ```

   The rm command automatically:
   - Removes the element from its markdown file
   - Removes the element from the model registry
   - Deletes all relations from this element
   - **Note**: Does NOT automatically update relations FROM other elements TO this element (you may need to clean those up)

4. **Verify the changes:**
   ```bash
   reqvire validate
   ```

   **Important**: After removing an element, validation may show errors if other elements still reference the removed element. You'll need to manually update or remove those relations.

## Important Notes

- **Global uniqueness**: Element names are globally unique, so you only need the element name
- **Relations cleanup**: The removed element's outgoing relations are deleted, but incoming relations (from other elements) are NOT automatically removed
- **Validation warnings**: Removing elements that are referenced by others will cause validation errors
- **Breaking changes**: Removing requirements or verifications can break traceability chains

## Remove Options

- `--dry-run`: Preview changes without applying
- `--json`: Output results in JSON format

## Error Cases

The rm operation will fail with a clear error if:
- The element name does not exist
- Multiple elements have the same name (should not happen due to uniqueness constraint)

## Examples

**Remove a requirement:**
```bash
reqvire rm "Deprecated Feature"
```

**Preview before removing:**
```bash
reqvire rm "Old Requirement" --dry-run
```

**Get JSON output:**
```bash
reqvire rm "Obsolete Element" --json
```

## When to Use rm

Use rm when:
- Removing deprecated or obsolete requirements
- Cleaning up test elements
- Removing duplicate or incorrect entries
- Refactoring the model structure

**Warning**: Be careful when removing elements that are referenced by other elements, as this will break those relations and cause validation errors.

## Cleanup After Removal

After removing an element, you may need to:
1. Find elements that referenced the removed element (validation will show these)
2. Update or remove those relations manually
3. Re-run validation to ensure model consistency

## Related Commands

- **Move element**: `reqvire mv <element-name> <target-file>`
- **Rename element**: `reqvire rename <current-name> <new-name>`
- **Add element**: `reqvire add <file> < element.md`
- **Search elements**: `reqvire search --filter-name="<pattern>"`
