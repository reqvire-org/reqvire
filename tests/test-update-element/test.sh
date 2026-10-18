#!/usr/bin/env bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
status=0
python3 "$TEST_SCRIPT_DIR/check_update.py" "$REAL_REQVIRE_BIN" "$TEST_DIR" \
  > "$TEST_DIR/output/update-checks.txt" 2> "$TEST_DIR/output/update-checks.stderr" || status=$?
cat "$TEST_DIR/output/update-checks.txt"
cat "$TEST_DIR/output/update-checks.stderr" >&2
diff -u "$TEST_SCRIPT_DIR/expected/checks.txt" "$TEST_DIR/output/update-checks.txt" || status=1
exit "$status"
