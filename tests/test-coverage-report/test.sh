#!/usr/bin/env bash
set -euo pipefail

# Test: Verification Coverage Report
# --------------------------------------
# Satisfies: specifications/Verifications/ReportsTests.md#verification-coverage-report-test
#
# Acceptance Criteria:
# - System shall provide a CLI command `coverage-report` that generates coverage reports
# - Command shall support `--json` flag for JSON output format
# - Coverage report shall include summary section with total counts and percentages
# - Coverage report shall show breakdown by verification type (test, analysis, inspection, demonstration)
# - Coverage report shall list satisfied verifications grouped by file and section
# - Coverage report shall list unsatisfied verifications with details
# - JSON output shall be valid and machine-readable
# - Text output shall be human-readable with clear formatting
#
# Test Criteria:
# - Command exits with success (0) return code
# - Basic coverage report contains expected sections and formatting
# - JSON output is valid and contains required fields
# - Coverage percentage is correctly calculated

# Test 1: Basic Coverage Report (Text Output)
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" coverage-report 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: coverage-report command exited with code $EXIT_CODE"
    exit 1
fi

# Check for expected header
if ! grep -q "=== Verification Coverage Report ===" <<< "$OUTPUT"; then
    echo "❌ FAILED: Missing coverage report header"
    exit 1
fi

# Check for Summary section
if ! grep -q "Summary:" <<< "$OUTPUT"; then
    echo "❌ FAILED: Missing Summary section"
    exit 1
fi

# Check for required summary fields
if ! grep -q "Total Verifications:" <<< "$OUTPUT"; then
    echo "❌ FAILED: Missing 'Total Verifications' in summary"
    exit 1
fi

if ! grep -q "Satisfied:" <<< "$OUTPUT"; then
    echo "❌ FAILED: Missing 'Satisfied' count in summary"
    exit 1
fi

if ! grep -q "Unsatisfied:" <<< "$OUTPUT"; then
    echo "❌ FAILED: Missing 'Unsatisfied' count in summary"
    exit 1
fi

# Check for Verification Types breakdown
if ! grep -q "Verification Types:" <<< "$OUTPUT"; then
    echo "❌ FAILED: Missing 'Verification Types' section"
    exit 1
fi

if ! grep -q "Test:" <<< "$OUTPUT"; then
    echo "❌ FAILED: Missing 'Test' verification type"
    exit 1
fi

if ! grep -q "Analysis:" <<< "$OUTPUT"; then
    echo "❌ FAILED: Missing 'Analysis' verification type"
    exit 1
fi

if ! grep -q "Inspection:" <<< "$OUTPUT"; then
    echo "❌ FAILED: Missing 'Inspection' verification type"
    exit 1
fi

if ! grep -q "Demonstration:" <<< "$OUTPUT"; then
    echo "❌ FAILED: Missing 'Demonstration' verification type"
    exit 1
fi

# Check for sections for satisfied/unsatisfied verifications
# At least one should be present
if ! grep -q "Satisfied Verifications:\|Unsatisfied Verifications:" <<< "$OUTPUT"; then
    echo "❌ FAILED: Missing Satisfied/Unsatisfied Verifications sections"
    exit 1
fi

# Check for visual indicators (✅ for satisfied, ❌ for unsatisfied)
if grep -q "Satisfied Verifications:" <<< "$OUTPUT"; then
    # If we have satisfied verifications section, it should contain ✅
    if ! grep -A 10 "Satisfied Verifications:" <<< "$OUTPUT" | grep -q "✅"; then
        echo "❌ FAILED: Satisfied verifications should be marked with ✅"
        exit 1
    fi
fi

if grep -q "Unsatisfied Verifications:" <<< "$OUTPUT"; then
    # If we have unsatisfied verifications section, it should contain ❌
    if ! grep -A 10 "Unsatisfied Verifications:" <<< "$OUTPUT" | grep -q "❌"; then
        echo "❌ FAILED: Unsatisfied verifications should be marked with ❌"
        exit 1
    fi
fi

# Test 2: JSON Coverage Report
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" coverage-report --json 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: coverage-report --json command exited with code $EXIT_CODE"
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

