#!/bin/bash
# Shared Bash/Git/curl/jq fixture and transport helpers. No remote services.
set -uo pipefail

MCP_PID=''
MCP_URL=''
BACKEND=''
ROOT=''
OWNER=''
CASE_DIR=''
COMPONENT=''
PIN_A=''
PIN_B=''
PIN_C=''
DOCS_PIN=''

die() {
    echo "FAIL: $*" >&2
    exit 1
}

git_do() {
    git "$@" || die "git $*"
}

commit_all() {
    git_do -C "$1" add -A
    git_do -C "$1" -c commit.gpgSign=false commit -qm "$2"
}

head_of() {
    git -C "$1" rev-parse HEAD || die "HEAD unavailable: $1"
}

assert_file() {
    diff -u "$1" "$2" || die "file differs: $2"
}

assert_equal() {
    [[ "$1" == "$2" ]] || die "$3: expected '$1', got '$2'"
}

assert_json() {
    local file="$1" filter="$2"
    shift 2
    jq -e "$@" "$filter" "$file" >/dev/null ||
        die "JSON assertion failed for $file: $filter"
}

fixture() {
    local label="$1" prepared="$2" layout="$3"
    CASE_DIR="$TEST_DIR/output/$BACKEND-$label"
    mkdir -p "$CASE_DIR/owner" "$CASE_DIR/remotes" "$CASE_DIR/seed-website" "$CASE_DIR/seed-docs" ||
        die 'Cannot create fixture directories'
    OWNER="$CASE_DIR/owner"
    ROOT="$OWNER"
    if [[ "$layout" == nested ]]; then
        ROOT="$OWNER/engineering"
        mkdir -p "$ROOT" || die 'Cannot create relative workspace'
    fi
    ROOT=$(cd "$ROOT" && pwd -P) || die 'Cannot normalize root'
    OWNER=$(cd "$OWNER" && pwd -P) || die 'Cannot normalize owner'
    COMPONENT="$ROOT/vendor-product/website"
    export GIT_AUTHOR_NAME='Workspace E2E' GIT_AUTHOR_EMAIL='workspace-e2e@example.invalid'
    export GIT_COMMITTER_NAME="$GIT_AUTHOR_NAME" GIT_COMMITTER_EMAIL="$GIT_AUTHOR_EMAIL"
    export GIT_TERMINAL_PROMPT=0 GIT_ALLOW_PROTOCOL=file GIT_OPTIONAL_LOCKS=0

    git_do -C "$CASE_DIR/seed-website" init -qb main
    cp "$TEST_SCRIPT_DIR/fixtures/website.md.txt" "$CASE_DIR/seed-website/Model.md" ||
        die 'Cannot prepare website model'
    printf 'version A\n' > "$CASE_DIR/seed-website/code.txt"
    commit_all "$CASE_DIR/seed-website" A
    PIN_A=$(head_of "$CASE_DIR/seed-website")
    printf 'version B\n' > "$CASE_DIR/seed-website/code.txt"
    commit_all "$CASE_DIR/seed-website" B
    PIN_B=$(head_of "$CASE_DIR/seed-website")
    git_do clone -q --bare "$CASE_DIR/seed-website" "$CASE_DIR/remotes/website.git"

    git_do -C "$CASE_DIR/seed-docs" init -qb main
    cp "$TEST_SCRIPT_DIR/fixtures/docs.md.txt" "$CASE_DIR/seed-docs/Model.md" ||
        die 'Cannot prepare documentation model'
    commit_all "$CASE_DIR/seed-docs" docs
    DOCS_PIN=$(head_of "$CASE_DIR/seed-docs")
    git_do clone -q --bare "$CASE_DIR/seed-docs" "$CASE_DIR/remotes/docs.git"

    git_do -C "$OWNER" init -qb main
    mkdir -p "$ROOT/repositories" "$ROOT/client" || die 'Cannot prepare root code'
    printf 'ordinary application content\n' > "$ROOT/repositories/keep.txt"
    if [[ "$BACKEND" == mcp ]]; then
        cp "$TEST_SCRIPT_DIR/fixtures/root.md.txt" "$ROOT/Model.md" ||
            die 'Cannot prepare admitted root model'
    fi
    if [[ "$layout" == nested ]]; then
        printf '/engineering/vendor-product/website/\n/engineering/vendor-product/nested/docs/\n/.repo/\n' > "$OWNER/.gitignore"
    else
        printf '/vendor-product/website/\n/vendor-product/nested/docs/\n/.repo/\n' > "$OWNER/.gitignore"
    fi
    cat > "$ROOT/reqvire-workspace.toml" <<EOF
# keep this comment
version = 1
repositories_dir = "vendor-product"

[[repositories]]
path = "website"
url = "file://$CASE_DIR/remotes/website.git"
ref = "refs/heads/main"
commit = "$PIN_A"
EOF
    commit_all "$OWNER" root
    if [[ "$prepared" == yes ]]; then
        mkdir -p "$ROOT/vendor-product" || die 'Cannot create checkout parent'
        git_do clone -q "$CASE_DIR/remotes/website.git" "$COMPONENT"
        git_do -C "$COMPONENT" checkout -q -B main "$PIN_A"
    fi
}

