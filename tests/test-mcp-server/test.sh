#!/bin/bash
set -uo pipefail

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
    '{jsonrpc:"2.0",id:1,method:"initialize",params:{protocolVersion:$version,capabilities:{},clientInfo:{name:"reqvire-test",version:"0"}}}'
}

tools_list_request() {
  jq -n -c '{jsonrpc:"2.0",id:2,method:"tools/list",params:{}}'
}

resources_list_request() {
  jq -n -c '{jsonrpc:"2.0",id:3,method:"resources/list",params:{}}'
}

workspace_status_request() {
  jq -n -c '{jsonrpc:"2.0",id:4,method:"tools/call",params:{name:"reqvire.workspace_status",arguments:{}}}'
}

read_element_request() {
  jq -n -c '{jsonrpc:"2.0",id:5,method:"tools/call",params:{name:"reqvire.read_element",arguments:{name:"Test Requirement Beta"}}}'
}

semantic_requirement_request() {
  jq -n -c '{jsonrpc:"2.0",id:11,method:"tools/call",params:{name:"reqvire.read_element",arguments:{name:"MCP Semantic Requirement"}}}'
}

ontology_search_request() {
  jq -n -c '{jsonrpc:"2.0",id:12,method:"tools/call",params:{name:"reqvire.search",arguments:{filter_type:"ontology"}}}'
}

ontologies_request() {
  jq -n -c '{jsonrpc:"2.0",id:14,method:"tools/call",params:{name:"reqvire.ontologies",arguments:{}}}'
}

ontologies_jsonld_request() {
  jq -n -c '{jsonrpc:"2.0",id:15,method:"tools/call",params:{name:"reqvire.ontologies",arguments:{format:"jsonld"}}}'
}

ontologies_full_request() {
  jq -n -c '{jsonrpc:"2.0",id:16,method:"tools/call",params:{name:"reqvire.ontologies",arguments:{full:true}}}'
}

model_request() {
  jq -n -c '{jsonrpc:"2.0",id:13,method:"tools/call",params:{name:"reqvire.model",arguments:{from:"Test Requirement Alpha"}}}'
}

collect_request() {
  jq -n -c '{jsonrpc:"2.0",id:6,method:"tools/call",params:{name:"reqvire.collect",arguments:{element_name:"Test Requirement Beta"}}}'
}

schema_error_request() {
  jq -n -c '{jsonrpc:"2.0",id:7,method:"tools/call",params:{name:"reqvire.collect",arguments:{}}}'
}

unknown_tool_request() {
  jq -n -c '{jsonrpc:"2.0",id:8,method:"tools/call",params:{name:"reqvire.validate",arguments:{}}}'
}

resource_read_request() {
  jq -n -c '{jsonrpc:"2.0",id:9,method:"resources/read",params:{uri:"reqvire://workspace/status"}}'
}

format_fix_rejected_request() {
  jq -n -c '{jsonrpc:"2.0",id:10,method:"tools/call",params:{name:"reqvire.format",arguments:{fix:true}}}'
}

cp -a "$TEST_SCRIPT_DIR/../test-json-file-output/specifications" "$TEST_DIR/"
cp -a "$TEST_SCRIPT_DIR/../test-json-file-output/docs" "$TEST_DIR/"
cat >> "$TEST_DIR/specifications/Requirements.md" <<'EOF'

### MCP Semantic Capability

MCP semantic capability for readable Access Token model concepts.

#### Metadata
  * type: capability

