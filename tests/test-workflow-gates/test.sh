#!/bin/bash
set -euo pipefail
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
bash "$TEST_SCRIPT_DIR/check_workflows.sh" "$REQVIRE_BIN" "$TEST_DIR/output/checks.txt"
diff -u "$TEST_SCRIPT_DIR/expected/checks.txt" "$TEST_DIR/output/checks.txt"
