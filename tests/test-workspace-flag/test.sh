#!/usr/bin/env bash
set -uo pipefail

fail() {
  echo "❌ FAILED: $1"
  if [ -n "${2:-}" ] && [ -f "$2" ]; then
    echo ""
    cat "$2"
  fi
  exit 1
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

pick_port() {
  python3 - <<'PY'
import socket
s = socket.socket()
s.bind(("127.0.0.1", 0))
print(s.getsockname()[1])
s.close()
PY
}

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTSIDE_DIR="$(mktemp -d -t reqvire-workspace-outside-XXXXXX)"

mkdir -p "$TEST_DIR/specifications" "$TEST_DIR/output"
cat > "$TEST_DIR/.reqvireignore" <<'EOF'
ignored/**
output/**
EOF

cat > "$TEST_DIR/specifications/Requirements.md" <<'EOF'
# Elements

### Workspace Flag Root

Root requirement for explicit workspace selection.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [Workspace Flag Child](#workspace-flag-child)
  * derive: [Workspace Flag Sibling](#workspace-flag-sibling)

---

### Workspace Flag Child

Requirement used to verify explicit workspace selection.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Workspace Flag Root](#workspace-flag-root)

---

### Workspace Flag Sibling

Requirement used to verify workspace-selected file moves.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Workspace Flag Root](#workspace-flag-root)

---
EOF

mkdir -p "$TEST_DIR/ignored"
cat > "$TEST_DIR/ignored/Invalid.md" <<'EOF'
# Elements

### Ignored Broken Element

This file is ignored by workspace .reqvireignore and must not affect validation.
EOF

set +e
(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$TEST_DIR" validate) > "$TEST_DIR/output/validate.txt" 2>&1
VALIDATE_EXIT=$?
set -e
if [ "$VALIDATE_EXIT" -ne 0 ]; then
  fail "validate should run against explicit workspace from outside cwd" "$TEST_DIR/output/validate.txt"
fi

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$TEST_DIR" search --short --filter-name "Workspace Flag Child") > "$TEST_DIR/output/search-short.txt" 2>&1 \
  || fail "search should run against explicit workspace from outside cwd" "$TEST_DIR/output/search-short.txt"
sed -i '${/^$/d;}' "$TEST_DIR/output/search-short.txt"
if ! diff -u "$TEST_SCRIPT_DIR/expected/search-short.txt" "$TEST_DIR/output/search-short.txt"; then
  fail "workspace search output does not match expected result"
fi

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$TEST_DIR" mv "Workspace Flag Child" specifications/Moved.md) > "$TEST_DIR/output/mv.txt" 2>&1 \
  || fail "mv should mutate selected workspace from outside cwd" "$TEST_DIR/output/mv.txt"
if ! grep -q "Workspace Flag Child" "$TEST_DIR/specifications/Moved.md"; then
  fail "mv should create target file inside selected workspace" "$TEST_DIR/specifications/Moved.md"
fi
if grep -q "### Workspace Flag Child" "$TEST_DIR/specifications/Requirements.md"; then
  fail "mv should remove moved element from original selected workspace file" "$TEST_DIR/specifications/Requirements.md"
fi

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$TEST_DIR" mv-file specifications/Requirements.md specifications/RenamedRequirements.md) > "$TEST_DIR/output/mv-file.txt" 2>&1 \
  || fail "mv-file should mutate selected workspace from outside cwd" "$TEST_DIR/output/mv-file.txt"
if [ -f "$TEST_DIR/specifications/Requirements.md" ]; then
  fail "mv-file should remove original file inside selected workspace" "$TEST_DIR/specifications/Requirements.md"
fi
if ! grep -q "Workspace Flag Root" "$TEST_DIR/specifications/RenamedRequirements.md"; then
  fail "mv-file should create renamed file inside selected workspace" "$TEST_DIR/specifications/RenamedRequirements.md"
fi

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$TEST_DIR" search --short --filter-name "Workspace Flag") > "$TEST_DIR/output/moved-search.txt" 2>&1 \
  || fail "search after workspace mutations should run" "$TEST_DIR/output/moved-search.txt"
grep -o 'specifications/[^ ]*#workspace-flag-[a-z]*' "$TEST_DIR/output/moved-search.txt" | sort > "$TEST_DIR/output/moved-file-element-ids.txt"
if ! diff -u "$TEST_SCRIPT_DIR/expected/moved-file-element-ids.txt" "$TEST_DIR/output/moved-file-element-ids.txt"; then
  fail "workspace mutation paths do not match expected selected-workspace files"
fi

(cd "$TEST_DIR" && git add . && git commit -m "Workspace mutations" >/dev/null 2>&1)
(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$TEST_DIR" change-impact --git-commit HEAD~1) > "$TEST_DIR/output/change-impact.txt" 2>&1 \
  || fail "change-impact should run against explicit workspace from outside cwd" "$TEST_DIR/output/change-impact.txt"
if ! grep -q "Workspace Flag Child" "$TEST_DIR/output/change-impact.txt"; then
  fail "change-impact should report selected workspace mutation impact" "$TEST_DIR/output/change-impact.txt"
fi

MCP_OUTPUT="$TEST_DIR/output/mcp-workspace.jsonl"
MCP_PORT="$(pick_port)"
(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$TEST_DIR" mcp --host 127.0.0.1 --port "$MCP_PORT") > "$MCP_OUTPUT.stdout" 2>"$MCP_OUTPUT.stderr" &
MCP_PID=$!
trap 'kill "$MCP_PID" >/dev/null 2>&1 || true; wait "$MCP_PID" >/dev/null 2>&1 || true' EXIT

INIT_REQUEST="$(jq -n -c '{jsonrpc:"2.0",id:1,method:"initialize",params:{protocolVersion:"2025-11-25",capabilities:{},clientInfo:{name:"reqvire-test",version:"0"}}}')"
STATUS_REQUEST="$(jq -n -c '{jsonrpc:"2.0",id:2,method:"tools/call",params:{name:"reqvire.workspace_status",arguments:{}}}')"

: > "$MCP_OUTPUT"
for _ in $(seq 1 50); do
  if curl -sS -o "$TEST_DIR/output/mcp-workspace-init.json" \
    -H 'Content-Type: application/json' \
    -H 'Accept: application/json, text/event-stream' \
    --data "$INIT_REQUEST" \
    "http://127.0.0.1:${MCP_PORT}/mcp" >/dev/null 2>&1; then
    if jq -e '.result.protocolVersion == "2025-11-25"' "$TEST_DIR/output/mcp-workspace-init.json" >/dev/null 2>&1; then
      break
    fi
  fi
  sleep 0.1
done

if ! jq -e '.result.protocolVersion == "2025-11-25"' "$TEST_DIR/output/mcp-workspace-init.json" >/dev/null 2>&1; then
  fail "MCP should initialize from explicit workspace" "$MCP_OUTPUT.stderr"
fi

cat "$TEST_DIR/output/mcp-workspace-init.json" >> "$MCP_OUTPUT"
printf '\n' >> "$MCP_OUTPUT"
curl -sS -o "$TEST_DIR/output/mcp-workspace-status.json" \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json, text/event-stream' \
  -H 'Mcp-Protocol-Version: 2025-11-25' \
  --data "$STATUS_REQUEST" \
  "http://127.0.0.1:${MCP_PORT}/mcp" \
  || fail "MCP workspace_status request should run" "$MCP_OUTPUT.stderr"
cat "$TEST_DIR/output/mcp-workspace-status.json" >> "$MCP_OUTPUT"
printf '\n' >> "$MCP_OUTPUT"

kill "$MCP_PID" >/dev/null 2>&1 || true
wait "$MCP_PID" >/dev/null 2>&1 || true
trap - EXIT

assert_jq_line "$MCP_OUTPUT" 1 '.result.protocolVersion == "2025-11-25"' "MCP should initialize from explicit workspace"
if ! json_line "$MCP_OUTPUT" 2 | jq -e --arg workspace "$TEST_DIR" '.result.structuredContent.workspace_root == $workspace' >/dev/null 2>&1; then
  fail "MCP workspace_status should report explicit workspace" "$MCP_OUTPUT"
fi
assert_jq_line "$MCP_OUTPUT" 2 '.result.structuredContent.model.valid == true' "MCP workspace_status should validate selected workspace"

set +e
(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$TEST_DIR/missing" validate) > "$TEST_DIR/output/invalid-workspace.txt" 2>&1
INVALID_WORKSPACE_EXIT=$?
set -e
if [ "$INVALID_WORKSPACE_EXIT" -eq 0 ]; then
  fail "invalid workspace should fail before command execution" "$TEST_DIR/output/invalid-workspace.txt"
fi
if ! grep -q "Failed to resolve workspace" "$TEST_DIR/output/invalid-workspace.txt"; then
  fail "invalid workspace error should be explicit" "$TEST_DIR/output/invalid-workspace.txt"
fi

set +e
(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" search --short --filter-name "Workspace Flag Child") > "$TEST_DIR/output/no-workspace.txt" 2>&1
NO_WORKSPACE_EXIT=$?
set -e
if [ "$NO_WORKSPACE_EXIT" -ne 0 ]; then
  fail "omitting workspace outside a model should preserve cwd behavior without crashing" "$TEST_DIR/output/no-workspace.txt"
fi
if grep -q "Workspace Flag Child" "$TEST_DIR/output/no-workspace.txt"; then
  fail "omitting workspace should not read the explicit workspace model from outside cwd" "$TEST_DIR/output/no-workspace.txt"
fi

exit 0
