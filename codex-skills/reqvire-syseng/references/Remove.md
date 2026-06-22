# Remove Element

Remove an existing model element from the specifications.

## Steps

1. **Understand the context:**
   - Identify the element to remove (by name)
   - Verify element exists in the model
   - Check if other elements have relations to this element

2. **Preview the remove operation:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" rm "<element-name>" --dry-run
   ```

   This shows:
   - Which file will be modified
   - The element that will be removed
   - Git-style diff showing the deletion

3. **Apply the removal:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" rm "<element-name>"
   ```

   The rm command automatically:
   - Removes the element from its markdown file
   - Removes the element from the model registry
   - Deletes all relations from this element
   - **Note**: Does NOT automatically update relations FROM other elements TO this element

4. **Verify the changes:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
   ```

   **Important**: After removing an element, validation may show errors if other elements still reference the removed element. You'll need to manually update or remove those relations.

## Important Notes

- **Global uniqueness**: Element names are globally unique, so you only need the element name
- **Relations cleanup**: The removed element's outgoing relations are deleted, but incoming relations (from other elements) are NOT automatically removed
- **Validation warnings**: Removing elements that are referenced by others will cause validation errors
- **Breaking changes**: Removing requirements or verifications can break traceability chains

## Options

- `--dry-run`: Preview changes without applying
- `--json`: Output results in JSON format

## Examples

**Remove a requirement:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" rm "Deprecated Capability"
```

**Preview before removing:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" rm "Old Requirement" --dry-run
```

## Cleanup After Removal

After removing an element, you may need to:
1. Find elements that referenced the removed element (validation will show these)
2. Update or remove those relations manually using [Link](Link.md) (unlink operation)
3. Re-run validation to ensure model consistency

## When to Use rm

Use rm when:
- Removing deprecated or obsolete requirements
- Cleaning up test elements
- Removing duplicate or incorrect entries
- Refactoring the model structure

**Warning**: Be careful when removing elements that are referenced by other elements, as this will break those relations and cause validation errors.
