#!/usr/bin/env bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

assert_file_matches() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u "$expected" "$actual"; then
    echo "FAILED: $description"
    echo ""
    echo "If changes are intentional, update $expected"
    exit 1
  fi
}

assert_summary_counts() {
  local output_file="$1"
  local expected_submodels="$2"
  local expected_requirements="$3"
  local expected_couplings="$4"
  local description="$5"

  local actual_submodels actual_requirements actual_couplings
  actual_submodels=$(grep -F -- "- **Submodels:** " "$output_file" | awk '{print $NF}' | head -n 1)
  actual_requirements=$(grep -F -- "- **Requirements:** " "$output_file" | awk '{print $NF}' | head -n 1)
  actual_couplings=$(grep -F -- "- **Cross-Submodel Couplings:** " "$output_file" | awk '{print $NF}' | head -n 1)

  if [ -z "$actual_submodels" ] || [ -z "$actual_requirements" ] || [ -z "$actual_couplings" ]; then
    echo "FAILED: $description"
    echo "Summary lines missing in $output_file"
    exit 1
  fi

  if [ "$actual_submodels" -ne "$expected_submodels" ]; then
    echo "FAILED: $description"
    echo "Expected summary submodels=$expected_submodels, got=$actual_submodels"
    exit 1
  fi

  if [ "$actual_requirements" -ne "$expected_requirements" ]; then
    echo "FAILED: $description"
    echo "Expected summary requirements=$expected_requirements, got=$actual_requirements"
    exit 1
  fi

  if [ "$actual_couplings" -ne "$expected_couplings" ]; then
    echo "FAILED: $description"
    echo "Expected summary cross-submodel couplings=$expected_couplings, got=$actual_couplings"
    exit 1
  fi
}

assert_json_summary_counts() {
  local json_file="$1"
  local expected_submodels="$2"
  local expected_requirements="$3"
  local expected_couplings="$4"
  local description="$5"

  local actual_submodels
  local actual_requirements
  local actual_couplings

  actual_submodels=$(jq -r '.summary.total_submodels' "$json_file")
  actual_requirements=$(jq -r '.summary.total_requirements' "$json_file")
  actual_couplings=$(jq -r '.summary.total_cross_submodel_couplings' "$json_file")

  if [ "$actual_submodels" != "$expected_submodels" ]; then
    echo "FAILED: $description"
    echo "Expected summary.total_submodels=$expected_submodels, got=$actual_submodels"
    exit 1
  fi

  if [ "$actual_requirements" != "$expected_requirements" ]; then
    echo "FAILED: $description"
    echo "Expected summary.total_requirements=$expected_requirements, got=$actual_requirements"
    exit 1
  fi

  if [ "$actual_couplings" != "$expected_couplings" ]; then
    echo "FAILED: $description"
    echo "Expected summary.total_cross_submodel_couplings=$expected_couplings, got=$actual_couplings"
    exit 1
  fi
}

# Test 1: text output
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" submodels 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: submodels command exited with code $EXIT_CODE"
  echo "$OUTPUT"
  exit 1
fi

printf "%s\n" "$OUTPUT" > "$TEST_DIR/output/submodels.actual.md"
assert_file_matches \
  "$TEST_SCRIPT_DIR/expected/expected_output.md" \
  "$TEST_DIR/output/submodels.actual.md" \
  "Submodels text output mismatch"
assert_summary_counts \
  "$TEST_DIR/output/submodels.actual.md" \
  2 8 2 \
  "Submodels text summary counts mismatch"

# Test 2: JSON stdout
set +e
JSON_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" submodels --json 2>&1)
JSON_EXIT=$?
set -e

if [ $JSON_EXIT -ne 0 ]; then
  echo "FAILED: submodels --json command exited with code $JSON_EXIT"
  echo "$JSON_OUTPUT"
  exit 1
fi

printf "%s\n" "$JSON_OUTPUT" > "$TEST_DIR/output/submodels.actual.json"
if ! diff -u \
  <(jq -S . "$TEST_SCRIPT_DIR/expected/expected_output.json") \
  <(jq -S . "$TEST_DIR/output/submodels.actual.json"); then
  echo "FAILED: Submodels JSON output mismatch"
  echo ""
  echo "If changes are intentional, update $TEST_SCRIPT_DIR/expected/expected_output.json"
  exit 1
fi
assert_json_summary_counts \
  "$TEST_DIR/output/submodels.actual.json" \
  2 8 2 \
  "Submodels JSON summary counts mismatch"

# Test 3: JSON file output
set +e
FILE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" submodels --json --output output/submodels.json 2>&1)
FILE_EXIT=$?
set -e

if [ $FILE_EXIT -ne 0 ]; then
  echo "FAILED: submodels --json --output command exited with code $FILE_EXIT"
  echo "$FILE_OUTPUT"
  exit 1
fi

