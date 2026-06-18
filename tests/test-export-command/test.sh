#!/bin/bash

# Test: Export Command Functionality
# --------------------------------------
# Satisfies: system-model/Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#export-command-verification
#
# Acceptance Criteria:
# - System writes index.html to the output directory
# - System writes assets/project-store.js containing window.reqvireProjectStore
# - System writes ontologies.ttl to the output directory
# - System writes all other embedded SPA bundle assets
# - Output directory is self-contained (no CDN-loaded resources)
# - System runs in quiet mode without verbose runtime-generation output

set -e

EXPORT_DIR="${TEST_DIR}/exported-site"

cd "$TEST_DIR"
"$REQVIRE_BIN" export --output "$EXPORT_DIR" > "${TEST_DIR}/export_output.log" 2>&1

# Test 1: index.html exists and contains the Explorer SPA shell
if [ ! -f "$EXPORT_DIR/index.html" ]; then
    echo "❌ FAILED: index.html not found in export output"
    exit 1
fi

if ! grep -qi "<!doctype html>" "$EXPORT_DIR/index.html"; then
    echo "❌ FAILED: index.html does not contain DOCTYPE"
    exit 1
fi

if ! grep -q '<div id="root"></div>' "$EXPORT_DIR/index.html" || ! grep -q "assets/explorer.js" "$EXPORT_DIR/index.html"; then
    echo "❌ FAILED: index.html does not contain the Explorer SPA bundle reference"
    exit 1
fi

# Test 2: assets/project-store.js exists and contains the project store
if [ ! -f "$EXPORT_DIR/assets/project-store.js" ]; then
    echo "❌ FAILED: assets/project-store.js not found in export output"
    exit 1
fi

if ! grep -q "reqvireProjectStore" "$EXPORT_DIR/assets/project-store.js"; then
    echo "❌ FAILED: assets/project-store.js does not contain reqvireProjectStore"
    exit 1
fi

if ! grep -q '"path": "specifications/Requirements.md"' "$EXPORT_DIR/assets/project-store.js"; then
    echo "❌ FAILED: Project Store is missing modeled source file records"
    exit 1
fi

if ! grep -q '"path": "scripts/evidence.sh"' "$EXPORT_DIR/assets/project-store.js" ||
   ! grep -q '"parent_folder": "scripts"' "$EXPORT_DIR/assets/project-store.js" ||
   ! grep -q '"id": "resource:scripts/evidence.sh"' "$EXPORT_DIR/assets/project-store.js" ||
   ! grep -q 'export command evidence' "$EXPORT_DIR/assets/project-store.js"; then
    echo "❌ FAILED: Project Store did not include the existing graph-referenced evidence file as a resource-backed tree file"
    exit 1
fi

if grep -q '"path": "notes/unrelated.md"' "$EXPORT_DIR/assets/project-store.js"; then
    echo "❌ FAILED: Project Store included an unrelated repository file in the model tree"
    exit 1
fi

# Test 3: ontologies.ttl exists
if [ ! -f "$EXPORT_DIR/ontologies.ttl" ]; then
    echo "❌ FAILED: ontologies.ttl not found in export output"
    exit 1
fi

# Test 4: No CDN references in index.html (self-contained)
if grep -qE "https?://(cdn\.|unpkg\.|jsdelivr\.|cdnjs\.)" "$EXPORT_DIR/index.html"; then
    echo "❌ FAILED: index.html references external CDN resources"
    exit 1
fi

# Test 5: Quiet mode — no verbose runtime-generation output
if grep -q "Updated diagrams" "${TEST_DIR}/export_output.log"; then
    echo "❌ FAILED: Diagram update messages present (quiet mode not working)"
    cat "${TEST_DIR}/export_output.log"
    exit 1
fi

# Test 6: Success message printed
if ! grep -q "Exported Explorer site to:" "${TEST_DIR}/export_output.log"; then
    echo "❌ FAILED: Export success message not printed"
    cat "${TEST_DIR}/export_output.log"
    exit 1
fi

echo "✅ PASSED: Export command test"
exit 0
