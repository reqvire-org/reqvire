#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
python3 "$SCRIPT_DIR/check_boundary.py" "$REQVIRE_BIN" "$TEST_DIR" "$SCRIPT_DIR"
diff -u "$SCRIPT_DIR/expected/checks.txt" "$TEST_DIR/checks.txt"