if ! echo "$FILE_OUTPUT" | grep -q "Output saved to output/submodels.json"; then
  echo "FAILED: missing output confirmation message"
  echo "$FILE_OUTPUT"
  exit 1
fi

if ! diff -u \
  <(jq -S . "$TEST_SCRIPT_DIR/expected/expected_output.json") \
  <(jq -S . "$TEST_DIR/output/submodels.json"); then
  echo "FAILED: Submodels JSON file output mismatch"
  echo ""
  echo "If changes are intentional, update $TEST_SCRIPT_DIR/expected/expected_output.json"
  exit 1
fi

# Test 4: --from capability text output
set +e
CAPABILITY_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" submodels --from "Capability One" 2>&1)
CAPABILITY_EXIT=$?
set -e

if [ $CAPABILITY_EXIT -ne 0 ]; then
  echo "FAILED: submodels --from Capability One command exited with code $CAPABILITY_EXIT"
  echo "$CAPABILITY_OUTPUT"
  exit 1
fi

printf "%s\n" "$CAPABILITY_OUTPUT" > "$TEST_DIR/output/submodels.from-capability-one.actual.md"
assert_file_matches \
  "$TEST_SCRIPT_DIR/expected/expected_from_capability_one_output.md" \
  "$TEST_DIR/output/submodels.from-capability-one.actual.md" \
  "Submodels --from Capability One text output mismatch"
assert_summary_counts \
  "$TEST_DIR/output/submodels.from-capability-one.actual.md" \
  1 5 2 \
  "Submodels --from Capability One summary counts mismatch"

# Test 5: --from capability JSON output
set +e
CAPABILITY_JSON_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" submodels --from "Capability One" --json 2>&1)
CAPABILITY_JSON_EXIT=$?
set -e

if [ $CAPABILITY_JSON_EXIT -ne 0 ]; then
  echo "FAILED: submodels --from Capability One --json command exited with code $CAPABILITY_JSON_EXIT"
  echo "$CAPABILITY_JSON_OUTPUT"
  exit 1
fi

printf "%s\n" "$CAPABILITY_JSON_OUTPUT" > "$TEST_DIR/output/submodels.from-capability-one.actual.json"
if ! diff -u \
  <(jq -S . "$TEST_SCRIPT_DIR/expected/expected_from_capability_one_output.json") \
  <(jq -S . "$TEST_DIR/output/submodels.from-capability-one.actual.json"); then
  echo "FAILED: Submodels --from Capability One JSON output mismatch"
  echo ""
  echo "If changes are intentional, update $TEST_SCRIPT_DIR/expected/expected_from_capability_one_output.json"
  exit 1
fi
assert_json_summary_counts \
  "$TEST_DIR/output/submodels.from-capability-one.actual.json" \
  1 5 2 \
  "Submodels --from Capability One JSON summary counts mismatch"

# Test 6: --from requirement text output
set +e
FROM_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" submodels --from "Root One" 2>&1)
FROM_EXIT=$?
set -e

if [ $FROM_EXIT -ne 0 ]; then
  echo "FAILED: submodels --from command exited with code $FROM_EXIT"
  echo "$FROM_OUTPUT"
  exit 1
fi

printf "%s\n" "$FROM_OUTPUT" > "$TEST_DIR/output/submodels.from-root-one.actual.md"
assert_file_matches \
  "$TEST_SCRIPT_DIR/expected/expected_from_root_one_output.md" \
  "$TEST_DIR/output/submodels.from-root-one.actual.md" \
  "Submodels --from text output mismatch"
assert_summary_counts \
  "$TEST_DIR/output/submodels.from-root-one.actual.md" \
  2 4 2 \
  "Submodels --from summary counts mismatch"

# Test 7: --from requirement JSON output
set +e
FROM_JSON_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" submodels --from "Root One" --json 2>&1)
FROM_JSON_EXIT=$?
set -e

if [ $FROM_JSON_EXIT -ne 0 ]; then
  echo "FAILED: submodels --from --json command exited with code $FROM_JSON_EXIT"
  echo "$FROM_JSON_OUTPUT"
  exit 1
fi

printf "%s\n" "$FROM_JSON_OUTPUT" > "$TEST_DIR/output/submodels.from-root-one.actual.json"
if ! diff -u \
  <(jq -S . "$TEST_SCRIPT_DIR/expected/expected_from_root_one_output.json") \
  <(jq -S . "$TEST_DIR/output/submodels.from-root-one.actual.json"); then
  echo "FAILED: Submodels --from JSON output mismatch"
  echo ""
  echo "If changes are intentional, update $TEST_SCRIPT_DIR/expected/expected_from_root_one_output.json"
  exit 1
fi
assert_json_summary_counts \
  "$TEST_DIR/output/submodels.from-root-one.actual.json" \
  2 4 2 \
  "Submodels --from JSON summary counts mismatch"

# Test 8: --from branch output should report only branch submodels
set +e
BRANCH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" submodels --from "Billing Requirement" 2>&1)
BRANCH_EXIT=$?
set -e

