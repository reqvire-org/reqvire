---
allowed-tools: Read, Bash(reqvire:*)
argument-hint: [element-name] [--to-file] [--to-section] [--index]
description: Move an element to a different file, section, or position
model: claude-sonnet-4-5-20250929
---

# Move Element

Move an existing model element to a different file, section, or position within the model.

## Current Model Context

- Total elements: !`reqvire search --json | jq -r '.global_counters.total_elements'`

## User Request

${1:+Element name: $1}
${1:-The user will provide element name and target location.}

## Steps

1. **Understand the context:**
   - Identify the element to move (by name)
   - Determine the target location (file, section, and/or index)
   - Verify element exists in the model

2. **Preview the move operation:**
   ```bash
   reqvire mv "<element-name>" --to-file="<file>" --to-section="<section>" --dry-run
   ```

   This shows:
   - Which files will be modified
   - Where the element will be moved
   - What relations will be updated
   - Git-style diffs for all affected files

3. **Apply the move:**
   ```bash
   reqvire mv "<element-name>" --to-file="<file>" --to-section="<section>"
   ```

   The mv command automatically:
   - Removes element from source file
   - Adds element to target file and section
   - Updates the element identifier to reflect new location
   - Updates all forward relations (from the element to others)
   - Updates all backward relations (from other elements to this one)
   - Maintains model consistency

4. **Verify the changes:**
   ```bash
   reqvire validate
   ```

## Important Notes

- **Global uniqueness**: Element names are globally unique, so you only need the element name
- **Path resolution**: File paths are resolved relative to current working directory
- **Automatic relation updates**: All relations throughout the model are automatically updated
- **Identifier update**: Element identifier changes from `<old-file>#<slug>` to `<new-file>#<slug>`
- **Optional parameters**: You can specify just --to-file, just --to-section, or both

## Move Options

- `--to-file="<file>"`: Move element to a different file
- `--to-section="<section>"`: Move element to a specific section (by heading text)
- `--index=<n>`: Position within target section (0-based, default: append to end)
- `--dry-run`: Preview changes without applying
- `--json`: Output results in JSON format

## Error Cases

The mv operation will fail with a clear error if:
- The element name does not exist
- The target file does not exist (must exist in git)
- The target section does not exist in the target file
- The operation would result in invalid model state

## Examples

**Move element to different file:**
```bash
reqvire mv "User Authentication" --to-file="specifications/Security.md"
```

**Move element to specific section:**
```bash
reqvire mv "Login Feature" --to-section="Authentication Requirements"
```

**Move element to different file and section:**
```bash
reqvire mv "Data Storage" --to-file="specs/Backend.md" --to-section="Database"
```

**Insert at specific position (index 0 = first element in section):**
```bash
reqvire mv "High Priority Req" --to-section="Critical Features" --index=0
```

**Preview before moving:**
```bash
reqvire mv "Feature X" --to-file="NewFile.md" --dry-run
```

**Get JSON output:**
```bash
reqvire mv "Element" --to-file="File.md" --json
```

## When to Use mv

Use mv when:
- Moving individual requirements or verifications between files
- Reorganizing elements within specification structure
- Moving elements to more appropriate sections
- Reordering elements within sections

**Note**: To move entire files with all their elements, use `reqvire mv-file` instead.

## Related Commands

- **Move file**: `reqvire mv-file "<source-file>" "<target-file>"`
- **Rename element**: `reqvire rename "<current-name>" "<new-name>"`
- **Remove element**: `reqvire rm "<element-name>"`
- **Add element**: `reqvire add --to-file="<file>" --to-section="<section>" < element.md`
