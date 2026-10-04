#!/bin/bash
set -uo pipefail
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
node --test --test-isolation=none "$TEST_SCRIPT_DIR/../test-serve-command/scripts/browser.test.mjs" > "$TEST_DIR/output/browser-lifecycle.txt" 2>&1 || {
    cat "$TEST_DIR/output/browser-lifecycle.txt"
    exit 1
}
status=0
python3 "$TEST_SCRIPT_DIR/check_worktrees.py" "$REQVIRE_BIN" "$TEST_DIR" > "$TEST_DIR/output/checks.txt" || status=1
diff -u "$TEST_SCRIPT_DIR/expected/checks.txt" "$TEST_DIR/output/checks.txt" || status=1
exit "$status"