if ! echo "$OUTPUT" | jq 'has("satisfied_verifications")' | grep -q true; then
    echo "❌ FAILED: JSON missing 'satisfied_verifications' field"
    exit 1
fi

if ! echo "$OUTPUT" | jq 'has("unsatisfied_verifications")' | grep -q true; then
    echo "❌ FAILED: JSON missing 'unsatisfied_verifications' field"
    exit 1
fi

# Check summary structure
if ! echo "$OUTPUT" | jq '.summary | has("total_verifications")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing 'total_verifications'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary | has("total_satisfied")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing 'total_satisfied'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary | has("total_unsatisfied")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing 'total_unsatisfied'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary | has("coverage_percentage")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing 'coverage_percentage'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary | has("verification_types")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing 'verification_types'"
    exit 1
fi

# Check verification_types structure
if ! echo "$OUTPUT" | jq '.summary.verification_types | has("test")' | grep -q true; then
    echo "❌ FAILED: JSON verification_types missing 'test'"
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

# Check that satisfied_verifications and unsatisfied_verifications have 'files' field
if ! echo "$OUTPUT" | jq '.satisfied_verifications | has("files")' | grep -q true; then
    echo "❌ FAILED: JSON satisfied_verifications missing 'files'"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.unsatisfied_verifications | has("files")' | grep -q true; then
    echo "❌ FAILED: JSON unsatisfied_verifications missing 'files'"
    exit 1
fi

# Test 3: Coverage Calculation
TOTAL=$(echo "$OUTPUT" | jq '.summary.total_verifications')
SATISFIED=$(echo "$OUTPUT" | jq '.summary.total_satisfied')
UNSATISFIED=$(echo "$OUTPUT" | jq '.summary.total_unsatisfied')
PERCENTAGE=$(echo "$OUTPUT" | jq '.summary.coverage_percentage')

# Verify that total = satisfied + unsatisfied
EXPECTED_TOTAL=$((SATISFIED + UNSATISFIED))
if [ "$TOTAL" -ne "$EXPECTED_TOTAL" ]; then
    echo "❌ FAILED: Total ($TOTAL) != Satisfied ($SATISFIED) + Unsatisfied ($UNSATISFIED)"
    exit 1
fi

# Verify percentage calculation (allow for floating point)
if [ "$TOTAL" -gt 0 ]; then
    EXPECTED_PERCENTAGE=$(echo "scale=2; $SATISFIED * 100 / $TOTAL" | bc)
    # Compare as strings after formatting to 1 decimal place
    ACTUAL_FORMATTED=$(printf "%.1f" "$PERCENTAGE")
    EXPECTED_FORMATTED=$(printf "%.1f" "$EXPECTED_PERCENTAGE")
    
    if [ "$ACTUAL_FORMATTED" != "$EXPECTED_FORMATTED" ]; then
        echo "❌ FAILED: Coverage percentage ($ACTUAL_FORMATTED) != expected ($EXPECTED_FORMATTED)"
        exit 1
    fi
fi

# Test 4: Expected Counts Validation

# Based on our test specifications, we expect:
# - 3 total verifications (Test Verification Satisfied, Test Verification Unsatisfied, Analysis Verification Test)
# - 2 satisfied (Test Verification Satisfied, Analysis Verification Test)  
# - 1 unsatisfied (Test Verification Unsatisfied)
EXPECTED_TOTAL=3
EXPECTED_SATISFIED=2
EXPECTED_UNSATISFIED=1

if [ "$TOTAL" -ne "$EXPECTED_TOTAL" ]; then
    echo "❌ FAILED: Expected $EXPECTED_TOTAL total verifications, got $TOTAL"
    exit 1
fi

if [ "$SATISFIED" -ne "$EXPECTED_SATISFIED" ]; then
    echo "❌ FAILED: Expected $EXPECTED_SATISFIED satisfied verifications, got $SATISFIED"
    exit 1
fi

if [ "$UNSATISFIED" -ne "$EXPECTED_UNSATISFIED" ]; then
    echo "❌ FAILED: Expected $EXPECTED_UNSATISFIED unsatisfied verifications, got $UNSATISFIED"
    exit 1
fi

