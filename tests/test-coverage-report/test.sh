#!/usr/bin/env bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Test: Verification Coverage Report
# --------------------------------------
# Satisfies: specifications/Verifications/ReportsTests.md#verification-coverage-report-test
#
# Acceptance Criteria:
# - System shall provide a CLI command `coverage` that generates coverage reports focusing on leaf requirements
# - Legacy command `coverage-report` is also supported (deprecated)
# - Command shall support `--json` flag for JSON output format
# - Coverage report shall include summary section with total counts and percentages for leaf requirements
# - Coverage report shall show breakdown by verification type (test, formal-proof, analysis, inspection, demonstration)
# - Coverage report shall list verified leaf requirements grouped by file and section
# - Coverage report shall list unverified leaf requirements with details
# - Coverage report shall list satisfied test-verification elements (those with satisfiedBy relations)
# - Coverage report shall list unsatisfied test-verification elements (those without satisfiedBy relations)
# - Non-test-verification elements (analysis, inspection, demonstration) are considered satisfied by default
# - JSON output shall be valid and machine-readable
# - Text output shall be human-readable with clear formatting
#
# Test Criteria:
# - Command exits with success (0) return code
# - Basic coverage report contains expected sections and formatting for leaf requirements
# - JSON output is valid and contains required fields for both leaf requirements and test-verification elements
# - Coverage percentages are correctly calculated separately for leaf requirements and evidence-backed verifications

# Test 1: Basic Coverage Report (Text Output)
echo "Starting test..." > "${TEST_DIR}/test_results.log"

echo "Running: reqvire coverage" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" coverage 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: coverage command exited with code $EXIT_CODE"
    exit 1
fi

# Compare output with expected output
if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected_output.md" <(echo "$OUTPUT"); then
    echo "❌ FAILED: Coverage report output does not match expected output"
    echo ""
    echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/expected_output.md"
    exit 1
fi

# Test 2: JSON Coverage Report
echo "Running: reqvire coverage --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" coverage --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: coverage --json command exited with code $EXIT_CODE"
    exit 1
fi

# Validate JSON structure
echo "$OUTPUT" | jq . >/dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "❌ FAILED: Output is not valid JSON"
    exit 1
fi

# Check for required top-level fields
if ! echo "$OUTPUT" | jq 'has("summary")' | grep -q true; then
    echo "❌ FAILED: JSON missing 'summary' field"
    exit 1
fi

if ! echo "$OUTPUT" | jq 'has("verified_leaf_requirements")' | grep -q true; then
    echo "❌ FAILED: JSON missing 'verified_leaf_requirements' field"
    exit 1
fi

if ! echo "$OUTPUT" | jq 'has("unverified_leaf_requirements")' | grep -q true; then
    echo "❌ FAILED: JSON missing 'unverified_leaf_requirements' field"
    exit 1
fi

if ! echo "$OUTPUT" | jq 'has("satisfied_test_verifications")' | grep -q true; then
    echo "❌ FAILED: JSON missing 'satisfied_test_verifications' field"
    exit 1
fi

if ! echo "$OUTPUT" | jq 'has("unsatisfied_test_verifications")' | grep -q true; then
    echo "❌ FAILED: JSON missing 'unsatisfied_test_verifications' field"
    exit 1
fi

# Check summary structure - Leaf Requirements
if ! echo "$OUTPUT" | jq '.summary | has("total_leaf_requirements")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing 'total_leaf_requirements'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary | has("verified_leaf_requirements")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing 'verified_leaf_requirements'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary | has("unverified_leaf_requirements")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing 'unverified_leaf_requirements'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary | has("leaf_requirements_coverage_percentage")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing 'leaf_requirements_coverage_percentage'"
    exit 1
fi

# Check summary structure - Test Verifications
if ! echo "$OUTPUT" | jq '.summary | has("total_test_verifications")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing 'total_test_verifications'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary | has("satisfied_test_verifications")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing 'satisfied_test_verifications'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary | has("unsatisfied_test_verifications")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing 'unsatisfied_test_verifications'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary | has("test_verifications_satisfaction_percentage")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing 'test_verifications_satisfaction_percentage'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary | has("verification_types")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing 'verification_types'"
    exit 1
fi

