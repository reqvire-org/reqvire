#!/usr/bin/env bash
set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$TEST_DIR" || exit 1
mkdir -p "$TEST_DIR/output"
"$REQVIRE_BIN" validate > "$TEST_DIR/output/validation.txt" 2>&1 || { cat "$TEST_DIR/output/validation.txt"; exit 1; }
python3 "$SCRIPT_DIR/check.py" "$REQVIRE_BIN" "$TEST_DIR" > "$TEST_DIR/output/checks.txt" || { cat "$TEST_DIR/output/checks.txt"; exit 1; }
diff -u "$SCRIPT_DIR/expected/checks.txt" "$TEST_DIR/output/checks.txt"
