#!/bin/bash
set -uo pipefail
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mkdir -p "$TEST_DIR/output"
python3 "$TEST_SCRIPT_DIR/check_http_access.py" "$REQVIRE_BIN" "$TEST_DIR" > "$TEST_DIR/output/checks.txt" || {
  cat "$TEST_DIR/output/checks.txt"
  exit 1
}
diff -u "$TEST_SCRIPT_DIR/expected/checks.txt" "$TEST_DIR/output/checks.txt"
