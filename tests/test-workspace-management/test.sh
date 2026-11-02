#!/bin/bash
# Native workspace preparation and management through CLI and typed MCP.
set -uo pipefail
TEST_SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
REQVIRE_SOURCE_ROOT=$(cd "$TEST_SCRIPT_DIR/../.." && pwd)
export TEST_SCRIPT_DIR REQVIRE_SOURCE_ROOT
mkdir -p "$TEST_DIR/output/logs" || exit 2
: > "$TEST_DIR/output/checks.txt"
status=0

run_case() {
    local backend="$1" name="$2" label="$1/$2" log="$TEST_DIR/output/logs/$1-$2.log"
    if bash "$TEST_SCRIPT_DIR/cases.sh" "$backend" "$name" > "$log" 2>&1; then
        printf 'PASS %s\n' "$label" >> "$TEST_DIR/output/checks.txt"
    else
        printf 'FAIL %s\n' "$label" >> "$TEST_DIR/output/checks.txt"
        echo "FAILED $label (log: $log)"
        tail -n 12 "$log"
        status=1
    fi
}

run_case model workspace-boundaries
for name in status add remove sync latest initial-ref freeze dirty-freeze selection single invalid-manifest migrate migrate-conflict dirty-sync diverged-sync latest-without-ref partial-sync migration-impact; do
    run_case cli "$name"
done
for name in discovery status add remove sync latest freeze dirty-freeze migrate readonly dirty-sync diverged-sync latest-without-ref shared-ownership rejected-composition subdirectory; do
    run_case mcp "$name"
done
diff -u "$TEST_SCRIPT_DIR/expected/checks.txt" "$TEST_DIR/output/checks.txt" || status=1
exit "$status"
