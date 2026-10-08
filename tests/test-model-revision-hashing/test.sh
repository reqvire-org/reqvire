#!/bin/bash
set -uo pipefail

# Model Revision Hash Stability Verification: exercise the real HTTP MCP server.
# The standard runner supplies a temporary Git workspace and the local CLI.
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mkdir -p "$TEST_DIR/output"

status=0
python3 "$TEST_SCRIPT_DIR/check_revision.py" \
  --binary "$REQVIRE_BIN" \
  --workspace "$TEST_DIR" \
  --fixtures "$TEST_SCRIPT_DIR/fixtures" \
  --expected "$TEST_SCRIPT_DIR/expected" \
  || status=$?

if [ "$status" -ne 0 ]; then
  echo "❌ FAILED: model revision hashing checks (artifacts: $TEST_DIR/output)"
fi
if ! diff -u "$TEST_SCRIPT_DIR/expected/checks.txt" "$TEST_DIR/output/checks.txt"; then
  echo "❌ FAILED: model revision hashing results differ from expected output"
  status=1
fi
exit "$status"