#### Attachments
  * [MCP Access Token Ontology](#mcp-access-token-ontology)

#### Relations
  * specifiedBy: [MCP Semantic Requirement](#mcp-semantic-requirement)
---

### MCP Access Token Ontology

Access token ontology for MCP semantic evidence.

#### Metadata
  * type: ontology

#### Ontology
```turtle
@prefix mcp: <urn:reqvire:test:mcp:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

mcp:AccessToken a owl:Class .
mcp:subject a owl:ObjectProperty .
```
---

### MCP Semantic Requirement

The system shall expose MCP semantic evidence for readable Access Token model concepts.

#### Metadata
  * type: requirement

#### Concept References
  * Access Token: mcp:AccessToken

#### Relations
  * specify: [MCP Semantic Capability](#mcp-semantic-capability)
  * refinedBy: [MCP Access Token Shape Contract](#mcp-access-token-shape-contract)
---

### MCP Access Token Shape Contract

MCP access token shape contract.

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [MCP Semantic Requirement](#mcp-semantic-requirement)

#### Shapes
```turtle
@prefix mcp: <urn:reqvire:test:mcp:> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

mcp:AccessTokenShape
  a sh:NodeShape ;
  sh:targetClass mcp:AccessToken ;
  sh:property [
    sh:path mcp:subject ;
  ] .
```
---
EOF

(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json --filter-name "Test Requirement Beta") > "$TEST_DIR/output/binary-search.json" 2>&1 \
  || fail "Reqvire binary search failed" "$TEST_DIR/output/binary-search.json"
jq -e '.global_counters.total_elements == 1' "$TEST_DIR/output/binary-search.json" >/dev/null \
  || fail "Reqvire binary search should find Test Requirement Beta" "$TEST_DIR/output/binary-search.json"

set +e
(cd "$TEST_DIR" && "$REQVIRE_BIN" mcp --transport stdio) > "$TEST_DIR/output/mcp-stdio-removed.stdout" 2> "$TEST_DIR/output/mcp-stdio-removed.stderr"
STDIO_EXIT=$?
set +e
if [ "$STDIO_EXIT" -eq 0 ]; then
  fail "stdio MCP transport should not be accepted"
fi

DEFAULT_OUTPUT="$TEST_DIR/output/mcp-default.jsonl"
DEFAULT_PORT="$(pick_port)"
DEFAULT_OUTPUT_PREFIX="$TEST_DIR/output/mcp-default"
start_http_mcp "$DEFAULT_PORT" "$DEFAULT_OUTPUT_PREFIX"
trap stop_http_mcp EXIT
wait_for_http_mcp "$DEFAULT_PORT" "$TEST_DIR/output/mcp-default-init.json" || fail "default MCP HTTP server did not start" "${DEFAULT_OUTPUT_PREFIX}.stderr"
run_http_mcp_sequence "$DEFAULT_PORT" "$DEFAULT_OUTPUT" \
  "$(init_request)" \
  "$(tools_list_request)" \
  "$(resources_list_request)" \
  "$(workspace_status_request)" \
  "$(read_element_request)" \
  "$(collect_request)" \
  "$(schema_error_request)" \
  "$(unknown_tool_request)" \
  "$(resource_read_request)" \
  "$(format_fix_rejected_request)" \
  "$(semantic_requirement_request)" \
  "$(ontology_search_request)" \
  "$(ontologies_request)" \
  "$(ontologies_jsonld_request)" \
  "$(ontologies_full_request)" || fail "default MCP HTTP request sequence failed"
stop_http_mcp
trap - EXIT

assert_jq_line "$DEFAULT_OUTPUT" 1 '.result.protocolVersion == "2025-11-25"' "initialize reports supported protocol"
assert_jq_line "$DEFAULT_OUTPUT" 1 '.result.capabilities.tools | type == "object"' "initialize reports standard tools capability"
assert_jq_line "$DEFAULT_OUTPUT" 1 '.result.capabilities.resources | type == "object"' "initialize reports standard resources capability"
assert_jq_line "$DEFAULT_OUTPUT" 1 '.result.capabilities.prompts == null and .result.capabilities.logging == null and .result.capabilities.completions == null' "initialize does not advertise unsupported capabilities"
assert_jq_line "$DEFAULT_OUTPUT" 1 '.result.serverInfo.name == "reqvire"' "initialize reports serverInfo"

assert_jq_line "$DEFAULT_OUTPUT" 2 '[.result.tools[].name] | index("reqvire.search") != null' "tools/list includes read tools"
assert_jq_line "$DEFAULT_OUTPUT" 2 '[.result.tools[].name] | index("reqvire.add_element") == null and index("reqvire.link") == null' "default tools/list omits mutation tools"
assert_jq_line "$DEFAULT_OUTPUT" 2 '[.result.tools[].name] | index("reqvire.mcp") == null and index("reqvire.serve") == null and index("reqvire.export") == null and index("reqvire.validate") == null' "tools/list omits server, export, and validate commands"
assert_jq_line "$DEFAULT_OUTPUT" 2 '[.result.tools[].name] | index("reqvire.command") == null and index("reqvire.shell") == null and index("reqvire.sout") == null' "tools/list omits shell-style tools"
assert_jq_line "$DEFAULT_OUTPUT" 2 'all(.result.tools[]; (.name|type=="string") and (.description|type=="string") and (.inputSchema.type=="object") and (.outputSchema|type=="object") and (.annotations|type=="object"))' "each tool has MCP tool contract fields"
assert_jq_line "$DEFAULT_OUTPUT" 2 'all(.result.tools[]; ((.inputSchema.properties // {}) | has("json") | not) and ((.inputSchema.properties // {}) | has("output") | not))' "tool schemas omit CLI transport options"
assert_jq_line "$DEFAULT_OUTPUT" 2 '.result.tools[] | select(.name=="reqvire.search") | .inputSchema.properties | has("filter_status") and has("filter_priority") and has("filter_risk") and has("filter_owner")' "search tool schema advertises governance metadata filters"
assert_jq_line "$DEFAULT_OUTPUT" 2 '.result.tools[] | select(.name=="reqvire.format") | .annotations.readOnlyHint == true and .inputSchema.properties.fix.enum == [false]' "format is preview-only in default mode"

assert_jq_line "$DEFAULT_OUTPUT" 3 '[.result.resources[].uri] | index("reqvire://workspace/status") != null' "resources/list exposes workspace status"
assert_jq_line "$DEFAULT_OUTPUT" 4 '.result.structuredContent.workspace_root | type == "string"' "workspace_status returns workspace root"
assert_jq_line "$DEFAULT_OUTPUT" 4 '.result.structuredContent.size_estimates_enabled == false' "workspace_status reports size estimates disabled by default"
assert_jq_line "$DEFAULT_OUTPUT" 4 '.result.structuredContent.git.head | type == "string"' "workspace_status returns git HEAD"
assert_jq_line "$DEFAULT_OUTPUT" 4 '.result.structuredContent.model.valid == true and (.result.structuredContent.model.fingerprint | type == "string")' "workspace_status returns model validity and fingerprint"
assert_jq_line "$DEFAULT_OUTPUT" 5 '.result.structuredContent.name == "Test Requirement Beta"' "read_element returns authoritative element"
assert_jq_line "$DEFAULT_OUTPUT" 5 '.result.structuredContent | has("size_estimate") | not' "read_element omits size estimate by default"
assert_jq_line "$DEFAULT_OUTPUT" 6 '.result.structuredContent.starting_element == "specifications/Requirements.md#test-requirement-beta" and (.result.structuredContent.items[] | select(.name=="Test Requirement Beta"))' "collect returns structured content"
assert_jq_line "$DEFAULT_OUTPUT" 7 '.error.code == -32602 and (.error.data.message | contains("element_name"))' "schema-invalid tool arguments return protocol error"
assert_jq_line "$DEFAULT_OUTPUT" 8 '.error.code == -32602' "unknown or unadvertised tool returns protocol error"
assert_jq_line "$DEFAULT_OUTPUT" 9 '.result.contents[0].uri == "reqvire://workspace/status" and .result.contents[0].mimeType == "application/json"' "resources/read returns JSON resource content"
assert_jq_line "$DEFAULT_OUTPUT" 10 '.error.code == -32602' "format fix is rejected by default schema"
assert_jq_line "$DEFAULT_OUTPUT" 11 '.result.structuredContent.concept_references[0].label == "Access Token" and .result.structuredContent.concept_references[0].iri == "mcp:AccessToken"' "read_element returns concept references"
assert_jq_line "$DEFAULT_OUTPUT" 12 '.result.structuredContent.files[]?.elements[] | select(.name=="MCP Access Token Ontology") | .ontology.ontology.content | contains("mcp:AccessToken")' "search returns ontology ADT content"
assert_jq_line "$DEFAULT_OUTPUT" 13 '.result.structuredContent.format == "turtle" and (.result.structuredContent.content | contains("mcp:AccessToken")) and (.result.structuredContent.content | contains("mcp:AccessTokenShape"))' "ontologies tool returns Turtle semantic content"
assert_jq_line "$DEFAULT_OUTPUT" 13 '.result.structuredContent.summary.ontology_blocks >= 1 and .result.structuredContent.summary.shape_blocks >= 1' "ontologies tool returns semantic index summary"
assert_jq_line "$DEFAULT_OUTPUT" 13 '.result.structuredContent.blocks[] | select(.source_name=="MCP Access Token Ontology" and .kind=="ontology")' "ontologies tool returns ontology block metadata"
assert_jq_line "$DEFAULT_OUTPUT" 14 '.result.structuredContent.format == "jsonld" and (.result.structuredContent.jsonld | type == "array") and (.result.structuredContent.jsonld | length) > 0' "ontologies tool returns JSON-LD semantic content"
assert_jq_line "$DEFAULT_OUTPUT" 15 '.result.structuredContent.full == true and (.result.structuredContent.content | contains("reqvire:attaches")) and (.result.structuredContent.content | contains("urn:reqvire:element:mcp-semantic-capability"))' "ontologies tool returns full model context triples"

UNSUPPORTED_PORT="$(pick_port)"
UNSUPPORTED_OUTPUT_PREFIX="$TEST_DIR/output/mcp-unsupported-protocol"
start_http_mcp "$UNSUPPORTED_PORT" "$UNSUPPORTED_OUTPUT_PREFIX"
trap stop_http_mcp EXIT
wait_for_http_mcp "$UNSUPPORTED_PORT" "$TEST_DIR/output/mcp-unsupported-startup-init.json" || fail "unsupported protocol test MCP HTTP server did not start" "${UNSUPPORTED_OUTPUT_PREFIX}.stderr"
UNSUPPORTED_HEADER_STATUS="$(curl -sS -o "$TEST_DIR/output/mcp-unsupported-protocol.txt" -w "%{http_code}" \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json, text/event-stream' \
  -H 'Mcp-Protocol-Version: 1900-01-01' \
  --data "$(tools_list_request)" \
  "http://127.0.0.1:${UNSUPPORTED_PORT}/mcp")"
stop_http_mcp
trap - EXIT
if [ "$UNSUPPORTED_HEADER_STATUS" != "400" ]; then
  fail "unsupported MCP-Protocol-Version header should be rejected" "$TEST_DIR/output/mcp-unsupported-protocol.txt"
fi

SIZE_OUTPUT="$TEST_DIR/output/mcp-size-estimates.jsonl"
SIZE_PORT="$(pick_port)"
SIZE_OUTPUT_PREFIX="$TEST_DIR/output/mcp-size-estimates"
start_http_mcp "$SIZE_PORT" "$SIZE_OUTPUT_PREFIX" --with-size-estimates
trap stop_http_mcp EXIT
wait_for_http_mcp "$SIZE_PORT" "$TEST_DIR/output/mcp-size-estimates-init.json" || fail "size-estimates MCP HTTP server did not start" "${SIZE_OUTPUT_PREFIX}.stderr"
run_http_mcp_sequence "$SIZE_PORT" "$SIZE_OUTPUT" \
  "$(init_request)" \
  "$(workspace_status_request)" \
  "$(read_element_request)" \
  "$(model_request)" \
  "$(resource_read_request)" || fail "size-estimates MCP HTTP request sequence failed"
stop_http_mcp
trap - EXIT

assert_jq_line "$SIZE_OUTPUT" 2 '.result.structuredContent.size_estimates_enabled == true' "workspace_status reports size estimates enabled"
assert_jq_line "$SIZE_OUTPUT" 3 '.result.structuredContent.size_estimate.content_bytes >= 0 and .result.structuredContent.size_estimate.rendered_context_bytes > 0 and .result.structuredContent.size_estimate.estimated_tokens > 0' "read_element includes size estimate when enabled"
assert_jq_line "$SIZE_OUTPUT" 4 '[.result.structuredContent.elements[]? | .. | objects | select(has("identifier") and has("name"))] as $elements | ($elements | length) > 0 and all($elements[]; (.size_estimate.content_bytes | type == "number") and (.size_estimate.rendered_context_bytes | type == "number") and (.size_estimate.estimated_tokens | type == "number"))' "model tool includes size estimates when enabled"
assert_jq_line "$SIZE_OUTPUT" 5 '.result.contents[0].text | fromjson | .size_estimates_enabled == true' "workspace status resource reports size estimates enabled"

DRY_RUN_OUTPUT="$TEST_DIR/output/mcp-mutation-dry-run.jsonl"
ADD_CONTENT="$(< "$TEST_SCRIPT_DIR/fixtures/mcp-added-requirement.md")"
DRY_RUN_PORT="$(pick_port)"
DRY_RUN_OUTPUT_PREFIX="$TEST_DIR/output/mcp-mutation-dry-run"
start_http_mcp "$DRY_RUN_PORT" "$DRY_RUN_OUTPUT_PREFIX" --enable-mutations
trap stop_http_mcp EXIT
wait_for_http_mcp "$DRY_RUN_PORT" "$TEST_DIR/output/mcp-mutation-dry-run-init.json" || fail "dry-run MCP HTTP server did not start" "${DRY_RUN_OUTPUT_PREFIX}.stderr"
run_http_mcp_sequence "$DRY_RUN_PORT" "$DRY_RUN_OUTPUT" \
  "$(init_request)" \
  "$(tools_list_request)" \
  "$(jq -n -c --arg content "$ADD_CONTENT" '{jsonrpc:"2.0",id:3,method:"tools/call",params:{name:"reqvire.add_element",arguments:{file:"specifications/Requirements.md",content:$content,dry_run:true}}}')" || fail "dry-run MCP HTTP request sequence failed"
stop_http_mcp
trap - EXIT

assert_jq_line "$DRY_RUN_OUTPUT" 2 '[.result.tools[].name] | index("reqvire.add_element") != null and index("reqvire.link") != null' "mutation mode advertises mutation tools"
assert_jq_line "$DRY_RUN_OUTPUT" 2 '.result.tools[] | select(.name=="reqvire.add_element") | .annotations.readOnlyHint == false' "mutation tools are non-read-only"
assert_jq_line "$DRY_RUN_OUTPUT" 2 '.result.tools[] | select(.name=="reqvire.format") | .inputSchema.properties.fix.type == "boolean"' "mutation mode exposes format fix argument"
assert_jq_line "$DRY_RUN_OUTPUT" 3 '.result.structuredContent.dry_run == true and (.result.structuredContent.diffs | length) >= 1' "dry-run mutation returns diffs without execution"
if grep -q "MCP Added Requirement" "$TEST_DIR/specifications/Requirements.md"; then
  fail "dry-run mutation modified the fixture file"
fi

MUTATION_OUTPUT="$TEST_DIR/output/mcp-mutation-execute.jsonl"
MUTATION_PORT="$(pick_port)"
MUTATION_OUTPUT_PREFIX="$TEST_DIR/output/mcp-mutation-execute"
start_http_mcp "$MUTATION_PORT" "$MUTATION_OUTPUT_PREFIX" --enable-mutations
trap stop_http_mcp EXIT
wait_for_http_mcp "$MUTATION_PORT" "$TEST_DIR/output/mcp-mutation-execute-init.json" || fail "mutation MCP HTTP server did not start" "${MUTATION_OUTPUT_PREFIX}.stderr"
run_http_mcp_sequence "$MUTATION_PORT" "$MUTATION_OUTPUT" \
  "$(init_request)" \
  "$(jq -n -c --arg content "$ADD_CONTENT" '{jsonrpc:"2.0",id:4,method:"tools/call",params:{name:"reqvire.add_element",arguments:{file:"specifications/Requirements.md",content:$content,dry_run:false}}}')" \
  "$(jq -n -c '{jsonrpc:"2.0",id:5,method:"tools/call",params:{name:"reqvire.read_element",arguments:{name:"MCP Added Requirement"}}}')" || fail "mutation MCP HTTP request sequence failed"
stop_http_mcp
trap - EXIT

assert_jq_line "$MUTATION_OUTPUT" 2 '.result.structuredContent.dry_run == false and (.result.structuredContent.diffs | length) >= 1' "executing mutation returns persisted diffs"
grep -q "MCP Added Requirement" "$TEST_DIR/specifications/Requirements.md" || fail "executing mutation did not update the fixture file"
assert_jq_line "$MUTATION_OUTPUT" 3 '.result.structuredContent.name == "MCP Added Requirement"' "post-mutation read observes refreshed model state"

HTTP_PORT="$(pick_port)"
HTTP_OUTPUT_PREFIX="$TEST_DIR/output/mcp-http"
start_http_mcp "$HTTP_PORT" "$HTTP_OUTPUT_PREFIX"
trap stop_http_mcp EXIT
wait_for_http_mcp "$HTTP_PORT" "$TEST_DIR/output/mcp-http-init.json" || fail "HTTP MCP server did not start" "${HTTP_OUTPUT_PREFIX}.stderr"

http_mcp_call "$HTTP_PORT" "$(tools_list_request)" "$TEST_DIR/output/mcp-http-tools.json" || fail "HTTP tools/list request failed"
jq -r '.result.tools[].name' "$TEST_DIR/output/mcp-http-tools.json" > "$TEST_DIR/output/mcp-http-tools.txt"
if ! diff -u "$TEST_SCRIPT_DIR/expected/http-read-tools.txt" "$TEST_DIR/output/mcp-http-tools.txt"; then
  fail "HTTP default tools/list does not match expected read-only tool set"
fi
jq -e '[.result.tools[].name] | index("reqvire.search") != null and index("reqvire.add_element") == null' "$TEST_DIR/output/mcp-http-tools.json" >/dev/null \
  || fail "HTTP default tools/list should expose read-only tools and omit mutation tools" "$TEST_DIR/output/mcp-http-tools.json"
jq -e 'all(.result.tools[]; (.inputSchema.type=="object") and (.outputSchema|type=="object") and (.annotations|type=="object"))' "$TEST_DIR/output/mcp-http-tools.json" >/dev/null \
  || fail "HTTP tools/list should expose schema and annotation fields" "$TEST_DIR/output/mcp-http-tools.json"

http_mcp_call "$HTTP_PORT" "$(resources_list_request)" "$TEST_DIR/output/mcp-http-resources.json" || fail "HTTP resources/list request failed"
jq -e '[.result.resources[].uri] | index("reqvire://workspace/status") != null' "$TEST_DIR/output/mcp-http-resources.json" >/dev/null \
  || fail "HTTP resources/list should expose workspace status" "$TEST_DIR/output/mcp-http-resources.json"

http_mcp_call "$HTTP_PORT" "$(workspace_status_request)" "$TEST_DIR/output/mcp-http-status.json" || fail "HTTP workspace_status request failed"
jq -e '.result.structuredContent.model.valid == true and (.result.structuredContent.model.fingerprint | type == "string")' "$TEST_DIR/output/mcp-http-status.json" >/dev/null \
  || fail "HTTP workspace_status should return structured model state" "$TEST_DIR/output/mcp-http-status.json"

http_mcp_call "$HTTP_PORT" "$(workspace_status_request)" "$TEST_DIR/output/mcp-http-loopback-origin.json" -H "Origin: http://localhost:9999" \
  || fail "HTTP loopback Origin should be accepted"
jq -e '.result.structuredContent.workspace_root | type == "string"' "$TEST_DIR/output/mcp-http-loopback-origin.json" >/dev/null \
  || fail "HTTP loopback Origin response should execute request" "$TEST_DIR/output/mcp-http-loopback-origin.json"

INVALID_ORIGIN_STATUS="$(curl -sS -o "$TEST_DIR/output/mcp-http-invalid-origin.txt" -w "%{http_code}" \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json, text/event-stream' \
  -H "Mcp-Protocol-Version: ${MCP_PROTOCOL_VERSION}" \
  -H 'Origin: https://evil.example' \
  --data "$(workspace_status_request)" \
  "http://127.0.0.1:${HTTP_PORT}/mcp")"
if [ "$INVALID_ORIGIN_STATUS" != "403" ]; then
  fail "HTTP non-loopback Origin should be rejected before tool execution" "$TEST_DIR/output/mcp-http-invalid-origin.txt"
fi

NULL_ORIGIN_STATUS="$(curl -sS -o "$TEST_DIR/output/mcp-http-null-origin.txt" -w "%{http_code}" \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json, text/event-stream' \
  -H "Mcp-Protocol-Version: ${MCP_PROTOCOL_VERSION}" \
  -H 'Origin: null' \
  --data "$(workspace_status_request)" \
  "http://127.0.0.1:${HTTP_PORT}/mcp")"
if [ "$NULL_ORIGIN_STATUS" != "403" ]; then
  fail "HTTP null Origin should be rejected before tool execution" "$TEST_DIR/output/mcp-http-null-origin.txt"
fi

GET_STATUS="$(curl -sS -o "$TEST_DIR/output/mcp-http-get.txt" -w "%{http_code}" "http://127.0.0.1:${HTTP_PORT}/mcp")"
if [ "$GET_STATUS" != "405" ]; then
  fail "HTTP GET without SSE streaming should return method-not-allowed" "$TEST_DIR/output/mcp-http-get.txt"
fi

stop_http_mcp

HTTP_MUTATION_PORT="$(pick_port)"
HTTP_MUTATION_OUTPUT_PREFIX="$TEST_DIR/output/mcp-http-mutations"
start_http_mcp "$HTTP_MUTATION_PORT" "$HTTP_MUTATION_OUTPUT_PREFIX" --enable-mutations
wait_for_http_mcp "$HTTP_MUTATION_PORT" "$TEST_DIR/output/mcp-http-mutations-init.json" || fail "HTTP mutation MCP server did not start" "${HTTP_MUTATION_OUTPUT_PREFIX}.stderr"

http_mcp_call "$HTTP_MUTATION_PORT" "$(tools_list_request)" "$TEST_DIR/output/mcp-http-mutation-tools.json" || fail "HTTP mutation tools/list request failed"
jq -r '.result.tools[].name' "$TEST_DIR/output/mcp-http-mutation-tools.json" > "$TEST_DIR/output/mcp-http-mutation-tools.txt"
if ! diff -u "$TEST_SCRIPT_DIR/expected/http-mutation-tools.txt" "$TEST_DIR/output/mcp-http-mutation-tools.txt"; then
  fail "HTTP mutation tools/list does not match expected mutation tool set"
fi
jq -e '[.result.tools[].name] | index("reqvire.add_element") != null and index("reqvire.link") != null' "$TEST_DIR/output/mcp-http-mutation-tools.json" >/dev/null \
  || fail "HTTP mutation mode should advertise mutation tools" "$TEST_DIR/output/mcp-http-mutation-tools.json"

HTTP_CONCURRENT_CONTENT_A="$(< "$TEST_SCRIPT_DIR/fixtures/http-concurrent-requirement-a.md")"
HTTP_CONCURRENT_CONTENT_B="$(< "$TEST_SCRIPT_DIR/fixtures/http-concurrent-requirement-b.md")"

HTTP_CONCURRENT_REQUEST_A="$(jq -n -c --arg content "$HTTP_CONCURRENT_CONTENT_A" '{jsonrpc:"2.0",id:11,method:"tools/call",params:{name:"reqvire.add_element",arguments:{file:"specifications/Requirements.md",content:$content,dry_run:false}}}')"
HTTP_CONCURRENT_REQUEST_B="$(jq -n -c --arg content "$HTTP_CONCURRENT_CONTENT_B" '{jsonrpc:"2.0",id:12,method:"tools/call",params:{name:"reqvire.add_element",arguments:{file:"specifications/Requirements.md",content:$content,dry_run:false}}}')"

http_mcp_call "$HTTP_MUTATION_PORT" "$HTTP_CONCURRENT_REQUEST_A" "$TEST_DIR/output/mcp-http-concurrent-a.json" &
HTTP_CURL_PID_A=$!
http_mcp_call "$HTTP_MUTATION_PORT" "$HTTP_CONCURRENT_REQUEST_B" "$TEST_DIR/output/mcp-http-concurrent-b.json" &
HTTP_CURL_PID_B=$!
wait "$HTTP_CURL_PID_A" || fail "first concurrent HTTP mutation failed" "$TEST_DIR/output/mcp-http-concurrent-a.json"
wait "$HTTP_CURL_PID_B" || fail "second concurrent HTTP mutation failed" "$TEST_DIR/output/mcp-http-concurrent-b.json"

jq -e '.result.structuredContent.dry_run == false' "$TEST_DIR/output/mcp-http-concurrent-a.json" >/dev/null \
  || fail "first concurrent HTTP mutation should execute" "$TEST_DIR/output/mcp-http-concurrent-a.json"
jq -e '.result.structuredContent.dry_run == false' "$TEST_DIR/output/mcp-http-concurrent-b.json" >/dev/null \
  || fail "second concurrent HTTP mutation should execute" "$TEST_DIR/output/mcp-http-concurrent-b.json"
grep '^### MCP HTTP Concurrent Requirement' "$TEST_DIR/specifications/Requirements.md" > "$TEST_DIR/output/mcp-http-concurrent-requirement-headings.txt"
if ! diff -u "$TEST_SCRIPT_DIR/expected/http-concurrent-requirement-headings.txt" "$TEST_DIR/output/mcp-http-concurrent-requirement-headings.txt"; then
  fail "serialized HTTP mutations should preserve both expected filesystem writes"
fi

http_mcp_call "$HTTP_MUTATION_PORT" "$(jq -n -c '{jsonrpc:"2.0",id:13,method:"tools/call",params:{name:"reqvire.read_element",arguments:{name:"MCP HTTP Concurrent Requirement A"}}}')" "$TEST_DIR/output/mcp-http-post-mutation-read.json" \
  || fail "HTTP post-mutation read request failed"
jq -e '.result.structuredContent.name == "MCP HTTP Concurrent Requirement A"' "$TEST_DIR/output/mcp-http-post-mutation-read.json" >/dev/null \
  || fail "HTTP post-mutation read should observe refreshed model state" "$TEST_DIR/output/mcp-http-post-mutation-read.json"

stop_http_mcp
trap - EXIT

INVALID_DIR="$(mktemp -d -t reqvire-mcp-invalid-XXXXXX)"
cp -a "$TEST_SCRIPT_DIR/fixtures/invalid-startup/." "$INVALID_DIR/"

set +e
INVALID_PORT="$(pick_port)"
(cd "$INVALID_DIR" && "$REQVIRE_BIN" mcp --host 127.0.0.1 --port "$INVALID_PORT") > "$TEST_DIR/output/mcp-invalid-startup.stdout" 2> "$TEST_DIR/output/mcp-invalid-startup.stderr" &
INVALID_PID=$!
for _ in $(seq 1 50); do
  if ! kill -0 "$INVALID_PID" >/dev/null 2>&1; then
    break
  fi
  sleep 0.1
done
if kill -0 "$INVALID_PID" >/dev/null 2>&1; then
  kill "$INVALID_PID" >/dev/null 2>&1 || true
fi
wait "$INVALID_PID"
INVALID_EXIT=$?
set +e

if [ "$INVALID_EXIT" -eq 0 ]; then
  fail "invalid model should prevent MCP startup"
fi
if ! grep -q "MCP startup validation failed" "$TEST_DIR/output/mcp-invalid-startup.stderr"; then
  fail "startup validation diagnostics should mention MCP startup validation" "$TEST_DIR/output/mcp-invalid-startup.stderr"
fi

exit 0
