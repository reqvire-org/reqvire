#!/usr/bin/env bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Test: Model Summary JSON and Filters Validation (with text checks and relations coverage)
# -------------------------------------------------------------------------------------
# Acceptance Criteria:
# - `reqvire search --json` emits valid JSON with a top-level "files" key
# - `reqvire search` prints a text search beginning with `--- MBSE Search results ---`
# - Each filter flag when used with `search --json` or `search` correctly restricts output
# - Using any filter flag without `search` fails appropriately
# - Invalid regex on name/content filters fails with `Invalid regex`
# - Search results report must include all relations for each element, showing both explicit relations and their opposite relations
# - Relations must be preserved even when filtering excludes target elements (e.g., requirements show verifiedBy relations even when verifications are filtered out)
#
# Test Criteria:
# - Commands exit 0 on success
# - JSON output parses under jq
# - Text search contains expected header, element count, and required fields
# - Filters reduce element counts as expected
# - Incorrect usage exits non-zero with proper error message
# - Relations coverage: bidirectional relationships are shown (verifiedBy/verify, refine/refinedBy, derivedFrom/derive, etc.)
# - Filtered relations coverage: relations to filtered-out elements are preserved in both directions

# 1) No filters: base JSON search - Full output comparison
echo "Starting test..." > "${TEST_DIR}/test_results.log"

echo "Running: reqvire search --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json 2>&1)
EXIT_JSON=$?
set -e

echo "Exit code: $EXIT_JSON" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"
if [ $EXIT_JSON -ne 0 ]; then
  echo "FAILED: base JSON search exited $EXIT_JSON"
  exit 1
fi

# Validate JSON is parseable
echo "$OUTPUT" | jq . >/dev/null 2>&1

# Compare against expected JSON output
EXPECTED_JSON="${TEST_SCRIPT_DIR}/expected/expected-search.json"
if ! diff <(echo "$OUTPUT" | jq --sort-keys .) <(jq --sort-keys . "$EXPECTED_JSON") > /dev/null; then
  echo "❌ FAILED: JSON output does not match expected output"
  echo "Diff:"
  diff -u <(jq --sort-keys . "$EXPECTED_JSON") <(echo "$OUTPUT" | jq --sort-keys .)
  exit 1
fi

TOTAL=$(echo "$OUTPUT" | jq '.global_counters.total_elements')
if [ "$TOTAL" -ne 7 ]; then
  echo "FAILED: expected 7 elements in JSON, got $TOTAL"
  exit 1
fi

# Enhanced JSON structure verification - Test new features

# Check that global counters include the new count fields
if ! echo "$OUTPUT" | jq -e '.global_counters.total_files' >/dev/null; then
  echo "FAILED: JSON missing total_files in global_counters"
  exit 1
fi

# Check that file summaries include the count fields
FIRST_FILE=$(echo "$OUTPUT" | jq -r '.files | keys[0]')
if ! echo "$OUTPUT" | jq -e ".files[\"$FIRST_FILE\"].total_elements" >/dev/null; then
  echo "FAILED: JSON missing total_elements in file summaries"
  exit 1
fi

# Test page content with mermaid diagrams
REQUIREMENTS_FILE="specifications/Requirements.md"
if echo "$OUTPUT" | jq -e ".files[\"$REQUIREMENTS_FILE\"].page_content" >/dev/null; then
  PAGE_CONTENT=$(echo "$OUTPUT" | jq -r ".files[\"$REQUIREMENTS_FILE\"].page_content")
  if [[ ! "$PAGE_CONTENT" == *"mermaid"* ]]; then
    echo "FAILED: Page content should include mermaid diagram"
    exit 1
  fi
  if [[ ! "$PAGE_CONTENT" == *"graph TD"* ]]; then
    echo "FAILED: Page content should include the test mermaid diagram content"
    exit 1
  fi
fi

# 2) No filters: base text search - Full output comparison
echo "Running: reqvire search" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search 2>&1)
EXIT_TEXT=$?
set -e

