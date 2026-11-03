#!/bin/bash
set -euo pipefail

SUITE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
python3 "$SUITE_DIR/check_runner.py" "$TEST_DIR/output/checks.txt"
bash "$SUITE_DIR/check_temp_paths.sh" "$TEST_DIR/output/path-alias" >> "$TEST_DIR/output/checks.txt"
diff -u "$SUITE_DIR/expected/checks.txt" "$TEST_DIR/output/checks.txt"
