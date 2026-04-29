---
allowed-tools: Read, Write, Edit, Bash(npx:*), SlashCommand
argument-hint: [feature-name]
description: Add a complete feature by orchestrating requirement and verification creation following MBSE workflow
model: claude-sonnet-4-5
---

# Add Feature

Add a complete feature by orchestrating multiple commands to create requirements, verifications, and proper traceability.

## Current Model Context

- Total requirements: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.global_counters.total_elements'`
- Verification coverage: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '.summary.leaf_requirements_coverage_percentage'`%

## User Request

${1:+Feature name: $1}
${1:-The user will provide feature details.}

## MBSE Workflow

This command orchestrates the complete workflow:
1. Define requirements (parent → children)
2. Create verifications for leaf requirements
3. Validate and check coverage

## Steps

1. **Understand the feature:**
   - Ask user for feature description
   - Identify if this derives from existing requirement
   - Plan requirement hierarchy (parent → leaf requirements)

2. **Create parent requirement (if needed):**
   ```bash
   /reqvire:add-requirement
   ```

   This creates the high-level feature requirement.

3. **Create leaf requirements:**

   For each specific capability:
   ```bash
   /reqvire:add-requirement
   ```

   Link each to the parent via `derivedFrom`.

4. **Create verifications for leaf requirements:**

   For each leaf requirement:
   ```bash
   /reqvire:add-verification
   ```

   This will:
   - Check if verification is needed (leaf vs parent)
   - Read all requirements in trace chain
   - Create verification with comprehensive test criteria
   - Link to tests if applicable

5. **Validate complete feature:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --filter-name="<feature-name>"
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" traces --filter-name="<feature-name>"
   ```

6. **Clean up model:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --fix
   ```

## Command Flow

```
/reqvire:add-feature
  ├─> /reqvire:add-requirement (parent)
  ├─> /reqvire:add-requirement (leaf 1)
  ├─> /reqvire:add-requirement (leaf 2)
  ├─> /reqvire:add-requirement (leaf 3)
  ├─> /reqvire:add-verification (for leaf 1)
  ├─> /reqvire:add-verification (for leaf 2)
  ├─> /reqvire:add-verification (for leaf 3)
  └─> reqvire lint --fix
```

## Best Practices

- **Requirements first**: Create all requirements before verifications
- **Hierarchical**: Parent requirement → leaf requirements
- **Verify leaves only**: Use `/reqvire:add-verification` for leaf requirements
- **Delegate**: Let individual commands handle their specific logic
- **Validate often**: Run validation after each major step

## Notes

- This is an orchestration command - it calls other commands
- Follow MBSE methodology: requirements → verifications → tests
- Each step uses specialized commands for consistency
- Run `reqvire coverage` at the end to confirm complete feature coverage
