#!/bin/bash

# Run all validation tests
# -----------------------

# Change to the script's directory
cd "$(dirname "$0")" || exit 1

# Make all test scripts executable
chmod +x ./*.sh

# Track overall success/failure
FAILURES=0

# Run each test and count failures
for test in ./test_*.sh; do
  echo "========================================="
  echo "Running test: $test"
  echo "========================================="
  
  if ! "$test"; then
    echo "Test $test FAILED"
    ((FAILURES++))
  else
    echo "Test $test PASSED"
  fi
  
  echo ""
done

# All tests should now run correctly

# Report results
echo "========================================="
if [ $FAILURES -eq 0 ]; then
  echo "All validation tests PASSED"
  exit 0
else
  echo "$FAILURES test(s) FAILED"
  exit 1
fi