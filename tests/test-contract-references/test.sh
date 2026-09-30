#!/bin/bash
set -uo pipefail
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mkdir -p "$TEST_DIR/output"
status=0
python3 "$TEST_SCRIPT_DIR/check_references.py" || status=$?
if ! diff -u "$TEST_SCRIPT_DIR/expected/checks.txt" "$TEST_DIR/output/checks.txt"; then
  status=1
fi
exit "$status"