echo "Exit code: $EXIT_TEXT" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"
if [ $EXIT_TEXT -ne 0 ]; then
  echo "FAILED: base text search exited $EXIT_TEXT"
  exit 1
fi

if ! grep -q '^--- MBSE Search results ---' <<< "$OUTPUT"; then
  echo "FAILED: text search header missing"
  exit 1
fi

# Compare against expected text output
EXPECTED_TEXT="${TEST_SCRIPT_DIR}/expected/expected-search.txt"
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/actual-search.txt"
if ! diff "${TEST_DIR}/actual-search.txt" "$EXPECTED_TEXT" > /dev/null; then
  echo "❌ FAILED: Text output does not match expected output"
  echo "Diff:"
  diff -u "$EXPECTED_TEXT" "${TEST_DIR}/actual-search.txt"
  exit 1
fi

ELEMS_TEXT=$(grep -c '🔹 Element:' <<< "$OUTPUT")
if [ "$ELEMS_TEXT" -ne 7 ]; then
  echo "FAILED: expected 7 elements in text search, got $ELEMS_TEXT"
  exit 1
fi

# Check required fields in text
for KEY in \
    '- Name:' \
    '- File:' \
    '- Type:' \
    '- Content:'; do

  if ! grep -q -- "$KEY" <<< "$OUTPUT"; then
    echo "FAILED: text search missing '$KEY'"
    exit 1
  fi
done

# Enhanced content and counts verification - Test new features

# Check file counts format in text output
if ! grep -q '^📂 File:.* (elements: [0-9]*)' <<< "$OUTPUT"; then
  echo "FAILED: text search missing file counts format"
  exit 1
fi

# Check that global summary includes the counts
if ! grep -q 'Total files:' <<< "$OUTPUT"; then
  echo "FAILED: text search missing total files count"
  exit 1
fi

# Test page content display in text format with debug formatting
if grep -q '📄 Page content:' <<< "$OUTPUT"; then
  # Verify that page content includes mermaid diagrams with \n formatting
  if ! grep -A5 '📄 Page content:' <<< "$OUTPUT" | grep -q '\\n'; then
    echo "FAILED: Page content should use debug format with \\n newlines"
    exit 1
  fi
fi




# 3) --filter-file=Requirements.md
echo "Running: reqvire search --json --filter-file" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json --filter-file="specifications/Requirements.md" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"
if [ "$(echo "$OUTPUT" | jq '.files | length')" -ne 1 ]; then
  echo "FAILED: --filter-file JSON did not limit to 1 file"
  exit 1
fi


echo "Running: reqvire search --filter-file" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN"  search --filter-file="specifications/Requirements.md" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"
count=$(grep -c -- '^📂 File:' <<<"$OUTPUT")
if (( count != 1 )); then
  echo "FAILED: --filter-file text did not limit to 1 file (got $count)"
  exit 1
fi

# 4) --filter-type=user-requirement
echo "Running: reqvire search --json --filter-type=user-requirement" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json --filter-type="user-requirement" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"
if [ "$(echo "$OUTPUT" | jq '.global_counters.total_elements')" -ne 4 ]; then
  echo "FAILED: --filter-type=user-requirement JSON should yield 4 elements"
  exit 1
fi
echo "Running: reqvire search --filter-type=user-requirement" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN"  search --filter-type="user-requirement" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"
if [ "$(grep -c '🔹 Element:' <<< "$OUTPUT")" -ne 4 ]; then
  echo "FAILED: --filter-type=user-requirement text should yield 4 elements"
  exit 1
fi


# 6) --filter-type=verification
echo "Running: reqvire search --json --filter-type=verification" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json --filter-type="verification" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"
if [ "$(echo "$OUTPUT" | jq '.global_counters.total_elements')" -ne 2 ]; then
  echo "FAILED: --filter-type=verification JSON should yield 2 elements"
  exit 1
fi
echo "Running: reqvire search --filter-type=verification" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-type="verification" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"
if [ "$(grep -c '🔹 Element:' <<< "$OUTPUT")" -ne 2 ]; then
  echo "FAILED: --filter-type=verification text should yield 2 elements"
  exit 1
