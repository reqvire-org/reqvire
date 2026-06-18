---
allowed-tools: Read, Bash(npx:*)
argument-hint: <element-name> <file> [index]
description: Move an element to a different file or position
model: claude-sonnet-4-5
---

# Move Element

Move an existing model element to a different file or position within the model.

## Current Model Context

- Total elements: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.global_counters.total_elements'`

## User Request

${1:+Element name: $1}
${1:-The user will provide element name and target location.}

## Steps

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

## Important Notes

- **Global uniqueness**: Element names are globally unique, so you only need the element name
- **Path resolution**: File paths are resolved relative to current working directory
- **Automatic relation updates**: All relations throughout the model are automatically updated
- **Identifier update**: Element identifier changes from `<old-file>#<slug>` to `<new-file>#<slug>`

## Move Options

- `<element-name>`: Name of element to move (required)
- `<file>`: Target file path (required)
- `[index]`: Position within target file (0-based, default: append to end)
- `--dry-run`: Preview changes without applying
- `--json`: Output results in JSON format

## Error Cases

The mv operation will fail with a clear error if:
- The element name does not exist
- The target file does not exist (must exist in git)
- The operation would result in invalid model state

## Examples

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

**Get JSON output:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv "Element" "File.md" --json
```

## When to Use mv

Use mv when:
- Moving individual requirements or verifications between files
- Reorganizing elements within specification structure
- Reordering elements within files

**Note**: To move entire files with all their elements, use `reqvire mv-file` instead.

## Related Commands

- **Move file**: `reqvire mv-file <source-file> <target-file>`
- **Rename element**: `reqvire rename <current-name> <new-name>`
- **Remove element**: `reqvire rm <element-name>`
- **Add element**: `reqvire add <file> < element.md`
