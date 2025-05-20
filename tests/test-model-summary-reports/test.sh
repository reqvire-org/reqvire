#!/usr/bin/env bash
set -euo pipefail

# Test: Model Summary JSON and Filters Validation (with text checks)
# ------------------------------------------------------------------
# Acceptance Criteria:
# - `reqvire --model-summary --json` emits valid JSON with a top-level "files" key
# - `reqvire --model-summary` prints a text summary beginning with `--- MBSE Model summary ---`
# - Each filter flag when used with `--model-summary --json` or `--model-summary` correctly restricts output
# - Using any filter flag without `--model-summary` fails appropriately
# - Invalid regex on name/content filters fails with `Invalid regex`
#
# Test Criteria:
# - Commands exit 0 on success
# - JSON output parses under jq
# - Text summary contains expected header, element count, and required fields
# - Filters reduce element counts as expected
# - Incorrect usage exits non-zero with proper error message

: "${REQVIRE_BIN:?REQVIRE_BIN must be set to the reqvire executable}"




# 1) No filters: base JSON summary
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary --json 2>&1)
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"
EXIT_JSON=$?
if [ $EXIT_JSON -ne 0 ]; then
  echo "FAILED: base JSON summary exited $EXIT_JSON"
  exit 1
fi

echo "$OUTPUT" | jq . >/dev/null 2>&1
if ! echo "$OUTPUT" | jq 'has("files")' | grep -q true; then
  echo "FAILED: JSON missing files"
  exit 1
fi

TOTAL=$(echo "$OUTPUT" | jq '.global_counters.total_elements')
if [ "$TOTAL" -ne 5 ]; then
  echo "FAILED: expected 5 elements in JSON, got $TOTAL"
  exit 1
fi


# 2) No filters: base text summary
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary 2>&1)
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"
EXIT_TEXT=$?
if [ $EXIT_TEXT -ne 0 ]; then
  echo "FAILED: base text summary exited $EXIT_TEXT"
  exit 1
fi

if ! grep -q '^--- MBSE Model summary ---' <<< "$OUTPUT"; then
  echo "FAILED: text summary header missing"
  exit 1
fi


ELEMS_TEXT=$(grep -c '🔹 Element:' <<< "$OUTPUT")
if [ "$ELEMS_TEXT" -ne 5 ]; then
  echo "FAILED: expected 5 elements in text summary, got $ELEMS_TEXT"
  exit 1
fi

# Check required fields in text
for KEY in \
    '- Name:' \
    '- Section:' \
    '- File:' \
    '- Type:' \
    '- Content:'; do

  if ! grep -q -- "$KEY" <<< "$OUTPUT"; then
    echo "FAILED: text summary missing '$KEY'"
    exit 1
  fi
done




# 3) --filter-file=Requirements.md
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary --json --filter-file="*/specifications/Requirements.md")
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"
if [ "$(echo "$OUTPUT" | jq '.files | length')" -ne 1 ]; then
  echo "FAILED: --filter-file JSON did not limit to 1 file"
  exit 1
fi


OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary --filter-file="*/specifications/Requirements.md")
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"
count=$(grep -c -- '^📂 File:' <<<"$OUTPUT")
if (( count != 1 )); then
  echo "FAILED: --filter-file text did not limit to 1 file (got $count)"
  exit 1
fi

# 4) --filter-section=Requirements
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary --json --filter-section="Requirements*")
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

COUNT_SEC_REQ=$(
  echo "$OUTPUT" | jq -r '
    .files
    | to_entries[]
    | select(.key | endswith("Requirements.md"))
    | .value.sections
    | to_entries
    | map(
        select(.key | test("^Requirements.*"))
        .value.elements
        | length
      )
    | add  // 0
  '
)

if [ "$COUNT_SEC_REQ" -ne 4 ]; then
  echo "FAILED: --filter-section=Requirements should yield 4 elements (got $COUNT_SEC_REQ)"
  exit 1
fi

OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary --filter-section="Requirements A")
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

# 1) Make sure exactly one “Requirements A” section is printed
sec_count=$(printf '%s\n' "$OUTPUT" \
  | grep -F -c "📖 Section: Requirements")

if [ "$sec_count" -ne 1 ]; then
  echo "FAILED: expected 1 'Section: Requirements' header (got $sec_count)"
  exit 1
fi

# 2) Make sure there are exactly 3 elements in total
elem_count=$(grep -c 'Element:' <<<"$OUTPUT")
if [ "$elem_count" -ne 3 ]; then
  echo "FAILED: expected 3 elements under Requirements (got $elem_count)"
  exit 1
fi



# 5) --filter-type=user-requirement
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary --json --filter-type="user-requirement")
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"
if [ "$(echo "$OUTPUT" | jq '.global_counters.total_elements')" -ne 3 ]; then
  echo "FAILED: --filter-type=user-requirement JSON should yield 3 elements"
  exit 1
