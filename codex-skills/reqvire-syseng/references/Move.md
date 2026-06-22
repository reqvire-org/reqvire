# Move Elements and Files

Move individual model elements or entire specification files to different locations.

---

## Move Element

Move an existing model element to a different file or position within the model.

### Steps

1. **Understand the context:**
   - Identify the element to move (by name)
   - Determine the target location (file and/or index)
   - Verify element exists in the model

2. **Preview the move operation:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv "<element-name>" "<target-file>" --dry-run
   ```

   This shows:
   - Which files will be modified
   - Where the element will be moved
   - What relations will be updated
   - Git-style diffs for all affected files

3. **Apply the move:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv "<element-name>" "<target-file>"
   ```

   The mv command automatically:
   - Removes element from source file
   - Adds element to target file
   - Updates the element identifier to reflect new location
   - Updates all forward relations (from the element to others)
   - Updates all backward relations (from other elements to this one)
   - Maintains model consistency

4. **Verify the changes:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
   ```

### Important Notes

- **Global uniqueness**: Element names are globally unique, so you only need the element name
- **Path resolution**: File paths are resolved relative to current working directory
- **Automatic relation updates**: All relations throughout the model are automatically updated
- **Identifier update**: Element identifier changes from `<old-file>#<slug>` to `<new-file>#<slug>`

### Move Options

- `<element-name>`: Name of element to move (required)
- `<file>`: Target file path (required)
- `[index]`: Position within target file (0-based, default: append to end)
- `--dry-run`: Preview changes without applying
- `--json`: Output results in JSON format

### Examples

**Move element to different file:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv "User Authentication" "system-model/Security.md"
```

**Insert at specific position (index 0 = first element in file):**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv "High Priority Req" "system-model/Critical.md" 0
```

**Preview before moving:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv "Capability X" "NewFile.md" --dry-run
```

---

## Move File

Move an entire specification file with all its elements to a new location, automatically updating all relations that reference elements in the moved file.

### Steps

1. **Understand the context:**
   - Identify the source file to move
   - Determine the target file location
   - Verify source file exists in the model

2. **Preview the file move operation:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-file "<source-file>" "<target-file>" --dry-run
   ```

   This shows:
   - Which files will be modified
   - How many elements will be moved
   - What relations will be updated
   - Git-style diffs for all affected files

3. **Apply the file move:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-file "<source-file>" "<target-file>"
   ```

   The mv-file command automatically:
   - Moves all elements from source file to target file
   - Updates all element identifiers to reflect new file path
   - Updates all incoming relations (from other files) to reference new location
   - Preserves all element content, metadata, and outgoing relations
   - Deletes the source file after successful move
   - Creates the target file with all moved elements

4. **Verify the changes:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
   ```

### Important Notes

- **Automatic relation updates**: All relations pointing to moved elements are automatically updated throughout the model
- **Identifier updates**: Element identifiers change from `<source-file>#<slug>` to `<target-file>#<slug>`
- **Atomic operation**: Either all elements move successfully or none move

### Squash Mode

When the target file already exists, use the **`--squash`** flag to merge all elements from source into the target file:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-file "<source-file>" "<existing-target-file>" --squash
```

**Squash behavior:**
- All source elements are appended to target file
- Target file's existing elements remain unchanged
- Source file is deleted after successful move
- All relations are updated throughout the model

**When to use --squash:**
- Consolidating multiple specification files into one
- Merging temporary/experimental specs back into main file
- Reorganizing model structure by combining related files

### Examples

**Move file within specifications directory:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-file "system-model/OldFile.md" "system-model/NewFile.md"
```

**Move file to different directory:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-file "system-model/Auth.md" "security/Authentication.md"
```

**Preview squash before applying:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-file "old/File.md" "system-model/MainFile.md" --squash --dry-run
```

**Squash elements from multiple files:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-file "temp/Experiments.md" "system-model/Requirements.md" --squash
```

---

## When to Use mv vs mv-file

- **`mv`**: Moving individual requirements or verifications between files
- **`mv-file`**: Moving entire files with all their elements; reorganizing specification file structure
- **`mv-file --squash`**: Merging multiple specification files into one
