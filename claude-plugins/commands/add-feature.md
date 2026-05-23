---
allowed-tools: Read, Write, Edit, Bash(npx:*), SlashCommand
argument-hint: [feature-name]
description: Add a complete feature by orchestrating feature, requirement, ontology context, and verification creation following MBSE workflow
model: claude-sonnet-4-5
---

# Add Feature

Add a complete feature by orchestrating multiple commands to create a feature anchor, requirements that specify it, optional ontology context, verifications, and proper traceability.

## Current Model Context

- Total features: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="feature" --json | jq -r '.global_counters.total_elements'`
- Total requirements: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type="requirement" --json | jq -r '.global_counters.total_elements'`
- Verification coverage: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '.summary.leaf_requirements_coverage_percentage'`%

## User Request

${1:+Feature name: $1}
${1:-The user will provide feature details.}

## MBSE Workflow

This command orchestrates the complete workflow:
1. Define the feature capability anchor.
2. Add source or ontology elements when shared domain language matters.
3. Create requirements that `specify` the feature.
4. Create verifications for leaf requirements.
5. Validate and check coverage.

## Steps

1. **Understand the feature:**
   - Ask user for feature description
   - Identify whether it belongs under an existing feature or should become a new feature root
   - Identify source context, semantic contracts, and concrete obligations
   - Plan subfeature hierarchy if the capability has meaningful slices

2. **Create feature or subfeature:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" add "<feature-file>" <<'EOF'
   ### Feature Name

   As a **stakeholder**, I want [capability], so that [outcome].

   #### Metadata
     * type: feature

   #### Details

   <capability anchor, stakeholder/source context, and scope>
   EOF
   ```

   If this is a subfeature, link it to the parent feature with `derivedFrom`.

3. **Add ontology context when needed:**

   Use source or `ontology` elements when the feature needs reusable vocabulary, ontology terms, source authority, or shared SHACL/OWL contract language. Add ontology elements under `requirements/Ontologies`, then attach them from the consuming feature.
   Ontology attached by features should define nouns, relationships, allowed semantic categories, and stable model rules. Exact commands, fields, URI patterns, workflow steps, outputs, file paths, and reject/write/emit behavior belong in requirement-owned refinements.

   For broader semantic contract refactoring, use:
   ```bash
   /reqvire:semantic-refactor
   ```

4. **Create requirements that specify the feature:**

   For each specific capability:
   ```bash
   /reqvire:add-requirement
   ```

   Link each requirement to the feature with `specify`. Use requirement `derivedFrom` only inside a requirement hierarchy.

5. **Create verifications for leaf requirements:**

   For each leaf requirement:
   ```bash
   /reqvire:add-verification
   ```

   This will:
   - Check if verification is needed (leaf vs parent)
   - Read all requirements in trace chain
   - Create verification with comprehensive test criteria
   - Link to tests if applicable

6. **Validate complete feature:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --filter-name="<feature-name>"
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" traces --filter-name="<feature-name>"
   ```

7. **Clean up model:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --fix
   ```

## Command Flow

```
/reqvire:add-feature
  ├─> create feature anchor
  ├─> add source refinements / ontology attachments when needed
  ├─> /reqvire:add-requirement (specifies feature)
  ├─> /reqvire:add-requirement (child requirement, if needed)
  ├─> /reqvire:add-verification (for leaf requirement)
  └─> reqvire lint --fix
```

## Best Practices

- **Feature first**: Create or identify the feature anchor before writing obligations
- **Capability vs obligation**: Feature answers what capability/domain area exists; requirement answers what the system shall do
- **Semantic contracts when useful**: Put reusable meaning in ontology elements; put local SHACL profiles in requirement-owned semantic contracts
- **Requirements first**: Create all requirements before verifications
- **Hierarchical**: Feature hierarchy uses feature `derivedFrom`; requirement hierarchy uses requirement `derivedFrom`
- **Traceable**: Requirements point to their feature with `specify`
- **Verify leaves only**: Use `/reqvire:add-verification` for leaf requirements
- **Delegate**: Let individual commands handle their specific logic
- **Validate often**: Run validation after each major step

## Notes

- This is an orchestration command - it calls other commands
- Follow MBSE methodology: feature → ontology context → requirements → verifications → tests
- Each step uses specialized commands for consistency
- Run `reqvire coverage` at the end to confirm complete feature coverage
