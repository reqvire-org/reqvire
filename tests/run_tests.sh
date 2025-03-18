#!/bin/bash
set +e  

# Global Variables
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REQFLOW_BIN="${REQFLOW_BIN:-$(pwd)/target/debug/reqflow}"
TMP_DIR="$(mktemp -d -t reqflow-e2e-XXXXXX)"

FIXTURES_DIR="$ROOT_DIR/"
mkdir -p "$FIXTURES_DIR"


# Cleanup on Exit
cleanup() {
    echo "🧹 Cleaning up temp directory: $TMP_DIR"
    rm -rf "$TMP_DIR"
}
trap cleanup EXIT

# Copy fixtures to TMP_DIR
echo "📂 Copying tests..."
cp -r "$FIXTURES_DIR" "$TMP_DIR/"

echo "🚀 ReqFlow binary: $REQFLOW_BIN"
echo "🗂 Temporary directory: $TMP_DIR"

# Function to run a single test case
run_test_case() {

    local test_folder="$1"
    local test_name="$(basename $test_folder)"
    local results_dir="$TMP_DIR/tests/${test_name}_results"
    
    mkdir -p results_dir

    echo "🔹  Running test ${test_name}"
    
    TEST_DIR="$TMP_DIR/tests/$test_name" REQFLOW_BIN="$REQFLOW_BIN" bash "$test_folder/test.sh"    
    local status=$?
    
    if [ $status -eq 0 ]; then
        echo "✅ $test_name - PASSED"
    else
        echo "❌ $test_name - FAILED" 
        cat "${TMP_DIR}/tests/$test_name/test_results.log"
    fi

    return $status
}


# Main Logic
if [[ $# -eq 1 ]]; then
    # Run specific test
    if [[ -d "$ROOT_DIR/$1" ]]; then
        run_test_case "$ROOT_DIR/$1"        
    else
        echo "❌ Error: Test case $1 not found!"
        exit 1
    fi
else
    # Run all test suites
    echo "🔄 Running all test suites..."
    for test_folder in "$ROOT_DIR/"test-*; do
        if [[ -d "$test_folder" ]]; then
            run_test_case "$test_folder"
        fi
    done
fi

# Print summary report
echo -e "\n📊 Test Summary:"
#cat "$RESULTS_DIR/test_results.log"

exit 0