# Check summary structure - Implementation Coverage
if ! echo "$OUTPUT" | jq '.summary | has("total_requirements_in_scope")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing 'total_requirements_in_scope'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary | has("covered_requirements")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing 'covered_requirements'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary | has("uncovered_requirements")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing 'uncovered_requirements'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary | has("implementation_coverage_percentage")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing 'implementation_coverage_percentage'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary | has("coverage_sources")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing 'coverage_sources'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary.coverage_sources | has("direct_satisfied")' | grep -q true; then
    echo "❌ FAILED: JSON summary coverage_sources missing 'direct_satisfied'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary.coverage_sources | has("contract_satisfied_via_reused_contract_context")' | grep -q true; then
    echo "❌ FAILED: JSON summary coverage_sources missing 'contract_satisfied_via_reused_contract_context'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary.coverage_sources | has("contract_satisfied_via_child")' | grep -q true; then
    echo "❌ FAILED: JSON summary coverage_sources missing 'contract_satisfied_via_child'"
    exit 1
fi

# Check verification_types structure
if ! echo "$OUTPUT" | jq '.summary.verification_types | has("test")' | grep -q true; then
    echo "❌ FAILED: JSON verification_types missing 'test'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary.verification_types | has("formal_proof")' | grep -q true; then
    echo "❌ FAILED: JSON verification_types missing 'formal_proof'"
    exit 1
fi

if [ "$(echo "$OUTPUT" | jq '.summary.verification_types.formal_proof')" -ne 2 ]; then
    echo "❌ FAILED: Expected 2 formal-proof verifications in verification type breakdown"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary.verification_types | has("analysis")' | grep -q true; then
    echo "❌ FAILED: JSON verification_types missing 'analysis'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary.verification_types | has("inspection")' | grep -q true; then
    echo "❌ FAILED: JSON verification_types missing 'inspection'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary.verification_types | has("demonstration")' | grep -q true; then
    echo "❌ FAILED: JSON verification_types missing 'demonstration'"
    exit 1
fi

# Check that all sections have 'files' field
if ! echo "$OUTPUT" | jq '.verified_leaf_requirements | has("files")' | grep -q true; then
    echo "❌ FAILED: JSON verified_leaf_requirements missing 'files'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.unverified_leaf_requirements | has("files")' | grep -q true; then
    echo "❌ FAILED: JSON unverified_leaf_requirements missing 'files'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.satisfied_test_verifications | has("files")' | grep -q true; then
    echo "❌ FAILED: JSON satisfied_test_verifications missing 'files'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.unsatisfied_test_verifications | has("files")' | grep -q true; then
    echo "❌ FAILED: JSON unsatisfied_test_verifications missing 'files'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.covered_requirements | has("files")' | grep -q true; then
    echo "❌ FAILED: JSON covered_requirements missing 'files'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.uncovered_requirements | has("files")' | grep -q true; then
    echo "❌ FAILED: JSON uncovered_requirements missing 'files'"
    exit 1
fi

if ! echo "$OUTPUT" | jq 'has("capability_coverage")' | grep -q true; then
    echo "❌ FAILED: JSON missing 'capability_coverage' field"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.capability_coverage | has("capabilities")' | grep -q true; then
    echo "❌ FAILED: JSON capability_coverage missing 'capabilities'"
    exit 1
fi

CAPABILITY_COUNT=$(echo "$OUTPUT" | jq '.capability_coverage.capabilities | length')
if [ "$CAPABILITY_COUNT" -ne 1 ]; then
    echo "❌ FAILED: Expected 1 capability coverage entry, got $CAPABILITY_COUNT"
    exit 1
fi

COVERAGE_CAPABILITY=$(echo "$OUTPUT" | jq '.capability_coverage.capabilities[] | select(.name == "Coverage Capability")')
if [ -z "$COVERAGE_CAPABILITY" ]; then
    echo "❌ FAILED: Missing Coverage Capability roll-up"
    exit 1
fi

if [ "$(echo "$COVERAGE_CAPABILITY" | jq '.aggregate_leaf_requirements')" -ne 5 ]; then
    echo "❌ FAILED: Coverage Capability should roll up 5 leaf requirements"
    exit 1
fi

if [ "$(echo "$COVERAGE_CAPABILITY" | jq '.aggregate_verified_leaf_requirements')" -ne 4 ]; then
    echo "❌ FAILED: Coverage Capability should roll up 4 verified leaf requirements"
    exit 1
