# Add Verification

Add a verification for an existing requirement following Reqvire's requirement verification coverage and requirement roll-up philosophy. If the user names a capability, use it to find the requirements that specify that capability; do not make the capability the `verify` target.

## Steps

1. **Check current verification state:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.global_counters.verifications'
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json | jq -r '{coverage: .summary.leaf_requirements_coverage_percentage, unverified: .summary.unverified_leaf_requirements}'
   ```

2. **Identify the requirement scope:**
   - Ask user which capability or requirement scope needs verification if not provided
   - Get the element identifier or name
   - If a capability is named, collect the requirements that specify it and choose the leaf requirements that need direct verification

3. **Check if verification is needed:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" traces --filter-name="<requirement-name>"
   ```

   Analyze the trace tree:
   - **Capability**: Not a direct verification target; inspect its specifying requirements
   - **Leaf requirement** (no children): Needs direct verification
   - **Parent requirement** (has children): Verification rolls up from children - usually no direct verification needed

4. **Check current coverage:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --filter-name="<requirement-name>"
   ```

5. **If a leaf requirement needs verification:**

   Choose verification type:
   - **verification-objective**: Mandatory planning/grouping parent for concrete verification work. Use only to organize concrete verification elements through `derivedFrom`; do not add `verify`, `verifiedBy`, or `satisfiedBy`.
   - **test-verification**: Automated testing (can have `satisfiedBy` to test code)
   - **analysis-verification**: Mathematical/computational analysis
   - **inspection-verification**: Manual inspection/review
   - **demonstration-verification**: Operational demonstration

6. **Read all capability and requirement context in trace chain:**

   For each requirement this verification will verify:
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-id="<requirement-id>"
   ```

   Extract:
   - Requirement content and owning capability context
   - Ontology/contract context reachable from the requirement and owning capability
   - All requirements in derivedFrom chain up to capability context
   - Build complete understanding of what needs verification

7. **Draft verification content:**

   Template for verification:
   ```markdown
   ### Verification Name

   [Description of how the requirements in the trace chain will be verified]

   #### Details

   ##### Acceptance Criteria
   - [Criterion for leaf requirement 1]
   - [Criterion for leaf requirement 2]
   - [Criterion that verifies parent requirement expectations through leaf tests]

   ##### Test Criteria
   - [How to test criterion 1]
   - [How to test criterion 2]
   - [Expected outcomes]

   #### Metadata
     * type: test-verification

   #### Relations
     * derivedFrom: [Verification Objective](../path/to/verifications.md#verification-objective)
     * verify: [Leaf Requirement](../path/to/element.md#leaf-requirement)
     * satisfiedBy: [test.sh](../../tests/test-name/test.sh)
   ```

   Note: Every concrete verification must have a `derivedFrom` relation to a `verification-objective` parent. Only evidence-backed concrete verification types (`test-verification` and `formal-proof-verification`) can have satisfiedBy relations. `verification-objective` cannot verify requirements and cannot have satisfiedBy evidence.

8. **Add verification using reqvire add command:**

   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" add "system-model/Verifications/<file>.md" <<'EOF'
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
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" add "system-model/Verifications/<file>.md" 0 <<'EOF'
   ...
   EOF
   ```

   The add command automatically:
   - Validates markdown format
   - Checks element name uniqueness
   - Validates relation format
   - Updates the file

9. **Update the verified requirements with verifiedBy relations:**
   Add `verifiedBy` relation to each verified requirement:
   ```markdown
   #### Relations
     * verifiedBy: [Verification Name](../Verifications/file.md#verification-name)
   ```

10. **Check updated coverage:**
    ```bash
    npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --filter-name="<requirement-name>"
    ```

11. **Verify roll-up and check for redundancies:**
    ```bash
    npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" traces --filter-name="<verification-name>"
    npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --json --output /tmp/lint.json
    ```

    Check if verification creates redundant verify relations (verifying both an element and an ancestor already covered through the trace path).

## Element Manipulation

After adding verifications, you may need to reorganize. See [Move](Move.md) and [Remove](Remove.md).

## Decision Logic

**If capability scope:**
- Do not target the capability with `verify`
- Identify the requirements that specify the capability
- Verify the most precise applicable requirements and let capability coverage roll up

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
- Read all requirements currently verified and their owning capability context
- Read all relevant trace chains
- Update test criteria to cover all verified scope comprehensively

## Best Practices

- **Read trace chain**: Always read full capability/requirement hierarchy to understand scope
- **Comprehensive criteria**: Test criteria must cover all verified requirements
- **Verify leaf requirements by default**: Focus on leaf-level requirement verification and let capability coverage roll up
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
1. Verify leaf requirements directly for requirement roll-up
2. Parent requirements inherit coverage from children
3. Capabilities receive coverage from verified requirements that specify them
4. One verification can verify multiple leaf requirements
5. Verification traces automatically propagate upward
6. Test criteria must cover all verified requirement scope in the trace chain

## Notes

- Verifications go in `system-model/Verifications/` directory
- Use two-space indentation for Relations entries
- Always read full trace chain before writing test criteria
- Run `reqvire lint --fix` after adding to remove redundancies
- Check `reqvire coverage` to confirm improvement
