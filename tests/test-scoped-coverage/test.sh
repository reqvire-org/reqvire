#!/bin/bash
set -uo pipefail

# Exercise scoped coverage through the CLI and a persistent HTTP MCP session.
# Membership expectations are authored independently of Reqvire's graph traversal.
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mkdir -p "$TEST_DIR/output"

status=0
python3 "$TEST_SCRIPT_DIR/check_scoped_coverage.py" \
  --binary "$REQVIRE_BIN" \
  --workspace "$TEST_DIR" \
  --expected "$TEST_SCRIPT_DIR/expected" \
  --fixtures "$TEST_SCRIPT_DIR/fixtures" || status=$?

if ! diff -u "$TEST_SCRIPT_DIR/expected/checks.txt" "$TEST_DIR/output/checks.txt"; then
  echo "FAILED: scoped coverage checks differ (artifacts: $TEST_DIR/output)"
  status=1
fi
exit "$status"
