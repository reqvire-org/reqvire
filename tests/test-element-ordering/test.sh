#!/bin/bash
# Test: Element Ordering Verification
# Verifies: Element Ordering Normalization, Format Command
#
# This test verifies that the format command reorders elements following
# the Element Ordering Behavior, ensuring parent elements appear before
# their children based on file-local derivedFrom hierarchy.

set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Helper function to compare files and show diff on failure
assert_file_matches() {
    local expected="$1"
    local actual="$2"
    local description="$3"

    if ! diff -u "$expected" "$actual"; then
        echo "❌ FAILED: $description"
        echo ""
        echo "If changes are intentional, update $expected"
        exit 1
    fi
}

# Run format command
cd "${TEST_DIR}" && "${REQVIRE_BIN}" format --fix > /dev/null 2>&1

# Compare with expected output
assert_file_matches "${TEST_SCRIPT_DIR}/expected/Unordered.md" \
    "${TEST_DIR}/requirements/Unordered.md" \
    "Element ordering after format does not match expected"

exit 0
