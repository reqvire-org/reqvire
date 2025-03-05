#!/bin/bash

# Test: Failed initialization when reqflow.yml exists
# ---------------------------------------------------
# Acceptance Criteria:
# - Attempt to initialize a project in a directory with existing reqflow.yml
# - Initialization should fail with an error message
# - No changes should be made to the existing structure
#
# Test Criteria:
# - Command exits with error (non-zero) return code
# - Error message mentions existing configuration file
# - Original reqflow.yml content remains unchanged
#
# Test Implementation for Verification:
# - See: specifications/Verifications/InitCommand.md

# Setup
TEST_NAME="Existing YML Config"
TEST_DIR="$(mktemp -d -t reqflow-test-init-XXXXXX)"
REQFLOW_BIN="${REQFLOW_BIN:-../../target/debug/reqflow}"
TEST_CONFIG_CONTENT="# Test configuration file - YML format"

echo "======== $TEST_NAME ========"
echo "Test directory: $TEST_DIR"

# Create a dummy reqflow.yml file
echo "$TEST_CONFIG_CONTENT" > "$TEST_DIR/reqflow.yml"
echo "Created dummy reqflow.yml"

# Run test
echo "Running init command (should fail)..."
OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" --init 2>&1)
EXIT_CODE=$?

# Verify results
if [ $EXIT_CODE -eq 0 ]; then
  echo "FAILED: Init command succeeded but should have failed"
  rm -rf "$TEST_DIR"
  exit 1
fi

# Check if error message contains expected text
if ! echo "$OUTPUT" | grep -q "configuration file.*exists"; then
  echo "FAILED: Error message does not mention existing configuration file"
  echo "Output was: $OUTPUT"
  rm -rf "$TEST_DIR"
  exit 1
fi

# Check if original file is unchanged
CURRENT_CONTENT=$(cat "$TEST_DIR/reqflow.yml")
if [ "$CURRENT_CONTENT" != "$TEST_CONFIG_CONTENT" ]; then
  echo "FAILED: Original reqflow.yml was modified"
  rm -rf "$TEST_DIR"
  exit 1
fi

echo "SUCCESS: Init command correctly failed when reqflow.yml exists"

# Cleanup
rm -rf "$TEST_DIR"
exit 0