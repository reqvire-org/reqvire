# Link and Unlink Elements

Create or remove relations between elements, or bind/unbind compatible requirement-owned contract elements.

---

## Linking Elements

Create a relation between elements or bind compatible requirement-owned contract elements. This handles both relations and contract_bindings.

### Steps

1. **Understand the context:**
   - Identify the source element (by name)
   - Determine if this is a relation or contract_bindings ('bindContract' keyword)
   - Identify the target (element name, file path, or URL)
   - Verify source element exists

2. **Preview the link operation:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "<source-element>" "<relation-type-or-bindContract>" "<target>" --dry-run
   ```

   This shows:
   - Which file will be modified
   - The relation/contract_bindings that will be added
   - Git-style diff for the affected file

3. **Apply the link:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "<source-element>" "<relation-type-or-bindContract>" "<target>"
   ```

   The link command automatically:
   - For relations: Adds to the source element's Relations section
   - For contract_bindings: Adds to the source element's Contract Bindings section
   - Calculates correct relative path for cross-file links
   - Creates the section if it doesn't exist
   - Maintains idempotency (no duplicate relations/contract_bindings)

4. **Verify the changes:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
   ```

### Supported Relation Types

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
| `bindContract` | Bind contract element | Bind compatible requirement-owned contracts to requirements |

### Target Types

**For relations:**
- Element name (e.g., "System Requirement")
- Internal file path (e.g., "src/impl.rs")
- External URL (e.g., "https://example.com/spec.html")

`definedBy` rule:
- `definedBy` must target a contract element (by name/identifier).
- Plain file-path targets are invalid for `definedBy`.

`satisfiedBy` / `satisfy` rule:
- Allowed source/target model element types are `requirement`, `test-verification`, and `formal-proof-verification`.
- `capability` is not allowed to use `satisfiedBy`/`satisfy`.
- `verification-objective` is not allowed to use `satisfiedBy`/`satisfy`.

`verifiedBy` / `verify` rule:
- Allowed concrete verification types are `test-verification`, `formal-proof-verification`, `analysis-verification`, `inspection-verification`, and `demonstration-verification`.
- `verification-objective` may use `derivedFrom`/`derive` inside verification-family hierarchy but is not a valid `verify` source or `verifiedBy` target.

**For bindContract:**
- Internal file path (e.g., "docs/SLA.pdf")
- Contract element name (e.g., "Performance Constraint")

### Important Notes

- **Duplicate detection**: The link command fails if the relation or contract_bindings already exists
- **Cross-section duplicates**: Cannot add a relation to a target that already exists as a contract_bindings (and vice versa)
- **Element names**: Use the exact element name as it appears in the heading
- **Cross-file links**: Relative paths are calculated automatically
- **Inverse relations**: Reqvire auto-generates inverse relations (e.g., derive from derivedFrom)

### Examples

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
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "System Requirement" bindContract "docs/SLA.pdf"
```

**Reuse a contract element:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "System Requirement" bindContract "Performance Constraint"
```

**Preview before linking:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "Capability X" trace "Capability Y" --dry-run
```

---

## Unlinking Elements

Remove an existing relation or contract_bindings between elements. The command auto-detects whether the target is a relation or contract_bindings.

### Steps

1. **Understand the context:**
   - Identify the source element (by name)
   - Identify the target (element name or file path)
   - The command will auto-detect if it's a relation or contract_bindings

2. **Preview the unlink operation:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" unlink "<source-element>" "<target>" --dry-run
   ```

3. **Apply the unlink:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" unlink "<source-element>" "<target>"
   ```

   The unlink command automatically:
   - Searches relations first, then contract_bindings
   - Removes the relation/contract_bindings from the source element
   - Cleans up empty sections automatically
   - Maintains model consistency

4. **Verify the changes:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
   ```

### Auto-Detection Behavior

1. **First**: Searches for a relation from source to target element
2. **Then**: If no relation found, searches for a contract_bindings matching the target
3. Only one relation per source-target pair is allowed, so no ambiguity

### Important Notes

- **Explicit relations only**: Only removes user-created relations (not auto-generated inverse relations)
- **Cleanup**: Empty Relations/Contract Bindings sections are removed automatically
- **Validation**: Consider model validity after unlinking (orphaned elements may cause validation errors)

### Examples

**Remove a relation (auto-detected):**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" unlink "Password Login Requirement" "Authentication Requirement"
```

**Remove a contract_bindings file:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" unlink "System Requirement" "docs/SLA.pdf"
```

**Preview before unlinking:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" unlink "Capability X" "Capability Y" --dry-run
```
