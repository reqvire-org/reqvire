#!/bin/bash
set +e  

# Global Variables
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REQVIRE_BIN="${REQVIRE_BIN:-$(pwd)/target/debug/reqvire}"
TMP_DIR="$(mktemp -d -t reqvire-e2e-XXXXXX)"


# Cleanup on Exit
cleanup() {
    echo "🧹 Cleaning up temp directory: $TMP_DIR"
    rm -rf "$TMP_DIR"
}
trap cleanup EXIT

echo "🚀 Reqvire binary: $REQVIRE_BIN"
echo "🗂 Temporary directory: $TMP_DIR"

# Function to run a single test case
run_test_case() {

    local test_folder="$1"
    local test_name="$(basename $test_folder)"
    

    TEST_DIR=$(mktemp -d -t reqvire-${test_name}-XXXXXX)
        
    # Copy fixtures to TMP_DIR
    cp -a "$test_folder/." "$TEST_DIR/"
    mkdir -p "${TEST_DIR}/output"  

    pushd $TEST_DIR  > /dev/null 2>&1
    # Create simple git repository to test changes
    git init > /dev/null 2>&1
    git config --local user.email "test@example.com" > /dev/null 2>&1 
    git config --local user.name "Test User" > /dev/null 2>&1
    git remote add origin 'https://dummy.example.com/dummy-repo.git'  > /dev/null 2>&1
    git add . > /dev/null 2>&1
    git commit -m "Initial commit" > /dev/null 2>&1
    popd  > /dev/null 2>&1
    
    echo "🔹  Running test ${test_name}"
    
    TEST_DIR="$TEST_DIR" REQVIRE_BIN="$REQVIRE_BIN" bash "$test_folder/test.sh"    
    local status=$?
    
    if [ $status -eq 0 ]; then
        echo "✅ $test_name - PASSED"
    else
        echo "❌ $test_name - FAILED" 
        cat "${TEST_DIR}/test_results.log"
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

exit 0