start_mcp() {
    local policy="$1" attempt
    MCP_PID=''
    MCP_URL=''
    if [[ "$policy" == mutations ]]; then
        "$REQVIRE_BIN" --workspace "$ROOT" mcp --host 127.0.0.1 --port 0 --enable-mutations \
            > "$CASE_DIR/server.stdout" 2> "$CASE_DIR/server.stderr" &
    else
        "$REQVIRE_BIN" --workspace "$ROOT" mcp --host 127.0.0.1 --port 0 \
            > "$CASE_DIR/server.stdout" 2> "$CASE_DIR/server.stderr" &
    fi
    MCP_PID=$!
    for attempt in $(seq 1 100); do
        if ! kill -0 "$MCP_PID" 2>/dev/null; then
            cat "$CASE_DIR/server.stderr" >&2
            die 'MCP server exited during fixture startup'
        fi
        MCP_URL=$(awk '/MCP HTTP server listening at/ {print $NF; exit}' "$CASE_DIR/server.stderr")
        if [[ -n "$MCP_URL" ]]; then
            mcp_rpc initialize \
                '{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"workspace-e2e","version":"1"}}' \
                "$CASE_DIR/initialize.json" || die 'MCP initialize failed'
            assert_json "$CASE_DIR/initialize.json" '.result.protocolVersion == "2025-11-25"'
            mcp_rpc tools/list '{}' "$CASE_DIR/tools.json" || die 'MCP discovery failed'
            return
        fi
        sleep 0.1
    done
    cat "$CASE_DIR/server.stderr" >&2
    die 'MCP startup timed out'
}

stop_mcp() {
    if [[ -n "$MCP_PID" ]]; then
        kill "$MCP_PID" 2>/dev/null || true
        wait "$MCP_PID" 2>/dev/null || true
        MCP_PID=''
    fi
}

mcp_rpc() {
    local method="$1" params="$2" output="$3"
    jq -n --arg method "$method" --argjson params "$params" \
        '{jsonrpc:"2.0",id:1,method:$method,params:$params}' > "$output.request" || return 1
    curl -sS --max-time 20 \
        -H 'Content-Type: application/json' \
        -H 'Accept: application/json, text/event-stream' \
        -H 'Mcp-Protocol-Version: 2025-11-25' \
        --data-binary "@$output.request" "$MCP_URL" > "$output" 2> "$output.stderr"
}

require_operation() {
    local operation="$1" name
    if [[ "$BACKEND" == cli ]]; then
        "$REQVIRE_BIN" --workspace "$ROOT" workspace "$operation" --help \
            > "$CASE_DIR/help-$operation.txt" 2>&1 ||
            die "workspace $operation is unavailable (see $CASE_DIR/help-$operation.txt)"
    else
        name="reqvire.workspace.$operation"
        [[ "$operation" != status ]] || name=reqvire.workspace_status
        assert_json "$CASE_DIR/tools.json" 'any(.result.tools[]; .name == $name)' --arg name "$name"
    fi
}

