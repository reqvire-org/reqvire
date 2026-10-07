#!/bin/bash
set +e

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SOURCE_REQVIRE_BIN="${REQVIRE_BIN:-$(pwd)/target/debug/reqvire}"
REQVIRE_BIN="$ROOT_DIR/reqvire_timing_wrapper.sh"

now_ms() {
    local value
    value="$(date +%s%N 2>/dev/null)"
    if [[ "$value" == *N* || -z "$value" ]]; then
        echo "$(($(date +%s) * 1000))"
    else
        echo "$((value / 1000000))"
    fi
}

format_ms() {
    local ms="$1"
    printf "%d.%03ds" "$((ms / 1000))" "$((ms % 1000))"
}

SUITE_STARTED_MS="$(now_ms)"
TMP_DIR="$(mktemp -d -t reqvire-e2e-XXXXXX)" || exit 1
REAL_REQVIRE_BIN="$TMP_DIR/reqvire"
ACTIVE_TEST_PID=""
ACTIVE_FIXTURE=""
CLEANUP_FAILED=false

stop_active_test() {
    [[ -n "$ACTIVE_TEST_PID" ]] || return 0
    # Capture descendants before their Python/shell owners exit, including MCP
    # servers deliberately launched into separate sessions by existing suites.
    if kill -0 -- "-$ACTIVE_TEST_PID" 2>/dev/null; then
        if ! python3 "$ROOT_DIR/stop_test_processes.py" "$ACTIVE_TEST_PID"; then
            CLEANUP_FAILED=true
            return 1
        fi
    fi
    wait "$ACTIVE_TEST_PID" 2>/dev/null || true
    ACTIVE_TEST_PID=""
}

cleanup() {
    stop_active_test || true
    if [[ -n "$ACTIVE_FIXTURE" ]]; then
        if [[ "$CLEANUP_FAILED" == true ]]; then
            echo "Cleanup incomplete; retained fixture: $ACTIVE_FIXTURE" >&2
        else
            rm -rf -- "$ACTIVE_FIXTURE"
        fi
    fi
    # The executable stays available until every owned process has stopped.
    if [[ "$CLEANUP_FAILED" == false ]]; then
        rm -f -- "$REAL_REQVIRE_BIN"
    fi
    # Completed failed fixtures remain at their reported paths. All successful
    # and interrupted fixtures have gone; remove the run root only when empty.
    rmdir -- "$TMP_DIR" 2>/dev/null || true
}

interrupted() {
    trap '' TERM INT HUP
    echo "Interrupted; test logs: $LOG_DIR" >&2
    exit "$1"
}

trap cleanup EXIT
trap 'interrupted 143' TERM
trap 'interrupted 130' INT
trap 'interrupted 129' HUP

LOG_ROOT="${REQVIRE_TEST_LOG_DIR:-/tmp/reqvire-test-logs}"
mkdir -p -- "$LOG_ROOT" || exit 1
LOG_ROOT="$(cd "$LOG_ROOT" && pwd)" || exit 1
LOG_DIR="$(mktemp -d "$LOG_ROOT/run-XXXXXX")" || exit 1
BENCHMARK_INVOCATIONS_FILE="$LOG_DIR/invocations.tsv"
BENCHMARK_TESTS_FILE="$LOG_DIR/tests.tsv"
: > "$BENCHMARK_INVOCATIONS_FILE"
: > "$BENCHMARK_TESTS_FILE"

source_executable="$(command -v "$SOURCE_REQVIRE_BIN")"
if [[ ! -f "$source_executable" || ! -x "$source_executable" ]]; then
    echo "Reqvire executable not found or not executable: $SOURCE_REQVIRE_BIN" >&2
    exit 127
fi
# Copy once rather than following Cargo's replaceable output for every call.
# An independent inode also protects against in-place replacement of the source.
if ! cp -p -- "$source_executable" "$REAL_REQVIRE_BIN"; then
    echo "Could not capture Reqvire executable: $SOURCE_REQVIRE_BIN" >&2
    exit 1
fi

echo "🚀 Reqvire binary: $SOURCE_REQVIRE_BIN"
echo "📌 Test executable: $REAL_REQVIRE_BIN"
echo "🗂 Temporary directory: $TMP_DIR"
echo "📝 Test logs directory: $LOG_DIR"

test_reqvire_stats() {
    awk -F'\t' -v name="$1" '
        $2 == name { total += $1; count++ }
        END { printf "%d\t%d", total, count }
    ' "$BENCHMARK_INVOCATIONS_FILE"
}

