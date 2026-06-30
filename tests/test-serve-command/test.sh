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

stop_server() {
    if [ -n "${SERVE_PID:-}" ]; then
        kill "$SERVE_PID" 2>/dev/null || true
        wait "$SERVE_PID" 2>/dev/null || true
        SERVE_PID=""
    fi
}

# Function to cleanup server on exit
cleanup() {
    stop_server
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

if echo "$STORE_CONTENT" | grep -q '"path": "scripts/evidence.sh"'; then
    echo "❌ FAILED: Project Store included a resource-only evidence file in the model tree"
    exit 1
fi

if ! echo "$STORE_CONTENT" | grep -q '"file_path": "scripts/evidence.sh"' ||
   ! echo "$STORE_CONTENT" | grep -q '"id": "resource:scripts/evidence.sh"' ||
   ! echo "$STORE_CONTENT" | grep -q 'serve command evidence'; then
    echo "❌ FAILED: Project Store did not include the existing graph-referenced evidence file as a resource"
    exit 1
fi

if echo "$STORE_CONTENT" | grep -q '"path": "notes/unrelated.md"'; then
    echo "❌ FAILED: Project Store included an unrelated repository file in the model tree"
    exit 1
fi

printf '\n' >> "$TEST_DIR/specifications/Requirements.md"
cat "$TEST_DIR/fixtures/direct-filesystem-store-regeneration-sentinel.md.txt" >> "$TEST_DIR/specifications/Requirements.md"

REFRESH_RESPONSE=$(curl -s -w "\n%{http_code}" "http://$TEST_HOST:$TEST_PORT/assets/project-store.js")
REFRESH_CODE=$(echo "$REFRESH_RESPONSE" | tail -n1)
REFRESH_CONTENT=$(echo "$REFRESH_RESPONSE" | sed '$d')
if [ "$REFRESH_CODE" != "200" ]; then
    echo "❌ FAILED: Project Store refresh request returned HTTP $REFRESH_CODE"
    exit 1
fi

if echo "$REFRESH_CONTENT" | grep -q "Direct Filesystem Store Regeneration Sentinel"; then
    echo "❌ FAILED: Project Store GET regenerated from disk instead of serving the cached runtime store"
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

stop_server

# Test 8: Embedded MCP endpoint can mutate the workspace and the served datastore refreshes.
MCP_PORT=$((9000 + RANDOM % 1000))
MCP_PROTOCOL_VERSION="2025-11-25"
MCP_CONTENT="$(cat "${TEST_DIR}/fixtures/serve-embedded-mcp-added-requirement.md.txt")"

"$REQVIRE_BIN" serve --host "$TEST_HOST" --port "$MCP_PORT" --enable-mcp --enable-mutations > "${TEST_DIR}/serve_mcp_output.log" 2>&1 &
SERVE_PID=$!

echo "Waiting for embedded MCP server to start on $TEST_HOST:$MCP_PORT..."
for i in {1..20}; do
    if curl -s "http://$TEST_HOST:$MCP_PORT/" >/dev/null 2>&1; then
        echo "Embedded MCP server started successfully on $TEST_HOST:$MCP_PORT"
        break
    fi
    if [ $i -eq 20 ]; then
        if grep -qi "Operation not permitted" "${TEST_DIR}/serve_mcp_output.log"; then
            echo "⚠ SKIPPED: Embedded MCP serve test cannot bind in this environment"
            exit 0
        fi
        echo "❌ FAILED: Embedded MCP server did not start within 10 seconds"
        cat "${TEST_DIR}/serve_mcp_output.log"
        exit 1
    fi
    sleep 0.5
done

if ! grep -q "MCP endpoint: http://$TEST_HOST:$MCP_PORT/mcp" "${TEST_DIR}/serve_mcp_output.log"; then
    echo "❌ FAILED: Embedded MCP endpoint URL not displayed"
    cat "${TEST_DIR}/serve_mcp_output.log"
    exit 1
fi

MCP_INIT_REQUEST="$(jq -n -c --arg version "$MCP_PROTOCOL_VERSION" '{jsonrpc:"2.0",id:1,method:"initialize",params:{protocolVersion:$version,capabilities:{},clientInfo:{name:"reqvire-serve-test",version:"0"}}}')"
MCP_TOOLS_REQUEST="$(jq -n -c '{jsonrpc:"2.0",id:2,method:"tools/list",params:{}}')"
MCP_MUTATION_REQUEST="$(jq -n -c --arg content "$MCP_CONTENT" '{jsonrpc:"2.0",id:3,method:"tools/call",params:{name:"reqvire.add_element",arguments:{file:"specifications/Requirements.md",content:$content,dry_run:false}}}')"

curl -sS -o "${TEST_DIR}/serve_mcp_init.json" \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json, text/event-stream' \
  --data "$MCP_INIT_REQUEST" \
  "http://$TEST_HOST:$MCP_PORT/mcp"

if ! jq -e '.result.protocolVersion == "2025-11-25"' "${TEST_DIR}/serve_mcp_init.json" >/dev/null; then
    echo "❌ FAILED: Embedded MCP initialize failed"
    cat "${TEST_DIR}/serve_mcp_init.json"
    exit 1
fi

curl -sS -o "${TEST_DIR}/serve_mcp_tools.json" \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json, text/event-stream' \
  -H "Mcp-Protocol-Version: $MCP_PROTOCOL_VERSION" \
  --data "$MCP_TOOLS_REQUEST" \
  "http://$TEST_HOST:$MCP_PORT/mcp"

if ! jq -e '[.result.tools[].name] | index("reqvire.add_element") != null and index("reqvire.link") != null' "${TEST_DIR}/serve_mcp_tools.json" >/dev/null; then
    echo "❌ FAILED: Embedded MCP mutation tools are not advertised"
    cat "${TEST_DIR}/serve_mcp_tools.json"
    exit 1
fi

curl -sS -o "${TEST_DIR}/serve_mcp_mutation.json" \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json, text/event-stream' \
  -H "Mcp-Protocol-Version: $MCP_PROTOCOL_VERSION" \
  --data "$MCP_MUTATION_REQUEST" \
  "http://$TEST_HOST:$MCP_PORT/mcp"

if ! jq -e '.result.structuredContent.dry_run == false and (.result.structuredContent.diffs | length) >= 1' "${TEST_DIR}/serve_mcp_mutation.json" >/dev/null; then
    echo "❌ FAILED: Embedded MCP mutation did not execute"
    cat "${TEST_DIR}/serve_mcp_mutation.json"
    exit 1
fi

grep -q "Serve Embedded MCP Added Requirement" "$TEST_DIR/specifications/Requirements.md" || {
    echo "❌ FAILED: Embedded MCP mutation did not persist to the fixture file"
    exit 1
}

STORE_HEADERS="${TEST_DIR}/serve_mcp_project_store.headers"
curl -sS -D "$STORE_HEADERS" -o "${TEST_DIR}/serve_mcp_project_store.js" "http://$TEST_HOST:$MCP_PORT/assets/project-store.js"
if ! grep -qi '^cache-control: no-store' "$STORE_HEADERS"; then
    echo "❌ FAILED: Project Store response is missing no-store cache control"
    cat "$STORE_HEADERS"
    exit 1
fi

if ! grep -q "Serve Embedded MCP Added Requirement" "${TEST_DIR}/serve_mcp_project_store.js"; then
    echo "❌ FAILED: Project Store did not refresh after embedded MCP mutation"
    exit 1
fi

# Clean up
cleanup

echo "✅ PASSED: Serve command test"
exit 0
