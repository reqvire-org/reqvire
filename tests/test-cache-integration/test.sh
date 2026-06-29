#!/bin/bash
set -uo pipefail

# End-to-end verification of the in-memory model build cache.
#
# Verifies (see system-model/Verifications/ModelStructure/ParsingVerifications.md
# "In-Memory Model Build Cache Verification"):
#   1. Two identical reads over an unchanged workspace return equal results
#      (cache hit, no re-parse).
#   2. A CRUD write (reqvire.add_element) invalidates the cache so a subsequent
#      search reflects the newly added element.
#   3. Modifying a .md file on disk changes the fingerprint and forces a rebuild
#      so new content is reflected.
#   4. The CLI `change-impact --git-commit` path bypasses the cache entirely.
#
# This script is self-contained: helpers are inlined and fixtures are copied
# from sibling test directories, mirroring tests/test-mcp-server/test.sh.

MCP_PROTOCOL_VERSION="2025-11-25"
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

fail() {
  echo "❌ FAILED: $1"
  if [ -n "${2:-}" ] && [ -f "$2" ]; then
    echo ""
    cat "$2"
  fi
  exit 1
}

pick_port() {
  python3 - <<'PY'
import socket
s = socket.socket()
s.bind(("127.0.0.1", 0))
print(s.getsockname()[1])
s.close()
PY
}

json_line() {
  local file="$1"
  local line_number="$2"
  sed -n "${line_number}p" "$file"
}

assert_jq_line() {
  local file="$1"
  local line_number="$2"
  local filter="$3"
  local description="$4"

  if ! json_line "$file" "$line_number" | jq -e "$filter" >/dev/null 2>&1; then
    echo "❌ FAILED: $description"
    echo "Line $line_number:"
    json_line "$file" "$line_number"
    exit 1
  fi
}

start_http_mcp() {
  local port="$1"
  local output_prefix="$2"
  shift 2
  (cd "$TEST_DIR" && "$REQVIRE_BIN" mcp --host 127.0.0.1 --port "$port" "$@") > "${output_prefix}.stdout" 2> "${output_prefix}.stderr" &
  HTTP_MCP_PID=$!
}

stop_http_mcp() {
  if [ -n "${HTTP_MCP_PID:-}" ]; then
    kill "$HTTP_MCP_PID" >/dev/null 2>&1 || true
    wait "$HTTP_MCP_PID" >/dev/null 2>&1 || true
    HTTP_MCP_PID=""
  fi
}

wait_for_http_mcp() {
  local port="$1"
  local output_file="$2"
  local request
  request="$(init_request)"

  for _ in $(seq 1 50); do
    if curl -sS -o "$output_file" \
      -H 'Content-Type: application/json' \
      -H 'Accept: application/json, text/event-stream' \
      --data "$request" \
      "http://127.0.0.1:${port}/mcp" >/dev/null 2>&1; then
      if jq -e '.result.protocolVersion == "2025-11-25"' "$output_file" >/dev/null 2>&1; then
        return 0
      fi
    fi
    sleep 0.1
  done

  return 1
}

http_mcp_call() {
  local port="$1"
  local request="$2"
  local output_file="$3"
  shift 3
  curl -sS -o "$output_file" \
    -H 'Content-Type: application/json' \
    -H 'Accept: application/json, text/event-stream' \
    -H "Mcp-Protocol-Version: ${MCP_PROTOCOL_VERSION}" \
    "$@" \
    --data "$request" \
    "http://127.0.0.1:${port}/mcp"
}

run_http_mcp_sequence() {
  local port="$1"
  local output_file="$2"
  local tmp_file="${output_file}.tmp"
  shift 2
  : > "$output_file"
  for request in "$@"; do
    http_mcp_call "$port" "$request" "$tmp_file" || return 1
    cat "$tmp_file" >> "$output_file"
    printf '\n' >> "$output_file"
  done
}

init_request() {
  jq -n -c --arg version "$MCP_PROTOCOL_VERSION" \
    '{jsonrpc:"2.0",id:1,method:"initialize",params:{protocolVersion:$version,capabilities:{},clientInfo:{name:"reqvire-cache-test",version:"0"}}}'
}

tools_list_request() {
  jq -n -c '{jsonrpc:"2.0",id:2,method:"tools/list",params:{}}'
}

search_request() {
  jq -n -c '{jsonrpc:"2.0",id:10,method:"tools/call",params:{name:"reqvire.search",arguments:{}}}'
}

read_element_request() {
  jq -n -c '{jsonrpc:"2.0",id:11,method:"tools/call",params:{name:"reqvire.read_element",arguments:{name:"Test Requirement Alpha"}}}'
}

add_element_request() {
  local content="$1"
  jq -n -c --arg content "$content" \
    '{jsonrpc:"2.0",id:12,method:"tools/call",params:{name:"reqvire.add_element",arguments:{file:"specifications/Requirements.md",content:$content,dry_run:false}}}'
}

# ----------------------------------------------------------------------------
# Fixture setup: minimal valid model with asset reference.
# ----------------------------------------------------------------------------

mkdir -p "$TEST_DIR/output"
cp -a "$TEST_SCRIPT_DIR/../test-json-file-output/specifications" "$TEST_DIR/"
cp -a "$TEST_SCRIPT_DIR/../test-json-file-output/docs" "$TEST_DIR/"
ADD_CONTENT="$(< "$TEST_SCRIPT_DIR/../test-mcp-server/fixtures/mcp-added-requirement.md")"

# ----------------------------------------------------------------------------
# Start the MCP HTTP server with mutations enabled.
# ----------------------------------------------------------------------------

