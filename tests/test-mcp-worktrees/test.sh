#!/bin/bash
set -uo pipefail
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
status=0
python3 "$TEST_SCRIPT_DIR/check_worktrees.py" "$REQVIRE_BIN" "$TEST_DIR" > "$TEST_DIR/output/checks.txt" || status=1
diff -u "$TEST_SCRIPT_DIR/expected/checks.txt" "$TEST_DIR/output/checks.txt" || status=1
exit "$status"
