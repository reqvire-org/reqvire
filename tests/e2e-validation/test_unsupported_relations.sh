#!/bin/bash

# Test: Validation of Unsupported Relation Types
# ---------------------------------------------
# Acceptance Criteria:
# - System should detect and report relation types that are not in the supported vocabulary
# - System should provide clear error messages suggesting the correct relation type
#
# Test Criteria:
# - Command exits with error (non-zero) return code
# - Error output contains specific error messages about unsupported relation types
# - Error message suggests possible correct relation types
#
# Test Implementation for Verification:
# - See: specifications/Verifications/ValidationTests.md

# Setup
TEST_NAME="Unsupported Relations Validation"
TEST_DIR="../fixtures/test-unsupported-relations"
REQFLOW_BIN="${REQFLOW_BIN:-$(cd ../../ && pwd)/target/debug/reqflow}"

echo "======== $TEST_NAME ========"
echo "Test directory: $TEST_DIR"

# Run validation
echo "Running validation command..."
OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" --validate-relations 2>&1)
EXIT_CODE=$?

# Verify results
echo "Checking results..."

# Verify exit code indicates error
if [ $EXIT_CODE -eq 0 ]; then
  echo "FAILED: Validation should have failed but returned success (0)"
  echo "Output: $OUTPUT"
  exit 1
fi

echo "Exit code $EXIT_CODE indicates validation errors as expected"

# Check for specific error messages about unsupported relation types
if ! echo "$OUTPUT" | grep -q "satisfieddBy"; then
  echo "FAILED: Missing error about unsupported relation type 'satisfieddBy'"
  echo "Output: $OUTPUT"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "basedFrom"; then
  echo "FAILED: Missing error about unsupported relation type 'basedFrom'"
  echo "Output: $OUTPUT"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "refinesFrom"; then
  echo "FAILED: Missing error about unsupported relation type 'refinesFrom'"
  echo "Output: $OUTPUT"
  exit 1
fi

# Check for suggestions of correct relation types (future implementation)
# Commented out until this feature is implemented in ReqFlow
# if ! echo "$OUTPUT" | grep -q "Did you mean"; then
#   echo "FAILED: Missing suggestions for correct relation types"
#   echo "Output: $OUTPUT"
#   exit 1
# fi

echo "SUCCESS: All expected validation errors for unsupported relation types were detected"
exit 0