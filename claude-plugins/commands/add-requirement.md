---
allowed-tools: Read, Bash(npx:*)
argument-hint: [requirement-name]
description: Add a new requirement to the Reqvire model with proper structure and traceability
model: claude-sonnet-4-5
---

# Add New Requirement

Add a new requirement to the Reqvire model following MBSE best practices.

## Current Model Context

- Total requirements: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="requirement" --json | jq -r '.global_counters.total_elements'`
- Total capabilities: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="capability" --json | jq -r '.global_counters.total_elements'`
- Verification coverage: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '.summary.leaf_requirements_coverage_percentage'`%
- Unverified leaf requirements: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '.summary.unverified_leaf_requirements'`

## User Request

${1:+Requirement name: $1}
${1:-The user will provide requirement details.}

## Steps

1. **Understand the context:**
   - Ask user for requirement details (name, description) if not provided
   - Identify the capability this requirement specifies
   - Identify parent requirement only if this is a child requirement inside a requirement hierarchy
   - Identify target file (user specifies or follows project conventions)

2. **Draft the requirement content:**

   Follow EARS patterns for requirement statements:
   - **Ubiquitous**: "The system shall [capability]"
   - **Event-driven**: "when [trigger] the system shall [response]"
   - **State-driven**: "while [state] the system shall [capability]"
   - **Unwanted**: "if [condition] then the system shall [response]"
   - **Optional**: "where [capability] the system shall [capability]"

   Template:
   ```markdown
   ### Requirement Name

   The system shall [capability/constraint following EARS patterns].

   #### Metadata
     * type: requirement

   #### Relations
     * specify: [Owning Capability](path/to/capabilities.md#owning-capability)
   ```

   Add `derivedFrom` only when this requirement is a child of another requirement:

   ```markdown
   #### Relations
     * specify: [Owning Capability](path/to/capabilities.md#owning-capability)
     * derivedFrom: [Parent Requirement](path/to/parent.md#parent-requirement)
   ```

   Optional details section for clarifications:
   ```markdown
   #### Details

   <details>
   <summary>Additional Context</summary>

   - Clarification points
   - Rationale
   - Examples

   </details>
   ```

3. **Add the requirement using reqvire add command:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" add "<file-path>" <<'EOF'
   ### Requirement Name

   The system shall [capability].

   #### Metadata
     * type: requirement

   #### Relations
     * specify: [Owning Capability](path.md#owning-capability)
   EOF
   ```

   Optional: Insert at specific position (0-based index):
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" add "<file-path>" 0 <<'EOF'
   ...
   EOF
   ```

   Alternative using pipe:
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" add "<file-path>" < element.md
   ```

   The add command automatically:
   - Validates markdown format
   - Checks element name uniqueness
   - Validates relation format
   - Updates the file

4. **Check if verification is needed:**
   - **Leaf requirement** (no derived children): Needs verification
   - **Parent requirement** (has derived children): Verification rolls up from children
   - **Capability**: May be directly verified when there is capability-level evidence; coverage also rolls up from requirements that specify it

   Run traces to check hierarchy:
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" traces --filter-name="<requirement-name>"
   ```

5. **Check coverage:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --filter-name="<requirement-name>"
   ```

6. **Next steps:**
   - If **leaf requirement**: Suggest `/add-verification` to create verification
   - If **parent requirement**: Explain verification will roll up from child requirements

## Element Manipulation

After adding requirements, you may need to reorganize:

**Move element to different file:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv "<element-name>" "<target-file>"
```

**Move element with specific position (0-based index):**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv "<element-name>" "<target-file>" 0
```

**Remove element:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" rm "<element-name>"
```

## Best Practices

- **Atomic requirements**: One capability per requirement
- **Refinement in Details**: Clarifications go in `#### Details`, not new requirements
- **Capability ownership**: Every requirement should specify exactly one owning capability
- **Leaf verification**: Leaf requirements need direct verification; capabilities may also be directly verified when evidence is capability-level
- **Roll-up coverage**: Parent requirements inherit verification from children
- **Clear**: Use ears patterns for consistency
- **Traceable**: Always link to the owning capability via `specify`; use `derivedFrom` only between requirements
- **Unique names**: Element names must be unique within each file

## Verification Philosophy

Reqvire uses **bottom roll-up verification coverage**:
- **Leaf requirements** must be verified directly
- **Parent requirements** inherit verification from their children
- **Capabilities** may be directly verified and also inherit coverage from requirements that specify them
- High-level requirements are rarely verified directly
- Run `reqvire traces` to see verification roll-up structure

## Notes

- Element names become URL fragments (spaces → hyphens, lowercase)
- Use two-space indentation for Relations entries
- Use `#### Details` for refinements that don't add capabilities
- Use `ontology` for shared ontology meaning attached by capabilities and `semantic-contract` elements for reusable SHACL shape profiles that explicitly `use` ontology and `constrain` requirements.
- Keep exact commands, fields, URI patterns, workflow steps, outputs, file paths, and reject/write/emit behavior in compatible requirement-owned `source`, `specification`, `constraint`, `behavior`, `state`, and `input-output` refinements. Use shape-only `semantic-contract` elements only for semantic checks that need explicit ontology-use and requirement-constraint relations.
- For complete capability (requirement + verification + test), use `/reqvire:add-capability`
