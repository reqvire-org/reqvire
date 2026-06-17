---
allowed-tools: Read, Edit, Bash(npx:*)
argument-hint: [capability-or-requirement-id]
description: Add a verification for an existing capability or requirement, checking scope against capability and requirement hierarchy
model: claude-sonnet-4-5
---

# Add Verification

Add a verification for an existing capability or requirement following Reqvire's direct capability verification and requirement roll-up philosophy.

## Current Model Context

- Total verifications: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.global_counters.verifications'`
- Verification coverage: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '.summary.leaf_requirements_coverage_percentage'`%
- Unverified leaf requirements: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '.summary.unverified_leaf_requirements'`

## User Request

${1:+Capability or requirement ID: $1}
${1:-The user will specify which capability or requirement needs verification.}

## Steps

1. **Identify the capability or requirement:**
   - Ask user which capability or requirement needs verification if not provided
   - Get the element identifier or name

2. **Check if verification is needed:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" traces --filter-name="<requirement-name>"
   ```

   Analyze the trace tree:
   - **Capability**: May be directly verified when evidence is capability-level
   - **Leaf requirement** (no children): Needs direct verification
   - **Parent requirement** (has children): Verification rolls up from children - usually no direct verification needed

3. **Check current coverage:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --filter-name="<requirement-name>"
   ```

4. **If capability or leaf requirement needs verification:**

   Choose verification type:
   - **verification-objective**: Mandatory planning/grouping parent for concrete verification work. Use only to organize concrete verification elements through `derivedFrom`; do not add `verify`, `verifiedBy`, or `satisfiedBy`.
   - **verification** (or test-verification): Automated testing
   - **analysis-verification**: Mathematical/computational analysis
   - **inspection-verification**: Manual inspection/review
   - **demonstration-verification**: Operational demonstration

5. **Read all capability and requirement context in trace chain:**

   For each capability or requirement this verification will verify:
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-id="<requirement-id>"
   ```

   Extract:
   - Capability or requirement content
   - Ontology/refinement context reachable from the capability or requirement
   - All requirements in derivedFrom chain up to capability context
   - Build complete understanding of what needs verification

6. **Draft verification content:**

   Template for verification:
   ```markdown
   ### Verification Name

   [Description of how the capability or all requirements in the trace chain will be verified]

   #### Details

   ##### Acceptance Criteria
   - [Criterion for leaf requirement 1]
   - [Criterion for leaf requirement 2]
   - [Criterion that verifies capability expectations or parent requirements through leaf tests]

   ##### Test Criteria
   - [How to test criterion 1]
   - [How to test criterion 2]
   - [Expected outcomes]

   #### Metadata
     * type: test-verification

   #### Relations
     * derivedFrom: [Verification Objective](../path/to/verifications.md#verification-objective)
     * verify: [Capability Or Leaf Requirement](../path/to/element.md#capability-or-leaf-requirement)
     * satisfiedBy: [test.sh](../../tests/test-name/test.sh)
   ```

   Note: Every concrete verification must have a `derivedFrom` relation to a `verification-objective` parent. Only evidence-backed concrete verification types (`test-verification` and `formal-proof-verification`) can have satisfiedBy relations. `verification-objective` cannot verify requirements/capabilities and cannot have satisfiedBy evidence.

7. **Add verification using reqvire add command:**

   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" add "requirements/Verifications/<file>.md" <<'EOF'
   ### Verification Name

   [Description of verification approach]

   #### Details

   ##### Acceptance Criteria
   - [What must be satisfied]
   - [Functional criteria]

   ##### Test Criteria
   - [How to verify]
   - [Expected behavior]

   #### Metadata
     * type: test-verification

   #### Relations
     * verify: [Requirement](../path.md#requirement)
   EOF
   ```

   Optional: Insert at specific position (0-based index):
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" add "requirements/Verifications/<file>.md" 0 <<'EOF'
   ...
   EOF
   ```

   Alternative using pipe:
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" add "requirements/Verifications/<file>.md" < element.md
   ```

   The add command automatically:
   - Validates markdown format
   - Checks element name uniqueness
   - Validates relation format
   - Updates the file

8. **Update the verified elements with verifiedBy relations:**
   Add `verifiedBy` relation to each verified capability or requirement:
   ```markdown
   #### Relations
     * verifiedBy: [Verification Name](../Verifications/file.md#verification-name)
   ```

9. **Check updated coverage:**
    ```bash
    npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --filter-name="<requirement-name>"
    ```

10. **Verify roll-up and check for redundancies:**
    ```bash
    npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" traces --filter-name="<verification-name>"
    npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --json --output /tmp/lint.json
    ```

    Check if verification creates redundant verify relations (verifying both an element and an ancestor already covered through the trace path).

## Element Manipulation

After adding verifications, you may need to reorganize:

**Move verification to different file:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv "<verification-name>" "requirements/Verifications/<file>.md"
```

**Move verification with specific position (0-based index):**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv "<verification-name>" "<target-file>" 0
```

**Remove verification:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" rm "<verification-name>"
```

## Decision Logic

**If capability target:**
- Verify directly only when evidence is about the capability itself, not just one implementation obligation
- Otherwise verify leaf requirements and let capability coverage roll up

**If parent requirement with children:**
- Explain verification rolls up from children
- Show trace tree demonstrating coverage
- Usually no direct verification needed

**If leaf requirement without verification:**
- Read ALL requirements in trace chain
- Create verification with test criteria covering entire chain
- Link to requirement(s)
- Add test linkage ONLY if type is test-verification AND test exists

**If existing verification needs update:**
- Read all capabilities or requirements currently verified
- Read all relevant trace chains
- Update test criteria to cover all verified scope comprehensively

## Best Practices

- **Read trace chain**: Always read full capability/requirement hierarchy to understand scope
- **Comprehensive criteria**: Test criteria must cover all verified capabilities or requirements
- **Verify leaf requirements by default**: Focus on leaf-level requirement verification unless evidence is capability-level
- **Direct capability verification**: Use when verification evidence proves a capability expectation directly
- **Roll-up coverage**: Parent requirements inherit from children
- **Avoid redundancy**: Don't verify both leaf and parent directly
- **Use traces**: Run `reqvire traces` to understand verification structure
- **Test links for test-verification only**: Only test-verifications link to test files
- **Verification objectives are mandatory planning parents**: Use `verification-objective` to group concrete verifications. Every concrete verification derives from one. It never uses `verify`, `verifiedBy`, or `satisfiedBy`.

## Verification Types

- **test-verification**: Links to automated test files via satisfiedBy
- **verification-objective**: Mandatory parent that groups verification intent and concrete verification work; no direct verify/satisfiedBy
- **analysis-verification**: No test linkage, verified through analysis
- **inspection-verification**: No test linkage, verified through manual inspection
- **demonstration-verification**: No test linkage, verified through demonstration

## Verification Philosophy

Reqvire uses **bottom roll-up verification**:
1. Verify capabilities directly when evidence is capability-level
2. Verify leaf requirements directly for requirement roll-up
3. Parent requirements inherit coverage from children
4. One verification can verify multiple capabilities or leaf requirements
5. Verification traces automatically propagate upward
6. Test criteria must cover all verified scope in the trace chain

## Notes

- Verifications go in `requirements/Verifications/` directory
- Use two-space indentation for Relations entries
- Always read full trace chain before writing test criteria
- Run `reqvire lint --fix` after adding to remove redundancies
- Check `reqvire coverage` to confirm improvement