fi

if [ "$(echo "$COVERAGE_CAPABILITY" | jq -r '.mark')" != "partial" ]; then
    echo "❌ FAILED: Coverage Capability mark should be partial"
    exit 1
fi

# Test 3: Coverage Calculation
# Leaf Requirements
TOTAL_LEAF=$(echo "$OUTPUT" | jq '.summary.total_leaf_requirements')
VERIFIED_LEAF=$(echo "$OUTPUT" | jq '.summary.verified_leaf_requirements')
UNVERIFIED_LEAF=$(echo "$OUTPUT" | jq '.summary.unverified_leaf_requirements')
LEAF_PERCENTAGE=$(echo "$OUTPUT" | jq '.summary.leaf_requirements_coverage_percentage')

# Test Verifications
TOTAL_TEST=$(echo "$OUTPUT" | jq '.summary.total_test_verifications')
SATISFIED_TEST=$(echo "$OUTPUT" | jq '.summary.satisfied_test_verifications')
UNSATISFIED_TEST=$(echo "$OUTPUT" | jq '.summary.unsatisfied_test_verifications')
TEST_PERCENTAGE=$(echo "$OUTPUT" | jq '.summary.test_verifications_satisfaction_percentage')

# Verify that total_leaf = verified_leaf + unverified_leaf
EXPECTED_TOTAL_LEAF=$((VERIFIED_LEAF + UNVERIFIED_LEAF))
if [ "$TOTAL_LEAF" -ne "$EXPECTED_TOTAL_LEAF" ]; then
    echo "❌ FAILED: Total leaf ($TOTAL_LEAF) != Verified leaf ($VERIFIED_LEAF) + Unverified leaf ($UNVERIFIED_LEAF)"
    exit 1
fi

# Verify that total_test = satisfied_test + unsatisfied_test
EXPECTED_TOTAL_TEST=$((SATISFIED_TEST + UNSATISFIED_TEST))
if [ "$TOTAL_TEST" -ne "$EXPECTED_TOTAL_TEST" ]; then
    echo "❌ FAILED: Total test ($TOTAL_TEST) != Satisfied test ($SATISFIED_TEST) + Unsatisfied test ($UNSATISFIED_TEST)"
    exit 1
fi

# Verify leaf requirements percentage calculation
if [ "$TOTAL_LEAF" -gt 0 ]; then
    EXPECTED_LEAF_PERCENTAGE=$(echo "scale=2; $VERIFIED_LEAF * 100 / $TOTAL_LEAF" | bc)
    ACTUAL_LEAF_FORMATTED=$(printf "%.1f" "$LEAF_PERCENTAGE")
    EXPECTED_LEAF_FORMATTED=$(printf "%.1f" "$EXPECTED_LEAF_PERCENTAGE")

    if [ "$ACTUAL_LEAF_FORMATTED" != "$EXPECTED_LEAF_FORMATTED" ]; then
        echo "❌ FAILED: Leaf requirements coverage percentage ($ACTUAL_LEAF_FORMATTED) != expected ($EXPECTED_LEAF_FORMATTED)"
        exit 1
    fi
fi

# Verify test verifications percentage calculation
if [ "$TOTAL_TEST" -gt 0 ]; then
    EXPECTED_TEST_PERCENTAGE=$(echo "scale=2; $SATISFIED_TEST * 100 / $TOTAL_TEST" | bc)
    ACTUAL_TEST_FORMATTED=$(printf "%.1f" "$TEST_PERCENTAGE")
    EXPECTED_TEST_FORMATTED=$(printf "%.1f" "$EXPECTED_TEST_PERCENTAGE")

    if [ "$ACTUAL_TEST_FORMATTED" != "$EXPECTED_TEST_FORMATTED" ]; then
        echo "❌ FAILED: Test verifications satisfaction percentage ($ACTUAL_TEST_FORMATTED) != expected ($EXPECTED_TEST_FORMATTED)"
        exit 1
    fi
fi

# Test 4: Expected Counts Validation

