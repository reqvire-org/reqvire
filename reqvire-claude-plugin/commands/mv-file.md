---
allowed-tools: Read, Bash(reqvire:*)
argument-hint: [source-file] [target-file]
description: Move entire specification file with all its elements to a new location
model: claude-sonnet-4-5-20250929
---

# Move File

Move an entire specification file with all its elements to a new location, automatically updating all relations that reference elements in the moved file.

## Current Model Context

- Total elements: !`reqvire search --json | jq -r '.global_counters.total_elements'`

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
   reqvire mv-file "<source-file>" "<target-file>" --dry-run
   ```

   This shows:
   - Which files will be modified
   - How many elements will be moved
   - What relations will be updated
   - Git-style diffs for all affected files

3. **Apply the file move:**
   ```bash
   reqvire mv-file "<source-file>" "<target-file>"
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
   reqvire validate
   ```

## Important Notes

- **Path resolution**: File paths are resolved relative to current working directory
- **Automatic relation updates**: All relations pointing to moved elements are automatically updated throughout the model
- **Identifier updates**: Element identifiers change from `<source-file>#<slug>` to `<target-file>#<slug>`
- **Subdirectory support**: Works correctly when executed from subdirectories within git repository
- **Atomic operation**: Either all elements move successfully or none move (on validation failure)

## Error Cases

The mv-file operation will fail with a clear error if:
- The source file does not exist or contains no elements
- The target file already exists in the model
- The operation would result in invalid model state

## Examples

**Move file within specifications directory:**
```bash
reqvire mv-file "specifications/OldFile.md" "specifications/NewFile.md"
```

**Move file to different directory:**
```bash
reqvire mv-file "specifications/Auth.md" "security/Authentication.md"
```

**Preview before moving:**
```bash
reqvire mv-file "specs/File.md" "requirements/File.md" --dry-run
```

**Get JSON output with element mappings:**
```bash
reqvire mv-file "old/path.md" "new/path.md" --json
```

**Works from subdirectories (paths relative to current directory):**
```bash
cd submodule/
reqvire mv-file "specs/File.md" "requirements/File.md"
```

## When to Use mv-file

Use mv-file when:
- Reorganizing specification file structure
- Moving all requirements for a feature to a different location
- Consolidating or splitting specification files
- Refactoring the model directory structure

**Note**: mv-file moves entire files with all their elements. To move individual elements between files, use `reqvire mv` instead.

## Related Commands

- **Move element**: `reqvire mv "<element-name>" --to-file="<file>" --to-section="<section>"`
- **Rename element**: `reqvire rename "<current-name>" "<new-name>"`
- **Remove element**: `reqvire rm "<element-name>"`
- **Add element**: `reqvire add --to-file="<file>" --to-section="<section>" < element.md`
