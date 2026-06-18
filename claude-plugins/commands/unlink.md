---
allowed-tools: Read, Bash(npx:*)
argument-hint: <source> <target>
description: Remove a relation or reused_contract_context (auto-detects type)
model: claude-sonnet-4-5
---

# Unlink Elements

Remove an existing relation or reused_contract_context between elements. The command auto-detects whether the target is a relation or reused_contract_context.

## Current Model Context

- Total elements: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.global_counters.total_elements'`

## User Request

${1:+Source element: $1}
${2:+Target: $2}
${1:-The user will provide source element and target.}

## Steps

1. **Understand the context:**
   - Identify the source element (by name)
   - Identify the target (element name or file path)
   - The command will auto-detect if it's a relation or reused_contract_context

2. **Preview the unlink operation:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" unlink "<source-element>" "<target>" --dry-run
   ```

   This shows:
   - Which file will be modified
   - The relation/reused_contract_context that will be removed
   - Git-style diff for the affected file

3. **Apply the unlink:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" unlink "<source-element>" "<target>"
   ```

   The unlink command automatically:
   - Searches relations first, then reused_contract_context
   - Removes the relation/reused_contract_context from the source element
   - Cleans up empty sections automatically
   - Maintains model consistency

4. **Verify the changes:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
   ```

## Auto-Detection Behavior

The unlink command auto-detects the type:
1. **First**: Searches for a relation from source to target element
2. **Then**: If no relation found, searches for an reused_contract_context matching the target
3. Only one relation per source-target pair is allowed, so no ambiguity

## Important Notes

- **Auto-detection**: No need to specify relation type - the command finds it automatically
- **Explicit relations only**: Only removes user-created relations (not auto-generated inverse relations)
- **Element names**: Use the exact element name as it appears in the heading
- **Cleanup**: Empty Relations/Reused Contract Context sections are removed automatically
- **Validation**: Consider model validity after unlinking (orphaned elements may cause validation errors)

## Unlink Options

- `<source>`: Name of source element (required)
- `<target>`: Element name or file path (required)
- `--dry-run`: Preview changes without applying

## Error Cases

The unlink operation will fail with a clear error if:
- The source element does not exist
- No relation or reused_contract_context found from source to target

## Examples

**Remove a relation (auto-detected):**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" unlink "Password Login Requirement" "Authentication Requirement"
```

**Remove an reused_contract_context file:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" unlink "System Requirement" "docs/SLA.pdf"
```

**Remove an reused element:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" unlink "System Requirement" "Performance Constraint"
```

**Preview before unlinking:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" unlink "Capability X" "Capability Y" --dry-run
```

## When to Use unlink

Use unlink when:
- Removing incorrect traceability links
- Refactoring requirement hierarchies
- Disconnecting deprecated verifications
- Cleaning up obsolete semantic relations
- Removing reused context from documents or files
- Removing reused contract elements

## Related Commands

- **Link elements**: `reqvire link <source> <relation-type-or-reusesContract> <target>`
- **Search relations**: `reqvire search --have-relations="derivedFrom"`
- **Search reused_contract_context**: `reqvire search --has-reused-contract-context`
