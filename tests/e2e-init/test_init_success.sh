#!/bin/bash

# Test: Successful initialization of a new ReqFlow project
# -------------------------------------------------------
# Acceptance Criteria:
# - Initialize a new project in an empty directory
# - Project structure should be created successfully
# - Configuration file (reqflow.yaml) should be created
# - Specifications directory structure should be created
#
# Test Criteria:
# - Command exits with success (0) return code
# - Expected directories and files exist
#
# Test Implementation for Verification:
# - See: specifications/Verifications/InitCommand.md

# Setup
TEST_NAME="Successful Init"
TEST_DIR="$(mktemp -d -t reqflow-test-init-XXXXXX)"
REQFLOW_BIN="${REQFLOW_BIN:-../../target/debug/reqflow}"

echo "======== $TEST_NAME ========"
echo "Test directory: $TEST_DIR"

# Run test
echo "Running init command..."
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

if [ ! -d "$TEST_DIR/specifications/SystemRequirements" ]; then
  echo "FAILED: SystemRequirements directory was not created"
  rm -rf "$TEST_DIR"
  exit 1
fi

if [ ! -d "$TEST_DIR/specifications/DesignSpecifications" ]; then
  echo "FAILED: DesignSpecifications directory was not created"
  rm -rf "$TEST_DIR"
  exit 1
fi

# Verifications directory no longer needed, removed check

echo "SUCCESS: ReqFlow project successfully initialized"

# Cleanup
rm -rf "$TEST_DIR"
exit 0