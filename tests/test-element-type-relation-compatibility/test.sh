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
# - invalid-refinement/  - Refinement types with relations (should FAIL)
# - valid-trace/         - Valid trace relations for all types (should PASS)
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

# Test 5: Refinement types with relations should fail
echo "Test 5: Refinement types with relations"

set +e
OUTPUT_REFINEMENT=$(cd "${TEST_DIR}/invalid-refinement" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_REFINEMENT=$?
set -e

if [ $EXIT_CODE_REFINEMENT -eq 0 ]; then
  echo "FAILED: Refinement types with relations should fail validation but returned success"
  echo "Output: $OUTPUT_REFINEMENT"
  exit 1
fi

# Check for refinement type error
if ! echo "$OUTPUT_REFINEMENT" | grep -qi "refinement\|constraint\|behavior\|specification\|cannot.*relation\|not.*allowed"; then
  echo "FAILED: Expected error message about refinement types not allowed to have relations"
  echo "Output: $OUTPUT_REFINEMENT"
  exit 1
fi

# Test 6: Valid trace relations should pass
echo "Test 6: Valid trace relations"

set +e
OUTPUT_TRACE=$(cd "${TEST_DIR}/valid-trace" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_TRACE=$?
set -e

if [ $EXIT_CODE_TRACE -ne 0 ]; then
  echo "FAILED: Valid trace relations should pass validation"
  echo "Exit code: $EXIT_CODE_TRACE"
  echo "Output: $OUTPUT_TRACE"
  exit 1
fi

# Test 7: Valid refinement refine relations should pass
echo "Test 7: Valid refinement refine relations"

set +e
OUTPUT_REFINEMENT_SATISFY=$(cd "${TEST_DIR}/valid-refinement-satisfy" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_REFINEMENT_SATISFY=$?
set -e

if [ $EXIT_CODE_REFINEMENT_SATISFY -ne 0 ]; then
  echo "FAILED: Valid refinement refine relations should pass validation"
  echo "Exit code: $EXIT_CODE_REFINEMENT_SATISFY"
  echo "Output: $OUTPUT_REFINEMENT_SATISFY"
  exit 1
fi

# Test 8: Refinement types with attachments should fail
echo "Test 8: Refinement types with attachments"

set +e
OUTPUT_REFINEMENT_ATTACH=$(cd "${TEST_DIR}/invalid-refinement-attachment" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_REFINEMENT_ATTACH=$?
set -e

if [ $EXIT_CODE_REFINEMENT_ATTACH -eq 0 ]; then
  echo "FAILED: Refinement types with attachments should fail validation but returned success"
  echo "Output: $OUTPUT_REFINEMENT_ATTACH"
  exit 1
fi

# Check for refinement attachment error
if ! echo "$OUTPUT_REFINEMENT_ATTACH" | grep -qi "refinement.*cannot have attachments\|cannot.*attachment"; then
  echo "FAILED: Expected error message about refinement types not allowed to have attachments"
  echo "Output: $OUTPUT_REFINEMENT_ATTACH"
  exit 1
fi

exit 0
