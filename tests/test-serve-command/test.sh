#!/bin/bash

# Test: Serve Command Functionality
# --------------------------------------
# Satisfies: specifications/Verifications/Misc.md#serve-command-verification
#
# Acceptance Criteria:
# - System starts HTTP server on specified host and port
# - System displays clickable terminal link to server URL
# - System serves index.html when accessing root URL
# - System serves embedded Explorer assets and generated Project Store data
# - System serves ontologies.ttl
# - System returns index.html for non-asset browser routes
# - System returns 404 for missing asset files
# - System sets correct Content-Type headers for different file types
# - System runs in quiet mode without verbose runtime-generation output
# - System displays instructions for Ctrl-C
#
# Test Criteria:
# - Command exits with success (0) return code
# - Server responds to HTTP requests on specified port
# - Root URL (/) serves index.html
# - HTML files are served with text/html content type
# - SVG files are served with image/svg+xml content type
# - Missing embedded asset paths return 404 status
# - Non-asset browser routes return index.html for SPA fallback
# - Runtime-generation verbose output is suppressed (quiet mode active)

set -e

# Use non-default host and random port to test custom options
TEST_HOST="127.0.0.1"
TEST_PORT=$((8000 + RANDOM % 1000))

# Start serve command in background with non-default host and port
cd "$TEST_DIR"
"$REQVIRE_BIN" serve --host "$TEST_HOST" --port "$TEST_PORT" > "${TEST_DIR}/serve_output.log" 2>&1 &
SERVE_PID=$!

# Function to cleanup server on exit
cleanup() {
    if [ -n "$SERVE_PID" ]; then
        kill "$SERVE_PID" 2>/dev/null || true
        wait "$SERVE_PID" 2>/dev/null || true
    fi
    rm -rf "${TEST_DIR}"
}
trap cleanup EXIT

# Wait for server to start (max 10 seconds)
echo "Waiting for server to start on $TEST_HOST:$TEST_PORT..."
for i in {1..20}; do
    if curl -s "http://$TEST_HOST:$TEST_PORT/" >/dev/null 2>&1; then
        echo "Server started successfully on $TEST_HOST:$TEST_PORT"
        break
    fi
    if [ $i -eq 20 ]; then
        if grep -qi "Operation not permitted" "${TEST_DIR}/serve_output.log"; then
            echo "⚠ SKIPPED: Serve test cannot bind in this environment"
            exit 0
        fi
        echo "❌ FAILED: Server did not start within 10 seconds"
        cat "${TEST_DIR}/serve_output.log"
        exit 1
    fi
    sleep 0.5
done

# Test 1: Check that instructions are displayed
if ! grep -q "Server running at:" "${TEST_DIR}/serve_output.log"; then
    echo "❌ FAILED: Server URL not displayed in output"
    cat "${TEST_DIR}/serve_output.log"
    exit 1
fi

if ! grep -q "Press Ctrl-C to stop" "${TEST_DIR}/serve_output.log"; then
    echo "❌ FAILED: Instructions for Ctrl-C not displayed"
    cat "${TEST_DIR}/serve_output.log"
    exit 1
fi

# Test 2: Root URL serves index.html
RESPONSE=$(curl -s -w "\n%{http_code}" "http://$TEST_HOST:$TEST_PORT/")
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
CONTENT=$(echo "$RESPONSE" | sed '$d')

if [ "$HTTP_CODE" != "200" ]; then
    echo "❌ FAILED: Root URL returned HTTP $HTTP_CODE instead of 200"
    exit 1
fi

if ! echo "$CONTENT" | grep -qi "<!doctype html>"; then
    echo "❌ FAILED: Root URL did not return HTML content"
    exit 1
fi

# Root URL must serve the compiled React Explorer SPA bundle.
if ! echo "$CONTENT" | grep -q '<div id="root"></div>' || ! echo "$CONTENT" | grep -q "assets/explorer.js"; then
    echo "❌ FAILED: Root URL did not return the Explorer SPA bundle"
    exit 1
fi

