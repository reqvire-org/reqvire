---
allowed-tools: Read, Bash(reqvire:*)
argument-hint: [requirement-name]
description: Add a new requirement to the Reqvire model with proper structure and traceability
model: claude-sonnet-4-5
---

# Add New Requirement

Add a new requirement to the Reqvire model following MBSE best practices.

## Current Model Context

- Total requirements: !`reqvire search --json | jq -r '.global_counters.total_elements'`
- Verification coverage: !`reqvire coverage --json | jq -r '.summary.leaf_requirements_coverage_percentage'`%
- Unverified leaf requirements: !`reqvire coverage --json | jq -r '.summary.unverified_leaf_requirements'`

## User Request

${1:+Requirement name: $1}
${1:-The user will provide requirement details.}

## Steps

1. **Understand the context:**
   - Ask user for requirement details (name, description) if not provided
   - Identify parent requirement if this is a derived requirement
   - Identify target file (user specifies or follows project conventions)

2. **Draft the requirement content:**

   Follow EARS patterns for requirement statements:
   - **Ubiquitous**: "The system shall [capability]"
   - **Event-driven**: "when [trigger] the system shall [response]"
   - **State-driven**: "while [state] the system shall [capability]"
   - **Unwanted**: "if [condition] then the system shall [response]"
   - **Optional**: "where [feature] the system shall [capability]"

   Template:
   ```markdown
   ### Requirement Name

   The system shall [capability/constraint following EARS patterns].

   #### Metadata
     * type: requirement

   #### Relations
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
   reqvire add "<file-path>" <<'EOF'
   ### Requirement Name

   The system shall [capability].

   #### Metadata
     * type: requirement

   #### Relations
     * derivedFrom: [Parent](path.md#parent)
   EOF
   ```

   Optional: Insert at specific position (0-based index):
   ```bash
   reqvire add "<file-path>" 0 <<'EOF'
   ...
   EOF
   ```

   Alternative using pipe:
   ```bash
   cat element.md | reqvire add "<file-path>"
   ```

   The add command automatically:
   - Validates markdown format
   - Checks element name uniqueness
   - Validates relation format
   - Updates the file

4. **Check if verification is needed:**
   - **Leaf requirement** (no derived children): Needs verification
   - **Parent requirement** (has derived children): Verification rolls up from children

   Run traces to check hierarchy:
   ```bash
   reqvire traces --filter-name="<requirement-name>"
   ```

5. **Check coverage:**
   ```bash
   reqvire coverage --filter-name="<requirement-name>"
   ```

6. **Next steps:**
   - If **leaf requirement**: Suggest `/add-verification` to create verification
   - If **parent requirement**: Explain verification will roll up from child requirements

## Element Manipulation

After adding requirements, you may need to reorganize:

**Move element to different file:**
```bash
reqvire mv "<element-name>" "<target-file>"
```

**Move element with specific position (0-based index):**
```bash
reqvire mv "<element-name>" "<target-file>" 0
```

**Remove element:**
```bash
reqvire rm "<element-name>"
```

## Best Practices

- **Atomic requirements**: One capability per requirement
- **Refinement in Details**: Clarifications go in `#### Details`, not new requirements
- **Leaf verification**: Only leaf requirements need direct verification
- **Roll-up coverage**: Parent requirements inherit verification from children
- **Clear**: Use ears patterns for consistency
- **Traceable**: Always link to parent via derivedFrom
- **Unique names**: Element names must be unique within each file

## Verification Philosophy

Reqvire uses **bottom roll-up verification coverage**:
- **Leaf requirements** must be verified directly
- **Parent requirements** inherit verification from their children
- High-level requirements are rarely verified directly
- Run `reqvire traces` to see verification roll-up structure

## Notes

- Element names become URL fragments (spaces → hyphens, lowercase)
- Use two-space indentation for Relations entries
- Use `#### Details` for refinements that don't add capabilities
- For complete feature (requirement + verification + test), use `/add-feature`
