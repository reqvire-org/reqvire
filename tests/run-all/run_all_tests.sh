#!/bin/bash

# Run all ReqFlow tests
# -------------------

# Change to the script's directory
cd "$(dirname "$0")/.." || exit 1

# Track overall success/failure
FAILURES=0
TOTAL_TESTS=0
PASSED_TESTS=0

# Function to run tests in a directory
run_tests_in_dir() {
  local dir=$1
  echo "================================================="
  echo "Running tests in $dir"
  echo "================================================="
  
  if [ -f "$dir/run_all_tests.sh" ]; then
    echo "Running test suite using run_all_tests.sh..."
    if ! (cd "$dir" && ./run_all_tests.sh); then
      echo "Test suite $dir FAILED"
      ((FAILURES++))
    else
      echo "Test suite $dir PASSED"
    fi
  else
    # Run individual tests if no run_all_tests.sh exists
    for test in "$dir"/test_*.sh; do
      if [ -f "$test" ]; then
        echo "----------------------------------------"
        echo "Running test: $test"
        echo "----------------------------------------"
        
        ((TOTAL_TESTS++))
        if ! "$test"; then
          echo "Test $test FAILED"
          ((FAILURES++))
        else
          echo "Test $test PASSED"
          ((PASSED_TESTS++))
        fi
        echo ""
      fi
    done
  fi
}

# Run tests in each test directory
for test_dir in e2e-*; do
  if [ -d "$test_dir" ]; then
    run_tests_in_dir "$test_dir"
  fi
done

# Report results
echo "================================================="
if [ $FAILURES -eq 0 ]; then
  echo "All test suites PASSED"
  exit 0
else
  echo "$FAILURES test suite(s) FAILED"
  exit 1
fi