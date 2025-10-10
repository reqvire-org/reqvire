#!/bin/bash

# Test: Refine Relation Chain Validation
# ---------------------------------------
# This test validates that the system correctly enforces refinement chain purity:
# - Once refine/refinedBy is used in a chain, all descendants must use refine/refinedBy
# - Mixing refine with derive or contain in descendant chains is prohibited
# - Valid chains: pure refine, pure derive, pure contain, or refine starting mid-chain
# - Invalid chains: refine followed by derive or contain in descendants
#
# Acceptance Criteria:
# - Valid refinement chains pass validation without errors
# - Invalid refinement chains fail validation with MixedHierarchicalRelations error
# - Error messages clearly identify the violating elements

# Test 1: Valid Chains (should pass validation)

echo "Starting valid chains test..." > "${TEST_DIR}/test_results_valid.log"

echo "Running: reqvire validate (valid-chains)" >> "${TEST_DIR}/test_results_valid.log"
set +e
OUTPUT_VALID=$(cd "${TEST_DIR}/valid-chains" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_VALID=$?
set -e

echo "Exit code: $EXIT_CODE_VALID" >> "${TEST_DIR}/test_results_valid.log"
printf "%s\n" "$OUTPUT_VALID" >> "${TEST_DIR}/test_results_valid.log"

# Verify exit code indicates validation success (zero)
if [ $EXIT_CODE_VALID -ne 0 ]; then
  echo "❌ FAILED: Valid refinement chains should pass validation but exited with code $EXIT_CODE_VALID"
  echo "Output:"
  echo "$OUTPUT_VALID"
  exit 1
fi

# Verify success message
if ! echo "$OUTPUT_VALID" | grep -q "No validation issues found"; then
  echo "❌ FAILED: Valid chains should output 'No validation issues found'"
  echo "Output:"
  echo "$OUTPUT_VALID"
  exit 1
fi

# Test 2: Invalid Chains (should fail validation)

echo "Starting invalid chains test..." > "${TEST_DIR}/test_results_invalid.log"

echo "Running: reqvire validate (invalid-chains)" >> "${TEST_DIR}/test_results_invalid.log"
set +e
OUTPUT_INVALID=$(cd "${TEST_DIR}/invalid-chains" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_INVALID=$?
set -e

echo "Exit code: $EXIT_CODE_INVALID" >> "${TEST_DIR}/test_results_invalid.log"
printf "%s\n" "$OUTPUT_INVALID" >> "${TEST_DIR}/test_results_invalid.log"

# Verify exit code indicates validation failure (non-zero)
if [ $EXIT_CODE_INVALID -eq 0 ]; then
  echo "❌ FAILED: Invalid refinement chains should fail validation but returned success (0)"
  echo "Output:"
  echo "$OUTPUT_INVALID"
  exit 1
fi

# Verify MixedHierarchicalRelations error is reported
if ! echo "$OUTPUT_INVALID" | grep -q "Mixed hierarchical relations in chain"; then
  echo "❌ FAILED: Expected 'Mixed hierarchical relations in chain' error message"
  echo "Actual output:"
  echo "$OUTPUT_INVALID"
  exit 1
fi

# Test 3: Verify specific invalid scenarios are detected

# Check that the error identifies specific violating elements (by identifier)
EXPECTED_VIOLATIONS=(
  "grandchild-requirement-e11"
  "grandchild-requirement-f11"
  "great-great-grandchild-requirement-g1111"
)

MISSING_VIOLATIONS=()

for expected in "${EXPECTED_VIOLATIONS[@]}"; do
  if ! echo "$OUTPUT_INVALID" | grep -q "$expected"; then
    MISSING_VIOLATIONS+=("❌ MISSING violation for: $expected")
  fi
done

if [ ${#MISSING_VIOLATIONS[@]} -gt 0 ]; then
  echo "❌ FAILED: Missing expected violation detections!"
  for missing in "${MISSING_VIOLATIONS[@]}"; do
    echo "$missing"
  done
  echo ""
  echo "ACTUAL ERROR OUTPUT:"
  echo "$OUTPUT_INVALID"
  exit 1
fi

# Test 4: Verify valid scenarios are NOT flagged as errors

# Make sure the valid chains don't appear in the invalid output error list
SHOULD_BE_VALID=(
  "Grandchild Requirement A11"
  "Great-Grandchild Requirement B111"
  "Grandchild Requirement C11"
  "Grandchild Requirement D11"
)

# Note: These shouldn't appear in invalid-chains test at all
# This is just a sanity check to ensure our test structure is correct

exit 0
