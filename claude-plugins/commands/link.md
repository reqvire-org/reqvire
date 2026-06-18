---
allowed-tools: Read, Bash(npx:*)
argument-hint: <source> <relation-type-or-reusesContract> <target>
description: Link elements with a relation or reuse files/elements
model: claude-sonnet-4-5
---

# Link Elements

Create a relation between elements or reuse files/contract elements. This unified command handles both relations and reused_contract_context.

## Current Model Context

- Total elements: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.global_counters.total_elements'`

## User Request

${1:+Source element: $1}
${2:+Relation type or 'reusesContract': $2}
${3:+Target: $3}
${1:-The user will provide source element, relation type (or 'reusesContract'), and target.}

## Steps

1. **Understand the context:**
   - Identify the source element (by name)
   - Determine if this is a relation or reused_contract_context ('reusesContract' keyword)
   - Identify the target (element name, file path, or URL)
   - Verify source element exists

2. **Preview the link operation:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "<source-element>" "<relation-type-or-reusesContract>" "<target>" --dry-run
   ```

   This shows:
   - Which file will be modified
   - The relation/reused_contract_context that will be added
   - Git-style diff for the affected file

3. **Apply the link:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "<source-element>" "<relation-type-or-reusesContract>" "<target>"
   ```

   The link command automatically:
   - For relations: Adds to the source element's Relations section
   - For reused_contract_context: Adds to the source element's Reused Contract Context section
   - Calculates correct relative path for cross-file links
   - Creates the section if it doesn't exist
   - Maintains idempotency (no duplicate relations/reused_contract_context)

4. **Verify the changes:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
   ```

## Supported Relation Types

| Relation Type | Description | Usage |
|---------------|-------------|-------|
| `derivedFrom` | Source derives from target | Child to parent inside the same family: capability, requirement, or ontology |
| `derive` | Source has derived target | Parent to child inside the same family: capability, requirement, or ontology |
| `verifiedBy` | Source is verified by target | Requirement to verification |
| `verify` | Source verifies target | Verification to requirement |
| `satisfiedBy` | Source is satisfied by target | Requirement to implementation |
| `satisfy` | Source satisfies target | Implementation to requirement |
| `definedBy` | Source owns target as contract | Requirement to requirement-owned contract |
| `define` | Source defines target | Contract element to compatible owner (auto-generated) |
| `reusesContract` | Reuse file or element | Reuse compatible requirement-owned contracts to requirements |

## Target Types

**For relations:**
- Element name (e.g., "System Requirement")
- Internal file path (e.g., "src/impl.rs")
- External URL (e.g., "https://example.com/spec.html")

`definedBy` rule:
- `definedBy` must target a contract element (by name/identifier).
- Plain file-path targets are invalid for `definedBy` (including `DesignDocuments/*.md` without `#fragment`).

`satisfiedBy` / `satisfy` rule:
- Allowed source/target model element types are `requirement`, `test-verification`, and `formal-proof-verification`.
- `capability` is not allowed to use `satisfiedBy`/`satisfy`.
- `verification-objective` is not allowed to use `satisfiedBy`/`satisfy`; it is a planning hierarchy node.

`verifiedBy` / `verify` rule:
- Allowed concrete verification types are `test-verification`, `formal-proof-verification`, `analysis-verification`, `inspection-verification`, and `demonstration-verification`.
- `verification-objective` may use `derivedFrom`/`derive` inside verification-family hierarchy but is not a valid `verify` source or `verifiedBy` target.

**For reusesContract:**
- Internal file path (e.g., "docs/SLA.pdf")
- Contract element name (e.g., "Performance Constraint")

## Important Notes

- **Duplicate detection**: The link command fails with an error if the relation or reused_contract_context already exists
- **Cross-section duplicates**: Cannot add a relation to a target that already exists as an reused_contract_context (and vice versa) - this is a validation error
- **Element names**: Use the exact element name as it appears in the heading
- **Cross-file links**: Relative paths are calculated automatically
- **Inverse relations**: Reqvire auto-generates inverse relations (e.g., derive from derivedFrom)
- **Reused Contract Context**: Only Contract elements (constraint, behavior, specification) can be reused

## Link Options

- `<source>`: Name of source element (required)
- `<relation-type-or-reusesContract>`: Relation type OR 'reusesContract' keyword (required)
- `<target>`: Element name, file path, or URL (required)
- `--dry-run`: Preview changes without applying

## Error Cases

The link operation will fail with a clear error if:
- The source element does not exist
- The target element does not exist (for element relations)
- The relation type is invalid
- For reusesContract: the target element is not a Contract type
- For reusesContract: the contract has no `define` relation (must be owned by a requirement via `definedBy` first)
- For reusesContract: source and contract's owner are in the same derivation hierarchy
- The relation or reused_contract_context already exists (duplicate)
- The target already exists in the other section (cross-section duplicate)

## Examples

**Link requirement to parent:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "Password Login Requirement" derivedFrom "Authentication Requirement"
```

**Link requirement to implementation file:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "Authentication Requirement" satisfiedBy "src/auth/login.rs"
```

**Link to external URL:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "Compliance Requirement" trace "https://example.com/regulations.html"
```

**Reuse a document:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "System Requirement" reusesContract "docs/SLA.pdf"
```

**Reuse a contract element:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "System Requirement" reusesContract "Performance Constraint"
```

**Preview before linking:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "Capability X" trace "Capability Y" --dry-run
```

## When to Use link

Use link when:
- Establishing traceability between requirements
- Connecting requirements to verifications
- Creating derivation hierarchies
- Adding semantically specific relations for impact analysis
- Linking to implementation files
- Reusing Contract documents or reference materials
- Reusing Contract contract elements (constraints, behaviors, specifications)

## Related Commands

- **Unlink**: `reqvire unlink <source> <target>` (auto-detects relation vs reused_contract_context)
- **Search relations**: `reqvire search --have-relations="derivedFrom"`
- **Search reused_contract_context**: `reqvire search --has-reused-contract-context`