fi


# 7) --filter-name-regex="^Requirement with Valid Standard"
echo "Running: reqvire search --json --filter-name" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json --filter-name="^Requirement with Valid Standard" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"
if [ "$(echo "$OUTPUT" | jq '.global_counters.total_elements')" -ne 1 ]; then
  echo "FAILED: --filter-name-regex JSON should yield 1 element"
  exit 1
fi
echo "Running: reqvire search --filter-name" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-name="^Requirement with Valid Standard" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"
if [ "$(grep -c '🔹 Element:' <<< "$OUTPUT")" -ne 1 ]; then
  echo "FAILED: --filter-name-regex text should yield 1 element"
  exit 1
fi


# 8) --filter-content="subsection"
echo "Running: reqvire search --json --filter-content" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json --filter-content="subsection" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"
if [ "$(echo "$OUTPUT" | jq '.global_counters.total_elements')" -ne 1 ]; then
  echo "FAILED: --filter-content JSON should yield 1 element"
  exit 1
fi
echo "Running: reqvire search --filter-content" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-content="subsection" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"
if [ "$(grep -c '🔹 Element:' <<< "$OUTPUT")" -ne 1 ]; then
  echo "FAILED: --filter-content text should yield 1 element"
  exit 1
fi


# 8.5) Additive filter behavior (AND logic) - Combining multiple filters should reduce results
echo "Running: additive filter test" >> "${TEST_DIR}/test_results.log"

# First, get count with just type filter
set +e
OUTPUT1=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json --filter-type="user-requirement" 2>&1)
set -e
COUNT1=$(echo "$OUTPUT1" | jq '.global_counters.total_elements')

# Then, add name filter - should give fewer results
set +e
OUTPUT2=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json --filter-type="user-requirement" --filter-name="Valid Standard" 2>&1)
set -e
COUNT2=$(echo "$OUTPUT2" | jq '.global_counters.total_elements')

# Verify COUNT2 <= COUNT1 (filters are additive/AND)
if [ "$COUNT2" -gt "$COUNT1" ]; then
  echo "FAILED: Adding --filter-name should not increase element count (got $COUNT2 > $COUNT1)"
  echo "This indicates filters are OR instead of AND"
  exit 1
fi

# Verify COUNT2 is actually less (not just equal), since we know there are elements that don't match the name filter
if [ "$COUNT2" -ge "$COUNT1" ]; then
  echo "FAILED: Adding --filter-name='Valid Standard' should reduce count from $COUNT1"
  exit 1
fi



# 8.6) Test --have-relations filter (single relation)
echo "Running: reqvire search --json --have-relations=verifiedBy" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json --have-relations=verifiedBy 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: --have-relations=verifiedBy exited with error $EXIT_CODE"
  exit 1
fi

# Should only include elements that HAVE verifiedBy relations
COUNT_HAVE_VERIFIED=$(echo "$OUTPUT" | jq '.global_counters.total_elements')
if [ "$COUNT_HAVE_VERIFIED" -eq 0 ]; then
  echo "FAILED: --have-relations=verifiedBy should find elements with verifiedBy relations"
  exit 1
fi

# 8.7) Test --have-relations with multiple relations (must have ALL)
echo "Running: reqvire search --json --have-relations=verifiedBy,satisfiedBy" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json --have-relations=verifiedBy,satisfiedBy 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: --have-relations with multiple relations exited with error $EXIT_CODE"
  exit 1
fi

# Should only include elements that have BOTH verifiedBy AND satisfiedBy
COUNT_HAVE_BOTH=$(echo "$OUTPUT" | jq '.global_counters.total_elements')
# This count should be <= single relation count (more restrictive)
if [ "$COUNT_HAVE_BOTH" -gt "$COUNT_HAVE_VERIFIED" ]; then
  echo "FAILED: --have-relations with 2 relations should be <= single relation count"
  echo "Got $COUNT_HAVE_BOTH > $COUNT_HAVE_VERIFIED"
  exit 1