if [ $BRANCH_EXIT -ne 0 ]; then
  echo "FAILED: submodels --from Billing Requirement exited with code $BRANCH_EXIT"
  echo "$BRANCH_OUTPUT"
  exit 1
fi

printf "%s\n" "$BRANCH_OUTPUT" > "$TEST_DIR/output/submodels.from-billing.actual.md"
assert_file_matches \
  "$TEST_SCRIPT_DIR/expected/expected_from_billing_output.md" \
  "$TEST_DIR/output/submodels.from-billing.actual.md" \
  "Submodels --from Billing Requirement text output mismatch"
assert_summary_counts \
  "$TEST_DIR/output/submodels.from-billing.actual.md" \
  1 1 0 \
  "Submodels --from Billing Requirement summary counts mismatch"

# Test 9: --from branch JSON output should report branch subtree summary
set +e
BRANCH_JSON_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" submodels --from "Billing Requirement" --json 2>&1)
BRANCH_JSON_EXIT=$?
set -e

if [ $BRANCH_JSON_EXIT -ne 0 ]; then
  echo "FAILED: submodels --from Billing Requirement --json exited with code $BRANCH_JSON_EXIT"
  echo "$BRANCH_JSON_OUTPUT"
  exit 1
fi

printf "%s\n" "$BRANCH_JSON_OUTPUT" > "$TEST_DIR/output/submodels.from-billing.actual.json"
if ! diff -u \
  <(jq -S . "$TEST_SCRIPT_DIR/expected/expected_from_billing_output.json") \
  <(jq -S . "$TEST_DIR/output/submodels.from-billing.actual.json"); then
  echo "FAILED: Submodels --from Billing Requirement JSON output mismatch"
  echo ""
  echo "If changes are intentional, update $TEST_SCRIPT_DIR/expected/expected_from_billing_output.json"
  exit 1
fi
assert_json_summary_counts \
  "$TEST_DIR/output/submodels.from-billing.actual.json" \
  1 1 0 \
  "Submodels --from Billing Requirement JSON summary counts mismatch"

# Test 10: --from leaf requirement should produce empty scoped result
set +e
LEAF_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" submodels --from "Invoice Requirement" 2>&1)
LEAF_EXIT=$?
set -e

if [ $LEAF_EXIT -ne 0 ]; then
  echo "FAILED: submodels --from Invoice Requirement exited with code $LEAF_EXIT"
  echo "$LEAF_OUTPUT"
  exit 1
fi

printf "%s\n" "$LEAF_OUTPUT" > "$TEST_DIR/output/submodels.from-invoice.actual.md"
assert_file_matches \
  "$TEST_SCRIPT_DIR/expected/expected_from_invoice_output.md" \
  "$TEST_DIR/output/submodels.from-invoice.actual.md" \
  "Submodels --from Invoice Requirement text output mismatch"
assert_summary_counts \
  "$TEST_DIR/output/submodels.from-invoice.actual.md" \
  0 0 0 \
  "Submodels --from Invoice Requirement summary counts mismatch"

# Test 11: --from leaf requirement JSON output should be empty result
set +e
LEAF_JSON_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" submodels --from "Invoice Requirement" --json 2>&1)
LEAF_JSON_EXIT=$?
set -e

if [ $LEAF_JSON_EXIT -ne 0 ]; then
  echo "FAILED: submodels --from Invoice Requirement --json exited with code $LEAF_JSON_EXIT"
  echo "$LEAF_JSON_OUTPUT"
  exit 1
fi

printf "%s\n" "$LEAF_JSON_OUTPUT" > "$TEST_DIR/output/submodels.from-invoice.actual.json"
if ! diff -u \
  <(jq -S . "$TEST_SCRIPT_DIR/expected/expected_from_invoice_output.json") \
  <(jq -S . "$TEST_DIR/output/submodels.from-invoice.actual.json"); then
  echo "FAILED: Submodels --from Invoice Requirement JSON output mismatch"
  echo ""
  echo "If changes are intentional, update $TEST_SCRIPT_DIR/expected/expected_from_invoice_output.json"
  exit 1
fi
assert_json_summary_counts \
  "$TEST_DIR/output/submodels.from-invoice.actual.json" \
  0 0 0 \
  "Submodels --from Invoice Requirement JSON summary counts mismatch"

# Test 12: --from missing root should fail
set +e
MISSING_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" submodels --from "Missing Root" 2>&1)
MISSING_EXIT=$?
set -e

if [ $MISSING_EXIT -eq 0 ]; then
  echo "FAILED: submodels --from Missing Root should fail"
  exit 1
fi

if ! echo "$MISSING_OUTPUT" | grep -qi "Submodel root 'Missing Root' not found"; then
  echo "FAILED: missing-root error message mismatch"
  echo "$MISSING_OUTPUT"
  exit 1
fi

exit 0
