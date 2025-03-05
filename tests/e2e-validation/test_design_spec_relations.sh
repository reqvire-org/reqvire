#!/bin/bash

# Test: Validation of Relations to Design Specification Documents
# ------------------------------------------------------------
# Acceptance Criteria:
# - System should correctly validate relations to files in the DesignSpecifications folder
# - System should properly handle markdown links to Design Specification Documents
#
# Test Criteria:
# - Command should correctly identify when a DSD file exists and validate the relation
# - Current implementation will fail until fixed
#
# Test Implementation for Verification:
# - Shows the need to add DSD files to registry for relation validation

# Setup
TEST_NAME="Design Specification Relations Validation"
TEST_DIR="../fixtures/test-design-spec-relations"
REQFLOW_BIN="${REQFLOW_BIN:-$(cd ../../ && pwd)/target/debug/reqflow}"

echo "======== $TEST_NAME ========"
echo "Test directory: $TEST_DIR"

# Run validation
echo "Running validation command..."
OUTPUT=$(cd "$TEST_DIR" && ls -la && ls -la DesignSpecifications/ && "$REQFLOW_BIN" . --validate-relations --verbose 2>&1)
EXIT_CODE=$?

# We don't need to run with explicit file reference - that's not how the tool is designed to work
# It needs to process all files, not just the one we're validating

# Use the result from running validation on all files

# Verify results
echo "Checking results..."

# Expected to succeed with the updated implementation
if [ $EXIT_CODE -eq 0 ]; then
  echo "SUCCESS: Test passed with the updated implementation."
  echo "The implementation now correctly validates relations to Design Specification Documents."
  echo ""
  echo "Output: $OUTPUT"
else
  echo "FAILED: Test failed with the updated implementation."
  echo "Relations to Design Specification Documents are still not being validated correctly."
  echo ""
  echo "Output: $OUTPUT"
  
  # Check if the error is specifically about DirectMessages.md
  if echo "$OUTPUT" | grep -q "DesignSpecifications/DirectMessages.md"; then
    echo "Issue detected: Relations to DesignSpecifications/DirectMessages.md not validating correctly."
    echo ""
    echo "Debug information:"
    echo "- Check if DSDs are being added to the registry with correct paths"
    echo "- Check if path normalization is handling DSDs correctly" 
    echo "- Check if file is properly processable"
  fi
  exit 1
fi

echo ""
echo "Implemented Changes:"
echo "1. Added path normalization to handle both normal files and fragment-only references"
echo "2. Updated validation logic to use normalized paths for consistent lookup"
echo ""
# Now exit with the actual test result (success or failure)
exit $EXIT_CODE