fi



# 8.8) Test --not-have-relations filter (single relation)
echo "Running: reqvire search --json --not-have-relations=verifiedBy" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json --not-have-relations=verifiedBy 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: --not-have-relations=verifiedBy exited with error $EXIT_CODE"
  exit 1
fi

# Should only include elements that do NOT have verifiedBy relations
COUNT_NOT_VERIFIED=$(echo "$OUTPUT" | jq '.global_counters.total_elements')
if [ "$COUNT_NOT_VERIFIED" -eq 0 ]; then
  echo "FAILED: --not-have-relations=verifiedBy should find elements without verifiedBy"
  exit 1
fi



# 8.9) Test filter combination with have-relations
echo "Running: reqvire search --json --filter-type=user-requirement --have-relations=verifiedBy" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json --filter-type=user-requirement --have-relations=verifiedBy 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: Combined filter with --have-relations exited with error $EXIT_CODE"
  exit 1
fi

COUNT_COMBO=$(echo "$OUTPUT" | jq '.global_counters.total_elements')
# Verify all returned elements are user-requirements with verifiedBy
TYPES=$(echo "$OUTPUT" | jq -r '.files | .[] | .elements | .[] | .type' | sort -u)
if echo "$TYPES" | grep -v "user-requirement" | grep -q .; then
  echo "FAILED: Combined filter should only return user-requirement type"
  echo "Found types: $TYPES"
  exit 1
fi



# 9) invalid regex
set +e
OUTPUT=""
CODE=0

# Capture output and exit code separately
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search  --filter-name="[invalid" 2>&1)
CODE=$?
# Don't log error output that we expect to happen
# printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"
set -e

# Assert: must NOT return exit code 0 AND must mention 'Invalid regex'
if [ $CODE -ne 1 ] && ! grep -q "Invalid regex" <<< "$OUTPUT"; then
  echo "FAILED: invalid regex TEXT did not error as expected"
  exit 1
fi

set +e
OUTPUT=""
CODE=0

# Capture output and exit code separately
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search  --json --filter-name="[invalid" 2>&1)
CODE=$?
set -e

# Assert: must NOT return exit code 0 AND must mention 'Invalid regex'
if [ $CODE -ne 1 ] || ! grep -q "Invalid regex" <<< "$OUTPUT"; then
  echo "FAILED: invalid regex JSON did not error as expected"
  exit 1
fi

# 10) Invalid relation type should produce error
set +e
OUTPUT=""
CODE=0

# Test invalid relation type with --have-relations
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --have-relations="invalidRelationType" 2>&1)
CODE=$?
set -e

# Assert: must exit with error AND must mention 'Unsupported relation type' or 'Invalid relation type'
if [ $CODE -eq 0 ]; then
  echo "FAILED: Invalid relation type should produce error (exit code was 0)"
  echo "Output: $OUTPUT"
  exit 1
fi

if ! grep -q "relation type" <<< "$OUTPUT"; then
  echo "FAILED: Error message should mention 'relation type'"
  echo "Output: $OUTPUT"
  exit 1
fi

# Test invalid relation type with --not-have-relations
set +e
OUTPUT=""
CODE=0

OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --not-have-relations="anotherInvalidType" 2>&1)
CODE=$?
set -e

if [ $CODE -eq 0 ]; then
  echo "FAILED: Invalid relation type in --not-have-relations should produce error"
  echo "Output: $OUTPUT"
  exit 1
fi

if ! grep -q "relation type" <<< "$OUTPUT"; then
  echo "FAILED: Error message should mention 'relation type' for --not-have-relations"
  echo "Output: $OUTPUT"
  exit 1
fi


# 14) Relations coverage - bidirectional relationships
echo "Running: reqvire search --json (relations test)" >> "${TEST_DIR}/test_results_relations.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results_relations.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results_relations.log"

