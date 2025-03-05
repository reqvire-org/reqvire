#!/bin/bash

# Test: Successful initialization when .reqflow.yml exists
# --------------------------------------------------------
# Acceptance Criteria:
# - Initialize a project in a directory with existing .reqflow.yml
# - Initialization should succeed despite the hidden config file
# - Project structure should be created successfully
#
# Test Criteria:
# - Command exits with success (0) return code
# - Expected directories and files exist
# - Original .reqflow.yml file remains untouched
# - New reqflow.yaml is created
#
# Test Implementation for Verification:
# - See: specifications/Verifications/InitCommand.md

# Setup
TEST_NAME="Dot Reqflow Config"
TEST_DIR="$(mktemp -d -t reqflow-test-init-XXXXXX)"
REQFLOW_BIN="${REQFLOW_BIN:-../../target/debug/reqflow}"
TEST_CONFIG_CONTENT="# Test hidden configuration file"

echo "======== $TEST_NAME ========"
echo "Test directory: $TEST_DIR"

# Create a dummy .reqflow.yml file
echo "$TEST_CONFIG_CONTENT" > "$TEST_DIR/.reqflow.yml"
echo "Created dummy .reqflow.yml"

# Run test
echo "Running init command (should succeed)..."
(cd "$TEST_DIR" && "$REQFLOW_BIN" --init)
EXIT_CODE=$?

# Verify results
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: Init command returned non-zero exit code: $EXIT_CODE"
  rm -rf "$TEST_DIR"
  exit 1
fi

# Check if expected files and directories exist
if [ ! -f "$TEST_DIR/reqflow.yaml" ]; then
  echo "FAILED: reqflow.yaml was not created"
  rm -rf "$TEST_DIR"
  exit 1
fi

if [ ! -d "$TEST_DIR/specifications" ]; then
  echo "FAILED: specifications directory was not created"
  rm -rf "$TEST_DIR"
  exit 1
fi

# Verifications directory no longer needed, removed check

# Check if original .reqflow.yml file is unchanged
CURRENT_CONTENT=$(cat "$TEST_DIR/.reqflow.yml")
if [ "$CURRENT_CONTENT" != "$TEST_CONFIG_CONTENT" ]; then
  echo "FAILED: Original .reqflow.yml was modified"
  rm -rf "$TEST_DIR"
  exit 1
fi

echo "SUCCESS: ReqFlow project successfully initialized despite .reqflow.yml"

# Cleanup
rm -rf "$TEST_DIR"
exit 0