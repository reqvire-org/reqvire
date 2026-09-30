#!/usr/bin/env bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mkdir -p "$TEST_DIR/output"
python3 "$TEST_SCRIPT_DIR/check_rollup.py" "$REQVIRE_BIN" "$TEST_DIR" "$TEST_SCRIPT_DIR/expected"
