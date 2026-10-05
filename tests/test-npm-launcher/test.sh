#!/bin/bash
set -euo pipefail
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
python3 "$TEST_SCRIPT_DIR/check_launcher.py" "$TEST_DIR/output"
diff -u "$TEST_SCRIPT_DIR/expected/checks.txt" "$TEST_DIR/output/checks.txt"
