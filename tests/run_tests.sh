#!/bin/bash
set +e  

# Global Variables
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REAL_REQVIRE_BIN="${REQVIRE_BIN:-$(pwd)/target/debug/reqvire}"
REQVIRE_BIN="$ROOT_DIR/reqvire_timing_wrapper.sh"
TMP_DIR="$(mktemp -d -t reqvire-e2e-XXXXXX)"
LOG_DIR="/tmp/reqvire-test-logs"
BENCHMARK_INVOCATIONS_FILE="$(mktemp -t reqvire-benchmark-invocations-XXXXXX.tsv)"
BENCHMARK_TESTS_FILE="$(mktemp -t reqvire-benchmark-tests-XXXXXX.tsv)"

format_ms() {
    local ms="$1"
    printf "%d.%03ds" "$((ms / 1000))" "$((ms % 1000))"
}

# Create persistent log directory
mkdir -p "$LOG_DIR"

# Cleanup on Exit
cleanup() {
    rm -rf "$TMP_DIR"
    rm -f "$BENCHMARK_INVOCATIONS_FILE" "$BENCHMARK_TESTS_FILE"
}
trap cleanup EXIT

echo "🚀 Reqvire binary: $REAL_REQVIRE_BIN"
echo "🗂 Temporary directory: $TMP_DIR"
echo "📝 Test logs directory: $LOG_DIR"

test_reqvire_stats() {
    local test_name="$1"
    awk -F'\t' -v name="$test_name" '
        $2 == name {
            total += $1;
            count++;
        }
        END {
            printf "%d\t%d", total, count;
        }
    ' "$BENCHMARK_INVOCATIONS_FILE"
}

print_benchmark_summary() {
    local total_reqvire_ms
    total_reqvire_ms="$(awk -F'\t' '{ total += $1 } END { print total + 0 }' "$BENCHMARK_INVOCATIONS_FILE")"
    local total_tests
    total_tests="$(wc -l < "$BENCHMARK_TESTS_FILE" | tr -d ' ')"
    local passed_count
    passed_count="$(awk -F'\t' '$3 == 0 { count++ } END { print count + 0 }' "$BENCHMARK_TESTS_FILE")"
    local failed_count
    failed_count="$(awk -F'\t' '$3 != 0 { count++ } END { print count + 0 }' "$BENCHMARK_TESTS_FILE")"

    echo ""
    echo "⏱ Benchmark summary"
    printf "Total elapsed: %s\n" "$(format_ms "$total_reqvire_ms")"
    printf "Tests: %s passed, %s failed, %s total\n" "$passed_count" "$failed_count" "$total_tests"
}

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

    # Save test output to persistent log file
    local log_file="${LOG_DIR}/${test_name}.log"
    TEST_DIR="$TEST_DIR" \
        REQVIRE_BIN="$REQVIRE_BIN" \
        REAL_REQVIRE_BIN="$REAL_REQVIRE_BIN" \
        REQVIRE_BENCHMARK_INVOCATIONS="$BENCHMARK_INVOCATIONS_FILE" \
        REQVIRE_BENCHMARK_TEST="$test_name" \
        bash "$test_folder/test.sh" > "$log_file" 2>&1
    local status=$?
    local stats reqvire_ms calls
    stats="$(test_reqvire_stats "$test_name")"
    reqvire_ms="${stats%%$'\t'*}"
    calls="${stats##*$'\t'}"
    printf "%s\t%s\t%s\t%s\n" "$reqvire_ms" "$calls" "$status" "$test_name" >> "$BENCHMARK_TESTS_FILE"
    local benchmark_suffix=" (reqvire $(format_ms "$reqvire_ms"))"

    if [ $status -eq 0 ]; then
        echo "✅ $test_name - PASSED$benchmark_suffix"
    else
        echo "❌ $test_name - FAILED$benchmark_suffix"
        echo "   Log file: $log_file"
        echo ""
        echo "   Full output:"
        cat "$log_file" | sed 's/^/   /'
        echo ""
    fi

    return $status
}


# Main Logic
if [[ $# -eq 1 ]]; then
    # Run specific test
    if [[ -d "$ROOT_DIR/$1" ]]; then
        run_test_case "$ROOT_DIR/$1"
        status=$?
        print_benchmark_summary
        exit $status
    else
        echo "❌ Error: Test case $1 not found!"
        exit 1
    fi
else
    # Run all test suites
    echo "🔄 Running all test suites..."
    overall_status=0
    for test_folder in "$ROOT_DIR/"test-*; do
        if [[ -d "$test_folder" ]]; then
            run_test_case "$test_folder"
            status=$?
            if [ $status -ne 0 ]; then
                overall_status=1
            fi
        fi
    done
    print_benchmark_summary
    exit $overall_status
fi

exit 0
