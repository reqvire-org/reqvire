---
allowed-tools: Read, Bash(npx:*)
argument-hint: [source-file] [target-file]
description: Move entire specification file with all its elements to a new location
model: claude-sonnet-4-5
---

# Move File

Move an entire specification file with all its elements to a new location, automatically updating all relations that reference elements in the moved file.

## Current Model Context

- Total elements: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.global_counters.total_elements'`

## User Request

${1:+Source file: $1}
${2:+Target file: $2}
${1:-The user will provide source and target file paths.}

## Steps

1. **Understand the context:**
   - Identify the source file to move
   - Determine the target file location
   - Verify source file exists in the model

2. **Preview the move operation:**
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

## Important Notes

- **Path resolution**: File paths are resolved relative to current working directory
- **Automatic relation updates**: All relations pointing to moved elements are automatically updated throughout the model
- **Identifier updates**: Element identifiers change from `<source-file>#<slug>` to `<target-file>#<slug>`
- **Subdirectory support**: Works correctly when executed from subdirectories within git repository
- **Atomic operation**: Either all elements move successfully or none move (on validation failure)

## Squash Mode

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

## Error Cases

The mv-file operation will fail with a clear error if:
- The source file does not exist or contains no elements
- The target file already exists in the model (unless --squash flag is provided)
- The operation would result in invalid model state

## Examples

**Move file within specifications directory:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-file "requirements/OldFile.md" "requirements/NewFile.md"
```

**Move file to different directory:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-file "requirements/Auth.md" "security/Authentication.md"
```

**Preview before moving:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-file "specs/File.md" "requirements/File.md" --dry-run
```

**Get JSON output with element mappings:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-file "old/path.md" "new/path.md" --json
```

**Works from subdirectories (paths relative to current directory):**
```bash
cd submodule/
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-file "specs/File.md" "requirements/File.md"
```

**Squash elements from multiple files:**
```bash
# Merge experimental specs into main requirements
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-file "temp/Experiments.md" "requirements/Requirements.md" --squash
```

**Preview squash before applying:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-file "old/File.md" "requirements/MainFile.md" --squash --dry-run
```

## When to Use mv-file

Use mv-file when:
- Reorganizing specification file structure
- Moving all requirements for a feature to a different location
- Consolidating or splitting specification files
- Refactoring the model directory structure

Use mv-file with --squash when:
- Merging multiple specification files into one
- Consolidating temporary or experimental requirements back into main specs
- Simplifying model structure by reducing file count

**Note**: mv-file moves entire files with all their elements. To move individual elements between files, use `reqvire mv` instead.

## Related Commands

- **Move element**: `reqvire mv <element-name> <target-file>`
- **Rename element**: `reqvire rename <current-name> <new-name>`
- **Remove element**: `reqvire rm <element-name>`
- **Add element**: `reqvire add <file> < element.md`
