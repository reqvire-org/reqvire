# Rename Element

Rename an existing model element while automatically updating all relation references throughout the model.

## Steps

1. **Understand the context:**
   - Identify the element to rename (current name)
   - Determine the new name
   - Verify element exists in the model

2. **Preview the rename operation:**
   ```text
   {"name": "reqvire.rename_element", "arguments": {"element_name": "<current-name>", "new_name": "<new-name>", "dry_run": true}}
   ```

   This shows:
   - Which files will be modified
   - What relations will be updated
   - The identifier change (old → new)

3. **Apply the rename:**
   ```text
   {"name": "reqvire.rename_element", "arguments": {"element_name": "<current-name>", "new_name": "<new-name>"}}
   ```

   The rename command automatically:
   - Updates the element's heading text in the markdown file
   - Updates the element identifier in the registry
   - Updates all forward relations (from the element to others)
   - Updates all backward relations (from other elements to this one)
   - Maintains model consistency

4. **Verify the changes:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
   ```

## Important Notes

- **Global uniqueness**: Element names are globally unique in Reqvire, so you only need the element name (not the full file path identifier)
- **Automatic relation updates**: All relations throughout the model are automatically updated
- **Identifier format**: The element identifier changes from `<file>#<old-name-slug>` to `<file>#<new-name-slug>`
- **Name slugification**: Element names are converted to slugs (lowercase, spaces → hyphens) for identifiers

## Error Cases

The rename operation will fail with a clear error if:
- The current element name does not exist
- The new name conflicts with an existing element

## Examples

**Rename a requirement:**
```text
{"name": "reqvire.rename_element", "arguments": {"element_name": "User Authentication", "new_name": "User Login Authentication"}}
```

**Preview before renaming:**
```text
{"name": "reqvire.rename_element", "arguments": {"element_name": "Data Storage", "new_name": "Persistent Data Storage", "dry_run": true}}
```

## When to Use Rename

Use rename when:
- A requirement or verification needs a better name
- Terminology changes in the project
- Consolidating or clarifying element names
- Refactoring the model structure

**Note**: Rename only changes the element name and heading. To move an element to a different file, use [Move](Move.md) instead.
