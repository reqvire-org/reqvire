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

## Move Folder

Move an entire folder subtree with all model files and local evidence files to a new location, automatically updating model identifiers, relations, contract bindings, concept references, and local path references that point into the moved subtree.

### Steps

1. **Understand the context:**
   - Identify the source folder to move
   - Determine the target folder location
   - Verify the source folder is inside the workspace and contains the intended model files
   - Check that the target folder does not already exist

2. **Preview the folder move operation:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-folder "<source-folder>" "<target-folder>" --dry-run
   ```

   This shows:
   - Which files will be modified
   - Which model identifiers will move to the new folder path
   - Which relations, contract bindings, concept references, and local file paths will be updated
   - Git-style diffs for affected files

3. **Apply the folder move:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-folder "<source-folder>" "<target-folder>"
   ```

   The mv-folder command automatically:
   - Moves every file under the source folder to the same relative path under the target folder
   - Updates element identifiers from `<source-folder>/...#<slug>` to `<target-folder>/...#<slug>`
   - Updates relations and contract bindings that reference moved elements
   - Updates concept references and local file paths that point into the moved folder
   - Preserves non-model files under the moved subtree
   - Removes the original folder after successful move

4. **Verify the changes:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
   ```

### Important Notes

- **Folder-level refactor**: Use `mv-folder` for package, subsystem, capability, or repository-folder reshapes where many files move together.
- **Target must be new**: Move into a new folder path. Use `mv-file --squash` for file-level consolidation into existing files.
- **No recursive target**: The target folder cannot be inside the source folder.
- **Reference safety**: Local references into the moved folder are rewritten; unrelated external paths are left unchanged.

### Examples

**Move a capability folder:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-folder "system-model/Drafts/Auth" "system-model/Identity/Auth"
```

**Preview a subsystem folder rename:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-folder "system-model/Platform" "system-model/CorePlatform" --dry-run
```

---

## When to Use mv vs mv-file vs mv-folder

- **`mv`**: Moving individual requirements or verifications between files
- **`mv-file`**: Moving entire files with all their elements; reorganizing specification file structure
- **`mv-file --squash`**: Merging multiple specification files into one
- **`mv-folder`**: Moving an entire folder subtree while preserving its internal file layout and updating references