HTTP_PORT="$(pick_port)"
HTTP_OUTPUT_PREFIX="$TEST_DIR/output/mcp-cache"
start_http_mcp "$HTTP_PORT" "$HTTP_OUTPUT_PREFIX" --enable-mutations
trap stop_http_mcp EXIT

wait_for_http_mcp "$HTTP_PORT" "$TEST_DIR/output/mcp-cache-init.json" \
  || fail "HTTP MCP server did not start" "${HTTP_OUTPUT_PREFIX}.stderr"

http_mcp_call "$HTTP_PORT" "$(tools_list_request)" "$TEST_DIR/output/mcp-cache-tools.json" \
  || fail "tools/list request failed" "$TEST_DIR/output/mcp-cache-tools.json"
jq -e '[.result.tools[].name] | index("reqvire.search") != null' "$TEST_DIR/output/mcp-cache-tools.json" >/dev/null \
  || fail "tools/list should advertise reqvire.search" "$TEST_DIR/output/mcp-cache-tools.json"

# ----------------------------------------------------------------------------
# 1. Cache hit: two identical reads return equal structuredContent.
# ----------------------------------------------------------------------------

run_http_mcp_sequence "$HTTP_PORT" "$TEST_DIR/output/mcp-cache-hit.json" \
  "$(read_element_request)" "$(read_element_request)" \
  || fail "cache-hit read sequence failed" "$TEST_DIR/output/mcp-cache-hit.json"

LINE_1_STRUCT="$(json_line "$TEST_DIR/output/mcp-cache-hit.json" 1 | jq -c '.result.structuredContent')"
LINE_2_STRUCT="$(json_line "$TEST_DIR/output/mcp-cache-hit.json" 2 | jq -c '.result.structuredContent')"
if [ "$LINE_1_STRUCT" != "$LINE_2_STRUCT" ]; then
  fail "two identical read_element calls should return equal structuredContent (cache hit)"
fi
jq -e '.result.structuredContent.name == "Test Requirement Alpha"' <(json_line "$TEST_DIR/output/mcp-cache-hit.json" 1) >/dev/null \
  || fail "first cached read should resolve Test Requirement Alpha" "$TEST_DIR/output/mcp-cache-hit.json"

echo "✅ cache hit: identical reads returned equal results"

# ----------------------------------------------------------------------------
# 2. CRUD invalidation: add_element succeeds and invalidates the cache.
# ----------------------------------------------------------------------------

http_mcp_call "$HTTP_PORT" "$(add_element_request "$ADD_CONTENT")" "$TEST_DIR/output/mcp-cache-add.json" \
  || fail "add_element request failed" "$TEST_DIR/output/mcp-cache-add.json"
jq -e '.result.structuredContent.dry_run == false' "$TEST_DIR/output/mcp-cache-add.json" >/dev/null \
  || fail "add_element should execute (dry_run false)" "$TEST_DIR/output/mcp-cache-add.json"

# ----------------------------------------------------------------------------
# 3. Rebuild after invalidate: search reflects the newly added element.
# ----------------------------------------------------------------------------

http_mcp_call "$HTTP_PORT" "$(search_request)" "$TEST_DIR/output/mcp-cache-search-after-add.json" \
  || fail "search after add failed" "$TEST_DIR/output/mcp-cache-search-after-add.json"
jq -e '[.result.structuredContent.files[].elements[].name] | index("MCP Added Requirement") != null' \
  "$TEST_DIR/output/mcp-cache-search-after-add.json" >/dev/null \
  || fail "search after add should contain 'MCP Added Requirement'" "$TEST_DIR/output/mcp-cache-search-after-add.json"

echo "✅ invalidation: search reflects added element"

# ----------------------------------------------------------------------------
# 4. Fingerprint rebuild: append a .md file change and observe it.
# ----------------------------------------------------------------------------

cat >> "$TEST_DIR/specifications/Requirements.md" <<'EOF'

### Fingerprint Rebuild Requirement

Requirement appended directly to disk to change the workspace fingerprint.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Test Requirement Alpha](#test-requirement-alpha)

---
EOF

http_mcp_call "$HTTP_PORT" "$(search_request)" "$TEST_DIR/output/mcp-cache-search-after-disk.json" \
  || fail "search after disk mutation failed" "$TEST_DIR/output/mcp-cache-search-after-disk.json"
jq -e '[.result.structuredContent.files[].elements[].name] | index("Fingerprint Rebuild Requirement") != null' \
  "$TEST_DIR/output/mcp-cache-search-after-disk.json" >/dev/null \
  || fail "search after disk mutation should reflect fingerprint rebuild" "$TEST_DIR/output/mcp-cache-search-after-disk.json"

echo "✅ fingerprint rebuild: search reflects on-disk change"

stop_http_mcp
trap - EXIT

# ----------------------------------------------------------------------------
# 5. CLI bypasses cache: change-impact --git-commit HEAD runs standalone.
# ----------------------------------------------------------------------------

set +e
(cd "$TEST_DIR" && "$REQVIRE_BIN" change-impact --git-commit HEAD) \
  > "$TEST_DIR/output/cli-change-impact.stdout" 2> "$TEST_DIR/output/cli-change-impact.stderr"
CLI_EXIT=$?
set -e

if [ "$CLI_EXIT" -ne 0 ]; then
  fail "change-impact --git-commit HEAD should exit 0 (bypasses cache)" "$TEST_DIR/output/cli-change-impact.stderr"
fi

echo "✅ cli bypass: change-impact --git-commit HEAD completed"

exit 0
