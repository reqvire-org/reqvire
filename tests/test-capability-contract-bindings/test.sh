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
(cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/capability-contract_bindings-valid.out 2>&1)
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: valid requirement-owned contract contract_bindings model should validate"
  cat /tmp/capability-contract_bindings-valid.out
  exit 1
fi
assert_diff "${TEST_SCRIPT_DIR}/expected/valid.txt" /tmp/capability-contract_bindings-valid.out "valid output mismatch"

copy_fixture "requirement-reuses-ontology.md.txt"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -eq 0 ]; then
  echo "FAILED: requirement contract_bindings to ontology should fail"
  exit 1
fi
if grep -qi "not an reusable element" <<< "$OUTPUT" && grep -qi "billing-ontology" <<< "$OUTPUT"; then
  printf "Requirement contract_bindings to ontology is rejected\n" > /tmp/capability-contract_bindings-requirement-ontology-invalid.out
else
  printf "%s\n" "$OUTPUT" > /tmp/capability-contract_bindings-requirement-ontology-invalid.out
fi
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/requirement-reuses-ontology.txt" \
  /tmp/capability-contract_bindings-requirement-ontology-invalid.out \
  "requirement ontology contract_bindings invalid output mismatch"

copy_fixture "capability-reuses-requirement-detail.md.txt"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -eq 0 ]; then
  echo "FAILED: capability contract_bindings to requirement-detail contract should fail"
  exit 1
fi
if grep -qi "cannot author contract_bindings" <<< "$OUTPUT"; then
  printf "Capability contract_bindings to requirement-detail contract is rejected\n" > /tmp/capability-contract_bindings-capability-invalid.out
else
  printf "%s\n" "$OUTPUT" > /tmp/capability-contract_bindings-capability-invalid.out
fi
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/capability-reuses-requirement-detail.txt" \
  /tmp/capability-contract_bindings-capability-invalid.out \
  "capability invalid contract_bindings output mismatch"

copy_fixture "requirement-reuses-requirement-semantic-contract.md.txt"
set +e
(cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/capability-contract_bindings-requirement-valid.out 2>&1)
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: requirement constrainedBy to semantic-contract should validate"
  cat /tmp/capability-contract_bindings-requirement-valid.out
  exit 1
fi
assert_diff "${TEST_SCRIPT_DIR}/expected/valid.txt" /tmp/capability-contract_bindings-requirement-valid.out "requirement semantic-contract constrainedBy valid output mismatch"

copy_fixture "requirement-reuses-capability-semantic-contract.md.txt"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -eq 0 ]; then
  echo "FAILED: capability contract_bindings to semantic-contract should fail"
  exit 1
fi
printf "Semantic-contract contract_bindings is rejected\n" > /tmp/capability-contract_bindings-requirement-invalid.out
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/requirement-reuses-capability-semantic-contract.txt" \
  /tmp/capability-contract_bindings-requirement-invalid.out \
  "requirement invalid contract_bindings output mismatch"

exit 0
