# Move Elements and Files

Move individual model elements or entire specification files to different locations.

---

## Workspace and Git Worktree Boundaries

- Path arguments are relative to the effective workspace root, not any Git repository root.
- In a parent workspace containing multiple eligible Git worktrees, `mv`, `mv-file`, `mv-folder`, and `mv-asset` may move model files or assets from one eligible worktree path to another using workspace-root-relative paths.
- Targets under workspace folders outside all eligible Git worktrees must be rejected; do not use Reqvire move or add mutations to create files in non-Git workspace folders.
- `--workspace` only selects the effective workspace root before command execution; cross-repo CRUD behavior belongs to workspace-scope mutation semantics, not to the workspace flag itself.

---

## Move Element

Move an existing model element to a different file or position within the model.

### Steps

1. **Understand the context:**
   - Identify the element to move (by name)
   - Determine the target location (file and/or index)
   - Verify element exists in the model

2. **Preview the move operation:**
   ```text
   {"name": "reqvire.move_element", "arguments": {"element_name": "<element-name>", "file": "<target-file>", "dry_run": true}}
   ```

   This shows:
   - Which files will be modified
   - Where the element will be moved
   - What relations will be updated
   - Git-style diffs for all affected files

3. **Apply the move:**
   ```text
   {"name": "reqvire.move_element", "arguments": {"element_name": "<element-name>", "file": "<target-file>"}}
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
- **Path resolution**: File paths are resolved relative to the effective workspace root
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
```text
{"name": "reqvire.move_element", "arguments": {"element_name": "User Authentication", "file": "system-model/Security.md"}}
```

**Insert at specific position (index 0 = first element in file):**
```text
{"name": "reqvire.move_element", "arguments": {"element_name": "High Priority Req", "file": "system-model/Critical.md"}}
```

**Preview before moving:**
```text
{"name": "reqvire.move_element", "arguments": {"element_name": "Capability X", "file": "NewFile.md", "dry_run": true}}
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
   ```text
   {"name": "reqvire.move_file", "arguments": {"source_file": "<source-file>", "target_file": "<target-file>", "dry_run": true}}
   ```

   This shows:
   - Which files will be modified
   - How many elements will be moved
   - What relations will be updated
   - Git-style diffs for all affected files

3. **Apply the file move:**
   ```text
   {"name": "reqvire.move_file", "arguments": {"source_file": "<source-file>", "target_file": "<target-file>"}}
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

```text
{"name": "reqvire.move_file", "arguments": {"source_file": "<source-file>", "target_file": "<existing-target-file>", "squash": true}}
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
```text
{"name": "reqvire.move_file", "arguments": {"source_file": "system-model/OldFile.md", "target_file": "system-model/NewFile.md"}}
```

**Move file to different directory:**
```text
{"name": "reqvire.move_file", "arguments": {"source_file": "system-model/Auth.md", "target_file": "security/Authentication.md"}}
```

**Preview squash before applying:**
```text
{"name": "reqvire.move_file", "arguments": {"source_file": "old/File.md", "target_file": "system-model/MainFile.md", "dry_run": true, "squash": true}}
```

**Squash elements from multiple files:**
```text
{"name": "reqvire.move_file", "arguments": {"source_file": "temp/Experiments.md", "target_file": "system-model/Requirements.md", "squash": true}}
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
   ```text
   {"name": "reqvire.move_folder", "arguments": {"source_folder": "<source-folder>", "target_folder": "<target-folder>", "dry_run": true}}
   ```

   This shows:
   - Which files will be modified
   - Which model identifiers will move to the new folder path
   - Which relations, contract bindings, concept references, and local file paths will be updated
   - Git-style diffs for affected files

3. **Apply the folder move:**
   ```text
   {"name": "reqvire.move_folder", "arguments": {"source_folder": "<source-folder>", "target_folder": "<target-folder>"}}
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
```text
{"name": "reqvire.move_folder", "arguments": {"source_folder": "system-model/Drafts/Auth", "target_folder": "system-model/Identity/Auth"}}
```

**Preview a subsystem folder rename:**
```text
{"name": "reqvire.move_folder", "arguments": {"source_folder": "system-model/Platform", "target_folder": "system-model/CorePlatform", "dry_run": true}}
```

---

## When to Use mv vs mv-file vs mv-folder

- **`mv`**: Moving individual requirements or verifications between files
- **`mv-file`**: Moving entire files with all their elements; reorganizing specification file structure
- **`mv-file --squash`**: Merging multiple specification files into one
- **`mv-folder`**: Moving an entire folder subtree while preserving its internal file layout and updating references