STORE_RESPONSE=$(curl -s -w "\n%{http_code}" "http://$TEST_HOST:$TEST_PORT/assets/project-store.js")
STORE_CODE=$(echo "$STORE_RESPONSE" | tail -n1)
STORE_CONTENT=$(echo "$STORE_RESPONSE" | sed '$d')
if [ "$STORE_CODE" != "200" ] || ! echo "$STORE_CONTENT" | grep -q "reqvireProjectStore"; then
    echo "❌ FAILED: Project Store data asset was not served"
    exit 1
fi

if ! echo "$STORE_CONTENT" | grep -q '"path": "specifications/Requirements.md"'; then
    echo "❌ FAILED: Project Store is missing modeled source file records"
    exit 1
fi

if ! echo "$STORE_CONTENT" | grep -q '"path": "scripts/evidence.sh"' ||
   ! echo "$STORE_CONTENT" | grep -q '"parent_folder": "scripts"' ||
   ! echo "$STORE_CONTENT" | grep -q '"id": "resource:scripts/evidence.sh"' ||
   ! echo "$STORE_CONTENT" | grep -q 'serve command evidence'; then
    echo "❌ FAILED: Project Store did not include the existing graph-referenced evidence file as a resource-backed tree file"
    exit 1
fi

if echo "$STORE_CONTENT" | grep -q '"path": "notes/unrelated.md"'; then
    echo "❌ FAILED: Project Store included an unrelated repository file in the model tree"
    exit 1
fi

# Test 3: Check Content-Type for HTML files
CONTENT_TYPE=$(curl -s -I "http://$TEST_HOST:$TEST_PORT/" | grep -i "content-type" | cut -d: -f2 | tr -d ' \r')
if [[ ! "$CONTENT_TYPE" =~ ^text/html ]]; then
    echo "❌ FAILED: HTML file has incorrect Content-Type: $CONTENT_TYPE"
    exit 1
fi

# Test 4: Check that ontologies.ttl is generated and served
RESPONSE=$(curl -s -w "\n%{http_code}" "http://$TEST_HOST:$TEST_PORT/ontologies.ttl")
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)

if [ "$HTTP_CODE" != "200" ]; then
    echo "❌ FAILED: ontologies.ttl returned HTTP $HTTP_CODE"
    exit 1
fi

CONTENT_TYPE=$(curl -s -I "http://$TEST_HOST:$TEST_PORT/ontologies.ttl" | grep -i "content-type" | cut -d: -f2 | tr -d ' \r')
if [[ ! "$CONTENT_TYPE" =~ ^text/turtle ]]; then
    echo "❌ FAILED: ontologies.ttl has incorrect Content-Type: $CONTENT_TYPE"
    exit 1
fi

# Test 5: Non-asset browser routes fall back to the SPA shell
RESPONSE=$(curl -s -w "\n%{http_code}" "http://$TEST_HOST:$TEST_PORT/model")
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
CONTENT=$(echo "$RESPONSE" | sed '$d')

if [ "$HTTP_CODE" != "200" ] || ! echo "$CONTENT" | grep -q '<div id="root"></div>'; then
    echo "❌ FAILED: SPA fallback route did not return index.html"
    exit 1
fi

# Test 6: Check 404 for non-existent bundled asset files
RESPONSE=$(curl -s -w "\n%{http_code}" "http://$TEST_HOST:$TEST_PORT/nonexistent.html")
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)

if [ "$HTTP_CODE" != "200" ]; then
    echo "❌ FAILED: Non-asset route returned HTTP $HTTP_CODE instead of SPA fallback"
    exit 1
fi

RESPONSE=$(curl -s -w "\n%{http_code}" "http://$TEST_HOST:$TEST_PORT/assets/missing.js")
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)

if [ "$HTTP_CODE" != "404" ]; then
    echo "❌ FAILED: Missing asset returned HTTP $HTTP_CODE instead of 404"
    exit 1
fi

# Test 7: Verify quiet mode (no verbose runtime-generation output)
if grep -q "Updated diagrams" "${TEST_DIR}/serve_output.log"; then
    echo "❌ FAILED: Diagram update messages present (quiet mode not working)"
    cat "${TEST_DIR}/serve_output.log"
    exit 1
fi

# Clean up
cleanup

echo "✅ PASSED: Serve command test"
exit 0