print_benchmark_summary() {
    local wall_ms total_reqvire_ms total_tests passed_count failed_count
    wall_ms="$(( $(now_ms) - SUITE_STARTED_MS ))"
    total_reqvire_ms="$(awk -F'\t' '{ total += $1 } END { print total + 0 }' "$BENCHMARK_INVOCATIONS_FILE")"
    total_tests="$(wc -l < "$BENCHMARK_TESTS_FILE" | tr -d ' ')"
    passed_count="$(awk -F'\t' '$4 == 0 { count++ } END { print count + 0 }' "$BENCHMARK_TESTS_FILE")"
    failed_count="$(awk -F'\t' '$4 != 0 { count++ } END { print count + 0 }' "$BENCHMARK_TESTS_FILE")"

    echo ""
    echo "⏱ Benchmark summary"
    printf "Suite wall time: %s\n" "$(format_ms "$wall_ms")"
    printf "Accumulated Reqvire time: %s\n" "$(format_ms "$total_reqvire_ms")"
    printf "Tests: %s passed, %s failed, %s total\n" "$passed_count" "$failed_count" "$total_tests"
}

prepare_fixture() {
    mkdir -p -- "$TEST_DIR" || return
    cp -a "$1/." "$TEST_DIR/" || return
    mkdir -p -- "$TEST_DIR/output" || return
    git -C "$TEST_DIR" init -q || return
    git -C "$TEST_DIR" config --local user.email "test@example.com" || return
    git -C "$TEST_DIR" config --local user.name "Test User" || return
    git -C "$TEST_DIR" remote add origin 'https://dummy.example.com/dummy-repo.git' || return
    git -C "$TEST_DIR" add . || return
    git -C "$TEST_DIR" -c commit.gpgSign=false commit -qm "Initial commit"
}

run_test_case() {
    local test_folder="$1"
    local test_name
    test_name="$(basename "$test_folder")"
    if [[ ! -f "$test_folder/test.sh" ]]; then
        echo "⏭️  $test_name - SKIPPED (missing test.sh)"
        return 0
    fi

    local started_ms status stats reqvire_ms calls wall_ms log_file benchmark_suffix
    started_ms="$(now_ms)"
    TEST_DIR="$TMP_DIR/$test_name"
    ACTIVE_FIXTURE="$TEST_DIR"
    CLEANUP_FAILED=false
    log_file="$LOG_DIR/$test_name.log"
    echo "🔹  Running test $test_name"

    # Bash job control gives this asynchronous suite a private process group on
    # Linux and macOS. Disable it again immediately to keep the runner quiet.
    set -m
    (
        trap - EXIT TERM INT HUP
        if prepare_fixture "$test_folder"; then
            TEST_DIR="$TEST_DIR" \
                REQVIRE_BIN="$REQVIRE_BIN" \
                REAL_REQVIRE_BIN="$REAL_REQVIRE_BIN" \
                REQVIRE_BENCHMARK_INVOCATIONS="$BENCHMARK_INVOCATIONS_FILE" \
                REQVIRE_BENCHMARK_TEST="$test_name" \
                bash "$test_folder/test.sh"
        else
            status=$?
            echo "FAILED: fixture setup for $test_name (exit $status)" >&2
            exit "$status"
        fi
    ) <&0 > "$log_file" 2>&1 &
    ACTIVE_TEST_PID=$!
    set +m
    wait "$ACTIVE_TEST_PID"
    status=$?
    stop_active_test || status=1

    wall_ms="$(( $(now_ms) - started_ms ))"
    stats="$(test_reqvire_stats "$test_name")"
    reqvire_ms="${stats%%$'\t'*}"
    calls="${stats##*$'\t'}"
    # wall_ms, accumulated_reqvire_ms, invocation_count, exit_status, suite_name
    printf "%s\t%s\t%s\t%s\t%s\n" "$wall_ms" "$reqvire_ms" "$calls" "$status" "$test_name" >> "$BENCHMARK_TESTS_FILE"
    benchmark_suffix=" (wall $(format_ms "$wall_ms"), reqvire $(format_ms "$reqvire_ms"), $calls calls)"

    if [[ "$status" -eq 0 ]]; then
        rm -rf -- "$TEST_DIR"
        echo "✅ $test_name - PASSED$benchmark_suffix"
    else
        echo "❌ $test_name - FAILED$benchmark_suffix"
        echo "   Log file: $log_file"
        echo "   Retained fixture: $TEST_DIR"
        echo ""
        echo "   Full output:"
        sed 's/^/   /' "$log_file"
        echo ""
    fi
    ACTIVE_FIXTURE=""
    return "$status"
}

if [[ $# -eq 1 ]]; then
    if [[ -d "$ROOT_DIR/$1" ]]; then
        run_test_case "$ROOT_DIR/$1"
        status=$?
        print_benchmark_summary
        exit "$status"
    else
        echo "❌ Error: Test case $1 not found!"
        exit 1
    fi
else
    echo "🔄 Running all test suites..."
    overall_status=0
    for test_folder in "$ROOT_DIR/"test-*; do
        if [[ -d "$test_folder" ]]; then
            run_test_case "$test_folder" || overall_status=1
            # Keep the failed cleanup's process identity for the EXIT retry.
            # Another suite must not replace it while its children may be alive.
            [[ "$CLEANUP_FAILED" == true ]] && break
        fi
    done
    print_benchmark_summary
    exit "$overall_status"
fi