workspace_call() {
    local operation="$1" arguments="$2" output="$3" name params
    if [[ "$BACKEND" == cli ]]; then
        set -- "$REQVIRE_BIN" --workspace "$ROOT" workspace "$operation"
        case "$operation" in
            add)
                set -- "$@" "$(jq -r '.url' <<< "$arguments")" \
                    --path "$(jq -r '.path' <<< "$arguments")" \
                    --ref "$(jq -r '.ref' <<< "$arguments")"
                ;;
            remove) set -- "$@" "$(jq -r '.path' <<< "$arguments")" ;;
            sync)
                if jq -e '.latest == true' <<< "$arguments" >/dev/null; then
                    set -- "$@" --latest
                fi
                ;;
            migrate)
                set -- "$@" --from "$(jq -r '.from' <<< "$arguments")"
                if jq -e '.source != null' <<< "$arguments" >/dev/null; then
                    set -- "$@" --source "$(jq -r '.source' <<< "$arguments")"
                fi
                ;;
        esac
        if jq -e '.dry_run == true' <<< "$arguments" >/dev/null; then
            set -- "$@" --dry-run
        fi
        "$@" --json > "$output" 2> "$output.stderr"
    else
        name="reqvire.workspace.$operation"
        [[ "$operation" != status ]] || name=reqvire.workspace_status
        # Read-only discovery does not expose branch selectors. Follow the
        # discovered schema instead of sending a mutation-context argument.
        if jq -e --arg name "$name" \
            'any(.result.tools[]; .name == $name and .inputSchema.properties.branch != null)' \
            "$CASE_DIR/tools.json" >/dev/null; then
            arguments=$(jq '. + {branch:"main"}' <<< "$arguments") || return 1
        fi
        params=$(jq -n --arg name "$name" --argjson args "$arguments" \
            '{name:$name,arguments:$args}') || return 1
        mcp_rpc tools/call "$params" "$output.raw" || return 1
        jq '.result.structuredContent // {}' "$output.raw" > "$output" || return 1
        jq -e '.error == null and .result.isError != true' "$output.raw" >/dev/null
    fi
}

ok_action() {
    local operation="$1" args="$2" output="$3"
    require_operation "$operation"
    if ! workspace_call "$operation" "$args" "$output"; then
        cat "$output" "$output.stderr" "$output.raw" 2>/dev/null >&2 || true
        die "workspace $operation failed"
    fi
    assert_json "$output" 'type == "object"'
}

status_projection() {
    local output="$1"
    ok_action status '{}' "$output"
    assert_json "$output" '(.context // .) | (.projects | type) == "array"'
    jq '(.context // .) | {repositories_dir, projects:[.projects[] | {path,ref,commit,head}] | sort_by(.path)}' \
        "$output" > "$output.projection" || die 'Cannot project workspace status'
}

assert_project() {
    local output="$1" path="$2" pin="$3" observed="$4"
    assert_json "$output" \
        '(.context // .) | any(.projects[]; .path == $path and .ref == "refs/heads/main" and .commit == $pin and .head == $observed)' \
        --arg path "$path" --arg pin "$pin" --arg observed "$observed"
}

prepare_backend() {
    [[ "$BACKEND" != mcp ]] || start_mcp mutations
}

commit_component_change() {
    printf 'local committed code change\n' > "$COMPONENT/code.txt"
    commit_all "$COMPONENT" local-code
    PIN_C=$(head_of "$COMPONENT")
}

legacy_fixture() {
    local label="$1"
    fixture "$label" yes flat
    mkdir -p "$ROOT/manifests" || die 'Cannot create legacy source directory'
    cat > "$ROOT/manifests/default.xml" <<EOF
<manifest>
  <remote name="local" fetch="file://$CASE_DIR/remotes/"/>
  <default remote="local" revision="refs/heads/main"/>
  <project name="website.git" path="vendor-product/website" revision="$PIN_A" upstream="refs/heads/main"/>
</manifest>
EOF
    # Native config is absent before conversion. The ordinary root model lets
    # MCP start under the existing validated-startup contract.
    git_do -C "$OWNER" rm -q reqvire-workspace.toml
    commit_all "$OWNER" legacy-manifest
    if [[ "$BACKEND" == cli ]]; then
        mkdir -p "$ROOT/.repo" || die 'Cannot create legacy manifest entry'
        ln -s ../manifests/default.xml "$ROOT/.repo/manifest.xml" ||
            die 'Cannot select legacy XML'
    fi
}
