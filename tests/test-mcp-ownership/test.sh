#!/bin/bash
set -uo pipefail
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
status=0
# Process-lifecycle checks must signal the worker, not the timing wrapper.
python3 "$TEST_SCRIPT_DIR/check_locks.py" "${REAL_REQVIRE_BIN:-$REQVIRE_BIN}" > "$TEST_DIR/output/locks.txt" || status=1
diff -u "$TEST_SCRIPT_DIR/expected/locks.txt" "$TEST_DIR/output/locks.txt" || status=1
python3 "$TEST_SCRIPT_DIR/check_git_observations.py" "${REAL_REQVIRE_BIN:-$REQVIRE_BIN}" > "$TEST_DIR/output/git-observations.txt" || status=1
diff -u "$TEST_SCRIPT_DIR/expected/git-observations.txt" "$TEST_DIR/output/git-observations.txt" || status=1
python3 "$TEST_SCRIPT_DIR/check_parallel_reads.py" --binary "${REAL_REQVIRE_BIN:-$REQVIRE_BIN}" > "$TEST_DIR/output/parallel-reads.txt" || status=1
diff -u "$TEST_SCRIPT_DIR/expected/parallel-reads.txt" "$TEST_DIR/output/parallel-reads.txt" || status=1
python3 "$TEST_SCRIPT_DIR/check_ownership.py" "$REQVIRE_BIN" "$TEST_DIR" > "$TEST_DIR/output/checks.txt" || status=1
diff -u "$TEST_SCRIPT_DIR/expected/checks.txt" "$TEST_DIR/output/checks.txt" || status=1

# General existing-element selection: names and canonical identifiers share validation and results.
SELECTION_SUITE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SELECTION_SUITE_DIR/../run_element_selection_checks.sh"
run_element_selection_checks mcp "$SELECTION_SUITE_DIR/expected/element-selection.txt" || status=1
exit "$status"