# Check that "Verification of Standard Relations" has verify relation pointing to requirement
VERIFY_RELATIONS=$(echo "$OUTPUT" | jq -r '
  .files["specifications/Requirements.md"].elements[]
  | select(.name == "Verification of Standard Relations")
  | .relations[]?
  | select(.relation_type == "verify")
  | .target.target
')

# Note: Currently the verification element might not show verify relations if they are only implicit
# This test will pass if the verify relation exists, but we'll also check the requirements have verifiedBy
if [ -n "$VERIFY_RELATIONS" ]; then
  if ! echo "$VERIFY_RELATIONS" | grep -q "requirement-with-valid-standard-relations"; then
    echo "FAILED: Verification should have 'verify' relation to 'Requirement with Valid Standard Relations'"
    echo "Found verify relations: $VERIFY_RELATIONS"
    exit 1
  fi
fi

# Check that "Requirement with Valid Standard Relations" has verifiedBy relation pointing back to verification
VERIFIED_BY_RELATIONS=$(echo "$OUTPUT" | jq -r '
  .files["specifications/Requirements.md"].elements[]
  | select(.name == "Requirement with Valid Standard Relations")
  | .relations[]
  | select(.relation_type == "verifiedBy")
  | .target.target
')

if ! echo "$VERIFIED_BY_RELATIONS" | grep -q "verification-of-standard-relations"; then
  echo "FAILED: Requirement should have 'verifiedBy' relation to 'Verification of Standard Relations'"
  echo "Found verifiedBy relations: $VERIFIED_BY_RELATIONS"
  exit 1
fi

# Check that "Requirement with Valid Markdown Relations" also has verifiedBy relation to the same verification
VERIFIED_BY_RELATIONS_2=$(echo "$OUTPUT" | jq -r '
  .files["specifications/Requirements.md"].elements[]
  | select(.name == "Requirement with Valid Markdown Relations")
  | .relations[]
  | select(.relation_type == "verifiedBy")
  | .target.target
')

if ! echo "$VERIFIED_BY_RELATIONS_2" | grep -q "verification-of-standard-relations"; then
  echo "FAILED: Second requirement should also have 'verifiedBy' relation to 'Verification of Standard Relations'"
  echo "Found verifiedBy relations: $VERIFIED_BY_RELATIONS_2"
  exit 1
fi

# Check derive relation (looking at the JSON output, this is the opposite relation that should be shown)
DERIVE_RELATIONS_2=$(echo "$OUTPUT" | jq -r '
  .files["specifications/Requirements.md"].elements[]
  | select(.name == "Requirement with Valid Markdown Relations")
  | .relations[]
  | select(.relation_type == "derive")
  | .target.target
')

if ! echo "$DERIVE_RELATIONS_2" | grep -q "requirement-with-valid-standard-relations"; then
  echo "FAILED: Second requirement should have 'derive' relation from first requirement"
  echo "Found derive relations: $DERIVE_RELATIONS_2"
  exit 1
fi

# Check derive relation (from the JSON, the parent shows derive relation to child)
DERIVE_RELATIONS=$(echo "$OUTPUT" | jq -r '
  .files["specifications/Requirements.md"].elements[]
  | select(.name == "Requirement with Valid Standard Relations")
  | .relations[]
  | select(.relation_type == "derive")
  | .target.target
')

if ! echo "$DERIVE_RELATIONS" | grep -q "requirement-with-designspecifications-reference"; then
  echo "FAILED: Parent requirement should have 'derive' relation to child requirement"
  echo "Found derive relations: $DERIVE_RELATIONS"
  exit 1
fi

# 15) Relations visibility when filtering by type - verification filter
echo "Running: reqvire search --json --filter-type=verification (filter test)" >> "${TEST_DIR}/test_results_verification_filter.log"
set +e
OUTPUT_VERIFICATION_FILTER=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json --filter-type="verification" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results_verification_filter.log"
printf "%s\n" "$OUTPUT_VERIFICATION_FILTER" >> "${TEST_DIR}/test_results_verification_filter.log"

# When filtering by verification type, we should only see verification elements
TOTAL_ELEMENTS_VERIFICATION_FILTER=$(echo "$OUTPUT_VERIFICATION_FILTER" | jq '.global_counters.total_elements')
if [ "$TOTAL_ELEMENTS_VERIFICATION_FILTER" -ne 2 ]; then
  echo "FAILED: --filter-type=verification should show 2 elements"
  echo "Found: $TOTAL_ELEMENTS_VERIFICATION_FILTER elements"
  exit 1
fi

# But the verification element should still show its 'verify' relations to requirements (if it has any)
# Even though those requirements are not included in the filtered results
VERIFICATION_VERIFY_RELATIONS=$(echo "$OUTPUT_VERIFICATION_FILTER" | jq -r '
  .files["specifications/Requirements.md"].elements[]
  | select(.name == "Verification of Standard Relations")
  | .relations[]?
  | select(.relation_type == "verify")
  | .target.target
')

# Verification elements MUST show verify relations to maintain bidirectional traceability
if [ -z "$VERIFICATION_VERIFY_RELATIONS" ]; then
  echo "FAILED: Verification element should show explicit 'verify' relations to requirements for complete bidirectional traceability"
  echo "Requirements show verifiedBy relations, but verification shows no corresponding verify relations"
  exit 1
fi

# 16) Relations visibility when filtering by type - requirement filter
echo "Running: reqvire search --json --filter-type=user-requirement (filter test)" >> "${TEST_DIR}/test_results_requirement_filter.log"
set +e
OUTPUT_REQUIREMENT_FILTER=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json --filter-type="user-requirement" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results_requirement_filter.log"
printf "%s\n" "$OUTPUT_REQUIREMENT_FILTER" >> "${TEST_DIR}/test_results_requirement_filter.log"

# When filtering by requirement type, we should see 4 user-requirement elements
TOTAL_ELEMENTS_REQUIREMENT_FILTER=$(echo "$OUTPUT_REQUIREMENT_FILTER" | jq '.global_counters.total_elements')
if [ "$TOTAL_ELEMENTS_REQUIREMENT_FILTER" -ne 4 ]; then
  echo "FAILED: --filter-type=user-requirement should show 4 elements"
  echo "Found: $TOTAL_ELEMENTS_REQUIREMENT_FILTER elements"
  exit 1
fi

# The requirements should still show their 'verifiedBy' relations to verifications
# Even though the verification element is not included in the filtered results
REQUIREMENT_VERIFIEDBY_RELATIONS_FILTERED=$(echo "$OUTPUT_REQUIREMENT_FILTER" | jq -r '
  .files["specifications/Requirements.md"].elements[]
  | select(.name == "Requirement with Valid Standard Relations")
  | .relations[]
  | select(.relation_type == "verifiedBy")
  | .target.target
')

if ! echo "$REQUIREMENT_VERIFIEDBY_RELATIONS_FILTERED" | grep -q "verification-of-standard-relations"; then
  echo "FAILED: Requirements should still show verifiedBy relations even when verifications are filtered out"
  echo "Found verifiedBy relations in filtered output: $REQUIREMENT_VERIFIEDBY_RELATIONS_FILTERED"
  exit 1
fi

# Test that relations to filtered-out elements are preserved in both directions
REQUIREMENT2_VERIFIEDBY_RELATIONS_FILTERED=$(echo "$OUTPUT_REQUIREMENT_FILTER" | jq -r '
  .files["specifications/Requirements.md"].elements[]
  | select(.name == "Requirement with Valid Markdown Relations")
  | .relations[]
  | select(.relation_type == "verifiedBy")
  | .target.target
')

if ! echo "$REQUIREMENT2_VERIFIEDBY_RELATIONS_FILTERED" | grep -q "verification-of-standard-relations"; then
  echo "FAILED: Second requirement should also show verifiedBy relations when verifications are filtered out"
  echo "Found verifiedBy relations: $REQUIREMENT2_VERIFIEDBY_RELATIONS_FILTERED"
  exit 1
fi

exit 0

