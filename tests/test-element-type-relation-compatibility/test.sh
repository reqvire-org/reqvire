#!/bin/bash

# Test: Element Type Relation Compatibility
# -----------------------------------------
# This test validates that the system enforces element type constraints for relations
# as defined in the Element Type Relation Compatibility matrix.
#
# Test fixtures are organized in subdirectories:
# - valid-cases/       - All valid relation combinations (should PASS)
# - invalid-derivedfrom/ - Invalid derivedFrom usages (should FAIL)
# - invalid-satisfiedby/ - Invalid satisfiedBy usages (should FAIL)
# - invalid-verifiedby/  - Invalid verifiedBy/verify usages (should FAIL)
# - invalid-contract/  - Contract types with relations (should FAIL)
# - invalid-capability-contracts/ - Capability-owned contracts (should FAIL)
#
# See TEST_MATRIX.md for complete test case documentation.

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Test 1: Valid cases should pass validation
echo "Test 1: Valid relation combinations"

set +e
OUTPUT_VALID=$(cd "${TEST_DIR}/valid-cases" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_VALID=$?
set -e

if [ $EXIT_CODE_VALID -ne 0 ]; then
  echo "FAILED: Valid relation combinations should pass validation"
  echo "Exit code: $EXIT_CODE_VALID"
  echo "Output: $OUTPUT_VALID"
  exit 1
fi

# Test 2: Invalid derivedFrom cases should fail
echo "Test 2: Invalid derivedFrom relations"

set +e
OUTPUT_DERIVEDFROM=$(cd "${TEST_DIR}/invalid-derivedfrom" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_DERIVEDFROM=$?
set -e

if [ $EXIT_CODE_DERIVEDFROM -eq 0 ]; then
  echo "FAILED: Invalid derivedFrom should fail validation but returned success"
  echo "Output: $OUTPUT_DERIVEDFROM"
  exit 1
fi

# Verify error messages mention the constraint violations
# Expected errors: verification using derivedFrom, requirement deriving from verification, other using derivedFrom
DERIVEDFROM_EXPECTED_PATTERNS=(
  "derivedFrom"
)

for pattern in "${DERIVEDFROM_EXPECTED_PATTERNS[@]}"; do
  if ! echo "$OUTPUT_DERIVEDFROM" | grep -qi "$pattern"; then
    echo "FAILED: Expected error message containing '$pattern'"
    echo "Output: $OUTPUT_DERIVEDFROM"
    exit 1
  fi
done

# Test 3: Invalid satisfiedBy cases should fail
echo "Test 3: Invalid satisfiedBy relations"

set +e
OUTPUT_SATISFIEDBY=$(cd "${TEST_DIR}/invalid-satisfiedby" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_SATISFIEDBY=$?
set -e

if [ $EXIT_CODE_SATISFIEDBY -eq 0 ]; then
  echo "FAILED: Invalid satisfiedBy should fail validation but returned success"
  echo "Output: $OUTPUT_SATISFIEDBY"
  exit 1
fi

# Check for non-test-verification satisfiedBy error
if ! echo "$OUTPUT_SATISFIEDBY" | grep -qi "satisfiedBy\|non-test-verification\|incompatible"; then
  echo "FAILED: Expected error message about satisfiedBy constraint"
  echo "Output: $OUTPUT_SATISFIEDBY"
  exit 1
fi

# Check capability specific satisfiedBy restriction
if ! echo "$OUTPUT_SATISFIEDBY" | grep -qi "capability"; then
  echo "FAILED: Expected error message mentioning capability satisfiedBy restriction"
  echo "Output: $OUTPUT_SATISFIEDBY"
  exit 1
fi

# Test 4: Invalid verifiedBy cases should fail
echo "Test 4: Invalid verifiedBy/verify relations"

set +e
OUTPUT_VERIFIEDBY=$(cd "${TEST_DIR}/invalid-verifiedby" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_VERIFIEDBY=$?
set -e

if [ $EXIT_CODE_VERIFIEDBY -eq 0 ]; then
  echo "FAILED: Invalid verifiedBy should fail validation but returned success"
  echo "Output: $OUTPUT_VERIFIEDBY"
  exit 1
fi

# Check for verifiedBy/verify constraint error
if ! echo "$OUTPUT_VERIFIEDBY" | grep -qi "verifiedBy\|verify\|incompatible"; then
  echo "FAILED: Expected error message about verifiedBy/verify constraint"
  echo "Output: $OUTPUT_VERIFIEDBY"
  exit 1
fi

# Test 5: Contract types with relations should fail
echo "Test 5: Contract types with relations"

set +e
OUTPUT_CONTRACT=$(cd "${TEST_DIR}/invalid-contract" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_CONTRACT=$?
set -e

if [ $EXIT_CODE_CONTRACT -eq 0 ]; then
  echo "FAILED: Contract types with relations should fail validation but returned success"
  echo "Output: $OUTPUT_CONTRACT"
  exit 1
fi

# Check for contract type error
if ! echo "$OUTPUT_CONTRACT" | grep -qi "contract\|constraint\|behavior\|specification\|cannot.*relation\|not.*allowed"; then
  echo "FAILED: Expected error message about contract types not allowed to have relations"
  echo "Output: $OUTPUT_CONTRACT"
  exit 1
fi

# Test 6: Capability-owned contracts should fail
echo "Test 6: Capability-owned contracts"

set +e
OUTPUT_CAPABILITY_CONTRACTS=$(cd "${TEST_DIR}/invalid-capability-contracts" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_CAPABILITY_CONTRACTS=$?
set -e

if [ $EXIT_CODE_CAPABILITY_CONTRACTS -eq 0 ]; then
  echo "FAILED: Capability-owned contracts should fail validation but returned success"
  echo "Output: $OUTPUT_CAPABILITY_CONTRACTS"
  exit 1
fi

if ! echo "$OUTPUT_CAPABILITY_CONTRACTS" | grep -qi "definedBy.*should connect a requirement\\|define.*should connect.*to a requirement"; then
  echo "FAILED: Expected error message about capability-owned contract restrictions"
  echo "Output: $OUTPUT_CAPABILITY_CONTRACTS"
  exit 1
fi

# Test 7: Valid contract define relations should pass
echo "Test 7: Valid contract define relations"

set +e
OUTPUT_CONTRACT_SATISFY=$(cd "${TEST_DIR}/valid-contract-satisfy" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_CONTRACT_SATISFY=$?
set -e

if [ $EXIT_CODE_CONTRACT_SATISFY -ne 0 ]; then
  echo "FAILED: Valid contract define relations should pass validation"
  echo "Exit code: $EXIT_CODE_CONTRACT_SATISFY"
  echo "Output: $OUTPUT_CONTRACT_SATISFY"
  exit 1
fi

# Test 8: Contract types with contract_bindings should fail
echo "Test 8: Contract types with contract_bindings"

set +e
OUTPUT_CONTRACT_ATTACH=$(cd "${TEST_DIR}/invalid-contract-contract-binding" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_CONTRACT_ATTACH=$?
set -e

if [ $EXIT_CODE_CONTRACT_ATTACH -eq 0 ]; then
  echo "FAILED: Contract types with contract_bindings should fail validation but returned success"
  echo "Output: $OUTPUT_CONTRACT_ATTACH"
  exit 1
fi

# Check for contract contract_bindings error
if ! echo "$OUTPUT_CONTRACT_ATTACH" | grep -qi "contract.*cannot have contract_bindings\|cannot.*contract_bindings"; then
  echo "FAILED: Expected error message about contract types not allowed to have contract_bindings"
  echo "Output: $OUTPUT_CONTRACT_ATTACH"
  exit 1
fi

# Test 10: Verification types with contract_bindings should fail
echo "Test 10: Verification types with contract_bindings"

set +e
OUTPUT_VERIFICATION_ATTACH=$(cd "${TEST_DIR}/invalid-verification-contract-binding" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_VERIFICATION_ATTACH=$?
set -e

if [ $EXIT_CODE_VERIFICATION_ATTACH -eq 0 ]; then
  echo "FAILED: Verification types with contract_bindings should fail validation but returned success"
  echo "Output: $OUTPUT_VERIFICATION_ATTACH"
  exit 1
fi

if ! echo "$OUTPUT_VERIFICATION_ATTACH" | grep -qi "cannot author contract_bindings\|verification evidence.*satisfiedBy\|verified targets.*verify"; then
  echo "FAILED: Expected error message about verification elements not authoring contract_bindings"
  echo "Output: $OUTPUT_VERIFICATION_ATTACH"
  exit 1
fi

exit 0