# Based on our test specifications, we expect:
# LEAF REQUIREMENTS:
# - 5 total leaf requirements
# - 4 verified
# - 1 unverified (Leaf Requirement Unverified)
# - 2 total test verifications and 2 total formal-proof verifications are evidence-backed
EXPECTED_TOTAL_LEAF=5
EXPECTED_VERIFIED_LEAF=4
EXPECTED_UNVERIFIED_LEAF=1

# EVIDENCE-BACKED VERIFICATIONS:
# - 4 total evidence-backed verifications (2 test, 2 formal-proof)
# - 2 satisfied (one test, one formal-proof)
# - 2 unsatisfied (one test, one formal-proof)
EXPECTED_TOTAL_TEST=4
EXPECTED_SATISFIED_TEST=2
EXPECTED_UNSATISFIED_TEST=2

if [ "$TOTAL_LEAF" -ne "$EXPECTED_TOTAL_LEAF" ]; then
    echo "❌ FAILED: Expected $EXPECTED_TOTAL_LEAF total leaf requirements, got $TOTAL_LEAF"
    exit 1
fi

if [ "$VERIFIED_LEAF" -ne "$EXPECTED_VERIFIED_LEAF" ]; then
    echo "❌ FAILED: Expected $EXPECTED_VERIFIED_LEAF verified leaf requirements, got $VERIFIED_LEAF"
    exit 1
fi

if [ "$UNVERIFIED_LEAF" -ne "$EXPECTED_UNVERIFIED_LEAF" ]; then
    echo "❌ FAILED: Expected $EXPECTED_UNVERIFIED_LEAF unverified leaf requirements, got $UNVERIFIED_LEAF"
    exit 1
fi

if [ "$TOTAL_TEST" -ne "$EXPECTED_TOTAL_TEST" ]; then
    echo "❌ FAILED: Expected $EXPECTED_TOTAL_TEST total test verifications, got $TOTAL_TEST"
    exit 1
fi

if [ "$SATISFIED_TEST" -ne "$EXPECTED_SATISFIED_TEST" ]; then
    echo "❌ FAILED: Expected $EXPECTED_SATISFIED_TEST satisfied test verifications, got $SATISFIED_TEST"
    exit 1
fi

if [ "$UNSATISFIED_TEST" -ne "$EXPECTED_UNSATISFIED_TEST" ]; then
    echo "❌ FAILED: Expected $EXPECTED_UNSATISFIED_TEST unsatisfied test verifications, got $UNSATISFIED_TEST"
    exit 1
fi

# Validate counts in detailed sections match summary
DETAILED_VERIFIED_LEAF_COUNT=$(echo "$OUTPUT" | jq '[.verified_leaf_requirements.files | to_entries[] | .value[] ] | length')
DETAILED_UNVERIFIED_LEAF_COUNT=$(echo "$OUTPUT" | jq '[.unverified_leaf_requirements.files | to_entries[] | .value[] ] | length')
DETAILED_SATISFIED_TEST_COUNT=$(echo "$OUTPUT" | jq '[.satisfied_test_verifications.files | to_entries[] | .value[] ] | length')
DETAILED_UNSATISFIED_TEST_COUNT=$(echo "$OUTPUT" | jq '[.unsatisfied_test_verifications.files | to_entries[] | .value[] ] | length')

if [ "$DETAILED_VERIFIED_LEAF_COUNT" -ne "$VERIFIED_LEAF" ]; then
    echo "❌ FAILED: Detailed verified leaf count ($DETAILED_VERIFIED_LEAF_COUNT) != summary count ($VERIFIED_LEAF)"
    exit 1
fi

if [ "$DETAILED_UNVERIFIED_LEAF_COUNT" -ne "$UNVERIFIED_LEAF" ]; then
    echo "❌ FAILED: Detailed unverified leaf count ($DETAILED_UNVERIFIED_LEAF_COUNT) != summary count ($UNVERIFIED_LEAF)"
    exit 1
fi

if [ "$DETAILED_SATISFIED_TEST_COUNT" -ne "$SATISFIED_TEST" ]; then
    echo "❌ FAILED: Detailed satisfied test count ($DETAILED_SATISFIED_TEST_COUNT) != summary count ($SATISFIED_TEST)"
    exit 1
fi

if [ "$DETAILED_UNSATISFIED_TEST_COUNT" -ne "$UNSATISFIED_TEST" ]; then
    echo "❌ FAILED: Detailed unsatisfied test count ($DETAILED_UNSATISFIED_TEST_COUNT) != summary count ($UNSATISFIED_TEST)"
    exit 1
