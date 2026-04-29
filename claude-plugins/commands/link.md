---
allowed-tools: Read, Bash(npx:*)
argument-hint: <source> <relation-type-or-attaching> <target>
description: Link elements with a relation or attach files/elements
model: claude-sonnet-4-5
---

# Link Elements

Create a relation between elements or attach files/refinement elements. This unified command handles both relations and attachments.

## Current Model Context

- Total elements: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.global_counters.total_elements'`

## User Request

${1:+Source element: $1}
${2:+Relation type or 'attaching': $2}
${3:+Target: $3}
${1:-The user will provide source element, relation type (or 'attaching'), and target.}

## Steps

1. **Understand the context:**
   - Identify the source element (by name)
   - Determine if this is a relation or attachment ('attaching' keyword)
   - Identify the target (element name, file path, or URL)
   - Verify source element exists

2. **Preview the link operation:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "<source-element>" "<relation-type-or-attaching>" "<target>" --dry-run
   ```

   This shows:
   - Which file will be modified
   - The relation/attachment that will be added
   - Git-style diff for the affected file

3. **Apply the link:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "<source-element>" "<relation-type-or-attaching>" "<target>"
   ```

   The link command automatically:
   - For relations: Adds to the source element's Relations section
   - For attachments: Adds to the source element's Attachments section
   - Calculates correct relative path for cross-file links
   - Creates the section if it doesn't exist
   - Maintains idempotency (no duplicate relations/attachments)

4. **Verify the changes:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
   ```

## Supported Relation Types

| Relation Type | Description | Usage |
|---------------|-------------|-------|
| `derivedFrom` | Source derives from target | Child to parent requirement |
| `derive` | Source has derived target | Parent to child requirement |
| `verifiedBy` | Source is verified by target | Requirement to verification |
| `verify` | Source verifies target | Verification to requirement |
| `satisfiedBy` | Source is satisfied by target | Requirement to implementation |
| `satisfy` | Source satisfies target | Implementation to requirement |
| `refinedBy` | Source owns target as refinement | Requirement to refinement element |
| `refine` | Source refines target | Refinement element to requirement (auto-generated) |
| `trace` | General traceability link | Any traceability relationship |
| `attaching` | Attach file or element | Attach documents or refinements |

## Target Types

**For relations:**
- Element name (e.g., "System Requirement")
- Internal file path (e.g., "src/impl.rs")
- External URL (e.g., "https://example.com/spec.html")

`refinedBy` rule:
- `refinedBy` must target a refinement element (by name/identifier).
- Plain file-path targets are invalid for `refinedBy` (including `DesignDocuments/*.md` without `#fragment`).

`satisfiedBy` / `satisfy` rule:
- Allowed source/target model element types are `requirement` and `test-verification`.
- `user-requirement` is not allowed to use `satisfiedBy`/`satisfy`.

**For attaching:**
- Internal file path (e.g., "docs/SLA.pdf")
- Refinement element name (e.g., "Performance Constraint")

## Important Notes

- **Duplicate detection**: The link command fails with an error if the relation or attachment already exists
- **Cross-section duplicates**: Cannot add a relation to a target that already exists as an attachment (and vice versa) - this is a validation error
- **Element names**: Use the exact element name as it appears in the heading
- **Cross-file links**: Relative paths are calculated automatically
- **Inverse relations**: Reqvire auto-generates inverse relations (e.g., derive from derivedFrom)
- **Attachments**: Only Refinement elements (constraint, behavior, specification) can be attached

## Link Options

- `<source>`: Name of source element (required)
- `<relation-type-or-attaching>`: Relation type OR 'attaching' keyword (required)
- `<target>`: Element name, file path, or URL (required)
- `--dry-run`: Preview changes without applying

## Error Cases

The link operation will fail with a clear error if:
- The source element does not exist
- The target element does not exist (for element relations)
- The relation type is invalid
- For attaching: the target element is not a Refinement type
- For attaching: the refinement has no `refine` relation (must be owned by a requirement via `refinedBy` first)
- For attaching: source and refinement's owner are in the same derivation hierarchy
- The relation or attachment already exists (duplicate)
- The target already exists in the other section (cross-section duplicate)

## Examples

**Link requirement to parent:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "Feature Requirement" derivedFrom "User Story"
```

**Link requirement to implementation file:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "Authentication Requirement" satisfiedBy "src/auth/login.rs"
```

**Link to external URL:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "Compliance Requirement" trace "https://example.com/regulations.html"
```

**Attach a document:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "System Requirement" attaching "docs/SLA.pdf"
```

**Attach a refinement element:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "System Requirement" attaching "Performance Constraint"
```

**Preview before linking:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "Feature X" trace "Feature Y" --dry-run
```

## When to Use link

Use link when:
- Establishing traceability between requirements
- Connecting requirements to verifications
- Creating derivation hierarchies
- Adding trace relations for impact analysis
- Linking to implementation files
- Attaching documents or reference materials
- Attaching refinement elements (constraints, behaviors, specifications)

## Related Commands

- **Unlink**: `reqvire unlink <source> <target>` (auto-detects relation vs attachment)
- **Search relations**: `reqvire search --have-relations="derivedFrom"`
- **Search attachments**: `reqvire search --has-attachments`
