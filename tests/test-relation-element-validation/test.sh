#!/bin/bash

# Test: Validation of Relation Element Types
# ----------------------------------------------------
# Acceptance Criteria:
# - System should validate that verifiedBy/verify relations connect requirements and verification elements
# - System should validate that satisfiedBy/satisfy relations connect requirements and implementation elements
# - System should report validation errors for incompatible element types in relations
# - System should return success exit code (0) despite validation errors for element type incompatibilities
#
# Test Criteria:
# - Command exits with success (zero) return code even when there are element type validation errors
# - Validation output contains specific error messages about incompatible element types for relations
# - Model processing continues despite element type validation errors
#

# Run validation
OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" --config "${TEST_DIR}/reqflow.yaml"  --validate --json 2>&1)
EXIT_CODE=$?

# Save output to log file
echo "$OUTPUT"
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/output/test_results.log"

# Check exit code (should be 0 even with element type validation errors)
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: Command should return exit code 0 despite validation errors, but returned $EXIT_CODE"
  exit 1
fi

# Verify that element type validation errors are reported
if ! echo "$OUTPUT" | grep -q "incompatible element types"; then
  echo "FAILED: No validation errors for incompatible element types found"
  exit 1
fi

# Check for specific error messages about verifiedBy relation
if ! echo "$OUTPUT" | grep -q "verifiedBy.*should connect a requirement to a verification element"; then
  echo "FAILED: Missing specific error about verifiedBy relation connecting incorrect element types"
  exit 1
fi

# Check for specific error messages about verify relation
if ! echo "$OUTPUT" | grep -q "verify.*should connect a verification element to a requirement"; then
  echo "FAILED: Missing specific error about verify relation connecting incorrect element types"
  exit 1
fi

# Verify that the validation continues processing the model despite errors
if ! echo "$OUTPUT" | grep -q "Valid-Requirement"; then
  echo "FAILED: Validation did not continue processing valid elements after finding errors"
  exit 1
fi

echo "Element type validation test PASSED!"
exit 0
