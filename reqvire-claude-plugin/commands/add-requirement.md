---
description: Add a new requirement to the Reqvire model with proper structure and traceability
---

# Add New Requirement

Add a new requirement to the Reqvire model following MBSE best practices.

## Steps

1. **Understand the context:**
   - Ask user for requirement details (name, description)
   - Identify parent requirement if this is a derived requirement
   - Determine which file should contain this requirement
   - Determine if this is a leaf requirement (no children) or parent requirement

2. **Locate the appropriate file:**
   - Check existing specifications structure
   - Follow organizational approach (separate vs co-located)
   - Identify correct section within the file

3. **Create the requirement element:**
   ```markdown
   ### Requirement Name

   The system shall [capability/constraint following ears patterns].

   #### Metadata
     * type: requirement

   #### Relations
     * derivedFrom: [Parent Requirement](path/to/parent.md#parent-requirement)
   ```

4. **Add refinement details if needed:**
   Use `#### Details` for clarifications that refine but don't change/add capabilities:
   ```markdown
   #### Details

   <details>
   <summary>Additional Context</summary>

   - Clarification points
   - Rationale
   - Examples

   </details>
   ```

5. **Follow ears patterns:**
   - **Ubiquitous**: "The system shall [capability]"
   - **Event-driven**: "when [trigger] the system shall [response]"
   - **State-driven**: "while [state] the system shall [capability]"
   - **Unwanted**: "if [condition] then the system shall [response]"
   - **Optional**: "where [feature] the system shall [capability]"

6. **Check if verification is needed:**
   - **Leaf requirement** (no derived children): Needs verification
   - **Parent requirement** (has derived children): Verification rolls up from children

   Run traces to check hierarchy:
   ```bash
   reqvire traces --filter-name="<requirement-name>"
   ```

7. **Validate the changes:**
   ```bash
   reqvire validate
   ```

8. **Check coverage:**
   ```bash
   reqvire coverage --filter-name="<requirement-name>"
   ```

9. **Next steps:**
   - If **leaf requirement**: Suggest `/add-verification` to create verification
   - If **parent requirement**: Explain verification will roll up from child requirements

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
