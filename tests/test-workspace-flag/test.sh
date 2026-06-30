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

init_multi_workspace_repos() {
  local workspace="$1"

  for repo in "$workspace/repo-a" "$workspace/repo-b"; do
    (cd "$repo" && git init >/dev/null 2>&1)
    (cd "$repo" && git config --local user.email "test@example.com" >/dev/null 2>&1)
    (cd "$repo" && git config --local user.name "Test User" >/dev/null 2>&1)
    (cd "$repo" && git add . >/dev/null 2>&1)
    (cd "$repo" && git commit -m "Initial commit" >/dev/null 2>&1)
  done
}

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTSIDE_DIR="$(mktemp -d -t reqvire-workspace-outside-XXXXXX)"

cp -a "$TEST_SCRIPT_DIR/fixtures/single-workspace/." "$TEST_DIR/"
mkdir -p "$TEST_DIR/output"

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
if [ "$NO_WORKSPACE_EXIT" -eq 0 ]; then
  fail "omitting workspace outside an eligible Git worktree should fail before model processing" "$TEST_DIR/output/no-workspace.txt"
fi
if ! grep -q "eligible Git worktree" "$TEST_DIR/output/no-workspace.txt"; then
  fail "no-workspace error should explain missing eligible Git worktree" "$TEST_DIR/output/no-workspace.txt"
fi

MULTI_WORKSPACE="$(mktemp -d -t reqvire-multi-workspace-XXXXXX)"
cp -a "$TEST_SCRIPT_DIR/fixtures/multi-workspace/." "$MULTI_WORKSPACE/"

init_multi_workspace_repos "$MULTI_WORKSPACE"

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$MULTI_WORKSPACE" validate) > "$TEST_DIR/output/multi-workspace-validate.txt" 2>&1 \
  || fail "validate should accept a non-Git workspace root with descendant Git worktrees" "$TEST_DIR/output/multi-workspace-validate.txt"

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$MULTI_WORKSPACE" search --json) > "$TEST_DIR/output/multi-workspace-search.json" 2>&1 \
  || fail "search should run over descendant Git worktrees only" "$TEST_DIR/output/multi-workspace-search.json"

jq -e '.files[].elements[] | select(.identifier == "repo-a/specifications/A.md#repo-a-requirement")' "$TEST_DIR/output/multi-workspace-search.json" >/dev/null \
  || fail "repo A identifier should be workspace-root-relative" "$TEST_DIR/output/multi-workspace-search.json"
jq -e '.files[].elements[] | select(.identifier == "repo-b/specifications/B.md#repo-b-requirement")' "$TEST_DIR/output/multi-workspace-search.json" >/dev/null \
  || fail "repo B identifier should be workspace-root-relative" "$TEST_DIR/output/multi-workspace-search.json"
if jq -e '.files[].elements[] | select(.name | test("Ignored|Non Git"))' "$TEST_DIR/output/multi-workspace-search.json" >/dev/null; then
  fail "non-Git and ignored workspace files should not be parsed" "$TEST_DIR/output/multi-workspace-search.json"
fi
jq -e '.files[].elements[] | select(.identifier == "repo-b/specifications/B.md#repo-b-requirement") | .relations[]? | select(.relation_type == "satisfiedBy" and .target.target == "repo-a/docs/evidence.txt")' "$TEST_DIR/output/multi-workspace-search.json" >/dev/null \
  || fail "root-relative links should resolve against the effective workspace root" "$TEST_DIR/output/multi-workspace-search.json"

cp "$MULTI_WORKSPACE/repo-b/specifications/InvalidNonGitEvidence.md.fixture" "$MULTI_WORKSPACE/repo-b/specifications/InvalidNonGitEvidence.md"
set +e
(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$MULTI_WORKSPACE" validate) > "$TEST_DIR/output/multi-workspace-non-git-evidence.txt" 2>&1
NON_GIT_EVIDENCE_EXIT=$?
set -e
rm -f "$MULTI_WORKSPACE/repo-b/specifications/InvalidNonGitEvidence.md"
if [ "$NON_GIT_EVIDENCE_EXIT" -eq 0 ]; then
  fail "non-Git evidence file should not satisfy an InternalPath relation" "$TEST_DIR/output/multi-workspace-non-git-evidence.txt"
fi
if ! grep -q "outside eligible Git worktrees" "$TEST_DIR/output/multi-workspace-non-git-evidence.txt"; then
  fail "non-Git evidence validation error should mention eligible Git worktrees" "$TEST_DIR/output/multi-workspace-non-git-evidence.txt"
fi

set +e
(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$MULTI_WORKSPACE" change-impact --git-commit HEAD) > "$TEST_DIR/output/multi-workspace-change-impact-commit.txt" 2>&1
CHANGE_IMPACT_MULTI_EXIT=$?
set -e
if [ "$CHANGE_IMPACT_MULTI_EXIT" -eq 0 ]; then
  fail "change-impact --git-commit should reject ambiguous multi-worktree workspaces" "$TEST_DIR/output/multi-workspace-change-impact-commit.txt"
fi
if ! grep -q "requires exactly one eligible Git worktree" "$TEST_DIR/output/multi-workspace-change-impact-commit.txt"; then
  fail "multi-worktree change-impact error should explain single-worktree commit adapter limit" "$TEST_DIR/output/multi-workspace-change-impact-commit.txt"
fi

exit 0
