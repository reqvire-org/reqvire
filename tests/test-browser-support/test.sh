#!/bin/bash
set -euo pipefail
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
node --test --test-isolation=none \
  "$TEST_SCRIPT_DIR/../browser.test.mjs" \
  "$TEST_SCRIPT_DIR/../test-serve-command/scripts/refresh-instrumentation.test.mjs" \
  > "$TEST_DIR/output/browser-support.log" 2>&1 || {
    cat "$TEST_DIR/output/browser-support.log"
    exit 1
  }
python3 "$TEST_SCRIPT_DIR/../test-scoped-coverage/browser-runner.test.py"
printf 'PASS browser-support\n' > "$TEST_DIR/output/checks.txt"
diff -u "$TEST_SCRIPT_DIR/expected/checks.txt" "$TEST_DIR/output/checks.txt"
