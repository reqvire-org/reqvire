---
allowed-tools: Read, Write, Edit, Bash(reqvire:*)
argument-hint: [requirement-id]
description: Add a verification for an existing requirement, checking if verification is needed based on requirement hierarchy
model: claude-sonnet-4-5-20250929
---

# Add Verification

Add a verification for an existing requirement following Reqvire's bottom roll-up verification philosophy.

## Current Model Context

- Total verifications: !`reqvire summary --json | jq -r '.global_counters.verifications'`
- Verification coverage: !`reqvire coverage --json | jq -r '.summary.leaf_requirements_coverage_percentage'`%
- Unverified leaf requirements: !`reqvire coverage --json | jq -r '.summary.unverified_leaf_requirements'`

## User Request

${1:+Requirement ID: $1}
${1:-The user will specify which requirement needs verification.}

## Steps

1. **Identify the requirement:**
   - Ask user which requirement needs verification if not provided
   - Get the requirement identifier or name

2. **Check if verification is needed:**
   ```bash
   reqvire traces --filter-name="<requirement-name>"
   ```

   Analyze the trace tree:
   - **Leaf requirement** (no children): Needs direct verification
   - **Parent requirement** (has children): Verification rolls up from children - usually no direct verification needed

3. **Check current coverage:**
   ```bash
   reqvire coverage --filter-name="<requirement-name>"
   ```

4. **If leaf requirement needs verification:**

   Choose verification type:
   - **verification** (or test-verification): Automated testing
   - **analysis-verification**: Mathematical/computational analysis
   - **inspection-verification**: Manual inspection/review
   - **demonstration-verification**: Operational demonstration

5. **Read all requirements in trace chain:**

   For each requirement this verification will verify:
   ```bash
   reqvire summary --filter-id="<requirement-id>"
   ```

   Extract:
   - Requirement content (capabilities and constraints)
   - All requirements in derivedFrom chain up to root
   - Build complete understanding of what needs verification

6. **Create verification with comprehensive test criteria:**

   In `specifications/Verifications/` directory:
   ```markdown
   ### Verification Name

   [Description of how ALL requirements in the trace chain will be verified]

   Test criteria must cover:
   - [Criterion for leaf requirement 1]
   - [Criterion for leaf requirement 2]
   - [Criterion that verifies parent capabilities through leaf tests]

   #### Metadata
     * type: verification

   #### Relations
     * verify: [Leaf Requirement 1](../path/to/req1.md#leaf-requirement-1)
     * verify: [Leaf Requirement 2](../path/to/req2.md#leaf-requirement-2)
   ```

7. **Link test (only for test-verification):**

   If verification type is `verification` or `test-verification` AND test file exists:
   ```markdown
   #### Relations
     * verify: [Requirement Name](../path/to/requirement.md#requirement-name)
     * satisfiedBy: [tests/test-name/test.sh](../../tests/test-name/test.sh)
   ```

8. **Update the requirements:**
   Add `verifiedBy` relation to each verified requirement:
   ```markdown
   #### Relations
     * derivedFrom: [Parent](...)
     * verifiedBy: [Verification Name](../Verifications/file.md#verification-name)
   ```

9. **Validate:**
   ```bash
   reqvire validate
   ```

10. **Check updated coverage:**
    ```bash
    reqvire coverage --filter-name="<requirement-name>"
    ```

11. **Verify roll-up and check for redundancies:**
    ```bash
    reqvire traces --filter-name="<verification-name>"
    reqvire lint --json > /tmp/lint.json
    ```

    Check if verification creates redundant verify relations (verifying both leaf and parent).

## Decision Logic

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
- Read all requirements currently verified
- Read all requirements in their trace chains
- Update test criteria to cover all requirements comprehensively

## Best Practices

- **Read trace chain**: Always read full requirement hierarchy to understand scope
- **Comprehensive criteria**: Test criteria must cover all verified requirements
- **Verify leaf requirements**: Focus on leaf-level verification
- **Roll-up coverage**: Parent requirements inherit from children
- **Avoid redundancy**: Don't verify both leaf and parent directly
- **Use traces**: Run `reqvire traces` to understand verification structure
- **Test links for test-verification only**: Only test-verifications link to test files

## Verification Types

- **test-verification**: Links to automated test files via satisfiedBy
- **analysis-verification**: No test linkage, verified through analysis
- **inspection-verification**: No test linkage, verified through manual inspection
- **demonstration-verification**: No test linkage, verified through demonstration

## Verification Philosophy

Reqvire uses **bottom roll-up verification**:
1. Verify leaf requirements directly
2. Parent requirements inherit coverage from children
3. One verification can verify multiple leaf requirements
4. Verification traces automatically propagate upward
5. Test criteria must cover ALL requirements in the trace chain

## Notes

- Verifications go in `specifications/Verifications/` directory
- Use two-space indentation for Relations entries
- Always read full trace chain before writing test criteria
- Run `reqvire lint --fix` after adding to remove redundancies
- Check `reqvire coverage` to confirm improvement
