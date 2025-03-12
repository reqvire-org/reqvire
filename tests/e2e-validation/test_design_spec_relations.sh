#!/bin/bash

# Test: Validation of Relations to Excluded Folder Documents
# ------------------------------------------------------------
# Acceptance Criteria:
# - System should correctly validate relations to files in excluded folders (like DesignSpecifications)
# - System should maintain relations even when folders are excluded from main processing
#
# Test Criteria:
# - Command should correctly validate relations to files in excluded_filename_patterns
# - Using '**/DesignSpecifications/**/*.md' pattern to exclude processing but not for relations
#
# Test Implementation for Verification:
# - Tests that relations to excluded files still work properly in the model

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
  echo "The implementation correctly validates relations to files in excluded folders."
  echo ""
  echo "Output: $OUTPUT"
else
  echo "FAILED: Test failed with the updated implementation."
  echo "Relations to files in excluded folders (DesignSpecifications) are not being validated correctly."
  echo ""
  echo "Output: $OUTPUT"
  
  # Check if the error is specifically about DirectMessages.md
  if echo "$OUTPUT" | grep -q "DesignSpecifications/DirectMessages.md"; then
    echo "Issue detected: Relations to DesignSpecifications/DirectMessages.md (in excluded folder) not validating correctly."
    echo ""
    echo "Debug information:"
    echo "- Check if files in excluded folders are being added to the registry for relation tracking"
    echo "- Check if path normalization is handling excluded folders correctly"
    echo "- Verify if validation is respecting the excluded_filename_patterns setting but still tracking relations"
  fi
  exit 1
fi

echo ""
echo "Implemented Changes:"
echo "1. Updated validation to handle relations to files in excluded folders"
echo "2. Separated processing path from relation validation to maintain references"
echo "3. Ensured relations work even when folders are excluded from processing"
echo ""
# Now exit with the actual test result (success or failure)
exit $EXIT_CODE