fi
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary --filter-type="user-requirement")
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"
if [ "$(grep -c '🔹 Element:' <<< "$OUTPUT")" -ne 3 ]; then
  echo "FAILED: --filter-type=user-requirement text should yield 3 elements"
  exit 1
fi


# 6) --filter-type=verification
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary --json --filter-type="verification")
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"
if [ "$(echo "$OUTPUT" | jq '.global_counters.total_elements')" -ne 1 ]; then
  echo "FAILED: --filter-type=verification JSON should yield 1 element"
  exit 1
fi
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary --filter-type="verification")
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"
if [ "$(grep -c '🔹 Element:' <<< "$OUTPUT")" -ne 1 ]; then
  echo "FAILED: --filter-type=verification text should yield 1 element"
  exit 1
fi


# 7) --filter-name-regex="^Requirement with Valid Standard"
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary --json --filter-name="^Requirement with Valid Standard")
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"
if [ "$(echo "$OUTPUT" | jq '.global_counters.total_elements')" -ne 1 ]; then
  echo "FAILED: --filter-name-regex JSON should yield 1 element"
  exit 1
fi
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary --filter-name="^Requirement with Valid Standard")
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"
if [ "$(grep -c '🔹 Element:' <<< "$OUTPUT")" -ne 1 ]; then
  echo "FAILED: --filter-name-regex text should yield 1 element"
  exit 1
fi


# 8) --filter-content="subsection"
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary --json --filter-content="subsection")
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"
if [ "$(echo "$OUTPUT" | jq '.global_counters.total_elements')" -ne 1 ]; then
  echo "FAILED: --filter-content JSON should yield 1 element"
  exit 1
fi
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary --filter-content="subsection")
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"
if [ "$(grep -c '🔹 Element:' <<< "$OUTPUT")" -ne 1 ]; then
  echo "FAILED: --filter-content text should yield 1 element"
  exit 1
fi


# 9) --filter-is-verified
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary --json --filter-is-not-verified)
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

if [ "$(echo "$OUTPUT" | jq '.global_counters.requirements_not_verified')" -ne 2 ]; then
  echo "FAILED: --filter-is-not-verified JSON should yield 2 elements"
  exit 1
fi

OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary --filter-is-not-verified)
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"
if ! grep -q "Requirements not verified: 2" <<< "$OUTPUT"; then
  echo "FAILED: Expected 'Requirements not verified: 2' but got:"
  echo "$OUTPUT"
  exit 1
fi


# 10) --filter-is-satisfied
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary --json --filter-is-not-satisfied)
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"
if [ "$(echo "$OUTPUT" | jq '.global_counters.requirements_not_satisfied')" -ne 1 ]; then
  echo "FAILED: --filter-is-not-satisfied JSON should yield 3 elements"
  exit 1
fi
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary --filter-is-not-satisfied)
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"
if ! grep -q "Requirements not satisfied: 1" <<< "$OUTPUT"; then
  echo "FAILED: Expected 'Requirements not satisfied: 1' but got:"
  echo "$OUTPUT"
  exit 1
fi


# 11) Combination: user-requirement + is-satisfied
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary --json \
           --filter-type="user-requirement" --filter-is-not-satisfied)
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"           
if [ "$(echo "$OUTPUT" | jq '.global_counters.total_elements')" -ne 1 ]; then
  echo "FAILED: combo JSON should yield 1 elements"
  exit 1
fi


           
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary \
           --filter-type="user-requirement" --filter-is-not-satisfied)
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"           
if [ "$(grep -c '🔹 Element:' <<< "$OUTPUT")" -ne 1 ]; then
  echo "FAILED: combo text should yield 1 elements"
  exit 1
fi


# 12) invalid regex
set +e
OUTPUT=""
CODE=0

# Capture output and exit code separately
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary  --filter-name="***" 2>&1)
CODE=$?
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"
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
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --model-summary  --json --filter-name="***" 2>&1)
CODE=$?
set -e

# Assert: must NOT return exit code 0 AND must mention 'Invalid regex'
if [ $CODE -ne 1 ] || ! grep -q "Invalid regex" <<< "$OUTPUT"; then
  echo "FAILED: invalid regex JSON did not error as expected"
  exit 1
fi

# 13) filter without --model-summary
set +e
OUTPUT=""
CODE=0

OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-model-summary-reports  --config "${TEST_DIR}/reqvire.yaml" --filter-file="*.md" 2>&1)
CODE=$?
set -e

printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"


if [ $CODE -ne 2 ]  || ! grep -qi "Usage: reqvire --model-summary" <<< "$OUTPUT"; then
  echo "FAILED: filter without --model-summary did not error"
  exit 1
fi

exit 0

