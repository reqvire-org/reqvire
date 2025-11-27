#!/bin/bash
# Test: Element Ordering Verification
# Verifies: Element Ordering Normalization, Format Command
#
# This test verifies that the format command reorders elements following
# the Element Ordering Behavior, ensuring parent elements appear before
# their children based on file-local derivedFrom hierarchy.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REQVIRE="${REQVIRE:-$SCRIPT_DIR/../../target/debug/reqvire}"

cd "$SCRIPT_DIR"

echo "=== Element Ordering Test ==="
echo ""

# Test 1: Verify initial file is unordered (child before parent)
echo "Test 1: Checking initial unordered state..."
FIRST_ELEMENT=$(grep -m1 "^### " requirements/Unordered.md | sed 's/### //')
if [ "$FIRST_ELEMENT" = "Child B" ]; then
    echo "  ✓ Initial file has child before parent (unordered state confirmed)"
else
    echo "  ✗ FAIL: Expected 'Child B' as first element, got '$FIRST_ELEMENT'"
    exit 1
fi

# Test 2: Run format and check that it reorders elements
echo ""
echo "Test 2: Running format command..."
$REQVIRE format --fix 2>&1 || true

# Test 3: Verify elements are now ordered correctly
echo ""
echo "Test 3: Verifying element ordering after format..."

# Extract element names in order
ELEMENTS=$(grep "^### " requirements/Unordered.md | sed 's/### //')
EXPECTED_ORDER="Parent A
Child A
Grandchild M
Grandchild Z
Child B
Standalone Element"

if [ "$ELEMENTS" = "$EXPECTED_ORDER" ]; then
    echo "  ✓ Elements are correctly ordered:"
    echo "$ELEMENTS" | sed 's/^/      /'
else
    echo "  ✗ FAIL: Element ordering incorrect"
    echo ""
    echo "  Expected order:"
    echo "$EXPECTED_ORDER" | sed 's/^/      /'
    echo ""
    echo "  Actual order:"
    echo "$ELEMENTS" | sed 's/^/      /'
    exit 1
fi

# Test 4: Compare with expected file
echo ""
echo "Test 4: Comparing with expected output..."
if diff -q requirements/Unordered.md expected/Unordered.md > /dev/null 2>&1; then
    echo "  ✓ Output matches expected file"
else
    echo "  ✗ FAIL: Output differs from expected"
    echo ""
    echo "  Differences:"
    diff requirements/Unordered.md expected/Unordered.md || true
    exit 1
fi

echo ""
echo "=== All Element Ordering Tests Passed ==="
