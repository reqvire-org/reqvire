#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

copy_fixture() {
  local fixture="$1"
  rm -rf "${TEST_DIR}/specifications"
  mkdir -p "${TEST_DIR}/specifications"
  cp "${TEST_SCRIPT_DIR}/fixtures/${fixture}" "${TEST_DIR}/specifications/Requirements.md"
}

assert_diff() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u "$expected" "$actual"; then
    echo "FAILED: ${description}"
    echo ""
    echo "If changes are intentional, update ${expected}"
    exit 1
  fi
}

copy_fixture "valid.md.txt"
set +e
(cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/capability-attachments-valid.out 2>&1)
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: valid capability ontology attachment should validate"
  cat /tmp/capability-attachments-valid.out
  exit 1
fi
assert_diff "${TEST_SCRIPT_DIR}/expected/valid.txt" /tmp/capability-attachments-valid.out "valid output mismatch"

copy_fixture "requirement-attaches-ontology.md.txt"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -eq 0 ]; then
  echo "FAILED: requirement attachment to ontology should fail"
  exit 1
fi
if grep -qi "Requirement attachments" <<< "$OUTPUT" && grep -qi "ontology" <<< "$OUTPUT"; then
  printf "Requirement attachment to ontology is rejected\n" > /tmp/capability-attachments-requirement-ontology-invalid.out
else
  printf "%s\n" "$OUTPUT" > /tmp/capability-attachments-requirement-ontology-invalid.out
fi
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/requirement-attaches-ontology.txt" \
  /tmp/capability-attachments-requirement-ontology-invalid.out \
  "requirement ontology attachment invalid output mismatch"

copy_fixture "capability-attaches-requirement-detail.md.txt"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -eq 0 ]; then
  echo "FAILED: capability attachment to requirement-detail refinement should fail"
  exit 1
fi
if grep -qi "Capability attachments may target ontology only" <<< "$OUTPUT"; then
  printf "Capability attachment to requirement-detail refinement is rejected\n" > /tmp/capability-attachments-capability-invalid.out
else
  printf "%s\n" "$OUTPUT" > /tmp/capability-attachments-capability-invalid.out
fi
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/capability-attaches-requirement-detail.txt" \
  /tmp/capability-attachments-capability-invalid.out \
  "capability invalid attachment output mismatch"

copy_fixture "requirement-attaches-requirement-semantic-contract.md.txt"
set +e
(cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/capability-attachments-requirement-valid.out 2>&1)
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: requirement constrainedBy to semantic-contract should validate"
  cat /tmp/capability-attachments-requirement-valid.out
  exit 1
fi
assert_diff "${TEST_SCRIPT_DIR}/expected/valid.txt" /tmp/capability-attachments-requirement-valid.out "requirement semantic-contract constrainedBy valid output mismatch"

copy_fixture "requirement-attaches-capability-semantic-contract.md.txt"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -eq 0 ]; then
  echo "FAILED: capability attachment to semantic-contract should fail"
  exit 1
fi
printf "Semantic-contract attachment is rejected\n" > /tmp/capability-attachments-requirement-invalid.out
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/requirement-attaches-capability-semantic-contract.txt" \
  /tmp/capability-attachments-requirement-invalid.out \
  "requirement invalid attachment output mismatch"

exit 0