fi

# Test 5: Element Details

# Check satisfied test verification structure
if [ "$SATISFIED_TEST" -gt 0 ]; then
    # Get first satisfied test verification
    FIRST_SATISFIED_TEST=$(echo "$OUTPUT" | jq '[.satisfied_test_verifications.files | to_entries[] | .value[] ] | .[0]')

    # Check required fields
    if ! echo "$FIRST_SATISFIED_TEST" | jq 'has("identifier")' | grep -q true; then
        echo "❌ FAILED: Satisfied test verification missing 'identifier'"
        exit 1
    fi

    if ! echo "$FIRST_SATISFIED_TEST" | jq 'has("name")' | grep -q true; then
        echo "❌ FAILED: Satisfied test verification missing 'name'"
        exit 1
    fi

    if ! echo "$FIRST_SATISFIED_TEST" | jq 'has("verification_type")' | grep -q true; then
        echo "❌ FAILED: Satisfied test verification missing 'verification_type'"
        exit 1
    fi

    if ! echo "$FIRST_SATISFIED_TEST" | jq 'has("satisfied_by")' | grep -q true; then
        echo "❌ FAILED: Satisfied test verification missing 'satisfied_by'"
        exit 1
    fi

    # Satisfied test verifications should have non-empty satisfied_by
    SATISFIED_BY_COUNT=$(echo "$FIRST_SATISFIED_TEST" | jq '.satisfied_by | length')
    if [ "$SATISFIED_BY_COUNT" -eq 0 ]; then
        echo "❌ FAILED: Satisfied test verification has empty 'satisfied_by' array"
        exit 1
    fi
fi

# Check unsatisfied test verification structure
if [ "$UNSATISFIED_TEST" -gt 0 ]; then
    # Get first unsatisfied test verification
    FIRST_UNSATISFIED_TEST=$(echo "$OUTPUT" | jq '[.unsatisfied_test_verifications.files | to_entries[] | .value[] ] | .[0]')

    # Check required fields
    if ! echo "$FIRST_UNSATISFIED_TEST" | jq 'has("identifier")' | grep -q true; then
        echo "❌ FAILED: Unsatisfied test verification missing 'identifier'"
        exit 1
    fi

    if ! echo "$FIRST_UNSATISFIED_TEST" | jq 'has("satisfied_by")' | grep -q true; then
        echo "❌ FAILED: Unsatisfied test verification missing 'satisfied_by'"
        exit 1
    fi

    # Unsatisfied test verifications should have empty satisfied_by
    SATISFIED_BY_COUNT=$(echo "$FIRST_UNSATISFIED_TEST" | jq '.satisfied_by | length')
    if [ "$SATISFIED_BY_COUNT" -ne 0 ]; then
        echo "❌ FAILED: Unsatisfied test verification has non-empty 'satisfied_by' array"
        exit 1
    fi
fi

# Check verified leaf requirement structure
if [ "$VERIFIED_LEAF" -gt 0 ]; then
    # Get first verified leaf requirement
    FIRST_VERIFIED_LEAF=$(echo "$OUTPUT" | jq '[.verified_leaf_requirements.files | to_entries[] | .value[] ] | .[0]')

    # Check required fields
    if ! echo "$FIRST_VERIFIED_LEAF" | jq 'has("identifier")' | grep -q true; then
        echo "❌ FAILED: Verified leaf requirement missing 'identifier'"
        exit 1
    fi

    if ! echo "$FIRST_VERIFIED_LEAF" | jq 'has("name")' | grep -q true; then
        echo "❌ FAILED: Verified leaf requirement missing 'name'"
        exit 1
    fi

    if ! echo "$FIRST_VERIFIED_LEAF" | jq 'has("verified_by")' | grep -q true; then
        echo "❌ FAILED: Verified leaf requirement missing 'verified_by'"
        exit 1
    fi

    # Verified leaf requirements should have non-empty verified_by
    VERIFIED_BY_COUNT=$(echo "$FIRST_VERIFIED_LEAF" | jq '.verified_by | length')
    if [ "$VERIFIED_BY_COUNT" -eq 0 ]; then
        echo "❌ FAILED: Verified leaf requirement has empty 'verified_by' array"
        exit 1
    fi
fi

exit 0