# Validate counts in detailed sections match summary
DETAILED_SATISFIED_COUNT=$(echo "$OUTPUT" | jq '[.satisfied_verifications.files | to_entries[] | .value[] ] | length')
DETAILED_UNSATISFIED_COUNT=$(echo "$OUTPUT" | jq '[.unsatisfied_verifications.files | to_entries[] | .value[] ] | length')

if [ "$DETAILED_SATISFIED_COUNT" -ne "$SATISFIED" ]; then
    echo "❌ FAILED: Detailed satisfied count ($DETAILED_SATISFIED_COUNT) != summary count ($SATISFIED)"
    exit 1
fi

if [ "$DETAILED_UNSATISFIED_COUNT" -ne "$UNSATISFIED" ]; then
    echo "❌ FAILED: Detailed unsatisfied count ($DETAILED_UNSATISFIED_COUNT) != summary count ($UNSATISFIED)"
    exit 1
fi

# Test 5: Verification Details

# If there are any satisfied verifications, check their structure
SATISFIED_COUNT=$(echo "$OUTPUT" | jq '[.satisfied_verifications.files | to_entries[] | .value[] ] | length')
if [ "$SATISFIED_COUNT" -gt 0 ]; then
    # Get first satisfied verification
    FIRST_SATISFIED=$(echo "$OUTPUT" | jq '[.satisfied_verifications.files | to_entries[] | .value[] ] | .[0]')
    
    # Check required fields
    if ! echo "$FIRST_SATISFIED" | jq 'has("identifier")' | grep -q true; then
        echo "❌ FAILED: Satisfied verification missing 'identifier'"
        exit 1
    fi
    
    if ! echo "$FIRST_SATISFIED" | jq 'has("name")' | grep -q true; then
        echo "❌ FAILED: Satisfied verification missing 'name'"
        exit 1
    fi
    
    if ! echo "$FIRST_SATISFIED" | jq 'has("section")' | grep -q true; then
        echo "❌ FAILED: Satisfied verification missing 'section'"
        exit 1
    fi
    
    if ! echo "$FIRST_SATISFIED" | jq 'has("verification_type")' | grep -q true; then
        echo "❌ FAILED: Satisfied verification missing 'verification_type'"
        exit 1
    fi
    
    if ! echo "$FIRST_SATISFIED" | jq 'has("satisfied_by")' | grep -q true; then
        echo "❌ FAILED: Satisfied verification missing 'satisfied_by'"
        exit 1
    fi
    
    # Satisfied verifications should have non-empty satisfied_by
    SATISFIED_BY_COUNT=$(echo "$FIRST_SATISFIED" | jq '.satisfied_by | length')
    if [ "$SATISFIED_BY_COUNT" -eq 0 ]; then
        echo "❌ FAILED: Satisfied verification has empty 'satisfied_by' array"
        exit 1
    fi
fi

# If there are any unsatisfied verifications, check their structure
UNSATISFIED_COUNT=$(echo "$OUTPUT" | jq '[.unsatisfied_verifications.files | to_entries[] | .value[] ] | length')
if [ "$UNSATISFIED_COUNT" -gt 0 ]; then
    # Get first unsatisfied verification
    FIRST_UNSATISFIED=$(echo "$OUTPUT" | jq '[.unsatisfied_verifications.files | to_entries[] | .value[] ] | .[0]')
    
    # Check required fields (same as satisfied)
    if ! echo "$FIRST_UNSATISFIED" | jq 'has("identifier")' | grep -q true; then
        echo "❌ FAILED: Unsatisfied verification missing 'identifier'"
        exit 1
    fi
    
    if ! echo "$FIRST_UNSATISFIED" | jq 'has("satisfied_by")' | grep -q true; then
        echo "❌ FAILED: Unsatisfied verification missing 'satisfied_by'"
        exit 1
    fi
    
    # Unsatisfied verifications should have empty satisfied_by
    SATISFIED_BY_COUNT=$(echo "$FIRST_UNSATISFIED" | jq '.satisfied_by | length')
    if [ "$SATISFIED_BY_COUNT" -ne 0 ]; then
        echo "❌ FAILED: Unsatisfied verification has non-empty 'satisfied_by' array"
        exit 1
    fi
fi

exit 0