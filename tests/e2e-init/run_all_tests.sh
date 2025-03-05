#!/bin/bash

# Test runner for initialization tests
# -----------------------------------
# This script runs all the e2e tests for the initialization feature
# and reports overall success or failure.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$SCRIPT_DIR" || exit 1

# Ensure the ReqFlow binary exists
REQFLOW_BIN="$REPO_ROOT/target/debug/reqflow"
if [ ! -x "$REQFLOW_BIN" ]; then
  echo "ERROR: ReqFlow binary not found at $REQFLOW_BIN"
  echo "Build ReqFlow first with: cargo build"
  exit 1
fi

# Export the binary path for use in test scripts
export REQFLOW_BIN

# Make all test scripts executable
chmod +x test_*.sh

echo "===================================="
echo "Running ReqFlow Init E2E Tests"
echo "===================================="

# Track total and failed tests
TOTAL_TESTS=0
FAILED_TESTS=0

# Run each test script
for test_script in test_*.sh; do
  TOTAL_TESTS=$((TOTAL_TESTS + 1))
  echo -e "\nRunning test: $test_script"
  
  # Run the test
  if ! ./"$test_script"; then
    echo "Test failed: $test_script"
    FAILED_TESTS=$((FAILED_TESTS + 1))
  fi
done

# Print summary
echo -e "\n===================================="
echo "Test Summary"
echo "===================================="
echo "Total tests: $TOTAL_TESTS"
echo "Failed tests: $FAILED_TESTS"
echo "Successful tests: $((TOTAL_TESTS - FAILED_TESTS))"

if [ $FAILED_TESTS -eq 0 ]; then
  echo -e "\nAll tests PASSED!"
  exit 0
else
  echo -e "\nSome tests FAILED!"
  exit 1
fi