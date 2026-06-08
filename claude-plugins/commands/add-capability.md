---
allowed-tools: Read, Write, Edit, Bash(npx:*), SlashCommand
argument-hint: [capability-name]
description: Add a complete capability by orchestrating capability, requirement, ontology context, and verification creation following MBSE workflow
model: claude-sonnet-4-5
---

# Add Capability

Add a complete capability by orchestrating multiple commands to create a first-class capability graph node, requirements that specify it, optional ontology/refinement context, verifications, and proper traceability.

## Current Model Context

- Total capabilities: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="capability" --json | jq -r '.global_counters.total_elements'`
- Total requirements: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="requirement" --json | jq -r '.global_counters.total_elements'`
- Verification coverage: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '.summary.leaf_requirements_coverage_percentage'`%

## User Request

${1:+Capability name: $1}
${1:-The user will provide capability details.}

## MBSE Workflow

This command orchestrates the complete workflow:
1. Define the coherent operational/system ability.
2. Add source, ontology, or capability-owned refinement elements when shared domain language or capability-level contracts matter.
3. Create requirements that `specify` the capability.
4. Create verifications for the capability or leaf requirements.
5. Validate and check coverage.

## Steps

1. **Understand the capability:**
   - Ask user for capability description
   - Identify whether it belongs under an existing capability or should become a new capability root
   - Identify source context, ontology context, semantic contracts, direct verification, and concrete obligations
   - Plan child capability hierarchy if the capability has independently verifiable slices
   - Reject modeling as a capability if it is only a UI screen, deployment artifact, code module, task, or low-level implementation detail

2. **Create capability or subcapability:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" add "<capability-file>" <<'EOF'
   ### Capability Name

   Short operational description of what the system is able to accomplish.

   #### Metadata
     * type: capability

   #### Details

   <capability meaning, stakeholder/source context, operational context, and scope>
   EOF
   ```

   If this is a subcapability, link it to the parent capability with `derivedFrom`.

3. **Add ontology context when needed:**

   Use source, compatible refinements, or `ontology` elements when the capability needs reusable vocabulary, ontology terms, source authority, or shared SHACL/OWL contract language. Add ontology elements under `requirements/Ontologies`, then attach them from the consuming capability.
   Ontology attached by capabilities should define nouns, relationships, allowed semantic categories, and stable model rules. Exact commands, fields, URI patterns, workflow steps, outputs, file paths, and reject/write/emit behavior belong in compatible refinements owned by the relevant capability or requirement; semantic contracts are requirement-owned only.

   For broader semantic contract refactoring, use:
   ```bash
   /reqvire:semantic-refactor
   ```

4. **Create requirements that specify the capability:**

   For each specific capability:
   ```bash
   /reqvire:add-requirement
   ```

   Link each requirement to the capability with `specify`. Use requirement `derivedFrom` only inside a requirement hierarchy.

5. **Create verifications for the capability or leaf requirements:**

   For each directly verified capability or leaf requirement:
   ```bash
   /reqvire:add-verification
   ```

   This will:
   - Check if verification is needed at capability level or leaf requirement level
   - Read all capability and requirement context in the trace chain
   - Create verification with comprehensive test criteria
   - Link to tests if applicable

6. **Validate complete capability:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --filter-name="<capability-name>"
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" traces --filter-name="<capability-name>"
   ```

7. **Clean up model:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --fix
   ```

## Command Flow

```
/reqvire:add-capability
  ├─> create capability graph node
  ├─> add source/refinement ownership and ontology attachments when needed
  ├─> /reqvire:add-requirement (specifies capability)
  ├─> /reqvire:add-requirement (child requirement, if needed)
  ├─> /reqvire:add-verification (for leaf requirement)
  └─> reqvire lint --fix
```

## Best Practices

- **Capability first**: Create or identify the coherent operational/system ability before writing obligations
- **Capability vs obligation**: Capability answers what the system can accomplish; requirement answers what the system shall do
- **Capability boundaries**: Do not model UI screens, deployment artifacts, code modules, tickets, or low-level implementation details as capabilities
- **Decompose intentionally**: Use child capabilities when verification, ownership, lifecycle, architecture impact, operational semantics, or requirement clusters differ
- **Semantic contracts when useful**: Put reusable meaning in ontology elements attached by capabilities; put local SHACL profiles in requirement-owned semantic contracts
- **Requirements first**: Create all requirements before verifications
- **Hierarchical**: Capability hierarchy uses capability `derivedFrom`; requirement hierarchy uses requirement `derivedFrom`
- **Traceable**: Requirements point to their capability with `specify`
- **Verify intentionally**: Use `/reqvire:add-verification` for direct capability evidence or leaf requirements
- **Delegate**: Let individual commands handle their specific logic
- **Validate often**: Run validation after each major step

## Notes

- This is an orchestration command - it calls other commands
- Follow MBSE methodology: capability → ontology/refinement context → requirements → verifications → tests
- Each step uses specialized commands for consistency
- Run `reqvire coverage` at the end to confirm complete capability coverage
